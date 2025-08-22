//! Spark domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa una idea, insight o momento de inspiración
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spark {
    /// Identificador único del spark
    pub spark_id: Uuid,
    /// ID del usuario que creó el spark
    pub user_id: String,
    /// ID de la sesión donde se generó (opcional)
    pub session_id: Option<Uuid>,
    /// ID del proyecto relacionado (opcional)
    pub project_id: Option<Uuid>,
    /// ID del topic relacionado (opcional)
    pub topic_id: Option<Uuid>,
    /// Título breve del spark
    pub title: String,
    /// Descripción detallada del spark
    pub content: String,
    /// Tipo de spark
    pub spark_type: SparkType,
    /// Categoría o dominio
    pub category: String,
    /// Tags para categorización
    pub tags: Vec<String>,
    /// Nivel de importancia
    pub importance: ImportanceLevel,
    /// Estado de seguimiento
    pub status: SparkStatus,
    /// Contexto donde surgió la idea
    pub context: SparkContext,
    /// Enlaces o referencias relacionadas
    pub references: Vec<String>,
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
    /// Fecha de revisión sugerida
    pub review_at: Option<DateTime<Utc>>,
    /// Número de veces que se ha revisado
    pub review_count: u32,
    /// Puntuación de utilidad (1-10)
    pub utility_score: Option<u8>,
}

/// Tipos de spark según su naturaleza
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SparkType {
    /// Idea para nueva funcionalidad
    Idea,
    /// Insight sobre el problema
    Insight,
    /// Solución a problema técnico
    Solution,
    /// Pregunta para investigar
    Question,
    /// Observación interesante
    Observation,
    /// Aprendizaje o lección
    Learning,
    /// Mejora de proceso
    Improvement,
    /// Conexión entre conceptos
    Connection,
    /// Reflexión personal
    Reflection,
    /// Inspiración creativa
    Inspiration,
}

/// Nivel de importancia
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ImportanceLevel {
    /// Baja importancia
    Low,
    /// Importancia media
    Medium,
    /// Alta importancia
    High,
    /// Crítica - requiere atención inmediata
    Critical,
}

/// Estado de seguimiento del spark
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SparkStatus {
    /// Recién capturado
    Captured,
    /// Bajo revisión
    Reviewing,
    /// Siendo desarrollado
    Developing,
    /// Implementado o aplicado
    Implemented,
    /// Archivado por ahora
    Archived,
    /// Descartado
    Discarded,
}

/// Contexto donde surgió el spark
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SparkContext {
    /// Actividad que se estaba realizando
    pub activity: String,
    /// Herramienta o tecnología involucrada
    pub tool: Option<String>,
    /// Archivo o ubicación específica
    pub location: Option<String>,
    /// Conversación o fuente que inspiró
    pub source: Option<String>,
    /// Estado mental (concentrado, creativo, etc.)
    pub mental_state: Option<String>,
}

impl Spark {
    /// Crear nuevo spark
    pub fn new(user_id: String, title: String, content: String, spark_type: SparkType) -> Self {
        let now = Utc::now();
        Self {
            spark_id: Uuid::new_v4(),
            user_id,
            session_id: None,
            project_id: None,
            topic_id: None,
            title,
            content,
            spark_type,
            category: "general".to_string(),
            tags: Vec::new(),
            importance: ImportanceLevel::Medium,
            status: SparkStatus::Captured,
            context: SparkContext::default(),
            references: Vec::new(),
            created_at: now,
            updated_at: now,
            review_at: Some(now + chrono::Duration::days(7)), // Review in a week
            review_count: 0,
            utility_score: None,
        }
    }

    /// Crear spark rápido con contexto mínimo
    pub fn quick(user_id: String, content: String) -> Self {
        Self::new(user_id, "Quick Spark".to_string(), content, SparkType::Idea)
    }

    /// Asociar con sesión
    pub fn with_session(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self.touch();
        self
    }

    /// Asociar con proyecto
    pub fn with_project(mut self, project_id: Uuid) -> Self {
        self.project_id = Some(project_id);
        self.touch();
        self
    }

    /// Asociar con topic
    pub fn with_topic(mut self, topic_id: Uuid) -> Self {
        self.topic_id = Some(topic_id);
        self.touch();
        self
    }

