//! Simple server implementation for testing

use axum::{
    routing::{get, post},
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
    handlers_simple::{
        health_check, 
        get_projects, 
        get_topics, get_topic_by_id, create_topic, update_topic,
        get_actions, get_action_by_id, create_action, update_action, delete_action, complete_action
    },
    dto::{
        common::ApiResponse,
    },
};

// Documentation generation removed

/// Root handler
async fn root_handler() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::success(json!({
        "service": "Bitacora API",
        "version": "0.1.0",
        "status": "running"
    })))
}

/// Create the application router
pub fn create_app() -> Router {
    // API routes
    let api_routes = Router::new()
        .route("/projects", get(get_projects))
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/:id", get(get_topic_by_id).put(update_topic))
        .route("/actions", get(get_actions).post(create_action))
        .route("/actions/:id", get(get_action_by_id).put(update_action).delete(delete_action))
        .route("/actions/:id/complete", post(complete_action));

    // Health routes
    let health_routes = Router::new()
        .route("/health", get(health_check));

    // Documentation removed

    // Build the main router
    Router::new()
        .nest("/api/v1", api_routes)
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

/// Start the HTTP server
pub async fn serve(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app();
    
    println!("🚀 Server starting on http://{}", addr);
    println!("📚 API Documentation: http://{}/docs/swagger-ui", addr);
    println!("❤️  Health Check: http://{}/health", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
