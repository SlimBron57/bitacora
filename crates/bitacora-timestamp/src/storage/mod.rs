//! Storage module - Storage abstractions

pub mod file_storage;
pub mod db_storage;

use async_trait::async_trait;
use crate::TimestampResult;

pub use file_storage::FileStorage;
pub use db_storage::DbStorage;

/// Trait for timestamp storage operations
#[async_trait]
pub trait TimestampStorage {
    /// Store timestamp
    async fn store(&self, timestamp: &str) -> TimestampResult<()>;
    
    /// Retrieve latest timestamp
    async fn get_latest(&self) -> TimestampResult<Option<String>>;
    
    /// Retrieve timestamp history
    async fn get_history(&self, limit: usize) -> TimestampResult<Vec<String>>;
    
    /// Clear stored timestamps
    async fn clear(&self) -> TimestampResult<()>;
}

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub storage_type: StorageType,
    pub file_path: Option<String>,
    pub database_url: Option<String>,
}

/// Storage type enumeration
#[derive(Debug, Clone)]
pub enum StorageType {
    File,
    Database,
    Memory,
}
