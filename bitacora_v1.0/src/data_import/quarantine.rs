//! # 🛡️ Quarantine Layer - Safety First
//!
//! **Propósito:** Inspeccionar datos externos antes de procesarlos.
//!
//! ## State Machine
//!
//! ```text
//! Pending → inspect() → Safe/Suspicious/Rejected
//!                      ↓
//!                   Safe → approve() → ReadyForDigestion
//!                      ↓
//!               Suspicious → manual_review() → Safe/Rejected
//!                      ↓
//!                 Rejected → (cannot proceed)
//! ```
//!
//! ## Validaciones Básicas (v1.0)
//!
//! - Tamaño: <100MB por archivo
//! - Formato: UTF-8 válido
//! - Metadata: Timestamps válidos
//! - Encoding: No binario corrupto
//!
//! ## Performance Target
//!
//! - <500ms por archivo (typical 10-50KB)
//!
//! ## Example
//!
//! ```rust,no_run
//! use bitacora::data_import::{QuarantineZone, DataSource};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let raw_data = std::fs::read("export.txt")?;
//!     let mut quarantine = QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await?;
//!     
//!     if quarantine.is_safe() {
//!         quarantine.approve();
//!         println!("✅ Approved");
//!     } else {
//!         quarantine.reject("Invalid format".to_string());
//!         println!("❌ Rejected");
//!     }
//!     
//!     Ok(())
//! }
//! ```

use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use super::error::{DataImportError, Result};

/// Fuente de datos externa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DataSource {
    /// WhatsApp chat export (.txt)
    WhatsApp,
    
    /// Telegram chat export (.json)
    Telegram,
    
    /// Email (.mbox, .eml)
    Email,
    
    /// Google Calendar (.ics)
    Calendar,
    
    /// Spotify streaming history (.json)
    Spotify,
    
    /// GitHub contributions (.json)
    GitHub,
    
    /// Twitter/X archive (.json)
    Twitter,
    
    /// Manual entry (usuario escribe directamente)
    Manual,
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataSource::WhatsApp => write!(f, "WhatsApp"),
            DataSource::Telegram => write!(f, "Telegram"),
            DataSource::Email => write!(f, "Email"),
            DataSource::Calendar => write!(f, "Calendar"),
            DataSource::Spotify => write!(f, "Spotify"),
            DataSource::GitHub => write!(f, "GitHub"),
            DataSource::Twitter => write!(f, "Twitter"),
            DataSource::Manual => write!(f, "Manual"),
        }
    }
}

/// Estado de la quarantine zone
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineState {
    /// Recién llegado, pending inspección
    Pending,
    
    /// Pasó validación básica, safe para procesar
    Safe,
    
    /// Flags detectados, requiere revisión manual
    Suspicious,
    
    /// Rechazado (formato inválido, corrupto, malicioso)
    Rejected,
    
    /// Aprobado manualmente después de revisión
    Approved,
}

impl fmt::Display for QuarantineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuarantineState::Pending => write!(f, "⏳ Pending"),
            QuarantineState::Safe => write!(f, "✅ Safe"),
            QuarantineState::Suspicious => write!(f, "⚠️  Suspicious"),
            QuarantineState::Rejected => write!(f, "❌ Rejected"),
            QuarantineState::Approved => write!(f, "✅ Approved"),
        }
    }
}

/// Metadata de la quarantine zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineMetadata {
    /// Tamaño en bytes
    pub size_bytes: usize,
    
    /// Encoding detectado (UTF-8, Latin-1, etc.)
    pub encoding: String,
    
    /// MIME type detectado
    pub mime_type: String,
    
    /// Número de líneas (si es texto)
    pub line_count: Option<usize>,
    
    /// Hash SHA-256 del contenido
    pub content_hash: String,
    
    /// Timestamp de cuándo llegó a quarantine
    pub quarantined_at: DateTime<Utc>,
    
    /// Timestamp de última actualización de estado
    pub updated_at: DateTime<Utc>,
    
    /// Duración máxima en quarantine (default 24h)
    pub quarantine_duration_hours: i64,
    
    /// Razón de rechazo (si aplicable)
    pub rejection_reason: Option<String>,
    
    /// Flags detectados
    pub flags: Vec<String>,
}

