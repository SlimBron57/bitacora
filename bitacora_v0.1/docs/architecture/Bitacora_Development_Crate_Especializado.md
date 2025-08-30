# 💻 **BITÁCORA DESARROLLO: CRATE ESPECIALIZADO CON MANIPULACIÓN DE ARCHIVOS**

## 📋 **ANÁLISIS DE REQUERIMIENTO**

**Tu Pregunta**: Crear un crate especializado en desarrollo con manipulación de archivos (CRUD como Cursor), ejecución de comandos (cargo run, cargo build), y que sea controlado desde Bitácora básica.

**Contexto Detectado**:
- **Nivel técnico**: Avanzado (experiencia con Cursor, desarrollo Rust)
- **Objetivo**: Crate especializado pero integrado con arquitectura modular
- **Enfoque**: CRUD de archivos + ejecución de comandos + navegación inteligente
- **Visión**: Desarrollo como extensión especializada de Bitácora

---

## 🎯 **SOLUCIÓN ARQUITECTÓNICA**

### **Estrategia de Especialización**

La clave está en crear un **crate completamente autónomo** que puede funcionar independientemente pero se integra perfectamente con Bitácora Básica:

**🔧 bitacora-development** (Crate Especializado):
```
src/
├── lib.rs                    # API principal del crate
├── file_manager.rs           # CRUD de archivos como Cursor
├── command_runner.rs         # Ejecución de comandos
├── project_analyzer.rs       # Análisis de estructura de proyectos
├── language_support/         # Soporte por lenguaje
│   ├── rust.rs              # Cargo, rustfmt, clippy
│   ├── typescript.rs        # npm, yarn, tsc
│   ├── python.rs           # pip, poetry, pytest
│   └── generic.rs          # Soporte genérico
├── integrations/            # Integraciones existentes
│   ├── git_integration.rs   # Git (del crate existente)
│   ├── navigator.rs         # Navigator (del crate existente)
│   ├── templates.rs         # Templates (del crate existente)
│   └── ai_generator.rs      # AI Generator (del crate existente)
├── workspace/              # Gestión de workspace
│   ├── detection.rs        # Detección automática tipo proyecto
│   ├── configuration.rs    # Configuración por proyecto
│   └── monitoring.rs       # Monitoreo cambios en tiempo real
└── errors.rs               # Errores específicos desarrollo
```

### **Implementación Práctica**

#### **Paso 1: File Manager (CRUD como Cursor)**

