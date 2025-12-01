//! QPX Decoder - Decode QPX binary format back to Rust structures
//!
//! Supports decoding:
//! - QuantumCore (0x60) .qpx files → QPXQuantumCore
//! - Future: Voxels, Branches, Entanglements

use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

use crate::fbcu::{FBCUCore, CompressionType, FBCUMetadata};
use crate::qpx::error::{QPXError, Result};
use crate::qpx::header::{QPXHeader, QPXMajorType};
use crate::qpx::pixel::Pixel;
use crate::qpx::encoder::{QPXQuantumCore, TemplateMetadata};
use crate::qpx::{QPX_MAGIC, QPX_MAGIC_END, QPX_VERSION};

/// QPX Decoder for reading .qpx binary files
pub struct QPXDecoder;

impl QPXDecoder {
    /// Decode a QuantumCore .qpx file
    /// 
    /// # Arguments
    /// * `bytes` - Complete .qpx file contents
    /// 
    /// # Returns
    /// * `QPXQuantumCore` with all data reconstructed
    /// 
    /// # Errors
    /// * `InvalidHeader` - Corrupted or invalid header
    /// * `ChecksumMismatch` - File integrity check failed
    /// * `DecodingFailed` - Malformed data
    pub fn decode_quantum_core(bytes: &[u8]) -> Result<QPXQuantumCore> {
        if bytes.len() < QPXHeader::SIZE + 64 {
            return Err(QPXError::DecodingFailed(
                format!("File too small: {} bytes (minimum {})", bytes.len(), QPXHeader::SIZE + 64)
            ));
        }

        // 1. Parse and validate header
        let header = QPXHeader::from_bytes(&bytes[..QPXHeader::SIZE])?;
        header.validate()?;

        if header.major_type != QPXMajorType::QuantumCore as u8 {
            return Err(QPXError::DecodingFailed(
                format!("Expected QuantumCore (0x60), got 0x{:02x}", header.major_type)
            ));
        }

        // 2. Verify file checksum (exclude footer itself)
        let footer_offset = header.footer_offset as usize;
        if footer_offset + 64 != bytes.len() {
            return Err(QPXError::DecodingFailed(
                format!("Footer offset mismatch: expected {}, got {}", 
                    bytes.len() - 64, footer_offset)
            ));
        }

        let data_to_hash = &bytes[..footer_offset];
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        let computed_checksum = hasher.finalize();

        let stored_checksum = &bytes[footer_offset..footer_offset + 32];
        if &computed_checksum[..] != stored_checksum {
            return Err(QPXError::ChecksumMismatch {
                expected: hex::encode(stored_checksum),
                actual: hex::encode(computed_checksum),
            });
        }

        // 3. Verify footer magic
        let magic_end = &bytes[footer_offset + 32..footer_offset + 36];
        if magic_end != QPX_MAGIC_END {
            return Err(QPXError::DecodingFailed(
                format!("Invalid footer magic: expected {:?}, got {:?}", 
                    QPX_MAGIC_END, magic_end)
            ));
        }

        // 4. Read PixelBlock
        let pixel_block_offset = header.pixel_block_offset as usize;
        let pixel_count = header.pixel_count as usize;
        let pixels = Self::read_pixel_block(bytes, pixel_block_offset, pixel_count)?;

        // 5. Read QuantumMeta
        let quantum_meta_offset = header.quantum_meta_offset as usize;
        let (fbcu_core, metadata, checksum, timestamp) = 
            Self::read_quantum_meta(bytes, quantum_meta_offset)?;

        // 6. Determine alpha from first pixel (convention: all core pixels same alpha)
        let alpha = pixels.first()
            .map(|p| p.alpha)
            .unwrap_or(255);

        Ok(QPXQuantumCore {
            fbcu_core,
            pixels,
            alpha,
            metadata,
            timestamp,
            checksum,
        })
    }

    /// Read PixelBlock from bytes
    fn read_pixel_block(bytes: &[u8], offset: usize, count: usize) -> Result<Vec<Pixel>> {
        let mut pixels = Vec::with_capacity(count);
        let mut cursor = offset;

        for i in 0..count {
            if cursor + Pixel::SIZE > bytes.len() {
                return Err(QPXError::DecodingFailed(
                    format!("Truncated PixelBlock: pixel {} at offset {}", i, cursor)
                ));
            }

            let pixel = Pixel::from_bytes(&bytes[cursor..cursor + Pixel::SIZE])?;
            pixels.push(pixel);
            cursor += Pixel::SIZE;
        }

        Ok(pixels)
    }

