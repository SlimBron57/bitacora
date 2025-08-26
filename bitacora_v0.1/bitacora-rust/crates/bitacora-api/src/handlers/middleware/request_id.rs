//! Request ID middleware for tracing and debugging

use axum::{
    extract::Request,
    http::{header, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

/// Header name for request ID
pub const REQUEST_ID_HEADER: &str = "X-Request-Id";

/// Middleware to add request ID to all requests and responses
pub async fn request_id_middleware(
    mut request: Request,
    next: Next,
) -> Response {
    // Generate or extract request ID
    let request_id = request
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Add request ID to request headers (for downstream handlers)
    request.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    // Add request ID to tracing span
    let span = tracing::info_span!("request", request_id = %request_id);
    let _guard = span.enter();

    // Process the request
    let mut response = next.run(request).await;

    // Add request ID to response headers
    response.headers_mut().insert(
        REQUEST_ID_HEADER,
        HeaderValue::from_str(&request_id).unwrap(),
    );

    response
}

/// Extract request ID from request headers
pub fn get_request_id_from_headers(headers: &axum::http::HeaderMap) -> Option<String> {
    headers
        .get(REQUEST_ID_HEADER)
        .and_then(|header| header.to_str().ok())
        .map(|s| s.to_string())
}
