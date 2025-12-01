//! # 🧬 Extraction Layer - 7D Parallel Nutrient Extraction
//!
//! **Propósito:** Extraer 7 dimensiones de nutrientes en paralelo.
//!
//! ## 7 Dimensiones
//!
//! 1. **Identity** (biographical) → TelescopeDB
//! 2. **Interests** (topics) → TopicGraph
//! 3. **Emotions** (tone) → EmotionalSpace
//! 4. **Relationships** (social) → SocialGraph
//! 5. **Temporal** (patterns) → TemporalAnalyzer
//! 6. **Behavioral** (habits) → BehavioralProfiler
//! 7. **Hyperlinks** (consumption) → HyperlinkIntelligence
//!
//! ## Performance Target
//!
//! - <10s parallel extraction (tokio::join!)
//!
//! ## Philosophy
//!
//! Bitácora is NOT a digital hoarder. It's a curator.
//!
//! - 🧘 **Conscious** - Knows what it ingests
//! - 🔬 **Selective** - Distinguishes signal from noise
//! - 🗑️ **Curating** - Proactively eliminates garbage
//! - 💎 **Refining** - Extracts ESSENCE, not everything

use async_trait::async_trait;
use chrono::{DateTime, Utc, Datelike, Timelike};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::digestion::{DigestedData, DigestedEntry};
use super::error::Result;

// ================================================================
// CORE TRAIT: NutrientExtractor
// ================================================================

/// Trait base para extracción de nutrientes de una dimensión específica
///
/// **Contract:**
/// 1. `dimension()` - Identifica qué dimensión extrae (1-7)
/// 2. `extract()` - Procesa DigestedData y retorna ExtractedNutrients
/// 3. `can_process()` - Verifica si puede procesar un DigestedEntry
///
/// **Performance:**
/// - `extract()`: <10s for 1000 entries (async, parallel cuando posible)
#[async_trait]
pub trait NutrientExtractor: Send + Sync {
    /// Dimensión que este extractor procesa
    fn dimension(&self) -> NutrientDimension;
    
    /// Extraer nutrientes de datos digeridos
    ///
    /// **Input:** DigestedData (output de DigestionPipeline)
    /// **Output:** ExtractedNutrients (dimensional data)
    /// **Errors:** ExtractionFailed si proceso falla
    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients>;
    
    /// Verifica si puede procesar un entry específico
    ///
    /// **Input:** DigestedEntry
    /// **Output:** true si puede extraer nutrientes de este entry
    fn can_process(&self, entry: &DigestedEntry) -> bool;
}

// ================================================================
// NUTRIENT STRUCTURES
// ================================================================

/// Dimensión de nutrientes (7D)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NutrientDimension {
    /// 1. Identity (biographical data)
    Biographical,
    
    /// 2. Interests (topics, keywords)
    Interests,
    
    /// 3. Emotions (sentiment, tone)
    Emotional,
    
    /// 4. Relationships (social graph)
    Social,
    
    /// 5. Temporal (time patterns)
    Temporal,
    
    /// 6. Behavioral (habits, routines)
    Behavioral,
    
    /// 7. Hyperlinks (consumption patterns)
    Hyperlinks,
}

impl NutrientDimension {
    /// Retorna todas las dimensiones en orden
    pub fn all() -> Vec<NutrientDimension> {
        vec![
            NutrientDimension::Biographical,
            NutrientDimension::Interests,
            NutrientDimension::Emotional,
            NutrientDimension::Social,
            NutrientDimension::Temporal,
            NutrientDimension::Behavioral,
            NutrientDimension::Hyperlinks,
        ]
    }
    
    /// Nombre human-readable
    pub fn name(&self) -> &str {
        match self {
            NutrientDimension::Biographical => "Biographical",
            NutrientDimension::Interests => "Interests",
            NutrientDimension::Emotional => "Emotional",
            NutrientDimension::Social => "Social",
            NutrientDimension::Temporal => "Temporal",
            NutrientDimension::Behavioral => "Behavioral",
            NutrientDimension::Hyperlinks => "Hyperlinks",
        }
    }
}

/// Nutrientes extraídos de una dimensión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedNutrients {
    /// Dimensión de estos nutrientes
    pub dimension: NutrientDimension,
    
    /// Nutrientes individuales extraídos
    pub nutrients: Vec<Nutrient>,
    
    /// Metadata del proceso de extracción
    pub extraction_metadata: ExtractionMetadata,
}

/// Nutriente individual extraído
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrient {
    /// ID único del nutriente
    pub id: String,
    
    /// ID del DigestedEntry del que se extrajo
    pub source_entry_id: String,
    
    /// Tipo de nutriente
    pub nutrient_type: String,
    
    /// Valor principal del nutriente
    pub value: String,
    
    /// Confianza de la extracción (0.0 a 1.0)
    pub confidence: f64,
    
    /// Metadata adicional específica del nutriente
    pub metadata: HashMap<String, String>,
    
    /// Timestamp de extracción
    pub extracted_at: DateTime<Utc>,
}

impl Nutrient {
    /// Crea un nuevo nutriente
    pub fn new(
        source_entry_id: String,
        nutrient_type: String,
        value: String,
        confidence: f64,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id,
            nutrient_type,
            value,
            confidence,
            metadata: HashMap::new(),
            extracted_at: Utc::now(),
        }
    }
    
    /// Añade metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Metadata del proceso de extracción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionMetadata {
    /// Cuándo se completó la extracción
    pub extracted_at: DateTime<Utc>,
    
    /// Duración del proceso (ms)
    pub duration_ms: u64,
    
    /// Número de entries procesados
    pub entries_processed: usize,
    
    /// Número de nutrientes extraídos
    pub nutrients_extracted: usize,
    
    /// Número de entries que no pudieron procesarse
    pub entries_skipped: usize,
    
    /// Confianza promedio
    pub avg_confidence: f64,
    
    /// Estadísticas adicionales
    pub stats: HashMap<String, String>,
}

impl ExtractionMetadata {
    /// Crea metadata inicial
    pub fn new(entries_processed: usize) -> Self {
        Self {
            extracted_at: Utc::now(),
            duration_ms: 0,
            entries_processed,
            nutrients_extracted: 0,
            entries_skipped: 0,
            avg_confidence: 0.0,
            stats: HashMap::new(),
        }
    }
    
    /// Finaliza metadata calculando estadísticas
    pub fn finalize(&mut self, nutrients: &[Nutrient]) {
        self.nutrients_extracted = nutrients.len();
        
        if !nutrients.is_empty() {
            let total_confidence: f64 = nutrients.iter().map(|n| n.confidence).sum();
            self.avg_confidence = total_confidence / nutrients.len() as f64;
        }
    }
}

// ================================================================
// INTEREST EXTRACTOR (Task 7.x.3.3)
// ================================================================

/// Extractor de intereses/topics de texto
///
/// **Estrategia (v1.0 - Simple & Effective):**
/// 1. Keyword extraction (frecuencia + relevancia)
/// 2. Pattern matching (URLs, hashtags, menciones)
/// 3. Basic topic clustering (co-occurrence)
///
/// **No usa NLP pesado** - Eso es Phase 2
pub struct InterestExtractor {
    /// Palabras comunes a ignorar (stopwords básico)
    stopwords: Vec<String>,
    
    /// Mínimo de caracteres para considerar palabra
    min_word_length: usize,
    
    /// Mínima frecuencia para considerar keyword
    min_frequency: usize,
}

impl InterestExtractor {
    /// Crea nuevo InterestExtractor con config default
    pub fn new() -> Self {
        Self {
            stopwords: Self::default_stopwords(),
            min_word_length: 4,
            min_frequency: 2,
        }
    }
    
