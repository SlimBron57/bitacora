//! Template domain models

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Template completo con estructura anidada JSON
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    /// Identificador único del template
    pub template_id: String,
    /// Nombre descriptivo del template
    pub name: String,
    /// Categoría del template
    pub category: TemplateCategory,
    /// Triggers que activan este template
    pub triggers: Vec<TemplateTrigger>,
    /// Estructura del template con secciones
    pub structure: TemplateStructure,
    /// Metadata del template
    pub metadata: TemplateMetadata,
    /// Variables requeridas para renderizar
    pub required_variables: Vec<String>,
    /// Variables opcionales con valores por defecto
    pub optional_variables: HashMap<String, serde_json::Value>,
}

/// Categorías de templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemplateCategory {
    /// Progress reports y milestone completions
    DevelopmentProgress,
    /// Debug session summaries
    DebuggingSession,
    /// Code review feedback
    CodeReview,
    /// Planning session results
    PlanningSession,
    /// Deployment reports
    DeploymentReport,
    /// Meeting summaries
    MeetingSummary,
    /// Research findings
    ResearchFindings,
    /// Error/issue reports
    ErrorReport,
    /// General action summary
    General,
}

/// Triggers que determinan cuándo usar un template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateTrigger {
    /// Tipo de trigger
    pub trigger_type: TriggerType,
    /// Condiciones específicas
    pub conditions: HashMap<String, serde_json::Value>,
    /// Prioridad del trigger (1 = highest)
    pub priority: u8,
}

/// Tipos de triggers para auto-detección
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TriggerType {
    /// Basado en tipo de acción
    ActionType,
    /// Basado en tags del action
    ActionTags,
    /// Basado en contexto (git branch, files, etc.)
    ActionContext,
    /// Basado en cantidad de actions relacionadas
    ActionSequence,
    /// Basado en tiempo transcurrido
    TimeBasedMilestone,
    /// Trigger manual por usuario
    ManualSelection,
}

/// Estructura anidada del template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateStructure {
    /// Secciones del template en orden
    pub sections: Vec<TemplateSection>,
    /// Separadores entre secciones
    pub section_separator: Option<String>,
    /// Footer opcional
    pub footer: Option<String>,
}

/// Sección individual del template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateSection {
    /// ID único de la sección
    pub section_id: String,
    /// Tipo de sección
    pub section_type: SectionType,
    /// Template de la sección
    pub template: String,
    /// Variables requeridas para esta sección
    pub required_vars: Vec<String>,
    /// Condiciones para mostrar esta sección
    pub conditions: Option<HashMap<String, serde_json::Value>>,
    /// Formato para listas (si aplica)
    pub list_format: Option<String>,
    /// Styling/formatting options
    pub formatting: Option<SectionFormatting>,
}

/// Tipos de secciones en templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SectionType {
    /// Header con status y título
    Header,
    /// Lista de logros/achievements
    Achievements,
    /// Métricas y estadísticas
    Metrics,
    /// Highlight técnico o arquitectural
    TechnicalHighlight,
    /// Pasos siguientes y recomendaciones
    NextSteps,
    /// Prompt inteligente para continuar
    IntelligentPrompt,
    /// Texto libre/narrativo
    FreeText,
    /// Lista genérica
    List,
    /// Tabla de datos
    Table,
    /// Code snippet
    CodeBlock,
}

/// Opciones de formato para secciones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SectionFormatting {
    /// Indentación
    pub indent_level: Option<u8>,
    /// Usar emojis
    pub use_emojis: bool,
    /// Usar markdown formatting
    pub use_markdown: bool,
    /// Máximo número de items en listas
    pub max_list_items: Option<usize>,
    /// Truncar texto largo
    pub max_text_length: Option<usize>,
}

/// Metadata del template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateMetadata {
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Última actualización
    pub updated_at: DateTime<Utc>,
    /// Versión del template
    pub version: String,
    /// Autor/creador
    pub author: Option<String>,
    /// Número de veces usado
    pub usage_count: u64,
    /// Score de efectividad (0-10)
    pub effectiveness_score: Option<f32>,
    /// Tags para búsqueda
    pub tags: Vec<String>,
    /// Descripción del template
    pub description: Option<String>,
}

/// Metadata ligera para almacenar en Action
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionTemplateMetadata {
    /// ID del template usado
    pub template_id: String,
    /// Versión del template
    pub template_version: String,
    /// Variables usadas para renderizar
    pub variables: HashMap<String, serde_json::Value>,
    /// Si fue auto-detectado o manual
    pub auto_detected: bool,
    /// Timestamp de aplicación
    pub applied_at: DateTime<Utc>,
    /// Score de satisfacción del usuario (opcional)
    pub user_satisfaction: Option<u8>,
}

/// Resultado de renderizado de template
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderedTemplate {
    /// Template ID usado
    pub template_id: String,
    /// Contenido renderizado
    pub content: String,
    /// Variables usadas
    pub variables: HashMap<String, serde_json::Value>,
    /// Metadata de renderizado
    pub render_metadata: RenderMetadata,
}

/// Metadata del proceso de renderizado
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderMetadata {
    /// Timestamp de renderizado
    pub rendered_at: DateTime<Utc>,
    /// Tiempo de renderizado en ms
    pub render_time_ms: u64,
    /// Engine usado
    pub engine_used: String,
    /// Secciones incluidas
    pub sections_rendered: Vec<String>,
    /// Warnings durante renderizado
    pub warnings: Vec<String>,
}

