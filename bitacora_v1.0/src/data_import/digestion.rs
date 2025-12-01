//! # 🔬 Digestion Layer - Source-Specific Processing
//!
//! **Propósito:** Procesar datos respetando la naturaleza única de cada fuente.
//!
//! WhatsApp ≠ Email ≠ Spotify ≠ GitHub
//!
//! ## Hybrid Architecture
//!
//! ```text
//! ┌────────────────────────────────────────────┐
//! │         LAYER 1: HARD-CODED CORE           │
//! │  (Parsing, Validation, Distribution)       │
//! │  → Compiled, Fast, Stable                  │
//! ├────────────────────────────────────────────┤
//! │      LAYER 2: TEMPLATE-BASED LOGIC         │
//! │  (Extraction Rules, Semantic Interpretation)│
//! │  → YAML, Flexible, Evolvable               │
//! ├────────────────────────────────────────────┤
//! │       LAYER 3: HARD-CODED CORE             │
//! │  (Error Handling, Routing, Safety)         │
//! │  → Compiled, Safe, Predictable             │
//! └────────────────────────────────────────────┘
//! ```
//!
//! ## Digesters Disponibles (v1.0)
//!
//! - WhatsAppDigester ✅
//! - TelegramDigester ⏸️
//! - EmailDigester ⏸️
//! - CalendarDigester ⏸️
//! - SpotifyDigester ⏸️
//! - GitHubDigester ⏸️
//! - TwitterDigester ⏸️
//!
//! ## Performance Target
//!
//! - <30s para 1000 mensajes (WhatsApp/Telegram)
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use bitacora::data_import::{QuarantineZone, DataSource};
//! use bitacora::data_import::digestion::{DigestionPipeline, WhatsAppDigester};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Phase 1: Quarantine
//!     let raw_data = std::fs::read("whatsapp_export.txt")?;
//!     let mut quarantine = QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await?;
//!     quarantine.approve();
//!     
//!     // Phase 2: Digestion
//!     let digester = WhatsAppDigester::new();
//!     let digested = digester.digest(&quarantine).await?;
//!     
//!     println!("✅ Digested {} messages", digested.entries.len());
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::error::{DataImportError, Result};
use super::quarantine::{DataSource, QuarantineZone};

// ================================================================
// CORE TRAIT: DigestionPipeline
// ================================================================

/// Trait para todos los digesters source-specific
///
/// **Contract:**
/// 1. `source_type()` - Identifica qué fuente procesa
/// 2. `validate_format()` - Verifica formato antes de digest
/// 3. `digest()` - Procesa datos y retorna DigestedData
///
/// **Performance:**
/// - `validate_format()`: <50ms (sync, fast fail)
/// - `digest()`: <30s for 1000 entries (async, heavy lifting)
#[async_trait]
pub trait DigestionPipeline: Send + Sync {
    /// Tipo de source que este digester procesa
    fn source_type(&self) -> DataSource;
    
    /// Digerir datos de quarantine
    ///
    /// **Input:** QuarantineZone (must be approved)
    /// **Output:** DigestedData (structured entries)
    /// **Errors:** DigestionFailed si formato inválido o parsing falla
    async fn digest(&self, quarantined: &QuarantineZone) -> Result<DigestedData>;
    
    /// Validar formato antes de digerir (fast fail)
    ///
    /// **Input:** Raw bytes
    /// **Output:** Ok(()) si formato válido
    /// **Errors:** DigestionFailed si formato inválido
    fn validate_format(&self, raw_data: &[u8]) -> Result<()>;
}

// ================================================================
// DIGESTED DATA STRUCTURES
// ================================================================

/// Datos digeridos (output de DigestionPipeline)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestedData {
    /// Source original
    pub source: DataSource,
    
    /// Entradas individuales extraídas
    pub entries: Vec<DigestedEntry>,
    
    /// Metadata del proceso de digestión
    pub digestion_metadata: DigestionMetadata,
}

/// Entrada individual digerida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestedEntry {
    /// ID único de la entrada
    pub id: String,
    
    /// Timestamp de la entrada original
    pub timestamp: DateTime<Utc>,
    
    /// Autor/sender (si aplicable)
    pub author: Option<String>,
    
    /// Contenido textual
    pub content: String,
    
    /// Tipo de contenido
    pub content_type: ContentType,
    
    /// Metadata específica de la fuente
    pub source_metadata: HashMap<String, String>,
}

