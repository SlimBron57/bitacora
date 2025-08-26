//! Project Management Command Handlers
//! 
//! Siguiendo la arquitectura secuencial: PROJECT → TOPIC → ACTION
//! PROJECT es el contenedor de alto nivel para temas y acciones relacionadas

use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bitacora_core::models::{Project, ProjectStatus, Priority};
use bitacora_core::errors::BitacoraError;
use bitacora_storage::repository::{Repository, ProjectRepository};
use crate::executor::{CommandHandler, ExecutionContext, ExecutionResult};
use crate::parser::ParsedCommand;

/// Handler para comandos relacionados with PROJECT management
/// 
/// PROJECT → TOPIC → ACTION
/// ^^^^^^^ Este handler gestiona el nivel más alto de la secuencia
pub struct ProjectHandler<R>
where 
    R: Repository<Project> + ProjectRepository + Send + Sync,
{
    project_repo: Arc<R>,
}

impl<R> ProjectHandler<R>
where
    R: Repository<Project> + ProjectRepository + Send + Sync,
{
    pub fn new(project_repo: Arc<R>) -> Self {
        Self { project_repo }
    }
}

#[async_trait]
impl<R> CommandHandler for ProjectHandler<R>
where
    R: Repository<Project> + ProjectRepository + Send + Sync,
{
    fn command_name(&self) -> &'static str {
        "project"
    }

    fn description(&self) -> &'static str {
        "Gestiona proyectos de desarrollo (nivel superior en PROJECT → TOPIC → ACTION)"
    }

    async fn handle(&self, context: &ExecutionContext, command: &ParsedCommand) -> ExecutionResult {
        if command.command != "project" {
            return ExecutionResult::error("Command not supported by ProjectHandler");
        }
        
        let subcommand_str = command.subcommand.as_ref()
            .ok_or_else(|| ExecutionResult::error("No subcommand provided for project"))?;
            
        match subcommand_str.as_str() {
            "create" => {
                let name = command.args.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| "new-project".to_string());
                let description = command.args.get("description")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let priority = command.args.get("priority")
                    .and_then(|v| v.as_str())
                    .and_then(|s| match s {
                        "low" => Some(Priority::Low),
                        "medium" => Some(Priority::Medium),
                        "high" => Some(Priority::High),
                        "urgent" => Some(Priority::Urgent),
                        _ => None,
                    });
                self.handle_create(context, name, description, priority).await
            }
            "list" => {
                let status = command.args.get("status")
                    .and_then(|v| v.as_str())
                    .and_then(|s| match s {
                        "planning" => Some(ProjectStatus::Planning),
                        "active" => Some(ProjectStatus::Active),
                        "paused" => Some(ProjectStatus::Paused),
                        "completed" => Some(ProjectStatus::Completed),
                        "archived" => Some(ProjectStatus::Archived),
                        _ => None,
                    });
                self.handle_list(context, status).await
            }
            "show" => {
                let name = command.args.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| "default".to_string());
                self.handle_show(context, name).await
            }
            "activate" => {
                let name = command.args.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| "default".to_string());
                self.handle_activate(context, name).await
            }
            "complete" => {
                let name = command.args.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| "default".to_string());
                self.handle_complete(context, name).await
            }
            "archive" => {
                let name = command.args.get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_else(|| "default".to_string());
                self.handle_archive(context, name).await
            }
            _ => ExecutionResult::error(&format!("Unknown subcommand: {}", subcommand_str)),
        }
    }
}

