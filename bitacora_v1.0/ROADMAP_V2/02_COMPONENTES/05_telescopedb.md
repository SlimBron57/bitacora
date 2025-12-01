```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/05_telescopedb.md
Versión: 1.5
Fecha Creación: 2025-11-27
Última Actualización: 2025-11-27
Autor: Sistema Bitácora - Arquitectura QPX v1.5 (documento reescrito desde cero)
Propósito: TelescopeDB como organismo cuántico 100% pixel-native con QPX + QuantumDao + EntanglementMap
Estado: 📋 ESPECIFICACIÓN v1.5 - Pixel-Native Revolution
Relacionado Con: 14_qpx-quantumdao-revolucion.md, 01_sistema-dual-databases.md, 02_flujo-datos-end-to-end.md
Implementa: DA-007 (TelescopeDB brecha crítica #1), DA-001 (Local-first), DA-011 (NO MongoDB)
Backup: 05_telescopedb.md.backup_v2.0 (arquitectura anterior para referencia)
# === FIN DATOS DE AUDITORÍA ===
```

# 🔭 TELESCOPEDB v1.5 - Organismo Cuántico Pixel-Native

> **"No es una base de datos. Es un organismo viviente que evoluciona con tu biografía."**

---

## 📚 TABLA DE CONTENIDOS

