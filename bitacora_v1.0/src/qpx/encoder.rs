//! QPX Encoder - Encode Rust structures to QPX binary format
//!
//! Supports encoding:
//! - FBCUCore → QuantumCore (0x60) .qpx files
//! - Future: Voxels, Branches, Entanglements

use std::io::{Cursor, Write};
use byteorder::{LittleEndian, WriteBytesExt};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

use crate::fbcu::{FBCUCore, CompressionType};
use crate::qpx::{QPXHeader, QPXMajorType, Pixel, QPX_MAGIC_END, error::*};

/// Template metadata for QPX storage
#[derive(Debug, Clone)]
pub struct TemplateMetadata {
    pub concept_name: String,
    pub category: String,
    pub tags: Vec<String>,
    pub original_path: String,
    pub original_filename: Option<String>,
    pub file_extension: Option<String>,
}

/// Extended FBCUCore for QPX encoding (includes embedding pixels)
#[derive(Debug, Clone)]
pub struct QPXQuantumCore {
    /// FBCU compressed data
    pub fbcu_core: FBCUCore,
    
    /// Embedding pixels (384D vector → 128 pixels)
    /// For semantic search in VoxelDB
    pub pixels: Vec<Pixel>,
    
    /// Alpha channel value (255=core, 128=helper, 50=deprecated)
    pub alpha: u8,
    
    /// Template metadata
    pub metadata: TemplateMetadata,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Content checksum (SHA256 of original markdown)
    pub checksum: [u8; 32],
}

pub struct QPXEncoder;

impl QPXEncoder {
    /// Encode QPXQuantumCore to binary .qpx format
    ///
    /// Structure:
    /// - Header (48 bytes)
    /// - PixelBlock (pixels.len() * 8 bytes)
    /// - QuantumMeta (variable)
    /// - Footer (64 bytes)
    pub fn encode_quantum_core(core: &QPXQuantumCore) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        
        // 1. Calculate offsets
        let header_size = QPXHeader::SIZE as u32;
        let pixel_block_offset = header_size;
        let pixel_block_size = (core.pixels.len() * Pixel::SIZE) as u32;
        let quantum_meta_offset = pixel_block_offset + pixel_block_size;
        
        // Estimate quantum_meta size (will be precise after writing)
        let meta_size_estimate = Self::estimate_meta_size(core);
        let footer_offset = quantum_meta_offset + meta_size_estimate;
        
        // 2. Build header
        let mut header = QPXHeader::new(QPXMajorType::QuantumCore);
        header.pixel_count = core.pixels.len() as u32;
        header.pixel_block_offset = pixel_block_offset;
        header.quantum_meta_offset = quantum_meta_offset;
        header.footer_offset = footer_offset;
        
        // 3. Write header
        buffer.extend_from_slice(&header.to_bytes()?);
        
        // 4. Write PixelBlock
        for pixel in &core.pixels {
            buffer.extend_from_slice(&pixel.to_bytes()?);
        }
        
        // 5. Write QuantumMeta
        let meta_start = buffer.len();
        Self::write_quantum_meta(&mut buffer, core)?;
        let actual_meta_size = (buffer.len() - meta_start) as u32;
        
        // 6. Update footer_offset in header if meta size changed
        let actual_footer_offset = quantum_meta_offset + actual_meta_size;
        if actual_footer_offset != footer_offset {
            // Rewrite header with correct footer offset
            let mut updated_header = header;
            updated_header.footer_offset = actual_footer_offset;
            let header_bytes = updated_header.to_bytes()?;
            buffer[0..48].copy_from_slice(&header_bytes);
        }
        
        // 7. Write Footer
        Self::write_footer(&mut buffer)?;
        
