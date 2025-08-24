//! Action domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Type alias for action identifiers
pub type ActionId = Uuid;

/// Representa una acción específica realizada dentro de una sesión de trabajo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    /// Identificador único de la acción
    pub action_id: Uuid,
    /// ID de la sesión a la que pertenece esta acción
    pub session_id: Uuid,
    /// ID del usuario (redundante pero útil para queries)
    pub user_id: String,
    /// Momento exacto cuando se realizó la acción
    pub timestamp: DateTime<Utc>,
    /// Descripción detallada de lo que se hizo
    pub description: String,
    /// Tipo de acción realizada
    pub action_type: ActionType,
    /// Tags para categorización y búsqueda
    pub tags: Vec<String>,
    /// Duración estimada en minutos (opcional)
    pub duration_minutes: Option<u32>,
    /// Contexto adicional (branch, archivos, etc.)
    pub context: ActionContext,
    /// Metadata del template de respuesta (opcional)
    pub template_metadata: Option<TemplateMetadata>,
}

/// Tipos de acciones que se pueden realizar
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActionType {
    /// Commit de Git
    GitCommit,
    /// Edición de archivos
    FileEdit,
    /// Debugging y resolución de problemas
    Debug,
    /// Ejecución de tests
    Test,
    /// Build/compilación
    Build,
    /// Deploy de aplicación
    Deploy,
    /// Reuniones y comunicación
    Meeting,
    /// Investigación y aprendizaje
    Research,
    /// Documentación
    Documentation,
    /// Planificación
    Planning,
}

/// Contexto adicional de una acción
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionContext {
    /// Branch de git donde se realizó la acción
    pub git_branch: Option<String>,
    /// Hash del commit relacionado
    pub git_commit_hash: Option<String>,
    /// Archivos afectados por la acción
    pub files_affected: Vec<String>,
    /// Información adicional específica del contexto
    pub additional_info: HashMap<String, String>,
}

/// Metadata ligera del template de respuesta
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateMetadata {
    /// ID del template usado
    pub template_id: String,
    /// Versión del template
    pub template_version: String,
    /// Variables usadas para renderizar
    pub variables: HashMap<String, serde_json::Value>,
    /// Si fue auto-detectado o seleccionado manualmente
    pub auto_detected: bool,
    /// Timestamp de cuando se aplicó el template
    pub applied_at: DateTime<Utc>,
}

impl ActionContext {
    pub fn default() -> Self {
        Self {
            git_branch: None,
            git_commit_hash: None,
            files_affected: Vec::new(),
            additional_info: HashMap::new(),
        }
    }
    
    pub fn git_context(branch: String, commit_hash: Option<String>) -> Self {
        Self {
            git_branch: Some(branch),
            git_commit_hash: commit_hash,
            files_affected: Vec::new(),
            additional_info: HashMap::new(),
        }
    }
}

impl Action {
    /// Crear nueva acción
    pub fn new(session_id: Uuid, user_id: String, description: String, action_type: ActionType) -> Self {
        Self {
            action_id: Uuid::new_v4(),
            session_id,
            user_id,
            timestamp: Utc::now(),
            description,
            action_type,
            tags: Vec::new(),
            duration_minutes: None,
            context: ActionContext::default(),
            template_metadata: None,
        }
    }

    /// Agregar tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    /// Establecer duración
    pub fn with_duration(mut self, minutes: u32) -> Self {
        self.duration_minutes = Some(minutes);
        self
    }

    /// Establecer contexto
    pub fn with_context(mut self, context: ActionContext) -> Self {
        self.context = context;
        self
    }

    /// Establecer template metadata
    pub fn with_template(mut self, template_metadata: TemplateMetadata) -> Self {
        self.template_metadata = Some(template_metadata);
        self
    }

    /// Verificar si tiene un tag específico
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Obtener la antigüedad de la acción
    pub fn age(&self) -> chrono::Duration {
        Utc::now() - self.timestamp
    }

    /// Verificar si la acción es reciente (menos de 1 hora)
    pub fn is_recent(&self) -> bool {
        self.age() < chrono::Duration::hours(1)
    }

    /// Verificar si tiene contexto de Git
    pub fn has_git_context(&self) -> bool {
        self.context.git_branch.is_some() || self.context.git_commit_hash.is_some()
    }
}

impl ActionType {
    /// Obtener todos los tipos disponibles
    pub fn all() -> Vec<ActionType> {
        vec![
            ActionType::GitCommit,
            ActionType::FileEdit,
            ActionType::Debug,
            ActionType::Test,
            ActionType::Build,
            ActionType::Deploy,
            ActionType::Meeting,
            ActionType::Research,
            ActionType::Documentation,
            ActionType::Planning,
        ]
    }

    /// Convertir a string
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::GitCommit => "git_commit",
            ActionType::FileEdit => "file_edit",
            ActionType::Debug => "debug",
            ActionType::Test => "test",
            ActionType::Build => "build",
            ActionType::Deploy => "deploy",
            ActionType::Meeting => "meeting",
            ActionType::Research => "research",
            ActionType::Documentation => "documentation",
            ActionType::Planning => "planning",
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Option<ActionType> {
        match s.to_lowercase().as_str() {
            "git_commit" | "commit" | "git" => Some(ActionType::GitCommit),
            "file_edit" | "edit" | "file" => Some(ActionType::FileEdit),
            "debug" | "debugging" => Some(ActionType::Debug),
            "test" | "testing" => Some(ActionType::Test),
            "build" | "compile" => Some(ActionType::Build),
            "deploy" | "deployment" => Some(ActionType::Deploy),
            "meeting" | "meet" => Some(ActionType::Meeting),
            "research" | "learning" => Some(ActionType::Research),
            "documentation" | "docs" | "doc" => Some(ActionType::Documentation),
            "planning" | "plan" => Some(ActionType::Planning),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_creation() {
        let action = Action::new(
            Uuid::new_v4(),
            "user123".to_string(),
            "Fixed bug in parser".to_string(),
            ActionType::Debug,
        );
        
        assert_eq!(action.user_id, "user123");
        assert_eq!(action.description, "Fixed bug in parser");
        assert_eq!(action.action_type, ActionType::Debug);
        assert_eq!(action.tags.len(), 0);
    }

    #[test]
    fn test_action_type_conversion() {
        assert_eq!(ActionType::GitCommit.as_str(), "git_commit");
        assert_eq!(ActionType::from_str("git_commit"), Some(ActionType::GitCommit));
        assert_eq!(ActionType::from_str("commit"), Some(ActionType::GitCommit));
        assert_eq!(ActionType::from_str("invalid"), None);
    }

    #[test]
    fn test_action_with_tags() {
        let action = Action::new(
            Uuid::new_v4(),
            "user".to_string(),
            "Test".to_string(),
            ActionType::Test,
        ).with_tags(vec!["urgent".to_string(), "bugfix".to_string()]);
        
        assert!(action.has_tag("urgent"));
        assert!(action.has_tag("URGENT")); // Case insensitive
        assert!(!action.has_tag("feature"));
    }

    #[test]
    fn test_action_with_context() {
        let context = ActionContext::git_context(
            "feature/new-parser".to_string(),
            Some("abc123".to_string()),
        );
        
        let action = Action::new(
            Uuid::new_v4(),
            "user".to_string(),
            "Updated parser".to_string(),
            ActionType::GitCommit,
        ).with_context(context);
        
        assert!(action.has_git_context());
        assert_eq!(action.context.git_branch, Some("feature/new-parser".to_string()));
        assert_eq!(action.context.git_commit_hash, Some("abc123".to_string()));
    }
}
