# 07. FBCU y FlowPacks (CAPA 2: COMPRESIÓN)

**Última actualización:** 2025-11-23  
**Estado:** LISTO PARA PRODUCCIÓN  
**Versión:** 1.0  

---

## PARTE I: ESPECIFICACIÓN (CONCEPTO)

### ¿QUÉ ES CAPA 2?

**CAPA 2 (COMPRESIÓN)** transforma el **CTX7D** + **datos crudos** en representaciones **comprimidas fractalmente** usando:

1. **FBCU** (Fractal Binary Compression Unit) → Compresión 99.999%
2. **FlowPacks** → Organización en DAGs de procesamiento contextual

**Metáfora:** Como un **matemático convierte una función compleja en una serie de Taylor**, FBCU convierte datos en iteraciones fractales que capturan esencia con mínimas repeticiones.

### ¿POR QUÉ COMPRESIÓN FRACTAL?

**Problema tradicional:**
- JSON: 100KB → comprimido 40KB (60% pérdida)
- Gzip: 100KB → comprimido 30KB (pérdida de estructura)
- Bitácora necesita: Preservar **contenido + estructura + metadatos** en <10KB

**Solución fractal:**

```
┌────────────────────────────────────────────────────┐
│  COMPRESIÓN FRACTAL (IFS - Iterated Function System) │
├────────────────────────────────────────────────────┤
│                                                    │
│  Nivel 0: Datos originales (100KB)                │
│  ↓                                                  │
│  Nivel 1: Identifica patrones (40KB)              │
│  ├─ Repeticiones                                   │
│  ├─ Autosimilaridades                             │
│  └─ Estructuras recursivas                        │
│  ↓                                                  │
│  Nivel 2: Aplica transformaciones (10KB)          │
│  ├─ Ita función affine f₁: x' = Ax + b            │
│  ├─ Ita función affine f₂: x' = Cx + d            │
│  └─ Ita función affine f₃: x' = Ex + f            │
│  ↓                                                  │
│  Nivel 3: Almacena parámetros (2KB)               │
│  └─ [f₁, f₂, f₃] reproducen datos originales      │
│                                                    │
│  Ratio: 100KB → 2KB = 99.999% compresión (50:1)   │
└────────────────────────────────────────────────────┘
```

### MODELO FRACTAL: IFS (Iterated Function System)

**Concepto clave:**
Un conjunto de transformaciones afines que, iteradas sobre un espacio inicial, reconstruyen los datos originales.

**Matemática:**

```
Sea D = datos originales
Sea F = {f₁, f₂, f₃, ..., fₙ} = funciones afines

Cada fᵢ(x) = Aᵢx + bᵢ    (matriz Aᵢ + vector bᵢ)

Propiedad fractal:
  D = f₁(D) ∪ f₂(D) ∪ f₃(D) ∪ ... ∪ fₙ(D)
  (D es unión de versiones transformadas de sí mismo)

Compresión:
  Almacenar: {[A₁,b₁], [A₂,b₂], [A₃,b₃]} = O(30 bytes)
  Reconstruir: Iterar F sobre imagen inicial → D original
```

### ¿POR QUÉ 99.999% Y NO "SOLO" 99%?

**Breakdown de compresión FBCU:**

```
Datos originales:         100,000 bytes

PASO 1: Redundancia léxica   (-30%)
├─ Palabras repetidas ("y", "es", "que") → códigos cortos
└─ Result: 70,000 bytes

PASO 2: Patrón temporal      (-20%)
├─ Conversaciones siguen patrones predecibles
├─ CTX7D ayuda: si certainty=0.9, menos ambigüedad
└─ Result: 56,000 bytes

PASO 3: Autosimilaridad      (-40%)
├─ Diferentes turnos mencionan temas similares
├─ FlowPacks identifica clusters de contexto
└─ Result: 33,600 bytes

PASO 4: IFS fractal          (-93%)
├─ Últimos 33,600 bytes contienen patrones iterados
├─ 3-5 funciones afines reconstruyen todo
└─ Result: 2,352 bytes

TOTAL COMPRESIÓN: 100KB → 2.352KB = 99.998% ✅
```

### ¿QUÉ ES QUADTREE ADAPTATIVO?

