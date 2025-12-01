//! Error types for Routier Navigator

use std::fmt;
use std::error::Error as StdError;

/// Result type for Routier operations
pub type Result<T> = std::result::Result<T, RoutierError>;

/// Errors that can occur in Routier Navigator
#[derive(Debug)]
pub enum RoutierError {
    /// Graph construction failed (cycles detected, invalid structure)
    GraphConstruction(String),
    
    /// Step not found in learning graph
    StepNotFound(String),
    
    /// Prerequisites not met for step
    PrerequisitesNotMet {
        step_id: String,
        missing: Vec<String>,
    },
    
    /// No available next steps (end of path or blocked)
    NoAvailableSteps,
    
    /// Invalid cognitive state data
    InvalidCognitiveState(String),
    
    /// Database operation failed
    DatabaseError(String),
    
    /// Serialization/deserialization failed
    SerializationError(String),
    
    /// Configuration invalid
    InvalidConfiguration(String),
}

impl fmt::Display for RoutierError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GraphConstruction(msg) => {
                write!(f, "Failed to construct learning graph: {}", msg)
            }
            Self::StepNotFound(id) => {
                write!(f, "Learning step '{}' not found in graph", id)
            }
            Self::PrerequisitesNotMet { step_id, missing } => {
                write!(
                    f,
                    "Prerequisites not met for step '{}': {:?}",
                    step_id, missing
                )
            }
            Self::NoAvailableSteps => {
                write!(f, "No available next steps (end of path or all blocked)")
            }
            Self::InvalidCognitiveState(msg) => {
                write!(f, "Invalid cognitive state: {}", msg)
            }
            Self::DatabaseError(msg) => {
                write!(f, "Database operation failed: {}", msg)
            }
            Self::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            Self::InvalidConfiguration(msg) => {
                write!(f, "Invalid configuration: {}", msg)
            }
        }
    }
}

impl StdError for RoutierError {}

// Conversions from other error types

impl From<serde_json::Error> for RoutierError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for RoutierError {
    fn from(err: std::io::Error) -> Self {
        Self::DatabaseError(err.to_string())
    }
}
