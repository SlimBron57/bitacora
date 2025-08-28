//! Bitacora API Server implementation
//!
//! This module provides the main HTTP server for the Bitacora API using Axum.

use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower::layer::util::{Stack, Identity};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::Level;

use crate::{
    handlers::{
        projects::AppState,
        health, projects, topics, actions,
    },
    errors::ApiError,
};

/// Main API server structure
pub struct ApiServer {
    app: Router,
    addr: SocketAddr,
}

impl ApiServer {
    /// Create a new API server instance
    pub fn new(addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create application state
        let state = AppState {
            project_service: Arc::new(()),
        };

        // Build the main router
        let app = create_router(state);

        Ok(Self { app, addr })
    }

    /// Start the API server
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!("🚀 Starting Bitacora API server on {}", self.addr);
        
        let listener = tokio::net::TcpListener::bind(&self.addr).await?;
        
        tracing::info!("🌟 Bitacora API is ready!");
        tracing::info!(" Health Check: http://{}/health", self.addr);
        
        axum::serve(listener, self.app).await?;
        
        Ok(())
    }

    /// Get the server's socket address
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

/// Create the main application router
fn create_router(state: AppState) -> Router {
    // API routes
    let api_routes = Router::new()
        // Project routes
        .route("/projects", get(projects::list_projects))
        .route("/projects", post(projects::create_project))
        .route("/projects/:project_id", get(projects::get_project))
        .route("/projects/:project_id", put(projects::update_project))
        .route("/projects/:project_id", delete(projects::delete_project))
        // Topic routes
        .route("/topics", get(topics::list_topics))
        .route("/topics", post(topics::create_topic))
        .route("/topics/:topic_id", get(topics::get_topic))
        .route("/topics/:topic_id", put(topics::update_topic))
        .route("/topics/:topic_id", delete(topics::delete_topic))
        // Action routes
        .route("/actions", get(actions::get_actions))
        .route("/actions", post(actions::create_action))
        .route("/actions/:action_id", get(actions::get_action))
        .route("/actions/:action_id", put(actions::update_action))
        .route("/actions/:action_id", delete(actions::delete_action))
        .route("/actions/:action_id/complete", post(actions::complete_action))
        // TODO: Add spark routes
        .with_state(state);

    // Health and system routes
    let health_routes = Router::new()
        .route("/health", get(health::health_check))
        .route("/ready", get(health::readiness_check))
        .route("/live", get(health::liveness_check))
        .route("/version", get(health::version_info));

    // Documentation generation removed - no docs routes

    Router::new()
        // API v1 routes
        .nest("/api/v1", api_routes)
        // Health routes (no versioning)
        .merge(health_routes)
    // Documentation generation removed
        // Root route
        .route("/", get(root_handler))
        // Global middleware
        .layer(create_middleware_stack())
        // Global error handler
        .fallback(handler_404)
}

/// Create the middleware stack for the application
fn create_middleware_stack() -> ServiceBuilder<Stack<
    TimeoutLayer,
    Stack<
        CorsLayer,
        Stack<
            TraceLayer<
                tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
            >,
            Stack<
                DefaultBodyLimit,
                Identity,
            >
        >
    >
>> {
    ServiceBuilder::new()
        // Body size limit (10MB) - innermost layer
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024))
        // Request tracing
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis)
                )
        )
        // CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .max_age(Duration::from_secs(3600))
        )
        // Request timeout - outermost layer
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}

/// Root handler - API information
async fn root_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "Bitacora API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "REST API for Bitacora - PROJECT→TOPIC→ACTION + SPARK architecture",
    // documentation removed
        "health": "/health",
        "status": "operational"
    }))
}

/// 404 handler for unmatched routes
async fn handler_404() -> impl IntoResponse {
    let error = ApiError::not_found("Endpoint", "The requested endpoint was not found");
    (StatusCode::NOT_FOUND, Json(error))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let addr = "127.0.0.1:0".parse().unwrap();
        let server = ApiServer::new(addr);
        assert!(server.is_ok());
    }

    #[tokio::test]
    async fn test_router_creation() {
        let state = AppState {
            project_service: Arc::new(()),
        };
        let router = create_router(state);
        // Just test that we can create the router without panicking
        assert!(!format!("{:?}", router).is_empty());
    }

    #[test]
    fn test_middleware_stack_creation() {
        // Test that middleware stack can be created
        let _middleware = create_middleware_stack();
    }
}
