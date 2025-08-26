//! Health Checker - Daemon health monitoring

use async_trait::async_trait;
use std::path::Path;
use tokio::fs;
use tracing::{info, error, warn, debug};

use crate::config::DaemonConfig;
use crate::TimestampResult;
use bitacora_core::errors::BitacoraError;

/// Health check status
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Warning(String),
    Critical(String),
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    pub status: HealthStatus,
    pub checks: Vec<CheckResult>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Individual check result
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub duration: std::time::Duration,
}

/// Trait for health checking operations
#[async_trait]
pub trait HealthChecking {
    /// Perform health check
    async fn check(&self) -> TimestampResult<HealthCheckResult>;
    
    /// Check specific component
    async fn check_component(&self, component: &str) -> TimestampResult<CheckResult>;
}

/// Health Checker implementation
pub struct HealthChecker {
    config: DaemonConfig,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new(config: &DaemonConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
    
    /// Check file system health
    async fn check_filesystem(&self) -> TimestampResult<CheckResult> {
        let start = std::time::Instant::now();
        let name = "filesystem".to_string();
        
        // Check if timestamp directory exists and is writable
        let timestamp_dir = Path::new(&self.config.timestamp_file).parent()
            .unwrap_or(Path::new("/tmp"));
        
        match fs::metadata(timestamp_dir).await {
            Ok(metadata) => {
                if metadata.is_dir() {
                    // Try to create a test file
                    let test_file = timestamp_dir.join(".health_check");
                    match fs::write(&test_file, "health_check").await {
                        Ok(_) => {
                            // Clean up test file
                            let _ = fs::remove_file(&test_file).await;
                            
                            Ok(CheckResult {
                                name,
                                status: HealthStatus::Healthy,
                                message: "Filesystem accessible and writable".to_string(),
                                duration: start.elapsed(),
                            })
                        }
                        Err(e) => {
                            Ok(CheckResult {
                                name,
                                status: HealthStatus::Critical(format!("Cannot write to directory: {}", e)),
                                message: format!("Directory not writable: {}", e),
                                duration: start.elapsed(),
                            })
                        }
                    }
                } else {
                    Ok(CheckResult {
                        name,
                        status: HealthStatus::Critical("Path is not a directory".to_string()),
                        message: "Timestamp path is not a directory".to_string(),
                        duration: start.elapsed(),
                    })
                }
            }
            Err(e) => {
                Ok(CheckResult {
                    name,
                    status: HealthStatus::Critical(format!("Directory not accessible: {}", e)),
                    message: format!("Cannot access directory: {}", e),
                    duration: start.elapsed(),
                })
            }
        }
    }
    
    /// Check timestamp file freshness
    async fn check_timestamp_freshness(&self) -> TimestampResult<CheckResult> {
        let start = std::time::Instant::now();
        let name = "timestamp_freshness".to_string();
        
        match fs::metadata(&self.config.timestamp_file).await {
            Ok(metadata) => {
                if let Ok(modified) = metadata.modified() {
                    let elapsed = std::time::SystemTime::now()
                        .duration_since(modified)
                        .unwrap_or(std::time::Duration::from_secs(0));
                    
                    let max_age = std::time::Duration::from_secs(self.config.max_timestamp_age);
                    
                    if elapsed <= max_age {
                        Ok(CheckResult {
                            name,
                            status: HealthStatus::Healthy,
                            message: format!("Timestamp file is fresh ({}s old)", elapsed.as_secs()),
                            duration: start.elapsed(),
                        })
                    } else {
                        Ok(CheckResult {
                            name,
                            status: HealthStatus::Warning(format!("Timestamp file is stale ({}s old)", elapsed.as_secs())),
                            message: format!("Timestamp file older than {}s", max_age.as_secs()),
                            duration: start.elapsed(),
                        })
                    }
                } else {
                    Ok(CheckResult {
                        name,
                        status: HealthStatus::Warning("Cannot get file modification time".to_string()),
                        message: "Unable to check file age".to_string(),
                        duration: start.elapsed(),
                    })
                }
            }
            Err(e) => {
                Ok(CheckResult {
                    name,
                    status: HealthStatus::Critical(format!("Timestamp file not accessible: {}", e)),
                    message: format!("Cannot access timestamp file: {}", e),
                    duration: start.elapsed(),
                })
            }
        }
    }
    
    /// Check system resources
    async fn check_system_resources(&self) -> TimestampResult<CheckResult> {
        let start = std::time::Instant::now();
        let name = "system_resources".to_string();
        
        use sysinfo::{System, SystemExt};
        
        let mut system = System::new_all();
        system.refresh_memory();
        
        let available_memory = system.available_memory();
        let total_memory = system.total_memory();
        let memory_usage_percent = ((total_memory - available_memory) as f64 / total_memory as f64) * 100.0;
        
        if memory_usage_percent < 90.0 {
            Ok(CheckResult {
                name,
                status: HealthStatus::Healthy,
                message: format!("Memory usage: {:.1}%", memory_usage_percent),
                duration: start.elapsed(),
            })
        } else if memory_usage_percent < 95.0 {
            Ok(CheckResult {
                name,
                status: HealthStatus::Warning(format!("High memory usage: {:.1}%", memory_usage_percent)),
                message: "Memory usage is high".to_string(),
                duration: start.elapsed(),
            })
        } else {
            Ok(CheckResult {
                name,
                status: HealthStatus::Critical(format!("Critical memory usage: {:.1}%", memory_usage_percent)),
                message: "Memory usage is critical".to_string(),
                duration: start.elapsed(),
            })
        }
    }
}

#[async_trait]
impl HealthChecking for HealthChecker {
    async fn check(&self) -> TimestampResult<HealthCheckResult> {
        debug!("Performing health checks");
        
        let mut checks = Vec::new();
        let mut overall_status = HealthStatus::Healthy;
        
        // Perform all health checks
        let filesystem_check = self.check_filesystem().await?;
        let timestamp_check = self.check_timestamp_freshness().await?;
        let resources_check = self.check_system_resources().await?;
        
        // Collect results
        checks.push(filesystem_check.clone());
        checks.push(timestamp_check.clone());
        checks.push(resources_check.clone());
        
        // Determine overall status
        for check in &checks {
            match &check.status {
                HealthStatus::Critical(_) => {
                    overall_status = HealthStatus::Critical("One or more critical issues detected".to_string());
                    break;
                }
                HealthStatus::Warning(_) => {
                    if matches!(overall_status, HealthStatus::Healthy) {
                        overall_status = HealthStatus::Warning("One or more warnings detected".to_string());
                    }
                }
                HealthStatus::Healthy => {}
            }
        }
        
        let result = HealthCheckResult {
            status: overall_status,
            checks,
            timestamp: chrono::Utc::now(),
        };
        
        match &result.status {
            HealthStatus::Healthy => debug!("Health check passed"),
            HealthStatus::Warning(msg) => warn!("Health check warning: {}", msg),
            HealthStatus::Critical(msg) => error!("Health check critical: {}", msg),
        }
        
        Ok(result)
    }
    
    async fn check_component(&self, component: &str) -> TimestampResult<CheckResult> {
        match component {
            "filesystem" => self.check_filesystem().await,
            "timestamp" => self.check_timestamp_freshness().await,
            "resources" => self.check_system_resources().await,
            _ => Err(BitacoraError::ValidationError(format!("Unknown component: {}", component))),
        }
    }
}