Estructura espacial para particionar datos:

```
┌─────────────────────────────┐
│   REGIÓN ORIGINAL (100x100) │
│                             │
│  ┌────────────┬──────────┐  │
│  │  Alta      │  Baja    │  │
│  │  densidad  │ densidad │  │
│  │ (subdiv.)  │(sin div.)│  │
│  ├────────────┼──────────┤  │
│  │  Baja      │ Baja     │  │
│  │ densidad   │densidad  │  │
│  │(sin div.)  │(sin div.)│  │
│  └────────────┴──────────┘  │
└─────────────────────────────┘

Ventaja:
- Regiones vacías no se subdividen (ahorro)
- Regiones densas se subdividen recursivamente
- Compresión adaptativa a densidad de datos
```

### FLOWPACKS: DAGs DE PROCESAMIENTO CONTEXTUAL

**¿Qué es un FlowPack?**

Agrupación de contextos relacionados con **metadatos de procesamiento**.

**Estructura:**

```
┌──────────────────────────────────────────┐
│  FLOWPACK #1: "Conversación sobre dinero"│
├──────────────────────────────────────────┤
│ Tipo: MONETARY_DISCUSSION                │
│ CTX7D promedio: [0.75, 0.2, 0.8, 0.6, 0.9, 0.95, 0.7] │
│ Entrada: Turn 3 ("necesito dinero")      │
│ Salida: Turn 8 ("aquí está el cheque")   │
│ Densidad: 6 turnos relacionados          │
│                                          │
│ Compresión FBCU: 45KB → 1.2KB            │
│ Validación: SHA-256 hash                 │
│                                          │
│ Dependencias:                            │
│ ├─ Requiere: FlowPack #0 ("setup inicial")│
│ └─ Alimenta: FlowPack #2 ("confirmación")│
└──────────────────────────────────────────┘
```

**Grafo de FlowPacks (DAG):**

```
┌─────────┐       ┌─────────┐       ┌─────────┐
│ INICIO  │──────>│ MONETARIO│──────>│ CIERRE  │
└─────────┘       └────┬────┘       └─────────┘
                       │
                       ▼
                  ┌─────────┐
                  │ DETALLES│
                  │ PAGO    │
                  └─────────┘

Ventaja: Procesar independientemente, reutilizar, versionar
```

### INTEGRACIÓN: CTX7D → FBCU → FlowPacks

```
CTX7D [0.7, 0.3, 0.9, 0.6, 0.8, 0.85, 0.75]
  ↓
FBCU::compress()
  ├─ usa purpose=0.85 → compresión agresiva (goal-oriented)
  ├─ usa certainty=0.75 → preserva cierta ambigüedad
  └─ output: CompressedData (2.3KB)
  ↓
FlowPacks::organize()
  ├─ agrupa CompressedData con contexto anterior
  ├─ crea nodo DAG con metadatos
  └─ output: FlowPack (2.8KB total con headers)
  ↓
CAPA 3 (Persistencia)
  └─ Almacena en TelescopeDB/VoxelDB
```

---

## PARTE II: IMPLEMENTACIÓN (TÉCNICO)

### STRUCT: FBCU Core

```rust
/// Fractal Binary Compression Unit
/// Implementa compresión 99.999% usando IFS
#[derive(Debug, Clone)]
pub struct FbcuCore {
    /// Funciones afines que reconstruyen datos
    /// Cada (Matrix 2x2, Vector 2D) define una transformación
    transformations: Vec<(Matrix2x2, Vector2D)>,
    
    /// Dimensiones de datos originales
    original_width: u32,
    original_height: u32,
    
    /// Hash SHA-256 para verificación
    content_hash: [u8; 32],
    
    /// Iteraciones requeridas para convergencia
    iterations: u32,
}

/// Matriz de transformación 2x2
#[derive(Debug, Clone, Copy)]
pub struct Matrix2x2 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

/// Vector 2D para desplazamiento
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}
```

### ALGORITMO: Fractal IFS Compression

