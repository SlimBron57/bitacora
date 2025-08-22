//! # Bitacora Core
//! 
//! Tipos de dominio centrales y traits para todo el sistema Bitacora.
//! Este crate contiene:
//! - Domain models (Session, Action, Project, etc.)
//! - Service traits 
//! - Error types
//! - Validation logic
//! - Common utilities

pub mod models;
pub mod traits; 
pub mod errors;
pub mod validators;
pub mod utils;

/// Re-exports comunes para facilitar el uso
pub mod prelude {
    pub use crate::errors::*;
    pub use crate::models::*;
    pub use crate::traits::*;
    
    pub type Result<T> = std::result::Result<T, crate::errors::BitacoraError>;
}
