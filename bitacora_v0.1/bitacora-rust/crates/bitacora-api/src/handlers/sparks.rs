//! Spark API handlers for Bitacora (Transversal Insights Service)
//!
//! Sparks capture insights, learnings, and cross-cutting observations at any level

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Result as AxumResult,
    Json,
};
use uuid::Uuid;

use crate::{
    dto::{
        sparks::{SparkDto, CreateSparkRequest, SparkSummaryDto, SparkAnalyticsDto},
        common::{ApiResponse, PaginationParams, FilterParams},
    },
    errors::{ApiError, ApiResult},
    handlers::projects::AppState,
};

/// Create a new spark insight
#[utoipa::path(
    post,
    path = "/api/v1/sparks",
    request_body = CreateSparkRequest,
    responses(
        (status = 201, description = "Spark created successfully", body = ApiResponse<SparkDto>),
        (status = 400, description = "Invalid spark data", body = ApiError),
        (status = 422, description = "Validation error", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "sparks"
)]
pub async fn create_spark(
    State(_state): State<AppState>,
    Json(request): Json<CreateSparkRequest>,
) -> AxumResult<(StatusCode, Json<ApiResponse<SparkDto>>)> {
    // Validate the request
    if let Err(errors) = request.validate() {
        let validation_errors = errors.into_iter()
            .map(|msg| crate::errors::ValidationFieldError::new("general", &msg))
            .collect();
        let api_error = ApiError::validation_error("Invalid spark data", validation_errors);
        return Err(api_error.into());
    }
    
    // TODO: Implement actual spark creation in repository
    
    // Mock response
    let now = chrono::Utc::now();
    let spark_dto = SparkDto {
        id: Uuid::new_v4(),
        context: request.context.clone(),
        spark_type: request.spark_type.clone(),
        title: request.title.clone(),
        content: request.content.clone(),
        insights: request.insights.clone(),
        impact: request.impact.unwrap_or_default(),
        tags: request.tags.unwrap_or_default(),
        related_entities: request.related_entities.unwrap_or_default(),
        metadata: request.metadata.unwrap_or_default(),
        created_at: now,
        updated_at: now,
        author: request.author.clone(),
        reviewed: false,
        review_notes: None,
        action_items: request.action_items.unwrap_or_default(),
    };
    
    Ok((StatusCode::CREATED, Json(ApiResponse::success_with_message(
        spark_dto, 
        "Spark insight captured successfully"
    ))))
}

/// Get sparks with filtering and pagination
#[utoipa::path(
    get,
    path = "/api/v1/sparks",
    params(
        PaginationParams,
        FilterParams,
        ("context_level" = Option<String>, Query, description = "Filter by context level (project, topic, action, global)"),
        ("spark_type" = Option<String>, Query, description = "Filter by spark type"),
        ("impact" = Option<String>, Query, description = "Filter by impact level"),
        ("reviewed" = Option<bool>, Query, description = "Filter by review status"),
    ),
    responses(
        (status = 200, description = "List of sparks retrieved successfully", 
         body = ApiResponse<Vec<SparkSummaryDto>>),
        (status = 400, description = "Invalid query parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "sparks"
)]
pub async fn get_sparks(
    State(_state): State<AppState>,
    Query(_pagination): Query<PaginationParams>,
    Query(_filters): Query<FilterParams>,
) -> AxumResult<Json<ApiResponse<Vec<SparkSummaryDto>>>> {
    // TODO: Implement actual spark retrieval from repository
    
    // Mock data
    let mock_sparks = vec![
        SparkSummaryDto {
            id: Uuid::new_v4(),
            title: "Better error handling pattern discovered".to_string(),
            spark_type: crate::dto::sparks::SparkType::Learning,
            impact: crate::dto::sparks::SparkImpact::High,
            context: crate::dto::sparks::SparkContext {
                level: crate::dto::sparks::ContextLevel::Action,
                entity_id: Uuid::new_v4(),
                description: "While implementing user validation".to_string(),
            },
            tags: vec!["error-handling".to_string(), "patterns".to_string()],
            reviewed: false,
            created_at: chrono::Utc::now() - chrono::Duration::hours(2),
            author: Some("developer@bitacora.io".to_string()),
        },
    ];
    
    Ok(Json(ApiResponse::success(mock_sparks)))
}

/// Get spark analytics and insights
#[utoipa::path(
    get,
    path = "/api/v1/sparks/analytics",
    responses(
        (status = 200, description = "Spark analytics retrieved successfully", 
         body = ApiResponse<SparkAnalyticsDto>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "sparks"
)]
pub async fn get_spark_analytics(
    State(_state): State<AppState>,
) -> AxumResult<Json<ApiResponse<SparkAnalyticsDto>>> {
    // TODO: Implement actual analytics from repository
    
    // Mock analytics
    let mut sparks_by_type = std::collections::HashMap::new();
    sparks_by_type.insert("learning".to_string(), 12);
    sparks_by_type.insert("idea".to_string(), 8);
    sparks_by_type.insert("solution".to_string(), 5);
    
    let mut sparks_by_impact = std::collections::HashMap::new();
    sparks_by_impact.insert("high".to_string(), 7);
    sparks_by_impact.insert("medium".to_string(), 15);
    sparks_by_impact.insert("low".to_string(), 3);
    
    let mut sparks_by_context = std::collections::HashMap::new();
    sparks_by_context.insert("project".to_string(), 5);
    sparks_by_context.insert("topic".to_string(), 12);
    sparks_by_context.insert("action".to_string(), 8);
    
    let analytics = SparkAnalyticsDto {
        total_sparks: 25,
        sparks_by_type,
        sparks_by_impact,
        sparks_by_context,
        recent_high_impact: vec![], // TODO: Add recent high-impact sparks
        top_tags: vec![
            crate::dto::sparks::TagCount { tag: "patterns".to_string(), count: 8 },
            crate::dto::sparks::TagCount { tag: "performance".to_string(), count: 6 },
            crate::dto::sparks::TagCount { tag: "security".to_string(), count: 4 },
        ],
    };
    
    Ok(Json(ApiResponse::success(analytics)))
}
