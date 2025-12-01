//! QPX (Quantum Pixel eXchange) - Bitácora Native Format v1.5
//! 
//! **Propósito:** Formato binario nativo para almacenamiento de datos en Bitácora.
//! 
//! **Reemplaza:** CBOR (obsoleto desde v1.5), JSON (nunca fue nativo)
//! 
//! **Especificación completa:** 
//! `ROADMAP_V2/01_ARQUITECTURA/14_qpx-quantumdao-revolucion.md`
//! 
//! ## Características
//! 
//! - **Pixel-native:** Todo dato es representable como pixel (RGBA + metadata)
//! - **QuantumCore:** Estructura completa con header (48 bytes) + blocks
//! - **Alpha multi-purpose:** 255=core, 128=helper, 50=deprecated, etc.
//! - **Entanglements:** Relaciones cuánticas entre cores nativo
//! - **Checksums:** SHA256 para integridad (header + footer)
//! 
//! ## Tipos QPX
//! 
//! - `Primitive (0x00)` - bool, int, string, uuid (1-5 bytes)
//! - `Pixel (0x20)` - Single pixel (8 bytes)
//! - `PixelBlock (0x40)` - Array de pixels
//! - `QuantumCore (0x60)` - FBCU Core completo (~200 bytes) ← Para BStradivarius
//! - `Entanglement (0x80)` - Referencias entre cores
//! - `Branch (0xA0)` - QuantumDao branches
//! 
//! ## Ejemplo Uso
//! 
//! ```rust,ignore
//! use bitacora::qpx::{QPXEncoder, QPXDecoder};
//! use bitacora::fbcu::FBCUCore;
//! 
//! // Encoding
//! let core = FBCUCore { /* ... */ };
//! let qpx_bytes = QPXEncoder::encode_quantum_core(&core)?;
//! 
//! // Write to file
//! std::fs::write("template.qpx", &qpx_bytes)?;
//! 
//! // Decoding
//! let qpx_bytes = std::fs::read("template.qpx")?;
//! let decoded_core = QPXDecoder::decode_quantum_core(&qpx_bytes)?;
//! 
//! assert_eq!(core, decoded_core);
//! ```

pub mod header;
pub mod pixel;
pub mod error;
pub mod encoder;
pub mod decoder;

pub use header::{QPXHeader, QPXMajorType};
pub use pixel::Pixel;
pub use error::{QPXError, Result};
pub use encoder::{QPXEncoder, QPXQuantumCore, TemplateMetadata};
pub use decoder::QPXDecoder;

/// QPX Version 1.5
pub const QPX_VERSION: u16 = 0x0015;

/// Magic bytes "QPX\0"
pub const QPX_MAGIC: [u8; 4] = [0x51, 0x50, 0x58, 0x00];

/// Magic bytes for footer "QPX\xFF"
pub const QPX_MAGIC_END: [u8; 4] = [0x51, 0x50, 0x58, 0xFF];

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constants() {
        assert_eq!(QPX_VERSION, 0x0015);
        assert_eq!(QPX_MAGIC, [0x51, 0x50, 0x58, 0x00]);
        assert_eq!(QPX_MAGIC_END, [0x51, 0x50, 0x58, 0xFF]);
    }
}
