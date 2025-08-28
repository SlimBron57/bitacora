# 🧵 Threading & Safety Mechanisms - Arquitectura Técnica

## 📋 **OVERVIEW**

Este documento detalla la implementación técnica de los mecanismos de threading y safety para el Sistema Híbrido de Navegación, basado en la propuesta de niveles de threading del usuario.

---

## 🏗️ **ARQUITECTURA DE THREADING**

### **Thread Isolation Strategy**

```rust
// Arquitectura principal de threading
pub struct ThreadManager {
    spark_pool: Arc<ThreadPool>,           // Nivel 0: Múltiples sparks
    project_pool: Arc<ThreadPool>,         // Nivel 1: Múltiples proyectos  
    topic_executor: Arc<Mutex<Executor>>,  // Nivel 2: Un proceso único
    action_executor: Arc<Mutex<Executor>>, // Nivel 3: Un proceso único
    safety_controller: Arc<SafetyController>,
}

// Thread safety para diferentes niveles
pub enum ThreadLevel {
    Spark(SparkThreadConfig),      // Level 0: Full threading
    Project(ProjectThreadConfig),  // Level 1: Project isolation
    Topic(TopicSerialConfig),      // Level 2: Serial execution
    Action(ActionSerialConfig),    // Level 3: Serial execution
}
```

### **Nivel 0: Spark Threading** ✅ FULL CONCURRENCY

```rust
pub struct SparkThreadConfig {
    max_concurrent_sparks: usize,
    timeout_seconds: u64,
    memory_limit_mb: usize,
}

impl SparkProcessor {
    pub async fn process_concurrent_sparks(
        &self, 
        sparks: Vec<Spark>
    ) -> Result<Vec<SparkResult>, SparkError> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent_sparks));
        
        let futures = sparks.into_iter().map(|spark| {
            let sem = semaphore.clone();
            let processor = self.clone();
            
            async move {
                let _permit = sem.acquire().await.unwrap();
                processor.process_single_spark(spark).await
            }
        });
        
        futures::future::try_join_all(futures).await
    }
}
```

**Justificación:** Sparks son completamente independientes, no hay riesgo de conflictos.

### **Nivel 1: Project Threading** ✅ PROJECT ISOLATION

```rust
pub struct ProjectThreadConfig {
    max_concurrent_projects: usize,
    workspace_isolation: bool,
    resource_limits: ResourceLimits,
}

impl ProjectManager {
    pub async fn execute_project_tasks(
        &self,
        projects: Vec<ProjectTask>
    ) -> Result<Vec<ProjectResult>, ProjectError> {
        // Cada proyecto ejecuta en workspace aislado
        let project_futures = projects.into_iter().map(|project| {
            let manager = self.clone();
            tokio::spawn(async move {
                manager.execute_isolated_project(project).await
            })
        });
        
        // Ejecutar todos los proyectos concurrentemente
        futures::future::try_join_all(project_futures).await
    }
}
```

**Justificación:** Proyectos diferentes son independientes, workspace isolation previene conflictos.

### **Nivel 2: Topic Serial Execution** ❌ NO THREADING

```rust
pub struct TopicSerialConfig {
    continuation_threshold: RiskLevel,
    user_consultation_timeout: Duration,
    rollback_strategy: RollbackStrategy,
}

impl TopicExecutor {
    pub async fn execute_topic_sequence(
        &self,
        topics: Vec<Topic>
    ) -> Result<Vec<TopicResult>, TopicError> {
        let mut results = Vec::new();
        
        for topic in topics {
            // Ejecutar un topic a la vez
            let result = self.execute_single_topic(topic).await?;
            
            // Evaluar si puede continuar automáticamente
            if self.requires_user_consultation(&result) {
                // Pausar y consultar usuario
                let user_decision = self.consult_user(&result).await?;
                if !user_decision.continue_execution {
                    break;
                }
            }
            
            results.push(result);
        }
        
        Ok(results)
    }
    
    fn requires_user_consultation(&self, result: &TopicResult) -> bool {
        match result.risk_level {
            RiskLevel::High => true,
            RiskLevel::Medium => !self.config.auto_continue_medium_risk,
            RiskLevel::Low => false,
        }
    }
}
```

**Justificación:** Topics pueden tener dependencias dentro del mismo proyecto.

### **Nivel 3: Action Serial Execution** ❌ NO THREADING

```rust
pub struct ActionSerialConfig {
    state_validation: bool,
    atomic_operations: bool,
    conflict_detection: bool,
}

impl ActionExecutor {
    pub async fn execute_action_sequence(
        &self,
        actions: Vec<Action>
    ) -> Result<Vec<ActionResult>, ActionError> {
        let mut results = Vec::new();
        let mut current_state = self.capture_initial_state().await?;
        
        for action in actions {
            // Validar estado antes de ejecutar
            self.validate_state_consistency(&current_state, &action)?;
            
            // Ejecutar acción atómicamente
            let result = self.execute_atomic_action(action, &current_state).await?;
            
            // Actualizar estado conocido
            current_state = self.update_state(current_state, &result)?;
            
            // Verificar si necesita consulta de usuario
            if self.requires_user_intervention(&result) {
                let decision = self.consult_user_for_action(&result).await?;
                if !decision.continue_sequence {
                    break;
                }
            }
            
            results.push(result);
        }
        
        Ok(results)
    }
}
```

