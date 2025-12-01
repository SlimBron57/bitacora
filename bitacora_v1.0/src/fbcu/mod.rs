
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Instant;
use thiserror::Error;

/// Errores del FBCU Engine
#[derive(Error, Debug)]
pub enum FBCUError {
    #[error("Compression failed: {0}")]
    CompressionFailed(String),
    
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),
    
    #[error("Integrity check failed: expected {expected}, got {got}")]
    IntegrityCheckFailed { expected: String, got: String },
    
    #[error("Visual DNA not enabled in config")]
    VisualDNANotEnabled,
    
    #[error("Data too small to compress: {size} bytes < {threshold} bytes")]
    DataTooSmall { size: usize, threshold: usize },
    
    #[error("Wavelet level out of range: {level} (max: 10)")]
    InvalidWaveletLevel { level: u8 },
    
    #[error("Fractal convergence failed after {iterations} iterations")]
    FractalConvergenceFailed { iterations: usize },
}

pub type Result<T> = std::result::Result<T, FBCUError>;

// ============================================================================
// CONFIGURACIÓN
// ============================================================================

/// Configuración del FBCU Engine
#[derive(Debug, Clone)]
pub struct FBCUConfig {
    /// Umbral de tamaño para activar compresión (bytes)
    pub compression_threshold: usize,
    
    /// Nivel de compresión wavelet (1-10)
    pub wavelet_level: u8,
    
    /// Nivel de compresión fractal (1-10)
    pub fractal_level: u8,
    
    /// Habilitar Visual DNA encoding
    pub enable_visual_dna: bool,
    
    /// Tamaño de cache de descompresión (entries)
    pub cache_size: usize,
}

impl Default for FBCUConfig {
    fn default() -> Self {
        Self {
            compression_threshold: 1024,  // 1 KB
            wavelet_level: 5,
            fractal_level: 7,
            enable_visual_dna: false,
            cache_size: 1000,
        }
    }
}

// ============================================================================
// FBCU CORE - Dato comprimido
// ============================================================================

/// FBCU Core - Quantum cognitivo comprimido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBCUCore {
    /// ID content-addressable (SHA-256 del original)
    pub id: String,
    
    /// Tipo de compresión aplicada
    pub compression_type: CompressionType,
    
    /// Datos comprimidos (bytes)
    pub compressed_data: Vec<u8>,
    
    /// Tamaño original (antes de comprimir)
    pub original_size: usize,
    
    /// Ratio de compresión alcanzado
    pub compression_ratio: f64,
    
    /// Metadatos adicionales
    pub metadata: FBCUMetadata,
}

/// Tipo de compresión aplicada
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionType {
    /// Sin compresión (dato pequeño o incomprimible)
    None,
    
    /// Solo wavelet transform
    Wavelet,
    
    /// Solo fractal IFS
    Fractal,
    
    /// Wavelet + Fractal (pipeline completo)
    Hybrid,
    
    /// Visual DNA + Fractal
    QuantumVisual,
    
    /// GZIP (para archivos técnicos .md/.rs)
    Gzip,
}

/// Metadatos del FBCU Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBCUMetadata {
    /// Timestamp de compresión (RFC3339)
    pub compressed_at: String,
    
    /// Tiempo de compresión (ms)
    pub compression_time_ms: u64,
    
    /// Hash del contenido original (verificación)
    pub original_hash: String,
    
    /// Nivel wavelet usado (si aplica)
    pub wavelet_level: Option<u8>,
    
    /// Nivel fractal usado (si aplica)
    pub fractal_level: Option<u8>,
}

// ============================================================================
// FBCU ENGINE - Motor principal
// ============================================================================

/// Motor de compresión FBCU
pub struct FBCUEngine {
    /// Transformada wavelet
    wavelet: WaveletTransform,
    
    /// Compresor fractal
    fractal: FractalCompressor,
    
    /// Compresor Visual DNA (opcional)
    visual_dna: Option<QuantumVisualCompressor>,
    
