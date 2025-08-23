//! Command errors for the Bitacora Commands system

use thiserror::Error;

/// Errors that can occur during command operations
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Invalid command syntax: {0}")]
    InvalidSyntax(String),

    #[error("Missing required argument: {0}")]
    MissingArgument(String),

    #[error("Invalid argument value: {0}")]
    InvalidArgument(String),

    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Permission denied for command: {0}")]
    PermissionDenied(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Git service error: {0}")]
    GitError(String),

    #[error("Session service error: {0}")]
    SessionError(String),

    #[error("Storage service error: {0}")]
    StorageError(String),

    #[error("Template service error: {0}")]
    TemplateError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Registry error: {0}")]
    RegistryError(String),
}

impl CommandError {
    pub fn command_not_found(cmd: &str) -> Self {
        Self::CommandNotFound(cmd.to_string())
    }

    pub fn invalid_syntax(msg: &str) -> Self {
        Self::InvalidSyntax(msg.to_string())
    }

    pub fn missing_argument(arg: &str) -> Self {
        Self::MissingArgument(arg.to_string())
    }

    pub fn execution_failed(msg: &str) -> Self {
        CommandError::ExecutionFailed(msg.to_string())
    }
    
    pub fn execution_error(msg: &str) -> Self {
        CommandError::ExecutionFailed(msg.to_string())
    }
    
    pub fn execution_timeout(command: &str, timeout_secs: u64) -> Self {
        CommandError::ExecutionFailed(format!("Command '{}' timed out after {}s", command, timeout_secs))
    }
    
    pub fn unsupported_command(cmd: &str) -> Self {
        CommandError::CommandNotFound(cmd.to_string())
    }
    
    pub fn unsupported_subcommand(cmd: &str, subcmd: &str) -> Self {
        CommandError::InvalidSyntax(format!("Unsupported subcommand '{}' for command '{}'", subcmd, cmd))
    }
    
    pub fn missing_subcommand(cmd: &str) -> Self {
        CommandError::MissingArgument(format!("Missing subcommand for '{}'", cmd))
    }
    
    pub fn handler_registration(message: &str) -> Self {
        CommandError::ExecutionFailed(message.to_string())
    }
    
    pub fn invalid_argument(arg: &str, msg: &str) -> Self {
        CommandError::InvalidArgument(format!("Invalid argument '{}': {}", arg, msg))
    }

    pub fn service_unavailable(service: &str) -> Self {
        Self::ServiceUnavailable(service.to_string())
    }
}
