//! Session context module

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Context information for a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: Uuid,
    pub working_directory: Option<String>,
    pub environment_variables: std::collections::HashMap<String, String>,
    pub tags: Vec<String>,
}

impl SessionContext {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            working_directory: None,
            environment_variables: std::collections::HashMap::new(),
            tags: Vec::new(),
        }
    }
}