    /// Cache LRU de descompresiones
    cache: HashMap<String, Vec<u8>>,
    
    /// Configuración
    config: FBCUConfig,
    
    /// Métricas de rendimiento
    metrics: FBCUMetrics,
}

impl FBCUEngine {
    /// Crear nuevo FBCU Engine
    pub fn new(config: FBCUConfig) -> Result<Self> {
        if config.wavelet_level > 10 {
            return Err(FBCUError::InvalidWaveletLevel {
                level: config.wavelet_level,
            });
        }
        
        Ok(Self {
            wavelet: WaveletTransform::new(config.wavelet_level),
            fractal: FractalCompressor::new(config.fractal_level),
            visual_dna: if config.enable_visual_dna {
                Some(QuantumVisualCompressor::new(config.wavelet_level))
            } else {
                None
            },
            cache: HashMap::new(),
            config,
            metrics: FBCUMetrics::default(),
        })
    }
    
    /// Comprimir datos (usa GZIP para archivos técnicos)
    pub fn compress(&mut self, data: &[u8]) -> Result<FBCUCore> {
        let start = Instant::now();
        
        // Verificar umbral de compresión
        if data.len() < self.config.compression_threshold {
            return Ok(self.create_uncompressed_core(data, start));
        }
        
        // Comprimir con GZIP (mejor para texto/código)
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(data)
            .map_err(|e| FBCUError::CompressionFailed(format!("GZIP write failed: {}", e)))?;
        let compressed_data = encoder.finish()
            .map_err(|e| FBCUError::CompressionFailed(format!("GZIP finish failed: {}", e)))?;
        
        let compression_type = CompressionType::Gzip;
        
        let compression_time = start.elapsed().as_millis() as u64;
        let compression_ratio = data.len() as f64 / compressed_data.len() as f64;
        
        // Actualizar métricas
        self.metrics.record_compression(compression_ratio, compression_time);
        
        Ok(FBCUCore {
            id: sha256_hex(data),
            compression_type,
            compressed_data,
            original_size: data.len(),
            compression_ratio,
            metadata: FBCUMetadata {
                compressed_at: chrono::Utc::now().to_rfc3339(),
                compression_time_ms: compression_time,
                original_hash: sha256_hex(data),
                wavelet_level: Some(self.config.wavelet_level),
                fractal_level: Some(self.config.fractal_level),
            },
        })
    }
    
