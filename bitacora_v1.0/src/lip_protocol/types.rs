// LIP Protocol - Types
// Tipos fundamentales del sistema LIP

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Trait que define el contrato de un Lens
pub trait LensInterface: Send + Sync {
    /// Identificador único del lens
    fn lens_id(&self) -> &str;
    
    /// Versión del lens
    fn version(&self) -> &str;
    
    /// Requisitos que el lens necesita del FBCU
    fn requires(&self) -> LensRequirements;
    
    /// Campos que el lens provee en su output
    fn provides(&self) -> Vec<String>;
    
    /// Quality bounds que el lens garantiza
    fn quality_bounds(&self) -> QualityBounds;
    
    /// Procesar FBCU data con este lens
    fn process(&self, fbcu_data: &HashMap<String, serde_json::Value>) 
        -> Result<LensOutput, LensError>;
}

/// Requisitos de un lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensRequirements {
    /// Campos requeridos del FBCU
    pub fields: Vec<String>,
    
    /// Embedding requerido (opcional)
    pub embedding: Option<EmbeddingRequirement>,
    
    /// Dimensiones contextuales requeridas
    pub context_dimensions: Vec<String>,
}

/// Requisito de embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequirement {
    pub model: String,
    pub dimension: usize,
}

/// Bounds de calidad garantizados
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityBounds {
    pub coherence_min: f64,
    pub lens_agreement_min: f64,
    pub confidence_min: f64,
}

impl Default for QualityBounds {
    fn default() -> Self {
        Self {
            coherence_min: 0.70,
            lens_agreement_min: 0.75,
            confidence_min: 0.65,
        }
    }
}

/// Métricas de calidad de output
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub coherence: f64,
    pub lens_agreement: f64,
    pub processing_confidence: f64,
}

impl QualityMetrics {
    pub fn new(coherence: f64, lens_agreement: f64, processing_confidence: f64) -> Self {
        Self {
            coherence,
            lens_agreement,
            processing_confidence,
        }
    }
}

/// Estado de validación
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationStatus {
    Passed,
    Failed(String),
    PartialSuccess(String),
}

/// Output de un lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensOutput {
    pub lens_id: String,
    pub version: String,
    pub fbcu_id: String,
    pub data: HashMap<String, serde_json::Value>,
    pub quality_metrics: QualityMetrics,
    pub validation_status: ValidationStatus,
    pub processed_at: DateTime<Utc>,
}

impl LensOutput {
    pub fn new(
        lens_id: String,
        version: String,
        fbcu_id: String,
        data: HashMap<String, serde_json::Value>,
        quality_metrics: QualityMetrics,
    ) -> Self {
        Self {
            lens_id,
            version,
            fbcu_id,
            data,
            quality_metrics,
            validation_status: ValidationStatus::Passed,
            processed_at: Utc::now(),
        }
    }
}

/// Errores del sistema LIP
#[derive(Debug, thiserror::Error)]
pub enum LensError {
    #[error("Lens not found: {0}")]
    LensNotFound(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Embedding dimension mismatch: expected {expected}, found {found}")]
    EmbeddingDimensionMismatch { expected: usize, found: usize },
    
    #[error("Quality bound violation: {metric} = {value}, minimum = {minimum}")]
    QualityBoundViolation { metric: String, value: f64, minimum: f64 },
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quality_bounds_default() {
        let bounds = QualityBounds::default();
        assert_eq!(bounds.coherence_min, 0.70);
        assert_eq!(bounds.lens_agreement_min, 0.75);
        assert_eq!(bounds.confidence_min, 0.65);
    }
    
    #[test]
    fn test_quality_metrics_creation() {
        let metrics = QualityMetrics::new(0.85, 0.90, 0.88);
        assert_eq!(metrics.coherence, 0.85);
        assert_eq!(metrics.lens_agreement, 0.90);
        assert_eq!(metrics.processing_confidence, 0.88);
    }
    
    #[test]
    fn test_lens_output_creation() {
        let mut data = HashMap::new();
        data.insert("test".to_string(), serde_json::json!("value"));
        
        let metrics = QualityMetrics::new(0.8, 0.85, 0.9);
        let output = LensOutput::new(
            "test_lens".to_string(),
            "1.0.0".to_string(),
            "fbcu_123".to_string(),
            data,
            metrics,
        );
        
        assert_eq!(output.lens_id, "test_lens");
        assert_eq!(output.version, "1.0.0");
        assert_eq!(output.validation_status, ValidationStatus::Passed);
    }
    
    #[test]
    fn test_validation_status_equality() {
        assert_eq!(ValidationStatus::Passed, ValidationStatus::Passed);
        assert_ne!(ValidationStatus::Passed, ValidationStatus::Failed("error".to_string()));
    }
}
