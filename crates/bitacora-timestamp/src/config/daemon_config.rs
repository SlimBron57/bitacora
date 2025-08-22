//! Daemon Configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use bitacora_core::errors::BitacoraError;

/// Daemon configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Timestamp file path
    pub timestamp_file: String,
    
    /// Timestamp format pattern
    pub timestamp_format: String,
    
    /// Update interval in seconds
    pub update_interval: u64,
    
    /// Health check interval in seconds
    pub check_interval: u64,
    
    /// Maximum age of timestamp file in seconds
    pub max_timestamp_age: u64,
    
    /// Command to execute for timestamp generation
    pub command: String,
    
    /// Maximum history entries to keep
    pub max_history_entries: usize,
    
    /// Working directory for daemon
    pub working_directory: String,
    
    /// PID file path
    pub pid_file: Option<String>,
    
    /// Log file path
    pub log_file: Option<String>,
    
    /// Enable daemon mode
    pub daemon_mode: bool,
}

impl DaemonConfig {
    /// Create new daemon configuration with defaults
    pub fn new() -> Self {
        Self {
            timestamp_file: "timestamp/timestamp.txt".to_string(),
            timestamp_format: "TIMESTAMP".to_string(), // YYYYMMDD-HHMM
            update_interval: 60, // 1 minute
            check_interval: 30,  // 30 seconds
            max_timestamp_age: 300, // 5 minutes
            command: "date +%Y%m%d-%H%M".to_string(),
            max_history_entries: 1000,
            working_directory: ".".to_string(),
            pid_file: Some("bitacora_timestamp.pid".to_string()),
            log_file: Some("bitacora_timestamp.log".to_string()),
            daemon_mode: false,
        }
    }
    
    /// Create from bitacora config
    pub fn from_bitacora_config(config: &bitacora_config::BitacoraConfig) -> Self {
        let mut daemon_config = Self::new();
        
        // Extract relevant configuration from BitacoraConfig
        if let Some(workspace_path) = &config.workspace.path {
            daemon_config.timestamp_file = format!("{}/timestamp/timestamp.txt", workspace_path);
            daemon_config.working_directory = workspace_path.clone();
        }
        
        // Override with any specific timestamp settings
        // TODO: Add timestamp-specific config fields to BitacoraConfig if needed
        
        daemon_config
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), BitacoraError> {
        // Validate timestamp file path
        if self.timestamp_file.is_empty() {
            return Err(BitacoraError::ValidationError("Timestamp file path cannot be empty".to_string()));
        }
        
        // Validate format
        if self.timestamp_format.is_empty() {
            return Err(BitacoraError::ValidationError("Timestamp format cannot be empty".to_string()));
        }
        
        // Validate intervals
        if self.update_interval == 0 {
            return Err(BitacoraError::ValidationError("Update interval must be greater than 0".to_string()));
        }
        
        if self.check_interval == 0 {
            return Err(BitacoraError::ValidationError("Check interval must be greater than 0".to_string()));
        }
        
        // Validate working directory exists
        let work_dir = PathBuf::from(&self.working_directory);
        if !work_dir.exists() {
            return Err(BitacoraError::ValidationError(
                format!("Working directory does not exist: {}", self.working_directory)
            ));
        }
        
        Ok(())
    }
    
    /// Get absolute timestamp file path
    pub fn get_timestamp_file_path(&self) -> PathBuf {
        let path = PathBuf::from(&self.timestamp_file);
        
        if path.is_absolute() {
            path
        } else {
            PathBuf::from(&self.working_directory).join(path)
        }
    }
    
    /// Get absolute PID file path
    pub fn get_pid_file_path(&self) -> Option<PathBuf> {
        self.pid_file.as_ref().map(|pid_file| {
            let path = PathBuf::from(pid_file);
            
            if path.is_absolute() {
                path
            } else {
                PathBuf::from(&self.working_directory).join(path)
            }
        })
    }
    
    /// Get absolute log file path
    pub fn get_log_file_path(&self) -> Option<PathBuf> {
        self.log_file.as_ref().map(|log_file| {
            let path = PathBuf::from(log_file);
            
            if path.is_absolute() {
                path
            } else {
                PathBuf::from(&self.working_directory).join(path)
            }
        })
    }
    
    /// Set timestamp file path
    pub fn with_timestamp_file<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.timestamp_file = path.as_ref().to_string_lossy().to_string();
        self
    }
    
    /// Set timestamp format
    pub fn with_format(mut self, format: &str) -> Self {
        self.timestamp_format = format.to_string();
        self
    }
    
    /// Set update interval
    pub fn with_update_interval(mut self, interval: u64) -> Self {
        self.update_interval = interval;
        self
    }
    
    /// Set working directory
    pub fn with_working_directory<P: AsRef<std::path::Path>>(mut self, path: P) -> Self {
        self.working_directory = path.as_ref().to_string_lossy().to_string();
        self
    }
    
    /// Enable daemon mode
    pub fn with_daemon_mode(mut self, enabled: bool) -> Self {
        self.daemon_mode = enabled;
        self
    }
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_daemon_config_validation() {
        let temp_dir = TempDir::new().unwrap();
        
        let mut config = DaemonConfig::new();
        config.working_directory = temp_dir.path().to_string_lossy().to_string();
        
        assert!(config.validate().is_ok());
        
        // Test invalid config
        config.update_interval = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_path_resolution() {
        let temp_dir = TempDir::new().unwrap();
        
        let config = DaemonConfig::new()
            .with_working_directory(temp_dir.path())
            .with_timestamp_file("timestamp.txt");
        
        let abs_path = config.get_timestamp_file_path();
        assert!(abs_path.is_absolute());
        assert!(abs_path.ends_with("timestamp.txt"));
    }
}