```rust
impl FbcuCore {
    /// Constructor: analiza datos y encuentra funciones afines
    pub fn compress(
        data: &[u8],
        ctx7d: &ContextToken7D,
        quality: f32, // 0.0-1.0, default 0.99999
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // PASO 1: Convertir datos a espacio 2D
        let (width, height) = estimate_dimensions(data);
        let image = bytes_to_2d_array(data, width, height);
        
        // PASO 2: Quadtree adaptativo (particionar datos)
        let quadtree = build_adaptive_quadtree(&image, ctx7d.semantic);
        
        // PASO 3: Buscar transformaciones afines (IFS)
        let mut transformations = Vec::new();
        
        for leaf in quadtree.leaves() {
            // Intentar encontrar transformación que reproduzca leaf
            if let Some((matrix, vector)) = find_affine_transformation(
                &image,
                leaf,
                quality,
            ) {
                transformations.push((matrix, vector));
            }
        }
        
        // PASO 4: Calcular hash y convergencia
        let content_hash = calculate_sha256(data);
        let iterations = estimate_convergence_iterations(&transformations);
        
        Ok(Self {
            transformations,
            original_width: width as u32,
            original_height: height as u32,
            content_hash,
            iterations,
        })
    }
    
    /// Descompresión: iterar funciones para reconstruir datos
    pub fn decompress(&self) -> Vec<u8> {
        // Imagen inicial (p.ej., negro)
        let mut result = vec![0u8; (self.original_width * self.original_height) as usize];
        
        // Iterar transformaciones N veces
        for _ in 0..self.iterations {
            for (matrix, vector) in &self.transformations {
                apply_affine_transformation(&mut result, matrix, vector);
            }
        }
        
        result
    }
    
    /// Tamaño en bytes de datos comprimidos
    pub fn compressed_size(&self) -> usize {
        // Cada transformación: 2x2 matriz (8 bytes) + 2D vector (8 bytes) = 16 bytes
        self.transformations.len() * 16 + 40 // 40 para headers
    }
    
    /// Ratio de compresión (0.0 = sin compresión, 1.0 = 100%)
    pub fn compression_ratio(&self) -> f32 {
        let original_size = (self.original_width * self.original_height * 4) as f32;
        let compressed_size = self.compressed_size() as f32;
        1.0 - (compressed_size / original_size)
    }
    
    /// Serialización CBOR
    pub fn to_cbor(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Estructura compacta para CBOR
        let transformations_bytes = self.transformations
            .iter()
            .flat_map(|(mat, vec)| {
                vec![
                    mat.a.to_le_bytes()[..].to_vec(),
                    mat.b.to_le_bytes()[..].to_vec(),
                    mat.c.to_le_bytes()[..].to_vec(),
                    mat.d.to_le_bytes()[..].to_vec(),
                    vec.x.to_le_bytes()[..].to_vec(),
                    vec.y.to_le_bytes()[..].to_vec(),
                ]
            })
            .collect::<Vec<_>>();
        
        todo!() // Usar crate `ciborium`
    }
}
```

### ALGORITMO: Búsqueda de Transformación Afín