impl Template {
    /// Crear nuevo template
    pub fn new(template_id: String, name: String, category: TemplateCategory) -> Self {
        let now = Utc::now();
        Self {
            template_id,
            name,
            category,
            triggers: Vec::new(),
            structure: TemplateStructure {
                sections: Vec::new(),
                section_separator: Some("\n\n".to_string()),
                footer: None,
            },
            metadata: TemplateMetadata {
                created_at: now,
                updated_at: now,
                version: "1.0".to_string(),
                author: None,
                usage_count: 0,
                effectiveness_score: None,
                tags: Vec::new(),
                description: None,
            },
            required_variables: Vec::new(),
            optional_variables: HashMap::new(),
        }
    }

    /// Agregar sección al template
    pub fn add_section(&mut self, section: TemplateSection) {
        self.structure.sections.push(section);
        self.touch();
    }

    /// Agregar trigger
    pub fn add_trigger(&mut self, trigger: TemplateTrigger) {
        self.triggers.push(trigger);
        self.touch();
    }

    /// Verificar si el template aplica para un contexto
    pub fn matches_context(&self, context: &HashMap<String, serde_json::Value>) -> bool {
        self.triggers.iter().any(|trigger| {
            trigger.conditions.iter().all(|(key, value)| {
                context.get(key).map_or(false, |v| v == value)
            })
        })
    }

    /// Incrementar contador de uso
    pub fn increment_usage(&mut self) {
        self.metadata.usage_count += 1;
        self.touch();
    }

    /// Actualizar score de efectividad
    pub fn update_effectiveness(&mut self, score: f32) {
        self.metadata.effectiveness_score = Some(score.clamp(0.0, 10.0));
        self.touch();
    }

    /// Actualizar timestamp
    fn touch(&mut self) {
        self.metadata.updated_at = Utc::now();
    }
}

impl TemplateSection {
    /// Crear nueva sección
    pub fn new(section_id: String, section_type: SectionType, template: String) -> Self {
        Self {
            section_id,
            section_type,
            template,
            required_vars: Vec::new(),
            conditions: None,
            list_format: None,
            formatting: Some(SectionFormatting::default()),
        }
    }

    /// Verificar si la sección debe ser renderizada
    pub fn should_render(&self, variables: &HashMap<String, serde_json::Value>) -> bool {
        // Verificar que todas las variables requeridas estén presentes
        if !self.required_vars.iter().all(|var| variables.contains_key(var)) {
            return false;
        }

        // Verificar condiciones adicionales
        if let Some(conditions) = &self.conditions {
            conditions.iter().all(|(key, expected)| {
                variables.get(key).map_or(false, |actual| actual == expected)
            })
        } else {
            true
        }
    }
}

impl SectionFormatting {
    pub fn default() -> Self {
        Self {
            indent_level: None,
            use_emojis: true,
            use_markdown: true,
            max_list_items: None,
            max_text_length: None,
        }
    }
}

impl ActionTemplateMetadata {
    /// Crear metadata para template aplicado
    pub fn for_template(template_id: String, template_version: String, auto_detected: bool) -> Self {
        Self {
            template_id,
            template_version,
            variables: HashMap::new(),
            auto_detected,
            applied_at: Utc::now(),
            user_satisfaction: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let template = Template::new(
            "milestone_brief".to_string(),
            "Milestone Completion Brief".to_string(),
            TemplateCategory::DevelopmentProgress,
        );
        
        assert_eq!(template.template_id, "milestone_brief");
        assert_eq!(template.category, TemplateCategory::DevelopmentProgress);
        assert_eq!(template.structure.sections.len(), 0);
        assert_eq!(template.metadata.usage_count, 0);
    }

    #[test]
    fn test_template_section() {
        let section = TemplateSection::new(
            "header".to_string(),
            SectionType::Header,
            "## {status} {title}".to_string(),
        );
        
        assert_eq!(section.section_type, SectionType::Header);
        assert!(section.formatting.is_some());
    }

    #[test]
    fn test_section_should_render() {
        let mut section = TemplateSection::new(
            "test".to_string(),
            SectionType::Header,
            "{title}".to_string(),
        );
        section.required_vars = vec!["title".to_string()];
        
        let mut vars = HashMap::new();
        assert!(!section.should_render(&vars)); // Missing required var
        
        vars.insert("title".to_string(), serde_json::Value::String("Test".to_string()));
        assert!(section.should_render(&vars)); // Has required var
    }

    #[test]
    fn test_template_context_matching() {
        let mut template = Template::new(
            "test".to_string(),
            "Test Template".to_string(),
            TemplateCategory::General,
        );
        
        let trigger = TemplateTrigger {
            trigger_type: TriggerType::ActionType,
            conditions: {
                let mut map = HashMap::new();
                map.insert("action_type".to_string(), serde_json::Value::String("Documentation".to_string()));
                map
            },
            priority: 1,
        };
        template.add_trigger(trigger);
        
        let context = {
            let mut map = HashMap::new();
            map.insert("action_type".to_string(), serde_json::Value::String("Documentation".to_string()));
            map
        };
        
        assert!(template.matches_context(&context));
    }
}