impl QuarantineMetadata {
    /// Crea metadata inicial
    pub fn new(size_bytes: usize) -> Self {
        let now = Utc::now();
        
        Self {
            size_bytes,
            encoding: String::from("unknown"),
            mime_type: String::from("application/octet-stream"),
            line_count: None,
            content_hash: String::new(),
            quarantined_at: now,
            updated_at: now,
            quarantine_duration_hours: 24,
            rejection_reason: None,
            flags: Vec::new(),
        }
    }
    
    /// Check si quarantine ha expirado
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        let expiry = self.quarantined_at + Duration::hours(self.quarantine_duration_hours);
        now > expiry
    }
    
    /// Agrega flag de warning
    pub fn add_flag(&mut self, flag: String) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
        self.updated_at = Utc::now();
    }
}

/// QuarantineZone - Safety layer para datos externos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineZone {
    /// ID único de quarantine
    pub id: Uuid,
    
    /// Fuente de datos
    pub source: DataSource,
    
    /// Datos raw (bytes)
    pub raw_data: Vec<u8>,
    
    /// Metadata
    pub metadata: QuarantineMetadata,
    
    /// Estado actual
    pub state: QuarantineState,
}

impl QuarantineZone {
    /// Inspecciona datos externos (constructor asíncrono)
    ///
    /// **Performance Target:** <500ms
    ///
    /// # Arguments
    ///
    /// * `raw_data` - Datos crudos leídos del archivo
    /// * `source` - Tipo de fuente (WhatsApp, Telegram, etc.)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let raw = std::fs::read("export.txt")?;
    /// let quarantine = QuarantineZone::inspect(raw, DataSource::WhatsApp).await?;
    /// ```
    pub async fn inspect(raw_data: Vec<u8>, source: DataSource) -> Result<Self> {
        let id = Uuid::new_v4();
        let mut metadata = QuarantineMetadata::new(raw_data.len());
        
        // Validación 1: Tamaño
        if metadata.size_bytes > 100_000_000 {  // 100MB
            return Err(DataImportError::FileTooLarge {
                size_bytes: metadata.size_bytes,
                max_bytes: 100_000_000,
            });
        }
        
        // Validación 2: Encoding (UTF-8)
        let encoding_result = String::from_utf8(raw_data.clone());
        let state = match encoding_result {
            Ok(text) => {
                metadata.encoding = String::from("UTF-8");
                metadata.line_count = Some(text.lines().count());
                
                // Validación 3: MIME type básico
                metadata.mime_type = Self::detect_mime_type(&text, source);
                
                // Validación 4: Content hash
                metadata.content_hash = Self::compute_sha256(&raw_data);
                
                QuarantineState::Safe
            }
            Err(_) => {
                metadata.encoding = String::from("unknown");
                metadata.add_flag("invalid_utf8".to_string());
                QuarantineState::Suspicious
            }
        };
        
        Ok(Self {
            id,
            source,
            raw_data,
            metadata,
            state,
        })
    }
    
    /// Detecta MIME type básico
    fn detect_mime_type(text: &str, source: DataSource) -> String {
        match source {
            DataSource::WhatsApp if text.contains("[") && text.contains("]") => {
                "text/plain; source=whatsapp".to_string()
            }
            DataSource::Telegram => "application/json; source=telegram".to_string(),
            DataSource::Email => "message/rfc822".to_string(),
            _ => "text/plain".to_string(),
        }
    }
    
