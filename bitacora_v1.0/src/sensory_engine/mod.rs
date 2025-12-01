//! # SENSORY ENGINE - Procesamiento Multimodal de Input
//!
//! Componente crítico que normaliza texto, audio y visual input a formato unificado
//! para alimentar el Context Token 7D.
//!
//! ## Arquitectura
//!
//! ```text
//! User Input → SENSORY ENGINE → NormalizedInput → CTX7D → TelescopeDB
//!    (multimodal)     ↓                                      ↓
//!                Text Processor                         VoxelDB
//!                Audio Transcriber (STUB Whisper)
//!                Visual Analyzer (STUB v2.0)
//! ```
//!
//! ## Responsabilidades
//!
//! 1. Procesar texto plano con detección de idioma
//! 2. Transcribir audio (STUB para v1.0, Whisper API en v2.0)
//! 3. Analizar tono y emoción
//! 4. Extraer metadata contextual (keywords, referencias, comandos)
//! 5. Normalizar a formato unificado
//! 6. Track costos API (DA-009)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::Instant;
use thiserror::Error;

/// Errores del SENSORY ENGINE
#[derive(Error, Debug)]
pub enum SensoryError {
    #[error("Input vacío o inválido")]
    InvalidInput,
    
    #[error("Error procesando texto: {0}")]
    TextProcessingError(String),
    
    #[error("Error transcribiendo audio: {0}")]
    AudioTranscriptionError(String),
    
    #[error("Error analizando visual: {0}")]
    VisualAnalysisError(String),
    
    #[error("Formato de audio no soportado: {0}")]
    UnsupportedAudioFormat(String),
    
    #[error("Idioma no detectado")]
    LanguageDetectionFailed,
    
    #[error("Error de configuración: {0}")]
    ConfigError(String),
    
    #[error("Error IO: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SensoryError>;

/// SENSORY ENGINE principal
pub struct SensoryEngine {
    /// Procesador de texto
    text_processor: TextProcessor,
    
    /// Transcriptor de audio (STUB)
    audio_transcriber: AudioTranscriber,
    
    /// Configuración
    config: SensoryConfig,
    
    /// Métricas de uso
    metrics: SensoryMetrics,
}

impl SensoryEngine {
    /// Crear nuevo SENSORY ENGINE
    pub fn new(config: SensoryConfig) -> Result<Self> {
        Ok(Self {
            text_processor: TextProcessor::new(config.clone()),
            audio_transcriber: AudioTranscriber::new(config.clone()),
            config,
            metrics: SensoryMetrics::default(),
        })
    }
    
    /// Procesar input de texto
    pub fn process_text(&mut self, text: String) -> Result<NormalizedInput> {
        if text.trim().is_empty() {
            return Err(SensoryError::InvalidInput);
        }
        
        self.metrics.total_inputs += 1;
        self.metrics.text_inputs += 1;
        
        self.text_processor.process(text)
    }
    
    /// Procesar input de audio (STUB para v1.0)
    pub fn process_audio(&mut self, audio_data: Vec<u8>, format: AudioFormat) -> Result<NormalizedInput> {
        if audio_data.is_empty() {
            return Err(SensoryError::InvalidInput);
        }
        
        self.metrics.total_inputs += 1;
        self.metrics.audio_inputs += 1;
        
        self.audio_transcriber.transcribe(audio_data, format)
    }
    
    /// Procesar input visual (STUB para v2.0)
    pub fn process_visual(&mut self, _image_data: Vec<u8>) -> Result<NormalizedInput> {
        // STUB: Visual processing será implementado en v2.0
        Err(SensoryError::VisualAnalysisError(
            "Visual processing no implementado en v1.0 (STUB para v2.0)".to_string()
        ))
    }
    
    /// Obtener métricas de uso
    pub fn metrics(&self) -> &SensoryMetrics {
        &self.metrics
    }
    
