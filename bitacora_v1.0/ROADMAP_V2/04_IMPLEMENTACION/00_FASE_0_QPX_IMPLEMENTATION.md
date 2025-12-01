# FASE 0: QPX (Quantum Pixel eXchange) Implementation

**Fecha:** 2025-11-30  
**Prioridad:** 🔴 CRÍTICA - BLOQUEADOR para BStradivarius v2.0  
**Duración:** 3 días (15h)  
**Estado:** 🚧 EN PROGRESO

---

## 🎯 OBJETIVO

Implementar formato nativo `.qpx` (Quantum Pixel eXchange) para todo el ecosistema Bitácora.

**Alcance:**
- QPXEncoder (QuantumCore type 0x60)
- QPXDecoder (with validation)
- VoxelDB integration (read/write .qpx files)
- Tests roundtrip (markdown → QPX → markdown)

**Beneficiarios:**
- ✅ BStradivarius v2.0 (templates storage)
- ✅ VoxelDB (embeddings storage)
- ✅ TelescopeDB (cores storage)
- ✅ QuantumDao (branches storage)

---

## 📐 ESPECIFICACIÓN QPX

### Header Structure (48 bytes)

```rust
pub struct QPXHeader {
    // Identification (8 bytes)
    pub magic: [u8; 4],              // "QPX\0" (0x51 0x50 0x58 0x00)
    pub version: u16,                // 0x0015 for v1.5
    pub flags: u8,                   // Bit flags (compression, encryption)
    pub major_type: u8,              // QPXMajorType (0x60 = QuantumCore)
    
    // Counts (8 bytes)
    pub pixel_count: u32,            // Number of pixels in PixelBlock
    pub entanglement_count: u16,     // Number of entanglements
    pub branch_count: u16,           // Number of branches (0 for templates)
    
    // Block Offsets (7 x u64 = 56 bytes total, but fits in remaining 32 bytes as u32)
    // We'll use u64 for future-proofing
    pub pixel_block_offset: u64,     // Offset to PixelBlock
    pub quantum_meta_offset: u64,    // Offset to QuantumMeta
    pub branch_table_offset: u64,    // Offset to BranchTable (0 if unused)
    pub entanglement_offset: u64,    // Offset to EntanglementMap (0 if unused)
    pub timeline_offset: u64,        // Offset to Timeline (0 if unused)
    pub context_offset: u64,         // Offset to Context (0 if unused)
    pub footer_offset: u64,          // Offset to Footer
}

// Total: 8 + 8 + 7*8 = 72 bytes
// We'll pack this to 48 bytes by using u32 offsets
```

### Optimized Header (48 bytes)

```rust
#[repr(C, packed)]
pub struct QPXHeader {
    // Identification (8 bytes)
    pub magic: [u8; 4],              // "QPX\0"
    pub version: u16,                // 0x0015
    pub flags: u8,
    pub major_type: u8,
    
    // Counts (8 bytes)
    pub pixel_count: u32,
    pub entanglement_count: u16,
    pub branch_count: u16,
    
    // Block Offsets (32 bytes = 8 x u32)
    pub pixel_block_offset: u32,
    pub quantum_meta_offset: u32,
    pub branch_table_offset: u32,
    pub entanglement_offset: u32,
    pub timeline_offset: u32,
    pub context_offset: u32,
    pub footer_offset: u32,
    pub reserved: u32,               // Padding to 48 bytes
}
```

### QPXMajorType Enum

```rust
#[repr(u8)]
pub enum QPXMajorType {
    Primitive = 0x00,       // bool, int, string, uuid
    Pixel = 0x20,           // Single pixel (5 bytes)
    PixelBlock = 0x40,      // Array of pixels
    QuantumCore = 0x60,     // FBCU Core (Full Mode) ← Para BStradivarius
    Entanglement = 0x80,    // Entanglement reference
    Branch = 0xA0,          // Branch metadata
    Reserved1 = 0xC0,
    Reserved2 = 0xE0,
}
```

### Pixel Structure (8 bytes)

