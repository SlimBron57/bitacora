# Git Service - Technical Implementation Guide

## 🏗️ Architecture Overview

The Git Service (`bitacora-git`) is a comprehensive async Git integration layer that provides intelligent automation for all Git operations in Bitacora. Built with Rust and designed around async/await patterns, it transforms manual Git workflows into automated, intelligent processes.

## 📦 Crate Structure

```
bitacora-git/
├── src/
│   ├── lib.rs                      # Main exports and re-exports
│   ├── errors.rs                   # Git-specific error types
│   ├── config.rs                   # Configuration structures
│   ├── service/                    # Core service implementations
│   │   ├── mod.rs                  # Service exports
│   │   ├── mod_service.rs          # GitService trait definition
│   │   ├── git_service_impl.rs     # Main service implementation
│   │   └── command_executor.rs     # Async Git command execution
│   ├── branch/                     # Branch management
│   │   ├── mod.rs                  # Branch exports
│   │   ├── branch_info.rs          # Data structures
│   │   ├── branch_manager.rs       # Branch operations and validation
│   │   ├── naming_strategy.rs      # Naming conventions
│   │   └── validation.rs           # Git compliance validation
│   ├── commit/                     # Commit operations
│   │   ├── mod.rs                  # Commit exports
│   │   ├── auto_commit.rs          # Automated commit logic
│   │   ├── push_counter.rs         # Push threshold management
│   │   ├── message_builder.rs      # Template-based commit messages
│   │   └── commit_info.rs          # Commit data structures
│   └── repository/                 # Repository management
│       ├── mod.rs                  # Repository exports
│       ├── repo_manager.rs         # Repository lifecycle
│       ├── status_checker.rs       # Health monitoring
│       └── repository_status.rs    # Status data structures
├── Cargo.toml                      # Dependencies and configuration
└── tests/                          # Integration tests (future)
```

## 🔌 Core Dependencies

```toml
[dependencies]
bitacora-core = { path = "../bitacora-core" }
tokio = { version = "1.0", features = ["process", "fs", "macros", "rt"] }
async-trait = "0.1"
tracing = "0.1"
regex = "1.0"
tempfile = "3.0"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

## 🚀 Key Technical Components

### 1. GitService Trait

**Location**: `src/service/mod_service.rs`

The main service interface defining all Git operations:

```rust
#[async_trait]
pub trait GitService: Send + Sync {
    // Repository lifecycle
    async fn init_repository(&self, path: &PathBuf) -> BitacoraResult<()>;
    async fn validate_repository(&self) -> Result<(), GitError>;
    
    // Branch operations
    async fn create_branch(&self, branch_name: &str) -> Result<BranchInfo, GitError>;
    async fn switch_branch(&self, branch_name: &str) -> Result<(), GitError>;
    async fn current_branch(&self) -> Result<BranchInfo, GitError>;
    async fn list_branches(&self) -> Result<Vec<BranchInfo>, GitError>;
    
    // Commit operations
    async fn commit(&self, message: &str) -> Result<CommitInfo, GitError>;
    async fn push(&self) -> Result<(), GitError>;
    async fn auto_push_if_needed(&self) -> Result<bool, GitError>;
    
