//! Storage layer for Bitacora V1.0
//! 
//! Provides database connectivity and repository implementations
//! for MongoDB with fallback options.

use bitacora_core::errors::BitacoraError;
use bitacora_config::BitacoraConfig;

pub mod connectors;
pub mod repository; 
pub mod query;
pub mod migration;
pub mod config;
pub mod errors;

// Re-export key types
pub use config::{StorageConfig, MigrationConfig};
pub use errors::{StorageError, StorageResult};
pub use connectors::{DatabaseConnector, MongoDbConnector, ConnectorManager};
pub use repository::{
    SessionRepository, ActionRepository, ProjectRepository, 
    TopicRepository, SparkRepository,
    MongoSessionRepository, MongoActionRepository, MongoProjectRepository,
    MongoTopicRepository, MongoSparkRepository
};

/// Initialize storage subsystem with configuration
pub async fn initialize_storage(config: &BitacoraConfig) -> Result<ConnectorManager, BitacoraError> {
    let connector_manager = ConnectorManager::new(config).await
        .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
    
    // Run migrations if needed
    connector_manager.run_migrations().await
        .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
    
    Ok(connector_manager)
}