    /// Read QuantumMeta block from bytes
    fn read_quantum_meta(
        bytes: &[u8], 
        offset: usize
    ) -> Result<(FBCUCore, TemplateMetadata, [u8; 32], DateTime<Utc>)> {
        let mut cursor = Cursor::new(&bytes[offset..]);

        // 1. Read compressed_data
        let compressed_data_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut compressed_data = vec![0u8; compressed_data_len];
        cursor.read_exact(&mut compressed_data)?;

        // 2. Read FBCU metadata
        let original_size = cursor.read_u64::<LittleEndian>()? as usize;
        let compressed_size = cursor.read_u64::<LittleEndian>()? as usize;
        let compression_ratio = cursor.read_f32::<LittleEndian>()? as f64;
        
        // 2b. Read compression type
        let comp_type_byte = cursor.read_u8()?;
        let compression_type = match comp_type_byte {
            0 => CompressionType::None,
            1 => CompressionType::Wavelet,
            2 => CompressionType::Fractal,
            3 => CompressionType::Hybrid,
            4 => CompressionType::QuantumVisual,
            5 => CompressionType::Gzip,
            _ => CompressionType::None, // Fallback
        };

        // 3. Read template metadata
        let concept_name = Self::read_string(&mut cursor)?;
        let category = Self::read_string(&mut cursor)?;
        
        let tags_count = cursor.read_u8()?;
        let mut tags = Vec::with_capacity(tags_count as usize);
        for _ in 0..tags_count {
            tags.push(Self::read_string(&mut cursor)?);
        }
        
        // 3b. Read original filename (Option<String>)
        let original_filename = if cursor.read_u8()? == 1 {
            Some(Self::read_string(&mut cursor)?)
        } else {
            None
        };
        
        // 3c. Read file extension (Option<String>)
        let file_extension = if cursor.read_u8()? == 1 {
            Some(Self::read_string(&mut cursor)?)
        } else {
            None
        };

        // 4. Read timestamps (NOTE: encoder writes created_at twice as created_at and updated_at)
        let created_at = cursor.read_i64::<LittleEndian>()?;
        let _updated_at = cursor.read_i64::<LittleEndian>()?; // Not used currently

        // 5. Read checksum
        let mut checksum = [0u8; 32];
        cursor.read_exact(&mut checksum)?;

        // 6. Construct FBCUCore
        let fbcu_core = FBCUCore {
            id: format!("decoded_{}", chrono::Utc::now().timestamp_millis()),
            compression_type,
            compressed_data,
            original_size,
            compression_ratio,
            metadata: FBCUMetadata {
                compressed_at: chrono::Utc::now().to_rfc3339(),
                compression_time_ms: 0,
                original_hash: hex::encode(&checksum),
                wavelet_level: None,
                fractal_level: None,
            },
        };

        // 7. Construct TemplateMetadata
        let metadata = TemplateMetadata {
            concept_name,
            category,
            tags,
            original_path: String::new(), // Not stored in QPX format (only in VoxelDB index)
            original_filename,
            file_extension,
        };

        // 8. Convert timestamp to DateTime
        let timestamp = DateTime::from_timestamp(created_at, 0)
            .unwrap_or_else(|| chrono::Utc::now());

        Ok((fbcu_core, metadata, checksum, timestamp))
    }

