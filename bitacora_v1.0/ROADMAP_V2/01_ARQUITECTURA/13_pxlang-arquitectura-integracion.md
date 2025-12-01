```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/01_ARQUITECTURA/13_pxlang-arquitectura-integracion.md
Versión: 1.0
Fecha Creación: 2025-11-26
Última Actualización: 2025-11-26
Propósito: Especificar arquitectura técnica de PXLang y su integración con TelescopeDB, VoxelDB y ShuiDao
Estado: ACTIVO
Autor: Eduardo + Bitácora AI
Relación:
  - Precede: 02_COMPONENTES/15_pxlang-symbolic-engine.md
  - Depende: 00_VISION/09_pxlang-vision-filosofia.md
  - Integra: 01_ARQUITECTURA/01_sistema-dual-databases.md
  - Integra: 01_ARQUITECTURA/12_shuidao-intention-detection.md
# === FIN DATOS DE AUDITORÍA ===
```

# 🏗️ PXLang: Arquitectura e Integración
## Diseño Técnico del Motor Simbólico

---

## 📚 TABLA DE CONTENIDOS

1. [Arquitectura de 3 Capas](#arquitectura-de-3-capas)
2. [Integración con TelescopeDB](#integración-con-telescopedb)
3. [Integración con VoxelDB](#integración-con-voxeldb)
4. [Integración con ShuiDao](#integración-con-shuidao)
5. [Flujo de Datos End-to-End](#flujo-de-datos-end-to-end)
6. [Estrategia de Storage](#estrategia-de-storage)
7. [Performance y Optimización](#performance-y-optimización)
8. [Versionado y Compatibilidad](#versionado-y-compatibilidad)

---

## 1. Arquitectura de 3 Capas

### Visión General

```
┌─────────────────────────────────────────────────┐
│           CAPA 3: INTEGRACIÓN                   │
│  ┌─────────────────────────────────────────┐    │
│  │ PXLangService (API pública)             │    │
│  │ ├─ encode_event() → PXScene             │    │
│  │ ├─ decode_scene() → Text                │    │
│  │ ├─ compress() / decompress()            │    │
│  │ └─ analyze_patterns()                   │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│        CAPA 2: CODEC (Compresión)               │
│  ┌─────────────────────────────────────────┐    │
│  │ PXCodec (trait)                         │    │
│  │ ├─ encode_token() → Vec<u8>            │    │
│  │ ├─ decode_token() ← Vec<u8>            │    │
│  │ ├─ encode_scene() → Vec<u8>            │    │
│  │ └─ decode_scene() ← Vec<u8>            │    │
│  │                                         │    │
│  │ Implementaciones:                       │    │
│  │ ├─ StaticCodec (PX-Core-256)           │    │
│  │ ├─ AdaptiveCodec (aprendizaje usuario) │    │
│  │ └─ HybridCodec (static + adaptive)     │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│     CAPA 1: DOMINIO SIMBÓLICO                   │
│  ┌─────────────────────────────────────────┐    │
│  │ Tipos básicos (Rust structs/enums)      │    │
│  │ ├─ PXToken                              │    │
│  │ ├─ PXFrame                              │    │
│  │ ├─ PXScene                              │    │
│  │ ├─ PXArc                                │    │
│  │ ├─ Emotion                              │    │
│  │ ├─ ActionKind                           │    │
│  │ ├─ ContextKind                          │    │
│  │ ├─ ObjectivityLevel                     │    │
│  │ └─ TemporalScope                        │    │
│  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────┘
```

### Separación de Responsabilidades

**Capa 1 (Dominio):**
- ✅ Define QUÉ es un símbolo PX
- ✅ Define estructura de escenas y arcos
- ✅ NO sabe cómo se guarda en disco
- ✅ NO sabe cómo se comprime

**Capa 2 (Codec):**
- ✅ Define CÓMO comprimir símbolos
- ✅ Mapea estructuras dominio ↔ bytes
- ✅ Múltiples estrategias (static, adaptive, hybrid)
- ✅ NO sabe qué hacer con los datos

**Capa 3 (Integración):**
- ✅ Define CUÁNDO y POR QUÉ usar PXLang
- ✅ Coordina con otros sistemas (TelescopeDB, VoxelDB)
- ✅ API pública para resto de Bitácora
- ✅ NO sabe detalles internos compresión

---

## 2. Integración con TelescopeDB

### Arquitectura Híbrida

```
┌──────────────────────────────────────────┐
│         MEMORIA BIOGRÁFICA               │
├──────────────────────────────────────────┤
│                                          │
│  PXLang (Índice Simbólico)              │
│  ├─ 2015-03-15: 😔💼→🚶‍♂️→😊👨‍👩‍👧      │ ← 14 bytes
│  ├─ 2016-07-20: 📚🎓→🌍✈️→🏠❤️          │ ← 12 bytes
│  └─ 2017-11-10: 💡🚀→📈💼→🎉🏆          │ ← 12 bytes
│                                          │
│             ↕ ref_id links               │
│                                          │
│  TelescopeDB (Archivo Rico)             │
│  ├─ note_2015_03_15.md (500 palabras)   │ ← 2.5 KB
│  ├─ audio_2016_07_20.m4a (2 min)        │ ← 500 KB
│  └─ video_2017_11_10.mp4 (5 min)        │ ← 15 MB
│                                          │
└──────────────────────────────────────────┘

Storage total:
├─ PXLang: 38 bytes (3 escenas)
├─ TelescopeDB: 16 MB (datos ricos)
└─ Ratio compresión índice: 421,000:1 ✅
```

### Estructura de Enlaces

```rust
// En PXScene
pub struct PXScene {
    pub id: String,                  // "scene_2015_03_15"
    pub tokens: Vec<PXToken>,        // [😔, 💼, →, ...]
    pub objectivity: ObjectivityLevel, // ◇2 (memoria estable)
    
    // LINK A TELESCOPEDB
    pub telescope_refs: Vec<TelescopeRef>,
    
    pub timestamp: i64,
    pub tags: Vec<String>,
}

pub struct TelescopeRef {
    pub id: String,           // "note_2015_03_15"
    pub kind: String,         // "text" | "audio" | "video" | "image"
    pub preview: Option<String>, // Primeras 100 palabras (opcional)
}
```

### Flujo de Consulta

```
Usuario: "¿Qué pasó en marzo 2015?"

1. PXLang busca por timestamp:
   └─ Encuentra: 😔💼→🚶‍♂️→😊👨‍👩‍👧 (scene_2015_03_15)

2. LLM interpreta símbolos:
   └─ "Período difícil trabajo → reflexión → reconexión familiar"

3. Usuario: "Dame más detalles"

4. PXLang consulta telescope_refs:
   └─ Lee: note_2015_03_15.md

5. LLM enriquece narrativa:
   └─ "El 15 de marzo de 2015 escribiste: 'Hoy renuncié...' [contenido completo]"
```

### Ventajas

- ✅ **Navegación rápida:** PXLang escanea 10 años en <1ms
- ✅ **Detalle bajo demanda:** TelescopeDB solo se lee si usuario pide más
- ✅ **Sincronización automática:** Cambios en TelescopeDB NO afectan PXLang
- ✅ **Storage eficiente:** Índice simbólico minúsculo vs datos ricos enormes

---

## 3. Integración con VoxelDB

### Patrones y Plantillas

```
VoxelDB almacena:
├─ Patrones emocionales típicos
├─ Tipos de escenas recurrentes
├─ Transiciones comunes en biografías
└─ Plantillas MTT-DSL contextuales

PXLang utiliza VoxelDB para:
├─ Comprimir DELTAS (no escena completa)
├─ Detectar escenas similares
├─ Sugerir símbolos basados en contexto
└─ Analizar patrones longitudinales
```

### Compresión por Plantillas

```rust
// Sin VoxelDB (storage completo)
PXScene {
    tokens: [😔, 💼, →, 🚶‍♂️, →, 😊, 👨‍👩‍👧],
    context: Work,
    objectivity: ◇2,
    // ... más metadata
}
// Storage: ~80 bytes

// Con VoxelDB (delta compression)
PXScene {
    template_id: "voxel_pattern_crisis_laboral", // Link a VoxelDB
    deltas: {
        // Solo lo que difiere de la plantilla
        "resolution": 👨‍👩‍👧,  // Template tiene 😊, usuario tiene familia
        "duration": "3 meses", // Template asume 1 mes
    },
    objectivity: ◇2,
}
// Storage: ~40 bytes (50% reducción adicional) ✅
```

### Análisis de Patrones

```
VoxelDB + PXLang permiten:

1. Detectar ciclos:
   └─ Usuario tiene patrón 😔💼 cada 18 meses
   └─ Insight: "Ciclo de burnout laboral"

2. Predecir resoluciones:
   └─ Últimas 3 veces que 😔💼 apareció, resolvió con 🚶‍♂️
   └─ Sugerencia: "Considera tomar un break"

3. Comparar con población:
   └─ Tu patrón 😔💼→😊👨‍👩‍👧 similar a 34% usuarios
   └─ Insight: "Resolución familiar común en crisis laboral"
```

### Estructura de Plantilla

```rust
// En VoxelDB
pub struct VoxelPattern {
    pub id: String,                    // "crisis_laboral_01"
    pub typical_tokens: Vec<PXToken>,  // [😔, 💼, →, ...]
    pub frequency: f32,                // 0.34 (34% usuarios)
    pub avg_duration: Duration,        // 3 meses típico
    pub typical_resolution: PXToken,   // 😊 (70% resuelven positivo)
    pub context: ContextKind,          // Work
}

// En PXScene
pub struct PXScene {
    pub template_id: Option<String>,   // Link a VoxelPattern
    pub deltas: HashMap<String, Value>, // Solo diferencias
    // ... resto campos
}
```

---

## 4. Integración con ShuiDao

### Detección Intencional con PXLang

```
ShuiDao (IntentionDetector) + PXLang:

Usuario: "Me siento atrapado en mi trabajo"

1. IntentionDetector detecta:
   └─ Intención: REFLECTIVE_DEEP
   └─ Tono: Tristeza, frustración

2. ShuiDao consulta PXLang histórico:
   └─ Busca patrones similares: 😔💼
   └─ Encuentra: 3 episodios previos (2015, 2018, 2022)

3. ShuiDao analiza resoluciones:
   └─ 2015: →🚶‍♂️→😊👨‍👩‍👧 (break + familia)
   └─ 2018: →📚→💡 (estudio + nueva skill)
   └─ 2022: →✈️→🌍 (viaje + perspectiva)

4. ShuiDao genera respuesta contextual:
   "Eduardo, he notado que esto te ha pasado antes. 
   Las veces anteriores encontraste alivio mediante:
   1. Reconectar con familia (2015)
   2. Aprender algo nuevo (2018)
   3. Cambiar de ambiente (2022)
   ¿Alguno de estos enfoques resuena contigo ahora?"
```

### CognitiveRouter + PXLang

```rust
impl CognitiveRouter {
    fn route_with_px_context(&self, msg: &Message) -> Route {
        // 1. Detectar intención (normal)
        let intention = self.intention_detector.detect(msg);
        
        // 2. Consultar historia simbólica (NUEVO)
        let px_history = self.pxlang_service.query_similar_scenes(
            &intention,
            last_n_years: 5
        );
        
        // 3. Ajustar ruta según patrones
        if px_history.has_recurring_pattern() {
            return Route::DeepReflective {
                context: px_history.summary(),
                suggested_actions: px_history.successful_resolutions(),
            };
        }
        
        // 4. Ruta normal si no hay patrón
        self.default_route(&intention)
    }
}
```

### ResponseSynthesizer + PXLang

```rust
impl ResponseSynthesizer {
    fn synthesize_with_px_context(&self, route: &Route) -> Response {
        // 1. Generar respuesta base (normal)
        let base_response = self.generate_base(route);
        
        // 2. Enriquecer con historia simbólica (NUEVO)
        if let Some(px_context) = route.px_context {
            let enriched = self.llm.generate(&format!(
                "Usuario dice: '{}'
                 Contexto histórico simbólico: {}
                 Patrones detectados: {}
                 Genera respuesta empática y contextual.",
                route.message,
                px_context.narrative(),
                px_context.patterns()
            ));
            return enriched;
        }
        
        base_response
    }
}
```

---

## 5. Flujo de Datos End-to-End

### Ingesta: Mensaje → PXLang

```
┌─────────────────────────────────────────────────┐
│  1. USUARIO ENVÍA MENSAJE                       │
│     "Hoy renuncié a mi trabajo"                 │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  2. SENSORY ENGINE + CTX7D                      │
│     ├─ Detecta: Evento significativo            │
│     ├─ Emoción: Miedo + Alivio                  │
│     ├─ Contexto: Trabajo                        │
│     └─ Importancia: Alta (8/10)                 │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  3. SHUIDAO ANALIZA                             │
│     ├─ IntentionDetector: LIFE_CHANGING_EVENT   │
│     ├─ EmotionalSpace: Tone = Mixto             │
│     └─ TopicGraph: Topic = Career_transition    │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  4. PXLANG ENCODING (Background)                │
│     EventDescription {                          │
│       natural_language: "Hoy renuncié...",      │
│       emotion_detected: [Fear, Relief],         │
│       context: Work,                            │
│       objectivity: ◇4, // Usuario lo confirma   │
│     }                                           │
│     ↓                                           │
│     PXLangService.encode_event()                │
│     ↓                                           │
│     PXScene {                                   │
│       tokens: [😰, 💼, ⚡, →, 😌],              │
│       objectivity: ◇4,                          │
│       telescope_refs: ["note_2025_11_26"],      │
│     }                                           │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  5. STORAGE (Dual)                              │
│     ├─ PXLang: Guarda escena simbólica (20 B)  │
│     └─ TelescopeDB: Guarda mensaje completo     │
└─────────────────────────────────────────────────┘
```

### Recuperación: Query → Narrativa

```
┌─────────────────────────────────────────────────┐
│  1. USUARIO PREGUNTA                            │
│     "¿Cuándo cambié de trabajo?"                │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  2. SHUIDAO DETECTA INTENCIÓN                   │
│     └─ MEMORY_QUERY (búsqueda biográfica)       │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  3. PXLANG SEARCH                               │
│     ├─ Busca símbolos: 💼⚡ (cambio trabajo)     │
│     ├─ Filtra por contexto: Work                │
│     └─ Encuentra: 3 escenas (2015, 2020, 2025)  │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  4. PXLANG DECODE (NARRATIVO v1.0)              │
│     PXLangService.decode_scene_to_text()        │
│     ↓                                           │
│     LLM interpreta símbolos:                    │
│     "Identifico 3 cambios laborales:            │
│      - 2015: Renunciaste tras crisis (😔💼→🚶)  │
│      - 2020: Transición a remoto (💼→🏠→😊)     │
│      - 2025: Renunciaste nuevamente (😰💼→😌)"  │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  5. ENRICHMENT (Si usuario pide)                │
│     ├─ Lee telescope_refs de cada escena        │
│     └─ Expande con detalles originales          │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│  6. RESPUESTA FINAL                             │
│     "Cambiaste de trabajo 3 veces: en 2015      │
│     renunciaste tras burnout, en 2020 pasaste   │
│     a remoto durante pandemia, y hace 1 mes     │
│     (nov 2025) renunciaste nuevamente."         │
└─────────────────────────────────────────────────┘
```

---

## 6. Estrategia de Storage

### Ubicación Física

```
Bitácora Storage Structure:
├─ telescopedb/
│  ├─ users/
│  │  └─ user_eduardo/
│  │     ├─ notes/
│  │     ├─ audio/
│  │     ├─ video/
│  │     └─ images/
│  │
│  └─ pxlang/  ← NUEVO
│     └─ user_eduardo/
│        ├─ biography.pxbio (archivo comprimido)
│        ├─ scenes_index.cbor (índice rápido)
│        └─ patterns_cache.cbor (VoxelDB links)
│
└─ voxeldb/
   └─ templates/
      └─ patterns/
         └─ biographical_patterns.voxel
```

### Formato .pxbio

```
Estructura archivo biography.pxbio:

┌────────────────────────────────────────┐
│  HEADER (32 bytes)                     │
│  ├─ Magic number: 0x50584C47 ("PXLG") │
│  ├─ Version: 1.0                       │
│  ├─ User ID hash                       │
│  ├─ Creation timestamp                 │
│  └─ Compression method: LZ4            │
├────────────────────────────────────────┤
│  SYMBOL TABLE (variable)               │
│  ├─ PX-Core-256 (estándar)            │
│  └─ User-specific symbols              │
├────────────────────────────────────────┤
│  SCENES DATA (comprimido)              │
│  ├─ Scene 1: [tokens, metadata]       │
│  ├─ Scene 2: [tokens, metadata]       │
│  └─ Scene N: [tokens, metadata]       │
├────────────────────────────────────────┤
│  ARCS INDEX (optional)                 │
│  └─ Referencias a agrupaciones         │
└────────────────────────────────────────┘

Tamaño típico (10 años):
├─ Header: 32 bytes
├─ Symbol table: 2 KB
├─ Scenes (500): 40 KB
├─ Arcs index: 1 KB
└─ Total: 43 KB ✅
```

### Sincronización

```rust
// Estrategia de sync
pub struct PXLangSync {
    local_cache: PathBuf,        // biography.pxbio local
    cloud_backup: Option<Url>,   // Usuario elige (Drive, etc)
    sync_strategy: SyncStrategy,
}

pub enum SyncStrategy {
    LocalOnly,           // No sync, solo dispositivo
    CloudBackup,         // Backup periódico a cloud
    RealtimeSync,        // Sync inmediato (WiFi only)
    ManualExport,        // Usuario exporta manualmente
}

impl PXLangSync {
    async fn sync(&self) -> Result<()> {
        match self.sync_strategy {
            SyncStrategy::LocalOnly => Ok(()),
            
            SyncStrategy::CloudBackup => {
                // Solo si WiFi + battery > 50%
                if self.should_sync() {
                    self.backup_to_cloud().await?;
                }
                Ok(())
            },
            
            SyncStrategy::RealtimeSync => {
                // Sync cada cambio (modo power user)
                self.sync_incremental().await
            },
            
            SyncStrategy::ManualExport => {
                // Usuario debe exportar explícitamente
                Ok(())
            },
        }
    }
}
```

---

## 7. Performance y Optimización

### Benchmarks Objetivo

```
Operación: encode_event() (mensaje → PXScene)
├─ Target: <10ms (P50), <50ms (P99)
├─ Bottleneck: LLM inference (símbolo selection)
└─ Optimización: Cache símbolos frecuentes

Operación: decode_scene_to_text() (PXScene → narrativa)
├─ Target: <100ms (P50), <500ms (P99)
├─ Bottleneck: LLM generation (narrativa)
└─ Optimización: Templates pre-generados

Operación: search_scenes() (query → PXScenes)
├─ Target: <5ms para 10 años (P50)
├─ Bottleneck: Scan secuencial
└─ Optimización: Índice CBOR + binary search

Operación: compress() (PXScene → bytes)
├─ Target: <1ms (P50)
├─ Bottleneck: LZ4 compression
└─ Optimización: Usar lz4_flex (Rust nativo)
```

### Cache Strategy

```rust
pub struct PXLangCache {
    // Cache escenas recientes (LRU)
    recent_scenes: LruCache<String, PXScene>,
    
    // Cache patrones frecuentes
    frequent_patterns: HashMap<String, VoxelPattern>,
    
    // Cache símbolos usuario-específicos
    user_symbols: HashMap<String, PXToken>,
}

impl PXLangCache {
    fn get_or_encode(&mut self, event: &EventDescription) -> PXScene {
        // 1. Revisar cache
        if let Some(cached) = self.recent_scenes.get(&event.hash()) {
            return cached.clone();
        }
        
        // 2. Encodear (llamada LLM)
        let scene = self.encoder.encode(event);
        
        // 3. Cachear
        self.recent_scenes.put(event.hash(), scene.clone());
        
        scene
    }
}
```

### Compresión Agresiva

```
Pipeline compresión:

1. Structural (Capa 1 → Capa 2):
   ├─ PXScene (Rust struct) → CBOR bytes
   └─ Reducción: ~30% (vs JSON)

2. Symbolic (Delta compression):
   ├─ Detectar similitud con VoxelDB patterns
   ├─ Guardar solo deltas
   └─ Reducción adicional: ~40%

3. Binary (LZ4):
   ├─ Comprimir CBOR con lz4_flex
   └─ Reducción adicional: ~60%

Ratio total: 1,000:1 (texto original → PXLang comprimido) ✅

Ejemplo:
├─ Texto: "Hoy renuncié a mi trabajo..." (500 palabras) = 2.5 KB
└─ PXLang: 😰💼⚡→😌 + metadata = 25 bytes
```

---

## 8. Versionado y Compatibilidad

### Versionado Semántico

```rust
pub struct PXVersion {
    pub major: u8,  // Breaking changes
    pub minor: u8,  // New features
    pub patch: u8,  // Bug fixes
}

// v1.0: Initial release (v1.0 Bitácora)
const PX_VERSION_1_0: PXVersion = PXVersion { major: 1, minor: 0, patch: 0 };

// v1.1: Unicode expansion (más símbolos)
const PX_VERSION_1_1: PXVersion = PXVersion { major: 1, minor: 1, patch: 0 };

// v2.0: Revelación pública + nuevas features
const PX_VERSION_2_0: PXVersion = PXVersion { major: 2, minor: 0, patch: 0 };
```

### Compatibilidad hacia Atrás

```rust
pub trait PXCodec {
    fn version(&self) -> PXVersion;
    
    fn can_decode(&self, data: &[u8]) -> bool {
        // Leer versión del header
        let data_version = self.read_version(data);
        
        // Codec actual puede leer versiones anteriores
        data_version.major <= self.version().major
    }
    
    fn decode_legacy(&self, data: &[u8]) -> Result<PXScene> {
        let data_version = self.read_version(data);
        
        match (data_version.major, data_version.minor) {
            (1, 0) => self.decode_v1_0(data),
            (1, 1) => self.decode_v1_1(data),
            (2, _) => self.decode_v2_0(data),
            _ => Err(PXCodecError::UnsupportedVersion),
        }
    }
}
```

### Migración de Datos

```rust
pub struct PXMigrator {
    from_version: PXVersion,
    to_version: PXVersion,
}

impl PXMigrator {
    // Migración v1.0 → v1.1 (sin breaking changes)
    fn migrate_1_0_to_1_1(&self, scene: PXScene) -> PXScene {
        // Añadir nuevos campos con defaults
        PXScene {
            extended_symbols: vec![],  // Nuevo campo
            ..scene
        }
    }
    
    // Migración v1.x → v2.0 (con breaking changes)
    fn migrate_1_x_to_2_0(&self, scene: PXScene) -> PXScene {
        // Re-encoding completo con nuevo codec
        let text = self.decode_to_text(&scene);
        self.encode_with_v2(text)
    }
}
```

---

## 🎯 Resumen Arquitectónico

**PXLang se integra como:**

1. **Capa de índice** sobre TelescopeDB (navegación rápida)
2. **Consumidor de plantillas** de VoxelDB (compresión delta)
3. **Proveedor de contexto** para ShuiDao (patrones históricos)
4. **Storage independiente** (.pxbio) con sync configurable
5. **API pública** (PXLangService) para resto de Bitácora

**Características clave:**
- ✅ Arquitectura de 3 capas (dominio, codec, integración)
- ✅ Compresión 1,000:1 (texto → símbolos)
- ✅ Performance <10ms encode, <5ms search
- ✅ Versionado + compatibilidad hacia atrás
- ✅ Local-first + cloud backup opcional

**Próximos pasos:**
- Implementar componentes (ver: 02_COMPONENTES/15_pxlang-symbolic-engine.md)
- Definir plan de implementación (ver: 04_IMPLEMENTACION/DA-035_pxlang_core.md)

---

*Documento: ROADMAP_V2/01_ARQUITECTURA/13_pxlang-arquitectura-integracion.md*  
*Versión: 1.0*  
*Estado: ACTIVO*  
*Próxima lectura: 02_COMPONENTES/15_pxlang-symbolic-engine.md*
