```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/05_telescopedb.md
Versión: 2.0
Fecha Creación: 2025-01-25
Fecha Actualización: 2025-10-26
Autor: Sistema Bitácora - Template MTT-DSL component_spec.yaml
Propósito: Especificación completa del componente TelescopeDB (memoria biográfica esférica)
Estado: ESPECIFICACIÓN - Pendiente implementación
Relacionado Con: BITA-2_ACA-7D_SPECIFICATION.md, SISTEMA_DUAL_DATABASES.md
Implementa: DA-007 (TelescopeDB brecha crítica #1), DA-001 (Local-first), DA-011 (NO MongoDB)
Template Usado: 07_TEMPLATES/component_spec.yaml v1.0
# === FIN DATOS DE AUDITORÍA ===
```

# 🔭 TELESCOPEDB - Base de Datos Biográfica Esférica

---

## 🎯 PROPÓSITO

**TelescopeDB** es el primer componente del **sistema dual-helix** de Bitácora v1.0, diseñado para almacenar y recuperar **memoria biográfica del usuario** en geometría esférica (r, θ, φ).

### El Problema que Resuelve

Los LLMs tradicionales no tienen memoria persistente de tus interacciones. Cada conversación es un "reset". TelescopeDB soluciona esto almacenando tu **historia personal completa** de forma local y privada.

**Escenario real:**
```
Usuario: "¿Recuerdas cuando debuggeamos ese problema de ownership hace 2 semanas?"

Sin TelescopeDB:
→ LLM no tiene acceso a historia previa
→ Respuesta genérica: "No tengo memoria de conversaciones anteriores"
→ Usuario debe re-explicar contexto completo

Con TelescopeDB:
→ Query esférica: r=0.8 (alta intensidad), θ=0.3 (técnico/debugging), φ=1.2 (frustración inicial)
→ Recupera FBCU Core de esa sesión (comprimido 99.999%)
→ Contextualiza: "Sí, el problema era un borrow checker conflict con Arc<Mutex<T>>. Lo resolvimos usando channels."
→ Respuesta personalizada en <50ms, 100% local
```

### Por Qué es Crítico

1. **Privacidad Total:** Toda tu historia queda en disco local (DA-001)
2. **Eficiencia:** Compresión fractal >99.99% (FBCU)
3. **Contextualización:** Alimenta dimensión #6 de Context Token 7D
4. **Sincronización Dual-Helix:** TelescopeDB + VoxelDB = memoria completa (episódica + procedimental)

### Relación con Arquitectura General

TelescopeDB es el **"cerebro episódico"** de Bitácora:
- TelescopeDB → Memoria episódica (qué pasó, cuándo, cómo te sentiste)
- VoxelDB → Memoria procedimental (qué templates funcionaron)
- Context Token 7D → Motor cognitivo (interpreta ambas)

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
┌─────────────────────────────────────────────────────────────┐
│                    BITÁCORA v1.0 PIPELINE                   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
               ┌──────────────────────────┐
               │   SENSORY ENGINE         │
               │   (Procesa input)        │
               └────────────┬─────────────┘
                            │
                            ▼
         ┌──────────────────────────────────────┐
         │    Context Token 7D Generator        │
         │    (Analiza en 7 dimensiones)        │
         └────────────┬─────────────────────────┘
                      │
                      ▼
    ┌─────────────────────────────────────────┐
    │         TELESCOPEDB                     │ ← AQUÍ ESTAMOS
    │    (Almacena FBCU Core)                 │
    │  • Compresión fractal 99.999%           │
    │  • Geometría esférica (r,θ,φ)           │
    │  • Query contextual <50ms               │
    └────────────┬────────────────────────────┘
                 │
                 ├─────────────────────┐
                 │                     │
                 ▼                     ▼
   ┌──────────────────┐      ┌──────────────────┐
   │   VOXELDB        │      │  Context Engine   │
   │  (Templates)     │←─────│  (Genera resp.)   │
   │                  │ sync │                   │
   └──────────────────┘      └──────────────────┘
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito | Frecuencia |
|------------|-----------|-----------|------------|
| **Context Token 7D** | → TelescopeDB | Guardar experiencia procesada | Cada interacción |
| **TelescopeDB** | → VoxelDB | Sincronizar metadatos semánticos | Async background |
| **Context Intelligence** | → TelescopeDB | Query historia relevante | 70% de requests |
| **FBCU Engine** | ↔ TelescopeDB | Compresión/descompresión | Cada insert/read |
| **SENSORY ENGINE** | → TelescopeDB | Persistir input multimodal | Cada entrada usuario |

### Qué Depende de TelescopeDB

