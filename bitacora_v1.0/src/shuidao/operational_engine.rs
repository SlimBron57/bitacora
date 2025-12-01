//! # Operational Project Engine
//!
//! Motor para gestión de proyectos operacionales (hacer cosas).
//!
//! ## Responsabilidades
//!
//! 1. **CreateProject**: Usuario necesita hacer algo → Crea proyecto con subtareas
//! 2. **ExecuteTask**: Marca tareas completadas, actualiza progreso
//! 3. **TrackProgress**: Muestra estado actual, recomienda próximo paso
//! 4. **Persistence**: Guarda proyectos en TelescopeDB
//!
//! ## Modo Operacional
//!
//! Cuando el usuario dice:
//! - "necesito instalar un switch"
//! - "quiero configurar Kubernetes"
//! - "tengo que hacer un backup"
//!
//! ShuiDao detecta intención **Operational** y activa este engine.
//!
//! ## Jerarquía
//!
//! ```text
//! OperationalProject
//!   ├─ SubProject 1
//!   │  ├─ Task 1.1 (Done)
//!   │  ├─ Task 1.2 (InProgress) ← current_focus
//!   │  └─ Task 1.3 (Todo)
//!   └─ SubProject 2
//!      ├─ Task 2.1 (Todo)
//!      └─ Task 2.2 (Todo)
//! ```
//!
//! **Performance Target**: <180ms engine processing
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 2 Days 1-2)

use crate::shuidao::error::{Result, ShuiDaoError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use uuid::Uuid;

// ============================================================================
// STRUCTURES
// ============================================================================

/// Operational Project Engine
///
/// Gestiona proyectos con jerarquía de subtareas
pub struct OperationalProjectEngine {
    /// Proyectos activos (project_id → OperationalProject)
    projects: Arc<RwLock<HashMap<String, OperationalProject>>>,
}

/// Proyecto operacional con tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalProject {
    pub id: String,
    pub name: String,
    pub category: ProjectCategory,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: ProjectStatus,
    pub completion: f32, // 0.0-1.0

    // Hierarchy
    pub sub_projects: Vec<SubProject>,
    pub current_focus: Option<String>, // ID de SubProject activo

    // Metadata
    pub estimated_hours: Option<f32>,
    pub actual_hours: Option<f32>,
    pub tags: Vec<String>,
}

/// SubProject (fase del proyecto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubProject {
    pub id: String,
    pub name: String,
    pub status: ProjectStatus,
    pub completion: f32,
    pub tasks: Vec<Task>,
    pub dependencies: Vec<String>, // IDs de otros SubProjects
}

/// Task individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Estado del proyecto
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    Active,
    Blocked,
    Completed,
    Cancelled,
}

/// Estado de tarea
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Skipped,
}

/// Categoría de proyecto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectCategory {
    Infrastructure,  // Networking, servers
    Software,        // Development projects
    Learning,        // Study projects
    Personal,        // Home projects
    Other(String),
}

/// Response del engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationalResponse {
    pub project_id: String,
    pub action: OperationalAction,
    pub next_steps: Vec<ActionRecommendation>,
    pub progress_summary: String,
    pub processing_time_ms: f64,
}

/// Acción ejecutada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationalAction {
    ProjectCreated { name: String, sub_projects: usize },
    ProjectUpdated { completion: f32, tasks_done: usize },
    TaskCompleted { task: String, remaining: usize },
    ProjectCompleted { name: String, total_tasks: usize },
}

/// Recomendación de acción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRecommendation {
    pub priority: Priority,
    pub description: String,
    pub estimated_time: Option<String>,
}

