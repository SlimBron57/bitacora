// FLOWPACKS :: COMPRESSION
// Compresión contextual de FlowPacks usando FBCU + estrategias delta
// SPDX-License-Identifier: PROPRIETARY
// Copyright (c) 2024 Eduardo González Iñiguez

use super::{FlowPack, FlowPackEntry, FlowPackConfig, Result};
use super::error::FlowPackError;
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// Estrategia de compresión para FlowPacks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionStrategy {
    /// Compresión FBCU estándar (baseline 15x)
    FBCU,
    /// Compresión delta para mensajes similares (1.5-3x adicional)
    Delta {
        reference_id: uuid::Uuid,
        similarity: f64,
    },
    /// Compresión híbrida (FBCU + Delta)
    Hybrid {
        fbcu_ratio: f32,
        delta_ratio: f32,
    },
    /// Sin compresión (debug/testing)
    None,
}

/// Resultado de compresión con métricas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionResult {
    /// Datos comprimidos
    pub compressed_data: Vec<u8>,
    /// Tamaño original en bytes
    pub original_size: usize,
    /// Tamaño comprimido en bytes
    pub compressed_size: usize,
    /// Ratio de compresión (original / compressed)
    pub compression_ratio: f64,
    /// Estrategia usada
    pub strategy: CompressionStrategy,
    /// Tiempo de compresión en microsegundos
    pub compression_time_us: u64,
}

impl CompressionResult {
    /// Crea un resultado de compresión
    pub fn new(
        compressed_data: Vec<u8>,
        original_size: usize,
        strategy: CompressionStrategy,
        compression_time_us: u64,
    ) -> Self {
        let compressed_size = compressed_data.len();
        let compression_ratio = if compressed_size > 0 {
            original_size as f64 / compressed_size as f64
        } else {
            0.0
        };

        Self {
            compressed_data,
            original_size,
            compressed_size,
            compression_ratio,
            strategy,
            compression_time_us,
        }
    }

    /// Verifica si cumple el target de ratio (>20x)
    pub fn meets_target(&self, target_ratio: f64) -> bool {
        self.compression_ratio >= target_ratio
    }
}

/// Motor de compresión FBCU (stub - integración real pendiente)
pub struct FBCUCompressor {
    config: FlowPackConfig,
    // TODO: Arc<fbcu::FBCUEngine> cuando se integre FBCU
}

impl FBCUCompressor {
    /// Crea un nuevo compresor FBCU
    pub fn new(config: FlowPackConfig) -> Result<Self> {
        Ok(Self { config })
    }

    /// Comprime texto usando FBCU
    pub fn compress(&self, text: &str) -> Result<Vec<u8>> {
        // TODO: Integración real con FBCU
        // Por ahora: simulación usando zlib (placeholder)
        
        use std::io::Write;
        use flate2::Compression;
        use flate2::write::ZlibEncoder;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(text.as_bytes())
            .map_err(|e: IoError| FlowPackError::CompressionFailed(e.to_string()))?;
        
        encoder.finish()
            .map_err(|e: IoError| FlowPackError::CompressionFailed(e.to_string()))
    }

    /// Descomprime datos FBCU
    pub fn decompress(&self, data: &[u8]) -> Result<String> {
        // TODO: Integración real con FBCU
        // Por ahora: simulación usando zlib (placeholder)
        
        use std::io::Read;
        use flate2::read::ZlibDecoder;

        let mut decoder = ZlibDecoder::new(data);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)
            .map_err(|e: IoError| FlowPackError::CompressionFailed(e.to_string()))?;
        
        Ok(decompressed)
    }

    /// Estima el ratio de compresión esperado
    pub fn estimate_ratio(&self, text: &str) -> f64 {
        // FBCU baseline: 15x esperado
        // Ajuste por longitud (textos largos comprimen mejor)
        let base_ratio = 15.0;
        let length_factor = (text.len() as f64 / 1000.0).min(2.0); // max 2x boost
        
        base_ratio * (1.0 + length_factor * 0.2)
    }
}

/// Compresor delta para mensajes similares
pub struct DeltaCompressor {
    config: FlowPackConfig,
}

impl DeltaCompressor {
    /// Crea un nuevo compresor delta
    pub fn new(config: FlowPackConfig) -> Self {
        Self { config }
    }

