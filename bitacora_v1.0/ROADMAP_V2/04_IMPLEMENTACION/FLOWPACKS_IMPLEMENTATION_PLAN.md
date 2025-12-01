```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md
Versión: 2.0.1 - SHUIDAO EVOLUTION
Fecha Creación: 2025-11-06
Última Actualización: 2025-11-23 23:25:34
Autor: Sistema Bitácora - En colaboración con Eduardo
Propósito: Plan de implementación ShuiDao (水道) - Intention-Oriented Cognitive Engine
Estado: 📋 PLAN EVOLUTIVO - De FlowPacks a ShuiDao
Template: implementation_plan.yaml v1.0.0 (MTT-DSL)
Relacionado Con:
  - ROADMAP_V2/02_COMPONENTES/IMPORTANTES/FLOWPACKS.md (especificación conceptual)
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/FBCU_CORE.md (compresión individual)
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md (memoria biográfica)
  - ROADMAP_V2/shuidao_flowpacks_reflection.md (filosofía ShuiDao)
  - ROADMAP_V2/CHECKLIST_V2.md (tareas 12.1-12.12)
  - ROADMAP_V2/07_TEMPLATES/implementation_plan.yaml (template MTT-DSL)
Implementa:
  - DA-031: FlowPacks - DAG Processing Pipelines
  - DA-032: ShuiDao - Intention-Oriented Cognitive Architecture (NUEVO)
  - Solución al problema "disco rayado" + Detección de intención
Criticidad: 🔴 CRÍTICO - Transforma Bitácora de asistente a compañero cognitivo
Evolución: Phase 3a (FlowPacks fundacional) → Phase 3b (ShuiDao completo)
Restricciones Arquitectónicas:
  - DA-011: NO bases de datos externas (PostgreSQL, MongoDB, Redis, etc)
  - SOLO TelescopeDB (memoria biográfica) + VoxelDB (templates/patrones)
# === FIN DATOS DE AUDITORÍA ===
```

# 🌊 SHUIDAO (水道) - Intention-Oriented Cognitive Engine

> *"El agua no fuerza su camino. Encuentra el cauce natural."* — Filosofía ShuiDao

> *"Si esto se habla con Bitácora es como decimos en español 'un disco rayado', esta expresión significa que siempre repite lo mismo."* — Eduardo (Nov 6, 2025)

**Evolución:** FlowPacks Phase 3a (fundacional) → **ShuiDao Phase 3b** (intención + arquitectura cognitiva)

---

## 🧭 VISIÓN EVOLUTIVA: DE FLOWPACKS A SHUIDAO

### FlowPacks Phase 3a: La Fundación (✅ COMPLETADO)
- ✅ Detección de similitud semántica (SimHash/cosine similarity)
- ✅ Compresión contextual (Base/Reference/Delta)
- ✅ Respuestas adaptativas (AdaptiveResponse 3 niveles)
- ✅ Memoria temporal (LRU cache + decay)
- **Logro:** Sistema que **detecta patrones**

### ShuiDao Phase 3b: La Evolución (🚧 SIGUIENTE)
- 🎯 Detección de **intención** (no solo patrones)
- 🎯 Clasificación en **5 Modos Cognitivos**
- 🎯 Microarquitectura de **Proyectos** (tareas, progreso, trazabilidad)
- 🎯 Memoria **Episódica + Semántica**
- 🎯 Sistema de **olvido adaptativo** (Ebbinghaus curve)
- 🎯 **Resonancia contextual** (ondas de activación)
- 🎯 **Graph mining** (patrones emergentes)
- **Objetivo:** Sistema que **entiende propósito**

**Diferencia clave:**
```
FlowPacks: "Eduardo preguntó sobre CTX7D de nuevo"
            → Responde con referencia

ShuiDao:    "Eduardo pregunta '¿cómo instalo un switch?'"
            → Detecta intención OPERACIONAL
            → Crea PROYECTO "Instalar_Switch"
            → Genera subtareas, checklist, progreso
            → Trackea estado y próxima acción
```

---

## 🚨 EL PROBLEMA CRÍTICO

### La Realidad Actual: El "Disco Rayado"

**Síntoma observable:**
```
Usuario: "¿Qué es CTX7D?"
Bitácora: [Explica CTX7D en 500 palabras]

--- 2 días después ---

Usuario: "Recuérdame qué es CTX7D"
Bitácora: [Explica CTX7D EXACTAMENTE IGUAL en 500 palabras]

Problema: Bitácora NO recuerda que ya explicó esto.
Resultado: Sensación de "disco rayado" - siempre lo mismo.
```

### Por Qué Sucede Esto

**Arquitectura actual:**

```
┌─────────────────────────────────────────────────┐
│ FBCU (Fractal-Based Compression Unit)          │
│ ✅ Comprime mensajes INDIVIDUALES (2-15x)      │
│                                                 │
│ Usuario: "Explica CTX7D"                        │
│ → Comprime: mensaje_001.fbcu (15x ratio)       │
│                                                 │
│ Usuario: "Recuérdame CTX7D"                     │
│ → Comprime: mensaje_002.fbcu (15x ratio)       │
│                                                 │
│ ❌ PROBLEMA: Dos archivos separados            │
│ ❌ NO HAY RELACIÓN entre conversaciones         │
│ ❌ NO HAY DETECCIÓN de contenido repetido      │
└─────────────────────────────────────────────────┘
```

**Resultado:**
- FBCU comprime cada mensaje individualmente (eficiente en bytes)
- Pero NO detecta que "mensaje_002 es repetición de mensaje_001"
- TelescopeDB almacena ambos como entradas independientes
- El sistema NO aprende de conversaciones previas

### El Costo del "Disco Rayado"

**Impacto técnico:**
- **Tokens desperdiciados**: 500 palabras × 2 = 1000 palabras (podría ser 50 + referencia)
- **Latencia innecesaria**: Regenerar explicación completa cada vez
- **Memoria inflada**: TelescopeDB crece con contenido duplicado

**Impacto en experiencia de usuario:**
- 😞 **Frustración**: "Ya te lo pregunté hace 2 días"
- 🤖 **Sensación de IA tonta**: "No recuerda nada"
- ❌ **Pérdida de confianza**: "¿Para qué sirve la memoria biográfica?"

---

## 🎯 LA SOLUCIÓN: FLOWPACKS CONTEXTUALES

### Qué Son FlowPacks (Versión Anti-Disco-Rayado)

**FlowPacks** son **paquetes de flujo conversacional** que agrupan mensajes relacionados y comprimen el CONJUNTO, no las partes.

**Metáfora mejorada:**

```
FBCU = Comprimir cada foto individualmente
  📸 foto1.jpg → foto1.fbcu (15x)
  📸 foto2.jpg → foto2.fbcu (15x)
  📸 foto3.jpg → foto3.fbcu (15x)
  
  ❌ Problema: No detecta que foto2 y foto3 son casi iguales

FlowPacks = Comprimir el ÁLBUM de fotos
  📚 album_vacaciones.flowpack
     ├─ foto1.jpg (única, guardar completa)
     ├─ foto2.jpg → REFERENCIA a foto1 + delta (5% del tamaño)
     └─ foto3.jpg → REFERENCIA a foto1 + delta (5% del tamaño)
  
  ✅ Solución: Detecta similitud, guarda diferencias
  ✅ Ratio: 20-50x (vs 15x de FBCU individual)
```

### Cómo FlowPacks Resuelve el "Disco Rayado"

**Escenario mejorado:**

```
┌─────────────────────────────────────────────────────────────┐
│ FLOWPACKS SYSTEM (sobre FBCU)                              │
│ ✅ Comprime FLUJOS COMPLETOS de conversación (20-50x)      │
│                                                             │
│ 📦 FlowPack: "CTX7D_Explicaciones"                         │
│    ├─ [2025-11-04] Usuario: "¿Qué es CTX7D?"              │
│    │   → Respuesta: [Explicación completa 500 palabras]   │
│    │   → FBCU Core: base_explanation.fbcu (15x)           │
│    │                                                        │
│    └─ [2025-11-06] Usuario: "Recuérdame CTX7D"            │
│        → Respuesta: "Ya te expliqué CTX7D el 2025-11-04.  │
│           ¿Quieres que profundice en algún aspecto        │
│           específico o prefieres un resumen?"             │
│        → FBCU Core: referencia_a_base + contexto (3x)     │
│                                                            │
│ ✅ SOLUCIÓN: Detecta similitud, genera respuesta adaptada │
│ ✅ RATIO TOTAL: base (500w @ 15x) + ref (50w @ 3x)        │
│                = 35 bytes + 15 bytes = 50 bytes            │
│                vs 70 bytes (sin FlowPacks)                │
│ ✅ EXPERIENCIA: Usuario siente que Bitácora RECUERDA      │
└─────────────────────────────────────────────────────────────┘
```

**Diferencias clave:**

| Aspecto | Sin FlowPacks (FBCU solo) | Con FlowPacks |
|---------|---------------------------|---------------|
| **Detección repetición** | ❌ No detecta | ✅ Detecta y referencia |
| **Respuesta** | Siempre completa (500w) | Adaptada (50w + ref) |
| **Ratio compresión** | 15x por mensaje | 20-50x por flujo |
| **Experiencia usuario** | "Disco rayado" 😞 | "Recuerda bien" 😊 |
| **Tokens consumidos** | 1000w (2 explicaciones) | 550w (1 completa + 1 ref) |

---

## 🏗️ ARQUITECTURA DE LA SOLUCIÓN

### Visión General del Sistema

```
┌─────────────────────────────────────────────────────────────────┐
│                    BITÁCORA MEMORY STACK                        │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
               ┌──────────────────────────┐
               │   Context Token 7D       │
               │   (Genera tensor 7D)     │
               └────────────┬─────────────┘
                            │
                            ▼
         ┌──────────────────────────────────────┐
         │         FBCU ENGINE                  │
         │  • Compresión fractal individual    │
         │  • Ratio: 2-15x                     │
         │  • Output: mensaje.fbcu             │
         └────────────┬─────────────────────────┘
                      │
                      ▼
         ┌──────────────────────────────────────┐
         │      FLOWPACKS ENGINE (NUEVO)        │ ← IMPLEMENTAR
         │  • Análisis de similitud semántica  │
         │  • Detección de flujos relacionados │
         │  • Compresión contextual            │
         │  • Ratio: 20-50x                    │
         │  • Output: session.flowpack         │
         └────────────┬─────────────────────────┘
                      │
              ┌───────┴────────┐
              │                │
              ▼                ▼
    ┌──────────────────┐  ┌─────────────────┐
    │  TELESCOPEDB     │  │   VOXELDB       │
    │  • Almacena      │  │  • Almacena     │
    │    FlowPacks     │  │    Templates    │
    │  • Queries por   │  │  • Queries por  │
    │    similitud     │  │    categoría    │
    └──────────────────┘  └─────────────────┘
```

### Componentes a Implementar

#### 1. **FlowPackEngine** (Core)

