//! Topic Management Command Handlers
//! 
//! Siguiendo la arquitectura secuencial: PROJECT → TOPIC → ACTION
//! TOPIC es el nivel medio que conecta proyectos con acciones específicas

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bitacora_core::models::{Topic, TopicStatus, Priority};
use bitacora_core::errors::BitacoraError;
use bitacora_storage::repository::{Repository, TopicRepository, ProjectRepository};
use bitacora_records::services::TopicService;
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::ParsedCommand;

/// Handler para comandos relacionados con TOPIC management
/// 
/// PROJECT → TOPIC → ACTION
///           ^^^^^^^ Este handler gestiona el nivel medio de la secuencia
pub struct TopicHandler<TR, PR>
where 
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    PR: Repository<bitacora_core::models::Project> + ProjectRepository + Send + Sync,
{
    topic_service: Arc<TopicService<TR>>,
    project_repo: Arc<PR>,
}

impl<TR, PR> TopicHandler<TR, PR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    PR: Repository<bitacora_core::models::Project> + ProjectRepository + Send + Sync,
{
    pub fn new(topic_service: Arc<TopicService<TR>>, project_repo: Arc<PR>) -> Self {
        Self { 
            topic_service,
            project_repo,
        }
    }
}

#[async_trait]
impl<TR, PR> CommandHandler for TopicHandler<TR, PR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    PR: Repository<bitacora_core::models::Project> + ProjectRepository + Send + Sync,
{
    fn command_name(&self) -> &'static str {
        "topic"
    }

    fn description(&self) -> &'static str {
        "Gestiona temas de desarrollo (nivel medio en PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        if command.command != "topic" {
            return ExecutionResult::error("Command not supported by TopicHandler");
        }
        
        // Implementation similar to ProjectHandler but for topics
        // This is simplified for now - real implementation would parse args properly
        ExecutionResult::success("Topic command processed".to_string())
    }
}

impl<TR, PR> TopicHandler<TR, PR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    PR: Repository<bitacora_core::models::Project> + ProjectRepository + Send + Sync,
{
    /// Crear nuevo TOPIC dentro de un PROJECT
    /// PROJECT → TOPIC → ACTION
    ///           ^^^^^^^ Continuamos la secuencia creando el nivel medio
    async fn handle_create(
        &self,
        context: &ExecutionContext,
        project_name: String,
        name: String,
        description: Option<String>,
        priority: Option<Priority>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, project_name.to_lowercase().replace(" ", "-"));
        
