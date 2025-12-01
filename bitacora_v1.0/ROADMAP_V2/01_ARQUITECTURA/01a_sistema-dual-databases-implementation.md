# 🛠️ SISTEMA DUAL DE BASES DE DATOS - IMPLEMENTACIÓN

**Ubicación:** `ROADMAP_V2/01_ARQUITECTURA/01a_sistema-dual-databases-implementation.md`  
**Versión:** 1.0 - IMPLEMENTACIÓN  
**Tipo:** IMPL (Código + Algoritmos + Detalles Técnicos)  
**Prerequisito:** Leer `01_sistema-dual-databases.md` primero (concepto)

---

## 📋 CÓMO LEER ESTE DOCUMENTO

**Este es el documento técnico.**

Si necesitas:
- ✅ Entender conceptos → lee `01_sistema-dual-databases.md` (SPEC)
- ✅ Código Rust → estás aquí (IMPL)
- ✅ Performance targets → estás aquí
- ✅ Milestones de implementación → estás aquí

---

## 🔭 TelescopeDB: IMPLEMENTACIÓN

### Estructura de Datos

```rust
// src/telescope_db/core.rs
pub struct TelescopeDB {
    storage_path: PathBuf,                    // .bitacora/telescope/
    cores: HashMap<String, FBCUCore>,         // id -> core (cache en memoria)
    spherical_index: SphericalIndex,          // Índice (r,θ,φ)
    pixel_storage: Option<PixelEncoder>,      // PNG encoding opcional
    compressor: FractalCompressor,            // Compresión 99.999%
}

pub struct FBCUCore {
    pub id: String,                           // SHA-256 content-addressable
    pub bita_header: BitaHeader,
    pub atomic_core: AtomicCore,              // Embeddings + anchors
    pub relational_triples: Vec<Triple>,      // Relaciones semánticas
    pub context_tensor: ContextTensor7D,      // 7 dimensiones completas
    pub provenance: Provenance,
    pub related_templates: Vec<String>,       // IDs de templates en VoxelDB
}

pub struct SphericalCoords {
    pub r: f64,         // [0, ∞) - Intensidad
    pub theta: f64,     // [0, 2π) - Categoría
    pub phi: f64,       // [0, π] - Valencia
}
```

### Operación 1: Insert (desde Context Token 7D)

```rust
impl TelescopeDB {
    pub async fn insert_from_ctx7d(&mut self, token: &ContextToken7D) -> Result<String> {
        // 1. Convertir CTX7D a FBCU Core
        let core = self.ctx7d_to_fbcu_core(token)?;
        
        // 2. Calcular coordenadas esféricas
        let coords = self.calculate_spherical_coords(&core.context_tensor);
        
        // 3. Comprimir con algoritmo fractal (código quantum reutilizado)
        let compressed = self.compressor.compress(&core)?;
        
        // 4. Calcular ID content-addressable
        let id = self.compute_content_id(&compressed)?;
        
        // 5. Guardar en disco
        self.save_fbcu_core(&id, &compressed)?;
        
        // 6. Indexar en geometría esférica
        self.spherical_index.insert(coords, id.clone())?;
        
        // 7. Opcional: Guardar píxel en imagen PNG
        if let Some(ref mut pixel_encoder) = self.pixel_storage {
            pixel_encoder.encode_and_save(&core, coords)?;
        }
        
        Ok(id)
    }
    
    fn calculate_spherical_coords(&self, ctx: &ContextTensor7D) -> SphericalCoords {
        SphericalCoords {
            // Intensidad = función de dimensión emocional + intencional
            r: self.calculate_intensity(ctx),
            
            // Categoría = función de dimensión semántica + contextual
            theta: self.calculate_category(ctx),
            
            // Valencia = función de dimensión emocional (positiva/negativa)
            phi: self.calculate_valence(ctx),
        }
    }
}
```