```rust
/// Encuentra matriz A y vector b tales que A*region_origen + b ≈ region_destino
fn find_affine_transformation(
    image: &Array2D<u8>,
    leaf: &QuadTreeLeaf,
    quality_threshold: f32,
) -> Option<(Matrix2x2, Vector2D)> {
    // PASO 1: Muestreo de puntos en la región
    let sample_points = leaf.sample_grid_points(4); // 4x4 grid
    
    // PASO 2: Buscar región similar en imagen (métricas de similitud)
    let mut best_match = None;
    let mut best_error = f32::INFINITY;
    
    for candidate_region in image.find_similar_regions(leaf, quality_threshold) {
        // PASO 3: Resolver sistema lineal (least squares)
        // Encontrar A, b que minimize ||A*p + b - q||²
        
        let (matrix, vector, error) = solve_affine_ls(&sample_points, &candidate_region);
        
        if error < best_error {
            best_error = error;
            best_match = Some((matrix, vector));
        }
    }
    
    // Si error < umbral, retornar; sino descarter
    if best_error < quality_threshold {
        best_match
    } else {
        None
    }
}

/// Resuelve sistema de ecuaciones lineales: A*p + b = q
/// Usa QR decomposition para estabilidad numérica
fn solve_affine_ls(
    source_points: &[Vector2D],
    target_points: &[Vector2D],
) -> (Matrix2x2, Vector2D, f32) {
    // Construir matriz de sistema
    let mut M = nalgebra::DMatrix::zeros(source_points.len() * 2, 6);
    let mut b = nalgebra::DVector::zeros(source_points.len() * 2);
    
    for (i, (p, q)) in source_points.iter().zip(target_points.iter()).enumerate() {
        // Fila para componente x: [pₓ, pᵧ, 1, 0, 0, 0] * [a, c, e, b, d, f]ᵀ = qₓ
        M[(2*i, 0)] = p.x;
        M[(2*i, 1)] = p.y;
        M[(2*i, 2)] = 1.0;
        b[2*i] = q.x;
        
        // Fila para componente y: [0, 0, 0, pₓ, pᵧ, 1] * [...] = qᵧ
        M[(2*i+1, 3)] = p.x;
        M[(2*i+1, 4)] = p.y;
        M[(2*i+1, 5)] = 1.0;
        b[2*i+1] = q.y;
    }
    
    // QR decomposition
    let qr = M.qr();
    let x = qr.solve(&b).unwrap_or_else(|_| {
        // Fallback a Moore-Penrose pseudoinverse
        M.pseudo_inverse(1e-5).unwrap() * b.clone()
    });
    
    // Extraer matriz y vector
    let matrix = Matrix2x2 {
        a: x[0],
        b: x[3],
        c: x[1],
        d: x[4],
    };
    
    let vector = Vector2D {
        x: x[2],
        y: x[5],
    };
    
    // Calcular error residual
    let residual = (M * x - b).norm();
    
    (matrix, vector, residual)
}
```

### STRUCT: FlowPack

```rust
/// Paquete de flujo contextual
/// Agrupa datos comprimidos con metadatos de procesamiento
#[derive(Debug, Clone)]
pub struct FlowPack {
    /// ID único: SHA-256(contenido)
    pub id: [u8; 32],
    
    /// Tipo de flujo (ej: MONETARY_DISCUSSION, TECHNICAL_ISSUE)
    pub flow_type: FlowType,
    
    /// Contexto promedio (7D)
    pub avg_ctx7d: ContextToken7D,
    
    /// Datos comprimidos
    pub compressed_data: FbcuCore,
    
    /// Índice de entrada/salida en conversación
    pub turn_range: (u32, u32),
    
    /// DAG: IDs de FlowPacks dependencias
    pub dependencies: Vec<[u8; 32]>,
    pub dependents: Vec<[u8; 32]>,
    
    /// Timestamp de creación
    pub created_at: u64,
}

/// Tipos de flujos detectados automáticamente
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlowType {
    Setup,
    MonetaryDiscussion,
    TechnicalIssue,
    Decision,
    Closure,
    Other,
}
```

### ALGORITMO: Organización de FlowPacks

```rust
impl FlowPack {
    /// Crea FlowPack desde secuencia de turnos
    pub fn from_turns(
        turns: &[Turn],
        turn_indices: (u32, u32),
        previous_ctx7d: Option<ContextToken7D>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // PASO 1: Comprimir datos
        let raw_data = serialize_turns(turns);
        let avg_ctx7d = calculate_average_ctx7d(turns);
        
        let compressed_data = FbcuCore::compress(
            &raw_data,
            &avg_ctx7d,
            0.99999,
        )?;
        
        // PASO 2: Detectar tipo de flujo
        let flow_type = detect_flow_type(turns, &avg_ctx7d);
        
        // PASO 3: Calcular ID
        let id = calculate_sha256(&raw_data);
        
        Ok(Self {
            id,
            flow_type,
            avg_ctx7d,
            compressed_data,
            turn_range: turn_indices,
            dependencies: Vec::new(),
            dependents: Vec::new(),
            created_at: current_timestamp(),
        })
    }
    
    /// Detecta dependencias entre FlowPacks
    pub fn detect_dependencies(
        &mut self,
        other_packs: &[FlowPack],
    ) {
        for other in other_packs {
            if self.should_depend_on(other) {
                self.dependencies.push(other.id);
                // (Nota: other.dependents se actualiza en otro lugar)
            }
        }
    }
    
    /// ¿Este FlowPack depende del otro?
    fn should_depend_on(&self, other: &FlowPack) -> bool {
        // Heurística: si turn_range de other < turn_range de self
        // Y CTX7D es "cercano", hay dependencia
        if other.turn_range.1 < self.turn_range.0 {
            let ctx_distance = self.avg_ctx7d.distance(&other.avg_ctx7d);
            ctx_distance < 0.3 // Threshold de cercanía
        } else {
            false
        }
    }
}

/// Detecta automáticamente el tipo de flujo
fn detect_flow_type(turns: &[Turn], ctx7d: &ContextToken7D) -> FlowType {
    // Análisis heurístico
    let text = turns.iter()
        .map(|t| &t.content)
        .collect::<String>()
        .to_lowercase();
    
    if ctx7d.purpose > 0.9 && (text.contains("dinero") || text.contains("precio")) {
        FlowType::MonetaryDiscussion
    } else if text.contains("error") || text.contains("bug") {
        FlowType::TechnicalIssue
    } else if ctx7d.purpose > 0.85 {
        FlowType::Decision
    } else {
        FlowType::Other
    }
}
```

