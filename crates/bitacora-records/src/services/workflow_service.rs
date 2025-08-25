//! Workflow Management Service

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;

use bitacora_core::models::{Topic, Spark, Action, SparkType, SparkStatus, ActionType, Priority};
use bitacora_core::errors::BitacoraError;
use bitacora_storage::repository::{Repository, TopicRepository, SparkRepository, ActionRepository};
use bitacora_storage::{StorageResult, StorageError};

use super::{TopicService, SparkService};

/// Service para gestión de workflows integrados Topic-Spark-Action
pub struct WorkflowService<TR, SR, AR>
where
    TR: Repository<Topic> + TopicRepository,
    SR: Repository<Spark> + SparkRepository,
    AR: Repository<Action> + ActionRepository,
{
    topic_service: TopicService<TR>,
    spark_service: SparkService<SR>,
    action_repo: Arc<AR>,
}

impl<TR, SR, AR> WorkflowService<TR, SR, AR>
where
    TR: Repository<Topic> + TopicRepository,
    SR: Repository<Spark> + SparkRepository,
    AR: Repository<Action> + ActionRepository,
{
    pub fn new(
        topic_service: TopicService<TR>,
        spark_service: SparkService<SR>,
        action_repo: Arc<AR>,
    ) -> Self {
        Self {
            topic_service,
            spark_service,
            action_repo,
        }
    }

    /// Crear Topic con Spark inicial derivado de una acción
    pub async fn create_topic_from_action_insight(
        &self,
        user_id: String,
        project_id: Option<Uuid>,
        action_description: String,
        insight_content: String,
        topic_title: String,
        topic_description: String,
    ) -> Result<(Topic, Spark), BitacoraError> {
        // Crear el Topic
        let topic = self.topic_service.create_topic(
            topic_title,
            topic_description,
            user_id.clone(),
            project_id,
        ).await?;

        // Crear Spark derivado de la acción y asociarlo al topic
        let mut spark = self.spark_service.create_spark_from_action(
            user_id,
            action_description,
            insight_content,
            SparkType::Insight,
        ).await?;

        // Asociar el spark al topic recién creado
        spark.topic_id = Some(topic.topic_id);
        self.spark_service.update_spark(spark.clone()).await?;

        Ok((topic, spark))
    }

    /// Crear Topic derivado de otro Topic con sus Sparks relacionados
    pub async fn create_derived_topic_with_sparks(
        &self,
        source_topic_id: &str,
        title: String,
        description: String,
        user_id: String,
        copy_related_sparks: bool,
    ) -> Result<(Topic, Vec<Spark>), BitacoraError> {
        // Crear topic derivado
        let new_topic = self.topic_service.create_derived_topic(
            source_topic_id,
            title,
            description,
            user_id.clone(),
        ).await?;

        let mut sparks = Vec::new();

        if copy_related_sparks {
            // Obtener sparks activos del topic fuente
            let all_sparks = self.spark_service.get_active_sparks().await?;
            let source_sparks: Vec<_> = all_sparks.into_iter()
                .filter(|spark| {
                    spark.topic_id.map(|id| id.to_string()).as_deref() == Some(source_topic_id)
                })
                .collect();

            // Crear copias de los sparks para el nuevo topic
            for source_spark in source_sparks {
                let new_spark = self.spark_service.create_spark(
                    user_id.clone(),
                    format!("Copy: {}", source_spark.title),
                    source_spark.content.clone(),
                    source_spark.spark_type.clone(),
                ).await?;
                
                // Asociar al nuevo topic
                let mut updated_spark = new_spark;
                updated_spark.topic_id = Some(new_topic.topic_id);
                self.spark_service.update_spark(updated_spark.clone()).await?;
                
                sparks.push(updated_spark);
            }
        }

        Ok((new_topic, sparks))
    }

    /// Workflow completo: Analizar actions de sesión y crear sparks automáticamente
    pub async fn auto_generate_sparks_from_session(
        &self,
        session_id: &str,
        user_id: String,
        topic_id: Option<Uuid>,
        project_id: Option<Uuid>,
    ) -> Result<Vec<Spark>, BitacoraError> {
        // Obtener actions de la sesión
        let actions = self.action_repo.get_by_session_id(session_id).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;

        let mut generated_sparks = Vec::new();

        for action in actions {
            // Solo procesar certain tipos de actions que pueden generar insights
            if matches!(action.action_type, 
                ActionType::FileEdit | 
                ActionType::Debug | 
                ActionType::Research |
                ActionType::Planning
            ) {
                // Generar insight basado en el tipo de acción
                let insight_content = self.generate_insight_from_action(&action);
                
                if !insight_content.is_empty() {
                    let mut spark = self.spark_service.create_spark_from_action(
                        user_id.clone(),
                        action.description.clone(),
                        insight_content,
                        SparkType::Insight,
                    ).await?;

                    // Asociar a topic y proyecto si están definidos
                    spark.topic_id = topic_id;
                    spark.project_id = project_id;
                    self.spark_service.update_spark(spark.clone()).await?;
                    
                    generated_sparks.push(spark);
                }
            }
        }

        Ok(generated_sparks)
    }

    /// Análisis de contexto: obtener sparks relacionados a un topic específico
    pub async fn get_topic_context_sparks(
        &self,
        topic_id: &str,
        include_status: Option<Vec<SparkStatus>>,
    ) -> Result<Vec<Spark>, BitacoraError> {
        let all_sparks = self.spark_service.list_sparks(None, None).await?;
        
        let topic_sparks = all_sparks.into_iter()
            .filter(|spark| {
                // Filtrar por topic
                let matches_topic = spark.topic_id
                    .map(|id| id.to_string())
                    .as_deref() == Some(topic_id);
                
                // Filtrar por estado si se especifica
                let matches_status = if let Some(ref statuses) = include_status {
                    statuses.contains(&spark.status)
                } else {
                    true
                };
                
                matches_topic && matches_status
            })
            .collect();

        Ok(topic_sparks)
    }

    /// Promover Spark a Topic: crear un nuevo topic basado en un spark existente
    pub async fn promote_spark_to_topic(
        &self,
        spark_id: &str,
        topic_title: String,
        topic_description: Option<String>,
    ) -> Result<(Topic, Spark), BitacoraError> {
        // Obtener el spark
        let spark = self.spark_service.get_spark(spark_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Spark not found".to_string()))?;

        // Crear topic basado en el spark
        let description = topic_description.unwrap_or_else(|| spark.content.clone());
        let topic = self.topic_service.create_topic(
            topic_title,
            description,
            spark.user_id.clone(),
            spark.project_id,
        ).await?;

        // Marcar el spark como implementado y asociarlo al nuevo topic
        let mut updated_spark = spark;
        updated_spark.topic_id = Some(topic.topic_id);
        self.spark_service.mark_implemented(&spark_id).await?;

        Ok((topic, updated_spark))
    }

    /// Generar reporte de actividad integrado: Topic-Spark-Action
    pub async fn generate_activity_report(
        &self,
        user_id: String,
        topic_id: Option<String>,
        limit_days: Option<u32>,
    ) -> Result<ActivityReport, BitacoraError> {
        let cutoff_date = limit_days.map(|days| {
            Utc::now() - chrono::Duration::days(days as i64)
        });

        // Obtener topics del usuario
        let mut topics = if let Some(topic_id) = topic_id {
            vec![self.topic_service.get_topic(&topic_id).await?
                .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?]
        } else {
            self.topic_service.get_user_topics(&user_id).await?
        };

        // Filtrar por fecha si se especifica
        if let Some(cutoff) = cutoff_date {
            topics.retain(|topic| topic.created_at >= cutoff);
        }

        // Obtener sparks relacionados
        let all_sparks = self.spark_service.get_active_sparks().await?;
        let sparks: Vec<_> = all_sparks.into_iter()
            .filter(|spark| {
                let matches_user = spark.user_id == user_id;
                let matches_date = cutoff_date.map_or(true, |cutoff| spark.created_at >= cutoff);
                let matches_topic = topic_id.as_ref().map_or(true, |id| {
                    spark.topic_id.map(|tid| tid.to_string()).as_deref() == Some(id)
                });
                
                matches_user && matches_date && matches_topic
            })
            .collect();

        Ok(ActivityReport {
            user_id,
            period_start: cutoff_date,
            period_end: Utc::now(),
            topics_count: topics.len(),
            sparks_count: sparks.len(),
            sparks_by_status: self.group_sparks_by_status(&sparks),
            topics,
            sparks,
        })
    }

    /// Helper: Generar insight content basado en una acción
    fn generate_insight_from_action(&self, action: &Action) -> String {
        match action.action_type {
            ActionType::FileEdit => {
                format!("File editing pattern detected: {}", action.description)
            },
            ActionType::Debug => {
                format!("Debugging insight: {}", action.description)
            },
            ActionType::Research => {
                format!("Research finding: {}", action.description)
            },
            ActionType::Planning => {
                format!("Planning insight: {}", action.description)
            },
            _ => String::new(),
        }
    }

    /// Helper: Agrupar sparks por estado
    fn group_sparks_by_status(&self, sparks: &[Spark]) -> std::collections::HashMap<SparkStatus, usize> {
        let mut counts = std::collections::HashMap::new();
        
        for spark in sparks {
            *counts.entry(spark.status.clone()).or_insert(0) += 1;
        }
        
        counts
    }
}

/// Reporte de actividad integrado
#[derive(Debug, Clone)]
pub struct ActivityReport {
    pub user_id: String,
    pub period_start: Option<chrono::DateTime<Utc>>,
    pub period_end: chrono::DateTime<Utc>,
    pub topics_count: usize,
    pub sparks_count: usize,
    pub sparks_by_status: std::collections::HashMap<SparkStatus, usize>,
    pub topics: Vec<Topic>,
    pub sparks: Vec<Spark>,
}