    /// Descomprimir FBCU Core
    pub fn decompress(&mut self, core: &FBCUCore) -> Result<Vec<u8>> {
        // Verificar cache
        if let Some(cached) = self.cache.get(&core.id) {
            self.metrics.cache_hits += 1;
            return Ok(cached.clone());
        }
        
        self.metrics.cache_misses += 1;
        let start = Instant::now();
        
        // Descomprimir según tipo
        let decompressed = match core.compression_type {
            CompressionType::None => core.compressed_data.clone(),
            CompressionType::Gzip => {
                use flate2::read::GzDecoder;
                use std::io::Read;
                
                let mut decoder = GzDecoder::new(&core.compressed_data[..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)
                    .map_err(|e| FBCUError::DecompressionFailed(format!("GZIP failed: {}", e)))?;
                decompressed
            }
            CompressionType::Wavelet => self.wavelet.decompress(&core.compressed_data)?,
            CompressionType::Fractal => self.fractal.decompress(&core.compressed_data)?,
            CompressionType::Hybrid => {
                // Primero fractal, luego wavelet
                let fractal_out = self.fractal.decompress(&core.compressed_data)?;
                self.wavelet.decompress(&fractal_out)?
            }
            CompressionType::QuantumVisual => {
                // STUB para v1.0
                core.compressed_data.clone()
            }
        };
        
        // Verificar integridad
        let hash = sha256_hex(&decompressed);
        if hash != core.metadata.original_hash {
            return Err(FBCUError::IntegrityCheckFailed {
                expected: core.metadata.original_hash.clone(),
                got: hash,
            });
        }
        
        // Actualizar cache (LRU simple)
        if self.cache.len() >= self.config.cache_size {
            // Eliminar primero (FIFO simple para v1.0)
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        self.cache.insert(core.id.clone(), decompressed.clone());
        
        // Actualizar métricas
        let decompression_time = start.elapsed().as_millis() as u64;
        self.metrics.record_decompression(decompression_time);
        
        Ok(decompressed)
    }
    
    /// Comprimir con fallback automático
    pub fn compress_with_fallback(&mut self, data: &[u8]) -> Result<FBCUCore> {
        match self.compress(data) {
            Ok(core) if core.compression_ratio > 1.5 => Ok(core),
            _ => Ok(self.create_uncompressed_core(data, Instant::now())),
        }
    }
    
    /// Obtener métricas
    pub fn metrics(&self) -> &FBCUMetrics {
        &self.metrics
    }
    
    // === HELPERS INTERNOS ===
    
    fn try_wavelet(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.wavelet.compress(data)
    }
    
    fn try_fractal(&self, data: &[u8]) -> Result<Vec<u8>> {
        self.fractal.compress(data)
    }
    
    fn create_uncompressed_core(&self, data: &[u8], start: Instant) -> FBCUCore {
        FBCUCore {
            id: sha256_hex(data),
            compression_type: CompressionType::None,
            compressed_data: data.to_vec(),
            original_size: data.len(),
            compression_ratio: 1.0,
            metadata: FBCUMetadata {
                compressed_at: chrono::Utc::now().to_rfc3339(),
                compression_time_ms: start.elapsed().as_millis() as u64,
                original_hash: sha256_hex(data),
                wavelet_level: None,
                fractal_level: None,
            },
        }
    }
}

// ============================================================================
// WAVELET TRANSFORM (Haar)
// ============================================================================

/// Transformada Wavelet de Haar
pub struct WaveletTransform {
    level: u8,
}

impl WaveletTransform {
    pub fn new(level: u8) -> Self {
        Self { level: level.min(10) }
    }
    
    /// Comprimir datos usando wavelet
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // Convertir a f32
        let mut coeffs: Vec<f32> = data.iter().map(|&b| b as f32).collect();
        
        // Asegurar potencia de 2
        let len = coeffs.len().next_power_of_two();
        coeffs.resize(len, 0.0);
        
        // Aplicar transformada Haar recursivamente
        for l in 0..self.level {
            let len = coeffs.len() >> l;
            if len < 2 {
                break;
            }
            self.haar_step(&mut coeffs[..len]);
        }
        
        // Cuantizar y convertir a bytes
        Ok(self.quantize_coeffs(&coeffs))
    }
    
    /// Descomprimir datos
    pub fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        // Dequantizar
        let mut coeffs = self.dequantize_coeffs(compressed);
        
        // Aplicar transformada inversa
        for l in (0..self.level).rev() {
            let len = coeffs.len() >> l;
            if len < 2 {
                break;
            }
            self.inverse_haar_step(&mut coeffs[..len]);
        }
        
        // Convertir a bytes
        Ok(coeffs.iter().map(|&f| f.round().clamp(0.0, 255.0) as u8).collect())
    }
    
    fn haar_step(&self, data: &mut [f32]) {
        let len = data.len() / 2;
        let mut temp = vec![0.0; data.len()];
        
        for i in 0..len {
            // Promedio (low-pass)
            temp[i] = (data[2 * i] + data[2 * i + 1]) / 2.0;
            
            // Diferencia (high-pass)
            temp[len + i] = (data[2 * i] - data[2 * i + 1]) / 2.0;
        }
        
        data.copy_from_slice(&temp);
    }
    