```rust
#[repr(C, packed)]
pub struct Pixel {
    pub r: u8,              // Red channel (semantic dimension)
    pub g: u8,              // Green channel (emotional dimension)
    pub b: u8,              // Blue channel (temporal dimension)
    pub alpha: u8,          // Multi-purpose (relevance, intensity, etc.)
    pub flags: u8,          // Special states
    pub entropy: u16,       // Uncertainty measure (0-65535)
    pub index: u8,          // Pixel index in group
}
```

### QuantumMeta Block Structure

```rust
pub struct QuantumMeta {
    // FBCU compressed data
    pub compressed_data_len: u32,
    pub compressed_data: Vec<u8>,
    
    // Original metadata
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f32,
    
    // Template metadata
    pub concept_name_len: u16,
    pub concept_name: String,
    pub category_len: u16,
    pub category: String,
    pub tags_count: u8,
    pub tags: Vec<String>,
    
    // Timestamps
    pub created_at: i64,
    pub updated_at: i64,
    
    // Checksum
    pub checksum: [u8; 32],         // SHA256 of original content
}
```

### Footer (64 bytes)

```rust
#[repr(C, packed)]
pub struct QPXFooter {
    pub file_checksum: [u8; 32],    // SHA256 of entire file (header to footer-64)
    pub magic_end: [u8; 4],         // "QPX\xFF" verification
    pub file_size: u64,             // Total file size
    pub padding: [u8; 20],          // Reserved for future use
}
```

---

## 🏗️ IMPLEMENTACIÓN

### Día 0: QPXEncoder (5-6h)

#### Archivos a crear:

```
src/
└── qpx/
    ├── mod.rs              ← Public exports
    ├── header.rs           ← QPXHeader struct + encoding
    ├── encoder.rs          ← QPXEncoder implementation
    ├── pixel.rs            ← Pixel struct
    └── error.rs            ← QPXError types
```

#### src/qpx/mod.rs

```rust
//! QPX (Quantum Pixel eXchange) - Bitácora Native Format
//! 
//! Version: 1.5
//! Specification: ROADMAP_V2/01_ARQUITECTURA/14_qpx-quantumdao-revolucion.md

pub mod header;
pub mod encoder;
pub mod decoder;
pub mod pixel;
pub mod error;

pub use header::{QPXHeader, QPXMajorType};
pub use encoder::QPXEncoder;
pub use decoder::QPXDecoder;
pub use pixel::Pixel;
pub use error::{QPXError, Result};

// Version constants
pub const QPX_VERSION: u16 = 0x0015;  // v1.5
pub const QPX_MAGIC: [u8; 4] = [0x51, 0x50, 0x58, 0x00];  // "QPX\0"
pub const QPX_MAGIC_END: [u8; 4] = [0x51, 0x50, 0x58, 0xFF];  // "QPX\xFF"
```

#### src/qpx/header.rs