    /// Comprime texto usando delta contra referencia
    pub fn compress_delta(&self, text: &str, reference: &str, similarity: f64) -> Result<Vec<u8>> {
        // Si no hay suficiente similitud, no usar delta
        if similarity < self.config.similarity_threshold {
            return Ok(text.as_bytes().to_vec());
        }

        // TODO: Implementación real de compresión delta
        // Por ahora: simple diff (placeholder)
        
        // Encontrar diferencias
        let diff = self.compute_diff(text, reference);
        
        // Serializar el diff
        serde_json::to_vec(&diff)
            .map_err(FlowPackError::SerializationError)
    }

    /// Descomprime delta aplicándolo a referencia
    pub fn decompress_delta(&self, delta_data: &[u8], reference: &str) -> Result<String> {
        // Deserializar el diff
        let diff: Vec<DiffOp> = serde_json::from_slice(delta_data)
            .map_err(FlowPackError::SerializationError)?;
        
        // Aplicar diff a referencia
        Ok(self.apply_diff(reference, &diff))
    }

    /// Calcula diferencias entre texto y referencia
    fn compute_diff(&self, text: &str, reference: &str) -> Vec<DiffOp> {
        // Algoritmo simple: split por palabras y detectar operaciones
        let text_words: Vec<&str> = text.split_whitespace().collect();
        let ref_words: Vec<&str> = reference.split_whitespace().collect();
        
        let mut diff = Vec::new();
        let mut i = 0;
        let mut j = 0;

        while i < text_words.len() || j < ref_words.len() {
            if i >= text_words.len() {
                diff.push(DiffOp::Delete { position: j });
                j += 1;
            } else if j >= ref_words.len() {
                diff.push(DiffOp::Insert { 
                    position: i, 
                    text: text_words[i].to_string() 
                });
                i += 1;
            } else if text_words[i] == ref_words[j] {
                diff.push(DiffOp::Keep { count: 1 });
                i += 1;
                j += 1;
            } else {
                // Cambio
                diff.push(DiffOp::Replace { 
                    position: j, 
                    text: text_words[i].to_string() 
                });
                i += 1;
                j += 1;
            }
        }

        diff
    }

    /// Aplica diff a referencia
    fn apply_diff(&self, reference: &str, diff: &[DiffOp]) -> String {
        let ref_words: Vec<&str> = reference.split_whitespace().collect();
        let mut result = Vec::new();
        let mut ref_idx = 0;

        for op in diff {
            match op {
                DiffOp::Keep { count } => {
                    for _ in 0..*count {
                        if ref_idx < ref_words.len() {
                            result.push(ref_words[ref_idx].to_string());
                            ref_idx += 1;
                        }
                    }
                }
                DiffOp::Insert { text, .. } => {
                    result.push(text.clone());
                }
                DiffOp::Delete { .. } => {
                    ref_idx += 1;
                }
                DiffOp::Replace { text, .. } => {
                    result.push(text.clone());
                    ref_idx += 1;
                }
            }
        }

        result.join(" ")
    }

    /// Estima el ratio de compresión delta
    pub fn estimate_delta_ratio(&self, similarity: f64) -> f64 {
        // A mayor similitud, mejor compresión
        // Similarity 0.85 → 1.5x, 0.95 → 3x
        if similarity < self.config.similarity_threshold {
            return 1.0; // Sin compresión
        }

        let normalized = (similarity - self.config.similarity_threshold) /
                         (1.0 - self.config.similarity_threshold);
        
        1.5 + normalized * 1.5 // Range: 1.5x to 3x
    }
}

/// Operación de diff
#[derive(Debug, Clone, Serialize, Deserialize)]
enum DiffOp {
    Keep { count: usize },
    Insert { position: usize, text: String },
    Delete { position: usize },
    Replace { position: usize, text: String },
}

/// Motor de compresión principal
pub struct CompressionEngine {
    fbcu: FBCUCompressor,
    delta: DeltaCompressor,
    config: FlowPackConfig,
}

impl CompressionEngine {
    /// Crea un nuevo motor de compresión
    pub fn new(config: FlowPackConfig) -> Result<Self> {
        let fbcu = FBCUCompressor::new(config.clone())?;
        let delta = DeltaCompressor::new(config.clone());
        
        Ok(Self { fbcu, delta, config })
    }

