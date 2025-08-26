//! Action Management Command Handlers
//! 
//! Siguiendo la arquitectura secuencial: PROJECT → TOPIC → ACTION
//! ACTION es el nivel final donde se ejecuta el trabajo específico

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use bitacora_core::models::{Action, ActionStatus, ActionType, Priority};
use bitacora_core::errors::BitacoraError;
use bitacora_storage::repository::{Repository, ActionRepository, TopicRepository};
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::{BitacoraCommand, ParsedCommand};

/// Handler para comandos relacionados con ACTION management
/// 
/// PROJECT → TOPIC → ACTION
///                   ^^^^^^^ Este handler gestiona el nivel final de la secuencia
pub struct ActionHandler<AR, TR>
where 
    AR: Repository<Action> + ActionRepository + Send + Sync,
    TR: Repository<bitacora_core::models::Topic> + TopicRepository + Send + Sync,
{
    action_repo: Arc<AR>,
    topic_repo: Arc<TR>,
}

impl<AR, TR> ActionHandler<AR, TR>
where
    AR: Repository<Action> + ActionRepository + Send + Sync,
    TR: Repository<bitacora_core::models::Topic> + TopicRepository + Send + Sync,
{
    pub fn new(action_repo: Arc<AR>, topic_repo: Arc<TR>) -> Self {
        Self { 
            action_repo,
            topic_repo,
        }
    }
}

#[async_trait]
impl<AR, TR> CommandHandler for ActionHandler<AR, TR>
where
    AR: Repository<Action> + ActionRepository + Send + Sync,
    TR: Repository<bitacora_core::models::Topic> + TopicRepository + Send + Sync,
{
    fn command_name(&self) -> &'static str {
        "action"
    }

    fn description(&self) -> &'static str {
        "Gestiona acciones de desarrollo (nivel final en PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        match command.command {
            BitacoraCommand::Action { subcommand } => {
                match subcommand {
                    ActionSubcommand::Create { topic_name, name, description, action_type, priority, estimated_duration } => {
                        self.handle_create(context, topic_name, name, description, action_type, priority, estimated_duration).await
                    }
                    ActionSubcommand::List { topic_name, status, action_type } => {
                        self.handle_list(context, topic_name, status, action_type).await
                    }
                    ActionSubcommand::Show { name } => {
                        self.handle_show(context, name).await
                    }
                    ActionSubcommand::Start { name } => {
                        self.handle_start(context, name).await
                    }
                    ActionSubcommand::Complete { name, result_description } => {
                        self.handle_complete(context, name, result_description).await
                    }
                    ActionSubcommand::Block { name, blocker_description } => {
                        self.handle_block(context, name, blocker_description).await
                    }
                    ActionSubcommand::Cancel { name, reason } => {
                        self.handle_cancel(context, name, reason).await
                    }
                }
            }
            _ => ExecutionResult::error("Command not supported by ActionHandler"),
        }
    }
}