**Performance Targets:**
- Insert: <100ms (incluye compresión fractal)
- Compression: >99.99% efficiency
- Storage: ~2 KB por FBCU Core comprimido

### Operación 2: Query Contextual

```rust
impl TelescopeDB {
    pub async fn query_contextual(&self, input: &str) -> Result<Vec<FBCUCore>> {
        // 1. Analizar input para obtener coordenadas aproximadas
        let query_coords = self.analyze_query_intent(input)?;
        
        // 2. Buscar en índice esférico (vecindad)
        let sphere_region = SphericalRegion {
            center: query_coords,
            radius: self.calculate_search_radius(input),
        };
        
        let candidate_ids = self.spherical_index.query_region(&sphere_region)?;
        
        // 3. Cargar cores y rankear por relevancia
        let mut results = Vec::new();
        for id in candidate_ids {
            let core = self.load_fbcu_core(&id)?;
            let relevance = self.calculate_relevance(&core, input)?;
            results.push((core, relevance));
        }
        
        // 4. Ordenar por relevancia
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(results.into_iter().map(|(core, _)| core).take(10).collect())
    }
}
```

**Performance Targets:**
- Query: <50ms para top-10 resultados
- Precision@10: >85%
- Recall@10: >75%

### Operación 3: Query Similar (Embeddings)

```rust
impl TelescopeDB {
    pub async fn query_similar(&self, embedding: &[f32]) -> Result<Vec<(FBCUCore, f64)>> {
        // 1. Búsqueda aproximada de vecinos (ANN - Approximate Nearest Neighbors)
        let candidate_ids = self.ann_index.search(embedding, k=100)?;
        
        // 2. Calcular similaridad exacta (cosine similarity)
        let mut results = Vec::new();
        for id in candidate_ids {
            let core = self.load_fbcu_core(&id)?;
            let core_embedding = &core.atomic_core.embedding.vec;
            let similarity = cosine_similarity(embedding, core_embedding);
            
            if similarity > 0.7 {  // Threshold
                results.push((core, similarity));
            }
        }
        
        // 3. Ordenar por similaridad
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(results.into_iter().take(10).collect())
    }
}
```

**Performance Targets:**
- Query: <50ms para top-10
- ANN accuracy: >90% (vs exact search)

---

## 🧊 VoxelDB: IMPLEMENTACIÓN

### Estructura de Datos

```rust
// src/voxel_db/core.rs
pub struct VoxelDB {
    storage_path: PathBuf,                    // .bitacora/voxel/
    voxels: HashMap<VoxelCoord, Voxel>,       // (x,y,z) -> voxel
    octree_index: OctreeIndex,                // Índice jerárquico
    wavelet_transform: WaveletTransform,      // Multi-resolución
}

pub struct Voxel {
    pub coord: VoxelCoord,                    // Posición (x, y, z)
    pub templates: Vec<TemplateRef>,          // Referencias a MTT-DSL
    pub density: f64,                         // Concentración de templates
    pub neighbors: [Option<VoxelId>; 26],     // 6 caras + 12 aristas + 8 esquinas
}

pub struct VoxelCoord {
    pub x: u32,         // [0, 1024) - Complejidad
    pub y: u32,         // [0, 1024) - Impacto
    pub z: u32,         // [0, 256) - Urgencia (layers)
}

pub struct TemplateRef {
    pub id: String,              // Template ID
    pub name: String,            // ej: "debugging_deep_dive"
    pub category: TemplateCategory,
    pub score: f64,              // Relevancia en este voxel
}
```

### Operación 1: Insert Template

