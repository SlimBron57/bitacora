# Code & Concepts Documentation

## 🧠 **CONCEPTOS TÉCNICOS IMPORTANTES**

### **1. Arquitectura de Dominio Orientada a Eventos (Event-Driven Domain Architecture)**

**Concepto**: Cada acción en Bitácora genera eventos que son rastreables, contextualizables y reportables.

**Implementación**:
```rust
// Action como evento central del sistema
pub struct Action {
    pub action_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action_type: ActionType,
    pub context: ActionContext,     // Git context, archivos, etc.
    pub template_metadata: Option<TemplateMetadata>, // Response templates
}
```

**Por qué es importante**: Permite trazabilidad completa de actividades de desarrollo, análisis retrospectivo y automatización de procesos.

---

### **2. Sistema de Templates Dinámicos con Detección Automática**

**Concepto**: Templates de respuesta almacenados en base de datos que se auto-detectan basándose en el contexto de la acción.

**Implementación**:
```rust
// Template con estructura JSON flexible
pub struct Template {
    pub template_id: String,
    pub triggers: Vec<String>,        // Auto-detection keywords
    pub structure: serde_json::Value, // JSON dinámico
    pub engine_type: TemplateEngineType,
}

// Detección automática basada en contexto
async fn detect_template_for_action(&self, action: &Action) -> TemplateResult<Option<String>>;
```

**Por qué es importante**: Elimina código hardcodeado de respuestas, permite personalización por usuario/proyecto y facilita mantenimiento.

---

### **3. Patrón Repository con Async Traits**

**Concepto**: Abstracción de persistencia usando traits async para flexibilidad de storage (MongoDB, PostgreSQL, etc.).

**Implementación**:
```rust
#[async_trait]
pub trait TemplateService: Send + Sync {
    async fn get_template(&self, template_id: &str) -> TemplateResult<Template>;
    async fn detect_template_for_action(&self, action: &Action) -> TemplateResult<Option<String>>;
    async fn render_with_action(&self, action: &Action) -> TemplateResult<String>;
}
```

**Por qué es importante**: Facilita testing con mocks, permite cambiar proveedores de datos sin afectar lógica de negocio.

---

### **4. Context-Rich Action Modeling**

**Concepto**: Las acciones no son solo texto, contienen contexto rico (git, archivos, duración, tags, etc.).

**Implementación**:
```rust
pub struct ActionContext {
    pub git_branch: Option<String>,
    pub git_commit_hash: Option<String>,
    pub files_affected: Vec<String>,
    pub additional_info: HashMap<String, String>,
}
```

**Por qué es importante**: Permite análisis detallado de productividad, automatización de CI/CD y reporting inteligente.

---

### **5. Template Engine Registry Pattern**

**Concepto**: Sistema pluggable de motores de templates (Handlebars, Tera, etc.) con selección automática.

**Implementación**:
```rust
pub struct TemplateRegistry {
    engines: Vec<Box<dyn TemplateEngine + Send + Sync>>,
}

impl TemplateRegistry {
    pub fn get_engine_for_template(&self, template: &Template) -> Option<&(dyn TemplateEngine + Send + Sync)>;
}
```

**Por qué es importante**: Extensibilidad sin modificar código core, soporte para múltiples formatos de template.

---

## 📚 **DOCUMENTACIÓN TÉCNICA REQUERIDA**

### **Immediate Documentation Needs**:

1. **API Specification** 📋
   - REST endpoints design
   - Request/Response schemas
   - Authentication flow
   - Error handling patterns

2. **Database Schema Design** 🗄️
   - MongoDB collection structure
   - Indexing strategy
   - Migration procedures
   - Data relationships

3. **Template System Guide** 🎨
   - Template creation guidelines
   - Variable injection patterns
   - Engine selection criteria
   - Custom template development

4. **Service Architecture Patterns** ⚙️
   - Dependency injection setup
   - Error propagation strategies
   - Async service composition
   - Testing patterns for services

5. **Deployment & Operations** 🚀
   - Docker containerization
   - Environment configuration
   - Monitoring and logging
   - Backup procedures

---

## 🔧 **IMPLEMENTATION PATTERNS ESTABLISHED**

### **Domain Model Pattern**:
- Rich domain objects with behavior
- Value objects for complex types
- Aggregate roots for consistency

### **Service Layer Pattern**:
- Async trait-based services
- Dependency injection ready
- Comprehensive error handling