    // Status and health
    async fn status(&self) -> Result<RepositoryStatus, GitError>;
    async fn is_working_directory_clean(&self) -> Result<bool, GitError>;
    async fn unpushed_commits_count(&self) -> Result<u32, GitError>;
}
```

**Design Decisions**:
- **Async-first**: All methods are async for non-blocking I/O
- **Rich return types**: Structured data instead of raw strings
- **Comprehensive coverage**: All essential Git operations included
- **Error handling**: Specific GitError types for different failure modes

### 2. CommandExecutor

**Location**: `src/service/command_executor.rs`

Handles safe, async execution of Git commands:

```rust
impl CommandExecutor {
    pub async fn execute(&self, args: &[&str]) -> Result<String, GitError> {
        debug!("Executing git command: git {}", args.join(" "));
        
        let output = Command::new("git")
            .current_dir(&self.repository_path)
            .args(args)
            .output()
            .await
            .map_err(|e| GitError::Io { source: e })?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            debug!("Git command succeeded: {}", stdout);
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            error!("Git command failed: {}", stderr);
            Err(GitError::CommandFailed { message: stderr })
        }
    }
}
```

**Key Features**:
- **Safe execution**: Always runs in correct directory context
- **Comprehensive logging**: Debug info for all operations
- **Error context**: Detailed error information from Git
- **UTF-8 handling**: Proper string conversion from Git output

### 3. BranchManager

**Location**: `src/branch/branch_manager.rs`

Intelligent branch management with validation and sanitization:

```rust
impl BranchManager {
    pub fn validate_branch_name(&self, name: &str) -> Result<(), GitError> {
        // Length check
        if name.len() > self.naming_config.max_length {
            return Err(GitError::BranchValidationFailed { 
                reason: format!("Branch name too long (max: {})", self.naming_config.max_length) 
            });
        }
        
        // Git-compliant regex validation
        let regex = Regex::new(r"^[a-zA-Z0-9._/-]+$").unwrap();
        if !regex.is_match(name) {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name contains invalid characters".to_string() 
            });
        }
        
        // Git-specific invalid patterns
        if name.starts_with('.') || name.ends_with(".lock") || name.contains("..") {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name violates Git naming rules".to_string() 
            });
        }
        
        Ok(())
    }
}
```

**Validation Rules**:
- **Git compliance**: Follows official Git branch naming rules
- **Length limits**: Configurable maximum length
- **Character restrictions**: Only allows safe characters
- **Reserved names**: Prevents conflicts with Git internals

### 4. PushCounter

**Location**: `src/commit/push_counter.rs`

File-based counter for intelligent auto-push decisions:

```rust
impl PushCounter {
    pub async fn increment(&self) -> Result<u32, GitError> {
        let current = self.current_count().await?;
        let new_count = current + 1;
        self.write_count(new_count).await?;
        debug!("Push counter incremented to: {}", new_count);
        Ok(new_count)
    }
    
    async fn write_count(&self, count: u32) -> Result<(), GitError> {
        // Ensure parent directory exists
        if let Some(parent) = self.counter_file_path.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| GitError::Io { source: e })?;
        }
        
        fs::write(&self.counter_file_path, count.to_string()).await
            .map_err(|e| GitError::Io { source: e })?;
            
        Ok(())
    }
}
```

**Features**:
- **Persistence**: Survives application restarts
- **Atomic operations**: Safe concurrent access
- **Directory creation**: Automatically creates cache directories
- **Error handling**: Comprehensive I/O error management

### 5. MessageBuilder

**Location**: `src/commit/message_builder.rs`

Template-based commit message generation:

```rust
impl MessageBuilder {
    pub fn build_session_message(&self, session: &Session, custom_message: Option<&str>) -> Result<String, GitError> {
        let mut variables = HashMap::new();
        variables.insert("session_id".to_string(), session.session_id.to_string());
        variables.insert("project".to_string(), 
            session.project_id.as_ref()
                .map(|id| id.to_string())
                .unwrap_or_else(|| "unknown".to_string())
        );
        
        if let Some(msg) = custom_message {
            variables.insert("message".to_string(), msg.to_string());
        } else {
            variables.insert("message".to_string(), 
                format!("Working on {}", 
                    session.description.clone().unwrap_or_else(|| "session".to_string())
                )
            );
        }
        
        self.substitute_variables(&self.template.session_template, &variables)
    }
}
```

**Template System**:
- **Variable substitution**: Context-aware message generation
- **Multiple templates**: Different formats for different commit types
- **Validation**: Ensures all placeholders are substituted
- **Fallback values**: Graceful handling of missing context

## ⚙️ Configuration System

**Location**: `src/config.rs`

Comprehensive configuration with sensible defaults:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub repository_path: PathBuf,
    pub default_branch: String,
    pub auto_push: AutoPushConfig,
    pub commit_template: CommitTemplate,
    pub branch_naming: BranchNamingConfig,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repository_path: PathBuf::from("."),
            default_branch: "master".to_string(),
            auto_push: AutoPushConfig::default(),
            commit_template: CommitTemplate::default(),
            branch_naming: BranchNamingConfig::default(),
        }
    }
}
```

