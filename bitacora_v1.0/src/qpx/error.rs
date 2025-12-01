//! QPX Error Types

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QPXError {
    #[error("Invalid QPX header: {0}")]
    InvalidHeader(String),
    
    #[error("Unsupported QPX version: {0:#x} (expected {1:#x})")]
    UnsupportedVersion(u16, u16),
    
    #[error("Invalid pixel data: {0}")]
    InvalidPixel(String),
    
    #[error("Encoding failed: {0}")]
    EncodingFailed(String),
    
    #[error("Decoding failed: {0}")]
    DecodingFailed(String),
    
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch {
        expected: String,
        actual: String,
    },
    
    #[error("Invalid offset: {0}")]
    InvalidOffset(String),
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, QPXError>;