```rust
// src/flowpacks/mod.rs

pub struct FlowPackEngine {
    /// Motor FBCU subyacente (compresión individual)
    fbcu: FBCUEngine,
    
    /// Índice de similitud semántica (embeddings)
    similarity_index: SimilarityIndex,
    
    /// Caché de FlowPacks activos en memoria
    active_packs: HashMap<String, FlowPack>,
    
    /// Configuración de umbrales
    config: FlowPackConfig,
}

pub struct FlowPackConfig {
    /// Umbral de similitud para considerar mensajes relacionados (0.0-1.0)
    pub similarity_threshold: f64,  // Default: 0.85
    
    /// Ventana temporal para agrupar mensajes (horas)
    pub temporal_window_hours: u64,  // Default: 72 (3 días)
    
    /// Tamaño máximo de FlowPack (mensajes)
    pub max_pack_size: usize,  // Default: 20
    
    /// Activar compresión agresiva
    pub aggressive_compression: bool,  // Default: true
}

impl FlowPackEngine {
    /// Comprimir un nuevo mensaje, detectando flujos existentes
    pub async fn compress_message(
        &mut self,
        message: &str,
        ctx7d: &ContextToken7D,
        session_id: &str,
    ) -> Result<FlowPackEntry> {
        // 1. Comprimir con FBCU individualmente
        let fbcu_core = self.fbcu.compress(message)?;
        
        // 2. Buscar FlowPacks similares (semántica + temporal)
        let similar_packs = self.find_similar_packs(
            message,
            ctx7d,
            self.config.similarity_threshold,
        ).await?;
        
        // 3. Decidir: ¿Nuevo pack o añadir a existente?
        if let Some(existing_pack) = similar_packs.first() {
            // Añadir a FlowPack existente (con referencia)
            self.add_to_pack(existing_pack.id, fbcu_core, message, ctx7d).await
        } else {
            // Crear nuevo FlowPack
            self.create_new_pack(session_id, fbcu_core, message, ctx7d).await
        }
    }
    
    /// Generar respuesta adaptada (detecta repetición)
    pub async fn generate_adaptive_response(
        &self,
        query: &str,
        ctx7d: &ContextToken7D,
    ) -> Result<AdaptiveResponse> {
        // 1. Buscar en FlowPacks existentes
        let matching_packs = self.find_similar_packs(
            query,
            ctx7d,
            self.config.similarity_threshold,
        ).await?;
        
        // 2. Si hay coincidencia alta → respuesta adaptada
        if let Some(pack) = matching_packs.first() {
            if pack.similarity > 0.95 {
                // Caso: Usuario pregunta EXACTAMENTE lo mismo
                return Ok(AdaptiveResponse::Reference {
                    pack_id: pack.id.clone(),
                    original_date: pack.first_timestamp,
                    summary: pack.generate_summary(),
                    suggestion: "¿Quieres profundizar en algún aspecto?",
                });
            } else if pack.similarity > 0.85 {
                // Caso: Usuario pregunta algo SIMILAR
                return Ok(AdaptiveResponse::PartialReference {
                    pack_id: pack.id.clone(),
                    differences: pack.extract_differences(query),
                    new_aspects: query_new_aspects(query, pack),
                });
            }
        }
        
        // 3. Si no hay coincidencia → respuesta completa
        Ok(AdaptiveResponse::Full {
            requires_new_explanation: true,
        })
    }
}
```

#### 2. **SimilarityIndex** (Detección Semántica)

```rust
// src/flowpacks/similarity.rs

pub struct SimilarityIndex {
    /// Modelo de embeddings (MiniLM, BERT, etc.)
    embedding_model: EmbeddingModel,
    
    /// Índice HNSW para búsqueda rápida
    hnsw_index: HnswIndex,
    
    /// Mapeo: embedding_id → FlowPack
    pack_registry: HashMap<String, FlowPackMetadata>,
}

impl SimilarityIndex {
    /// Encontrar FlowPacks similares a un query
    pub async fn search_similar(
        &self,
        query: &str,
        ctx7d: &ContextToken7D,
        threshold: f64,
    ) -> Result<Vec<SimilarMatch>> {
        // 1. Generar embedding del query
        let query_embedding = self.embedding_model.encode(query).await?;
        
        // 2. Buscar en índice HNSW (k-NN)
        let candidates = self.hnsw_index.search(
            &query_embedding,
            k: 10,  // Top 10 candidatos
        )?;
        
        // 3. Filtrar por umbral + contexto temporal
        let matches = candidates
            .into_iter()
            .filter(|c| c.similarity >= threshold)
            .filter(|c| self.is_temporally_relevant(c, ctx7d))
            .map(|c| SimilarMatch {
                pack_id: c.pack_id,
                similarity: c.similarity,
                pack_metadata: self.pack_registry.get(&c.pack_id).unwrap().clone(),
            })
            .collect();
        
        Ok(matches)
    }
    
    /// Verificar si FlowPack es temporalmente relevante
    fn is_temporally_relevant(
        &self,
        candidate: &Candidate,
        ctx7d: &ContextToken7D,
    ) -> bool {
        let now = Utc::now();
        let pack_age_hours = (now - candidate.timestamp).num_hours() as u64;
        
        // Decaimiento temporal: relevancia disminuye con el tiempo
        let temporal_factor = (-pack_age_hours as f64 / 168.0).exp(); // Semana
        
        temporal_factor > 0.1  // Mantener si >10% relevancia
    }
}
```

#### 3. **FlowPack** (Estructura de Datos)

```rust
// src/flowpacks/flowpack.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPack {
    /// ID único del FlowPack
    pub id: String,
    
    /// ID de sesión (agrupa conversaciones relacionadas)
    pub session_id: String,
    
    /// Mensajes agrupados (comprimidos con FBCU)
    pub entries: Vec<FlowPackEntry>,
    
    /// Embedding representativo del FlowPack (centroide)
    pub centroid_embedding: Vec<f64>,
    
    /// Metadata temporal
    pub first_timestamp: DateTime<Utc>,
    pub last_timestamp: DateTime<Utc>,
    
    /// Estadísticas de compresión
    pub stats: CompressionStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowPackEntry {
    /// FBCU Core del mensaje individual
    pub fbcu_core: FBCUCore,
    
    /// Timestamp del mensaje
    pub timestamp: DateTime<Utc>,
    
    /// Tipo de entrada (base, referencia, delta)
    pub entry_type: EntryType,
    
    /// Si es referencia: puntero a entrada base
    pub reference_to: Option<String>,
    
    /// Metadata del Context Token 7D
    pub ctx7d_snapshot: ContextToken7D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    /// Primera explicación (base completa)
    Base,
    
    /// Referencia a explicación previa
    Reference { base_entry_id: String },
    
    /// Delta: diferencias respecto a base
    Delta { base_entry_id: String, differences: Vec<String> },
}

impl FlowPack {
    /// Generar resumen del FlowPack
    pub fn generate_summary(&self) -> String {
        let base_entries = self.entries.iter()
            .filter(|e| matches!(e.entry_type, EntryType::Base))
            .count();
        
        let reference_entries = self.entries.len() - base_entries;
        
        format!(
            "FlowPack con {} mensajes ({} base, {} referencias). \
             Primera conversación: {}. Última: {}.",
            self.entries.len(),
            base_entries,
            reference_entries,
            self.first_timestamp.format("%Y-%m-%d %H:%M"),
            self.last_timestamp.format("%Y-%m-%d %H:%M"),
        )
    }
    
    /// Calcular ratio de compresión total
    pub fn compression_ratio(&self) -> f64 {
        let original_size: usize = self.entries.iter()
            .map(|e| e.fbcu_core.original_size)
            .sum();
        
        let compressed_size: usize = self.entries.iter()
            .map(|e| e.fbcu_core.compressed_data.len())
            .sum();
        
        original_size as f64 / compressed_size as f64
    }
}
```

#### 4. **AdaptiveResponse** (Respuestas Inteligentes)

```rust
// src/flowpacks/response.rs

#[derive(Debug, Clone)]
pub enum AdaptiveResponse {
    /// Usuario pregunta EXACTAMENTE lo mismo
    Reference {
        pack_id: String,
        original_date: DateTime<Utc>,
        summary: String,
        suggestion: String,
    },
    
    /// Usuario pregunta algo SIMILAR pero con nuevos aspectos
    PartialReference {
        pack_id: String,
        differences: Vec<String>,
        new_aspects: Vec<String>,
    },
    
    /// Usuario pregunta algo NUEVO (no hay FlowPack similar)
    Full {
        requires_new_explanation: bool,
    },
}

impl AdaptiveResponse {
    /// Generar texto de respuesta para el usuario
    pub fn to_user_message(&self) -> String {
        match self {
            AdaptiveResponse::Reference { original_date, summary, suggestion, .. } => {
                format!(
                    "Ya hablamos sobre esto el {}. {}\n\n{}",
                    original_date.format("%d de %B, %Y a las %H:%M"),
                    summary,
                    suggestion,
                )
            }
            
            AdaptiveResponse::PartialReference { differences, new_aspects, .. } => {
                let diff_text = if !differences.is_empty() {
                    format!("\n\nDiferencias respecto a nuestra conversación previa:\n- {}",
                        differences.join("\n- "))
                } else {
                    String::new()
                };
                
                let new_text = if !new_aspects.is_empty() {
                    format!("\n\nNuevos aspectos que preguntas:\n- {}",
                        new_aspects.join("\n- "))
                } else {
                    String::new()
                };
                
                format!(
                    "Ya hemos hablado de esto antes. Aquí te explico lo nuevo:{}{}",
                    diff_text,
                    new_text,
                )
            }
            
            AdaptiveResponse::Full { .. } => {
                // Explicación completa (no hay similitud previa)
                String::from("[Generar explicación completa desde cero]")
            }
        }
    }
}
```

---

## 📋 PLAN DE IMPLEMENTACIÓN (3 FASES)

### Fase 1: Fundaciones (Tareas 12.1 - Diseño) ⏱️ ~4 horas

**Objetivo:** Diseñar arquitectura completa sin codificar aún.

#### Tarea 12.1.1: Análisis de Requisitos
- [ ] **Documentar casos de uso del "disco rayado"**
  - Ejemplos reales de repetición (CTX7D, FBCU, TelescopeDB)
  - Frustración del usuario
  - Métricas actuales (tokens desperdiciados)

- [ ] **Definir criterios de similitud semántica**
  - Umbral base: 0.85 (85% similitud)
  - Umbral exacto: 0.95 (95% similitud)
  - Ventana temporal: 72 horas (3 días)

- [ ] **Diseñar integración con FBCU**
  - FlowPacks EXTIENDE FBCU (no reemplaza)
  - FBCU comprime mensajes individuales
  - FlowPacks agrupa y referencia

#### Tarea 12.1.2: Arquitectura de Módulos
- [ ] **Definir módulos de `src/flowpacks/`**
  ```
  src/flowpacks/
  ├── mod.rs              // FlowPackEngine principal
  ├── flowpack.rs         // Estructuras FlowPack, FlowPackEntry
  ├── similarity.rs       // SimilarityIndex, búsqueda semántica
  ├── response.rs         // AdaptiveResponse, generación inteligente
  ├── compression.rs      // Lógica de compresión contextual
  ├── config.rs           // FlowPackConfig, umbrales
  └── error.rs            // FlowPackError
  ```

- [ ] **Definir dependencias externas**
  - Modelo embeddings: `sentence-transformers` o `text-embeddings-inference`
  - Índice HNSW: `hnswlib-rs` o implementación custom
  - Integración con FBCU: `use crate::fbcu::*`