    /// Resetear métricas
    pub fn reset_metrics(&mut self) {
        self.metrics = SensoryMetrics::default();
    }
}

/// Configuración de SENSORY ENGINE
#[derive(Debug, Clone)]
pub struct SensoryConfig {
    /// Habilitar Whisper API (vs STUB)
    pub use_whisper_api: bool,
    
    /// Threshold de duración para usar API (segundos)
    pub api_threshold_seconds: u32,
    
    /// Idioma por defecto si no se detecta
    pub default_language: String,
    
    /// Habilitar análisis de sentimiento
    pub enable_sentiment_analysis: bool,
}

impl Default for SensoryConfig {
    fn default() -> Self {
        Self {
            use_whisper_api: false,  // STUB en v1.0
            api_threshold_seconds: 10,
            default_language: "es".to_string(),
            enable_sentiment_analysis: true,
        }
    }
}

/// Input normalizado (output de SENSORY ENGINE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedInput {
    /// ID único (content-addressable SHA-256)
    pub id: String,
    
    /// Contenido normalizado (texto)
    pub content: String,
    
    /// Modalidad original
    pub modality: InputModality,
    
    /// Idioma detectado (ISO 639-1: "es", "en", etc.)
    pub language: String,
    
    /// Metadata específico de modalidad
    pub metadata: InputMetadata,
    
    /// Timestamp de procesamiento
    pub processed_at: DateTime<Utc>,
    
    /// Análisis de tono y emoción
    pub tone_analysis: ToneAnalysis,
    
    /// Keywords extraídas
    pub extracted_keywords: Vec<String>,
    
    /// Referencias detectadas (URLs, paths, comandos)
    pub references: Vec<Reference>,
}

impl NormalizedInput {
    /// Generar ID content-addressable
    fn generate_id(content: &str, timestamp: &DateTime<Utc>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        format!("sensory_{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum InputModality {
    Text,
    Audio,
    Visual,
    Mixed,
}

/// Metadata específico de cada modalidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMetadata {
    /// Para audio: duración en segundos
    pub duration_seconds: Option<f64>,
    
    /// Para audio: velocidad de habla (palabras/minuto)
    pub speech_rate_wpm: Option<u32>,
    
    /// Encoding original
    pub encoding: String,
    
    /// Tamaño del input original (bytes)
    pub original_size_bytes: usize,
    
    /// Tiempo de procesamiento (ms)
    pub processing_time_ms: u64,
}

impl Default for InputMetadata {
    fn default() -> Self {
        Self {
            duration_seconds: None,
            speech_rate_wpm: None,
            encoding: "UTF-8".to_string(),
            original_size_bytes: 0,
            processing_time_ms: 0,
        }
    }
}

/// Análisis de tono y emoción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneAnalysis {
    /// Tono general
    pub overall_tone: Tone,
    
    /// Sentimiento (-1.0 = muy negativo, 0.0 = neutral, 1.0 = muy positivo)
    pub sentiment_score: f64,
    
    /// Nivel de urgencia (0.0 = no urgente, 1.0 = crítico)
    pub urgency_level: f64,
    
    /// Nivel de énfasis (0.0 = normal, 1.0 = muy enfatizado)
    pub emphasis_level: f64,
    
    /// Confianza del análisis (0.0-1.0)
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Tone {
    Urgent,
    Neutral,
    Casual,
    Formal,
    Frustrated,
    Excited,
    Confused,
}

impl Default for ToneAnalysis {
    fn default() -> Self {
        Self {
            overall_tone: Tone::Neutral,
            sentiment_score: 0.0,
            urgency_level: 0.0,
            emphasis_level: 0.0,
            confidence: 0.5,
        }
    }
}

/// Referencia detectada en el input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    /// Tipo de referencia
    pub ref_type: ReferenceType,
    
    /// Valor de la referencia
    pub value: String,
    
