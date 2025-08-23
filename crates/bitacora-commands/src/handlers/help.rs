//! Help command handler

use crate::{
    errors::CommandError,
    executor::{CommandHandler, ExecutionContext, ExecutionResult},
    parser::ParsedCommand,
    registry::{CommandMetadata, RegisterableCommandHandler},
};
use async_trait::async_trait;
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Handler for help commands
#[derive(Debug)]
pub struct HelpHandler {
    // Store references to all registered commands for help generation
    commands: Arc<RwLock<HashMap<String, CommandMetadata>>>,
}

impl HelpHandler {
    pub fn new() -> Self {
        Self {
            commands: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register command metadata for help generation
    pub async fn register_command_metadata(&self, metadata: CommandMetadata) {
        let mut commands = self.commands.write().await;
        commands.insert(metadata.name.clone(), metadata);
    }

    /// Generate general help text
    async fn generate_general_help(&self) -> String {
        let commands = self.commands.read().await;
        let mut help = String::from("Bitacora - Development Workflow Tracker\n\n");
        help.push_str("USAGE:\n");
        help.push_str("    bitacora [OPTIONS] <COMMAND>\n\n");
        help.push_str("OPTIONS:\n");
        help.push_str("    -v, --verbose        Enable verbose output\n");
        help.push_str("    -c, --config <FILE>  Configuration file path\n");
        help.push_str("    -f, --format <FMT>   Output format (text, json, yaml)\n");
        help.push_str("    -h, --help           Print help information\n");
        help.push_str("    -V, --version        Print version information\n\n");
        help.push_str("COMMANDS:\n");

        // Group commands by category
        let mut categories: HashMap<String, Vec<&CommandMetadata>> = HashMap::new();
        for metadata in commands.values() {
            categories.entry(metadata.category.clone())
                .or_insert_with(Vec::new)
                .push(metadata);
        }

        // Sort categories and commands within each category
        let mut sorted_categories: Vec<_> = categories.into_iter().collect();
        sorted_categories.sort_by(|a, b| a.0.cmp(&b.0));

        for (category, mut commands_in_category) in sorted_categories {
            help.push_str(&format!("  {}:\n", category.to_uppercase()));
            commands_in_category.sort_by(|a, b| a.name.cmp(&b.name));

            for cmd in commands_in_category {
                help.push_str(&format!("    {:12} {}\n", cmd.name, cmd.description));
            }
            help.push('\n');
        }

        help.push_str("Use 'bitacora help <command>' for more information on a specific command.\n");
        help
    }

    /// Generate help for a specific command
    async fn generate_command_help(&self, command_name: &str) -> Result<String, CommandError> {
        let commands = self.commands.read().await;
        let metadata = commands.get(command_name)
            .ok_or_else(|| CommandError::unsupported_command(command_name))?;

        let mut help = format!("bitacora-{} - {}\n\n", metadata.name, metadata.description);
        
        // Usage section
        help.push_str("USAGE:\n");
        if metadata.subcommands.is_empty() {
            help.push_str(&format!("    bitacora {} [OPTIONS]", metadata.name));
            if !metadata.arguments.is_empty() {
                for arg in &metadata.arguments {
                    if arg.required {
                        help.push_str(&format!(" <{}>", arg.name.to_uppercase()));
                    } else {
                        help.push_str(&format!(" [{}]", arg.name.to_uppercase()));
                    }
                }
            }
            help.push_str("\n\n");
        } else {
            help.push_str(&format!("    bitacora {} [OPTIONS] <SUBCOMMAND>\n\n", metadata.name));
        }

        // Arguments section
        if !metadata.arguments.is_empty() {
            help.push_str("ARGS:\n");
            for arg in &metadata.arguments {
                let required_str = if arg.required { " (required)" } else { "" };
                help.push_str(&format!("    {:12} {}{}\n", 
                    format!("<{}>", arg.name.to_uppercase()), 
                    arg.description,
                    required_str
                ));
            }
            help.push('\n');
        }

        // Options/Flags section
        if !metadata.flags.is_empty() {
            help.push_str("OPTIONS:\n");
            for flag in &metadata.flags {
                let flag_name = if let Some(ref short) = flag.short {
                    format!("-{}, --{}", short, flag.name)
                } else {
                    format!("    --{}", flag.name)
                };
                help.push_str(&format!("    {:20} {}\n", flag_name, flag.description));
            }
            help.push('\n');
        }

        // Subcommands section
        if !metadata.subcommands.is_empty() {
            help.push_str("SUBCOMMANDS:\n");
            for subcmd in &metadata.subcommands {
                help.push_str(&format!("    {:12} {}\n", subcmd, 
                    format!("{} {}", metadata.name, subcmd)));
            }
            help.push('\n');
        }

        // Examples section
        if !metadata.examples.is_empty() {
            help.push_str("EXAMPLES:\n");
            for example in &metadata.examples {
                help.push_str(&format!("    {}\n", example));
            }
            help.push('\n');
        }

        // Aliases section
        if !metadata.aliases.is_empty() {
            help.push_str("ALIASES:\n");
            help.push_str(&format!("    {}\n\n", metadata.aliases.join(", ")));
        }

        Ok(help)
    }
}

impl Default for HelpHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandHandler for HelpHandler {
    fn command_name(&self) -> &str {
        "help"
    }

    async fn execute(
        &self,
        command: &ParsedCommand,
        _context: &ExecutionContext,
    ) -> Result<ExecutionResult, CommandError> {
        let help_text = if let Some(target_command) = command.args.get("command").and_then(|v| v.as_str()) {
            self.generate_command_help(target_command).await?
        } else {
            self.generate_general_help().await
        };

        Ok(ExecutionResult {
            command_id: Uuid::new_v4(),
            success: true,
            output: help_text,
            error: None,
            duration: std::time::Duration::from_millis(10),
            timestamp: chrono::Utc::now(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("action".to_string(), json!("help"));
                if let Some(cmd) = command.args.get("command").and_then(|v| v.as_str()) {
                    meta.insert("target_command".to_string(), json!(cmd));
                }
                meta
            },
        })
    }

    fn help_text(&self) -> String {
        "Display help information about Bitacora commands".to_string()
    }
}

#[async_trait]
impl RegisterableCommandHandler for HelpHandler {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "help".to_string(),
            description: "Display help information".to_string(),
            category: "system".to_string(),
            version: "1.0.0".to_string(),
            aliases: vec!["h".to_string(), "man".to_string()],
            examples: vec![
                "bitacora help".to_string(),
                "bitacora help session".to_string(),
                "bitacora help git".to_string(),
                "bitacora h status".to_string(),
            ],
            arguments: vec![],
            flags: vec![],
            subcommands: vec![],
        }
    }