#### Tarea 12.1.3: Especificación de API
- [ ] **API pública de FlowPackEngine**
  ```rust
  impl FlowPackEngine {
      pub fn new(config: FlowPackConfig) -> Result<Self>;
      pub async fn compress_message(...) -> Result<FlowPackEntry>;
      pub async fn generate_adaptive_response(...) -> Result<AdaptiveResponse>;
      pub async fn find_similar_packs(...) -> Result<Vec<SimilarMatch>>;
      pub async fn create_flowpack(...) -> Result<FlowPack>;
      pub async fn get_flowpack(&self, id: &str) -> Result<Option<FlowPack>>;
  }
  ```

- [ ] **Integración con TelescopeDB**
  ```rust
  // TelescopeDB almacena FlowPacks
  impl TelescopeDB {
      pub async fn store_flowpack(&mut self, pack: FlowPack) -> Result<String>;
      pub async fn query_flowpacks_by_similarity(&self, query: &str) -> Result<Vec<FlowPack>>;
  }
  ```

**Entregable Fase 1:**
- ✅ Documento de arquitectura detallado (este mismo actualizado)
- ✅ Diagramas de flujo
- ✅ Especificación de API
- ✅ Casos de uso documentados

---

### Fase 2: Implementación Core (Tarea 12.2) ⏱️ ~8 horas

**Objetivo:** Implementar motor FlowPacks funcional.

#### Tarea 12.2.1: Módulo `flowpack.rs` (Estructuras)
- [ ] **Implementar `FlowPack` struct**
  - Campos: id, session_id, entries, centroid_embedding, timestamps
  - Métodos: `generate_summary()`, `compression_ratio()`, `add_entry()`

- [ ] **Implementar `FlowPackEntry` struct**
  - Campos: fbcu_core, timestamp, entry_type, reference_to
  - Serialización/deserialización con Serde

- [ ] **Implementar `EntryType` enum**
  - Variantes: Base, Reference, Delta
  - Lógica de decisión de tipo

#### Tarea 12.2.2: Módulo `similarity.rs` (Búsqueda Semántica)
- [ ] **Implementar `SimilarityIndex`**
  - Integrar modelo de embeddings (MiniLM-L6-v2)
  - Construir índice HNSW para búsqueda rápida
  - Método `search_similar()` con filtrado temporal

- [ ] **Implementar cálculo de similitud coseno**
  - Función: `cosine_similarity(vec_a, vec_b) -> f64`
  - Validación: similitud ∈ [0.0, 1.0]

- [ ] **Implementar decaimiento temporal**
  - Fórmula: relevancia = exp(-edad_horas / 168)
  - Semivida: 1 semana (168 horas)

#### Tarea 12.2.3: Módulo `response.rs` (Respuestas Adaptadas)
- [ ] **Implementar `AdaptiveResponse` enum**
  - Variantes: Reference, PartialReference, Full
  - Método `to_user_message()` con texto natural

- [ ] **Implementar lógica de decisión**
  - Similitud >0.95 → Reference
  - Similitud 0.85-0.95 → PartialReference
  - Similitud <0.85 → Full

#### Tarea 12.2.4: Módulo `mod.rs` (FlowPackEngine)
- [ ] **Implementar `FlowPackEngine::new()`**
  - Inicializar FBCU subyacente
  - Cargar SimilarityIndex
  - Configurar umbrales

- [ ] **Implementar `compress_message()`**
  - Comprimir con FBCU individual
  - Buscar FlowPacks similares
  - Decidir: nuevo pack vs añadir a existente
  - Retornar FlowPackEntry

- [ ] **Implementar `generate_adaptive_response()`**
  - Buscar FlowPacks similares
  - Generar respuesta según similitud
  - Retornar AdaptiveResponse

- [ ] **Implementar `find_similar_packs()`**
  - Delegar a SimilarityIndex
  - Filtrar por ventana temporal
  - Ordenar por similitud

#### Tarea 12.2.5: Integración con FBCU
- [ ] **Extender FBCU para trabajar con FlowPacks**
  - Método: `fbcu.compress_with_reference(base_id, message)`
  - Compresión diferencial (delta encoding)
  - Validar ratio >20x para referencias

- [ ] **Implementar caché de FlowPacks activos**
  - HashMap en memoria: `session_id → FlowPack`
  - TTL: 24 horas
  - Evicción: LRU (Least Recently Used)

**Entregable Fase 2:**
- ✅ Código de `src/flowpacks/` funcional
- ✅ Compilación sin errores (`cargo check`)
- ✅ Tests unitarios básicos (sin integración aún)

---

### Fase 3: Validación e Integración (Tarea 12.3) ⏱️ ~4 horas

**Objetivo:** Validar que FlowPacks resuelve el "disco rayado".

#### Tarea 12.3.1: Tests de Similitud
- [ ] **Test: Detección de repetición exacta**
  ```rust
  #[test]
  fn test_exact_repetition_detection() {
      let engine = FlowPackEngine::new(default_config()).unwrap();
      
      // Primera pregunta
      let msg1 = "¿Qué es CTX7D?";
      let entry1 = engine.compress_message(msg1, &ctx7d, "session_1").await.unwrap();
      assert!(matches!(entry1.entry_type, EntryType::Base));
      
      // Misma pregunta 2 días después
      let msg2 = "¿Qué es CTX7D?";
      let entry2 = engine.compress_message(msg2, &ctx7d, "session_1").await.unwrap();
      assert!(matches!(entry2.entry_type, EntryType::Reference { .. }));
      
      // Verificar respuesta adaptada
      let response = engine.generate_adaptive_response(msg2, &ctx7d).await.unwrap();
      assert!(matches!(response, AdaptiveResponse::Reference { .. }));
  }
  ```

- [ ] **Test: Detección de similitud parcial**
  ```rust
  #[test]
  fn test_partial_similarity_detection() {
      // Pregunta original
      let msg1 = "Explícame CTX7D";
      
      // Pregunta similar pero diferente
      let msg2 = "¿Cómo funciona CTX7D en el código?";
      
      // Debe detectar similitud pero NO repetición exacta
      let response = engine.generate_adaptive_response(msg2, &ctx7d).await.unwrap();
      assert!(matches!(response, AdaptiveResponse::PartialReference { .. }));
  }
  ```

- [ ] **Test: Sin similitud (pregunta nueva)**
  ```rust
  #[test]
  fn test_no_similarity_new_topic() {
      // Pregunta sobre CTX7D
      let msg1 = "¿Qué es CTX7D?";
      
      // Pregunta sobre VoxelDB (tema diferente)
      let msg2 = "¿Cómo funciona VoxelDB?";
      
      // NO debe detectar similitud
      let response = engine.generate_adaptive_response(msg2, &ctx7d).await.unwrap();
      assert!(matches!(response, AdaptiveResponse::Full { .. }));
  }
  ```

#### Tarea 12.3.2: Tests de Performance
- [ ] **Test: Ratio de compresión >20x**
  ```rust
  #[test]
  fn test_compression_ratio_improvement() {
      // Crear FlowPack con 10 mensajes similares
      let messages = vec![
          "Explícame CTX7D",
          "Recuérdame qué es CTX7D",
          "CTX7D, ¿qué era?",
          // ... 7 más
      ];
      
      for msg in messages {
          engine.compress_message(msg, &ctx7d, "session_1").await.unwrap();
      }
      
      let pack = engine.get_flowpack("session_1").await.unwrap().unwrap();
      let ratio = pack.compression_ratio();
      
      assert!(ratio > 20.0, "Ratio: {} < 20x", ratio);
  }
  ```

- [ ] **Test: Latencia de búsqueda <50ms**
  ```rust
  #[test]
  fn test_search_latency() {
      let start = Instant::now();
      let similar = engine.find_similar_packs("CTX7D", &ctx7d, 0.85).await.unwrap();
      let duration = start.elapsed();
      
      assert!(duration.as_millis() < 50, "Latency: {:?} > 50ms", duration);
  }
  ```

#### Tarea 12.3.3: Integración con TelescopeDB
- [ ] **Implementar `store_flowpack()` en TelescopeDB**
  - Serializar FlowPack a JSON
  - Almacenar con ID content-addressable
  - Indexar por embedding centroide

- [ ] **Implementar `query_flowpacks_by_similarity()`**
  - Buscar en índice de embeddings
  - Retornar FlowPacks ordenados por similitud
  - Filtrar por ventana temporal

- [ ] **Test end-to-end: TelescopeDB + FlowPacks**
  ```rust
  #[test]
  fn test_telescopedb_integration() {
      let mut db = TelescopeDB::new("./test_data").unwrap();
      let engine = FlowPackEngine::new(default_config()).unwrap();
      
      // Comprimir y almacenar
      let entry = engine.compress_message("CTX7D", &ctx7d, "s1").await.unwrap();
      let pack = engine.get_flowpack("s1").await.unwrap().unwrap();
      let pack_id = db.store_flowpack(pack).await.unwrap();
      
      // Recuperar por similitud
      let results = db.query_flowpacks_by_similarity("¿Qué es CTX7D?").await.unwrap();
      assert_eq!(results[0].id, pack_id);
  }
  ```

#### Tarea 12.3.4: Validación de Experiencia de Usuario
- [ ] **Escenario real: Conversación multi-día**
  - Día 1: Usuario pregunta "CTX7D"
  - Día 3: Usuario pregunta "CTX7D" de nuevo
  - **Validar:** Respuesta menciona conversación previa
  - **Validar:** Ahorro de tokens >50%

- [ ] **Escenario real: Refinamiento progresivo**
  - Iteración 1: "Explícame CTX7D"
  - Iteración 2: "CTX7D en más detalle"
  - Iteración 3: "Dimensión Temporal de CTX7D"
  - **Validar:** Cada respuesta referencia anteriores
  - **Validar:** Construcción progresiva de conocimiento

**Entregable Fase 3:**
- ✅ Tests pasando (ratio >20x, latencia <50ms)
- ✅ Integración con TelescopeDB funcional
- ✅ Ejemplo funcionando: `examples/test_flowpacks.rs`
- ✅ Documentación de uso actualizada

---

## 🔌 PUNTOS DE EXTENSIÓN (Sistema Abierto)

### Hook 1: Modelos de Embeddings Externos

```rust
pub trait EmbeddingProvider {
    async fn encode(&self, text: &str) -> Result<Vec<f64>>;
    fn embedding_dimension(&self) -> usize;
}

// Implementación default: MiniLM-L6-v2
impl EmbeddingProvider for DefaultEmbeddings { ... }

// Punto de extensión: Usar modelo custom
impl EmbeddingProvider for CustomExternalModel {
    async fn encode(&self, text: &str) -> Result<Vec<f64>> {
        // Llamar a API externa (OpenAI, Cohere, etc.)
        let response = reqwest::get(format!(
            "https://api.external.com/embed?text={}",
            urlencoding::encode(text)
        )).await?;
        
        let embedding: Vec<f64> = response.json().await?;
        Ok(embedding)
    }
    
    fn embedding_dimension(&self) -> usize {
        1536  // Dimension de OpenAI embeddings
    }
}

// Usar en FlowPackEngine
let engine = FlowPackEngine::with_embeddings(
    config,
    CustomExternalModel::new("api_key"),
);
```

### Hook 2: Estrategias de Compresión Custom