    /// Contexto donde aparece
    pub context: Option<String>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReferenceType {
    Url,
    FilePath,
    Command,
    CodeBlock,
    Mention,
}

/// Procesador de texto
struct TextProcessor {
    config: SensoryConfig,
    common_words: HashMap<String, usize>,
}

impl TextProcessor {
    fn new(config: SensoryConfig) -> Self {
        // Palabras comunes para filtrar keywords
        let common_words: HashMap<String, usize> = [
            ("el", 1), ("la", 1), ("de", 1), ("que", 1), ("y", 1),
            ("a", 1), ("en", 1), ("un", 1), ("ser", 1), ("se", 1),
            ("the", 1), ("a", 1), ("an", 1), ("and", 1), ("or", 1),
        ].iter().map(|(k, v)| (k.to_string(), *v)).collect();
        
        Self {
            config,
            common_words,
        }
    }
    
    fn process(&self, text: String) -> Result<NormalizedInput> {
        let start = Instant::now();
        
        // Normalizar texto
        let normalized_text = self.normalize_text(&text);
        
        // Detectar idioma (simple heuristic)
        let language = self.detect_language(&normalized_text);
        
        // Analizar tono
        let tone_analysis = self.analyze_tone(&normalized_text);
        
        // Extraer keywords
        let keywords = self.extract_keywords(&normalized_text);
        
        // Detectar referencias
        let references = self.detect_references(&normalized_text);
        
        let processing_time = start.elapsed().as_millis() as u64;
        let processed_at = Utc::now();
        
        Ok(NormalizedInput {
            id: NormalizedInput::generate_id(&normalized_text, &processed_at),
            content: normalized_text.clone(),
            modality: InputModality::Text,
            language,
            metadata: InputMetadata {
                original_size_bytes: text.len(),
                processing_time_ms: processing_time,
                ..Default::default()
            },
            processed_at,
            tone_analysis,
            extracted_keywords: keywords,
            references,
        })
    }
    
    fn normalize_text(&self, text: &str) -> String {
        // Normalizar espacios
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    fn detect_language(&self, text: &str) -> String {
        // Simple heuristic: detectar palabras en español vs inglés
        let spanish_words = ["el", "la", "de", "que", "es", "por", "para", "con", "cómo", "qué"];
        let english_words = ["the", "is", "are", "this", "that", "how", "what", "can", "will"];
        
        let text_lower = text.to_lowercase();
        let spanish_count = spanish_words.iter()
            .filter(|&&word| text_lower.contains(word))
            .count();
        let english_count = english_words.iter()
            .filter(|&&word| text_lower.contains(word))
            .count();
        
        if spanish_count > english_count {
            "es".to_string()
        } else if english_count > 0 {
            "en".to_string()
        } else {
            self.config.default_language.clone()
        }
    }
    
    fn analyze_tone(&self, text: &str) -> ToneAnalysis {
        let mut tone_analysis = ToneAnalysis::default();
        
        // Detectar urgencia
        let urgency_keywords = ["urgent", "urgente", "ahora", "ya", "crítico", "emergencia", "help", "ayuda", "ASAP"];
        let text_lower = text.to_lowercase();
        let urgency_count = urgency_keywords.iter()
            .filter(|&&kw| text_lower.contains(kw))
            .count();
        
        tone_analysis.urgency_level = (urgency_count as f64 / 3.0).min(1.0);
        
        // Detectar énfasis (mayúsculas, signos de exclamación)
        let uppercase_count = text.chars().filter(|c| c.is_uppercase() && c.is_alphabetic()).count();
        let alpha_count = text.chars().filter(|c| c.is_alphabetic()).count();
        let exclamation_count = text.chars().filter(|&c| c == '!').count();
        
        let uppercase_ratio = if alpha_count > 0 {
            uppercase_count as f64 / alpha_count as f64
        } else {
            0.0
        };
        
        tone_analysis.emphasis_level = (uppercase_ratio * 2.0 + exclamation_count as f64 * 0.3).min(1.0);
        
        // Determinar tono general
        tone_analysis.overall_tone = if tone_analysis.urgency_level > 0.7 {
            Tone::Urgent
        } else if uppercase_ratio > 0.3 || exclamation_count > 2 {
            Tone::Frustrated
        } else if text.contains('?') && (text_lower.contains("cómo") || text_lower.contains("how")) {
            Tone::Confused
        } else if text_lower.contains("por favor") || text_lower.contains("please") {
            Tone::Formal
        } else if exclamation_count > 0 && tone_analysis.urgency_level < 0.3 {
            Tone::Excited
        } else {
            Tone::Neutral
        };
        
        // Análisis de sentimiento simple
        let positive_words = ["gracias", "excelente", "perfecto", "bueno", "thanks", "great", "perfect", "good"];
        let negative_words = ["error", "problema", "mal", "falla", "no funciona", "broken", "issue", "fail"];
        
        let positive_count = positive_words.iter()
            .filter(|&&word| text_lower.contains(word))
            .count() as f64;
        let negative_count = negative_words.iter()
            .filter(|&&word| text_lower.contains(word))
            .count() as f64;
        
        tone_analysis.sentiment_score = ((positive_count - negative_count) / 5.0).clamp(-1.0, 1.0);
        tone_analysis.confidence = 0.75;
        
        tone_analysis
    }
    
    fn extract_keywords(&self, text: &str) -> Vec<String> {
        let words: Vec<String> = text
            .split_whitespace()
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| w.len() > 3 && !self.common_words.contains_key(w))
            .collect();
        
        // Contar frecuencia y retornar top keywords
        let mut freq_map: HashMap<String, usize> = HashMap::new();
        for word in words {
            *freq_map.entry(word).or_insert(0) += 1;
        }
        
        let mut keywords: Vec<_> = freq_map.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        
        keywords.into_iter()
            .take(10)
            .map(|(word, _)| word)
            .collect()
    }
    
