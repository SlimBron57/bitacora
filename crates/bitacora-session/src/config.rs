//! Configuration for Session Management Service

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the Session Management Service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Base directory for session storage
    pub storage_path: PathBuf,
    /// Maximum number of concurrent active sessions
    pub max_active_sessions: u32,
    /// Enable automatic session state persistence
    pub auto_persist: bool,
    /// Session timeout in minutes (0 = no timeout)
    pub session_timeout_minutes: u32,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./sessions"),
            max_active_sessions: 5,
            auto_persist: true,
            session_timeout_minutes: 0,
        }
    }
}