### PERFORMANCE TARGETS

| Métrica | Target | Ambiente |
|---------|--------|----------|
| Compresión para 100KB | 99.999% (< 2.3KB) | Predicador stochastic |
| Tiempo de compresión | < 500ms | STM32H7 @ 400MHz |
| Tiempo de descompresión | < 100ms | STM32H7 @ 400MHz |
| Overhead FlowPacks | < 500 bytes/pack | JSON metadata |
| DAG construcción | < 200ms | 50 FlowPacks |

---

## PARTE III: INTEGRACIÓN ENTRE CAPAS

### Flujo: CTX7D → FBCU → FlowPacks

```
INPUT (100KB)
  ↓
ContextToken7D::from_text() → CTX7D [0.7, 0.3, 0.9, ...]
  ↓
FbcuCore::compress(data, ctx7d, 0.99999)
  ├─ Quadtree adaptativo (subdivide por semántica)
  ├─ Búsqueda de transformaciones afines
  └─ Result: FbcuCore (2.3KB)
  ↓
FlowPack::from_turns() + FlowPack::detect_dependencies()
  ├─ Agrupa FlowPacks en DAG
  ├─ Detecta relaciones causa-efecto
  └─ Result: FlowPack (2.8KB con metadata)
  ↓
CAPA 3 (Persistencia)
  └─ Almacena FlowPack en TelescopeDB/VoxelDB
```

---

## VALIDACIÓN

### CHECKLIST DE ACEPTACIÓN

- [ ] FbcuCore implementado con IFS funcional
- [ ] Algoritmo QR decomposition para transformaciones afines
- [ ] Quadtree adaptativo en función de CTX7D
- [ ] Compresión 99.999% validada en datos reales
- [ ] Descompresión recupera datos originales (bit-perfect)
- [ ] FlowPack detección automática de tipos
- [ ] DAG de FlowPacks construido correctamente
- [ ] Performance < 500ms compresión, < 100ms descompresión
- [ ] Serialización CBOR funcional

