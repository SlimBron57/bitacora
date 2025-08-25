//! Workflow Management Command Handlers
//! 
//! WORKFLOW integra toda la arquitectura secuencial PROJECT → TOPIC → ACTION
//! con SPARK como servicio transversal para gestión completa del flujo de trabajo

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use bitacora_core::models::{Project, Topic, Action, Spark, WorkflowSummary};
use bitacora_core::errors::BitacoraError;
use bitacora_records::services::WorkflowService;
use bitacora_storage::repository::{Repository, TopicRepository, ActionRepository, SparkRepository};
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::{BitacoraCommand, ParsedCommand};

/// Handler para comandos de WORKFLOW (integración completa)
/// 
/// Gestiona la arquitectura completa: PROJECT → TOPIC → ACTION + SPARK (transversal)
pub struct WorkflowHandler<TR, AR, SR>
where 
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    AR: Repository<Action> + ActionRepository + Send + Sync, 
    SR: Repository<Spark> + SparkRepository + Send + Sync,
{
    workflow_service: Arc<WorkflowService<TR, AR, SR>>,
}

impl<TR, AR, SR> WorkflowHandler<TR, AR, SR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    AR: Repository<Action> + ActionRepository + Send + Sync,
    SR: Repository<Spark> + SparkRepository + Send + Sync,
{
    pub fn new(workflow_service: Arc<WorkflowService<TR, AR, SR>>) -> Self {
        Self { workflow_service }
    }
}

#[async_trait]
impl<TR, AR, SR> CommandHandler for WorkflowHandler<TR, AR, SR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    AR: Repository<Action> + ActionRepository + Send + Sync,
    SR: Repository<Spark> + SparkRepository + Send + Sync,
{
    fn command_name(&self) -> &'static str {
        "workflow"
    }

    fn description(&self) -> &'static str {
        "Gestiona flujos de trabajo completos (PROJECT → TOPIC → ACTION + SPARK)"
    }

    async fn handle(&self, context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        match command.command {
            BitacoraCommand::Workflow { subcommand } => {
                match subcommand {
                    WorkflowSubcommand::Status { project_name } => {
                        self.handle_status(context, project_name).await
                    }
                    WorkflowSubcommand::Summary { project_name, include_sparks } => {
                        self.handle_summary(context, project_name, include_sparks).await
                    }
                    WorkflowSubcommand::Progress { project_name } => {
                        self.handle_progress(context, project_name).await
                    }
                    WorkflowSubcommand::Timeline { project_name, days } => {
                        self.handle_timeline(context, project_name, days).await
                    }
                    WorkflowSubcommand::Insights { project_name, context_filter } => {
                        self.handle_insights(context, project_name, context_filter).await
                    }
                }
            }
            _ => ExecutionResult::error("Command not supported by WorkflowHandler"),
        }
    }
}

