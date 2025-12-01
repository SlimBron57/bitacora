// MttDslLens - Análisis de patterns de debugging vía MTT-DSL

use crate::lip_protocol::types::*;
use std::collections::HashMap;
use serde_json::json;

/// Lens para análisis de debugging patterns usando MTT-DSL
pub struct MttDslLens;

impl LensInterface for MttDslLens {
    fn lens_id(&self) -> &str {
        "mtt_dsl_lens"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn requires(&self) -> LensRequirements {
        LensRequirements {
            fields: vec!["debug_context".to_string(), "template_id".to_string()],
            embedding: None,
            context_dimensions: vec!["complexity".to_string()],
        }
    }
    
    fn provides(&self) -> Vec<String> {
        vec![
            "pattern_type".to_string(),
            "recommendations".to_string(),
            "template_match_score".to_string(),
        ]
    }
    
    fn quality_bounds(&self) -> QualityBounds {
        QualityBounds {
            coherence_min: 0.65,
            lens_agreement_min: 0.70,
            confidence_min: 0.75,
        }
    }
    
    fn process(&self, fbcu_data: &HashMap<String, serde_json::Value>) -> Result<LensOutput, LensError> {
        let debug_context = fbcu_data.get("debug_context")
            .ok_or_else(|| LensError::MissingField("debug_context".to_string()))?;
        
        let template_id = fbcu_data.get("template_id")
            .and_then(|t| t.as_str())
            .ok_or_else(|| LensError::MissingField("template_id".to_string()))?;
        
        let pattern_type = Self::identify_pattern(debug_context);
        let recommendations = Self::generate_recommendations(&pattern_type, template_id);
        let match_score = Self::calculate_template_match(debug_context, template_id);
        
        let mut data = HashMap::new();
        data.insert("pattern_type".to_string(), json!(pattern_type));
        data.insert("recommendations".to_string(), json!(recommendations));
        data.insert("template_match_score".to_string(), json!(match_score));
        
        let confidence = if match_score > 0.8 { 0.90 } else { 0.75 };
        let metrics = QualityMetrics::new(0.82, 0.78, confidence);
        
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

impl MttDslLens {
    fn identify_pattern(debug_context: &serde_json::Value) -> String {
        if let Some(obj) = debug_context.as_object() {
            if obj.contains_key("error_type") {
                return "error_handling".to_string();
            } else if obj.contains_key("performance") {
                return "performance_analysis".to_string();
            } else if obj.contains_key("state") {
                return "state_inspection".to_string();
            }
        }
        
        "general_debugging".to_string()
    }
    
    fn generate_recommendations(pattern: &str, template_id: &str) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        match pattern {
            "error_handling" => {
                recommendations.push("Verify error propagation chain".to_string());
                recommendations.push("Check error context preservation".to_string());
            }
            "performance_analysis" => {
                recommendations.push("Profile critical sections".to_string());
                recommendations.push("Analyze algorithmic complexity".to_string());
            }
            "state_inspection" => {
                recommendations.push("Validate state transitions".to_string());
                recommendations.push("Check invariants".to_string());
            }
            _ => {
                recommendations.push(format!("Apply template: {}", template_id));
            }
        }
        
        recommendations
    }
    
    fn calculate_template_match(debug_context: &serde_json::Value, template_id: &str) -> f64 {
        let context_keys = debug_context.as_object()
            .map(|obj| obj.len())
            .unwrap_or(0);
        
        let base_score = (context_keys as f64 / 10.0).min(1.0);
        
        let template_bonus = if template_id.contains("error") || template_id.contains("performance") {
            0.1
        } else {
            0.0
        };
        
        (base_score + template_bonus).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn create_test_fbcu() -> HashMap<String, serde_json::Value> {
        let mut fbcu = HashMap::new();
        
        let debug_context = json!({
            "error_type": "NullPointerException",
            "stack_trace": ["fn_a", "fn_b", "fn_c"],
            "variables": {"x": 10, "y": null}
        });
        
        fbcu.insert("debug_context".to_string(), debug_context);
        fbcu.insert("template_id".to_string(), json!("error_handling_001"));
        fbcu.insert("id".to_string(), json!("test_fbcu_mtt"));
        
        let context_tensor = json!({"complexity": 0.65});
        fbcu.insert("context_tensor".to_string(), context_tensor);
        
        fbcu
    }
    
    #[test]
    fn test_mtt_dsl_lens_process() {
        let lens = MttDslLens;
        let fbcu = create_test_fbcu();
        
        let result = lens.process(&fbcu);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert_eq!(output.lens_id, "mtt_dsl_lens");
        assert!(output.data.contains_key("pattern_type"));
    }
}
