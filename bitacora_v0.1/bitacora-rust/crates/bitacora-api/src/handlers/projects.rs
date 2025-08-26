//! Project API handlers for Bitacora
//!
//! Handles CRUD operations for projects following REST conventions

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Result as AxumResult,
    Json,
};
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    dto::{
        projects::{ProjectDto, CreateProjectRequest, UpdateProjectRequest, ProjectSummaryDto},
        common::{ApiResponse, PaginationParams, PaginatedResponse, FilterParams},
    },
    errors::{ApiError, ApiResult},
};

// TODO: Replace with actual service when repository layer is integrated
pub type ProjectService = ();

/// Application state containing services
#[derive(Clone)]
pub struct AppState {
    pub project_service: Arc<ProjectService>,
}

/// List all projects with pagination and filtering
#[utoipa::path(
    get,
    path = "/api/v1/projects",
    params(
        PaginationParams,
        FilterParams
    ),
    responses(
        (status = 200, description = "List of projects retrieved successfully", 
         body = ApiResponse<PaginatedResponse<ProjectSummaryDto>>),
        (status = 400, description = "Invalid query parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "projects"
)]
pub async fn list_projects(
    State(_state): State<AppState>,
    Query(pagination): Query<PaginationParams>,
    Query(_filters): Query<FilterParams>,
) -> AxumResult<Json<ApiResponse<PaginatedResponse<ProjectSummaryDto>>>> {
    // TODO: Implement actual project retrieval from repository
    
    // Mock data for now
    let mock_projects = vec![
        ProjectSummaryDto {
            id: Uuid::new_v4(),
            name: "Sample Project 1".to_string(),
            status: crate::dto::projects::ProjectStatus::Active,
            topic_count: 5,
            completion_percentage: 65.0,
            created_at: chrono::Utc::now() - chrono::Duration::days(10),
            updated_at: chrono::Utc::now() - chrono::Duration::hours(2),
        },
        ProjectSummaryDto {
            id: Uuid::new_v4(),
            name: "Sample Project 2".to_string(),
            status: crate::dto::projects::ProjectStatus::Planning,
            topic_count: 3,
            completion_percentage: 20.0,
            created_at: chrono::Utc::now() - chrono::Duration::days(5),
            updated_at: chrono::Utc::now() - chrono::Duration::hours(1),
        },
    ];
    
    let total_items = mock_projects.len() as u64;
    let paginated_response = PaginatedResponse::new(
        mock_projects, 
        pagination.page, 
        pagination.limit, 
        total_items
    );
    
    Ok(Json(ApiResponse::success(paginated_response)))
}