impl<TR, AR, SR> WorkflowHandler<TR, AR, SR>
where
    TR: Repository<Topic> + TopicRepository + Send + Sync,
    AR: Repository<Action> + ActionRepository + Send + Sync,
    SR: Repository<Spark> + SparkRepository + Send + Sync,
{
    /// Mostrar estado actual del workflow de un proyecto
    /// PROJECT → TOPIC → ACTION + SPARK (vista integrada)
    async fn handle_status(
        &self,
        context: &ExecutionContext,
        project_name: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        
        if let Some(proj_name) = project_name {
            let project_id = format!("{}_{}", user_id, proj_name.to_lowercase().replace(" ", "-"));
            
            match self.workflow_service.get_project_summary(&project_id).await {
                Ok(summary) => {
                    let output = format!(
                        "📊 WORKFLOW STATUS para PROJECT '{}'\n\
                         \n\
                         🔄 Arquitectura Secuencial:\n\
                         PROJECT → TOPIC → ACTION\n\
                         ^^^^^^^ → ^^^^^^ → ^^^^^^^\n\
                         \n\
                         📈 Resumen de Progreso:\n\
                         📁 Proyecto: {}\n\
                         📋 TOPICs: {} total\n\
                         ⚡ ACTIONs: {} total\n\
                         ✨ SPARKs relacionados: {} (servicio transversal)\n\
                         \n\
                         📊 Distribución de Estados:\n\
                         📋 TOPICs Activos: {}\n\
                         ⚡ ACTIONs En Progreso: {}\n\
                         ✅ ACTIONs Completadas: {}\n\
                         ✨ SPARKs Capturados: {}\n\
                         \n\
                         💡 Usa 'workflow summary {}' para detalles completos",
                        proj_name,
                        summary.project_name,
                        summary.total_topics,
                        summary.total_actions,
                        summary.total_sparks,
                        summary.active_topics,
                        summary.in_progress_actions,
                        summary.completed_actions,
                        summary.captured_sparks,
                        proj_name
                    );
                    ExecutionResult::success(output)
                }
                Err(e) => ExecutionResult::error(&format!("Error obteniendo status del workflow: {}", e)),
            }
        } else {
            ExecutionResult::error("Especifica un proyecto: 'workflow status --project <nombre>'")
        }
    }

    /// Generar resumen completo del workflow
    async fn handle_summary(
        &self,
        context: &ExecutionContext,
        project_name: String,
        include_sparks: Option<bool>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, project_name.to_lowercase().replace(" ", "-"));
        let show_sparks = include_sparks.unwrap_or(true);
        
        match self.workflow_service.generate_workflow_summary(&project_id, show_sparks).await {
            Ok(summary) => {
                let mut output = String::new();
                
                output.push_str(&format!(
                    "📋 WORKFLOW SUMMARY: {}\n\
                     \n\
                     🔄 Arquitectura Completa:\n\
                     PROJECT → TOPIC → ACTION + SPARK (transversal)\n\
                     \n\
                     📊 Métricas Generales:\n\
                     📁 Proyecto: {}\n\
                     📋 TOPICs: {}\n\
                     ⚡ ACTIONs: {}\n",
                    project_name,
                    summary.project_name,
                    summary.total_topics,
                    summary.total_actions
                ));

                if show_sparks {
                    output.push_str(&format!("✨ SPARKs: {}\n", summary.total_sparks));
                }

                output.push_str(&format!(
                    "\n\
                     📈 Progreso por Nivel:\n\
                     \n\
                     📋 TOPIC Level (PROJECT → TOPIC → ACTION):\n\
                     {}    🟢 Activos: {}\n\
                     {}    ✅ Completados: {}\n\
                     {}    📦 Archivados: {}\n\
                     \n\
                     ⚡ ACTION Level (PROJECT → TOPIC → ACTION):\n\
                     {}    🚀 En Progreso: {}\n\
                     {}    ✅ Completadas: {}\n\
                     {}    🚫 Bloqueadas: {}\n\
                     {}    ❌ Canceladas: {}\n",
                    "   ", summary.active_topics,
                    "   ", summary.completed_topics,
                    "   ", summary.archived_topics,
                    "   ", summary.in_progress_actions,
                    "   ", summary.completed_actions,
                    "   ", summary.blocked_actions,
                    "   ", summary.cancelled_actions
                ));

                if show_sparks {
                    output.push_str(&format!(
                        "\n\
                         ✨ SPARK Level (TRANSVERSAL):\n\
                         {}    💡 Capturados: {}\n\
                         {}    👁️ Revisados: {}\n\
                         {}    🎯 Aplicados: {}\n\
                         {}    📦 Archivados: {}\n",
                        "   ", summary.captured_sparks,
                        "   ", summary.reviewed_sparks,
                        "   ", summary.applied_sparks,
                        "   ", summary.archived_sparks
                    ));
                }

                output.push_str(&format!(
                    "\n\
                     🎯 Eficiencia del Workflow:\n\
                     {}    📊 Tasa de Completado TOPICs: {:.1}%\n\
                     {}    📊 Tasa de Completado ACTIONs: {:.1}%\n",
                    "   ", 
                    if summary.total_topics > 0 { 
                        (summary.completed_topics as f64 / summary.total_topics as f64) * 100.0 
                    } else { 0.0 },
                    "   ",
                    if summary.total_actions > 0 { 
                        (summary.completed_actions as f64 / summary.total_actions as f64) * 100.0 
                    } else { 0.0 }
                ));

                if show_sparks && summary.total_sparks > 0 {
                    output.push_str(&format!(
                        "{}    ✨ Tasa de Aplicación SPARKs: {:.1}%\n",
                        "   ",
                        (summary.applied_sparks as f64 / summary.total_sparks as f64) * 100.0
                    ));
                }

                output.push_str(&format!(
                    "\n\
                     💡 Próximos Pasos:\n\
                     • Ver progreso: 'workflow progress {}'\n\
                     • Ver timeline: 'workflow timeline {}'\n\
                     • Ver insights: 'workflow insights {}'",
                    project_name, project_name, project_name
                ));

                ExecutionResult::success(output)
            }
            Err(e) => ExecutionResult::error(&format!("Error generando resumen del workflow: {}", e)),
        }
    }

    /// Mostrar progreso del workflow con métricas de productividad
    async fn handle_progress(
        &self,
        context: &ExecutionContext,
        project_name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, project_name.to_lowercase().replace(" ", "-"));
        
        match self.workflow_service.calculate_progress_metrics(&project_id).await {
            Ok(metrics) => {
                let output = format!(
                    "📈 WORKFLOW PROGRESS: {}\n\
                     \n\
                     🔄 Flujo Secuencial:\n\
                     PROJECT → TOPIC → ACTION\n\
                     \n\
                     📊 Métricas de Productividad:\n\
                     \n\
                     📋 TOPIC Progress:\n\
                     {}    ▓▓▓▓▓▓▓▓▓▓ {:.1}% ({}/{})\n\
                     {}    🟢 En progreso: {}\n\
                     {}    ⏳ Pendientes: {}\n\
                     \n\
                     ⚡ ACTION Progress:\n\
                     {}    ▓▓▓▓▓▓▓▓▓▓ {:.1}% ({}/{})\n\
                     {}    🚀 En progreso: {}\n\
                     {}    🚫 Bloqueadas: {}\n\
                     {}    ⏳ Pendientes: {}\n\
                     \n\
                     ✨ SPARK Insights (Transversal):\n\
                     {}    💡 Total capturados: {}\n\
                     {}    🎯 Aplicados: {} ({:.1}%)\n\
                     {}    👁️ Pendientes de revisión: {}\n\
                     \n\
                     🎯 Recomendaciones:\n\
                     • Prioriza ACTIONs bloqueadas: {} pendientes\n\
                     • Revisa SPARKs capturados: {} sin revisar\n\
                     • Mantén el momentum: {:.1}% de progreso general",
                    project_name,
                    "   ", 
                    if metrics.total_topics > 0 { 
                        (metrics.completed_topics as f64 / metrics.total_topics as f64) * 100.0 
                    } else { 0.0 },
                    metrics.completed_topics, metrics.total_topics,
                    "   ", metrics.active_topics,
                    "   ", metrics.total_topics - metrics.completed_topics - metrics.active_topics,
                    "   ",
                    if metrics.total_actions > 0 { 
                        (metrics.completed_actions as f64 / metrics.total_actions as f64) * 100.0 
                    } else { 0.0 },
                    metrics.completed_actions, metrics.total_actions,
                    "   ", metrics.in_progress_actions,
                    "   ", metrics.blocked_actions,
                    "   ", metrics.total_actions - metrics.completed_actions - metrics.in_progress_actions - metrics.blocked_actions,
                    "   ", metrics.total_sparks,
                    "   ", metrics.applied_sparks,
                    if metrics.total_sparks > 0 { 
                        (metrics.applied_sparks as f64 / metrics.total_sparks as f64) * 100.0 
                    } else { 0.0 },
                    "   ", metrics.captured_sparks,
                    metrics.blocked_actions,
                    metrics.captured_sparks,
                    if metrics.total_topics + metrics.total_actions > 0 {
                        ((metrics.completed_topics + metrics.completed_actions) as f64 / (metrics.total_topics + metrics.total_actions) as f64) * 100.0
                    } else { 0.0 }
                );
                ExecutionResult::success(output)
            }
            Err(e) => ExecutionResult::error(&format!("Error calculando métricas de progreso: {}", e)),
        }
    }

    /// Mostrar timeline del workflow con actividad reciente
    async fn handle_timeline(
        &self,
        context: &ExecutionContext,
        project_name: String,
        days: Option<i32>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, project_name.to_lowercase().replace(" ", "-"));
        let time_range = days.unwrap_or(7);
        
        match self.workflow_service.get_activity_timeline(&project_id, time_range).await {
            Ok(activities) => {
                if activities.is_empty() {
                    ExecutionResult::success(format!(
                        "📅 No hay actividad registrada en los últimos {} días para PROJECT '{}'", 
                        time_range, project_name
                    ))
                } else {
                    let mut output = String::new();
                    output.push_str(&format!(
                        "📅 WORKFLOW TIMELINE: {} (últimos {} días)\n\
                         \n\
                         🔄 Actividad en PROJECT → TOPIC → ACTION:\n\
                         \n",
                        project_name, time_range
                    ));
                    
                    for activity in activities {
                        let icon = match activity.activity_type.as_str() {
                            "topic_created" => "📋 ➕",
                            "topic_completed" => "📋 ✅",
                            "action_created" => "⚡ ➕",
                            "action_started" => "⚡ 🚀",
                            "action_completed" => "⚡ ✅",
                            "action_blocked" => "⚡ 🚫",
                            "spark_captured" => "✨ 💡",
                            "spark_applied" => "✨ 🎯",
                            _ => "📝",
                        };
                        
                        output.push_str(&format!(
                            "   {} {} | {}\n\
                             {}    📅 {}\n\
                             {}    🆔 {}\n",
                            icon,
                            activity.description,
                            activity.entity_name,
                            "   ",
                            activity.timestamp.format("%Y-%m-%d %H:%M"),
                            "   ",
                            activity.entity_id
                        ));
                    }
                    
                    output.push_str(&format!(
                        "\n\
                         📊 Resumen de Actividad ({} días):\n\
                         {}    {} eventos registrados\n\
                         {}    Distribuidos en PROJECT → TOPIC → ACTION + SPARK\n\
                         \n\
                         💡 Usa 'workflow progress {}' para ver métricas de productividad",
                        time_range,
                        "   ", activities.len(),
                        "   ",
                        project_name
                    ));
                    
                    ExecutionResult::success(output)
                }
            }
            Err(e) => ExecutionResult::error(&format!("Error obteniendo timeline: {}", e)),
        }
    }

    /// Analizar insights capturados en SPARKs relacionados al workflow
    async fn handle_insights(
        &self,
        context: &ExecutionContext,
        project_name: String,
        context_filter: Option<String>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, project_name.to_lowercase().replace(" ", "-"));
        
        match self.workflow_service.analyze_workflow_insights(&project_id, context_filter.as_deref()).await {
            Ok(insights) => {
                if insights.is_empty() {
                    ExecutionResult::success(format!(
                        "✨ No hay SPARKs (insights) relacionados con PROJECT '{}'", 
                        project_name
                    ))
                } else {
                    let mut output = String::new();
                    output.push_str(&format!(
                        "✨ WORKFLOW INSIGHTS: {}\n\
                         \n\
                         🔄 SPARKs Transversales en PROJECT → TOPIC → ACTION:\n\
                         \n",
                        project_name
                    ));
                    
                    let mut by_context = std::collections::HashMap::new();
                    for insight in &insights {
                        by_context.entry(&insight.context).or_insert_with(Vec::new).push(insight);
                    }
                    
                    for (context, context_insights) in by_context {
                        let context_icon = match context.as_str() {
                            "Project" => "📁",
                            "Topic" => "📋", 
                            "Action" => "⚡",
                            _ => "🌟",
                        };
                        
                        output.push_str(&format!(
                            "   {} Contexto {} ({} insights):\n",
                            context_icon, context, context_insights.len()
                        ));
                        
                        for insight in context_insights {
                            let status_icon = match insight.status.as_str() {
                                "Captured" => "💡",
                                "Reviewed" => "👁️",
                                "Applied" => "🎯",
                                "Archived" => "📦",
                                _ => "✨",
                            };
                            
                            let preview = if insight.content.len() > 60 {
                                format!("{}...", &insight.content[..60])
                            } else {
                                insight.content.clone()
                            };
                            
                            output.push_str(&format!(
                                "   {}    {} {} | {}\n\
                                 {}    {}    📝 \"{}\"\n\
                                 {}    {}    📅 {}\n",
                                "   ", status_icon, insight.spark_id.split('_').last().unwrap_or(&insight.spark_id), 
                                insight.status,
                                "   ", "   ", preview,
                                "   ", "   ", insight.created_at.format("%Y-%m-%d %H:%M")
                            ));
                        }
                        output.push('\n');
                    }
                    
                    output.push_str(&format!(
                        "📊 Análisis de Insights:\n\
                         {}    ✨ Total capturados: {}\n\
                         {}    🎯 Aplicados: {} ({:.1}%)\n\
                         {}    👁️ Pendientes revisión: {}\n\
                         \n\
                         💡 Recomendación: Revisa y aplica los insights pendientes\n\
                         para maximizar el valor del flujo PROJECT → TOPIC → ACTION",
                        "   ", insights.len(),
                        "   ", 
                        insights.iter().filter(|i| i.status == "Applied").count(),
                        if !insights.is_empty() {
                            (insights.iter().filter(|i| i.status == "Applied").count() as f64 / insights.len() as f64) * 100.0
                        } else { 0.0 },
                        "   ",
                        insights.iter().filter(|i| i.status == "Captured").count()
                    ));
                    
                    ExecutionResult::success(output)
                }
            }
            Err(e) => ExecutionResult::error(&format!("Error analizando insights: {}", e)),
        }
    }
}

/// Subcomandos para gestión de WORKFLOWs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowSubcommand {
    /// Mostrar estado actual del workflow
    Status {
        project_name: Option<String>,
    },
    /// Generar resumen completo del workflow
    Summary {
        project_name: String,
        include_sparks: Option<bool>,
    },
    /// Mostrar progreso con métricas
    Progress {
        project_name: String,
    },
    /// Mostrar timeline de actividad
    Timeline {
        project_name: String,
        days: Option<i32>,
    },
    /// Analizar insights capturados
    Insights {
        project_name: String,
        context_filter: Option<String>,
    },
}
