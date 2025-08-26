//! Session state module

use serde::{Deserialize, Serialize};

/// Represents the state of a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub is_active: bool,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

impl SessionState {
    pub fn new() -> Self {
        Self {
            is_active: false,
            last_activity: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
    
    pub fn active() -> Self {
        Self {
            is_active: true,
            last_activity: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl Default for SessionState {
    fn default() -> Self {
        Self::new()
    }
}
