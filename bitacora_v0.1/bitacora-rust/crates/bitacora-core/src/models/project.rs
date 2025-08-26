//! Project domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa un proyecto de desarrollo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    /// Identificador único del proyecto
    pub project_id: Uuid,
    /// ID del usuario propietario
    pub user_id: String,
    /// Nombre del proyecto
    pub name: String,
    /// Descripción detallada (opcional)
    pub description: Option<String>,
    /// URL del repositorio git
    pub repository_url: Option<String>,
    /// Estado actual del proyecto
    pub status: ProjectStatus,
    /// Stack tecnológico utilizado
    pub tech_stack: Vec<String>,
    /// Tags para categorización
    pub tags: Vec<String>,
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
    /// Fecha de finalización (si aplica)
    pub completed_at: Option<DateTime<Utc>>,
}

/// Estado del proyecto
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    /// En fase de planificación
    Planning,
    /// Desarrollo activo
    Active,
    /// Proyecto completado
    Completed,
    /// En pausa temporal
    OnHold,
    /// Archivado (inactivo permanente)
    Archived,
    /// Cancelado
    Cancelled,
}

impl Project {
    /// Crear nuevo proyecto
    pub fn new(user_id: String, name: String) -> Self {
        let now = Utc::now();
        Self {
            project_id: Uuid::new_v4(),
            user_id,
            name,
            description: None,
            repository_url: None,
            status: ProjectStatus::Planning,
            tech_stack: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            completed_at: None,
        }
    }

    /// Establecer descripción
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.touch();
        self
    }

    /// Establecer repositorio
    pub fn with_repository(mut self, url: String) -> Self {
        self.repository_url = Some(url);
        self.touch();
        self
    }

    /// Establecer stack tecnológico
    pub fn with_tech_stack(mut self, stack: Vec<String>) -> Self {
        self.tech_stack = stack;
        self.touch();
        self
    }

    /// Agregar tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.touch();
        self
    }

    /// Cambiar estado del proyecto
    pub fn set_status(&mut self, status: ProjectStatus) {
        if matches!(status, ProjectStatus::Completed) {
            self.completed_at = Some(Utc::now());
        }
        self.status = status;
        self.touch();
    }

    /// Marcar proyecto como activo
    pub fn start(&mut self) {
        self.set_status(ProjectStatus::Active);
    }

    /// Completar proyecto
    pub fn complete(&mut self) {
        self.set_status(ProjectStatus::Completed);
    }

    /// Poner en pausa
    pub fn pause(&mut self) {
        self.set_status(ProjectStatus::OnHold);
    }

    /// Archivar proyecto
    pub fn archive(&mut self) {
        self.set_status(ProjectStatus::Archived);
    }

    /// Verificar si está activo
    pub fn is_active(&self) -> bool {
        matches!(self.status, ProjectStatus::Active)
    }

    /// Verificar si está completado
    pub fn is_completed(&self) -> bool {
        matches!(self.status, ProjectStatus::Completed)
    }

    /// Obtener duración del proyecto
    pub fn duration(&self) -> chrono::Duration {
        let end_time = self.completed_at.unwrap_or_else(Utc::now);
        end_time - self.created_at
    }

    /// Verificar si el proyecto tiene una tecnología específica
    pub fn has_technology(&self, tech: &str) -> bool {
        self.tech_stack.iter().any(|t| t.eq_ignore_ascii_case(tech))
    }

    /// Verificar si el proyecto tiene un tag específico
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Actualizar timestamp de modificación
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl ProjectStatus {
    /// Obtener todas las variantes
    pub fn all() -> Vec<ProjectStatus> {
        vec![
            ProjectStatus::Planning,
            ProjectStatus::Active,
            ProjectStatus::Completed,
            ProjectStatus::OnHold,
            ProjectStatus::Archived,
            ProjectStatus::Cancelled,
        ]
    }

    /// Convertir a string
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Planning => "planning",
            ProjectStatus::Active => "active",
            ProjectStatus::Completed => "completed",
            ProjectStatus::OnHold => "on_hold",
            ProjectStatus::Archived => "archived",
            ProjectStatus::Cancelled => "cancelled",
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Option<ProjectStatus> {
        match s.to_lowercase().as_str() {
            "planning" | "plan" => Some(ProjectStatus::Planning),
            "active" | "in_progress" => Some(ProjectStatus::Active),
            "completed" | "done" | "finished" => Some(ProjectStatus::Completed),
            "on_hold" | "paused" | "hold" => Some(ProjectStatus::OnHold),
            "archived" | "archive" => Some(ProjectStatus::Archived),
            "cancelled" | "canceled" => Some(ProjectStatus::Cancelled),
            _ => None,
        }
    }

    /// Verificar si es un estado final
    pub fn is_final(&self) -> bool {
        matches!(self, ProjectStatus::Completed | ProjectStatus::Archived | ProjectStatus::Cancelled)
    }

    /// Verificar si es un estado activo
    pub fn is_active_state(&self) -> bool {
        matches!(self, ProjectStatus::Active | ProjectStatus::Planning)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("user123".to_string(), "My Awesome Project".to_string());
        
        assert_eq!(project.user_id, "user123");
        assert_eq!(project.name, "My Awesome Project");
        assert_eq!(project.status, ProjectStatus::Planning);
        assert!(!project.is_active());
        assert!(!project.is_completed());
    }

    #[test]
    fn test_project_lifecycle() {
        let mut project = Project::new("user".to_string(), "Test Project".to_string());
        
        // Start project
        project.start();
        assert!(project.is_active());
        assert_eq!(project.status, ProjectStatus::Active);
        
        // Complete project
        project.complete();
        assert!(project.is_completed());
        assert!(project.completed_at.is_some());
    }

    #[test]
    fn test_project_with_tech_stack() {
        let project = Project::new("user".to_string(), "Rust Project".to_string())
            .with_tech_stack(vec!["Rust".to_string(), "Tokio".to_string(), "MongoDB".to_string()]);
        
        assert!(project.has_technology("rust"));
        assert!(project.has_technology("TOKIO"));
        assert!(!project.has_technology("Python"));
    }

    #[test]
    fn test_project_status_conversion() {
        assert_eq!(ProjectStatus::Active.as_str(), "active");
        assert_eq!(ProjectStatus::from_str("active"), Some(ProjectStatus::Active));
        assert_eq!(ProjectStatus::from_str("done"), Some(ProjectStatus::Completed));
        assert!(ProjectStatus::Completed.is_final());
        assert!(ProjectStatus::Active.is_active_state());
    }
}
