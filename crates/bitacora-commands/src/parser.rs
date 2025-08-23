//! Command parsing system that converts string inputs to structured commands

use crate::errors::CommandError;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main command parser for Bitacora CLI
#[derive(Parser, Debug, Clone)]
#[command(name = "bitacora")]
#[command(about = "Bitacora Development Workflow Tracker")]
#[command(version = "0.1.0")]
pub struct BitacoraCommand {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,
    
    /// Output format (json, yaml, text)
    #[arg(short, long, global = true, default_value = "text")]
    pub format: String,
}

/// All available Bitacora commands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Session management commands
    Session {
        #[command(subcommand)]
        action: SessionCommands,
    },
    
    /// Git integration commands
    Git {
        #[command(subcommand)]
        action: GitCommands,
    },
    
    /// Template system commands
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
    
    /// Storage and data commands
    Storage {
        #[command(subcommand)]
        action: StorageCommands,
    },
    
    /// System status and information
    Status {
        /// Show detailed status
        #[arg(short, long)]
        detailed: bool,
        
        /// Show only specific component (session, git, storage, templates)
        #[arg(short, long)]
        component: Option<String>,
    },
    
    /// Quick session start (shorthand)
    Start {
        /// Session description
        description: String,
        
        /// Project ID to associate
        #[arg(short, long)]
        project: Option<String>,
    },
    
    /// Quick session stop (shorthand)
    Stop {
        /// Session summary
        summary: Option<String>,
    },
    
    /// Quick action logging (shorthand)
    Action {
        /// Action description
        description: String,
        
        /// Action type
        #[arg(short, long, default_value = "development")]
        action_type: String,
    },
    
    /// List recent sessions, actions, etc.
    List {
        /// What to list (sessions, actions, templates, projects)
        #[arg(default_value = "sessions")]
        what: String,
        
        /// Number of items to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    
    /// Configuration management
    Config {
        #[command(subcommand)]
        action: ConfigCommands,
    },
    
    /// Help and documentation
    Help {
        /// Command to get help for
        command: Option<String>,
    },
}

/// Session-specific commands
#[derive(Subcommand, Debug, Clone)]
pub enum SessionCommands {
    /// Create a new session
    Create {
        /// Session description
        description: String,
        
        /// Project ID to associate
        #[arg(short, long)]
        project: Option<String>,
    },
    
    /// Start an existing session
    Start {
        /// Session ID to start
        session_id: String,
        
        /// Additional context
        #[arg(short, long)]
        context: Option<String>,
    },
    
    /// Pause current session
    Pause {
        /// Session ID (optional, defaults to active)
        session_id: Option<String>,
    },
    
    /// Resume a paused session
    Resume {
        /// Session ID to resume
        session_id: String,
    },
    
    /// End a session
    End {
        /// Session ID (optional, defaults to active)
        session_id: Option<String>,
        
        /// Session summary
        #[arg(short, long)]
        summary: Option<String>,
    },
    
    /// Get session information
    Info {
        /// Session ID to get info for
        session_id: String,
    },
    
    /// List sessions
    List {
        /// Only show active sessions
        #[arg(short, long)]
        active: bool,
        
        /// Maximum number of sessions to show
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    
    /// Get session metrics
    Metrics,
}

/// Git integration commands
#[derive(Subcommand, Debug, Clone)]
pub enum GitCommands {
    /// Get repository status
    Status,
    
    /// Create a branch
    Branch {
        /// Branch name
        name: String,
        
        /// Base branch (defaults to current)
        #[arg(short, long)]
        base: Option<String>,
    },
    
    /// Commit changes
    Commit {
        /// Commit message
        message: String,
        
        /// Add all changes before committing
        #[arg(short, long)]
        add_all: bool,
    },
    
    /// Push changes
    Push {
        /// Remote name (defaults to origin)
        #[arg(short, long, default_value = "origin")]
        remote: String,
        
        /// Force push
        #[arg(short, long)]
        force: bool,
    },
    
    /// Pull changes
    Pull {
        /// Remote name (defaults to origin)
        #[arg(short, long, default_value = "origin")]
        remote: String,
    },
    
    /// Tag a commit
    Tag {
        /// Tag name
        name: String,
        
        /// Tag message
        #[arg(short, long)]
        message: Option<String>,
    },
}

/// Template system commands
#[derive(Subcommand, Debug, Clone)]
pub enum TemplateCommands {
    /// List available templates
    List {
        /// Template category to filter by
        #[arg(short, long)]
        category: Option<String>,
    },
    
