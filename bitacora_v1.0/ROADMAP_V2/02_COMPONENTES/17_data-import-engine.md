```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/17_data-import-engine.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 12:00:00
Autor: Sistema Bitácora - Phase 7.x Data Import
Propósito: Especificación técnica del Data Import Engine con digestión metabólica
Estado: 📋 ESPECIFICACIÓN v1.0 - Ready for Implementation
Relacionado Con:
  - 00_VISION/09_metabolic-digestion-vision.md (filosofía)
  - 01_ARQUITECTURA/18_metabolic-digestion-system.md (arquitectura)
  - 04_IMPLEMENTACION/PHASE_7X_DATA_IMPORT.md (plan 6 semanas)
Implementa: DA-036 (Data Import as Metabolic Process)
# === FIN DATOS DE AUDITORÍA ===
```

# 🧬 DATA IMPORT ENGINE - Especificación Técnica

> **"No ingerir, digerir. No almacenar, extraer. No ignorar, comprender."**

---

## 📚 TABLA DE CONTENIDOS

1. [Propósito](#propósito)
2. [Arquitectura de 5 Fases](#arquitectura-de-5-fases)
3. [Componente 1: QuarantineZone](#componente-1-quarantinezone)
4. [Componente 2: HybridDigester](#componente-2-hybriddigester)
5. [Componente 3: NutrientExtractor](#componente-3-nutrientextractor)
6. [Componente 4: CoherenceValidator](#componente-4-coherencevalidator)
7. [Componente 5: NutrientDistributor](#componente-5-nutrientdistributor)
8. [Template System](#template-system)
9. [Hyperlink Intelligence](#hyperlink-intelligence)
10. [API Principal](#api-principal)
11. [Performance Targets](#performance-targets)

---

## 🎯 PROPÓSITO

### ¿Qué es Data Import Engine?

El **Data Import Engine** es el sistema que permite a Bitácora importar y **digerir** datos de plataformas externas (WhatsApp, Telegram, Spotify, GitHub, etc.) con:

1. **Digestión metabólica** (5 fases: Quarantine → Digest → Extract → Validate → Distribute)
2. **Respeto por fuente** (cada plataforma tiene su digester único)
3. **Hybrid architecture** (core hard-coded + logic templated)
4. **Hyperlink intelligence** (análisis de URLs compartidas)
5. **Onboarding invisible** (30s vs 30min manual)

### ¿Qué Problema Resuelve?

**Problema:**
```
Usuario nuevo:
  1. Bitácora: "Cuéntame sobre ti"
  2. Usuario: [30 minutos de Q&A tedioso] 😫
  3. Resultado: Comprensión superficial, usuario abandonó
```

**Solución:**
```rust
// Usuario importa WhatsApp backup
let import_result = DataImportEngine::new()
    .import_whatsapp("chat_backup.txt")
    .await?;

// Engine automáticamente:
// 1. Quarantine (inspección segura)
// 2. Digestion (WhatsApp-specific understanding)
// 3. Extraction (7 nutrientes en paralelo)
// 4. Validation (detect conflicts)
// 5. Distribution (TelescopeDB, TopicGraph, etc)

// Resultado: [30 segundos después]
// ✅ 250 biographical facts extracted
// ✅ 45 interest topics discovered
// ✅ 120 relationships mapped
// ✅ 80 hyperlinks analyzed
// ✅ Ready to chat with deep understanding
```

---

## 🏗️ ARQUITECTURA DE 5 FASES

```
┌──────────────────────────────────────────────────────────────┐
│                    DATA IMPORT ENGINE                         │
│                                                               │
│  External File → [1] → [2] → [3] → [4] → [5] → Subsystems   │
│                                                               │
│  [1] QuarantineZone (Inspección segura)                      │
│  [2] HybridDigester (Source-specific processing)             │
│  [3] NutrientExtractor (7D parallel extraction)              │
│  [4] CoherenceValidator (Conflict detection)                 │
│  [5] NutrientDistributor (Routing to subsystems)             │
└──────────────────────────────────────────────────────────────┘
```

### Flujo End-to-End

```rust
// 1. User imports file
/import whatsapp chat_backup.txt

// 2. QuarantineZone inspection
QuarantineZone::inspect()
  ├─ Integrity check (SHA-256)
  ├─ Format detection (text, JSON, CSV)
  ├─ Threat scan (malware, exploits)
  ├─ PII detection (phone numbers, emails)
  └─ Quality assessment (0.0-1.0 score)

// 3. User approves
/quarantine approve abc123

// 4. HybridDigester processing
WhatsAppDigester::digest()
  ├─ Core: Parse message format (timestamp, sender, content)
  ├─ Template: Extract keywords (rust, AI, emojis)
  └─ Output: DigestedMessages

// 5. NutrientExtractor (parallel)
tokio::join!(
  extract_biographical(),
  extract_emotional(),
  extract_behavioral(),
  extract_relational(),
  extract_temporal(),
  extract_interests(),
  extract_hyperlinks(),  // ← NEW: Hyperlink Intelligence
)

// 6. CoherenceValidator
CoherenceValidator::validate()
  ├─ Detect conflicts (temporal, identity, interests)
  ├─ Cross-source validation (WhatsApp vs Telegram)
  └─ Interactive resolution (user picks correct version)

// 7. NutrientDistributor (parallel)
tokio::try_join!(
  distribute_to_telescopedb(),
  distribute_to_topicgraph(),
  distribute_to_emotionalspace(),
  distribute_to_memorybridge(),
  distribute_to_socialgraph(),
)

// 8. Success!
ImportResult {
  nutrients_extracted: 450,
  conflicts_resolved: 3,
  subsystems_updated: 5,
  duration: Duration::from_secs(28),  // <30s target
}
```

---

## 🛡️ COMPONENTE 1: QuarantineZone

### Propósito
**Primera línea de defensa.** Inspeccionar todo dato externo antes de procesarlo.

### Structs Principales

```rust
/// Estado de un item en cuarentena
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineZone {
    /// ID único (UUID v4)
    pub id: QuarantineId,
    
    /// Origen del dato
    pub source: DataSource,
    
    /// Datos crudos (sin procesar)
    pub raw_data: Vec<u8>,
    
    /// Metadata de inspección
    pub metadata: QuarantineMetadata,
    
    /// Estado actual
    pub state: QuarantineState,
    
    /// Timestamp de llegada
    pub arrived_at: DateTime<Utc>,
}

/// Metadata de inspección
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuarantineMetadata {
    /// Hash SHA-256 del archivo
    pub file_hash: [u8; 32],
    
    /// Tamaño en bytes
    pub size_bytes: usize,
    
    /// Formato detectado
    pub detected_format: FileFormat,
    
    /// PII encontrada
    pub pii_detected: Vec<PIIType>,
    
    /// Score de calidad (0.0 = corrupto, 1.0 = perfecto)
    pub quality_score: f32,
    
    /// Amenazas detectadas (si any)
    pub threats: Vec<ThreatType>,
}

/// Estados del item en cuarentena
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuarantineState {
    /// Pendiente de inspección
    Pending,
    
    /// Inspección en progreso
    Inspecting,
    
    /// Seguro para procesar
    Safe { confidence: f32 },
    
    /// Sospechoso (requiere revisión manual)
    Suspicious { reason: String },
    
    /// Rechazado (peligroso o corrupto)
    Rejected { reason: String },
}

/// Tipos de PII detectables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PIIType {
    PhoneNumber(String),
    EmailAddress(String),
    CreditCard(String),
    SocialSecurity(String),
    HomeAddress(String),
}

/// Tipos de amenazas detectables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatType {
    Malware { signature: String },
    Exploit { cve: String },
    SuspiciousScript,
    UnknownBinary,
}
```

### Métodos Principales

```rust
impl QuarantineZone {
    /// Inspeccionar archivo recién llegado
    pub async fn inspect(raw_data: Vec<u8>, source: DataSource) -> Result<Self> {
        // 1. Integrity check
        let file_hash = sha256(&raw_data);
        
        // 2. Format detection
        let detected_format = detect_format(&raw_data)?;
        
        // 3. Threat scan
        let threats = scan_threats(&raw_data)?;
        
        // 4. PII detection
        let pii_detected = scan_pii(&raw_data)?;
        
        // 5. Quality assessment
        let quality_score = assess_quality(&raw_data, &detected_format)?;
        
        // 6. Determine state
        let state = if !threats.is_empty() {
            QuarantineState::Rejected {
                reason: format!("Threats detected: {:?}", threats),
            }
        } else if quality_score < 0.3 {
            QuarantineState::Rejected {
                reason: "Low quality score (<0.3)".to_string(),
            }
        } else if quality_score < 0.7 || !pii_detected.is_empty() {
            QuarantineState::Suspicious {
                reason: "Medium quality or PII detected".to_string(),
            }
        } else {
            QuarantineState::Safe {
                confidence: quality_score,
            }
        };
        
        Ok(Self {
            id: QuarantineId::new(),
            source,
            raw_data,
            metadata: QuarantineMetadata {
                file_hash,
                size_bytes: raw_data.len(),
                detected_format,
                pii_detected,
                quality_score,
                threats,
            },
            state,
            arrived_at: Utc::now(),
        })
    }
    
    /// Usuario aprueba manualmente (override Suspicious)
    pub fn approve(&mut self) {
        self.state = QuarantineState::Safe { confidence: 0.8 };
    }
    
    /// Usuario rechaza manualmente
    pub fn reject(&mut self, reason: String) {
        self.state = QuarantineState::Rejected { reason };
    }
    
    /// Verificar si está listo para digest
    pub fn is_ready_for_digestion(&self) -> bool {
        matches!(self.state, QuarantineState::Safe { .. })
    }
}
```

### CLI Commands

```bash
# Importar archivo (automáticamente va a quarantine)
/import whatsapp chat_backup.txt

# Listar items en cuarentena
/quarantine list

# Inspeccionar item específico
/quarantine inspect abc123

# Aprobar para digestión
/quarantine approve abc123

# Rechazar
/quarantine reject abc123 "Reason: corrupted file"
```

### Performance Targets

- **Inspection latency:** <500ms per file
- **Throughput:** 20 files/second
- **PII detection accuracy:** >95%
- **Threat detection false positives:** <5%

---

## 🍽️ COMPONENTE 2: HybridDigester

### Propósito
**Digestión con respeto a la fuente.** Cada plataforma tiene su digester único.

### Arquitectura Híbrida

```
┌────────────────────────────────────────────┐
│         LAYER 1: HARD-CODED CORE           │
│  (Parsing, Validation, Distribution)       │
│  → Compiled, Fast, Stable                  │
├────────────────────────────────────────────┤
│      LAYER 2: TEMPLATE-BASED LOGIC         │
│  (Extraction Rules, Semantic Interpretation)│
│  → YAML, Flexible, Evolvable               │
├────────────────────────────────────────────┤
│       LAYER 3: HARD-CODED CORE             │
│  (Error Handling, Routing, Safety)         │
│  → Compiled, Safe, Predictable             │
└────────────────────────────────────────────┘
```

### Structs Principales

```rust
/// Trait para todos los digesters
#[async_trait]
pub trait DigestionPipeline: Send + Sync {
    /// Tipo de source que este digester procesa
    fn source_type(&self) -> DataSource;
    
    /// Digerir datos de quarantine
    async fn digest(&self, quarantined: &QuarantineZone) -> Result<DigestedData>;
    
    /// Validar formato antes de digerir
    fn validate_format(&self, raw_data: &[u8]) -> Result<()>;
}

/// Digester híbrido (core + templates)
pub struct HybridDigester {
    /// Core hard-coded (parsing, validation)
    core: CoreDigester,
    
    /// Template engine (extraction rules)
    template_engine: TemplateEngine,
    
    /// Template cargado
    template: DigestionTemplate,
}

/// Core digester (hard-coded, stable)
pub struct CoreDigester {
    /// Parser específico de plataforma
    parser: Box<dyn Parser>,
    
    /// Validator de formato
    validator: FormatValidator,
}

/// Template engine (flexible, evolvable)
pub struct TemplateEngine {
    /// Templates cargados
    templates: HashMap<DataSource, DigestionTemplate>,
    
    /// Watcher para hot-reload
    watcher: TemplateWatcher,
}
```

### Source-Specific Digesters

#### WhatsAppDigester

```rust
/// Digester para WhatsApp
pub struct WhatsAppDigester {
    hybrid: HybridDigester,
}

impl WhatsAppDigester {
    /// Características únicas de WhatsApp
    fn understand_whatsapp_nature(&self, msg: &Message) -> WhatsAppContext {
        WhatsAppContext {
            // Groups = relaciones cercanas
            is_group: msg.chat_id.starts_with("group_"),
            group_size: self.get_group_size(msg.chat_id),
            
            // Emojis = contexto emocional
            emoji_usage: self.extract_emojis(msg.content),
            
            // Multimedia = momentos significativos
            has_multimedia: msg.media.is_some(),
            
            // Frecuencia alta = intimidad
            message_frequency: self.calculate_frequency(msg.sender),
        }
    }
}

#[async_trait]
impl DigestionPipeline for WhatsAppDigester {
    fn source_type(&self) -> DataSource {
        DataSource::WhatsApp
    }
    
    async fn digest(&self, quarantined: &QuarantineZone) -> Result<DigestedData> {
        // 1. Core: Parse WhatsApp format
        let messages = self.hybrid.core.parser.parse(&quarantined.raw_data)?;
        
        // 2. Template: Extract keywords, interests
        let template_data = self.hybrid.template_engine.execute(&messages)?;
        
        // 3. WhatsApp-specific understanding
        let contexts: Vec<WhatsAppContext> = messages
            .iter()
            .map(|msg| self.understand_whatsapp_nature(msg))
            .collect();
        
        Ok(DigestedData {
            messages,
            template_data,
            platform_context: PlatformContext::WhatsApp(contexts),
        })
    }
}
```

#### EmailDigester

```rust
/// Digester para Email
pub struct EmailDigester {
    hybrid: HybridDigester,
}

impl EmailDigester {
    /// Características únicas de Email
    fn understand_email_nature(&self, email: &Email) -> EmailContext {
        EmailContext {
            // Subject = tema key
            subject_topics: self.extract_topics(email.subject),
            
            // CC/BCC = mapa de poder
            collaboration_network: self.map_recipients(email),
            
            // Attachments = documentos profesionales
            professional_docs: email.attachments.iter()
                .filter(|att| att.is_professional())
                .cloned()
                .collect(),
            
            // Timing = hábitos laborales
            sent_during_work_hours: self.is_work_hours(email.timestamp),
        }
    }
}
```

#### SpotifyDigester

```rust
/// Digester para Spotify
pub struct SpotifyDigester {
    hybrid: HybridDigester,
}

impl SpotifyDigester {
    /// Características únicas de Spotify
    fn understand_spotify_nature(&self, track: &Track) -> SpotifyContext {
        SpotifyContext {
            // Genre = mood state
            emotional_category: self.genre_to_emotion(track.genre),
            
            // Playlist order = journey emocional
            playlist_position: track.playlist_index,
            
            // Time of day = rutina
            listening_time: track.played_at,
            
            // Repetición = significancia
            play_count: self.get_play_count(track.id),
        }
    }
}
```

### Template Integration

```rust
impl HybridDigester {
    /// Ejecutar template sobre datos parseados
    pub fn execute_template(&self, data: &ParsedData) -> Result<TemplateData> {
        let template = &self.template;
        
        // Extract según reglas del template
        let interests = template.extraction.interests
            .extract_from(data)?;
        
        let relationships = template.extraction.relationships
            .extract_from(data)?;
        
        let hyperlinks = if template.hyperlinks.enabled {
            template.hyperlinks.extract_from(data)?
        } else {
            Vec::new()
        };
        
        Ok(TemplateData {
            interests,
            relationships,
            hyperlinks,
        })
    }
    
    /// Hot-reload template (sin reiniciar)
    pub fn reload_template(&mut self, source: DataSource) -> Result<()> {
        let new_template = self.template_engine.load_template(source)?;
        self.template = new_template;
        Ok(())
    }
}
```

### Factory Pattern

```rust
/// Factory para crear digester correcto según source
pub struct DigesterFactory;

impl DigesterFactory {
    /// Crear digester apropiado
    pub fn create(source: DataSource) -> Result<Box<dyn DigestionPipeline>> {
        match source {
            DataSource::WhatsApp => Ok(Box::new(WhatsAppDigester::new()?)),
            DataSource::Telegram => Ok(Box::new(TelegramDigester::new()?)),
            DataSource::Email => Ok(Box::new(EmailDigester::new()?)),
            DataSource::Calendar => Ok(Box::new(CalendarDigester::new()?)),
            DataSource::Spotify => Ok(Box::new(SpotifyDigester::new()?)),
            DataSource::GitHub => Ok(Box::new(GitHubDigester::new()?)),
            DataSource::Twitter => Ok(Box::new(TwitterDigester::new()?)),
        }
    }
}
```

### Performance Targets

- **Digestion latency:** <30s for 1000 messages
- **Template execution:** <100ms per message
- **Hot-reload:** <200ms
- **Memory peak:** <500MB

---

## 🧬 COMPONENTE 3: NutrientExtractor

### Propósito
**Extracción multi-dimensional paralela.** 7 extractores corriendo simultáneamente.

### Structs Principales

```rust
/// Orchestrator de extracción
pub struct NutrientExtractor {
    /// Extractores especializados
    biographical: BiographicalExtractor,
    emotional: EmotionalExtractor,
    behavioral: BehavioralExtractor,
    relational: RelationalExtractor,
    temporal: TemporalExtractor,
    interests: InterestExtractor,
    hyperlinks: HyperlinkExtractor,  // ← NEW
}

/// Resultado de extracción completa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedNutrients {
    /// Dimensión 1: Biographical
    pub identity: IdentityNutrients,
    
    /// Dimensión 2: Emotional
    pub emotional: EmotionalNutrients,
    
    /// Dimensión 3: Behavioral
    pub behavioral: BehavioralNutrients,
    
    /// Dimensión 4: Relational
    pub relational: RelationalNutrients,
    
    /// Dimensión 5: Temporal
    pub temporal: TemporalNutrients,
    
    /// Dimensión 6: Interests
    pub interests: Vec<InterestNutrient>,
    
    /// Dimensión 7: Hyperlink Intelligence (NEW)
    pub hyperlinks: HyperlinkIntelligence,
}
```

### Extraction Paralela

```rust
impl NutrientExtractor {
    /// Extraer todos los nutrientes en paralelo
    pub async fn extract_all(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        // Parallel extraction con tokio::join!
        let (
            identity,
            emotional,
            behavioral,
            relational,
            temporal,
            interests,
            hyperlinks,
        ) = tokio::join!(
            self.biographical.extract(digested),
            self.emotional.extract(digested),
            self.behavioral.extract(digested),
            self.relational.extract(digested),
            self.temporal.extract(digested),
            self.interests.extract(digested),
            self.hyperlinks.extract(digested),  // ← NEW
        );
        
        Ok(ExtractedNutrients {
            identity: identity?,
            emotional: emotional?,
            behavioral: behavioral?,
            relational: relational?,
            temporal: temporal?,
            interests: interests?,
            hyperlinks: hyperlinks?,
        })
    }
}
```

### Extractores Individuales

#### BiographicalExtractor

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityNutrients {
    /// Nombre completo
    pub full_name: Option<String>,
    pub confidence_name: f32,
    
    /// Edad
    pub age: Option<u8>,
    pub confidence_age: f32,
    
    /// Ubicación
    pub location: Option<String>,
    pub confidence_location: f32,
    
    /// Ocupación
    pub occupation: Option<String>,
    pub confidence_occupation: f32,
    
    /// Evidencias
    pub evidence: Vec<Evidence>,
}

impl BiographicalExtractor {
    pub async fn extract(&self, digested: &DigestedData) -> Result<IdentityNutrients> {
        // Buscar patrones de nombre
        let full_name = self.extract_name(digested)?;
        
        // Inferir edad de menciones
        let age = self.infer_age(digested)?;
        
        // Detectar ubicación (ciudad, país)
        let location = self.detect_location(digested)?;
        
        // Inferir ocupación de keywords
        let occupation = self.infer_occupation(digested)?;
        
        Ok(IdentityNutrients {
            full_name,
            age,
            location,
            occupation,
            confidence_name: 0.85,
            confidence_age: 0.60,
            confidence_location: 0.75,
            confidence_occupation: 0.80,
            evidence: vec![/* ... */],
        })
    }
}
```

#### EmotionalExtractor

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalNutrients {
    /// Baseline VADC (Valence, Arousal, Dominance, Certainty)
    pub baseline: EmotionalProfile,
    
    /// Patrones emocionales detectados
    pub patterns: Vec<EmotionalPattern>,
    
    /// Triggers detectados
    pub triggers: Vec<EmotionalTrigger>,
}

impl EmotionalExtractor {
    pub async fn extract(&self, digested: &DigestedData) -> Result<EmotionalNutrients> {
        // Calcular baseline VADC
        let baseline = self.calculate_baseline_vadc(digested)?;
        
        // Detectar patrones (e.g., frustración cuando debuggea)
        let patterns = self.detect_patterns(digested)?;
        
        // Detectar triggers (e.g., "ownership" → frustration)
        let triggers = self.detect_triggers(digested)?;
        
        Ok(EmotionalNutrients {
            baseline,
            patterns,
            triggers,
        })
    }
}
```

#### HyperlinkExtractor (NEW)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperlinkIntelligence {
    /// URLs extraídas y clasificadas
    pub links: Vec<ClassifiedLink>,
    
    /// Perfil de consumo
    pub consumption_profile: ConsumptionProfile,
    
    /// Comportamiento de sharing
    pub sharing_behavior: SharingBehavior,
    
    /// Efficiency score (0.0 = distracción, 1.0 = deep work)
    pub efficiency_score: f32,
    
    /// Rol social inferido
    pub social_role: SocialRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedLink {
    /// URL original
    pub url: String,
    
    /// URL expandida (si era short link)
    pub expanded_url: Option<String>,
    
    /// Plataforma (YouTube, Spotify, GitHub, etc.)
    pub platform: Platform,
    
    /// Metadata fetched
    pub metadata: LinkMetadata,
    
    /// Intent inferido
    pub intent: LinkIntent,
    
    /// Categoría de contenido
    pub content_category: ContentCategory,
}

impl HyperlinkExtractor {
    pub async fn extract(&self, digested: &DigestedData) -> Result<HyperlinkIntelligence> {
        // 1. Extract URLs
        let raw_urls = self.extract_urls(digested)?;
        
        // 2. Expand short links
        let expanded = self.expand_short_links(raw_urls).await?;
        
        // 3. Classify platforms
        let classified = self.classify_platforms(expanded)?;
        
        // 4. Fetch metadata (parallel)
        let links = self.fetch_metadata_parallel(classified).await?;
        
        // 5. Infer intent
        let links = self.infer_intent(links, digested)?;
        
        // 6. Analyze consumption profile
        let consumption_profile = self.analyze_consumption(&links)?;
        
        // 7. Analyze sharing behavior
        let sharing_behavior = self.analyze_sharing(&links)?;
        
        // 8. Calculate efficiency score
        let efficiency_score = self.calculate_efficiency(&links)?;
        
        // 9. Infer social role
        let social_role = self.infer_social_role(&links, &sharing_behavior)?;
        
        Ok(HyperlinkIntelligence {
            links,
            consumption_profile,
            sharing_behavior,
            efficiency_score,
            social_role,
        })
    }
}
```

### Performance Targets

- **Extraction latency:** <10s for 1000 messages (parallel)
- **Accuracy:** >90% per nutrient
- **Memory peak:** <300MB
- **Throughput:** 100 messages/second

---

## ✅ COMPONENTE 4: CoherenceValidator

### Propósito
**Detectar conflictos y garantizar coherencia.** Cross-validation entre fuentes.

### Structs Principales

```rust
/// Validator de coherencia
pub struct CoherenceValidator {
    /// Detector de conflictos
    conflict_detector: ConflictDetector,
    
    /// Checker de consistencia
    consistency_checker: ConsistencyChecker,
    
    /// Estimador de verdad
    truth_estimator: TruthEstimator,
}

/// Reporte de validación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// ¿Es coherente?
    pub is_coherent: bool,
    
    /// Confidence global (0.0-1.0)
    pub overall_confidence: f32,
    
    /// Conflictos detectados
    pub conflicts: Vec<Conflict>,
    
    /// Resoluciones aplicadas
    pub resolutions: Vec<Resolution>,
}

/// Tipos de conflictos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Conflict {
    /// Temporal: eventos en orden imposible
    Temporal {
        event_a: String,
        event_b: String,
        reason: String,
    },
    
    /// Identity: mismo usuario, nombres diferentes
    Identity {
        name_a: String,
        name_b: String,
        source_a: DataSource,
        source_b: DataSource,
    },
    
    /// Interest: contradicción ("odio X" pero menciona X positivamente)
    Interest {
        negative_mention: String,
        positive_mention: String,
        topic: String,
    },
}
```

### Métodos Principales

```rust
impl CoherenceValidator {
    /// Validar nutrientes extraídos
    pub async fn validate(&self, nutrients: &ExtractedNutrients) -> Result<ValidationReport> {
        // 1. Detect conflicts
        let temporal_conflicts = self.conflict_detector.detect_temporal(nutrients)?;
        let identity_conflicts = self.conflict_detector.detect_identity(nutrients)?;
        let interest_conflicts = self.conflict_detector.detect_interest(nutrients)?;
        
        let conflicts = [
            temporal_conflicts,
            identity_conflicts,
            interest_conflicts,
        ].concat();
        
        // 2. Check consistency (cross-source)
        let consistency_score = self.consistency_checker.check(nutrients)?;
        
        // 3. Estimate truth
        let confidence_scores = self.truth_estimator.estimate(nutrients)?;
        
        // 4. Interactive resolution (si hay conflictos)
        let resolutions = if !conflicts.is_empty() {
            self.resolve_interactively(&conflicts).await?
        } else {
            Vec::new()
        };
        
        Ok(ValidationReport {
            is_coherent: conflicts.is_empty(),
            overall_confidence: consistency_score,
            conflicts,
            resolutions,
        })
    }
    
    /// Resolución interactiva (CLI)
    async fn resolve_interactively(&self, conflicts: &[Conflict]) -> Result<Vec<Resolution>> {
        let mut resolutions = Vec::new();
        
        for conflict in conflicts {
            // Present conflict to user
            println!("⚠️  Conflict detected:");
            println!("{:?}", conflict);
            println!("Options:");
            println!("1. Accept version A");
            println!("2. Accept version B");
            println!("3. Accept both (context-dependent)");
            
            // User chooses resolution
            let choice = read_user_input()?;
            
            let resolution = match choice {
                1 => Resolution::AcceptA,
                2 => Resolution::AcceptB,
                3 => Resolution::AcceptBoth,
                _ => Resolution::Skip,
            };
            
            resolutions.push(resolution);
        }
        
        Ok(resolutions)
    }
}
```

### Performance Targets

- **Validation latency:** <2s for 500 nutrients
- **Conflict detection accuracy:** >95%
- **False positives:** <5%

---

## 📡 COMPONENTE 5: NutrientDistributor

### Propósito
**Rutear nutrientes a subsistemas correctos.** Parallel distribution.

### Structs Principales

```rust
/// Distributor de nutrientes
pub struct NutrientDistributor {
    /// Conexiones a subsistemas
    telescope_db: Arc<TelescopeDB>,
    topic_graph: Arc<TopicGraph>,
    emotional_space: Arc<EmotionalSpace>,
    memory_bridge: Arc<MemoryBridge>,
    social_graph: Arc<SocialGraph>,
    icebreaker: Arc<IceBreaker>,
}

/// Reporte de distribución
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionReport {
    /// Nutrients distribuidos por subsistema
    pub telescopedb: usize,
    pub topicgraph: usize,
    pub emotionalspace: usize,
    pub memorybridge: usize,
    pub socialgraph: usize,
    pub icebreaker: usize,
    
    /// Duración total
    pub duration: Duration,
    
    /// Errores parciales
    pub partial_failures: Vec<String>,
}
```

### Métodos de Distribución

```rust
impl NutrientDistributor {
    /// Distribuir todos los nutrientes en paralelo
    pub async fn distribute_all(&self, nutrients: &ExtractedNutrients) -> Result<DistributionReport> {
        let start = Instant::now();
        
        // Parallel distribution con tokio::try_join!
        let (
            telescopedb_count,
            topicgraph_count,
            emotionalspace_count,
            memorybridge_count,
            socialgraph_count,
            icebreaker_count,
        ) = tokio::try_join!(
            self.distribute_identity(nutrients),
            self.distribute_interests(nutrients),
            self.distribute_emotional(nutrients),
            self.distribute_behavioral(nutrients),
            self.distribute_relational(nutrients),
            self.distribute_hyperlinks(nutrients),
        )?;
        
        Ok(DistributionReport {
            telescopedb: telescopedb_count,
            topicgraph: topicgraph_count,
            emotionalspace: emotionalspace_count,
            memorybridge: memorybridge_count,
            socialgraph: socialgraph_count,
            icebreaker: icebreaker_count,
            duration: start.elapsed(),
            partial_failures: Vec::new(),
        })
    }
    
    /// Distribuir identity → TelescopeDB + IceBreaker
    async fn distribute_identity(&self, nutrients: &ExtractedNutrients) -> Result<usize> {
        let identity = &nutrients.identity;
        
        // TelescopeDB: biographical entries
        self.telescope_db.insert_biographical(identity).await?;
        
        // IceBreaker: boost confidence
        self.icebreaker.update_identity(identity).await?;
        
        Ok(2)
    }
    
    /// Distribuir interests → TopicGraph + IceBreaker
    async fn distribute_interests(&self, nutrients: &ExtractedNutrients) -> Result<usize> {
        let interests = &nutrients.interests;
        
        // TopicGraph: add/update topics + edges
        for interest in interests {
            self.topic_graph.add_interest(interest).await?;
        }
        
        // IceBreaker: personalization
        self.icebreaker.update_interests(interests).await?;
        
        Ok(interests.len())
    }
    
    /// Distribuir hyperlinks → TopicGraph + BiographicalProfile
    async fn distribute_hyperlinks(&self, nutrients: &ExtractedNutrients) -> Result<usize> {
        let hyperlinks = &nutrients.hyperlinks;
        
        // TopicGraph: interests from links
        for link in &hyperlinks.links {
            self.topic_graph.add_link_interest(link).await?;
        }
        
        // BiographicalProfile: consumption patterns, efficiency score
        self.telescope_db.update_consumption_profile(&hyperlinks.consumption_profile).await?;
        self.telescope_db.update_efficiency_score(hyperlinks.efficiency_score).await?;
        
        Ok(hyperlinks.links.len())
    }
}
```

### Performance Targets

- **Distribution latency:** <3s for 500 nutrients (parallel)
- **Throughput:** 200 nutrients/second
- **Error handling:** Partial failures don't block others

---

## 🎨 TEMPLATE SYSTEM

### YAML Structure

```yaml
# templates/digesters/whatsapp_v1.yaml
version: "1.0"
source: "WhatsApp"
author: "Eduardo Gil"
date: "2025-11-29"

extends: "base_chat.yaml"  # Inheritance

extraction:
  identity:
    patterns:
      - field: "sender"
        regex: "^[A-Z][a-z]+ [A-Z][a-z]+$"
        confidence: 0.9
  
  interests:
    keywords:
      technology:
        rust: ["rust", "🦀", "cargo", "tokio", "async"]
        ai: ["AI", "LLM", "GPT", "Claude"]
      music:
        genres: ["canción", "🎵", "playlist", "spotify"]
    
    context_boost:
      - condition: "interest + emoji in same message"
        boost: 0.2
      - condition: "repeated mention (>3 times)"
        boost: 0.3
  
  relationships:
    strength_indicators:
      very_strong:
        - daily_messages: "> 10"
        - emoji_usage: "> 5"
        - multimedia_shared: "> 2"
      strong:
        - daily_messages: "> 5"
        - emoji_usage: "> 2"
      medium:
        - daily_messages: "> 1"

hyperlinks:
  enabled: true
  expand_short_urls: true
  fetch_metadata: true
  cache_duration: 86400  # 24 hours
  
  platforms_priority:
    - YouTube
    - Spotify
    - GitHub
    - Twitter
  
  intent_inference:
    self_reference:
      patterns: ["shared to myself", "para mi", "revisar luego", "save"]
      confidence: 0.95
    recommendation:
      patterns: ["deberías", "te recomiendo", "check this out", "mira esto"]
      confidence: 0.85
    question:
      patterns: ["qué opinas", "has visto", "what do you think"]
      confidence: 0.80

distribution:
  biographical: "TelescopeDB"
  interests: "TopicGraph"
  emotional: "EmotionalSpace"
  relationships: "SocialGraph"
  hyperlinks: ["TopicGraph", "BiographicalProfile"]
```

### Template Inheritance

```yaml
# templates/digesters/base_chat.yaml (base)
version: "1.0"
source: "BaseChat"

extraction:
  identity:
    patterns:
      - field: "sender"
        regex: "^[A-Z][a-z]+.*$"
        confidence: 0.7

# templates/digesters/telegram_v1.yaml (inherits)
extends: "base_chat.yaml"

extraction:
  identity:
    patterns:  # Override: Telegram allows @usernames
      - field: "sender"
        regex: "^@[a-zA-Z0-9_]+$"
        confidence: 0.9
```

### A/B Testing

```yaml
# Experiment config
experiments:
  whatsapp_emoji_boost:
    control: "whatsapp_v1.yaml"
    variant: "whatsapp_v2_emoji_boost.yaml"
    sample_size: 100
    metrics:
      - accuracy_delta
      - confidence_delta
```

### Performance Targets

- **Template loading:** <50ms
- **Template execution:** <100ms per message
- **Hot-reload:** <200ms
- **A/B test analysis:** <5s for 100 samples

---

## 🔗 HYPERLINK INTELLIGENCE

### URL Extraction

```rust
impl HyperlinkExtractor {
    /// Extract all URLs from messages
    fn extract_urls(&self, digested: &DigestedData) -> Result<Vec<String>> {
        let url_regex = Regex::new(
            r"https?://[^\s]+"
        )?;
        
        let mut urls = Vec::new();
        for message in &digested.messages {
            for capture in url_regex.captures_iter(&message.content) {
                urls.push(capture[0].to_string());
            }
        }
        
        Ok(urls)
    }
}
```

### URL Expansion

```rust
impl HyperlinkExtractor {
    /// Expand short URLs (bit.ly, t.co, etc.)
    async fn expand_short_links(&self, urls: Vec<String>) -> Result<Vec<String>> {
        let mut expanded = Vec::new();
        
        for url in urls {
            if self.is_short_url(&url) {
                let full_url = self.follow_redirects(&url, 5).await?;
                expanded.push(full_url);
            } else {
                expanded.push(url);
            }
        }
        
        Ok(expanded)
    }
    
    /// Follow redirects (max 5 hops)
    async fn follow_redirects(&self, url: &str, max_hops: usize) -> Result<String> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(max_hops))
            .build()?;
        
        let response = client.get(url).send().await?;
        Ok(response.url().to_string())
    }
}
```

### Platform Classification

```rust
impl HyperlinkExtractor {
    /// Classify platform from URL
    fn classify_platform(&self, url: &str) -> Platform {
        if url.contains("youtube.com") || url.contains("youtu.be") {
            Platform::YouTube
        } else if url.contains("spotify.com") {
            Platform::Spotify
        } else if url.contains("github.com") {
            Platform::GitHub
        } else if url.contains("twitter.com") || url.contains("x.com") {
            Platform::Twitter
        } else {
            Platform::Generic
        }
    }
}
```

### Metadata Fetching

```rust
impl HyperlinkExtractor {
    /// Fetch metadata (Open Graph)
    async fn fetch_metadata(&self, url: &str) -> Result<LinkMetadata> {
        // Check cache first
        if let Some(cached) = self.cache.get(url) {
            return Ok(cached);
        }
        
        // Fetch HTML
        let html = reqwest::get(url).await?.text().await?;
        let document = scraper::Html::parse_document(&html);
        
        // Parse Open Graph tags
        let title = self.extract_og_tag(&document, "og:title")?;
        let description = self.extract_og_tag(&document, "og:description")?;
        let image = self.extract_og_tag(&document, "og:image");
        
        let metadata = LinkMetadata {
            title,
            description,
            image,
            fetched_at: Utc::now(),
        };
        
        // Cache for 24h
        self.cache.set(url, metadata.clone(), Duration::from_secs(86400));
        
        Ok(metadata)
    }
}
```

### Intent Inference

```rust
impl HyperlinkExtractor {
    /// Infer intent from context
    fn infer_intent(&self, link: &ClassifiedLink, digested: &DigestedData) -> LinkIntent {
        let message = self.find_message_with_link(&link.url, digested);
        
        // Self-reference patterns
        if message.is_self_share() {
            return LinkIntent::SelfReference { confidence: 0.95 };
        }
        
        // Recommendation patterns
        if message.content.contains_any(&["deberías", "te recomiendo", "check this"]) {
            return LinkIntent::Recommendation { confidence: 0.85 };
        }
        
        // Question patterns
        if message.content.contains_any(&["qué opinas", "has visto"]) {
            return LinkIntent::Question { confidence: 0.80 };
        }
        
        LinkIntent::Unknown
    }
}
```

### Efficiency Scoring

```rust
impl HyperlinkExtractor {
    /// Calculate efficiency score (0.0 = distraction, 1.0 = deep work)
    fn calculate_efficiency(&self, links: &[ClassifiedLink]) -> f32 {
        let mut deep_work_count = 0;
        let mut entertainment_count = 0;
        
        for link in links {
            match link.content_category {
                ContentCategory::Educational | ContentCategory::Technical => {
                    deep_work_count += 1;
                }
                ContentCategory::Entertainment | ContentCategory::Social => {
                    entertainment_count += 1;
                }
                _ => {}
            }
        }
        
        let total = deep_work_count + entertainment_count;
        if total == 0 {
            return 0.5;  // Neutral
        }
        
        deep_work_count as f32 / total as f32
    }
}
```

### Performance Targets

- **URL extraction:** <100ms for 1000 messages
- **URL expansion:** <500ms per short link
- **Metadata fetching:** <1s per URL (with cache)
- **Intent inference:** <50ms per link
- **Efficiency scoring:** <200ms for 100 links

---

## 🔌 API PRINCIPAL

### High-Level API

```rust
/// Main entry point
pub struct DataImportEngine {
    quarantine: QuarantineManager,
    digesters: DigesterFactory,
    extractor: NutrientExtractor,
    validator: CoherenceValidator,
    distributor: NutrientDistributor,
}

impl DataImportEngine {
    /// Import and process external data
    pub async fn import(&self, source: DataSource, file_path: &str) -> Result<ImportResult> {
        // 1. Read file
        let raw_data = tokio::fs::read(file_path).await?;
        
        // 2. Quarantine
        let quarantined = QuarantineZone::inspect(raw_data, source).await?;
        
        // Wait for approval if suspicious
        if !quarantined.is_ready_for_digestion() {
            return Ok(ImportResult::PendingApproval(quarantined.id));
        }
        
        // 3. Digest
        let digester = self.digesters.create(source)?;
        let digested = digester.digest(&quarantined).await?;
        
        // 4. Extract
        let nutrients = self.extractor.extract_all(&digested).await?;
        
        // 5. Validate
        let validation = self.validator.validate(&nutrients).await?;
        
        if !validation.is_coherent {
            // Handle conflicts interactively
            println!("⚠️  Conflicts detected, resolving...");
        }
        
        // 6. Distribute
        let distribution = self.distributor.distribute_all(&nutrients).await?;
        
        Ok(ImportResult::Success {
            nutrients_extracted: distribution.telescopedb + distribution.topicgraph + /* ... */,
            duration: distribution.duration,
        })
    }
}
```

### CLI Commands

```bash
# Import commands
/import whatsapp chat_backup.txt
/import telegram messages.json
/import email mbox_export.mbox
/import spotify listening_history.json
/import github starred_repos.json
/import auto ~/Downloads  # Auto-detect format

# Quarantine management
/quarantine list
/quarantine inspect abc123
/quarantine approve abc123
/quarantine reject abc123 "Reason: corrupted"

# Template management
/reload templates
/test template whatsapp_v2.yaml sample_chat.txt
/compare whatsapp_v1.yaml whatsapp_v2.yaml sample_chat.txt

# Insights
/insights links  # Hyperlink intelligence
/insights consumption  # Consumption profile
/insights efficiency  # Efficiency score
/memory  # Recall imported data
```

---

## ⚡ PERFORMANCE TARGETS

### Phase-by-Phase Targets

| Phase | Target Latency | Throughput | Memory Peak |
|-------|----------------|------------|-------------|
| **1. Quarantine** | <500ms | 20 files/s | <100MB |
| **2. Digestion** | <30s (1000 msgs) | 33 msgs/s | <500MB |
| **3. Extraction** | <10s (1000 msgs) | 100 msgs/s | <300MB |
| **4. Validation** | <2s (500 nutrients) | 250 nutrients/s | <200MB |
| **5. Distribution** | <3s (500 nutrients) | 166 nutrients/s | <200MB |
| **Total Pipeline** | <45s (1000 msgs) | 22 msgs/s | <500MB peak |

### End-to-End Targets

- **Onboarding time:** <30s (vs 30min manual = 60x improvement)
- **Extraction accuracy:** >90% per nutrient dimension
- **User satisfaction:** >8.0/10
- **Zero data loss:** 100% preservation
- **Platform support:** 7+ sources (WhatsApp, Telegram, Email, Calendar, Spotify, GitHub, Twitter)

### Scalability Targets

- **1,000 messages:** <30s
- **10,000 messages:** <3min
- **100,000 messages:** <30min

---

**Fecha:** 2025-11-29  
**Versión:** 1.0  
**Estado:** ✅ Especificación completa, ready for implementation  
**Próximo paso:** Implementar 7.x.1.1 (QuarantineZone struct)  
**Related Docs:**
- [09_metabolic-digestion-vision.md](../00_VISION/09_metabolic-digestion-vision.md) — Filosofía
- [18_metabolic-digestion-system.md](../01_ARQUITECTURA/18_metabolic-digestion-system.md) — Arquitectura
- [PHASE_7X_DATA_IMPORT.md](../04_IMPLEMENTACION/PHASE_7X_DATA_IMPORT.md) — Plan 6 semanas
