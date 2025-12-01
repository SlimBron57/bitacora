```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/01_sensory-engine.md
Versión: 1.0
Fecha Creación: 2025-10-26
Autor: Sistema Bitácora - Template MTT-DSL component_spec.yaml
Propósito: Especificación completa del componente SENSORY ENGINE (procesamiento multimodal de input)
Estado: ESPECIFICACIÓN - Pendiente implementación
Relacionado Con: BITA-2_ACA-7D_SPECIFICATION.md, SISTEMA_DUAL_DATABASES.md
Implementa: DA-009 (SENSORY ENGINE brecha crítica #3)
Template Usado: 07_TEMPLATES/component_spec.yaml v1.0
# === FIN DATOS DE AUDITORÍA ===
```

# 🎤 SENSORY ENGINE - Procesamiento Multimodal de Input

---

## 🎯 PROPÓSITO

**SENSORY ENGINE** es el componente de entrada multimodal que procesa **texto, voz y datos visuales** (futuro), normalizándolos en un formato unificado para alimentar el **Context Token 7D**.

### El Problema que Resuelve

Las interacciones humanas no son solo texto. Hablamos, compartimos imágenes, links, archivos. Un sistema que solo entiende texto pierde el **70% del contexto comunicativo** (tono de voz, urgencia, formato del input).

**Escenario real:**
```
Usuario envía mensaje de voz: "Urgente! El servidor está caído, 
necesito saber cómo reiniciar el servicio de PostgreSQL AHORA"

Sin SENSORY ENGINE:
→ Sistema no puede procesar audio
→ Usuario debe transcribir manualmente
→ Se pierde tono de urgencia y contexto emocional
→ Respuesta genérica sin priorización

Con SENSORY ENGINE:
→ Whisper API transcribe audio (2 segundos)
→ Detecta tono urgente + keywords técnicos
→ Context Token 7D: emotional=0.9, intentional=0.95, syntactic=technical
→ TelescopeDB busca experiencias previas con PostgreSQL
→ Respuesta priorizada: "Comando exacto: sudo systemctl restart postgresql"
→ Total: <5 segundos desde audio hasta solución
```

### Por Qué es Crítico

1. **Naturalidad:** Permite interacción humana real (no solo typing)
2. **Contexto Enriquecido:** Tono de voz + pausas + énfasis = mejor análisis 7D
3. **Accesibilidad:** Usuarios pueden hablar mientras conducen, caminan, cocinan
4. **Eficiencia:** Hablar es 3x más rápido que escribir

### Relación con Arquitectura General

SENSORY ENGINE es el **"traductor universal"** de Bitácora:
- Input multimodal → Formato normalizado
- Normalizado → Context Token 7D
- CTX7D → TelescopeDB + VoxelDB + HubSpoke

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                   USER INTERACTION LAYER                    │
└─────────────────────────────────────────────────────────────┘
           │              │              │
           ▼              ▼              ▼
    ┌──────────┐  ┌──────────┐  ┌──────────┐
    │  TEXT    │  │  AUDIO   │  │  VISUAL  │
    │  Input   │  │  Input   │  │  Input   │
    │          │  │ (Whisper)│  │ (futuro) │
    └────┬─────┘  └────┬─────┘  └────┬─────┘
         │             │             │
         └─────────────┼─────────────┘
                       │
                       ▼
         ┌─────────────────────────────────┐
         │      SENSORY ENGINE             │ ← AQUÍ ESTAMOS
         │  • Text Processor               │
         │  • Audio Transcriber (Whisper)  │
         │  • Visual Analyzer (prep)       │
         │  • Input Normalizer             │
         │  • Metadata Extractor           │
         └────────────┬────────────────────┘
                      │
                      ▼
         ┌─────────────────────────────────┐
         │   Normalized Input (JSON)       │
         │  {                               │
         │    content: "...",               │
         │    modality: "audio",            │
         │    metadata: {                   │
         │      tone: "urgent",             │
         │      duration_s: 8.5,            │
         │      detected_keywords: [...]    │
         │    }                             │
         │  }                               │
         └────────────┬────────────────────┘
                      │
                      ▼
         ┌─────────────────────────────────┐
         │    Context Token 7D Generator    │
         │    (Analiza input normalizado)   │
         └────────────┬────────────────────┘
                      │
                      ▼
         ┌─────────────────────────────────┐
         │         TELESCOPEDB              │
         │    (Persiste experiencia)        │
         └──────────────────────────────────┘
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito | Frecuencia |
|------------|-----------|-----------|------------|
| **User Interface** | → SENSORY ENGINE | Enviar input multimodal | Cada interacción |
| **SENSORY ENGINE** | → Context Token 7D | Pasar input normalizado | Cada procesamiento |
| **SENSORY ENGINE** | → TelescopeDB | Persistir input crudo + metadata | Cada entrada |
| **Whisper API** | ← SENSORY ENGINE | Transcribir audio | Solo inputs de voz |
| **SENSORY ENGINE** | → Cost Tracking | Registrar costos API | Cada llamada externa |