    /// Establecer categoría
    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self.touch();
        self
    }

    /// Agregar tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.touch();
        self
    }

    /// Establecer importancia
    pub fn with_importance(mut self, importance: ImportanceLevel) -> Self {
        self.importance = importance;
        self.touch();
        self
    }

    /// Establecer contexto
    pub fn with_context(mut self, context: SparkContext) -> Self {
        self.context = context;
        self.touch();
        self
    }

    /// Agregar referencia
    pub fn add_reference(&mut self, reference: String) {
        if !self.references.contains(&reference) {
            self.references.push(reference);
            self.touch();
        }
    }

    /// Actualizar contenido
    pub fn update_content(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.touch();
    }

    /// Cambiar estado
    pub fn set_status(&mut self, status: SparkStatus) {
        self.status = status;
        self.touch();
    }

    /// Marcar como en revisión
    pub fn start_review(&mut self) {
        self.status = SparkStatus::Reviewing;
        self.review_count += 1;
        self.touch();
    }

    /// Marcar como implementado
    pub fn implement(&mut self) -> Result<(), String> {
        if !self.can_implement() {
            return Err("Spark must be reviewed before implementation".to_string());
        }
        
        self.status = SparkStatus::Implemented;
        self.touch();
        Ok(())
    }

    /// Archivar spark
    pub fn archive(&mut self) {
        self.status = SparkStatus::Archived;
        self.touch();
    }

    /// Descartar spark
    pub fn discard(&mut self) {
        self.status = SparkStatus::Discarded;
        self.touch();
    }

    /// Programar próxima revisión
    pub fn schedule_review(&mut self, days_from_now: i64) {
        self.review_at = Some(Utc::now() + chrono::Duration::days(days_from_now));
        self.touch();
    }

    /// Establecer puntuación de utilidad
    pub fn rate_utility(&mut self, score: u8) -> Result<(), String> {
        if score < 1 || score > 10 {
            return Err("Utility score must be between 1 and 10".to_string());
        }
        
        self.utility_score = Some(score);
        self.touch();
        Ok(())
    }

    /// Verificar si puede ser implementado
    pub fn can_implement(&self) -> bool {
        matches!(self.status, SparkStatus::Reviewing | SparkStatus::Developing)
    }

    /// Verificar si está pendiente de revisión
    pub fn needs_review(&self) -> bool {
        if let Some(review_date) = self.review_at {
            Utc::now() >= review_date && matches!(self.status, SparkStatus::Captured | SparkStatus::Reviewing)
        } else {
            false
        }
    }

    /// Verificar si está activo (no archivado ni descartado)
    pub fn is_active(&self) -> bool {
        !matches!(self.status, SparkStatus::Archived | SparkStatus::Discarded)
    }

    /// Verificar si está implementado
    pub fn is_implemented(&self) -> bool {
        matches!(self.status, SparkStatus::Implemented)
    }

    /// Verificar si es de alta prioridad
    pub fn is_high_priority(&self) -> bool {
        matches!(self.importance, ImportanceLevel::High | ImportanceLevel::Critical)
    }

    /// Verificar si tiene un tag específico
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Obtener edad en días
    pub fn age_in_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }

    /// Verificar si está relacionado con un proyecto o topic
    pub fn is_related_to(&self, project_id: Option<Uuid>, topic_id: Option<Uuid>) -> bool {
        (project_id.is_some() && self.project_id == project_id) ||
        (topic_id.is_some() && self.topic_id == topic_id)
    }

    /// Obtener descripción del contexto
    pub fn context_summary(&self) -> String {
        let mut summary = self.context.activity.clone();
        
        if let Some(tool) = &self.context.tool {
            summary.push_str(&format!(" (using {})", tool));
        }
        
        if let Some(location) = &self.context.location {
            summary.push_str(&format!(" at {}", location));
        }
        
        summary
    }

    /// Generar resumen del spark
    pub fn summary(&self) -> String {
        format!(
            "{} [{}] - {} ({})",
            self.title,
            self.spark_type.as_str(),
            self.importance.as_str(),
            self.status.as_str()
        )
    }

    /// Actualizar timestamp de modificación
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl SparkType {
    /// Convertir a string
    pub fn as_str(&self) -> &'static str {
        match self {
            SparkType::Idea => "idea",
            SparkType::Insight => "insight",
            SparkType::Solution => "solution",
            SparkType::Question => "question",
            SparkType::Observation => "observation",
            SparkType::Learning => "learning",
            SparkType::Improvement => "improvement",
            SparkType::Connection => "connection",
            SparkType::Reflection => "reflection",
            SparkType::Inspiration => "inspiration",
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Option<SparkType> {
        match s.to_lowercase().as_str() {
            "idea" => Some(SparkType::Idea),
            "insight" => Some(SparkType::Insight),
            "solution" => Some(SparkType::Solution),
            "question" => Some(SparkType::Question),
            "observation" => Some(SparkType::Observation),
            "learning" => Some(SparkType::Learning),
            "improvement" => Some(SparkType::Improvement),
            "connection" => Some(SparkType::Connection),
            "reflection" => Some(SparkType::Reflection),
            "inspiration" => Some(SparkType::Inspiration),
            _ => None,
        }
    }

    /// Obtener icono representativo
    pub fn icon(&self) -> &'static str {
        match self {
            SparkType::Idea => "💡",
            SparkType::Insight => "🔍",
            SparkType::Solution => "🔧",
            SparkType::Question => "❓",
            SparkType::Observation => "👁️",
            SparkType::Learning => "📚",
            SparkType::Improvement => "⬆️",
            SparkType::Connection => "🔗",
            SparkType::Reflection => "🤔",
            SparkType::Inspiration => "✨",
        }
    }
}

