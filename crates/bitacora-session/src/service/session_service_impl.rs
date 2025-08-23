use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, info, warn, error};
use chrono::{DateTime, Utc};

use async_trait::async_trait;
use bitacora_core::models::{Session, Action, SessionId, ActionId, SessionStatus};
use bitacora_core::prelude::*;

use crate::config::SessionConfig;
use crate::errors::SessionError;
use crate::service::{SessionService, SessionStats, SessionMetrics};
use crate::state::{SessionState, SessionStateManager};
use crate::context::{SessionContext, SessionContextManager};
use crate::events::{SessionEvent, SessionEventManager};
use crate::integration::IntegrationLayer;

/// Main implementation of the SessionService trait
pub struct SessionServiceImpl {
    config: SessionConfig,
    state_manager: Arc<SessionStateManager>,
    context_manager: Arc<SessionContextManager>,
    event_manager: Arc<SessionEventManager>,
    integration_layer: Arc<IntegrationLayer>,
    active_sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    session_registry: Arc<RwLock<HashMap<SessionId, SessionMetadata>>>,
}

/// Internal metadata for session tracking
#[derive(Debug, Clone)]
struct SessionMetadata {
    session: Session,
    state: SessionState,
    context: SessionContext,
    last_activity: DateTime<Utc>,
    statistics: SessionStats,
}

impl SessionServiceImpl {
    /// Create a new SessionService instance
    pub async fn new(config: SessionConfig) -> Result<Self, SessionError> {
        info!("Initializing SessionService with config: {:?}", config);

        // Initialize component managers
        let state_manager = Arc::new(
            SessionStateManager::new(config.state_persistence.clone()).await
                .map_err(|e| SessionError::configuration_error(format!("State manager init failed: {}", e)))?
        );

        let context_manager = Arc::new(
            SessionContextManager::new(config.context.clone()).await
                .map_err(|e| SessionError::configuration_error(format!("Context manager init failed: {}", e)))?
        );

        let event_manager = Arc::new(
            SessionEventManager::new(config.events.clone()).await
                .map_err(|e| SessionError::configuration_error(format!("Event manager init failed: {}", e)))?
        );

        let integration_layer = Arc::new(
            IntegrationLayer::new(config.integrations.clone()).await
                .map_err(|e| SessionError::configuration_error(format!("Integration layer init failed: {}", e)))?
        );

        // Initialize session tracking structures
        let active_sessions = Arc::new(RwLock::new(HashMap::new()));
        let session_registry = Arc::new(RwLock::new(HashMap::new()));

        let service = Self {
            config,
            state_manager,
            context_manager,
            event_manager,
            integration_layer,
            active_sessions,
            session_registry,
        };

        // Perform recovery if enabled
        if service.config.lifecycle.enable_recovery {
            service.perform_recovery().await?;
        }

        info!("SessionService initialized successfully");
        Ok(service)
    }

    /// Perform session recovery on startup
    async fn perform_recovery(&self) -> Result<(), SessionError> {
        debug!("Starting session recovery process");

        let interrupted_sessions = self.state_manager.find_interrupted_sessions().await?;
        
        if !interrupted_sessions.is_empty() {
            info!("Found {} interrupted sessions, recovering...", interrupted_sessions.len());
            
            for session_id in interrupted_sessions {
                if let Err(e) = self.recover_session(&session_id).await {
                    error!("Failed to recover session {}: {}", session_id, e);
                }
            }
        }

        debug!("Session recovery process completed");
        Ok(())
    }

    /// Recover a specific interrupted session
    async fn recover_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        debug!("Recovering session: {}", session_id);

        // Load session state and context
        let state = self.state_manager.load_session_state(session_id).await?;
        let context = self.context_manager.load_session_context(session_id).await?;

        // Recreate session metadata
        let session = Session::builder()
            .session_id(session_id.clone())
            .status(SessionStatus::Paused) // Recovered sessions start paused
            .build();

        let metadata = SessionMetadata {
            session: session.clone(),
            state,
            context,
            last_activity: Utc::now(),
            statistics: SessionStats::default(),
        };

        // Register the recovered session
        self.session_registry.write().await.insert(session_id.clone(), metadata);

        // Emit recovery event
        self.event_manager.emit_event(SessionEvent::SessionRecovered {
            session_id: session_id.clone(),
            timestamp: Utc::now(),
        }).await?;