/// Prioridad
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl OperationalProjectEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create new operational project
    ///
    /// **Algorithm**:
    /// 1. Parse user input to extract project name and tasks
    /// 2. Create project structure with SubProjects
    /// 3. Generate initial tasks
    /// 4. Set current_focus to first SubProject
    /// 5. Return response with next steps
    pub fn create_project(&self, input: &str) -> Result<OperationalResponse> {
        let start = Instant::now();

        // Extract project info (simple heuristic for now)
        let project_name = self.extract_project_name(input);
        let category = self.infer_category(input);
        let sub_projects = self.generate_sub_projects(input, &category);

        // Create project
        let project = OperationalProject {
            id: Uuid::new_v4().to_string(),
            name: project_name.clone(),
            category: category.clone(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: ProjectStatus::Planning,
            completion: 0.0,
            sub_projects: sub_projects.clone(),
            current_focus: sub_projects.first().map(|sp| sp.id.clone()),
            estimated_hours: None,
            actual_hours: None,
            tags: Vec::new(),
        };

        // Store project
        let project_id = project.id.clone();
        {
            let mut projects = self.projects.write().unwrap();
            projects.insert(project_id.clone(), project.clone());
        }

        // Generate next steps
        let next_steps = self.generate_initial_recommendations(&project);
        let processing_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(OperationalResponse {
            project_id: project_id.clone(),
            action: OperationalAction::ProjectCreated {
                name: project_name,
                sub_projects: sub_projects.len(),
            },
            next_steps,
            progress_summary: format!(
                "Proyecto creado con {} sub-proyectos y {} tareas totales",
                sub_projects.len(),
                sub_projects.iter().map(|sp| sp.tasks.len()).sum::<usize>()
            ),
            processing_time_ms,
        })
    }

    /// Complete a task
    pub fn complete_task(&self, project_id: &str, task_id: &str) -> Result<OperationalResponse> {
        let start = Instant::now();

        let mut projects = self.projects.write().unwrap();
        let project = projects
            .get_mut(project_id)
            .ok_or_else(|| ShuiDaoError::OperationalEngineError {
                operation: "complete_task".to_string(),
                details: format!("Project not found: {}", project_id),
            })?;

        // Find and complete task
        let mut task_found = false;
        let mut task_description = String::new();
        let mut remaining_tasks = 0;

        for sub_project in &mut project.sub_projects {
            for task in &mut sub_project.tasks {
                if task.id == task_id {
                    task.status = TaskStatus::Done;
                    task.completed_at = Some(Utc::now());
                    task_description = task.description.clone();
                    task_found = true;
                } else if task.status != TaskStatus::Done {
                    remaining_tasks += 1;
                }
            }

            // Update sub_project completion
            let total_tasks = sub_project.tasks.len();
            let completed_tasks = sub_project
                .tasks
                .iter()
                .filter(|t| t.status == TaskStatus::Done)
                .count();
            sub_project.completion = completed_tasks as f32 / total_tasks as f32;

            if sub_project.completion == 1.0 {
                sub_project.status = ProjectStatus::Completed;
            }
        }

        if !task_found {
            return Err(ShuiDaoError::OperationalEngineError {
                operation: "complete_task".to_string(),
                details: format!("Task not found: {}", task_id),
            });
        }

        // Update project completion
        let total_tasks: usize = project.sub_projects.iter().map(|sp| sp.tasks.len()).sum();
        let completed_tasks: usize = project
            .sub_projects
            .iter()
            .flat_map(|sp| &sp.tasks)
            .filter(|t| t.status == TaskStatus::Done)
            .count();
        project.completion = completed_tasks as f32 / total_tasks as f32;
        project.last_updated = Utc::now();

        if project.completion == 1.0 {
            project.status = ProjectStatus::Completed;
        }

        let next_steps = self.generate_progress_recommendations(project);
        let processing_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        let action = if project.status == ProjectStatus::Completed {
            OperationalAction::ProjectCompleted {
                name: project.name.clone(),
                total_tasks,
            }
        } else {
            OperationalAction::TaskCompleted {
                task: task_description,
                remaining: remaining_tasks,
            }
        };

        Ok(OperationalResponse {
            project_id: project_id.to_string(),
            action,
            next_steps,
            progress_summary: format!(
                "Progreso: {}/{} tareas completadas ({:.0}%)",
                completed_tasks,
                total_tasks,
                project.completion * 100.0
            ),
            processing_time_ms,
        })
    }

    /// Get project status
    pub fn get_project(&self, project_id: &str) -> Result<OperationalProject> {
        let projects = self.projects.read().unwrap();
        projects
            .get(project_id)
            .cloned()
            .ok_or_else(|| ShuiDaoError::OperationalEngineError {
                operation: "get_project".to_string(),
                details: format!("Project not found: {}", project_id),
            })
    }

    /// List all projects
    pub fn list_projects(&self) -> Vec<OperationalProject> {
        let projects = self.projects.read().unwrap();
        projects.values().cloned().collect()
    }

    // ========================================
    // PRIVATE HELPERS
    // ========================================

    fn extract_project_name(&self, input: &str) -> String {
        // Simple extraction: remove trigger words
        input
            .replace("necesito", "")
            .replace("quiero", "")
            .replace("tengo que", "")
            .replace("voy a", "")
            .trim()
            .to_string()
    }

    fn infer_category(&self, input: &str) -> ProjectCategory {
        let lower = input.to_lowercase();
        if lower.contains("switch")
            || lower.contains("router")
            || lower.contains("network")
            || lower.contains("server")
        {
            ProjectCategory::Infrastructure
        } else if lower.contains("code")
            || lower.contains("software")
            || lower.contains("app")
            || lower.contains("kubernetes")
        {
            ProjectCategory::Software
        } else if lower.contains("aprender") || lower.contains("estudiar") {
            ProjectCategory::Learning
        } else {
            ProjectCategory::Other("General".to_string())
        }
    }

    fn generate_sub_projects(&self, input: &str, category: &ProjectCategory) -> Vec<SubProject> {
        // Generate generic sub-projects based on category
        match category {
            ProjectCategory::Infrastructure => vec![
                SubProject {
                    id: Uuid::new_v4().to_string(),
                    name: "Preparación".to_string(),
                    status: ProjectStatus::Planning,
                    completion: 0.0,
                    tasks: vec![
                        Task {
                            id: Uuid::new_v4().to_string(),
                            description: "Revisar documentación".to_string(),
                            status: TaskStatus::Todo,
                            created_at: Utc::now(),
                            completed_at: None,
                            notes: None,
                        },
                        Task {
                            id: Uuid::new_v4().to_string(),
                            description: "Verificar requisitos".to_string(),
                            status: TaskStatus::Todo,
                            created_at: Utc::now(),
                            completed_at: None,
                            notes: None,
                        },
                    ],
                    dependencies: Vec::new(),
                },
                SubProject {
                    id: Uuid::new_v4().to_string(),
                    name: "Ejecución".to_string(),
                    status: ProjectStatus::Planning,
                    completion: 0.0,
                    tasks: vec![
                        Task {
                            id: Uuid::new_v4().to_string(),
                            description: "Instalar componentes".to_string(),
                            status: TaskStatus::Todo,
                            created_at: Utc::now(),
                            completed_at: None,
                            notes: None,
                        },
                        Task {
                            id: Uuid::new_v4().to_string(),
                            description: "Configurar sistema".to_string(),
                            status: TaskStatus::Todo,
                            created_at: Utc::now(),
                            completed_at: None,
                            notes: None,
                        },
                    ],
                    dependencies: Vec::new(),
                },
                SubProject {
                    id: Uuid::new_v4().to_string(),
                    name: "Validación".to_string(),
                    status: ProjectStatus::Planning,
                    completion: 0.0,
                    tasks: vec![Task {
                        id: Uuid::new_v4().to_string(),
                        description: "Verificar funcionamiento".to_string(),
                        status: TaskStatus::Todo,
                        created_at: Utc::now(),
                        completed_at: None,
                        notes: None,
                    }],
                    dependencies: Vec::new(),
                },
            ],
            _ => vec![SubProject {
                id: Uuid::new_v4().to_string(),
                name: "Tareas principales".to_string(),
                status: ProjectStatus::Planning,
                completion: 0.0,
                tasks: vec![
                    Task {
                        id: Uuid::new_v4().to_string(),
                        description: format!("Iniciar: {}", input),
                        status: TaskStatus::Todo,
                        created_at: Utc::now(),
                        completed_at: None,
                        notes: None,
                    },
                    Task {
                        id: Uuid::new_v4().to_string(),
                        description: "Completar tarea".to_string(),
                        status: TaskStatus::Todo,
                        created_at: Utc::now(),
                        completed_at: None,
                        notes: None,
                    },
                ],
                dependencies: Vec::new(),
            }],
        }
    }

    fn generate_initial_recommendations(
        &self,
        project: &OperationalProject,
    ) -> Vec<ActionRecommendation> {
        let mut recommendations = Vec::new();

        if let Some(first_sub) = project.sub_projects.first() {
            if let Some(first_task) = first_sub.tasks.first() {
                recommendations.push(ActionRecommendation {
                    priority: Priority::High,
                    description: format!("Comenzar con: {}", first_task.description),
                    estimated_time: Some("30 min".to_string()),
                });
            }
        }

        recommendations.push(ActionRecommendation {
            priority: Priority::Medium,
            description: "Revisar sub-proyectos y ajustar plan si es necesario".to_string(),
            estimated_time: Some("15 min".to_string()),
        });

        recommendations
    }

    fn generate_progress_recommendations(
        &self,
        project: &OperationalProject,
    ) -> Vec<ActionRecommendation> {
        let mut recommendations = Vec::new();

        // Find next task
        for sub_project in &project.sub_projects {
            if let Some(next_task) = sub_project
                .tasks
                .iter()
                .find(|t| t.status == TaskStatus::Todo)
            {
                recommendations.push(ActionRecommendation {
                    priority: Priority::High,
                    description: format!("Próxima tarea: {}", next_task.description),
                    estimated_time: Some("30 min".to_string()),
                });
                break;
            }
        }

        if recommendations.is_empty() && project.status != ProjectStatus::Completed {
            recommendations.push(ActionRecommendation {
                priority: Priority::Medium,
                description: "Revisar estado del proyecto".to_string(),
                estimated_time: Some("10 min".to_string()),
            });
        }

        recommendations
    }
}

