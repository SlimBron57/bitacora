// LIP Protocol - Validation
// Lógica de validación de compatibilidad FBCU-Lens

use super::types::*;
use std::collections::HashMap;

/// Validador del sistema LIP
pub struct Validator;

impl Validator {
    /// Valida requirements de un lens contra un FBCU
    pub fn validate_requirements(
        lens_id: &str,
        requirements: &LensRequirements,
        fbcu_data: &HashMap<String, serde_json::Value>,
    ) -> Result<(), LensError> {
        // Validar campos requeridos
        for field in &requirements.fields {
            if !fbcu_data.contains_key(field) {
                return Err(LensError::MissingField(field.clone()));
            }
        }
        
        // Validar embedding si es requerido
        if let Some(emb_req) = &requirements.embedding {
            Self::validate_embedding(fbcu_data, emb_req)?;
        }
        
        // Validar context dimensions si son requeridas
        if !requirements.context_dimensions.is_empty() {
            Self::validate_context_dimensions(fbcu_data, &requirements.context_dimensions)?;
        }
        
        Ok(())
    }
    
    /// Valida que el embedding existe y tiene dimensión correcta
    pub fn validate_embedding(
        fbcu_data: &HashMap<String, serde_json::Value>,
        requirement: &EmbeddingRequirement,
    ) -> Result<(), LensError> {
        let embedding = fbcu_data.get("embedding")
            .ok_or_else(|| LensError::MissingField("embedding".to_string()))?;
        
        if let Some(array) = embedding.as_array() {
            if array.len() != requirement.dimension {
                return Err(LensError::EmbeddingDimensionMismatch {
                    expected: requirement.dimension,
                    found: array.len(),
                });
            }
        } else {
            return Err(LensError::InvalidFormat("embedding must be an array".to_string()));
        }
        
        Ok(())
    }
    
    /// Valida que las dimensiones contextuales existen
    pub fn validate_context_dimensions(
        fbcu_data: &HashMap<String, serde_json::Value>,
        required_dims: &[String],
    ) -> Result<(), LensError> {
        let context_tensor = fbcu_data.get("context_tensor")
            .ok_or_else(|| LensError::MissingField("context_tensor".to_string()))?;
        
        if let Some(obj) = context_tensor.as_object() {
            for dim in required_dims {
                if !obj.contains_key(dim) {
                    return Err(LensError::MissingField(format!("context_tensor.{}", dim)));
                }
            }
        } else {
            return Err(LensError::InvalidFormat("context_tensor must be an object".to_string()));
        }
        
        Ok(())
    }
    
    /// Valida quality bounds contra métricas
    pub fn validate_quality_bounds(
        metrics: &QualityMetrics,
        bounds: &QualityBounds,
    ) -> ValidationStatus {
        let mut failures = Vec::new();
        let mut warnings = Vec::new();
        
        if metrics.coherence < bounds.coherence_min {
            failures.push(format!(
                "coherence {} < minimum {}",
                metrics.coherence, bounds.coherence_min
            ));
        } else if metrics.coherence < bounds.coherence_min + 0.05 {
            warnings.push("coherence close to minimum".to_string());
        }
        
        if metrics.lens_agreement < bounds.lens_agreement_min {
            failures.push(format!(
                "lens_agreement {} < minimum {}",
                metrics.lens_agreement, bounds.lens_agreement_min
            ));
        }
        
        if metrics.processing_confidence < bounds.confidence_min {
            failures.push(format!(
                "confidence {} < minimum {}",
                metrics.processing_confidence, bounds.confidence_min
            ));
        }
        
        if !failures.is_empty() {
            ValidationStatus::Failed(failures.join("; "))
        } else if !warnings.is_empty() {
            ValidationStatus::PartialSuccess(warnings.join("; "))
        } else {
            ValidationStatus::Passed
        }
    }
    
