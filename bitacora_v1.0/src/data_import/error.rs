//! # ❌ Data Import Errors
//!
//! Errores específicos del Data Import Engine

use thiserror::Error;

/// Errores del Data Import Engine
#[derive(Debug, Error)]
pub enum DataImportError {
    #[error("File too large: {size_bytes} bytes (max: {max_bytes})")]
    FileTooLarge {
        size_bytes: usize,
        max_bytes: usize,
    },

    #[error("Invalid encoding: expected {expected}, found {found}")]
    InvalidEncoding {
        expected: String,
        found: String,
    },

    #[error("Quarantine zone not ready for digestion: {0}")]
    QuarantineNotReady(String),

    #[error("Digestion failed for source {data_source}: {reason}")]
    DigestionFailed {
        data_source: String,
        reason: String,
    },

    #[error("Extraction failed for dimension {dimension}: {reason}")]
    ExtractionFailed {
        dimension: String,
        reason: String,
    },

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Distribution failed to {subsystem}: {reason}")]
    DistributionFailed {
        subsystem: String,
        reason: String,
    },

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Template parsing failed: {0}")]
    TemplateParseError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, DataImportError>;