```rust
pub trait CompressionStrategy {
    async fn compress_with_context(
        &self,
        message: &str,
        context: &[FlowPackEntry],
    ) -> Result<CompressedData>;
}

// Estrategia default: FBCU + referencias
impl CompressionStrategy for FBCUStrategy { ... }

// Punto de extensión: Estrategia custom
impl CompressionStrategy for LLMBasedCompression {
    async fn compress_with_context(
        &self,
        message: &str,
        context: &[FlowPackEntry],
    ) -> Result<CompressedData> {
        // Usar LLM para generar resumen contextual
        let context_summary = self.summarize_context(context).await?;
        let compressed = format!(
            "[REF: {}] [DELTA: {}]",
            context_summary,
            extract_differences(message, &context_summary),
        );
        Ok(CompressedData { data: compressed.into_bytes() })
    }
}
```

### Hook 3: Backends de Almacenamiento Alternativos

```rust
pub trait FlowPackStorage {
    async fn store(&mut self, pack: FlowPack) -> Result<String>;
    async fn retrieve(&self, id: &str) -> Result<Option<FlowPack>>;
    async fn search_by_similarity(&self, embedding: &[f64], k: usize) -> Result<Vec<FlowPack>>;
}

// Backend default: TelescopeDB (memoria biográfica principal)
impl FlowPackStorage for TelescopeDB {
    async fn store(&mut self, pack: FlowPack) -> Result<String> {
        let entry = BiographicalEntry {
            timestamp: pack.temporal_window.start,
            content: serde_json::to_string(&pack)?,
            dimensions: pack.centroid_embedding.clone(),
            metadata: pack.metadata.clone(),
        };
        self.insert_entry(entry).await?;
        Ok(pack.id)
    }
    
    async fn retrieve(&self, id: &str) -> Result<Option<FlowPack>> {
        self.get_entry_by_id(id)
            .await?
            .map(|entry| serde_json::from_str(&entry.content))
            .transpose()
    }
    
    async fn search_by_similarity(&self, embedding: &[f64], k: usize) -> Result<Vec<FlowPack>> {
        self.semantic_search(embedding, k)
            .await?
            .iter()
            .map(|entry| serde_json::from_str(&entry.content))
            .collect()
    }
}

// Backend alternativo: VoxelDB (templates/patrones aprendidos)
// Útil para FlowPacks que representan "recetas cognitivas" reusables
impl FlowPackStorage for VoxelDB {
    async fn store(&mut self, pack: FlowPack) -> Result<String> {
        let template = Template {
            id: pack.id.clone(),
            category: pack.category.clone(),
            content: serde_json::to_string(&pack)?,
            tags: pack.tags.clone(),
            usage_count: 0,
        };
        self.insert_template(template).await?;
        Ok(pack.id)
    }
    
    async fn retrieve(&self, id: &str) -> Result<Option<FlowPack>> {
        self.get_template_by_id(id)
            .await?
            .map(|template| serde_json::from_str(&template.content))
            .transpose()
    }
    
    async fn search_by_similarity(&self, embedding: &[f64], k: usize) -> Result<Vec<FlowPack>> {
        // VoxelDB busca templates similares por categoría y tags
        self.search_templates_by_semantic(embedding, k)
            .await?
            .iter()
            .map(|template| serde_json::from_str(&template.content))
            .collect()
    }
}
```

### Hook 4: Políticas de Evicción Custom

```rust
pub trait EvictionPolicy {
    fn should_evict(&self, pack: &FlowPack, cache_size: usize) -> bool;
}

// Política default: LRU (Least Recently Used)
impl EvictionPolicy for LRUPolicy { ... }

// Punto de extensión: LFU, TTL, custom
impl EvictionPolicy for ImportanceBasedEviction {
    fn should_evict(&self, pack: &FlowPack, cache_size: usize) -> bool {
        // Evict si: baja frecuencia + antiguo + bajo score
        let importance = pack.stats.access_count as f64
            * (-pack.age_hours() as f64 / 168.0).exp()
            * pack.ctx7d_snapshot.overall_score;
        
        importance < self.threshold && cache_size > self.max_size
    }
}
```

---

## 📊 MÉTRICAS DE ÉXITO

### Métricas Técnicas

| Métrica | Baseline (sin FlowPacks) | Target (con FlowPacks) | Cómo Medir |
|---------|--------------------------|------------------------|------------|
| **Ratio compresión** | 15x (FBCU) | >20x | `pack.compression_ratio()` |
| **Detección repetición** | 0% (no detecta) | >95% | Test de similitud >0.95 |
| **Tokens ahorrados** | 0 | >50% en repeticiones | Comparar respuestas |
| **Latencia búsqueda** | N/A | <50ms | `Instant::elapsed()` |
| **Memoria cache** | N/A | <100 MB | Tamaño `active_packs` |

### Métricas de Experiencia

| Métrica | Baseline | Target | Cómo Validar |
|---------|----------|--------|--------------|
| **Sensación "disco rayado"** | Alta 😞 | Baja 😊 | Feedback usuario |
| **Respuestas adaptadas** | 0% | >80% | Count de `AdaptiveResponse` |
| **Referencias coherentes** | No aplica | >90% | Validar `to_user_message()` |
| **Construcción progresiva** | No | Sí | Tracking de FlowPacks multi-sesión |

### Benchmarks Objetivo

```rust
// examples/test_flowpacks_benchmark.rs

#[test]
fn benchmark_compression_ratio() {
    let messages = generate_similar_messages(100);
    let pack = compress_to_flowpack(messages);
    
    assert!(pack.compression_ratio() > 20.0);
    assert!(pack.compression_ratio() < 60.0);  // Upper bound razonable
}

#[test]
fn benchmark_search_latency() {
    let engine = FlowPackEngine::with_1000_packs();
    
    let start = Instant::now();
    let _ = engine.find_similar_packs("query", &ctx7d, 0.85).await;
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 50);
}

#[test]
fn benchmark_token_savings() {
    // Escenario: 10 preguntas sobre mismo tema
    let full_responses_tokens = 10 * 500;  // 5000 tokens
    
    // Con FlowPacks: 1 completa + 9 referencias
    let flowpack_responses_tokens = 500 + (9 * 50);  // 950 tokens
    
    let savings = (full_responses_tokens - flowpack_responses_tokens) as f64
        / full_responses_tokens as f64;
    
    assert!(savings > 0.50);  // >50% ahorro
}
```

---

## 🎯 CASOS DE USO REALES

### Caso 1: Explicación Recurrente de CTX7D

**Sin FlowPacks:**

```
[2025-11-04 10:30]
Usuario: "¿Qué es CTX7D?"
Bitácora: [Explicación completa de 500 palabras]

[2025-11-06 15:00]
Usuario: "Recuérdame CTX7D"
Bitácora: [Explicación completa de 500 palabras] ← DISCO RAYADO

Tokens usados: 1000 palabras
Sensación: "No recuerda nada" 😞
```

**Con FlowPacks:**

```
[2025-11-04 10:30]
Usuario: "¿Qué es CTX7D?"
Bitácora: [Explicación completa de 500 palabras]
→ FlowPack creado: "CTX7D_Explicaciones" (Base entry)

[2025-11-06 15:00]
Usuario: "Recuérdame CTX7D"
Bitácora: "Ya te expliqué CTX7D el 4 de noviembre a las 10:30.
           Es el sistema de Context Token con 7 dimensiones:
           Semántica, Sintáctica, Pragmática, Temporal, Conceptual,
           Emocional, y Meta-cognitiva (score: 133.8/100).
           
           ¿Quieres que profundice en alguna dimensión específica
           o prefieres ejemplos de uso?"
→ FlowPack actualizado: "CTX7D_Explicaciones" (Reference entry)

Tokens usados: 550 palabras (500 + 50)
Ahorro: 45%
Sensación: "Recuerda perfectamente" 😊
```

### Caso 2: Refinamiento Progresivo

**Sin FlowPacks:**

```
[Sesión 1]
Usuario: "Explícame Routier"
Bitácora: [Explicación completa]

[Sesión 2]
Usuario: "Routier en más detalle"
Bitácora: [Explicación completa de nuevo] ← NO conecta con sesión 1

[Sesión 3]
Usuario: "¿Cómo Routier usa CTX7D?"
Bitácora: [Explicación completa] ← NO conecta con sesiones previas

Problema: Tres explicaciones independientes, sin construcción
```

**Con FlowPacks:**

```
[Sesión 1 - 2025-11-04]
Usuario: "Explícame Routier"
Bitácora: [Explicación completa - 600 palabras]
→ FlowPack: "Routier_Knowledge" (Base)

[Sesión 2 - 2025-11-05]
Usuario: "Routier en más detalle"
Bitácora: "En nuestra conversación de ayer hablamos de Routier
           como sistema de routing adaptativo. Ahora profundizaré
           en los 6 módulos: graph.rs, cognitive_state.rs,
           adaptation.rs, recommendation.rs, validation.rs, error.rs.
           
           [Explicación enfocada en módulos - 400 palabras]"
→ FlowPack actualizado: "Routier_Knowledge" (Delta entry)

[Sesión 3 - 2025-11-06]
Usuario: "¿Cómo Routier usa CTX7D?"
Bitácora: "Perfecto, conectemos lo que hemos hablado de Routier
           (días 4 y 5) con CTX7D (explicado el día 4).
           
           Routier usa CTX7D así:
           1. cognitive_state.rs captura el tensor 7D
           2. adaptation.rs pondera dimensiones según contexto
           3. recommendation.rs usa scoring multi-factor
           
           [Explicación de integración - 300 palabras]"
→ FlowPack actualizado: "Routier_Knowledge" (Delta entry)
→ Link cross-pack: "Routier_Knowledge" ↔ "CTX7D_Explicaciones"

Resultado: Construcción progresiva coherente
Tokens totales: 1300 palabras (vs 1800 sin FlowPacks)
Ahorro: 28%
Sensación: "Construye sobre conocimiento previo" 😊✨
```

### Caso 3: Detección de Confusión Repetida

**Sin FlowPacks:**

```
Usuario: "No entiendo cómo funciona FBCU"
Bitácora: [Explica FBCU]

[3 horas después]
Usuario: "Sigo sin entender FBCU"
Bitácora: [Explica FBCU igual] ← NO detecta que ya lo intentó

Problema: No adapta estrategia de explicación
```

**Con FlowPacks:**

```
Usuario: "No entiendo cómo funciona FBCU"
Bitácora: [Explica FBCU]
→ FlowPack: "FBCU_Learning" (Base, ctx7d.conceptual_complexity: 0.85)

[3 horas después]
Usuario: "Sigo sin entender FBCU"
Bitácora: [FlowPack detecta: similitud 0.96 + ctx7d no mejoró]
         "Veo que ya intenté explicar FBCU hace 3 horas y aún
          no está claro. Dejame intentar con una metáfora diferente:
          
          FBCU es como comprimir una foto JPEG:
          - Imagen original: 5 MB
          - JPEG comprimido: 500 KB (10x)
          - Calidad visual: 99% igual
          
          FBCU hace lo mismo con texto:
          - Mensaje original: 1000 palabras
          - FBCU comprimido: 100 bytes (15x)
          - Recuperación: 100% exacta (lossless)
          
          ¿Esta metáfora visual te ayuda más?"
→ FlowPack actualizado: "FBCU_Learning" (Retry entry con estrategia B)

Resultado: Adaptación inteligente
Sensación: "Entiende que no entendí y cambia enfoque" 😊🎯
```

