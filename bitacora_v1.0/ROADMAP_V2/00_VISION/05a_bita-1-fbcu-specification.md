
# BITA‑1 / FBCU v1.0 — Implementación de Referencia (Especificación + Guía de Código)

**Estado:** Draft estable (v1.0)  
**Ámbito:** Definición canónica del *Formato Bitácora Cognitivo Unificado (FBCU)*, esquema de **Cápsula 7D** para TelescopeDB, **Overlays** (Plasticidad/Topología), **LIP** (Lens Interface Protocol), **FlowPack** para VoxelDB y **Manifiesto BITA‑1**.  
**Objetivo:** Que un agente de codificación pueda implementarlo **al pie de la letra** en Rust (o lenguaje equivalente), con almacenamiento local-first, direccionamiento por contenido y compatibilidad con dispositivos de baja gama.

---

## 0. Principios Operativos

1. **Inmutabilidad del núcleo**: cada *Cognitive Quantum* (FBCU Core) es inmutable y direccionado por su hash canónico (**content-addressable ID**).
2. **Mutación vía Overlays**: cualquier aprendizaje/estado dinámico (plasticidad, rutas topológicas, contadores de acceso) se guarda en **Overlays** versionados que referencian el `id` del FBCU Core.
3. **Codificación canónica**: el almacenamiento canónico es **CBOR** determinista (DAG-CBOR style: claves ordenadas, sin NaN ambiguos, sin floats exóticos). La vista legible es JSON/XML **derivada**.
4. **Compatibilidad con lentes**: cada FBCU incluye encabezado BITA‑1 con `lens_ontology` y `compat_signature`. El LIP define el **contrato** de I/O y calidad esperada.
5. **Índices laterales**: Bloom/MinHash/ANN vecindarios son opcionales y se guardan como sidecars. No afectan la semántica del objeto.

---

## 1. Estructura de archivos y naming

```
.bitacora/
  telescope/
    cores/
      <sha256>.fbc          # FBCU Core (CBOR canónico)
    overlays/
      plasticity/
        <sha256-core>__ovp__<seq>.ovp   # Overlay de Plasticidad
      topology/
        <sha256-core>__ovt__<seq>.ovt   # Overlay Topológico
    sidecars/
      <sha256-core>.bloom
      <sha256-core>.minhash
    manifest/
      BITA-1.manifest.cbor
  voxel/
    flowpacks/
      <flow-id>.fp          # FlowPack (CBOR canónico)
    lenses/
      <lens-id>.lip         # LIP: contrato de lente
```

**Extensiones:**
- `.fbc`: FBCU Core (CBOR)  
- `.ovp`: Overlay Plasticidad (CBOR)  
- `.ovt`: Overlay Topología (CBOR)  
- `.fp` : FlowPack VoxelDB (CBOR)  
- `.lip`: LIP (CBOR)  
- `.manifest.cbor`: Manifiesto BITA‑1

---

## 2. Encabezado BITA‑1 (obligatorio)

Campos mínimos en **todos** los objetos canónicos:

- `id` (string, hex): SHA‑256 del **payload canónico** sin el propio `id` (ver §5).  
- `version` (u16): versión semántica del esquema (para FBCU v1.0 usar `100`).  
- `created_at` (RFC3339).  
- `author` (opcional).  
- `lens_ontology` (string, ej. `"semantic-v2"`).  
- `compat_signature` (string, hash que resume compatibilidad de lente/formato).  
- `checksum` (string, SHA‑256 del *payload bruto* — útil para integridad de transporte).

---

## 3. Esquema FBCU Core (Cápsula 7D, inmutable)

> **Propósito:** capturar significado estable. Nada aquí se modifica tras persistir.

### 3.1 Campos (orden canónico de claves)

1. `bita_header` (objeto BITA‑1, §2).  
2. `atomic_core`  
   - `embedding` `{ model: string, dim: u16, type: "dense", vec: bytes|array }`  
   - `symbolic_anchors`: lista de `Anchor`
     - `Anchor` `{ id?: string, lang?: string, curie?: string, label?: string, weight: f32 }`