```rust
// bitacora-development/src/file_manager.rs
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use tree_sitter::{Language, Parser, Tree};

pub struct DevelopmentFileManager {
    workspace_root: PathBuf,
    language_parsers: HashMap<String, Parser>,
    file_watcher: FileWatcher,
    indexer: CodeIndexer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationRequest {
    pub operation: FileOperation,
    pub path: PathBuf,
    pub content: Option<String>,
    pub options: FileOperationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileOperation {
    Create,
    Read,
    Update,
    Delete,
    Move,
    Copy,
    Search,
    Navigate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationOptions {
    pub backup_before_change: bool,
    pub format_after_save: bool,
    pub lint_after_save: bool,
    pub update_imports: bool,
    pub track_in_git: bool,
}

impl DevelopmentFileManager {
    pub fn new(workspace_root: PathBuf) -> Result<Self> {
        let mut language_parsers = HashMap::new();
        
        // Inicializar parsers para diferentes lenguajes
        language_parsers.insert("rust".to_string(), Self::create_rust_parser()?);
        language_parsers.insert("typescript".to_string(), Self::create_ts_parser()?);
        language_parsers.insert("python".to_string(), Self::create_python_parser()?);
        
        let file_watcher = FileWatcher::new(&workspace_root)?;
        let indexer = CodeIndexer::new(&workspace_root)?;
        
        Ok(Self {
            workspace_root,
            language_parsers,
            file_watcher,
            indexer,
        })
    }

    // CRUD Operations (estilo Cursor)
    pub async fn create_file(
        &mut self,
        request: FileOperationRequest
    ) -> Result<FileOperationResult> {
        let full_path = self.workspace_root.join(&request.path);
        
        // Validar que el directorio padre existe
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Crear archivo con contenido
        let content = request.content.unwrap_or_default();
        fs::write(&full_path, &content).await?;
        
        // Post-procesamiento según opciones
        if request.options.format_after_save {
            self.format_file(&full_path).await?;
        }
        
        if request.options.update_imports {
            self.update_imports(&full_path).await?;
        }
        
        if request.options.track_in_git {
            self.git_add(&full_path).await?;
        }
        
        // Actualizar índice
        self.indexer.index_file(&full_path).await?;
        
        Ok(FileOperationResult {
            operation: FileOperation::Create,
            path: request.path,
            success: true,
            details: Some("Archivo creado exitosamente".to_string()),
        })
    }
    
    pub async fn read_file(
        &self,
        path: &Path
    ) -> Result<FileContent> {
        let full_path = self.workspace_root.join(path);
        let content = fs::read_to_string(&full_path).await?;
        
        // Detectar lenguaje y parsear si es necesario
        let language = self.detect_language(&full_path)?;
        let syntax_tree = if let Some(parser) = self.language_parsers.get(&language) {
            Some(self.parse_content(parser, &content)?)
        } else {
            None
        };
        
        Ok(FileContent {
            path: path.to_path_buf(),
            content,
            language,
            syntax_tree,
            line_count: content.lines().count(),
            char_count: content.chars().count(),
        })
    }
    
    pub async fn update_file(
        &mut self,
        request: FileOperationRequest
    ) -> Result<FileOperationResult> {
        let full_path = self.workspace_root.join(&request.path);
        
        // Backup si está habilitado
        if request.options.backup_before_change {
            self.create_backup(&full_path).await?;
        }
        
        // Actualizar contenido
        let content = request.content.expect("Content required for update");
        fs::write(&full_path, &content).await?;
        
        // Post-procesamiento
        if request.options.format_after_save {
            self.format_file(&full_path).await?;
        }
        
        if request.options.lint_after_save {
            let lint_results = self.lint_file(&full_path).await?;
            // TODO: Reportar warnings/errors
        }
        
        // Re-indexar
        self.indexer.update_file_index(&full_path).await?;
        
        Ok(FileOperationResult::success(FileOperation::Update, request.path))
    }
    
    pub async fn intelligent_search(
        &self,
        query: SearchQuery
    ) -> Result<Vec<SearchResult>> {
        match query.search_type {
            SearchType::TextContent => {
                self.search_in_content(&query.pattern).await
            }
            SearchType::SymbolDefinition => {
                self.indexer.find_symbol_definitions(&query.pattern).await
            }
            SearchType::SymbolReferences => {
                self.indexer.find_symbol_references(&query.pattern).await
            }
            SearchType::FilePath => {
                self.search_file_paths(&query.pattern).await
            }
        }
    }
    
    // Funcionalidades específicas de desarrollo
    pub async fn refactor_rename(
        &mut self,
        symbol: &str,
        new_name: &str
    ) -> Result<RefactorResult> {
        // 1. Encontrar todas las referencias
        let references = self.indexer.find_symbol_references(symbol).await?;
        
        // 2. Aplicar cambios en todos los archivos
        let mut changed_files = Vec::new();
        for reference in references {
            let file_content = self.read_file(&reference.file).await?;
            let updated_content = self.replace_symbol_in_content(
                &file_content.content,
                symbol,
                new_name,
                &reference.location
            )?;
            
            let update_request = FileOperationRequest {
                operation: FileOperation::Update,
                path: reference.file.clone(),
                content: Some(updated_content),
                options: FileOperationOptions::default(),
            };
            
            self.update_file(update_request).await?;
            changed_files.push(reference.file);
        }
        
        Ok(RefactorResult {
            operation: "rename".to_string(),
            files_changed: changed_files,
            success: true,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
    pub language: String,
    pub syntax_tree: Option<Tree>,
    pub line_count: usize,
    pub char_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub pattern: String,
    pub search_type: SearchType,
    pub file_filters: Vec<String>,
    pub case_sensitive: bool,
    pub regex_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    TextContent,
    SymbolDefinition,
    SymbolReferences,
    FilePath,
}
```