### Qué Depende de SENSORY ENGINE

**Crítico (no puede funcionar sin SENSORY ENGINE):**
- Context Token 7D (necesita input normalizado)
- TelescopeDB (almacena output de SENSORY)
- Toda la pipeline de respuesta (sin input, no hay procesamiento)

**Importante (degraded mode sin SENSORY ENGINE):**
- Solo funcionaría input de texto plano
- No hay metadata de tono, urgencia, contexto multimodal

---

## 📋 RESPONSABILIDADES CORE

SENSORY ENGINE tiene **7 responsabilidades fundamentales**:

### 1. **Procesamiento de Texto Plano** (MUST HAVE)
- Normalizar encoding (UTF-8)
- Detectar idioma automáticamente
- Extraer keywords y entidades
- Sanitizar input (XSS, injection prevention)

### 2. **Transcripción de Audio via Whisper** (MUST HAVE)
- Integración con Whisper API (OpenAI)
- Fallback a Whisper local si API falla
- Detectar idioma del audio
- Extraer metadata: duración, pausas, velocidad

### 3. **Análisis de Tono y Emoción** (MUST HAVE)
- Detectar tono de voz (urgente, neutral, casual)
- Análisis de sentimiento (positivo/neutral/negativo)
- Identificar énfasis (palabras en mayúsculas, repetición)
- Calcular dimensión emocional para CTX7D

### 4. **Normalización de Input** (MUST HAVE)
- Output estandarizado: `NormalizedInput` struct
- Metadata unificado independiente de modalidad
- Timestamp preciso (UTC)
- Content-addressable ID (SHA-256)

### 5. **Extracción de Metadata Contextual** (MUST HAVE)
- Detectar keywords técnicos (lenguajes, frameworks, comandos)
- Identificar intención (pregunta, comando, reflexión)
- Extraer referencias (URLs, file paths, comandos)
- Calcular complejidad del input

### 6. **Procesamiento Visual (Preparación v2.0)** (NICE TO HAVE)
- Stub para análisis de imágenes
- OCR para texto en imágenes
- Detección de diagramas, código screenshots
- Integración con GPT-4V (futuro)

### 7. **Cost Tracking y Optimización** (MUST HAVE - DA-009)
- Trackear costos de Whisper API ($0.006/minuto)
- Optimizar: usar Whisper local para audios cortos (<10s)
- Cache de transcripciones frecuentes
- Documentar en `SANDBOX/cost_tracking/`

---

## 🗂️ ESTRUCTURAS DE DATOS

### Estructura Principal: SensoryEngine

