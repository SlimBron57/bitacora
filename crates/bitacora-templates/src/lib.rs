//! Bitacora Templates - Dynamic response template system
//! 
//! This crate provides a flexible template system for rendering structured responses
//! to user actions in the Bitacora development tracking system.

pub mod models;
pub mod services;
pub mod engines;
pub mod error;

// Re-exports
pub use models::*;
pub use services::*;
pub use engines::*;
pub use error::*;
