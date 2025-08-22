//! Daemon Manager - Daemon lifecycle management

use async_trait::async_trait;
use bitacora_core::errors::BitacoraError;
use tokio::time::{sleep, Duration};
use tracing::{info, error, debug};

use crate::config::DaemonConfig;
use crate::TimestampResult;
use super::ProcessManager;
use super::HealthChecker;
use super::process::ProcessOperations;
use super::health_check::HealthChecking;

/// Trait for daemon lifecycle management
#[async_trait]
pub trait DaemonLifecycle {
    /// Start the daemon
    async fn start(&self) -> TimestampResult<()>;
    
    /// Stop the daemon gracefully
    async fn stop(&self) -> TimestampResult<()>;
    
    /// Restart the daemon
    async fn restart(&self) -> TimestampResult<()>;
    
    /// Check if daemon is running
    async fn is_running(&self) -> bool;
}

/// Daemon Manager implementation
pub struct DaemonManager {
    config: DaemonConfig,
    process_manager: ProcessManager,
    health_checker: HealthChecker,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl DaemonManager {
    /// Create new daemon manager
    pub fn new(config: DaemonConfig) -> Self {
        Self {
            process_manager: ProcessManager::new(&config),
            health_checker: HealthChecker::new(&config),
            config,
            running: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }
    
    /// Run daemon main loop
    pub async fn run(&self) -> TimestampResult<()> {
        info!("Starting daemon main loop");
        
        // Mark as running
        self.running.store(true, std::sync::atomic::Ordering::SeqCst);
        
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.check_interval));
        
        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            interval.tick().await;
            
            // Perform health checks
            if let Err(e) = self.health_checker.check().await {
                error!("Health check failed: {}", e);
                // TODO: Handle health check failures
            }
            
            debug!("Daemon heartbeat");
        }
        
        info!("Daemon main loop stopped");
        Ok(())
    }
}

#[async_trait]
impl DaemonLifecycle for DaemonManager {
    async fn start(&self) -> TimestampResult<()> {
        info!("Starting daemon");
        
        if self.is_running().await {
            return Err(BitacoraError::Infrastructure("Daemon already running".to_string()));
        }
        
        // Start daemon components
        self.process_manager.start().await?;
        
        info!("Daemon started successfully");
        Ok(())
    }
    
    async fn stop(&self) -> TimestampResult<()> {
        info!("Stopping daemon gracefully");
        
        // Signal daemon to stop
        self.running.store(false, std::sync::atomic::Ordering::SeqCst);
        
        // Stop daemon components
        self.process_manager.stop().await?;
        
        info!("Daemon stopped successfully");
        Ok(())
    }
    
    async fn restart(&self) -> TimestampResult<()> {
        info!("Restarting daemon");
        
        self.stop().await?;
        sleep(Duration::from_secs(1)).await; // Brief pause
        self.start().await?;
        
        info!("Daemon restarted successfully");
        Ok(())
    }
    
    async fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::SeqCst)
    }
}
