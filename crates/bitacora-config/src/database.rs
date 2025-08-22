use serde::{Deserialize, Serialize};
use crate::{ConfigError, ConfigResult};

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database provider (mongodb, sqlite)
    #[serde(default = "default_provider")]
    pub provider: String,
    
    /// MongoDB configuration
    #[serde(default)]
    pub mongodb: MongoDbConfig,
    
    /// SQLite configuration (fallback)
    #[serde(default)]
    pub sqlite: SqliteConfig,
    
    /// Connection pool settings
    #[serde(default)]
    pub pool: PoolConfig,
}

/// MongoDB specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDbConfig {
    /// MongoDB host
    #[serde(default = "default_mongo_host")]
    pub host: String,
    
    /// MongoDB port
    #[serde(default = "default_mongo_port")]
    pub port: u16,
    
    /// Database name
    #[serde(default = "default_mongo_database")]
    pub database: String,
    
    /// Username (optional)
    pub username: Option<String>,
    
    /// Password (optional, secured)
    /// Note: Should be provided via environment variable for security
    pub password: Option<String>,
    
    /// Authentication database (optional)
    pub auth_source: Option<String>,
    
    /// Use TLS/SSL
    #[serde(default)]
    pub tls: bool,
    
    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

/// SQLite specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    /// SQLite database file path
    #[serde(default = "default_sqlite_path")]
    pub path: String,
    
    /// Enable WAL mode
    #[serde(default = "default_true")]
    pub wal_mode: bool,
    
    /// Foreign key constraints
    #[serde(default = "default_true")]
    pub foreign_keys: bool,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    
    /// Minimum number of connections
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    
    /// Connection timeout in seconds
    #[serde(default = "default_timeout")]
    pub connection_timeout: u64,
    
    /// Idle timeout in seconds
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64,
}

impl DatabaseConfig {
    /// Get the appropriate connection string based on provider
    pub fn connection_string(&self) -> String {
        match self.provider.as_str() {
            "mongodb" => self.mongodb_connection_string(),
            "sqlite" => self.sqlite_connection_string(),
            _ => self.mongodb_connection_string(), // Default to MongoDB
        }
    }
    
    /// Build MongoDB connection string
    pub fn mongodb_connection_string(&self) -> String {
        let mut url = if let (Some(username), Some(password)) = (&self.mongodb.username, &self.mongodb.password) {
            format!(
                "mongodb://{}:{}@{}:{}",
                username,
                password,
                self.mongodb.host,
                self.mongodb.port
            )
        } else {
            format!("mongodb://{}:{}", self.mongodb.host, self.mongodb.port)
        };
        
        // Add database name
        url.push('/');
        url.push_str(&self.mongodb.database);
        
        // Add query parameters
        let mut params = Vec::new();
        
        if let Some(auth_source) = &self.mongodb.auth_source {
            params.push(format!("authSource={}", auth_source));
        }
        
        if self.mongodb.tls {
            params.push("tls=true".to_string());
        }
        
        params.push(format!("connectTimeoutMS={}", self.mongodb.timeout_seconds * 1000));
        params.push(format!("maxPoolSize={}", self.pool.max_connections));
        params.push(format!("minPoolSize={}", self.pool.min_connections));
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        
        url
    }
    
    /// Build SQLite connection string
    pub fn sqlite_connection_string(&self) -> String {
        let mut url = format!("sqlite:{}", self.sqlite.path);
        
        let mut params = Vec::new();
        
        if self.sqlite.wal_mode {
            params.push("journal_mode=WAL");
        }
        
        if self.sqlite.foreign_keys {
            params.push("foreign_keys=ON");
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        
        url
    }
    
    /// Validate database configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate provider
        if !["mongodb", "sqlite"].contains(&self.provider.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid database provider: {}. Must be 'mongodb' or 'sqlite'", self.provider)
            ));
        }
        
        // Validate MongoDB config if using MongoDB
        if self.provider == "mongodb" {
            if self.mongodb.host.is_empty() {
                return Err(ConfigError::ValidationError("MongoDB host cannot be empty".to_string()));
            }
            
            if self.mongodb.database.is_empty() {
                return Err(ConfigError::ValidationError("MongoDB database name cannot be empty".to_string()));
            }
            
            if self.mongodb.port == 0 {
                return Err(ConfigError::ValidationError("MongoDB port must be greater than 0".to_string()));
            }
        }
        
        // Validate SQLite config if using SQLite
        if self.provider == "sqlite" {
            if self.sqlite.path.is_empty() {
                return Err(ConfigError::ValidationError("SQLite path cannot be empty".to_string()));
            }
        }
        
        // Validate pool config
        if self.pool.max_connections == 0 {
            return Err(ConfigError::ValidationError("Max connections must be greater than 0".to_string()));
        }
        
        if self.pool.min_connections > self.pool.max_connections {
            return Err(ConfigError::ValidationError("Min connections cannot be greater than max connections".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if using MongoDB
    pub fn is_mongodb(&self) -> bool {
        self.provider == "mongodb"
    }
    
    /// Check if using SQLite
    pub fn is_sqlite(&self) -> bool {
        self.provider == "sqlite"
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            mongodb: MongoDbConfig::default(),
            sqlite: SqliteConfig::default(),
            pool: PoolConfig::default(),
        }
    }
}

impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            host: default_mongo_host(),
            port: default_mongo_port(),
            database: default_mongo_database(),
            username: None,
            password: None,
            auth_source: None,
            tls: false,
            timeout_seconds: default_timeout(),
        }
    }
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            path: default_sqlite_path(),
            wal_mode: default_true(),
            foreign_keys: default_true(),
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            connection_timeout: default_timeout(),
            idle_timeout: default_idle_timeout(),
        }
    }
}

// Default value functions
fn default_provider() -> String { "mongodb".to_string() }
fn default_mongo_host() -> String { "localhost".to_string() }
fn default_mongo_port() -> u16 { 27017 }
fn default_mongo_database() -> String { "bitacora_db".to_string() }
fn default_sqlite_path() -> String { "./data/bitacora.db".to_string() }
fn default_timeout() -> u64 { 30 }
fn default_idle_timeout() -> u64 { 300 }
fn default_max_connections() -> u32 { 10 }
fn default_min_connections() -> u32 { 1 }
fn default_true() -> bool { true }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_database_config() {
        let config = DatabaseConfig::default();
        assert_eq!(config.provider, "mongodb");
        assert!(config.is_mongodb());
        assert!(!config.is_sqlite());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_mongodb_connection_string() {
        let config = DatabaseConfig {
            provider: "mongodb".to_string(),
            mongodb: MongoDbConfig {
                username: Some("user".to_string()),
                password: Some("pass".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        
        let conn_str = config.mongodb_connection_string();
        assert!(conn_str.starts_with("mongodb://user:pass@localhost:27017"));
        assert!(conn_str.contains("bitacora_db"));
    }

    #[test]
    fn test_sqlite_connection_string() {
        let config = DatabaseConfig {
            provider: "sqlite".to_string(),
            ..Default::default()
        };
        
        let conn_str = config.sqlite_connection_string();
        assert!(conn_str.starts_with("sqlite:"));
        assert!(conn_str.contains("bitacora.db"));
    }

    #[test]
    fn test_invalid_provider() {
        let config = DatabaseConfig {
            provider: "invalid".to_string(),
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }
}
