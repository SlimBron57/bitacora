//! DTO module exports for Bitacora API

pub mod common;
pub mod health;
pub mod projects;
pub mod topics;
pub mod actions;
pub mod sparks;

// Re-exports for convenience
pub use common::{ApiResponse, PaginationParams, PaginatedResponse, FilterParams};
pub use health::{HealthStatus, HealthResponse};
pub use projects::{ProjectDto, CreateProjectRequest, UpdateProjectRequest};
pub use topics::{TopicDto, CreateTopicRequest, UpdateTopicRequest};
pub use actions::{ActionDto, CreateActionRequest, UpdateActionRequest};
pub use sparks::{SparkDto, CreateSparkRequest};
