// LIP Protocol - Engine
// Motor principal para gestión y aplicación de lentes

use super::types::*;
use super::validation::Validator;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Configuración del motor LIP
#[derive(Debug, Clone)]
pub struct LipConfig {
    /// Modo estricto (rechazar si quality < bounds)
    pub strict_mode: bool,
    
    /// Permitir lentes sin embeddings
    pub allow_no_embedding: bool,
}

impl Default for LipConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            allow_no_embedding: true,
        }
    }
}

/// Motor principal del LIP Protocol
pub struct LipEngine {
    /// Lentes registrados (ID -> Lens)
    lenses: HashMap<String, Box<dyn LensInterface>>,
    
    /// Configuración del motor
    config: LipConfig,
}

impl LipEngine {
    /// Crea un nuevo motor LIP con configuración por defecto
    pub fn new() -> Self {
        Self {
            lenses: HashMap::new(),
            config: LipConfig::default(),
        }
    }
    
    /// Crea un nuevo motor LIP con configuración personalizada
    pub fn with_config(config: LipConfig) -> Self {
        Self {
            lenses: HashMap::new(),
            config,
        }
    }
    
    /// Registra un nuevo lens en el motor
    pub fn register_lens(&mut self, lens: Box<dyn LensInterface>) -> Result<(), LensError> {
        // Validar contrato del lens
        Validator::validate_lens_contract(lens.as_ref())?;
        
        let lens_id = lens.lens_id().to_string();
        
        // Registrar lens
        self.lenses.insert(lens_id, lens);
        
        Ok(())
    }
    
    /// Aplica un lens a un FBCU
    pub fn apply_lens(
        &self,
        lens_id: &str,
        fbcu_id: &str,
        fbcu_data: &HashMap<String, serde_json::Value>,
    ) -> Result<LensOutput, LensError> {
        // Obtener lens
        let lens = self.lenses
            .get(lens_id)
            .ok_or_else(|| LensError::LensNotFound(lens_id.to_string()))?;
        
        // Validar requisitos
        let requirements = lens.requires();
        Validator::validate_requirements(lens_id, &requirements, fbcu_data)?;
        
        // Procesar con el lens
        let mut output = lens.process(fbcu_data)?;
        
        // Validar quality bounds
        let bounds = lens.quality_bounds();
        let validation_status = Validator::validate_quality_bounds(&output.quality_metrics, &bounds);
        
        // En modo estricto, rechazar si no pasó validación
        if self.config.strict_mode {
            if let ValidationStatus::Failed(_) = &validation_status {
                return Err(LensError::QualityBoundViolation {
                    metric: "multiple".to_string(),
                    value: 0.0,
                    minimum: 0.0,
                });
            }
        }
        
        // Actualizar estado de validación
        output.validation_status = validation_status;
        
        Ok(output)
    }
    
    /// Lista todos los lentes registrados
    pub fn list_lenses(&self) -> Vec<String> {
        self.lenses.keys().cloned().collect()
    }
    
    /// Obtiene información de un lens específico
    pub fn get_lens_info(&self, lens_id: &str) -> Option<LensInfo> {
        self.lenses.get(lens_id).map(|lens| LensInfo {
            id: lens.lens_id().to_string(),
            version: lens.version().to_string(),
            requires: lens.requires(),
            provides: lens.provides(),
            quality_bounds: lens.quality_bounds(),
        })
    }
    
    /// Verifica si un FBCU es compatible con un lens
    pub fn is_compatible(
        &self,
        lens_id: &str,
        fbcu_data: &HashMap<String, serde_json::Value>,
    ) -> bool {
        if let Some(lens) = self.lenses.get(lens_id) {
            let requirements = lens.requires();
            Validator::validate_requirements(lens_id, &requirements, fbcu_data).is_ok()
        } else {
            false
        }
    }
}

impl Default for LipEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Información sobre un lens registrado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensInfo {
    pub id: String,
    pub version: String,
    pub requires: LensRequirements,
    pub provides: Vec<String>,
    pub quality_bounds: QualityBounds,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    // Mock lens para testing
    struct MockLens;
    
    impl LensInterface for MockLens {
        fn lens_id(&self) -> &str {
            "mock_lens"
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
            vec!["mock_output".to_string()]
        }
        
        fn quality_bounds(&self) -> QualityBounds {
            QualityBounds::default()
        }
        
        fn process(&self, _fbcu_data: &HashMap<String, serde_json::Value>) -> Result<LensOutput, LensError> {
            let mut data = HashMap::new();
            data.insert("mock_output".to_string(), json!("processed"));
            
            let metrics = QualityMetrics::new(0.85, 0.90, 0.88);
            
            Ok(LensOutput::new(
                self.lens_id().to_string(),
                self.version().to_string(),
                "test_fbcu".to_string(),
                data,
                metrics,
            ))
        }
    }
    
    fn create_test_fbcu() -> HashMap<String, serde_json::Value> {
        let mut fbcu = HashMap::new();
        fbcu.insert("content".to_string(), json!("test content"));
        fbcu
    }
    
    #[test]
    fn test_engine_creation() {
        let engine = LipEngine::new();
        assert_eq!(engine.list_lenses().len(), 0);
    }
    
    #[test]
    fn test_register_lens() {
        let mut engine = LipEngine::new();
        let lens = Box::new(MockLens);
        
        let result = engine.register_lens(lens);
        assert!(result.is_ok());
        assert_eq!(engine.list_lenses().len(), 1);
    }
    
    #[test]
    fn test_apply_lens_success() {
        let mut engine = LipEngine::new();
        engine.register_lens(Box::new(MockLens)).unwrap();
        
        let fbcu = create_test_fbcu();
        let result = engine.apply_lens("mock_lens", "test_fbcu", &fbcu);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.lens_id, "mock_lens");
        assert_eq!(output.version, "1.0.0");
    }
    
    #[test]
    fn test_apply_lens_not_found() {
        let engine = LipEngine::new();
        let fbcu = create_test_fbcu();
        
        let result = engine.apply_lens("nonexistent", "test_fbcu", &fbcu);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), LensError::LensNotFound(_)));
    }
    
    #[test]
    fn test_get_lens_info() {
        let mut engine = LipEngine::new();
        engine.register_lens(Box::new(MockLens)).unwrap();
        
        let info = engine.get_lens_info("mock_lens");
        assert!(info.is_some());
        
        let info = info.unwrap();
        assert_eq!(info.id, "mock_lens");
        assert_eq!(info.version, "1.0.0");
    }
    
    #[test]
    fn test_is_compatible() {
        let mut engine = LipEngine::new();
        engine.register_lens(Box::new(MockLens)).unwrap();
        
        let fbcu = create_test_fbcu();
        assert!(engine.is_compatible("mock_lens", &fbcu));
        
        let empty_fbcu = HashMap::new();
        assert!(!engine.is_compatible("mock_lens", &empty_fbcu));
    }
}
