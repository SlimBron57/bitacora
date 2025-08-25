//! Bitacora Records - Business Logic Layer
//! 
//! This crate contains the business logic for managing sessions, actions,
//! topics, sparks and their relationships.

pub mod services;

// Re-export services
pub use services::*;

// Re-export core models that we work with
pub use bitacora_core::models::*;
pub use bitacora_core::errors::*;
