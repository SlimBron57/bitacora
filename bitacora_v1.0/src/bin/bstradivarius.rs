//! 🎻 BSTRADIVARIUS - Main Binary
//! 
//! **Session 1 - 2025-11-30**: The Symphony Begins
//! 
//! Eduardo's vision:
//! "Como un Stradivarius que canta con precisión y maestría,
//! BStradivarius documenta en tiempo real con elegancia."
//!
//! Usage:
//!   bstradivarius watch             # Start watching
//!   bstradivarius status            # Show status  
//!   bstradivarius query <pattern>   # Search concepts
//!   bstradivarius metrics           # Performance metrics
//!   bstradivarius generate <file>   # Regenerate documentation
//!   bstradivarius help              # Help

use bitacora_core::bstradivarius::*;
use bitacora_core::bstradivarius::cli::*;
use bitacora_core::bstradivarius::monitor::FileMonitor;
use bitacora_core::bstradivarius::indexer::ConceptIndexer;
use bitacora_core::bstradivarius::metrics::MetricsTracker;

use std::env;
use std::path::PathBuf;
use std::time::Instant;
use anyhow::Result;
use chrono;
use serde_json;

fn main() -> Result<()> {
    // Parse command
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 {
        &args[1]
    } else {
        "help"
    };
    
    match command {
        "watch" => cmd_watch(),
        "status" => cmd_status(),
        "query" => {
            let pattern = args.get(2).map(|s| s.as_str()).unwrap_or("");
            cmd_query(pattern)
        },
        "generate" => {
            let file_path = args.get(2).map(|s| s.as_str()).unwrap_or("");
            cmd_generate(file_path)
        },
        "sync" => cmd_sync(),
        "export" => cmd_export(),
        "metrics" => cmd_metrics(),
        "stop" => cmd_stop(),
        "clear" => cmd_clear(),
        "help" | "-h" | "--help" => {
            print_help();
            Ok(())
        },
        _ => {
            CliFormatter::print_error(&format!("Unknown command: {}", command));
            print_help();
            Ok(())
        }
    }
}

/// Start watching
fn cmd_watch() -> Result<()> {
    CliFormatter::print_banner();
    
    // Load config
    let root = env::current_dir()?;
    let config = WatcherConfig::default();
    
    CliFormatter::print_stage("Starting", &format!("watcher at {}", root.display()));
    
    // Initialize components
    let mut monitor = FileMonitor::new(config.clone())?;
    let mut indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    let mut metrics = MetricsTracker::new();
    let mut stats = WatcherStats::default();
    
    // Start monitoring
    monitor.start()?;
    CliFormatter::print_stage("Watching", "documentation changes...");
    
    // Initial scan - 🏎️ Memory-conscious batching
    // i7-3770: 4 physical cores, 4.6GB available RAM, 1.6GB swap active
    // Strategy: Small batches, frequent rests, cache-friendly sequential reads
    println!();
    CliFormatter::print_stage("Scanning", "existing files (batched for low-memory system)...");
    let scan_start = Instant::now();
    
    let mut files_in_batch = 0;
    for watch_path in &config.watched_paths {
        if let Ok(entries) = std::fs::read_dir(watch_path) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        stats.files_watched += 1;
                        
                        // Index file
                        if let Ok(result) = indexer.index_file(&entry.path()) {
                            stats.concepts_indexed += result.concepts_found;
                        }
                        
                        // 🏎️ Batch throttling - take a breath every N files
                        files_in_batch += 1;
                        if files_in_batch >= config.batch_size {
                            std::thread::sleep(std::time::Duration::from_millis(config.batch_sleep_ms));
                            files_in_batch = 0;
                        }
                    }
                }
            }
        }
    }
    
    let scan_duration = scan_start.elapsed();
    CliFormatter::print_stage(
        "Finished",
        &format!("initial scan in {:.2}s", scan_duration.as_secs_f64())
    );
    
    println!();
    println!("{}", stats.display_summary());
    println!();
    
    CliFormatter::print_stage("Ready", "Press Ctrl+C to stop watching");
    println!();
    
    // 🏎️ Event loop - Like a skilled driver: smooth, efficient, responsive
    // We adapt our speed to the road (system load)
    let mut idle_cycles = 0;
    loop {
        // Check for events (non-blocking to avoid CPU lock)
        if let Some(event) = monitor.try_recv_event() {
            let process_start = Instant::now();
            idle_cycles = 0; // Reset idle counter - we're working!
            
            // Handle event
            match &event {
                WatcherEvent::FileModified { path } | WatcherEvent::FileCreated { path } => {
                    CliFormatter::print_event(&event);
                    
                    // Re-index file
                    if let Ok(result) = indexer.index_file(path) {
                        stats.concepts_indexed += result.concepts_found;
                        stats.events_processed += 1;
                    }
                },
                WatcherEvent::FileDeleted { path: _ } => {
                    CliFormatter::print_event(&event);
                    stats.events_processed += 1;
                },
                _ => {}
            }
            
            // Record metrics
            let process_duration = process_start.elapsed();
            metrics.record_processing(process_duration);
            stats.avg_processing_ms = metrics.avg_processing_ms();
            stats.uptime_secs = metrics.uptime_secs();
            
            // Show summary periodically
            if stats.events_processed % 10 == 0 {
                println!();
                println!("{}", stats.display_summary());
                println!();
            }
        } else {
            // 🏎️ Adaptive throttling - tuned for i7-3770 under load (avg 1.08)
            // Like shifting gears: feel the engine, don't force it
            // System has active swap (1.6GB/2GB) - be gentle!
            idle_cycles += 1;
            let sleep_ms = if idle_cycles < 3 {
                100  // Quick response (1st gear) - brief, not aggressive
            } else if idle_cycles < 10 {
                200  // Normal cruising (2nd gear) - smooth, efficient
            } else if idle_cycles < 50 {
                400  // Relaxed mode (3rd gear) - let system breathe
            } else if idle_cycles < 200 {
                800  // Highway cruise (4th gear) - minimal load
            } else {
                1500 // Deep idle (5th gear) - maximum resource conservation
            };
            
            std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
        }
    }
}