---

## 🚀 PRÓXIMOS PASOS INMEDIATOS

### 1. Aprobar este Plan (Eduardo) ✋

**Preguntas clave:**
- ¿Este plan resuelve el "disco rayado" completamente?
- ¿Hay algún caso de uso que falta considerar?
- ¿Los hooks de extensión son suficientes para integrar sistemas externos?
- ¿Las 3 fases (16 horas total) son realistas?

### 2. Implementar Fase 1 (Diseño Detallado) 📐

**Siguiente acción inmediata:**
- Crear diagramas de flujo en Mermaid
- Especificar API completa con ejemplos de código
- Documentar casos edge (¿qué pasa si similitud = 0.85 exacto?)
- Definir mensajes de error y manejo

### 3. Actualizar CHECKLIST_V2.md 📋

```markdown
### 🔄 FlowPacks - Compresión Contextual (Brecha #10)
- [ ] 12.1 - Diseñar sistema compresión contextual (2025-11-06 17:16:10)
  - [ ] 12.1.1 - Análisis de requisitos (casos "disco rayado")
  - [ ] 12.1.2 - Arquitectura de módulos (7 archivos)
  - [ ] 12.1.3 - Especificación de API pública
- [ ] 12.2 - Implementar `src/flowpacks/` (pendiente)
  - [ ] 12.2.1 - flowpack.rs (estructuras)
  - [ ] 12.2.2 - similarity.rs (búsqueda semántica)
  - [ ] 12.2.3 - response.rs (respuestas adaptadas)
  - [ ] 12.2.4 - mod.rs (FlowPackEngine)
  - [ ] 12.2.5 - Integración con FBCU
- [ ] 12.3 - Validar mejoras de rendimiento (pendiente)
  - [ ] 12.3.1 - Tests de similitud (3 escenarios)
  - [ ] 12.3.2 - Tests de performance (ratio >20x, latencia <50ms)
  - [ ] 12.3.3 - Integración con TelescopeDB
  - [ ] 12.3.4 - Validación experiencia usuario
```

### 4. Preparar Entorno de Desarrollo 🛠️

```bash
# Instalar dependencias para embeddings
cargo add sentence-transformers  # O equivalente en Rust

# Instalar dependencias para índice HNSW
cargo add hnswlib-rs

# Verificar FBCU está disponible
cargo check --features fbcu
```

---

## 🌊 FASE 4: SHUIDAO - INTENTION-ORIENTED COGNITIVE ENGINE (Phase 3b)

### Visión: El Camino del Agua (水道)

**ShuiDao** representa la evolución natural de FlowPacks hacia un sistema que no solo **recuerda**, sino que **entiende la intención** detrás de cada interacción.

**Filosofía:**
- **Agua = Información** (fluye, se adapta, encuentra su camino)
- **Canal = ShuiDao** (guía pero no fuerza, permite flujo natural)
- **Memoria como ecosistema vivo** (no archivo muerto)

---

### 🎯 LOS 5 MODOS COGNITIVOS DE SHUIDAO

ShuiDao clasifica cada interacción en **modos de intención**, cada uno con su propia arquitectura:

#### 1️⃣ **MODO CONVERSACIONAL** (General Memory Mode)
**Intención:** Conocimiento general, curiosidad, exploración casual

**Ejemplo:**
```
Usuario: "¿Cómo se llama la planta roja de Navidad?"
ShuiDao detecta: Pregunta conversacional sin objetivo operacional
Arquitectura: FlowPack ligero (Base/Reference)
Respuesta: Explicación directa sin estructura de proyecto
```

**Características:**
- Memoria semántica (qué)
- Sin trazabilidad compleja
- Respuestas adaptativas (Reference si ya se preguntó)
- Guardado en FlowPacks estándar

---

#### 2️⃣ **MODO OPERACIONAL** (Project Mode) 🔥 CLAVE
**Intención:** Completar algo en el mundo real (instalar, configurar, reparar)

**Ejemplo:**
```
Usuario: "¿Cómo instalo un switch?"
ShuiDao detecta: Verbo de acción + objetivo técnico → PROYECTO REAL
```

**Activa microarquitectura completa:**

```rust
pub struct OperationalProject {
    // Identificación
    pub id: ProjectId,
    pub name: String,  // "Instalar_Switch_Wifi"
    pub created: DateTime<Utc>,
    
    // Estructura jerárquica
    pub sub_projects: Vec<SubProject>,
    pub tasks: Vec<Task>,
    pub checklist: Checklist,
    
    // Estado y progreso
    pub status: ProjectStatus,  // NotStarted, InProgress, Blocked, Completed
    pub progress: ProgressTracker,
    pub completion_percentage: f32,
    
    // Trazabilidad
    pub history: Vec<ProjectEvent>,
    pub next_action: Option<ActionRecommendation>,
    
    // Contexto
    pub context: ContextSnapshot,  // CTX7D snapshot al momento de crear
    pub related_flowpacks: Vec<FlowPackId>,
}

pub struct SubProject {
    pub id: SubProjectId,
    pub name: String,  // "Cableado", "Configuración router", "Pruebas"
    pub tasks: Vec<TaskId>,
    pub dependencies: Vec<SubProjectId>,  // Qué debe completarse antes
    pub status: ProjectStatus,
}

pub struct Task {
    pub id: TaskId,
    pub description: String,  // "Comprar cable CAT6"
    pub status: TaskStatus,  // Pending, InProgress, Done, Skipped
    pub assigned_to: Option<String>,  // Usuario (por defecto Eduardo)
    pub due_date: Option<DateTime<Utc>>,
    pub notes: Vec<String>,
}

pub struct ProgressTracker {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub blocked_tasks: usize,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub velocity: f32,  // tareas/día
}

pub struct ActionRecommendation {
    pub description: String,  // "Complete configuración del router"
    pub priority: Priority,
    pub context: String,  // Por qué es la próxima acción
}
```

**Flujo completo:**

```
Usuario: "¿Cómo instalo un switch?"
    ↓
[1] ShuiDao IntentionDetector
    → Verbo: "instalo" (acción operativa)
    → Tema: "switch" (técnico)
    → Clasificación: MODO OPERACIONAL ✅
    ↓
[2] ProjectGenerator
    → Crea: Proyecto "Instalar_Switch_Wifi"
    → Genera sub-proyectos:
        ├─ Preparación (comprar equipo)
        ├─ Cableado (conexiones físicas)
        ├─ Configuración (setup inicial)
        └─ Pruebas (validación)
    → Genera tareas para cada sub-proyecto
    → Establece dependencias
    ↓
[3] Respuesta al usuario
    "Perfecto, vamos a instalar un switch.
     He creado un proyecto con 4 fases:
     
     📦 Fase 1: Preparación (3 tareas)
        ☐ Verificar tipo de switch (managed/unmanaged)
        ☐ Comprar cable CAT6
        ☐ Preparar herramientas
     
     🔌 Fase 2: Cableado (2 tareas)
        ☐ Conectar switch a router
        ☐ Conectar dispositivos al switch
     
     ⚙️ Fase 3: Configuración (2 tareas)
        ☐ Configurar IP estática
        ☐ Setup VLANs (si aplica)
     
     ✅ Fase 4: Pruebas (2 tareas)
        ☐ Test de conectividad
        ☐ Validar velocidad
     
     👉 Próxima acción: Verifica el tipo de switch que necesitas.
     ¿Managed o unmanaged?"
    ↓
[4] Trazabilidad continua
    → Cada respuesta actualiza progreso
    → Usuario puede preguntar: "¿Qué falta del switch?"
    → ShuiDao responde: "Completaste 5/9 tareas. Falta configuración."
```

**Características:**
- Memoria episódica (cuándo/cómo) + semántica (qué)
- Persistencia en TelescopeDB como `ProjectEntry`
- Actualización continua de estado
- Detección de bloqueos ("No puedo completar X porque falta Y")
- Sugerencias inteligentes de próxima acción

---

#### 3️⃣ **MODO PROCEDURAL** (Recipes / Step-by-Step Mode)
**Intención:** Ejecutar proceso repetible paso a paso (recetas, tutoriales)

**Ejemplo:**
```
Usuario: "¿Cómo hago una torta de zanahoria?"
ShuiDao detecta: Proceso culinario → MODO PROCEDURAL
```

**Arquitectura similar a Proyecto pero optimizada para repetición:**

```rust
pub struct ProceduralRecipe {
    pub name: String,  // "Torta de Zanahoria"
    pub category: RecipeCategory,  // Cooking, DIY, Tutorial
    
    // Ingredientes/Materiales
    pub ingredients: Vec<Ingredient>,
    
    // Pasos ordenados
    pub steps: Vec<Step>,
    
    // Opcional: guardado para repetir
    pub bookmarked: bool,
    pub times_executed: usize,
    pub success_rate: f32,
}

pub struct Step {
    pub number: usize,
    pub description: String,
    pub duration: Option<Duration>,
    pub tips: Vec<String>,
    pub completed: bool,
}
```

**Respuesta ejemplo:**
```
"Aquí está la receta completa:

🥕 TORTA DE ZANAHORIA

Ingredientes:
☐ 300g harina
☐ 200g azúcar
☐ 3 huevos
☐ 250g zanahoria rallada
☐ 100ml aceite
☐ 1 cdta polvo de hornear

Pasos (60 min total):
1️⃣ Precalentar horno a 180°C (5 min)
2️⃣ Mezclar ingredientes secos (2 min)
3️⃣ Batir huevos + azúcar (5 min)
4️⃣ Incorporar zanahoria rallada (3 min)
5️⃣ Combinar todo (5 min)
6️⃣ Hornear 40 min

💾 ¿Quieres que guarde esta receta para futuras ocasiones?"
```

**Diferencia con Modo Operacional:**
- Foco en **repetibilidad** (no proyecto único)
- Checklist imprimible
- Puede guardarse como template en VoxelDB
- Menos trazabilidad de estado (se ejecuta y termina)

---

#### 4️⃣ **MODO APRENDIZAJE** (Learning Path Mode)
**Intención:** Construir conocimiento progresivo (teoría, habilidades)

**Ejemplo:**
```
Usuario: "Enséñame teoría musical"
ShuiDao detecta: Petición de aprendizaje → MODO APRENDIZAJE
```

**Arquitectura de ruta de aprendizaje:**

```rust
pub struct LearningPath {
    pub topic: String,  // "Teoría Musical"
    pub current_level: LearningLevel,  // Beginner, Intermediate, Advanced
    
    // Estructura de conocimiento
    pub modules: Vec<LearningModule>,
    pub completed_modules: HashSet<ModuleId>,
    
    // Tracking de comprensión
    pub confusion_points: Vec<ConfusionMarker>,
    pub mastery_indicators: HashMap<ConceptId, f32>,
    
    // Progreso temporal
    pub sessions: Vec<LearningSession>,
    pub total_time_invested: Duration,
    
    // Adaptación
    pub learning_style: LearningStyle,  // Visual, Auditiva, Práctica
    pub recommended_next: Option<ModuleId>,
}

pub struct LearningModule {
    pub id: ModuleId,
    pub name: String,  // "Escalas mayores"
    pub concepts: Vec<Concept>,
    pub prerequisites: Vec<ModuleId>,
    pub exercises: Vec<Exercise>,
    pub status: ModuleStatus,
}

pub struct ConfusionMarker {
    pub concept: ConceptId,
    pub detected_at: DateTime<Utc>,
    pub repetition_count: usize,  // Cuántas veces preguntó lo mismo
    pub resolution_strategy: Option<String>,  // Qué funcionó
}
```