**Justificación:** Actions pueden modificar estado compartido, requieren ejecución serial.

---

## 🛡️ **SAFETY MECHANISMS**

### **Safety Controller Architecture**

```rust
pub struct SafetyController {
    lock_manager: LockManager,
    conflict_detector: ConflictDetector,
    rollback_manager: RollbackManager,
    risk_assessor: RiskAssessor,
}

impl SafetyController {
    pub async fn acquire_execution_lock(
        &self,
        level: ThreadLevel,
        resources: Vec<ResourceId>
    ) -> Result<ExecutionLock, LockError> {
        match level {
            ThreadLevel::Spark(_) => {
                // Sparks no requieren locks, son independientes
                Ok(ExecutionLock::None)
            },
            ThreadLevel::Project(_) => {
                // Project-level locking
                self.lock_manager.acquire_project_locks(resources).await
            },
            ThreadLevel::Topic(_) => {
                // Topic serialization lock
                self.lock_manager.acquire_topic_lock().await
            },
            ThreadLevel::Action(_) => {
                // Action serialization + state lock
                self.lock_manager.acquire_action_lock().await
            }
        }
    }
}
```

### **Lock Strategy por Nivel**

#### **Nivel 0 (Sparks): No Locks** 
```rust
// Sin locks - independiente total
pub struct SparkExecution {
    // No shared state, no locks needed
}
```

#### **Nivel 1 (Projects): Project Isolation Locks**
```rust
pub struct ProjectLock {
    project_id: ProjectId,
    workspace_path: PathBuf,
    resource_locks: Vec<ResourceLock>,
}
```

#### **Nivel 2 (Topics): Serial Execution Lock**
```rust
pub struct TopicLock {
    global_topic_mutex: Arc<Mutex<()>>,
    current_topic_id: Option<TopicId>,
}
```

#### **Nivel 3 (Actions): State Consistency Lock**
```rust
pub struct ActionLock {
    global_action_mutex: Arc<Mutex<()>>,
    state_lock: Arc<RwLock<SystemState>>,
}
```

---

## 🔄 **CONFLICT DETECTION & RESOLUTION**

### **Conflict Detection Matrix**

```rust
pub struct ConflictDetector {
    resource_map: Arc<RwLock<HashMap<ResourceId, ThreadLevel>>>,
    dependency_graph: Arc<RwLock<DependencyGraph>>,
}

impl ConflictDetector {
    pub fn detect_potential_conflicts(
        &self,
        level: ThreadLevel,
        resources: &[ResourceId]
    ) -> Vec<ConflictRisk> {
        match level {
            ThreadLevel::Spark(_) => {
                // Sparks no generan conflictos
                vec![]
            },
            ThreadLevel::Project(_) => {
                // Verificar conflictos de workspace
                self.check_workspace_conflicts(resources)
            },
            ThreadLevel::Topic(_) => {
                // Verificar dependencias entre topics  
                self.check_topic_dependencies(resources)
            },
            ThreadLevel::Action(_) => {
                // Verificar conflictos de estado
                self.check_state_conflicts(resources)
            }
        }
    }
}
```

### **Rollback Strategies**

```rust
pub enum RollbackStrategy {
    None,                           // Para Sparks
    WorkspaceRevert,               // Para Projects  
    TopicSequenceUndo,            // Para Topics
    AtomicActionRollback,         // Para Actions
}

pub struct RollbackManager {
    snapshots: HashMap<ExecutionId, SystemSnapshot>,
    rollback_strategies: HashMap<ThreadLevel, RollbackStrategy>,
}

impl RollbackManager {
    pub async fn rollback_execution(
        &self,
        execution_id: ExecutionId,
        level: ThreadLevel
    ) -> Result<(), RollbackError> {
        let strategy = self.rollback_strategies.get(&level)
            .ok_or(RollbackError::NoStrategy)?;
            
        match strategy {
            RollbackStrategy::None => Ok(()),
            RollbackStrategy::WorkspaceRevert => {
                self.revert_workspace_changes(execution_id).await
            },
            RollbackStrategy::TopicSequenceUndo => {
                self.undo_topic_sequence(execution_id).await  
            },
            RollbackStrategy::AtomicActionRollback => {
                self.rollback_atomic_actions(execution_id).await
            }
        }
    }
}
```

---

## ⚡ **PERFORMANCE OPTIMIZATIONS**

### **Thread Pool Configuration**

```rust
pub struct ThreadPoolConfig {
    // Spark pool: CPU intensive, muchos threads
    spark_pool_size: usize,        // Default: CPU cores * 2
    spark_queue_size: usize,       // Default: 1000
    
    // Project pool: I/O intensive, threads moderados
    project_pool_size: usize,      // Default: CPU cores
    project_queue_size: usize,     // Default: 100
}

impl ThreadPoolConfig {
    pub fn optimize_for_system() -> Self {
        let cpu_count = num_cpus::get();
        
        Self {
            spark_pool_size: cpu_count * 2,    // CPU bound tasks
            spark_queue_size: 1000,
            project_pool_size: cpu_count,      // Balanced I/O + CPU
            project_queue_size: 100,
        }
    }
}
```

