```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/06_voxeldb.md
Versión: 1.5
Fecha Creación: 2025-11-27
Última Actualización: 2025-11-27
Autor: Sistema Bitácora - Arquitectura QPX v1.5 (documento reescrito desde cero)
Propósito: VoxelDB como organismo semántico 100% pixel-native con QPX + HNSW + Dual-Helix
Estado: 📋 ESPECIFICACIÓN v1.5 - Pixel-Native Revolution
Relacionado Con: 14_qpx-quantumdao-revolucion.md, 05_telescopedb.md, 01_sistema-dual-databases.md
Implementa: DA-008 (VoxelDB brecha crítica #2), DA-001 (Local-first), DA-011 (NO MongoDB)
Backup: 06_voxeldb.md.backup_v1.0 (arquitectura anterior para referencia)
# === FIN DATOS DE AUDITORÍA ===
```

# 🧊 VOXELDB v1.5 - Organismo Semántico Pixel-Native

> **"No es un vector database. Es un espacio semántico viviente codificado en píxeles."**

---

## 📚 TABLA DE CONTENIDOS

0. [Aclaración Arquitectónica: QPX vs VoxelDB](#aclaración-arquitectónica-qpx-vs-voxeldb)
1. [Propósito](#propósito)
2. [Arquitectura QPX para Embeddings](#arquitectura-qpx-para-embeddings)
3. [HNSW Pixel-Native](#hnsw-pixel-native)
4. [Dual-Helix Sync con TelescopeDB](#dual-helix-sync-con-telescopedb)
5. [Spherical Encoding](#spherical-encoding)
6. [Storage Format: QPX Variable-Length](#storage-format-qpx-variable-length)
7. [API Principal](#api-principal)
8. [Casos de Uso](#casos-de-uso)
9. [Performance Targets](#performance-targets)

---

## 🔍 ACLARACIÓN ARQUITECTÓNICA: QPX vs VoxelDB

**CRÍTICO - Entender la diferencia:**

```
┌─────────────────────────────────────────────────────────────┐
│ QPX (Quantum Pixel eXchange)                                │
│ ↓                                                           │
│ Formato de encoding variable-length                        │
│ (como CBOR, Protobuf, MessagePack)                         │
│                                                             │
│ Responsabilidad:                                            │
│ - Encode/decode embeddings a bytes                         │
│ - Variable-length (1 byte → 4KB para embedding 1536D)     │
│ - Spherical encoding para vectores                         │
│ - NO almacena, NO indexa, NO hace búsquedas                │
└─────────────────────────────────────────────────────────────┘
                        ↓ usa QPX como formato
┌─────────────────────────────────────────────────────────────┐
│ VoxelDB                                                     │
│ ↓                                                           │
│ Base de datos semántica (Segunda DB que usa QPX)          │
│                                                             │
│ Responsabilidad:                                            │
│ - Almacenar Voxels (embeddings) en disco (.qpx files)     │
│ - Indexar con HNSW para búsqueda de vecindad              │
│ - Query por similitud vectorial (cosine, L2)              │
│ - Spherical coordinates para contextualización            │
│ - Dual-helix sync con TelescopeDB                          │
│ - Alpha channel tracking 200-255                           │
└─────────────────────────────────────────────────────────────┘
```

**Analogía clara:**

```rust
// ❌ INCORRECTO: "QPX es una base de datos de vectores"
let db = QPX::new_vector_db();  // NO existe

// ✅ CORRECTO: "VoxelDB usa formato QPX para embeddings"
let voxel_db = VoxelDB::new("./data")?;  // DB que usa QPX

// QPX es solo el encoder/decoder para embeddings
let embedding = vec![0.1, 0.2, ..., 0.9];  // 1536D vector
let qpx_bytes = QPXEncoder::encode_embedding(&embedding)?;  // QPX = formato
voxel_db.insert_raw(qpx_bytes).await?;  // VoxelDB = storage + HNSW
```

**En resumen:**
- **QPX** = Protocolo de encoding (aquí usado para embeddings)
- **VoxelDB** = Base de datos semántica con HNSW
- **TelescopeDB** = Base de datos biográfica con coordenadas esféricas
- Ambas DBs usan QPX, pero para propósitos diferentes

---

## 🎯 PROPÓSITO

### ¿Qué es VoxelDB v1.5?

**VoxelDB** es el **almacén de memoria semántica** de Bitácora. Es la segunda base de datos del sistema **dual-helix**, complementaria a TelescopeDB:

1. **Almacena embeddings como Voxels** (QPX format)
2. **Indexa con HNSW** para búsqueda de vecindad rápida
3. **Sincroniza con TelescopeDB** (dual-helix: biografía + semántica)
4. **Spherical encoding** para contextualización (intensidad + tema + emoción)
5. **Alpha channel 200-255** para rastrear origen de embeddings
6. **100% pixel-native** (NO ChromaDB, NO Pinecone, NO Qdrant)

### ¿Qué Problema Resuelve?

**Problema clásico:**
```
Usuario: "Busca conversaciones similares sobre concurrency en Rust"

Vector DB tradicional:
❌ Búsqueda puramente vectorial (sin contexto biográfico)
❌ No sabe cuándo sucedieron las conversaciones
❌ No puede filtrar por origen (WhatsApp vs native)
❌ No integra con memoria episódica
```

**Solución VoxelDB v1.5:**
```rust
// 1. Query con embedding + contexto esférico + alpha filter
let query_embedding = embed("concurrency en Rust");

let query = VoxelQuery {
    embedding: query_embedding,
    k: 10,  // Top-10 vecinos
    spherical_context: Some(SphericalCoords {
        r: 0.8,      // Alta intensidad
        theta: 0.3,  // Técnico
        phi: 1.2,    // Frustración (común en debugging)
    }),
    alpha_filter: Some(255),  // Solo native (no imports)
    time_range: Some(30.days()),
};

let voxels = voxel_db.query(&query).await?;
// → Top-10 embeddings similares

// 2. Dual-helix: Recuperar cores biográficos asociados
for voxel in voxels {
    let core = telescope_db.get(&voxel.core_id).await?;
    println!("📅 {}: {}", core.timestamp, voxel.text_preview);
}

// Resultado:
// ✅ Embeddings similares encontrados (<50ms)
// ✅ Contexto biográfico completo (cuándo, dónde, cómo te sentías)
// ✅ Filtrado por origen (solo conversaciones nativas)
// ✅ 100% local, 100% pixel-native
```

### Los 5 Imposibles que VoxelDB v1.5 Logra

```rust
/// IMPOSIBLE #1: 100% Pixel-Native Vector Storage (NO ChromaDB)
pub struct VoxelDB {
    voxels: HashMap<VoxelId, QPXEncoded>,  // Todo es QPX
    hnsw_index: HNSWIndex,                 // HNSW sobre QPX
}

/// IMPOSIBLE #2: Dual-Helix Sync (BiografíaⓈ Semántica)
pub struct DualHelixSync {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    sync_interval: Duration,
}

/// IMPOSIBLE #3: Spherical Context para Embeddings
pub struct Voxel {
    pub embedding: Vec<f32>,     // 1536D vector
    pub coords: SphericalCoords, // r, θ, φ para contexto
    pub core_id: CoreId,         // Link a TelescopeDB
    pub alpha: u8,               // 255=Native, 210=WhatsApp, etc
}

/// IMPOSIBLE #4: HNSW con QPX Variable-Length
pub struct HNSWIndex {
    layers: Vec<Layer>,
    entry_point: VoxelId,
    qpx_encoded: bool,  // Nodos encoded en QPX
}

/// IMPOSIBLE #5: Query con Filtros Biográficos
pub struct VoxelQuery {
    pub embedding: Vec<f32>,
    pub k: usize,
    pub spherical_context: Option<SphericalCoords>,
    pub alpha_filter: Option<u8>,
    pub time_range: Option<Duration>,
    pub core_filter: Option<Vec<CoreId>>,  // Filtrar por cores específicos
}
```

---

## 🏗️ ARQUITECTURA QPX PARA EMBEDDINGS

### Estructura de un Voxel

```rust
/// Un Voxel es la unidad fundamental de memoria semántica en VoxelDB
pub struct Voxel {
    /// Identificador único (SHA-256 del embedding)
    pub id: VoxelId,
    
    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,
    
    /// Embedding (típicamente 1536D para text-embedding-3-small)
    pub embedding: Vec<f32>,
    
    /// Coordenadas esféricas para contexto
    pub coords: SphericalCoords {
        r: f64,      // Intensidad (0.0-1.0)
        theta: f64,  // Categoría temática (0-2π)
        phi: f64,    // Valencia emocional (0-π)
    },
    
    /// Link a QuantumCore en TelescopeDB
    pub core_id: CoreId,
    
    /// Alpha channel para trazabilidad
    pub alpha: u8,  // 255=Native, 210=WhatsApp, 200=MySQL, etc
    
    /// Preview del texto original (primeros 200 chars)
    pub text_preview: String,
    
    /// Metadata QPX
    pub qpx_meta: VoxelMetadata {
        compressed_size: usize,
        original_size: usize,
        encoding_mode: EncodingMode,  // Compact | Full
    },
}

/// Metadata de encoding QPX
pub struct VoxelMetadata {
    pub compressed_size: usize,
    pub original_size: usize,
    pub encoding_mode: EncodingMode,
}

pub enum EncodingMode {
    Compact,  // Para embeddings pequeños (<256D)
    Full,     // Para embeddings grandes (≥256D)
}
```

### Storage en Disco (100% Pixel-Native)

```rust
/// VoxelDB NO usa ChromaDB/Pinecone. Todo es QPX en archivos binarios.
pub struct VoxelDBStorage {
    /// data/voxel/embeddings/[year]/[month]/[voxel_id].qpx
    voxels_dir: PathBuf,
    
    /// data/voxel/index/hnsw.idx (HNSW graph serializado)
    hnsw_index: HNSWIndexFile,
    
    /// data/voxel/index/spherical.idx (coordenadas esféricas)
    spherical_index: SphericIndexFile,
    
    /// data/voxel/sync/dual_helix.log (sync con TelescopeDB)
    dual_helix_log: DualHelixLog,
}

impl VoxelDB {
    /// Guardar Voxel en disco
    pub async fn insert(&mut self, voxel: Voxel) -> Result<VoxelId> {
        // 1. Encode embedding a QPX
        let qpx_bytes = self.encode_embedding_qpx(&voxel)?;
        
        // 2. Calcular path: data/voxel/embeddings/2025/11/[voxel_id].qpx
        let path = self.compute_storage_path(&voxel);
        
        // 3. Escribir archivo
        fs::write(&path, qpx_bytes).await?;
        
        // 4. Actualizar HNSW index
        self.hnsw_index.insert(voxel.id, &voxel.embedding)?;
        
        // 5. Actualizar índice esférico
        self.spherical_index.insert(voxel.id, voxel.coords)?;
        
        // 6. Dual-helix: Link con TelescopeDB
        self.dual_helix_sync.link_voxel_to_core(voxel.id, voxel.core_id).await?;
        
        Ok(voxel.id)
    }
    
    /// Recuperar Voxel desde disco
    pub async fn get(&self, id: &VoxelId) -> Result<Voxel> {
        // 1. Buscar en índice esférico
        let path = self.spherical_index.get_path(id)?;
        
        // 2. Leer archivo QPX
        let qpx_bytes = fs::read(&path).await?;
        
        // 3. Decode QPX → Voxel
        let voxel = self.decode_embedding_qpx(&qpx_bytes)?;
        
        // 4. Validar integridad
        if voxel.id != *id {
            return Err(VoxelError::CorruptedData);
        }
        
        Ok(voxel)
    }
}
```

---

## 🔗 HNSW PIXEL-NATIVE

### HNSW sobre QPX

```rust
/// HNSW (Hierarchical Navigable Small World) implementado sobre QPX
pub struct HNSWIndex {
    /// Layers del grafo (layer 0 = todos los nodos, layer N = entry points)
    pub layers: Vec<Layer>,
    
    /// Entry point del grafo (nodo de inicio para búsquedas)
    pub entry_point: VoxelId,
    
    /// M = número de conexiones por nodo
    pub m: usize,
    
    /// ef_construction = tamaño de lista dinámica durante construcción
    pub ef_construction: usize,
    
    /// Todos los nodos están encoded en QPX
    pub qpx_encoded: bool,
}

/// Layer del grafo HNSW
pub struct Layer {
    pub level: usize,
    pub nodes: HashMap<VoxelId, HNSWNode>,
}

/// Nodo HNSW
pub struct HNSWNode {
    pub voxel_id: VoxelId,
    pub neighbors: Vec<VoxelId>,  // Conexiones en este layer
    pub qpx_path: PathBuf,         // Path al archivo .qpx
}

impl HNSWIndex {
    /// Crear nuevo HNSW index
    pub fn new(m: usize, ef_construction: usize) -> Self {
        HNSWIndex {
            layers: vec![Layer { level: 0, nodes: HashMap::new() }],
            entry_point: VoxelId::default(),
            m,
            ef_construction,
            qpx_encoded: true,
        }
    }
    
    /// Insertar nuevo voxel en HNSW
    pub fn insert(&mut self, voxel_id: VoxelId, embedding: &[f32]) -> Result<()> {
        // 1. Calcular layer aleatorio (exponencial decay)
        let target_layer = self.random_layer();
        
        // 2. Encontrar vecinos más cercanos en cada layer
        let mut entry = self.entry_point;
        
        for layer in (target_layer + 1..self.layers.len()).rev() {
            entry = self.search_layer(entry, embedding, 1, layer)?[0];
        }
        
        // 3. Insertar en layers [0..target_layer]
        for layer in 0..=target_layer {
            let candidates = self.search_layer(entry, embedding, self.ef_construction, layer)?;
            
            let neighbors = self.select_neighbors(candidates, self.m);
            
            self.add_node(voxel_id, neighbors, layer)?;
            
            // Actualizar entry point si estamos en top layer
            if layer == self.layers.len() - 1 {
                self.entry_point = voxel_id;
            }
        }
        
        Ok(())
    }
    
    /// Búsqueda KNN (K-Nearest Neighbors)
    pub async fn search(
        &self,
        query_embedding: &[f32],
        k: usize,
        ef: usize,
    ) -> Result<Vec<VoxelId>> {
        // 1. Empezar desde entry point en top layer
        let mut entry = self.entry_point;
        
        // 2. Descender por layers hasta layer 0
        for layer in (1..self.layers.len()).rev() {
            entry = self.search_layer(entry, query_embedding, 1, layer)?[0];
        }
        
        // 3. Búsqueda exhaustiva en layer 0
        let candidates = self.search_layer(entry, query_embedding, ef, 0)?;
        
        // 4. Seleccionar top-k
        let top_k: Vec<_> = candidates.into_iter().take(k).collect();
        
        Ok(top_k)
    }
    
    /// Búsqueda en un layer específico
    fn search_layer(
        &self,
        entry: VoxelId,
        query_embedding: &[f32],
        ef: usize,
        layer: usize,
    ) -> Result<Vec<VoxelId>> {
        let mut visited = HashSet::new();
        let mut candidates = BinaryHeap::new();  // Max-heap (mayor distancia primero)
        let mut results = BinaryHeap::new();      // Min-heap (menor distancia primero)
        
        // Leer embedding del entry point (desde QPX)
        let entry_embedding = self.load_embedding(&entry).await?;
        let entry_dist = cosine_distance(query_embedding, &entry_embedding);
        
        candidates.push(Reverse((OrderedFloat(entry_dist), entry)));
        results.push((OrderedFloat(entry_dist), entry));
        visited.insert(entry);
        
        while let Some(Reverse((current_dist, current_id))) = candidates.pop() {
            // Si current_dist > worst en results, terminamos
            if let Some((worst_dist, _)) = results.peek() {
                if current_dist > *worst_dist && results.len() >= ef {
                    break;
                }
            }
            
            // Explorar vecinos
            let node = &self.layers[layer].nodes[&current_id];
            for neighbor_id in &node.neighbors {
                if visited.contains(neighbor_id) {
                    continue;
                }
                
                visited.insert(*neighbor_id);
                
                // Leer embedding del vecino (desde QPX)
                let neighbor_embedding = self.load_embedding(neighbor_id).await?;
                let neighbor_dist = cosine_distance(query_embedding, &neighbor_embedding);
                
                candidates.push(Reverse((OrderedFloat(neighbor_dist), *neighbor_id)));
                results.push((OrderedFloat(neighbor_dist), *neighbor_id));
                
                // Mantener solo top-ef
                if results.len() > ef {
                    results.pop();
                }
            }
        }
        
        Ok(results.into_sorted_vec().into_iter().map(|(_, id)| id).collect())
    }
    
    /// Cargar embedding desde disco (QPX)
    async fn load_embedding(&self, voxel_id: &VoxelId) -> Result<Vec<f32>> {
        let node = self.find_node(voxel_id)?;
        let qpx_bytes = fs::read(&node.qpx_path).await?;
        let voxel = QPXDecoder::decode_voxel(&qpx_bytes)?;
        Ok(voxel.embedding)
    }
}
```

---

## 🌀 DUAL-HELIX SYNC CON TELESCOPEDB

### Sincronización Biográfica ⓈⓈ Semántica

```rust
/// DualHelixSync = Motor de sincronización entre TelescopeDB y VoxelDB
pub struct DualHelixSync {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    embedding_service: Arc<EmbeddingService>,
    sync_interval: Duration,
}

impl DualHelixSync {
    /// Sync continuo (background task)
    pub async fn run_continuous_sync(&self) -> ! {
        loop {
            self.sync_cycle().await;
            tokio::time::sleep(self.sync_interval).await;
        }
    }
    
    /// Ciclo de sincronización
    async fn sync_cycle(&self) {
        // 1. TelescopeDB → VoxelDB: Nuevos cores sin embedding
        self.sync_cores_to_voxels().await;
        
        // 2. VoxelDB → TelescopeDB: Voxels sin core asociado
        self.sync_voxels_to_cores().await;
        
        // 3. Update coordenadas esféricas (ambos lados)
        self.sync_spherical_coords().await;
    }
    
    /// 1. Sincronizar cores nuevos → crear voxels
    async fn sync_cores_to_voxels(&self) {
        // Buscar cores sin voxel asociado
        let unsynced_cores = self.telescope_db
            .query_without_voxel()
            .await
            .unwrap();
        
        for core in unsynced_cores {
            // Extraer texto del core
            let text = self.extract_text_from_core(&core);
            
            // Generar embedding
            let embedding = self.embedding_service.embed(&text).await.unwrap();
            
            // Crear voxel
            let voxel = Voxel {
                id: VoxelId::new(),
                timestamp: core.timestamp,
                embedding,
                coords: core.coords.clone(),
                core_id: core.id,
                alpha: core.alpha,
                text_preview: text.chars().take(200).collect(),
                qpx_meta: VoxelMetadata {
                    compressed_size: 0,
                    original_size: 0,
                    encoding_mode: EncodingMode::Full,
                },
            };
            
            // Insertar en VoxelDB
            self.voxel_db.insert(voxel).await.unwrap();
            
            log::info!("✅ Synced core {} → voxel", core.id);
        }
    }
    
    /// 2. Sincronizar voxels sin core → crear cores
    async fn sync_voxels_to_cores(&self) {
        // Buscar voxels sin core asociado
        let unsynced_voxels = self.voxel_db
            .query_without_core()
            .await
            .unwrap();
        
        for voxel in unsynced_voxels {
            // Crear core mínimo
            let core = QuantumCore {
                id: CoreId::new(),
                timestamp: voxel.timestamp,
                coords: voxel.coords.clone(),
                qpx_data: QPXEncoder::encode_text(&voxel.text_preview)?,
                alpha: voxel.alpha,
                branch: None,
                quantum_meta: QuantumMetadata {
                    intensity: voxel.coords.r,
                    probability: 0.5,
                    progress: 0.0,
                },
                entanglements: vec![],
            };
            
            // Insertar en TelescopeDB
            self.telescope_db.insert(core.clone()).await.unwrap();
            
            // Update voxel con core_id
            self.voxel_db.update_core_link(voxel.id, core.id).await.unwrap();
            
            log::info!("✅ Synced voxel {} → core", voxel.id);
        }
    }
    
    /// 3. Sincronizar coordenadas esféricas (si cambian)
    async fn sync_spherical_coords(&self) {
        // Obtener cores y voxels desincronizados
        let mismatched = self.find_coord_mismatches().await.unwrap();
        
        for (core_id, voxel_id) in mismatched {
            let core = self.telescope_db.get(&core_id).await.unwrap();
            
            // Actualizar voxel con coords del core (TelescopeDB es source of truth)
            self.voxel_db.update_coords(voxel_id, core.coords).await.unwrap();
            
            log::info!("✅ Synced coords: {} ↔ {}", core_id, voxel_id);
        }
    }
}
```

---

## 🌍 SPHERICAL ENCODING

### Coordenadas Esféricas para Embeddings

```rust
/// Coordenadas esféricas (r, θ, φ) heredadas de TelescopeDB
pub struct SphericalCoords {
    pub r: f64,      // Intensidad: 0.0 (baja) - 1.0 (alta)
    pub theta: f64,  // Categoría temática: 0.0 - 2π
    pub phi: f64,    // Valencia emocional: 0.0 - π
}

impl SphericalCoords {
    /// Calcular desde Context Token 7D (mismo que TelescopeDB)
    pub fn from_ctx7d(ctx: &ContextToken7D) -> Self {
        // r = Intensidad
        let r = (ctx.emotional.abs() + ctx.intentional).clamp(0.0, 1.0);
        
        // theta = Categoría temática (0-2π)
        let theta = Self::compute_theta(ctx);
        
        // phi = Valencia emocional (0-π)
        let phi = Self::compute_phi(ctx);
        
        SphericalCoords { r, theta, phi }
    }
    
    fn compute_theta(ctx: &ContextToken7D) -> f64 {
        let normalized = (ctx.semantic + ctx.contextual) / 2.0;
        normalized * 2.0 * std::f64::consts::PI
    }
    
    fn compute_phi(ctx: &ContextToken7D) -> f64 {
        (ctx.emotional + 1.0) / 2.0 * std::f64::consts::PI
    }
    
    /// Distancia euclidiana entre dos puntos esféricos
    pub fn distance(&self, other: &SphericalCoords) -> f64 {
        let dr = (self.r - other.r).powi(2);
        let dtheta = (self.theta - other.theta).powi(2);
        let dphi = (self.phi - other.phi).powi(2);
        
        (dr + dtheta + dphi).sqrt()
    }
}
```

### Query con Contexto Esférico

```rust
impl VoxelDB {
    /// Query con embedding + contexto esférico
    pub async fn query(&self, query: &VoxelQuery) -> Result<Vec<Voxel>> {
        // 1. HNSW search (similitud vectorial)
        let candidate_ids = self.hnsw_index
            .search(&query.embedding, query.k * 3, 100)  // 3x para filtros
            .await?;
        
        // 2. Cargar voxels completos
        let mut candidates = Vec::new();
        for id in candidate_ids {
            if let Ok(voxel) = self.get(&id).await {
                candidates.push(voxel);
            }
        }
        
        // 3. Filtrar por alpha channel (si especificado)
        if let Some(alpha_filter) = query.alpha_filter {
            candidates.retain(|v| v.alpha == alpha_filter);
        }
        
        // 4. Filtrar por time range (si especificado)
        if let Some(time_range) = query.time_range {
            let cutoff = Utc::now() - time_range;
            candidates.retain(|v| v.timestamp >= cutoff);
        }
        
        // 5. Filtrar por contexto esférico (si especificado)
        if let Some(spherical_context) = &query.spherical_context {
            candidates.retain(|v| {
                let distance = v.coords.distance(spherical_context);
                distance < 0.5  // Threshold configurable
            });
        }
        
        // 6. Re-rank por similitud combinada
        candidates.sort_by(|a, b| {
            let sim_a = self.combined_similarity(&query.embedding, a, &query.spherical_context);
            let sim_b = self.combined_similarity(&query.embedding, b, &query.spherical_context);
            sim_b.partial_cmp(&sim_a).unwrap()
        });
        
        // 7. Limitar a top-k
        candidates.truncate(query.k);
        
        Ok(candidates)
    }
    
    /// Similitud combinada (vectorial + esférica)
    fn combined_similarity(
        &self,
        query_embedding: &[f32],
        voxel: &Voxel,
        spherical_context: &Option<SphericalCoords>,
    ) -> f64 {
        // Similitud vectorial (cosine)
        let vector_sim = cosine_similarity(query_embedding, &voxel.embedding);
        
        // Similitud esférica (si hay contexto)
        let spherical_sim = if let Some(context) = spherical_context {
            let distance = voxel.coords.distance(context);
            1.0 - distance.min(1.0)
        } else {
            1.0  // Sin penalización si no hay contexto
        };
        
        // Combinar (60% vectorial, 40% esférica)
        0.6 * vector_sim + 0.4 * spherical_sim
    }
}
```

---

## 💾 STORAGE FORMAT: QPX VARIABLE-LENGTH

### Encoding de Embeddings a QPX

```rust
impl VoxelDB {
    /// Encode embedding → QPX bytes
    fn encode_embedding_qpx(&self, voxel: &Voxel) -> Result<Vec<u8>> {
        let mut encoder = QPXEmbeddingEncoder::new();
        
        // Decidir mode basado en dimensionalidad
        if voxel.embedding.len() < 256 {
            encoder.encode_compact(voxel)
        } else {
            encoder.encode_full(voxel)
        }
    }
    
    /// Decode QPX bytes → embedding
    fn decode_embedding_qpx(&self, bytes: &[u8]) -> Result<Voxel> {
        let mut decoder = QPXEmbeddingDecoder::new(bytes);
        
        let major_type = decoder.read_major_type()?;
        
        match major_type {
            MajorType::FloatArray => decoder.decode_compact(),
            MajorType::EmbeddingBlock => decoder.decode_full(),
            _ => Err(QPXError::InvalidEmbeddingType),
        }
    }
}

/// QPX Encoder específico para embeddings
pub struct QPXEmbeddingEncoder {
    buffer: Vec<u8>,
}

impl QPXEmbeddingEncoder {
    /// Compact mode: Array de floats comprimido
    fn encode_compact(&mut self, voxel: &Voxel) -> Result<Vec<u8>> {
        // Type 6 (FloatArray) + dimensión + valores
        self.buffer.push(0xC0);  // Major type 6
        self.write_u16(voxel.embedding.len() as u16)?;
        
        // Cuantización a 16-bit (reduce 50% tamaño)
        for &value in &voxel.embedding {
            let quantized = (value * 32767.0).round() as i16;
            self.buffer.extend_from_slice(&quantized.to_le_bytes());
        }
        
        Ok(self.buffer.clone())
    }
    
    /// Full mode: Embedding block con metadata
    fn encode_full(&mut self, voxel: &Voxel) -> Result<Vec<u8>> {
        // Header (64 bytes)
        let header = EmbeddingHeader {
            magic: [0x45, 0x4D, 0x42],  // "EMB"
            version: 0x15,              // v1.5
            dimensions: voxel.embedding.len() as u32,
            r: voxel.coords.r,
            theta: voxel.coords.theta,
            phi: voxel.coords.phi,
            alpha: voxel.alpha,
            timestamp: voxel.timestamp.timestamp(),
            core_id: voxel.core_id.to_bytes(),
            // ... resto del header (64 bytes total)
        };
        
        self.buffer.extend_from_slice(&header.to_bytes());
        
        // Embedding data (cuantizado 16-bit)
        for &value in &voxel.embedding {
            let quantized = (value * 32767.0).round() as i16;
            self.buffer.extend_from_slice(&quantized.to_le_bytes());
        }
        
        // Footer (checksum)
        let checksum = self.calculate_checksum();
        self.buffer.extend_from_slice(&checksum.to_le_bytes());
        
        Ok(self.buffer.clone())
    }
}
```

---

## 🔌 API PRINCIPAL

### Operaciones CRUD

```rust
impl VoxelDB {
    /// CREATE: Insertar nuevo Voxel
    pub async fn insert(&mut self, voxel: Voxel) -> Result<VoxelId> {
        // 1. Validar
        self.validate_voxel(&voxel)?;
        
        // 2. Encode QPX
        let qpx_bytes = self.encode_embedding_qpx(&voxel)?;
        
        // 3. Write to disk
        let path = self.compute_storage_path(&voxel);
        fs::write(&path, qpx_bytes).await?;
        
        // 4. Update HNSW index
        self.hnsw_index.insert(voxel.id, &voxel.embedding)?;
        
        // 5. Update spherical index
        self.spherical_index.insert(voxel.id, voxel.coords)?;
        
        // 6. Dual-helix: Link with TelescopeDB
        if voxel.core_id != CoreId::default() {
            self.dual_helix_sync.link_voxel_to_core(voxel.id, voxel.core_id).await?;
        }
        
        Ok(voxel.id)
    }
    
    /// READ: Obtener Voxel por ID
    pub async fn get(&self, id: &VoxelId) -> Result<Voxel> {
        let path = self.spherical_index.get_path(id)?;
        let qpx_bytes = fs::read(&path).await?;
        let voxel = self.decode_embedding_qpx(&qpx_bytes)?;
        
        if voxel.id != *id {
            return Err(VoxelError::CorruptedData);
        }
        
        Ok(voxel)
    }
    
    /// UPDATE: Actualizar Voxel existente
    pub async fn update(&mut self, id: &VoxelId, updater: impl FnOnce(&mut Voxel)) -> Result<()> {
        let mut voxel = self.get(id).await?;
        
        updater(&mut voxel);
        
        // Re-encode y write
        let qpx_bytes = self.encode_embedding_qpx(&voxel)?;
        let path = self.compute_storage_path(&voxel);
        fs::write(&path, qpx_bytes).await?;
        
        // Update HNSW (si embedding cambió)
        self.hnsw_index.update(voxel.id, &voxel.embedding)?;
        
        Ok(())
    }
    
    /// DELETE: Eliminar Voxel (soft delete)
    pub async fn delete(&mut self, id: &VoxelId) -> Result<()> {
        let voxel = self.get(id).await?;
        let old_path = self.compute_storage_path(&voxel);
        let new_path = self.compute_deleted_path(&voxel);
        
        fs::rename(&old_path, &new_path).await?;
        
        // Remove from indices
        self.hnsw_index.remove(id)?;
        self.spherical_index.remove(id)?;
        
        Ok(())
    }
}
```

### Query Operations

```rust
impl VoxelDB {
    /// Query por similitud vectorial + contexto
    pub async fn query(&self, query: &VoxelQuery) -> Result<Vec<Voxel>> {
        // Ver implementación en sección anterior
    }
    
    /// Query por core_id (dual-helix lookup)
    pub async fn query_by_core(&self, core_id: &CoreId) -> Result<Option<Voxel>> {
        let all_voxels = self.get_all_voxels().await?;
        
        Ok(all_voxels.into_iter().find(|v| v.core_id == *core_id))
    }
    
    /// Query por alpha channel (origen)
    pub async fn query_by_alpha(&self, alpha: u8) -> Result<Vec<Voxel>> {
        let all_voxels = self.get_all_voxels().await?;
        
        let filtered: Vec<_> = all_voxels.into_iter()
            .filter(|v| v.alpha == alpha)
            .collect();
        
        Ok(filtered)
    }
    
    /// Query por coordenadas esféricas
    pub async fn query_by_spherical(&self, center: SphericalCoords, radius: f64) -> Result<Vec<Voxel>> {
        let candidates = self.spherical_index
            .search_radius(center, radius)
            .await?;
        
        let mut voxels = Vec::new();
        for id in candidates {
            if let Ok(voxel) = self.get(&id).await {
                voxels.push(voxel);
            }
        }
        
        Ok(voxels)
    }
}
```

---

## 🎯 CASOS DE USO

### Caso 1: Búsqueda Semántica Básica

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let voxel_db = VoxelDB::new("./data")?;
    let embedding_service = EmbeddingService::new()?;
    
    // Usuario busca: "concurrency en Rust"
    let query_text = "concurrency en Rust";
    let query_embedding = embedding_service.embed(query_text).await?;
    
    let query = VoxelQuery {
        embedding: query_embedding,
        k: 5,
        spherical_context: None,
        alpha_filter: None,
        time_range: None,
        core_filter: None,
    };
    
    let results = voxel_db.query(&query).await?;
    
    println!("🔍 Top-5 resultados:");
    for voxel in results {
        println!("  - {}: {}", voxel.timestamp, voxel.text_preview);
    }
    
    Ok(())
}
```

### Caso 2: Búsqueda con Contexto Biográfico

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let voxel_db = VoxelDB::new("./data")?;
    let telescope_db = TelescopeDB::new("./data")?;
    let embedding_service = EmbeddingService::new()?;
    
    // Usuario busca con contexto emocional
    let query_text = "debugging ownership";
    let query_embedding = embedding_service.embed(query_text).await?;
    
    let query = VoxelQuery {
        embedding: query_embedding,
        k: 5,
        spherical_context: Some(SphericalCoords {
            r: 0.8,      // Alta intensidad
            theta: 0.3,  // Técnico
            phi: 1.2,    // Frustrado
        }),
        alpha_filter: Some(255),  // Solo native
        time_range: Some(Duration::days(30)),
        core_filter: None,
    };
    
    let voxels = voxel_db.query(&query).await?;
    
    // Dual-helix: Recuperar contexto biográfico
    for voxel in voxels {
        let core = telescope_db.get(&voxel.core_id).await?;
        println!("📅 {}: {}", core.timestamp, voxel.text_preview);
        println!("   Intensidad: {:.2}, Emoción: {:.2}", core.coords.r, core.coords.phi);
    }
    
    Ok(())
}
```

### Caso 3: Dual-Helix Sync Automático

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let telescope_db = Arc::new(TelescopeDB::new("./data")?);
    let voxel_db = Arc::new(VoxelDB::new("./data")?);
    let embedding_service = Arc::new(EmbeddingService::new()?);
    
    let dual_helix = DualHelixSync {
        telescope_db: telescope_db.clone(),
        voxel_db: voxel_db.clone(),
        embedding_service,
        sync_interval: Duration::minutes(5),
    };
    
    // Background task: sync continuo
    tokio::spawn(async move {
        dual_helix.run_continuous_sync().await;
    });
    
    // Main: Usuario inserta core en TelescopeDB
    let core = QuantumCore {
        id: CoreId::new(),
        timestamp: Utc::now(),
        coords: SphericalCoords { r: 0.9, theta: 0.3, phi: 1.5 },
        qpx_data: encode("Implementé HNSW sobre QPX"),
        alpha: 255,
        branch: None,
        quantum_meta: QuantumMetadata {
            intensity: 0.9,
            probability: 0.95,
            progress: 1.0,
        },
        entanglements: vec![],
    };
    
    telescope_db.insert(core.clone()).await?;
    
    // Esperar sync (5 min)
    tokio::time::sleep(Duration::minutes(6)).await;
    
    // Verificar que voxel fue creado
    let voxel = voxel_db.query_by_core(&core.id).await?;
    assert!(voxel.is_some());
    println!("✅ Dual-helix sync completado: core {} ↔ voxel", core.id);
    
    Ok(())
}
```

---

## ⚡ PERFORMANCE TARGETS

### Objetivos v1.5

| Operación | Target | Justificación |
|-----------|--------|---------------|
| **insert()** | <15ms | Encoding QPX + write file + HNSW insert |
| **get()** | <5ms | Read file + decode QPX |
| **query() KNN** | <50ms | HNSW search + spherical filter (k=10) |
| **query() with filters** | <100ms | HNSW + alpha + time + spherical |
| **dual_helix_sync()** | <500ms | Batch sync de 100 cores/voxels |
| **HNSW build** | <1s/1000 voxels | Construcción incremental |

### Métricas de Compresión

| Tipo de Dato | Original | QPX Compact | QPX Full | Ratio |
|--------------|----------|-------------|----------|-------|
| Float32 | 4 bytes | 2 bytes | 2 bytes | 2:1 |
| Embedding 768D | 3 KB | 1.5 KB | 1.6 KB | 2:1 |
| Embedding 1536D | 6 KB | 3 KB | 3.1 KB | 2:1 |
| Voxel completo | - | ~3.2 KB | ~3.3 KB | - |

### Escalabilidad

```
Voxels almacenados: 1M voxels
Tamaño promedio: 3.2 KB/voxel
Storage total: 3.2 GB

Con 100 voxels/día:
→ 1M voxels en ~27 años
→ Storage manejable en SSD consumer
→ HNSW search <50ms con M=16, ef_construction=200
```

---

## 🚀 PRÓXIMOS PASOS

### Implementación v1.5 (Prioridad CRÍTICA)

1. ✅ **Especificación completa** (este documento)
2. 🔄 **Implementar QPX embedding encoder** (src/core/qpx/)
3. 🔄 **Implementar HNSW pixel-native** (src/voxel/hnsw/)
4. 🔄 **Implementar DualHelixSync** (src/voxel/dual_helix/)
5. 🔄 **Integración con embedding service** (src/services/embedding/)
6. 🔄 **Tests end-to-end** (examples/test_voxeldb.rs)

### Integración con Ecosistema

- **TelescopeDB** ↔ VoxelDB (dual-helix sync bidireccional)
- **CTX7D** → VoxelDB (coordenadas esféricas)
- **SensoryEngine** → VoxelDB (embeddings de inputs)
- **PXLang** → VoxelDB (query language)

---

**Estado:** 📋 ESPECIFICACIÓN v1.5 COMPLETA  
**Complejidad:** ⚠️ ALTA - Componente crítico #2  
**Prioridad:** 🔴 CRÍTICA - Complemento esencial de TelescopeDB

---

*Generado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Pixel-Native Revolution*  
*"No es un vector database. Es un espacio semántico viviente codificado en píxeles."* 🧊✨
