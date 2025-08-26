//! Error types for Session Management Service

use thiserror::Error;

/// Error types for session management operations
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: String },

    #[error("Session already active: {session_id}")]
    SessionAlreadyActive { session_id: String },

    #[error("Session not active: {session_id}")]
    SessionNotActive { session_id: String },

    #[error("Invalid state transition from {current} to {target}")]
    InvalidStateTransition { current: String, target: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Persistence error: {message}")]
    PersistenceError { message: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    #[error("Service error: {message}")]
    ServiceError { message: String },
}

impl SessionError {
    pub fn session_not_found(session_id: String) -> Self {
        Self::SessionNotFound { session_id }
    }

    pub fn session_already_active(session_id: String) -> Self {
        Self::SessionAlreadyActive { session_id }
    }

    pub fn session_not_active(session_id: String) -> Self {
        Self::SessionNotActive { session_id }
    }

    pub fn invalid_state_transition(current: String, target: String) -> Self {
        Self::InvalidStateTransition { current, target }
    }

    pub fn configuration_error(message: &str) -> Self {
        Self::ConfigurationError { message: message.to_string() }
    }

    pub fn persistence_error(message: &str) -> Self {
        Self::PersistenceError { message: message.to_string() }
    }

    pub fn serialization_error(message: &str) -> Self {
        Self::SerializationError { message: message.to_string() }
    }

    pub fn service_error(message: &str) -> Self {
        Self::ServiceError { message: message.to_string() }
    }
}
