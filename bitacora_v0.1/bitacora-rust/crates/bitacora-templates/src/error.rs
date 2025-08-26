//! Template system errors

use thiserror::Error;

/// Template system errors
#[derive(Error, Debug)]
pub enum TemplateError {
    /// Template not found
    #[error("Template not found: {template_id}")]
    NotFound { template_id: String },
    
    /// Invalid template format
    #[error("Invalid template format: {reason}")]
    InvalidFormat { reason: String },
    
    /// Missing required variables
    #[error("Missing required variables: {variables:?}")]
    MissingVariables { variables: Vec<String> },
    
    /// Template rendering failed
    #[error("Template rendering failed: {reason}")]
    RenderError { reason: String },
    
    /// Engine error
    #[error("Template engine error: {engine}: {reason}")]
    EngineError { engine: String, reason: String },
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Database error (placeholder for future)
    #[error("Database error: {reason}")]
    DatabaseError { reason: String },
}

pub type TemplateResult<T> = Result<T, TemplateError>;