**Crítico (no puede funcionar sin TelescopeDB):**
- Context Token 7D (dimensión #6 biográfica)
- Context Intelligence (recuperación de historia)
- Expertise Generation (análisis de patrones)

**Importante (degraded mode sin TelescopeDB):**
- VoxelDB (funciona pero sin referencias biográficas)
- Breakthrough Detection (funciona pero sin contexto histórico)

---

## 📋 RESPONSABILIDADES CORE

TelescopeDB tiene **8 responsabilidades fundamentales**:

### 1. **Almacenamiento de FBCU Cores** (MUST HAVE)
- Guardar experiencias como FBCU Cores (Fractal-Based Compression Unit)
- Compresión fractal >99.99% (validado en quantum compressor)
- Storage: ~2 KB por core comprimido

### 2. **Indexación Esférica** (MUST HAVE)
- Coordenadas esféricas (r, θ, φ) calculadas desde Context Token 7D
- r = Intensidad emocional/intencional
- θ = Categoría temática (técnica, personal, social...)
- φ = Valencia emocional (positivo/neutral/negativo)

### 3. **Query Contextual Rápido** (MUST HAVE)
- Búsqueda por vecindad esférica: "experiencias similares en coordenadas"
- Tiempo objetivo: <50ms para top-10 resultados
- Precision@10 > 85%

### 4. **Query por Embeddings** (MUST HAVE)
- Similarity search usando embeddings del FBCU Core
- ANN (Approximate Nearest Neighbors) con HNSW
- Cosine similarity > 0.7 para resultados relevantes

### 5. **Sincronización con VoxelDB** (MUST HAVE)
- Exportar metadatos semánticos a VoxelDB
- Hash bidireccional: TelescopeDB ←→ VoxelDB
- Mantener coherencia entre memoria episódica y procedimental

### 6. **Import desde src/sandbox/** (MUST HAVE - DA-014)
- Parsear archivos biográficos existentes
- Convertir a FBCU Cores
- Validar formato y calcular coordenadas

### 7. **Versionado de Cores** (NICE TO HAVE)
- Almacenar múltiples versiones de una experiencia
- Rollback si actualización corrompe datos
- Git-like branching interno

### 8. **Storage Opcional en PNG** (NICE TO HAVE)
- Encodear FBCU Cores como píxeles en imagen PNG
- Visual debug: cada píxel = una experiencia
- Compresión adicional via PNG compression

---

## 🗂️ ESTRUCTURAS DE DATOS

### Estructura Principal: TelescopeDB

```rust
// src/cells/telescopedb/mod.rs

pub struct TelescopeDB {
    /// Directorio raíz de almacenamiento
    storage_path: PathBuf,  // .bitacora/telescope/
    
    /// Mapa de cores (content-addressable por SHA-256)
    cores: HashMap<String, FBCUCore>,
    
    /// Índice esférico (r,θ,φ)
    spherical_index: SphericalIndex,
    
    /// Índice de embeddings (HNSW para ANN)
    ann_index: HnswIndex,
    
    /// Motor de compresión fractal
    compressor: FractalCompressor,
    
    /// Opcional: Pixel storage en PNG
    pixel_storage: Option<PixelEncoder>,
    
    /// Referencia a VoxelDB (sincronización)
    voxel_ref: Option<Arc<RwLock<VoxelDB>>>,
    
    /// Métricas de uso
    metrics: TelescopeMetrics,
}

/// FBCU Core - Unidad comprimida de memoria biográfica
pub struct FBCUCore {
    /// ID único (SHA-256 del contenido)
    pub id: String,
    
    /// Header BITA-2
    pub bita_header: BitaHeader,
    
    /// Núcleo atómico (embeddings + anchors)
    pub atomic_core: AtomicCore,
    
    /// Relaciones semánticas
    pub relational_triples: Vec<Triple>,
    
    /// Tensor de contexto 7D completo
    pub context_tensor: ContextTensor7D,
    
    /// Provenance (origen del dato)
    pub provenance: Provenance,
    
    /// Coordenadas esféricas
    pub coords: SphericalCoords,
}

/// Coordenadas esféricas
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphericalCoords {
    /// Intensidad: [0, ∞)
    /// Calculado de: dimensión emocional + intencional
    pub r: f64,
    
    /// Categoría temática: [0, 2π)
    /// 0 = técnico, π/3 = personal, 2π/3 = social, π = creativo, etc.
    pub theta: f64,
    
    /// Valencia emocional: [0, π]
    /// 0 = muy positivo, π/2 = neutral, π = muy negativo
    pub phi: f64,
}

impl SphericalCoords {
    /// Distancia euclidiana en espacio esférico
    pub fn distance(&self, other: &Self) -> f64 {
        // Fórmula: d = sqrt(r1² + r2² - 2*r1*r2*cos(angle))
        // donde angle = arccos(sin(φ1)*sin(φ2)*cos(θ1-θ2) + cos(φ1)*cos(φ2))
        
        let angle = (self.phi.sin() * other.phi.sin() * (self.theta - other.theta).cos()
                    + self.phi.cos() * other.phi.cos()).acos();
        
        (self.r.powi(2) + other.r.powi(2) - 2.0 * self.r * other.r * angle.cos()).sqrt()
    }
    
    /// Convertir a coordenadas cartesianas (para visualización)
    pub fn to_cartesian(&self) -> (f64, f64, f64) {
        let x = self.r * self.phi.sin() * self.theta.cos();
        let y = self.r * self.phi.sin() * self.theta.sin();
        let z = self.r * self.phi.cos();
        (x, y, z)
    }
}

/// Tensor de contexto 7D (del Context Token 7D)
pub struct ContextTensor7D {
    pub semantic: f64,      // Dimensión 1
    pub syntactic: f64,     // Dimensión 2
    pub emotional: f64,     // Dimensión 3
    pub intentional: f64,   // Dimensión 4
    pub contextual: f64,    // Dimensión 5
    pub biographical: f64,  // Dimensión 6 ← TelescopeDB alimenta esta
    pub relational: f64,    // Dimensión 7
}

impl ContextTensor7D {
    /// Calcular coordenadas esféricas desde tensor 7D
    pub fn to_spherical_coords(&self) -> SphericalCoords {
        SphericalCoords {
            // Intensidad = función de emocional + intencional
            r: ((self.emotional.powi(2) + self.intentional.powi(2)) / 2.0).sqrt(),
            
            // Categoría = función de semántico + contextual
            theta: (self.semantic.atan2(self.contextual) + std::f64::consts::PI) % (2.0 * std::f64::consts::PI),
            
            // Valencia = función de emocional normalizada
            phi: (1.0 - self.emotional).clamp(0.0, 1.0) * std::f64::consts::PI,
        }
    }
}

/// Núcleo atómico del FBCU Core
pub struct AtomicCore {
    /// Embedding principal (1536 dims para OpenAI, 768 para local)
    pub embedding: Embedding,
    
    /// Anchors semánticos (palabras clave)
    pub anchors: Vec<String>,
    
    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,
    
    /// Contenido original (comprimido)
    pub content: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Embedding {
    pub vec: Vec<f32>,
    pub model: String,  // "text-embedding-ada-002", "local-bge", etc.
}
```

---

## 🔌 API PÚBLICA

### Operaciones Principales

```rust
impl TelescopeDB {
    /// Crear nueva instancia de TelescopeDB
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        let spherical_index = SphericalIndex::new()?;
        let ann_index = HnswIndex::new(dimension: 1536, m: 16, ef_construction: 200)?;
        let compressor = FractalCompressor::new()?;
        
        Ok(Self {
            storage_path,
            cores: HashMap::new(),
            spherical_index,
            ann_index,
            compressor,
            pixel_storage: None,
            voxel_ref: None,
            metrics: TelescopeMetrics::default(),
        })
    }
    
    /// Conectar con VoxelDB (para sincronización)
    pub fn connect_voxeldb(&mut self, voxeldb: Arc<RwLock<VoxelDB>>) {
        self.voxel_ref = Some(voxeldb);
    }
    
    /// Insertar FBCU Core desde Context Token 7D
    pub async fn insert_from_ctx7d(&mut self, token: &ContextToken7D) -> Result<String> {
        // 1. Convertir CTX7D a FBCU Core
        let core = self.ctx7d_to_fbcu_core(token)?;
        
        // 2. Calcular coordenadas esféricas
        let coords = token.context_tensor.to_spherical_coords();
        
        // 3. Comprimir con algoritmo fractal
        let compressed_content = self.compressor.compress(&core.atomic_core.content)?;
        
        // 4. Calcular ID content-addressable
        let id = self.compute_content_id(&compressed_content)?;
        
        // 5. Crear FBCU Core completo
        let fbcu_core = FBCUCore {
            id: id.clone(),
            bita_header: BitaHeader::default(),
            atomic_core: AtomicCore {
                embedding: core.atomic_core.embedding.clone(),
                anchors: core.atomic_core.anchors.clone(),
                timestamp: Utc::now(),
                content: compressed_content,
            },
            relational_triples: core.relational_triples.clone(),
            context_tensor: token.context_tensor.clone(),
            provenance: Provenance::from_ctx7d(token),
            coords,
        };
        
        // 6. Guardar en disco
        self.save_fbcu_core(&fbcu_core)?;
        
        // 7. Indexar en geometría esférica
        self.spherical_index.insert(coords, id.clone())?;
        
        // 8. Indexar embedding en HNSW
        self.ann_index.add(id.clone(), &fbcu_core.atomic_core.embedding.vec)?;
        
        // 9. Añadir a HashMap en memoria
        self.cores.insert(id.clone(), fbcu_core.clone());
        
        // 10. Opcional: Guardar píxel en PNG
        if let Some(ref mut pixel_encoder) = self.pixel_storage {
            pixel_encoder.encode_and_save(&fbcu_core, coords)?;
        }
        
        // 11. Notificar a VoxelDB (si conectado)
        if let Some(ref voxeldb) = self.voxel_ref {
            let vdb = voxeldb.write().await;
            // TODO: Export metadata to VoxelDB
        }
        
        Ok(id)
    }
    
    /// Query contextual: buscar experiencias similares por coordenadas
    pub async fn query_contextual(&self, coords: SphericalCoords, radius: f64) -> Result<Vec<FBCUCore>> {
        // 1. Buscar en índice esférico dentro de radio
        let candidate_ids = self.spherical_index.query_sphere(&coords, radius)?;
        
        // 2. Cargar cores y calcular distancia exacta
        let mut results: Vec<(FBCUCore, f64)> = candidate_ids.iter()
            .filter_map(|id| {
                self.load_fbcu_core(id).ok().map(|core| {
                    let distance = coords.distance(&core.coords);
                    (core, distance)
                })
            })
            .collect();
        
        // 3. Ordenar por distancia (más cercano primero)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        // 4. Retornar top-10
        Ok(results.into_iter().take(10).map(|(c, _)| c).collect())
    }
    
    /// Query semántico: buscar por similaridad de embeddings
    pub async fn query_semantic(&self, query: &str) -> Result<Vec<(FBCUCore, f64)>> {
        // 1. Generar embedding del query (usando mismo modelo que cores)
        let query_embedding = self.generate_embedding(query).await?;
        
        // 2. Búsqueda ANN en HNSW
        let neighbors = self.ann_index.search(&query_embedding.vec, k: 10, ef: 50)?;
        
        // 3. Cargar cores con similaridad
        let results: Vec<(FBCUCore, f64)> = neighbors.iter()
            .filter_map(|(id, similarity)| {
                self.load_fbcu_core(id).ok().map(|core| (core, *similarity))
            })
            .collect();
        
        Ok(results)
    }
    
    /// Query híbrido: contextual + semántico
    pub async fn query_hybrid(
        &self,
        coords: SphericalCoords,
        radius: f64,
        query: &str,
    ) -> Result<Vec<FBCUCore>> {
        // 1. Query contextual (filtro espacial)
        let spatial_results = self.query_contextual(coords, radius).await?;
        
        // 2. Query semántico dentro de resultados espaciales
        let query_embedding = self.generate_embedding(query).await?;
        
        let mut scored: Vec<(FBCUCore, f64)> = spatial_results.into_iter()
            .map(|core| {
                let similarity = cosine_similarity(
                    &query_embedding.vec,
                    &core.atomic_core.embedding.vec
                );
                (core, similarity)
            })
            .collect();
        
        // 3. Ordenar por similaridad semántica
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // 4. Retornar top-10
        Ok(scored.into_iter().take(10).map(|(c, _)| c).collect())
    }
    
    /// Import desde src/sandbox/ (DA-014)
    pub async fn import_from_sandbox(&mut self, sandbox_path: &Path) -> Result<ImportStats> {
        let mut stats = ImportStats::default();
        
        // 1. Listar archivos en sandbox
        let entries = std::fs::read_dir(sandbox_path)?;
        
        for entry in entries {
            let path = entry?.path();
            
            // 2. Parsear archivo biográfico
            match self.parse_biographical_file(&path) {
                Ok(ctx7d_tokens) => {
                    // 3. Insertar cada token como FBCU Core
                    for token in ctx7d_tokens {
                        match self.insert_from_ctx7d(&token).await {
                            Ok(_) => stats.success += 1,
                            Err(e) => {
                                stats.failed += 1;
                                stats.errors.push(format!("{}: {}", path.display(), e));
                            }
                        }
                    }
                }
                Err(e) => {
                    stats.failed += 1;
                    stats.errors.push(format!("{}: {}", path.display(), e));
                }
            }
        }
        
        Ok(stats)
    }
    
    /// Exportar metadatos a VoxelDB (sincronización)
    pub async fn sync_to_voxeldb(&self) -> Result<()> {
        if let Some(ref voxeldb) = self.voxel_ref {
            let vdb = voxeldb.write().await;
            
            for (id, core) in &self.cores {
                // TODO: Export semantic metadata to VoxelDB
                // Permite que VoxelDB referencie experiencias biográficas
            }
        }
        
        Ok(())
    }
}

#[derive(Default)]
pub struct ImportStats {
    pub success: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}
```

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito | Crítico |
|------------|---------|-----------|---------|
| **Context Token 7D** | v1.0 | Fuente de FBCU Cores (cada interacción genera CTX7D) | ✅ SÍ |
| **FBCU Engine** | v1.0 | Compresión fractal del contenido | ✅ SÍ |
| **VoxelDB** | v1.0 | Sincronización dual-helix (metadata) | ❌ NO (opcional) |
| **SENSORY ENGINE** | v1.0 | Generación de input para CTX7D | ✅ SÍ |

### Crates Externos

```toml
[dependencies]
# Serialización
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Embeddings y vectores
ndarray = "0.15"
hnsw = "0.11"  # HNSW ANN index

# Hashing
sha2 = "0.10"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Dates
chrono = "0.4"

# Logging
tracing = "0.1"

# Compresión (para FBCU)
flate2 = "1.0"  # Fallback compression

# Opcional: PNG encoding
png = "0.17"  # Para pixel storage
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

### Benchmarks Esperados

| Operación | Target | Justificación | Status |
|-----------|--------|---------------|--------|
| **insert_from_ctx7d()** | <100ms | Incluye compresión fractal (validado en quantum compressor) | ⏸️ TBD |
| **query_contextual() k=10** | <50ms | Spherical index lookup es O(log n) | ⏸️ TBD |
| **query_semantic() k=10** | <50ms | HNSW ANN search (approx 90% accuracy) | ⏸️ TBD |
| **query_hybrid() k=10** | <80ms | Combinación de spatial + semantic | ⏸️ TBD |
| **import_from_sandbox()** | >95% success rate | Parseo robusto de formatos biográficos | ⏸️ TBD |
| **compression_ratio** | >99.99% | Validado en B20250915-data-compressor | ⏸️ TBD |

### Complejidad Algorítmica

| Operación | Complejidad | Notas |
|-----------|-------------|-------|
| Insert | O(log n × d) | Spherical insert + HNSW add |
| Contextual Query | O(log n + k) | Spherical query + sort |
| Semantic Query | O(log n × d) | HNSW search (approx) |
| Hybrid Query | O(log n + k×d) | Spatial + semantic scoring |

**Donde:**
- n = Número de FBCU Cores en TelescopeDB
- d = Dimensión del embedding (1536)
- k = Número de resultados solicitados

### Uso de Memoria

**Estimación para 10,000 cores:**
- Cores comprimidos: ~2 KB × 10,000 = 20 MB
- Embeddings (f32): 1536 × 4 bytes × 10,000 = 60 MB
- Índice Spherical: ~2 MB
- Índice HNSW: ~100 MB (depende de m y ef)

**Total:** ~182 MB para 10,000 experiencias biográficas (muy eficiente)

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spherical_coords_distance() {
        let c1 = SphericalCoords { r: 1.0, theta: 0.0, phi: 0.0 };
        let c2 = SphericalCoords { r: 1.0, theta: std::f64::consts::PI, phi: std::f64::consts::PI };
        
        let distance = c1.distance(&c2);
        assert!(distance > 0.0);
        assert!(distance <= 2.0);  // Max distance in unit sphere
    }
    
    #[test]
    fn test_ctx7d_to_spherical_conversion() {
        let tensor = ContextTensor7D {
            semantic: 0.8,
            syntactic: 0.6,
            emotional: 0.9,  // High emotional intensity
            intentional: 0.7,
            contextual: 0.5,
            biographical: 0.4,
            relational: 0.3,
        };
        
        let coords = tensor.to_spherical_coords();
        
        // High emotional → high r
        assert!(coords.r > 0.5);
        
        // Bounds check
        assert!(coords.theta >= 0.0 && coords.theta < 2.0 * std::f64::consts::PI);
        assert!(coords.phi >= 0.0 && coords.phi <= std::f64::consts::PI);
    }
    
    #[tokio::test]
    async fn test_insert_and_query_contextual() {
        let mut telescopedb = TelescopeDB::new(PathBuf::from("/tmp/test_telescope")).unwrap();
        
        let ctx7d = create_test_context_token();
        let id = telescopedb.insert_from_ctx7d(&ctx7d).await.unwrap();
        
        let coords = ctx7d.context_tensor.to_spherical_coords();
        let results = telescopedb.query_contextual(coords, 0.5).await.unwrap();
        
        assert!(!results.is_empty());
        assert_eq!(results[0].id, id);
    }
}
```

### Integration Tests

```rust
// tests/integration/telescope_voxel_sync.rs

#[tokio::test]
async fn test_telescopedb_voxeldb_synchronization() {
    // Setup
    let voxeldb = Arc::new(RwLock::new(VoxelDB::new(...).unwrap()));
    let mut telescopedb = TelescopeDB::new(...).unwrap();
    telescopedb.connect_voxeldb(voxeldb.clone());
    
    // Insert biographical entry
    let ctx7d = test_context_token();
    let telescope_id = telescopedb.insert_from_ctx7d(&ctx7d).await.unwrap();
    
    // Sync to VoxelDB
    telescopedb.sync_to_voxeldb().await.unwrap();
    
    // Verify metadata exists in VoxelDB
    let vdb = voxeldb.read().await;
    // TODO: Verify sync
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_spherical_distance_symmetric(
        r1 in 0.0f64..10.0,
        theta1 in 0.0f64..(2.0 * std::f64::consts::PI),
        phi1 in 0.0f64..std::f64::consts::PI,
        r2 in 0.0f64..10.0,
        theta2 in 0.0f64..(2.0 * std::f64::consts::PI),
        phi2 in 0.0f64..std::f64::consts::PI,
    ) {
        let c1 = SphericalCoords { r: r1, theta: theta1, phi: phi1 };
        let c2 = SphericalCoords { r: r2, theta: theta2, phi: phi2 };
        
        // Distance must be symmetric
        let d1 = c1.distance(&c2);
        let d2 = c2.distance(&c1);
        prop_assert!((d1 - d2).abs() < 1e-10);
    }
}
```

### Performance Benchmarks

```rust
// benches/telescopedb_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_insert(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut telescopedb = setup_telescopedb();
    
    c.bench_function("insert_from_ctx7d", |b| {
        b.to_async(&rt).iter(|| async {
            let ctx7d = generate_test_ctx7d();
            telescopedb.insert_from_ctx7d(black_box(&ctx7d))
                .await
                .unwrap()
        })
    });
}

fn bench_query_contextual(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let telescopedb = setup_telescopedb_with_10k_cores();
    let coords = SphericalCoords { r: 0.5, theta: 1.0, phi: 1.5 };
    
    c.bench_function("query_contextual_k10", |b| {
        b.to_async(&rt).iter(|| async {
            telescopedb.query_contextual(black_box(coords), black_box(0.3))
                .await
                .unwrap()
        })
    });
}

criterion_group!(benches, bench_insert, bench_query_contextual);
criterion_main!(benches);
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/cells/telescopedb/error.rs

#[derive(Debug, thiserror::Error)]
pub enum TelescopeDBError {
    #[error("FBCU Core not found: {0}")]
    CoreNotFound(String),
    
    #[error("Invalid spherical coordinates: r={r}, theta={theta}, phi={phi}")]
    InvalidCoordinates { r: f64, theta: f64, phi: f64 },
    
    #[error("Embedding generation failed: {0}")]
    EmbeddingFailed(String),
    
    #[error("Compression failed: {0}")]
    CompressionError(String),
    
    #[error("Spherical index error: {0}")]
    SphericalIndexError(String),
    
    #[error("HNSW index error: {0}")]
    HNSWError(String),
    
    #[error("VoxelDB not connected")]
    VoxelNotConnected,
    
    #[error("Import failed: {0}")]
    ImportError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Context Token 7D conversion failed: {0}")]
    CTX7DConversionError(String),
}

pub type Result<T> = std::result::Result<T, TelescopeDBError>;
```

### Estrategias de Recuperación

```rust
impl TelescopeDB {
    /// Query con fallback: si contextual falla, intenta semantic
    pub async fn query_with_fallback(
        &self,
        coords: SphericalCoords,
        query: &str,
    ) -> Result<Vec<FBCUCore>> {
        // Intento 1: Query contextual
        match self.query_contextual(coords, 0.5).await {
            Ok(results) if !results.is_empty() => return Ok(results),
            _ => {}
        }
        
        // Intento 2: Query semántico
        match self.query_semantic(query).await {
            Ok(results) if !results.is_empty() => {
                return Ok(results.into_iter().map(|(c, _)| c).collect());
            }
            _ => {}
        }
        
        // Fallback final: retornar cores más recientes
        Ok(self.get_recent_cores(10)?)
    }
    
    /// Reconstruir índices si están corruptos
    pub fn rebuild_indices(&mut self) -> Result<()> {
        tracing::warn!("Rebuilding TelescopeDB indices...");
        
        // 1. Cargar todos los cores del disco
        let cores = self.load_all_cores()?;
        
        // 2. Limpiar índices
        self.spherical_index = SphericalIndex::new()?;
        self.ann_index = HnswIndex::new(1536, 16, 200)?;
        
        // 3. Re-indexar
        for core in cores {
            self.spherical_index.insert(core.coords, core.id.clone())?;
            self.ann_index.add(core.id.clone(), &core.atomic_core.embedding.vec)?;
        }
        
        tracing::info!("Indices rebuilt successfully: {} cores", self.spherical_index.len());
        
        Ok(())
    }
}
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **00_VISION/BITA-2_ACA-7D_SPECIFICATION.md** - Especificación de FBCU Cores y dual-helix
- **00_VISION/DECISIONES_ARQUITECTONICAS.md** - DA-007 (TelescopeDB brecha #1), DA-001 (local-first), DA-011 (NO MongoDB)
- **01_ARQUITECTURA/SISTEMA_DUAL_DATABASES.md** - Arquitectura completa TelescopeDB + VoxelDB
- **02_COMPONENTES/CRITICOS/VOXELDB.md** - Componente complementario (cúbico)
- **02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md** - Fuente de contexto 7D

### Código de Referencia

- **B20250915-data-compressor/** - Quantum compressor (validación de compresión fractal 99.999%)
- **src/sandbox/** - Archivos biográficos existentes para import

### Papers y Referencias Técnicas

- **HNSW Algorithm:** [Efficient and robust approximate nearest neighbor search](https://arxiv.org/abs/1603.09320)
- **Spherical Coordinates:** [Wikipedia - Spherical coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system)
- **Cosine Similarity:** [Wikipedia - Cosine Similarity](https://en.wikipedia.org/wiki/Cosine_similarity)

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Esta Semana)

1. ✅ **Crear estructura de directorios:** `src/cells/telescopedb/`
2. ✅ **Definir structs principales:** `TelescopeDB`, `FBCUCore`, `SphericalCoords`
3. ✅ **Implementar Spherical Index:** Insert + query por coordenadas
4. ✅ **Stub de compresión fractal:** Mock inicial, implementación real con FBCU después
5. ✅ **Tests unitarios básicos:** SphericalCoords, CTX7D conversion

### Mejoras v2.0 (Futuro)

1. **HNSW Index completo:** Implementación nativa o integración con `faiss`
2. **Pixel Storage:** PNG encoding de FBCU Cores para visualización
3. **Versionado de cores:** Git-like branching para editar experiencias
4. **Auto-tuning:** Ajustar parámetros de índice basándose en dataset
5. **Distributed TelescopeDB:** Sharding para >1M cores

---

**Estado:** 📋 Especificación completa - Listo para implementación  
**Complejidad:** 🔴 ALTA - Requiere geometría esférica + compresión fractal + dual-helix sync  
**Prioridad:** 🔴 CRÍTICA - Es brecha #1 según DA-007

---

*Generado: 26 Octubre 2025*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec v1.0*  
*"TelescopeDB: Donde tu historia se vuelve contexto"* 🔭✨

# 📊 TelescopeDB - Base Datos Biográfica

**Brecha:** #1 (CRÍTICA - Prioridad absoluta)  
**Fase:** 1 (Semanas 1-2)  
**Estado:** ❌ NO IMPLEMENTADO  
**Decisión Arquitectónica:** DA-007

---

## 🎯 PROPÓSITO

TelescopeDB es la **base de datos biográfica local** que almacena la historia completa de interacciones, decisiones y contexto del usuario.

**Analogía:** "Telescopio" que observa y registra la trayectoria del usuario a través del tiempo.

---

## 🔷 CARACTERÍSTICAS CLAVE

### ✅ Local-First (DA-001, DA-011)
- ❌ NO usar MongoDB
- ✅ Usar SQLite o JSON local
- ✅ Persistencia en disco local

### ✅ Integración Biográfica
- Alimenta dimensión #6 de Context Token 7D
- Punto de convergencia con HarmonyEngine (si implementado)
- Import desde `src/sandbox/` (DA-014)

### ✅ Schema 7D
```rust
struct BiographicalEntry {
    timestamp: String,           // ISO 8601
    content: String,             // Entrada biográfica
    dimensions: Vec<DimensionValue>,  // 7 dimensiones CTX7D
    metadata: HashMap<String, String>,
    tags: Vec<String>,
}

struct DimensionValue {
    dimension_id: u8,            // 1-7
    value: f64,                  // Valor normalizado 0.0-1.0
    confidence: f64,             // Confianza 0.0-1.0
}
```

---

## 🛠️ OPERACIONES CRUD

### Create
```rust
async fn insert(&self, entry: BiographicalEntry) -> Result<EntryId>
```
- Agrega nueva entrada biográfica
- Calcula valores 7D automáticamente (o acepta manual)
- Retorna ID único

### Read
```rust
async fn get_by_timestamp(&self, timestamp: &str) -> Result<BiographicalEntry>
async fn get_by_id(&self, id: EntryId) -> Result<BiographicalEntry>
async fn query_range(&self, start: &str, end: &str) -> Result<Vec<BiographicalEntry>>
```
- Recupera entradas por timestamp o ID
- Consultas por rango temporal

### Update
```rust
async fn update(&self, id: EntryId, entry: BiographicalEntry) -> Result<()>
```
- Actualiza entrada existente
- Mantiene versionado (opcional)

### Delete
```rust
async fn delete(&self, id: EntryId) -> Result<()>
```
- Elimina entrada (con confirmación)
- Opcional: soft delete (marcar inactivo)

---

## 🔗 INTEGRACIONES

### src/sandbox/ (DA-014)
```rust
// Import desde src/sandbox/ a TelescopeDB
async fn import_from_sandbox(&self, path: &str) -> Result<ImportStats>
```
- Lee archivos biográficos desde `src/sandbox/`
- Parsea y valida formato
- Inserta en TelescopeDB
- Retorna estadísticas de import

### Context Token 7D (Dimensión #6)
```rust
// Alimenta dimensión biográfica de CTX7D
async fn get_biographical_context(&self) -> Result<Vec<BiographicalEntry>>
```
- Recupera contexto biográfico relevante
- Calcula valores 7D actualizados
- Integra con sistema CTX7D

### VoxelDB (Metadatos)
```rust
// Alimenta metadatos a VoxelDB para búsqueda semántica
async fn export_metadata_to_voxeldb(&self) -> Result<()>
```
- Exporta metadatos de entradas a VoxelDB
- Sincroniza embeddings
- Permite búsqueda semántica de biografía

---

## 📋 API ENDPOINTS (Propuestos)

### Alta Prioridad
- `POST /api/telescopedb/create` - Crear entrada biográfica
- `GET /api/telescopedb/read?id={id}` - Leer entrada por ID
- `GET /api/telescopedb/read?timestamp={ts}` - Leer por timestamp
- `PUT /api/telescopedb/update` - Actualizar entrada
- `DELETE /api/telescopedb/delete?id={id}` - Eliminar entrada
- `GET /api/telescopedb/query?start={ts1}&end={ts2}` - Consulta rango

### Media Prioridad
- `POST /api/telescopedb/import` - Importar desde src/sandbox/
- `GET /api/telescopedb/export` - Exportar a formato externo
- `GET /api/telescopedb/stats` - Estadísticas base datos

---

## 🧪 VALIDACIÓN Y TESTING

### Script de Validación
```bash
# examples/test_telescopedb.rs
cargo run --example test_telescopedb
```

**Tests a Implementar:**
1. **CRUD básico:** Create, Read, Update, Delete
2. **Rendimiento:** ≥1000 operaciones/s sin degradación
3. **Integridad:** Validar schema 7D
4. **Integración:** Import desde src/sandbox/
5. **Concurrencia:** Operaciones simultáneas sin corrupción

### Asserts Explícitos
```rust
// Test Create + Read
let entry = BiographicalEntry { /* ... */ };
let id = db.insert(entry.clone()).await.expect("Insert failed");
let retrieved = db.get_by_id(id).await.expect("Get failed");
assert_eq!(retrieved.content, entry.content);
assert_eq!(retrieved.dimensions.len(), 7);
println!("✅ CRUD test passed");

// Test Rendimiento
let start = Instant::now();
for _ in 0..1000 {
    db.insert(generate_entry()).await.expect("Insert failed");
}
let duration = start.elapsed();
assert!(duration.as_secs() < 1, "Rendimiento < 1000 ops/s");
println!("✅ Rendimiento validado: {:?}", duration);
```

---

## 📊 MÉTRICAS DE ÉXITO

| Métrica | Objetivo | Validación |
|---------|----------|------------|
| **Rendimiento** | ≥1000 ops/s | Benchmark insert |
| **Schema 7D** | 7 dimensiones | Assert len == 7 |
| **Integridad** | 0 corrupciones | Tests concurrencia |
| **Import** | ≥95% éxito | Import desde sandbox |
| **Latencia p95** | <10ms | Benchmark read |

---

## 🚀 PLAN DE IMPLEMENTACIÓN

### Semana 1: Fundamentos
- [ ] 1.1 - Diseñar schema biográfico completo
- [ ] 1.2 - Implementar `src/cells/telescopedb.rs` (estructura base)
- [ ] 1.3 - CRUD operations (Create, Read)

### Semana 2: Completar y Validar
- [ ] 1.4 - CRUD operations (Update, Delete)
- [ ] 1.5 - Integración `src/sandbox/` import
- [ ] 1.6 - Script `examples/test_telescopedb.rs`
- [ ] 1.7 - Validar rendimiento (≥1000 ops/s)
- [ ] 1.8 - Documentar API en `06_DOCUMENTACION/API_ENDPOINTS.md`

---

## ⚠️ DECISIONES ARQUITECTÓNICAS RELEVANTES

- **DA-001:** Local-First Architecture (SQLite/JSON, NO MongoDB)
- **DA-007:** TelescopeDB es Brecha Crítica #1 (prioridad absoluta)
- **DA-011:** NO MongoDB en v1.0
- **DA-014:** `src/sandbox/` integra con TelescopeDB

---

## 🔗 DEPENDENCIAS

### Dependencias Previas
- ✅ Ninguna (es fundacional)

### Dependencias Posteriores
- VoxelDB depende de TelescopeDB (metadatos)
- SENSORY ENGINE depende de TelescopeDB (almacenamiento)
- Expertise Gen depende de TelescopeDB (biografía)
- LIP depende de TelescopeDB (persistencia)

---

## 📚 REFERENCIAS

- **Brecha #1:** `FUSION_BAYESIANA/02_GAP_ANALYSIS.md`
- **Roadmap Fase 1:** `FUSION_BAYESIANA/07_PLAN_IMPLEMENTACION.md`
- **Dimensión Biográfica CTX7D:** `FUSION_BAYESIANA/06_HARMONY_CTX7D.md`
- **SANDBOX Integration:** `FUSION_BAYESIANA/04_SANDBOX_INTEGRATION.md`

---

**Estado:** 🔴 CRÍTICO - Iniciar implementación INMEDIATAMENTE  
**Próxima acción:** 1.1 - Diseñar schema biográfico completo

---

*Generado por Sistema Bitácora v1.0 - Fusion Bayesiana Methodology*  
*Última actualización: 2025-01-25*
