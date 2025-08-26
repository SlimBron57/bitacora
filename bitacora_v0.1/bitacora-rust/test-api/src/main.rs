use axum::{
    extract::{Path, Query},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Basic DTOs
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub uptime: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]  
pub struct Topic {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Action {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub action_type: String, // "task", "reminder", "milestone", etc.
    pub status: String, // "pending", "in_progress", "completed", "cancelled"
    pub priority: String, // "low", "medium", "high", "urgent"
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Spark {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub spark_type: String, // "idea", "insight", "question", "observation"
    pub tags: Vec<String>,
    pub project_id: Option<Uuid>,
    pub topic_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct Pagination {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// Handlers
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "API is healthy", body = ApiResponse<HealthStatus>)
    )
)]
async fn health_check() -> Json<ApiResponse<HealthStatus>> {
    let status = HealthStatus {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: "0.1.0".to_string(),
        uptime: "1h".to_string(),
    };
    Json(ApiResponse::success(status))
}

#[utoipa::path(
    get,
    path = "/projects",
    tag = "Projects",
    responses(
        (status = 200, description = "List of projects", body = ApiResponse<Vec<Project>>)
    )
)]
async fn get_projects(Query(_pagination): Query<Pagination>) -> Json<ApiResponse<Vec<Project>>> {
    let projects = vec![
        Project {
            id: Uuid::new_v4(),
            name: "Test Project 1".to_string(),
            description: Some("A test project".to_string()),
            status: "active".to_string(),
            created_at: Utc::now(),
        },
        Project {
            id: Uuid::new_v4(),
            name: "Test Project 2".to_string(),
            description: Some("Another test project".to_string()),
            status: "planning".to_string(),
            created_at: Utc::now(),
        }
    ];
    Json(ApiResponse::success(projects))
}

#[utoipa::path(
    get,
    path = "/projects/{id}/topics",
    tag = "Topics",
    responses(
        (status = 200, description = "List of topics", body = ApiResponse<Vec<Topic>>)
    )
)]
async fn get_topics(Path(project_id): Path<Uuid>) -> Json<ApiResponse<Vec<Topic>>> {
    let topics = vec![
        Topic {
            id: Uuid::new_v4(),
            project_id,
            title: "API Development".to_string(),
            description: Some("Develop REST API".to_string()),
            status: "in_progress".to_string(),
            created_at: Utc::now(),
        },
        Topic {
            id: Uuid::new_v4(),
            project_id,
            title: "Database Design".to_string(),
            description: Some("Design database schema".to_string()),
            status: "completed".to_string(),
            created_at: Utc::now(),
        }
    ];
    Json(ApiResponse::success(topics))
}

#[utoipa::path(
    get,
    path = "/topics/{id}/actions",
    tag = "Actions",
    responses(
        (status = 200, description = "List of actions for a topic", body = ApiResponse<Vec<Action>>)
    )
)]
async fn get_actions(Path(topic_id): Path<Uuid>) -> Json<ApiResponse<Vec<Action>>> {
    let actions = vec![
        Action {
            id: Uuid::new_v4(),
            topic_id,
            title: "Setup development environment".to_string(),
            description: Some("Configure Rust project with dependencies".to_string()),
            action_type: "task".to_string(),
            status: "completed".to_string(),
            priority: "high".to_string(),
            due_date: Some(Utc::now()),
            created_at: Utc::now(),
            completed_at: Some(Utc::now()),
        },
        Action {
            id: Uuid::new_v4(),
            topic_id,
            title: "Implement REST endpoints".to_string(),
            description: Some("Create CRUD operations for all entities".to_string()),
            action_type: "task".to_string(),
            status: "in_progress".to_string(),
            priority: "high".to_string(),
            due_date: Some(Utc::now()),
            created_at: Utc::now(),
            completed_at: None,
        },
        Action {
            id: Uuid::new_v4(),
            topic_id,
            title: "Review API documentation".to_string(),
            description: Some("Ensure Swagger docs are complete".to_string()),
            action_type: "milestone".to_string(),
            status: "pending".to_string(),
            priority: "medium".to_string(),
            due_date: Some(Utc::now()),
            created_at: Utc::now(),
            completed_at: None,
        }
    ];
    Json(ApiResponse::success(actions))
}

#[utoipa::path(
    get,
    path = "/sparks",
    tag = "Sparks",
    responses(
        (status = 200, description = "List of sparks", body = ApiResponse<Vec<Spark>>)
    )
)]
async fn get_sparks(Query(_pagination): Query<Pagination>) -> Json<ApiResponse<Vec<Spark>>> {
    let sparks = vec![
        Spark {
            id: Uuid::new_v4(),
            title: "API Performance Optimization".to_string(),
            content: "Consider implementing caching layer for frequently accessed endpoints to improve response times".to_string(),
            spark_type: "idea".to_string(),
            tags: vec!["performance".to_string(), "api".to_string(), "caching".to_string()],
            project_id: Some(Uuid::new_v4()),
            topic_id: None,
            created_at: Utc::now(),
            updated_at: None,
        },
        Spark {
            id: Uuid::new_v4(),
            title: "Database Schema Insight".to_string(),
            content: "MongoDB collections should use compound indexes for project_id + created_at queries".to_string(),
            spark_type: "insight".to_string(),
            tags: vec!["database".to_string(), "mongodb".to_string(), "indexing".to_string()],
            project_id: Some(Uuid::new_v4()),
            topic_id: Some(Uuid::new_v4()),
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
        },
        Spark {
            id: Uuid::new_v4(),
            title: "Error Handling Question".to_string(),
            content: "How should we handle partial failures in batch operations? Should we rollback or continue?".to_string(),
            spark_type: "question".to_string(),
            tags: vec!["error-handling".to_string(), "batch-operations".to_string()],
            project_id: None,
            topic_id: None,
            created_at: Utc::now(),
            updated_at: None,
        }
    ];
    Json(ApiResponse::success(sparks))
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        health_check,
        get_projects,
        get_topics,
        get_actions,
        get_sparks
    ),
    components(
        schemas(
            ApiResponse<HealthStatus>,
            ApiResponse<Vec<Project>>,
            ApiResponse<Vec<Topic>>,
            ApiResponse<Vec<Action>>,
            ApiResponse<Vec<Spark>>,
            HealthStatus,
            Project,
            Topic,
            Action,
            Spark,
            Pagination
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Projects", description = "Project management"),
        (name = "Topics", description = "Topic management"),
        (name = "Actions", description = "Action management"),
        (name = "Sparks", description = "Spark management")
    ),
    info(
        title = "Bitacora Test API",
        description = "Test API for Bitacora system",
        version = "0.1.0"
    )
)]
struct ApiDoc;

// Create router
fn create_app() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/projects", get(get_projects))
        .route("/projects/:id/topics", get(get_topics))
        .route("/topics/:id/actions", get(get_actions))
        .route("/sparks", get(get_sparks))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(
            tower::ServiceBuilder::new()
                .layer(tower_http::cors::CorsLayer::permissive())
                .layer(tower_http::timeout::TimeoutLayer::new(std::time::Duration::from_secs(30)))
        )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = create_app();
    let addr: SocketAddr = "0.0.0.0:3001".parse().unwrap();
    
    println!("🚀 Test API Server starting on http://{}", addr);
    println!("📚 Swagger UI: http://{}/swagger-ui", addr);
    println!("❤️ Health Check: http://{}/health", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
