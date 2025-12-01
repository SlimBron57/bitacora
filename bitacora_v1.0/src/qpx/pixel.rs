//! QPX Pixel Structure (8 bytes)
//! 
//! Represents a quantum pixel with:
//! - RGB channels (semantic/emotional/temporal dimensions)
//! - Alpha channel (multi-purpose: relevance, intensity, progress, etc.)
//! - Flags (special states)
//! - Entropy (uncertainty measure)
//! - Index (pixel position in group)

use std::io::{Cursor, Read, Write};
use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use crate::qpx::error::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    /// Red channel - Semantic dimension (0-255)
    pub r: u8,
    
    /// Green channel - Emotional dimension (0-255)
    pub g: u8,
    
    /// Blue channel - Temporal dimension (0-255)
    pub b: u8,
    
    /// Alpha channel - Multi-purpose (0-255)
    /// 
    /// Context-dependent meanings:
    /// - **Templates:** 255=core, 128=helper, 50=deprecated
    /// - **Memories:** Emotional intensity (255=trauma, 50=routine)
    /// - **Tasks:** Progress (255=100%, 128=50%, 0=not started)
    /// - **Branches:** Probability (220=86% success, 80=31% experimental)
    pub alpha: u8,
    
    /// Flags for special states
    pub flags: u8,
    
    /// Entropy - Uncertainty measure (0-65535)
    pub entropy: u16,
    
    /// Index - Pixel position in group
    pub index: u8,
}

impl Pixel {
    /// Size in bytes when serialized
    pub const SIZE: usize = 8;
    
    /// Create new pixel with RGBA values
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
    
    /// Create pixel with all fields
    pub fn with_full_metadata(
        r: u8,
        g: u8,
        b: u8,
        alpha: u8,
        flags: u8,
        entropy: u16,
        index: u8,
    ) -> Self {
        Self { r, g, b, alpha, flags, entropy, index }
    }
    
    /// Serialize pixel to bytes (8 bytes, little-endian)
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
    
    /// Deserialize pixel from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            return Err(QPXError::InvalidPixel(format!(
                "Pixel data too short: {} bytes (expected 8)",
                bytes.len()
            )));
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
    
    /// Get pixel as RGB tuple
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    
    /// Get pixel as RGBA tuple
    pub fn rgba(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.alpha)
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pixel_creation() {
        let pixel = Pixel::new(180, 150, 255, 200);
        assert_eq!(pixel.r, 180);
        assert_eq!(pixel.g, 150);
        assert_eq!(pixel.b, 255);
        assert_eq!(pixel.alpha, 200);
        assert_eq!(pixel.flags, 0);
        assert_eq!(pixel.entropy, 0);
        assert_eq!(pixel.index, 0);
    }
    
    #[test]
    fn test_pixel_serialization() {
        let pixel = Pixel::with_full_metadata(100, 150, 200, 255, 1, 1000, 5);
        let bytes = pixel.to_bytes().unwrap();
        
        assert_eq!(bytes.len(), 8);
        assert_eq!(bytes[0], 100);  // r
        assert_eq!(bytes[1], 150);  // g
        assert_eq!(bytes[2], 200);  // b
        assert_eq!(bytes[3], 255);  // alpha
        assert_eq!(bytes[4], 1);    // flags
        // entropy (u16 LE): 1000 = 0x03E8 = [0xE8, 0x03]
        assert_eq!(bytes[5], 0xE8);
        assert_eq!(bytes[6], 0x03);
        assert_eq!(bytes[7], 5);    // index
    }
    
    #[test]
    fn test_pixel_roundtrip() {
        let original = Pixel::with_full_metadata(42, 84, 126, 168, 2, 5000, 10);
        let bytes = original.to_bytes().unwrap();
        let decoded = Pixel::from_bytes(&bytes).unwrap();
        
        assert_eq!(original, decoded);
    }
    
    #[test]
    fn test_pixel_too_short() {
        let bytes = [1, 2, 3]; // Only 3 bytes
        let result = Pixel::from_bytes(&bytes);
        
        assert!(result.is_err());
        match result {
            Err(QPXError::InvalidPixel(msg)) => {
                assert!(msg.contains("too short"));
            }
            _ => panic!("Expected InvalidPixel error"),
        }
    }
    
    #[test]
    fn test_pixel_accessors() {
        let pixel = Pixel::new(10, 20, 30, 40);
        assert_eq!(pixel.rgb(), (10, 20, 30));
        assert_eq!(pixel.rgba(), (10, 20, 30, 40));
    }
}
