//! # Bitacora API Server
//! 
//! REST API implementation for Bitacora following the PROJECT→TOPIC→ACTION + SPARK architecture.
//! 
//! ## Architecture Overview
//! 
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                    API Layer                        │
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  │
//! │  │   Projects  │  │   Topics    │  │   Actions   │  │
//! │  │   Handler   │  │   Handler   │  │   Handler   │  │
//! │  └─────────────┘  └─────────────┘  └─────────────┘  │
//! │         │                 │                 │        │
//! │         └─────────────────┼─────────────────┘        │
//! │                           │                          │
//! │              ┌─────────────────────────────┐         │
//! │              │      SPARK Service          │         │
//! │              │   (Transversal Insights)    │         │
//! │              └─────────────────────────────┘         │
//! └─────────────────────────────────────────────────────┘
//!                            │
//!                  ┌─────────────────────────────┐
//!                  │     Business Logic          │
//!                  │  (bitacora-records crate)   │
//!                  └─────────────────────────────┘
//!                            │
//!                  ┌─────────────────────────────┐
//!                  │      Persistence            │
//!                  │  (bitacora-storage crate)   │
//!                  └─────────────────────────────┘
//! ```
//!
//! ## Key Features
//! - RESTful endpoints for CRUD operations
//! - OpenAPI 3.0 documentation with Swagger UI
//! - CORS support for web clients
//! - Request logging and tracing
//! - Error handling and validation
//! - Session management integration

pub mod dto;
pub mod handlers;
pub mod handlers_simple;  // Simple handlers for testing
pub mod server_simple;    // Simple server for testing
pub mod server_minimal;   // Minimal server for basic testing
// pub mod middleware;  // Moved to handlers/middleware
pub mod server;
pub mod documentation;
pub mod errors;

// Re-exports for convenience
pub use server::ApiServer;
pub use errors::{ApiError, ApiResult};

use utoipa::OpenApi;

/// OpenAPI documentation for Bitacora API
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::projects::list_projects,
        handlers::projects::create_project,
        handlers::projects::get_project,
        handlers::projects::update_project,
        handlers::projects::delete_project,
        handlers::topics::list_topics,
        handlers::topics::create_topic,
        handlers::topics::get_topic,
        handlers::topics::update_topic,
        handlers::topics::delete_topic,
        handlers::actions::list_actions,
        handlers::actions::create_action,
        handlers::actions::get_action,
        handlers::actions::update_action,
        handlers::actions::delete_action,
        handlers::actions::complete_action,
        handlers::sparks::create_spark,
        handlers::sparks::get_sparks,
        handlers::health::health_check,
    ),
    components(
        schemas(
            dto::projects::ProjectDto,
            dto::projects::CreateProjectRequest,
            dto::projects::UpdateProjectRequest,
            dto::topics::TopicDto,
            dto::topics::CreateTopicRequest,
            dto::topics::UpdateTopicRequest,
            dto::actions::ActionDto,
            dto::actions::CreateActionRequest,
            dto::actions::UpdateActionRequest,
            dto::sparks::SparkDto,
            dto::sparks::CreateSparkRequest,
            dto::common::ApiResponse<serde_json::Value>,
            dto::common::PaginationParams,
            dto::common::PaginatedResponse<serde_json::Value>,
            errors::ApiError,
        )
    ),
    tags(
        (name = "projects", description = "Project management operations"),
        (name = "topics", description = "Topic management operations"),
        (name = "actions", description = "Action management operations"), 
        (name = "sparks", description = "Spark insights (transversal service)"),
        (name = "health", description = "Service health and status"),
    ),
    info(
        title = "Bitacora API",
        version = "1.0.0",
        description = "REST API for Bitacora - PROJECT→TOPIC→ACTION + SPARK architecture",
        contact(
            name = "Bitacora Team",
            email = "dev@bitacora.io"
        ),
        license(
            name = "MIT"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Development server"),
        (url = "https://api.bitacora.io", description = "Production server")
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_server_creation() {
        let server = ApiServer::new("127.0.0.1:0".parse().unwrap());
        assert!(server.is_ok());
    }
}