    /// Stopwords básicos español + inglés
    fn default_stopwords() -> Vec<String> {
        vec![
            // Español
            "que", "para", "con", "por", "como", "pero", "este", "esta", "esta",
            "esto", "son", "los", "las", "una", "uno", "del", "ese", "ese",
            "todo", "toda", "todos", "todas", "más", "muy", "bien", "ahí",
            "donde", "cuando", "porque", "cual", "desde", "hasta", "sobre",
            // Inglés
            "the", "and", "for", "with", "that", "this", "from", "have",
            "they", "what", "when", "where", "which", "about", "there",
            "their", "would", "could", "should", "will", "can", "then",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
    
    /// Extrae keywords de un texto
    fn extract_keywords(&self, text: &str) -> HashMap<String, usize> {
        let mut word_freq: HashMap<String, usize> = HashMap::new();
        
        // Normalizar y tokenizar
        let normalized = text.to_lowercase();
        let words: Vec<&str> = normalized
            .split(|c: char| !c.is_alphanumeric() && c != 'á' && c != 'é' && c != 'í' && c != 'ó' && c != 'ú' && c != 'ñ')
            .filter(|w| w.len() >= self.min_word_length)
            .filter(|w| !self.stopwords.contains(&w.to_string()))
            .collect();
        
        // Contar frecuencias
        for word in words {
            *word_freq.entry(word.to_string()).or_insert(0) += 1;
        }
        
        word_freq
    }
    
    /// Extrae URLs de un texto
    fn extract_urls(&self, text: &str) -> Vec<String> {
        let mut urls = Vec::new();
        
        // Regex simple para URLs
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                urls.push(word.to_string());
            } else if word.contains("www.") {
                urls.push(format!("https://{}", word));
            }
        }
        
        urls
    }
}

#[async_trait]
impl NutrientExtractor for InterestExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Interests
    }
    
    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        let mut nutrients = Vec::new();
        
        // Filtrar solo entries de texto
        let text_entries: Vec<_> = digested
            .entries
            .iter()
            .filter(|e| self.can_process(e))
            .collect();
        
        metadata.entries_skipped = digested.entries.len() - text_entries.len();
        
        // Acumular keywords de todos los textos
        let mut global_keywords: HashMap<String, usize> = HashMap::new();
        let mut global_urls: Vec<String> = Vec::new();
        
        for entry in &text_entries {
            // Extraer keywords
            let keywords = self.extract_keywords(&entry.content);
            for (word, freq) in keywords {
                *global_keywords.entry(word).or_insert(0) += freq;
            }
            
            // Extraer URLs
            let urls = self.extract_urls(&entry.content);
            global_urls.extend(urls);
        }
        
        // Crear nutrientes de keywords (top N)
        let mut sorted_keywords: Vec<_> = global_keywords.iter().collect();
        sorted_keywords.sort_by(|a, b| b.1.cmp(a.1));
        
        for (word, freq) in sorted_keywords.iter().take(50) {
            if **freq >= self.min_frequency {
                let confidence = (**freq as f64 / text_entries.len() as f64).min(1.0);
                
                let nutrient = Nutrient::new(
                    "aggregated".to_string(),
                    "keyword".to_string(),
                    word.to_string(),
                    confidence,
                )
                .with_metadata("frequency".to_string(), freq.to_string())
                .with_metadata("total_entries".to_string(), text_entries.len().to_string());
                
                nutrients.push(nutrient);
            }
        }
        
        // Contar URLs únicas antes de iterar
        let unique_urls_count = global_urls.len();
        
        // Crear nutrientes de URLs
        for url in global_urls {
            let nutrient = Nutrient::new(
                "aggregated".to_string(),
                "url".to_string(),
                url.clone(),
                0.9,
            )
            .with_metadata("detected_in".to_string(), "text".to_string());
            
            nutrients.push(nutrient);
        }
        
        // Finalizar metadata
        metadata.duration_ms = start.elapsed().as_millis() as u64;
        metadata.finalize(&nutrients);
        metadata.stats.insert(
            "unique_keywords".to_string(),
            global_keywords.len().to_string(),
        );
        metadata.stats.insert(
            "unique_urls".to_string(),
            unique_urls_count.to_string(),
        );
        
        Ok(ExtractedNutrients {
            dimension: self.dimension(),
            nutrients,
            extraction_metadata: metadata,
        })
    }
    
    fn can_process(&self, entry: &DigestedEntry) -> bool {
        use super::digestion::ContentType;
        matches!(entry.content_type, ContentType::Text) && !entry.content.is_empty()
    }
}

impl Default for InterestExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// EMOTIONAL EXTRACTOR (Task 7.x.3.4)
// ================================================================

/// Extractor de emociones/sentiment de texto
///
/// **Estrategia (v1.0 - Lexicon-based):**
/// 1. Sentiment lexicon (palabras positivas/negativas)
/// 2. Emoji detection (emocional signals)
/// 3. Punctuation patterns (!!!, ???, ...)
/// 4. Capitalization (GRITANDO vs normal)
///
/// **No usa ML** - Eso es Phase 2
pub struct EmotionalExtractor {
    /// Palabras positivas (español + inglés)
    positive_words: Vec<String>,
    
    /// Palabras negativas (español + inglés)
    negative_words: Vec<String>,
    
    /// Emojis positivos
    positive_emojis: Vec<String>,
    
    /// Emojis negativos
    negative_emojis: Vec<String>,
}

impl EmotionalExtractor {
    /// Crea nuevo EmotionalExtractor
    pub fn new() -> Self {
        Self {
            positive_words: Self::default_positive_words(),
            negative_words: Self::default_negative_words(),
            positive_emojis: Self::default_positive_emojis(),
            negative_emojis: Self::default_negative_emojis(),
        }
    }
    
