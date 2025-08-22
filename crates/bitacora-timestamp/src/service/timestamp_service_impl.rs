//! Timestamp Service Implementation

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::path::Path;
use tokio::fs;
use tracing::{info, error, debug, warn};

use bitacora_core::errors::BitacoraError;
use crate::config::DaemonConfig;
use crate::storage::{TimestampStorage, FileStorage};
use crate::service::FormatParser;
use crate::TimestampResult;

/// Timestamp service trait
#[async_trait]
pub trait TimestampService {
    /// Generate a new timestamp
    async fn generate_timestamp(&self) -> TimestampResult<String>;
    
    /// Get current timestamp formatted
    async fn get_current_timestamp(&self) -> TimestampResult<String>;
    
    /// Parse timestamp format
    async fn parse_format(&self, format: &str) -> TimestampResult<String>;
    
    /// Validate timestamp format
    async fn validate_timestamp(&self, timestamp: &str) -> TimestampResult<bool>;
    
    /// Store timestamp
    async fn store_timestamp(&self, timestamp: &str) -> TimestampResult<()>;
}

/// Timestamp Service Implementation
pub struct TimestampServiceImpl {
    config: DaemonConfig,
    storage: Box<dyn TimestampStorage + Send + Sync>,
    format_parser: FormatParser,
}

impl TimestampServiceImpl {
    /// Create new timestamp service
    pub fn new(config: DaemonConfig) -> Self {
        let storage = Box::new(FileStorage::new(&config));
        let format_parser = FormatParser::new();
        
        Self {
            config,
            storage,
            format_parser,
        }
    }
    
    /// Create timestamp service with custom storage
    pub fn with_storage(
        config: DaemonConfig, 
        storage: Box<dyn TimestampStorage + Send + Sync>
    ) -> Self {
        Self {
            config,
            storage,
            format_parser: FormatParser::new(),
        }
    }
    
    /// Generate timestamp in specified format
    pub async fn generate_formatted_timestamp(&self, format: &str) -> TimestampResult<String> {
        let now = Utc::now();
        self.format_parser.format_timestamp(&now, format).await
    }
    
    /// Get timestamp for specific date
    pub async fn get_timestamp_for_date(&self, date: DateTime<Utc>) -> TimestampResult<String> {
        self.format_parser.format_timestamp(&date, &self.config.timestamp_format).await
    }
    
    /// Update timestamp file with current time
    async fn update_timestamp_file(&self) -> TimestampResult<()> {
        let timestamp = self.generate_timestamp().await?;
        
        // Store using storage layer
        self.storage.store(&timestamp).await?;
        
        // Also update the legacy file for backward compatibility
        if let Some(parent) = Path::new(&self.config.timestamp_file).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .map_err(|e| BitacoraError::OperationError(format!("Failed to create directory: {}", e)))?;
            }
        }
        
        fs::write(&self.config.timestamp_file, &timestamp).await
            .map_err(|e| BitacoraError::OperationError(format!("Failed to write timestamp file: {}", e)))?;
        
        debug!("Timestamp updated: {}", timestamp);
        Ok(())
    }
    
    /// Start continuous timestamp updates
    pub async fn start_continuous_updates(&self) -> TimestampResult<()> {
        info!("Starting continuous timestamp updates");
        
        let mut interval = tokio::time::interval(
            std::time::Duration::from_secs(self.config.update_interval)
        );
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.update_timestamp_file().await {
                error!("Failed to update timestamp: {}", e);
                // Continue running despite errors
            }
        }
    }
}

#[async_trait]
impl TimestampService for TimestampServiceImpl {
    async fn generate_timestamp(&self) -> TimestampResult<String> {
        let now = Utc::now();
        self.format_parser.format_timestamp(&now, &self.config.timestamp_format).await
    }
    
    async fn get_current_timestamp(&self) -> TimestampResult<String> {
        self.generate_timestamp().await
    }
    
    async fn parse_format(&self, format: &str) -> TimestampResult<String> {
        let now = Utc::now();
        self.format_parser.format_timestamp(&now, format).await
    }
    
    async fn validate_timestamp(&self, timestamp: &str) -> TimestampResult<bool> {
        self.format_parser.validate_timestamp(timestamp, &self.config.timestamp_format).await
    }
    
    async fn store_timestamp(&self, timestamp: &str) -> TimestampResult<()> {
        self.storage.store(timestamp).await
    }
}