3. `relational_triples` (lista de `Triple`)
   - `Triple` `{ id?: string, subject: Ref, predicate: Pred, object: Ref, strength?: f32, schema?: string }`
   - `Ref` `{ type: "entity"|"concept"|"emotion"|"action", ref?: string, label?: string }`
   - `Pred` `{ type: "causal"|"relational"|"emotional_state"|..., label: string }`
4. `context_tensor`  
   - `temporal` `{ absolute?: RFC3339, half_life_days?: f32, tag?: string }`  
   - `spatial`  `{ physical?: string, abstract?: string }`  
   - `schematic` `{ active_frame?: string, roles?: { [role_id]: string } }`
5. `affective_epistemic_field`  
   - `emotional_vector` `{ valence: f32, arousal: f32, dominance: f32 }`  **rango:** `[0.0 .. 1.0]`  
   - `epistemic_status` `{ certainty: f32, source?: string, type?: string, contradiction_tolerance?: "low"|"med"|"high" }`  
   - `pragmatic_intent` `{ function?: string, speech_act?: string, expected_effect?: string }`
6. `provenance`  
   - `creation_context` `{ source?: string, extraction_method?: string, lens_version?: string }`  
   - `transformation_history` `[{ timestamp: RFC3339, type: string }]`
7. `semantic_health_metrics` (opcional)  
   - `{ coherence?: f32, lens_agreement?: f32, compression_fidelity?: f32 }`  **rango:** `[0.0 .. 1.0]`

**Notas de normalización:**
- `half_life_days` sustituye decaimientos arbitrarios.  
- `emotional_vector` y `certainty` normalizados a `[0..1]`.  
- `anchors` deben usar **CURIEs** (`schema:term`) cuando aplique.

---

## 4. Overlays (mutables, referencian `core_id`)

### 4.1 Overlay de Plasticidad (`.ovp`)
```
{
  bita_header: {...},         # id del overlay, no del core
  core_id: "<sha256-core>",
  fast_memory: { status: "active"|"idle", strength: f32, last_accessed?: RFC3339, access_count?: u64 },
  slow_memory: { status: "consolidating"|"stable", strength: f32, first_encoded?: RFC3339, consolidation_cycles?: u32, stability_threshold?: f32 },
  hebbian_weights: [ { from: string, to: string, weight: f32, rule?: string, updated?: RFC3339 } ],
  relevance_gates: { emotional: { threshold: f32, current: f32 }, novelty: { threshold: f32, current: f32 }, consolidation: { threshold: f32, current: f32 } },
  journal: [ { event_time: RFC3339, op: "inc"|"set"|"decay", path: "/fast_memory/access_count", value: number } ],
  seq: u64                      # número de secuencia monotónico
}
```

### 4.2 Overlay Topológico (`.ovt`)
```
{
  bita_header: {...},
  core_id: "<sha256-core>",
  recovery_paths: [ { id?: string, nodes: [string], weight?: f32, type?: "semantic_chain"|"categorical"|"contextual_opposition" } ],
  neighborhoods: [ { metric: "cosine"|"jaccard", k: u16, neighbors: [ { id: "<sha256-core>", score: f32 } ] } ],
  homology_signature?: { betti_0?: u32, betti_1?: u32, betti_2?: u32 },  # opcional, offline
  seq: u64
}
```

**Reglas de consistencia:**
- `seq` estrictamente creciente por `core_id` y tipo de overlay.
- Resolver concurrente vía **journaling** + *last-writer-wins* por `event_time` o sumatorias monotónicas (CRDT-lite).

---

## 5. Codificación CBOR canónica + Hash de contenido

### 5.1 Reglas de canonicidad
- Orden lexicográfico de claves (UTF-8).  
- Sin NaN, sin `inf`; usar floats finitos IEEE‑754.  
- Bytes del embedding preferible en `bytes` (no array de floats) usando **f32 little-endian** empaquetado (opcional), documentando `dim`.  
- No incluir `bita_header.id` en el cálculo del hash (se escribe después).

### 5.2 Procedimiento
1. Serializar el objeto **sin** `bita_header.id`.  
2. Canonicalizar (CBOR determinista).  
3. `id = SHA-256(cbor_bytes)`.  
4. Grabar el objeto con `bita_header.id = hex(id)` y `checksum = SHA-256(raw_bytes)`.

---