```rust
impl VoxelDB {
    pub async fn insert_template(&mut self, template: &MTTDSLTemplate) -> Result<()> {
        // 1. Calcular coordenadas cúbicas desde metadata del template
        let coords = self.calculate_cubic_coords(template)?;
        
        // 2. Obtener o crear voxel
        let voxel = self.voxels.entry(coords)
            .or_insert_with(|| Voxel::new(coords));
        
        // 3. Agregar template al voxel
        voxel.templates.push(TemplateRef {
            id: template.id.clone(),
            name: template.name.clone(),
            category: template.category.clone(),
            score: template.relevance_score,
        });
        
        // 4. Actualizar densidad
        voxel.density = voxel.templates.len() as f64 / MAX_TEMPLATES_PER_VOXEL as f64;
        
        // 5. Actualizar octree index
        self.octree_index.insert(coords, voxel.clone())?;
        
        // 6. Conectar vecinos (26 direcciones)
        self.update_neighbors(coords)?;
        
        Ok(())
    }
    
    fn calculate_cubic_coords(&self, template: &MTTDSLTemplate) -> Result<VoxelCoord> {
        Ok(VoxelCoord {
            // Complejidad basada en número de parámetros
            x: (template.complexity * 1024.0) as u32,
            
            // Impacto basado en metadata emocional
            y: (template.emotional_impact * 1024.0) as u32,
            
            // Urgencia basada en prioridad
            z: (template.urgency * 256.0) as u32,
        })
    }
}
```

**Performance Targets:**
- Insert: <10ms
- Neighborhood update: <5ms

### Operación 2: Query Spatial (Context-Aware)

```rust
impl VoxelDB {
    pub async fn find_templates_spatial(&self, ctx7d: &ContextToken7D) -> Result<Vec<MTTDSLTemplate>> {
        // 1. Convertir CTX7D a coordenadas de búsqueda
        let search_coords = self.ctx7d_to_cubic_coords(ctx7d)?;
        
        // 2. Definir región de búsqueda (cubo 3D)
        let search_region = CubicRegion {
            center: search_coords,
            half_extents: self.calculate_search_radius(ctx7d),
        };
        
        // 3. Query octree (logarítmico)
        let candidate_voxels = self.octree_index.query_region(&search_region)?;
        
        // 4. Recolectar templates de voxels
        let mut templates = Vec::new();
        for voxel in candidate_voxels {
            for template_ref in &voxel.templates {
                let template = self.load_template(&template_ref.id)?;
                let relevance = self.calculate_template_relevance(&template, ctx7d)?;
                templates.push((template, relevance));
            }
        }
        
        // 5. Rankear por relevancia
        templates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(templates.into_iter().map(|(t, _)| t).take(5).collect())
    }
}
```

**Performance Targets:**
- Spatial query: <30ms
- Octree depth: ≤8 niveles
- Templates returned: top-5

### Operación 3: Navigate Neighborhood

```rust
impl VoxelDB {
    pub async fn navigate_neighborhood(&self, voxel_id: &VoxelId, direction: Direction3D) -> Result<Option<Voxel>> {
        let current_voxel = self.load_voxel(voxel_id)?;
        
        // Direcciones 3D (26 posibles):
        // 6 caras: ±X, ±Y, ±Z
        // 12 aristas: combinaciones de 2 ejes
        // 8 esquinas: combinaciones de 3 ejes
        
        let neighbor_index = direction.to_index(); // [0..26)
        
        if let Some(ref neighbor_id) = current_voxel.neighbors[neighbor_index] {
            let neighbor = self.load_voxel(neighbor_id)?;
            Ok(Some(neighbor))
        } else {
            Ok(None)
        }
    }
}
```

**Performance Targets:**
- Navigation: <5ms per hop
- Neighbor lookup: O(1) con array indexing

---

## 🔄 SINCRONIZACIÓN DUAL-HELIX

```rust
impl BitacoraDualDB {
    pub async fn synchronize_after_insert(
        &mut self,
        core_id: &str,
        template_ids: &[String]
    ) -> Result<()> {
        // 1. Actualizar FBCU Core con templates relacionados
        let mut core = self.telescope_db.load_fbcu_core(core_id)?;
        core.related_templates.extend_from_slice(template_ids);
        self.telescope_db.update_core(&core)?;
        
        // 2. Actualizar templates con ejemplo biográfico
        for template_id in template_ids {
            let mut template = self.voxel_db.load_template(template_id)?;
            template.biographical_examples.push(core_id.to_string());
            self.voxel_db.update_template(&template)?;
        }
        
        Ok(())
    }
}
```

