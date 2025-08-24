//! Command execution engine that runs parsed commands

use crate::{
    config::CommandConfig,
    errors::CommandError,
    parser::ParsedCommand,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Result of command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub command_id: Uuid,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Command execution context
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub session_id: Option<String>,
    pub project_id: Option<String>,
    pub workspace_path: String,
    pub user_config: HashMap<String, serde_json::Value>,
    pub environment: HashMap<String, String>,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            session_id: None,
            project_id: None,
            workspace_path: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            user_config: HashMap::new(),
            environment: HashMap::new(),
        }
    }
}

/// Trait for command handlers that execute specific commands
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// The command name this handler is responsible for
    fn command_name(&self) -> &str;
    
    /// Execute the command
    async fn execute(
        &self,
        command: &ParsedCommand,
        context: &ExecutionContext,
    ) -> Result<ExecutionResult, CommandError>;
    
    /// Validate that the command can be executed
    async fn validate(&self, command: &ParsedCommand) -> Result<(), CommandError> {
        // Default validation - just check that command matches
        if command.command != self.command_name() {
            return Err(CommandError::unsupported_command(&command.command));
        }
        Ok(())
    }
    
    /// Get help text for this command
    fn help_text(&self) -> String {
        format!("Help for {} command", self.command_name())
    }
}

/// Command execution engine
pub struct CommandExecutor {
    handlers: Arc<RwLock<HashMap<String, Arc<dyn CommandHandler>>>>,
    config: CommandConfig,
    active_executions: Arc<RwLock<HashMap<Uuid, Instant>>>,
}

impl CommandExecutor {
    pub fn new(config: CommandConfig) -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            config,
            active_executions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a command handler
    pub async fn register_handler(&self, handler: Arc<dyn CommandHandler>) -> Result<(), CommandError> {
        let command_name = handler.command_name().to_string();
        let mut handlers = self.handlers.write().await;
        
        if handlers.contains_key(&command_name) {
            return Err(CommandError::handler_registration(
                &format!("Handler for command '{}' already exists", command_name)
            ));
        }
        
        handlers.insert(command_name, handler);
        Ok(())
    }

    /// Unregister a command handler
    pub async fn unregister_handler(&self, command_name: &str) -> Result<(), CommandError> {
        let mut handlers = self.handlers.write().await;
        handlers.remove(command_name);
        Ok(())
    }

    /// Get list of registered commands
    pub async fn list_commands(&self) -> Vec<String> {
        let handlers = self.handlers.read().await;
        handlers.keys().cloned().collect()
    }

    /// Execute a parsed command
    pub async fn execute(
        &self,
        command: &ParsedCommand,
        context: Option<ExecutionContext>,
    ) -> Result<ExecutionResult, CommandError> {
        let command_id = Uuid::new_v4();
        let start_time = Instant::now();
        
        // Record active execution
        {
            let mut active = self.active_executions.write().await;
            active.insert(command_id, start_time);
        }

        // Set up timeout
        let timeout_duration = Duration::from_secs(self.config.default_timeout_seconds);
        
        // Execute with timeout
        let result = match tokio::time::timeout(
            timeout_duration,
            self.execute_internal(command, context.unwrap_or_default())
        ).await {
            Ok(result) => result,
            Err(_) => Err(CommandError::execution_timeout(&command.command, timeout_duration.as_secs())),
        };

        // Clean up active execution tracking
        {
            let mut active = self.active_executions.write().await;
            active.remove(&command_id);
        }

        // Convert result to ExecutionResult
        let duration = start_time.elapsed();
        match result {
            Ok(mut exec_result) => {
                exec_result.command_id = command_id;
                exec_result.duration = duration;
                Ok(exec_result)
            }
            Err(error) => Ok(ExecutionResult {
                command_id,
                success: false,
                output: String::new(),
                error: Some(error.to_string()),
                duration,
                timestamp: chrono::Utc::now(),
                metadata: HashMap::new(),
            }),
        }
    }

    async fn execute_internal(
        &self,
        command: &ParsedCommand,
        context: ExecutionContext,
    ) -> Result<ExecutionResult, CommandError> {
        let handlers = self.handlers.read().await;
        
        let handler = handlers.get(&command.command)
            .ok_or_else(|| CommandError::unsupported_command(&command.command))?;

        // Validate command before execution
        handler.validate(command).await?;

        // Execute the command
        handler.execute(command, &context).await
    }

    /// Cancel a running command by ID
    pub async fn cancel_execution(&self, command_id: Uuid) -> Result<(), CommandError> {
        let active = self.active_executions.read().await;
        if active.contains_key(&command_id) {
            // In a real implementation, we would need to track tokio task handles
            // and cancel them. For now, we just remove from tracking.
            drop(active);
            let mut active = self.active_executions.write().await;
            active.remove(&command_id);
            Ok(())
        } else {
            Err(CommandError::execution_error("Command not found or already completed"))
        }
    }

    /// Get currently active executions
    pub async fn get_active_executions(&self) -> Vec<(Uuid, Duration)> {
        let active = self.active_executions.read().await;
        let now = Instant::now();
        active.iter()
            .map(|(id, start_time)| (*id, now.duration_since(*start_time)))
            .collect()
    }

    /// Get help for a specific command
    pub async fn get_command_help(&self, command_name: &str) -> Result<String, CommandError> {
        let handlers = self.handlers.read().await;
        let handler = handlers.get(command_name)
            .ok_or_else(|| CommandError::unsupported_command(command_name))?;
        
        Ok(handler.help_text())
    }