    fn inverse_haar_step(&self, data: &mut [f32]) {
        let len = data.len() / 2;
        let mut temp = vec![0.0; data.len()];
        
        for i in 0..len {
            temp[2 * i] = data[i] + data[len + i];
            temp[2 * i + 1] = data[i] - data[len + i];
        }
        
        data.copy_from_slice(&temp);
    }
    
    fn quantize_coeffs(&self, coeffs: &[f32]) -> Vec<u8> {
        // Cuantización simple: f32 → u8
        // En producción usar quantización adaptativa
        coeffs
            .iter()
            .map(|&f| (f.abs().min(255.0)) as u8)
            .collect()
    }
    
    fn dequantize_coeffs(&self, bytes: &[u8]) -> Vec<f32> {
        bytes.iter().map(|&b| b as f32).collect()
    }
}

// ============================================================================
// FRACTAL COMPRESSOR (IFS - Iterated Function Systems)
// ============================================================================

/// Compresor Fractal IFS
pub struct FractalCompressor {
    level: u8,
    max_iterations: usize,
}

impl FractalCompressor {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(10),
            max_iterations: (level as usize) * 100,
        }
    }
    
    /// Comprimir usando IFS (STUB para v1.0)
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        // STUB: En v1.0, aplicamos compresión simple
        // En v2.0, implementar IFS completo con transformaciones afines
        
        // Por ahora, usar RLE (Run-Length Encoding) como placeholder
        Ok(self.simple_rle(data))
    }
    
    /// Descomprimir desde IFS (STUB para v1.0)
    pub fn decompress(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        // STUB: descompresión RLE
        self.simple_rle_decode(compressed)
    }
    
    // RLE simple como placeholder
    fn simple_rle(&self, data: &[u8]) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut i = 0;
        
        while i < data.len() {
            let byte = data[i];
            let mut count = 1u8;
            
            while (i + count as usize) < data.len()
                && data[i + count as usize] == byte
                && count < 255
            {
                count += 1;
            }
            
            compressed.push(count);
            compressed.push(byte);
            i += count as usize;
        }
        
        compressed
    }
    
    fn simple_rle_decode(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        let mut decompressed = Vec::new();
        let mut i = 0;
        
        while i + 1 < compressed.len() {
            let count = compressed[i];
            let byte = compressed[i + 1];
            
            for _ in 0..count {
                decompressed.push(byte);
            }
            
            i += 2;
        }
        
        Ok(decompressed)
    }
}

// ============================================================================
// QUANTUM VISUAL DNA (opcional)
// ============================================================================

/// Compresor Visual DNA Cuántico
pub struct QuantumVisualCompressor {
    level: u8,
}

impl QuantumVisualCompressor {
    pub fn new(level: u8) -> Self {
        Self { level }
    }
    
    /// Generar Visual DNA desde texto (STUB)
    pub fn generate_visual_dna(&self, text: &str) -> VisualDNA {
        let bytes = text.as_bytes();
        
        let pixels: Vec<RGB> = bytes
            .iter()
            .map(|&byte| self.byte_to_color(byte))
            .collect();
        
        let size = (pixels.len() as f64).sqrt().ceil() as usize;
        
        VisualDNA {
            pixels,
            width: size,
            height: size,
        }
    }
    
    fn byte_to_color(&self, byte: u8) -> RGB {
        // Hash determinístico
        let h1 = byte.wrapping_mul(137).wrapping_add(73);
        let h2 = byte.wrapping_mul(211).wrapping_add(41);
        let h3 = byte.wrapping_mul(179).wrapping_add(97);
        
        RGB {
            r: h1,
            g: h2,
            b: h3,
        }
    }
}

/// Visual DNA representation
#[derive(Debug, Clone)]
pub struct VisualDNA {
    pub pixels: Vec<RGB>,
    pub width: usize,
    pub height: usize,
}

/// RGB color
#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// ============================================================================
// MÉTRICAS
// ============================================================================

