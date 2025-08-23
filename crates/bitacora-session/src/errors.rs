//! Error types for Session Management Service

use thiserror::Error;

/// Error types for session management operations
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Session already active: {0}")]
    SessionAlreadyActive(String),

    #[error("Session not active: {0}")]
    SessionNotActive(String),

    #[error("Invalid state transition from {0} to {1}")]
    InvalidStateTransition(String, String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Persistence error: {0}")]
    PersistenceError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Service error: {0}")]
    ServiceError(String),
}

impl SessionError {
    pub fn session_not_found(id: String) -> Self {
        Self::SessionNotFound(id)
    }

    pub fn session_already_active(id: String) -> Self {
        Self::SessionAlreadyActive(id)
    }

    pub fn session_not_active(id: String) -> Self {
        Self::SessionNotActive(id)
    }

    pub fn invalid_state_transition(from: String, to: String) -> Self {
        Self::InvalidStateTransition(from, to)
    }

    pub fn configuration_error(msg: &str) -> Self {
        Self::ConfigurationError(msg.to_string())
    }

    pub fn persistence_error(msg: &str) -> Self {
        Self::PersistenceError(msg.to_string())
    }

    pub fn serialization_error(msg: &str) -> Self {
        Self::SerializationError(msg.to_string())
    }

    pub fn service_error(msg: &str) -> Self {
        Self::ServiceError(msg.to_string())
    }
}
