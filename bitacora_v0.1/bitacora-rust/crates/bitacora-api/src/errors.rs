//! Error handling for Bitacora API
//!
//! Provides comprehensive error handling with proper HTTP status codes and
//! structured error responses for API clients.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

/// Standard API result type
pub type ApiResult<T> = Result<T, ApiError>;

/// Comprehensive API error types with HTTP status code mapping
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", content = "details")]
pub enum ApiError {
    /// Resource not found (404)
    NotFound {
        resource: String,
        id: String,
    },
    /// Bad request - invalid input (400)
    BadRequest {
        message: String,
        field: Option<String>,
    },
    /// Validation error (422)
    ValidationError {
        message: String,
        errors: Vec<ValidationFieldError>,
    },
    /// Conflict - resource already exists (409)
    Conflict {
        resource: String,
        message: String,
    },
    /// Internal server error (500)
    InternalError {
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        trace_id: Option<String>,
    },
    /// Service unavailable (503)
    ServiceUnavailable {
        service: String,
        message: String,
    },
    /// Unauthorized access (401)
    Unauthorized {
        message: String,
    },
    /// Forbidden access (403)
    Forbidden {
        resource: String,
        action: String,
    },
    /// Database connection error (503)
    DatabaseError {
        operation: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: ApiError,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub path: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound { resource, id } => {
                write!(f, "{} with id '{}' not found", resource, id)
            }
            ApiError::BadRequest { message, field } => {
                if let Some(field) = field {
                    write!(f, "Bad request for field '{}': {}", field, message)
                } else {
                    write!(f, "Bad request: {}", message)
                }
            }
            ApiError::ValidationError { message, errors } => {
                let fields: Vec<String> = errors.iter().map(|e| e.field.clone()).collect();
                write!(f, "Validation error: {} (fields: {})", message, fields.join(", "))
            }
            ApiError::Conflict { resource, message } => {
                write!(f, "Conflict with {}: {}", resource, message)
            }
            ApiError::InternalError { message, .. } => {
                write!(f, "Internal error: {}", message)
            }
            ApiError::ServiceUnavailable { service, message } => {
                write!(f, "Service '{}' unavailable: {}", service, message)
            }
            ApiError::Unauthorized { message } => {
                write!(f, "Unauthorized: {}", message)
            }
            ApiError::Forbidden { resource, action } => {
                write!(f, "Forbidden: cannot {} {}", action, resource)
            }
            ApiError::DatabaseError { operation, message } => {
                write!(f, "Database error during '{}': {}", operation, message)
            }
        }
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::ValidationError { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::Conflict { .. } => StatusCode::CONFLICT,
            ApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ServiceUnavailable { .. } => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden { .. } => StatusCode::FORBIDDEN,
            ApiError::DatabaseError { .. } => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Create a not found error for a specific resource
    pub fn not_found(resource: &str, id: &str) -> Self {
        Self::NotFound {
            resource: resource.to_string(),
            id: id.to_string(),
        }
    }

    /// Create a bad request error with optional field specification
    pub fn bad_request(message: &str, field: Option<&str>) -> Self {
        Self::BadRequest {
            message: message.to_string(),
            field: field.map(|f| f.to_string()),
        }
    }

    /// Create a validation error with field-specific errors
    pub fn validation_error(message: &str, errors: Vec<ValidationFieldError>) -> Self {
        Self::ValidationError {
            message: message.to_string(),
            errors,
        }
    }

    /// Create a conflict error for resource conflicts
    pub fn conflict(resource: &str, message: &str) -> Self {
        Self::Conflict {
            resource: resource.to_string(),
            message: message.to_string(),
        }
    }

    /// Create an internal error with optional trace ID
    pub fn internal(message: &str, trace_id: Option<String>) -> Self {
        Self::InternalError {
            message: message.to_string(),
            trace_id,
        }
    }

    /// Create a database error
    pub fn database_error(operation: &str, message: &str) -> Self {
        Self::DatabaseError {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = ErrorResponse {
            error: self,
            timestamp: chrono::Utc::now(),
            path: None, // Could be populated by middleware
        };

        (status, Json(error_response)).into_response()
    }
}

// Conversion from thiserror
impl From<Box<dyn std::error::Error + Send + Sync>> for ApiError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ApiError::internal(&err.to_string(), None)
    }
}

// Validation field error convenience constructor
impl ValidationFieldError {
    pub fn new(field: &str, message: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(ApiError::not_found("Project", "123").status_code(), StatusCode::NOT_FOUND);
        assert_eq!(ApiError::bad_request("Invalid input", None).status_code(), StatusCode::BAD_REQUEST);
        assert_eq!(ApiError::internal("Server error", None).status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_error_display() {
        let error = ApiError::not_found("Project", "abc-123");
        assert_eq!(error.to_string(), "Project with id 'abc-123' not found");
    }

    #[test]
    fn test_validation_error() {
        let errors = vec![
            ValidationFieldError::new("name", "Name is required"),
            ValidationFieldError::new("email", "Invalid email format"),
        ];
        let error = ApiError::validation_error("Validation failed", errors);
        
        match error {
            ApiError::ValidationError { errors, .. } => {
                assert_eq!(errors.len(), 2);
                assert_eq!(errors[0].field, "name");
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}