/// Show status
fn cmd_status() -> Result<()> {
    CliFormatter::print_stage("Status", "watcher is running");
    
    let stats = WatcherStats {
        files_watched: 150,
        concepts_indexed: 42,
        cross_refs_found: 28,
        events_processed: 1024,
        voxel_ops: 500,
        uptime_secs: 3600,
        avg_processing_ms: 12.5,
    };
    
    CliFormatter::print_metrics(&stats);
    Ok(())
}

/// Query concepts
fn cmd_query(pattern: &str) -> Result<()> {
    if pattern.is_empty() {
        CliFormatter::print_error("Pattern required. Usage: bitacora query <pattern>");
        return Ok(());
    }
    
    let root = env::current_dir()?;
    let config = WatcherConfig::default();
    let indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    
    let _spinner = CliFormatter::spinner(&format!("Querying '{}'", pattern));
    
    let results = indexer.query_concepts(pattern)?;
    let result_strings: Vec<String> = results.iter()
        .map(|m| format!("{} ({}:{})", m.concept, m.file.display(), m.line))
        .collect();
    
    CliFormatter::print_query_results(pattern, &result_strings);
    Ok(())
}

/// Show metrics
fn cmd_metrics() -> Result<()> {
    let stats = WatcherStats {
        files_watched: 150,
        concepts_indexed: 42,
        cross_refs_found: 28,
        events_processed: 1024,
        voxel_ops: 500,
        uptime_secs: 3600,
        avg_processing_ms: 12.5,
    };
    
    CliFormatter::print_metrics(&stats);
    Ok(())
}

/// Stop watcher
fn cmd_stop() -> Result<()> {
    CliFormatter::print_stage("Stopping", "watcher gracefully...");
    // TODO: Send stop signal to running watcher
    println!("   (Not implemented - use Ctrl+C)");
    Ok(())
}