### **Template System Pattern**:
- Dynamic template loading
- Context-aware rendering
- Multi-engine support

### **Repository Pattern**:
- Abstract data access
- Async/await throughout
- Type-safe operations

---

## 📈 **ARCHITECTURAL DECISIONS RECORD (ADR)**

### **ADR-001: Rust + Axum + MongoDB**
**Decision**: Use Rust with Axum web framework and MongoDB for persistence
**Rationale**: Performance, type safety, modern async patterns, document-based storage for flexible schemas
**Status**: Implemented

### **ADR-002: Domain-First Design**
**Decision**: Start with rich domain models before infrastructure
**Rationale**: Ensures business logic is not coupled to persistence or presentation layers
**Status**: Implemented

### **ADR-003: Dynamic Template System**
**Decision**: Database-stored templates with runtime detection
**Rationale**: Eliminates hardcoded responses, enables customization, supports multiple output formats
**Status**: 95% Implemented

### **ADR-004: Event-Driven Architecture**
**Decision**: Model all activities as events (Actions) with rich context
**Rationale**: Enables powerful analytics, audit trails, and automation triggers
**Status**: Implemented

### **ADR-005: Async-First Service Layer**
**Decision**: All services use async traits for I/O operations
**Rationale**: Better resource utilization, non-blocking operations, scalability
**Status**: In Progress

### **ADR-006: Git Service Integration Architecture**
**Decision**: Comprehensive async Git service with intelligent automation
**Rationale**: Eliminates manual Git overhead, ensures consistent workflows, provides rich context for commits
**Status**: ✅ Implemented (August 22, 2025)

---

### **4. Git Service: Async Command Execution Pattern**

**Concepto**: Ejecución asíncrona de comandos Git con manejo robusto de errores y logging comprehensivo.

**Implementación**:
```rust
// CommandExecutor para operaciones Git seguras
pub struct CommandExecutor {
    repository_path: PathBuf,
}

impl CommandExecutor {
    // Ejecuta comando con manejo completo de errores
    pub async fn execute(&self, args: &[&str]) -> Result<String, GitError> {
        let output = Command::new("git")
            .current_dir(&self.repository_path)
            .args(args)
            .output()
            .await
            .map_err(|e| GitError::Io { source: e })?;
            
        // Manejo inteligente de success/failure
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(GitError::CommandFailed { 
                message: String::from_utf8_lossy(&output.stderr).trim().to_string() 
            })
        }
    }
}
```

**Por qué es importante**: 
- **Async Operations**: Git commands no bloquean la UI ni otras operaciones
- **Error Recovery**: Manejo granular de diferentes tipos de errores Git
- **Logging Integration**: Cada comando se loggea para debugging y auditoría
- **Path Safety**: Operaciones siempre ejecutadas en el directorio correcto

---

### **5. Intelligent Branch Management con Sanitización**

**Concepto**: Gestión automática de branches con naming strategies y validación Git-compliant.

**Implementación**:
```rust
// BranchManager con validación completa
pub struct BranchManager {
    naming_config: BranchNamingConfig,
}

impl BranchManager {
    // Validación exhaustiva de nombres de branch
    pub fn validate_branch_name(&self, name: &str) -> Result<(), GitError> {
        // Git branch name rules:
        // - No spaces, ~, ^, :, ?, *, [, \, ..
        // - No start with . or end with .lock
        // - No double dots ..
        let regex = Regex::new(r"^[a-zA-Z0-9._/-]+$").unwrap();
        
        if !regex.is_match(name) {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name contains invalid characters".to_string() 
            });
        }
        
        // Check reserved names
        if matches!(name, "HEAD" | "master" | "main" | "origin" | "upstream") {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name is reserved".to_string() 
            });
        }
    }

    // Sanitización inteligente de nombres
    pub fn sanitize_branch_name(&self, name: &str) -> String {
        let sanitized = name.to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' | '.' | '_' | '-' | '/' => c,
                ' ' | '\t' | '\n' => '-',
                _ => '_',
            })
            .collect::<String>();
            
        // Remove consecutive special characters y truncate
        // Implementation ensures Git compliance
    }
}
```

**Por qué es importante**:
- **Git Compliance**: Garantiza que todos los nombres de branch son válidos
- **User-Friendly**: Convierte input del usuario en nombres válidos automáticamente
- **Consistent Strategy**: Todos los branches siguen la misma convención
- **Collision Avoidance**: Previene conflictos con nombres reservados

---

### **6. Auto-Push con Threshold Management**