---

## 🎯 IMPLEMENTACIÓN: ORDEN RECOMENDADO

### Fase 1: TelescopeDB Básico (Semanas 1-3)

```rust
// Milestone 1.1: FBCU Core structures
✅ src/telescope_db/core.rs
✅ src/telescope_db/fbcu_core.rs
✅ src/telescope_db/bita_header.rs

// Milestone 1.2: Compression (reutilizar quantum)
✅ src/telescope_db/compression/fractal.rs
✅ src/telescope_db/compression/cbor.rs

// Milestone 1.3: Storage
✅ src/telescope_db/storage.rs
✅ src/telescope_db/manifest.rs

// Milestone 1.4: Spherical index
✅ src/telescope_db/spherical_index.rs
✅ src/telescope_db/query_engine.rs

// Tests:
cargo test --package telescope_db
```

### Fase 2: VoxelDB Básico (Semanas 4-5)

```rust
// Milestone 2.1: Voxel structures
✅ src/voxel_db/core.rs
✅ src/voxel_db/voxel.rs
✅ src/voxel_db/template_ref.rs

// Milestone 2.2: Octree index
✅ src/voxel_db/octree_index.rs
✅ src/voxel_db/wavelet_index.rs

// Milestone 2.3: Template storage
✅ src/voxel_db/template_storage.rs
✅ src/voxel_db/neighborhood.rs

// Milestone 2.4: Spatial queries
✅ src/voxel_db/query_engine.rs

// Tests:
cargo test --package voxel_db
```

### Fase 3: Integración Dual (Semana 6)

```rust
// Milestone 3.1: Dual DB wrapper
✅ src/dual_db/mod.rs
✅ src/dual_db/synchronization.rs

// Milestone 3.2: Cross-references
✅ src/dual_db/cross_reference.rs

// Milestone 3.3: Unified query interface
✅ src/dual_db/unified_query.rs

// Tests:
cargo test --package dual_db --test integration_tests
```

---

## 📚 CÓDIGO DE REFERENCIA

Reutilizar desde B20250915-data-compressor:

```rust
// 1. FractalCompressor
B20250915-data-compressor/src/quantum_compression/fractal_engine.rs
→ src/telescope_db/compression/fractal.rs

// 2. WaveletTransform
B20250915-data-compressor/src/quantum_compression/wavelet_transform.rs
→ src/voxel_db/wavelet_index.rs

// 3. QuantumVisualCompressor (opcional)
B20250915-data-compressor/src/quantum_compression/visual_compressor.rs
→ src/telescope_db/pixel_encoder.rs
```

---

## ✅ CRITERIOS DE ÉXITO

### TelescopeDB:
- [ ] Insert FBCU Core: <100ms
- [ ] Compression ratio: >99.99%
- [ ] Query contextual: <50ms (top-10)
- [ ] Query similar: <50ms (top-10)
- [ ] Storage: ~2 KB/core comprimido
- [ ] Content-addressable IDs: 100% deterministas
- [ ] Tests: >80% coverage

### VoxelDB:
- [ ] Insert template: <10ms
- [ ] Spatial query: <30ms
- [ ] Neighborhood navigation: <5ms per hop
- [ ] Octree depth: ≤8 niveles
- [ ] Storage: ~1 KB/voxel
- [ ] 26-directional navigation funcional
- [ ] Tests: >80% coverage

### Integración:
- [ ] Sincronización dual: <20ms
- [ ] Cross-references: 100% consistentes
- [ ] Unified query: <100ms end-to-end
- [ ] Integration tests: >70% coverage

---

*Implementación técnica del Sistema Dual de Bitácora*  
*"Código Rust, performance métricas, milestones claros"*
