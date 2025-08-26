//! # Bitacora Session Management
//! 
//! This crate provides session management functionality for the Bitacora development workflow system.
//! It offers a simplified implementation focusing on core session lifecycle management.

pub mod config;
pub mod context;
pub mod errors;
pub mod events;
pub mod service;
pub mod state;

// Re-exports for easy access
pub use config::SessionConfig;
pub use context::SessionContext;
pub use errors::SessionError;
pub use events::SessionEvent;
// pub use service::{SessionService, SessionServiceImpl}; // TODO: Implement when needed
pub use state::SessionState;