### **Resource Management**

```rust
pub struct ResourceManager {
    memory_monitor: MemoryMonitor,
    cpu_monitor: CpuMonitor,
    disk_monitor: DiskMonitor,
}

impl ResourceManager {
    pub fn should_throttle_execution(&self, level: ThreadLevel) -> bool {
        match level {
            ThreadLevel::Spark(_) => {
                // Throttle sparks si CPU > 90%
                self.cpu_monitor.usage_percent() > 90.0
            },
            ThreadLevel::Project(_) => {
                // Throttle projects si memoria > 80%
                self.memory_monitor.usage_percent() > 80.0
            },
            ThreadLevel::Topic(_) | ThreadLevel::Action(_) => {
                // Serial execution, no throttling
                false
            }
        }
    }
}
```

---

## 🧪 **TESTING STRATEGY**

### **Unit Tests per Level**

```rust
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_spark_concurrent_execution() {
        // Test múltiples sparks simultáneos
        let sparks = generate_test_sparks(10);
        let results = spark_processor.process_concurrent_sparks(sparks).await;
        assert_eq!(results.len(), 10);
        assert!(all_completed_successfully(&results));
    }
    
    #[tokio::test]  
    async fn test_project_isolation() {
        // Test que proyectos no interfieren entre sí
        let project_a = create_test_project("project_a");
        let project_b = create_test_project("project_b"); 
        
        let (result_a, result_b) = join!(
            project_manager.execute_project(project_a),
            project_manager.execute_project(project_b)
        );
        
        assert!(result_a.is_ok());
        assert!(result_b.is_ok());
        assert_no_resource_conflicts(&result_a, &result_b);
    }
    
    #[tokio::test]
    async fn test_topic_serial_execution() {
        // Test que topics ejecutan secuencialmente
        let topics = generate_dependent_topics();
        let start_time = Instant::now();
        
        let results = topic_executor.execute_topic_sequence(topics).await;
        
        // Verificar ejecución serial (no overlapping)
        assert!(results.len() > 0);
        verify_serial_execution_order(&results);
    }
}
```

---

## 📊 **MONITORING & OBSERVABILITY** 

### **Thread Monitoring**

```rust
pub struct ThreadMonitor {
    active_sparks: Arc<AtomicUsize>,
    active_projects: Arc<AtomicUsize>,
    topic_execution_time: Arc<Mutex<Duration>>,
    action_execution_time: Arc<Mutex<Duration>>,
}

impl ThreadMonitor {
    pub fn report_system_status(&self) -> SystemThreadStatus {
        SystemThreadStatus {
            concurrent_sparks: self.active_sparks.load(Ordering::Relaxed),
            concurrent_projects: self.active_projects.load(Ordering::Relaxed),
            topic_execution_active: self.topic_execution_time.is_locked(),
            action_execution_active: self.action_execution_time.is_locked(),
            timestamp: Utc::now(),
        }
    }
}
```

---

## 🎯 **IMPLEMENTACIÓN INCREMENTAL**

### **Phase 1: Basic Threading (Sparks + Projects)**
```rust
// Implementar solo Nivel 0 y 1
- SparkThreadPool básico
- ProjectIsolation básico  
- Safety mechanisms mínimos
```

### **Phase 2: Serial Execution (Topics + Actions)**  
```rust
// Agregar Nivel 2 y 3
- TopicExecutor con locks
- ActionExecutor con state management
- Conflict detection básico
```

### **Phase 3: Advanced Safety**
```rust
// Safety mechanisms completos
- RollbackManager completo
- ConflictDetector avanzado
- Resource monitoring
```

### **Phase 4: Optimization**
```rust
// Performance optimizations
- Thread pool tuning
- Memory management
- Monitoring dashboard
```

---

## 📝 **CONFIGURACIÓN DE EJEMPLO**

```toml
# config/threading.toml
[threading]
enabled = true
strategy = "hybrid" # core | threads | hybrid

[threading.spark_pool]
size = 8
queue_size = 1000
timeout_seconds = 300

[threading.project_pool]  
size = 4
queue_size = 100
isolation_level = "workspace"

[threading.serial_execution]
topic_consultation_required = true
action_atomic_execution = true
rollback_enabled = true

[safety]
risk_assessment = true
conflict_detection = true  
automatic_rollback = true
```

---

## 🚀 **PRÓXIMOS PASOS**

1. **Implementar Thread Pools básicos** (Spark + Project levels)
2. **Diseñar Serial Executors** (Topic + Action levels)  
3. **Desarrollar Safety Controller** (Locks, Conflicts, Rollbacks)
4. **Testing exhaustivo** por cada nivel
5. **Integration testing** del sistema completo
6. **Performance benchmarking** y optimization

---

*Threading & Safety Architecture - Bitacora V1.0 Hybrid Navigator*  
*Documentado el 27 de Agosto, 2025*
