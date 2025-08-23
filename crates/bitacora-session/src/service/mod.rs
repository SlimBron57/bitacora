use async_trait::async_trait;
use bitacora_core::models::{Session, Action, SessionId, ActionId};
use crate::errors::SessionError;
use crate::state::SessionState;
use crate::context::SessionContext;
use crate::events::SessionEvent;

// Re-export simple implementation for now
mod simple_session_service;
pub use simple_session_service::SessionServiceImpl;

/// Main trait defining session management operations
#[async_trait]
pub trait SessionService: Send + Sync {
    // === Session Lifecycle Management ===
    
    /// Create a new development session
    async fn create_session(
        &self,
        description: &str,
        project_id: Option<String>,
    ) -> Result<Session, SessionError>;

    /// Start an existing session
    async fn start_session(&self, session_id: &SessionId) -> Result<(), SessionError>;

    /// Pause an active session
    async fn pause_session(&self, session_id: &SessionId) -> Result<(), SessionError>;

    /// Resume a paused session
    async fn resume_session(&self, session_id: &SessionId) -> Result<(), SessionError>;

    /// End a session with optional summary
    async fn end_session(
        &self,
        session_id: &SessionId,
        summary: Option<&str>,
    ) -> Result<(), SessionError>;

    // === Session Query Operations ===

    /// Get current active session
    async fn get_active_session(&self) -> Result<Option<Session>, SessionError>;

    /// Get session by ID
    async fn get_session(&self, session_id: &SessionId) -> Result<Option<Session>, SessionError>;

    /// List sessions with optional filters
    async fn list_sessions(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Session>, SessionError>;

    /// Search sessions by description or metadata
    async fn search_sessions(&self, query: &str) -> Result<Vec<Session>, SessionError>;

    // === Session State Management ===

    /// Get current session state
    async fn get_session_state(&self, session_id: &SessionId) -> Result<SessionState, SessionError>;

    /// Update session state
    async fn update_session_state(
        &self,
        session_id: &SessionId,
        state: SessionState,
    ) -> Result<(), SessionError>;

    /// Get session context (working directory, environment, etc.)
    async fn get_session_context(
        &self,
        session_id: &SessionId,
    ) -> Result<SessionContext, SessionError>;

    /// Update session context
    async fn update_session_context(
        &self,
        session_id: &SessionId,
        context: SessionContext,
    ) -> Result<(), SessionError>;

    // === Action Management within Sessions ===

    /// Record an action within a session
    async fn record_action(
        &self,
        session_id: &SessionId,
        action: Action,
    ) -> Result<ActionId, SessionError>;

    /// Get actions for a session
    async fn get_session_actions(
        &self,
        session_id: &SessionId,
    ) -> Result<Vec<Action>, SessionError>;

    /// Update an action within a session
    async fn update_action(
        &self,
        session_id: &SessionId,
        action_id: &ActionId,
        action: Action,
    ) -> Result<(), SessionError>;

    // === Session Events ===

    /// Subscribe to session events
    async fn subscribe_events(
        &self,
        session_id: &SessionId,
    ) -> Result<tokio::sync::mpsc::Receiver<SessionEvent>, SessionError>;

    /// Get session event history
    async fn get_session_events(
        &self,
        session_id: &SessionId,
        limit: Option<usize>,
    ) -> Result<Vec<SessionEvent>, SessionError>;

    // === Session Recovery ===

    /// Recover sessions that were interrupted
    async fn recover_interrupted_sessions(&self) -> Result<Vec<Session>, SessionError>;

    /// Check if session recovery is needed
    async fn needs_recovery(&self) -> Result<bool, SessionError>;

    // === Session Validation ===

    /// Validate session state consistency
    async fn validate_session(&self, session_id: &SessionId) -> Result<bool, SessionError>;

    /// Cleanup orphaned session data
    async fn cleanup_orphaned_data(&self) -> Result<u32, SessionError>;

    // === Statistics and Analytics ===

    /// Get session statistics
    async fn get_session_stats(&self, session_id: &SessionId) -> Result<SessionStats, SessionError>;

    /// Get overall session metrics
    async fn get_session_metrics(&self) -> Result<SessionMetrics, SessionError>;
}

/// Session statistics for a specific session
#[derive(Debug, Clone)]
pub struct SessionStats {
    pub total_duration: std::time::Duration,
    pub active_duration: std::time::Duration,
    pub action_count: u32,
    pub pause_count: u32,
    pub git_operations: u32,
    pub context_changes: u32,
}

/// Overall session metrics across all sessions
#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub total_sessions: u32,
    pub active_sessions: u32,
    pub completed_sessions: u32,
    pub average_session_duration: std::time::Duration,
    pub total_development_time: std::time::Duration,
    pub most_productive_hour: Option<u8>,
    pub most_active_day: Option<String>,
}
