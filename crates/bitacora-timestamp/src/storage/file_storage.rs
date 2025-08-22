//! File Storage - File-based storage (compatibility)

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, error, info};

use bitacora_core::errors::BitacoraError;
use crate::config::DaemonConfig;
use crate::storage::TimestampStorage;
use crate::TimestampResult;

/// File-based timestamp storage implementation
pub struct FileStorage {
    file_path: PathBuf,
    history_path: PathBuf,
    max_history: usize,
}

impl FileStorage {
    /// Create new file storage
    pub fn new(config: &DaemonConfig) -> Self {
        let file_path = PathBuf::from(&config.timestamp_file);
        let history_path = file_path.with_extension("history");
        
        Self {
            file_path,
            history_path,
            max_history: config.max_history_entries,
        }
    }
    
    /// Create file storage with custom path
    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        let file_path = PathBuf::from(path.as_ref());
        let history_path = file_path.with_extension("history");
        
        Self {
            file_path,
            history_path,
            max_history: 1000, // Default
        }
    }
    
    /// Ensure parent directory exists
    async fn ensure_directory(&self) -> TimestampResult<()> {
        if let Some(parent) = self.file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await
                    .map_err(|e| BitacoraError::OperationError(format!("Failed to create directory: {}", e)))?;
                debug!("Created directory: {:?}", parent);
            }
        }
        Ok(())
    }
    
    /// Append to history file
    async fn append_to_history(&self, timestamp: &str) -> TimestampResult<()> {
        self.ensure_directory().await?;
        
        // Create history entry with current time
        let now = chrono::Utc::now();
        let history_entry = format!("{}: {}\n", now.format("%Y-%m-%d %H:%M:%S UTC"), timestamp);
        
        // Append to history file
        match fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.history_path)
            .await
        {
            Ok(mut file) => {
                use tokio::io::AsyncWriteExt;
                
                file.write_all(history_entry.as_bytes()).await
                    .map_err(|e| BitacoraError::OperationError(format!("Failed to write history: {}", e)))?;
                
                debug!("Appended to history: {}", timestamp);
                
                // Trim history if too large
                self.trim_history().await?;
                
                Ok(())
            }
            Err(e) => {
                error!("Failed to open history file: {}", e);
                Err(BitacoraError::OperationError(format!("Failed to open history file: {}", e)))
            }
        }
    }
    
    /// Trim history file to max_history entries
    async fn trim_history(&self) -> TimestampResult<()> {
        if !self.history_path.exists() {
            return Ok(());
        }
        
        let content = fs::read_to_string(&self.history_path).await
            .map_err(|e| BitacoraError::OperationError(format!("Failed to read history: {}", e)))?;
        
        let lines: Vec<&str> = content.lines().collect();
        
        if lines.len() > self.max_history {
            let keep_lines: Vec<&str> = lines.into_iter()
                .skip(lines.len() - self.max_history)
                .collect();
            
            let new_content = keep_lines.join("\n") + "\n";
            
            fs::write(&self.history_path, new_content).await
                .map_err(|e| BitacoraError::OperationError(format!("Failed to trim history: {}", e)))?;
            
            debug!("Trimmed history to {} entries", self.max_history);
        }
        
        Ok(())
    }
    
    /// Read history entries
    async fn read_history_entries(&self) -> TimestampResult<Vec<String>> {
        if !self.history_path.exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&self.history_path).await
            .map_err(|e| BitacoraError::OperationError(format!("Failed to read history: {}", e)))?;
        
        let entries: Vec<String> = content.lines()
            .filter_map(|line| {
                // Parse format: "2025-08-22 14:30:45 UTC: 20250822-1430"
                if let Some(pos) = line.find(": ") {
                    Some(line[pos + 2..].to_string())
                } else {
                    None
                }
            })
            .collect();
        
        Ok(entries)
    }
    
    /// Get file path
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }
    
    /// Get history path
    pub fn history_path(&self) -> &Path {
        &self.history_path
    }
}

#[async_trait]
impl TimestampStorage for FileStorage {
    async fn store(&self, timestamp: &str) -> TimestampResult<()> {
        debug!("Storing timestamp to file: {}", timestamp);
        
        // Ensure directory exists
        self.ensure_directory().await?;
        
        // Write to main timestamp file
        fs::write(&self.file_path, timestamp).await
            .map_err(|e| BitacoraError::OperationError(format!("Failed to write timestamp file: {}", e)))?;
        
        // Append to history
        self.append_to_history(timestamp).await?;
        
        info!("Timestamp stored: {}", timestamp);
        Ok(())
    }
    
    async fn get_latest(&self) -> TimestampResult<Option<String>> {
        if !self.file_path.exists() {
            debug!("Timestamp file does not exist");
            return Ok(None);
        }
        
        let content = fs::read_to_string(&self.file_path).await
            .map_err(|e| BitacoraError::OperationError(format!("Failed to read timestamp file: {}", e)))?;
        
        let timestamp = content.trim().to_string();
        
        if timestamp.is_empty() {
            Ok(None)
        } else {
            debug!("Retrieved latest timestamp: {}", timestamp);
            Ok(Some(timestamp))
        }
    }
    
    async fn get_history(&self, limit: usize) -> TimestampResult<Vec<String>> {
        let mut entries = self.read_history_entries().await?;
        
        // Return most recent entries
        entries.reverse(); // Most recent first
        
        if limit > 0 && entries.len() > limit {
            entries.truncate(limit);
        }
        
        debug!("Retrieved {} history entries", entries.len());
        Ok(entries)
    }
    
    async fn clear(&self) -> TimestampResult<()> {
        info!("Clearing timestamp storage");
        
        // Remove main file
        if self.file_path.exists() {
            fs::remove_file(&self.file_path).await
                .map_err(|e| BitacoraError::OperationError(format!("Failed to remove timestamp file: {}", e)))?;
        }
        
        // Remove history file
        if self.history_path.exists() {
            fs::remove_file(&self.history_path).await
                .map_err(|e| BitacoraError::OperationError(format!("Failed to remove history file: {}", e)))?;
        }
        
        info!("Timestamp storage cleared");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_file_storage() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("timestamp.txt");
        
        let storage = FileStorage::with_path(&file_path);
        
        // Test store
        storage.store("20250822-1430").await.unwrap();
        
        // Test get_latest
        let latest = storage.get_latest().await.unwrap();
        assert_eq!(latest, Some("20250822-1430".to_string()));
        
        // Test history
        storage.store("20250822-1431").await.unwrap();
        let history = storage.get_history(10).await.unwrap();
        assert_eq!(history.len(), 2);
        
        // Test clear
        storage.clear().await.unwrap();
        let latest = storage.get_latest().await.unwrap();
        assert_eq!(latest, None);
    }
}
