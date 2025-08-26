//! External data connectors for documentation and resources

use async_trait::async_trait;
use bitacora_core::models::analysis::*;
use crate::errors::{AnalyticsError, AnalyticsResult};
use std::collections::HashMap;
use serde_json::Value;
use uuid::Uuid;

pub mod github;
pub mod confluence;
pub mod jira;
pub mod slack;
pub mod web_scraper;
pub mod file_system;
pub mod api_client;

/// Main connector manager that orchestrates all external connectors
pub struct ConnectorManager {
    connectors: HashMap<String, Box<dyn ExternalConnector>>,
    active_syncs: dashmap::DashMap<Uuid, SyncStatus>,
}

/// Status of an ongoing sync operation
#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub source_id: Uuid,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub progress: SyncProgress,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SyncProgress {
    Starting,
    InProgress { current: usize, total: Option<usize> },
    Completed { processed: usize },
    Failed { error: String },
}

/// Trait that all external connectors must implement
#[async_trait]
pub trait ExternalConnector: Send + Sync {
    /// Get the connector type identifier
    fn connector_type(&self) -> &str;
    
    /// Test if the connector can connect with given config
    async fn test_connection(&self, config: &ConnectorConfig) -> AnalyticsResult<bool>;
    
    /// Sync data from external source
    async fn sync_data(&self, source: &ExternalSource) -> AnalyticsResult<Vec<ExternalDocument>>;
    
    /// Search for specific content in the external source
    async fn search_content(&self, source: &ExternalSource, query: &str) -> AnalyticsResult<Vec<ExternalDocument>>;
    
    /// Get metadata about the external source
    async fn get_source_metadata(&self, config: &ConnectorConfig) -> AnalyticsResult<SourceMetadata>;
}

/// Document retrieved from external source
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExternalDocument {
    pub document_id: String,
    pub title: String,
    pub content: String,
    pub content_type: ContentType,
    pub url: Option<String>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    pub author: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, Value>,
    pub relevance_score: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ContentType {
    Markdown,
    Html,
    PlainText,
    Code(String), // Language
    Json,
    Yaml,
    Other(String),
}

/// Metadata about an external source
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceMetadata {
    pub total_documents: Option<usize>,
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    pub available_filters: Vec<FilterOption>,
    pub rate_limits: Option<RateLimits>,
    pub supported_operations: Vec<SupportedOperation>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilterOption {
    pub field: String,
    pub display_name: String,
    pub filter_type: FilterType,
    pub possible_values: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FilterType {
    Select,
    MultiSelect,
    Text,
    Date,
    Number,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimits {
    pub requests_per_hour: Option<u32>,
    pub requests_per_day: Option<u32>,
    pub concurrent_requests: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SupportedOperation {
    Read,
    Search,
    Subscribe, // For real-time updates
    Webhook,   // For push notifications
}

impl ConnectorManager {
    pub fn new() -> Self {
        let mut manager = Self {
            connectors: HashMap::new(),
            active_syncs: dashmap::DashMap::new(),
        };
        
        // Register default connectors
        manager.register_default_connectors();
        manager
    }
    
    fn register_default_connectors(&mut self) {
        // TODO: Register built-in connectors
        // self.register_connector(Box::new(github::GitHubConnector::new()));
        // self.register_connector(Box::new(confluence::ConfluenceConnector::new()));
        // self.register_connector(Box::new(web_scraper::WebScraperConnector::new()));
    }
    
    /// Register a new connector
    pub fn register_connector(&mut self, connector: Box<dyn ExternalConnector>) {
        let connector_type = connector.connector_type().to_string();
        self.connectors.insert(connector_type, connector);
    }
    
    /// Get available connector types
    pub fn get_connector_types(&self) -> Vec<String> {
        self.connectors.keys().cloned().collect()
    }
    
    /// Test connection for a specific source
    pub async fn test_connection(&self, source: &ExternalSource) -> AnalyticsResult<bool> {
        let connector = self.connectors
            .get(&source.connector_config.connector_type)
            .ok_or_else(|| AnalyticsError::connector_error(
                &source.connector_config.connector_type,
                "Connector not found"
            ))?;
            
        connector.test_connection(&source.connector_config).await
    }
    
    /// Start syncing data from external source
    pub async fn start_sync(&self, source: &ExternalSource) -> AnalyticsResult<Uuid> {
        let sync_id = Uuid::new_v4();
        
        // Record sync start
        self.active_syncs.insert(sync_id, SyncStatus {
            source_id: source.source_id,
            started_at: chrono::Utc::now(),
            progress: SyncProgress::Starting,
            last_error: None,
        });
        
        let connector = self.connectors
            .get(&source.connector_config.connector_type)
            .ok_or_else(|| AnalyticsError::connector_error(
                &source.connector_config.connector_type,
                "Connector not found"
            ))?;
        
        // Clone for async operation
        let source_clone = source.clone();
        let sync_map = self.active_syncs.clone();
        
        // Spawn sync task
        tokio::spawn(async move {
            match connector.sync_data(&source_clone).await {
                Ok(documents) => {
                    sync_map.insert(sync_id, SyncStatus {
                        source_id: source_clone.source_id,
                        started_at: chrono::Utc::now(),
                        progress: SyncProgress::Completed { processed: documents.len() },
                        last_error: None,
                    });
                    
                    // TODO: Store documents in analytics storage
                    tracing::info!("Sync completed for source {}: {} documents", 
                                 source_clone.source_id, documents.len());
                },
                Err(e) => {
                    sync_map.insert(sync_id, SyncStatus {
                        source_id: source_clone.source_id,
                        started_at: chrono::Utc::now(),
                        progress: SyncProgress::Failed { error: e.to_string() },
                        last_error: Some(e.to_string()),
                    });
                    tracing::error!("Sync failed for source {}: {}", 
                                  source_clone.source_id, e);
                }
            }
        });
        
        Ok(sync_id)
    }
    
    /// Get sync status
    pub fn get_sync_status(&self, sync_id: &Uuid) -> Option<SyncStatus> {
        self.active_syncs.get(sync_id).map(|s| s.clone())
    }
    
    /// Search content across multiple sources
    pub async fn search_across_sources(
        &self, 
        sources: &[ExternalSource], 
        query: &str
    ) -> AnalyticsResult<Vec<(Uuid, Vec<ExternalDocument>)>> {
        let mut results = Vec::new();
        
        for source in sources {
            if let Some(connector) = self.connectors.get(&source.connector_config.connector_type) {
                match connector.search_content(source, query).await {
                    Ok(documents) => {
                        results.push((source.source_id, documents));
                    }
                    Err(e) => {
                        tracing::warn!("Search failed for source {}: {}", source.source_id, e);
                        // Continue with other sources
                    }
                }
            }
        }
        
        Ok(results)
    }
}