**Flujo inteligente:**

```
Usuario: "Enséñame teoría musical"
    ↓
ShuiDao crea LearningPath:
    Module 1: Notas y escalas (Beginner)
    Module 2: Intervalos (Beginner)
    Module 3: Acordes básicos (Beginner)
    Module 4: Progresiones (Intermediate)
    ...
    ↓
3 días después...
Usuario: "No entiendo los intervalos"
    ↓
ShuiDao detecta:
    → ConfusionMarker en "Intervalos"
    → Similitud alta con pregunta anterior (0.96)
    → Tiempo corto (3 días)
    → Conclusión: Estrategia inicial NO funcionó
    ↓
ShuiDao adapta:
    "Veo que los intervalos siguen sin estar claros.
     Dejame intentar con un enfoque diferente:
     
     🎹 Explicación visual + ejemplos prácticos
     [Diagrama de piano con intervalos marcados]
     [Audio examples]
     
     ¿Esto ayuda más que la explicación teórica?"
```

**Características:**
- Detección de confusión recurrente
- Adaptación de estrategia pedagógica
- Tracking de progreso real (no solo completado, sino ENTENDIDO)
- Sugerencias de siguiente módulo basadas en mastery
- Conexión con Routier (learning paths adaptativos)

---

#### 5️⃣ **MODO OCIO** (Light Mode)
**Intención:** Curiosidad ligera, trivias, entretenimiento

**Ejemplo:**
```
Usuario: "¿Por qué el cielo es azul?"
ShuiDao detecta: Curiosidad sin objetivo → MODO OCIO
```

**Arquitectura minimalista:**
- Respuesta breve y directa
- FlowPack ultra-ligero (sin estructura compleja)
- No genera proyectos ni tracking
- Guardado mínimo (puede olvidarse rápido)

**Respuesta ejemplo:**
```
"El cielo es azul por la dispersión de Rayleigh:
la luz solar se dispersa más en longitudes de onda
cortas (azul) que largas (rojo).

🌅 Dato curioso: Por eso los atardeceres son rojos."
```

**Diferencia con otros modos:**
- Sin persistencia profunda
- Sin trazabilidad
- Sin estructura de proyecto
- Decay rápido en memoria (48h vs 7 días)

---

### 🧠 SISTEMA DE DETECCIÓN DE INTENCIÓN

**¿Cómo ShuiDao decide el modo?**

```rust
pub struct IntentionDetector {
    // Análisis sintáctico
    verb_classifier: VerbClassifier,
    
    // Análisis semántico
    topic_analyzer: TopicAnalyzer,
    
    // Análisis emocional
    tone_detector: ToneDetector,
    
    // Contexto histórico
    conversation_history: ConversationHistory,
}

impl IntentionDetector {
    pub fn detect_mode(&self, message: &str, ctx7d: &ContextToken7D) -> CognitiveMode {
        // 1. Análisis de verbos
        let verbs = self.verb_classifier.extract_verbs(message);
        let verb_intent = self.classify_verb_intent(&verbs);
        
        // 2. Análisis de tema
        let topic = self.topic_analyzer.identify_topic(message);
        let topic_category = self.categorize_topic(&topic);
        
        // 3. Análisis emocional
        let emotional_state = self.tone_detector.analyze(ctx7d);
        
        // 4. Contexto histórico
        let recent_mode = self.conversation_history.recent_mode();
        
        // 5. Decisión multi-factor
        match (verb_intent, topic_category, emotional_state) {
            // Operacional: verbos de acción + tema técnico
            (VerbIntent::Action, TopicCategory::Technical, _) => 
                CognitiveMode::Operational,
            
            // Procedural: verbos de proceso + tema práctico
            (VerbIntent::Process, TopicCategory::Practical, _) => 
                CognitiveMode::Procedural,
            
            // Aprendizaje: verbos de enseñanza + tema conceptual
            (VerbIntent::Learning, TopicCategory::Conceptual, _) => 
                CognitiveMode::Learning,
            
            // Ocio: preguntas simples + bajo engagement
            (VerbIntent::Query, _, EmotionalState::Casual) => 
                CognitiveMode::Light,
            
            // Default: Conversacional
            _ => CognitiveMode::Conversational,
        }
    }
}
```

**Clasificación de verbos:**

```rust
pub enum VerbIntent {
    Action,     // "instalar", "configurar", "reparar", "hacer"
    Process,    // "preparar", "cocinar", "construir", "seguir pasos"
    Learning,   // "enseñar", "explicar", "entender", "aprender"
    Query,      // "qué", "por qué", "cómo", "cuándo"
    Memory,     // "recordar", "resumir", "qué hablamos"
}
```

**Clasificación de temas:**

```rust
pub enum TopicCategory {
    Technical,      // Redes, instalaciones, configuraciones
    Practical,      // Recetas, DIY, tutoriales
    Conceptual,     // Teoría, conceptos abstractos
    Biographical,   // Experiencias personales, memoria
    Casual,         // Trivias, curiosidades
    Creative,       // Arte, música, escritura
}
```

---

### 🎨 ARQUITECTURA DE MEMORIA DUAL

**ShuiDao implementa memoria como los humanos: Episódica + Semántica**

```rust
pub struct DualMemorySystem {
    // Memoria Semántica: QUÉ
    semantic: SemanticMemory,
    
    // Memoria Episódica: CUÁNDO/CÓMO
    episodic: EpisodicMemory,
    
    // Puentes entre ambas
    bridges: Vec<MemoryBridge>,
}

pub struct SemanticMemory {
    // Grafo de conceptos
    concepts: HashMap<ConceptId, ConceptNode>,
    relations: Graph<ConceptId, RelationType>,
    
    // Frecuencia y relevancia
    frequency: HashMap<ConceptId, f32>,
    importance: HashMap<ConceptId, f32>,
}

pub struct EpisodicMemory {
    // Episodios temporales
    episodes: Vec<Episode>,
    temporal_index: BTreeMap<Timestamp, EpisodeId>,
    
    // Marcadores emocionales
    emotional_markers: HashMap<EpisodeId, EmotionalState>,
    
    // Contexto situacional
    context_snapshots: HashMap<EpisodeId, ContextToken7D>,
}

pub struct Episode {
    pub id: EpisodeId,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub mode: CognitiveMode,
    pub emotional_state: EmotionalState,
    pub related_concepts: Vec<ConceptId>,
    pub project: Option<ProjectId>,  // Si fue parte de un proyecto
}

pub struct MemoryBridge {
    pub semantic_node: ConceptId,      // "CTX7D"
    pub episodic_events: Vec<EpisodeId>,  // [ep_001, ep_015, ep_042]
    pub strength: f32,  // Qué tan fuerte es la conexión
}
```

**Ejemplo de uso:**

```
Usuario: "Recuérdame qué hablamos sobre CTX7D"

ShuiDao busca:
1. Semántica: ConceptNode "CTX7D"
   → Relacionado con: "Context Token", "7 dimensiones", "score 133.8"
   
2. Episódica: MemoryBridge "CTX7D" → [ep_001, ep_015, ep_042]
   → ep_001: Día 1, 10:30am, modo Conversacional, emoción: curioso
   → ep_015: Día 5, 3pm, modo Aprendizaje, emoción: confundido
   → ep_042: Día 12, 8pm, modo Operacional (Proyecto), emoción: entusiasmado

Respuesta:
"Hemos hablado de CTX7D en 3 ocasiones:

📅 Primera vez (Día 1, 10:30am):
   Te expliqué qué es el Context Token 7D y sus dimensiones.
   Estabas curioso por entender el concepto.

📅 Segunda vez (Día 5, 3pm):
   Profundizamos en la implementación técnica.
   Detecté confusión en la parte de cálculo de scores.
   
📅 Tercera vez (Día 12, 8pm):
   Lo usamos en el proyecto 'Integración_HubSpoke'.
   Estabas entusiasmado configurando el sistema.

¿Quieres profundizar en algún aspecto específico
o continuar con el proyecto de integración?"
```

---

### 🌀 SISTEMA DE RESONANCIA CONTEXTUAL

**Concepto:** Cada mensaje crea "ondas" que activan conceptos relacionados.

```rust
pub struct ContextualResonance {
    // Grafo de conceptos
    concept_graph: Graph<ConceptId, RelationStrength>,
    
    // Patrones de activación
    activation_patterns: HashMap<ConceptId, ActivationWave>,
}

impl ContextualResonance {
    pub fn propagate(&mut self, message: &str) -> ResonancePattern {
        let initial_concepts = self.extract_concepts(message);
        let mut waves = Vec::new();
        
        // Onda 1: Conceptos directamente mencionados
        let wave_1 = initial_concepts.clone();
        waves.push(Wave { concepts: wave_1, strength: 1.0 });
        
        // Onda 2: Conceptos directamente relacionados
        let wave_2 = self.expand_connections(&wave_1, depth: 1);
        waves.push(Wave { concepts: wave_2, strength: 0.7 });
        
        // Onda 3: Conceptos indirectamente relacionados
        let wave_3 = self.expand_connections(&wave_2, depth: 1);
        waves.push(Wave { concepts: wave_3, strength: 0.4 });
        
        // Onda 4: Patrones emergentes (conexiones no obvias)
        let wave_4 = self.find_emergent_patterns(&waves);
        waves.push(Wave { concepts: wave_4, strength: 0.2 });
        
        ResonancePattern {
            waves,
            peak_resonance: self.calculate_peak(&waves),
            decay_time: Duration::hours(72),
        }
    }
}
```

**Ejemplo práctico:**

```
Usuario: "¿Cómo integro CTX7D con HubSpoke?"

Resonancia:
Onda 1 (directa):
  - CTX7D
  - HubSpoke

Onda 2 (1er grado):
  - Context Token (padre de CTX7D)
  - Multi-Agent (padre de HubSpoke)
  - FBCU (compañero de CTX7D)
  - LLM Routing (función de HubSpoke)

Onda 3 (2do grado):
  - TelescopeDB (usa Context Token)
  - Claude/GPT (providers de HubSpoke)
  - Sensory Engine (genera CTX7D)

Onda 4 (emergente):
  - HarmonyEngine (usa CTX7D para composición musical)
  - MTT-DSL (templates que referencian CTX7D)
  - Routier (learning paths con scoring CTX7D)

ShuiDao responde:
"Para integrar CTX7D con HubSpoke:

📊 Conexiones directas:
  - HubSpoke usa CTX7D para scoring de LLM selection
  - CTX7D score determina qué provider es óptimo

🔗 También te puede interesar:
  - Sensory Engine genera el CTX7D inicial
  - TelescopeDB almacena historial de scores
  - Routier usa el mismo sistema de scoring

🎨 Proyectos relacionados:
  - Ya usaste CTX7D en HarmonyEngine (Día 12)
  - Podrías aplicar patrón similar aquí

¿Quieres que cree un proyecto para esta integración?"
```

