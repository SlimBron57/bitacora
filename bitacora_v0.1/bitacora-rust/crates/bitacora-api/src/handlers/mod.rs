//! API handlers for Bitacora REST endpoints

pub mod projects;
pub mod topics;
pub mod actions;
pub mod sparks;
pub mod health;
pub mod middleware;

// Re-exports for convenience
pub use projects::*;
pub use topics::*;
pub use actions::*;
pub use sparks::*;
pub use health::*;
