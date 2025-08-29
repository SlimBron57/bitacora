# 📚 BitacoraNavigator API Documentation

## Core Components

### `HybridNavigator`

The main navigation engine with AI-powered decision making and multi-level threading.

#### Constructor Methods
```rust
impl HybridNavigator {
    /// Create a new navigator with default configuration
    pub fn new_core() -> Result<Self>
    
    /// Create navigator with specific mode
    pub fn with_mode(mode: NavigatorMode) -> Result<Self>
    
    /// Create navigator with custom configuration  
    pub fn with_config(mode: NavigatorMode, config: NavigatorConfig) -> Result<Self>
}
```

#### Navigation Methods
```rust
impl HybridNavigator {
    /// Execute basic navigation with context
    pub async fn navigate(&self, context: &NavigationContext) -> Result<Vec<ContentResult>>
    
    /// Update navigation indices
    pub fn update_indices(&mut self, new_indices: Vec<NavigationIndex>) -> Result<()>
}
```

---

### `BitaflowNavigatorEngine`

Integration engine that combines HybridNavigator with BitaFlow DSL templates.

#### Constructor
```rust
impl BitaflowNavigatorEngine {
    /// Create new engine with navigator instance
    pub fn new(hybrid_navigator: HybridNavigator) -> Self
}
```

#### Template Management
```rust
impl BitaflowNavigatorEngine {
    /// Load template from BFL content string
    pub fn load_template(&mut self, bfl_content: &str) -> Result<String, NavigatorError>
    
    /// Load template from .bfl file
    pub async fn load_template_from_file(&mut self, file_path: &str) -> Result<String, NavigatorError>
    
    /// List all loaded templates
    pub fn list_templates(&self) -> Vec<&NavigatorTemplate>
    
    /// Execute specific template by alias
    pub async fn execute_template(
        &mut self, 
        alias: &str, 
        context: HashMap<String, String>
    ) -> Result<NavigationResult, NavigatorError>
    
    /// Generate new template using AI
    pub async fn generate_template(
        &mut self,
        domain: &str,
        topic: &str, 
        requirements: &str
    ) -> Result<String, NavigatorError>
}
```

---

### `AliasValidator`

Validates and manages BitaFlow template aliases following BITA-NAV-* pattern.

#### Constructor
```rust
impl AliasValidator {
    /// Create new validator with default regex pattern
    pub fn new() -> Self
}
```

#### Validation Methods
```rust
impl AliasValidator {
    /// Validate alias format
    pub fn validate_alias(&self, alias: &str) -> Result<(), NavigatorError>
    
    /// Parse alias components (domain, topic, version)
    pub fn parse_alias(&self, alias: &str) -> Result<(String, String, u32), NavigatorError>
    
    /// Generate new alias for navigator
    pub fn generate_navigator_alias(&self, domain: &str, topic: &str) -> Result<String, NavigatorError>
    
    /// Increment version of existing alias
    pub fn increment_version(&self, alias: &str) -> Result<String, NavigatorError>
}
```

---

## Data Structures

### `NavigatorTemplate`

Template structure containing all metadata and navigation flow.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigatorTemplate {
    /// Unique template identifier (BITA-NAV-DOMAIN-TOPIC-v1)
    pub alias: String,
    
    /// Human-readable template name
    pub name: String,
    
    /// Template description and purpose
    pub description: String,
    
    /// Domain specialization (debug, code-review, etc.)
    pub domain: String,
    
    /// Specific topic within domain
    pub topic: String,
    
    /// Template version number
    pub version: u32,
    
    /// Autonomy level for execution
    pub autonomy_level: AutonomyLevel,
    
    /// Required threading level
    pub thread_level: ThreadLevel,
    
    /// Raw BFL navigation content
    pub bfl_content: String,
    
    /// Template variables for substitution
    pub variables: HashMap<String, String>,
    
    /// Performance and usage metrics
    pub metrics: NavigatorMetrics,
}
```

### `NavigationResult`

Result structure returned after template execution.

```rust
#[derive(Debug, Clone)]
pub struct NavigationResult {
    /// Whether execution was successful
    pub success: bool,
    
    /// List of actions performed
    pub actions_taken: Vec<String>,
    
    /// Total execution time in seconds
    pub execution_time: f64,
    
    /// Final output message
    pub output: String,
}
```

### `AutonomyLevel`

Defines the level of autonomous execution permitted.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutonomyLevel {
    /// Complete autonomous execution
    Full,
    
    /// Only pre-approved actions allowed
    Restricted,
    
    /// Interactive confirmation for critical actions
    Interactive,
    
    /// Manual control with step logging
    Manual,
}
```

### `ThreadLevel`