    /// Palabras positivas básicas
    fn default_positive_words() -> Vec<String> {
        vec![
            // Español
            "feliz", "amor", "excelente", "genial", "increíble", "hermoso", "bueno",
            "gracias", "perfecto", "maravilloso", "alegre", "contento", "bien",
            "mejor", "éxito", "lindo", "bonito", "fantástico", "super", "jaja",
            "jeje", "jiji", "encanta", "adoro", "amo",
            // Inglés
            "happy", "love", "excellent", "great", "amazing", "beautiful", "good",
            "thanks", "perfect", "wonderful", "glad", "nice", "best", "success",
            "awesome", "fantastic", "haha", "hehe",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
    
    /// Palabras negativas básicas
    fn default_negative_words() -> Vec<String> {
        vec![
            // Español
            "triste", "malo", "terrible", "horrible", "odio", "miedo", "preocupado",
            "enojado", "molesto", "difícil", "problema", "error", "fallo", "mal",
            "peor", "nunca", "imposible", "dolor", "sufrir",
            // Inglés
            "sad", "bad", "terrible", "horrible", "hate", "fear", "worried",
            "angry", "upset", "difficult", "problem", "error", "fail", "never",
            "impossible", "pain", "suffer", "wrong", "worst",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
    
    /// Emojis positivos
    fn default_positive_emojis() -> Vec<String> {
        vec!["😊", "😁", "😄", "😃", "🙂", "😍", "🥰", "❤️", "💕", "👍", "✅", "🎉", "🎊", "💪", "😎"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Emojis negativos
    fn default_negative_emojis() -> Vec<String> {
        vec!["😢", "😭", "😞", "😔", "😟", "😠", "😡", "💔", "👎", "❌", "😰", "😨", "😩"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Analiza sentiment de un texto
    fn analyze_sentiment(&self, text: &str) -> SentimentAnalysis {
        let lower = text.to_lowercase();
        
        let mut positive_count = 0;
        let mut negative_count = 0;
        
        // Contar palabras positivas
        for word in &self.positive_words {
            positive_count += lower.matches(word.as_str()).count();
        }
        
        // Contar palabras negativas
        for word in &self.negative_words {
            negative_count += lower.matches(word.as_str()).count();
        }
        
        // Contar emojis positivos
        for emoji in &self.positive_emojis {
            positive_count += text.matches(emoji.as_str()).count();
        }
        
        // Contar emojis negativos
        for emoji in &self.negative_emojis {
            negative_count += text.matches(emoji.as_str()).count();
        }
        
        // Patrones de puntuación
        let exclamations = text.matches('!').count();
        let questions = text.matches('?').count();
        
        if exclamations >= 2 {
            positive_count += 1; // Emoción fuerte
        }
        
        if questions >= 2 {
            negative_count += 1; // Confusión/preocupación
        }
        
        // Capitalización
        let uppercase_words = text
            .split_whitespace()
            .filter(|w| w.len() > 2 && w.chars().all(|c| c.is_uppercase()))
            .count();
        
        if uppercase_words > 0 {
            positive_count += uppercase_words; // Énfasis (puede ser positivo o negativo, asumimos positivo)
        }
        
        // Calcular score
        let total = positive_count + negative_count;
        let sentiment_score = if total > 0 {
            (positive_count as f64 - negative_count as f64) / total as f64
        } else {
            0.0 // Neutral
        };
        
        // Clasificar
        let sentiment = if sentiment_score > 0.2 {
            Sentiment::Positive
        } else if sentiment_score < -0.2 {
            Sentiment::Negative
        } else {
            Sentiment::Neutral
        };
        
        // Calcular intensidad
        let intensity = ((positive_count + negative_count) as f64 / 10.0).min(1.0);
        
        SentimentAnalysis {
            sentiment,
            score: sentiment_score,
            intensity,
            positive_signals: positive_count,
            negative_signals: negative_count,
        }
    }
}

#[async_trait]
impl NutrientExtractor for EmotionalExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Emotional
    }
    
    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        let mut nutrients = Vec::new();
        
        // Filtrar solo entries de texto
        let text_entries: Vec<_> = digested
            .entries
            .iter()
            .filter(|e| self.can_process(e))
            .collect();
        
        metadata.entries_skipped = digested.entries.len() - text_entries.len();
        
        // Analizar sentiment de cada mensaje
        let mut total_positive = 0;
        let mut total_negative = 0;
        let mut total_neutral = 0;
        
        for entry in &text_entries {
            let analysis = self.analyze_sentiment(&entry.content);
            
            // Crear nutriente por mensaje
            let nutrient = Nutrient::new(
                entry.id.clone(),
                "sentiment".to_string(),
                format!("{:?}", analysis.sentiment),
                analysis.intensity,
            )
            .with_metadata("score".to_string(), analysis.score.to_string())
            .with_metadata("intensity".to_string(), analysis.intensity.to_string())
            .with_metadata("positive_signals".to_string(), analysis.positive_signals.to_string())
            .with_metadata("negative_signals".to_string(), analysis.negative_signals.to_string());
            
            nutrients.push(nutrient);
            
            // Acumular estadísticas
            match analysis.sentiment {
                Sentiment::Positive => total_positive += 1,
                Sentiment::Negative => total_negative += 1,
                Sentiment::Neutral => total_neutral += 1,
            }
        }
        
        // Finalizar metadata
        metadata.duration_ms = start.elapsed().as_millis() as u64;
        metadata.finalize(&nutrients);
        metadata.stats.insert("positive_messages".to_string(), total_positive.to_string());
        metadata.stats.insert("negative_messages".to_string(), total_negative.to_string());
        metadata.stats.insert("neutral_messages".to_string(), total_neutral.to_string());
        
        let positive_pct = if !text_entries.is_empty() {
            (total_positive as f64 / text_entries.len() as f64 * 100.0).round()
        } else {
            0.0
        };
        metadata.stats.insert("positive_percentage".to_string(), format!("{:.1}", positive_pct));
        
        Ok(ExtractedNutrients {
            dimension: self.dimension(),
            nutrients,
            extraction_metadata: metadata,
        })
    }
    
    fn can_process(&self, entry: &DigestedEntry) -> bool {
        use super::digestion::ContentType;
        matches!(entry.content_type, ContentType::Text) && !entry.content.is_empty()
    }
}

impl Default for EmotionalExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// BIOGRAPHICAL EXTRACTOR (Task 7.x.3.2)
// ================================================================

/// Extract biographical information (names, locations, age, occupation)
///
/// **Features:**
/// - Name detection (author metadata + NER patterns)
/// - Location detection (city/country patterns)
/// - Age inference (birthday messages, age mentions)
/// - Occupation detection (job-related keywords)
/// - Confidence scoring per field
/// - Evidence tracking
pub struct BiographicalExtractor {
    /// Common name patterns for detection
    name_patterns: Vec<Regex>,
    
    /// Location patterns (cities, countries)
    location_patterns: Vec<Regex>,
    
    /// Age/birthday patterns
    age_patterns: Vec<Regex>,
    
    /// Occupation keywords
    occupation_keywords: Vec<String>,
}

impl BiographicalExtractor {
    pub fn new() -> Self {
        Self {
            name_patterns: Self::build_name_patterns(),
            location_patterns: Self::build_location_patterns(),
            age_patterns: Self::build_age_patterns(),
            occupation_keywords: Self::build_occupation_keywords(),
        }
    }
    
    fn build_name_patterns() -> Vec<Regex> {
        vec![
            // "Me llamo X", "Mi nombre es X"
            Regex::new(r"(?i)(me llamo|mi nombre es|soy)\s+([A-Z][a-záéíóúñ]+)").unwrap(),
            // "I'm X", "My name is X"
            Regex::new(r"(?i)(i'?m|my name is)\s+([A-Z][a-z]+)").unwrap(),
        ]
    }
    
    fn build_location_patterns() -> Vec<Regex> {
        vec![
            // "Vivo en X", "Estoy en X"
            Regex::new(r"(?i)(vivo en|estoy en|desde)\s+([A-Z][a-záéíóúñ]+)").unwrap(),
            // "I live in X", "I'm from X"
            Regex::new(r"(?i)(i live in|i'?m from|living in)\s+([A-Z][a-z]+)").unwrap(),
        ]
    }
    
    fn build_age_patterns() -> Vec<Regex> {
        vec![
            // "Tengo X años", "cumplí X"
            Regex::new(r"(?i)(tengo|cumplí|cumplo)\s+(\d{1,2})\s+(años?|añitos)").unwrap(),
            // "I'm X years old", "I turned X"
            Regex::new(r"(?i)(i'?m|i turned)\s+(\d{1,2})\s+(years? old)?").unwrap(),
            // Birthday dates
            Regex::new(r"(?i)(mi cumpleaños|my birthday|nací)\s*(es|is|el)?\s*(\d{1,2})[/-](\d{1,2})").unwrap(),
        ]
    }
    
    fn build_occupation_keywords() -> Vec<String> {
        vec![
            // Spanish
            "trabajo", "empresa", "oficina", "jefe", "compañero", "proyecto",
            "reunión", "cliente", "empleado", "ingeniero", "doctor", "profesor",
            "desarrollador", "diseñador", "gerente", "director", "estudiante",
            // English
            "work", "job", "company", "office", "boss", "colleague", "project",
            "meeting", "client", "employee", "engineer", "doctor", "teacher",
            "developer", "designer", "manager", "director", "student",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }
    
    /// Extract biographical data from conversation
    fn extract_biographical_data(&self, digested: &DigestedData) -> BiographicalData {
        let mut data = BiographicalData::new();
        
        // 1. Names from author metadata
        let authors: std::collections::HashSet<_> = digested
            .entries
            .iter()
            .filter_map(|e| e.author.as_ref())
            .map(|a| a.trim().to_string())
            .collect();
        
        for author in authors {
            if !author.is_empty() && author != "Unknown" {
                data.names.push(NameEvidence {
                    name: author.clone(),
                    confidence: 0.95, // High confidence (from metadata)
                    source: "author_metadata".to_string(),
                    context: None,
                });
            }
        }
        
        // 2. Names from content patterns
        for entry in &digested.entries {
            let content = &entry.content;
            
            for pattern in &self.name_patterns {
                if let Some(captures) = pattern.captures(content) {
                    if let Some(name_match) = captures.get(2) {
                        let name = name_match.as_str().to_string();
                        data.names.push(NameEvidence {
                            name: name.clone(),
                            confidence: 0.7, // Medium confidence (from pattern)
                            source: "content_pattern".to_string(),
                            context: Some(content.clone()),
                        });
                    }
                }
            }
            
            // 3. Locations from patterns
            for pattern in &self.location_patterns {
                if let Some(captures) = pattern.captures(content) {
                    if let Some(location_match) = captures.get(2) {
                        let location = location_match.as_str().to_string();
                        data.locations.push(LocationEvidence {
                            location: location.clone(),
                            confidence: 0.6, // Medium-low confidence (needs validation)
                            source: "content_pattern".to_string(),
                            context: Some(content.clone()),
                        });
                    }
                }
            }
            
            // 4. Age from patterns
            for pattern in &self.age_patterns {
                if let Some(captures) = pattern.captures(content) {
                    // Try to extract age number
                    for i in 2..captures.len() {
                        if let Some(age_match) = captures.get(i) {
                            if let Ok(age) = age_match.as_str().parse::<u32>() {
                                if age >= 5 && age <= 120 { // Sanity check
                                    data.age_mentions.push(AgeEvidence {
                                        age,
                                        confidence: 0.8,
                                        source: "content_pattern".to_string(),
                                        context: Some(content.clone()),
                                        timestamp: entry.timestamp,
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            
            // 5. Occupation from keywords
            let content_lower = content.to_lowercase();
            for keyword in &self.occupation_keywords {
                if content_lower.contains(keyword) {
                    data.occupation_mentions.push(OccupationEvidence {
                        keyword: keyword.clone(),
                        confidence: 0.5, // Low confidence (just keyword presence)
                        source: "keyword_match".to_string(),
                        context: Some(content.clone()),
                    });
                }
            }
        }
        
        data
    }
}

#[async_trait]
impl NutrientExtractor for BiographicalExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Biographical
    }
    
    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        let mut nutrients = Vec::new();
        
        // Extract biographical data
        let bio_data = self.extract_biographical_data(digested);
        
        // Convert to nutrients
        
        // 1. Names (aggregated with highest confidence)
        let mut name_map: HashMap<String, f64> = HashMap::new();
        for evidence in &bio_data.names {
            let entry = name_map.entry(evidence.name.clone()).or_insert(0.0);
            *entry = entry.max(evidence.confidence);
        }
        
        let unique_names_count = name_map.len();
        
        for (name, confidence) in name_map {
            let nutrient = Nutrient::new(
                format!("name_{}", uuid::Uuid::new_v4()),
                "name".to_string(),
                name.clone(),
                confidence,
            )
            .with_metadata("field".to_string(), "name".to_string())
            .with_metadata("evidence_count".to_string(), 
                bio_data.names.iter().filter(|e| e.name == name).count().to_string());
            
            nutrients.push(nutrient);
        }
        
        // 2. Locations (aggregated)
        let mut location_map: HashMap<String, f64> = HashMap::new();
        for evidence in &bio_data.locations {
            let entry = location_map.entry(evidence.location.clone()).or_insert(0.0);
            *entry = entry.max(evidence.confidence);
        }
        
        let unique_locations_count = location_map.len();
        
        for (location, confidence) in location_map {
            let nutrient = Nutrient::new(
                format!("location_{}", uuid::Uuid::new_v4()),
                "location".to_string(),
                location.clone(),
                confidence,
            )
            .with_metadata("field".to_string(), "location".to_string())
            .with_metadata("evidence_count".to_string(),
                bio_data.locations.iter().filter(|e| e.location == location).count().to_string());
            
            nutrients.push(nutrient);
        }
        
        // 3. Age (most recent mention with highest confidence)
        if let Some(age_evidence) = bio_data.age_mentions.iter()
            .max_by(|a, b| {
                // Compare by timestamp first, then confidence
                match a.timestamp.cmp(&b.timestamp) {
                    std::cmp::Ordering::Equal => a.confidence.partial_cmp(&b.confidence).unwrap(),
                    other => other,
                }
            }) {
            let nutrient = Nutrient::new(
                format!("age_{}", uuid::Uuid::new_v4()),
                "age".to_string(),
                age_evidence.age.to_string(),
                age_evidence.confidence,
            )
            .with_metadata("field".to_string(), "age".to_string())
            .with_metadata("timestamp".to_string(), age_evidence.timestamp.to_rfc3339());
            
            nutrients.push(nutrient);
        }
        
        // 4. Occupation (keyword frequency)
        let mut occupation_freq: HashMap<String, usize> = HashMap::new();
        for evidence in &bio_data.occupation_mentions {
            *occupation_freq.entry(evidence.keyword.clone()).or_insert(0) += 1;
        }
        
        let occupation_keywords_count = occupation_freq.len();
        
        // Only include if frequency > 2 (reduce noise)
        for (keyword, freq) in occupation_freq {
            if freq > 2 {
                let confidence = (freq as f64 / digested.entries.len() as f64).min(0.8);
                let nutrient = Nutrient::new(
                    format!("occupation_{}", uuid::Uuid::new_v4()),
                    "occupation_indicator".to_string(),
                    keyword.clone(),
                    confidence,
                )
                .with_metadata("field".to_string(), "occupation".to_string())
                .with_metadata("frequency".to_string(), freq.to_string());
                
                nutrients.push(nutrient);
            }
        }
        
        // Finalize metadata
        metadata.duration_ms = start.elapsed().as_millis() as u64;
        metadata.finalize(&nutrients);
        metadata.stats.insert("unique_names".to_string(), unique_names_count.to_string());
        metadata.stats.insert("unique_locations".to_string(), unique_locations_count.to_string());
        metadata.stats.insert("age_mentions".to_string(), bio_data.age_mentions.len().to_string());
        metadata.stats.insert("occupation_keywords".to_string(), occupation_keywords_count.to_string());
        
        Ok(ExtractedNutrients {
            dimension: self.dimension(),
            nutrients,
            extraction_metadata: metadata,
        })
    }
    
    fn can_process(&self, entry: &DigestedEntry) -> bool {
        use super::digestion::ContentType;
        matches!(entry.content_type, ContentType::Text) && !entry.content.is_empty()
    }
}

impl Default for BiographicalExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Biographical data structure
#[derive(Debug, Clone)]
struct BiographicalData {
    names: Vec<NameEvidence>,
    locations: Vec<LocationEvidence>,
    age_mentions: Vec<AgeEvidence>,
    occupation_mentions: Vec<OccupationEvidence>,
}

impl BiographicalData {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            locations: Vec::new(),
            age_mentions: Vec::new(),
            occupation_mentions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct NameEvidence {
    name: String,
    confidence: f64,
    source: String,
    context: Option<String>,
}

#[derive(Debug, Clone)]
struct LocationEvidence {
    location: String,
    confidence: f64,
    source: String,
    context: Option<String>,
}

#[derive(Debug, Clone)]
struct AgeEvidence {
    age: u32,
    confidence: f64,
    source: String,
    context: Option<String>,
    timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct OccupationEvidence {
    keyword: String,
    confidence: f64,
    source: String,
    context: Option<String>,
}

// ================================================================
// HELPER STRUCTURES
// ================================================================

/// Sentiment classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

/// Resultado de análisis de sentiment
#[derive(Debug, Clone)]
struct SentimentAnalysis {
    /// Clasificación (Positive/Negative/Neutral)
    sentiment: Sentiment,
    
    /// Score (-1.0 a 1.0)
    score: f64,
    
    /// Intensidad emocional (0.0 a 1.0)
    intensity: f64,
    
    /// Número de señales positivas detectadas
    positive_signals: usize,
    
    /// Número de señales negativas detectadas
    negative_signals: usize,
}

// ================================================================
// TEMPORAL EXTRACTOR (Task 7.x.3.6)
// ================================================================

/// Extract temporal patterns (routines, life events, activity patterns)
///
/// **Features:**
/// - Daily routines detection (morning/afternoon/evening/night patterns)
/// - Weekly patterns (weekday vs weekend behavior)
/// - Life events timeline (significant dates, milestones)
/// - Communication frequency analysis
/// - Response time patterns
/// - Seasonal/cyclical patterns
pub struct TemporalExtractor {
    /// Minimum messages required to detect a routine
    min_routine_samples: usize,
}

impl TemporalExtractor {
    pub fn new() -> Self {
        Self {
            min_routine_samples: 5,
        }
    }
    
    /// Analyze temporal patterns from conversation
    fn analyze_temporal_patterns(&self, digested: &DigestedData) -> TemporalAnalysis {
        let mut analysis = TemporalAnalysis::new();
        
        if digested.entries.is_empty() {
            return analysis;
        }
        
        // 1. Time of day patterns
        let mut hour_counts: HashMap<u32, usize> = HashMap::new();
        for entry in &digested.entries {
            let hour = entry.timestamp.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }
        
        // Classify into time slots
        let morning = (6..12).map(|h| *hour_counts.get(&h).unwrap_or(&0)).sum::<usize>();
        let afternoon = (12..18).map(|h| *hour_counts.get(&h).unwrap_or(&0)).sum::<usize>();
        let evening = (18..23).map(|h| *hour_counts.get(&h).unwrap_or(&0)).sum::<usize>();
        let night = (0..6).chain(23..24).map(|h| *hour_counts.get(&h).unwrap_or(&0)).sum::<usize>();
        
        let total = morning + afternoon + evening + night;
        if total > 0 {
            analysis.time_of_day = TimeOfDayPattern {
                morning_pct: morning as f64 / total as f64,
                afternoon_pct: afternoon as f64 / total as f64,
                evening_pct: evening as f64 / total as f64,
                night_pct: night as f64 / total as f64,
            };
            
            // Determine most active period
            analysis.most_active_period = if morning >= afternoon && morning >= evening && morning >= night {
                "morning".to_string()
            } else if afternoon >= evening && afternoon >= night {
                "afternoon".to_string()
            } else if evening >= night {
                "evening".to_string()
            } else {
                "night".to_string()
            };
        }
        
        // 2. Day of week patterns
        let mut weekday_count = 0;
        let mut weekend_count = 0;
        
        for entry in &digested.entries {
            let weekday = entry.timestamp.weekday();
            match weekday {
                chrono::Weekday::Mon | chrono::Weekday::Tue | chrono::Weekday::Wed 
                | chrono::Weekday::Thu | chrono::Weekday::Fri => weekday_count += 1,
                chrono::Weekday::Sat | chrono::Weekday::Sun => weekend_count += 1,
            }
        }
        
        let total_days = weekday_count + weekend_count;
        if total_days > 0 {
            analysis.weekday_weekend = WeekdayWeekendPattern {
                weekday_pct: weekday_count as f64 / total_days as f64,
                weekend_pct: weekend_count as f64 / total_days as f64,
            };
        }
        
        // 3. Communication frequency
        let duration_days = (digested.entries.last().unwrap().timestamp 
            - digested.entries.first().unwrap().timestamp).num_days();
        
        if duration_days > 0 {
            analysis.messages_per_day = digested.entries.len() as f64 / duration_days as f64;
            
            analysis.frequency_pattern = if analysis.messages_per_day >= 20.0 {
                FrequencyPattern::MultipleDaily
            } else if analysis.messages_per_day >= 5.0 {
                FrequencyPattern::Daily
            } else if analysis.messages_per_day >= 0.5 {
                FrequencyPattern::Weekly
            } else if analysis.messages_per_day >= 0.1 {
                FrequencyPattern::Monthly
            } else {
                FrequencyPattern::Sporadic
            };
        }
        
        // 4. Response time analysis (if multiple authors)
        if digested.digestion_metadata.stats.get("participant_count")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0) > 1 {
            
            analysis.avg_response_time = self.calculate_avg_response_time(&digested.entries);
        }
        
        // 5. Life events detection (significant date patterns)
        analysis.life_events = self.detect_life_events(&digested.entries);
        
        analysis
    }
    
    /// Calculate average response time between authors
    fn calculate_avg_response_time(&self, entries: &[DigestedEntry]) -> Option<i64> {
        let mut response_times = Vec::new();
        let mut last_author: Option<&str> = None;
        let mut last_timestamp: Option<DateTime<Utc>> = None;
        
        for entry in entries {
            if let Some(author) = &entry.author {
                if let (Some(prev_author), Some(prev_time)) = (last_author, last_timestamp) {
                    if prev_author != author.as_str() {
                        let diff = (entry.timestamp - prev_time).num_minutes();
                        if diff > 0 && diff < 60 * 24 { // Max 24h
                            response_times.push(diff);
                        }
                    }
                }
                last_author = Some(author.as_str());
                last_timestamp = Some(entry.timestamp);
            }
        }
        
        if response_times.is_empty() {
            None
        } else {
            Some(response_times.iter().sum::<i64>() / response_times.len() as i64)
        }
    }
    
    /// Detect significant life events from messages
    fn detect_life_events(&self, entries: &[DigestedEntry]) -> Vec<LifeEvent> {
        let mut events = Vec::new();
        
        // Keywords for different event types
        let birthday_keywords = ["cumpleaños", "birthday", "cumplí", "feliz cumple", "happy birthday", "nací"];
        let milestone_keywords = ["graduación", "graduation", "boda", "wedding", "trabajo nuevo", "new job", "promoción", "promotion"];
        let travel_keywords = ["viaje", "travel", "vacaciones", "vacation", "aeropuerto", "airport", "vuelo", "flight"];
        
        for entry in entries {
            use super::digestion::ContentType;
            if !matches!(entry.content_type, ContentType::Text) {
                continue;
            }
            
            let content_lower = entry.content.to_lowercase();
            
            // Check birthday
            if birthday_keywords.iter().any(|&kw| content_lower.contains(kw)) {
                events.push(LifeEvent {
                    event_type: "birthday".to_string(),
                    timestamp: entry.timestamp,
                    confidence: 0.8,
                    description: entry.content.chars().take(100).collect(),
                });
            }
            
            // Check milestones
            if milestone_keywords.iter().any(|&kw| content_lower.contains(kw)) {
                events.push(LifeEvent {
                    event_type: "milestone".to_string(),
                    timestamp: entry.timestamp,
                    confidence: 0.7,
                    description: entry.content.chars().take(100).collect(),
                });
            }
            
            // Check travel
            if travel_keywords.iter().any(|&kw| content_lower.contains(kw)) {
                events.push(LifeEvent {
                    event_type: "travel".to_string(),
                    timestamp: entry.timestamp,
                    confidence: 0.6,
                    description: entry.content.chars().take(100).collect(),
                });
            }
        }
        
        events
    }
}

#[async_trait]
impl NutrientExtractor for TemporalExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Temporal
    }
    
    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        let mut nutrients = Vec::new();
        
        // Analyze temporal patterns
        let analysis = self.analyze_temporal_patterns(digested);
        
        // Convert to nutrients
        
        // 1. Time of day pattern
        let time_pattern_json = serde_json::to_string(&analysis.time_of_day)?;
        nutrients.push(Nutrient::new(
            "time_of_day_pattern".to_string(),
            "time_of_day".to_string(),
            time_pattern_json,
            0.9,
        )
        .with_metadata("most_active".to_string(), analysis.most_active_period.clone())
        .with_metadata("morning_pct".to_string(), format!("{:.2}", analysis.time_of_day.morning_pct))
        .with_metadata("afternoon_pct".to_string(), format!("{:.2}", analysis.time_of_day.afternoon_pct))
        .with_metadata("evening_pct".to_string(), format!("{:.2}", analysis.time_of_day.evening_pct))
        .with_metadata("night_pct".to_string(), format!("{:.2}", analysis.time_of_day.night_pct)));
        
        // 2. Weekday/weekend pattern
        let weekday_pattern_json = serde_json::to_string(&analysis.weekday_weekend)?;
        nutrients.push(Nutrient::new(
            "weekday_weekend_pattern".to_string(),
            "weekday_weekend".to_string(),
            weekday_pattern_json,
            0.9,
        )
        .with_metadata("weekday_pct".to_string(), format!("{:.2}", analysis.weekday_weekend.weekday_pct))
        .with_metadata("weekend_pct".to_string(), format!("{:.2}", analysis.weekday_weekend.weekend_pct)));
        
        // 3. Frequency pattern
        nutrients.push(Nutrient::new(
            "frequency_pattern".to_string(),
            "frequency".to_string(),
            format!("{:?}", analysis.frequency_pattern),
            0.95,
        )
        .with_metadata("messages_per_day".to_string(), format!("{:.2}", analysis.messages_per_day)));
        
        // 4. Response time (if available)
        if let Some(avg_response_min) = analysis.avg_response_time {
            nutrients.push(Nutrient::new(
                "avg_response_time".to_string(),
                "response_time".to_string(),
                format!("{} minutes", avg_response_min),
                0.85,
            )
            .with_metadata("minutes".to_string(), avg_response_min.to_string()));
        }
        
        // 5. Life events
        for event in &analysis.life_events {
            let event_json = serde_json::to_string(event)?;
            nutrients.push(Nutrient::new(
                format!("life_event_{}", event.timestamp.timestamp()),
                "life_event".to_string(),
                event_json,
                event.confidence,
            )
            .with_metadata("event_type".to_string(), event.event_type.clone())
            .with_metadata("timestamp".to_string(), event.timestamp.to_rfc3339())
            .with_metadata("description_preview".to_string(), event.description.chars().take(50).collect()));
        }
        
        // Finalize metadata
        metadata.duration_ms = start.elapsed().as_millis() as u64;
        metadata.finalize(&nutrients);
        metadata.stats.insert("time_of_day_analyzed".to_string(), "true".to_string());
        metadata.stats.insert("most_active_period".to_string(), analysis.most_active_period.clone());
        metadata.stats.insert("frequency_pattern".to_string(), format!("{:?}", analysis.frequency_pattern));
        metadata.stats.insert("life_events_detected".to_string(), analysis.life_events.len().to_string());
        if let Some(avg) = analysis.avg_response_time {
            metadata.stats.insert("avg_response_time_min".to_string(), avg.to_string());
        }
        
        Ok(ExtractedNutrients {
            dimension: self.dimension(),
            nutrients,
            extraction_metadata: metadata,
        })
    }
    
    fn can_process(&self, entry: &DigestedEntry) -> bool {
        // Temporal patterns are extracted from the entire conversation,
        // not individual entries
        true
    }
}

impl Default for TemporalExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Temporal analysis results
#[derive(Debug, Clone)]
struct TemporalAnalysis {
    time_of_day: TimeOfDayPattern,
    most_active_period: String,
    weekday_weekend: WeekdayWeekendPattern,
    messages_per_day: f64,
    frequency_pattern: FrequencyPattern,
    avg_response_time: Option<i64>, // minutes
    life_events: Vec<LifeEvent>,
}

impl TemporalAnalysis {
    fn new() -> Self {
        Self {
            time_of_day: TimeOfDayPattern::default(),
            most_active_period: "unknown".to_string(),
            weekday_weekend: WeekdayWeekendPattern::default(),
            messages_per_day: 0.0,
            frequency_pattern: FrequencyPattern::Unknown,
            avg_response_time: None,
            life_events: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TimeOfDayPattern {
    morning_pct: f64,    // 6am-12pm
    afternoon_pct: f64,  // 12pm-6pm
    evening_pct: f64,    // 6pm-11pm
    night_pct: f64,      // 11pm-6am
}

impl Default for TimeOfDayPattern {
    fn default() -> Self {
        Self {
            morning_pct: 0.0,
            afternoon_pct: 0.0,
            evening_pct: 0.0,
            night_pct: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeekdayWeekendPattern {
    weekday_pct: f64,
    weekend_pct: f64,
}

impl Default for WeekdayWeekendPattern {
    fn default() -> Self {
        Self {
            weekday_pct: 0.0,
            weekend_pct: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrequencyPattern {
    MultipleDaily,  // 20+ messages/day
    Daily,          // 5-20 messages/day
    Weekly,         // 0.5-5 messages/day
    Monthly,        // 0.1-0.5 messages/day
    Sporadic,       // <0.1 messages/day
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LifeEvent {
    event_type: String,
    timestamp: DateTime<Utc>,
    confidence: f64,
    description: String,
}

// ================================================================
// 6. BEHAVIORAL EXTRACTOR - Communication Style & Habits
// ================================================================

/// BehavioralExtractor: Analiza patrones de comunicación y hábitos conductuales
///
/// **Detecta:**
/// 1. Communication Style: Formal/Casual/Mixed, based on language patterns
/// 2. Message Characteristics: Length patterns (short/medium/long), punctuation usage
/// 3. Activity Patterns: Session clustering, break durations, active/inactive periods
/// 4. Engagement Style: Response patterns (immediate/delayed), message bursts vs steady
///
/// **Confidence Scoring:**
/// - High (0.8-1.0): Strong patterns from 100+ samples
/// - Medium (0.5-0.8): Moderate patterns from 50-100 samples
/// - Low (0.0-0.5): Weak patterns or <50 samples
pub struct BehavioralExtractor;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CommunicationStyle {
    Formal,     // Uses formal language, complete sentences, proper punctuation
    Casual,     // Informal language, abbreviations, emojis
    Mixed,      // Combination of both styles
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageLengthPattern {
    VeryShort,  // <10 characters (avg)
    Short,      // 10-50 characters
    Medium,     // 50-150 characters
    Long,       // 150-500 characters
    VeryLong,   // >500 characters
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum EngagementStyle {
    Bursty,     // Messages come in bursts (3+ messages within 5 minutes)
    Steady,     // Evenly distributed throughout the day
    Delayed,    // Long gaps between messages (hours)
    Unknown,
}

#[async_trait]
impl NutrientExtractor for BehavioralExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Behavioral
    }

    fn can_process(&self, entry: &DigestedEntry) -> bool {
        !entry.content.is_empty()
    }

    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut nutrients = Vec::new();

        // Communication style analysis
        let style_nutrient = self.analyze_communication_style(&digested.entries);
        nutrients.push(style_nutrient);

        // Message length patterns
        let length_nutrient = self.analyze_message_length(&digested.entries);
        nutrients.push(length_nutrient);

        // Engagement style
        let engagement_nutrient = self.analyze_engagement_style(&digested.entries);
        nutrients.push(engagement_nutrient);

        // Punctuation usage
        let punctuation_nutrient = self.analyze_punctuation_patterns(&digested.entries);
        nutrients.push(punctuation_nutrient);

        let duration_ms = start.elapsed().as_millis() as u64;

        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        metadata.duration_ms = duration_ms;
        metadata.finalize(&nutrients);
        metadata.stats.insert("communication_style_analyzed".to_string(), "true".to_string());
        metadata.stats.insert("message_length_analyzed".to_string(), "true".to_string());
        metadata.stats.insert("engagement_style_analyzed".to_string(), "true".to_string());
        metadata.stats.insert("punctuation_analyzed".to_string(), "true".to_string());

        Ok(ExtractedNutrients {
            dimension: NutrientDimension::Behavioral,
            nutrients,
            extraction_metadata: metadata,
        })
    }
}

impl BehavioralExtractor {
    /// Analyze communication style (formal vs casual)
    fn analyze_communication_style(&self, entries: &[DigestedEntry]) -> Nutrient {
        let mut formal_indicators = 0;
        let mut casual_indicators = 0;
        let total_messages = entries.len();

        // Formal indicators: proper punctuation, complete words, no emojis
        // Casual indicators: abbreviations, emojis, lowercase only
        for entry in entries {
            let content = entry.content.to_lowercase();

            // Formal indicators
            if content.contains("please") || content.contains("thank you") || content.contains("regards") {
                formal_indicators += 1;
            }
            if entry.content.chars().next().map_or(false, |c| c.is_uppercase()) {
                formal_indicators += 1;
            }

            // Casual indicators
            if content.contains("lol") || content.contains("omg") || content.contains("btw") {
                casual_indicators += 1;
            }
            if entry.content.contains('😊') || entry.content.contains('❤') || entry.content.contains('👍') {
                casual_indicators += 1;
            }
            if content.chars().all(|c| !c.is_uppercase() || !c.is_alphabetic()) {
                casual_indicators += 1;
            }
        }

        let formal_pct = formal_indicators as f64 / (total_messages as f64 * 2.0); // 2 indicators per message max
        let casual_pct = casual_indicators as f64 / (total_messages as f64 * 3.0); // 3 indicators per message max

        let style = if formal_pct > 0.4 && casual_pct < 0.3 {
            CommunicationStyle::Formal
        } else if casual_pct > 0.4 && formal_pct < 0.3 {
            CommunicationStyle::Casual
        } else if formal_pct > 0.2 && casual_pct > 0.2 {
            CommunicationStyle::Mixed
        } else {
            CommunicationStyle::Unknown
        };

        let confidence = ((formal_pct - casual_pct).abs() * 2.0).min(1.0).max(0.5);

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "communication_style".to_string(),
            value: format!("{:?}", style),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("formal_score".to_string(), format!("{:.2}", formal_pct)),
                ("casual_score".to_string(), format!("{:.2}", casual_pct)),
                ("total_messages".to_string(), total_messages.to_string()),
            ]),
        }
    }

    /// Analyze average message length patterns
    fn analyze_message_length(&self, entries: &[DigestedEntry]) -> Nutrient {
        let total_chars: usize = entries.iter().map(|e| e.content.len()).sum();
        let avg_length = if entries.is_empty() {
            0.0
        } else {
            total_chars as f64 / entries.len() as f64
        };

        let pattern = if avg_length < 10.0 {
            MessageLengthPattern::VeryShort
        } else if avg_length < 50.0 {
            MessageLengthPattern::Short
        } else if avg_length < 150.0 {
            MessageLengthPattern::Medium
        } else if avg_length < 500.0 {
            MessageLengthPattern::Long
        } else {
            MessageLengthPattern::VeryLong
        };

        let confidence = if entries.len() >= 100 {
            0.9
        } else if entries.len() >= 50 {
            0.7
        } else {
            0.5
        };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "message_length".to_string(),
            value: format!("{:?}", pattern),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("average_length".to_string(), format!("{:.1}", avg_length)),
                ("total_messages".to_string(), entries.len().to_string()),
            ]),
        }
    }

    /// Analyze engagement style (bursty, steady, delayed)
    fn analyze_engagement_style(&self, entries: &[DigestedEntry]) -> Nutrient {
        if entries.len() < 2 {
            return Nutrient {
                id: uuid::Uuid::new_v4().to_string(),
                source_entry_id: "aggregated".to_string(),
                nutrient_type: "engagement_style".to_string(),
                value: "Unknown".to_string(),
                confidence: 0.0,
                extracted_at: Utc::now(),
                metadata: HashMap::new(),
            };
        }

        // Sort entries by timestamp
        let mut sorted_entries = entries.to_vec();
        sorted_entries.sort_by_key(|e| e.timestamp);

        let mut burst_count = 0;
        let mut long_gap_count = 0;
        let mut gaps = Vec::new();

        for i in 1..sorted_entries.len() {
            let gap = sorted_entries[i].timestamp.signed_duration_since(sorted_entries[i - 1].timestamp);
            let gap_minutes = gap.num_minutes();
            gaps.push(gap_minutes);

            if gap_minutes <= 5 {
                burst_count += 1;
            } else if gap_minutes >= 60 {
                long_gap_count += 1;
            }
        }

        let burst_pct = burst_count as f64 / gaps.len() as f64;
        let long_gap_pct = long_gap_count as f64 / gaps.len() as f64;

        let style = if burst_pct > 0.4 {
            EngagementStyle::Bursty
        } else if long_gap_pct > 0.4 {
            EngagementStyle::Delayed
        } else {
            EngagementStyle::Steady
        };

        let confidence = if gaps.len() >= 100 {
            0.9
        } else if gaps.len() >= 50 {
            0.7
        } else {
            0.5
        };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "engagement_style".to_string(),
            value: format!("{:?}", style),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("burst_percentage".to_string(), format!("{:.2}", burst_pct)),
                ("long_gap_percentage".to_string(), format!("{:.2}", long_gap_pct)),
                ("total_gaps_analyzed".to_string(), gaps.len().to_string()),
            ]),
        }
    }

    /// Analyze punctuation usage patterns
    fn analyze_punctuation_patterns(&self, entries: &[DigestedEntry]) -> Nutrient {
        let mut exclamation_count = 0;
        let mut question_count = 0;
        let mut period_count = 0;
        let mut emoji_count = 0;

        for entry in entries {
            exclamation_count += entry.content.matches('!').count();
            question_count += entry.content.matches('?').count();
            period_count += entry.content.matches('.').count();
            emoji_count += entry.content.chars().filter(|c| {
                let c = *c as u32;
                (0x1F600..=0x1F64F).contains(&c) || // Emoticons
                (0x1F300..=0x1F5FF).contains(&c) || // Misc Symbols
                (0x1F680..=0x1F6FF).contains(&c) || // Transport
                (0x2600..=0x26FF).contains(&c) ||   // Misc symbols
                (0x2700..=0x27BF).contains(&c)      // Dingbats
            }).count();
        }

        let total_messages = entries.len() as f64;
        let exclamation_per_msg = exclamation_count as f64 / total_messages;
        let question_per_msg = question_count as f64 / total_messages;
        let period_per_msg = period_count as f64 / total_messages;
        let emoji_per_msg = emoji_count as f64 / total_messages;

        let enthusiasm_score = exclamation_per_msg + emoji_per_msg;
        let confidence = if entries.len() >= 100 { 0.9 } else if entries.len() >= 50 { 0.7 } else { 0.5 };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "punctuation_pattern".to_string(),
            value: if enthusiasm_score > 1.0 { "Enthusiastic" } else if period_per_msg > exclamation_per_msg { "Formal" } else { "Balanced" }.to_string(),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("exclamations_per_msg".to_string(), format!("{:.2}", exclamation_per_msg)),
                ("questions_per_msg".to_string(), format!("{:.2}", question_per_msg)),
                ("periods_per_msg".to_string(), format!("{:.2}", period_per_msg)),
                ("emojis_per_msg".to_string(), format!("{:.2}", emoji_per_msg)),
                ("enthusiasm_score".to_string(), format!("{:.2}", enthusiasm_score)),
            ]),
        }
    }
}

// ================================================================
// 7. RELATIONSHIP EXTRACTOR - Social Graph & Dynamics
// ================================================================

/// RelationshipExtractor: Analiza relaciones sociales y dinámicas interpersonales
///
/// **Detecta:**
/// 1. Relationship Participants: Who communicates with whom
/// 2. Interaction Strength: Message frequency between participants
/// 3. Relationship Type: Family/Friend/Romantic/Professional (basic classification)
/// 4. Communication Balance: Bidirectional vs unidirectional
///
/// **Confidence Scoring:**
/// - High (0.8-1.0): Clear patterns from 50+ interactions
/// - Medium (0.5-0.8): Moderate patterns from 20-50 interactions
/// - Low (0.0-0.5): Weak patterns or <20 interactions
pub struct RelationshipExtractor;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum RelationshipType {
    Romantic,      // Love language, terms of endearment, high emotional content
    Family,        // Family references, formal/informal mix
    Friend,        // Casual, shared interests, social activities
    Professional,  // Formal language, work-related content
    Ambiguous,     // Cannot classify with confidence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelationshipPair {
    person_a: String,
    person_b: String,
    interaction_count: usize,
    relationship_type: RelationshipType,
    confidence: f64,
}

#[async_trait]
impl NutrientExtractor for RelationshipExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Social
    }

    fn can_process(&self, entry: &DigestedEntry) -> bool {
        entry.author.is_some()
    }

    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        let start = std::time::Instant::now();
        let mut nutrients = Vec::new();

        // Extract unique participants
        let participants = self.extract_participants(&digested.entries);
        nutrients.push(participants);

        // Analyze interaction patterns
        let interactions = self.analyze_interactions(&digested.entries);
        nutrients.extend(interactions);

        // Detect relationship types (if multiple authors)
        let authors: Vec<String> = digested.entries.iter()
            .filter_map(|e| e.author.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        if authors.len() >= 2 {
            let relationship = self.classify_relationship(&digested.entries, &authors);
            nutrients.push(relationship);
        }

        // Communication balance analysis
        let balance = self.analyze_communication_balance(&digested.entries);
        nutrients.push(balance);

        let duration_ms = start.elapsed().as_millis() as u64;

        let mut metadata = ExtractionMetadata::new(digested.entries.len());
        metadata.duration_ms = duration_ms;
        metadata.finalize(&nutrients);
        metadata.stats.insert("participants_detected".to_string(), authors.len().to_string());
        metadata.stats.insert("relationship_classified".to_string(), (authors.len() >= 2).to_string());

        Ok(ExtractedNutrients {
            dimension: NutrientDimension::Social,
            nutrients,
            extraction_metadata: metadata,
        })
    }
}

impl RelationshipExtractor {
    /// Extract unique participants from conversation
    fn extract_participants(&self, entries: &[DigestedEntry]) -> Nutrient {
        let participants: std::collections::HashSet<String> = entries.iter()
            .filter_map(|e| e.author.clone())
            .collect();

        let participant_list: Vec<String> = participants.iter().cloned().collect();
        let confidence = if entries.len() >= 50 { 0.95 } else if entries.len() >= 20 { 0.8 } else { 0.6 };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "participants".to_string(),
            value: participant_list.join(", "),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("participant_count".to_string(), participants.len().to_string()),
                ("total_messages".to_string(), entries.len().to_string()),
            ]),
        }
    }

    /// Analyze interaction patterns between participants
    fn analyze_interactions(&self, entries: &[DigestedEntry]) -> Vec<Nutrient> {
        let mut author_counts: HashMap<String, usize> = HashMap::new();
        
        for entry in entries {
            if let Some(author) = &entry.author {
                *author_counts.entry(author.clone()).or_insert(0) += 1;
            }
        }

        let total_messages = entries.len();
        let mut nutrients = Vec::new();

        for (author, count) in author_counts.iter() {
            let percentage = (*count as f64 / total_messages as f64) * 100.0;
            let confidence = if *count >= 50 { 0.9 } else if *count >= 20 { 0.7 } else { 0.5 };

            nutrients.push(Nutrient {
                id: uuid::Uuid::new_v4().to_string(),
                source_entry_id: "aggregated".to_string(),
                nutrient_type: "interaction_strength".to_string(),
                value: author.clone(),
                confidence,
                extracted_at: Utc::now(),
                metadata: HashMap::from([
                    ("message_count".to_string(), count.to_string()),
                    ("percentage".to_string(), format!("{:.1}", percentage)),
                ]),
            });
        }

        nutrients
    }

    /// Classify relationship type based on content
    fn classify_relationship(&self, entries: &[DigestedEntry], authors: &[String]) -> Nutrient {
        let mut romantic_score = 0.0;
        let mut family_score = 0.0;
        let mut friend_score = 0.0;
        let mut professional_score = 0.0;

        // Romantic indicators
        let romantic_keywords = ["amor", "love", "te amo", "i love you", "cariño", "mi amor", 
                                  "babe", "baby", "corazón", "heart", "❤", "😘", "💕"];
        
        // Family indicators
        let family_keywords = ["mamá", "papá", "mom", "dad", "hijo", "hija", "hermano", 
                               "hermana", "abuela", "abuelo", "tío", "tía"];
        
        // Friend indicators
        let friend_keywords = ["amigo", "friend", "compa", "bro", "viejo", "wey", 
                               "salir", "fiesta", "party", "chelas", "cerveza"];
        
        // Professional indicators
        let professional_keywords = ["reunión", "meeting", "proyecto", "project", "cliente",
                                      "jefe", "boss", "trabajo", "work", "oficina", "office"];

        for entry in entries {
            let content_lower = entry.content.to_lowercase();
            
            for keyword in &romantic_keywords {
                if content_lower.contains(keyword) {
                    romantic_score += 1.0;
                }
            }
            
            for keyword in &family_keywords {
                if content_lower.contains(keyword) {
                    family_score += 1.0;
                }
            }
            
            for keyword in &friend_keywords {
                if content_lower.contains(keyword) {
                    friend_score += 1.0;
                }
            }
            
            for keyword in &professional_keywords {
                if content_lower.contains(keyword) {
                    professional_score += 1.0;
                }
            }
        }

        let total_score = romantic_score + family_score + friend_score + professional_score;
        
        let (relationship_type, type_score) = if total_score == 0.0 {
            (RelationshipType::Ambiguous, 0.0)
        } else {
            let romantic_pct = romantic_score / total_score;
            let family_pct = family_score / total_score;
            let friend_pct = friend_score / total_score;
            let professional_pct = professional_score / total_score;

            if romantic_pct > 0.4 {
                (RelationshipType::Romantic, romantic_pct)
            } else if family_pct > 0.4 {
                (RelationshipType::Family, family_pct)
            } else if professional_pct > 0.4 {
                (RelationshipType::Professional, professional_pct)
            } else if friend_pct > 0.3 {
                (RelationshipType::Friend, friend_pct)
            } else {
                (RelationshipType::Ambiguous, 0.0)
            }
        };

        let confidence = if entries.len() >= 100 && type_score > 0.3 {
            0.8
        } else if entries.len() >= 50 && type_score > 0.2 {
            0.6
        } else {
            0.4
        };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "relationship_type".to_string(),
            value: format!("{:?}", relationship_type),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("participants".to_string(), authors.join(" & ")),
                ("romantic_score".to_string(), format!("{:.2}", romantic_score)),
                ("family_score".to_string(), format!("{:.2}", family_score)),
                ("friend_score".to_string(), format!("{:.2}", friend_score)),
                ("professional_score".to_string(), format!("{:.2}", professional_score)),
            ]),
        }
    }

    /// Analyze communication balance (bidirectional vs unidirectional)
    fn analyze_communication_balance(&self, entries: &[DigestedEntry]) -> Nutrient {
        let mut author_counts: HashMap<String, usize> = HashMap::new();
        
        for entry in entries {
            if let Some(author) = &entry.author {
                *author_counts.entry(author.clone()).or_insert(0) += 1;
            }
        }

        if author_counts.len() < 2 {
            return Nutrient {
                id: uuid::Uuid::new_v4().to_string(),
                source_entry_id: "aggregated".to_string(),
                nutrient_type: "communication_balance".to_string(),
                value: "Monologue".to_string(),
                confidence: 0.9,
                extracted_at: Utc::now(),
                metadata: HashMap::from([
                    ("author_count".to_string(), "1".to_string()),
                    ("balance_type".to_string(), "monologue".to_string()),
                ]),
            };
        }

        let counts: Vec<usize> = author_counts.values().cloned().collect();
        let max_count = *counts.iter().max().unwrap() as f64;
        let min_count = *counts.iter().min().unwrap() as f64;
        
        let balance_ratio = min_count / max_count;
        
        let (balance_type, confidence) = if balance_ratio >= 0.7 {
            ("Balanced", 0.9)
        } else if balance_ratio >= 0.4 {
            ("Moderately_Balanced", 0.7)
        } else {
            ("Unbalanced", 0.8)
        };

        Nutrient {
            id: uuid::Uuid::new_v4().to_string(),
            source_entry_id: "aggregated".to_string(),
            nutrient_type: "communication_balance".to_string(),
            value: balance_type.to_string(),
            confidence,
            extracted_at: Utc::now(),
            metadata: HashMap::from([
                ("balance_ratio".to_string(), format!("{:.2}", balance_ratio)),
                ("author_count".to_string(), author_counts.len().to_string()),
                ("most_active_count".to_string(), max_count.to_string()),
                ("least_active_count".to_string(), min_count.to_string()),
            ]),
        }
    }
}
