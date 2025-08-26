//! Database connectors

use async_trait::async_trait;
use bitacora_config::BitacoraConfig;
use crate::{StorageResult, StorageConfig};

pub mod mongodb_connector;

// Re-export for convenience
pub use mongodb_connector::MongoDbConnector;

/// Database connector trait
#[async_trait]
pub trait DatabaseConnector: Send + Sync {
    /// Connect to the database
    async fn connect(&self, config: &StorageConfig) -> StorageResult<()>;
    
    /// Check if connection is healthy
    async fn health_check(&self) -> StorageResult<bool>;
    
    /// Get database name
    async fn database_name(&self) -> String;
    
    /// Close connection
    async fn disconnect(&self) -> StorageResult<()>;
}

/// Connector manager for handling multiple database types
pub struct ConnectorManager {
    primary_connector: Box<dyn DatabaseConnector>,
    fallback_connector: Option<Box<dyn DatabaseConnector>>,
    config: StorageConfig,
}

impl ConnectorManager {
    /// Create new connector manager from BitacoraConfig
    pub async fn new(bitacora_config: &BitacoraConfig) -> StorageResult<Self> {
        let config = StorageConfig::from_bitacora_config(bitacora_config);
        
        // Create primary connector based on configuration
        let primary: Box<dyn DatabaseConnector> = if config.is_mongodb() {
            Box::new(MongoDbConnector::new())
        } else {
            // TODO: Add SQLite connector when ready
            return Err(crate::StorageError::ConfigError(
                "SQLite connector not yet implemented".to_string()
            ));
        };
        
        let mut manager = Self {
            primary_connector: primary,
            fallback_connector: None,
            config,
        };
        
        // Try to connect immediately
        manager.connect().await?;
        
        Ok(manager)
    }

    /// Add fallback connector
    pub fn with_fallback(mut self, fallback: Box<dyn DatabaseConnector>) -> Self {
        self.fallback_connector = Some(fallback);
        self
    }

    /// Connect using primary or fallback connector
    pub async fn connect(&self) -> StorageResult<()> {
        match self.primary_connector.connect(&self.config).await {
            Ok(_) => Ok(()),
            Err(e) => {
                if let Some(fallback) = &self.fallback_connector {
                    tracing::warn!("Primary connector failed, trying fallback: {}", e);
                    fallback.connect(&self.config).await
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Health check for active connector
    pub async fn health_check(&self) -> StorageResult<bool> {
        self.primary_connector.health_check().await
    }
    
    /// Run migrations
    pub async fn run_migrations(&self) -> StorageResult<()> {
        if !self.config.migration.auto_migrate {
            return Ok(());
        }
        
        // TODO: Implement migration runner
        tracing::info!("Migration system not yet implemented");
        Ok(())
    }
}