**Configuration Categories**:
- **Repository settings**: Path, default branch
- **Auto-push behavior**: Threshold, counter file location
- **Commit templates**: Message formats for different contexts
- **Branch naming**: Timestamp usage, prefixes, length limits

## 🧪 Testing Strategy

**Location**: `src/lib.rs` (tests module)

Comprehensive unit tests covering key functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_branch_manager_validation() {
        let config = BranchNamingConfig::default();
        let manager = BranchManager::new(config);
        
        // Valid names
        assert!(manager.validate_branch_name("feature-test").is_ok());
        assert!(manager.validate_branch_name("20250822-1234-feature").is_ok());
        
        // Invalid names
        assert!(manager.validate_branch_name("").is_err());
        assert!(manager.validate_branch_name("master").is_err());
        assert!(manager.validate_branch_name("branch with spaces").is_err());
    }

    #[tokio::test]
    async fn test_push_counter() {
        let temp_dir = TempDir::new().unwrap();
        let counter_file = temp_dir.path().join("counter.txt");
        let counter = PushCounter::new(counter_file);
        
        // Initial state
        assert_eq!(counter.current_count().await.unwrap(), 0);
        
        // Increment
        assert_eq!(counter.increment().await.unwrap(), 1);
        assert_eq!(counter.increment().await.unwrap(), 2);
        
        // Reset
        counter.reset().await.unwrap();
        assert_eq!(counter.current_count().await.unwrap(), 0);
    }
}
```

**Test Coverage**:
- **Branch validation**: All validation rules tested
- **Push counter**: File persistence and operations
- **Message building**: Template substitution
- **Configuration**: Default values and serialization

## 🔧 Integration Points

### With bitacora-core

- **Error types**: Integrates with `BitacoraError` hierarchy
- **Domain models**: Uses `Session` and `Action` models
- **Utilities**: Leverages timestamp utilities

### With bitacora-storage

- **Repository state**: Can persist Git status in database
- **Session tracking**: Correlates Git operations with sessions
- **Audit trail**: All Git operations logged for analysis

### With bitacora-config

- **Dynamic configuration**: Runtime configuration updates
- **Environment-specific settings**: Different configs per environment
- **User preferences**: Per-user Git settings

## 🚀 Performance Characteristics

### Async Operations
- **Non-blocking**: All Git operations are async
- **Concurrent safe**: Multiple operations can run simultaneously
- **Resource efficient**: Uses tokio's async runtime

### Caching Strategy
- **Repository state**: Caches current branch, status
- **Command results**: Intelligent caching of expensive operations
- **Configuration**: Config loaded once, cached in memory

### Error Recovery
- **Graceful degradation**: Falls back to basic operations if advanced features fail
- **Retry logic**: Automatic retry for network operations
- **Clear error messages**: Actionable error information

## 📊 Monitoring and Observability

### Logging Integration
- **Structured logging**: Uses `tracing` crate for structured logs
- **Operation tracking**: Each Git operation logged with context
- **Performance metrics**: Command execution times tracked

### Health Monitoring
- **Repository health checks**: Validates Git repository state
- **Dependency checks**: Ensures Git is available and functional
- **Status reporting**: Rich status information for monitoring systems

## 🔄 Future Extensions

### Planned Enhancements
- **AI-powered commit messages**: Use LLM for intelligent message generation
- **Conflict resolution**: Automated merge conflict resolution
- **Branch cleanup**: Intelligent branch deletion and archiving
- **Team collaboration**: Branch sharing and synchronization features

### Integration Roadmap
- **CI/CD integration**: Hooks for continuous integration systems
- **Code review integration**: Automatic PR creation and management
- **Analytics**: Advanced Git analytics and reporting
- **Multi-repository support**: Manage multiple repositories from single service

---

*This technical documentation provides the complete implementation details for understanding, maintaining, and extending the Git Service in Bitacora.*