impl Default for OperationalProjectEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = OperationalProjectEngine::new();
        assert_eq!(engine.list_projects().len(), 0);
    }

    #[test]
    fn test_create_project_infrastructure() {
        let engine = OperationalProjectEngine::new();
        let response = engine
            .create_project("necesito instalar un switch de networking")
            .unwrap();

        assert!(matches!(
            response.action,
            OperationalAction::ProjectCreated { .. }
        ));
        assert_eq!(engine.list_projects().len(), 1);

        let project = engine.get_project(&response.project_id).unwrap();
        assert_eq!(project.sub_projects.len(), 3); // Preparación, Ejecución, Validación
        assert!(matches!(project.category, ProjectCategory::Infrastructure));
    }

    #[test]
    fn test_complete_task() {
        let engine = OperationalProjectEngine::new();
        let create_response = engine.create_project("instalar switch").unwrap();
        let project = engine.get_project(&create_response.project_id).unwrap();

        let task_id = project.sub_projects[0].tasks[0].id.clone();

        let complete_response = engine
            .complete_task(&create_response.project_id, &task_id)
            .unwrap();

        assert!(matches!(
            complete_response.action,
            OperationalAction::TaskCompleted { .. }
        ));

        let updated_project = engine.get_project(&create_response.project_id).unwrap();
        assert!(updated_project.completion > 0.0);
    }

    #[test]
    fn test_project_completion() {
        let engine = OperationalProjectEngine::new();
        let create_response = engine.create_project("instalar switch").unwrap();
        let mut project = engine.get_project(&create_response.project_id).unwrap();

        // Complete all tasks
        for sub_project in &project.sub_projects {
            for task in &sub_project.tasks {
                let _ = engine.complete_task(&create_response.project_id, &task.id);
            }
        }

        project = engine.get_project(&create_response.project_id).unwrap();
        assert_eq!(project.completion, 1.0);
        assert_eq!(project.status, ProjectStatus::Completed);
    }

    #[test]
    fn test_performance_target() {
        let engine = OperationalProjectEngine::new();
        let start = Instant::now();
        let response = engine.create_project("configurar kubernetes").unwrap();
        let elapsed = start.elapsed().as_secs_f64() * 1000.0;

        // Target: <180ms
        assert!(
            elapsed < 180.0,
            "Engine processing took {:.2}ms (target: <180ms)",
            elapsed
        );
        assert!(
            response.processing_time_ms < 180.0,
            "Reported time: {:.2}ms",
            response.processing_time_ms
        );
    }

    #[test]
    fn test_category_inference() {
        let engine = OperationalProjectEngine::new();

        let infra_response = engine.create_project("instalar router").unwrap();
        let infra_project = engine.get_project(&infra_response.project_id).unwrap();
        assert!(matches!(
            infra_project.category,
            ProjectCategory::Infrastructure
        ));

        let software_response = engine.create_project("desarrollar app").unwrap();
        let software_project = engine.get_project(&software_response.project_id).unwrap();
        assert!(matches!(
            software_project.category,
            ProjectCategory::Software
        ));
    }

    #[test]
    fn test_task_not_found() {
        let engine = OperationalProjectEngine::new();
        let response = engine.create_project("test project").unwrap();

        let result = engine.complete_task(&response.project_id, "invalid-task-id");
        assert!(result.is_err());
    }

    #[test]
    fn test_project_not_found() {
        let engine = OperationalProjectEngine::new();
        let result = engine.get_project("invalid-project-id");
        assert!(result.is_err());
    }
}