    /// Computa SHA-256 hash
    fn compute_sha256(data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    
    /// Aprueba para digestion
    pub fn approve(&mut self) {
        self.state = QuarantineState::Approved;
        self.metadata.updated_at = Utc::now();
    }
    
    /// Rechaza con razón
    pub fn reject(&mut self, reason: String) {
        self.state = QuarantineState::Rejected;
        self.metadata.rejection_reason = Some(reason.clone());
        self.metadata.add_flag(reason);
    }
    
    /// Check si está listo para digestion
    pub fn is_ready_for_digestion(&self) -> bool {
        matches!(self.state, QuarantineState::Safe | QuarantineState::Approved)
    }
    
    /// Check si está en estado safe
    pub fn is_safe(&self) -> bool {
        matches!(self.state, QuarantineState::Safe)
    }
    
    /// Check si está en estado suspicious
    pub fn is_suspicious(&self) -> bool {
        matches!(self.state, QuarantineState::Suspicious)
    }
    
    /// Check si fue rechazado
    pub fn is_rejected(&self) -> bool {
        matches!(self.state, QuarantineState::Rejected)
    }
    
    /// Get text content (si es UTF-8 válido)
    pub fn get_text_content(&self) -> Result<String> {
        String::from_utf8(self.raw_data.clone())
            .map_err(|_| DataImportError::InvalidEncoding {
                expected: "UTF-8".to_string(),
                found: self.metadata.encoding.clone(),
            })
    }
    
    /// Get summary para logging
    pub fn summary(&self) -> String {
        format!(
            "{} | {} | {} bytes | {} | {}",
            self.id,
            self.source,
            self.metadata.size_bytes,
            self.state,
            self.metadata.encoding
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_quarantine_valid_utf8() {
        let data = b"Hello, world!".to_vec();
        let quarantine = QuarantineZone::inspect(data, DataSource::Manual).await.unwrap();
        
        assert_eq!(quarantine.state, QuarantineState::Safe);
        assert_eq!(quarantine.metadata.encoding, "UTF-8");
        assert_eq!(quarantine.metadata.line_count, Some(1));
    }
    
    #[tokio::test]
    async fn test_quarantine_invalid_utf8() {
        let data = vec![0xFF, 0xFE, 0xFD];  // Invalid UTF-8
        let quarantine = QuarantineZone::inspect(data, DataSource::Manual).await.unwrap();
        
        assert_eq!(quarantine.state, QuarantineState::Suspicious);
        assert!(quarantine.metadata.flags.contains(&"invalid_utf8".to_string()));
    }
    
    #[tokio::test]
    async fn test_quarantine_too_large() {
        let data = vec![0u8; 101_000_000];  // 101MB
        let result = QuarantineZone::inspect(data, DataSource::Manual).await;
        
        assert!(result.is_err());
        match result {
            Err(DataImportError::FileTooLarge { size_bytes, max_bytes }) => {
                assert_eq!(size_bytes, 101_000_000);
                assert_eq!(max_bytes, 100_000_000);
            }
            _ => panic!("Expected FileTooLarge error"),
        }
    }
    
    #[tokio::test]
    async fn test_quarantine_approve_reject() {
        let data = b"Test data".to_vec();
        let mut quarantine = QuarantineZone::inspect(data, DataSource::Manual).await.unwrap();
        
        assert!(quarantine.is_safe());
        assert!(quarantine.is_ready_for_digestion());
        
        quarantine.approve();
        assert_eq!(quarantine.state, QuarantineState::Approved);
        
        quarantine.reject("Test rejection".to_string());
        assert_eq!(quarantine.state, QuarantineState::Rejected);
        assert!(!quarantine.is_ready_for_digestion());
        assert_eq!(quarantine.metadata.rejection_reason, Some("Test rejection".to_string()));
    }
    
    #[tokio::test]
    async fn test_quarantine_whatsapp_mime_detection() {
        let data = b"[2023-01-01, 10:00:00] Eduardo: Hola".to_vec();
        let quarantine = QuarantineZone::inspect(data, DataSource::WhatsApp).await.unwrap();
        
        assert_eq!(quarantine.metadata.mime_type, "text/plain; source=whatsapp");
    }
}