```rust
// src/cells/sensory_engine/mod.rs

pub struct SensoryEngine {
    /// Procesador de texto
    text_processor: TextProcessor,
    
    /// Transcriptor de audio (Whisper)
    audio_transcriber: AudioTranscriber,
    
    /// Analizador visual (stub para v2.0)
    visual_analyzer: Option<VisualAnalyzer>,
    
    /// Configuración de procesamiento
    config: SensoryConfig,
    
    /// Tracker de costos API
    cost_tracker: CostTracker,
    
    /// Cache de procesamiento reciente
    processing_cache: LruCache<String, NormalizedInput>,
    
    /// Métricas de uso
    metrics: SensoryMetrics,
}

/// Configuración de SENSORY ENGINE
#[derive(Debug, Clone)]
pub struct SensoryConfig {
    /// Habilitar Whisper API (vs local)
    pub use_whisper_api: bool,
    
    /// Threshold de duración para usar API (segundos)
    /// Audios <10s usan Whisper local, >10s usan API
    pub api_threshold_seconds: u32,  // Default: 10
    
    /// Modelo de Whisper ("whisper-1" para API)
    pub whisper_model: String,  // Default: "whisper-1"
    
    /// Habilitar detección de idioma automática
    pub auto_detect_language: bool,  // Default: true
    
    /// Idioma por defecto si no se detecta
    pub default_language: String,  // Default: "es"
    
    /// Habilitar análisis de sentimiento
    pub enable_sentiment_analysis: bool,  // Default: true
    
    /// Tamaño de cache de procesamiento
    pub cache_size: usize,  // Default: 500
}

impl Default for SensoryConfig {
    fn default() -> Self {
        Self {
            use_whisper_api: true,
            api_threshold_seconds: 10,
            whisper_model: "whisper-1".to_string(),
            auto_detect_language: true,
            default_language: "es".to_string(),
            enable_sentiment_analysis: true,
            cache_size: 500,
        }
    }
}

/// Input normalizado (output de SENSORY ENGINE)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedInput {
    /// ID único (content-addressable)
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
    
    /// Keywords y entidades extraídas
    pub extracted_keywords: Vec<String>,
    
    /// Referencias detectadas (URLs, paths, comandos)
    pub references: Vec<Reference>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum InputModality {
    Text,
    Audio,
    Visual,
    Mixed,  // Múltiples modalidades
}

/// Metadata específico de cada modalidad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMetadata {
    /// Para audio: duración en segundos
    pub duration_seconds: Option<f64>,
    
    /// Para audio: velocidad de habla (palabras/minuto)
    pub speech_rate_wpm: Option<u32>,
    
    /// Para audio: pausas detectadas (timestamps)
    pub pauses: Vec<f64>,
    
    /// Para visual: dimensiones (ancho × alto)
    pub dimensions: Option<(u32, u32)>,
    
    /// Para visual: formato (PNG, JPEG, etc.)
    pub format: Option<String>,
    
    /// Encoding original (UTF-8, etc.)
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
            pauses: Vec::new(),
            dimensions: None,
            format: None,
            encoding: "UTF-8".to_string(),
            original_size_bytes: 0,
            processing_time_ms: 0,
        }
    }
}

/// Análisis de tono y emoción
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneAnalysis {
    /// Tono general (urgent, neutral, casual, formal)
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
    CodeSnippet,
    TechnicalTerm,
    Person,
    Project,
}

/// Procesador de texto
pub struct TextProcessor {
    /// Detector de idioma
    language_detector: LanguageDetector,
    
    /// Extractor de keywords
    keyword_extractor: KeywordExtractor,
    
    /// Analizador de sentimiento
    sentiment_analyzer: Option<SentimentAnalyzer>,
}

impl TextProcessor {
    pub fn new(config: &SensoryConfig) -> Self {
        Self {
            language_detector: LanguageDetector::new(),
            keyword_extractor: KeywordExtractor::new(),
            sentiment_analyzer: if config.enable_sentiment_analysis {
                Some(SentimentAnalyzer::new())
            } else {
                None
            },
        }
    }
    
    /// Procesar texto plano
    pub fn process(&self, text: &str) -> Result<NormalizedInput> {
        let start = Instant::now();
        
        // 1. Normalizar encoding
        let normalized_text = self.normalize_encoding(text)?;
        
        // 2. Detectar idioma
        let language = self.language_detector.detect(&normalized_text)?;
        
        // 3. Extraer keywords
        let keywords = self.keyword_extractor.extract(&normalized_text, &language)?;
        
        // 4. Analizar tono
        let tone_analysis = self.analyze_tone(&normalized_text)?;
        
        // 5. Detectar referencias
        let references = self.detect_references(&normalized_text)?;
        
        let processing_time = start.elapsed().as_millis() as u64;
        
        Ok(NormalizedInput {
            id: sha256(&normalized_text),
            content: normalized_text,
            modality: InputModality::Text,
            language,
            metadata: InputMetadata {
                original_size_bytes: text.len(),
                processing_time_ms: processing_time,
                ..Default::default()
            },
            processed_at: Utc::now(),
            tone_analysis,
            extracted_keywords: keywords,
            references,
        })
    }
    
    fn analyze_tone(&self, text: &str) -> Result<ToneAnalysis> {
        let mut tone_analysis = ToneAnalysis::default();
        
        // Detectar urgencia (keywords como "urgente", "ahora", "YA")
        let urgency_keywords = ["urgent", "ahora", "ya", "crítico", "emergencia", "help"];
        let text_lower = text.to_lowercase();
        let urgency_count = urgency_keywords.iter()
            .filter(|&&kw| text_lower.contains(kw))
            .count();
        
        tone_analysis.urgency_level = (urgency_count as f64 / 3.0).min(1.0);
        
        // Detectar énfasis (mayúsculas, repetición)
        let uppercase_ratio = text.chars().filter(|c| c.is_uppercase()).count() as f64 
                             / text.chars().filter(|c| c.is_alphabetic()).count() as f64;
        tone_analysis.emphasis_level = uppercase_ratio * 2.0;
        
        // Determinar tono general
        tone_analysis.overall_tone = if tone_analysis.urgency_level > 0.7 {
            Tone::Urgent
        } else if uppercase_ratio > 0.3 {
            Tone::Frustrated
        } else if text.contains('?') && text.contains("cómo") {
            Tone::Confused
        } else {
            Tone::Neutral
        };
        
        // Análisis de sentimiento (si está habilitado)
        if let Some(ref analyzer) = self.sentiment_analyzer {
            tone_analysis.sentiment_score = analyzer.analyze(text)?;
        }
        
        tone_analysis.confidence = 0.8;
        
        Ok(tone_analysis)
    }
    
    fn detect_references(&self, text: &str) -> Result<Vec<Reference>> {
        let mut references = Vec::new();
        
        // Detectar URLs
        let url_regex = regex::Regex::new(r"https?://[^\s]+").unwrap();
        for cap in url_regex.captures_iter(text) {
            references.push(Reference {
                ref_type: ReferenceType::Url,
                value: cap[0].to_string(),
                context: Some(self.get_context(text, &cap[0])),
            });
        }
        
        // Detectar file paths (Unix y Windows)
        let path_regex = regex::Regex::new(r"(?:[a-zA-Z]:)?[/\\][\w/\\.-]+").unwrap();
        for cap in path_regex.captures_iter(text) {
            references.push(Reference {
                ref_type: ReferenceType::FilePath,
                value: cap[0].to_string(),
                context: Some(self.get_context(text, &cap[0])),
            });
        }
        
        // Detectar comandos (líneas que empiezan con $ o >)
        for line in text.lines() {
            if line.trim_start().starts_with('$') || line.trim_start().starts_with('>') {
                references.push(Reference {
                    ref_type: ReferenceType::Command,
                    value: line.trim_start().trim_start_matches(['$', '>']).trim().to_string(),
                    context: None,
                });
            }
        }
        
        Ok(references)
    }
}

/// Transcriptor de audio (Whisper)
pub struct AudioTranscriber {
    /// Cliente OpenAI (para Whisper API)
    openai_client: Option<OpenAIClient>,
    
    /// Whisper local (fallback)
    local_whisper: Option<WhisperLocal>,
    
    /// Configuración
    config: SensoryConfig,
}

impl AudioTranscriber {
    pub fn new(config: SensoryConfig, openai_api_key: Option<String>) -> Self {
        Self {
            openai_client: openai_api_key.map(|key| OpenAIClient::new(key)),
            local_whisper: Some(WhisperLocal::new()),
            config,
        }
    }
    
    /// Transcribir audio
    pub async fn transcribe(&mut self, audio_data: &[u8], format: AudioFormat) -> Result<NormalizedInput> {
        let start = Instant::now();
        
        // 1. Determinar duración del audio
        let duration = self.estimate_duration(audio_data, format)?;
        
        // 2. Decidir si usar API o local
        let (transcription, cost) = if self.config.use_whisper_api 
                                       && duration > self.config.api_threshold_seconds as f64 
                                       && self.openai_client.is_some() {
            // Usar Whisper API
            let result = self.transcribe_via_api(audio_data, format).await?;
            (result.text, Some(result.cost))
        } else {
            // Usar Whisper local
            let text = self.transcribe_local(audio_data, format).await?;
            (text, None)
        };
        
        // 3. Analizar transcripción con TextProcessor
        let text_processor = TextProcessor::new(&self.config);
        let mut normalized = text_processor.process(&transcription)?;
        
        // 4. Actualizar metadata específico de audio
        normalized.modality = InputModality::Audio;
        normalized.metadata.duration_seconds = Some(duration);
        normalized.metadata.speech_rate_wpm = Some(self.calculate_speech_rate(&transcription, duration));
        normalized.metadata.pauses = self.detect_pauses(audio_data)?;
        normalized.metadata.processing_time_ms = start.elapsed().as_millis() as u64;
        
        // 5. Trackear costo si se usó API
        if let Some(cost) = cost {
            // TODO: Registrar en cost_tracker
        }
        
        Ok(normalized)
    }
    
    async fn transcribe_via_api(&self, audio_data: &[u8], format: AudioFormat) -> Result<TranscriptionResult> {
        let client = self.openai_client.as_ref()
            .ok_or_else(|| SensoryError::WhisperAPINotConfigured)?;
        
        // Llamar a Whisper API
        let response = client.transcribe(audio_data, format, &self.config.whisper_model).await?;
        
        // Calcular costo ($0.006 por minuto)
        let duration_minutes = self.estimate_duration(audio_data, format)? / 60.0;
        let cost = duration_minutes * 0.006;
        
        Ok(TranscriptionResult {
            text: response.text,
            language: response.language.unwrap_or_else(|| self.config.default_language.clone()),
            cost,
        })
    }
    
    async fn transcribe_local(&self, audio_data: &[u8], format: AudioFormat) -> Result<String> {
        let whisper = self.local_whisper.as_ref()
            .ok_or_else(|| SensoryError::WhisperLocalNotAvailable)?;
        
        whisper.transcribe(audio_data, format).await
    }
    
    fn calculate_speech_rate(&self, transcription: &str, duration_seconds: f64) -> u32 {
        let word_count = transcription.split_whitespace().count();
        ((word_count as f64 / duration_seconds) * 60.0) as u32
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Ogg,
    Flac,
    WebM,
}

struct TranscriptionResult {
    text: String,
    language: String,
    cost: f64,
}
```

