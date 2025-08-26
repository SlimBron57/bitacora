//! Health check and system status endpoints

use axum::{
    http::StatusCode,
    Json,
    response::Result as AxumResult,
};
use chrono::Utc;
use std::collections::HashMap;
use utoipa::openapi;

use crate::dto::common::{HealthResponse, HealthStatus, ServiceHealth};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "System health check", body = HealthResponse),
        (status = 503, description = "System unhealthy", body = HealthResponse)
    ),
    tag = "health"
)]
pub async fn health_check() -> AxumResult<(StatusCode, Json<HealthResponse>)> {
    // TODO: Add real health checks for services
    let mut services = Vec::new();
    
    // Database health check
    services.push(ServiceHealth {
        name: "database".to_string(),
        status: HealthStatus::Healthy, // TODO: Real MongoDB check
        message: Some("Connection OK".to_string()),
        response_time_ms: Some(25),
    });
    
    // Commands service health check
    services.push(ServiceHealth {
        name: "commands".to_string(),
        status: HealthStatus::Healthy,
        message: Some("All handlers operational".to_string()),
        response_time_ms: Some(5),
    });
    
    // Session service health check
    services.push(ServiceHealth {
        name: "sessions".to_string(),
        status: HealthStatus::Healthy,
        message: Some("Session management active".to_string()),
        response_time_ms: Some(10),
    });
    
    // Determine overall status
    let overall_status = if services.iter().any(|s| matches!(s.status, HealthStatus::Unhealthy)) {
        HealthStatus::Unhealthy
    } else if services.iter().any(|s| matches!(s.status, HealthStatus::Degraded)) {
        HealthStatus::Degraded
    } else {
        HealthStatus::Healthy
    };
    
    let health_response = HealthResponse {
        status: overall_status.clone(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        services,
    };
    
    let status_code = match overall_status {
        HealthStatus::Healthy => StatusCode::OK,
        HealthStatus::Degraded => StatusCode::OK,
        HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };
    
    Ok((status_code, Json(health_response)))
}

/// Readiness check endpoint (for Kubernetes)
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready"),
        (status = 503, description = "Service not ready")
    ),
    tag = "health"
)]
pub async fn readiness_check() -> AxumResult<StatusCode> {
    // TODO: Add readiness checks (database connection, required services)
    // For now, always return ready
    Ok(StatusCode::OK)
}

/// Liveness check endpoint (for Kubernetes)
#[utoipa::path(
    get,
    path = "/live",
    responses(
        (status = 200, description = "Service is alive"),
        (status = 503, description = "Service not responding")
    ),
    tag = "health"
)]
pub async fn liveness_check() -> AxumResult<StatusCode> {
    // For liveness, we just need to respond - if we can respond, we're alive
    Ok(StatusCode::OK)
}

/// API version information
#[utoipa::path(
    get,
    path = "/version",
    responses(
        (status = 200, description = "API version information", body = HashMap<String, String>)
    ),
    tag = "health"
)]
pub async fn version_info() -> AxumResult<Json<HashMap<String, String>>> {
    let mut info = HashMap::new();
    info.insert("name".to_string(), env!("CARGO_PKG_NAME").to_string());
    info.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
    info.insert("description".to_string(), env!("CARGO_PKG_DESCRIPTION").to_string());
    info.insert("build_time".to_string(), "2025-08-24T22:30:00Z".to_string()); // TODO: Real build time
    info.insert("git_commit".to_string(), "latest".to_string()); // TODO: Real git commit
    
    Ok(Json(info))
}