        // Verificar que el proyecto existe
        match self.project_repo.get_by_id(&project_id).await {
            Ok(Some(project)) => {
                let desc = description.unwrap_or_else(|| format!("Tema {} en proyecto {}", name, project_name));
                let prio = priority.unwrap_or(Priority::Medium);

                // Usar el servicio para crear el topic
                match self.topic_service.create_topic(
                    name.clone(),
                    desc,
                    user_id.clone(),
                    project_id.clone(),
                    prio,
                    vec![], // tags vacíos por ahora
                ).await {
                    Ok(topic) => ExecutionResult::success(format!(
                        "✅ TOPIC '{}' creado en PROJECT '{}'\n   📝 ID: {}\n   🔄 Flujo: PROJECT → TOPIC → ACTION\n                        ^^^^^^^ Estás aquí\n   💡 Próximo: crear ACTIONs dentro de este tema", 
                        name, project_name, topic.topic_id
                    )),
                    Err(e) => ExecutionResult::error(&format!("Error creando topic: {}", e)),
                }
            }
            Ok(None) => ExecutionResult::error(&format!("PROJECT '{}' no encontrado. Créalo primero con 'project create'", project_name)),
            Err(e) => ExecutionResult::error(&format!("Error verificando proyecto: {}", e)),
        }
    }

    /// Listar TOPICs dentro de un PROJECT
    async fn handle_list(
        &self,
        context: &ExecutionContext,
        project_name: Option<String>,
        status_filter: Option<TopicStatus>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        
        if let Some(proj_name) = project_name {
            let project_id = format!("{}_{}", user_id, proj_name.to_lowercase().replace(" ", "-"));
            
            // Usar servicio para obtener topics del proyecto
            match self.topic_service.get_topics_by_project(&project_id).await {
                Ok(topics) => {
                    let filtered_topics: Vec<_> = if let Some(status) = status_filter {
                        topics.into_iter().filter(|t| t.status == status).collect()
                    } else {
                        topics
                    };

                    if filtered_topics.is_empty() {
                        ExecutionResult::success(format!("📝 No se encontraron TOPICs en PROJECT '{}'", proj_name))
                    } else {
                        let mut output = String::new();
                        output.push_str(&format!("📝 TOPICs en PROJECT '{}' (PROJECT → TOPIC → ACTION):\n", proj_name));
                        
                        for topic in filtered_topics {
                            output.push_str(&format!(
                                "   📋 {} ({})\n      🎯 Prioridad: {:?} | Estado: {:?}\n      📅 Creado: {}\n",
                                topic.name,
                                topic.topic_id,
                                topic.priority,
                                topic.status,
                                topic.created_at.format("%Y-%m-%d %H:%M")
                            ));
                        }
                        output.push_str("\n💡 Usa 'action list --topic <nombre>' para ver ACTIONs dentro de un topic");
                        
                        ExecutionResult::success(output)
                    }
                }
                Err(e) => ExecutionResult::error(&format!("Error listando topics: {}", e)),
            }
        } else {
            // Listar todos los topics del usuario
            match self.topic_service.get_user_topics(user_id).await {
                Ok(topics) => {
                    let filtered_topics: Vec<_> = if let Some(status) = status_filter {
                        topics.into_iter().filter(|t| t.status == status).collect()
                    } else {
                        topics
                    };

                    if filtered_topics.is_empty() {
                        ExecutionResult::success("📝 No se encontraron TOPICs".to_string())
                    } else {
                        let mut output = String::new();
                        output.push_str("📝 Todos los TOPICs (PROJECT → TOPIC → ACTION):\n");
                        
                        for topic in filtered_topics {
                            output.push_str(&format!(
                                "   📋 {} ({})\n      📁 Proyecto: {}\n      🎯 Prioridad: {:?} | Estado: {:?}\n      📅 Creado: {}\n",
                                topic.name,
                                topic.topic_id,
                                topic.project_id,
                                topic.priority,
                                topic.status,
                                topic.created_at.format("%Y-%m-%d %H:%M")
                            ));
                        }
                        output.push_str("\n💡 Usa 'topic list --project <nombre>' para filtrar por proyecto");
                        
                        ExecutionResult::success(output)
                    }
                }
                Err(e) => ExecutionResult::error(&format!("Error listando topics: {}", e)),
            }
        }
    }

    /// Mostrar detalles de un TOPIC específico
    async fn handle_show(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let topic_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.topic_service.get_topic(&topic_id).await {
            Ok(Some(topic)) => {
                let output = format!(
                    "📋 TOPIC: {}\n\
                     🆔 ID: {}\n\
                     📁 Proyecto: {}\n\
                     📝 Descripción: {}\n\
                     🎯 Prioridad: {:?}\n\
                     📊 Estado: {:?}\n\
                     🏷️  Tags: {}\n\
                     📅 Creado: {}\n\
                     📅 Actualizado: {}\n\
                     \n\
                     🔄 Flujo de trabajo:\n\
                     PROJECT → TOPIC → ACTION\n\
                               ^^^^^^^ Estás aquí\n\
                     \n\
                     💡 Próximos pasos:\n\
                     • Crea ACTIONs: 'action create --topic {}'\n\
                     • Ve ACTIONs existentes: 'action list --topic {}'",
                    topic.name,
                    topic.topic_id,
                    topic.project_id,
                    topic.description,
                    topic.priority,
                    topic.status,
                    topic.tags.join(", "),
                    topic.created_at.format("%Y-%m-%d %H:%M"),
                    topic.updated_at.format("%Y-%m-%d %H:%M"),
                    name,
                    name
                );
                ExecutionResult::success(output)
            }
            Ok(None) => ExecutionResult::error(&format!("Topic '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error buscando topic: {}", e)),
        }
    }

    /// Activar TOPIC (cambiar estado a Active)
    async fn handle_activate(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let topic_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.topic_service.update_topic_status(&topic_id, TopicStatus::Active).await {
            Ok(true) => ExecutionResult::success(format!(
                "✅ TOPIC '{}' activado\n🔄 Ahora puedes crear y trabajar en ACTIONs dentro de este tema", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Topic '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error activando topic: {}", e)),
        }
    }

    /// Completar TOPIC (cambiar estado a Completed)
    async fn handle_complete(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let topic_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.topic_service.update_topic_status(&topic_id, TopicStatus::Completed).await {
            Ok(true) => ExecutionResult::success(format!(
                "🎉 TOPIC '{}' completado!\n📈 Todas las ACTIONs quedan registradas, tema listo para análisis", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Topic '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error completando topic: {}", e)),
        }
    }

    /// Archivar TOPIC (cambiar estado a Archived)
    async fn handle_archive(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let topic_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.topic_service.update_topic_status(&topic_id, TopicStatus::Archived).await {
            Ok(true) => ExecutionResult::success(format!(
                "📦 TOPIC '{}' archivado\n💾 ACTIONs conservadas para referencia futura", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Topic '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error archivando topic: {}", e)),
        }
    }
}