#### **Paso 2: Command Runner (Ejecución de Comandos)**

```rust
// bitacora-development/src/command_runner.rs
use tokio::process::{Command, Child};
use tokio::io::{AsyncBufReadExt, BufReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct DevelopmentCommandRunner {
    workspace_root: PathBuf,
    language_configs: HashMap<String, LanguageConfig>,
    active_processes: HashMap<String, Child>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRequest {
    pub command_type: CommandType,
    pub language: Option<String>,
    pub arguments: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub environment: HashMap<String, String>,
    pub background: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    // Rust commands
    CargoBuild,
    CargoRun,
    CargoTest,
    CargoClippy,
    CargoFmt,
    CargoCheck,
    
    // Node.js/TypeScript commands
    NpmInstall,
    NpmBuild,
    NpmTest,
    NpmStart,
    YarnBuild,
    
    // Python commands
    PipInstall,
    PythonRun,
    PytestRun,
    
    // Generic commands
    Custom(String),
    Script(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub command: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration: std::time::Duration,
    pub background_process_id: Option<String>,
}

impl DevelopmentCommandRunner {
    pub fn new(workspace_root: PathBuf) -> Result<Self> {
        let mut language_configs = HashMap::new();
        
        // Configurar comandos por lenguaje
        language_configs.insert("rust".to_string(), LanguageConfig {
            build_command: vec!["cargo".to_string(), "build".to_string()],
            run_command: vec!["cargo".to_string(), "run".to_string()],
            test_command: vec!["cargo".to_string(), "test".to_string()],
            format_command: Some(vec!["cargo".to_string(), "fmt".to_string()]),
            lint_command: Some(vec!["cargo".to_string(), "clippy".to_string()]),
        });
        
        language_configs.insert("typescript".to_string(), LanguageConfig {
            build_command: vec!["npm".to_string(), "run".to_string(), "build".to_string()],
            run_command: vec!["npm".to_string(), "start".to_string()],
            test_command: vec!["npm".to_string(), "test".to_string()],
            format_command: Some(vec!["npx".to_string(), "prettier".to_string(), "--write".to_string(), ".".to_string()]),
            lint_command: Some(vec!["npx".to_string(), "eslint".to_string(), ".".to_string()]),
        });
        
        Ok(Self {
            workspace_root,
            language_configs,
            active_processes: HashMap::new(),
        })
    }
    
    pub async fn execute_command(
        &mut self,
        request: CommandRequest
    ) -> Result<CommandResult> {
        let start_time = std::time::Instant::now();
        
        // Resolver comando basado en tipo y lenguaje
        let (command, args) = self.resolve_command(&request)?;
        
        let working_dir = request.working_directory
            .unwrap_or_else(|| self.workspace_root.clone());
        
        // Crear proceso
        let mut cmd = Command::new(&command);
        cmd.args(&args)
           .current_dir(&working_dir);
        
        // Añadir variables de entorno
        for (key, value) in request.environment {
            cmd.env(key, value);
        }
        
        if request.background {
            // Ejecutar en background
            let child = cmd.stdout(std::process::Stdio::piped())
                          .stderr(std::process::Stdio::piped())
                          .spawn()?;
            
            let process_id = uuid::Uuid::new_v4().to_string();
            self.active_processes.insert(process_id.clone(), child);
            
            Ok(CommandResult {
                command: format!("{} {}", command, args.join(" ")),
                exit_code: 0, // Still running
                stdout: String::new(),
                stderr: String::new(),
                duration: start_time.elapsed(),
                background_process_id: Some(process_id),
            })
        } else {
            // Ejecutar y esperar resultado
            let output = cmd.output().await?;
            
            Ok(CommandResult {
                command: format!("{} {}", command, args.join(" ")),
                exit_code: output.status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                duration: start_time.elapsed(),
                background_process_id: None,
            })
        }
    }
    
    // Comandos específicos como métodos de conveniencia
    pub async fn cargo_build(&mut self) -> Result<CommandResult> {
        self.execute_command(CommandRequest {
            command_type: CommandType::CargoBuild,
            language: Some("rust".to_string()),
            arguments: vec![],
            working_directory: None,
            environment: HashMap::new(),
            background: false,
        }).await
    }
    
    pub async fn cargo_run(&mut self, args: Vec<String>) -> Result<CommandResult> {
        self.execute_command(CommandRequest {
            command_type: CommandType::CargoRun,
            language: Some("rust".to_string()),
            arguments: args,
            working_directory: None,
            environment: HashMap::new(),
            background: false,
        }).await
    }
    
    pub async fn cargo_test(&mut self, test_filter: Option<String>) -> Result<CommandResult> {
        let mut args = vec![];
        if let Some(filter) = test_filter {
            args.push(filter);
        }
        
        self.execute_command(CommandRequest {
            command_type: CommandType::CargoTest,
            language: Some("rust".to_string()),
            arguments: args,
            working_directory: None,
            environment: HashMap::new(),
            background: false,
        }).await
    }
    
    // Stream de output en tiempo real para comandos largos
    pub async fn execute_with_streaming<F>(
        &mut self,
        request: CommandRequest,
        mut output_handler: F
    ) -> Result<CommandResult>
    where
        F: FnMut(StreamOutput) + Send + 'static,
    {
        let (command, args) = self.resolve_command(&request)?;
        let working_dir = request.working_directory
            .unwrap_or_else(|| self.workspace_root.clone());
        
        let mut child = Command::new(&command)
            .args(&args)
            .current_dir(&working_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        
        // Stream stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            
            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    output_handler(StreamOutput::Stdout(line));
                }
            });
        }
        
        // Stream stderr
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            
            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    output_handler(StreamOutput::Stderr(line));
                }
            });
        }
        
        let status = child.wait().await?;
        
        Ok(CommandResult {
            command: format!("{} {}", command, args.join(" ")),
            exit_code: status.code().unwrap_or(-1),
            stdout: "Streamed".to_string(),
            stderr: "Streamed".to_string(),
            duration: std::time::Duration::from_secs(0), // TODO: Track actual duration
            background_process_id: None,
        })
    }
    
    fn resolve_command(&self, request: &CommandRequest) -> Result<(String, Vec<String>)> {
        match &request.command_type {
            CommandType::CargoBuild => {
                let mut args = vec!["build".to_string()];
                args.extend(request.arguments.clone());
                Ok(("cargo".to_string(), args))
            }
            CommandType::CargoRun => {
                let mut args = vec!["run".to_string()];
                if !request.arguments.is_empty() {
                    args.push("--".to_string());
                    args.extend(request.arguments.clone());
                }
                Ok(("cargo".to_string(), args))
            }
            CommandType::CargoTest => {
                let mut args = vec!["test".to_string()];
                args.extend(request.arguments.clone());
                Ok(("cargo".to_string(), args))
            }
            CommandType::Custom(command) => {
                Ok((command.clone(), request.arguments.clone()))
            }
            _ => {
                // Resolver basado en configuración de lenguaje
                if let Some(language) = &request.language {
                    if let Some(config) = self.language_configs.get(language) {
                        match &request.command_type {
                            CommandType::CargoBuild => Ok((config.build_command[0].clone(), config.build_command[1..].to_vec())),
                            _ => Err(DevelopmentError::UnsupportedCommand(format!("{:?}", request.command_type)))
                        }
                    } else {
                        Err(DevelopmentError::UnsupportedLanguage(language.clone()))
                    }
                } else {
                    Err(DevelopmentError::LanguageRequired)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StreamOutput {
    Stdout(String),
    Stderr(String),
}
```

