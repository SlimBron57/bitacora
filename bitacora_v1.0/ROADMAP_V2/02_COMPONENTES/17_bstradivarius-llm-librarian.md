```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/17_bstradivarius-llm-librarian.md
Versión: 1.0
Fecha Creación: 2025-11-30
Última Actualización: 2025-11-30 16:30:00
Propósito: Especificación técnica completa BStradivarius v2.0 - Bibliotecario LLM Ecosistema
Estado: 🎯 ESPECIFICACIÓN TÉCNICA - Post DA-035
Autor: Sistema Bitácora - Refactorización Ecosystem Native
Template: 07_TEMPLATES/component_spec.yaml v1.0
Relacionado Con:
  - 00_VISION/DA-035_BSTRADIVARIUS_ECOSYSTEM_LIBRARIAN.md (decisión arquitectónica)
  - BSTRADIVARIUS_ARCHITECTURE_ANALYSIS.md (análisis inconsistencia)
  - 02_COMPONENTES/03_fbcu-core.md (FBCU engine)
  - 02_COMPONENTES/13_shuidao-cognitive-engine.md (ShuiDao engine)
  - 02_COMPONENTES/09_hubspoke-navigator.md (LLM routing)
  - 02_COMPONENTES/06_voxeldb.md (VoxelDB storage)
Implementa: Refactorización completa BStradivarius tradicional → ecosistema native
Bloqueado Por: Ninguno (FBCU ✅, ShuiDao ✅, HubSpoke ✅ implementados)
Tiempo Estimado: 80-100h (3 fases)
# === FIN DATOS DE AUDITORÍA ===
```

# 🎻 BSTRADIVARIUS v2.0 - BIBLIOTECARIO LLM DEL ECOSISTEMA

## Especificación Técnica Componente

> **Propósito**: *"Bibliotecario inteligente que permite a desarrolladores y LLMs navegar, entender y utilizar la documentación de Bitácora (y cualquier proyecto) durante todo el ciclo de desarrollo."*

> **Principio**: *"Un sistema que se auto-documenta debe usar su propia arquitectura innovadora."*

---

## 📚 TABLA DE CONTENIDOS

