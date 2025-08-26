//! Global error handling middleware

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use tracing::error;

use crate::errors::{ApiError, ErrorResponse};

/// Global error handler middleware
pub async fn error_handler_middleware(
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;
    
    // If the response is an error status, we might want to log it
    if response.status().is_server_error() {
        error!("Server error occurred: {}", response.status());
    }
    
    response
}

/// Convert any error into a proper API error response
pub async fn handle_error(err: Box<dyn std::error::Error + Send + Sync>) -> impl IntoResponse {
    let api_error = ApiError::internal(&err.to_string(), None);
    
    error!("Unhandled error: {}", err);
    
    let error_response = ErrorResponse {
        error: api_error.clone(),
        timestamp: chrono::Utc::now(),
        path: None, // Could be populated with request path
    };
    
    (api_error.status_code(), Json(error_response))
}