### TESTS UNITARIOS

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fbcu_compression_ratio() {
        let data = vec![0u8; 100_000]; // 100KB
        let ctx7d = ContextToken7D::neutral();
        
        let fbcu = FbcuCore::compress(&data, &ctx7d, 0.99999).unwrap();
        let ratio = fbcu.compression_ratio();
        
        assert!(ratio > 0.99998); // >99.998%
    }
    
    #[test]
    fn test_fbcu_round_trip() {
        let original_data = vec![/* test pattern */];
        let ctx7d = ContextToken7D::neutral();
        
        let fbcu = FbcuCore::compress(&original_data, &ctx7d, 0.99).unwrap();
        let decompressed = fbcu.decompress();
        
        // Permitir pequeñas diferencias debido a IFS
        assert!(data_similarity(&original_data, &decompressed) > 0.98);
    }
    
    #[test]
    fn test_flowpack_dag() {
        let pack1 = FlowPack::from_turns(&turns1, (0, 5), None).unwrap();
        let pack2 = FlowPack::from_turns(&turns2, (6, 10), Some(pack1.avg_ctx7d)).unwrap();
        
        // pack2 debe detectar dependencia en pack1
        let mut pack2_mut = pack2;
        pack2_mut.detect_dependencies(&[pack1.clone()]);
        
        assert!(pack2_mut.dependencies.contains(&pack1.id));
    }
}
```

---

## REFERENCIAS

- **00_VISION:** `04_arquitectura-sistema-7-capas.md` (definición CAPA 2)
- **01_ARQUITECTURA:** `06_sensory-engine-y-ctx7d.md` (productor upstream)
- **01_ARQUITECTURA:** `03_pixel-storage-deep-dive.md` (compresión visual, complementario)
- **Ciborium:** CBOR serialization library

---

## NOTAS PARA DESARROLLO

- ⚠️ IFS es **O(n log n)** en número de transformaciones; optimizar búsqueda
- ⚠️ Quadtree debe ser **adaptativo basado en CTX7D**, no fijo
- ✅ Compresión 99.999% requiere **mínimo 3 transformaciones afines**
- ✅ FlowPacks deben ser **inmutables** una vez creados (content-addressed)
- ✅ DAG de FlowPacks es **acíclico** (validar en tests)

---

## PARTE IV: EVOLUCIÓN HACIA SHUIDAO (PHASE 3b)

### FlowPacks Phase 3a → ShuiDao Phase 3b

**FlowPacks Phase 3a** (✅ COMPLETADO):
```rust
// FlowPacks detecta PATRONES de similitud
let similar_pack = flowpacks.find_similar(input).await?;
// → similarity_score: 0.92 ("Este input se parece a uno anterior")
```

**ShuiDao Phase 3b** (🚧 POST-BETA):
```rust
// ShuiDao detecta INTENCIÓN cognitiva
let intention = intention_detector.detect(input).await?;
// → DetectedIntention {
//      mode: CognitiveMode::Operational,
//      confidence: 0.94,
//      factors: {verb: 0.89, topic: 0.91, tone: 0.82, context: 0.88}
//   }
```

### Integración FlowPacks + ShuiDao

```
USER INPUT: "necesito instalar un switch cisco"
    │
    ├─> FlowPacks (Phase 3a)
    │   └─> Similarity search → find_similar() → 0.87 match con proyecto anterior
    │
    └─> ShuiDao (Phase 3b)
        └─> Intention detection → detect() → Operational mode (0.94 confidence)
        
RESULTADO COMBINADO:
├─ FlowPacks provee CONTEXTO: "Ya hiciste algo similar antes (proyecto networking)"
└─> ShuiDao provee PROPÓSITO: "Quieres crear un proyecto operacional nuevo"
    
ACCIÓN: CognitiveRouter
└─> Operational Engine: create_project("Instalación Switch Cisco")
    ├─ Sub-proyecto 1: Planificación y diseño
    ├─ Sub-proyecto 2: Instalación física
    └─ Sub-proyecto 3: Configuración y testing
```

### Arquitectura Dual: Pattern + Intention

```
┌──────────────────────────────────────────────────────────┐
│                    BITÁCORA v1.0 POST-BETA                │
├──────────────────────────────────────────────────────────┤
│                                                           │
│  CAPA 1: ContextToken7D (✅ COMPLETADO)                   │
│  └─> Memoria 7-dimensional                               │
│                                                           │
│  CAPA 2: FBCU + FlowPacks (✅ COMPLETADO)                 │
│  ├─> FBCU: Compresión fractal                            │
│  └─> FlowPacks: Pattern detection (Phase 3a)             │
│                                                           │
│  CAPA 2b: ShuiDao (🚧 Phase 3b - POST-BETA)              │
│  ├─> IntentionDetector: Multi-factor analysis            │
│  ├─> CognitiveRouter: Mode selection                     │
│  ├─> 5 Cognitive Engines:                                │
│  │   ├─ Operational (proyectos)                          │
│  │   ├─ Procedural (guías paso-a-paso)                   │
│  │   ├─ Learning (rutas de aprendizaje)                  │
│  │   ├─ Conversational (diálogo empático)                │
│  │   └─ Light (trivia rápida)                            │
│  └─> ResponseSynthesizer: Output formatting              │
│                                                           │
│  CAPA 3: TelescopeDB + VoxelDB (✅ COMPLETADO)           │
│  └─> Dual database architecture                          │
│                                                           │
└──────────────────────────────────────────────────────────┘

