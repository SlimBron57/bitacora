//! # Bitacora Navigator - Sistema Híbrido de Navegación
//!
//! Este crate implementa el Sistema Híbrido de Navegación de Bitacora, que combina
//! la eficiencia de índices con la flexibilidad de queries dinámicas, añadiendo un
//! motor de decisiones AI que automatiza la selección de estrategias.
//!
//! ## Arquitectura Principal
//!
//! - **HybridNavigator**: Estructura principal del navegador híbrido
//! - **NavigatorMode**: Modos de operación (Core, Advanced, AI-Driven)
//! - **ThreadManager**: Gestión especializada de threading (4 niveles)
//! - **SafetyController**: Control de locks y detección de conflictos
//! - **AIDecisionEngine**: Motor de decisiones AI configurable
//!
//! ## Threading Especializado
//!
//! - **Nivel 0**: Múltiples Sparks simultáneos
//! - **Nivel 1**: Múltiples Proyectos aislados
//! - **Nivel 2**: Serial por Topic dentro de Project
//! - **Nivel 3**: Serial completo (un Action a la vez)

pub mod core;
pub mod modes;
pub mod threading;
pub mod safety;
pub mod ai_engine;
pub mod errors;
pub mod bitaflow;
pub mod yaml_parser;
pub mod variable_substitution;
pub mod template_repository;

// Re-exports principales
pub use core::HybridNavigator;
pub use modes::NavigatorMode;
pub use threading::ThreadManager;
pub use safety::SafetyController;
pub use ai_engine::AIDecisionEngine;
pub use errors::{NavigatorError, Result};
pub use bitaflow::BitaflowNavigatorEngine;
pub use template_repository::TemplateRepository;

// Constantes del sistema
pub const NAVIGATOR_VERSION: &str = "1.0.0";
pub const MAX_CONCURRENT_SPARKS: usize = 10;
pub const MAX_PROJECT_ISOLATION: usize = 5;

// Traits públicos
#[async_trait::async_trait]
pub trait NavigatorCore {
    /// Navegar usando el modo híbrido
    async fn navigate(&self, context: NavigationContext) -> Result<NavigationResult>;
    
    /// Actualizar índices en background
    async fn update_indices(&self) -> Result<()>;
    
    /// Consultar directamente la base de datos
    async fn query_database(&self, query: DatabaseQuery) -> Result<QueryResult>;
}

/// Contexto de navegación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NavigationContext {
    pub project_id: Option<uuid::Uuid>,
    pub topic_id: Option<uuid::Uuid>,
    pub action_id: Option<uuid::Uuid>,
    pub spark_context: Option<String>,
    pub user_query: String,
    pub ai_assistance_level: AIAssistanceLevel,
}

/// Resultado de navegación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NavigationResult {
    pub strategy_used: NavigationStrategy,
    pub execution_time: std::time::Duration,
    pub results: Vec<ContentResult>,
    pub confidence_score: f32,
    pub next_suggestions: Vec<String>,
}

/// Estrategia utilizada para navegación
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NavigationStrategy {
    IndexBased,
    QueryBased,
    HybridOptimized,
    AIDecided,
}

/// Resultado de contenido
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContentResult {
    pub content_type: ContentType,
    pub content_id: uuid::Uuid,
    pub title: String,
    pub snippet: Option<String>,
    pub relevance_score: f32,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Tipo de contenido
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ContentType {
    Project,
    Topic,
    Action,
    Spark,
    Note,
    Code,
}

/// Query de base de datos
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseQuery {
    pub query_type: QueryType,
    pub filters: std::collections::HashMap<String, serde_json::Value>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Tipo de query
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum QueryType {
    FindProjects,
    FindTopics,
    FindActions,
    FindSparks,
    SearchContent,
    AnalyzePattern,
}

/// Resultado de query
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryResult {
    pub total_results: usize,
    pub execution_time: std::time::Duration,
    pub results: Vec<serde_json::Value>,
}

/// Nivel de asistencia AI
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum AIAssistanceLevel {
    None,
    Basic,
    Standard,
    Advanced,
    Full,
}

impl Default for AIAssistanceLevel {
    fn default() -> Self {
        AIAssistanceLevel::Standard
    }
}

impl Default for NavigationContext {
    fn default() -> Self {
        Self {
            project_id: None,
            topic_id: None,
            action_id: None,
            spark_context: None,
            user_query: String::new(),
            ai_assistance_level: AIAssistanceLevel::default(),
        }
    }
}