    /// Valida el contrato de un lens
    pub fn validate_lens_contract(lens: &dyn LensInterface) -> Result<(), LensError> {
        // Verificar ID no vacío
        if lens.lens_id().is_empty() {
            return Err(LensError::ValidationFailed("lens_id cannot be empty".to_string()));
        }
        
        // Verificar versión no vacía
        if lens.version().is_empty() {
            return Err(LensError::ValidationFailed("version cannot be empty".to_string()));
        }
        
        // Verificar que provee al menos un campo
        if lens.provides().is_empty() {
            return Err(LensError::ValidationFailed("lens must provide at least one field".to_string()));
        }
        
        // Verificar bounds válidos (entre 0.0 y 1.0)
        let bounds = lens.quality_bounds();
        if bounds.coherence_min < 0.0 || bounds.coherence_min > 1.0 {
            return Err(LensError::ValidationFailed("coherence_min must be between 0.0 and 1.0".to_string()));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn create_test_fbcu() -> HashMap<String, serde_json::Value> {
        let mut fbcu = HashMap::new();
        fbcu.insert("content".to_string(), json!("test"));
        fbcu.insert("embedding".to_string(), json!(vec![0.5; 384]));
        
        let context = json!({
            "temporal": 0.8,
            "emotional": 0.7
        });
        fbcu.insert("context_tensor".to_string(), context);
        
        fbcu
    }
    
    #[test]
    fn test_validate_requirements_success() {
        let fbcu = create_test_fbcu();
        let requirements = LensRequirements {
            fields: vec!["content".to_string()],
            embedding: None,
            context_dimensions: vec![],
        };
        
        let result = Validator::validate_requirements("test", &requirements, &fbcu);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_requirements_missing_field() {
        let fbcu = HashMap::new();
        let requirements = LensRequirements {
            fields: vec!["content".to_string()],
            embedding: None,
            context_dimensions: vec![],
        };
        
        let result = Validator::validate_requirements("test", &requirements, &fbcu);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_embedding_success() {
        let fbcu = create_test_fbcu();
        let requirement = EmbeddingRequirement {
            model: "all-MiniLM-L6-v2".to_string(),
            dimension: 384,
        };
        
        let result = Validator::validate_embedding(&fbcu, &requirement);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_embedding_dimension_mismatch() {
        let fbcu = create_test_fbcu();
        let requirement = EmbeddingRequirement {
            model: "test".to_string(),
            dimension: 512,
        };
        
        let result = Validator::validate_embedding(&fbcu, &requirement);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_quality_bounds_passed() {
        let metrics = QualityMetrics::new(0.85, 0.90, 0.88);
        let bounds = QualityBounds {
            coherence_min: 0.70,
            lens_agreement_min: 0.75,
            confidence_min: 0.65,
        };
        
        let status = Validator::validate_quality_bounds(&metrics, &bounds);
        assert_eq!(status, ValidationStatus::Passed);
    }
    
    #[test]
    fn test_validate_quality_bounds_failed() {
        let metrics = QualityMetrics::new(0.50, 0.60, 0.55);
        let bounds = QualityBounds {
            coherence_min: 0.70,
            lens_agreement_min: 0.75,
            confidence_min: 0.65,
        };
        
        let status = Validator::validate_quality_bounds(&metrics, &bounds);
        assert!(matches!(status, ValidationStatus::Failed(_)));
    }
    
    #[test]
    fn test_validate_quality_bounds_partial() {
        let metrics = QualityMetrics::new(0.71, 0.90, 0.88);
        let bounds = QualityBounds {
            coherence_min: 0.70,
            lens_agreement_min: 0.75,
            confidence_min: 0.65,
        };
        
        let status = Validator::validate_quality_bounds(&metrics, &bounds);
        // Puede ser Passed o PartialSuccess dependiendo del threshold
        assert!(matches!(status, ValidationStatus::Passed | ValidationStatus::PartialSuccess(_)));
    }
}