    fn detect_references(&self, text: &str) -> Vec<Reference> {
        let mut references = Vec::new();
        
        // Detectar URLs (simple pattern)
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                references.push(Reference {
                    ref_type: ReferenceType::Url,
                    value: word.to_string(),
                    context: None,
                });
            }
        }
        
        // Detectar file paths
        for word in text.split_whitespace() {
            if word.starts_with('/') || word.starts_with("./") || word.contains(":\\") {
                references.push(Reference {
                    ref_type: ReferenceType::FilePath,
                    value: word.to_string(),
                    context: None,
                });
            }
        }
        
        // Detectar comandos (líneas que empiezan con $ o >, o contienen "$ comando")
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('$') || trimmed.starts_with('>') {
                references.push(Reference {
                    ref_type: ReferenceType::Command,
                    value: trimmed.trim_start_matches(['$', '>', ' ']).to_string(),
                    context: None,
                });
            } else if trimmed.contains("$ ") || trimmed.contains("> ") {
                // Detectar comandos en medio de línea (ej: "texto $ cargo build")
                if let Some(pos) = trimmed.find("$ ") {
                    let cmd = &trimmed[pos+2..];
                    if !cmd.is_empty() {
                        references.push(Reference {
                            ref_type: ReferenceType::Command,
                            value: cmd.to_string(),
                            context: None,
                        });
                    }
                }
            }
        }
        
        // Detectar bloques de código (entre backticks)
        if text.contains("```") {
            references.push(Reference {
                ref_type: ReferenceType::CodeBlock,
                value: "Code block detected".to_string(),
                context: None,
            });
        }
        
        references
    }
}

/// Transcriptor de audio (STUB para v1.0)
struct AudioTranscriber {
    config: SensoryConfig,
}

impl AudioTranscriber {
    fn new(config: SensoryConfig) -> Self {
        Self { config }
    }
    