---

## 🔌 API PÚBLICA

### Operaciones Principales

```rust
impl SensoryEngine {
    /// Crear nueva instancia de SENSORY ENGINE
    pub fn new(config: SensoryConfig, openai_api_key: Option<String>) -> Result<Self> {
        Ok(Self {
            text_processor: TextProcessor::new(&config),
            audio_transcriber: AudioTranscriber::new(config.clone(), openai_api_key),
            visual_analyzer: None,  // v2.0
            config,
            cost_tracker: CostTracker::new(),
            processing_cache: LruCache::new(config.cache_size),
            metrics: SensoryMetrics::default(),
        })
    }
    
    /// Procesar input de texto
    pub async fn process_text(&mut self, text: String) -> Result<NormalizedInput> {
        // Verificar cache
        let cache_key = sha256(&text);
        if let Some(cached) = self.processing_cache.get(&cache_key) {
            self.metrics.cache_hits += 1;
            return Ok(cached.clone());
        }
        
        self.metrics.cache_misses += 1;
        
        // Procesar
        let normalized = self.text_processor.process(&text)?;
        
        // Añadir a cache
        self.processing_cache.put(cache_key, normalized.clone());
        
        // Actualizar métricas
        self.metrics.total_text_inputs += 1;
        
        Ok(normalized)
    }
    
    /// Procesar input de audio
    pub async fn process_audio(
        &mut self,
        audio_data: Vec<u8>,
        format: AudioFormat,
    ) -> Result<NormalizedInput> {
        let normalized = self.audio_transcriber.transcribe(&audio_data, format).await?;
        
        // Actualizar métricas
        self.metrics.total_audio_inputs += 1;
        if let Some(duration) = normalized.metadata.duration_seconds {
            self.metrics.total_audio_duration_seconds += duration;
        }
        
        Ok(normalized)
    }
    
    /// Procesar input visual (stub v2.0)
    pub async fn process_visual(
        &mut self,
        _image_data: Vec<u8>,
        _format: ImageFormat,
    ) -> Result<NormalizedInput> {
        // TODO: Implementar en v2.0 con GPT-4V
        Err(SensoryError::VisualProcessingNotImplemented.into())
    }
    
    /// Procesar input mixto (múltiples modalidades)
    pub async fn process_mixed(
        &mut self,
        inputs: Vec<(InputModality, Vec<u8>)>,
    ) -> Result<NormalizedInput> {
        // Procesar cada modalidad por separado
        let mut results = Vec::new();
        for (modality, data) in inputs {
            match modality {
                InputModality::Text => {
                    let text = String::from_utf8(data)?;
                    results.push(self.process_text(text).await?);
                }
                InputModality::Audio => {
                    // Asumir formato WAV por defecto
                    results.push(self.process_audio(data, AudioFormat::Wav).await?);
                }
                _ => {}
            }
        }
        
        // Combinar resultados
        self.merge_normalized_inputs(results)
    }
    
    fn merge_normalized_inputs(&self, inputs: Vec<NormalizedInput>) -> Result<NormalizedInput> {
        if inputs.is_empty() {
            return Err(SensoryError::NoInputsToMerge.into());
        }
        
        // Combinar contenidos
        let merged_content = inputs.iter()
            .map(|i| i.content.as_str())
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");
        
        // Combinar keywords
        let mut all_keywords: Vec<String> = inputs.iter()
            .flat_map(|i| i.extracted_keywords.clone())
            .collect();
        all_keywords.sort();
        all_keywords.dedup();
        
        // Combinar referencias
        let all_references: Vec<Reference> = inputs.iter()
            .flat_map(|i| i.references.clone())
            .collect();
        
        // Calcular tono promedio
        let avg_sentiment = inputs.iter().map(|i| i.tone_analysis.sentiment_score).sum::<f64>() 
                           / inputs.len() as f64;
        let avg_urgency = inputs.iter().map(|i| i.tone_analysis.urgency_level).sum::<f64>() 
                         / inputs.len() as f64;
        
        Ok(NormalizedInput {
            id: sha256(&merged_content),
            content: merged_content,
            modality: InputModality::Mixed,
            language: inputs[0].language.clone(),
            metadata: InputMetadata::default(),
            processed_at: Utc::now(),
            tone_analysis: ToneAnalysis {
                overall_tone: Tone::Neutral,
                sentiment_score: avg_sentiment,
                urgency_level: avg_urgency,
                emphasis_level: 0.0,
                confidence: 0.7,
            },
            extracted_keywords: all_keywords,
            references: all_references,
        })
    }
    
    /// Obtener métricas de uso
    pub fn get_metrics(&self) -> &SensoryMetrics {
        &self.metrics
    }
    
    /// Obtener estadísticas de costos
    pub fn get_cost_stats(&self) -> CostStats {
        self.cost_tracker.get_stats()
    }
}

#[derive(Default)]
pub struct SensoryMetrics {
    pub total_text_inputs: usize,
    pub total_audio_inputs: usize,
    pub total_visual_inputs: usize,
    pub total_audio_duration_seconds: f64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

pub struct CostTracker {
    total_cost_usd: f64,
    whisper_api_calls: usize,
}

impl CostTracker {
    pub fn new() -> Self {
        Self {
            total_cost_usd: 0.0,
            whisper_api_calls: 0,
        }
    }
    
    pub fn get_stats(&self) -> CostStats {
        CostStats {
            total_cost_usd: self.total_cost_usd,
            whisper_api_calls: self.whisper_api_calls,
        }
    }
}

pub struct CostStats {
    pub total_cost_usd: f64,
    pub whisper_api_calls: usize,
}
```

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito | Crítico |
|------------|---------|-----------|---------|
| **Context Token 7D** | v1.0 | Recibe input normalizado | ✅ SÍ |
| **TelescopeDB** | v1.0 | Almacena input crudo + metadata | ✅ SÍ |
| **Cost Tracking** | v1.0 | Registra costos de APIs | ✅ SÍ (DA-009) |

