//! QPX Header Structure (48 bytes)
//! 
//! Optimized for CPU cache lines (64 bytes = header + 16 bytes metadata)

use std::io::{Cursor, Read, Write};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use crate::qpx::{QPX_MAGIC, QPX_VERSION, error::*};

/// QPX Header - 48 bytes fixed size
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QPXHeader {
    /// Magic bytes "QPX\0" (4 bytes)
    pub magic: [u8; 4],
    
    /// Version number (2 bytes) - 0x0015 for v1.5
    pub version: u16,
    
    /// Flags (1 byte) - Bit flags for compression, encryption, etc.
    pub flags: u8,
    
    /// Major type (1 byte) - QPXMajorType enum value
    pub major_type: u8,
    
    /// Number of pixels in PixelBlock (4 bytes)
    pub pixel_count: u32,
    
    /// Number of entanglements (2 bytes)
    pub entanglement_count: u16,
    
    /// Number of branches (2 bytes)
    pub branch_count: u16,
    
    /// Offset to PixelBlock from file start (4 bytes)
    pub pixel_block_offset: u32,
    
    /// Offset to QuantumMeta from file start (4 bytes)
    pub quantum_meta_offset: u32,
    
    /// Offset to BranchTable from file start (4 bytes, 0 if unused)
    pub branch_table_offset: u32,
    
    /// Offset to EntanglementMap from file start (4 bytes, 0 if unused)
    pub entanglement_offset: u32,
    
    /// Offset to Timeline from file start (4 bytes, 0 if unused)
    pub timeline_offset: u32,
    
    /// Offset to Context from file start (4 bytes, 0 if unused)
    pub context_offset: u32,
    
    /// Offset to Footer from file start (4 bytes)
    pub footer_offset: u32,
    
    /// Reserved for future use (4 bytes)
    pub reserved: u32,
}

/// QPX Major Types (block type identifier)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QPXMajorType {
    /// Primitive types: bool, int, string, uuid (1-5 bytes)
    Primitive = 0x00,
    
    /// Single pixel (8 bytes)
    Pixel = 0x20,
    
    /// Array of pixels (variable)
    PixelBlock = 0x40,
    
    /// FBCU Core with full metadata (~200 bytes typical) - For BStradivarius templates
    QuantumCore = 0x60,
    
    /// Entanglement reference to another core
    Entanglement = 0x80,
    
    /// QuantumDao branch metadata
    Branch = 0xA0,
    
    /// Reserved for future use
    Reserved1 = 0xC0,
    
    /// Reserved for future use
    Reserved2 = 0xE0,
}

impl QPXHeader {
    /// Header size in bytes (fixed)
    pub const SIZE: usize = 48;
    
    /// Create new header with given major type
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
    
    /// Serialize header to 48 bytes (little-endian)
    pub fn to_bytes(&self) -> Result<[u8; 48]> {
        let mut buffer = [0u8; 48];
        let mut cursor = Cursor::new(&mut buffer[..]);
        
        // Identification (8 bytes)
        cursor.write_all(&self.magic)?;
        cursor.write_u16::<LittleEndian>(self.version)?;
        cursor.write_u8(self.flags)?;
        cursor.write_u8(self.major_type)?;
        
        // Counts (8 bytes)
        cursor.write_u32::<LittleEndian>(self.pixel_count)?;
        cursor.write_u16::<LittleEndian>(self.entanglement_count)?;
        cursor.write_u16::<LittleEndian>(self.branch_count)?;
        
        // Offsets (32 bytes = 8 x u32)
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
    
    /// Deserialize header from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 48 {
            return Err(QPXError::InvalidHeader(format!(
                "Header too short: {} bytes (expected 48)",
                bytes.len()
            )));
        }
        
        let mut cursor = Cursor::new(bytes);
        