/// Tipo de contenido
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentType {
    /// Mensaje de texto
    Text,
    
    /// Multimedia (imagen, video, audio)
    Multimedia { mime_type: String },
    
    /// Attachment (documento, archivo)
    Attachment { filename: String },
    
    /// Sistema (notificación, cambio de configuración)
    System,
}

/// Metadata del proceso de digestión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestionMetadata {
    /// Cuándo se completó la digestión
    pub digested_at: DateTime<Utc>,
    
    /// Duración del proceso (ms)
    pub duration_ms: u64,
    
    /// Número de entradas procesadas
    pub entries_count: usize,
    
    /// Número de entradas fallidas
    pub failed_count: usize,
    
    /// Errores encontrados (si hubo)
    pub errors: Vec<String>,
    
    /// Estadísticas adicionales
    pub stats: HashMap<String, String>,
}

impl DigestionMetadata {
    /// Crea metadata inicial
    pub fn new(entries_count: usize) -> Self {
        Self {
            digested_at: Utc::now(),
            duration_ms: 0,
            entries_count,
            failed_count: 0,
            errors: Vec::new(),
            stats: HashMap::new(),
        }
    }
}

// ================================================================
// WHATSAPP DIGESTER
// ================================================================

/// Digester específico para WhatsApp exports
///
/// **Formato esperado:**
/// ```text
/// [2023-01-01, 10:00:00] Eduardo: Hola mundo
/// [2023-01-01, 10:05:00] María: Hola!
/// ```
///
/// **Features:**
/// - Parsing de timestamps WhatsApp
/// - Detección de autor por línea
/// - Manejo de mensajes multilinea
/// - Detección de multimedia: <Media omitted>
/// - Detección de system messages
///
/// **Performance:**
/// - ~30 mensajes/ms (1000 mensajes en 30s)
pub struct WhatsAppDigester {
    /// Config opcional (para v1.1 con templates)
    config: WhatsAppConfig,
}

/// Configuración de WhatsAppDigester
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppConfig {
    /// Permitir mensajes sin timestamp (false = strict)
    pub allow_missing_timestamps: bool,
    
    /// Ignorar system messages
    pub ignore_system_messages: bool,
    
    /// Encoding esperado
    pub expected_encoding: String,
}

impl Default for WhatsAppConfig {
    fn default() -> Self {
        Self {
            allow_missing_timestamps: false,
            ignore_system_messages: false,
            expected_encoding: "UTF-8".to_string(),
        }
    }
}

impl WhatsAppDigester {
    /// Crea nuevo WhatsAppDigester con config default
    pub fn new() -> Self {
        Self {
            config: WhatsAppConfig::default(),
        }
    }
    
    /// Crea con config custom
    pub fn with_config(config: WhatsAppConfig) -> Self {
        Self { config }
    }
    