    fn transcribe(&self, audio_data: Vec<u8>, format: AudioFormat) -> Result<NormalizedInput> {
        let start = Instant::now();
        
        // STUB: En v1.0, simulamos transcripción
        let simulated_text = format!(
            "[AUDIO TRANSCRIPTION STUB] Audio de {} bytes en formato {:?}. \
            En v2.0 se integrará Whisper API para transcripción real.",
            audio_data.len(),
            format
        );
        
        let processing_time = start.elapsed().as_millis() as u64;
        let processed_at = Utc::now();
        
        // Calcular duración estimada (simulada: 1 byte = 0.001 segundos)
        let estimated_duration = (audio_data.len() as f64) * 0.001;
        
        Ok(NormalizedInput {
            id: NormalizedInput::generate_id(&simulated_text, &processed_at),
            content: simulated_text,
            modality: InputModality::Audio,
            language: self.config.default_language.clone(),
            metadata: InputMetadata {
                duration_seconds: Some(estimated_duration),
                speech_rate_wpm: Some(150), // Promedio 150 palabras/minuto
                original_size_bytes: audio_data.len(),
                processing_time_ms: processing_time,
                ..Default::default()
            },
            processed_at,
            tone_analysis: ToneAnalysis::default(),
            extracted_keywords: vec!["audio".to_string(), "stub".to_string()],
            references: vec![],
        })
    }
}

/// Formato de audio soportado
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AudioFormat {
    Mp3,
    Wav,
    Ogg,
    M4a,
}

/// Métricas de uso del SENSORY ENGINE
#[derive(Debug, Clone, Default)]
pub struct SensoryMetrics {
    pub total_inputs: u64,
    pub text_inputs: u64,
    pub audio_inputs: u64,
    pub visual_inputs: u64,
    pub total_processing_time_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_text_simple() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let result = engine.process_text("Hola mundo, esto es una prueba".to_string()).unwrap();
        
        assert_eq!(result.modality, InputModality::Text);
        assert_eq!(result.language, "es");
        assert!(result.content.contains("Hola"));
    }
    
    #[test]
    fn test_urgency_detection() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let result = engine.process_text("URGENTE! El servidor está caído AHORA".to_string()).unwrap();
        
        assert_eq!(result.tone_analysis.overall_tone, Tone::Urgent);
        assert!(result.tone_analysis.urgency_level > 0.5);
        assert!(result.tone_analysis.emphasis_level > 0.3);
    }
    
    #[test]
    fn test_language_detection() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let spanish = engine.process_text("Cómo puedo hacer esto con la base de datos".to_string()).unwrap();
        assert_eq!(spanish.language, "es");
        
        let english = engine.process_text("How can I do this with the database".to_string()).unwrap();
        assert_eq!(english.language, "en");
    }
    
    #[test]
    fn test_reference_detection() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let text = "Revisa https://example.com y el archivo /home/user/test.txt\n$ cargo build".to_string();
        let result = engine.process_text(text).unwrap();
        
        assert_eq!(result.references.len(), 3);
        assert!(result.references.iter().any(|r| r.ref_type == ReferenceType::Url));
        assert!(result.references.iter().any(|r| r.ref_type == ReferenceType::FilePath));
        assert!(result.references.iter().any(|r| r.ref_type == ReferenceType::Command));
    }
    
    #[test]
    fn test_audio_stub() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let fake_audio = vec![0u8; 1000];
        let result = engine.process_audio(fake_audio, AudioFormat::Mp3).unwrap();
        
        assert_eq!(result.modality, InputModality::Audio);
        assert!(result.content.contains("STUB"));
        assert!(result.metadata.duration_seconds.is_some());
    }
    
    #[test]
    fn test_metrics_tracking() {
        let mut engine = SensoryEngine::new(SensoryConfig::default()).unwrap();
        
        let _ = engine.process_text("Test 1".to_string());
        let _ = engine.process_text("Test 2".to_string());
        let _ = engine.process_audio(vec![0u8; 100], AudioFormat::Wav);
        
        let metrics = engine.metrics();
        assert_eq!(metrics.total_inputs, 3);
        assert_eq!(metrics.text_inputs, 2);
        assert_eq!(metrics.audio_inputs, 1);
    }
}