---

### 🧩 SISTEMA DE OLVIDO ADAPTATIVO

**Ebbinghaus Forgetting Curve** adaptado para memoria artificial:

```rust
pub struct AdaptiveForgetting {
    decay_rate: f32,  // Velocidad de olvido base
    importance_weights: HashMap<ConceptId, f32>,
}

impl AdaptiveForgetting {
    pub fn retention_strength(&self, entry: &FlowPackEntry) -> f32 {
        let elapsed_hours = entry.age_hours();
        
        // Curva base de Ebbinghaus
        let base_retention = (-elapsed_hours / self.decay_rate).exp();
        
        // Boost por importancia
        let importance = self.importance_weights
            .get(&entry.concept_id)
            .unwrap_or(&0.5);
        let importance_boost = importance.sqrt() * 0.1;
        
        // Boost por interacciones recientes
        let interaction_boost = (entry.interaction_count as f32).sqrt() * 0.05;
        
        // Boost por recencia
        let recency_boost = if elapsed_hours < 24.0 { 0.2 } else { 0.0 };
        
        // Penalización por falta de uso
        let last_access_hours = entry.hours_since_last_access();
        let neglect_penalty = if last_access_hours > 168.0 {
            -0.1 * (last_access_hours / 168.0).ln()
        } else {
            0.0
        };
        
        (base_retention + importance_boost + interaction_boost + recency_boost + neglect_penalty)
            .clamp(0.0, 1.0)
    }
    
    pub fn should_forget(&self, entry: &FlowPackEntry) -> bool {
        let retention = self.retention_strength(entry);
        let importance = self.calculate_importance(entry);
        let connectivity = entry.connections.len();
        
        // Olvida si: baja retención Y baja importancia Y pocas conexiones
        retention < 0.3 && importance < 0.4 && connectivity < 2
    }
    
    pub fn should_consolidate(&self, entry: &FlowPackEntry) -> bool {
        let retention = self.retention_strength(entry);
        let importance = self.calculate_importance(entry);
        
        // Consolida (mover a memoria permanente) si: alta importancia o múltiples accesos
        importance > 0.8 || entry.interaction_count > 5
    }
}
```

**Proceso de consolidación:**

```rust
pub enum MemoryTier {
    WorkingMemory,    // RAM, decay rápido (24-72h)
    ShortTerm,        // Decay medio (1-4 semanas)
    LongTerm,         // Decay lento (meses)
    Permanent,        // Sin decay (núcleo de conocimiento)
}

pub struct MemoryConsolidation {
    pub fn consolidate(&mut self, entry: &FlowPackEntry) -> MemoryTier {
        match (entry.interaction_count, entry.importance) {
            (count, imp) if count > 10 && imp > 0.9 => MemoryTier::Permanent,
            (count, imp) if count > 5 || imp > 0.7 => MemoryTier::LongTerm,
            (count, _) if count > 2 => MemoryTier::ShortTerm,
            _ => MemoryTier::WorkingMemory,
        }
    }
}
```

---

### 📊 GRAPH MINING - PATRONES EMERGENTES

```rust
pub struct EmergentPatternMiner {
    concept_graph: Graph<ConceptId, RelationType>,
}

impl EmergentPatternMiner {
    // Detectar triángulos conceptuales (A→B→C→A)
    pub fn find_concept_triangles(&self) -> Vec<ConceptTriangle> {
        // Si Eduardo pregunta A, luego B, luego C, y vuelve a A
        // → Sistema detecta patrón circular de aprendizaje
        
        let mut triangles = Vec::new();
        
        for node_a in self.concept_graph.nodes() {
            for node_b in self.concept_graph.neighbors(node_a) {
                for node_c in self.concept_graph.neighbors(node_b) {
                    if self.concept_graph.has_edge(node_c, node_a) {
                        triangles.push(ConceptTriangle {
                            nodes: [node_a, node_b, node_c],
                            strength: self.calculate_triangle_strength(node_a, node_b, node_c),
                        });
                    }
                }
            }
        }
        
        triangles
    }
    
    // Detectar "hubs" de conocimiento
    pub fn find_knowledge_hubs(&self) -> Vec<KnowledgeHub> {
        // Conceptos con muchas conexiones = temas centrales para Eduardo
        
        self.concept_graph
            .nodes()
            .filter_map(|node| {
                let degree = self.concept_graph.degree(node);
                if degree > 5 {
                    Some(KnowledgeHub {
                        concept: node,
                        connections: degree,
                        centrality: self.calculate_betweenness_centrality(node),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    
    // Detectar "bridges" entre dominios
    pub fn find_domain_bridges(&self) -> Vec<DomainBridge> {
        // Conexiones inesperadas entre temas distantes
        // Ej: "CTX7D" ← bridge → "Música" (vía HarmonyEngine)
        
        let clusters = self.detect_communities();
        let mut bridges = Vec::new();
        
        for edge in self.concept_graph.edges() {
            let cluster_a = clusters.get(&edge.source);
            let cluster_b = clusters.get(&edge.target);
            
            if cluster_a != cluster_b {
                bridges.push(DomainBridge {
                    source_domain: cluster_a.clone(),
                    target_domain: cluster_b.clone(),
                    bridge_concept: edge.source,
                    strength: edge.weight,
                });
            }
        }
        
        bridges
    }
    
    // Minar secuencias de aprendizaje
    pub fn mine_learning_sequences(&self) -> Vec<LearningPath> {
        // Patrones temporales: Eduardo siempre pregunta A antes que B
        // → Sistema puede predecir próxima pregunta
        
        let temporal_sequence = self.extract_temporal_patterns();
        let mut learning_paths = Vec::new();
        
        for sequence in temporal_sequence {
            if sequence.len() >= 3 {
                let confidence = self.calculate_sequence_confidence(&sequence);
                if confidence > 0.7 {
                    learning_paths.push(LearningPath {
                        sequence: sequence.clone(),
                        confidence,
                        next_prediction: self.predict_next_concept(&sequence),
                    });
                }
            }
        }
        
        learning_paths
    }
}
```

---

### 🚀 PLAN DE IMPLEMENTACIÓN SHUIDAO

**Fase 4.1: Detección de Intención (8 horas)**
- [ ] 4.1.1 - Implementar `IntentionDetector`
  - VerbClassifier (análisis de verbos)
  - TopicAnalyzer (clasificación de temas)
  - ToneDetector (estado emocional)
  - ConversationHistory (contexto previo)
- [ ] 4.1.2 - Implementar `CognitiveMode` enum + routing
- [ ] 4.1.3 - Tests de clasificación de intención (100 casos)

**Fase 4.2: Modo Operacional (12 horas)**
- [ ] 4.2.1 - Implementar `OperationalProject` structures
  - Project, SubProject, Task, Checklist
  - ProgressTracker, ActionRecommendation
- [ ] 4.2.2 - Implementar `ProjectGenerator` automático
- [ ] 4.2.3 - Implementar trazabilidad de estado
- [ ] 4.2.4 - Tests de proyectos end-to-end

**Fase 4.3: Modo Procedural (6 horas)**
- [ ] 4.3.1 - Implementar `ProceduralRecipe` structures
- [ ] 4.3.2 - Generador de checklists imprimibles
- [ ] 4.3.3 - Integración con VoxelDB (templates)
- [ ] 4.3.4 - Tests de recetas

**Fase 4.4: Modo Aprendizaje (10 horas)**
- [ ] 4.4.1 - Implementar `LearningPath` structures
- [ ] 4.4.2 - Implementar `ConfusionDetector`
- [ ] 4.4.3 - Implementar adaptación de estrategia pedagógica
- [ ] 4.4.4 - Integración con Routier
- [ ] 4.4.5 - Tests de aprendizaje adaptativo

**Fase 4.5: Memoria Dual (8 horas)**
- [ ] 4.5.1 - Implementar `SemanticMemory` (concept graph)
- [ ] 4.5.2 - Implementar `EpisodicMemory` (temporal index)
- [ ] 4.5.3 - Implementar `MemoryBridge` (conexiones)
- [ ] 4.5.4 - Tests de recuperación dual

**Fase 4.6: Olvido Adaptativo (6 horas)**
- [ ] 4.6.1 - Implementar `AdaptiveForgetting` (Ebbinghaus curve)
- [ ] 4.6.2 - Implementar `MemoryConsolidation` (tiers)
- [ ] 4.6.3 - Implementar garbage collection selectivo
- [ ] 4.6.4 - Tests de consolidación

**Fase 4.7: Resonancia Contextual (8 horas)**
- [ ] 4.7.1 - Implementar `ContextualResonance` (wave propagation)
- [ ] 4.7.2 - Implementar detección de patrones emergentes
- [ ] 4.7.3 - Implementar cross-domain activation
- [ ] 4.7.4 - Tests de resonancia

**Fase 4.8: Graph Mining (10 horas)**
- [ ] 4.8.1 - Implementar `EmergentPatternMiner`
- [ ] 4.8.2 - Algoritmos de detección de triángulos
- [ ] 4.8.3 - Algoritmos de detección de hubs
- [ ] 4.8.4 - Algoritmos de detección de bridges
- [ ] 4.8.5 - Mining de secuencias de aprendizaje
- [ ] 4.8.6 - Tests de minería de patrones

**Fase 4.9: Integration & Validation (8 horas)**
- [ ] 4.9.1 - Integrar todos los modos cognitivos
- [ ] 4.9.2 - Tests end-to-end por modo
- [ ] 4.9.3 - Benchmarks de performance
- [ ] 4.9.4 - Validación de experiencia de usuario
- [ ] 4.9.5 - Documentación completa

**Total estimado: 76 horas (~10 días de trabajo)**

---

## 📚 REFERENCIAS Y CONTEXTO

### Documentos Relacionados

1. **`ROADMAP_V2/02_COMPONENTES/IMPORTANTES/FLOWPACKS.md`**
   - Especificación conceptual (QUÉ y POR QUÉ)
   - Este documento es complementario (CÓMO y CUÁNDO)

2. **`ROADMAP_V2/02_COMPONENTES/CRITICOS/FBCU_CORE.md`**
   - FBCU comprime mensajes individuales
   - FlowPacks EXTIENDE FBCU para flujos contextuales

3. **`ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md`**
   - TelescopeDB almacena memoria biográfica
   - FlowPacks mejora la forma en que se almacena y recupera

4. **`ROADMAP_V2/CHECKLIST_V2.md`**
   - Tareas 12.1-12.3 (FlowPacks implementation)
   - Brecha #10 del Gap Analysis

### Decisiones Arquitectónicas Aplicables

- **DA-031:** FlowPacks - DAG Processing Pipelines
- **DA-004:** FBCU prioridad alta (base para FlowPacks)
- **DA-007:** TelescopeDB como Brecha Crítica #1 (integración necesaria)
- **DA-001:** Local-First Architecture (embeddings locales preferidos)

### Inspiración y Nomenclatura

**Por qué "FlowPacks":**
- **Flow:** Flujo conversacional (no mensajes aislados)
- **Pack:** Empaquetado/compresión (comprime el conjunto)
- **Distintivo:** No es "ContextCompressionUnit" genérico
- **Evocativo:** Como un "mochila de conversaciones" que llevas contigo