#### **Paso 3: Integración con Bitácora Básica**

```rust
// bitacora-development/src/lib.rs
pub struct DevelopmentExtension {
    file_manager: DevelopmentFileManager,
    command_runner: DevelopmentCommandRunner,
    project_analyzer: ProjectAnalyzer,
    
    // Integraciones existentes
    git_integration: Option<GitIntegration>,
    navigator: Option<HybridNavigator>,
    template_system: Option<TemplateRepository>,
    ai_generator: Option<AITemplateGenerator>,
}

impl BitacoraExtension for DevelopmentExtension {
    fn extension_id(&self) -> &str {
        "bitacora-development"
    }
    
    fn extension_type(&self) -> ExtensionType {
        ExtensionType::Development
    }
    
    fn supports_project_type(&self, project_type: &ProjectType) -> bool {
        matches!(project_type, ProjectType::Development)
    }
    
    async fn enhance_project(
        &self,
        project: &mut Project,
        context: &ExtensionContext
    ) -> Result<()> {
        // Detectar tipo de proyecto desarrollo
        let project_type = self.project_analyzer
            .detect_project_type(&context.project_path)
            .await?;
        
        // Configurar herramientas específicas
        match project_type {
            DetectedProjectType::Rust => {
                self.setup_rust_project(project, context).await?;
            }
            DetectedProjectType::TypeScript => {
                self.setup_typescript_project(project, context).await?;
            }
            DetectedProjectType::Python => {
                self.setup_python_project(project, context).await?;
            }
            _ => {
                self.setup_generic_project(project, context).await?;
            }
        }
        
        Ok(())
    }
    
    async fn provide_actions(
        &self,
        project: &Project
    ) -> Result<Vec<ExtensionAction>> {
        let mut actions = vec![];
        
        // Acciones básicas de archivo
        actions.extend(vec![
            ExtensionAction::new("create-file", "Crear archivo"),
            ExtensionAction::new("edit-file", "Editar archivo"),
            ExtensionAction::new("search-code", "Buscar en código"),
            ExtensionAction::new("refactor-rename", "Refactorizar/Renombrar"),
        ]);
        
        // Acciones específicas según tipo de proyecto
        if let Some(metadata) = &project.metadata {
            if let Ok(dev_metadata) = serde_json::from_value::<DevelopmentMetadata>(metadata.clone()) {
                match dev_metadata.language.as_str() {
                    "rust" => {
                        actions.extend(vec![
                            ExtensionAction::new("cargo-build", "Cargo Build"),
                            ExtensionAction::new("cargo-run", "Cargo Run"),
                            ExtensionAction::new("cargo-test", "Cargo Test"),
                            ExtensionAction::new("cargo-clippy", "Cargo Clippy"),
                            ExtensionAction::new("cargo-fmt", "Cargo Format"),
                        ]);
                    }
                    "typescript" => {
                        actions.extend(vec![
                            ExtensionAction::new("npm-install", "NPM Install"),
                            ExtensionAction::new("npm-build", "NPM Build"),
                            ExtensionAction::new("npm-test", "NPM Test"),
                            ExtensionAction::new("npm-start", "NPM Start"),
                        ]);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(actions)
    }
}

// API pública del crate
impl DevelopmentExtension {
    pub async fn create_file(&mut self, path: &Path, content: &str) -> Result<()> {
        self.file_manager.create_file(FileOperationRequest {
            operation: FileOperation::Create,
            path: path.to_path_buf(),
            content: Some(content.to_string()),
            options: FileOperationOptions::default(),
        }).await?;
        Ok(())
    }
    
    pub async fn build_project(&mut self) -> Result<CommandResult> {
        // Detectar tipo de proyecto y ejecutar build apropiado
        let project_type = self.project_analyzer
            .detect_current_project_type()
            .await?;
        
        match project_type {
            DetectedProjectType::Rust => {
                self.command_runner.cargo_build().await
            }
            DetectedProjectType::TypeScript => {
                self.command_runner.execute_command(CommandRequest {
                    command_type: CommandType::NpmBuild,
                    language: Some("typescript".to_string()),
                    arguments: vec![],
                    working_directory: None,
                    environment: HashMap::new(),
                    background: false,
                }).await
            }
            _ => {
                Err(DevelopmentError::UnsupportedProjectType)
            }
        }
    }
    
    pub async fn run_project(&mut self, args: Vec<String>) -> Result<CommandResult> {
        let project_type = self.project_analyzer
            .detect_current_project_type()
            .await?;
        
        match project_type {
            DetectedProjectType::Rust => {
                self.command_runner.cargo_run(args).await
            }
            DetectedProjectType::TypeScript => {
                self.command_runner.execute_command(CommandRequest {
                    command_type: CommandType::NpmStart,
                    language: Some("typescript".to_string()),
                    arguments: args,
                    working_directory: None,
                    environment: HashMap::new(),
                    background: false,
                }).await
            }
            _ => {
                Err(DevelopmentError::UnsupportedProjectType)
            }
        }
    }
}
```