```rust
use std::io::{Write, Cursor};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use crate::qpx::{QPX_MAGIC, QPX_VERSION, error::*};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct QPXHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub flags: u8,
    pub major_type: u8,
    pub pixel_count: u32,
    pub entanglement_count: u16,
    pub branch_count: u16,
    pub pixel_block_offset: u32,
    pub quantum_meta_offset: u32,
    pub branch_table_offset: u32,
    pub entanglement_offset: u32,
    pub timeline_offset: u32,
    pub context_offset: u32,
    pub footer_offset: u32,
    pub reserved: u32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QPXMajorType {
    Primitive = 0x00,
    Pixel = 0x20,
    PixelBlock = 0x40,
    QuantumCore = 0x60,
    Entanglement = 0x80,
    Branch = 0xA0,
    Reserved1 = 0xC0,
    Reserved2 = 0xE0,
}

impl QPXHeader {
    pub const SIZE: usize = 48;
    
    pub fn new(major_type: QPXMajorType) -> Self {
        Self {
            magic: QPX_MAGIC,
            version: QPX_VERSION,
            flags: 0,
            major_type: major_type as u8,
            pixel_count: 0,
            entanglement_count: 0,
            branch_count: 0,
            pixel_block_offset: 0,
            quantum_meta_offset: 0,
            branch_table_offset: 0,
            entanglement_offset: 0,
            timeline_offset: 0,
            context_offset: 0,
            footer_offset: 0,
            reserved: 0,
        }
    }
    
    pub fn to_bytes(&self) -> Result<[u8; 48]> {
        let mut buffer = [0u8; 48];
        let mut cursor = Cursor::new(&mut buffer[..]);
        
        cursor.write_all(&self.magic)?;
        cursor.write_u16::<LittleEndian>(self.version)?;
        cursor.write_u8(self.flags)?;
        cursor.write_u8(self.major_type)?;
        cursor.write_u32::<LittleEndian>(self.pixel_count)?;
        cursor.write_u16::<LittleEndian>(self.entanglement_count)?;
        cursor.write_u16::<LittleEndian>(self.branch_count)?;
        cursor.write_u32::<LittleEndian>(self.pixel_block_offset)?;
        cursor.write_u32::<LittleEndian>(self.quantum_meta_offset)?;
        cursor.write_u32::<LittleEndian>(self.branch_table_offset)?;
        cursor.write_u32::<LittleEndian>(self.entanglement_offset)?;
        cursor.write_u32::<LittleEndian>(self.timeline_offset)?;
        cursor.write_u32::<LittleEndian>(self.context_offset)?;
        cursor.write_u32::<LittleEndian>(self.footer_offset)?;
        cursor.write_u32::<LittleEndian>(self.reserved)?;
        
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 48 {
            return Err(QPXError::InvalidHeader("Header too short".into()));
        }
        
        let mut cursor = Cursor::new(bytes);
        
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic)?;
        
        if magic != QPX_MAGIC {
            return Err(QPXError::InvalidHeader(format!(
                "Invalid magic bytes: expected {:?}, got {:?}",
                QPX_MAGIC, magic
            )));
        }
        
        Ok(Self {
            magic,
            version: cursor.read_u16::<LittleEndian>()?,
            flags: cursor.read_u8()?,
            major_type: cursor.read_u8()?,
            pixel_count: cursor.read_u32::<LittleEndian>()?,
            entanglement_count: cursor.read_u16::<LittleEndian>()?,
            branch_count: cursor.read_u16::<LittleEndian>()?,
            pixel_block_offset: cursor.read_u32::<LittleEndian>()?,
            quantum_meta_offset: cursor.read_u32::<LittleEndian>()?,
            branch_table_offset: cursor.read_u32::<LittleEndian>()?,
            entanglement_offset: cursor.read_u32::<LittleEndian>()?,
            timeline_offset: cursor.read_u32::<LittleEndian>()?,
            context_offset: cursor.read_u32::<LittleEndian>()?,
            footer_offset: cursor.read_u32::<LittleEndian>()?,
            reserved: cursor.read_u32::<LittleEndian>()?,
        })
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.magic != QPX_MAGIC {
            return Err(QPXError::InvalidHeader("Invalid magic".into()));
        }
        
        if self.version != QPX_VERSION {
            return Err(QPXError::UnsupportedVersion(self.version));
        }
        
        // Validate offsets are in order
        if self.pixel_block_offset > 0 && self.pixel_block_offset < QPXHeader::SIZE as u32 {
            return Err(QPXError::InvalidHeader("pixel_block_offset overlaps header".into()));
        }
        
        Ok(())
    }
}
```

#### src/qpx/pixel.rs

```rust
use std::io::{Write, Cursor};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use crate::qpx::error::*;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub alpha: u8,
    pub flags: u8,
    pub entropy: u16,
    pub index: u8,
}

impl Pixel {
    pub const SIZE: usize = 8;
    
    pub fn new(r: u8, g: u8, b: u8, alpha: u8) -> Self {
        Self {
            r,
            g,
            b,
            alpha,
            flags: 0,
            entropy: 0,
            index: 0,
        }
    }
    
    pub fn to_bytes(&self) -> Result<[u8; 8]> {
        let mut buffer = [0u8; 8];
        let mut cursor = Cursor::new(&mut buffer[..]);
        
        cursor.write_u8(self.r)?;
        cursor.write_u8(self.g)?;
        cursor.write_u8(self.b)?;
        cursor.write_u8(self.alpha)?;
        cursor.write_u8(self.flags)?;
        cursor.write_u16::<LittleEndian>(self.entropy)?;
        cursor.write_u8(self.index)?;
        
        Ok(buffer)
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(QPXError::InvalidPixel("Pixel data too short".into()));
        }
        
        let mut cursor = Cursor::new(bytes);
        
        Ok(Self {
            r: cursor.read_u8()?,
            g: cursor.read_u8()?,
            b: cursor.read_u8()?,
            alpha: cursor.read_u8()?,
            flags: cursor.read_u8()?,
            entropy: cursor.read_u16::<LittleEndian>()?,
            index: cursor.read_u8()?,
        })
    }
}
```

