```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/18_hyperlink-extractor.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 20:30:00
Autor: Sistema Bitácora + Eduardo
Propósito: Especificación completa de HyperlinkExtractor - Hyperlink Intelligence System
Estado: 🎯 ACTIVO - Especificación de implementación
Relacionado Con:
  - src/data_import/extraction.rs (implementación)
  - ROADMAP_V2/CHECKLIST_V2.md Task 7.x.3.8 (tracking)
  - ROADMAP_V2/02_COMPONENTES/17_data-import-engine.md (contexto general)
  - ROADMAP_V2/02_COMPONENTES/18.1_hyperlink-content-extraction.md (📚 content extraction técnico)
  - ROADMAP_V2/METOD_DOCS.md (metodología seguida)
Categoría: COMPONENT
Prioridad: ALTA - Último extractor pendiente (8/8)
# === FIN DATOS DE AUDITORÍA ===
```

# 🔗 HyperlinkExtractor - Hyperlink Intelligence System

> **Component:** HyperlinkExtractor (Task 7.x.3.8 - ÚLTIMO EXTRACTOR)  
> **Purpose:** Analizar URLs compartidas para inferir intereses, comportamiento de consumo y rol social

---

## 📋 TABLA DE CONTENIDOS

1. [Component Overview](#component-overview)
2. [Real Data Context](#real-data-context)
3. [Architecture & Design](#architecture--design)
4. [Platform Classification](#platform-classification)
5. [Implementation Plan](#implementation-plan)
6. [Testing Strategy](#testing-strategy)
7. [Performance Targets](#performance-targets)
8. [Future Enhancements](#future-enhancements)

---

## 🎯 COMPONENT OVERVIEW

### Propósito Principal

**Extraer inteligencia de URLs compartidas** para entender:
1. **Intereses consumidos** (qué tipo de contenido consume)
2. **Comportamiento de sharing** (comparte recursos, recomienda, auto-promociona)
3. **Eficiencia cognitiva** (deep work vs distracción)
4. **Rol social** (curator, consumer, creator, connector)

### Responsabilidades

```yaml
Primary:
  - Extraer URLs de mensajes (regex + context)
  - Clasificar plataforma (YouTube, GitHub, Spotify, etc.)
  - Expandir short URLs (bit.ly → URL real)
  - Inferir intent (self-reference, recommendation, collaboration)
  
Secondary:
  - Calcular efficiency score (productivo vs entretenimiento)
  - Identificar patrones de consumo (frecuencia, categorías)
  - Inferir rol social (curator, creator, consumer)
```

### Position in Architecture

```
DigestedData
    │
    ├─> BiographicalExtractor   ─┐
    ├─> InterestExtractor        │
    ├─> EmotionalExtractor       │
    ├─> TemporalExtractor        ├─> HyperlinkExtractor
    ├─> BehavioralExtractor      │   (integra todo)
    └─> RelationshipExtractor   ─┘
                                  │
                            ExtractedNutrients
                            (Hyperlinks Dimension)
```

---

## 📊 REAL DATA CONTEXT

### Paula Roque Chat Analysis

**Source:** `data/imports/whatsapp/_chat.txt`  
**Messages:** 1,354 entries  
**URLs Detected:** 298 (22% of messages)  
**Date Range:** June-November 2025 (178 days)

### URL Distribution (Expected)

```yaml
Plataformas Esperadas:
  YouTube: ~40% (videos, música)
  Spotify: ~25% (música, podcasts)
  WhatsApp Media: ~15% (wa.me/*, _chat attachments)
  Social Media: ~10% (Instagram, Facebook)
  Others: ~10% (GitHub, Google Drive, etc.)

Categorías de Contenido:
  Entertainment: ~50% (música, videos)
  Social: ~25% (mensajes, grupos)
  Education: ~15% (tutoriales, cursos)
  Professional: ~10% (GitHub, docs)
```

### Key Insights from Previous Analysis

From `test_digestion_deep_analysis.rs` (Task 7.x.3.11):
- **Hyperlinks readiness: 95%** (EXCELLENT - Ready for implementation)
- 298 URLs detected → High volume for pattern analysis
- Rich context available for intent inference
- Most messages with URLs include surrounding text (context)

---

## 🏗️ ARCHITECTURE & DESIGN

### Core Structs

#### 1. HyperlinkExtractor

```rust
/// Extractor principal de hyperlinks
pub struct HyperlinkExtractor;

#[async_trait]
impl NutrientExtractor for HyperlinkExtractor {
    fn dimension(&self) -> NutrientDimension {
        NutrientDimension::Hyperlinks
    }

    fn can_process(&self, entry: &DigestedEntry) -> bool {
        self.contains_url(&entry.content)
    }

    async fn extract(&self, digested: &DigestedData) -> Result<ExtractedNutrients> {
        // 1. Extract URLs
        // 2. Classify platforms
        // 3. Extract context
        // 4. Infer intent
        // 5. Calculate patterns
    }
}
```

#### 2. ClassifiedLink

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedLink {
    /// URL original
    pub url: String,
    
    /// Plataforma detectada
    pub platform: Platform,
    
    /// Categoría de contenido
    pub category: ContentCategory,
    
    /// Intent inferido
    pub intent: LinkIntent,
    
    /// Contexto del mensaje
    pub context: String,
    
    /// Confidence (0.0-1.0)
    pub confidence: f64,
}
```

#### 3. Platform Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    // Media
    YouTube,
    Spotify,
    SoundCloud,
    Vimeo,
    
    // Social
    Instagram,
    Facebook,
    Twitter,
    LinkedIn,
    TikTok,
    WhatsApp,  // wa.me links
    
    // Professional
    GitHub,
    GitLab,
    StackOverflow,
    
    // Productivity
    GoogleDrive,
    Dropbox,
    Notion,
    
    // Short URLs
    Bitly,
    TinyUrl,
    
    // Unknown
    Other(String),
}
```

#### 4. ContentCategory

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentCategory {
    Entertainment,  // Videos, música
    Education,      // Tutoriales, cursos
    News,           // Noticias, artículos
    Social,         // Mensajes, grupos
    Professional,   // GitHub repos, docs
    Shopping,       // E-commerce
    Unknown,
}
```

#### 5. LinkIntent

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkIntent {
    /// Auto-referencia ("mira mi repo", "mi video")
    SelfReference { confidence: f64 },
    
    /// Recomendación ("te recomiendo", "checa esto")
    Recommendation { confidence: f64 },
    
    /// Colaboración ("vamos a trabajar en", "ayúdame con")
    Collaboration { confidence: f64 },
    
    /// Compartir contenido sin contexto claro
    Sharing { confidence: f64 },
    
    /// No se pudo inferir
    Unknown,
}
```

---

## 🎨 PLATFORM CLASSIFICATION

### Detection Patterns

```rust
impl HyperlinkExtractor {
    /// Detecta la plataforma de una URL
    fn classify_platform(&self, url: &str) -> Platform {
        let url_lower = url.to_lowercase();
        
        // Media platforms
        if url_lower.contains("youtube.com") || url_lower.contains("youtu.be") {
            return Platform::YouTube;
        }
        if url_lower.contains("spotify.com") {
            return Platform::Spotify;
        }
        if url_lower.contains("soundcloud.com") {
            return Platform::SoundCloud;
        }
        
        // Social platforms
        if url_lower.contains("instagram.com") {
            return Platform::Instagram;
        }
        if url_lower.contains("facebook.com") || url_lower.contains("fb.com") {
            return Platform::Facebook;
        }
        if url_lower.contains("twitter.com") || url_lower.contains("x.com") {
            return Platform::Twitter;
        }
        if url_lower.contains("wa.me") || url_lower.contains("whatsapp.com") {
            return Platform::WhatsApp;
        }
        
        // Professional platforms
        if url_lower.contains("github.com") {
            return Platform::GitHub;
        }
        if url_lower.contains("gitlab.com") {
            return Platform::GitLab;
        }
        if url_lower.contains("stackoverflow.com") {
            return Platform::StackOverflow;
        }
        
        // Productivity
        if url_lower.contains("drive.google.com") {
            return Platform::GoogleDrive;
        }
        if url_lower.contains("dropbox.com") {
            return Platform::Dropbox;
        }
        if url_lower.contains("notion.so") {
            return Platform::Notion;
        }
        
        // Short URLs
        if url_lower.contains("bit.ly") {
            return Platform::Bitly;
        }
        if url_lower.contains("tinyurl.com") {
            return Platform::TinyUrl;
        }
        
        Platform::Other(url.to_string())
    }
}
```

### Category Mapping

```rust
impl Platform {
    /// Mapea plataforma → categoría por defecto
    fn default_category(&self) -> ContentCategory {
        match self {
            Platform::YouTube | Platform::Spotify | Platform::SoundCloud | Platform::Vimeo => {
                ContentCategory::Entertainment
            }
            Platform::GitHub | Platform::GitLab | Platform::StackOverflow => {
                ContentCategory::Professional
            }
            Platform::Instagram | Platform::Facebook | Platform::Twitter | Platform::WhatsApp => {
                ContentCategory::Social
            }
            Platform::GoogleDrive | Platform::Dropbox | Platform::Notion => {
                ContentCategory::Professional
            }
            _ => ContentCategory::Unknown,
        }
    }
}
```

---

## 🔨 IMPLEMENTATION PLAN

### Phase 1: Basic Extraction (v1.0)

**Goal:** Extraer y clasificar URLs básicas

```rust
impl HyperlinkExtractor {
    /// Extrae URLs usando regex
    fn extract_urls(&self, entries: &[DigestedEntry]) -> Vec<(String, String)> {
        let url_regex = Regex::new(
            r"https?://[^\s<>\"{}|\\^`\[\]]+"
        ).unwrap();
        
        let mut urls = Vec::new();
        
        for entry in entries {
            for cap in url_regex.captures_iter(&entry.content) {
                let url = cap.get(0).unwrap().as_str().to_string();
                urls.push((url, entry.id.clone()));
            }
        }
        
        urls
    }
    
    /// Extrae contexto alrededor de URL (30 chars antes/después)
    fn extract_context(&self, content: &str, url: &str) -> String {
        if let Some(pos) = content.find(url) {
            let start = pos.saturating_sub(30);
            let end = (pos + url.len() + 30).min(content.len());
            content[start..end].to_string()
        } else {
            String::new()
        }
    }
}
```

**Nutrients Extracted (Phase 1):**
- `url` (raw URL)
- `platform` (YouTube, Spotify, etc.)
- `category` (Entertainment, Professional, etc.)
- `context` (surrounding text)

### Phase 2: Intent Inference (v1.1)

**Goal:** Inferir intención del share

```rust
impl HyperlinkExtractor {
    /// Infiere intent usando keywords del contexto
    fn infer_intent(&self, context: &str, author: &Option<String>) -> LinkIntent {
        let context_lower = context.to_lowercase();
        
        // Self-reference indicators
        let self_keywords = ["mi ", "my ", "yo ", "i ", "mio", "mine"];
        let self_score = self_keywords.iter()
            .filter(|&k| context_lower.contains(k))
            .count() as f64 / self_keywords.len() as f64;
        
        // Recommendation indicators
        let rec_keywords = ["recomiendo", "recommend", "checa", "check", "mira", "look", "te va a gustar", "you'll like"];
        let rec_score = rec_keywords.iter()
            .filter(|&k| context_lower.contains(k))
            .count() as f64 / rec_keywords.len() as f64;
        
        // Collaboration indicators
        let col_keywords = ["trabajar en", "work on", "ayuda", "help", "colaborar", "collaborate"];
        let col_score = col_keywords.iter()
            .filter(|&k| context_lower.contains(k))
            .count() as f64 / col_keywords.len() as f64;
        
        // Determine intent
        if self_score > 0.3 {
            LinkIntent::SelfReference { confidence: self_score }
        } else if rec_score > 0.3 {
            LinkIntent::Recommendation { confidence: rec_score }
        } else if col_score > 0.3 {
            LinkIntent::Collaboration { confidence: col_score }
        } else if !context.is_empty() {
            LinkIntent::Sharing { confidence: 0.5 }
        } else {
            LinkIntent::Unknown
        }
    }
}
```

**Nutrients Extracted (Phase 2):**
- `intent` (SelfReference, Recommendation, Collaboration, Sharing)
- `intent_confidence` (0.0-1.0)

### Phase 3: Consumption Analysis (v1.2)

**Goal:** Analizar patrones de consumo

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsumptionProfile {
    /// Links por categoría
    category_distribution: HashMap<ContentCategory, usize>,
    
    /// Links por plataforma
    platform_distribution: HashMap<Platform, usize>,
    
    /// Frecuencia (links/día)
    links_per_day: f64,
    
    /// Efficiency score (0.0 = pura distracción, 1.0 = puro deep work)
    efficiency_score: f64,
}

impl HyperlinkExtractor {
    fn analyze_consumption(&self, links: &[ClassifiedLink]) -> ConsumptionProfile {
        let mut category_dist = HashMap::new();
        let mut platform_dist = HashMap::new();
        
        for link in links {
            *category_dist.entry(link.category.clone()).or_insert(0) += 1;
            *platform_dist.entry(link.platform.clone()).or_insert(0) += 1;
        }
        
        // Calculate efficiency score
        let entertainment = *category_dist.get(&ContentCategory::Entertainment).unwrap_or(&0) as f64;
        let professional = *category_dist.get(&ContentCategory::Professional).unwrap_or(&0) as f64;
        let education = *category_dist.get(&ContentCategory::Education).unwrap_or(&0) as f64;
        
        let total = links.len() as f64;
        let efficiency_score = if total > 0.0 {
            (professional + education) / total
        } else {
            0.5
        };
        
        ConsumptionProfile {
            category_distribution: category_dist,
            platform_distribution: platform_dist,
            links_per_day: 0.0, // Calculate from date range
            efficiency_score,
        }
    }
}
```

**Nutrients Extracted (Phase 3):**
- `consumption_profile` (categorías, plataformas, frecuencia)
- `efficiency_score` (0.0-1.0)

### Phase 4: Social Role Inference (v1.3)

**Goal:** Inferir rol social basado en sharing patterns

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocialRole {
    /// Comparte muchos recursos valiosos (curator)
    Curator { confidence: f64 },
    
    /// Crea contenido propio (creator)
    Creator { confidence: f64 },
    
    /// Consume pero no comparte mucho (consumer)
    Consumer { confidence: f64 },
    
    /// Conecta personas y recursos (connector)
    Connector { confidence: f64 },
    
    /// No se pudo determinar
    Unknown,
}

impl HyperlinkExtractor {
    fn infer_social_role(&self, links: &[ClassifiedLink], total_messages: usize) -> SocialRole {
        let share_rate = links.len() as f64 / total_messages as f64;
        
        let self_ref_count = links.iter()
            .filter(|l| matches!(l.intent, LinkIntent::SelfReference { .. }))
            .count();
        
        let rec_count = links.iter()
            .filter(|l| matches!(l.intent, LinkIntent::Recommendation { .. }))
            .count();
        
        let col_count = links.iter()
            .filter(|l| matches!(l.intent, LinkIntent::Collaboration { .. }))
            .count();
        
        // Creator: >30% self-references
        if self_ref_count as f64 / links.len() as f64 > 0.3 {
            return SocialRole::Creator { confidence: 0.7 };
        }
        
        // Curator: >50% recommendations + high share rate
        if rec_count as f64 / links.len() as f64 > 0.5 && share_rate > 0.2 {
            return SocialRole::Curator { confidence: 0.8 };
        }
        
        // Connector: high collaboration rate
        if col_count as f64 / links.len() as f64 > 0.3 {
            return SocialRole::Connector { confidence: 0.7 };
        }
        
        // Consumer: low share rate
        if share_rate < 0.1 {
            return SocialRole::Consumer { confidence: 0.6 };
        }
        
        SocialRole::Unknown
    }
}
```

**Nutrients Extracted (Phase 4):**
- `social_role` (Curator, Creator, Consumer, Connector)
- `role_confidence` (0.0-1.0)

---

## 🧪 TESTING STRATEGY

### Test 1: URL Extraction Accuracy

**File:** `examples/test_hyperlink_extractor.rs`

```rust
#[tokio::test]
async fn test_url_extraction_accuracy() {
    let sample_messages = vec![
        "Check this out: https://youtube.com/watch?v=123",
        "My repo: https://github.com/user/project",
        "Listen to https://spotify.com/track/abc",
        "No URL here",
    ];
    
    let extracted = extract_urls(&sample_messages);
    
    assert_eq!(extracted.len(), 3); // 3 URLs found
    assert!(extracted.iter().any(|u| u.contains("youtube")));
    assert!(extracted.iter().any(|u| u.contains("github")));
    assert!(extracted.iter().any(|u| u.contains("spotify")));
}
```

### Test 2: Platform Classification

```rust
#[test]
fn test_platform_classification() {
    let extractor = HyperlinkExtractor;
    
    assert_eq!(
        extractor.classify_platform("https://youtube.com/watch?v=123"),
        Platform::YouTube
    );
    
    assert_eq!(
        extractor.classify_platform("https://github.com/user/repo"),
        Platform::GitHub
    );
    
    assert_eq!(
        extractor.classify_platform("https://spotify.com/track/abc"),
        Platform::Spotify
    );
}
```

### Test 3: Intent Inference

```rust
#[test]
fn test_intent_inference() {
    let extractor = HyperlinkExtractor;
    
    // Self-reference
    let intent1 = extractor.infer_intent("Mira mi video: ", &Some("Eduardo".to_string()));
    assert!(matches!(intent1, LinkIntent::SelfReference { .. }));
    
    // Recommendation
    let intent2 = extractor.infer_intent("Te recomiendo este tutorial: ", &None);
    assert!(matches!(intent2, LinkIntent::Recommendation { .. }));
    
    // Collaboration
    let intent3 = extractor.infer_intent("Ayúdame con este proyecto: ", &None);
    assert!(matches!(intent3, LinkIntent::Collaboration { .. }));
}
```

### Test 4: Real Data Integration

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load Paula Roque chat
    let quarantine = QuarantineZone::inspect(
        std::fs::read("data/imports/whatsapp/_chat.txt")?,
        DataSource::WhatsApp
    ).await?;
    
    let digester = WhatsAppDigester::default();
    let digested = digester.digest(&quarantine).await?;
    
    // Extract hyperlinks
    let extractor = HyperlinkExtractor;
    let extracted = extractor.extract(&digested).await?;
    
    // Validate results
    assert!(extracted.nutrients.len() >= 298); // Expected ~298 URLs
    
    // Check platform distribution
    let youtube_count = extracted.nutrients.iter()
        .filter(|n| n.value.contains("YouTube"))
        .count();
    
    println!("YouTube links: {}", youtube_count);
    println!("Total nutrients: {}", extracted.nutrients.len());
    
    Ok(())
}
```

### Test 5: Performance Validation

```rust
#[tokio::test]
async fn test_performance_target() {
    let start = std::time::Instant::now();
    
    // Load 1,354 messages
    let digested = load_test_data().await?;
    
    // Extract
    let extractor = HyperlinkExtractor;
    let _ = extractor.extract(&digested).await?;
    
    let duration = start.elapsed();
    
    // Target: <10s for 1000 entries
    // For 1,354 entries: ~13.5s max
    assert!(duration.as_secs() < 15);
    
    println!("Extraction time: {:?}", duration);
}
```

---

## ⚡ PERFORMANCE TARGETS

### Latency Requirements

```yaml
Extraction Time:
  Target: <10s for 1000 entries
  Paula Roque (1,354 entries): <15s expected
  Acceptable: <100x slower than target (100s = 1.6min)
  
Per-Message Processing:
  URL extraction: <1ms/message
  Platform classification: <0.5ms/URL
  Intent inference: <2ms/URL (includes NLP)
  
Memory Usage:
  Peak: <200MB for 1,354 entries
  Per URL: ~1KB (URL + metadata + context)
```

### Accuracy Targets

```yaml
URL Detection:
  Recall: >95% (detect 95% of URLs)
  Precision: >90% (90% detected are real URLs)
  
Platform Classification:
  Accuracy: >95% (known platforms)
  Coverage: >80% (classify 80% of URLs)
  
Intent Inference:
  High Confidence (>0.7): >60% of URLs
  Medium Confidence (0.5-0.7): >30% of URLs
  Low/Unknown (<0.5): <10% of URLs
```

---

## 🚀 FUTURE ENHANCEMENTS (Post-v1.0)

> **📚 NOTA IMPORTANTE:** La extracción profunda de contenido multimedia (audio, video, websites) está documentada en detalle en el documento hijo:
> 
> **[18.1_hyperlink-content-extraction.md](./18.1_hyperlink-content-extraction.md)**
> 
> Este documento cubre:
> - YouTube metadata + transcripts (yt-dlp)
> - Spotify tracks/playlists (Web API)
> - Webpage scraping (Open Graph, main content)
> - Audio transcription (Whisper API)
> - Video processing (ffmpeg)
> - Caching layer (SQLite)
> - Rate limiting & budget tracking
> 
> Consultar ese documento para detalles técnicos de implementación.

---

### Phase 5: URL Expansion Service

**Goal:** Expandir short URLs para análisis completo

```rust
struct UrlExpander {
    client: reqwest::Client,
    cache: HashMap<String, String>,
}

impl UrlExpander {
    async fn expand(&mut self, short_url: &str) -> Result<String> {
        // Check cache
        if let Some(expanded) = self.cache.get(short_url) {
            return Ok(expanded.clone());
        }
        
        // Follow redirects (max 5 hops)
        let response = self.client
            .get(short_url)
            .send()
            .await?;
        
        let expanded = response.url().to_string();
        
        // Cache result
        self.cache.insert(short_url.to_string(), expanded.clone());
        
        Ok(expanded)
    }
}
```

### Phase 6: Metadata Fetching ⚠️ DEPRECADO

> **⚠️ ATENCIÓN:** Esta sección ha sido reemplazada por documentación completa en:
> 
> **→ [18.1_hyperlink-content-extraction.md](./18.1_hyperlink-content-extraction.md)**
> 
> El nuevo documento incluye:
> - Architecture completa (5 extractors específicos)
> - Implementación detallada por plataforma (YouTube, Spotify, Webpage, Audio, Video)
> - Caching layer (SQLite, 7 días TTL)
> - Rate limiting & budget tracking
> - Error handling & graceful degradation
> - Testing strategy & benchmarks
> - 6-phase implementation roadmap
> 
> **No usar el código stub de abajo** - es simplificado. Referirse al documento hijo para arquitectura real.

<details>
<summary>Código stub antiguo (colapsado, no usar)</summary>

```rust
struct LinkMetadata {
    title: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    duration: Option<Duration>, // For videos/audio
}

impl HyperlinkExtractor {
    async fn fetch_metadata(&self, url: &str) -> Result<LinkMetadata> {
        // Fetch HTML
        let html = self.client.get(url).send().await?.text().await?;
        
        // Parse Open Graph tags
        let title = extract_og_tag(&html, "og:title");
        let description = extract_og_tag(&html, "og:description");
        let image = extract_og_tag(&html, "og:image");
        
        Ok(LinkMetadata {
            title,
            description,
            image_url: image,
            duration: None, // Would need API for this
        })
    }
}
```

</details>

### Phase 7: Template-Based Extraction

**Goal:** Configurar reglas via YAML templates

```yaml
# templates/extractors/hyperlink_v1.yaml
hyperlink_extraction:
  enabled: true
  
  url_patterns:
    - regex: 'https?://[^\s<>\"{}|\\^`\[\]]+'
      confidence: 0.95
  
  platform_rules:
    YouTube:
      domains: ['youtube.com', 'youtu.be']
      category: Entertainment
    GitHub:
      domains: ['github.com']
      category: Professional
  
  intent_keywords:
    SelfReference:
      - 'mi '
      - 'my '
      - 'yo '
    Recommendation:
      - 'recomiendo'
      - 'recommend'
      - 'checa'
  
  efficiency_weights:
    Entertainment: 0.0
    Professional: 1.0
    Education: 0.8
    Social: 0.3
```

---

## 📝 IMPLEMENTATION CHECKLIST

### Phase 1: Basic Extraction ✅ (Current Focus)

- [ ] Create `HyperlinkExtractor` struct
- [ ] Implement `extract_urls()` with regex
- [ ] Implement `classify_platform()` for 15+ platforms
- [ ] Implement `extract_context()` (30 chars before/after)
- [ ] Create test file `test_hyperlink_extractor.rs`
- [ ] Validate with Paula Roque chat (298 URLs expected)
- [ ] Update `mod.rs` exports
- [ ] Update CHECKLIST_V2.md Task 7.x.3.8

### Phase 2: Intent Inference 🔜 (Next)

- [ ] Implement `infer_intent()` with keywords
- [ ] Add confidence scoring
- [ ] Test intent accuracy (>60% high confidence)

### Phase 3: Consumption Analysis 🔜

- [ ] Implement `analyze_consumption()`
- [ ] Calculate efficiency score
- [ ] Generate consumption profile

### Phase 4: Social Role 🔜

- [ ] Implement `infer_social_role()`
- [ ] Test role classification

---

## 🎓 LECCIONES DE EXTRACTORES ANTERIORES

### 1. Performance

- **Temporal:** 16ms para 1,354 → 846x faster
- **Behavioral:** 18ms para 1,354 → 752x faster
- **Relationship:** 60ms para 1,354 → 226x faster

**Expectativa HyperlinkExtractor:** 30-50ms (URL parsing más complejo)

### 2. Confidence Scoring

```rust
// Pattern establecido:
let confidence = if samples >= 100 {
    0.9
} else if samples >= 50 {
    0.7
} else {
    0.5
};
```

### 3. Metadata Structure

```rust
// Siempre incluir:
metadata: HashMap::from([
    ("total_urls".to_string(), urls.len().to_string()),
    ("platforms_detected".to_string(), platforms.len().to_string()),
    ("efficiency_score".to_string(), format!("{:.2}", score)),
]),
```

### 4. Test Pattern

```rust
// Estructura consistente:
1. Quarantine
2. Digestion
3. Extraction
4. Results Analysis (5-7 sections)
5. Pattern Insights
6. Performance Assessment
7. Quality Assessment
```

---

## 🎯 SUCCESS CRITERIA

### Must Have (v1.0)

- ✅ Extract 95%+ of URLs from Paula Roque chat (>280 URLs)
- ✅ Classify 80%+ of platforms correctly
- ✅ Extract context for 90%+ of URLs
- ✅ Performance: <50ms for 1,354 entries
- ✅ Quality score: >90%

### Nice to Have (v1.1+)

- Intent inference with >60% high confidence
- Efficiency score calculation
- Social role classification
- URL expansion service
- Metadata fetching

---

## 📚 REFERENCIAS

### Documentos Relacionados

1. **ROADMAP_V2/02_COMPONENTES/17_data-import-engine.md**
   - Sección "Hyperlink Intelligence" (líneas 785-1000)
   - Architecture overview

2. **ROADMAP_V2/CHECKLIST_V2.md**
   - Task 7.x.3.8: HyperlinkExtractor
   - Task 7.x.7: Hyperlink Intelligence (Phase 7.x.7)

3. **examples/test_digestion_deep_analysis.rs**
   - Hyperlinks readiness: 95% (EXCELLENT)
   - 298 URLs detected in Paula Roque chat

### Código de Referencia

1. **src/data_import/extraction.rs**
   - BiographicalExtractor (regex patterns)
   - TemporalExtractor (pattern enums)
   - RelationshipExtractor (classification logic)

2. **ROADMAP_V2/02_COMPONENTES/01_sensory-engine.md**
   - URL regex pattern (línea 512)
   - Reference extraction pattern

---

**Estado:** 🎯 READY FOR IMPLEMENTATION  
**Prioridad:** ALTA - Último extractor (8/8)  
**Próximo Paso:** Implementar Phase 1 (Basic Extraction)