    /// Get general help with list of all commands
    pub async fn get_general_help(&self) -> String {
        let handlers = self.handlers.read().await;
        let mut help = String::from("Available commands:\n\n");
        
        let mut commands: Vec<_> = handlers.keys().collect();
        commands.sort();
        
        for command in commands {
            if let Some(handler) = handlers.get(command) {
                help.push_str(&format!("  {}: {}\n", command, handler.help_text()));
            }
        }
        
        help.push_str("\nUse 'bitacora help <command>' for detailed help on a specific command.\n");
        help
    }
}

/// Mock command handler for testing
#[derive(Debug)]
pub struct MockCommandHandler {
    command_name: String,
    should_succeed: bool,
    execution_time: Duration,
}

impl MockCommandHandler {
    pub fn new(command_name: &str, should_succeed: bool) -> Self {
        Self {
            command_name: command_name.to_string(),
            should_succeed,
            execution_time: Duration::from_millis(100),
        }
    }

    pub fn with_execution_time(mut self, duration: Duration) -> Self {
        self.execution_time = duration;
        self
    }
}

#[async_trait]
impl CommandHandler for MockCommandHandler {
    fn command_name(&self) -> &str {
        &self.command_name
    }

    async fn execute(
        &self,
        command: &ParsedCommand,
        _context: &ExecutionContext,
    ) -> Result<ExecutionResult, CommandError> {
        // Simulate execution time
        tokio::time::sleep(self.execution_time).await;

        if self.should_succeed {
            Ok(ExecutionResult {
                command_id: Uuid::new_v4(), // Will be overridden by executor
                success: true,
                output: format!("Successfully executed '{}' command", command.command),
                error: None,
                duration: self.execution_time, // Will be overridden by executor
                timestamp: chrono::Utc::now(),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("mock".to_string(), serde_json::Value::Bool(true));
                    meta
                },
            })
        } else {
            Err(CommandError::execution_error("Mock command failed"))
        }
    }

    fn help_text(&self) -> String {
        format!("Mock handler for '{}' command (testing purposes)", self.command_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{CommandParser, ParsedCommand};
    use std::collections::HashMap;

    fn create_test_command() -> ParsedCommand {
        ParsedCommand {
            command: "test".to_string(),
            subcommand: None,
            args: HashMap::new(),
            flags: HashMap::new(),
            raw_input: "test".to_string(),
            timestamp: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_handler_registration() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let handler = Arc::new(MockCommandHandler::new("test", true));
        executor.register_handler(handler).await.unwrap();

        let commands = executor.list_commands().await;
        assert!(commands.contains(&"test".to_string()));
    }

    #[tokio::test]
    async fn test_command_execution_success() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let handler = Arc::new(MockCommandHandler::new("test", true));
        executor.register_handler(handler).await.unwrap();

        let command = create_test_command();
        let result = executor.execute(&command, None).await.unwrap();

        assert!(result.success);
        assert!(result.error.is_none());
        assert!(!result.output.is_empty());
    }

    #[tokio::test]
    async fn test_command_execution_failure() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let handler = Arc::new(MockCommandHandler::new("test", false));
        executor.register_handler(handler).await.unwrap();

        let command = create_test_command();
        let result = executor.execute(&command, None).await.unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_unsupported_command() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let command = create_test_command();
        let result = executor.execute(&command, None).await.unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
        let error_msg = result.error.unwrap();
        println!("Actual error message: '{}'", error_msg);
        assert!(error_msg.to_lowercase().contains("unsupported") || error_msg.to_lowercase().contains("handler") || error_msg.to_lowercase().contains("not found"));
    }

    #[tokio::test]
    async fn test_execution_timeout() {
        let mut config = CommandConfig::default();
        config.default_timeout_seconds = 1; // 1 second timeout
        let executor = CommandExecutor::new(config);

        // Handler that takes 2 seconds (longer than timeout)
        let handler = Arc::new(
            MockCommandHandler::new("test", true)
                .with_execution_time(Duration::from_secs(2))
        );
        executor.register_handler(handler).await.unwrap();

        let command = create_test_command();
        let result = executor.execute(&command, None).await.unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("timeout"));
    }

    #[tokio::test]
    async fn test_active_executions_tracking() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let handler = Arc::new(
            MockCommandHandler::new("test", true)
                .with_execution_time(Duration::from_millis(500))
        );
        executor.register_handler(handler).await.unwrap();

        let command = create_test_command();
        
        // Start execution in background
        let executor_clone1 = Arc::new(executor);
        let executor_clone2 = executor_clone1.clone();
        let command_clone = command.clone();
        let execution_handle = tokio::spawn(async move {
            executor_clone1.execute(&command_clone, None).await
        });

        // Give it a moment to start
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Check that it's tracked as active
        let active = executor_clone2.get_active_executions().await;
        assert_eq!(active.len(), 1);

        // Wait for completion
        let result = execution_handle.await.unwrap().unwrap();
        assert!(result.success);

        // Check that it's no longer active
        let active = executor_clone2.get_active_executions().await;
        assert_eq!(active.len(), 0);
    }

    #[tokio::test]
    async fn test_help_text() {
        let config = CommandConfig::default();
        let executor = CommandExecutor::new(config);

        let handler = Arc::new(MockCommandHandler::new("test", true));
        executor.register_handler(handler).await.unwrap();

        let help = executor.get_command_help("test").await.unwrap();
        assert!(help.contains("test"));

        let general_help = executor.get_general_help().await;
        assert!(general_help.contains("Available commands"));
        assert!(general_help.contains("test"));
    }
}
