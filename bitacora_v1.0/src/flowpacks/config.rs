//! Configuración de FlowPacks Anti-Disco-Rayado

use std::path::PathBuf;

/// Configuración del FlowPackEngine
#[derive(Debug, Clone)]
pub struct FlowPackConfig {
    /// Umbral de similitud para mensajes relacionados (0.0-1.0)
    /// Mensajes con similitud ≥ threshold se agrupan en mismo FlowPack
    /// Default: 0.85 (85%)
    pub similarity_threshold: f64,
    
    /// Umbral para repetición EXACTA (0.0-1.0)
    /// Similitud ≥ exact_threshold → AdaptiveResponse::Reference
    /// Default: 0.95 (95%)
    pub exact_threshold: f64,
    
    /// Ventana temporal para agrupar mensajes (horas)
    /// Mensajes fuera de ventana no se consideran para similitud
    /// Default: 72 (3 días)
    pub temporal_window_hours: u64,
    
    /// Tamaño máximo de FlowPack (número de mensajes)
    /// Default: 20
    pub max_pack_size: usize,
    
    /// Tamaño de cache de FlowPacks activos (entries)
    /// LRU cache en memoria RAM
    /// Default: 100
    pub cache_size: usize,
    
    /// Activar compresión agresiva FBCU (mayor ratio, más CPU)
    /// Default: true
    pub aggressive_compression: bool,
    
    /// Nivel de compresión wavelet (1-10)
    /// Default: 6
    pub wavelet_level: u8,
    
    /// Nivel de compresión fractal (1-10)
    /// Default: 8
    pub fractal_level: u8,
    
    /// Path al modelo de embeddings
    /// None = usa modelo embebido (MiniLM-L6-v2)
    /// Default: None
    pub embedding_model_path: Option<PathBuf>,
    
    /// Dimensión de los embeddings
    /// MiniLM-L6-v2: 384
    /// BERT-base: 768
    /// Default: 384
    pub embedding_dimension: usize,
    
    /// Número de vecinos en búsqueda HNSW (k-NN)
    /// Default: 10
    pub hnsw_k: usize,
    
    /// Parámetro ef_construction de HNSW (calidad del índice)
    /// Mayor = mejor calidad, más lento build
    /// Default: 200
    pub hnsw_ef_construction: usize,
    
    /// Parámetro ef_search de HNSW (recall en búsqueda)
    /// Mayor = mejor recall, más lento search
    /// Default: 50
    pub hnsw_ef_search: usize,
    
    /// Parámetro M de HNSW (conexiones por nodo)
    /// Mayor = mejor calidad, más memoria
    /// Default: 16
    pub hnsw_m: usize,
}

impl Default for FlowPackConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.85,
            exact_threshold: 0.95,
            temporal_window_hours: 72,
            max_pack_size: 20,
            cache_size: 100,
            aggressive_compression: true,
            wavelet_level: 6,
            fractal_level: 8,
            embedding_model_path: None,
            embedding_dimension: 384,
            hnsw_k: 10,
            hnsw_ef_construction: 200,
            hnsw_ef_search: 50,
            hnsw_m: 16,
        }
    }
}

impl FlowPackConfig {
    /// Configuración optimizada para velocidad
    pub fn fast() -> Self {
        Self {
            similarity_threshold: 0.80,
            exact_threshold: 0.90,
            temporal_window_hours: 48,
            max_pack_size: 15,
            cache_size: 50,
            aggressive_compression: false,
            wavelet_level: 4,
            fractal_level: 5,
            embedding_dimension: 384,
            hnsw_k: 5,
            hnsw_ef_construction: 100,
            hnsw_ef_search: 30,
            hnsw_m: 12,
            ..Default::default()
        }
    }
    
    /// Configuración optimizada para calidad
    pub fn high_quality() -> Self {
        Self {
            similarity_threshold: 0.90,
            exact_threshold: 0.97,
            temporal_window_hours: 168,
            max_pack_size: 30,
            cache_size: 200,
            aggressive_compression: true,
            wavelet_level: 8,
            fractal_level: 9,
            embedding_dimension: 768,
            hnsw_k: 20,
            hnsw_ef_construction: 400,
            hnsw_ef_search: 100,
            hnsw_m: 24,
            ..Default::default()
        }
    }
    
    /// Validar configuración
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.similarity_threshold) {
            return Err(format!("similarity_threshold debe estar en [0.0, 1.0]: {}", self.similarity_threshold));
        }
        
        if !(0.0..=1.0).contains(&self.exact_threshold) {
            return Err(format!("exact_threshold debe estar en [0.0, 1.0]: {}", self.exact_threshold));
        }
        
        if self.exact_threshold < self.similarity_threshold {
            return Err(format!("exact_threshold ({}) debe ser >= similarity_threshold ({})",
                self.exact_threshold, self.similarity_threshold));
        }
        
        if !(1..=10).contains(&self.wavelet_level) {
            return Err(format!("wavelet_level debe estar en [1, 10]: {}", self.wavelet_level));
        }
        
        if !(1..=10).contains(&self.fractal_level) {
            return Err(format!("fractal_level debe estar en [1, 10]: {}", self.fractal_level));
        }
        
        if self.max_pack_size == 0 {
            return Err("max_pack_size debe ser > 0".to_string());
        }
        
        if self.cache_size == 0 {
            return Err("cache_size debe ser > 0".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config_valid() {
        let config = FlowPackConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_fast_config_valid() {
        let config = FlowPackConfig::fast();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_high_quality_config_valid() {
        let config = FlowPackConfig::high_quality();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_threshold() {
        let mut config = FlowPackConfig::default();
        config.similarity_threshold = 1.5;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_exact_less_than_similarity() {
        let mut config = FlowPackConfig::default();
        config.exact_threshold = 0.80;
        config.similarity_threshold = 0.90;
        assert!(config.validate().is_err());
    }
}
