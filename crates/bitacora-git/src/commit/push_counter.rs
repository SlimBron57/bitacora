use tokio::fs;
use std::path::PathBuf;
use crate::GitError;
use tracing::{debug, error};

/// Manages push counter for auto-push functionality
pub struct PushCounter {
    counter_file_path: PathBuf,
}

impl PushCounter {
    pub fn new(counter_file_path: PathBuf) -> Self {
        Self { counter_file_path }
    }
    
    /// Read current counter value
    pub async fn current_count(&self) -> Result<u32, GitError> {
        match fs::read_to_string(&self.counter_file_path).await {
            Ok(content) => {
                let count = content.trim().parse::<u32>()
                    .map_err(|e| GitError::ParseError { 
                        message: format!("Invalid counter format: {}", e) 
                    })?;
                debug!("Current push counter: {}", count);
                Ok(count)
            }
            Err(_) => {
                // File doesn't exist, start with 0
                debug!("Push counter file not found, starting with 0");
                self.write_count(0).await?;
                Ok(0)
            }
        }
    }
    
    /// Increment counter by 1
    pub async fn increment(&self) -> Result<u32, GitError> {
        let current = self.current_count().await?;
        let new_count = current + 1;
        self.write_count(new_count).await?;
        debug!("Push counter incremented to: {}", new_count);
        Ok(new_count)
    }
    
    /// Reset counter to 0
    pub async fn reset(&self) -> Result<(), GitError> {
        self.write_count(0).await?;
        debug!("Push counter reset to 0");
        Ok(())
    }
    
    /// Set counter to specific value
    pub async fn set_count(&self, count: u32) -> Result<(), GitError> {
        self.write_count(count).await?;
        debug!("Push counter set to: {}", count);
        Ok(())
    }
    
    /// Write count to file
    async fn write_count(&self, count: u32) -> Result<(), GitError> {
        // Ensure parent directory exists
        if let Some(parent) = self.counter_file_path.parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                error!("Failed to create counter directory: {}", e);
                return Err(GitError::Io { source: e });
            }
        }
        
        // Write count to file
        fs::write(&self.counter_file_path, count.to_string()).await
            .map_err(|e| {
                error!("Failed to write push counter: {}", e);
                GitError::Io { source: e }
            })?;
            
        Ok(())
    }
    
    /// Check if threshold is reached
    pub async fn is_threshold_reached(&self, threshold: u32) -> Result<bool, GitError> {
        let current = self.current_count().await?;
        Ok(current >= threshold)
    }
}
