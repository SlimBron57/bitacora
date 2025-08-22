//! Process Manager - Process management

use async_trait::async_trait;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{info, error, debug, warn};

use crate::config::DaemonConfig;
use crate::TimestampResult;
use bitacora_core::errors::BitacoraError;

/// Trait for process management operations
#[async_trait]
pub trait ProcessOperations {
    /// Start managed processes
    async fn start(&self) -> TimestampResult<()>;
    
    /// Stop managed processes
    async fn stop(&self) -> TimestampResult<()>;
    
    /// Get process status
    async fn status(&self) -> TimestampResult<ProcessStatus>;
}

/// Process status information
#[derive(Debug, Clone)]
pub struct ProcessStatus {
    pub pid: Option<u32>,
    pub running: bool,
    pub uptime: std::time::Duration,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

/// Process Manager implementation
pub struct ProcessManager {
    config: DaemonConfig,
    process_handle: std::sync::Arc<tokio::sync::Mutex<Option<tokio::process::Child>>>,
}

impl ProcessManager {
    /// Create new process manager
    pub fn new(config: &DaemonConfig) -> Self {
        Self {
            config: config.clone(),
            process_handle: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        }
    }
    
    /// Check if process is alive
    async fn is_process_alive(&self, pid: u32) -> bool {
        match nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(pid as i32),
            None
        ) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    /// Get system process information
    async fn get_process_info(&self, pid: u32) -> TimestampResult<(u64, f32)> {
        use sysinfo::{System, SystemExt, ProcessExt};
        
        let mut system = System::new_all();
        system.refresh_processes();
        
        if let Some(process) = system.process(sysinfo::Pid::from(pid as usize)) {
            let memory = process.memory();
            let cpu = process.cpu_usage();
            Ok((memory, cpu))
        } else {
            Err(BitacoraError::OperationError(format!("Process {} not found", pid)))
        }
    }
}

#[async_trait]
impl ProcessOperations for ProcessManager {
    async fn start(&self) -> TimestampResult<()> {
        info!("Starting timestamp process");
        
        let mut handle = self.process_handle.lock().await;
        
        if handle.is_some() {
            warn!("Process already running");
            return Ok(());
        }
        
        // Start the timestamp generation process
        let child = Command::new("sh")
            .arg("-c")
            .arg(&self.config.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| BitacoraError::OperationError(format!("Failed to start process: {}", e)))?;
        
        debug!("Process started with PID: {:?}", child.id());
        *handle = Some(child);
        
        info!("Timestamp process started successfully");
        Ok(())
    }
    
    async fn stop(&self) -> TimestampResult<()> {
        info!("Stopping timestamp process");
        
        let mut handle = self.process_handle.lock().await;
        
        if let Some(mut child) = handle.take() {
            // Try graceful shutdown first
            if let Some(pid) = child.id() {
                debug!("Sending SIGTERM to process {}", pid);
                
                match nix::sys::signal::kill(
                    nix::unistd::Pid::from_raw(pid as i32),
                    nix::sys::signal::Signal::SIGTERM
                ) {
                    Ok(_) => {
                        // Wait for graceful shutdown
                        match tokio::time::timeout(
                            std::time::Duration::from_secs(5),
                            child.wait()
                        ).await {
                            Ok(Ok(_)) => {
                                info!("Process terminated gracefully");
                                return Ok(());
                            }
                            Ok(Err(e)) => {
                                error!("Error waiting for process: {}", e);
                            }
                            Err(_) => {
                                warn!("Process didn't terminate gracefully, forcing kill");
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to send SIGTERM: {}", e);
                    }
                }
            }
            
            // Force kill if graceful shutdown failed
            if let Err(e) = child.kill().await {
                error!("Failed to kill process: {}", e);
                return Err(BitacoraError::OperationError(format!("Failed to kill process: {}", e)));
            }
        }
        
        info!("Timestamp process stopped");
        Ok(())
    }
    
    async fn status(&self) -> TimestampResult<ProcessStatus> {
        let handle = self.process_handle.lock().await;
        
        if let Some(child) = handle.as_ref() {
            if let Some(pid) = child.id() {
                let running = self.is_process_alive(pid).await;
                
                let (memory_usage, cpu_usage) = if running {
                    self.get_process_info(pid).await.unwrap_or((0, 0.0))
                } else {
                    (0, 0.0)
                };
                
                Ok(ProcessStatus {
                    pid: Some(pid),
                    running,
                    uptime: std::time::Duration::from_secs(0), // TODO: Calculate actual uptime
                    memory_usage,
                    cpu_usage,
                })
            } else {
                Ok(ProcessStatus {
                    pid: None,
                    running: false,
                    uptime: std::time::Duration::from_secs(0),
                    memory_usage: 0,
                    cpu_usage: 0.0,
                })
            }
        } else {
            Ok(ProcessStatus {
                pid: None,
                running: false,
                uptime: std::time::Duration::from_secs(0),
                memory_usage: 0,
                cpu_usage: 0.0,
            })
        }
    }
}
