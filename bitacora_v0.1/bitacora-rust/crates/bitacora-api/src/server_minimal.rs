//! Minimal server implementation for immediate testing

use axum::{
    routing::get,
    Router, Json,
};
use serde_json::{json, Value};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};
// Documentation generation removed

use crate::{
    handlers_simple::health_check,
    dto::{
        common::ApiResponse,
    },
};

// Documentation generation removed

/// Root handler
async fn root_handler() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::success(json!({
        "service": "Bitacora API - Minimal Version",
        "version": "0.1.0",
        "status": "running"
    })))
}

/// Create the minimal application router
pub fn create_minimal_app() -> Router {
    // Health routes only
    let health_routes = Router::new()
        .route("/health", get(health_check));

    // Documentation removed

    // Build the main router
    Router::new()
        .merge(health_routes)
    // docs removed
        .route("/", get(root_handler))
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_headers(Any)
                        .allow_methods(Any)
                )
        )
}

/// Start the minimal HTTP server
pub async fn serve_minimal(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_minimal_app();
    
    println!("🚀 Minimal Server starting on http://{}", addr);
    println!("📚 API Documentation: http://{}/swagger-ui", addr);
    println!("❤️  Health Check: http://{}/health", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
