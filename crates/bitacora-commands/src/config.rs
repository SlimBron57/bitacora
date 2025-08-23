//! Configuration for the Commands system

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;

/// Configuration for the Commands system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// Enable command history logging
    pub enable_history: bool,
    /// Maximum command history entries to keep
    pub max_history_entries: usize,
    /// Command aliases mapping
    pub aliases: HashMap<String, String>,
    /// Custom command directories to scan
    pub custom_command_dirs: Vec<PathBuf>,
    /// Enable dynamic command loading
    pub enable_dynamic_loading: bool,
    /// Default timeout for command execution (seconds)
    pub default_timeout_seconds: u64,
    /// Enable command validation
    pub enable_validation: bool,
}

impl Default for CommandConfig {
    fn default() -> Self {
        let mut aliases = HashMap::new();
        
        // Default aliases
        aliases.insert("s".to_string(), "status".to_string());
        aliases.insert("st".to_string(), "start".to_string());
        aliases.insert("end".to_string(), "stop".to_string());
        aliases.insert("ls".to_string(), "list".to_string());
        aliases.insert("h".to_string(), "help".to_string());
        aliases.insert("q".to_string(), "quit".to_string());
        
        Self {
            enable_history: true,
            max_history_entries: 1000,
            aliases,
            custom_command_dirs: vec![],
            enable_dynamic_loading: false,
            default_timeout_seconds: 30, // 5 minutes
            enable_validation: true,
        }
    }
}

impl CommandConfig {
    pub fn with_aliases(mut self, aliases: HashMap<String, String>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn with_custom_dirs(mut self, dirs: Vec<PathBuf>) -> Self {
        self.custom_command_dirs = dirs;
        self
    }

    pub fn enable_dynamic_loading(mut self) -> Self {
        self.enable_dynamic_loading = true;
        self
    }

    pub fn set_timeout(mut self, seconds: u64) -> Self {
        self.default_timeout_seconds = seconds;
        self
    }
}