    fn category(&self) -> &str {
        "system"
    }

    fn aliases(&self) -> Vec<String> {
        vec!["h".to_string(), "man".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metadata() -> CommandMetadata {
        CommandMetadata {
            name: "test".to_string(),
            description: "Test command".to_string(),
            category: "testing".to_string(),
            version: "1.0.0".to_string(),
            aliases: vec!["t".to_string()],
            examples: vec!["bitacora test".to_string()],
            arguments: vec![],
            flags: vec![],
            subcommands: vec!["run".to_string(), "clean".to_string()],
        }
    }

    fn create_help_command(target: Option<&str>) -> ParsedCommand {
        let mut args = HashMap::new();
        if let Some(cmd) = target {
            args.insert("command".to_string(), json!(cmd));
        }

        ParsedCommand {
            command: "help".to_string(),
            subcommand: None,
            args,
            flags: HashMap::new(),
            raw_input: if let Some(cmd) = target {
                format!("help {}", cmd)
            } else {
                "help".to_string()
            },
            timestamp: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_general_help() {
        let handler = HelpHandler::new();
        handler.register_command_metadata(create_test_metadata()).await;

        let command = create_help_command(None);
        let result = handler.execute(&command, &ExecutionContext::default()).await.unwrap();

        assert!(result.success);
        assert!(result.output.contains("Bitacora - Development Workflow Tracker"));
        assert!(result.output.contains("test"));
        assert!(result.output.contains("TESTING"));
    }

    #[tokio::test]
    async fn test_command_specific_help() {
        let handler = HelpHandler::new();
        handler.register_command_metadata(create_test_metadata()).await;

        let command = create_help_command(Some("test"));
        let result = handler.execute(&command, &ExecutionContext::default()).await.unwrap();

        assert!(result.success);
        assert!(result.output.contains("bitacora-test"));
        assert!(result.output.contains("Test command"));
        assert!(result.output.contains("run"));
        assert!(result.output.contains("clean"));
    }

    #[tokio::test]
    async fn test_help_for_nonexistent_command() {
        let handler = HelpHandler::new();

        let command = create_help_command(Some("nonexistent"));
        let result = handler.execute(&command, &ExecutionContext::default()).await;

        assert!(result.is_err());
    }
}
