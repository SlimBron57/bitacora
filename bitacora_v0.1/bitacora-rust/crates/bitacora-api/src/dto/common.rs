//! Common DTOs and shared structures for the Bitacora API

use serde::{Deserialize, Serialize};

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn success_with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn message_only(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: true,
            data: None,
            message: Some(message.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Pagination parameters for list endpoints
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: u32,
    
    /// Number of items per page
    #[serde(default = "default_limit")]  
    pub limit: u32,
    
    /// Sort field
    pub sort_by: Option<String>,
    
    /// Sort direction (asc/desc)
    #[serde(default = "default_sort_order")]
    pub order: SortOrder,
}

/// Alias for backward compatibility
pub type PaginationQuery = PaginationParams;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl PaginationMeta {
    pub fn new(page: u32, limit: u32, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (limit as f64)).ceil() as u32;
        
        Self {
            page,
            limit,
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, limit: u32, total_items: u64) -> Self {
        Self {
            items,
            pagination: PaginationMeta::new(page, limit, total_items),
        }
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
            sort_by: None,
            order: SortOrder::Asc,
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Asc
    }
}

fn default_page() -> u32 { 1 }
fn default_limit() -> u32 { 20 }
fn default_sort_order() -> SortOrder { SortOrder::Asc }

/// Filter parameters for searching
#[derive(Debug, Deserialize)]
pub struct FilterParams {
    /// Search query string
    pub q: Option<String>,
    
    /// Filter by status
    pub status: Option<String>,
    
    /// Filter by date range (ISO format)
    pub created_after: Option<chrono::DateTime<chrono::Utc>>,
    pub created_before: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub version: String,
    pub services: Vec<ServiceHealth>,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}
#[derive(Debug, Serialize)]
pub struct ServiceHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub response_time_ms: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("test data");
        assert!(response.success);
        assert_eq!(response.data, Some("test data"));
        assert!(response.message.is_none());
    }

    #[test]
    fn test_pagination_meta() {
        let meta = PaginationMeta::new(2, 10, 35);
        assert_eq!(meta.page, 2);
        assert_eq!(meta.limit, 10);
        assert_eq!(meta.total_items, 35);
        assert_eq!(meta.total_pages, 4);
        assert!(meta.has_next);
        assert!(meta.has_previous);
    }

    #[test]
    fn test_pagination_params_defaults() {
        let params = PaginationParams::default();
        assert_eq!(params.page, 1);
        assert_eq!(params.limit, 20);
        assert!(matches!(params.order, SortOrder::Asc));
    }
}
