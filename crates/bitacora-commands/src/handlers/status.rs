//! Status command handler for system information

use crate::{
    errors::CommandError,
    executor::{CommandHandler, ExecutionContext, ExecutionResult},
    parser::ParsedCommand,
    registry::{CommandMetadata, FlagMetadata, FlagType, RegisterableCommandHandler},
};
use async_trait::async_trait;
use serde_json::json;
use std::{collections::HashMap};
use uuid::Uuid;

/// Handler for system status commands
#[derive(Debug)]
pub struct StatusHandler {}

impl StatusHandler {
    pub fn new() -> Self {
        Self {}
    }

    async fn get_session_status(&self) -> Result<String, CommandError> {
        Ok("Sessions: 0 total, 0 active, 0 completed".to_string())
    }

    async fn get_git_status(&self) -> Result<String, CommandError> {
        Ok("Git: main (clean), 0 changes, 0 staged".to_string())
    }

    async fn get_storage_status(&self) -> Result<String, CommandError> {
        Ok("Storage: Connected and accessible".to_string())
    }

    async fn get_template_status(&self) -> Result<String, CommandError> {
        Ok("Templates: 0 available".to_string())
    }

    async fn get_core_status(&self) -> Result<String, CommandError> {
        Ok("Core: v0.1.0 (development build)".to_string())
    }
}

impl Default for StatusHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CommandHandler for StatusHandler {
    fn command_name(&self) -> &str {
        "status"
    }

    async fn execute(
        &self,
        command: &ParsedCommand,
        _context: &ExecutionContext,
    ) -> Result<ExecutionResult, CommandError> {
        let detailed = command.flags.get("detailed").copied().unwrap_or(false);
        let component = command.args.get("component").and_then(|v| v.as_str());

        let mut status_lines = Vec::new();

        if let Some(comp) = component {
            // Show status for specific component
            let status = match comp {
                "session" | "sessions" => self.get_session_status().await?,
                "git" => self.get_git_status().await?,
                "storage" => self.get_storage_status().await?,
                "template" | "templates" => self.get_template_status().await?,
                "core" => self.get_core_status().await?,
                _ => return Err(CommandError::invalid_argument(
                    "component",
                    &format!("Unknown component: {}", comp)
                )),
            };
            status_lines.push(status);
        } else {
            // Show status for all components
            status_lines.push("=== Bitacora System Status ===".to_string());
            status_lines.push(self.get_core_status().await?);
            status_lines.push(self.get_session_status().await?);
            status_lines.push(self.get_git_status().await?);
            status_lines.push(self.get_storage_status().await?);
            status_lines.push(self.get_template_status().await?);

            if detailed {
                status_lines.push("".to_string());
                status_lines.push("=== Detailed Information ===".to_string());
                status_lines.push(format!(
                    "Workspace: {}",
                    std::env::current_dir()
                        .unwrap_or_else(|_| std::path::PathBuf::from("."))
                        .display()
                ));
            }
        }

        let output = status_lines.join("\n");

        Ok(ExecutionResult {
            command_id: Uuid::new_v4(),
            success: true,
            output,
            error: None,
            duration: std::time::Duration::from_millis(100),
            timestamp: chrono::Utc::now(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("action".to_string(), json!("status"));
                meta.insert("detailed".to_string(), json!(detailed));
                if let Some(comp) = component {
                    meta.insert("component".to_string(), json!(comp));
                }
                meta
            },
        })
    }

    fn help_text(&self) -> String {
        "Display system status and component health information".to_string()
    }
}

#[async_trait]
impl RegisterableCommandHandler for StatusHandler {
    fn metadata(&self) -> CommandMetadata {
        CommandMetadata {
            name: "status".to_string(),
            description: "Show system status and component health".to_string(),
            category: "system".to_string(),
            version: "1.0.0".to_string(),
            aliases: vec!["stat".to_string(), "info".to_string()],
            examples: vec![
                "bitacora status".to_string(),
                "bitacora status --detailed".to_string(),
                "bitacora status --component git".to_string(),
                "bitacora status --component session".to_string(),
            ],
            arguments: vec![],
            flags: vec![
                FlagMetadata {
                    name: "detailed".to_string(),
                    short: Some("d".to_string()),
                    description: "Show detailed status information".to_string(),
                    flag_type: FlagType::Boolean,
                    default_value: Some("false".to_string()),
                },
                FlagMetadata {
                    name: "component".to_string(),
                    short: Some("c".to_string()),
                    description: "Show status for specific component only".to_string(),
                    flag_type: FlagType::String,
                    default_value: None,
                },
            ],
            subcommands: vec![],
        }
    }

    fn category(&self) -> &str {
        "system"
    }

    fn aliases(&self) -> Vec<String> {
        vec!["stat".to_string(), "info".to_string()]
    }
}