### Crates Externos

```toml
[dependencies]
# Serialización
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client (para Whisper API)
reqwest = { version = "0.11", features = ["json", "multipart"] }

# Hashing
sha2 = "0.10"

# Regex para detección de referencias
regex = "1.10"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Cache
lru = "0.12"

# Dates
chrono = "0.4"

# Detección de idioma
whichlang = "0.1"  # Lightweight language detector

# Análisis de sentimiento (opcional)
vader_sentiment = { version = "0.1", optional = true }

# Whisper local (fallback)
whisper-rs = { version = "0.10", optional = true }

# Logging
tracing = "0.1"
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

### Benchmarks Esperados

| Operación | Target | Justificación | Status |
|-----------|--------|---------------|--------|
| **process_text() 100 chars** | <5ms | Regex + keyword extraction es O(n) | ⏸️ TBD |
| **process_text() 10K chars** | <50ms | Procesamiento lineal | ⏸️ TBD |
| **process_audio() 10s (API)** | <2s | Whisper API latency ~1.5s | ⏸️ TBD |
| **process_audio() 10s (local)** | <5s | Whisper local más lento pero sin latencia red | ⏸️ TBD |
| **cache_hit_rate** | >70% | Usuarios repiten queries similares | ⏸️ TBD |
| **language_detection_accuracy** | >95% | whichlang library benchmark | ⏸️ TBD |

### Complejidad Algorítmica

| Operación | Complejidad | Notas |
|-----------|-------------|-------|
| Text Processing | O(n) | n = longitud del texto |
| Keyword Extraction | O(n log n) | Sort de keywords |
| Reference Detection | O(n × r) | r = número de regexes (~5) |
| Audio Transcription (API) | O(1) + latency | Procesamiento en servidor OpenAI |
| Audio Transcription (local) | O(d × m) | d = duración, m = modelo size |

**Donde:**
- n = Longitud del input (caracteres)
- d = Duración del audio (segundos)
- r = Número de patrones de referencia
- m = Tamaño del modelo Whisper

### Uso de Memoria

**Estimación para 1 hora de uso:**
- Cache (500 entries): ~5 MB
- Text buffers: ~2 MB
- Audio buffers (temporal): ~10 MB (liberado después de transcribir)
- Whisper local model (si cargado): ~1.5 GB (memoria compartida)

**Total:** ~17 MB sin Whisper local, ~1.5 GB con Whisper local

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_process_text_basic() {
        let mut engine = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
        let text = "Hola, necesito ayuda con Rust ownership".to_string();
        
        let result = engine.process_text(text).await.unwrap();
        
        assert_eq!(result.modality, InputModality::Text);
        assert_eq!(result.language, "es");
        assert!(result.extracted_keywords.contains(&"Rust".to_string()));
        assert!(result.extracted_keywords.contains(&"ownership".to_string()));
    }
    
    #[tokio::test]
    async fn test_urgency_detection() {
        let mut engine = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
        let urgent_text = "URGENTE! El servidor está caído AHORA!".to_string();
        
        let result = engine.process_text(urgent_text).await.unwrap();
        
        assert!(result.tone_analysis.urgency_level > 0.7);
        assert_eq!(result.tone_analysis.overall_tone, Tone::Urgent);
    }
    
    #[tokio::test]
    async fn test_reference_detection() {
        let mut engine = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
        let text = "Revisa https://example.com y ejecuta $ cargo build".to_string();
        
        let result = engine.process_text(text).await.unwrap();
        
        assert_eq!(result.references.len(), 2);
        assert!(result.references.iter().any(|r| r.ref_type == ReferenceType::Url));
        assert!(result.references.iter().any(|r| r.ref_type == ReferenceType::Command));
    }
    
    #[test]
    fn test_speech_rate_calculation() {
        let transcriber = AudioTranscriber::new(SensoryConfig::default(), None);
        let transcription = "Hello world this is a test of five words per second";
        let duration = 5.0;  // 5 segundos
        
        let wpm = transcriber.calculate_speech_rate(transcription, duration);
        
        // 10 palabras en 5 segundos = 120 wpm
        assert_eq!(wpm, 120);
    }
}
```