#### src/qpx/error.rs

```rust
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QPXError {
    #[error("Invalid QPX header: {0}")]
    InvalidHeader(String),
    
    #[error("Unsupported QPX version: {0:#x}")]
    UnsupportedVersion(u16),
    
    #[error("Invalid pixel data: {0}")]
    InvalidPixel(String),
    
    #[error("Encoding failed: {0}")]
    EncodingFailed(String),
    
    #[error("Decoding failed: {0}")]
    DecodingFailed(String),
    
    #[error("Checksum mismatch")]
    ChecksumMismatch,
    
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, QPXError>;
```

#### src/qpx/encoder.rs

```rust
use std::io::{Write, Cursor};
use byteorder::{LittleEndian, WriteBytesExt};
use sha2::{Sha256, Digest};
use crate::fbcu::FBCUCore;
use crate::qpx::{QPXHeader, QPXMajorType, Pixel, QPX_MAGIC_END, error::*};

pub struct QPXEncoder;

impl QPXEncoder {
    /// Encode FBCUCore to QPX QuantumCore format
    pub fn encode_quantum_core(core: &FBCUCore) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        
        // 1. Calculate offsets
        let header_size = QPXHeader::SIZE as u32;
        let pixel_block_offset = header_size;
        let pixel_block_size = (core.pixels.len() * Pixel::SIZE) as u32;
        let quantum_meta_offset = pixel_block_offset + pixel_block_size;
        
        // Estimate quantum_meta size
        let meta_size = Self::estimate_meta_size(core);
        let footer_offset = quantum_meta_offset + meta_size;
        
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
        Self::write_quantum_meta(&mut buffer, core)?;
        
        // 6. Write Footer
        Self::write_footer(&mut buffer)?;
        
        Ok(buffer)
    }
    
    fn estimate_meta_size(core: &FBCUCore) -> u32 {
        let base = 4 + core.compressed_data.len() + 8 + 8 + 4; // compressed_data_len + data + sizes + ratio
        let metadata = 2 + core.metadata.concept_name.len() +
                      2 + core.metadata.category.len() +
                      1 + core.metadata.tags.iter().map(|t| 2 + t.len()).sum::<usize>();
        let timestamps = 8 + 8;
        let checksum = 32;
        
        (base + metadata + timestamps + checksum) as u32
    }
    
    fn write_quantum_meta(buffer: &mut Vec<u8>, core: &FBCUCore) -> Result<()> {
        let mut cursor = Cursor::new(buffer);
        cursor.set_position(cursor.get_ref().len() as u64);
        
        // Compressed data
        cursor.write_u32::<LittleEndian>(core.compressed_data.len() as u32)?;
        cursor.write_all(&core.compressed_data)?;
        
        // Sizes
        cursor.write_u64::<LittleEndian>(core.original_size)?;
        cursor.write_u64::<LittleEndian>(core.compressed_size)?;
        
        // Compression ratio
        let ratio = if core.original_size > 0 {
            core.compressed_size as f32 / core.original_size as f32
        } else {
            0.0
        };
        cursor.write_f32::<LittleEndian>(ratio)?;
        
        // Metadata
        cursor.write_u16::<LittleEndian>(core.metadata.concept_name.len() as u16)?;
        cursor.write_all(core.metadata.concept_name.as_bytes())?;
        
        cursor.write_u16::<LittleEndian>(core.metadata.category.len() as u16)?;
        cursor.write_all(core.metadata.category.as_bytes())?;
        
        cursor.write_u8(core.metadata.tags.len() as u8)?;
        for tag in &core.metadata.tags {
            cursor.write_u16::<LittleEndian>(tag.len() as u16)?;
            cursor.write_all(tag.as_bytes())?;
        }
        
        // Timestamps
        cursor.write_i64::<LittleEndian>(core.timestamp.timestamp())?;
        cursor.write_i64::<LittleEndian>(core.timestamp.timestamp())?; // updated_at same as created
        
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
        
        // File checksum
        cursor.write_all(&file_checksum)?;
        
        // Magic end
        cursor.write_all(&QPX_MAGIC_END)?;
        
        // File size
        let file_size = cursor.position() + 8 + 20; // +64 total footer
        cursor.write_u64::<LittleEndian>(file_size)?;
        
        // Padding
        cursor.write_all(&[0u8; 20])?;
        
        Ok(())
    }
}
```

#### Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::fbcu::FBCUCore;
    use crate::qpx::Pixel;
    use chrono::Utc;
    
    #[test]
    fn test_encode_header() {
        let header = QPXHeader::new(QPXMajorType::QuantumCore);
        let bytes = header.to_bytes().unwrap();
        
        assert_eq!(bytes.len(), 48);
        assert_eq!(&bytes[0..4], b"QPX\0");
        assert_eq!(bytes[7], 0x60); // QuantumCore type
    }
    
    #[test]
    fn test_encode_pixel() {
        let pixel = Pixel::new(180, 150, 255, 200);
        let bytes = pixel.to_bytes().unwrap();
        
        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0], 180); // r
        assert_eq!(bytes[3], 200); // alpha
    }
    
    #[test]
    fn test_encode_quantum_core() {
        let core = FBCUCore {
            compressed_data: vec![1, 2, 3, 4],
            original_size: 1000,
            compressed_size: 4,
            pixels: vec![Pixel::new(100, 150, 200, 255); 128],
            alpha: 255,
            metadata: TemplateMetadata {
                concept_name: "test_template".into(),
                category: "test".into(),
                tags: vec!["tag1".into()],
                original_path: "test.md".into(),
            },
            timestamp: Utc::now(),
            checksum: [0u8; 32],
        };
        
        let encoded = QPXEncoder::encode_quantum_core(&core).unwrap();
        
        // Verify header
        assert_eq!(&encoded[0..4], b"QPX\0");
        
        // Verify has content
        assert!(encoded.len() > 48); // Header + data
        
        // Verify footer magic
        let footer_start = encoded.len() - 64;
        let footer_magic_pos = footer_start + 32;
        assert_eq!(&encoded[footer_magic_pos..footer_magic_pos+4], b"QPX\xFF");
    }
}
```

---

### Tasks para Día 0

- [ ] Crear src/qpx/mod.rs
- [ ] Crear src/qpx/header.rs
- [ ] Crear src/qpx/pixel.rs
- [ ] Crear src/qpx/error.rs
- [ ] Crear src/qpx/encoder.rs
- [ ] Agregar dependencias a Cargo.toml (byteorder, sha2)
- [ ] Tests unitarios header
- [ ] Tests unitarios pixel
- [ ] Test encode_quantum_core
- [ ] Documentación inline

**Checkpoint Día 0:** QPXEncoder funcional, tests passing

---

### Día 1: QPXDecoder (5-6h)

Implementar decoder completo en `src/qpx/decoder.rs` con:
- decode_quantum_core()
- Validación checksums
- Error handling corrupted files
- Tests roundtrip

### Día 2: VoxelDB Integration (4-5h)

Modificar `src/voxeldb/mod.rs`:
- write_template_qpx()
- read_template_qpx()
- get_template_path()
- index_template()
- Tests E2E

---

**Documento creado:** 2025-11-30  
**Autor:** Sistema Bitácora - Plan Implementación FASE 0  
**Estado:** 🚧 LISTO PARA EMPEZAR  
**Siguiente:** Crear src/qpx/mod.rs y comenzar implementación