0. [Aclaración Arquitectónica: QPX vs TelescopeDB](#aclaración-arquitectónica-qpx-vs-telescopedb)
1. [Propósito](#propósito)
2. [Arquitectura QPX](#arquitectura-qpx)
3. [QuantumDao: Git para Biografías](#quantumdao-git-para-biografías)
4. [EntanglementMap: 5 Tipos de Relaciones](#entanglementmap-5-tipos-de-relaciones)
5. [Topology Monitoring & Self-Healing](#topology-monitoring--self-healing)
6. [Storage Format: QPX Variable-Length](#storage-format-qpx-variable-length)
7. [Indexación Esférica con Alpha Channel](#indexación-esférica-con-alpha-channel)
8. [API Principal](#api-principal)
9. [Casos de Uso](#casos-de-uso)
10. [Performance Targets](#performance-targets)

---

## 🔍 ACLARACIÓN ARQUITECTÓNICA: QPX vs TelescopeDB

**CRÍTICO - Entender la diferencia:**

```
┌─────────────────────────────────────────────────────────────┐
│ QPX (Quantum Pixel eXchange)                                │
│ ↓                                                           │
│ Formato de encoding variable-length                        │
│ (como CBOR, Protobuf, MessagePack)                         │
│                                                             │
│ Responsabilidad:                                            │
│ - Encode/decode datos a bytes                              │
│ - Variable-length (1 byte → 200 bytes)                     │
│ - 5 major types (Primitives, Pixel, Block, Core, Array)   │
│ - NO almacena, NO indexa, NO hace queries                  │
└─────────────────────────────────────────────────────────────┘
                        ↓ usa QPX como formato
┌─────────────────────────────────────────────────────────────┐
│ TelescopeDB                                                 │
│ ↓                                                           │
│ Base de datos biográfica (Primera DB que usa QPX)         │
│                                                             │
│ Responsabilidad:                                            │
│ - Almacenar QuantumCores en disco (.qpx files)            │
│ - Indexar coordenadas esféricas                            │
│ - Query por vecindad, tiempo, alpha, entanglements        │
│ - QuantumDao (Git workflow)                                │
│ - EntanglementMap (5 tipos)                                │
│ - Topology Monitoring & Self-Healing                       │
└─────────────────────────────────────────────────────────────┘
                        ↓ dual-helix sync
┌─────────────────────────────────────────────────────────────┐
│ VoxelDB                                                     │
│ ↓                                                           │
│ Base de datos semántica (Segunda DB que usa QPX)          │
│                                                             │
│ Responsabilidad:                                            │
│ - Almacenar embeddings como voxels (.qpx files)           │
│ - Indexar con HNSW para búsqueda semántica                │
│ - Query por similitud vectorial                            │
│ - Spherical encoding en QPX                                │
└─────────────────────────────────────────────────────────────┘
```

**Analogía clara:**

```rust
// ❌ INCORRECTO: "QPX es una base de datos"
let db = QPX::new();  // NO existe, QPX no es DB

// ✅ CORRECTO: "TelescopeDB usa formato QPX"
let telescope = TelescopeDB::new("./data")?;  // DB que usa QPX
let voxel = VoxelDB::new("./data")?;          // DB que usa QPX

// QPX es solo el encoder/decoder
let core = QuantumCore { /* ... */ };
let qpx_bytes = QPXEncoder::encode(&core)?;  // QPX = formato
telescope.insert_raw(qpx_bytes).await?;      // TelescopeDB = storage
```

**En resumen:**
- **QPX** = Protocolo de encoding (como JSON, CBOR)
- **TelescopeDB** = Base de datos #1 (memoria biográfica)
- **VoxelDB** = Base de datos #2 (embeddings semánticos)
- Ambas DBs usan QPX como formato interno

---

## 🎯 PROPÓSITO

### ¿Qué es TelescopeDB v1.5?

**TelescopeDB** es el **almacén de memoria biográfica cuántica** de Bitácora. No es una base de datos tradicional (NO SQLite, NO MongoDB). Es un **organismo viviente** que:

1. **Almacena memorias como QuantumCores** (QPX format)
2. **Versiona tu biografía como Git** (QuantumDao con branches)
3. **Descubre relaciones automáticamente** (EntanglementMap)
4. **Se auto-repara** (Topology Monitoring detecta broken links y orphaned nodes)
5. **Indexa en geometría esférica** (r, θ, φ) para búsqueda contextual
6. **Rastrea origen con alpha channel** (200-255 range)

### ¿Qué Problema Resuelve?

**Problema clásico:**
```
Usuario: "¿Recuerdas cuando debuggeamos el problema de ownership hace 2 semanas?"

LLM tradicional:
❌ No tiene memoria persistente
❌ No tiene contexto histórico
❌ No puede relacionar eventos pasados
❌ Cada sesión es un "reset"
```

**Solución TelescopeDB v1.5:**
```rust
// 1. Query con coordenadas esféricas + alpha filter
let query = SphericQuery {
    center: (r: 0.8, θ: 0.3, φ: 1.2),  // Alta intensidad, técnico, frustrado
    radius: 0.2,
    time_range: Some(14.days()),
    alpha_filter: Some(255),  // Solo native (no imports)
};

let results = telescope_db.query_spherical(&query).await?;
// → Recupera QuantumCores de esa sesión

// 2. Traverse entanglements
for core in results {
    let related = telescope_db.get_entangled(&core.id, EntanglementType::Causal).await?;
    // → Encuentra eventos causalmente relacionados
}

// 3. Topology check
let health = topology_monitor.check_link(&core.id).await?;
// → Verifica que link sigue válido, no huérfano

// Resultado:
// ✅ Memoria recuperada: "Sí, borrow checker conflict con Arc<Mutex<T>>"
// ✅ Contexto completo: 3 intentos previos, 2 soluciones fallidas
// ✅ Relaciones: Linked a "Rust concurrency patterns" topic
// ✅ Tiempo: <50ms, 100% local, 100% pixel-native
```

### Los 5 Imposibles que TelescopeDB v1.5 Logra

```rust
/// IMPOSIBLE #1: 100% Pixel-Native Storage (NO SQLite)
pub struct TelescopeDB {
    cores: HashMap<CoreId, QPXEncoded>,  // Todo es QPX, nada más
}

/// IMPOSIBLE #2: Git Workflow para Biografía
pub struct QuantumDao {
    main: Branch,                    // Vida diaria
    projects: HashMap<BranchId, ProjectBranch>,  // Proyectos reales
    commits: Vec<BiographicalCommit>,
    merges: Vec<RealityFusion>,      // Merge projects → main
}

/// IMPOSIBLE #3: Relaciones Auto-Discovered
pub struct EntanglementMap {
    causal: Vec<CausalLink>,         // A causó B (30 min window)
    temporal: Vec<TemporalLink>,     // A y B ocurrieron juntos
    semantic: Vec<SemanticLink>,     // A y B son similares (>0.85)
    emotional: Vec<EmotionalLink>,   // A y B tienen tono similar
    operational: Vec<OperationalLink>,  // A y B del mismo proyecto
}

/// IMPOSIBLE #4: Self-Healing Topology
pub struct TopologyMonitor {
    broken_links: Vec<BrokenLink>,
    orphaned_nodes: Vec<OrphanedNode>,
    cycles: Vec<Cycle>,
    auto_healer: AutoHealer,
}

/// IMPOSIBLE #5: Variable-Length Encoding (1 byte → 200 bytes)
pub enum QPXEncoded {
    Compact(Vec<u8>),   // <10 pixels, simple data
    Full {              // Complex QuantumCore
        header: [u8; 48],
        blocks: Vec<PixelBlock>,
    },
}
```

---

## 🏗️ ARQUITECTURA QPX

### Estructura de un QuantumCore

```rust
/// Un QuantumCore es la unidad fundamental de memoria en TelescopeDB
pub struct QuantumCore {
    /// Identificador único (SHA-256 hash del contenido)
    pub id: CoreId,
    
    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,
    
    /// Coordenadas esféricas para indexación
    pub coords: SphericalCoords {
        r: f64,      // Intensidad (0.0-1.0)
        theta: f64,  // Categoría temática (0-2π)
        phi: f64,    // Valencia emocional (0-π)
    },
    
    /// Contenido QPX encoded (variable-length)
    pub qpx_data: QPXEncoded,
    
    /// Alpha channel para trazabilidad
    pub alpha: u8,  // 255=Native, 210=WhatsApp, 200=MySQL, etc
    
    /// Branch context (None = main, Some = project)
    pub branch: Option<BranchId>,
    
    /// Metadata cuántica
    pub quantum_meta: QuantumMetadata {
        intensity: f64,      // 0.0-1.0
        probability: f64,    // Probabilidad de relevancia futura
        progress: f64,       // Si es proyecto: % completado
    },
    
    /// Entanglements hacia otros cores
    pub entanglements: Vec<Entanglement>,
}

/// Entanglement = relación cuántica entre dos cores
pub struct Entanglement {
    pub target_id: CoreId,
    pub ent_type: EntanglementType,
    pub strength: f64,  // 0.0-1.0
    pub direction: Direction,  // Bidirectional | Unidirectional
}

pub enum EntanglementType {
    Causal,       // A causó B
    Temporal,     // A y B ocurrieron juntos
    Semantic,     // A y B son similares
    Emotional,    // A y B tienen tono similar
    Operational,  // A y B del mismo proyecto
}
```

### Storage en Disco (100% Pixel-Native)

```rust
/// TelescopeDB NO usa SQLite. Todo es QPX en archivos binarios.
pub struct TelescopeDBStorage {
    /// data/telescope/cores/[year]/[month]/[core_id].qpx
    cores_dir: PathBuf,
    
    /// data/telescope/index/spherical.idx (coordenadas esféricas)
    spherical_index: SphericIndexFile,
    
    /// data/telescope/index/entanglements.idx (grafo de relaciones)
    entanglement_index: EntanglementGraphFile,
    
    /// data/telescope/dao/branches.qpx (QuantumDao state)
    dao_state: QuantumDaoFile,
    
    /// data/telescope/topology/health.log (topology monitoring)
    topology_log: TopologyHealthLog,
}

impl TelescopeDB {
    /// Guardar QuantumCore en disco
    pub async fn insert(&mut self, core: QuantumCore) -> Result<CoreId> {
        // 1. Encode a QPX
        let qpx_bytes = self.encode_qpx(&core)?;
        
        // 2. Calcular path: data/telescope/cores/2025/11/[core_id].qpx
        let path = self.compute_storage_path(&core);
        
        // 3. Escribir archivo
        fs::write(&path, qpx_bytes).await?;
        
        // 4. Actualizar índices
        self.spherical_index.insert(core.id, core.coords)?;
        self.entanglement_index.add_node(core.id)?;
        
        // 5. Si tiene branch, actualizar QuantumDao
        if let Some(branch_id) = core.branch {
            self.dao.add_commit(branch_id, core.id)?;
        }
        
        // 6. Topology check (async background)
        self.topology_monitor.check_new_node(core.id).await;
        
        Ok(core.id)
    }
    
    /// Recuperar QuantumCore desde disco
    pub async fn get(&self, id: &CoreId) -> Result<QuantumCore> {
        // 1. Buscar en índice esférico
        let path = self.spherical_index.get_path(id)?;
        
        // 2. Leer archivo QPX
        let qpx_bytes = fs::read(&path).await?;
        
        // 3. Decode QPX → QuantumCore
        let core = self.decode_qpx(&qpx_bytes)?;
        
        // 4. Validar integridad
        if core.id != *id {
            return Err(TelescopeError::CorruptedData);
        }
        
        Ok(core)
    }
}
```

---

## 🌳 QUANTUMDAO: GIT PARA BIOGRAFÍAS

### Conceptos Fundamentales

```rust
/// QuantumDao = Sistema de versioning biográfico
pub struct QuantumDao {
    /// main = vida diaria (branch principal)
    pub main: Branch,
    
    /// projects = branches de proyectos reales
    pub projects: HashMap<BranchId, ProjectBranch>,
    
    /// commits = snapshots biográficos
    pub commits: Vec<BiographicalCommit>,
    
    /// merges = fusiones de realidades
    pub merges: Vec<RealityFusion>,
}

/// Branch = línea temporal de memoria
pub struct Branch {
    pub id: BranchId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub parent: Option<BranchId>,  // None = main
    pub head: CoreId,              // Último commit
}

/// ProjectBranch = Branch para proyecto operacional
pub struct ProjectBranch {
    pub branch: Branch,
    pub project: Project,
    pub metadata: QuantumMetadata {
        intensity: f64,      // Actividad actual
        probability: f64,    // Probabilidad de completar
        progress: f64,       // % avance
    },
}

/// Proyecto operacional (ShuiDao integration)
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub jobs: Vec<Job>,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
}
```

### Workflow Git-Style

```rust
impl QuantumDao {
    /// Crear nuevo proyecto (como git branch)
    pub fn create_project_branch(&mut self, name: &str) -> Result<BranchId> {
        let branch_id = BranchId::new();
        
        let project_branch = ProjectBranch {
            branch: Branch {
                id: branch_id,
                name: name.to_string(),
                created_at: Utc::now(),
                parent: Some(self.main.id),
                head: self.main.head.clone(),
            },
            project: Project {
                id: ProjectId::new(),
                name: name.to_string(),
                jobs: Vec::new(),
                created_at: Utc::now(),
                deadline: None,
            },
            metadata: QuantumMetadata {
                intensity: 1.0,  // Alta al inicio
                probability: 0.5,
                progress: 0.0,
            },
        };
        
        self.projects.insert(branch_id, project_branch);
        
        Ok(branch_id)
    }
    
    /// Commit en proyecto (guardar memoria asociada)
    pub fn commit_to_project(
        &mut self,
        branch_id: BranchId,
        core_id: CoreId,
        message: &str,
    ) -> Result<()> {
        let commit = BiographicalCommit {
            id: CommitId::new(),
            branch_id,
            core_id,
            message: message.to_string(),
            timestamp: Utc::now(),
        };
        
        self.commits.push(commit);
        
        // Actualizar head del branch
        if let Some(branch) = self.projects.get_mut(&branch_id) {
            branch.branch.head = core_id;
            
            // Actualizar progress si mensaje lo indica
            if message.contains("completado") || message.contains("finished") {
                branch.metadata.progress += 0.1;
            }
        }
        
        Ok(())
    }
    
    /// Merge proyecto → main (proyecto completado)
    pub fn merge_project_to_main(&mut self, branch_id: BranchId) -> Result<()> {
        let project_branch = self.projects.get(&branch_id)
            .ok_or(DaoError::BranchNotFound)?;
        
        // Crear merge commit
        let merge = RealityFusion {
            id: MergeId::new(),
            from_branch: branch_id,
            to_branch: self.main.id,
            merged_at: Utc::now(),
            strategy: MergeStrategy::Squash,  // Consolidar commits
        };
        
        self.merges.push(merge);
        
        // Mover head de main al último commit del proyecto
        self.main.head = project_branch.branch.head.clone();
        
        // Archivar branch (no eliminar, mantener historia)
        if let Some(mut branch) = self.projects.remove(&branch_id) {
            branch.metadata.intensity = 0.0;  // Ya no activo
            branch.metadata.progress = 1.0;   // Completado
            // TODO: Move to archived_projects
        }
        
        Ok(())
    }
    
    /// Ver historia de un proyecto (git log)
    pub fn get_project_history(&self, branch_id: BranchId) -> Result<Vec<BiographicalCommit>> {
        let commits: Vec<_> = self.commits
            .iter()
            .filter(|c| c.branch_id == branch_id)
            .cloned()
            .collect();
        
        Ok(commits)
    }
}
```

### Ejemplo Real: "Implementar TelescopeDB"

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut telescope_db = TelescopeDB::new("./data")?;
    let mut dao = &mut telescope_db.dao;
    
    // 1. Crear branch para proyecto
    let branch_id = dao.create_project_branch("Implementar TelescopeDB")?;
    println!("✅ Branch creado: {:?}", branch_id);
    
    // 2. Commits durante el proyecto (memorias asociadas)
    let core1 = QuantumCore {
        id: CoreId::new(),
        timestamp: Utc::now(),
        coords: SphericalCoords { r: 0.9, theta: 0.3, phi: 1.5 },
        qpx_data: encode("Diseñé arquitectura QPX"),
        alpha: 255,
        branch: Some(branch_id),
        quantum_meta: QuantumMetadata {
            intensity: 1.0,
            probability: 0.8,
            progress: 0.2,
        },
        entanglements: vec![],
    };
    
    telescope_db.insert(core1.clone()).await?;
    dao.commit_to_project(branch_id, core1.id, "Diseño arquitectura completado")?;
    
    // 3. Más commits...
    let core2 = QuantumCore {
        // ... memoria de implementación
        branch: Some(branch_id),
        quantum_meta: QuantumMetadata {
            intensity: 0.8,
            probability: 0.9,
            progress: 0.6,
        },
        // ...
    };
    
    telescope_db.insert(core2.clone()).await?;
    dao.commit_to_project(branch_id, core2.id, "Storage QPX implementado")?;
    
    // 4. Proyecto completado → merge to main
    dao.merge_project_to_main(branch_id)?;
    println!("✅ Proyecto completado y mergeado a main");
    
    // 5. Ver historia
    let history = dao.get_project_history(branch_id)?;
    println!("📜 Historia del proyecto:");
    for commit in history {
        println!("  - {}: {}", commit.timestamp, commit.message);
    }
    
    Ok(())
}
```

---

## 🔗 ENTANGLEMENTMAP: 5 TIPOS DE RELACIONES

### Auto-Discovery de Entanglements

```rust
/// EntanglementDiscovery = Motor que descubre relaciones automáticamente
pub struct EntanglementDiscovery {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    git_adapter: Option<Arc<GitAdapter>>,
}

impl EntanglementDiscovery {
    /// Descubrir entanglements para un core recién insertado
    pub async fn discover_for_core(&self, core_id: &CoreId) -> Result<Vec<Entanglement>> {
        let core = self.telescope_db.get(core_id).await?;
        let mut entanglements = Vec::new();
        
        // 1. CAUSAL: Buscar cores en ventana temporal (30 min antes/después)
        entanglements.extend(self.discover_causal(&core).await?);
        
        // 2. TEMPORAL: Buscar cores con mismo timestamp (±5 min)
        entanglements.extend(self.discover_temporal(&core).await?);
        
        // 3. SEMANTIC: Buscar cores con similitud semántica >0.85
        entanglements.extend(self.discover_semantic(&core).await?);
        
        // 4. EMOTIONAL: Buscar cores con tono emocional similar
        entanglements.extend(self.discover_emotional(&core).await?;
        
        // 5. OPERATIONAL: Buscar cores del mismo proyecto/job/task
        entanglements.extend(self.discover_operational(&core).await?);
        
        Ok(entanglements)
    }
    
    /// 1. CAUSAL: A causó B (ventana temporal + similitud)
    async fn discover_causal(&self, core: &QuantumCore) -> Result<Vec<Entanglement>> {
        let window_start = core.timestamp - Duration::minutes(30);
        let window_end = core.timestamp + Duration::minutes(30);
        
        let candidates = self.telescope_db
            .query_time_range(window_start, window_end)
            .await?;
        
        let mut causal = Vec::new();
        
        for candidate in candidates {
            if candidate.id == core.id {
                continue;
            }
            
            // Calcular peso causal
            let temporal_proximity = self.temporal_proximity(core, &candidate);
            let semantic_similarity = self.semantic_similarity(core, &candidate);
            let branch_match = core.branch == candidate.branch;
            
            let causal_weight = temporal_proximity * semantic_similarity * if branch_match { 1.2 } else { 1.0 };
            
            if causal_weight > 0.7 {
                causal.push(Entanglement {
                    target_id: candidate.id,
                    ent_type: EntanglementType::Causal,
                    strength: causal_weight,
                    direction: Direction::Unidirectional,
                });
            }
        }
        
        Ok(causal)
    }
    
    /// 2. TEMPORAL: Ocurrieron juntos (Git commits ±5 min)
    async fn discover_temporal(&self, core: &QuantumCore) -> Result<Vec<Entanglement>> {
        if self.git_adapter.is_none() {
            return Ok(Vec::new());
        }
        
        let git = self.git_adapter.as_ref().unwrap();
        
        // Buscar Git commits cerca de core.timestamp
        let commits = git.get_commits_around(core.timestamp, Duration::minutes(5))?;
        
        let mut temporal = Vec::new();
        
        for commit in commits {
            // Buscar cores asociados al commit
            let related_cores = self.telescope_db
                .query_by_metadata("git_commit", &commit.hash)
                .await?;
            
            for related in related_cores {
                if related.id == core.id {
                    continue;
                }
                
                temporal.push(Entanglement {
                    target_id: related.id,
                    ent_type: EntanglementType::Temporal,
                    strength: 0.9,  // Alta porque es mismo commit
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        Ok(temporal)
    }
    
    /// 3. SEMANTIC: Similitud semántica >0.85
    async fn discover_semantic(&self, core: &QuantumCore) -> Result<Vec<Entanglement>> {
        // Obtener embedding del core
        let embedding = self.extract_embedding(core)?;
        
        // Buscar cores similares usando HNSW
        let similar = self.telescope_db
            .query_by_embedding(&embedding, 0.85)
            .await?;
        
        let mut semantic = Vec::new();
        
        for candidate in similar {
            if candidate.id == core.id {
                continue;
            }
            
            let similarity = self.cosine_similarity(&embedding, &self.extract_embedding(&candidate)?);
            
            if similarity > 0.85 {
                semantic.push(Entanglement {
                    target_id: candidate.id,
                    ent_type: EntanglementType::Semantic,
                    strength: similarity,
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        Ok(semantic)
    }
    
    /// 4. EMOTIONAL: Tono emocional similar (alpha channel ±20, g ±30)
    async fn discover_emotional(&self, core: &QuantumCore) -> Result<Vec<Entanglement>> {
        // Extraer alpha y g channel del primer pixel
        let (alpha, g) = self.extract_emotional_signature(core)?;
        
        // Buscar cores con signature similar
        let alpha_range = (alpha.saturating_sub(20), alpha.saturating_add(20));
        let g_range = (g.saturating_sub(30), g.saturating_add(30));
        
        let candidates = self.telescope_db
            .query_by_emotional_range(alpha_range, g_range)
            .await?;
        
        let mut emotional = Vec::new();
        
        for candidate in candidates {
            if candidate.id == core.id {
                continue;
            }
            
            let (cand_alpha, cand_g) = self.extract_emotional_signature(&candidate)?;
            
            // Distancia euclidiana normalizada
            let alpha_diff = (alpha as f64 - cand_alpha as f64).abs() / 255.0;
            let g_diff = (g as f64 - cand_g as f64).abs() / 255.0;
            let distance = (alpha_diff.powi(2) + g_diff.powi(2)).sqrt();
            
            let similarity = 1.0 - distance;
            
            if similarity > 0.75 {
                emotional.push(Entanglement {
                    target_id: candidate.id,
                    ent_type: EntanglementType::Emotional,
                    strength: similarity,
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        Ok(emotional)
    }
    
    /// 5. OPERATIONAL: Mismo Project/Job/Task (ShuiDao integration)
    async fn discover_operational(&self, core: &QuantumCore) -> Result<Vec<Entanglement>> {
        if core.branch.is_none() {
            return Ok(Vec::new());  // No está en proyecto
        }
        
        let branch_id = core.branch.unwrap();
        
        // Buscar todos los cores del mismo branch
        let same_branch = self.telescope_db
            .query_by_branch(branch_id)
            .await?;
        
        let mut operational = Vec::new();
        
        for candidate in same_branch {
            if candidate.id == core.id {
                continue;
            }
            
            operational.push(Entanglement {
                target_id: candidate.id,
                ent_type: EntanglementType::Operational,
                strength: 1.0,  // Máximo porque es mismo proyecto
                direction: Direction::Bidirectional,
            });
        }
        
        Ok(operational)
    }
}
```

---

## 🏥 TOPOLOGY MONITORING & SELF-HEALING

### Sistema de Auto-Reparación

```rust
/// TopologyMonitor = Vigilante de la salud del grafo
pub struct TopologyMonitor {
    telescope_db: Arc<TelescopeDB>,
    check_interval: Duration,
}

impl TopologyMonitor {
    /// Monitoreo continuo (background task)
    pub async fn run_continuous_monitoring(&self) -> ! {
        loop {
            let health = self.check_topology_health().await;
            
            if health.integrity_score < 0.9 {
                self.auto_heal(&health).await;
            }
            
            self.log_health_report(&health);
            
            tokio::time::sleep(self.check_interval).await;
        }
    }
    
    /// Check completo de salud del grafo
    async fn check_topology_health(&self) -> TopologyHealth {
        let mut health = TopologyHealth::default();
        
        // 1. Detectar broken links
        health.broken_links = self.detect_broken_links().await;
        
        // 2. Detectar orphaned nodes
        health.orphaned_nodes = self.detect_orphaned_nodes().await;
        
        // 3. Detectar cycles peligrosos
        health.cycles = self.detect_dangerous_cycles().await;
        
        // 4. Calcular integrity score
        health.integrity_score = self.calculate_integrity(&health);
        
        health
    }
    
    /// Detectar broken links (entanglements a cores inexistentes)
    async fn detect_broken_links(&self) -> Vec<BrokenLink> {
        let all_cores = self.telescope_db.get_all_cores().await.unwrap();
        let mut broken = Vec::new();
        
        for core in all_cores {
            for ent in &core.entanglements {
                // Verificar que target existe
                if !self.telescope_db.exists(&ent.target_id).await.unwrap() {
                    broken.push(BrokenLink {
                        from: core.id.clone(),
                        to: ent.target_id.clone(),
                        ent_type: ent.ent_type,
                        severity: Severity::Critical,
                    });
                }
            }
        }
        
        broken
    }
    
    /// Detectar orphaned nodes (sin entanglements + no usado >90 días)
    async fn detect_orphaned_nodes(&self) -> Vec<OrphanedNode> {
        let all_cores = self.telescope_db.get_all_cores().await.unwrap();
        let mut orphaned = Vec::new();
        
        for core in all_cores {
            if core.entanglements.is_empty() {
                let last_accessed = self.get_last_access_time(&core.id).await.unwrap();
                let days_unused = (Utc::now() - last_accessed).num_days();
                
                if days_unused > 90 {
                    orphaned.push(OrphanedNode {
                        node_id: core.id.clone(),
                        days_unused,
                        reason: "No entanglements + unused > 90 days".into(),
                    });
                }
            }
        }
        
        orphaned
    }
    
    /// Auto-reparación del grafo
    async fn auto_heal(&self, health: &TopologyHealth) {
        // 1. Reparar broken links
        for broken_link in &health.broken_links {
            self.repair_broken_link(broken_link).await;
        }
        
        // 2. Adoptar orphaned nodes
        for orphan in &health.orphaned_nodes {
            self.adopt_orphan(orphan).await;
        }
        
        // 3. Resolver cycles
        for cycle in &health.cycles {
            self.resolve_cycle(cycle).await;
        }
    }
    
    /// Reparar broken link (buscar core similar para re-link)
    async fn repair_broken_link(&self, link: &BrokenLink) {
        // Buscar core similar al deleted
        let similar = self.telescope_db
            .find_similar_to_deleted(&link.to)
            .await
            .unwrap();
        
        if let Some(replacement) = similar.first() {
            // Crear nuevo entanglement a replacement
            self.telescope_db.add_entanglement(
                &link.from,
                Entanglement {
                    target_id: replacement.id.clone(),
                    strength: 0.7,  // Más débil (es inferido)
                    ent_type: link.ent_type,
                    direction: Direction::Bidirectional,
                }
            ).await.unwrap();
            
            log::info!("✅ Repaired broken link: {} → {}", link.from, replacement.id);
        }
    }
    
    /// Adoptar orphaned node (conectar a collection "historical_archives")
    async fn adopt_orphan(&self, orphan: &OrphanedNode) {
        self.telescope_db.add_entanglement(
            &orphan.node_id,
            Entanglement {
                target_id: "collection:historical_archives".into(),
                strength: 0.5,
                ent_type: EntanglementType::Operational,
                direction: Direction::Bidirectional,
            }
        ).await.unwrap();
        
        log::info!("✅ Adopted orphan: {} → historical_archives", orphan.node_id);
    }
}

pub struct TopologyHealth {
    pub broken_links: Vec<BrokenLink>,
    pub orphaned_nodes: Vec<OrphanedNode>,
    pub cycles: Vec<Cycle>,
    pub integrity_score: f64,  // 0.0 - 1.0
}
```

---

## 💾 STORAGE FORMAT: QPX VARIABLE-LENGTH

### Encoding de QuantumCore a QPX

```rust
impl TelescopeDB {
    /// Encode QuantumCore → QPX bytes
    fn encode_qpx(&self, core: &QuantumCore) -> Result<Vec<u8>> {
        let mut encoder = QPXEncoder::new();
        
        // 1. Decidir mode (Compact vs Full)
        let pixel_count = self.estimate_pixel_count(core);
        let score = core.quantum_meta.intensity;
        
        if pixel_count < 10 && score < 1.0 {
            // Compact mode
            encoder.encode_compact(core)
        } else {
            // Full mode
            encoder.encode_full(core)
        }
    }
    
    /// Decode QPX bytes → QuantumCore
    fn decode_qpx(&self, bytes: &[u8]) -> Result<QuantumCore> {
        let mut decoder = QPXDecoder::new(bytes);
        
        // 1. Leer header para determinar mode
        let major_type = decoder.read_major_type()?;
        
        match major_type {
            MajorType::Primitives | MajorType::SinglePixel | MajorType::PixelBlock => {
                // Compact mode
                decoder.decode_compact()
            }
            MajorType::QuantumCore => {
                // Full mode
                decoder.decode_full()
            }
            _ => Err(QPXError::UnknownMajorType),
        }
    }
}

/// QPXEncoder para QuantumCore
pub struct QPXEncoder {
    buffer: Vec<u8>,
}

impl QPXEncoder {
    /// Compact mode: Variable-length encoding
    fn encode_compact(&mut self, core: &QuantumCore) -> Result<Vec<u8>> {
        // Type 0 (Primitives) para metadata
        self.write_primitive_u8(core.alpha)?;
        self.write_primitive_f64(core.coords.r)?;
        self.write_primitive_f64(core.coords.theta)?;
        self.write_primitive_f64(core.coords.phi)?;
        
        // Type 1 (SinglePixel) para cada pixel del core
        for pixel in self.extract_pixels(core)? {
            self.write_pixel(pixel)?;
        }
        
        Ok(self.buffer.clone())
    }
    
    /// Full mode: 48-byte header + pixel blocks
    fn encode_full(&mut self, core: &QuantumCore) -> Result<Vec<u8>> {
        // Header (48 bytes)
        let header = QuantumHeader {
            magic: [0x51, 0x50, 0x58],  // "QPX"
            version: 0x15,              // v1.5
            score: core.quantum_meta.intensity,
            pixel_count: self.estimate_pixel_count(core) as u32,
            r: core.coords.r,
            theta: core.coords.theta,
            phi: core.coords.phi,
            alpha: core.alpha,
            timestamp: core.timestamp.timestamp(),
            branch_id: core.branch.map(|b| b.0).unwrap_or(0),
            // ... resto del header (48 bytes total)
        };
        
        self.buffer.extend_from_slice(&header.to_bytes());
        
        // Pixel blocks
        for pixel in self.extract_pixels(core)? {
            self.write_pixel(pixel)?;
        }
        
        // Footer (checksum)
        let checksum = self.calculate_checksum();
        self.buffer.extend_from_slice(&checksum.to_le_bytes());
        
        Ok(self.buffer.clone())
    }
    
    fn write_pixel(&mut self, pixel: Pixel) -> Result<()> {
        // Type 1 (SinglePixel) + RGBA
        self.buffer.push(0x20);  // Major type 1
        self.buffer.push(pixel.r);
        self.buffer.push(pixel.g);
        self.buffer.push(pixel.b);
        self.buffer.push(pixel.a);
        Ok(())
    }
}
```

---

## 🌍 INDEXACIÓN ESFÉRICA CON ALPHA CHANNEL

### Coordenadas Esféricas

```rust
/// Coordenadas esféricas (r, θ, φ)
pub struct SphericalCoords {
    pub r: f64,      // Intensidad: 0.0 (baja) - 1.0 (alta)
    pub theta: f64,  // Categoría temática: 0.0 - 2π
    pub phi: f64,    // Valencia emocional: 0.0 - π
}

impl SphericalCoords {
    /// Calcular desde Context Token 7D
    pub fn from_ctx7d(ctx: &ContextToken7D) -> Self {
        // r = Intensidad (combinación de emotional + intentional)
        let r = (ctx.emotional.abs() + ctx.intentional).clamp(0.0, 1.0);
        
        // theta = Categoría temática (0-2π)
        let theta = Self::compute_theta(ctx);
        
        // phi = Valencia emocional (0-π)
        let phi = Self::compute_phi(ctx);
        
        SphericalCoords { r, theta, phi }
    }
    
    fn compute_theta(ctx: &ContextToken7D) -> f64 {
        // Mapear semantic + contextual → 0-2π
        let normalized = (ctx.semantic + ctx.contextual) / 2.0;
        normalized * 2.0 * std::f64::consts::PI
    }
    
    fn compute_phi(ctx: &ContextToken7D) -> f64 {
        // Mapear emotional → 0-π
        // -1.0 (negativo) → 0, 0.0 (neutral) → π/2, 1.0 (positivo) → π
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

### Query Esférica con Alpha Filter

```rust
impl TelescopeDB {
    /// Query por vecindad esférica + alpha channel filter
    pub async fn query_spherical(
        &self,
        query: &SphericQuery,
    ) -> Result<Vec<QuantumCore>> {
        let mut results = Vec::new();
        
        // 1. Buscar en índice esférico
        let candidates = self.spherical_index
            .search_radius(query.center, query.radius)
            .await?;
        
        // 2. Filtrar por alpha channel (si especificado)
        let filtered = if let Some(alpha_filter) = query.alpha_filter {
            candidates.into_iter()
                .filter(|core| core.alpha == alpha_filter)
                .collect()
        } else {
            candidates
        };
        
        // 3. Filtrar por time range (si especificado)
        let time_filtered = if let Some(time_range) = query.time_range {
            filtered.into_iter()
                .filter(|core| {
                    let age = Utc::now() - core.timestamp;
                    age <= time_range
                })
                .collect()
        } else {
            filtered
        };
        
        // 4. Ordenar por distancia
        let mut sorted = time_filtered;
        sorted.sort_by(|a, b| {
            let dist_a = a.coords.distance(&query.center);
            let dist_b = b.coords.distance(&query.center);
            dist_a.partial_cmp(&dist_b).unwrap()
        });
        
        // 5. Limitar resultados
        results.extend(sorted.into_iter().take(query.limit.unwrap_or(10)));
        
        Ok(results)
    }
}

pub struct SphericQuery {
    pub center: SphericalCoords,
    pub radius: f64,
    pub alpha_filter: Option<u8>,
    pub time_range: Option<Duration>,
    pub limit: Option<usize>,
}
```

---

## 🔌 API PRINCIPAL

### Operaciones CRUD

```rust
impl TelescopeDB {
    /// CREATE: Insertar nuevo QuantumCore
    pub async fn insert(&mut self, core: QuantumCore) -> Result<CoreId> {
        // 1. Validar
        self.validate_core(&core)?;
        
        // 2. Discover entanglements
        let entanglements = self.entanglement_discovery
            .discover_for_core(&core.id)
            .await?;
        
        let mut core_with_ents = core;
        core_with_ents.entanglements = entanglements;
        
        // 3. Encode QPX
        let qpx_bytes = self.encode_qpx(&core_with_ents)?;
        
        // 4. Write to disk
        let path = self.compute_storage_path(&core_with_ents);
        fs::write(&path, qpx_bytes).await?;
        
        // 5. Update indices
        self.spherical_index.insert(core_with_ents.id, core_with_ents.coords)?;
        self.entanglement_index.add_node(core_with_ents.id)?;
        
        // 6. QuantumDao update
        if let Some(branch_id) = core_with_ents.branch {
            self.dao.add_commit(branch_id, core_with_ents.id)?;
        }
        
        // 7. Topology check (background)
        self.topology_monitor.check_new_node(core_with_ents.id).await;
        
        Ok(core_with_ents.id)
    }
    
    /// READ: Obtener QuantumCore por ID
    pub async fn get(&self, id: &CoreId) -> Result<QuantumCore> {
        let path = self.spherical_index.get_path(id)?;
        let qpx_bytes = fs::read(&path).await?;
        let core = self.decode_qpx(&qpx_bytes)?;
        
        if core.id != *id {
            return Err(TelescopeError::CorruptedData);
        }
        
        Ok(core)
    }
    
    /// UPDATE: Actualizar QuantumCore existente
    pub async fn update(&mut self, id: &CoreId, updater: impl FnOnce(&mut QuantumCore)) -> Result<()> {
        let mut core = self.get(id).await?;
        
        updater(&mut core);
        
        // Re-encode y write
        let qpx_bytes = self.encode_qpx(&core)?;
        let path = self.compute_storage_path(&core);
        fs::write(&path, qpx_bytes).await?;
        
        // Update indices
        self.spherical_index.update(core.id, core.coords)?;
        
        Ok(())
    }
    
    /// DELETE: Eliminar QuantumCore (soft delete)
    pub async fn delete(&mut self, id: &CoreId) -> Result<()> {
        // Soft delete: mover a deleted/
        let core = self.get(id).await?;
        let old_path = self.compute_storage_path(&core);
        let new_path = self.compute_deleted_path(&core);
        
        fs::rename(&old_path, &new_path).await?;
        
        // Remove from indices
        self.spherical_index.remove(id)?;
        self.entanglement_index.remove_node(id)?;
        
        Ok(())
    }
}
```

### Query Operations

```rust
impl TelescopeDB {
    /// Query por vecindad esférica
    pub async fn query_spherical(&self, query: &SphericQuery) -> Result<Vec<QuantumCore>> {
        // Ver implementación en sección anterior
    }
    
    /// Query por entanglement traversal
    pub async fn get_entangled(
        &self,
        id: &CoreId,
        ent_type: EntanglementType,
    ) -> Result<Vec<QuantumCore>> {
        let core = self.get(id).await?;
        let mut results = Vec::new();
        
        for ent in core.entanglements {
            if ent.ent_type == ent_type {
                if let Ok(related) = self.get(&ent.target_id).await {
                    results.push(related);
                }
            }
        }
        
        Ok(results)
    }
    
    /// Query por range temporal
    pub async fn query_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<QuantumCore>> {
        let all_cores = self.get_all_cores().await?;
        
        let filtered: Vec<_> = all_cores.into_iter()
            .filter(|c| c.timestamp >= start && c.timestamp <= end)
            .collect();
        
        Ok(filtered)
    }
    
    /// Query por branch (proyecto específico)
    pub async fn query_by_branch(&self, branch_id: BranchId) -> Result<Vec<QuantumCore>> {
        let all_cores = self.get_all_cores().await?;
        
        let filtered: Vec<_> = all_cores.into_iter()
            .filter(|c| c.branch == Some(branch_id))
            .collect();
        
        Ok(filtered)
    }
    
    /// Query por alpha channel (origen de datos)
    pub async fn query_by_alpha(&self, alpha: u8) -> Result<Vec<QuantumCore>> {
        let all_cores = self.get_all_cores().await?;
        
        let filtered: Vec<_> = all_cores.into_iter()
            .filter(|c| c.alpha == alpha)
            .collect();
        
        Ok(filtered)
    }
}
```

---

## 🎯 CASOS DE USO

### Caso 1: Guardar Memoria de Sesión

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut telescope_db = TelescopeDB::new("./data")?;
    
    // Usuario debuggeó problema de ownership
    let ctx7d = ContextToken7D {
        temporal: 0.95,
        semantic: 0.88,
        contextual: 0.82,
        relational: 0.70,
        emotional: -0.3,  // Frustrado al inicio
        intentional: 0.92,
        biographical: 0.85,
    };
    
    let coords = SphericalCoords::from_ctx7d(&ctx7d);
    
    let core = QuantumCore {
        id: CoreId::new(),
        timestamp: Utc::now(),
        coords,
        qpx_data: encode("Debugged borrow checker conflict with Arc<Mutex<T>>. Solved using channels."),
        alpha: 255,  // Native
        branch: None,  // En main (no proyecto específico)
        quantum_meta: QuantumMetadata {
            intensity: 0.9,
            probability: 1.0,  // Alta certeza (ya resuelto)
            progress: 1.0,
        },
        entanglements: vec![],
    };
    
    let id = telescope_db.insert(core).await?;
    println!("✅ Memoria guardada: {:?}", id);
    
    Ok(())
}
```

### Caso 2: Recuperar Memorias Similares

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let telescope_db = TelescopeDB::new("./data")?;
    
    // Usuario pregunta: "¿Recuerdas cuando debuggeamos ownership?"
    let query_coords = SphericalCoords {
        r: 0.8,      // Alta intensidad
        theta: 0.3,  // Técnico
        phi: 1.2,    // Frustración inicial
    };
    
    let query = SphericQuery {
        center: query_coords,
        radius: 0.3,
        alpha_filter: Some(255),  // Solo native
        time_range: Some(Duration::days(30)),
        limit: Some(5),
    };
    
    let results = telescope_db.query_spherical(&query).await?;
    
    println!("🔍 Memorias similares encontradas:");
    for core in results {
        println!("  - {}: {:?}", core.timestamp, core.id);
        
        // Traverse entanglements
        let related = telescope_db.get_entangled(&core.id, EntanglementType::Causal).await?;
        println!("    Relacionados: {}", related.len());
    }
    
    Ok(())
}
```

### Caso 3: Proyecto con Branches

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut telescope_db = TelescopeDB::new("./data")?;
    
    // Crear proyecto "Implementar TelescopeDB"
    let branch_id = telescope_db.dao.create_project_branch("Implementar TelescopeDB")?;
    
    // Día 1: Diseño
    let core1 = QuantumCore {
        id: CoreId::new(),
        timestamp: Utc::now(),
        coords: SphericalCoords { r: 0.9, theta: 0.3, phi: 1.5 },
        qpx_data: encode("Diseñé arquitectura QPX con variable-length encoding"),
        alpha: 255,
        branch: Some(branch_id),
        quantum_meta: QuantumMetadata {
            intensity: 1.0,
            probability: 0.8,
            progress: 0.1,
        },
        entanglements: vec![],
    };
    
    telescope_db.insert(core1.clone()).await?;
    telescope_db.dao.commit_to_project(branch_id, core1.id, "Diseño arquitectura")?;
    
    // Día 2: Implementación
    let core2 = QuantumCore {
        id: CoreId::new(),
        timestamp: Utc::now() + Duration::days(1),
        coords: SphericalCoords { r: 0.85, theta: 0.3, phi: 1.6 },
        qpx_data: encode("Implementé QPXEncoder con compact y full mode"),
        alpha: 255,
        branch: Some(branch_id),
        quantum_meta: QuantumMetadata {
            intensity: 0.9,
            probability: 0.9,
            progress: 0.5,
        },
        entanglements: vec![
            Entanglement {
                target_id: core1.id,
                ent_type: EntanglementType::Causal,
                strength: 0.95,
                direction: Direction::Unidirectional,
            },
        ],
    };
    
    telescope_db.insert(core2.clone()).await?;
    telescope_db.dao.commit_to_project(branch_id, core2.id, "Implementación QPX")?;
    
    // Proyecto completado → merge
    telescope_db.dao.merge_project_to_main(branch_id)?;
    
    // Ver historia
    let history = telescope_db.dao.get_project_history(branch_id)?;
    println!("📜 Historia del proyecto:");
    for commit in history {
        println!("  {}: {}", commit.timestamp, commit.message);
    }
    
    Ok(())
}
```

---

## ⚡ PERFORMANCE TARGETS

### Objetivos v1.5

| Operación | Target | Justificación |
|-----------|--------|---------------|
| **insert()** | <10ms | Encoding QPX + write file + update indices |
| **get()** | <5ms | Read file + decode QPX |
| **query_spherical()** | <50ms | Spherical index + top-10 results |
| **query_by_embedding()** | <100ms | HNSW search + decode |
| **entanglement_discovery** | <200ms | 5 discovery algorithms |
| **topology_check()** | <500ms | Full graph health check |
| **auto_heal()** | <2s | Repair broken links + adopt orphans |

### Métricas de Compresión

| Tipo de Dato | Original | QPX Compact | QPX Full | Ratio |
|--------------|----------|-------------|----------|-------|
| Boolean | 1 byte | 1 byte | 1 byte | 1:1 |
| Small int (0-255) | 4 bytes | 1 byte | 1 byte | 4:1 |
| Pixel RGBA | 4 bytes | 5 bytes | 5 bytes | 0.8:1 |
| QuantumCore simple | - | ~20 bytes | ~200 bytes | - |
| QuantumCore complejo | - | ~50 bytes | ~500 bytes | - |

### Escalabilidad

```
Cores almacenados: 1M cores
Tamaño promedio: 200 bytes/core
Storage total: 200 MB

Con 100 cores/día:
→ 1M cores en ~27 años
→ Storage manejable en SSD consumer
→ Query <50ms con indices apropiados
```

---

## 🚀 PRÓXIMOS PASOS

### Implementación v1.5 (Prioridad CRÍTICA)

1. ✅ **Especificación completa** (este documento)
2. 🔄 **Implementar storage QPX** (src/core/qpx/)
3. 🔄 **Implementar QuantumDao** (src/dao/)
4. 🔄 **Implementar EntanglementDiscovery** (src/entanglement/)
5. 🔄 **Implementar TopologyMonitor** (src/topology/)
6. 🔄 **Tests end-to-end** (examples/test_telescopedb.rs)

### Integración con Ecosistema

- **SensoryEngine** → TelescopeDB (persistir inputs)
- **CTX7D** → TelescopeDB (calcular coordenadas esféricas)
- **VoxelDB** → TelescopeDB (sincronización dual-helix)
- **ShuiDao** → TelescopeDB (Project/Job/Task entanglements)
- **PXLang** → TelescopeDB (query language)

---

**Estado:** 📋 ESPECIFICACIÓN v1.5 COMPLETA  
**Complejidad:** ⚠️ ALTA - Componente crítico #1  
**Prioridad:** 🔴 CRÍTICA - Bloquea todo el pipeline

---

*Generado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Pixel-Native Revolution*  
*"No es una base de datos. Es un organismo cuántico viviente."* 🔭✨