## 6. LIP — Lens Interface Protocol

```
{
  bita_header: {...},
  lens_id: "string",
  version: "semver",
  requires: {
    anchors?: true,
    triples?: true,
    embedding?: { model: "multimodal-v3", dim: 1024 },
    fields?: ["context_tensor.temporal", "affective_epistemic_field.emotional_vector"]
  },
  provides: {
    outputs: ["summary", "action_plan", "explanations"],
    quality_bounds: { coherence_min: 0.80, lens_agreement_min: 0.85 }
  },
  explain_hints?: ["highlight_triplets", "trace_paths"]
}
```

**Contratos:** si `requires` no satisface, el lente debe fallar con error **determinista** y mensaje claro.

---

## 7. FlowPack (VoxelDB)

```
{
  bita_header: {...},
  flow_id: "string",
  dag: {
    nodes: [ { id: "N1", op: "summarize", params_schema: {...}, ports: { in: ["text"], out: ["summary"] } }, ... ],
    edges: [ { from: "N1:out", to: "N2:in" }, ... ]
  },
  contracts: { lip_refs: [ "<lens-id>" ], quality_bounds?: {...} },
  tests: [ { name: "smoke", inputs: {...}, expect: {...} } ],
  references: { fbc_ids: ["<sha256-core>", ...] }
}
```

---

## 8. Manifiesto BITA‑1

Índice de `cores`, `overlays`, `flowpacks`, `lenses`, políticas de retención/aging y matriz de compatibilidad. Debe incluir:
- `objects`: `{ type, id, path, created_at }[]`
- `policies`: `{ retention_days, half_life_days, encryption: "none"|"device"|"user-key" }`
- `compat_matrix`: `{ lens_id -> [ ids compatibles ] }`

---

## 9. Implementación en Rust (referencia)

> Dependencias sugeridas: `serde`, `serde_cbor`, `serde_with`, `sha2`, `blake3` (opcional), `time` o `chrono`, `hex`, `anyhow`, `thiserror`.

### 9.1 Tipos básicos

```rust
// Cargo.toml (fragmento)
/*
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_cbor = "0.11"
serde_with = "3"
sha2 = "0.10"
hex = "0.4"
time = { version = "0.3", features = ["formatting", "parsing"] }
anyhow = "1"
thiserror = "1"
*/
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BitaHeader {
    pub id: Option<String>,           // se completa tras hash
    pub version: u16,                 // 100 para v1.0
    pub created_at: String,           // RFC3339
    pub author: Option<String>,
    pub lens_ontology: String,
    pub compat_signature: String,
    pub checksum: Option<String>,     // integridad transporte
}
```

