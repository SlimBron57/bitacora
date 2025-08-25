//! Spark Management Command Handlers
//! 
//! SPARK es un servicio TRANSVERSAL que puede activarse en cualquier momento
//! durante el flujo PROJECT → TOPIC → ACTION para capturar insights y conocimientos

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use bitacora_core::models::{Spark, SparkStatus, SparkContext};
use bitacora_core::errors::BitacoraError;
use bitacora_records::services::SparkService;
use bitacora_storage::repository::{Repository, SparkRepository};
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::{BitacoraCommand, ParsedCommand};

/// Handler para comandos SPARK (servicio transversal)
/// 
/// SPARK es TRANSVERSAL al flujo PROJECT → TOPIC → ACTION
/// Puede ser activado en cualquier momento para capturar insights
pub struct SparkHandler<R>
where 
    R: Repository<Spark> + SparkRepository + Send + Sync,
{
    spark_service: Arc<SparkService<R>>,
}

impl<R> SparkHandler<R>
where
    R: Repository<Spark> + SparkRepository + Send + Sync,
{
    pub fn new(spark_service: Arc<SparkService<R>>) -> Self {
        Self { spark_service }
    }
}

#[async_trait]
impl<R> CommandHandler for SparkHandler<R>
where
    R: Repository<Spark> + SparkRepository + Send + Sync,
{
    fn command_name(&self) -> &'static str {
        "spark"
    }

    fn description(&self) -> &'static str {
        "Captura insights y conocimientos (servicio TRANSVERSAL a PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        match command.command {
            BitacoraCommand::Spark { subcommand } => {
                match subcommand {
                    SparkSubcommand::Capture { content, context_type, tags, related_id } => {
                        self.handle_capture(context, content, context_type, tags, related_id).await
                    }
                    SparkSubcommand::List { status, context_type, tag } => {
                        self.handle_list(context, status, context_type, tag).await
                    }
                    SparkSubcommand::Show { spark_id } => {
                        self.handle_show(context, spark_id).await
                    }
                    SparkSubcommand::Review { spark_id } => {
                        self.handle_review(context, spark_id).await
                    }
                    SparkSubcommand::Apply { spark_id, application_notes } => {
                        self.handle_apply(context, spark_id, application_notes).await
                    }
                    SparkSubcommand::Archive { spark_id } => {
                        self.handle_archive(context, spark_id).await
                    }
                }
            }
            _ => ExecutionResult::error("Command not supported by SparkHandler"),
        }
    }
}

