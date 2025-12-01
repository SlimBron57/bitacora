// bitacora_v1.0/src/watcher/mod.rs
//! 🔥 BITACORA WATCHER - Meta-Loop System
//!
//! **Philosophy**: "Bitácora documentándose mientras se construye. En tiempo real."
//!
//! **Purpose**:
//! - Monitors ROADMAP_V2/ + BITACORA_KNOWLEDGE_GRAPH/ changes
//! - Indexes concepts with VoxelDB (REAL motor)
//! - CLI like `cargo` (terminal-first UX)
//! - Performance metrics real-time
//!
//! **Session 1 - 2025-11-30**: Meta-documentation begins 🤯💥💎
//!
//! **Eduardo's vision**: "Pongamos esto a correr temporalmente mientras validamos
//! documentación actual. Comenzamos a trabajar en modo Bitácora desde ahora,
//! en un proceso corriendo en esta máquina, como lo hace cargo en rust."

pub mod cli;
pub mod document_graph;
pub mod indexer;
pub mod monitor;
pub mod metrics;
pub mod config;
pub mod fbcu_integration;
pub mod template_engine;
pub mod flow_query;
pub mod git_integration;
pub mod narrative_builder;

use std::path::PathBuf;
use std::time::Duration;
use anyhow::Result;

/// Main watcher configuration
/// 
/// 🏎️ Optimized for modest hardware like i7-3770 (2012) with limited available RAM.
/// Philosophy: Like a skilled driver, we feel the machine and adapt our style.
/// - Small memory footprint (batch processing)
/// - Adaptive CPU usage (intelligent throttling)  
/// - Cache-friendly I/O patterns (sequential reads)
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Root directory to watch (typically bitacora_v1.0/)
    pub root_path: PathBuf,
    
    /// Watched directories
    pub watched_paths: Vec<PathBuf>,
    
    /// Poll interval for file changes
    pub poll_interval: Duration,
    
    /// VoxelDB storage path
    pub voxel_db_path: PathBuf,
    
    /// Enable real-time metrics display
    pub show_metrics: bool,
    
    /// Enable verbose logging
    pub verbose: bool,
    
    /// 🏎️ Max files to scan in one batch (avoid memory spikes on modest hardware)
    pub batch_size: usize,
    
    /// 🏎️ Sleep between batches in milliseconds (give system breathing room)
    pub batch_sleep_ms: u64,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        let root = PathBuf::from(".");
        
        Self {
            root_path: root.clone(),
            watched_paths: vec![
                root.join("ROADMAP_V2"),
                root.join("BITACORA_KNOWLEDGE_GRAPH"),
                root.join("src"),
                root.join("examples"),
            ],
            poll_interval: Duration::from_secs(2),
            voxel_db_path: root.join("data").join("watcher_voxeldb"),
            show_metrics: true,
            verbose: false,
            // 🏎️ Optimized for i7-3770 with 4.6GB available RAM + active swap
            // Small batches = less memory pressure, more cache-friendly
            batch_size: 10,        // Process 10 files at a time (gentle on 4.6GB available)
            batch_sleep_ms: 100,   // 100ms rest (let system breathe, reduce swap thrashing)
        }
    }
}

/// Watcher operational states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatcherState {
    /// Initial startup
    Initializing,
    /// Scanning initial files
    Scanning,
    /// Active monitoring
    Watching,
    /// Processing detected change
    Processing,
    /// Error state (recoverable)
    Error,
    /// Graceful shutdown
    Stopping,
    /// Fully stopped
    Stopped,
}

/// Event types detected by watcher
#[derive(Debug, Clone)]
pub enum WatcherEvent {
    /// New file created
    FileCreated { path: PathBuf },
    
    /// File modified
    FileModified { path: PathBuf },
    
    /// File deleted
    FileDeleted { path: PathBuf },
    
    /// Directory created
    DirCreated { path: PathBuf },
    
    /// Concept detected in document
    ConceptDetected {
        file: PathBuf,
        concept: String,
        line: usize,
    },
    
    /// Cross-reference found
    CrossRefDetected {
        source: PathBuf,
        target: String,
    },
    
    /// Performance metric update
    MetricUpdate {
        metric: String,
        value: f64,
    },
}

/// Main watcher statistics
#[derive(Debug, Clone, Default)]
pub struct WatcherStats {
    /// Files currently watched
    pub files_watched: usize,
    
    /// Total concepts indexed
    pub concepts_indexed: usize,
    
    /// Cross-references found
    pub cross_refs_found: usize,
    
    /// Events processed
    pub events_processed: u64,
    
    /// VoxelDB operations
    pub voxel_ops: u64,
    
    /// Uptime in seconds
    pub uptime_secs: u64,
    
    /// Average processing time (ms)
    pub avg_processing_ms: f64,
}

impl WatcherStats {
    /// Display stats in cargo-like format
    pub fn display_summary(&self) -> String {
        format!(
            "   Watching {} files | {} concepts | {} cross-refs | {} events | {:.2}ms avg",
            self.files_watched,
            self.concepts_indexed,
            self.cross_refs_found,
            self.events_processed,
            self.avg_processing_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_watcher_config_default() {
        let config = WatcherConfig::default();
        assert!(config.watched_paths.len() >= 2);
        assert_eq!(config.poll_interval, Duration::from_secs(2));
    }
    
    #[test]
    fn test_watcher_stats_display() {
        let stats = WatcherStats {
            files_watched: 150,
            concepts_indexed: 42,
            cross_refs_found: 28,
            events_processed: 1024,
            voxel_ops: 500,
            uptime_secs: 300,
            avg_processing_ms: 12.5,
        };
        
        let summary = stats.display_summary();
        assert!(summary.contains("150 files"));
        assert!(summary.contains("42 concepts"));
    }
}
