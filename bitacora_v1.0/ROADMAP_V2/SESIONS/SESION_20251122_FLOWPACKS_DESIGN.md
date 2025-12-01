# 🎨 SESIÓN DE DISEÑO: FlowPacks Anti-Disco-Rayado

```yaml
Fecha: 2025-11-22
Componente: FlowPacks (Contextual Compression)
Fase: Phase 1 - Design (4 horas)
Referencia: FLOWPACKS_IMPLEMENTATION_PLAN.md (44KB)
Objetivo: Diseñar arquitectura completa ANTES de codificar
```

---

## 🎯 PROBLEMA A RESOLVER

**Síntoma:** "Disco rayado" - Bitácora repite mismas explicaciones sin detectar que el usuario ya preguntó antes.

**Causa raíz:**
- FBCU comprime mensajes INDIVIDUALES (2-15x)
- NO hay detección de similitud semántica entre conversaciones
- NO hay relación entre mensajes de diferentes sesiones
- TelescopeDB almacena entries independientes

**Impacto:**
- 😞 Usuario frustrado: "Ya te lo pregunté hace 2 días"
- 💸 Tokens desperdiciados: 1000 palabras (2 explicaciones) vs 550 (1 + referencia)
- 🤖 Sensación de IA tonta: "No recuerda nada"

---

## 🏗️ ARQUITECTURA DISEÑADA

### Módulos (7 archivos)

```
src/flowpacks/
├── mod.rs              # FlowPackEngine (orquestador principal)
├── flowpack.rs         # FlowPack, FlowPackEntry, EntryType
├── similarity.rs       # SimilarityIndex (embeddings + búsqueda)
├── response.rs         # AdaptiveResponse (Reference/PartialReference/Full)
├── compression.rs      # Estrategias de compresión contextual
├── config.rs           # FlowPackConfig (umbrales, parámetros)
└── error.rs            # FlowPackError
```

### Flujo de Datos

```
Usuario: "¿Qué es CTX7D?"
    ↓
[1] FlowPackEngine.compress_message()
    ├─> FBCU.compress() → FBCUCore (15x ratio)
    ├─> SimilarityIndex.search_similar() → Vec<SimilarMatch>
    │   ├─ Genera embedding (MiniLM-L6-v2, 384 dims)
    │   └─ Busca en HNSW index (k=10, threshold=0.85)
    └─> Decisión:
        ├─ NO similar (< 0.85) → Crear nuevo FlowPack (Base)
        └─ SÍ similar (≥ 0.85) → Añadir a FlowPack existente
            ├─ Muy similar (≥ 0.95) → EntryType::Reference
            └─ Similar (0.85-0.95) → EntryType::Delta
    ↓
[2] FlowPack almacenado en TelescopeDB
    ├─ ID: "fp_session1_1732320000"
    ├─ Entries: [entry_0 (Base), entry_1 (Reference), ...]
    ├─ Centroid embedding: [0.23, -0.45, ..., 0.67]
    └─ Stats: compression_ratio = 25.3x
```

```
Usuario (2 días después): "Recuérdame CTX7D"
    ↓
[3] FlowPackEngine.generate_adaptive_response()
    ├─> SimilarityIndex.search_similar()
    │   └─> Encuentra FlowPack similar (similarity=0.96)
    ├─> Decisión basada en similarity:
    │   ├─ > 0.95 → AdaptiveResponse::Reference
    │   │   └─> "Ya te expliqué esto el 2025-11-20..."
    │   ├─ 0.85-0.95 → AdaptiveResponse::PartialReference
    │   │   └─> "Hablamos de esto antes, aquí lo nuevo..."
    │   └─ < 0.85 → AdaptiveResponse::Full
    │       └─> Explicación completa desde cero
    └─> Actualizar FlowPack (añadir referencia)
```

---

## 📐 ESTRUCTURAS DE DATOS (Diseño)

### FlowPack (contenedor de mensajes relacionados)

