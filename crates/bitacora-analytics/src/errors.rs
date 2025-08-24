//! Error types for Analytics Service

use thiserror::Error;

/// Error types for analytics operations
#[derive(Debug, Error)]
pub enum AnalyticsError {
    #[error("Analysis not found: {analysis_id}")]
    AnalysisNotFound { analysis_id: String },

    #[error("External source error: {source_id} - {message}")]
    ExternalSourceError { source_id: String, message: String },

    #[error("Connector error: {connector_type} - {message}")]
    ConnectorError { connector_type: String, message: String },

    #[error("Authentication failed for source: {source_id}")]
    AuthenticationFailed { source_id: String },

    #[error("Sync failed: {message}")]
    SyncError { message: String },

    #[error("Content parsing error: {message}")]
    ContentParsingError { message: String },

    #[error("Analysis engine error: {message}")]
    EngineError { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Rate limit exceeded for source: {source_id}")]
    RateLimitExceeded { source_id: String },

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] bitacora_storage::StorageError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl AnalyticsError {
    pub fn external_source_error(source_id: String, message: &str) -> Self {
        Self::ExternalSourceError { source_id, message: message.to_string() }
    }

    pub fn connector_error(connector_type: &str, message: &str) -> Self {
        Self::ConnectorError { 
            connector_type: connector_type.to_string(), 
            message: message.to_string() 
        }
    }

    pub fn sync_error(message: &str) -> Self {
        Self::SyncError { message: message.to_string() }
    }

    pub fn content_parsing_error(message: &str) -> Self {
        Self::ContentParsingError { message: message.to_string() }
    }

    pub fn engine_error(message: &str) -> Self {
        Self::EngineError { message: message.to_string() }
    }

    pub fn configuration_error(message: &str) -> Self {
        Self::ConfigurationError { message: message.to_string() }
    }
}

pub type AnalyticsResult<T> = std::result::Result<T, AnalyticsError>;
