```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/01_ARQUITECTURA/13_hybrid-intelligence-engine.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 18:30:00
Autor: Sistema Bitácora + Eduardo
Propósito: Arquitectura técnica del sistema híbrido Local + LLM
Estado: 🎯 ACTIVO - Especificación técnica para implementación
Relacionado Con:
  - 00_VISION/DA-035_HYBRID_INTELLIGENCE_ENGINE.md (visión)
  - 02_COMPONENTES/09_relationship-psychology-analyzer.md (componente)
  - src/data_import/extraction.rs (implementación)
Categoría: ARCHITECTURE
Prioridad: ALTA
# === FIN DATOS DE AUDITORÍA ===
```

# 🏗️ Hybrid Intelligence Engine - Arquitectura Técnica

> **Implementación:** Phase 7.x.3+ (Extraction Layer Enhancement)

---

## 📋 TABLA DE CONTENIDOS

1. [System Overview](#system-overview)
2. [Component Architecture](#component-architecture)
3. [Data Structures](#data-structures)
4. [Algorithms](#algorithms)
5. [API Specification](#api-specification)
6. [Performance](#performance)
7. [Security](#security)
8. [Testing Strategy](#testing-strategy)

---

## 🎯 SYSTEM OVERVIEW

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│           Application Layer (CLI / Mobile)               │
└───────────────────┬─────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────┐
│         HybridIntelligenceEngine (Orchestrator)         │
├─────────────────────────────────────────────────────────┤
│  • Confidence-based routing                             │
│  • Budget management                                    │
│  • Cache coordination                                   │
│  • Metrics collection                                   │
└───────┬─────────────────────────────────────────────────┘
        │
        ├──────────────────────┬──────────────────────────
        ▼                      ▼                      
┌──────────────────┐    ┌──────────────────┐
│  Local Analyzers │    │  LLM Subsystem   │
├──────────────────┤    ├──────────────────┤
│• EmotionalExt    │    │• Anonymizer      │
│• InterestExt     │    │• LLM Client      │
│• BiographicalExt │    │• Cache Manager   │
│• RelationshipExt │    │• Learning Engine │
└──────────────────┘    └──────────────────┘
```

### Module Location

```
src/data_import/
├── extraction.rs          # NutrientExtractor trait + base impl
├── hybrid_intelligence.rs # NEW - HybridIntelligenceEngine
├── anonymizer.rs          # NEW - PII removal & anonymization
├── llm_client.rs          # NEW - Multi-provider LLM client
├── cache.rs               # NEW - LLM response cache
└── learning.rs            # NEW - Progressive learning engine
```

---

## 🧩 COMPONENT ARCHITECTURE

### 1. HybridIntelligenceEngine

**Responsibilities:**
- Orchestrate local vs LLM decision
- Manage confidence thresholds
- Track budget & usage
- Collect metrics

**Dependencies:**
- `NutrientExtractor` (local analyzers)
- `Anonymizer`
- `LLMClient`
- `CacheManager`
- `LearningEngine`

**Configuration:**
```rust
pub struct HybridConfig {
    /// Threshold para usar local (0.0-1.0)
    pub confidence_threshold: f64,  // default: 0.7
    
    /// Habilitar LLM (opt-in)
    pub llm_enabled: bool,  // default: false
    
    /// Proveedor LLM
    pub llm_provider: LLMProvider,  // default: None
    
    /// Budget limits
    pub max_queries_per_session: usize,  // default: 10
    pub max_queries_per_month: usize,    // default: 100
    pub max_cost_per_month_usd: f64,     // default: 1.0
    
    /// Anonymization level
    pub anonymization_level: AnonymizationLevel,  // default: Maximum
    
    /// Cache settings
    pub cache_enabled: bool,           // default: true
    pub cache_ttl_days: u32,           // default: 30
    pub cache_shared: bool,            // default: false (opt-in)
}
```

### 2. Anonymizer

**Responsibilities:**
- Remove PII (names, emails, phones, locations)
- Preserve linguistic structure
- Generate anonymous context

**PII Detection:**
```rust
pub struct PIIDetector {
    /// Regex patterns for detection
    patterns: PIIPatterns,
    
    /// NER model (optional, future)
    ner_model: Option<NERModel>,
}

pub struct PIIPatterns {
    pub phone_regex: Regex,      // \d{3}-\d{3}-\d{4}
    pub email_regex: Regex,      // [a-z]+@[a-z]+\.[a-z]+
    pub url_regex: Regex,        // https?://...
    pub location_regex: Regex,   // [A-Z][a-z]+(ville|ton|burg)
    pub name_patterns: Vec<String>,  // From author metadata
}
```

### 3. LLMClient

**Responsibilities:**
- Abstract multi-provider API
- Handle rate limiting
- Retry logic
- Cost tracking

**Supported Providers:**
```rust
pub enum LLMProvider {
    OpenAI {
        api_key: String,
        model: String,  // "gpt-4", "gpt-3.5-turbo"
    },
    Anthropic {
        api_key: String,
        model: String,  // "claude-3-sonnet-20240229"
    },
    Groq {
        api_key: String,
        model: String,  // "mixtral-8x7b-32768"
    },
    LocalLLaMA {
        endpoint: String,  // "http://localhost:11434"
        model: String,     // "llama3:70b"
    },
    None,
}
```

### 4. CacheManager

**Responsibilities:**
- Store LLM responses
- Key generation (hash-based)
- TTL management
- Hit rate tracking

**Storage:**
```rust
// SQLite backend
CREATE TABLE llm_cache (
    cache_key TEXT PRIMARY KEY,
    query_hash TEXT NOT NULL,
    response_json TEXT NOT NULL,
    confidence REAL NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    hit_count INTEGER DEFAULT 0,
    provider TEXT NOT NULL,
    cost_usd REAL NOT NULL
);

CREATE INDEX idx_expires ON llm_cache(expires_at);
CREATE INDEX idx_query_hash ON llm_cache(query_hash);
```

### 5. LearningEngine

**Responsibilities:**
- User corrections → Weight updates
- LLM patterns → Lexicon expansion
- A/B testing results → Threshold optimization

**Learning Flow:**
```rust
pub trait LearningEngine {
    /// Update weights from user correction
    fn learn_from_correction(
        &mut self,
        message: &str,
        predicted: Sentiment,
        actual: Sentiment,
    );
    
    /// Extract patterns from LLM response
    fn learn_from_llm(
        &mut self,
        message: &str,
        llm_response: &LLMResponse,
    );
    
    /// Optimize threshold based on A/B test
    fn optimize_threshold(
        &mut self,
        test_results: &ABTestResults,
    );
}
```

---

## 📦 DATA STRUCTURES

### Core Types

```rust
// ═══════════════════════════════════════════════════════════
// ANALYSIS RESULT (Enhanced with confidence)
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult<T> {
    /// Resultado del análisis
    pub result: T,
    
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    
    /// Fuente del análisis
    pub source: AnalysisSource,
    
    /// Razón de baja confianza
    pub low_confidence_reason: Option<String>,
    
    /// Metadata adicional
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalysisSource {
    Local {
        analyzer: String,
        duration_ms: u64,
    },
    CachedLLM {
        cached_at: DateTime<Utc>,
        original_provider: String,
    },
    LLM {
        provider: String,
        model: String,
        cost_usd: f64,
        latency_ms: u64,
    },
}

// ═══════════════════════════════════════════════════════════
// ANONYMIZED CONTEXT
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedContext {
    /// Mensaje anonimizado
    pub anonymized_message: String,
    
    /// Metadata agregada (no identificable)
    pub conversation_metadata: ConversationMetadata,
    
    /// Nivel de anonimización usado
    pub anonymization_level: AnonymizationLevel,
    
    /// PII items removidos (tipos, no valores)
    pub pii_removed: Vec<PIIType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub message_number: usize,
    pub total_messages: usize,
    pub duration_days: i64,
    pub sentiment_distribution: SentimentDistribution,
    pub frequency_pattern: FrequencyPattern,
    pub participant_count: usize,
    pub language: Option<String>,  // "es", "en", etc
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentDistribution {
    pub positive_pct: f64,
    pub neutral_pct: f64,
    pub negative_pct: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrequencyPattern {
    Daily,
    Weekly,
    Monthly,
    Sporadic,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnonymizationLevel {
    Maximum,  // Solo estructura lingüística
    High,     // + metadata agregada
    Medium,   // + contexto temporal/cultural
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PIIType {
    Name,
    Phone,
    Email,
    Address,
    Location,
    URL,
    CreditCard,
    SSN,
    Other(String),
}

// ═══════════════════════════════════════════════════════════
// LLM REQUEST/RESPONSE
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f64>,
    pub stop_sequences: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub text: String,
    pub model: String,
    pub provider: String,
    pub latency_ms: u64,
    pub tokens_used: TokenUsage,
    pub cost_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
}

// ═══════════════════════════════════════════════════════════
// CACHE ENTRY
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub cache_key: String,
    pub query_hash: String,
    pub response: LLMResponse,
    pub confidence: f64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hit_count: usize,
}
```

---

## 🧮 ALGORITHMS

### Confidence Scoring

```rust
impl EmotionalExtractor {
    fn calculate_confidence(&self, analysis: &SentimentAnalysis) -> f64 {
        let total_signals = analysis.positive_signals + analysis.negative_signals;
        
        // Factor 1: Signal strength
        let signal_factor = if total_signals == 0 {
            0.1  // No signals → very low confidence
        } else if total_signals >= 5 {
            1.0  // Many signals → high confidence
        } else {
            (total_signals as f64) / 5.0
        };
        
        // Factor 2: Score magnitude
        let score_factor = analysis.score.abs();
        
        // Factor 3: Ambiguity (mixed signals)
        let ambiguity_penalty = if analysis.positive_signals > 0 
            && analysis.negative_signals > 0 {
            let ratio = analysis.positive_signals.min(analysis.negative_signals) as f64
                / analysis.positive_signals.max(analysis.negative_signals) as f64;
            ratio * 0.3  // Max penalty: 0.3
        } else {
            0.0
        };
        
        // Combined confidence
        let confidence = (signal_factor * 0.6 + score_factor * 0.4) 
            - ambiguity_penalty;
        
        confidence.clamp(0.0, 1.0)
    }
}
```

### Cache Key Generation

```rust
impl CacheManager {
    fn generate_cache_key(&self, context: &AnonymizedContext) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // Hash message content
        hasher.update(context.anonymized_message.as_bytes());
        
        // Hash relevant metadata (NOT full context)
        hasher.update(&context.conversation_metadata.participant_count.to_le_bytes());
        hasher.update(&context.anonymization_level.to_string().as_bytes());
        
        // Generate hash
        let result = hasher.finalize();
        format!("cache_{:x}", result)
    }
}
```

### Budget Tracking

```rust
pub struct BudgetTracker {
    config: BudgetConfig,
    usage: BudgetUsage,
    db: Database,
}

impl BudgetTracker {
    pub fn can_make_query(&self) -> Result<bool> {
        // Check session limit
        if self.usage.queries_this_session >= self.config.max_queries_per_session {
            return Ok(false);
        }
        
        // Check monthly limit (queries)
        let monthly_queries = self.db.count_queries_this_month()?;
        if monthly_queries >= self.config.max_queries_per_month {
            return Ok(false);
        }
        
        // Check monthly limit (cost)
        let monthly_cost = self.db.sum_cost_this_month()?;
        if monthly_cost >= self.config.max_cost_per_month_usd {
            return Ok(false);
        }
        
        Ok(true)
    }
    
    pub fn record_query(&mut self, cost_usd: f64) -> Result<()> {
        self.usage.queries_this_session += 1;
        self.usage.total_cost_usd += cost_usd;
        
        self.db.insert_query_log(QueryLog {
            timestamp: Utc::now(),
            cost_usd,
            provider: self.config.provider.to_string(),
        })?;
        
        Ok(())
    }
}
```

---

## 🔌 API SPECIFICATION

### Main Interface

```rust
#[async_trait]
pub trait HybridAnalyzer: Send + Sync {
    /// Analiza con decisión híbrida (local vs LLM)
    async fn analyze_hybrid<T>(
        &mut self,
        entry: &DigestedEntry,
        context: &ConversationContext,
    ) -> Result<AnalysisResult<T>>
    where
        T: Serialize + DeserializeOwned + Send;
    
    /// Force local analysis (sin LLM)
    fn analyze_local<T>(
        &self,
        entry: &DigestedEntry,
    ) -> Result<AnalysisResult<T>>
    where
        T: Serialize + DeserializeOwned;
    
    /// Get current config
    fn config(&self) -> &HybridConfig;
    
    /// Update config
    fn set_config(&mut self, config: HybridConfig);
    
    /// Get usage stats
    fn usage_stats(&self) -> BudgetUsage;
    
    /// Clear cache
    async fn clear_cache(&mut self) -> Result<()>;
}
```

### Usage Example

```rust
// Setup
let mut engine = HybridIntelligenceEngine::new(HybridConfig {
    confidence_threshold: 0.7,
    llm_enabled: true,
    llm_provider: LLMProvider::Anthropic {
        api_key: env::var("ANTHROPIC_API_KEY")?,
        model: "claude-3-sonnet-20240229".to_string(),
    },
    max_queries_per_session: 10,
    ..Default::default()
});

// Analyze
let result: AnalysisResult<Sentiment> = engine
    .analyze_hybrid(&digested_entry, &conversation_context)
    .await?;

// Check source
match result.source {
    AnalysisSource::Local { .. } => {
        println!("✅ Local (confidence: {:.2})", result.confidence);
    }
    AnalysisSource::CachedLLM { .. } => {
        println!("💾 Cached (confidence: {:.2})", result.confidence);
    }
    AnalysisSource::LLM { cost_usd, .. } => {
        println!("🤖 LLM (cost: ${:.4}, confidence: {:.2})", 
            cost_usd, result.confidence);
    }
}

// Use result
match result.result {
    Sentiment::Positive => handle_positive(),
    Sentiment::Negative => handle_negative(),
    Sentiment::Neutral => handle_neutral(),
}
```

---

## ⚡ PERFORMANCE

### Latency Targets

```yaml
Local Analysis:
  p50: <50ms
  p95: <100ms
  p99: <200ms

LLM Query (first time):
  p50: <500ms
  p95: <1s
  p99: <2s

Cache Hit:
  p50: <5ms
  p95: <10ms
  p99: <20ms
```

### Throughput

```yaml
Local:
  messages_per_second: >1000
  
Hybrid (93% local, 7% LLM):
  messages_per_second: >50
  
Full LLM:
  messages_per_second: ~5
```

### Memory Usage

```yaml
Base:
  resident: ~50MB
  
With Cache (1000 entries):
  resident: ~70MB
  
With LLM Client:
  resident: ~100MB
```

---

## 🔐 SECURITY

### Threat Model

```yaml
Threats:
  1. PII Leakage via LLM:
     Mitigation: Anonymization + audit trail
     
  2. Cache Poisoning:
     Mitigation: Signature verification, TTL
     
  3. Budget Exhaustion Attack:
     Mitigation: Rate limiting, session limits
     
  4. API Key Exposure:
     Mitigation: Secure storage, environment vars
     
  5. Cache Timing Attack:
     Mitigation: Constant-time lookups
```

### Audit Trail

```rust
pub struct AuditLog {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub user_id: Option<String>,  // Hashed
    pub message_id: String,         // Hashed
    pub anonymization_level: AnonymizationLevel,
    pub llm_provider: Option<String>,
    pub pii_removed: Vec<PIIType>,
    pub cost_usd: f64,
}

pub enum AuditEventType {
    LocalAnalysis,
    CacheHit,
    LLMQuery,
    UserCorrection,
    ConfigChange,
}
```

---

## 🧪 TESTING STRATEGY

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_confidence_scoring() {
        let extractor = EmotionalExtractor::new();
        
        // High confidence case
        let result = extractor.analyze_with_confidence("Te amo mucho amor");
        assert!(result.confidence > 0.9);
        
        // Low confidence case
        let result = extractor.analyze_with_confidence("Nos vemos");
        assert!(result.confidence < 0.5);
    }
    
    #[test]
    fn test_pii_removal() {
        let anonymizer = Anonymizer::new();
        
        let original = "Llama a Paula al 352-555-1234";
        let anonymized = anonymizer.anonymize(original);
        
        assert!(!anonymized.contains("Paula"));
        assert!(!anonymized.contains("352-555-1234"));
        assert!(anonymized.contains("[PHONE]"));
    }
    
    #[tokio::test]
    async fn test_cache_hit() {
        let mut engine = HybridIntelligenceEngine::new_test();
        
        // First query (miss)
        let result1 = engine.analyze_hybrid(&entry, &context).await?;
        assert!(matches!(result1.source, AnalysisSource::LLM { .. }));
        
        // Second query (hit)
        let result2 = engine.analyze_hybrid(&entry, &context).await?;
        assert!(matches!(result2.source, AnalysisSource::CachedLLM { .. }));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_hybrid_with_real_data() {
    // Load Paula Roque chat
    let digested = load_test_chat("paula_roque.txt").await?;
    
    let mut engine = HybridIntelligenceEngine::new(test_config());
    
    let mut local_count = 0;
    let mut llm_count = 0;
    let mut cache_count = 0;
    
    for entry in &digested.entries {
        let result = engine.analyze_hybrid(entry, &context).await?;
        
        match result.source {
            AnalysisSource::Local { .. } => local_count += 1,
            AnalysisSource::LLM { .. } => llm_count += 1,
            AnalysisSource::CachedLLM { .. } => cache_count += 1,
        }
    }
    
    // Assertions
    assert!(local_count > 700);  // >90% local
    assert!(llm_count < 100);    // <10% LLM
    assert_eq!(local_count + llm_count + cache_count, digested.entries.len());
}
```

### Benchmark Tests

```rust
#[bench]
fn bench_local_analysis(b: &mut Bencher) {
    let extractor = EmotionalExtractor::new();
    let message = "Te amo mucho mi amor";
    
    b.iter(|| {
        extractor.analyze_with_confidence(message)
    });
}

#[bench]
fn bench_anonymization(b: &mut Bencher) {
    let anonymizer = Anonymizer::new();
    let message = "Paula, llámame al 352-555-1234";
    
    b.iter(|| {
        anonymizer.anonymize(message)
    });
}
```

---

## 📚 REFERENCES

- DA-035: Hybrid Intelligence Engine (Vision)
- src/data_import/extraction.rs: NutrientExtractor base
- examples/test_extraction_real_data.rs: Real data validation

---

**Status:** 🟢 Ready for Implementation  
**Next:** Create Component Doc (09_relationship-psychology-analyzer.md)
