//! Sistema de compresión de backups

/// Trait para compresión de datos
pub trait Compressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

/// Compresor GZIP
pub struct GzipCompressor;

impl Compressor for GzipCompressor {
    fn compress(&self, _data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!("Implementar compresión GZIP")
    }
    
    fn decompress(&self, _data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        todo!("Implementar descompresión GZIP")
    }
}