### Integration Tests

```rust
// tests/integration/sensory_to_ctx7d.rs

#[tokio::test]
async fn test_sensory_to_context_token_7d() {
    // Setup
    let mut sensory = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
    let ctx7d_generator = ContextToken7DGenerator::new();
    
    // Procesar input
    let text = "Estoy frustrado, el código no compila y llevo 2 horas debuggeando".to_string();
    let normalized = sensory.process_text(text).await.unwrap();
    
    // Generar Context Token 7D
    let ctx7d = ctx7d_generator.generate_from_normalized(&normalized).await.unwrap();
    
    // Verificar dimensiones
    assert!(ctx7d.context_tensor.emotional > 0.6);  // Frustración detectada
    assert!(ctx7d.context_tensor.syntactic < 0.5);   // Lenguaje informal
    assert!(ctx7d.context_tensor.semantic > 0.7);    // Keywords técnicos detectados
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_all_text_inputs_are_processable(
        text in "\\PC{1,1000}"  // Any Unicode string 1-1000 chars
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut engine = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
        
        rt.block_on(async {
            // No debe fallar con ningún input válido
            let result = engine.process_text(text).await;
            prop_assert!(result.is_ok());
        });
    }
}
```

### Performance Benchmarks

```rust
// benches/sensory_engine_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_text_processing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut engine = SensoryEngine::new(SensoryConfig::default(), None).unwrap();
    let text = "This is a sample text with multiple keywords like Rust, async, and performance".to_string();
    
    c.bench_function("process_text_100_chars", |b| {
        b.to_async(&rt).iter(|| async {
            engine.process_text(black_box(text.clone())).await.unwrap()
        });
    });
}

criterion_group!(benches, bench_text_processing);
criterion_main!(benches);
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/cells/sensory_engine/error.rs

#[derive(Debug, thiserror::Error)]
pub enum SensoryError {
    #[error("Whisper API not configured (missing API key)")]
    WhisperAPINotConfigured,
    
    #[error("Whisper local not available")]
    WhisperLocalNotAvailable,
    
    #[error("Audio transcription failed: {0}")]
    TranscriptionFailed(String),
    
    #[error("Language detection failed")]
    LanguageDetectionFailed,
    
    #[error("Visual processing not implemented yet (v2.0)")]
    VisualProcessingNotImplemented,
    
    #[error("No inputs to merge")]
    NoInputsToMerge,
    
    #[error("Invalid audio format: {0}")]
    InvalidAudioFormat(String),
    
    #[error("UTF-8 encoding error: {0}")]
    EncodingError(#[from] std::string::FromUtf8Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, SensoryError>;
```

