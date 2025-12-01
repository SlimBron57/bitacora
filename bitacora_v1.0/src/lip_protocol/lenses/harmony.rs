// HarmonyLens - Análisis de coherencia armónica en FBCUs

use crate::lip_protocol::types::*;
use std::collections::HashMap;
use serde_json::json;

/// Lens para análisis de coherencia armónica
pub struct HarmonyLens;

impl LensInterface for HarmonyLens {
    fn lens_id(&self) -> &str {
        "harmony_lens"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn requires(&self) -> LensRequirements {
        LensRequirements {
            fields: vec!["embedding".to_string()],
            embedding: Some(EmbeddingRequirement {
                model: "all-MiniLM-L6-v2".to_string(),
                dimension: 384,
            }),
            context_dimensions: vec!["temporal".to_string(), "emotional".to_string()],
        }
    }
    
    fn provides(&self) -> Vec<String> {
        vec![
            "harmony_score".to_string(),
            "dissonance_regions".to_string(),
            "dimension_coherence".to_string(),
        ]
    }
    
    fn quality_bounds(&self) -> QualityBounds {
        QualityBounds {
            coherence_min: 0.70,
            lens_agreement_min: 0.75,
            confidence_min: 0.65,
        }
    }
    
    fn process(&self, fbcu_data: &HashMap<String, serde_json::Value>) -> Result<LensOutput, LensError> {
        // Extraer embedding
        let embedding = fbcu_data.get("embedding")
            .ok_or_else(|| LensError::MissingField("embedding".to_string()))?;
        
        let embedding_vec: Vec<f64> = serde_json::from_value(embedding.clone())
            .map_err(|e| LensError::InvalidFormat(format!("embedding: {}", e)))?;
        
        // Verificar dimensión
        if embedding_vec.len() != 384 {
            return Err(LensError::EmbeddingDimensionMismatch {
                expected: 384,
                found: embedding_vec.len(),
            });
        }
        
        // Análisis de coherencia armónica
        let harmony_score = Self::calculate_harmony(&embedding_vec);
        let dissonance_regions = Self::detect_dissonance(&embedding_vec);
        let dimension_coherence = Self::analyze_dimensions(fbcu_data);
        
        // Construir output
        let mut data = HashMap::new();
        data.insert("harmony_score".to_string(), json!(harmony_score));
        data.insert("dissonance_regions".to_string(), json!(dissonance_regions));
        data.insert("dimension_coherence".to_string(), json!(dimension_coherence));
        
        // Métricas de calidad
        let metrics = QualityMetrics::new(
            harmony_score,
            0.85,
            0.90,
        );
        
        Ok(LensOutput::new(
            self.lens_id().to_string(),
            self.version().to_string(),
            fbcu_data.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            data,
            metrics,
        ))
    }
}

impl HarmonyLens {
    /// Calcula score de coherencia armónica
    fn calculate_harmony(embedding: &[f64]) -> f64 {
        let mean = embedding.iter().sum::<f64>() / embedding.len() as f64;
        let variance = embedding.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / embedding.len() as f64;
        
        let normalized_variance = variance.sqrt() / mean.abs().max(1e-6);
        let harmony = 1.0 / (1.0 + normalized_variance);
        
        harmony.clamp(0.0, 1.0)
    }
    
    /// Detecta regiones de disonancia
    fn detect_dissonance(embedding: &[f64]) -> Vec<usize> {
        let mut dissonant_regions = Vec::new();
        
        for i in 0..embedding.len().saturating_sub(1) {
            let diff = (embedding[i + 1] - embedding[i]).abs();
            if diff > 0.5 {
                dissonant_regions.push(i);
            }
        }
        
        dissonant_regions
    }
    
    /// Analiza coherencia entre dimensiones
    fn analyze_dimensions(fbcu_data: &HashMap<String, serde_json::Value>) -> HashMap<String, f64> {
        let mut coherence = HashMap::new();
        
        if let Some(temporal) = fbcu_data.get("context_tensor")
            .and_then(|ct| ct.get("temporal"))
            .and_then(|t| t.as_f64())
        {
            coherence.insert("temporal".to_string(), temporal);
        }
        
        if let Some(emotional) = fbcu_data.get("context_tensor")
            .and_then(|ct| ct.get("emotional"))
            .and_then(|e| e.as_f64())
        {
            coherence.insert("emotional".to_string(), emotional);
        }
        
        coherence
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn create_test_fbcu() -> HashMap<String, serde_json::Value> {
        let mut fbcu = HashMap::new();
        let embedding = vec![0.5; 384];
        fbcu.insert("embedding".to_string(), json!(embedding));
        
        let context = json!({"temporal": 0.8, "emotional": 0.7});
        fbcu.insert("context_tensor".to_string(), context);
        fbcu.insert("id".to_string(), json!("test_fbcu_1"));
        
        fbcu
    }
    
    #[test]
    fn test_harmony_lens_process() {
        let lens = HarmonyLens;
        let fbcu = create_test_fbcu();
        
        let result = lens.process(&fbcu);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.lens_id, "harmony_lens");
        assert!(output.data.contains_key("harmony_score"));
    }
}
