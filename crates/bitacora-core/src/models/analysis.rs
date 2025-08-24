//! Analysis domain model with external documentation support

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Type alias for analysis identifiers
pub type AnalysisId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub analysis_id: AnalysisId,
    pub analysis_type: AnalysisType,
    pub context: AnalysisContext,
    pub scope: AnalysisScope,
    pub priority: AnalysisPriority,
    pub insights: Vec<Insight>,
    pub recommendations: Vec<Recommendation>,
    pub external_sources: Vec<ExternalSource>,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisType {
    RecordContextual,      // Análisis del record activo
    RoadmapStrategic,     // Análisis de roadmap general
    CrossProject,         // Análisis entre proyectos
    Predictive,          // Análisis predictivo
    DocumentationBased,   // Análisis basado en docs externas
    Hybrid,              // Combinación de interno + externo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisScope {
    CurrentRecord(Uuid),
    Project(Uuid),
    Global,
    ExternalContext(String), // URL, repo, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisPriority {
    Critical,
    High,
    Medium,
    Low,
    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub insight_id: Uuid,
    pub title: String,
    pub description: String,
    pub insight_type: InsightType,
    pub confidence: f64,
    pub source_references: Vec<SourceReference>,
    pub actionable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsightType {
    Pattern,           // Patrón identificado
    Anomaly,          // Anomalía detectada
    Opportunity,      // Oportunidad identificada
    Risk,             // Riesgo potencial
    Optimization,     // Posible optimización
    ExternalGuidance, // Guía de documentación externa
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommendation_id: Uuid,
    pub title: String,
    pub description: String,
    pub action_type: RecommendedActionType,
    pub priority: AnalysisPriority,
    pub estimated_impact: ImpactLevel,
    pub required_resources: Vec<String>,
    pub external_references: Vec<ExternalReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendedActionType {
    NextStep,              // Próxima acción sugerida
    ProcessImprovement,    // Mejora de proceso
    ResourceAllocation,    // Asignación de recursos
    ExternalConsultation,  // Consultar documentación externa
    CrossProjectSync,      // Sincronización entre proyectos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    High,
    Medium,
    Low,
}

/// External documentation sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSource {
    pub source_id: Uuid,
    pub source_type: ExternalSourceType,
    pub connector_config: ConnectorConfig,
    pub last_sync: Option<DateTime<Utc>>,
    pub status: SourceStatus,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalSourceType {
    GitRepository,    // GitHub, GitLab, etc.
    Documentation,    // Confluence, Notion, etc.
    ApiEndpoint,      // REST API
    Database,         // External database
    FileSystem,       // Local or network files
    WebScraping,      // Web pages
    SlackWorkspace,   // Slack channels/messages
    JiraProject,      // Jira issues/projects
    Custom(String),   // Custom connector
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub connector_type: String,
    pub endpoint: String,
    pub authentication: AuthenticationConfig,
    pub sync_frequency: SyncFrequency,
    pub filters: Vec<ContentFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationConfig {
    None,
    ApiKey(String),
    Bearer(String),
    Basic { username: String, password: String },
    OAuth { token: String, refresh_token: Option<String> },
    Custom(HashMap<String, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceStatus {
    Active,
    Syncing,
    Error(String),
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceReference {
    pub source_id: Option<Uuid>, // None para fuentes internas
    pub reference_type: ReferenceType,
    pub location: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    InternalSpark,
    InternalAction,
    InternalTopic,
    ExternalDoc,
    ExternalCode,
    ExternalIssue,
    ExternalDiscussion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalReference {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub source_type: ExternalSourceType,
    pub last_updated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisContext {
    pub current_record_id: Option<Uuid>,
    pub current_project_id: Option<Uuid>,
    pub user_id: String,
    pub session_context: HashMap<String, String>,
    pub external_context: Vec<ExternalContextItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalContextItem {
    pub source_id: Uuid,
    pub content_type: String,
    pub relevance_score: f64,
    pub extracted_data: HashMap<String, serde_json::Value>,
}

impl Analysis {
    pub fn builder() -> AnalysisBuilder {
        AnalysisBuilder::new()
    }
    
    pub fn has_external_sources(&self) -> bool {
        !self.external_sources.is_empty()
    }
    
    pub fn get_high_confidence_insights(&self) -> Vec<&Insight> {
        self.insights.iter()
            .filter(|insight| insight.confidence > 0.8)
            .collect()
    }
}

pub struct AnalysisBuilder {
    analysis_type: Option<AnalysisType>,
    context: Option<AnalysisContext>,
    scope: Option<AnalysisScope>,
    priority: AnalysisPriority,
    external_sources: Vec<ExternalSource>,
}

impl AnalysisBuilder {
    pub fn new() -> Self {
        Self {
            analysis_type: None,
            context: None,
            scope: None,
            priority: AnalysisPriority::Medium,
            external_sources: Vec::new(),
        }
    }
    
    pub fn analysis_type(mut self, analysis_type: AnalysisType) -> Self {
        self.analysis_type = Some(analysis_type);
        self
    }
    
    pub fn context(mut self, context: AnalysisContext) -> Self {
        self.context = Some(context);
        self
    }
    
    pub fn scope(mut self, scope: AnalysisScope) -> Self {
        self.scope = Some(scope);
        self
    }
    
    pub fn priority(mut self, priority: AnalysisPriority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn add_external_source(mut self, source: ExternalSource) -> Self {
        self.external_sources.push(source);
        self
    }
    
    pub fn build(self) -> Analysis {
        Analysis {
            analysis_id: Uuid::new_v4(),
            analysis_type: self.analysis_type.unwrap_or(AnalysisType::RecordContextual),
            context: self.context.unwrap_or_else(|| AnalysisContext {
                current_record_id: None,
                current_project_id: None,
                user_id: "unknown".to_string(),
                session_context: HashMap::new(),
                external_context: Vec::new(),
            }),
            scope: self.scope.unwrap_or(AnalysisScope::Global),
            priority: self.priority,
            insights: Vec::new(),
            recommendations: Vec::new(),
            external_sources: self.external_sources,
            confidence_score: 0.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expires_at: None,
        }
    }
}
