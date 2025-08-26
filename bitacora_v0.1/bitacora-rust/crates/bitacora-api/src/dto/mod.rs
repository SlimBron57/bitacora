//! DTO module exports for Bitacora API

pub mod common;
pub mod projects;
pub mod topics;
pub mod actions;
pub mod sparks;

// Re-exports for convenience
pub use common::{ApiResponse, PaginationParams, PaginatedResponse, HealthResponse, FilterParams};
pub use projects::{ProjectDto, CreateProjectRequest, UpdateProjectRequest};
pub use topics::{TopicDto, CreateTopicRequest, UpdateTopicRequest};
pub use actions::{ActionDto, CreateActionRequest, UpdateActionRequest};
pub use sparks::{SparkDto, CreateSparkRequest};
