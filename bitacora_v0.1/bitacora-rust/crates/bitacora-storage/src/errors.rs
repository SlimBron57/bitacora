//! Storage layer errors

use thiserror::Error;

/// Storage layer error types
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Database operation failed: {0}")]
    OperationFailed(String),

    #[error("Document not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Migration error: {0}")]
    MigrationError(String),

    #[error("Index creation error: {0}")]
    IndexError(String),

    #[error("Transaction error: {0}")]
    TransactionError(String),
}

/// Storage operation result type
pub type StorageResult<T> = Result<T, StorageError>;

impl From<mongodb::error::Error> for StorageError {
    fn from(err: mongodb::error::Error) -> Self {
        StorageError::OperationFailed(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}