**Por qué "ShuiDao" (水道):**
- **水 (Shui) = Agua:** Fluida, adaptativa, encuentra su camino natural
- **道 (Dao) = Camino/Vía:** Guía sin forzar, permite flujo orgánico
- **Filosofía:** El agua no fuerza, se adapta; ShuiDao no impone, comprende
- **Poético:** Representa la memoria como ecosistema vivo, no archivo muerto

---

## 🎯 CONCLUSIÓN: LA TRANSFORMACIÓN DE BITÁCORA

Este plan de implementación evoluciona **FlowPacks** (Phase 3a) hacia **ShuiDao** (Phase 3b), transformando Bitácora de un asistente que recuerda a un **compañero que comprende**.

### Lo Que Logramos en Phase 3a (✅ COMPLETADO)

✅ **Problema resuelto:** "Disco rayado" - repetición conversacional  
✅ **Solución implementada:** FlowPacks con compresión contextual 20-50x  
✅ **Arquitectura establecida:** 7 módulos, SimHash/Bloom Filters  
✅ **Tests pasando:** 10/10 integration tests, 35/38 lib tests  
✅ **Fundación sólida:** Sistema que **detecta patrones**

### Lo Que Lograremos en Phase 3b (🚧 PRÓXIMO)

🎯 **Evolución:** De detectar patrones a **comprender intención**  
🎯 **5 Modos Cognitivos:** Conversacional, Operacional, Procedural, Aprendizaje, Ocio  
🎯 **Proyectos Reales:** Microarquitectura con sub-proyectos, tareas, progreso  
🎯 **Memoria Dual:** Episódica (cuándo/cómo) + Semántica (qué)  
🎯 **Olvido Inteligente:** Curva de Ebbinghaus + consolidación adaptativa  
🎯 **Resonancia Contextual:** Activación de ondas conceptuales  
🎯 **Graph Mining:** Detección de patrones emergentes no programados  
🎯 **Objetivo final:** Sistema que **entiende propósito**

### El Impacto Transformador

**Antes de FlowPacks (Baseline):**
- Bitácora repite explicaciones completas cada vez
- Sensación de "disco rayado" 😞
- Tokens desperdiciados, latencia alta
- Usuario frustrado: "¿Para qué sirve la memoria?"

**Con FlowPacks Phase 3a (✅ ACTUAL):**
- Bitácora detecta repetición y adapta respuesta
- Sensación de "realmente recuerda" 😊
- Ahorro >50% tokens, latencia <50ms
- Usuario satisfecho: "Construye sobre conocimiento previo"

**Con ShuiDao Phase 3b (🌊 FUTURO):**
- Bitácora entiende **intención** detrás de cada pregunta
- Sensación de "compañero que me conoce" 😊✨🤝
- Crea proyectos, trackea progreso, sugiere próxima acción
- Usuario entusiasmado: "Es mi segundo cerebro"

### Diferencias Clave: Asistente vs Compañero

| Aspecto | Asistente (sin FlowPacks) | Con FlowPacks 3a | Con ShuiDao 3b |
|---------|---------------------------|------------------|----------------|
| **Memoria** | Datos independientes | Patrones semánticos | Intención + contexto |
| **Respuesta** | Siempre completa | Adaptada a similitud | Adaptada a propósito |
| **Estructura** | Plana | FlowPacks (Base/Ref/Delta) | Proyectos + tareas + progreso |
| **Aprendizaje** | No aprende | Detecta repetición | Detecta confusión, adapta estrategia |
| **Proactividad** | Reactivo | Semi-proactivo | Proactivo (sugiere próxima acción) |
| **Experiencia** | "Herramienta útil" 🛠️ | "Memoria real" 😊 | "Compañero cognitivo" 🤝 |

### Métricas de Éxito Phase 3b

**Técnicas:**
- ✅ Detección de intención: >90% accuracy
- ✅ Clasificación de modo: >85% correcta
- ✅ Creación de proyectos: automática cuando aplica
- ✅ Tracking de progreso: persistente entre sesiones
- ✅ Latencia: <100ms (incluye análisis de intención)
- ✅ Memoria dual: <200MB en RAM (100 proyectos + 1000 episodios)

**Experiencia:**
- ✅ Sensación "compañero": >90% feedback positivo
- ✅ Proyectos útiles: >80% completados por usuario
- ✅ Aprendizaje adaptativo: >75% mejora en comprensión
- ✅ Próximas acciones relevantes: >85% seguidas
- ✅ Olvido inteligente: <5% falsos positivos (olvidar importante)

### Casos de Uso Transformadores

**Caso 1: De Pregunta a Proyecto Completo**
```
Sin ShuiDao:
Usuario: "¿Cómo instalo un switch?"
Bitácora: [Explica pasos]
Fin.

Con ShuiDao:
Usuario: "¿Cómo instalo un switch?"
Bitácora: [Detecta intención OPERACIONAL]
          [Crea proyecto "Instalar_Switch_Wifi"]
          [Genera 4 fases, 9 tareas, checklist]
          [Trackea progreso]
          
3 días después:
Usuario: "¿Qué falta del switch?"
Bitácora: "Completaste 6/9 tareas (67%).
           Falta configurar IP y hacer pruebas.
           ¿Empezamos con la configuración?"
```

**Caso 2: Aprendizaje Adaptativo**
```
Día 1: Usuario pregunta teoría musical
       ShuiDao crea LearningPath con 8 módulos

Día 3: Usuario: "No entiendo los intervalos"
       ShuiDao detecta: ConfusionMarker (similitud 0.96)
       ShuiDao adapta: "Dejame con metáfora visual..."
       
Día 7: Usuario completa módulo intervalos
       ShuiDao sugiere: "Listo para acordes (siguiente nivel)"
```

**Caso 3: Memoria Episódica Rica**
```
Usuario: "Recuérdame qué hablamos sobre CTX7D"

ShuiDao responde:
"Hemos hablado de CTX7D en 3 ocasiones:

� Primera vez (Día 1, 10:30am, modo Conversacional):
   Te expliqué el concepto básico.
   Estabas curioso por entender las 7 dimensiones.

📅 Segunda vez (Día 5, 3pm, modo Aprendizaje):
   Profundizamos en la implementación.
   Detecté confusión en el cálculo de scores.

📅 Tercera vez (Día 12, 8pm, modo Operacional):
   Lo usamos en el proyecto 'Integración_HubSpoke'.
   Estabas entusiasmado configurando el sistema.

🔗 También está conectado con:
   - HarmonyEngine (composición musical)
   - Routier (learning paths)
   - TelescopeDB (almacenamiento de scores)

¿Quieres profundizar en algún aspecto o continuar
con la integración de HubSpoke?"
```

---

## 📋 RESUMEN EJECUTIVO PARA CHECKLIST

### Tareas a Agregar en CHECKLIST_V2.md

**FlowPacks Phase 3a (✅ COMPLETADO):**
- [x] 12.1 - Diseño conceptual (4h) ✅
- [x] 12.2 - Implementación core (8h) ✅
- [x] 12.3 - Validation (4h) ✅

**ShuiDao Phase 3b (🚧 PRÓXIMO):**
- [ ] 12.4 - Detección de Intención (8h)
  - [ ] 12.4.1 - IntentionDetector (VerbClassifier, TopicAnalyzer, ToneDetector)
  - [ ] 12.4.2 - CognitiveMode enum + routing
  - [ ] 12.4.3 - Tests de clasificación (100 casos)

- [ ] 12.5 - Modo Operacional (12h)
  - [ ] 12.5.1 - OperationalProject structures (Project, SubProject, Task)
  - [ ] 12.5.2 - ProjectGenerator automático
  - [ ] 12.5.3 - Trazabilidad de estado (ProgressTracker, ActionRecommendation)
  - [ ] 12.5.4 - Tests end-to-end de proyectos

- [ ] 12.6 - Modo Procedural (6h)
  - [ ] 12.6.1 - ProceduralRecipe structures
  - [ ] 12.6.2 - Generador de checklists imprimibles
  - [ ] 12.6.3 - Integración con VoxelDB (templates)
  - [ ] 12.6.4 - Tests de recetas

- [ ] 12.7 - Modo Aprendizaje (10h)
  - [ ] 12.7.1 - LearningPath structures
  - [ ] 12.7.2 - ConfusionDetector
  - [ ] 12.7.3 - Adaptación de estrategia pedagógica
  - [ ] 12.7.4 - Integración con Routier
  - [ ] 12.7.5 - Tests de aprendizaje adaptativo

- [ ] 12.8 - Memoria Dual (8h)
  - [ ] 12.8.1 - SemanticMemory (concept graph)
  - [ ] 12.8.2 - EpisodicMemory (temporal index)
  - [ ] 12.8.3 - MemoryBridge (conexiones)
  - [ ] 12.8.4 - Tests de recuperación dual

- [ ] 12.9 - Olvido Adaptativo (6h)
  - [ ] 12.9.1 - AdaptiveForgetting (Ebbinghaus curve)
  - [ ] 12.9.2 - MemoryConsolidation (tiers)
  - [ ] 12.9.3 - Garbage collection selectivo
  - [ ] 12.9.4 - Tests de consolidación

- [ ] 12.10 - Resonancia Contextual (8h)
  - [ ] 12.10.1 - ContextualResonance (wave propagation)
  - [ ] 12.10.2 - Detección de patrones emergentes
  - [ ] 12.10.3 - Cross-domain activation
  - [ ] 12.10.4 - Tests de resonancia

- [ ] 12.11 - Graph Mining (10h)
  - [ ] 12.11.1 - EmergentPatternMiner
  - [ ] 12.11.2 - Detección de triángulos
  - [ ] 12.11.3 - Detección de hubs
  - [ ] 12.11.4 - Detección de bridges
  - [ ] 12.11.5 - Mining de secuencias de aprendizaje
  - [ ] 12.11.6 - Tests de minería de patrones

- [ ] 12.12 - Integration ShuiDao (8h)
  - [ ] 12.12.1 - Integración de todos los modos
  - [ ] 12.12.2 - Tests end-to-end por modo
  - [ ] 12.12.3 - Benchmarks de performance
  - [ ] 12.12.4 - Validación de experiencia
  - [ ] 12.12.5 - Documentación completa ShuiDao

**Total Phase 3b:** 76 horas (~10 días de trabajo concentrado)

---

**Estado:** 📋 PLAN EVOLUTIVO COMPLETO - Phase 3a ✅ | Phase 3b 🚧  
**Criticidad:** 🔴 TRANSFORMADOR - Diferencia entre asistente y compañero  
**Complejidad:** 🔴 ALTA (arquitectura cognitiva completa)  
**Tiempo estimado:** Phase 3a: 16h ✅ | Phase 3b: 76h  
**Filosofía:** 🌊 El agua encuentra su camino - ShuiDao entiende el propósito

---

*"No basta con recordar. Hay que comprender."* 🌊→🧠

---
*Actualizado: 2025-11-23 23:25:34*  
*Sistema Bitácora v1.0 - MTT-DSL Template: implementation_plan v1.0.0*  
*Validación: ✅ Arquitectura dual database (TelescopeDB + VoxelDB únicamente)*  
*Sistema Bitácora v1.0 - Plan de Implementación ShuiDao*  
*Inspiración: Eduardo + Filosofía del Agua + Arquitectura Cognitiva*
