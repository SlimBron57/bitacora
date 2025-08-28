//! Health check DTOs for Bitacora API

use serde::{Deserialize, Serialize};

/// Health status enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System is unhealthy
    Unhealthy,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Overall health status
    pub status: HealthStatus,
    /// Timestamp of the health check
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Optional message
    pub message: Option<String>,
    /// Service version
    pub version: String,
}

impl Default for HealthStatus {
    fn default() -> Self {
        HealthStatus::Healthy
    }
}

impl HealthResponse {
    /// Create a healthy response
    pub fn healthy() -> Self {
        Self {
            status: HealthStatus::Healthy,
            timestamp: chrono::Utc::now(),
            message: Some("Service is running normally".to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
    
    /// Create an unhealthy response with message
    pub fn unhealthy(message: &str) -> Self {
        Self {
            status: HealthStatus::Unhealthy,
            timestamp: chrono::Utc::now(),
            message: Some(message.to_string()),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
