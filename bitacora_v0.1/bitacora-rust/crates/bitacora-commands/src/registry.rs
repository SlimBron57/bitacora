//! Command registry system for managing and discovering available commands

use crate::{
    errors::CommandError,
    executor::{CommandHandler, CommandExecutor},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::RwLock;

/// Metadata about a registered command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub aliases: Vec<String>,
    pub examples: Vec<String>,
    pub arguments: Vec<ArgumentMetadata>,
    pub flags: Vec<FlagMetadata>,
    pub subcommands: Vec<String>,
}

/// Metadata about a command argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentMetadata {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub arg_type: ArgumentType,
    pub default_value: Option<String>,
    pub valid_values: Option<Vec<String>>,
}

/// Metadata about a command flag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagMetadata {
    pub name: String,
    pub short: Option<String>,
    pub description: String,
    pub flag_type: FlagType,
    pub default_value: Option<String>,
}

/// Types of command arguments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    String,
    Number,
    Boolean,
    Path,
    Url,
    Email,
    Date,
    Duration,
    Custom(String),
}

/// Types of command flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagType {
    Boolean,
    String,
    Number,
    Multiple(Box<FlagType>),
}

/// Enhanced command handler trait that provides metadata
#[async_trait]
pub trait RegisterableCommandHandler: CommandHandler {
    /// Get metadata about this command
    fn metadata(&self) -> CommandMetadata;
    
    /// Get command category for organization
    fn category(&self) -> &str {
        "general"
    }
    
    /// Get command aliases
    fn aliases(&self) -> Vec<String> {
        vec![]
    }
}

/// Command registry that manages all available commands
pub struct CommandRegistry {
    commands: Arc<RwLock<HashMap<String, Arc<dyn RegisterableCommandHandler>>>>,
    metadata: Arc<RwLock<HashMap<String, CommandMetadata>>>,
    categories: Arc<RwLock<HashMap<String, Vec<String>>>>,
    aliases: Arc<RwLock<HashMap<String, String>>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
            categories: Arc::new(RwLock::new(HashMap::new())),
            aliases: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a command handler with full metadata
    pub async fn register(&self, handler: Arc<dyn RegisterableCommandHandler>) -> Result<(), CommandError> {
        let command_name = handler.command_name().to_string();
        let metadata = handler.metadata();
        let category = handler.category().to_string();
        let aliases = handler.aliases();

        // Check for conflicts
        {
            let commands = self.commands.read().await;
            if commands.contains_key(&command_name) {
                return Err(CommandError::handler_registration(
                    &format!("Command '{}' is already registered", command_name)
                ));
            }
        }

        // Check alias conflicts
        {
            let existing_aliases = self.aliases.read().await;
            for alias in &aliases {
                if existing_aliases.contains_key(alias) {
                    return Err(CommandError::handler_registration(
                        &format!("Alias '{}' is already in use", alias)
                    ));
                }
            }
        }

        // Register command
        {
            let mut commands = self.commands.write().await;
            commands.insert(command_name.clone(), handler);
        }

        // Store metadata
        {
            let mut metadata_map = self.metadata.write().await;
            metadata_map.insert(command_name.clone(), metadata);
        }

        // Update categories
        {
            let mut categories = self.categories.write().await;
            categories.entry(category).or_insert_with(Vec::new).push(command_name.clone());
        }

        // Register aliases
        {
            let mut aliases_map = self.aliases.write().await;
            for alias in aliases {
                aliases_map.insert(alias, command_name.clone());
            }
        }

        Ok(())
    }

    /// Unregister a command
    pub async fn unregister(&self, command_name: &str) -> Result<(), CommandError> {
        let metadata = {
            let commands = self.commands.read().await;
            if !commands.contains_key(command_name) {
                return Err(CommandError::handler_registration(
                    &format!("Command '{}' is not registered", command_name)
                ));
            }
            
            let metadata_map = self.metadata.read().await;
            metadata_map.get(command_name).cloned()
        };

        if let Some(meta) = metadata {
            // Remove from commands
            {
                let mut commands = self.commands.write().await;
                commands.remove(command_name);
            }

            // Remove metadata
            {
                let mut metadata_map = self.metadata.write().await;
                metadata_map.remove(command_name);
            }

            // Remove from categories
            {
                let mut categories = self.categories.write().await;
                for command_list in categories.values_mut() {
                    command_list.retain(|cmd| cmd != command_name);
                }
            }

            // Remove aliases
            {
                let mut aliases_map = self.aliases.write().await;
                for alias in &meta.aliases {
                    aliases_map.remove(alias);
                }
            }
        }

        Ok(())
    }