#### **Paso 4: API Endpoints Específicos**

```rust
// bitacora-development/src/api.rs
use axum::{Router, routing::{get, post}, Json};

pub fn create_development_routes() -> Router {
    Router::new()
        .route("/api/dev/files", post(create_file))
        .route("/api/dev/files/*path", get(read_file).put(update_file).delete(delete_file))
        .route("/api/dev/search", post(search_code))
        .route("/api/dev/commands/build", post(build_project))
        .route("/api/dev/commands/run", post(run_project))
        .route("/api/dev/commands/test", post(test_project))
        .route("/api/dev/refactor/rename", post(refactor_rename))
}

async fn create_file(Json(request): Json<CreateFileRequest>) -> Result<Json<FileOperationResult>> {
    // TODO: Implementar
    Ok(Json(FileOperationResult::success(FileOperation::Create, request.path)))
}

async fn build_project() -> Result<Json<CommandResult>> {
    // TODO: Implementar
    Ok(Json(CommandResult {
        command: "cargo build".to_string(),
        exit_code: 0,
        stdout: "Build successful".to_string(),
        stderr: String::new(),
        duration: std::time::Duration::from_secs(5),
        background_process_id: None,
    }))
}
```

---

## 🚀 **PLAN DE IMPLEMENTACIÓN**

