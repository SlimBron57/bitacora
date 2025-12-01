// bitacora_v1.0/src/watcher/config.rs
//! ⚙️ Watcher Configuration
//!
//! User-configurable settings for watcher behavior

use super::*;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Serializable config (TOML/YAML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatcherConfigFile {
    /// Watched paths (relative to root)
    #[serde(default = "default_watched_paths")]
    pub watched_paths: Vec<String>,
    
    /// Poll interval in seconds
    #[serde(default = "default_poll_interval")]
    pub poll_interval_secs: u64,
    
    /// VoxelDB path
    #[serde(default = "default_voxel_db_path")]
    pub voxel_db_path: String,
    
    /// Show metrics in terminal
    #[serde(default = "default_show_metrics")]
    pub show_metrics: bool,
    
    /// Verbose logging
    #[serde(default)]
    pub verbose: bool,
    
    /// Concepts to ignore
    #[serde(default)]
    pub ignore_concepts: Vec<String>,
    
    /// File patterns to ignore
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
}

fn default_watched_paths() -> Vec<String> {
    vec![
        "ROADMAP_V2".to_string(),
        "BITACORA_KNOWLEDGE_GRAPH".to_string(),
        "src".to_string(),
        "examples".to_string(),
    ]
}

fn default_poll_interval() -> u64 {
    2
}

fn default_voxel_db_path() -> String {
    "data/watcher_voxeldb".to_string()
}

fn default_show_metrics() -> bool {
    true
}

fn default_ignore_patterns() -> Vec<String> {
    vec![
        ".git".to_string(),
        "target".to_string(),
        "00_BACKUPS".to_string(),
        "*.backup".to_string(),
    ]
}

impl Default for WatcherConfigFile {
    fn default() -> Self {
        Self {
            watched_paths: default_watched_paths(),
            poll_interval_secs: default_poll_interval(),
            voxel_db_path: default_voxel_db_path(),
            show_metrics: default_show_metrics(),
            verbose: false,
            ignore_concepts: vec![],
            ignore_patterns: default_ignore_patterns(),
        }
    }
}

impl WatcherConfigFile {
    /// Load config from file (TOML)
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read config file")?;
        
        toml::from_str(&content)
            .context("Failed to parse config file")
    }
    
    /// Save config to file (TOML)
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(path, content)
            .context("Failed to write config file")
    }
    
    /// Convert to runtime config
    pub fn to_runtime_config(&self, root: PathBuf) -> WatcherConfig {
        WatcherConfig {
            root_path: root.clone(),
            watched_paths: self.watched_paths.iter()
                .map(|p| root.join(p))
                .collect(),
            poll_interval: Duration::from_secs(self.poll_interval_secs),
            voxel_db_path: root.join(&self.voxel_db_path),
            show_metrics: self.show_metrics,
            verbose: self.verbose,
            // 🏎️ Use defaults for resource management
            batch_size: 10,
            batch_sleep_ms: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = WatcherConfigFile::default();
        assert!(config.watched_paths.contains(&"ROADMAP_V2".to_string()));
        assert_eq!(config.poll_interval_secs, 2);
        assert!(config.show_metrics);
    }
    
    #[test]
    fn test_to_runtime_config() {
        let file_config = WatcherConfigFile::default();
        let runtime_config = file_config.to_runtime_config(PathBuf::from("/test"));
        
        assert_eq!(runtime_config.root_path, PathBuf::from("/test"));
        assert!(runtime_config.watched_paths.len() >= 2);
    }
}
