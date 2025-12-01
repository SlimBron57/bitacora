// bitacora_v1.0/src/watcher/monitor.rs
//! 🔍 File System Monitor
//!
//! Watches documentation directories for changes.
//! Triggers indexing on modification.

use super::*;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::path::Path;
use anyhow::{Context, Result};

/// File system monitor
pub struct FileMonitor {
    config: WatcherConfig,
    event_tx: Sender<WatcherEvent>,
    event_rx: Receiver<WatcherEvent>,
}

impl FileMonitor {
    /// Create new monitor
    pub fn new(config: WatcherConfig) -> Result<Self> {
        let (event_tx, event_rx) = channel();
        
        Ok(Self {
            config,
            event_tx,
            event_rx,
        })
    }
    
    /// Start monitoring
    pub fn start(&mut self) -> Result<()> {
        use notify::RecommendedWatcher;
        
        let tx = self.event_tx.clone();
        
        // Create file watcher
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                Self::handle_fs_event(event, &tx);
            }
        })?;
        
        // Watch all configured paths
        for path in &self.config.watched_paths {
            if path.exists() {
                watcher.watch(path, RecursiveMode::Recursive)
                    .with_context(|| format!("Failed to watch {:?}", path))?;
            }
        }
        
        Ok(())
    }
    
    /// Handle file system event
    fn handle_fs_event(event: Event, tx: &Sender<WatcherEvent>) {
        match event.kind {
            EventKind::Create(_) => {
                for path in event.paths {
                    if Self::is_relevant_file(&path) {
                        let _ = tx.send(WatcherEvent::FileCreated { path });
                    }
                }
            },
            EventKind::Modify(_) => {
                for path in event.paths {
                    if Self::is_relevant_file(&path) {
                        let _ = tx.send(WatcherEvent::FileModified { path });
                    }
                }
            },
            EventKind::Remove(_) => {
                for path in event.paths {
                    if Self::is_relevant_file(&path) {
                        let _ = tx.send(WatcherEvent::FileDeleted { path });
                    }
                }
            },
            _ => {}
        }
    }
    
    /// Check if file is relevant (not .git, target/, etc.)
    fn is_relevant_file(path: &Path) -> bool {
        // Must be markdown or rust file
        let is_md = path.extension().and_then(|s| s.to_str()) == Some("md");
        let is_rs = path.extension().and_then(|s| s.to_str()) == Some("rs");
        
        if !is_md && !is_rs {
            return false;
        }
        
        // Skip ignored paths
        let path_str = path.to_string_lossy();
        let ignored = [
            ".git",
            "target",
            "node_modules",
            "00_BACKUPS",
            ".backup",
        ];
        
        for ignore in &ignored {
            if path_str.contains(ignore) {
                return false;
            }
        }
        
        true
    }
    
    /// Receive next event (blocking)
    pub fn recv_event(&self) -> Result<WatcherEvent> {
        self.event_rx.recv()
            .context("Failed to receive event")
    }
    
    /// Try receive event (non-blocking)
    pub fn try_recv_event(&self) -> Option<WatcherEvent> {
        self.event_rx.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_relevant_file() {
        let md_file = PathBuf::from("ROADMAP_V2/test.md");
        assert!(FileMonitor::is_relevant_file(&md_file));
        
        let rs_file = PathBuf::from("src/main.rs");
        assert!(FileMonitor::is_relevant_file(&rs_file));
        
        let ignored = PathBuf::from(".git/config");
        assert!(!FileMonitor::is_relevant_file(&ignored));
        
        let backup = PathBuf::from("00_BACKUPS/test.md");
        assert!(!FileMonitor::is_relevant_file(&backup));
    }
}
