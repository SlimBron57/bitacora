use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::{ConfigError, ConfigResult};

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_level")]
    pub level: String,
    
    /// Log format (json, pretty, compact)
    #[serde(default = "default_format")]
    pub format: String,
    
    /// Enable console output
    #[serde(default = "default_true")]
    pub console: bool,
    
    /// File logging configuration
    pub file: Option<FileLoggingConfig>,
    
    /// Structured logging fields
    #[serde(default)]
    pub fields: LoggingFields,
    
    /// Log filtering configuration
    #[serde(default)]
    pub filters: Vec<LogFilter>,
    
    /// Performance logging
    #[serde(default)]
    pub performance: PerformanceLoggingConfig,
}

/// File logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLoggingConfig {
    /// Log file path
    pub path: PathBuf,
    
    /// Enable log rotation
    #[serde(default = "default_true")]
    pub rotate: bool,
    
    /// Maximum file size before rotation (in bytes)
    #[serde(default = "default_max_size")]
    pub max_size: u64,
    
    /// Number of rotated files to keep
    #[serde(default = "default_max_files")]
    pub max_files: u32,
    
    /// Compression for rotated files
    #[serde(default)]
    pub compress: bool,
}

/// Structured logging fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingFields {
    /// Include timestamp
    #[serde(default = "default_true")]
    pub timestamp: bool,
    
    /// Include log level
    #[serde(default = "default_true")]
    pub level: bool,
    
    /// Include module name
    #[serde(default = "default_true")]
    pub module: bool,
    
    /// Include file name and line number
    #[serde(default)]
    pub location: bool,
    
    /// Include thread id
    #[serde(default)]
    pub thread_id: bool,
    
    /// Include request id for HTTP requests
    #[serde(default = "default_true")]
    pub request_id: bool,
    
    /// Include user id for authenticated requests
    #[serde(default = "default_true")]
    pub user_id: bool,
    
    /// Include session id
    #[serde(default = "default_true")]
    pub session_id: bool,
}

/// Log filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogFilter {
    /// Target to filter (module path or crate name)
    pub target: String,
    
    /// Log level for this target
    pub level: String,
}

/// Performance logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceLoggingConfig {
    /// Enable performance logging
    #[serde(default)]
    pub enabled: bool,
    
    /// Log slow requests (threshold in milliseconds)
    #[serde(default = "default_slow_threshold")]
    pub slow_request_threshold_ms: u64,
    
    /// Log database query performance
    #[serde(default)]
    pub database_queries: bool,
    
    /// Log external API calls
    #[serde(default)]
    pub external_calls: bool,
    
    /// Include request/response sizes
    #[serde(default)]
    pub include_sizes: bool,
}

