//! Database configuration management

use bitacora_config::{BitacoraConfig, database::DatabaseConfig as ConfigDatabaseConfig};
use serde::{Deserialize, Serialize};

/// Storage configuration wrapper for compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub database: ConfigDatabaseConfig,
    pub migration: MigrationConfig,
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    pub auto_migrate: bool,
    pub migration_timeout: u64,
    pub backup_before_migration: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            auto_migrate: true,
            migration_timeout: 30, // seconds
            backup_before_migration: true,
        }
    }
}

impl StorageConfig {
    /// Create from BitacoraConfig
    pub fn from_bitacora_config(config: &BitacoraConfig) -> Self {
        Self {
            database: config.database.clone(),
            migration: MigrationConfig::default(),
        }
    }
    
    /// Get connection string
    pub fn connection_string(&self) -> String {
        self.database.connection_string()
    }
    
    /// Check if using MongoDB
    pub fn is_mongodb(&self) -> bool {
        self.database.provider == "mongodb"
    }
    
    /// Check if using SQLite
    pub fn is_sqlite(&self) -> bool {
        self.database.provider == "sqlite"
    }
}

// Legacy compatibility - transitional 
#[deprecated(note = "Use StorageConfig::from_bitacora_config instead")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database type (mongodb, sqlite)
    pub db_type: DatabaseType,
    /// Connection string
    pub connection_string: String,
    /// Database name
    pub database_name: String,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Maximum connection pool size
    pub max_pool_size: u32,
    /// Enable logging for database operations
    pub enable_logging: bool,
}

/// Supported database types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    MongoDB,
    SQLite,
}

impl DatabaseConfig {
    /// Create MongoDB configuration
    pub fn mongodb(connection_string: String, database_name: String) -> Self {
        Self {
            db_type: DatabaseType::MongoDB,
            connection_string,
            database_name,
            connection_timeout: 30,
            max_pool_size: 10,
            enable_logging: false,
        }
    }

    /// Create SQLite configuration
    pub fn sqlite(database_path: String) -> Self {
        Self {
            db_type: DatabaseType::SQLite,
            connection_string: format!("sqlite:{}", database_path),
            database_name: "bitacora.db".to_string(),
            connection_timeout: 10,
            max_pool_size: 5,
            enable_logging: false,
        }
    }

    /// Create default development configuration
    pub fn development() -> Self {
        Self::mongodb(
            "mongodb://bitacora:dev_password@localhost:27017".to_string(),
            "bitacora_dev".to_string(),
        )
    }

    /// Create default production configuration
    pub fn production() -> Self {
        Self::mongodb(
            std::env::var("MONGODB_URL")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            std::env::var("MONGODB_DATABASE")
                .unwrap_or_else(|_| "bitacora".to_string()),
        ).with_logging(true)
    }

    /// Enable logging
    pub fn with_logging(mut self, enable: bool) -> Self {
        self.enable_logging = enable;
        self
    }

    /// Set connection timeout
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.connection_timeout = timeout;
        self
    }

    /// Set max pool size
    pub fn with_pool_size(mut self, size: u32) -> Self {
        self.max_pool_size = size;
        self
    }
}