impl ImportanceLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImportanceLevel::Low => "low",
            ImportanceLevel::Medium => "medium",
            ImportanceLevel::High => "high",
            ImportanceLevel::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Option<ImportanceLevel> {
        match s.to_lowercase().as_str() {
            "low" => Some(ImportanceLevel::Low),
            "medium" | "med" => Some(ImportanceLevel::Medium),
            "high" => Some(ImportanceLevel::High),
            "critical" | "crit" => Some(ImportanceLevel::Critical),
            _ => None,
        }
    }
}

impl SparkStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SparkStatus::Captured => "captured",
            SparkStatus::Reviewing => "reviewing",
            SparkStatus::Developing => "developing",
            SparkStatus::Implemented => "implemented",
            SparkStatus::Archived => "archived",
            SparkStatus::Discarded => "discarded",
        }
    }
}

impl SparkContext {
    pub fn default() -> Self {
        Self {
            activity: "unknown".to_string(),
            tool: None,
            location: None,
            source: None,
            mental_state: None,
        }
    }

    pub fn new(activity: String) -> Self {
        Self {
            activity,
            tool: None,
            location: None,
            source: None,
            mental_state: None,
        }
    }

    pub fn coding(tool: String, location: String) -> Self {
        Self {
            activity: "coding".to_string(),
            tool: Some(tool),
            location: Some(location),
            source: None,
            mental_state: Some("focused".to_string()),
        }
    }

    pub fn debugging(tool: String, location: String) -> Self {
        Self {
            activity: "debugging".to_string(),
            tool: Some(tool),
            location: Some(location),
            source: None,
            mental_state: Some("analytical".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spark_creation() {
        let spark = Spark::new(
            "user123".to_string(),
            "Great idea".to_string(),
            "This could improve performance".to_string(),
            SparkType::Idea,
        );
        
        assert_eq!(spark.user_id, "user123");
        assert_eq!(spark.title, "Great idea");
        assert_eq!(spark.spark_type, SparkType::Idea);
        assert_eq!(spark.status, SparkStatus::Captured);
        assert_eq!(spark.importance, ImportanceLevel::Medium);
        assert!(spark.review_at.is_some());
    }

    #[test]
    fn test_spark_lifecycle() {
        let mut spark = Spark::new(
            "user".to_string(),
            "Test Idea".to_string(),
            "Content".to_string(),
            SparkType::Solution,
        );
        
        // Start review
        spark.start_review();
        assert_eq!(spark.status, SparkStatus::Reviewing);
        assert_eq!(spark.review_count, 1);
        
        // Implement
        assert!(spark.implement().is_ok());
        assert!(spark.is_implemented());
        
        // Archive
        spark.archive();
        assert!(!spark.is_active());
    }

    #[test]
    fn test_spark_context() {
        let context = SparkContext::coding(
            "VS Code".to_string(),
            "src/main.rs".to_string(),
        );
        
        let spark = Spark::new(
            "user".to_string(),
            "Optimization idea".to_string(),
            "Use HashMap instead".to_string(),
            SparkType::Improvement,
        ).with_context(context);
        
        assert_eq!(spark.context.activity, "coding");
        assert_eq!(spark.context.tool, Some("VS Code".to_string()));
        assert!(spark.context_summary().contains("VS Code"));
    }

    #[test]
    fn test_spark_utility_rating() {
        let mut spark = Spark::quick("user".to_string(), "Test content".to_string());
        
        assert!(spark.rate_utility(8).is_ok());
        assert_eq!(spark.utility_score, Some(8));
        
        assert!(spark.rate_utility(11).is_err()); // Invalid score
        assert!(spark.rate_utility(0).is_err()); // Invalid score
    }

    #[test]
    fn test_spark_review_scheduling() {
        let mut spark = Spark::quick("user".to_string(), "Test".to_string());
        
        spark.schedule_review(1); // Tomorrow
        assert!(spark.review_at.is_some());
        
        // Simulate time passing (can't easily test without time manipulation)
        // In real scenario, needs_review() would return true after the date
    }
}