### **Fase 1: Core File & Command Operations**
1. **DevelopmentFileManager** con CRUD básico
2. **DevelopmentCommandRunner** con comandos principales
3. **Integración básica** con Bitácora

### **Fase 2: Funcionalidades Avanzadas**
1. **Búsqueda inteligente** con indexación
2. **Refactoring automático** 
3. **Streaming de comandos** para feedback en tiempo real

### **Fase 3: Integraciones Completas**
1. **Navigator integration** para navegación de código
2. **AI Generator integration** para generación automática
3. **Git integration** para control de versiones

### **Fase 4: Optimización y UX**
1. **Auto-detección** de tipos de proyecto
2. **Configuraciones inteligentes** por proyecto
3. **Monitoreo en tiempo real** de cambios

---

## 🎯 **CASOS DE USO PRÁCTICOS**

### **Desarrollador Rust**
```bash
# Desde Bitácora Básica, activar extensión desarrollo
bitacora enable-extension development

# Crear proyecto Rust
bitacora create-project "Mi API" --type development --language rust

# Operaciones de archivo (estilo Cursor)
bitacora dev create-file src/models/user.rs
bitacora dev edit-file src/main.rs
bitacora dev search-code "async fn" --type definition

# Comandos de desarrollo
bitacora dev build
bitacora dev run -- --port 8080
bitacora dev test user_tests

# Refactoring inteligente
bitacora dev refactor rename User NewUser
```

### **Desarrollador TypeScript**
```bash
# Cambiar a proyecto TypeScript
cd my-react-app
bitacora dev detect-project  # Auto-detecta TypeScript/React

# Operaciones específicas
bitacora dev npm install
bitacora dev npm run build
bitacora dev npm start -- --port 3000

# Búsqueda avanzada
bitacora dev search-code "useState" --type references
```

Esta arquitectura te da un **crate completamente especializado** que funciona como Cursor pero integrado perfectamente con el ecosistema Bitácora, manteniendo la separación de responsabilidades que planteaste.

---

*Documento creado: August 29, 2025*
*Metodología aplicada: AI→USR Técnico-Práctico*
*Contexto: Crate especializado, CRUD de archivos, ejecución comandos*
