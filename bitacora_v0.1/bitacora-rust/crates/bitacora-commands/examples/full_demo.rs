//! Comprehensive example demonstrating the Bitacora commands system
//! 
//! This example shows how to:
//! 1. Set up all the services
//! 2. Initialize the command registry and executor
//! 3. Register all command handlers
//! 4. Parse and execute various commands
//! 5. Handle different output formats
//! 6. Demonstrate the complete workflow

use bitacora_commands::{
    config::CommandConfig,
    executor::{CommandExecutor, ExecutionContext},
    handlers::{ConfigHandler, HelpHandler, SessionHandler, StatusHandler, GitHandler, TemplateHandler, StorageHandler},
    parser::CommandParser,
    registry::CommandRegistry,
    CommandError,
};

// Mock services for demonstration
use bitacora_core::{BitacoraCore, SystemInfo};
use bitacora_git::{GitService, GitServiceImpl};
use bitacora_session::{SessionService, SessionServiceImpl};
use bitacora_storage::{StorageService, StorageServiceImpl};
use bitacora_templates::{TemplateService, TemplateServiceImpl};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bitacora Commands System Demo");
    println!("==================================\n");

    // Step 1: Initialize all services
    println!("📦 Initializing services...");
    let core_service = Arc::new(BitacoraCore::new().await?);
    let git_service = Arc::new(GitServiceImpl::new(".".to_string()).await?) as Arc<dyn GitService>;
    let session_service = Arc::new(SessionServiceImpl::new()) as Arc<dyn SessionService>;
    let storage_service = Arc::new(StorageServiceImpl::new().await?) as Arc<dyn StorageService>;
    let template_service = Arc::new(TemplateServiceImpl::new().await?) as Arc<dyn TemplateService>;
    println!("✅ Services initialized\n");

    // Step 2: Set up command system
    println!("⚙️ Setting up command system...");
    let config = CommandConfig::default();
    let registry = Arc::new(CommandRegistry::new());
    let executor = Arc::new(CommandExecutor::new(config.clone()));
    let parser = CommandParser::new();
    println!("✅ Command system ready\n");

    // Step 3: Register all command handlers
    println!("📝 Registering command handlers...");
    
    // Session handler (fully implemented)
    let session_handler = Arc::new(SessionHandler::new(Arc::clone(&session_service)));
    registry.register(Arc::clone(&session_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    // Status handler (fully implemented)
    let status_handler = Arc::new(StatusHandler::new(
        Arc::clone(&core_service),
        Arc::clone(&git_service),
        Arc::clone(&session_service),
        Arc::clone(&storage_service),
        Arc::clone(&template_service),
    ));
    registry.register(Arc::clone(&status_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    // Help handler (fully implemented)
    let help_handler = Arc::new(HelpHandler::new());
    registry.register(Arc::clone(&help_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    // Register placeholder handlers
    let git_handler = Arc::new(GitHandler::new(Arc::clone(&git_service)));
    registry.register(Arc::clone(&git_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    let template_handler = Arc::new(TemplateHandler::new(Arc::clone(&template_service)));
    registry.register(Arc::clone(&template_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    let storage_handler = Arc::new(StorageHandler::new(Arc::clone(&storage_service)));
    registry.register(Arc::clone(&storage_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;
    
    let config_handler = Arc::new(ConfigHandler::new());
    registry.register(Arc::clone(&config_handler) as Arc<dyn bitacora_commands::registry::RegisterableCommandHandler>).await?;

    // Register handlers with executor
    registry.register_with_executor(&executor).await?;
    
    // Set up help handler with all command metadata
    for command in registry.list_commands().await {
        if let Some(metadata) = registry.get_metadata(&command).await {
            help_handler.register_command_metadata(metadata).await;
        }
    }
    
    println!("✅ {} handlers registered\n", registry.list_commands().await.len());

    // Step 4: Demonstrate command execution
    println!("🎯 Demonstrating command execution...\n");

    let execution_context = ExecutionContext::default();

    // Demo commands to execute
    let demo_commands = vec![
        // Help commands
        "help",
        "help session",
        
        // Status commands
        "status",
        "status --detailed",
        "status --component session",
        
        // Session commands
        "session create 'Demo session for testing'",
        "session metrics",
        "session list",
        
        // Quick commands (shortcuts)
        "start 'Quick session test'",
        
        // Other service commands (placeholders)
        "git status",
        "template list",
        "storage test",
        "config show",
    ];

    for cmd_str in demo_commands {
        println!("$ bitacora {}", cmd_str);
        
        match parser.parse(cmd_str) {
            Ok(parsed_command) => {
                match executor.execute(&parsed_command, Some(execution_context.clone())).await {
                    Ok(result) => {
                        if result.success {
                            println!("✅ Success ({}ms):", result.duration.as_millis());
                            // Truncate long output for demo
                            let output = if result.output.len() > 200 {
                                format!("{}...", &result.output[..200])
                            } else {
                                result.output
                            };
                            println!("{}\n", output);
                        } else {
                            println!("❌ Failed:");
                            if let Some(error) = result.error {
                                println!("   Error: {}\n", error);
                            }
                        }
                    }
                    Err(e) => {
                        println!("💥 Execution error: {}\n", e);
                    }
                }
            }
            Err(e) => {
                println!("🚫 Parse error: {}\n", e);
            }
        }
    }

    // Step 5: Demonstrate registry features
    println!("📊 Registry Features Demo");
    println!("========================\n");
    
    println!("Available categories:");
    for category in registry.get_categories().await {
        let commands = registry.list_by_category(&category).await;
        println!("  {}: {:?}", category, commands);
    }
    println!();
    
    println!("Available aliases:");
    let aliases = registry.get_aliases().await;
    for (alias, target) in aliases {
        println!("  {} -> {}", alias, target);
    }
    println!();
    
    println!("Search results for 'session':");
    let search_results = registry.search("session").await;
    for result in search_results {
        println!("  {} - {}", result.name, result.description);
    }
    println!();

    // Step 6: Demonstrate completion generation
    println!("🔍 Shell Completion Data");
    println!("=======================\n");
    
    let completions = registry.generate_completions().await;
    println!("Commands available for completion: {}", completions.commands.len());
    for cmd in completions.commands.iter().take(3) {
        println!("  {} - {}", cmd.name, cmd.description);
    }
    if completions.commands.len() > 3 {
        println!("  ... and {} more", completions.commands.len() - 3);
    }
    println!();

    // Step 7: Demonstrate active execution tracking
    println!("⏱️  Execution Monitoring");
    println!("=======================\n");
    
    let active_executions = executor.get_active_executions().await;
    println!("Currently active executions: {}", active_executions.len());
    
    println!("Available commands: {:?}", executor.list_commands().await);
    
    println!("\n🎉 Demo completed successfully!");
    println!("\nThe Bitacora commands system is ready to integrate all services");
    println!("into a unified command-line experience. Each handler can be");
    println!("expanded to provide full functionality for their respective domains.");

    Ok(())
}

/// Helper function to demonstrate error handling
async fn demonstrate_error_handling(
    parser: &CommandParser,
    executor: &CommandExecutor,
) -> Result<(), CommandError> {
    println!("🚨 Error Handling Demo");
    println!("======================\n");
    
    let error_cases = vec![
        ("", "Empty command"),
        ("invalid-command", "Unknown command"),
        ("session invalid-subcommand", "Invalid subcommand"),
        ("session create", "Missing required argument"),
    ];

    for (cmd_str, expected_error) in error_cases {
        println!("Testing: '{}' (expecting: {})", cmd_str, expected_error);
        
        if cmd_str.is_empty() {
            match parser.parse(cmd_str) {
                Err(e) => println!("  ✅ Parse error caught: {}", e),
                Ok(_) => println!("  ❌ Expected parse error but succeeded"),
            }
        } else {
            match parser.parse(cmd_str) {
                Ok(parsed) => {
                    match executor.execute(&parsed, None).await {
                        Ok(result) if !result.success => {
                            println!("  ✅ Execution failed as expected: {:?}", result.error);
                        }
                        Ok(_) => println!("  ❌ Expected execution error but succeeded"),
                        Err(e) => println!("  ✅ Execution error caught: {}", e),
                    }
                }
                Err(e) => println!("  ✅ Parse error caught: {}", e),
            }
        }
        println!();
    }
    
    Ok(())
}

/// Helper function to demonstrate different output formats
fn demonstrate_output_formats() {
    println!("📄 Output Format Examples");
    println!("=========================\n");
    
    println!("Text format (default):");
    println!("Session: abc123");
    println!("Status: Active");
    println!("Duration: 125s\n");
    
    println!("JSON format (--format json):");
    println!(r#"{{
  "session_id": "abc123",
  "status": "Active",
  "duration": 125,
  "metadata": {{
    "project_id": "my-project"
  }}
}}"#);
    println!();
    
    println!("YAML format (--format yaml):");
    println!("session_id: abc123");
    println!("status: Active");
    println!("duration: 125");
    println!("metadata:");
    println!("  project_id: my-project");
    println!();
}