        // Read magic
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic)?;
        
        if magic != QPX_MAGIC {
            return Err(QPXError::InvalidHeader(format!(
                "Invalid magic bytes: expected {:?}, got {:?}",
                QPX_MAGIC, magic
            )));
        }
        
        // Read rest of header
        let version = cursor.read_u16::<LittleEndian>()?;
        let flags = cursor.read_u8()?;
        let major_type = cursor.read_u8()?;
        let pixel_count = cursor.read_u32::<LittleEndian>()?;
        let entanglement_count = cursor.read_u16::<LittleEndian>()?;
        let branch_count = cursor.read_u16::<LittleEndian>()?;
        let pixel_block_offset = cursor.read_u32::<LittleEndian>()?;
        let quantum_meta_offset = cursor.read_u32::<LittleEndian>()?;
        let branch_table_offset = cursor.read_u32::<LittleEndian>()?;
        let entanglement_offset = cursor.read_u32::<LittleEndian>()?;
        let timeline_offset = cursor.read_u32::<LittleEndian>()?;
        let context_offset = cursor.read_u32::<LittleEndian>()?;
        let footer_offset = cursor.read_u32::<LittleEndian>()?;
        let reserved = cursor.read_u32::<LittleEndian>()?;
        
        Ok(Self {
            magic,
            version,
            flags,
            major_type,
            pixel_count,
            entanglement_count,
            branch_count,
            pixel_block_offset,
            quantum_meta_offset,
            branch_table_offset,
            entanglement_offset,
            timeline_offset,
            context_offset,
            footer_offset,
            reserved,
        })
    }
    
    /// Validate header integrity
    pub fn validate(&self) -> Result<()> {
        // Check magic
        if self.magic != QPX_MAGIC {
            return Err(QPXError::InvalidHeader("Invalid magic bytes".into()));
        }
        
        // Check version
        if self.version != QPX_VERSION {
            return Err(QPXError::UnsupportedVersion(self.version, QPX_VERSION));
        }
        
        // Validate major type
        match self.major_type {
            0x00 | 0x20 | 0x40 | 0x60 | 0x80 | 0xA0 | 0xC0 | 0xE0 => {},
            _ => return Err(QPXError::InvalidHeader(format!(
                "Invalid major_type: {:#x}",
                self.major_type
            ))),
        }
        
        // Validate offsets don't overlap with header
        if self.pixel_block_offset > 0 && self.pixel_block_offset < Self::SIZE as u32 {
            return Err(QPXError::InvalidOffset(
                format!("pixel_block_offset ({}) overlaps header", self.pixel_block_offset)
            ));
        }
        
        if self.quantum_meta_offset > 0 && self.quantum_meta_offset < Self::SIZE as u32 {
            return Err(QPXError::InvalidOffset(
                format!("quantum_meta_offset ({}) overlaps header", self.quantum_meta_offset)
            ));
        }
        
        // Validate offsets are in order (if used)
        if self.pixel_block_offset > 0 && self.quantum_meta_offset > 0 {
            if self.pixel_block_offset >= self.quantum_meta_offset {
                return Err(QPXError::InvalidOffset(
                    "pixel_block must come before quantum_meta".into()
                ));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_header_creation() {
        let header = QPXHeader::new(QPXMajorType::QuantumCore);
        assert_eq!(header.magic, QPX_MAGIC);
        assert_eq!(header.version, QPX_VERSION);
        assert_eq!(header.major_type, 0x60);
    }
    
    #[test]
    fn test_header_serialization() {
        let mut header = QPXHeader::new(QPXMajorType::QuantumCore);
        header.pixel_count = 128;
        header.pixel_block_offset = 48;
        header.quantum_meta_offset = 1072;
        header.footer_offset = 2000;
        
        let bytes = header.to_bytes().unwrap();
        assert_eq!(bytes.len(), 48);
        
        // Verify magic
        assert_eq!(&bytes[0..4], b"QPX\0");
        
        // Verify version (little-endian)
        assert_eq!(bytes[4], 0x15);
        assert_eq!(bytes[5], 0x00);
        
        // Verify major_type
        assert_eq!(bytes[7], 0x60);
    }
    
    #[test]
    fn test_header_roundtrip() {
        let mut original = QPXHeader::new(QPXMajorType::QuantumCore);
        original.pixel_count = 256;
        original.entanglement_count = 5;
        original.pixel_block_offset = 48;
        original.quantum_meta_offset = 2096;
        original.footer_offset = 5000;
        
        let bytes = original.to_bytes().unwrap();
        let decoded = QPXHeader::from_bytes(&bytes).unwrap();
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_header_validation() {
        let header = QPXHeader::new(QPXMajorType::QuantumCore);
        assert!(header.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_magic() {
        let mut bytes = [0u8; 48];
        bytes[0..4].copy_from_slice(b"XXXX");
        
        let result = QPXHeader::from_bytes(&bytes);
        assert!(result.is_err());
        
        match result {
            Err(QPXError::InvalidHeader(msg)) => {
                assert!(msg.contains("Invalid magic"));
            }
            _ => panic!("Expected InvalidHeader error"),
        }
    }
    
    #[test]
    fn test_header_too_short() {
        let bytes = [0u8; 20]; // Only 20 bytes
        let result = QPXHeader::from_bytes(&bytes);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_offset() {
        let mut header = QPXHeader::new(QPXMajorType::QuantumCore);
        header.pixel_block_offset = 10; // Overlaps with header (< 48)
        
        let result = header.validate();
        assert!(result.is_err());
        
        match result {
            Err(QPXError::InvalidOffset(msg)) => {
                assert!(msg.contains("overlaps header"));
            }
            _ => panic!("Expected InvalidOffset error"),
        }
    }
}