/// Create a new project
#[utoipa::path(
    post,
    path = "/api/v1/projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created successfully", body = ApiResponse<ProjectDto>),
        (status = 400, description = "Invalid project data", body = ApiError),
        (status = 422, description = "Validation error", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "projects"
)]
pub async fn create_project(
    State(_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> AxumResult<(StatusCode, Json<ApiResponse<ProjectDto>>)> {
    // Validate the request
    if let Err(errors) = request.validate() {
        let validation_errors = errors.into_iter()
            .map(|msg| crate::errors::ValidationFieldError::new("general", &msg))
            .collect();
        let api_error = ApiError::validation_error("Invalid project data", validation_errors);
        return Err(api_error.into());
    }
    
    // TODO: Implement actual project creation in repository
    
    // Mock response
    let now = chrono::Utc::now();
    let project_dto = ProjectDto {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        description: request.description.clone(),
        status: request.status.unwrap_or_default(),
        tags: request.tags.unwrap_or_default(),
        metadata: request.metadata.unwrap_or_default(),
        created_at: now,
        updated_at: now,
        topic_count: 0,
        completed_actions: 0,
        total_actions: 0,
        completion_percentage: 0.0,
    };
    
    Ok((StatusCode::CREATED, Json(ApiResponse::success_with_message(
        project_dto, 
        "Project created successfully"
    ))))
}

/// Get a specific project by ID
#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project unique identifier")
    ),
    responses(
        (status = 200, description = "Project retrieved successfully", body = ApiResponse<ProjectDto>),
        (status = 404, description = "Project not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "projects"
)]
pub async fn get_project(
    State(_state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> AxumResult<Json<ApiResponse<ProjectDto>>> {
    // TODO: Implement actual project retrieval from repository
    
    // For now, return a mock project or not found
    if project_id.to_string().starts_with("00000000") {
        let api_error = ApiError::not_found("Project", &project_id.to_string());
        return Err(api_error.into());
    }
    
    // Mock response
    let now = chrono::Utc::now();
    let project_dto = ProjectDto {
        id: project_id,
        name: "Sample Project".to_string(),
        description: Some("A sample project for demonstration".to_string()),
        status: crate::dto::projects::ProjectStatus::Active,
        tags: vec!["development".to_string(), "api".to_string()],
        metadata: std::collections::HashMap::new(),
        created_at: now - chrono::Duration::days(7),
        updated_at: now - chrono::Duration::hours(3),
        topic_count: 4,
        completed_actions: 8,
        total_actions: 12,
        completion_percentage: 66.7,
    };
    
    Ok(Json(ApiResponse::success(project_dto)))
}

/// Update an existing project
#[utoipa::path(
    put,
    path = "/api/v1/projects/{project_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project unique identifier")
    ),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated successfully", body = ApiResponse<ProjectDto>),
        (status = 400, description = "Invalid project data", body = ApiError),
        (status = 404, description = "Project not found", body = ApiError),
        (status = 422, description = "Validation error", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "projects"
)]
pub async fn update_project(
    State(_state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(request): Json<UpdateProjectRequest>,
) -> AxumResult<Json<ApiResponse<ProjectDto>>> {
    // Validate the request
    if let Err(errors) = request.validate() {
        let validation_errors = errors.into_iter()
            .map(|msg| crate::errors::ValidationFieldError::new("general", &msg))
            .collect();
        let api_error = ApiError::validation_error("Invalid project update data", validation_errors);
        return Err(api_error.into());
    }
    
    // Check if there are any changes
    if !request.has_changes() {
        let api_error = ApiError::bad_request("No changes provided in update request", None);
        return Err(api_error.into());
    }
    
    // TODO: Check if project exists and implement actual update in repository
    
    if project_id.to_string().starts_with("00000000") {
        let api_error = ApiError::not_found("Project", &project_id.to_string());
        return Err(api_error.into());
    }
    
    // Mock response - merge existing data with updates
    let now = chrono::Utc::now();
    let updated_project = ProjectDto {
        id: project_id,
        name: request.name.unwrap_or_else(|| "Sample Project".to_string()),
        description: request.description.or_else(|| Some("Updated description".to_string())),
        status: request.status.unwrap_or(crate::dto::projects::ProjectStatus::Active),
        tags: request.tags.unwrap_or_else(|| vec!["development".to_string(), "updated".to_string()]),
        metadata: request.metadata.unwrap_or_default(),
        created_at: now - chrono::Duration::days(7), // Keep original
        updated_at: now, // Update timestamp
        topic_count: 4,
        completed_actions: 8,
        total_actions: 12,
        completion_percentage: 66.7,
    };
    
    Ok(Json(ApiResponse::success_with_message(
        updated_project,
        "Project updated successfully"
    )))
}

/// Delete a project
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{project_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project unique identifier")
    ),
    responses(
        (status = 200, description = "Project deleted successfully", body = ApiResponse<()>),
        (status = 404, description = "Project not found", body = ApiError),
        (status = 409, description = "Project cannot be deleted (has dependencies)", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "projects"
)]
pub async fn delete_project(
    State(_state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> AxumResult<Json<ApiResponse<()>>> {
    // TODO: Check if project exists and has no dependencies
    
    if project_id.to_string().starts_with("00000000") {
        let api_error = ApiError::not_found("Project", &project_id.to_string());
        return Err(api_error.into());
    }
    
    // Check for dependencies (mock logic)
    if project_id.to_string().contains("aaaa") {
        let api_error = ApiError::conflict("Project", 
            "Cannot delete project that has active topics or actions");
        return Err(api_error.into());
    }
    
    // TODO: Implement actual project deletion in repository
    
    Ok(Json(ApiResponse::message_only("Project deleted successfully")))
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add unit tests for handlers once repository layer is integrated
    
    #[test]
    fn test_app_state_creation() {
        let state = AppState {
            project_service: Arc::new(()),
        };
        // Just verify we can create the state
        assert!(Arc::strong_count(&state.project_service) >= 1);
    }
}