```rust
pub struct FlowPack {
    /// ID único: "fp_{session_id}_{timestamp}"
    pub id: String,
    
    /// ID de sesión (agrupa conversaciones)
    pub session_id: String,
    
    /// Mensajes agrupados (comprimidos)
    pub entries: Vec<FlowPackEntry>,
    
    /// Embedding centroide (promedio de todos)
    /// Dimensión: 384 (MiniLM-L6-v2)
    pub centroid_embedding: Vec<f64>,
    
    /// Timestamps
    pub first_timestamp: DateTime<Utc>,
    pub last_timestamp: DateTime<Utc>,
    
    /// Estadísticas
    pub stats: CompressionStats,
}
```

**Decisión de diseño:**
- ¿Por qué centroid embedding? → Búsqueda rápida sin recalcular todos los entries
- ¿Actualizar centroid al añadir entry? → SÍ, promedio incremental
- ¿Límite de entries? → Config: max_pack_size (default: 20)

### FlowPackEntry (mensaje individual)

```rust
pub struct FlowPackEntry {
    /// ID único: "{pack_id}_{index}"
    pub id: String,
    
    /// FBCU Core (compresión individual 15x)
    pub fbcu_core: FBCUCore,
    
    /// Timestamp del mensaje
    pub timestamp: DateTime<Utc>,
    
    /// Tipo: Base, Reference, Delta
    pub entry_type: EntryType,
    
    /// Si es Reference/Delta: ID de la base
    pub reference_to: Option<String>,
    
    /// Snapshot del CTX7D (metadata)
    pub ctx7d_snapshot: ContextToken7D,
    
    /// Texto original (para análisis)
    pub original_text: String,
}
```

**Decisión de diseño:**
- ¿Guardar original_text? → SÍ, para análisis de diferencias (extract_differences)
- ¿Guardar CTX7D completo? → SÍ, para entender contexto emocional/temporal

### EntryType (clasificación de entrada)

```rust
pub enum EntryType {
    /// Primera explicación (completa)
    Base,
    
    /// Repetición exacta (>0.95 similitud)
    Reference {
        base_entry_id: String,
    },
    
    /// Similar pero con diferencias (0.85-0.95)
    Delta {
        base_entry_id: String,
        differences: Vec<String>, // Palabras nuevas
    },
}
```

**Decisión de diseño:**
- ¿Por qué separar Reference y Delta? → Compresión diferencial (Delta guarda solo diffs)
- ¿Cómo calcular differences? → Set difference de palabras (baseline: Jaccard)

### AdaptiveResponse (respuesta inteligente)

```rust
pub enum AdaptiveResponse {
    /// Usuario pregunta EXACTAMENTE lo mismo
    Reference {
        pack_id: String,
        original_date: DateTime<Utc>,
        summary: String,
        suggestion: String,
    },
    
    /// Usuario pregunta algo SIMILAR
    PartialReference {
        pack_id: String,
        differences: Vec<String>,
        new_aspects: Vec<String>,
    },
    
    /// Usuario pregunta algo NUEVO
    Full {
        requires_new_explanation: bool,
    },
}
```

**Decisión de diseño:**
- ¿Cómo generar suggestion? → Template: "¿Quieres profundizar en [aspecto]?"
- ¿Detectar new_aspects? → NLP simple: palabras nuevas + clustering semántico

---

## 🔍 SIMILARITY INDEX (Diseño Detallado)

### Modelo de Embeddings

**Opción A: Modelo local (sentence-transformers)**
- Modelo: `all-MiniLM-L6-v2`
- Dimensión: 384
- Velocidad: ~100 sentences/sec (CPU)
- Ventaja: Local-first, sin API calls
- Desventaja: Requiere modelo descargado (~90MB)

**Opción B: API externa (OpenAI/Cohere)**
- Modelo: `text-embedding-ada-002` (OpenAI)
- Dimensión: 1536
- Velocidad: ~1000 sentences/sec (API)
- Ventaja: Mayor calidad
- Desventaja: Costo, dependencia externa

**DECISIÓN: Opción A (MiniLM-L6-v2)**
- Razón: DA-001 (Local-First Architecture)
- Trade-off: Calidad 90% vs costo $0

### HNSW Index

**Parámetros diseñados:**
```rust
HnswConfig {
    m: 16,                // Conexiones por nodo
    ef_construction: 200, // Calidad del índice
    ef_search: 50,        // Recall en búsqueda
    max_elements: 10000,  // Máximo FlowPacks
}
```