        info!("Successfully recovered session: {}", session_id);
        Ok(())
    }

    /// Generate a new unique session ID
    async fn generate_session_id(&self) -> SessionId {
        // Use timestamp + UUID for uniqueness
        let timestamp = bitacora_timestamp::generate_timestamp();
        let uuid = uuid::Uuid::new_v4();
        SessionId::from(format!("{}_{}", timestamp, uuid.simple()))
    }

    /// Validate session state transition
    fn validate_state_transition(
        current_status: &SessionStatus,
        target_status: &SessionStatus,
    ) -> Result<(), SessionError> {
        use SessionStatus::*;

        let valid_transition = match (current_status, target_status) {
            // From Created
            (Created, Active) => true,
            // From Active
            (Active, Paused) => true,
            (Active, Completed) => true,
            // From Paused
            (Paused, Active) => true,
            (Paused, Completed) => true,
            // Self-transitions (updates)
            (status, target) if status == target => true,
            // All other transitions are invalid
            _ => false,
        };

        if !valid_transition {
            return Err(SessionError::invalid_state_transition(
                current_status.to_string(),
                target_status.to_string(),
            ));
        }

        Ok(())
    }

    /// Update session metadata after state change
    async fn update_session_metadata(
        &self,
        session_id: &SessionId,
        new_status: SessionStatus,
    ) -> Result<(), SessionError> {
        let mut registry = self.session_registry.write().await;
        
        if let Some(metadata) = registry.get_mut(session_id) {
            metadata.session.status = new_status;
            metadata.last_activity = Utc::now();
            
            // Update statistics
            self.update_session_statistics(&mut metadata.statistics, &new_status).await;
            
            Ok(())
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    /// Update session statistics
    async fn update_session_statistics(&self, stats: &mut SessionStats, status: &SessionStatus) {
        match status {
            SessionStatus::Active => {
                // Will be updated by background task
            },
            SessionStatus::Paused => {
                stats.pause_count += 1;
            },
            SessionStatus::Completed => {
                // Final statistics calculation
            },
            _ => {}
        }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(
        &self,
        description: &str,
        project_id: Option<String>,
    ) -> Result<Session, SessionError> {
        info!("Creating new session: {}", description);

        let session_id = self.generate_session_id().await;
        
        let session = Session::builder()
            .session_id(session_id.clone())
            .description(description.to_string())
            .project_id(project_id)
            .status(SessionStatus::Created)
            .created_at(Utc::now())
            .build();

        // Initialize session context and state
        let context = SessionContext::new(&session_id);
        let state = SessionState::new(&session_id);

        // Create session metadata
        let metadata = SessionMetadata {
            session: session.clone(),
            state: state.clone(),
            context: context.clone(),
            last_activity: Utc::now(),
            statistics: SessionStats::default(),
        };

        // Store in registry
        self.session_registry.write().await.insert(session_id.clone(), metadata);

        // Persist state and context
        self.state_manager.save_session_state(&session_id, &state).await?;
        self.context_manager.save_session_context(&session_id, &context).await?;

        // Emit creation event
        self.event_manager.emit_event(SessionEvent::SessionCreated {
            session_id: session_id.clone(),
            description: description.to_string(),
            timestamp: Utc::now(),
        }).await?;

        info!("Session created successfully: {}", session_id);
        Ok(session)
    }

    async fn start_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        info!("Starting session: {}", session_id);

        // Get current session state
        let current_session = {
            let registry = self.session_registry.read().await;
            registry.get(session_id)
                .ok_or_else(|| SessionError::session_not_found(session_id.to_string()))?
                .session.clone()
        };

        // Validate state transition
        Self::validate_state_transition(&current_session.status, &SessionStatus::Active)?;

        // Check if there's already an active session
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.is_empty() {
            return Err(SessionError::session_already_active(
                active_sessions.keys().next().unwrap().to_string()
            ));
        }
        drop(active_sessions);

        // Start integration services
        self.integration_layer.start_session(session_id, &current_session).await
            .map_err(|e| SessionError::integration_error("integration_layer", e.to_string()))?;

        // Update session status
        self.update_session_metadata(session_id, SessionStatus::Active).await?;

        // Add to active sessions
        self.active_sessions.write().await.insert(session_id.clone(), current_session);

        // Emit start event
        self.event_manager.emit_event(SessionEvent::SessionStarted {
            session_id: session_id.clone(),
            timestamp: Utc::now(),
        }).await?;

        info!("Session started successfully: {}", session_id);
        Ok(())
    }

    async fn pause_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        info!("Pausing session: {}", session_id);

        // Verify session is active
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.contains_key(session_id) {
            return Err(SessionError::session_not_active(session_id.to_string()));
        }
        drop(active_sessions);

        // Pause integration services
        self.integration_layer.pause_session(session_id).await
            .map_err(|e| SessionError::integration_error("integration_layer", e.to_string()))?;

        // Update session status
        self.update_session_metadata(session_id, SessionStatus::Paused).await?;

        // Remove from active sessions
        self.active_sessions.write().await.remove(session_id);

        // Emit pause event
        self.event_manager.emit_event(SessionEvent::SessionPaused {
            session_id: session_id.clone(),
            timestamp: Utc::now(),
        }).await?;

        info!("Session paused successfully: {}", session_id);
        Ok(())
    }

    async fn resume_session(&self, session_id: &SessionId) -> Result<(), SessionError> {
        info!("Resuming session: {}", session_id);

        // Get current session
        let current_session = {
            let registry = self.session_registry.read().await;
            registry.get(session_id)
                .ok_or_else(|| SessionError::session_not_found(session_id.to_string()))?
                .session.clone()
        };

        // Validate state transition
        Self::validate_state_transition(&current_session.status, &SessionStatus::Active)?;

        // Check if there's already an active session
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.is_empty() {
            return Err(SessionError::session_already_active(
                active_sessions.keys().next().unwrap().to_string()
            ));
        }
        drop(active_sessions);

        // Resume integration services
        self.integration_layer.resume_session(session_id, &current_session).await
            .map_err(|e| SessionError::integration_error("integration_layer", e.to_string()))?;

        // Update session status
        self.update_session_metadata(session_id, SessionStatus::Active).await?;

        // Add to active sessions
        self.active_sessions.write().await.insert(session_id.clone(), current_session);

        // Emit resume event
        self.event_manager.emit_event(SessionEvent::SessionResumed {
            session_id: session_id.clone(),
            timestamp: Utc::now(),
        }).await?;

        info!("Session resumed successfully: {}", session_id);
        Ok(())
    }

    async fn end_session(
        &self,
        session_id: &SessionId,
        summary: Option<&str>,
    ) -> Result<(), SessionError> {
        info!("Ending session: {}", session_id);

        // Get current session
        let current_session = {
            let registry = self.session_registry.read().await;
            registry.get(session_id)
                .ok_or_else(|| SessionError::session_not_found(session_id.to_string()))?
                .session.clone()
        };

        // End integration services
        self.integration_layer.end_session(session_id, summary).await
            .map_err(|e| SessionError::integration_error("integration_layer", e.to_string()))?;

        // Update session status
        self.update_session_metadata(session_id, SessionStatus::Completed).await?;

        // Remove from active sessions
        self.active_sessions.write().await.remove(session_id);

        // Emit end event
        self.event_manager.emit_event(SessionEvent::SessionEnded {
            session_id: session_id.clone(),
            summary: summary.map(|s| s.to_string()),
            timestamp: Utc::now(),
        }).await?;

        info!("Session ended successfully: {}", session_id);
        Ok(())
    }

    async fn get_active_session(&self) -> Result<Option<Session>, SessionError> {
        let active_sessions = self.active_sessions.read().await;
        Ok(active_sessions.values().next().cloned())
    }

    async fn get_session(&self, session_id: &SessionId) -> Result<Option<Session>, SessionError> {
        let registry = self.session_registry.read().await;
        Ok(registry.get(session_id).map(|metadata| metadata.session.clone()))
    }

    async fn list_sessions(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Session>, SessionError> {
        let registry = self.session_registry.read().await;
        let sessions: Vec<Session> = registry.values()
            .map(|metadata| metadata.session.clone())
            .collect();

        let offset = offset.unwrap_or(0);
        let end = if let Some(limit) = limit {
            std::cmp::min(offset + limit, sessions.len())
        } else {
            sessions.len()
        };

        Ok(sessions[offset..end].to_vec())
    }

    async fn search_sessions(&self, query: &str) -> Result<Vec<Session>, SessionError> {
        let registry = self.session_registry.read().await;
        let sessions: Vec<Session> = registry.values()
            .filter(|metadata| {
                metadata.session.description
                    .as_ref()
                    .map(|desc| desc.to_lowercase().contains(&query.to_lowercase()))
                    .unwrap_or(false)
            })
            .map(|metadata| metadata.session.clone())
            .collect();

        Ok(sessions)
    }

    // Implement remaining methods with placeholder implementations
    async fn get_session_state(&self, session_id: &SessionId) -> Result<SessionState, SessionError> {
        self.state_manager.load_session_state(session_id).await
    }

    async fn update_session_state(
        &self,
        session_id: &SessionId,
        state: SessionState,
    ) -> Result<(), SessionError> {
        self.state_manager.save_session_state(session_id, &state).await
    }

    async fn get_session_context(
        &self,
        session_id: &SessionId,
    ) -> Result<SessionContext, SessionError> {
        self.context_manager.load_session_context(session_id).await
    }

    async fn update_session_context(
        &self,
        session_id: &SessionId,
        context: SessionContext,
    ) -> Result<(), SessionError> {
        self.context_manager.save_session_context(session_id, &context).await
    }

    async fn record_action(
        &self,
        _session_id: &SessionId,
        _action: Action,
    ) -> Result<ActionId, SessionError> {
        todo!("Implement action recording")
    }

    async fn get_session_actions(
        &self,
        _session_id: &SessionId,
    ) -> Result<Vec<Action>, SessionError> {
        todo!("Implement session actions retrieval")
    }

    async fn update_action(
        &self,
        _session_id: &SessionId,
        _action_id: &ActionId,
        _action: Action,
    ) -> Result<(), SessionError> {
        todo!("Implement action updating")
    }

    async fn subscribe_events(
        &self,
        session_id: &SessionId,
    ) -> Result<tokio::sync::mpsc::Receiver<SessionEvent>, SessionError> {
        self.event_manager.subscribe_session_events(session_id).await
    }

    async fn get_session_events(
        &self,
        session_id: &SessionId,
        limit: Option<usize>,
    ) -> Result<Vec<SessionEvent>, SessionError> {
        self.event_manager.get_session_events(session_id, limit).await
    }

    async fn recover_interrupted_sessions(&self) -> Result<Vec<Session>, SessionError> {
        let interrupted_session_ids = self.state_manager.find_interrupted_sessions().await?;
        let mut recovered_sessions = Vec::new();

        for session_id in interrupted_session_ids {
            if let Ok(()) = self.recover_session(&session_id).await {
                if let Some(session) = self.get_session(&session_id).await? {
                    recovered_sessions.push(session);
                }
            }
        }

        Ok(recovered_sessions)
    }

    async fn needs_recovery(&self) -> Result<bool, SessionError> {
        let interrupted_sessions = self.state_manager.find_interrupted_sessions().await?;
        Ok(!interrupted_sessions.is_empty())
    }

    async fn validate_session(&self, session_id: &SessionId) -> Result<bool, SessionError> {
        // Check if session exists in registry
        let exists_in_registry = self.session_registry.read().await.contains_key(session_id);
        
        if !exists_in_registry {
            return Ok(false);
        }

        // Validate state consistency
        let state_valid = self.state_manager.validate_session_state(session_id).await?;
        let context_valid = self.context_manager.validate_session_context(session_id).await?;

        Ok(state_valid && context_valid)
    }

    async fn cleanup_orphaned_data(&self) -> Result<u32, SessionError> {
        let mut cleanup_count = 0;

        // Cleanup orphaned state files
        cleanup_count += self.state_manager.cleanup_orphaned_states().await?;

        // Cleanup orphaned context data
        cleanup_count += self.context_manager.cleanup_orphaned_contexts().await?;

        info!("Cleaned up {} orphaned data items", cleanup_count);
        Ok(cleanup_count)
    }

    async fn get_session_stats(&self, session_id: &SessionId) -> Result<SessionStats, SessionError> {
        let registry = self.session_registry.read().await;
        registry.get(session_id)
            .map(|metadata| metadata.statistics.clone())
            .ok_or_else(|| SessionError::session_not_found(session_id.to_string()))
    }

    async fn get_session_metrics(&self) -> Result<SessionMetrics, SessionError> {
        let registry = self.session_registry.read().await;
        
        let total_sessions = registry.len() as u32;
        let active_sessions = self.active_sessions.read().await.len() as u32;
        let completed_sessions = registry.values()
            .filter(|m| m.session.status == SessionStatus::Completed)
            .count() as u32;

        // Calculate average duration (placeholder implementation)
        let average_session_duration = std::time::Duration::from_secs(2 * 60 * 60); // 2 hours
        let total_development_time = std::time::Duration::from_secs(
            (completed_sessions as u64) * 2 * 60 * 60
        );

        Ok(SessionMetrics {
            total_sessions,
            active_sessions,
            completed_sessions,
            average_session_duration,
            total_development_time,
            most_productive_hour: Some(14), // 2 PM
            most_active_day: Some("Wednesday".to_string()),
        })
    }
}

impl Default for SessionStats {
    fn default() -> Self {
        Self {
            total_duration: std::time::Duration::from_secs(0),
            active_duration: std::time::Duration::from_secs(0),
            action_count: 0,
            pause_count: 0,
            git_operations: 0,
            context_changes: 0,
        }
    }
}