impl LoggingConfig {
    /// Validate logging configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.level.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid log level: {}. Must be one of: {}", 
                    self.level, 
                    valid_levels.join(", "))
            ));
        }
        
        // Validate log format
        let valid_formats = ["json", "pretty", "compact"];
        if !valid_formats.contains(&self.format.as_str()) {
            return Err(ConfigError::ValidationError(
                format!("Invalid log format: {}. Must be one of: {}", 
                    self.format, 
                    valid_formats.join(", "))
            ));
        }
        
        // Validate file config if present
        if let Some(file) = &self.file {
            if file.max_size == 0 {
                return Err(ConfigError::ValidationError("File max size must be greater than 0".to_string()));
            }
            
            if file.max_files == 0 {
                return Err(ConfigError::ValidationError("File max files must be greater than 0".to_string()));
            }
        }
        
        // Validate filters
        for filter in &self.filters {
            if !valid_levels.contains(&filter.level.as_str()) {
                return Err(ConfigError::ValidationError(
                    format!("Invalid filter level: {}. Must be one of: {}", 
                        filter.level, 
                        valid_levels.join(", "))
                ));
            }
            
            if filter.target.is_empty() {
                return Err(ConfigError::ValidationError("Filter target cannot be empty".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Get effective log level as tracing::Level
    pub fn tracing_level(&self) -> Result<tracing::Level, ConfigError> {
        match self.level.as_str() {
            "trace" => Ok(tracing::Level::TRACE),
            "debug" => Ok(tracing::Level::DEBUG),
            "info" => Ok(tracing::Level::INFO),
            "warn" => Ok(tracing::Level::WARN),
            "error" => Ok(tracing::Level::ERROR),
            _ => Err(ConfigError::InvalidValue(format!("Invalid log level: {}", self.level)))
        }
    }
    
    /// Check if file logging is enabled
    pub fn is_file_logging_enabled(&self) -> bool {
        self.file.is_some()
    }
    
    /// Check if performance logging is enabled
    pub fn is_performance_logging_enabled(&self) -> bool {
        self.performance.enabled
    }
    
    /// Get log file path if configured
    pub fn log_file_path(&self) -> Option<&PathBuf> {
        self.file.as_ref().map(|f| &f.path)
    }
    
    /// Check if JSON format is used
    pub fn is_json_format(&self) -> bool {
        self.format == "json"
    }
    
    /// Check if pretty format is used
    pub fn is_pretty_format(&self) -> bool {
        self.format == "pretty"
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_level(),
            format: default_format(),
            console: default_true(),
            file: None,
            fields: LoggingFields::default(),
            filters: vec![],
            performance: PerformanceLoggingConfig::default(),
        }
    }
}

impl Default for LoggingFields {
    fn default() -> Self {
        Self {
            timestamp: default_true(),
            level: default_true(),
            module: default_true(),
            location: false,
            thread_id: false,
            request_id: default_true(),
            user_id: default_true(),
            session_id: default_true(),
        }
    }
}

impl Default for PerformanceLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            slow_request_threshold_ms: default_slow_threshold(),
            database_queries: false,
            external_calls: false,
            include_sizes: false,
        }
    }
}

// Default value functions
fn default_level() -> String { "info".to_string() }
fn default_format() -> String { "pretty".to_string() }
fn default_true() -> bool { true }
fn default_max_size() -> u64 { 10 * 1024 * 1024 } // 10MB
fn default_max_files() -> u32 { 5 }
fn default_slow_threshold() -> u64 { 1000 } // 1 second

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_default_logging_config() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, "pretty");
        assert!(config.console);
        assert!(!config.is_file_logging_enabled());
        assert!(!config.is_performance_logging_enabled());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_tracing_level_conversion() {
        let config = LoggingConfig {
            level: "debug".to_string(),
            ..Default::default()
        };
        
        assert_eq!(config.tracing_level().unwrap(), tracing::Level::DEBUG);
    }

    #[test]
    fn test_invalid_log_level() {
        let config = LoggingConfig {
            level: "invalid".to_string(),
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_log_format() {
        let config = LoggingConfig {
            format: "invalid".to_string(),
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_file_logging_config() {
        let config = LoggingConfig {
            file: Some(FileLoggingConfig {
                path: PathBuf::from("/tmp/bitacora.log"),
                rotate: true,
                max_size: 1024 * 1024,
                max_files: 3,
                compress: true,
            }),
            ..Default::default()
        };
        
        assert!(config.is_file_logging_enabled());
        assert_eq!(config.log_file_path().unwrap(), &PathBuf::from("/tmp/bitacora.log"));
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_log_filters() {
        let config = LoggingConfig {
            filters: vec![
                LogFilter {
                    target: "bitacora_storage".to_string(),
                    level: "debug".to_string(),
                },
                LogFilter {
                    target: "mongodb".to_string(),
                    level: "warn".to_string(),
                },
            ],
            ..Default::default()
        };
        
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_filter_level() {
        let config = LoggingConfig {
            filters: vec![
                LogFilter {
                    target: "test".to_string(),
                    level: "invalid".to_string(),
                },
            ],
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_performance_logging() {
        let config = LoggingConfig {
            performance: PerformanceLoggingConfig {
                enabled: true,
                slow_request_threshold_ms: 500,
                database_queries: true,
                external_calls: true,
                include_sizes: true,
            },
            ..Default::default()
        };
        
        assert!(config.is_performance_logging_enabled());
        assert!(config.validate().is_ok());
    }
}
