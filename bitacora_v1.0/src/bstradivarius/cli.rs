// bitacora_v1.0/src/bstradivarius/cli.rs
//! 🎻 BStradivarius CLI - Precision Interface
//!
//! Terminal-first design with Stradivarius elegance.
//! "Como lo hace cargo en rust - medir performance y updates bajo solicitud en la terminal"

use super::*;
use colored::*;
use std::io::{self, Write};

/// CLI commands
#[derive(Debug, Clone)]
pub enum WatchCommand {
    /// Start watching
    Start,
    
    /// Show current stats
    Status,
    
    /// Query concepts
    Query { pattern: String },
    
    /// Show performance metrics
    Metrics,
    
    /// Stop watcher gracefully
    Stop,
    
    /// Clear VoxelDB index
    Clear,
    
    /// Help
    Help,
}

/// CLI output formatter (cargo-style)
pub struct CliFormatter;

impl CliFormatter {
    /// Print startup banner
    pub fn print_banner() {
        println!("{}", "╔═══════════════════════════════════════════════════════════╗".bright_magenta());
        println!("{}", "║     🎻 BSTRADIVARIUS - Meta-Loop System v0.1.0           ║".bright_magenta());
        println!("{}", "║                                                           ║".bright_magenta());
        println!("{}", "║     Precision documentation, masterful performance 🎭✨   ║".bright_magenta());
        println!("{}", "╚═══════════════════════════════════════════════════════════╝".bright_magenta());
        println!();
    }
    
    /// Print stage (like cargo's "Compiling", "Finished")
    pub fn print_stage(stage: &str, message: &str) {
        let stage_colored = match stage {
            "Starting" => stage.bright_green(),
            "Watching" => stage.bright_cyan(),
            "Processing" => stage.bright_yellow(),
            "Finished" => stage.bright_green().bold(),
            "Error" => stage.bright_red().bold(),
            _ => stage.white(),
        };
        
        println!("  {} {}", stage_colored.bold(), message);
    }
    
    /// Print file event (like cargo's build progress)
    pub fn print_event(event: &WatcherEvent) {
        match event {
            WatcherEvent::FileModified { path } => {
                let filename = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                println!("     {} {}", "Modified".bright_yellow(), filename);
            },
            WatcherEvent::FileCreated { path } => {
                let filename = path.file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");
                println!("      {} {}", "Created".bright_green(), filename);
            },
            WatcherEvent::ConceptDetected { concept, .. } => {
                println!("     {} concept: {}", "Indexed".bright_cyan(), concept.bright_white());
            },
            WatcherEvent::CrossRefDetected { target, .. } => {
                println!("    {} → {}", "CrossRef".bright_magenta(), target);
            },
            _ => {}
        }
    }
    
    /// Print metrics table (cargo-style summary)
    pub fn print_metrics(stats: &WatcherStats) {
        println!();
        println!("{}", "Performance Metrics:".bright_cyan().bold());
        println!("{}", "─────────────────────────────────────".bright_black());
        
        // Files & Concepts
        println!("  {} {}", 
            "Files watched:".bright_white(), 
            format!("{}", stats.files_watched).bright_green()
        );
        println!("  {} {}",
            "Concepts indexed:".bright_white(),
            format!("{}", stats.concepts_indexed).bright_green()
        );
        println!("  {} {}",
            "Cross-references:".bright_white(),
            format!("{}", stats.cross_refs_found).bright_green()
        );
        
        println!();
        
        // Performance
        println!("  {} {}",
            "Events processed:".bright_white(),
            format!("{}", stats.events_processed).bright_yellow()
        );
        println!("  {} {}",
            "VoxelDB ops:".bright_white(),
            format!("{}", stats.voxel_ops).bright_yellow()
        );
        println!("  {} {}",
            "Avg processing:".bright_white(),
            format!("{:.2}ms", stats.avg_processing_ms).bright_cyan()
        );
        
        println!();
        
        // Uptime
        let uptime_str = format_uptime(stats.uptime_secs);
        println!("  {} {}",
            "Uptime:".bright_white(),
            uptime_str.bright_blue()
        );
        
        println!("{}", "─────────────────────────────────────".bright_black());
    }
    
    /// Print query results
    pub fn print_query_results(pattern: &str, results: &[String]) {
        println!();
        println!("{} {}", "Query:".bright_cyan().bold(), pattern.bright_white());
        println!("{}", "─────────────────────────────────────".bright_black());
        
        if results.is_empty() {
            println!("  {}", "No results found".bright_black());
        } else {
            for (i, result) in results.iter().enumerate() {
                println!("  {}. {}", 
                    format!("{}", i + 1).bright_yellow(),
                    result
                );
            }
        }
        
        println!();
        println!("{} {} results", 
            "Found".bright_green().bold(),
            results.len()
        );
    }
    
    /// Print error
    pub fn print_error(error: &str) {
        eprintln!("{} {}", "error:".bright_red().bold(), error);
    }
    
    /// Print warning
    pub fn print_warning(warning: &str) {
        println!("{} {}", "warning:".bright_yellow().bold(), warning);
    }
    
    /// Progress spinner (for long operations)
    pub fn spinner(message: &str) -> SpinnerGuard {
        print!("     {} {}... ", "Running".bright_yellow(), message);
        io::stdout().flush().unwrap();
        SpinnerGuard
    }
}

/// Spinner guard (auto-clears on drop)
pub struct SpinnerGuard;

impl Drop for SpinnerGuard {
    fn drop(&mut self) {
        println!("{}", "Done".bright_green());
    }
}

/// Format uptime human-readable
fn format_uptime(secs: u64) -> String {
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, mins, secs)
    } else if mins > 0 {
        format!("{}m {}s", mins, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Print help
pub fn print_help() {
    println!();
    println!("{}", "BStradivarius 🎻 - Commands".bright_magenta().bold());
    println!("{}", "═══════════════════════════════════════════".bright_black());
    println!();
    
    let commands = vec![
        ("bstradivarius watch", "Start watching documentation changes"),
        ("bstradivarius status", "Show current watcher status"),
        ("bstradivarius query <pattern>", "Search indexed concepts"),
        ("bstradivarius metrics", "Display performance metrics"),
        ("bstradivarius generate <file>", "Regenerate documentation"),
        ("bstradivarius compress <file>", "Compress markdown to QPX format"),
        ("bstradivarius decompress <id>", "Decompress QPX template to markdown"),
        ("bstradivarius fbcu-stats", "Show FBCU compression statistics"),
        ("bstradivarius stop", "Stop watcher gracefully"),
        ("bstradivarius clear", "Clear VoxelDB index"),
        ("bstradivarius help", "Show this help message"),
    ];
    
    for (cmd, desc) in commands {
        println!("  {:<35} {}", 
            cmd.bright_magenta(),
            desc.white()
        );
    }
    
    println!();
    println!("{}", "Examples:".bright_cyan().bold());
    println!("  {} Watch documentation in real-time",
        "bstradivarius watch".bright_yellow()
    );
    println!("  {} Find all HumanRecognition references",
        "bstradivarius query HumanRecognition".bright_yellow()
    );
    println!("  {} Regenerate knowledge graph index",
        "bstradivarius generate KNOWLEDGE_INDEX.md".bright_yellow()
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_uptime() {
        assert_eq!(format_uptime(45), "45s");
        assert_eq!(format_uptime(120), "2m 0s");
        assert_eq!(format_uptime(3665), "1h 1m 5s");
    }
}