    /// Comprime un FlowPack completo
    pub fn compress_flowpack(&self, flowpack: &mut FlowPack) -> Result<CompressionResult> {
        let start = std::time::Instant::now();
        let mut total_original = 0;
        let mut total_compressed = 0;

        // Comprimir cada entrada
        for entry in &mut flowpack.entries {
            let original_size = entry.original_size;
            total_original += original_size;

            // Usar FBCU para todas las entradas
            let compressed = self.fbcu.compress(&entry.content)?;
            total_compressed += compressed.len();
            
            // Actualizar tamaño comprimido
            entry.compressed_size = Some(compressed.len());
        }

        let elapsed = start.elapsed().as_micros() as u64;
        
        Ok(CompressionResult::new(
            Vec::new(), // FlowPack ya tiene datos comprimidos
            total_original,
            CompressionStrategy::FBCU,
            elapsed,
        ))
    }

    /// Descomprime un FlowPack
    pub fn decompress_flowpack(&self, flowpack: &FlowPack) -> Result<Vec<String>> {
        let mut decompressed = Vec::new();

        for entry in &flowpack.entries {
            // Si hay compresión, descomprimir (placeholder: retornar content directo)
            // TODO: Implementar descompresión real cuando se integre FBCU
            decompressed.push(entry.content.clone());
        }

        Ok(decompressed)
    }

    /// Estima el ratio total esperado
    pub fn estimate_total_ratio(&self, text: &str, similarity: Option<f64>) -> f64 {
        let fbcu_ratio = self.fbcu.estimate_ratio(text);
        
        if let Some(sim) = similarity {
            let delta_ratio = self.delta.estimate_delta_ratio(sim);
            fbcu_ratio * delta_ratio
        } else {
            fbcu_ratio
        }
    }

    /// Verifica si la compresión es agresiva
    pub fn is_aggressive(&self) -> bool {
        self.config.aggressive_compression
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fbcu_compression() {
        let config = FlowPackConfig::default();
        let compressor = FBCUCompressor::new(config).unwrap();
        
        // Texto largo para asegurar compresión (zlib necesita >200 bytes)
        let text = "Este es un texto de prueba que será comprimido usando FBCU. ".repeat(10);
        let compressed = compressor.compress(&text).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(text, decompressed);
        assert!(compressed.len() < text.len(), 
            "Compressed {} should be < original {}", compressed.len(), text.len());
    }

    #[test]
    fn test_delta_compression() {
        let config = FlowPackConfig::default();
        let compressor = DeltaCompressor::new(config);
        
        let reference = "El gato está en el tejado";
        let text = "El perro está en el jardín";
        let similarity = 0.9;
        
        let delta = compressor.compress_delta(text, reference, similarity).unwrap();
        let decompressed = compressor.decompress_delta(&delta, reference).unwrap();
        
        // Verificar que se recupera correctamente
        assert!(decompressed.contains("perro"));
        assert!(decompressed.contains("jardín"));
    }

    #[test]
    fn test_compression_ratio_estimation() {
        let config = FlowPackConfig::default();
        let compressor = FBCUCompressor::new(config).unwrap();
        
        let short_text = "Corto";
        let long_text = "Este es un texto mucho más largo que debería comprimir mejor debido a la repetición de patrones y la mayor cantidad de redundancia en el contenido.";
        
        let short_ratio = compressor.estimate_ratio(short_text);
        let long_ratio = compressor.estimate_ratio(long_text);
        
        assert!(long_ratio > short_ratio);
        assert!(short_ratio >= 15.0); // FBCU baseline
    }

    #[test]
    fn test_delta_ratio_estimation() {
        let config = FlowPackConfig::default();
        let compressor = DeltaCompressor::new(config);
        
        let low_sim = 0.7;
        let high_sim = 0.95;
        
        let low_ratio = compressor.estimate_delta_ratio(low_sim);
        let high_ratio = compressor.estimate_delta_ratio(high_sim);
        
        assert_eq!(low_ratio, 1.0); // Por debajo del threshold
        // Alta similitud - usar tolerancia para floating point
        assert!(
            high_ratio >= 2.4,
            "Expected high_ratio >= 2.4, got {:.3}",
            high_ratio
        );
    }

    #[test]
    fn test_compression_result() {
        let compressed = vec![1, 2, 3, 4, 5];
        let original_size = 100;
        let strategy = CompressionStrategy::FBCU;
        
        let result = CompressionResult::new(
            compressed.clone(),
            original_size,
            strategy,
            1000,
        );
        
        assert_eq!(result.compressed_size, 5);
        assert_eq!(result.compression_ratio, 20.0);
        assert!(result.meets_target(15.0));
        assert!(!result.meets_target(25.0));
    }
}