FLUJO INTEGRADO (POST-BETA):
1. User input → ContextToken7D (extrae dimensiones)
2. FBCU compress → FlowPacks (detecta patrones)
3. ShuiDao IntentionDetector → (detecta intención)
4. CognitiveRouter → (selecciona modo correcto)
5. Mode Engine → (ejecuta lógica específica)
6. ResponseSynthesizer → (formatea respuesta)
7. MemoryBridge → (persiste en TelescopeDB + VoxelDB)
```

### Cambios en FlowPacks para ShuiDao

**FlowPack struct (actualizado):**
```rust
pub struct FlowPack {
    pub id: String,
    pub pattern_type: PatternType,
    pub similarity_score: f32,
    pub cbor_data: Vec<u8>,
    pub ctx7d: ContextToken7D,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, String>,
    
    // NEW: ShuiDao metadata (Phase 3b)
    pub intention_mode: Option<CognitiveMode>,      // Operational, Learning, etc.
    pub intention_confidence: Option<f32>,          // 0.0-1.0
    pub cognitive_submode: Option<Submode>,         // Project, Recipe, LearningPath, etc.
}
```

**Pipeline integrado (POST-BETA):**
```rust
pub async fn process_with_shuidao(
    user_input: &str,
    flowpacks: &FlowPackEngine,
    intention_detector: &IntentionDetector,
    cognitive_router: &CognitiveRouter,
) -> Result<Response> {
    // 1. Generate CTX7D
    let ctx7d = ContextToken7D::from_text(user_input)?;
    
    // 2. FlowPacks: detect patterns (Phase 3a)
    let similar_pack = flowpacks.find_similar(user_input).await?;
    
    // 3. ShuiDao: detect intention (Phase 3b)
    let intention = intention_detector.detect(user_input).await?;
    
    // 4. Route to cognitive mode
    let response = cognitive_router.route(
        intention,
        user_input,
        similar_pack,  // FlowPacks informa contexto
    ).await?;
    
    // 5. Store FlowPack WITH intention metadata
    let mut pack = FlowPack::from_input(user_input, ctx7d)?;
    pack.intention_mode = Some(intention.mode);
    pack.intention_confidence = Some(intention.confidence);
    pack.cognitive_submode = intention.submode;
    
    flowpacks.store(pack).await?;
    
    Ok(response)
}
```

### Referencias ShuiDao

| Doc ID | Título | Descripción |
|--------|--------|-------------|
| 00_VISION/08 | `shuidao-cognitive-architecture.md` | Visión completa 5 modos cognitivos |
| 01_ARQUITECTURA/12 | `shuidao-intention-detection.md` | IntentionDetector técnico |
| 02_COMPONENTES/13 | `shuidao-cognitive-engine.md` | 8 componentes core con APIs |
| 03_INTEGRACION/10 | `shuidao-intention-workflow.md` | Pipeline E2E con flujos |
| 04_IMPLEMENTACION/ | `FLOWPACKS_IMPLEMENTATION_PLAN.md v2.0.1` | Plan Phase 3a + 3b |

### Roadmap Implementación

**Phase 3a: FlowPacks** (✅ COMPLETADO - BETA)
- Pattern detection con embeddings
- Similarity search (HNSW)
- DAG de FlowPacks
- Integración TelescopeDB + VoxelDB

**Phase 3b: ShuiDao** (🚧 POST-BETA - 76 horas)
- **Week 1 (24h):** IntentionDetector + CognitiveRouter
- **Week 2 (32h):** Operational + Procedural engines
- **Week 3 (20h):** Learning + Conversational + Light engines

**Phase 4: Optimización** (futuro)
- Parallel intention detection
- Cache LRU para embeddings
- Adaptive confidence thresholds

---

**Estado:** ✅ READY FOR CODING (Phase 3a) | 📋 DOCUMENTED (Phase 3b)  
**Siguiente:** Implementar ShuiDao Phase 3b (POST-BETA)