### Estrategias de Recuperación

```rust
impl SensoryEngine {
    /// Procesar audio con fallback automático
    pub async fn process_audio_with_fallback(
        &mut self,
        audio_data: Vec<u8>,
        format: AudioFormat,
    ) -> Result<NormalizedInput> {
        // Intento 1: Whisper API
        if self.config.use_whisper_api && self.audio_transcriber.openai_client.is_some() {
            match self.audio_transcriber.transcribe_via_api(&audio_data, format).await {
                Ok(result) => {
                    tracing::info!("Transcribed via Whisper API");
                    return self.text_processor.process(&result.text);
                }
                Err(e) => {
                    tracing::warn!("Whisper API failed: {}, falling back to local", e);
                }
            }
        }
        
        // Intento 2: Whisper local
        match self.audio_transcriber.transcribe_local(&audio_data, format).await {
            Ok(text) => {
                tracing::info!("Transcribed via local Whisper");
                self.text_processor.process(&text)
            }
            Err(e) => {
                tracing::error!("Both Whisper API and local failed");
                Err(e)
            }
        }
    }
}
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **00_VISION/BITA-2_ACA-7D_SPECIFICATION.md** - Input normalizado alimenta Context Token 7D
- **00_VISION/DECISIONES_ARQUITECTONICAS.md** - DA-009 (SENSORY ENGINE brecha #3), DA-009 (cost tracking)
- **02_COMPONENTES/CRITICOS/TELESCOPEDB.md** - Almacena output de SENSORY ENGINE
- **02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md** - Consumidor de input normalizado
- **SANDBOX/cost_tracking/** - Documentación de costos API

### APIs Externas

- **OpenAI Whisper API:** [https://platform.openai.com/docs/guides/speech-to-text](https://platform.openai.com/docs/guides/speech-to-text)
- **Pricing:** $0.006/minuto de audio

### Código de Referencia

- **crates/bitacora-core/src/models/analysis.rs** - Estructuras de análisis de contexto
- **crates/bitacora-analytics/src/connectors/** - Procesadores de input externo

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Esta Semana)

1. ✅ **Crear estructura base:** `src/cells/sensory_engine/mod.rs`
2. ✅ **Implementar TextProcessor:** Keyword extraction, tone analysis
3. ✅ **Stub de AudioTranscriber:** Mock inicial, integración real después
4. ✅ **Tests unitarios:** process_text(), urgency detection, references
5. ✅ **Integración con Context Token 7D:** Pasar `NormalizedInput`

### Mejoras v2.0 (Futuro)

1. **Whisper API Integration:** Conectar con OpenAI Whisper real
2. **Whisper Local:** Integrar whisper-rs para fallback
3. **Visual Processing:** GPT-4V para análisis de imágenes
4. **Advanced NLP:** BERT/Transformers para mejor keyword extraction
5. **Multi-language:** Soporte completo para idiomas no-latinos

---

**Estado:** 📋 Especificación completa - Listo para implementación  
**Complejidad:** 🟡 MEDIA - Requiere integración con APIs externas + NLP básico  
**Prioridad:** 🔴 CRÍTICA - Es brecha #3 según DA-009

---

*Generado: 26 Octubre 2025*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec v1.0*  
*"SENSORY ENGINE: Donde tus palabras se vuelven contexto"* 🎤✨
