//! Tipos de error para FlowPacks Anti-Disco-Rayado

use thiserror::Error;

/// Errores del FlowPack Engine (Contextual Compression)
#[derive(Error, Debug)]
pub enum FlowPackError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    
    #[error("Similarity search failed: {0}")]
    SimilaritySearchFailed(String),
    
    #[error("Embedding generation failed: {0}")]
    EmbeddingFailed(String),
    
    #[error("FlowPack not found: {0}")]
    PackNotFound(String),
    
    #[error("Empty FlowPack (no entries)")]
    EmptyPack,
    
    #[error("FlowPack is full: {size} entries (max: {max})")]
    PackFull { size: usize, max: usize },
    
    #[error("Invalid similarity threshold: {0} (must be 0.0-1.0)")]
    InvalidThreshold(f64),
    
    #[error("HNSW index error: {0}")]
    HnswIndexError(String),
    
    #[error("Embedding model not loaded")]
    EmbeddingModelNotLoaded,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("FBCU error: {0}")]
    FBCUError(String),
    
    #[error("TelescopeDB error: {0}")]
    TelescopeDBError(String),
}

pub type Result<T> = std::result::Result<T, FlowPackError>;