**Decisión de diseño:**
- ¿Por qué HNSW y no FAISS? → HNSW es Rust-native, FAISS necesita FFI
- ¿Rebuild index cada vez? → NO, incremental add
- ¿Persistencia del index? → SÍ, serializar a disk (bincode)

---

## 🎯 API PÚBLICA (Diseño)

### FlowPackEngine

```rust
impl FlowPackEngine {
    /// Constructor
    pub fn new(config: FlowPackConfig) -> Result<Self>;
    
    /// Comprimir mensaje con detección de contexto
    pub async fn compress_message(
        &mut self,
        message: &str,
        ctx7d: &ContextToken7D,
        session_id: &str,
    ) -> Result<FlowPackEntry>;
    
    /// Generar respuesta adaptada
    pub async fn generate_adaptive_response(
        &self,
        query: &str,
        ctx7d: &ContextToken7D,
    ) -> Result<AdaptiveResponse>;
    
    /// Buscar FlowPacks similares
    pub async fn find_similar_packs(
        &self,
        query: &str,
        ctx7d: &ContextToken7D,
        threshold: f64,
    ) -> Result<Vec<SimilarMatch>>;
    
    /// Obtener FlowPack por ID
    pub fn get_flowpack(&self, id: &str) -> Option<&FlowPack>;
    
    /// Estadísticas
    pub fn stats(&self) -> &EngineStats;
}
```

### SimilarityIndex

```rust
impl SimilarityIndex {
    /// Constructor (carga modelo embeddings)
    pub fn new(model_path: Option<PathBuf>) -> Result<Self>;
    
    /// Generar embedding
    pub async fn encode(&self, text: &str) -> Result<Vec<f64>>;
    
    /// Buscar similares (k-NN)
    pub async fn search_similar(
        &self,
        query: &str,
        ctx7d: &ContextToken7D,
        threshold: f64,
    ) -> Result<Vec<SimilarMatch>>;
    
    /// Añadir FlowPack al índice
    pub fn add_to_index(&mut self, pack_id: String, embedding: Vec<f64>) -> Result<()>;
}
```

---

## ⚙️ CONFIGURACIÓN (Diseño)

```rust
pub struct FlowPackConfig {
    // Umbrales de similitud
    pub similarity_threshold: f64,    // 0.85 (85%)
    pub exact_threshold: f64,         // 0.95 (95%)
    
    // Ventana temporal
    pub temporal_window_hours: u64,   // 72h (3 días)
    
    // Límites
    pub max_pack_size: usize,         // 20 mensajes
    pub cache_size: usize,            // 100 FlowPacks en RAM
    
    // Compresión
    pub aggressive_compression: bool, // true
    pub wavelet_level: u8,            // 6
    pub fractal_level: u8,            // 8
    
    // Embeddings
    pub embedding_model_path: Option<PathBuf>, // None = default MiniLM
    pub embedding_dimension: usize,   // 384
    
    // HNSW
    pub hnsw_k: usize,               // 10 (top-k results)
    pub hnsw_ef_construction: usize, // 200
    pub hnsw_ef_search: usize,       // 50
    pub hnsw_m: usize,               // 16
}
```

**Presets diseñados:**
- `FlowPackConfig::default()` → Balance (85% threshold, 72h window)
- `FlowPackConfig::fast()` → Velocidad (80% threshold, 48h window, HNSW reducido)
- `FlowPackConfig::high_quality()` → Calidad (90% threshold, 168h window, HNSW aumentado)

---

## 🧪 TESTS DISEÑADOS

### Test 1: Detección de repetición exacta

```rust
#[tokio::test]
async fn test_exact_repetition_detection() {
    let mut engine = FlowPackEngine::new(FlowPackConfig::default()).unwrap();
    let ctx7d = ContextToken7D::default();
    
    // Primera pregunta
    let msg1 = "¿Qué es CTX7D?";
    let entry1 = engine.compress_message(msg1, &ctx7d, "session_1").await.unwrap();
    assert!(matches!(entry1.entry_type, EntryType::Base));
    
    // Misma pregunta (debe detectar)
    let msg2 = "¿Qué es CTX7D?";
    let entry2 = engine.compress_message(msg2, &ctx7d, "session_1").await.unwrap();
    assert!(matches!(entry2.entry_type, EntryType::Reference { .. }));
    
    // Respuesta adaptada
    let response = engine.generate_adaptive_response(msg2, &ctx7d).await.unwrap();
    assert!(matches!(response, AdaptiveResponse::Reference { .. }));
}
```

