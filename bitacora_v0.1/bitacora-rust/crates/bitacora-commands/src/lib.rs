//! Bitacora Commands - Command-line interface and command execution system
//! 
//! This crate provides the command-line interface and command execution engine
//! for Bitacora, integrating all other services into a cohesive user experience.

pub mod config;
pub mod errors;
pub mod executor;
pub mod handlers;
pub mod parser;
pub mod registry;

pub use config::CommandConfig;
pub use errors::CommandError;
pub use executor::{CommandExecutor, CommandHandler, ExecutionContext, ExecutionResult};
pub use parser::{BitacoraCommand, CommandParser, Commands, ParsedCommand};
pub use registry::{CommandRegistry, RegisterableCommandHandler, CommandMetadata};