**Concepto**: Sistema inteligente de auto-push basado en contador de commits con persistencia en archivos.

**Implementación**:
```rust
// PushCounter con persistencia async
pub struct PushCounter {
    counter_file_path: PathBuf,
}

impl PushCounter {
    // Lectura async del contador con fallback a 0
    pub async fn current_count(&self) -> Result<u32, GitError> {
        match fs::read_to_string(&self.counter_file_path).await {
            Ok(content) => content.trim().parse::<u32>(),
            Err(_) => {
                // File doesn't exist, start with 0
                self.write_count(0).await?;
                Ok(0)
            }
        }
    }
    
    // Auto-push logic en GitServiceImpl
    pub async fn auto_push_if_needed(&self) -> Result<bool, GitError> {
        if !self.config.auto_push.enabled {
            return Ok(false);
        }
        
        let unpushed_count = self.unpushed_commits_count().await?;
        
        if unpushed_count >= self.config.auto_push.push_threshold {
            self.push().await?;
            Ok(true) // Push realizado
        } else {
            Ok(false) // Push no necesario
        }
    }
}
```

**Por qué es importante**:
- **Intelligent Timing**: Push automático en momentos significativos, no spamming
- **File Persistence**: El contador sobrevive restarts de la aplicación
- **Configurable Threshold**: Cada proyecto puede tener diferentes necesidades
- **Network Optimization**: Reduce llamadas de red agrupando commits

---

### **7. Template-Based Commit Messages**

**Concepto**: Generación automática de mensajes de commit usando templates con variables contextuales.

**Implementación**:
```rust
// MessageBuilder con template system
pub struct MessageBuilder {
    template: CommitTemplate,
}

impl MessageBuilder {
    // Construcción de mensaje con contexto de session
    pub fn build_session_message(&self, session: &Session, custom_message: Option<&str>) -> Result<String, GitError> {
        let mut variables = HashMap::new();
        variables.insert("session_id".to_string(), session.session_id.to_string());
        variables.insert("project".to_string(), session.project_id.map_or("unknown".to_string(), |id| id.to_string()));
        
        // Template: "Session: {session_id} - {message}"
        self.substitute_variables(&self.template.session_template, &variables)
    }
    
    // Variable substitution con validation
    fn substitute_variables(&self, template: &str, variables: &HashMap<String, String>) -> Result<String, GitError> {
        let mut result = template.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        // Check for unsubstituted placeholders
        if result.contains('{') && result.contains('}') {
            return Err(GitError::ParseError { 
                message: format!("Unsubstituted placeholders in template: {}", template) 
            });
        }
        
        Ok(result)
    }
}
```

**Por qué es importante**:
- **Contextual Richness**: Mensajes contienen metadata relevante automáticamente
- **Team Consistency**: Todos los commits siguen el mismo formato
- **Template Flexibility**: Diferentes tipos de commits (session, action, branch) tienen templates específicos
- **Error Prevention**: Validation previene templates malformados

---

### **8. Repository Health Monitoring**

**Concepto**: Monitoreo continuo del estado del repositorio con health checks y reporting detallado.

**Implementación**:
```rust
// StatusChecker para monitoreo de repositorio
pub struct StatusChecker {
    executor: CommandExecutor,
}

impl StatusChecker {
    // Health check comprehensivo
    pub async fn health_check(&self) -> Result<HealthStatus, GitError> {
        let mut issues = Vec::new();
        
        // Check Git availability
        if let Err(e) = self.executor.check_git_available().await {
            issues.push(format!("Git not available: {}", e));
        }
        
        // Check repository state
        if !self.executor.is_git_repository().await? {
            issues.push("Not in a Git repository".to_string());
        } else {
            // Repository-specific checks
            if self.executor.execute_check(&["rev-parse", "HEAD"]).await?.is_none() {
                issues.push("Repository has no commits".to_string());
            }
            
            let status = self.get_status().await?;
            if !status.untracked_files.is_empty() {
                issues.push(format!("{} untracked files", status.untracked_files.len()));
            }
        }
        
        Ok(HealthStatus {
            is_healthy: issues.is_empty(),
            issues,
        })
    }
}
```

**Por qué es importante**:
- **Proactive Monitoring**: Detecta problemas antes que causen failures
- **Rich Diagnostics**: Información detallada para troubleshooting
- **Automated Recovery**: Base para implementar auto-healing en el futuro
- **Integration Ready**: Se puede integrar con sistemas de monitoring externos
