// SemanticLens - Análisis de profundidad semántica

use crate::lip_protocol::types::*;
use std::collections::HashMap;
use serde_json::json;

/// Lens para análisis de profundidad semántica
pub struct SemanticLens;

impl LensInterface for SemanticLens {
    fn lens_id(&self) -> &str {
        "semantic_lens"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn requires(&self) -> LensRequirements {
        LensRequirements {
            fields: vec!["content".to_string()],
            embedding: None,
            context_dimensions: vec![],
        }
    }
    
    fn provides(&self) -> Vec<String> {
        vec![
            "semantic_depth".to_string(),
            "key_concepts".to_string(),
            "conceptual_density".to_string(),
        ]
    }
    
    fn quality_bounds(&self) -> QualityBounds {
        QualityBounds {
            coherence_min: 0.60,
            lens_agreement_min: 0.65,
            confidence_min: 0.70,
        }
    }
    
    fn process(&self, fbcu_data: &HashMap<String, serde_json::Value>) -> Result<LensOutput, LensError> {
        let content = fbcu_data.get("content")
            .and_then(|c| c.as_str())
            .ok_or_else(|| LensError::MissingField("content".to_string()))?;
        
        let semantic_depth = Self::calculate_depth(content);
        let key_concepts = Self::extract_concepts(content);
        let conceptual_density = Self::calculate_density(content, &key_concepts);
        
        let mut data = HashMap::new();
        data.insert("semantic_depth".to_string(), json!(semantic_depth));
        data.insert("key_concepts".to_string(), json!(key_concepts));
        data.insert("conceptual_density".to_string(), json!(conceptual_density));
        
        let metrics = QualityMetrics::new(0.80, 0.75, semantic_depth);
        
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

impl SemanticLens {
    fn calculate_depth(content: &str) -> f64 {
        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count = words.len() as f64;
        
        if word_count == 0.0 {
            return 0.0;
        }
        
        let unique_words: std::collections::HashSet<&str> = words.iter().cloned().collect();
        let lexical_diversity = unique_words.len() as f64 / word_count;
        
        let avg_word_length = words.iter()
            .map(|w| w.len() as f64)
            .sum::<f64>() / word_count;
        
        let depth = (lexical_diversity * 0.6 + (avg_word_length / 10.0) * 0.4).min(1.0);
        
        depth
    }
    
    fn extract_concepts(content: &str) -> Vec<String> {
        let words: Vec<&str> = content.split_whitespace().collect();
        
        let mut concepts: Vec<String> = words.iter()
            .filter(|w| w.len() > 4)
            .take(10)
            .map(|w| w.to_lowercase())
            .collect();
        
        concepts.sort();
        concepts.dedup();
        
        concepts
    }
    
    fn calculate_density(content: &str, concepts: &[String]) -> f64 {
        let word_count = content.split_whitespace().count() as f64;
        
        if word_count == 0.0 {
            return 0.0;
        }
        
        let concept_count = concepts.len() as f64;
        let density = (concept_count / word_count * 10.0).min(1.0);
        
        density
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn create_test_fbcu() -> HashMap<String, serde_json::Value> {
        let mut fbcu = HashMap::new();
        
        let content = "Bayesian fusion combines multiple probabilistic models";
        
        fbcu.insert("content".to_string(), json!(content));
        fbcu.insert("id".to_string(), json!("test_fbcu_semantic"));
        
        fbcu
    }
    
    #[test]
    fn test_semantic_lens_process() {
        let lens = SemanticLens;
        let fbcu = create_test_fbcu();
        
        let result = lens.process(&fbcu);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.lens_id, "semantic_lens");
        assert!(output.data.contains_key("semantic_depth"));
    }
}