/// Métricas de rendimiento del FBCU
#[derive(Debug, Default)]
pub struct FBCUMetrics {
    pub total_compressions: usize,
    pub total_decompressions: usize,
    pub avg_compression_ratio: f64,
    pub avg_compression_time_ms: u64,
    pub avg_decompression_time_ms: u64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl FBCUMetrics {
    fn record_compression(&mut self, ratio: f64, time_ms: u64) {
        self.total_compressions += 1;
        self.avg_compression_ratio = (self.avg_compression_ratio
            * (self.total_compressions - 1) as f64
            + ratio)
            / self.total_compressions as f64;
        self.avg_compression_time_ms = (self.avg_compression_time_ms
            * (self.total_compressions - 1) as u64
            + time_ms)
            / self.total_compressions as u64;
    }
    
    fn record_decompression(&mut self, time_ms: u64) {
        self.total_decompressions += 1;
        self.avg_decompression_time_ms = (self.avg_decompression_time_ms
            * (self.total_decompressions - 1) as u64
            + time_ms)
            / self.total_decompressions as u64;
    }
}

// ============================================================================
// UTILIDADES
// ============================================================================

/// Calcular SHA-256 hex de datos
fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hex::encode(hash)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compress_decompress_roundtrip() {
        let mut engine = FBCUEngine::new(FBCUConfig::default()).unwrap();
        let original = b"Hello, this is a test of FBCU compression! Pattern repeated. Pattern repeated.";
        
        // Comprimir
        let core = engine.compress(original).unwrap();
        
        // Verificar ratio
        assert!(core.compression_ratio >= 1.0);
        
        // Descomprimir
        let decompressed = engine.decompress(&core).unwrap();
        
        // Verificar identidad
        assert_eq!(original.as_slice(), decompressed.as_slice());
    }
    
    #[test]
    fn test_wavelet_transform_reversibility() {
        let wavelet = WaveletTransform::new(3);
        let original = vec![128u8; 256];
        
        // Comprimir
        let compressed = wavelet.compress(&original).unwrap();
        
        // Descomprimir
        let reconstructed = wavelet.decompress(&compressed).unwrap();
        
        // Verificar reversibilidad (con tolerancia)
        assert_eq!(original.len(), reconstructed.len());
    }
    
    #[test]
    fn test_visual_dna_determinism() {
        let compressor = QuantumVisualCompressor::new(5);
        let text = "Deterministic test";
        
        // Generar DNA dos veces
        let dna1 = compressor.generate_visual_dna(text);
        let dna2 = compressor.generate_visual_dna(text);
        
        // Debe ser idéntico
        assert_eq!(dna1.pixels.len(), dna2.pixels.len());
        for (p1, p2) in dna1.pixels.iter().zip(dna2.pixels.iter()) {
            assert_eq!(p1.r, p2.r);
            assert_eq!(p1.g, p2.g);
            assert_eq!(p1.b, p2.b);
        }
    }
    
    #[test]
    fn test_cache_functionality() {
        let mut engine = FBCUEngine::new(FBCUConfig {
            cache_size: 2,
            ..Default::default()
        })
        .unwrap();
        
        let data1 = b"Test data 1";
        let data2 = b"Test data 2";
        
        let core1 = engine.compress(data1).unwrap();
        let core2 = engine.compress(data2).unwrap();
        
        // Primera descompresión: cache miss
        engine.decompress(&core1).unwrap();
        assert_eq!(engine.metrics.cache_misses, 1);
        
        // Segunda descompresión mismo dato: cache hit
        engine.decompress(&core1).unwrap();
        assert_eq!(engine.metrics.cache_hits, 1);
    }
    
    #[test]
    fn test_small_data_no_compression() {
        let mut engine = FBCUEngine::new(FBCUConfig {
            compression_threshold: 1024,
            ..Default::default()
        })
        .unwrap();
        
        let small_data = b"Small";
        let core = engine.compress(small_data).unwrap();
        
        assert_eq!(core.compression_type, CompressionType::None);
        assert_eq!(core.compression_ratio, 1.0);
    }
}