impl<R> SparkHandler<R>
where
    R: Repository<Spark> + SparkRepository + Send + Sync,
{
    /// Capturar un nuevo SPARK (insight/conocimiento) de forma transversal
    /// 
    /// TRANSVERSAL: Puede ser usado desde PROJECT, TOPIC o ACTION
    async fn handle_capture(
        &self,
        context: &ExecutionContext,
        content: String,
        context_type: Option<SparkContext>,
        tags: Option<Vec<String>>,
        related_id: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let spark_context = context_type.unwrap_or(SparkContext::General);
        let spark_tags = tags.unwrap_or_default();
        
        match self.spark_service.capture_spark(
            content.clone(),
            user_id.clone(),
            spark_context,
            spark_tags.clone(),
            related_id,
        ).await {
            Ok(spark) => {
                let context_info = match spark_context {
                    SparkContext::Project => "📁 Contexto: PROJECT",
                    SparkContext::Topic => "📋 Contexto: TOPIC", 
                    SparkContext::Action => "⚡ Contexto: ACTION",
                    SparkContext::General => "🌟 Contexto: GENERAL",
                };
                
                let tags_info = if spark_tags.is_empty() {
                    "".to_string()
                } else {
                    format!("\n🏷️  Tags: {}", spark_tags.join(", "))
                };

                ExecutionResult::success(format!(
                    "✨ SPARK capturado exitosamente!\n\
                     🆔 ID: {}\n\
                     {}{}\n\
                     📝 Contenido: {}\n\
                     \n\
                     🔄 SERVICIO TRANSVERSAL activado:\n\
                     PROJECT → TOPIC → ACTION\n\
                         ✨ SPARK puede activarse en cualquier momento\n\
                     \n\
                     💡 Usa 'spark review {}' para revisar este insight",
                    spark.spark_id,
                    context_info,
                    tags_info,
                    content,
                    spark.spark_id
                ))
            }
            Err(e) => ExecutionResult::error(&format!("Error capturando spark: {}", e)),
        }
    }

    /// Listar SPARKs con filtros opcionales
    async fn handle_list(
        &self,
        context: &ExecutionContext,
        status_filter: Option<SparkStatus>,
        context_filter: Option<SparkContext>,
        tag_filter: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        
        match self.spark_service.get_user_sparks(user_id).await {
            Ok(sparks) => {
                let mut filtered_sparks = sparks;
                
                if let Some(status) = status_filter {
                    filtered_sparks.retain(|s| s.status == status);
                }
                
                if let Some(context) = context_filter {
                    filtered_sparks.retain(|s| s.context == context);
                }
                
                if let Some(tag) = tag_filter {
                    filtered_sparks.retain(|s| s.tags.contains(&tag));
                }

                if filtered_sparks.is_empty() {
                    ExecutionResult::success("✨ No se encontraron SPARKs".to_string())
                } else {
                    let mut output = String::new();
                    output.push_str("✨ SPARKs (SERVICIO TRANSVERSAL):\n");
                    output.push_str("   Puede activarse en cualquier momento durante PROJECT → TOPIC → ACTION\n\n");
                    
                    for spark in filtered_sparks {
                        let context_icon = match spark.context {
                            SparkContext::Project => "📁",
                            SparkContext::Topic => "📋",
                            SparkContext::Action => "⚡",
                            SparkContext::General => "🌟",
                        };
                        
                        let preview = if spark.content.len() > 50 {
                            format!("{}...", &spark.content[..50])
                        } else {
                            spark.content.clone()
                        };
                        
                        let tags_display = if spark.tags.is_empty() {
                            "".to_string()
                        } else {
                            format!(" | 🏷️  {}", spark.tags.join(", "))
                        };

                        output.push_str(&format!(
                            "   ✨ {} ({}) \n\
                             {}    {} {:?} | Estado: {:?}{}\n\
                             {}    📝 \"{}\"\n\
                             {}    📅 {}\n",
                            spark.spark_id.split('_').last().unwrap_or(&spark.spark_id),
                            spark.spark_id,
                            "   ",
                            context_icon,
                            spark.context,
                            spark.status,
                            tags_display,
                            "   ",
                            preview,
                            "   ",
                            spark.created_at.format("%Y-%m-%d %H:%M")
                        ));
                    }
                    output.push_str("\n💡 Usa 'spark show <id>' para ver detalles completos");
                    
                    ExecutionResult::success(output)
                }
            }
            Err(e) => ExecutionResult::error(&format!("Error listando sparks: {}", e)),
        }
    }

    /// Mostrar detalles completos de un SPARK
    async fn handle_show(
        &self,
        context: &ExecutionContext,
        spark_id: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let full_spark_id = if spark_id.contains('_') {
            spark_id
        } else {
            format!("{}_{}", user_id, spark_id)
        };
        
        match self.spark_service.get_spark(&full_spark_id).await {
            Ok(Some(spark)) => {
                let context_info = match spark.context {
                    SparkContext::Project => "📁 PROJECT",
                    SparkContext::Topic => "📋 TOPIC",
                    SparkContext::Action => "⚡ ACTION", 
                    SparkContext::General => "🌟 GENERAL",
                };
                
                let tags_info = if spark.tags.is_empty() {
                    "Sin tags".to_string()
                } else {
                    spark.tags.join(", ")
                };

                let related_info = if let Some(related) = &spark.related_id {
                    format!("🔗 Relacionado: {}\n", related)
                } else {
                    "".to_string()
                };

                let output = format!(
                    "✨ SPARK: {}\n\
                     🆔 ID completo: {}\n\
                     📝 Contenido: {}\n\
                     {} Contexto: {}\n\
                     📊 Estado: {:?}\n\
                     🏷️  Tags: {}\n\
                     {}📅 Capturado: {}\n\
                     📅 Actualizado: {}\n\
                     \n\
                     🔄 SERVICIO TRANSVERSAL:\n\
                     PROJECT → TOPIC → ACTION\n\
                         ✨ SPARK activo en cualquier momento\n\
                     \n\
                     💡 Comandos disponibles:\n\
                     • Revisar: 'spark review {}'\n\
                     • Aplicar: 'spark apply {}'\n\
                     • Archivar: 'spark archive {}'",
                    spark.spark_id.split('_').last().unwrap_or(&spark.spark_id),
                    spark.spark_id,
                    spark.content,
                    context_info.chars().next().unwrap_or('🌟'),
                    context_info.chars().skip(2).collect::<String>(),
                    spark.status,
                    tags_info,
                    related_info,
                    spark.created_at.format("%Y-%m-%d %H:%M"),
                    spark.updated_at.format("%Y-%m-%d %H:%M"),
                    spark.spark_id.split('_').last().unwrap_or(&spark.spark_id),
                    spark.spark_id.split('_').last().unwrap_or(&spark.spark_id),
                    spark.spark_id.split('_').last().unwrap_or(&spark.spark_id)
                );
                ExecutionResult::success(output)
            }
            Ok(None) => ExecutionResult::error(&format!("Spark '{}' no encontrado", spark_id)),
            Err(e) => ExecutionResult::error(&format!("Error buscando spark: {}", e)),
        }
    }

    /// Marcar SPARK como revisado
    async fn handle_review(
        &self,
        context: &ExecutionContext,
        spark_id: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let full_spark_id = if spark_id.contains('_') {
            spark_id
        } else {
            format!("{}_{}", user_id, spark_id)
        };
        
        match self.spark_service.update_spark_status(&full_spark_id, SparkStatus::Reviewed).await {
            Ok(true) => ExecutionResult::success(format!(
                "👁️ SPARK '{}' marcado como revisado\n💡 Usa 'spark apply {}' si quieres aplicar este insight", 
                spark_id, spark_id
            )),
            Ok(false) => ExecutionResult::error(&format!("Spark '{}' no encontrado", spark_id)),
            Err(e) => ExecutionResult::error(&format!("Error revisando spark: {}", e)),
        }
    }

    /// Aplicar SPARK (marcar como aplicado con notas)
    async fn handle_apply(
        &self,
        context: &ExecutionContext,
        spark_id: String,
        application_notes: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let full_spark_id = if spark_id.contains('_') {
            spark_id.clone()
        } else {
            format!("{}_{}", user_id, spark_id)
        };
        
        match self.spark_service.update_spark_status(&full_spark_id, SparkStatus::Applied).await {
            Ok(true) => {
                let notes_msg = if let Some(notes) = application_notes {
                    format!("📝 Notas de aplicación: {}\n", notes)
                } else {
                    "".to_string()
                };

                ExecutionResult::success(format!(
                    "🎯 SPARK '{}' aplicado exitosamente!\n{}✨ Insight integrado en tu flujo de trabajo", 
                    spark_id, notes_msg
                ))
            }
            Ok(false) => ExecutionResult::error(&format!("Spark '{}' no encontrado", spark_id)),
            Err(e) => ExecutionResult::error(&format!("Error aplicando spark: {}", e)),
        }
    }

    /// Archivar SPARK
    async fn handle_archive(
        &self,
        context: &ExecutionContext,
        spark_id: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let full_spark_id = if spark_id.contains('_') {
            spark_id.clone()
        } else {
            format!("{}_{}", user_id, spark_id)
        };
        
        match self.spark_service.update_spark_status(&full_spark_id, SparkStatus::Archived).await {
            Ok(true) => ExecutionResult::success(format!(
                "📦 SPARK '{}' archivado\n💾 Insight conservado para referencia futura", 
                spark_id
            )),
            Ok(false) => ExecutionResult::error(&format!("Spark '{}' no encontrado", spark_id)),
            Err(e) => ExecutionResult::error(&format!("Error archivando spark: {}", e)),
        }
    }
}

/// Subcomandos para gestión de SPARKs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SparkSubcommand {
    /// Capturar nuevo spark/insight
    Capture {
        content: String,
        context_type: Option<SparkContext>,
        tags: Option<Vec<String>>,
        related_id: Option<String>,
    },
    /// Listar sparks con filtros
    List {
        status: Option<SparkStatus>,
        context_type: Option<SparkContext>,
        tag: Option<String>,
    },
    /// Mostrar detalles de spark
    Show {
        spark_id: String,
    },
    /// Marcar spark como revisado
    Review {
        spark_id: String,
    },
    /// Aplicar spark con notas opcionales
    Apply {
        spark_id: String,
        application_notes: Option<String>,
    },
    /// Archivar spark
    Archive {
        spark_id: String,
    },
}