    /// Parsea una línea de WhatsApp export
    ///
    /// **Formatos:**
    /// - `[6/3/25, 21:47:50] Paula Roque: Message`
    /// - `‎[6/3/25, 23:16:37] Paula Roque: ‎<attached: 00000003-PHOTO-2025-06-03-23-16-37.jpg>`
    fn parse_line(&self, line: &str) -> Option<DigestedEntry> {
        // Remove invisible characters (WhatsApp adds \u{200E} sometimes)
        let clean_line = line.trim_start_matches('\u{200E}');
        
        // Pattern: [M/D/YY, HH:MM:SS] Author: Message
        if !clean_line.starts_with('[') {
            return None;
        }
        
        // Buscar cierre de timestamp
        let timestamp_end = clean_line.find(']')?;
        let timestamp_str = &clean_line[1..timestamp_end];
        
        // Parse timestamp
        let timestamp = self.parse_timestamp(timestamp_str)?;
        
        // Rest of line
        let rest = clean_line[timestamp_end + 1..].trim();
        
        // Check if it's a system message (no colon)
        if !rest.contains(':') {
            if self.config.ignore_system_messages {
                return None;
            }
            
            return Some(DigestedEntry {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp,
                author: None,
                content: rest.to_string(),
                content_type: ContentType::System,
                source_metadata: HashMap::new(),
            });
        }
        
        // Split author and content
        let colon_pos = rest.find(':')?;
        let author = rest[..colon_pos].trim().to_string();
        let mut content = rest[colon_pos + 1..].trim().to_string();
        
        // Remove invisible characters from content
        content = content.trim_start_matches('\u{200E}').to_string();
        
        // Detect content type
        let content_type = if content.contains("<attached:") {
            // Extract filename from <attached: filename.jpg>
            let filename = content
                .split("<attached:")
                .nth(1)
                .and_then(|s| s.split('>').next())
                .unwrap_or("unknown")
                .trim()
                .to_string();
            
            // Determine type by extension
            if filename.ends_with(".jpg") || filename.ends_with(".png") || filename.ends_with(".webp") {
                ContentType::Multimedia {
                    mime_type: "image/jpeg".to_string(),
                }
            } else if filename.ends_with(".opus") || filename.ends_with(".mp3") {
                ContentType::Multimedia {
                    mime_type: "audio/opus".to_string(),
                }
            } else if filename.ends_with(".mp4") || filename.ends_with(".avi") {
                ContentType::Multimedia {
                    mime_type: "video/mp4".to_string(),
                }
            } else if filename.ends_with(".pdf") || filename.ends_with(".docx") {
                ContentType::Attachment { filename }
            } else {
                ContentType::Multimedia {
                    mime_type: "application/octet-stream".to_string(),
                }
            }
        } else if content.contains("<Media omitted>") 
            || content.contains("<imagen omitida>")
            || content.contains("<audio omitido>")
            || content.contains("<video omitido>") {
            ContentType::Multimedia {
                mime_type: "application/octet-stream".to_string(),
            }
        } else if content.contains("documento omitido") {
            ContentType::Attachment {
                filename: "unknown".to_string(),
            }
        } else {
            ContentType::Text
        };
        
        Some(DigestedEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            author: Some(author),
            content,
            content_type,
            source_metadata: HashMap::new(),
        })
    }
    
    /// Parse timestamp WhatsApp format
    ///
    /// **Formatos soportados:**
    /// - `[6/3/25, 21:47:50]` (formato export estándar M/D/YY, HH:MM:SS)
    /// - `[2023-01-01, 10:00:00]` (formato alternativo YYYY-MM-DD)
    fn parse_timestamp(&self, timestamp_str: &str) -> Option<DateTime<Utc>> {
        let parts: Vec<&str> = timestamp_str.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        
        let date_str = parts[0].trim();
        let time_str = parts[1].trim();
        
        // Try format 1: M/D/YY (export real)
        if let Some(dt) = self.parse_mdy_format(date_str, time_str) {
            return Some(dt);
        }
        
        // Try format 2: YYYY-MM-DD (alternativo)
        if let Some(dt) = self.parse_iso_format(date_str, time_str) {
            return Some(dt);
        }
        
        None
    }
    
    /// Parse formato M/D/YY, HH:MM:SS (formato export real)
    fn parse_mdy_format(&self, date_str: &str, time_str: &str) -> Option<DateTime<Utc>> {
        let date_parts: Vec<&str> = date_str.split('/').collect();
        if date_parts.len() != 3 {
            return None;
        }
        
        let month: u32 = date_parts[0].parse().ok()?;
        let day: u32 = date_parts[1].parse().ok()?;
        let year: i32 = date_parts[2].parse::<i32>().ok()?;
        
        // Convert 2-digit year to 4-digit (25 -> 2025)
        let full_year = if year < 100 {
            2000 + year
        } else {
            year
        };
        
        // Parse time
        let time_parts: Vec<&str> = time_str.split(':').collect();
        if time_parts.len() != 3 {
            return None;
        }
        
        let hour: u32 = time_parts[0].parse().ok()?;
        let minute: u32 = time_parts[1].parse().ok()?;
        let second: u32 = time_parts[2].parse().ok()?;
        
        // Build DateTime
        use chrono::NaiveDate;
        let naive_date = NaiveDate::from_ymd_opt(full_year, month, day)?;
        let naive_time = chrono::NaiveTime::from_hms_opt(hour, minute, second)?;
        let naive_datetime = naive_date.and_time(naive_time);
        
        Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc))
    }
    
    /// Parse formato YYYY-MM-DD, HH:MM:SS (alternativo)
    fn parse_iso_format(&self, date_str: &str, time_str: &str) -> Option<DateTime<Utc>> {
        let date_parts: Vec<&str> = date_str.split('-').collect();
        if date_parts.len() != 3 {
            return None;
        }
        
        // Parse time (HH:MM:SS)
        let time_parts: Vec<&str> = time_str.split(':').collect();
        if time_parts.len() < 2 {
            return None;
        }
        
        // Build RFC3339 string
        let rfc3339 = if time_parts.len() == 2 {
            format!("{}T{}:00Z", date_str, time_str)
        } else {
            format!("{}T{}Z", date_str, time_str)
        };
        
        // Parse to DateTime
        chrono::DateTime::parse_from_rfc3339(&rfc3339)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

impl Default for WhatsAppDigester {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DigestionPipeline for WhatsAppDigester {
    fn source_type(&self) -> DataSource {
        DataSource::WhatsApp
    }
    
    fn validate_format(&self, raw_data: &[u8]) -> Result<()> {
        // 1. Check UTF-8
        let text = String::from_utf8(raw_data.to_vec())
            .map_err(|_| DataImportError::DigestionFailed {
                data_source: "WhatsApp".to_string(),
                reason: "Invalid UTF-8 encoding".to_string(),
            })?;
        
        // 2. Check basic WhatsApp format (at least one line with timestamp)
        let has_valid_line = text.lines().any(|line| {
            line.starts_with('[') && line.contains(']') && line.contains(':')
        });
        
        if !has_valid_line {
            return Err(DataImportError::DigestionFailed {
                data_source: "WhatsApp".to_string(),
                reason: "No valid WhatsApp format lines found".to_string(),
            });
        }
        
        Ok(())
    }
    
    async fn digest(&self, quarantined: &QuarantineZone) -> Result<DigestedData> {
        let start = std::time::Instant::now();
        
        // 1. Validate readiness
        if !quarantined.is_ready_for_digestion() {
            return Err(DataImportError::QuarantineNotReady(
                format!("QuarantineZone {} not approved", quarantined.id)
            ));
        }
        
        // 2. Get text content
        let text = quarantined.get_text_content()?;
        
        // 3. Validate format
        self.validate_format(quarantined.raw_data.as_slice())?;
        
        // 4. Parse all lines (with multiline support)
        let mut entries = Vec::new();
        let mut failed_count = 0;
        let mut errors = Vec::new();
        let mut current_entry: Option<DigestedEntry> = None;
        
        for (line_num, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            // Try to parse as new message
            match self.parse_line(line) {
                Some(entry) => {
                    // If we had a previous entry, save it
                    if let Some(prev) = current_entry.take() {
                        entries.push(prev);
                    }
                    // Start new entry
                    current_entry = Some(entry);
                }
                None => {
                    // Not a new message - could be continuation
                    if let Some(ref mut entry) = current_entry {
                        // Append to current message content
                        entry.content.push('\n');
                        entry.content.push_str(line.trim());
                    } else {
                        // No current message - failed line
                        failed_count += 1;
                        if failed_count <= 10 {
                            errors.push(format!("Line {}: Failed to parse (no active message)", line_num + 1));
                        }
                    }
                }
            }
        }
        
        // Save last entry
        if let Some(entry) = current_entry {
            entries.push(entry);
        }
        
        // 5. Build metadata
        let duration_ms = start.elapsed().as_millis() as u64;
        let mut metadata = DigestionMetadata::new(entries.len());
        metadata.duration_ms = duration_ms;
        metadata.failed_count = failed_count;
        metadata.errors = errors;
        metadata.stats.insert("total_lines".to_string(), text.lines().count().to_string());
        metadata.stats.insert("parsed_messages".to_string(), entries.len().to_string());
        
        // Calculate success rate (lines with timestamp vs total non-empty lines)
        let non_empty_lines = text.lines().filter(|l| !l.trim().is_empty()).count();
        metadata.stats.insert("success_rate".to_string(), 
            format!("{:.2}%", (entries.len() as f64 / non_empty_lines as f64) * 100.0));
        
        Ok(DigestedData {
            source: DataSource::WhatsApp,
            entries,
            digestion_metadata: metadata,
        })
    }
}

// ================================================================
// TELEGRAM DIGESTER
// ================================================================

/// Digester específico para Telegram JSON exports
///
/// **Formato esperado:**
/// ```json
/// {
///   "name": "Chat Name",
///   "type": "personal_chat",
///   "messages": [
///     {
///       "id": 1,
///       "type": "message",
///       "date": "2023-01-01T10:00:00",
///       "from": "Eduardo",
///       "text": "Hello world"
///     }
///   ]
/// }
/// ```
///
/// **Features:**
/// - JSON parsing with serde
/// - Detección de: text, sticker, media, service messages
/// - Preserva: channels, bots, forwards, replies
/// - Extrae: content consumption patterns
///
/// **Performance:**
/// - ~50 mensajes/ms (similar a WhatsApp)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramExport {
    name: Option<String>,
    #[serde(rename = "type")]
    chat_type: Option<String>,
    messages: Vec<TelegramMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TelegramMessage {
    id: i64,
    #[serde(rename = "type")]
    message_type: String,
    date: String,
    from: Option<String>,
    from_id: Option<String>,
    text: Option<serde_json::Value>,  // Can be string or array of objects
    
    #[serde(default)]
    reply_to_message_id: Option<i64>,
    #[serde(default)]
    forwarded_from: Option<String>,
    #[serde(default)]
    sticker_emoji: Option<String>,
    #[serde(default)]
    photo: Option<String>,
    #[serde(default)]
    file: Option<String>,
}

pub struct TelegramDigester {
    config: TelegramConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    /// Incluir mensajes de servicio (user joined, etc.)
    pub include_service_messages: bool,
    
    /// Preservar información de forwards
    pub preserve_forward_info: bool,
    
    /// Encoding esperado
    pub expected_encoding: String,
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self {
            include_service_messages: true,
            preserve_forward_info: true,
            expected_encoding: "UTF-8".to_string(),
        }
    }
}

impl TelegramDigester {
    /// Crea nuevo TelegramDigester con config default
    pub fn new() -> Self {
        Self {
            config: TelegramConfig::default(),
        }
    }
    
    /// Crea con config custom
    pub fn with_config(config: TelegramConfig) -> Self {
        Self { config }
    }
    
    /// Extrae texto del field text (puede ser string o array)
    fn extract_text(&self, text_value: &serde_json::Value) -> String {
        match text_value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Array(arr) => {
                // Array de objetos con "text" field
                arr.iter()
                    .filter_map(|item| {
                        if let serde_json::Value::Object(obj) = item {
                            obj.get("text").and_then(|v| v.as_str())
                        } else if let serde_json::Value::String(s) = item {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            }
            _ => String::new(),
        }
    }
    
    /// Parse timestamp Telegram format (ISO 8601)
    fn parse_timestamp(&self, date_str: &str) -> Option<DateTime<Utc>> {
        chrono::DateTime::parse_from_rfc3339(date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
    
    /// Convierte TelegramMessage a DigestedEntry
    fn convert_message(&self, msg: &TelegramMessage) -> Option<DigestedEntry> {
        // Parse timestamp
        let timestamp = self.parse_timestamp(&msg.date)?;
        
        // Extract author
        let author = msg.from.clone();
        
        // Determine content and type
        let (content, content_type) = if msg.message_type == "service" {
            if !self.config.include_service_messages {
                return None;
            }
            (format!("Service: {}", msg.message_type), ContentType::System)
        } else if let Some(sticker) = &msg.sticker_emoji {
            (format!("Sticker: {}", sticker), ContentType::Text)
        } else if msg.photo.is_some() {
            ("Photo".to_string(), ContentType::Multimedia {
                mime_type: "image/jpeg".to_string(),
            })
        } else if let Some(file) = &msg.file {
            (format!("File: {}", file), ContentType::Attachment {
                filename: file.clone(),
            })
        } else if let Some(text_value) = &msg.text {
            (self.extract_text(text_value), ContentType::Text)
        } else {
            ("Unknown message type".to_string(), ContentType::System)
        };
        
        // Build source metadata
        let mut source_metadata = HashMap::new();
        source_metadata.insert("telegram_id".to_string(), msg.id.to_string());
        source_metadata.insert("message_type".to_string(), msg.message_type.clone());
        
        if let Some(reply_to) = msg.reply_to_message_id {
            source_metadata.insert("reply_to".to_string(), reply_to.to_string());
        }
        
        if let Some(forwarded) = &msg.forwarded_from {
            if self.config.preserve_forward_info {
                source_metadata.insert("forwarded_from".to_string(), forwarded.clone());
            }
        }
        
        Some(DigestedEntry {
            id: format!("telegram_{}", msg.id),
            timestamp,
            author,
            content,
            content_type,
            source_metadata,
        })
    }
}

impl Default for TelegramDigester {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DigestionPipeline for TelegramDigester {
    fn source_type(&self) -> DataSource {
        DataSource::Telegram
    }
    
    fn validate_format(&self, raw_data: &[u8]) -> Result<()> {
        // 1. Check UTF-8
        let text = String::from_utf8(raw_data.to_vec())
            .map_err(|_| DataImportError::DigestionFailed {
                data_source: "Telegram".to_string(),
                reason: "Invalid UTF-8 encoding".to_string(),
            })?;
        
        // 2. Check JSON format
        let _: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| DataImportError::DigestionFailed {
                data_source: "Telegram".to_string(),
                reason: format!("Invalid JSON: {}", e),
            })?;
        
        Ok(())
    }
    
    async fn digest(&self, quarantined: &QuarantineZone) -> Result<DigestedData> {
        let start = std::time::Instant::now();
        
        // 1. Validate readiness
        if !quarantined.is_ready_for_digestion() {
            return Err(DataImportError::QuarantineNotReady(
                format!("QuarantineZone {} not approved", quarantined.id)
            ));
        }
        
        // 2. Get text content
        let text = quarantined.get_text_content()?;
        
        // 3. Validate format
        self.validate_format(quarantined.raw_data.as_slice())?;
        
        // 4. Parse JSON
        let export: TelegramExport = serde_json::from_str(&text)
            .map_err(|e| DataImportError::DigestionFailed {
                data_source: "Telegram".to_string(),
                reason: format!("Failed to parse Telegram JSON: {}", e),
            })?;
        
        // 5. Convert messages
        let mut entries = Vec::new();
        let mut failed_count = 0;
        let mut errors = Vec::new();
        
        for (idx, msg) in export.messages.iter().enumerate() {
            match self.convert_message(msg) {
                Some(entry) => entries.push(entry),
                None => {
                    failed_count += 1;
                    if failed_count <= 10 {
                        errors.push(format!("Message {}: Failed to convert", idx + 1));
                    }
                }
            }
        }
        
        // 6. Build metadata
        let duration_ms = start.elapsed().as_millis() as u64;
        let mut metadata = DigestionMetadata::new(entries.len());
        metadata.duration_ms = duration_ms;
        metadata.failed_count = failed_count;
        metadata.errors = errors;
        metadata.stats.insert("total_messages".to_string(), export.messages.len().to_string());
        metadata.stats.insert("chat_name".to_string(), export.name.unwrap_or_default());
        metadata.stats.insert("chat_type".to_string(), export.chat_type.unwrap_or_default());
        metadata.stats.insert("success_rate".to_string(), 
            format!("{:.2}%", (entries.len() as f64 / export.messages.len() as f64) * 100.0));
        
        Ok(DigestedData {
            source: DataSource::Telegram,
            entries,
            digestion_metadata: metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_whatsapp_digester_source_type() {
        let digester = WhatsAppDigester::new();
        assert_eq!(digester.source_type(), DataSource::WhatsApp);
    }
    
    #[test]
    fn test_whatsapp_validate_format_valid() {
        let digester = WhatsAppDigester::new();
        let data = b"[2023-01-01, 10:00:00] Eduardo: Hola mundo";
        assert!(digester.validate_format(data).is_ok());
    }
    
    #[test]
    fn test_whatsapp_validate_format_invalid() {
        let digester = WhatsAppDigester::new();
        let data = b"This is not a WhatsApp export";
        assert!(digester.validate_format(data).is_err());
    }
    
    #[test]
    fn test_whatsapp_parse_timestamp() {
        let digester = WhatsAppDigester::new();
        let timestamp = digester.parse_timestamp("2023-01-01, 10:00:00");
        assert!(timestamp.is_some());
    }
    
    #[test]
    fn test_whatsapp_parse_line_text() {
        let digester = WhatsAppDigester::new();
        let line = "[2023-01-01, 10:00:00] Eduardo: Hola mundo";
        let entry = digester.parse_line(line).expect("Should parse");
        
        assert_eq!(entry.author, Some("Eduardo".to_string()));
        assert_eq!(entry.content, "Hola mundo");
        assert_eq!(entry.content_type, ContentType::Text);
    }
    
    #[test]
    fn test_whatsapp_parse_line_multimedia() {
        let digester = WhatsAppDigester::new();
        let line = "[2023-01-01, 10:00:00] Eduardo: <Media omitted>";
        let entry = digester.parse_line(line).expect("Should parse");
        
        assert!(matches!(entry.content_type, ContentType::Multimedia { .. }));
    }
    
    #[test]
    fn test_whatsapp_parse_line_system() {
        let digester = WhatsAppDigester::new();
        let line = "[2023-01-01, 10:00:00] Eduardo joined the group";
        let entry = digester.parse_line(line).expect("Should parse");
        
        assert_eq!(entry.author, None);
        assert_eq!(entry.content_type, ContentType::System);
    }
}