Defines the threading isolation level required.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadLevel {
    /// Level 0: Maximum isolation per spark
    SparkIsolated,
    
    /// Level 1: Isolated within project boundaries
    ProjectIsolated,
    
    /// Level 2: Serial execution within topic
    TopicSerial,
    
    /// Level 3: Completely serial execution
    FullSerial,
}
```

### `NavigatorMetrics`

Performance and usage tracking for templates.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigatorMetrics {
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    
    /// Average execution time in seconds
    pub avg_execution_time: f64,
    
    /// Total number of times used
    pub usage_count: u32,
    
    /// Timestamp of last usage
    pub last_used: Option<String>,
}
```

---

## Error Types

### `NavigatorError`

Main error type for all navigator operations.

```rust
pub enum NavigatorError {
    InitializationError(String),
    ConfigurationError(String),
    ThreadingError(String),
    SafetyError(String),
    AIEngineError(String),
    NetworkError(String),
    FileSystemError(String),
    DataError(String),
    ValidationError(String),
    TemplateError(String),
    ExecutionError(String),
    IntegrationError(String),
}
```

#### Helper Methods
```rust
impl NavigatorError {
    pub fn initialization<S: Into<String>>(msg: S) -> Self
    pub fn configuration<S: Into<String>>(msg: S) -> Self
    pub fn threading<S: Into<String>>(msg: S) -> Self
    pub fn safety<S: Into<String>>(msg: S) -> Self
    pub fn ai_engine<S: Into<String>>(msg: S) -> Self
    pub fn network<S: Into<String>>(msg: S) -> Self
    pub fn filesystem<S: Into<String>>(msg: S) -> Self
    pub fn data<S: Into<String>>(msg: S) -> Self
    pub fn validation<S: Into<String>>(msg: S) -> Self
    pub fn template<S: Into<String>>(msg: S) -> Self
    pub fn execution<S: Into<String>>(msg: S) -> Self
    pub fn integration<S: Into<String>>(msg: S) -> Self
}
```

---

## Usage Examples

### Basic Template Loading and Execution
```rust
use bitacora_navigator::{HybridNavigator, BitaflowNavigatorEngine};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create navigator and engine
    let navigator = HybridNavigator::new_core()?;
    let mut engine = BitaflowNavigatorEngine::new(navigator);
    
    // Load template from file
    let alias = engine.load_template_from_file("debug-error.bfl").await?;
    
    // Prepare context
    let mut context = HashMap::new();
    context.insert("error_type".to_string(), "NullPointerException".to_string());
    context.insert("severity".to_string(), "HIGH".to_string());
    
    // Execute template
    let result = engine.execute_template(&alias, context).await?;
    
    println!("Success: {}", result.success);
    println!("Actions taken: {}", result.actions_taken.len());
    println!("Execution time: {:.2}s", result.execution_time);
    
    Ok(())
}
```

### Alias Management
```rust
use bitacora_navigator::AliasValidator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let validator = AliasValidator::new();
    
    // Generate new alias
    let alias = validator.generate_navigator_alias("code-review", "pull-request")?;
    println!("Generated: {}", alias); // BITA-NAV-CODE-REVIEW-PULL-REQUEST-v1
    
    // Parse existing alias
    let (domain, topic, version) = validator.parse_alias(&alias)?;
    println!("Domain: {}, Topic: {}, Version: {}", domain, topic, version);
    
    // Increment version
    let next_alias = validator.increment_version(&alias)?;
    println!("Next version: {}", next_alias); // BITA-NAV-CODE-REVIEW-PULL-REQUEST-v2
    
    Ok(())
}
```

### Custom Template Creation
```rust
// Create template programmatically
let template_content = r#"---
name: "Custom Workflow Navigator"
alias: "BITA-NAV-CUSTOM-WORKFLOW-v1"
domain: "custom"
topic: "workflow"
autonomy_level: "Interactive"
thread_level: "ProjectIsolated"
variables:
  input_param: "{{user_input}}"
---

# Custom Navigation Flow
1. **Initialize workflow** → `{{initialization}}`
2. **Process input** → `{{input_processing}}`
3. **Generate output** → `{{output_generation}}`
"#;

let alias = engine.load_template(template_content)?;
```

---

## Thread Safety

All components are designed to be thread-safe:

- `HybridNavigator` uses internal locking for concurrent access
- `BitaflowNavigatorEngine` manages template state safely
- `AliasValidator` is stateless and inherently thread-safe

## Performance Considerations

- Templates are cached in memory after loading
- Threading levels optimize resource usage
- Metrics collection has minimal overhead
- AI decisions are cached when possible

---

*For more examples and advanced usage, see the [examples/](../examples/) directory.*