impl<R> ProjectHandler<R>
where
    R: Repository<Project> + ProjectRepository + Send + Sync,
{
    /// Crear nuevo PROJECT 
    /// PROJECT → TOPIC → ACTION
    /// ^^^^^^^ Iniciamos la secuencia creando el contenedor de alto nivel
    async fn handle_create(
        &self,
        context: &ExecutionContext,
        name: String,
        description: Option<String>,
        priority: Option<Priority>,
    ) -> ExecutionResult {
        let user_id = context.user_id.clone();
        let desc = description.unwrap_or_else(|| format!("Proyecto {}", name));
        let prio = priority.unwrap_or(Priority::Medium);

        // Crear proyecto usando el modelo de dominio
        match Project::new(name.clone(), desc, user_id, prio) {
            Ok(project) => {
                match self.project_repo.create(&project).await {
                    Ok(_) => ExecutionResult::success(format!(
                        "✅ PROJECT '{}' creado exitosamente\n   📁 ID: {}\n   🎯 Listo para crear TOPICs dentro de este proyecto", 
                        name, project.project_id
                    )),
                    Err(e) => ExecutionResult::error(&format!("Error creando proyecto: {}", e)),
                }
            }
            Err(e) => ExecutionResult::error(&format!("Error en datos del proyecto: {}", e)),
        }
    }

    /// Listar PROJECTs por estado
    async fn handle_list(
        &self,
        context: &ExecutionContext,
        status_filter: Option<ProjectStatus>,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        
        match self.project_repo.get_by_user_id(user_id).await {
            Ok(projects) => {
                let filtered_projects: Vec<_> = if let Some(status) = status_filter {
                    projects.into_iter().filter(|p| p.status == status).collect()
                } else {
                    projects
                };

                if filtered_projects.is_empty() {
                    ExecutionResult::success("📂 No se encontraron proyectos".to_string())
                } else {
                    let mut output = String::new();
                    output.push_str("📂 PROYECTOS (PROJECT → TOPIC → ACTION):\n");
                    
                    for project in filtered_projects {
                        output.push_str(&format!(
                            "   📁 {} ({})\n      🎯 Prioridad: {:?} | Estado: {:?}\n      📅 Creado: {}\n",
                            project.name,
                            project.project_id,
                            project.priority,
                            project.status,
                            project.created_at.format("%Y-%m-%d %H:%M")
                        ));
                    }
                    output.push_str("\n💡 Usa 'topic list --project <nombre>' para ver TOPICs dentro de un proyecto");
                    
                    ExecutionResult::success(output)
                }
            }
            Err(e) => ExecutionResult::error(&format!("Error listando proyectos: {}", e)),
        }
    }

    /// Mostrar detalles de un PROJECT específico
    async fn handle_show(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.project_repo.get_by_id(&project_id).await {
            Ok(Some(project)) => {
                let output = format!(
                    "📁 PROJECT: {}\n\
                     🆔 ID: {}\n\
                     📝 Descripción: {}\n\
                     🎯 Prioridad: {:?}\n\
                     📊 Estado: {:?}\n\
                     📅 Creado: {}\n\
                     📅 Actualizado: {}\n\
                     \n\
                     🔄 Flujo de trabajo:\n\
                     PROJECT → TOPIC → ACTION\n\
                     ^^^^^^^ Estás aquí\n\
                     \n\
                     💡 Próximos pasos:\n\
                     • Crea TOPICs: 'topic create --project {}'\n\
                     • Ve TOPICs existentes: 'topic list --project {}'",
                    project.name,
                    project.project_id,
                    project.description,
                    project.priority,
                    project.status,
                    project.created_at.format("%Y-%m-%d %H:%M"),
                    project.updated_at.format("%Y-%m-%d %H:%M"),
                    name,
                    name
                );
                ExecutionResult::success(output)
            }
            Ok(None) => ExecutionResult::error(&format!("Proyecto '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error buscando proyecto: {}", e)),
        }
    }

    /// Activar PROJECT (cambiar estado a Active)
    async fn handle_activate(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.project_repo.update_project_status(&project_id, ProjectStatus::Active).await {
            Ok(true) => ExecutionResult::success(format!(
                "✅ PROJECT '{}' activado\n🔄 Ahora puedes trabajar en TOPICs dentro de este proyecto", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Proyecto '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error activando proyecto: {}", e)),
        }
    }

    /// Completar PROJECT (cambiar estado a Completed)  
    async fn handle_complete(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.project_repo.update_project_status(&project_id, ProjectStatus::Completed).await {
            Ok(true) => ExecutionResult::success(format!(
                "🎉 PROJECT '{}' completado exitosamente!\n📈 Todos los TOPICs y ACTIONs quedan registrados para análisis futuro", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Proyecto '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error completando proyecto: {}", e)),
        }
    }

    /// Archivar PROJECT (cambiar estado a Archived)
    async fn handle_archive(
        &self,
        context: &ExecutionContext,
        name: String,
    ) -> ExecutionResult {
        let user_id = &context.user_id;
        let project_id = format!("{}_{}", user_id, name.to_lowercase().replace(" ", "-"));
        
        match self.project_repo.update_project_status(&project_id, ProjectStatus::Archived).await {
            Ok(true) => ExecutionResult::success(format!(
                "📦 PROJECT '{}' archivado\n💾 TOPICs y ACTIONs conservados para referencia futura", 
                name
            )),
            Ok(false) => ExecutionResult::error(&format!("Proyecto '{}' no encontrado", name)),
            Err(e) => ExecutionResult::error(&format!("Error archivando proyecto: {}", e)),
        }
    }
}