### 9.2 FBCU Core structs

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embedding {
    pub model: String,
    pub dim: u16,
    pub r#type: String,       // "dense"
    #[serde(with = "serde_bytes")]
    pub vec: Vec<u8>,         // f32 little-endian packed (dim * 4 bytes)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Anchor {
    pub id: Option<String>,
    pub lang: Option<String>,
    pub curie: Option<String>,      // ej. "emo:hope"
    pub label: Option<String>,
    pub weight: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ref {
    pub r#type: String,             // "entity" | "concept" | ...
    pub ref_id: Option<String>,     // uuid/sha/cuid
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pred {
    pub r#type: String,             // "causal" | "relational" | ...
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Triple {
    pub id: Option<String>,
    pub subject: Ref,
    pub predicate: Pred,
    pub object: Ref,
    pub strength: Option<f32>,
    pub schema: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Temporal {
    pub absolute: Option<String>,     // RFC3339
    pub half_life_days: Option<f32>,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spatial {
    pub physical: Option<String>,
    pub r#abstract: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Schematic {
    pub active_frame: Option<String>,
    pub roles: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContextTensor {
    pub temporal: Option<Temporal>,
    pub spatial: Option<Spatial>,
    pub schematic: Option<Schematic>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmotionalVector {
    pub valence: f32,   // [0..1]
    pub arousal: f32,   // [0..1]
    pub dominance: f32, // [0..1]
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EpistemicStatus {
    pub certainty: f32,                // [0..1]
    pub source: Option<String>,
    pub r#type: Option<String>,
    pub contradiction_tolerance: Option<String>, // "low"|"med"|"high"
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PragmaticIntent {
    pub function: Option<String>,
    pub speech_act: Option<String>,
    pub expected_effect: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AffectiveEpistemicField {
    pub emotional_vector: EmotionalVector,
    pub epistemic_status: Option<EpistemicStatus>,
    pub pragmatic_intent: Option<PragmaticIntent>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreationContext {
    pub source: Option<String>,
    pub extraction_method: Option<String>,
    pub lens_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HistoryEvent {
    pub timestamp: String,  // RFC3339
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Provenance {
    pub creation_context: Option<CreationContext>,
    pub transformation_history: Option<Vec<HistoryEvent>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SemanticHealthMetrics {
    pub coherence: Option<f32>,
    pub lens_agreement: Option<f32>,
    pub compression_fidelity: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AtomicCore {
    pub embedding: Embedding,
    pub symbolic_anchors: Vec<Anchor>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FbcuCore {
    pub bita_header: BitaHeader,
    pub atomic_core: AtomicCore,
    pub relational_triples: Vec<Triple>,
    pub context_tensor: Option<ContextTensor>,
    pub affective_epistemic_field: Option<AffectiveEpistemicField>,
    pub provenance: Option<Provenance>,
    pub semantic_health_metrics: Option<SemanticHealthMetrics>,
}
```

### 9.3 Overlays

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FastMemory { pub status: String, pub strength: f32, pub last_accessed: Option<String>, pub access_count: Option<u64> }
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlowMemory { pub status: String, pub strength: f32, pub first_encoded: Option<String>, pub consolidation_cycles: Option<u32>, pub stability_threshold: Option<f32> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Hebb { pub from: String, pub to: String, pub weight: f32, pub rule: Option<String>, pub updated: Option<String> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Gates { pub threshold: f32, pub current: f32 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RelevanceGates { pub emotional: Gates, pub novelty: Gates, pub consolidation: Gates }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalEvent { pub event_time: String, pub op: String, pub path: String, pub value: f64 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OverlayPlasticity {
    pub bita_header: BitaHeader,
    pub core_id: String,
    pub fast_memory: Option<FastMemory>,
    pub slow_memory: Option<SlowMemory>,
    pub hebbian_weights: Vec<Hebb>,
    pub relevance_gates: RelevanceGates,
    pub journal: Vec<JournalEvent>,
    pub seq: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neighbor { pub id: String, pub score: f32 }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Neighborhood { pub metric: String, pub k: u16, pub neighbors: Vec<Neighbor> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TopologySig { pub betti_0: Option<u32>, pub betti_1: Option<u32>, pub betti_2: Option<u32> }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OverlayTopology {
    pub bita_header: BitaHeader,
    pub core_id: String,
    pub recovery_paths: Vec<std::collections::BTreeMap<String, serde_cbor::Value>>,
    pub neighborhoods: Vec<Neighborhood>,
    pub homology_signature: Option<TopologySig>,
    pub seq: u64,
}
```

### 9.4 LIP y FlowPack

```rust
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lip {
    pub bita_header: BitaHeader,
    pub lens_id: String,
    pub version: String,
    pub requires: serde_cbor::Value,
    pub provides: serde_cbor::Value,
    pub explain_hints: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowNode {
    pub id: String,
    pub op: String,
    pub params_schema: Option<serde_cbor::Value>,
    pub ports: Option<std::collections::BTreeMap<String, Vec<String>>>, // in/out
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowEdge { pub from: String, pub to: String }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlowPack {
    pub bita_header: BitaHeader,
    pub flow_id: String,
    pub dag: std::collections::BTreeMap<String, serde_cbor::Value>, // nodes/edges
    pub contracts: std::collections::BTreeMap<String, serde_cbor::Value>, // lip_refs, quality_bounds
    pub tests: Vec<std::collections::BTreeMap<String, serde_cbor::Value>>,
    pub references: std::collections::BTreeMap<String, serde_cbor::Value>, // fbc_ids
}
```

### 9.5 Serialización canónica + hash

```rust
use sha2::{Digest, Sha256};
use anyhow::Result;

fn to_canonical_cbor<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut ser = serde_cbor::ser::Serializer::new(&mut buf);
    ser.self_describe() // opcional; puede omitirse
        .canonical(true); // **clave**: orden determinista
    value.serialize(&mut ser)?;
    Ok(buf)
}

fn compute_id_without_id_field(mut core: FbcuCore) -> Result<(String, Vec<u8>)> {
    // Eliminar id del header para el hash
    let mut header = core.bita_header.clone();
    header.id = None;
    core.bita_header = header;
    let bytes = to_canonical_cbor(&core)?;
    let hash = Sha256::digest(&bytes);
    Ok((hex::encode(hash), bytes))
}
```

### 9.6 Persistencia (write/read)

```rust
use std::fs;
use anyhow::{anyhow, Context};

pub fn write_fbcu_core(mut core: FbcuCore, path: &str) -> Result<()> {
    let (id, bytes_no_id) = compute_id_without_id_field(core.clone())?;
    core.bita_header.id = Some(id.clone());

    // checksum del payload bruto (sin el id) o del objeto final (elige y sé consistente)
    let checksum = Sha256::digest(&bytes_no_id);
    core.bita_header.checksum = Some(hex::encode(checksum));

    // serializar final (con id)
    let final_bytes = to_canonical_cbor(&core)?;
    fs::write(path, final_bytes)?;
    Ok(())
}

pub fn read_fbcu_core(path: &str) -> Result<FbcuCore> {
    let bytes = fs::read(path)?;
    let core: FbcuCore = serde_cbor::from_slice(&bytes).context("CBOR inválido")?;

    // Validar id
    let (recomputed, _) = compute_id_without_id_field(core.clone())?;
    let stored = core.bita_header.id.clone().ok_or(anyhow!("id faltante"))?;
    if recomputed != stored {
        return Err(anyhow!("ID de contenido no coincide"));
    }
    Ok(core)
}
```

### 9.7 Construcción de ejemplo (embedding + anchors)

```rust
fn pack_f32_le(vec: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(vec.len() * 4);
    for f in vec { out.extend_from_slice(&f.to_le_bytes()); }
    out
}

// Ejemplo de creación mínima de FBCU Core
fn example_core() -> FbcuCore {
    let header = BitaHeader {
        id: None,
        version: 100,
        created_at: "2025-10-25T14:30:00Z".into(),
        author: Some("Eduardo".into()),
        lens_ontology: "semantic-v2".into(),
        compat_signature: "sha256:...".into(),
        checksum: None,
    };
    let emb = Embedding {
        model: "multimodal-v3".into(),
        dim: 4,
        r#type: "dense".into(),
        vec: pack_f32_le(&[0.1, 0.2, 0.3, 0.4]),
    };
    let anchors = vec![ Anchor { id: None, lang: Some("es".into()), curie: Some("emo:esperanza".into()), label: Some("esperanza".into()), weight: 0.92 } ];
    let atomic = AtomicCore { embedding: emb, symbolic_anchors: anchors };

    let triples = vec![ Triple {
        id: Some("t1".into()),
        subject: Ref { r#type: "entity".into(), ref_id: Some("uuid_persona".into()), label: Some("Juan".into()) },
        predicate: Pred { r#type: "emotional_state".into(), label: "experimenta".into() },
        object: Ref { r#type: "emotion".into(), ref_id: Some("uuid_esperanza".into()), label: Some("esperanza".into()) },
        strength: Some(0.87),
        schema: Some("Agente-Accion-Paciente".into()),
    }];

    FbcuCore {
        bita_header: header,
        atomic_core: atomic,
        relational_triples: triples,
        context_tensor: None,
        affective_epistemic_field: None,
        provenance: None,
        semantic_health_metrics: None,
    }
}
```

### 9.8 Overlay update (journaling)

```rust
fn overlay_plasticity_inc_access(mut ov: OverlayPlasticity) -> OverlayPlasticity {
    let now = "2025-10-25T14:35:00Z".to_string(); // usa time::OffsetDateTime::now_utc().format(...)
    if let Some(ref mut fm) = ov.fast_memory {
        fm.last_accessed = Some(now.clone());
        let c = fm.access_count.unwrap_or(0) + 1;
        fm.access_count = Some(c);
        ov.journal.push(JournalEvent { event_time: now, op: "inc".into(), path: "/fast_memory/access_count".into(), value: 1.0 });
    }
    ov.seq += 1;
    ov
}
```

---

## 10. Handshake de lentes

1. Cargar `FBCU Core` → validar `id`.  
2. Verificar `lens_ontology` y `compat_signature`.  
3. Cargar `LIP` del lente.  
4. Chequear `requires`. Si falta, **fallar determinísticamente** con lista de carencias.  
5. Ejecutar → generar outputs y, opcionalmente, actualizar **Overlays**.

---

## 11. Validaciones (reglas duras)

- `valence|arousal|dominance|certainty` ∈ `[0..1]`.  
- `dim` de `embedding` × 4 == `vec.len()`.  
- `half_life_days` > 0 cuando exista.  
- `seq` de overlays **monótono** por par (`core_id`, tipo overlay).  
- `id` contenido == SHA‑256(CBOR sin `id`).

---

## 12. Pruebas mínimas

- **Canonicidad**: serializar → hash → reserializar → hash debe coincidir.  
- **Metamórfica**: cambiar compresión externa no varía `lens_agreement` ± ε.  
- **Fuzz semántico** (triples/anchors): el lente mantiene `coherence` ≥ umbral.  
- **Integridad overlays**: merges concurrentes preservan monotonicidad de `seq` y no rompen journaling.

---

## 13. Seguridad y cifrado

- Cifrado opcional por archivo: `"device"` (clave segura del dispositivo) o `"user-key"` (clave del usuario).  
- Nunca cifrar `bita_header` completo: dejar legibles como mínimo `version`, `created_at`, `lens_ontology` para ruteo.  
- Mantener `checksum` para verificar integridad antes de descifrar.

---

## 14. Rendimiento (targets guía, móviles gama baja)

- Escritura núcleo FBCU: < 5 ms para 1–4 KB (sin embedding grande).  
- Lectura + validación ID: < 3 ms.  
- Overlay update: < 2 ms.  
- Embeddings grandes: usar almacenamiento aparte y referenciar por `blob_id` si supera 64 KB.

---

## 15. Migración/Versionado

- `version` global (u16). Transformadores de migración `v_current -> v_next` deben ser puros (sin side-effects).  
- Conservar historial en `transformation_history` para auditoría.  
- Back-compat: campos extra deben ignorarse (serde `deny_unknown_fields` **desactivado** en lectura).

---

## 16. Vista JSON de ejemplo (derivada, no canónica)

```json
{
  "bita_header": {
    "id": "a3f2...",
    "version": 100,
    "created_at": "2025-10-25T14:30:00Z",
    "author": "Eduardo",
    "lens_ontology": "semantic-v2",
    "compat_signature": "sha256:...",
    "checksum": "f1c2..."
  },
  "atomic_core": {
    "embedding": { "model": "multimodal-v3", "dim": 4, "type": "dense", "vec": "<bytes>" },
    "symbolic_anchors": [ { "lang": "es", "curie": "emo:esperanza", "label": "esperanza", "weight": 0.92 } ]
  },
  "relational_triples": [
    { "id": "t1", "subject": {"type":"entity","ref_id":"uuid_persona","label":"Juan"},
      "predicate":{"type":"emotional_state","label":"experimenta"},
      "object":{"type":"emotion","ref_id":"uuid_esperanza","label":"esperanza"},
      "strength":0.87, "schema":"Agente-Accion-Paciente" }
  ]
}
```

---

## 17. Checklist de Done Definition (DD)

- [ ] FBCU Core serializa CBOR canónico y valida `id`.  
- [ ] Overlays escriben/leen con `seq` monotónico.  
- [ ] LIP valida `requires` de manera determinista.  
- [ ] FlowPack ejecuta tests `smoke`.  
- [ ] Manifiesto BITA‑1 indexa todo y pasa validaciones.  
- [ ] Golden tests de canonicidad y metamórficos verdes.

---

### Fin — BITA‑1 / FBCU v1.0

> Con esto, un agente puede implementar almacenamiento, lectura, validación, overlays, lentes y flujos **sin ambigüedad**. Si necesitas un *scaffold* de repo (estructura + tests), puedo generarlo a partir de este documento.