    /// Read a length-prefixed string (u16 length + bytes)
    fn read_string(cursor: &mut Cursor<&[u8]>) -> Result<String> {
        let len = cursor.read_u16::<LittleEndian>()? as usize;
        let mut bytes = vec![0u8; len];
        cursor.read_exact(&mut bytes)?;
        
        String::from_utf8(bytes).map_err(|e| 
            QPXError::DecodingFailed(format!("Invalid UTF-8 string: {}", e))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qpx::encoder::QPXEncoder;

    fn create_test_core() -> QPXQuantumCore {
        let fbcu_core = FBCUCore {
            id: "test_decoder_123".into(),
            compression_type: CompressionType::Hybrid,
            compressed_data: vec![1, 2, 3, 4, 5, 6, 7, 8],
            original_size: 2000,
            compression_ratio: 0.004,
            metadata: FBCUMetadata {
                compressed_at: "2025-11-30T15:00:00Z".into(),
                compression_time_ms: 25,
                original_hash: "def456".into(),
                wavelet_level: Some(5),
                fractal_level: Some(3),
            },
        };

        let pixels = vec![Pixel::new(50, 100, 150, 255); 64];

        QPXQuantumCore {
            fbcu_core,
            pixels,
            alpha: 255,
            metadata: TemplateMetadata {
                concept_name: "Test Concept".into(),
                category: "Testing".into(),
                tags: vec!["test".into(), "decoder".into()],
                original_path: "/tmp/test.md".into(),
            },
            timestamp: chrono::Utc::now(),
            checksum: [0xAB; 32],
        }
    }

    #[test]
    fn test_decode_header() {
        let core = create_test_core();
        let encoded = QPXEncoder::encode_quantum_core(&core).unwrap();

        // Just decode header to verify structure
        let header = QPXHeader::from_bytes(&encoded[..QPXHeader::SIZE]).unwrap();
        assert_eq!(header.magic, QPX_MAGIC);
        assert_eq!(header.version, QPX_VERSION);
        assert_eq!(header.major_type, QPXMajorType::QuantumCore as u8);
        assert_eq!(header.pixel_count, 64);
    }

    #[test]
    fn test_decode_quantum_core() {
        let original = create_test_core();
        let encoded = QPXEncoder::encode_quantum_core(&original).unwrap();

        // Decode back
        let decoded = QPXDecoder::decode_quantum_core(&encoded).unwrap();

        // Verify FBCU data
        assert_eq!(decoded.fbcu_core.compressed_data, original.fbcu_core.compressed_data);
        assert_eq!(decoded.fbcu_core.original_size, original.fbcu_core.original_size);
        // Compression ratio loses precision in f32 encoding, use approximate comparison
        assert!((decoded.fbcu_core.compression_ratio - original.fbcu_core.compression_ratio).abs() < 0.0001,
            "Compression ratio mismatch: {} vs {}", decoded.fbcu_core.compression_ratio, original.fbcu_core.compression_ratio);

        // Verify pixels
        assert_eq!(decoded.pixels.len(), original.pixels.len());
        assert_eq!(decoded.pixels[0], original.pixels[0]);

        // Verify metadata
        assert_eq!(decoded.metadata.concept_name, original.metadata.concept_name);
        assert_eq!(decoded.metadata.category, original.metadata.category);
        assert_eq!(decoded.metadata.tags, original.metadata.tags);
        assert_eq!(decoded.alpha, original.alpha);
    }

    #[test]
    fn test_roundtrip() {
        let original = create_test_core();
        
        // Encode → Decode → Encode again
        let encoded1 = QPXEncoder::encode_quantum_core(&original).unwrap();
        let decoded = QPXDecoder::decode_quantum_core(&encoded1).unwrap();
        let encoded2 = QPXEncoder::encode_quantum_core(&decoded).unwrap();

        // File sizes should be similar (timestamps may differ slightly)
        assert!((encoded1.len() as i64 - encoded2.len() as i64).abs() < 50,
            "Roundtrip size mismatch: {} vs {}", encoded1.len(), encoded2.len());

        // Headers should match (except offsets if sizes differ)
        let header1 = QPXHeader::from_bytes(&encoded1[..QPXHeader::SIZE]).unwrap();
        let header2 = QPXHeader::from_bytes(&encoded2[..QPXHeader::SIZE]).unwrap();
        assert_eq!(header1.magic, header2.magic);
        assert_eq!(header1.version, header2.version);
        assert_eq!(header1.pixel_count, header2.pixel_count);
    }

    #[test]
    fn test_corrupted_magic() {
        let core = create_test_core();
        let mut encoded = QPXEncoder::encode_quantum_core(&core).unwrap();

        // Corrupt header magic
        encoded[0] = 0xFF;

        let result = QPXDecoder::decode_quantum_core(&encoded);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QPXError::InvalidHeader(_)));
    }

    #[test]
    fn test_corrupted_checksum() {
        let core = create_test_core();
        let mut encoded = QPXEncoder::encode_quantum_core(&core).unwrap();

        // Corrupt footer checksum (last 64 bytes, first 32 are checksum)
        let footer_offset = encoded.len() - 64;
        encoded[footer_offset] ^= 0xFF;

        let result = QPXDecoder::decode_quantum_core(&encoded);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QPXError::ChecksumMismatch { .. }));
    }

    #[test]
    fn test_truncated_file() {
        let core = create_test_core();
        let encoded = QPXEncoder::encode_quantum_core(&core).unwrap();

        // Truncate file
        let truncated = &encoded[..encoded.len() / 2];

        let result = QPXDecoder::decode_quantum_core(truncated);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_major_type() {
        let core = create_test_core();
        let mut encoded = QPXEncoder::encode_quantum_core(&core).unwrap();

        // Change major_type in header (byte 7)
        encoded[7] = QPXMajorType::Pixel as u8;

        let result = QPXDecoder::decode_quantum_core(&encoded);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), QPXError::DecodingFailed(_)));
    }
}
