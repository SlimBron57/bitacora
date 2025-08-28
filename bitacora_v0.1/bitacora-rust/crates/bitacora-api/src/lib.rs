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

// API documentation generation removed - Swagger/UI not required

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_server_creation() {
        let server = ApiServer::new("127.0.0.1:0".parse().unwrap());
        assert!(server.is_ok());
    }
}