    /// Get a command handler by name or alias
    pub async fn get_handler(&self, name_or_alias: &str) -> Option<Arc<dyn RegisterableCommandHandler>> {
        // First try direct lookup
        {
            let commands = self.commands.read().await;
            if let Some(handler) = commands.get(name_or_alias) {
                return Some(Arc::clone(handler));
            }
        }

        // Then try alias lookup
        {
            let aliases = self.aliases.read().await;
            if let Some(command_name) = aliases.get(name_or_alias) {
                let commands = self.commands.read().await;
                return commands.get(command_name).map(Arc::clone);
            }
        }

        None
    }

    /// Get metadata for a command
    pub async fn get_metadata(&self, command_name: &str) -> Option<CommandMetadata> {
        let metadata = self.metadata.read().await;
        metadata.get(command_name).cloned()
    }

    /// List all registered commands
    pub async fn list_commands(&self) -> Vec<String> {
        let commands = self.commands.read().await;
        commands.keys().cloned().collect()
    }

    /// List commands by category
    pub async fn list_by_category(&self, category: &str) -> Vec<String> {
        let categories = self.categories.read().await;
        categories.get(category).cloned().unwrap_or_else(Vec::new)
    }

    /// Get all available categories
    pub async fn get_categories(&self) -> Vec<String> {
        let categories = self.categories.read().await;
        categories.keys().cloned().collect()
    }

    /// Get all aliases and their target commands
    pub async fn get_aliases(&self) -> HashMap<String, String> {
        let aliases = self.aliases.read().await;
        aliases.clone()
    }

    /// Search commands by name or description
    pub async fn search(&self, query: &str) -> Vec<CommandMetadata> {
        let metadata = self.metadata.read().await;
        let query_lower = query.to_lowercase();
        
        metadata.values()
            .filter(|meta| {
                meta.name.to_lowercase().contains(&query_lower) ||
                meta.description.to_lowercase().contains(&query_lower) ||
                meta.aliases.iter().any(|alias| alias.to_lowercase().contains(&query_lower))
            })
            .cloned()
            .collect()
    }

    /// Register all handlers with a command executor
    pub async fn register_with_executor(&self, executor: &CommandExecutor) -> Result<(), CommandError> {
        let commands = self.commands.read().await;
        
        for handler in commands.values() {
            let base_handler: Arc<dyn CommandHandler> = handler.clone();
            executor.register_handler(base_handler).await?;
        }
        
        Ok(())
    }

    /// Generate completion data for shell completion
    pub async fn generate_completions(&self) -> CompletionData {
        let commands = self.commands.read().await;
        let metadata = self.metadata.read().await;
        let aliases = self.aliases.read().await;
        
        let mut completions = CompletionData {
            commands: Vec::new(),
            aliases: aliases.clone(),
            categories: HashMap::new(),
        };

        for command_name in commands.keys() {
            if let Some(meta) = metadata.get(command_name) {
                completions.commands.push(CommandCompletion {
                    name: meta.name.clone(),
                    description: meta.description.clone(),
                    subcommands: meta.subcommands.clone(),
                    arguments: meta.arguments.iter().map(|arg| arg.name.clone()).collect(),
                    flags: meta.flags.iter().map(|flag| {
                        if let Some(ref short) = flag.short {
                            vec![format!("--{}", flag.name), format!("-{}", short)]
                        } else {
                            vec![format!("--{}", flag.name)]
                        }
                    }).flatten().collect(),
                });
            }
        }

        let categories = self.categories.read().await;
        completions.categories = categories.clone();

        completions
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Data structure for shell completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionData {
    pub commands: Vec<CommandCompletion>,
    pub aliases: HashMap<String, String>,
    pub categories: HashMap<String, Vec<String>>,
}

/// Completion data for a single command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandCompletion {
    pub name: String,
    pub description: String,
    pub subcommands: Vec<String>,
    pub arguments: Vec<String>,
    pub flags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        executor::ExecutionResult,
        parser::ParsedCommand,
    };
    use std::collections::HashMap;

