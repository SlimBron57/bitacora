//! Error types para Bitacora

use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitacoraError {
    #[error("Domain error: {0}")]
    Domain(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
