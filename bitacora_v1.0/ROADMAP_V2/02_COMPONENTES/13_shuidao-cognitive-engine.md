```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/13_shuidao-cognitive-engine.md
Versión: 1.5
Fecha Creación: 2025-11-27
Última Actualización: 2025-11-27
Autor: Sistema Bitácora - Arquitectura QPX v1.5 (documento reescrito desde cero)
Propósito: ShuiDao como motor cognitivo 100% integrado con QuantumDao + Project/Job/Task
Estado: 📋 ESPECIFICACIÓN v1.5 - Cognitive Revolution
Relacionado Con: 14_qpx-quantumdao-revolucion.md, 05_telescopedb.md, 06_voxeldb.md
Implementa: DA-032 (ShuiDao cognitive architecture), DA-033 (Topic detection), DA-034 (Emotional space)
Backup: 13_shuidao-cognitive-engine.md.backup_v1.0 (arquitectura anterior para referencia)
# === FIN DATOS DE AUDITORÍA ===
```

# 🧠 SHUIDAO v1.5 - Motor Cognitivo Cuántico

> **"No es un intent detector. Es un compañero que comprende tu realidad operacional."**

---

## 📚 TABLA DE CONTENIDOS

1. [Propósito](#propósito)
2. [Integración con QuantumDao](#integración-con-quantumdao)
3. [Project/Job/Task Hierarchy](#projectjobtask-hierarchy)
4. [Operational Entanglements](#operational-entanglements)
5. [Intention Detection](#intention-detection)
6. [Topic Graph (Sub-componente 13a)](#topic-graph-sub-componente-13a)
7. [Emotional Space (Sub-componente 13b)](#emotional-space-sub-componente-13b)
8. [Cognitive Modes](#cognitive-modes)
9. [API Principal](#api-principal)
10. [Casos de Uso](#casos-de-uso)

---

## 🎯 PROPÓSITO

### ¿Qué es ShuiDao v1.5?

**ShuiDao** (水道 = "canal de agua") es el **motor cognitivo** de Bitácora que comprende tu **realidad operacional**:

1. **Detecta intenciones** (no solo keywords)
2. **Gestiona proyectos reales** via QuantumDao branches
3. **Trackea progreso** con Project/Job/Task hierarchy
4. **Descubre operational entanglements** (cores del mismo proyecto)
5. **Adapta respuestas** según modo cognitivo (Exploratory, Operational, Reflective, etc)
6. **Integra Topic Graph** (qué temas discutimos)
7. **Integra Emotional Space** (cómo te sientes)

### ¿Qué Problema Resuelve?

**Problema clásico:**
```
Usuario: "Tengo que instalar un switch en la oficina"

LLM tradicional:
❌ Responde con documentación genérica de redes
❌ No crea tracking de proyecto
❌ No relaciona con conversaciones previas
❌ Olvida el contexto en siguiente sesión
```

**Solución ShuiDao v1.5:**
```rust
// 1. Intention Detection
let intention = shuidao.detect_intention("Tengo que instalar un switch").await?;
// → IntentionType::OperationalProject (confidence: 0.89)

// 2. Crear proyecto en QuantumDao
let branch_id = telescope_db.dao.create_project_branch("Instalar switch oficina")?;

// 3. Desglosar en Jobs y Tasks
let project = shuidao.decompose_project("Instalar switch oficina", branch_id).await?;
// Project:
//   Job 1: "Comprar hardware"
//     - Task: Investigar modelos
//     - Task: Comparar precios
//     - Task: Realizar compra
//   Job 2: "Instalación física"
//     - Task: Planificar cableado
//     - Task: Instalar switch
//     - Task: Conectar dispositivos
//   Job 3: "Configuración"
//     - Task: Configurar VLANs
//     - Task: Testing

// 4. Persistir en TelescopeDB
for job in project.jobs {
    for task in job.tasks {
        let core = QuantumCore {
            id: CoreId::new(),
            timestamp: Utc::now(),
            coords: SphericalCoords { r: 0.7, theta: 0.5, phi: 1.4 },
            qpx_data: encode(&task),
            alpha: 255,
            branch: Some(branch_id),  // Linked to project
            quantum_meta: QuantumMetadata {
                intensity: 0.7,
                probability: 0.8,
                progress: 0.0,
            },
            entanglements: vec![],
        };
        
        telescope_db.insert(core).await?;
    }
}

// 5. Track progreso
let progress = shuidao.get_project_progress(branch_id).await?;
// → 0/9 tasks completed

// Resultado:
// ✅ Proyecto creado en QuantumDao
// ✅ Desglose en 3 Jobs, 9 Tasks
// ✅ Tracking de progreso persistente
// ✅ Operational entanglements auto-discovered
// ✅ Contexto disponible en próximas sesiones
```

### Los 5 Imposibles que ShuiDao v1.5 Logra

```rust
/// IMPOSIBLE #1: Integración Nativa con QuantumDao
pub struct ShuiDao {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    quantum_dao: Arc<QuantumDao>,  // Direct access
}

/// IMPOSIBLE #2: Project/Job/Task Hierarchy Operacional
pub struct Project {
    pub id: ProjectId,
    pub branch_id: BranchId,  // QuantumDao branch
    pub jobs: Vec<Job>,
    pub progress: f64,        // 0.0-1.0
}

pub struct Job {
    pub id: JobId,
    pub project_id: ProjectId,
    pub tasks: Vec<Task>,
    pub status: JobStatus,
}

pub struct Task {
    pub id: TaskId,
    pub job_id: JobId,
    pub core_id: CoreId,      // Link to TelescopeDB
    pub completed: bool,
}

/// IMPOSIBLE #3: Operational Entanglement Discovery
pub struct OperationalEntanglement {
    pub source: CoreId,
    pub target: CoreId,
    pub relation: OperationalRelation,
}

pub enum OperationalRelation {
    SameProject(BranchId),
    SameJob(JobId),
    SameTask(TaskId),
    Dependency,   // Task A depende de Task B
    Blocking,     // Task A bloquea Task B
}

/// IMPOSIBLE #4: Intention Detection Multi-Modal
pub struct IntentionDetector {
    pub modes: Vec<IntentionMode>,
    pub confidence_threshold: f64,
}

pub enum IntentionType {
    OperationalProject,     // "Tengo que hacer X"
    QuickQuestion,          // "¿Cómo funciona X?"
    ExploratoryLearning,    // "Quiero aprender sobre X"
    ReflectiveMusing,       // "Pensando en X..."
    EmotionalExpression,    // "Me siento X"
    TopicDiscussion,        // "Sobre X..."
}

/// IMPOSIBLE #5: Cognitive Mode Switching
pub struct CognitiveRouter {
    pub current_mode: CognitiveMode,
    pub mode_history: Vec<ModeTransition>,
}

pub enum CognitiveMode {
    Operational,   // Modo ejecución (Projects/Jobs/Tasks)
    Exploratory,   // Modo aprendizaje
    Reflective,    // Modo reflexión
    Social,        // Modo conversación
    Technical,     // Modo debugging/coding
}
```

---

## 🌳 INTEGRACIÓN CON QUANTUMDAO

### ShuiDao como Orquestador de QuantumDao

```rust
/// ShuiDao orquesta QuantumDao para gestión de proyectos
impl ShuiDao {
    /// Crear proyecto desde intención detectada
    pub async fn create_project_from_intention(
        &mut self,
        intention: &Intention,
    ) -> Result<Project> {
        // 1. Crear branch en QuantumDao
        let branch_name = self.generate_project_name(&intention.text);
        let branch_id = self.quantum_dao.create_project_branch(&branch_name)?;
        
        // 2. Analizar complejidad y descomponer
        let decomposition = self.decompose_project(&intention.text, branch_id).await?;
        
        // 3. Crear Project
        let project = Project {
            id: ProjectId::new(),
            branch_id,
            name: branch_name,
            jobs: decomposition.jobs,
            created_at: Utc::now(),
            progress: 0.0,
            metadata: ProjectMetadata {
                complexity: decomposition.complexity,
                estimated_duration: decomposition.estimated_duration,
                priority: intention.urgency,
            },
        };
        
        // 4. Persistir jobs y tasks en TelescopeDB
        for job in &project.jobs {
            for task in &job.tasks {
                let core = self.create_task_core(task, branch_id).await?;
                self.telescope_db.insert(core).await?;
                
                // Commit en QuantumDao
                self.quantum_dao.commit_to_project(
                    branch_id,
                    core.id,
                    &format!("Created task: {}", task.description),
                )?;
            }
        }
        
        // 5. Guardar project metadata
        self.save_project(&project).await?;
        
        Ok(project)
    }
    
    /// Actualizar progreso de proyecto
    pub async fn update_task_status(
        &mut self,
        task_id: TaskId,
        completed: bool,
    ) -> Result<()> {
        let task = self.get_task(&task_id).await?;
        let project = self.get_project_by_task(&task_id).await?;
        
        // 1. Update task
        self.tasks.get_mut(&task_id).unwrap().completed = completed;
        
        // 2. Commit en QuantumDao
        if completed {
            let core = self.telescope_db.get(&task.core_id).await?;
            self.quantum_dao.commit_to_project(
                project.branch_id,
                core.id,
                &format!("Completed task: {}", task.description),
            )?;
            
            // Update core metadata
            self.telescope_db.update(&task.core_id, |core| {
                core.quantum_meta.progress = 1.0;
            }).await?;
        }
        
        // 3. Recalcular progreso del proyecto
        let new_progress = self.calculate_project_progress(&project).await?;
        self.projects.get_mut(&project.id).unwrap().progress = new_progress;
        
        // 4. Si proyecto completado → merge to main
        if new_progress >= 1.0 {
            self.quantum_dao.merge_project_to_main(project.branch_id)?;
            log::info!("✅ Project completed and merged: {}", project.name);
        }
        
        Ok(())
    }
    
    /// Obtener progreso de proyecto
    pub async fn get_project_progress(&self, project_id: &ProjectId) -> Result<ProjectProgress> {
        let project = self.get_project(project_id).await?;
        
        let mut total_tasks = 0;
        let mut completed_tasks = 0;
        
        for job in &project.jobs {
            for task in &job.tasks {
                total_tasks += 1;
                if task.completed {
                    completed_tasks += 1;
                }
            }
        }
        
        Ok(ProjectProgress {
            project_id: *project_id,
            total_tasks,
            completed_tasks,
            percentage: if total_tasks > 0 {
                (completed_tasks as f64 / total_tasks as f64) * 100.0
            } else {
                0.0
            },
            estimated_completion: self.estimate_completion_date(&project),
        })
    }
}
```

---

## 📊 PROJECT/JOB/TASK HIERARCHY

### Jerarquía Operacional

```rust
/// Project = Objetivo de alto nivel (meses/semanas)
pub struct Project {
    pub id: ProjectId,
    pub branch_id: BranchId,  // QuantumDao branch
    pub name: String,
    pub description: String,
    pub jobs: Vec<Job>,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub progress: f64,  // 0.0-1.0
    pub metadata: ProjectMetadata,
}

pub struct ProjectMetadata {
    pub complexity: Complexity,
    pub estimated_duration: Duration,
    pub priority: Priority,
    pub tags: Vec<String>,
}

pub enum Complexity {
    Simple,      // 1-3 tasks
    Medium,      // 4-10 tasks
    Complex,     // 11-30 tasks
    Epic,        // 30+ tasks
}

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Job = Sub-objetivo dentro de proyecto (días/semanas)
pub struct Job {
    pub id: JobId,
    pub project_id: ProjectId,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Task>,
    pub status: JobStatus,
    pub dependencies: Vec<JobId>,  // Jobs que deben completarse antes
}

pub enum JobStatus {
    NotStarted,
    InProgress,
    Blocked,
    Completed,
}

/// Task = Acción concreta (horas/días)
pub struct Task {
    pub id: TaskId,
    pub job_id: JobId,
    pub core_id: CoreId,  // Link to TelescopeDB
    pub description: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Vec<String>,
}
```

### Descomposición Automática

```rust
impl ShuiDao {
    /// Descomponer proyecto en Jobs y Tasks
    pub async fn decompose_project(
        &self,
        project_description: &str,
        branch_id: BranchId,
    ) -> Result<ProjectDecomposition> {
        // 1. Analizar descripción con LLM
        let analysis = self.llm_service.analyze_project(project_description).await?;
        
        // 2. Extraer Jobs
        let jobs = analysis.phases.iter().map(|phase| {
            Job {
                id: JobId::new(),
                project_id: ProjectId::default(),  // Will be set
                name: phase.name.clone(),
                description: phase.description.clone(),
                tasks: phase.tasks.iter().map(|task| {
                    Task {
                        id: TaskId::new(),
                        job_id: JobId::default(),  // Will be set
                        core_id: CoreId::default(), // Will be created
                        description: task.clone(),
                        completed: false,
                        completed_at: None,
                        notes: Vec::new(),
                    }
                }).collect(),
                status: JobStatus::NotStarted,
                dependencies: Vec::new(),
            }
        }).collect();
        
        Ok(ProjectDecomposition {
            jobs,
            complexity: self.calculate_complexity(&analysis),
            estimated_duration: analysis.estimated_duration,
        })
    }
    
    /// Ejemplo de descomposición
    pub fn example_decomposition() -> ProjectDecomposition {
        ProjectDecomposition {
            jobs: vec![
                Job {
                    id: JobId::new(),
                    project_id: ProjectId::default(),
                    name: "Comprar hardware".into(),
                    description: "Adquirir switch y materiales".into(),
                    tasks: vec![
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Investigar modelos de switches".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Comparar precios".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Realizar compra".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                    ],
                    status: JobStatus::NotStarted,
                    dependencies: Vec::new(),
                },
                Job {
                    id: JobId::new(),
                    project_id: ProjectId::default(),
                    name: "Instalación física".into(),
                    description: "Instalar switch en oficina".into(),
                    tasks: vec![
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Planificar ubicación y cableado".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Instalar switch físicamente".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Conectar dispositivos".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                    ],
                    status: JobStatus::NotStarted,
                    dependencies: vec![JobId::default()],  // Depende de Job 1
                },
                Job {
                    id: JobId::new(),
                    project_id: ProjectId::default(),
                    name: "Configuración software".into(),
                    description: "Configurar switch y network".into(),
                    tasks: vec![
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Configurar VLANs".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                        Task {
                            id: TaskId::new(),
                            job_id: JobId::default(),
                            core_id: CoreId::default(),
                            description: "Testing de conectividad".into(),
                            completed: false,
                            completed_at: None,
                            notes: Vec::new(),
                        },
                    ],
                    status: JobStatus::NotStarted,
                    dependencies: vec![JobId::default()],  // Depende de Job 2
                },
            ],
            complexity: Complexity::Medium,
            estimated_duration: Duration::days(7),
        }
    }
}
```

---

## 🔗 OPERATIONAL ENTANGLEMENTS

### Auto-Discovery de Relaciones Operacionales

```rust
/// OperationalEntanglementDiscovery = Descubre relaciones entre cores del mismo proyecto
pub struct OperationalEntanglementDiscovery {
    telescope_db: Arc<TelescopeDB>,
    shuidao: Arc<ShuiDao>,
}

impl OperationalEntanglementDiscovery {
    /// Descubrir entanglements operacionales para un core
    pub async fn discover(&self, core_id: &CoreId) -> Result<Vec<Entanglement>> {
        let core = self.telescope_db.get(core_id).await?;
        
        // Si no está en branch, no hay operational entanglements
        if core.branch.is_none() {
            return Ok(Vec::new());
        }
        
        let branch_id = core.branch.unwrap();
        
        let mut entanglements = Vec::new();
        
        // 1. Buscar todos los cores del mismo branch
        let same_branch_cores = self.telescope_db
            .query_by_branch(branch_id)
            .await?;
        
        for candidate in same_branch_cores {
            if candidate.id == *core_id {
                continue;
            }
            
            // 2. Determinar tipo de relación operacional
            let relation = self.determine_operational_relation(&core, &candidate).await?;
            
            if let Some(rel_type) = relation {
                entanglements.push(Entanglement {
                    target_id: candidate.id,
                    ent_type: EntanglementType::Operational,
                    strength: 1.0,  // Máxima (mismo proyecto)
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        Ok(entanglements)
    }
    
    /// Determinar relación operacional específica
    async fn determine_operational_relation(
        &self,
        core_a: &QuantumCore,
        core_b: &QuantumCore,
    ) -> Result<Option<OperationalRelation>> {
        // Get tasks associated with cores
        let task_a = self.shuidao.get_task_by_core(&core_a.id).await?;
        let task_b = self.shuidao.get_task_by_core(&core_b.id).await?;
        
        if let (Some(task_a), Some(task_b)) = (task_a, task_b) {
            // Same task?
            if task_a.id == task_b.id {
                return Ok(Some(OperationalRelation::SameTask(task_a.id)));
            }
            
            // Same job?
            if task_a.job_id == task_b.job_id {
                return Ok(Some(OperationalRelation::SameJob(task_a.job_id)));
            }
            
            // Same project (different jobs)
            let job_a = self.shuidao.get_job(&task_a.job_id).await?;
            let job_b = self.shuidao.get_job(&task_b.job_id).await?;
            
            if job_a.project_id == job_b.project_id {
                return Ok(Some(OperationalRelation::SameProject(core_a.branch.unwrap())));
            }
        }
        
        Ok(None)
    }
}

pub enum OperationalRelation {
    SameProject(BranchId),
    SameJob(JobId),
    SameTask(TaskId),
    Dependency,
    Blocking,
}
```

---

## 🎯 INTENTION DETECTION

### Detector Multi-Modal de Intenciones

```rust
/// IntentionDetector = Comprende la intención del usuario
pub struct IntentionDetector {
    voxel_db: Arc<VoxelDB>,
    llm_service: Arc<LLMService>,
    embedding_service: Arc<EmbeddingService>,
}

impl IntentionDetector {
    /// Detectar intención desde input de usuario
    pub async fn detect(&self, user_input: &str) -> Result<Intention> {
        // 1. Quick patterns (reglas heurísticas)
        if let Some(intention) = self.detect_quick_patterns(user_input) {
            return Ok(intention);
        }
        
        // 2. Semantic analysis (embeddings + VoxelDB)
        let embedding = self.embedding_service.embed(user_input).await?;
        
        let similar_intentions = self.voxel_db.query(&VoxelQuery {
            embedding: embedding.clone(),
            k: 5,
            spherical_context: None,
            alpha_filter: Some(255),  // Solo native
            time_range: Some(Duration::days(90)),
            core_filter: None,
        }).await?;
        
        // 3. LLM analysis (si no hay match semántico claro)
        let llm_analysis = if similar_intentions.is_empty() {
            Some(self.llm_service.analyze_intention(user_input).await?)
        } else {
            None
        };
        
        // 4. Combinar evidencias
        let intention = self.synthesize_intention(
            user_input,
            &similar_intentions,
            llm_analysis,
        ).await?;
        
        Ok(intention)
    }
    
    /// Patterns rápidos (reglas heurísticas)
    fn detect_quick_patterns(&self, text: &str) -> Option<Intention> {
        let text_lower = text.to_lowercase();
        
        // Operational project keywords
        if text_lower.contains("tengo que") ||
           text_lower.contains("necesito hacer") ||
           text_lower.contains("debo instalar") ||
           text_lower.contains("proyecto:") {
            return Some(Intention {
                intention_type: IntentionType::OperationalProject,
                confidence: 0.85,
                text: text.to_string(),
                urgency: self.detect_urgency(&text_lower),
            });
        }
        
        // Quick question keywords
        if text_lower.starts_with("¿") ||
           text_lower.starts_with("cómo") ||
           text_lower.starts_with("qué es") ||
           text_lower.starts_with("cuál") {
            return Some(Intention {
                intention_type: IntentionType::QuickQuestion,
                confidence: 0.90,
                text: text.to_string(),
                urgency: Priority::Medium,
            });
        }
        
        // Exploratory learning keywords
        if text_lower.contains("quiero aprender") ||
           text_lower.contains("me interesa") ||
           text_lower.contains("explícame sobre") {
            return Some(Intention {
                intention_type: IntentionType::ExploratoryLearning,
                confidence: 0.80,
                text: text.to_string(),
                urgency: Priority::Low,
            });
        }
        
        // Reflective musing keywords
        if text_lower.contains("pensando en") ||
           text_lower.contains("reflexiono sobre") ||
           text_lower.contains("me pregunto") {
            return Some(Intention {
                intention_type: IntentionType::ReflectiveMusing,
                confidence: 0.75,
                text: text.to_string(),
                urgency: Priority::Low,
            });
        }
        
        // Emotional expression keywords
        if text_lower.contains("me siento") ||
           text_lower.contains("estoy frustrado") ||
           text_lower.contains("estoy feliz") {
            return Some(Intention {
                intention_type: IntentionType::EmotionalExpression,
                confidence: 0.85,
                text: text.to_string(),
                urgency: Priority::High,
            });
        }
        
        None
    }
    
    fn detect_urgency(&self, text: &str) -> Priority {
        if text.contains("urgente") || text.contains("ahora") || text.contains("ya") {
            Priority::Critical
        } else if text.contains("pronto") || text.contains("hoy") {
            Priority::High
        } else if text.contains("esta semana") || text.contains("próximamente") {
            Priority::Medium
        } else {
            Priority::Low
        }
    }
}

pub struct Intention {
    pub intention_type: IntentionType,
    pub confidence: f64,
    pub text: String,
    pub urgency: Priority,
}

pub enum IntentionType {
    OperationalProject,     // "Tengo que hacer X"
    QuickQuestion,          // "¿Cómo funciona X?"
    ExploratoryLearning,    // "Quiero aprender sobre X"
    ReflectiveMusing,       // "Pensando en X..."
    EmotionalExpression,    // "Me siento X"
    TopicDiscussion,        // "Sobre X..."
}
```

---

## 📈 TOPIC GRAPH (SUB-COMPONENTE 13a)

### Detección de Topics

```rust
/// TopicGraph = Detecta temas recurrentes en conversaciones
/// 
/// Ver especificación completa en:
/// ROADMAP_V2/02_COMPONENTES/13a_shuidao-topic-graph.md
pub struct TopicGraph {
    topics: HashMap<TopicId, Topic>,
    topic_embeddings: HashMap<TopicId, Vec<f32>>,
    voxel_db: Arc<VoxelDB>,
}

pub struct Topic {
    pub id: TopicId,
    pub name: String,
    pub keywords: Vec<String>,
    pub frequency: usize,
    pub last_discussed: DateTime<Utc>,
    pub cores: Vec<CoreId>,  // Cores que discuten este topic
}

impl TopicGraph {
    /// Detectar topic desde input
    pub async fn detect_topic(&self, user_input: &str) -> Result<Option<TopicId>> {
        // 1. Generar embedding
        let embedding = self.embedding_service.embed(user_input).await?;
        
        // 2. Buscar topics similares en VoxelDB
        let similar_topics = self.voxel_db.query(&VoxelQuery {
            embedding: embedding.clone(),
            k: 3,
            spherical_context: None,
            alpha_filter: Some(255),
            time_range: None,
            core_filter: None,
        }).await?;
        
        // 3. Match si similitud > 0.85
        if let Some(voxel) = similar_topics.first() {
            let similarity = cosine_similarity(&embedding, &voxel.embedding);
            if similarity > 0.85 {
                return Ok(Some(self.voxel_to_topic_id(voxel)));
            }
        }
        
        Ok(None)
    }
    
    /// Crear nuevo topic
    pub async fn create_topic(
        &mut self,
        name: &str,
        keywords: Vec<String>,
        core_id: CoreId,
    ) -> Result<TopicId> {
        let topic_id = TopicId::new();
        
        let topic = Topic {
            id: topic_id,
            name: name.to_string(),
            keywords,
            frequency: 1,
            last_discussed: Utc::now(),
            cores: vec![core_id],
        };
        
        self.topics.insert(topic_id, topic);
        
        Ok(topic_id)
    }
}
```

---

## 😊 EMOTIONAL SPACE (SUB-COMPONENTE 13b)

### Detección de Estado Emocional

```rust
/// EmotionalSpace = Detecta y trackea estado emocional del usuario
/// 
/// Ver especificación completa en:
/// ROADMAP_V2/02_COMPONENTES/13b_shuidao-emotional-space.md
pub struct EmotionalSpace {
    current_state: EmotionalState,
    history: Vec<EmotionalTransition>,
}

pub struct EmotionalState {
    pub valence: f64,      // -1.0 (negativo) a 1.0 (positivo)
    pub arousal: f64,      // 0.0 (calmado) a 1.0 (excitado)
    pub dominance: f64,    // 0.0 (controlado) a 1.0 (en control)
    pub frustration: f64,  // 0.0 (sin frustración) a 1.0 (muy frustrado)
}

impl EmotionalSpace {
    /// Detectar emoción desde input
    pub async fn detect_emotion(&self, user_input: &str) -> Result<EmotionalState> {
        // 1. Análisis de keywords emocionales
        let emotion_score = self.analyze_emotional_keywords(user_input);
        
        // 2. Análisis de tono con LLM (si disponible)
        let llm_emotion = self.llm_service
            .analyze_emotion(user_input)
            .await
            .ok();
        
        // 3. Combinar evidencias
        let state = EmotionalState {
            valence: emotion_score.valence,
            arousal: emotion_score.arousal,
            dominance: emotion_score.dominance,
            frustration: emotion_score.frustration,
        };
        
        Ok(state)
    }
    
    fn analyze_emotional_keywords(&self, text: &str) -> EmotionalState {
        let text_lower = text.to_lowercase();
        
        let mut valence = 0.0;
        let mut arousal = 0.5;
        let mut dominance = 0.5;
        let mut frustration = 0.0;
        
        // Positive keywords
        if text_lower.contains("feliz") || text_lower.contains("genial") {
            valence += 0.5;
        }
        
        // Negative keywords
        if text_lower.contains("frustrado") || text_lower.contains("molesto") {
            valence -= 0.5;
            frustration += 0.6;
            arousal += 0.3;
        }
        
        // Calm keywords
        if text_lower.contains("tranquilo") || text_lower.contains("relajado") {
            arousal -= 0.3;
        }
        
        // Control keywords
        if text_lower.contains("no puedo") || text_lower.contains("no sé") {
            dominance -= 0.4;
            frustration += 0.3;
        }
        
        EmotionalState {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
            frustration: frustration.clamp(0.0, 1.0),
        }
    }
}
```

---

## 🧭 COGNITIVE MODES

### Router de Modos Cognitivos

```rust
/// CognitiveRouter = Selecciona modo cognitivo apropiado
pub struct CognitiveRouter {
    current_mode: CognitiveMode,
    mode_history: Vec<ModeTransition>,
}

pub enum CognitiveMode {
    Operational,   // Ejecutando proyectos/tareas
    Exploratory,   // Aprendiendo/explorando
    Reflective,    // Reflexionando/pensando
    Social,        // Conversación casual
    Technical,     // Debugging/coding intenso
    Creative,      // Brainstorming/ideación
}

impl CognitiveRouter {
    /// Seleccionar modo cognitivo desde intención
    pub fn select_mode(&mut self, intention: &Intention) -> CognitiveMode {
        let mode = match intention.intention_type {
            IntentionType::OperationalProject => CognitiveMode::Operational,
            IntentionType::QuickQuestion => CognitiveMode::Technical,
            IntentionType::ExploratoryLearning => CognitiveMode::Exploratory,
            IntentionType::ReflectiveMusing => CognitiveMode::Reflective,
            IntentionType::EmotionalExpression => CognitiveMode::Social,
            IntentionType::TopicDiscussion => CognitiveMode::Exploratory,
        };
        
        // Registrar transición
        if mode != self.current_mode {
            self.mode_history.push(ModeTransition {
                from: self.current_mode,
                to: mode,
                timestamp: Utc::now(),
                reason: intention.intention_type,
            });
            
            self.current_mode = mode;
        }
        
        mode
    }
    
    /// Obtener strategy de respuesta para modo actual
    pub fn get_response_strategy(&self) -> ResponseStrategy {
        match self.current_mode {
            CognitiveMode::Operational => ResponseStrategy {
                style: ResponseStyle::ActionOriented,
                detail_level: DetailLevel::Structured,
                include_next_steps: true,
                track_progress: true,
            },
            CognitiveMode::Exploratory => ResponseStrategy {
                style: ResponseStyle::Educational,
                detail_level: DetailLevel::Comprehensive,
                include_next_steps: false,
                track_progress: false,
            },
            CognitiveMode::Reflective => ResponseStrategy {
                style: ResponseStyle::Conversational,
                detail_level: DetailLevel::Minimal,
                include_next_steps: false,
                track_progress: false,
            },
            CognitiveMode::Technical => ResponseStrategy {
                style: ResponseStyle::Precise,
                detail_level: DetailLevel::Technical,
                include_next_steps: true,
                track_progress: false,
            },
            CognitiveMode::Social => ResponseStrategy {
                style: ResponseStyle::Empathetic,
                detail_level: DetailLevel::Minimal,
                include_next_steps: false,
                track_progress: false,
            },
            CognitiveMode::Creative => ResponseStrategy {
                style: ResponseStyle::Inspirational,
                detail_level: DetailLevel::HighLevel,
                include_next_steps: false,
                track_progress: false,
            },
        }
    }
}

pub struct ResponseStrategy {
    pub style: ResponseStyle,
    pub detail_level: DetailLevel,
    pub include_next_steps: bool,
    pub track_progress: bool,
}

pub enum ResponseStyle {
    ActionOriented,
    Educational,
    Conversational,
    Precise,
    Empathetic,
    Inspirational,
}

pub enum DetailLevel {
    Minimal,
    HighLevel,
    Structured,
    Comprehensive,
    Technical,
}
```

---

## 🔌 API PRINCIPAL

### Operaciones Core

```rust
impl ShuiDao {
    /// Procesar input de usuario (entrada principal)
    pub async fn process_input(&mut self, user_input: &str) -> Result<ShuiDaoResponse> {
        // 1. Detect intention
        let intention = self.intention_detector.detect(user_input).await?;
        
        // 2. Select cognitive mode
        let mode = self.cognitive_router.select_mode(&intention);
        
        // 3. Route según intention type
        let response = match intention.intention_type {
            IntentionType::OperationalProject => {
                self.handle_operational_project(&intention).await?
            }
            IntentionType::QuickQuestion => {
                self.handle_quick_question(&intention).await?
            }
            IntentionType::ExploratoryLearning => {
                self.handle_exploratory_learning(&intention).await?
            }
            IntentionType::ReflectiveMusing => {
                self.handle_reflective_musing(&intention).await?
            }
            IntentionType::EmotionalExpression => {
                self.handle_emotional_expression(&intention).await?
            }
            IntentionType::TopicDiscussion => {
                self.handle_topic_discussion(&intention).await?
            }
        };
        
        // 4. Detect topic
        if let Some(topic_id) = self.topic_graph.detect_topic(user_input).await? {
            response.topic = Some(topic_id);
        }
        
        // 5. Detect emotion
        let emotional_state = self.emotional_space.detect_emotion(user_input).await?;
        response.emotional_state = Some(emotional_state);
        
        Ok(response)
    }
    
    /// Handler para operational project
    async fn handle_operational_project(&mut self, intention: &Intention) -> Result<ShuiDaoResponse> {
        // 1. Crear proyecto
        let project = self.create_project_from_intention(intention).await?;
        
        // 2. Generar respuesta
        let response = ShuiDaoResponse {
            mode: CognitiveMode::Operational,
            message: format!(
                "Entiendo que necesitas: {}\n\nHe creado un proyecto con {} jobs y {} tasks en total.\n\n{}",
                project.name,
                project.jobs.len(),
                project.jobs.iter().map(|j| j.tasks.len()).sum::<usize>(),
                self.format_project_summary(&project),
            ),
            project: Some(project),
            topic: None,
            emotional_state: None,
        };
        
        Ok(response)
    }
    
    /// Obtener proyectos activos
    pub async fn get_active_projects(&self) -> Result<Vec<Project>> {
        let active: Vec<_> = self.projects
            .values()
            .filter(|p| p.progress < 1.0)
            .cloned()
            .collect();
        
        Ok(active)
    }
    
    /// Obtener proyecto por ID
    pub async fn get_project(&self, project_id: &ProjectId) -> Result<Project> {
        self.projects
            .get(project_id)
            .cloned()
            .ok_or(ShuiDaoError::ProjectNotFound)
    }
}

pub struct ShuiDaoResponse {
    pub mode: CognitiveMode,
    pub message: String,
    pub project: Option<Project>,
    pub topic: Option<TopicId>,
    pub emotional_state: Option<EmotionalState>,
}
```

---

## 🎯 CASOS DE USO

### Caso 1: Crear Proyecto Operacional

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut shuidao = ShuiDao::new("./data")?;
    
    // Usuario: "Tengo que instalar un switch en la oficina"
    let response = shuidao.process_input(
        "Tengo que instalar un switch en la oficina"
    ).await?;
    
    println!("Modo: {:?}", response.mode);
    // → Operational
    
    println!("Respuesta: {}", response.message);
    // → "Entiendo que necesitas: Instalar switch oficina
    //    He creado un proyecto con 3 jobs y 8 tasks en total..."
    
    if let Some(project) = response.project {
        println!("Proyecto creado: {}", project.name);
        println!("Branch QuantumDao: {:?}", project.branch_id);
        println!("Progreso: {:.0}%", project.progress * 100.0);
    }
    
    Ok(())
}
```

### Caso 2: Actualizar Progreso

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut shuidao = ShuiDao::new("./data")?;
    
    // Obtener proyecto activo
    let projects = shuidao.get_active_projects().await?;
    let project = projects.first().unwrap();
    
    // Marcar primera task como completada
    let first_task_id = project.jobs[0].tasks[0].id;
    shuidao.update_task_status(first_task_id, true).await?;
    
    // Ver progreso actualizado
    let progress = shuidao.get_project_progress(&project.id).await?;
    println!("Progreso: {}/{} tasks ({:.1}%)",
        progress.completed_tasks,
        progress.total_tasks,
        progress.percentage,
    );
    // → "Progreso: 1/8 tasks (12.5%)"
    
    Ok(())
}
```

### Caso 3: Detección de Emotional State

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut shuidao = ShuiDao::new("./data")?;
    
    // Usuario frustrado
    let response = shuidao.process_input(
        "Estoy frustrado, no puedo hacer que el switch funcione"
    ).await?;
    
    if let Some(emotion) = response.emotional_state {
        println!("Estado emocional detectado:");
        println!("  Valence: {:.2} (negativo)", emotion.valence);
        println!("  Arousal: {:.2}", emotion.arousal);
        println!("  Dominance: {:.2} (baja)", emotion.dominance);
        println!("  Frustration: {:.2} (alta)", emotion.frustration);
        
        // Adaptar respuesta según emoción
        if emotion.frustration > 0.6 {
            println!("\n💡 Respuesta empática activada");
        }
    }
    
    Ok(())
}
```

---

## ⚡ PERFORMANCE TARGETS

### Objetivos v1.5

| Operación | Target | Justificación |
|-----------|--------|---------------|
| **process_input()** | <100ms | Intention detection + routing |
| **detect_intention()** | <50ms | Pattern matching + embeddings |
| **create_project()** | <200ms | Decomposition + QuantumDao setup |
| **update_task_status()** | <20ms | Update task + recalc progress |
| **get_project_progress()** | <10ms | Simple calculation |
| **detect_topic()** | <50ms | VoxelDB query |
| **detect_emotion()** | <30ms | Keyword analysis |

---

## 🚀 PRÓXIMOS PASOS

### Implementación v1.5 (Prioridad ALTA)

1. ✅ **Especificación completa** (este documento)
2. 🔄 **Implementar IntentionDetector** (src/shuidao/intention/)
3. 🔄 **Implementar Project/Job/Task hierarchy** (src/shuidao/project/)
4. 🔄 **Implementar OperationalEntanglementDiscovery** (src/shuidao/entanglement/)
5. 🔄 **Implementar CognitiveRouter** (src/shuidao/router/)
6. 🔄 **Integración con QuantumDao** (src/dao/)
7. 🔄 **Tests end-to-end** (examples/test_shuidao.rs)

### Integración con Ecosistema

- **QuantumDao** ↔ ShuiDao (project branches management)
- **TelescopeDB** → ShuiDao (biographical context para projects)
- **VoxelDB** → ShuiDao (templates + topic detection)
- **TopicGraph** (13a) → ShuiDao (topic tracking)
- **EmotionalSpace** (13b) → ShuiDao (emotional awareness)

---

**Estado:** 📋 ESPECIFICACIÓN v1.5 COMPLETA  
**Complejidad:** ⚠️ ALTA - Motor cognitivo central  
**Prioridad:** 🟡 ALTA - Diferenciador clave del sistema

---

*Generado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Pixel-Native Revolution*  
*"No es un intent detector. Es un compañero que comprende tu realidad operacional."* 🧠✨