    /// Apply a template
    Apply {
        /// Template name
        template: String,
        
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: String,
        
        /// Template variables (key=value format)
        #[arg(short, long)]
        vars: Vec<String>,
    },
    
    /// Create a new template
    Create {
        /// Template name
        name: String,
        
        /// Template description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Template category
        #[arg(short, long)]
        category: Option<String>,
    },
    
    /// Validate a template
    Validate {
        /// Template name or path
        template: String,
    },
}

/// Storage commands
#[derive(Subcommand, Debug, Clone)]
pub enum StorageCommands {
    /// Test storage connection
    Test,
    
    /// Backup data
    Backup {
        /// Backup destination
        #[arg(short, long)]
        destination: Option<String>,
    },
    
    /// Restore from backup
    Restore {
        /// Backup source
        source: String,
    },
    
    /// Clean old data
    Clean {
        /// Days to keep (older will be deleted)
        #[arg(short, long, default_value = "30")]
        keep_days: u32,
        
        /// Dry run (don't actually delete)
        #[arg(short, long)]
        dry_run: bool,
    },
}

/// Configuration commands
#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Specific config section to show
        section: Option<String>,
    },
    
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        
        /// Configuration value
        value: String,
    },
    
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    
    /// Reset configuration to defaults
    Reset {
        /// Confirm the reset
        #[arg(short, long)]
        confirm: bool,
    },
}

/// Parsed command with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCommand {
    pub command: String,
    pub subcommand: Option<String>,
    pub args: HashMap<String, serde_json::Value>,
    pub flags: HashMap<String, bool>,
    pub raw_input: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Command parser that handles string input and converts to structured commands
#[derive(Debug)]
pub struct CommandParser {
    aliases: HashMap<String, String>,
}