    /// Mock registerable command handler for testing
    #[derive(Debug)]
    struct MockRegisterableHandler {
        name: String,
        metadata: CommandMetadata,
    }

    impl MockRegisterableHandler {
        fn new(name: &str, category: &str) -> Self {
            Self {
                name: name.to_string(),
                metadata: CommandMetadata {
                    name: name.to_string(),
                    description: format!("Mock {} command", name),
                    category: category.to_string(),
                    version: "1.0.0".to_string(),
                    aliases: vec![format!("{}s", name)],
                    examples: vec![format!("bitacora {}", name)],
                    arguments: vec![],
                    flags: vec![],
                    subcommands: vec![],
                },
            }
        }
    }

    #[async_trait]
    impl CommandHandler for MockRegisterableHandler {
        fn command_name(&self) -> &str {
            &self.name
        }

        async fn execute(
            &self,
            _command: &ParsedCommand,
            _context: &crate::executor::ExecutionContext,
        ) -> Result<ExecutionResult, CommandError> {
            Ok(ExecutionResult {
                command_id: uuid::Uuid::new_v4(),
                success: true,
                output: format!("Executed {}", self.name),
                error: None,
                duration: std::time::Duration::from_millis(100),
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
            })
        }
    }

    #[async_trait]
    impl RegisterableCommandHandler for MockRegisterableHandler {
        fn metadata(&self) -> CommandMetadata {
            self.metadata.clone()
        }

        fn category(&self) -> &str {
            &self.metadata.category
        }

        fn aliases(&self) -> Vec<String> {
            self.metadata.aliases.clone()
        }
    }

    #[tokio::test]
    async fn test_command_registration() {
        let registry = CommandRegistry::new();
        let handler = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler).await.unwrap();

        let commands = registry.list_commands().await;
        assert!(commands.contains(&"test".to_string()));
        
        let metadata = registry.get_metadata("test").await;
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().name, "test");
    }

    #[tokio::test]
    async fn test_alias_registration() {
        let registry = CommandRegistry::new();
        let handler = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler.clone()).await.unwrap();

        let retrieved = registry.get_handler("tests").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().command_name(), "test");
    }

    #[tokio::test]
    async fn test_category_organization() {
        let registry = CommandRegistry::new();
        
        let handler1 = Arc::new(MockRegisterableHandler::new("build", "development"));
        let handler2 = Arc::new(MockRegisterableHandler::new("deploy", "deployment"));
        let handler3 = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler1).await.unwrap();
        registry.register(handler2).await.unwrap();
        registry.register(handler3).await.unwrap();

        let dev_commands = registry.list_by_category("development").await;
        assert_eq!(dev_commands.len(), 2);
        assert!(dev_commands.contains(&"build".to_string()));
        assert!(dev_commands.contains(&"test".to_string()));

        let deploy_commands = registry.list_by_category("deployment").await;
        assert_eq!(deploy_commands.len(), 1);
        assert!(deploy_commands.contains(&"deploy".to_string()));
    }

    #[tokio::test]
    async fn test_command_search() {
        let registry = CommandRegistry::new();
        let handler = Arc::new(MockRegisterableHandler::new("build", "development"));

        registry.register(handler).await.unwrap();

        let results = registry.search("build").await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "build");

        let results = registry.search("Mock").await;
        assert_eq!(results.len(), 1);

        let results = registry.search("nonexistent").await;
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_command_unregistration() {
        let registry = CommandRegistry::new();
        let handler = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler).await.unwrap();
        assert!(registry.get_handler("test").await.is_some());

        registry.unregister("test").await.unwrap();
        assert!(registry.get_handler("test").await.is_none());
    }

    #[tokio::test]
    async fn test_duplicate_registration_error() {
        let registry = CommandRegistry::new();
        let handler1 = Arc::new(MockRegisterableHandler::new("test", "development"));
        let handler2 = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler1).await.unwrap();
        let result = registry.register(handler2).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_completion_generation() {
        let registry = CommandRegistry::new();
        let handler = Arc::new(MockRegisterableHandler::new("test", "development"));

        registry.register(handler).await.unwrap();

        let completions = registry.generate_completions().await;
        assert_eq!(completions.commands.len(), 1);
        assert_eq!(completions.commands[0].name, "test");
        assert!(completions.aliases.contains_key("tests"));
    }
}
