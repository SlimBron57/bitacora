//! Repository layer for data access

use async_trait::async_trait;
use bitacora_core::models::*;
use crate::StorageResult;
use chrono::{DateTime, Utc};

pub mod session_repository;
pub mod action_repository;
pub mod project_repository;
pub mod topic_repository;
pub mod spark_repository;

// Re-export implementations
pub use session_repository::MongoSessionRepository;
pub use action_repository::MongoActionRepository;
pub use project_repository::MongoProjectRepository;
pub use topic_repository::MongoTopicRepository;
pub use spark_repository::MongoSparkRepository;

/// Generic repository trait for CRUD operations
#[async_trait]
pub trait Repository<T>: Send + Sync {
    /// Create a new entity
    async fn create(&self, entity: &T) -> StorageResult<()>;
    
    /// Get entity by ID (using string ID)
    async fn get_by_id(&self, id: &str) -> StorageResult<Option<T>>;
    
    /// Update existing entity
    async fn update(&self, entity: &T) -> StorageResult<bool>;
    
    /// Delete entity by ID
    async fn delete(&self, id: &str) -> StorageResult<bool>;
    
    /// List entities with pagination
    async fn list(&self, limit: Option<usize>, offset: Option<usize>) -> StorageResult<Vec<T>>;
}

/// Session-specific repository operations
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Get active session for user
    async fn get_active_session(&self, user_id: &str) -> StorageResult<Option<Session>>;
    
    /// Get sessions by topic ID
    async fn get_by_topic_id(&self, topic_id: &str) -> StorageResult<Vec<Session>>;
    
    /// Get user sessions within date range
    async fn get_user_sessions_in_range(
        &self,
        user_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> StorageResult<Vec<Session>>;
    
    /// End session (set end_time)
    async fn end_session(&self, session_id: &str) -> StorageResult<bool>;
    
    /// Get session statistics for user
    async fn get_user_session_stats(&self, user_id: &str) -> StorageResult<SessionStats>;
}

/// Action-specific repository operations
#[async_trait]
pub trait ActionRepository: Send + Sync {
    /// Get actions for session
    async fn get_by_session_id(&self, session_id: &str) -> StorageResult<Vec<Action>>;
    
    /// Get actions by type
    async fn get_by_action_type(&self, action_type: &ActionType) -> StorageResult<Vec<Action>>;
    
    /// Get user actions within date range
    async fn get_user_actions(
        &self,
        user_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> StorageResult<Vec<Action>>;
    
    /// Get action count by type
    async fn get_action_count_by_type(&self, action_type: &ActionType) -> StorageResult<u64>;
    
    /// Get recent actions
    async fn get_recent_actions(&self, limit: usize) -> StorageResult<Vec<Action>>;
}

/// Project-specific repository operations
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Get user projects
    async fn get_by_user_id(&self, user_id: &str) -> StorageResult<Vec<Project>>;
    
    /// Get projects by status
    async fn get_by_status(&self, status: &ProjectStatus) -> StorageResult<Vec<Project>>;
    
    /// Search projects by name
    async fn search_by_name(&self, name_query: &str) -> StorageResult<Vec<Project>>;
    
    /// Get project statistics
    async fn get_project_stats(&self, project_id: &str) -> StorageResult<ProjectStats>;
    
    /// Get recent projects
    async fn get_recent_projects(&self, limit: usize) -> StorageResult<Vec<Project>>;
    
    /// Update project status
    async fn update_project_status(&self, project_id: &str, status: ProjectStatus) -> StorageResult<bool>;
}

/// Topic repository trait
#[async_trait]
pub trait TopicRepository: Send + Sync {
    /// Get topics by user ID
    async fn get_by_user_id(&self, user_id: &str) -> StorageResult<Vec<Topic>>;
    
    /// Get active topics
    async fn get_active_topics(&self) -> StorageResult<Vec<Topic>>;
    
    /// Get topics by status
    async fn get_by_status(&self, status: &str) -> StorageResult<Vec<Topic>>;
}

/// Spark repository trait
#[async_trait]
pub trait SparkRepository: Send + Sync {
    /// Get sparks by session ID
    async fn get_by_session_id(&self, session_id: &str) -> StorageResult<Vec<Spark>>;
    
    /// Get sparks by tags
    async fn get_by_tags(&self, tags: &[String]) -> StorageResult<Vec<Spark>>;
    
    /// Search sparks by content
    async fn search_content(&self, query: &str) -> StorageResult<Vec<Spark>>;
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_sessions: u64,
    pub total_duration_minutes: u64,
    pub average_session_duration_minutes: f64,
    pub active_sessions: u32,
}

/// Action statistics  
#[derive(Debug, Clone)]
pub struct ActionStats {
    pub total_actions: u64,
    pub actions_this_week: u64,
    pub actions_this_month: u64,
    pub most_common_tags: Vec<(String, u64)>,
    pub actions_by_type: Vec<(ActionType, u64)>,
}

/// Project statistics  
#[derive(Debug, Clone)]
pub struct ProjectStats {
    pub project_id: String,
    pub total_sessions: u64,
    pub total_actions: u64,
    pub last_activity: Option<DateTime<Utc>>,
    pub completion_percentage: f64,
    pub active_contributors: u64,
}

/// Topic statistics (used in session repository)
#[derive(Debug, Clone)]
pub struct TopicStats {
    pub topic_id: String,
    pub session_count: u32,
    pub last_session_date: Option<DateTime<Utc>>,
}