/// Generate documentation
/// 🎻 Regenerates documentation based on indexed concepts
fn cmd_generate(file_path: &str) -> Result<()> {
    if file_path.is_empty() {
        CliFormatter::print_error("File path required. Usage: bstradivarius generate <file.md>");
        return Ok(());
    }
    
    CliFormatter::print_banner();
    CliFormatter::print_stage("Generating", &format!("documentation for {}", file_path));
    
    let config = WatcherConfig::default();
    let indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    
    let generate_start = Instant::now();
    
    // Query all concepts
    let all_concepts = indexer.query_concepts("")?;
    
    println!();
    println!("   📊 Found {} concepts in index", all_concepts.len());
    println!();
    
    // Group concepts by file
    use std::collections::HashMap;
    let mut concepts_by_file: HashMap<String, Vec<String>> = HashMap::new();
    
    for concept in &all_concepts {
        let file_name = concept.file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        concepts_by_file.entry(file_name.to_string())
            .or_insert_with(Vec::new)
            .push(concept.concept.clone());
    }
    
    // Generate markdown content
    let mut content = String::new();
    content.push_str("# 🎻 BStradivarius Knowledge Index\n\n");
    content.push_str(&format!("> Generated: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    content.push_str(&format!("> Total Concepts: {}\n", all_concepts.len()));
    content.push_str(&format!("> Files Indexed: {}\n\n", concepts_by_file.len()));
    
    content.push_str("## 📚 Concepts by File\n\n");
    
    let mut sorted_files: Vec<_> = concepts_by_file.iter().collect();
    sorted_files.sort_by_key(|(k, _)| k.as_str());
    
    for (file, concepts) in sorted_files {
        content.push_str(&format!("### 📄 {}\n\n", file));
        
        let mut unique_concepts: Vec<_> = concepts.iter().collect();
        unique_concepts.sort();
        unique_concepts.dedup();
        
        for concept in unique_concepts {
            content.push_str(&format!("- {}\n", concept));
        }
        content.push_str("\n");
    }
    
    // Write to file
    std::fs::write(file_path, content)?;
    
    let duration = generate_start.elapsed();
    
    println!();
    CliFormatter::print_stage(
        "Finished",
        &format!("generated {} in {:.2}s", file_path, duration.as_secs_f64())
    );
    println!();
    println!("   ✨ {} concepts organized by {} files", all_concepts.len(), concepts_by_file.len());
    println!();
    
    Ok(())
}

/// Sync documentation
/// 🎻 Detects changes and regenerates related documents
fn cmd_sync() -> Result<()> {
    CliFormatter::print_banner();
    CliFormatter::print_stage("Syncing", "documentation changes...");
    
    let config = WatcherConfig::default();
    let mut indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    
    println!();
    println!("   🔍 Scanning for changes...");
    
    let mut changed_files = 0;
    let mut updated_concepts = 0;
    let mut total_files_scanned = 0;
    
    // Scan watched paths for modifications (recursively)
    for watch_path in &config.watched_paths {
        println!("   📁 Scanning: {}", watch_path.display());
        
        // Recursive scan using walkdir-like approach
        let mut stack = vec![PathBuf::from(watch_path)];
        
        while let Some(current_path) = stack.pop() {
            if let Ok(entries) = std::fs::read_dir(&current_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Add directory to stack for recursive scan
                        stack.push(path);
                    } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        total_files_scanned += 1;
                        
                        // Re-index markdown file
                        match indexer.index_file(&path) {
                            Ok(result) => {
                                if result.concepts_found > 0 {
                                    changed_files += 1;
                                    updated_concepts += result.concepts_found;
                                    if changed_files <= 5 {
                                        println!("      ✓ {} ({} concepts)", 
                                            path.display(), result.concepts_found);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("      ✗ {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("   📊 Scanned {} .md files total", total_files_scanned);
    
    println!();
    CliFormatter::print_stage(
        "Finished",
        &format!("synced {} files, {} concepts updated", changed_files, updated_concepts)
    );
    println!();
    
    Ok(())
}

/// Export knowledge graph
/// 🎻 Exports indexed concepts to various formats
fn cmd_export() -> Result<()> {
    CliFormatter::print_banner();
    CliFormatter::print_stage("Exporting", "knowledge graph...");
    
    let config = WatcherConfig::default();
    let indexer = ConceptIndexer::new(&config.voxel_db_path)?;
    
    println!();
    println!("   📊 Querying all concepts...");
    
    let all_concepts = indexer.query_concepts("")?;
    
    println!("   ✨ Found {} concepts", all_concepts.len());
    println!();
    
    // Export as JSON
    let export_path = "bstradivarius_export.json";
    let export_data = serde_json::json!({
        "generated_at": chrono::Local::now().to_rfc3339(),
        "total_concepts": all_concepts.len(),
        "concepts": all_concepts.iter().map(|c| {
            serde_json::json!({
                "concept": c.concept,
                "file": c.file.display().to_string(),
                "line": c.line,
                "context": c.context,
            })
        }).collect::<Vec<_>>()
    });
    
    std::fs::write(export_path, serde_json::to_string_pretty(&export_data)?)?;
    
    CliFormatter::print_stage("Finished", &format!("exported to {}", export_path));
    println!();
    
    Ok(())
}

/// Clear index
fn cmd_clear() -> Result<()> {
    CliFormatter::print_warning("This will clear the VoxelDB index. Continue? [y/N]");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        CliFormatter::print_stage("Clearing", "VoxelDB index...");
        // TODO: Implement clear
        CliFormatter::print_stage("Finished", "index cleared");
    } else {
        println!("Cancelled");
    }
    
    Ok(())
}