1. [Propósito](#propósito)
2. [Arquitectura v2.0](#arquitectura-v20)
3. [Componentes Integrados](#componentes-integrados)
4. [API Pública](#api-pública)
5. [Casos de Uso](#casos-de-uso)
6. [Integración con Ecosystem](#integración-con-ecosystem)
7. [Performance Targets](#performance-targets)
8. [Testing Strategy](#testing-strategy)
9. [Migration Path v1.0 → v2.0](#migration-path-v10--v20)

---

## 🎯 PROPÓSITO

### Problema que Resuelve

**BStradivarius v1.0** (Traditional):
```yaml
PROBLEMA:
  - Usa HashMap tradicional (no semantic search)
  - Guarda JSON raw (no FBCU compression)
  - String matching con regex (no QPX queries)
  - Solo retorna listas (no LLM responses)
  - NO puede regenerar docs (solo metadatos)

IMPACTO:
  - Inconsistencia arquitectónica (criatura terrestre en ecosistema acuático)
  - No aprovecha innovaciones de Bitácora
  - LLMs no pueden usarlo efectivamente
  - Documentación no recuperable
```

**BStradivarius v2.0** (Ecosystem Native):
```yaml
SOLUCIÓN:
  - Semantic search con QPX
  - FBCU compression (regeneración posible)
  - ShuiDao intention detection
  - HubSpoke LLM responses contextuales
  - API para LLMs externos

BENEFICIOS:
  - Coherencia arquitectónica completa
  - Dogfooding de innovaciones
  - Bibliotecario LLM funcional
  - Sistema se auto-documenta correctamente
```

### Rol Principal

> *"Bibliotecario inteligente para desarrolladores y LLMs"*

**Responsabilidades**:

1. **Indexación Continua** (v1.0 ✅ + v2.0 mejoras)
   - Escanear directorios documentación
   - Indexar con VoxelDB Octree
   - Comprimir con FBCU (NEW)
   - Watch mode auto-actualización

2. **Consultas Manuales (CLI)** (v1.0 ✅ + v2.0 semantic)
   - Desarrolladores buscan conceptos
   - Queries semánticos QPX (NEW)
   - Regenerar docs perdidos (NEW)
   - Respuestas rápidas (Light mode)

3. **Consultas Programáticas (API)** (v2.0 ��)
   - LLMs consultan durante desarrollo
   - Context augmentation prompts
   - Intention detection ShuiDao
   - Responses generadas LLM

4. **Guía Implementación** (v2.0 🆕)
   - Operational mode: proyectos
   - Learning mode: explicaciones
   - Sugerir próximos pasos
   - Trackear progreso

---

## 🏗️ ARQUITECTURA V2.0

### Diagrama General

\`\`\`
┌──────────────────────────────────────────────────────────────────┐
│                    BSTRADIVARIUS v2.0                            │
│                 Bibliotecario LLM Ecosistema                     │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────┐        ┌────────────────┐                  │
│  │  CLI MANUAL    │        │  API EXTERNA   │                  │
│  │ (Desarrollador)│        │  (LLMs)        │                  │
│  └────────┬───────┘        └────────┬───────┘                  │
│           │                         │                           │
│           └─────────┬───────────────┘                           │
│                     │                                           │
│          ┌──────────▼──────────────┐                           │
│          │  BstradivariusLibrarian │                           │
│          │  (Main Coordinator)     │                           │
│          └──────────┬──────────────┘                           │
│                     │                                           │
│     ┌───────────────┼───────────────┐                          │
│     │               │               │                          │
│  ┌──▼───┐      ┌───▼────┐     ┌───▼────┐                     │
│  │ FBCU │      │  QPX   │     │ShuiDao │                     │
│  │Engine│      │Queries │     │ + LLM  │                     │
│  └──┬───┘      └───┬────┘     └───┬────┘                     │
│     │              │              │                           │
│     └──────────┬───┴──────────┬───┘                           │
│                │              │                               │
│          ┌─────▼──────────────▼─────┐                         │
│          │      VoxelDB Octree      │                         │
│          │  (Spatial Indexing 3D)   │                         │
│          └──────────────────────────┘                         │
│                                                                │
└──────────────────────────────────────────────────────────────────┘
\`\`\`

### Struct Principal

\`\`\`rust
/// BStradivarius v2.0 - Bibliotecario LLM
pub struct BstradivariusLibrarian {
    // Core indexing (v1.0)
    voxel_db: VoxelDB,
    name_index: HashMap<String, String>,  // Backward compat
    patterns: Vec<IndexPattern>,          // Backward compat
    
    // NEW: Compression engine (Fase 1)
    fbcu_engine: FBCUEngine,
    compression_cache: LruCache<String, Vec<u8>>,
    
    // NEW: Query language (Fase 2)
    pxlang_engine: PXLangEngine,
    qpx_parser: QPXParser,
    semantic_cache: LruCache<String, SemanticQueryResult>,
    
    // NEW: Cognitive understanding (Fase 3)
    intention_detector: IntentionDetector,
    cognitive_router: CognitiveRouter,
    
    // NEW: LLM integration (Fase 3)
    hubspoke_navigator: HubSpokeNavigator,
    response_synthesizer: ResponseSynthesizer,
    conversation_history: Vec<Conversation>,
    
    // Config
    config: LibrarianConfig,
}

pub struct LibrarianConfig {
    // Storage
    pub storage_path: PathBuf,
    pub compression_enabled: bool,
    pub compression_threshold: usize,  // Min size to compress
    
    // Query
    pub semantic_enabled: bool,
    pub semantic_threshold: f64,  // Min confidence for semantic
    
    // LLM
    pub llm_enabled: bool,
    pub max_context_tokens: usize,
    pub response_timeout: Duration,
    
    // Cache
    pub cache_size: usize,
    pub cache_ttl: Duration,
}
\`\`\`

---

## 🧩 COMPONENTES INTEGRADOS

### 1. FBCU Engine (Fase 1)

**Propósito**: Comprimir contenido completo documentación

\`\`\`rust
impl BstradivariusLibrarian {
    /// Almacenar concepto con contenido comprimido
    pub fn store_concept_compressed(
        &mut self,
        concept: &str,
        content: &str,
        metadata: ConceptMetadata,
    ) -> Result<()> {
        // 1. Compress with FBCU
        let compressed = self.fbcu_engine.compress(content.as_bytes())?;
        
        // 2. Create template
        let mut template = TemplateEntry::new(
            concept.to_string(),
            metadata.category,
            String::new(),  // Empty for backward compat
        );
        template.compressed_content = Some(compressed);
        template.tags = metadata.tags;
        
        // 3. Store in VoxelDB
        self.voxel_db.save_template(&template)?;
        
        // 4. Update index
        self.name_index.insert(concept.to_string(), template.id.clone());
        
        Ok(())
    }
    
    /// Regenerar markdown desde contenido comprimido
    pub fn regenerate_markdown(&self, concept: &str) -> Result<String> {
        // 1. Find template
        let template = self.voxel_db.get_template_by_name(concept)?;
        
        // 2. Check cache
        if let Some(cached) = self.compression_cache.get(&template.id) {
            return Ok(String::from_utf8(cached.clone())?);
        }
        
        // 3. Decompress
        if let Some(compressed) = &template.compressed_content {
            let decompressed = self.fbcu_engine.decompress(compressed)?;
            let content = String::from_utf8(decompressed.clone())?;
            
            // 4. Cache result
            self.compression_cache.put(template.id.clone(), decompressed);
            
            Ok(content)
        } else {
            Err(LibrarianError::NoCompressedContent(concept.to_string()))
        }
    }
}
\`\`\`

**Dependencias**:
- `FBCUEngine`: `src/fbcu/mod.rs` (✅ implementado)
- `TemplateEntry`: `src/voxeldb/mod.rs` (✅ implementado)

---

### 2. QPX Query Engine (Fase 2)

**Propósito**: Semantic search vs string matching

\`\`\`rust
impl BstradivariusLibrarian {
    /// Query semántico con QPX
    pub async fn query_semantic(
        &self,
        query: &str,
    ) -> Result<SemanticQueryResult> {
        // 1. Check cache
        if let Some(cached) = self.semantic_cache.get(query) {
            return Ok(cached.clone());
        }
        
        // 2. Parse natural language
        let pxquery = self.pxlang_engine.parse_natural(query)?;
        
        // 3. Execute spatial search
        let templates = self.voxel_db.query_spatial(&pxquery).await?;
        
        // 4. Score by semantic relevance
        let scored = self.score_semantic_relevance(&templates, &pxquery)?;
        
        // 5. Extract related topics
        let related = self.extract_related_topics(&scored)?;
        
        let result = SemanticQueryResult {
            templates: scored,
            semantic_context: pxquery.context(),
            related_topics: related,
            query_intent: pxquery.detected_intent(),
        };
        
        // 6. Cache result
        self.semantic_cache.put(query.to_string(), result.clone());
        
        Ok(result)
    }
    
    /// Score semantic relevance
    fn score_semantic_relevance(
        &self,
        templates: &[TemplateEntry],
        query: &PXQuery,
    ) -> Result<Vec<ScoredTemplate>> {
        templates.iter().map(|t| {
            let score = self.calculate_relevance_score(t, query)?;
            Ok(ScoredTemplate {
                template: t.clone(),
                score,
                factors: RelevanceFactors {
                    embedding_similarity: score * 0.4,
                    topic_overlap: score * 0.3,
                    spatial_distance: score * 0.2,
                    recency: score * 0.1,
                },
            })
        }).collect()
    }
}
\`\`\`

**Dependencias**:
- `PXLangEngine`: specs completos (⏳ implementación parcial)
- `QPXParser`: specs completos (❌ no implementado aún)
- `VoxelDB::query_spatial()`: (⏳ por implementar)

---

### 3. ShuiDao + LLM (Fase 3)

**Propósito**: Respuestas inteligentes con contexto

\`\`\`rust
impl BstradivariusLibrarian {
    /// Responder pregunta con LLM
    pub async fn ask(
        &mut self,
        question: &str,
    ) -> Result<IntelligentResponse> {
        // 1. Detect intention
        let intention = self.intention_detector.detect(question)?;
        
        // 2. Route to appropriate mode
        match intention.mode {
            CognitiveMode::Learning => self.ask_learning(question, &intention).await,
            CognitiveMode::Operational => self.ask_operational(question, &intention).await,
            CognitiveMode::Light => self.ask_light(question).await,
            _ => Err(LibrarianError::UnsupportedMode(intention.mode)),
        }
    }
    
    /// Learning mode: explicar conceptos
    async fn ask_learning(
        &mut self,
        question: &str,
        intention: &DetectedIntention,
    ) -> Result<IntelligentResponse> {
        // 1. Query semantic templates
        let query_result = self.query_semantic(question).await?;
        
        // 2. Decompress content
        let content = self.decompress_templates(&query_result.templates)?;
        
        // 3. Build LLM context
        let context = self.build_llm_context(&content, question)?;
        
        // 4. Generate explanation
        let explanation = self.hubspoke_navigator
            .generate_explanation(question, &context)
            .await?;
        
        // 5. Suggest learning path
        let next_steps = self.suggest_learning_path(&query_result.templates)?;
        
        // 6. Store conversation
        self.conversation_history.push(Conversation {
            question: question.to_string(),
            response: explanation.clone(),
            mode: CognitiveMode::Learning,
            timestamp: Utc::now(),
        });
        
        Ok(IntelligentResponse::Learning {
            explanation,
            references: query_result.templates,
            next_steps,
        })
    }
}
\`\`\`

**Dependencias**:
- `IntentionDetector`: `src/shuidao/intention_detector.rs` (✅ implementado)
- `CognitiveRouter`: `src/shuidao/cognitive_router.rs` (✅ implementado)
- `HubSpokeNavigator`: `src/multi_agent/hubspoke.rs` (✅ implementado)

---

## �� API PÚBLICA

### CLI Commands

\`\`\`bash
# === FASE 1: FBCU ===

# Sync con compression
bstradivarius sync --with-content

# Regenerar markdown
bstradivarius regenerate "VoxelDB - Base de Datos Cúbica"
bstradivarius regenerate --all --output ./regenerated/

# === FASE 2: QPX ===

# Query semántico
bstradivarius query-semantic "diseño de sistemas de almacenamiento"
bstradivarius query-semantic "cómo implementar templates MTT-DSL"

# Query tradicional (backward compat)
bstradivarius query "arquitectura"

# === FASE 3: SHUIDAO + LLM ===

# Ask (modo inteligente)
bstradivarius ask "¿Cómo implemento un template MTT-DSL?"
bstradivarius ask --interactive  # Conversación continua

# === COMÚN ===

# Métricas
bstradivarius metrics
bstradivarius metrics --detailed

# Export
bstradivarius export
bstradivarius export --format json --output knowledge.json

# Watch mode
bstradivarius watch
\`\`\`

### Rust API (para integración)

\`\`\`rust
use bitacora::bstradivarius::BstradivariusLibrarian;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize
    let mut librarian = BstradivariusLibrarian::new("./data")?;
    
    // Sync documentation
    librarian.sync_directory("./ROADMAP_V2").await?;
    
    // Query semantic
    let results = librarian.query_semantic(
        "diseño de bases de datos"
    ).await?;
    
    println!("Found {} results", results.templates.len());
    for scored in results.templates {
        println!("  - {} (score: {:.2})", 
            scored.template.name, 
            scored.score
        );
    }
    
    // Ask question (LLM)
    let response = librarian.ask(
        "¿Cómo funciona VoxelDB?"
    ).await?;
    
    match response {
        IntelligentResponse::Learning { explanation, references, .. } => {
            println!("Explanation:\\n{}", explanation);
            println!("\\nReferences:");
            for r in references {
                println!("  - {}", r.file);
            }
        }
        _ => {}
    }
    
    Ok(())
}
\`\`\`

### REST API (para LLMs externos)

\`\`\`http
# Query semantic
POST /api/v1/query-semantic
Content-Type: application/json

{
  "query": "diseño de sistemas de almacenamiento",
  "max_results": 10,
  "min_score": 0.7
}

# Response
{
  "results": [
    {
      "name": "VoxelDB - Base de Datos Cúbica",
      "file": "02_COMPONENTES/06_voxeldb.md",
      "line": 127,
      "score": 0.92,
      "context": "Almacenamiento espacial de templates...",
      "related_topics": ["TelescopeDB", "FBCU", "Octree"]
    }
  ],
  "query_time_ms": 45,
  "semantic_context": {...}
}

# Ask question (LLM)
POST /api/v1/ask
Content-Type: application/json

{
  "question": "¿Cómo implemento un template MTT-DSL?",
  "mode": "learning",  // or "operational", "light"
  "max_context_tokens": 4000
}

# Response
{
  "answer": "Para implementar un template MTT-DSL:\\n\\n1. ...",
  "references": [...],
  "next_steps": [...],
  "confidence": 0.89,
  "response_time_ms": 2340
}
\`\`\`

---

## 📊 PERFORMANCE TARGETS

| Fase | Métrica | Target | v1.0 Actual |
|------|---------|--------|-------------|
| **Fase 1: FBCU** | Compression Ratio | >80% | 0% (no compression) |
| | Sync Time | <3s | 0.91s |
| | Regeneration Accuracy | 100% | N/A (no posible) |
| | Storage Growth | +20% | - |
| **Fase 2: QPX** | Query Time | <100ms | <100ms (string match) |
| | Semantic Accuracy | >90% | N/A (no semantic) |
| | False Positives | <10% | ~30% (string match) |
| | Related Topics | 3-5/result | 0 |
| **Fase 3: ShuiDao+LLM** | Intention Accuracy | >90% | N/A |
| | LLM Response Time | <3s | N/A |
| | Context Relevance | >85% | N/A |
| | API Latency | <200ms | N/A |

---

## 🧪 TESTING STRATEGY

### Unit Tests

\`\`\`rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // === FASE 1: FBCU ===
    
    #[test]
    fn test_store_concept_compressed() {
        let mut librarian = BstradivariusLibrarian::new_test()?;
        let content = "# Test Doc\\n\\nContent here...".repeat(100);
        
        librarian.store_concept_compressed(
            "test_concept",
            &content,
            ConceptMetadata::default(),
        )?;
        
        let template = librarian.voxel_db.get_template_by_name("test_concept")?;
        assert!(template.compressed_content.is_some());
        
        // Verify compression ratio
        let compressed_size = template.compressed_content.as_ref().unwrap().len();
        let original_size = content.len();
        let ratio = (original_size - compressed_size) as f64 / original_size as f64;
        assert!(ratio > 0.80, "Compression ratio too low: {:.2}", ratio);
    }
    
    #[test]
    fn test_regenerate_markdown_accuracy() {
        let mut librarian = BstradivariusLibrarian::new_test()?;
        let original = "# Test\\n\\n## Section\\n\\nContent...";
        
        librarian.store_concept_compressed("test", original, ConceptMetadata::default())?;
        let regenerated = librarian.regenerate_markdown("test")?;
        
        assert_eq!(original, regenerated, "Regeneration not 100% accurate");
    }
    
    // === FASE 2: QPX ===
    
    #[tokio::test]
    async fn test_query_semantic_accuracy() {
        let librarian = BstradivariusLibrarian::new_test()?;
        
        // Query for "storage systems"
        let results = librarian.query_semantic("sistemas de almacenamiento").await?;
        
        // Should find VoxelDB, TelescopeDB, FBCU
        let names: Vec<_> = results.templates.iter()
            .map(|s| &s.template.name)
            .collect();
        
        assert!(names.iter().any(|n| n.contains("VoxelDB")));
        assert!(names.iter().any(|n| n.contains("TelescopeDB")));
        assert!(results.templates.len() >= 3);
    }
    
    #[tokio::test]
    async fn test_query_semantic_performance() {
        let librarian = BstradivariusLibrarian::new_test()?;
        
        let start = Instant::now();
        let _ = librarian.query_semantic("arquitectura").await?;
        let elapsed = start.elapsed();
        
        assert!(elapsed.as_millis() < 100, "Query too slow: {:?}", elapsed);
    }
    
    // === FASE 3: SHUIDAO + LLM ===
    
    #[tokio::test]
    async fn test_ask_learning_mode() {
        let mut librarian = BstradivariusLibrarian::new_test()?;
        
        let response = librarian.ask("¿Cómo funciona VoxelDB?").await?;
        
        match response {
            IntelligentResponse::Learning { explanation, references, .. } => {
                assert!(!explanation.is_empty());
                assert!(!references.is_empty());
                assert!(explanation.contains("VoxelDB"));
            }
            _ => panic!("Expected Learning mode"),
        }
    }
    
    #[tokio::test]
    async fn test_intention_detection_accuracy() {
        let librarian = BstradivariusLibrarian::new_test()?;
        
        // Learning intention
        let intent1 = librarian.intention_detector.detect("¿Cómo funciona X?")?;
        assert_eq!(intent1.mode, CognitiveMode::Learning);
        
        // Operational intention
        let intent2 = librarian.intention_detector.detect("necesito implementar Y")?;
        assert_eq!(intent2.mode, CognitiveMode::Operational);
        
        // Light intention
        let intent3 = librarian.intention_detector.detect("¿qué es Z?")?;
        assert_eq!(intent3.mode, CognitiveMode::Light);
    }
}
\`\`\`

### Integration Tests

\`\`\`rust
// examples/test_bstradivarius_v2_e2e.rs

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎻 BStradivarius v2.0 E2E Test\\n");
    
    // === TEST 1: Full Sync with Compression ===
    println!("TEST 1: Sync with FBCU compression...");
    let mut librarian = BstradivariusLibrarian::new("./data/bstrad_test")?;
    
    let sync_result = librarian.sync_directory("./ROADMAP_V2").await?;
    println!("  ✅ Synced {} files", sync_result.files_processed);
    println!("  ✅ Compression ratio: {:.2}%", sync_result.avg_compression_ratio * 100.0);
    
    // === TEST 2: Semantic Query ===
    println!("\\nTEST 2: Semantic query...");
    let query_result = librarian.query_semantic("diseño de bases de datos").await?;
    println!("  ✅ Found {} results", query_result.templates.len());
    for (i, scored) in query_result.templates.iter().take(3).enumerate() {
        println!("  {}. {} (score: {:.2})", 
            i + 1, 
            scored.template.name, 
            scored.score
        );
    }
    
    // === TEST 3: Regenerate Markdown ===
    println!("\\nTEST 3: Regenerate markdown...");
    let regenerated = librarian.regenerate_markdown("VoxelDB - Base de Datos Cúbica")?;
    println!("  ✅ Regenerated {} bytes", regenerated.len());
    
    // === TEST 4: LLM Ask ===
    println!("\\nTEST 4: Ask LLM question...");
    let response = librarian.ask("¿Cómo implemento un template MTT-DSL?").await?;
    match response {
        IntelligentResponse::Learning { explanation, references, next_steps } => {
            println!("  ✅ Explanation: {} chars", explanation.len());
            println!("  ✅ References: {}", references.len());
            println!("  ✅ Next steps: {}", next_steps.len());
        }
        _ => println!("  ❌ Unexpected response type"),
    }
    
    println!("\\n🎉 All tests passed!");
    Ok(())
}
\`\`\`

---

## �� MIGRATION PATH v1.0 → v2.0

### Backward Compatibility

\`\`\`rust
impl BstradivariusLibrarian {
    /// Legacy query (string matching) - backward compatible
    pub fn query(&self, pattern: &str) -> Result<Vec<ConceptMatch>> {
        // Maintain v1.0 behavior
        let pattern_lower = pattern.to_lowercase();
        
        let matches: Vec<_> = self.voxel_db
            .query_templates_by_category(&TemplateCategory::Technical)?
            .into_iter()
            .filter(|t| t.name.to_lowercase().contains(&pattern_lower))
            .map(|t| self.template_to_match(&t))
            .collect();
        
        Ok(matches)
    }
    
    /// Auto-detect: semantic vs pattern query
    pub async fn query_auto(&self, query: &str) -> Result<QueryResult> {
        // Heuristic: if query looks like natural language, use semantic
        if self.is_natural_language(query) {
            let semantic = self.query_semantic(query).await?;
            Ok(QueryResult::Semantic(semantic))
        } else {
            let pattern = self.query(query)?;
            Ok(QueryResult::Pattern(pattern))
        }
    }
}
\`\`\`

### Migration Checklist

- [ ] **Phase 1**: Add FBCU compression (non-breaking)
  - Existing v1.0 templates continue working
  - New templates stored with compression
  - Regeneration available for new templates

- [ ] **Phase 2**: Add semantic queries (additive)
  - `query` command unchanged (legacy)
  - `query-semantic` NEW command (opt-in)
  - Auto-detect mode available

- [ ] **Phase 3**: Add LLM ask (additive)
  - All v1.0 commands unchanged
  - `ask` NEW command (opt-in)
  - API NEW endpoints (no breaking changes)

---

## 📚 REFERENCES

### Code
- `src/bstradivarius/indexer.rs` - Current v1.0 implementation
- `src/fbcu/mod.rs` - FBCU engine (✅ ready)
- `src/shuidao/mod.rs` - ShuiDao engine (✅ ready)
- `src/multi_agent/hubspoke.rs` - LLM routing (✅ ready)
- `src/voxeldb/mod.rs` - VoxelDB (⏳ needs query_spatial())

### Docs
- `00_VISION/DA-035_BSTRADIVARIUS_ECOSYSTEM_LIBRARIAN.md` - Decision
- `BSTRADIVARIUS_ARCHITECTURE_ANALYSIS.md` - Analysis
- `02_COMPONENTES/03_fbcu-core.md` - FBCU specs
- `02_COMPONENTES/13_shuidao-cognitive-engine.md` - ShuiDao specs
- `01_ARQUITECTURA/15_pxlang-qpx-query-language.md` - QPX specs

---

**Documento creado**: 2025-11-30 16:30:00  
**Autor**: Sistema Bitácora v1.0  
**Status**: 🎯 ESPECIFICACIÓN COMPLETA - Ready for implementation  
**Next**: Plan implementación detallado (04_IMPLEMENTACION/)