### Test 2: Ratio de compresión >20x

```rust
#[tokio::test]
async fn test_compression_ratio() {
    let mut engine = FlowPackEngine::new(FlowPackConfig::default()).unwrap();
    
    // 10 mensajes similares
    for i in 0..10 {
        let msg = format!("Explicame CTX7D variación {}", i);
        engine.compress_message(&msg, &ctx7d, "session_1").await.unwrap();
    }
    
    let pack = engine.get_flowpack("fp_session_1_*").unwrap();
    let ratio = pack.compression_ratio();
    assert!(ratio > 20.0, "Ratio: {} < 20x", ratio);
}
```

### Test 3: Latencia de búsqueda <50ms

```rust
#[tokio::test]
async fn test_search_latency() {
    let engine = /* ... engine con 100 FlowPacks ... */;
    
    let start = Instant::now();
    let _ = engine.find_similar_packs("CTX7D", &ctx7d, 0.85).await.unwrap();
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 50, "Latency: {:?} > 50ms", duration);
}
```

---

## 🚨 RIESGOS IDENTIFICADOS

### Riesgo 1: Modelo embeddings no disponible
**Mitigación:** Fallback a embeddings simples (TF-IDF) si modelo no se carga

### Riesgo 2: HNSW index memory usage
**Mitigación:** LRU cache con límite configurable, persistencia a disk

### Riesgo 3: Similitud falsa positiva
**Mitigación:** Threshold ajustable, validación manual inicial

### Riesgo 4: Performance en >1000 FlowPacks
**Mitigación:** HNSW escala bien, tests de stress con 10k packs

---

## 📊 MÉTRICAS DE ÉXITO

### Técnicas
- Ratio compresión: >20x (objetivo: 20-50x)
- Latencia búsqueda: <50ms
- Detección repetición: >95% accuracy
- Memoria cache: <100MB (100 FlowPacks)

### UX
- Tokens ahorrados: >50% en repeticiones
- Respuestas adaptadas: >80% cuando aplica
- Sensación "recuerda bien": Feedback usuario positivo

---

## ✅ DECISIONES DE DISEÑO CLAVE

1. **Embeddings locales (MiniLM)** vs API externa
   - ✅ Local (DA-001)
   
2. **HNSW index** vs FAISS
   - ✅ HNSW (Rust-native, no FFI)
   
3. **Guardar original_text**
   - ✅ SÍ (análisis de diferencias)
   
4. **Centroid embedding**
   - ✅ Promedio incremental (O(1) update)
   
5. **Threshold defaults**
   - ✅ 0.85 similarity, 0.95 exact
   
6. **Temporal decay**
   - ✅ Exp decay: e^(-hours/168)
   
7. **Integration con TelescopeDB**
   - ✅ FlowPacks como BiographicalEntry especial

---

## 🚀 PRÓXIMOS PASOS

**Phase 1 (Design) - COMPLETO ✅**
- [x] Arquitectura de módulos
- [x] Estructuras de datos
- [x] API pública
- [x] Tests diseñados
- [x] Riesgos identificados

**Phase 2 (Implementation) - SIGUIENTE:**
1. Implementar módulos básicos (error.rs, config.rs)
2. Implementar estructuras (flowpack.rs)
3. Implementar similarity.rs (STUB primero)
4. Implementar mod.rs (FlowPackEngine)
5. Implementar response.rs

**Phase 3 (Validation) - DESPUÉS:**
1. Tests unitarios
2. Integration con FBCU
3. Integration con TelescopeDB
4. Performance benchmarks

---

**Estado:** ✅ DISEÑO COMPLETO  
**Duración:** ~3 horas  
**Siguiente:** Phase 2 - Core Implementation  

---

*Generado: 2025-11-22*  
*Sistema Bitácora v1.0 - FlowPacks Anti-Disco-Rayado Design*