impl CommandParser {
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }

    pub fn with_aliases(mut self, aliases: HashMap<String, String>) -> Self {
        self.aliases = aliases;
        self
    }

    /// Parse a command string into a structured ParsedCommand
    pub fn parse(&self, input: &str) -> Result<ParsedCommand, CommandError> {
        let input = input.trim();
        
        if input.is_empty() {
            return Err(CommandError::invalid_syntax("Empty command"));
        }

        // Apply aliases
        let processed_input = self.apply_aliases(input);
        
        // Split the command using shlex for proper quote handling
        let args = shlex::split(&processed_input)
            .ok_or_else(|| CommandError::invalid_syntax("Invalid command syntax"))?;
            
        if args.is_empty() {
            return Err(CommandError::invalid_syntax("No command found"));
        }

        // Try to parse with clap
        let parsed = match BitacoraCommand::try_parse_from(&args) {
            Ok(cmd) => cmd,
            Err(e) => {
                return Err(CommandError::invalid_syntax(&format!("Parse error: {}", e)));
            }
        };

        // Convert to our internal representation
        self.convert_to_parsed_command(&parsed, input)
    }

    fn apply_aliases(&self, input: &str) -> String {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some(first) = parts.first() {
            if let Some(alias_target) = self.aliases.get(*first) {
                let mut result = alias_target.clone();
                if parts.len() > 1 {
                    result.push(' ');
                    result.push_str(&parts[1..].join(" "));
                }
                return result;
            }
        }
        input.to_string()
    }

    fn convert_to_parsed_command(&self, cmd: &BitacoraCommand, raw_input: &str) -> Result<ParsedCommand, CommandError> {
        let mut args = HashMap::new();
        let mut flags = HashMap::new();
        
        // Extract global flags
        flags.insert("verbose".to_string(), cmd.verbose);
        
        if let Some(ref config) = cmd.config {
            args.insert("config".to_string(), serde_json::Value::String(config.clone()));
        }
        
        args.insert("format".to_string(), serde_json::Value::String(cmd.format.clone()));

        let (command, subcommand) = match &cmd.command {
            Commands::Session { action } => {
                let (sub, sub_args) = self.extract_session_command_info(action);
                args.extend(sub_args);
                ("session".to_string(), Some(sub))
            }
            Commands::Git { action } => {
                let (sub, sub_args) = self.extract_git_command_info(action);
                args.extend(sub_args);
                ("git".to_string(), Some(sub))
            }
            Commands::Template { action } => {
                let (sub, sub_args) = self.extract_template_command_info(action);
                args.extend(sub_args);
                ("template".to_string(), Some(sub))
            }
            Commands::Storage { action } => {
                let (sub, sub_args) = self.extract_storage_command_info(action);
                args.extend(sub_args);
                ("storage".to_string(), Some(sub))
            }
            Commands::Status { detailed, component } => {
                flags.insert("detailed".to_string(), *detailed);
                if let Some(ref comp) = component {
                    args.insert("component".to_string(), serde_json::Value::String(comp.clone()));
                }
                ("status".to_string(), None)
            }
            Commands::Start { description, project } => {
                args.insert("description".to_string(), serde_json::Value::String(description.clone()));
                if let Some(ref proj) = project {
                    args.insert("project".to_string(), serde_json::Value::String(proj.clone()));
                }
                ("start".to_string(), None)
            }
            Commands::Stop { summary } => {
                if let Some(ref summ) = summary {
                    args.insert("summary".to_string(), serde_json::Value::String(summ.clone()));
                }
                ("stop".to_string(), None)
            }
            Commands::Action { description, action_type } => {
                args.insert("description".to_string(), serde_json::Value::String(description.clone()));
                args.insert("type".to_string(), serde_json::Value::String(action_type.clone()));
                ("action".to_string(), None)
            }
            Commands::List { what, limit } => {
                args.insert("what".to_string(), serde_json::Value::String(what.clone()));
                args.insert("limit".to_string(), serde_json::Value::Number((*limit).into()));
                ("list".to_string(), None)
            }
            Commands::Config { action } => {
                let (sub, sub_args) = self.extract_config_command_info(action);
                args.extend(sub_args);
                ("config".to_string(), Some(sub))
            }
            Commands::Help { command } => {
                if let Some(ref cmd) = command {
                    args.insert("command".to_string(), serde_json::Value::String(cmd.clone()));
                }
                ("help".to_string(), None)
            }
        };

        Ok(ParsedCommand {
            command,
            subcommand,
            args,
            flags,
            raw_input: raw_input.to_string(),
            timestamp: chrono::Utc::now(),
        })
    }

    fn extract_session_command_info(&self, action: &SessionCommands) -> (String, HashMap<String, serde_json::Value>) {
        let mut args = HashMap::new();
        
        let subcommand = match action {
            SessionCommands::Create { description, project } => {
                args.insert("description".to_string(), serde_json::Value::String(description.clone()));
                if let Some(ref proj) = project {
                    args.insert("project".to_string(), serde_json::Value::String(proj.clone()));
                }
                "create"
            }
            SessionCommands::Start { session_id, context } => {
                args.insert("session_id".to_string(), serde_json::Value::String(session_id.clone()));
                if let Some(ref ctx) = context {
                    args.insert("context".to_string(), serde_json::Value::String(ctx.clone()));
                }
                "start"
            }
            SessionCommands::Pause { session_id } => {
                if let Some(ref id) = session_id {
                    args.insert("session_id".to_string(), serde_json::Value::String(id.clone()));
                }
                "pause"
            }
            SessionCommands::Resume { session_id } => {
                args.insert("session_id".to_string(), serde_json::Value::String(session_id.clone()));
                "resume"
            }
            SessionCommands::End { session_id, summary } => {
                if let Some(ref id) = session_id {
                    args.insert("session_id".to_string(), serde_json::Value::String(id.clone()));
                }
                if let Some(ref summ) = summary {
                    args.insert("summary".to_string(), serde_json::Value::String(summ.clone()));
                }
                "end"
            }
            SessionCommands::Info { session_id } => {
                args.insert("session_id".to_string(), serde_json::Value::String(session_id.clone()));
                "info"
            }
            SessionCommands::List { active, limit } => {
                args.insert("active".to_string(), serde_json::Value::Bool(*active));
                args.insert("limit".to_string(), serde_json::Value::Number((*limit).into()));
                "list"
            }
            SessionCommands::Metrics => "metrics",
        };
        
        (subcommand.to_string(), args)
    }

    fn extract_git_command_info(&self, action: &GitCommands) -> (String, HashMap<String, serde_json::Value>) {
        let mut args = HashMap::new();
        
        let subcommand = match action {
            GitCommands::Status => "status",
            GitCommands::Branch { name, base } => {
                args.insert("name".to_string(), serde_json::Value::String(name.clone()));
                if let Some(ref b) = base {
                    args.insert("base".to_string(), serde_json::Value::String(b.clone()));
                }
                "branch"
            }
            GitCommands::Commit { message, add_all } => {
                args.insert("message".to_string(), serde_json::Value::String(message.clone()));
                args.insert("add_all".to_string(), serde_json::Value::Bool(*add_all));
                "commit"
            }
            GitCommands::Push { remote, force } => {
                args.insert("remote".to_string(), serde_json::Value::String(remote.clone()));
                args.insert("force".to_string(), serde_json::Value::Bool(*force));
                "push"
            }
            GitCommands::Pull { remote } => {
                args.insert("remote".to_string(), serde_json::Value::String(remote.clone()));
                "pull"
            }
            GitCommands::Tag { name, message } => {
                args.insert("name".to_string(), serde_json::Value::String(name.clone()));
                if let Some(ref msg) = message {
                    args.insert("message".to_string(), serde_json::Value::String(msg.clone()));
                }
                "tag"
            }
        };
        
        (subcommand.to_string(), args)
    }

    fn extract_template_command_info(&self, action: &TemplateCommands) -> (String, HashMap<String, serde_json::Value>) {
        let mut args = HashMap::new();
        
        let subcommand = match action {
            TemplateCommands::List { category } => {
                if let Some(ref cat) = category {
                    args.insert("category".to_string(), serde_json::Value::String(cat.clone()));
                }
                "list"
            }
            TemplateCommands::Apply { template, output, vars } => {
                args.insert("template".to_string(), serde_json::Value::String(template.clone()));
                args.insert("output".to_string(), serde_json::Value::String(output.clone()));
                if !vars.is_empty() {
                    let vars_array: Vec<serde_json::Value> = vars.iter()
                        .map(|v| serde_json::Value::String(v.clone()))
                        .collect();
                    args.insert("vars".to_string(), serde_json::Value::Array(vars_array));
                }
                "apply"
            }
            TemplateCommands::Create { name, description, category } => {
                args.insert("name".to_string(), serde_json::Value::String(name.clone()));
                if let Some(ref desc) = description {
                    args.insert("description".to_string(), serde_json::Value::String(desc.clone()));
                }
                if let Some(ref cat) = category {
                    args.insert("category".to_string(), serde_json::Value::String(cat.clone()));
                }
                "create"
            }
            TemplateCommands::Validate { template } => {
                args.insert("template".to_string(), serde_json::Value::String(template.clone()));
                "validate"
            }
        };
        
        (subcommand.to_string(), args)
    }

    fn extract_storage_command_info(&self, action: &StorageCommands) -> (String, HashMap<String, serde_json::Value>) {
        let mut args = HashMap::new();
        
        let subcommand = match action {
            StorageCommands::Test => "test",
            StorageCommands::Backup { destination } => {
                if let Some(ref dest) = destination {
                    args.insert("destination".to_string(), serde_json::Value::String(dest.clone()));
                }
                "backup"
            }
            StorageCommands::Restore { source } => {
                args.insert("source".to_string(), serde_json::Value::String(source.clone()));
                "restore"
            }
            StorageCommands::Clean { keep_days, dry_run } => {
                args.insert("keep_days".to_string(), serde_json::Value::Number((*keep_days).into()));
                args.insert("dry_run".to_string(), serde_json::Value::Bool(*dry_run));
                "clean"
            }
        };
        
        (subcommand.to_string(), args)
    }

    fn extract_config_command_info(&self, action: &ConfigCommands) -> (String, HashMap<String, serde_json::Value>) {
        let mut args = HashMap::new();
        
        let subcommand = match action {
            ConfigCommands::Show { section } => {
                if let Some(ref sect) = section {
                    args.insert("section".to_string(), serde_json::Value::String(sect.clone()));
                }
                "show"
            }
            ConfigCommands::Set { key, value } => {
                args.insert("key".to_string(), serde_json::Value::String(key.clone()));
                args.insert("value".to_string(), serde_json::Value::String(value.clone()));
                "set"
            }
            ConfigCommands::Get { key } => {
                args.insert("key".to_string(), serde_json::Value::String(key.clone()));
                "get"
            }
            ConfigCommands::Reset { confirm } => {
                args.insert("confirm".to_string(), serde_json::Value::Bool(*confirm));
                "reset"
            }
        };
        
        (subcommand.to_string(), args)
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}
