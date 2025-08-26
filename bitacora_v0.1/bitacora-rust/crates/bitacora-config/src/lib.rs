use figment::{Figment, providers::{Format, Toml, Env}};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod database;
pub mod server;
pub mod logging;
pub mod integration;

pub use database::DatabaseConfig;
pub use server::ServerConfig;
pub use logging::LoggingConfig;
pub use integration::IntegrationConfig;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitacoraConfig {
    /// Application environment (development, staging, production)
    #[serde(default = "default_environment")]
    pub environment: String,
    
    /// Database configuration
    pub database: DatabaseConfig,
    
    /// Server configuration
    #[serde(default)]
    pub server: ServerConfig,
    
    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
    
    /// External integrations configuration
    #[serde(default)]
    pub integration: IntegrationConfig,
}

/// Configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    LoadError(#[from] figment::Error),
    
    #[error("Configuration validation failed: {0}")]
    ValidationError(String),
    
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),
    
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
}

pub type ConfigResult<T> = Result<T, ConfigError>;

impl BitacoraConfig {
    /// Load configuration from files and environment variables
    pub fn load() -> ConfigResult<Self> {
        let environment = std::env::var("BITACORA_ENV").unwrap_or_else(|_| "development".to_string());
        
        let config = Figment::new()
            // Start with base config
            .merge(Toml::file("config/base.toml"))
            // Override with environment-specific config
            .merge(Toml::file(format!("config/{}.toml", environment)))
            // Override with environment variables
            .merge(Env::prefixed("BITACORA_"))
            .extract()?;
        
        Self::validate(&config)?;
        
        Ok(config)
    }
    
    /// Load configuration from a specific file
    pub fn from_file<P: AsRef<Path>>(path: P) -> ConfigResult<Self> {
        let config = Figment::new()
            .merge(Toml::file(path.as_ref()))
            .merge(Env::prefixed("BITACORA_"))
            .extract()?;
        
        Self::validate(&config)?;
        
        Ok(config)
    }
    
    /// Validate configuration values
    fn validate(config: &Self) -> ConfigResult<()> {
        // Validate database configuration
        config.database.validate()?;
        
        // Validate server configuration
        config.server.validate()?;
        
        // Validate environment value
        if !["development", "staging", "production", "testing"].contains(&config.environment.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid environment: {}. Must be one of: development, staging, production, testing", 
                    config.environment)
            ));
        }
        
        Ok(())
    }
    
    /// Get database connection string
    pub fn database_url(&self) -> String {
        self.database.connection_string()
    }
    
    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
    
    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
    
    /// Get server bind address
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

impl Default for BitacoraConfig {
    fn default() -> Self {
        Self {
            environment: default_environment(),
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            integration: IntegrationConfig::default(),
        }
    }
}

fn default_environment() -> String {
    "development".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = BitacoraConfig::default();
        assert_eq!(config.environment, "development");
        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_config_from_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
environment = "testing"

[database]
provider = "sqlite"
sqlite_path = "/tmp/test.db"

[server]
port = 8080
host = "127.0.0.1"

[logging]
level = "debug"
"#
        ).unwrap();

        let config = BitacoraConfig::from_file(file.path()).unwrap();
        assert_eq!(config.environment, "testing");
        assert_eq!(config.server.port, 8080);
    }

    #[test]
    fn test_invalid_environment() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"
environment = "invalid"

[database]
provider = "sqlite"
sqlite_path = "/tmp/test.db"
"#
        ).unwrap();

        let result = BitacoraConfig::from_file(file.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid environment"));
    }
}