impl<AR, TR> ActionHandler<AR, TR>
where
    AR: Repository<Action> + ActionRepository + Send + Sync,
    TR: Repository<bitacora_core::models::Topic> + TopicRepository + Send + Sync,
{
    /// Crear nueva ACTION dentro de un TOPIC
    /// PROJECT → TOPIC → ACTION
    ///                   ^^^^^^^ Completamos la secuencia con trabajo específico
    async fn handle_create(
        &self,
        context: &ExecutionContext,
        topic_name: String,
        name: String,
        description: Option<String>,
        action_type: Option<ActionType>,
        priority: Option<Priority>,
        estimated_duration: Option<i64>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let topic_id = format!("{}_{}", user_id, topic_name.to_lowercase().replace(" ", "-"));
        
        // Verificar que el topic existe
        match self.topic_repo.get_by_id(&topic_id).await {
            Ok(Some(topic)) => {
                let desc = description.unwrap_or_else(|| format!("Acción {} en tema {}", name, topic_name));
                let act_type = action_type.unwrap_or(ActionType::Development);
                let prio = priority.unwrap_or(Priority::Medium);

                // Crear action usando el modelo de dominio
                match Action::new(
                    name.clone(),
                    desc.clone(),
                    user_id.clone(),
                    topic_id.clone(),
                    act_type,
                    prio,
                    estimated_duration,
                ) {
                    Ok(action) => {
                        match self.action_repo.create(&action).await {
                            Ok(_) => ExecutionResult::success(format!(
                                "✅ ACTION '{}' creada en TOPIC '{}'\n   📝 ID: {}\n   🔄 Flujo: PROJECT → TOPIC → ACTION\n                              ^^^^^^^ Completado!\n   💡 Usa 'action start {}' para comenzar a trabajar", 
                                name, topic_name, action.action_id, name
                            )),
                            Err(e) => ExecutionResult::error(&format!("Error creando action: {}", e)),
                        }
                    }
                    Err(e) => ExecutionResult::error(&format!("Error en datos de la action: {}", e)),
                }
            }
            Ok(None) => ExecutionResult::error(&format!("TOPIC '{}' no encontrado. Créalo primero con 'topic create'", topic_name)),
            Err(e) => ExecutionResult::error(&format!("Error verificando topic: {}", e)),
        }
    }

    /// Listar ACTIONs dentro de un TOPIC o todas las del usuario
    async fn handle_list(
        &self,
        context: &ExecutionContext,
        topic_name: Option<String>,
        status_filter: Option<ActionStatus>,
        type_filter: Option<ActionType>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        
        let actions = if let Some(topic) = topic_name {
            let topic_id = format!("{}_{}", user_id, topic.to_lowercase().replace(" ", "-"));
            match self.action_repo.get_by_topic_id(&topic_id).await {
                Ok(actions) => actions,
                Err(e) => return ExecutionResult::error(&format!("Error obteniendo actions del topic: {}", e)),
            }
        } else {
            match self.action_repo.get_by_user_id(user_id).await {
                Ok(actions) => actions,
                Err(e) => return ExecutionResult::error(&format!("Error obteniendo actions del usuario: {}", e)),
            }
        };

        let mut filtered_actions = actions;
        
        if let Some(status) = status_filter {
            filtered_actions.retain(|a| a.status == status);
        }
        
        if let Some(action_type) = type_filter {
            filtered_actions.retain(|a| a.action_type == action_type);
        }

        if filtered_actions.is_empty() {
            let context_msg = if topic_name.is_some() {
                format!("en TOPIC '{}'", topic_name.unwrap())
            } else {
                "".to_string()
            };
            ExecutionResult::success(format!("⚡ No se encontraron ACTIONs {}", context_msg))
        } else {
            let mut output = String::new();
            let header = if let Some(topic) = &topic_name {
                format!("⚡ ACTIONs en TOPIC '{}' (PROJECT → TOPIC → ACTION):\n", topic)
            } else {
                "⚡ Todas las ACTIONs (PROJECT → TOPIC → ACTION):\n".to_string()
            };
            output.push_str(&header);
            
            for action in filtered_actions {
                let duration_info = if let Some(dur) = action.estimated_duration_minutes {
                    format!(" | ⏱️ ~{}min", dur)
                } else {
                    "".to_string()
                };

                output.push_str(&format!(
                    "   ⚡ {} ({})\n      📋 Tipo: {:?} | Estado: {:?} | Prioridad: {:?}{}\n      📅 Creado: {}\n",
                    action.name,
                    action.action_id,
                    action.action_type,
                    action.status,
                    action.priority,
                    duration_info,
                    action.created_at.format("%Y-%m-%d %H:%M")
                ));
            }
            output.push_str("\n💡 Usa 'action start <nombre>' para comenzar una acción");
            
            ExecutionResult::success(output)
        }
    }

    /// Mostrar detalles de una ACTION específica
    async fn handle_show(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let action_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.action_repo.get_by_id(&action_id).await {
            Ok(Some(action)) => {
                let duration_info = if let Some(dur) = action.estimated_duration_minutes {
                    format!("⏱️ Duración estimada: {} minutos\n", dur)
                } else {
                    "".to_string()
                };

                let time_info = match (action.started_at, action.completed_at) {
                    (Some(started), Some(completed)) => {
                        let duration = completed.signed_duration_since(started);
                        format!(
                            "🕐 Iniciado: {}\n🏁 Completado: {}\n⏱️ Duración real: {} minutos\n",
                            started.format("%Y-%m-%d %H:%M"),
                            completed.format("%Y-%m-%d %H:%M"),
                            duration.num_minutes()
                        )
                    }
                    (Some(started), None) => {
                        format!("🕐 Iniciado: {}\n", started.format("%Y-%m-%d %H:%M"))
                    }
                    _ => "".to_string()
                };

                let result_info = if let Some(result) = &action.result_description {
                    format!("📋 Resultado: {}\n", result)
                } else {
                    "".to_string()
                };

                let output = format!(
                    "⚡ ACTION: {}\n\
                     🆔 ID: {}\n\
                     📝 Descripción: {}\n\
                     📋 Tema: {}\n\
                     🔧 Tipo: {:?}\n\
                     📊 Estado: {:?}\n\
                     🎯 Prioridad: {:?}\n\
                     {}\
                     📅 Creado: {}\n\
                     📅 Actualizado: {}\n\
                     {}\
                     {}\
                     \n\
                     🔄 Flujo de trabajo completado:\n\
                     PROJECT → TOPIC → ACTION\n\
                                       ^^^^^^^ Trabajo específico aquí\n\
                     \n\
                     💡 Comandos disponibles:\n\
                     • Iniciar: 'action start {}'\n\
                     • Completar: 'action complete {}'\n\
                     • Bloquear: 'action block {}'",
                    action.name,
                    action.action_id,
                    action.description,
                    action.topic_id,
                    action.action_type,
                    action.status,
                    action.priority,
                    duration_info,
                    action.created_at.format("%Y-%m-%d %H:%M"),
                    action.updated_at.format("%Y-%m-%d %H:%M"),
                    time_info,
                    result_info,
                    name,
                    name,
                    name
                );
                ExecutionResult::success(output)
            }
            Ok(None) => ExecutionResult::error(&format!("Action '{}' no encontrada", name)),
            Err(e) => ExecutionResult::error(&format!("Error buscando action: {}", e)),
        }
    }

    /// Iniciar ACTION (cambiar estado a InProgress y marcar tiempo)
    async fn handle_start(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let action_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.action_repo.update_action_status(&action_id, ActionStatus::InProgress).await {
            Ok(true) => {
                // También actualizar el tiempo de inicio
                match self.action_repo.update_action_started_at(&action_id, Some(Utc::now())).await {
                    Ok(_) => ExecutionResult::success(format!(
                        "🚀 ACTION '{}' iniciada!\n⏱️ Tiempo de inicio registrado\n💪 ¡A trabajar en el nivel final de PROJECT → TOPIC → ACTION!", 
                        name
                    )),
                    Err(e) => ExecutionResult::error(&format!("Error actualizando tiempo de inicio: {}", e)),
                }
            }
            Ok(false) => ExecutionResult::error(&format!("Action '{}' no encontrada", name)),
            Err(e) => ExecutionResult::error(&format!("Error iniciando action: {}", e)),
        }
    }

    /// Completar ACTION (marcar como completada y registrar resultado)
    async fn handle_complete(
        &self,
        context: &ExecutionContext,
        name: String,
        result_description: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let action_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        // Actualizar estado a completado
        match self.action_repo.update_action_status(&action_id, ActionStatus::Completed).await {
            Ok(true) => {
                let now = Utc::now();
                
                // Actualizar tiempo de completado
                if let Err(e) = self.action_repo.update_action_completed_at(&action_id, Some(now)).await {
                    return ExecutionResult::error(&format!("Error actualizando tiempo de completado: {}", e));
                }
                
                // Actualizar descripción del resultado si se proporciona
                if let Some(result) = result_description {
                    if let Err(e) = self.action_repo.update_action_result(&action_id, Some(result.clone())).await {
                        return ExecutionResult::error(&format!("Error actualizando resultado: {}", e));
                    }
                }

                let result_msg = if let Some(result) = &result_description {
                    format!("📋 Resultado: {}\n", result)
                } else {
                    "".to_string()
                };

                ExecutionResult::success(format!(
                    "🎉 ACTION '{}' completada exitosamente!\n{}⏱️ Tiempo de completado registrado\n✨ Flujo PROJECT → TOPIC → ACTION finalizado para esta acción", 
                    name, result_msg
                ))
            }
            Ok(false) => ExecutionResult::error(&format!("Action '{}' no encontrada", name)),
            Err(e) => ExecutionResult::error(&format!("Error completando action: {}", e)),
        }
    }

    /// Bloquear ACTION (marcar como bloqueada con descripción del bloqueador)
    async fn handle_block(
        &self,
        context: &ExecutionContext,
        name: String,
        blocker_description: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let action_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        // Actualizar estado a bloqueado
        match self.action_repo.update_action_status(&action_id, ActionStatus::Blocked).await {
            Ok(true) => {
                // Actualizar descripción del bloqueador
                match self.action_repo.update_action_result(&action_id, Some(format!("BLOQUEADO: {}", blocker_description))).await {
                    Ok(_) => ExecutionResult::success(format!(
                        "🚫 ACTION '{}' bloqueada\n📝 Motivo: {}\n💡 Resuelve el bloqueo y usa 'action start {}' para continuar", 
                        name, blocker_description, name
                    )),
                    Err(e) => ExecutionResult::error(&format!("Error actualizando descripción del bloqueo: {}", e)),
                }
            }
            Ok(false) => ExecutionResult::error(&format!("Action '{}' no encontrada", name)),
            Err(e) => ExecutionResult::error(&format!("Error bloqueando action: {}", e)),
        }
    }

    /// Cancelar ACTION (marcar como cancelada con razón)
    async fn handle_cancel(
        &self,
        context: &ExecutionContext,
        name: String,
        reason: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let action_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        // Actualizar estado a cancelado
        match self.action_repo.update_action_status(&action_id, ActionStatus::Cancelled).await {
            Ok(true) => {
                // Actualizar razón de cancelación si se proporciona
                if let Some(reason) = &reason {
                    if let Err(e) = self.action_repo.update_action_result(&action_id, Some(format!("CANCELADO: {}", reason))).await {
                        return ExecutionResult::error(&format!("Error actualizando razón de cancelación: {}", e));
                    }
                }

                let reason_msg = if let Some(reason) = reason {
                    format!("📝 Razón: {}\n", reason)
                } else {
                    "".to_string()
                };

                ExecutionResult::success(format!(
                    "❌ ACTION '{}' cancelada\n{}📊 Datos conservados para análisis futuro", 
                    name, reason_msg
                ))
            }
            Ok(false) => ExecutionResult::error(&format!("Action '{}' no encontrada", name)),
            Err(e) => ExecutionResult::error(&format!("Error cancelando action: {}", e)),
        }
    }
}

/// Subcomandos para gestión de ACTIONs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionSubcommand {
    /// Crear nueva action en un topic
    Create {
        topic_name: String,
        name: String,
        description: Option<String>,
        action_type: Option<ActionType>,
        priority: Option<Priority>,
        estimated_duration: Option<i64>,
    },
    /// Listar actions
    List {
        topic_name: Option<String>,
        status: Option<ActionStatus>,
        action_type: Option<ActionType>,
    },
    /// Mostrar detalles de action
    Show {
        name: String,
    },
    /// Iniciar action
    Start {
        name: String,
    },
    /// Completar action
    Complete {
        name: String,
        result_description: Option<String>,
    },
    /// Bloquear action
    Block {
        name: String,
        blocker_description: String,
    },
    /// Cancelar action
    Cancel {
        name: String,
        reason: Option<String>,
    },
}
