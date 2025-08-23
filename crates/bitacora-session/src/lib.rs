//! # Bitacora Session Management
//! 
//! This crate provides session management functionality for the Bitacora development workflow system.
//! It offers a simplified implementation focusing on core session lifecycle management.

pub mod config;
pub mod errors;
pub mod service;

// Re-exports for easy access
pub use config::SessionConfig;
pub use errors::SessionError;
pub use service::{SessionService, SessionServiceImpl};