        Ok(buffer)
    }
    
    fn estimate_meta_size(core: &QPXQuantumCore) -> u32 {
        // compressed_data_len (4) + data + original_size (8) + compression_ratio (8)
        let fbcu_size = 4 + core.fbcu_core.compressed_data.len() + 8 + 8;
        
        // concept_name_len (2) + name + category_len (2) + category + tags_count (1) + tags
        let metadata_size = 2 + core.metadata.concept_name.len() +
                           2 + core.metadata.category.len() +
                           1 + core.metadata.tags.iter().map(|t| 2 + t.len()).sum::<usize>();
        
        // timestamps (8 + 8) + checksum (32)
        let misc_size = 16 + 32;
        
        (fbcu_size + metadata_size + misc_size) as u32
    }
    
    fn write_quantum_meta(buffer: &mut Vec<u8>, core: &QPXQuantumCore) -> Result<()> {
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(cursor.get_ref().len() as u64);
        
        // FBCU compressed data
        cursor.write_u32::<LittleEndian>(core.fbcu_core.compressed_data.len() as u32)?;
        cursor.write_all(&core.fbcu_core.compressed_data)?;
        
        // Sizes
        cursor.write_u64::<LittleEndian>(core.fbcu_core.original_size as u64)?;
        cursor.write_u64::<LittleEndian>(core.fbcu_core.compressed_data.len() as u64)?;
        
        // Compression ratio (as f32)
        cursor.write_f32::<LittleEndian>(core.fbcu_core.compression_ratio as f32)?;
        
        // Compression type (1 byte)
        let comp_type = match core.fbcu_core.compression_type {
            CompressionType::None => 0u8,
            CompressionType::Wavelet => 1u8,
            CompressionType::Fractal => 2u8,
            CompressionType::Hybrid => 3u8,
            CompressionType::QuantumVisual => 4u8,
            CompressionType::Gzip => 5u8,
        };
        cursor.write_u8(comp_type)?;
        
        // Template metadata
        cursor.write_u16::<LittleEndian>(core.metadata.concept_name.len() as u16)?;
        cursor.write_all(core.metadata.concept_name.as_bytes())?;
        
        cursor.write_u16::<LittleEndian>(core.metadata.category.len() as u16)?;
        cursor.write_all(core.metadata.category.as_bytes())?;
        
        cursor.write_u8(core.metadata.tags.len() as u8)?;
        for tag in &core.metadata.tags {
            cursor.write_u16::<LittleEndian>(tag.len() as u16)?;
            cursor.write_all(tag.as_bytes())?;
        }
        
        // Original filename (Option<String>)
        if let Some(ref filename) = core.metadata.original_filename {
            cursor.write_u8(1)?; // Has filename
            cursor.write_u16::<LittleEndian>(filename.len() as u16)?;
            cursor.write_all(filename.as_bytes())?;
        } else {
            cursor.write_u8(0)?; // No filename
        }
        
        // File extension (Option<String>)
        if let Some(ref ext) = core.metadata.file_extension {
            cursor.write_u8(1)?; // Has extension
            cursor.write_u16::<LittleEndian>(ext.len() as u16)?;
            cursor.write_all(ext.as_bytes())?;
        } else {
            cursor.write_u8(0)?; // No extension
        }
        
        // Timestamps
        cursor.write_i64::<LittleEndian>(core.timestamp.timestamp())?;
        cursor.write_i64::<LittleEndian>(core.timestamp.timestamp())?; // updated_at = created_at initially
        
        // Checksum
        cursor.write_all(&core.checksum)?;
        
        Ok(())
    }
    
    fn write_footer(buffer: &mut Vec<u8>) -> Result<()> {
        // Calculate file checksum (SHA256 of everything written so far)
        let mut hasher = Sha256::new();
        hasher.update(&buffer);
        let file_checksum = hasher.finalize();
        
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(cursor.get_ref().len() as u64);
        
        // File checksum (32 bytes)
        cursor.write_all(&file_checksum)?;
        
        // Magic end "QPX\xFF" (4 bytes)
        cursor.write_all(&QPX_MAGIC_END)?;
        
        // File size (8 bytes)
        let file_size = cursor.position() + 8 + 20; // Current + file_size field + padding
        cursor.write_u64::<LittleEndian>(file_size)?;
        
        // Padding (20 bytes)
        cursor.write_all(&[0u8; 20])?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fbcu::{FBCUCore, CompressionType, FBCUMetadata};
    
    fn create_test_core() -> QPXQuantumCore {
        // Create test FBCU core
        let fbcu_core = FBCUCore {
            id: "test_id_123".into(),
            compression_type: CompressionType::Hybrid,
            compressed_data: vec![1, 2, 3, 4, 5],
            original_size: 1000,
            compression_ratio: 0.005, // 99.5% compression
            metadata: FBCUMetadata {
                compressed_at: "2025-11-30T12:00:00Z".into(),
                compression_time_ms: 42,
                original_hash: "abc123".into(),
                wavelet_level: Some(6),
                fractal_level: None,
            },
        };
        
        // Create test pixels (128 pixels for 384D embedding)
        let pixels = vec![Pixel::new(100, 150, 200, 255); 128];
        
        QPXQuantumCore {
            fbcu_core,
            pixels,
            alpha: 255,
            metadata: TemplateMetadata {
                concept_name: "test_template".into(),
                category: "test".into(),
                tags: vec!["tag1".into(), "tag2".into()],
                original_path: "test.md".into(),
            },
            timestamp: Utc::now(),
            checksum: [0u8; 32],
        }
    }
    
    #[test]
    fn test_encode_quantum_core() {
        let core = create_test_core();
        let encoded = QPXEncoder::encode_quantum_core(&core).unwrap();
        
        // Verify header magic
        assert_eq!(&encoded[0..4], b"QPX\0");
        
        // Verify major type (QuantumCore = 0x60)
        assert_eq!(encoded[7], 0x60);
        
        // Verify has content beyond header
        assert!(encoded.len() > 48);
        
        // Verify footer magic exists near end
        let has_footer_magic = encoded.windows(4)
            .any(|window| window == b"QPX\xff");
        assert!(has_footer_magic, "Footer magic not found");
    }
    
    #[test]
    fn test_estimate_meta_size() {
        let core = create_test_core();
        let estimate = QPXEncoder::estimate_meta_size(&core);
        
        // Should be reasonable size (> 50 bytes, < 10KB for test data)
        assert!(estimate > 50);
        assert!(estimate < 10000);
    }
    
    #[test]
    fn test_encoded_structure() {
        let core = create_test_core();
        let encoded = QPXEncoder::encode_quantum_core(&core).unwrap();
        
        // Parse header to verify structure
        let header = QPXHeader::from_bytes(&encoded).unwrap();
        header.validate().unwrap();
        
        // Verify pixel count
        assert_eq!(header.pixel_count, 128);
        
        // Verify offsets are sensible
        assert_eq!(header.pixel_block_offset, 48); // Right after header
        assert!(header.quantum_meta_offset > 48); // After header
        assert!(header.footer_offset > header.quantum_meta_offset); // After meta
    }
}
