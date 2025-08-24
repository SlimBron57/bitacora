use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use chrono::{DateTime, Utc};

use async_trait::async_trait;
use bitacora_core::models::{Session, Action, SessionId, ActionId, SessionStatus};

use crate::config::SessionConfig;
use crate::errors::SessionError;
use crate::service::{SessionService, SessionStats, SessionMetrics};
use crate::state::SessionState;
use crate::context::SessionContext;
use crate::events::SessionEvent;

/// Simple in-memory implementation of SessionService
pub struct SessionServiceImpl {
    config: SessionConfig,
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    active_sessions: Arc<RwLock<HashMap<SessionId, DateTime<Utc>>>>,
}

impl SessionServiceImpl {
    pub async fn new(config: SessionConfig) -> std::result::Result<Self, SessionError> {
        Ok(Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn shutdown(&self) -> std::result::Result<(), SessionError> {
        info!("Shutting down session service");
        
        // End all active sessions
        let active_sessions = self.active_sessions.read().await;
        let session_ids: Vec<_> = active_sessions.keys().cloned().collect();
        drop(active_sessions);

        for session_id in session_ids {
            if let Err(e) = self.end_session(&session_id, Some("Service shutdown")).await {
                warn!("Failed to end session {} during shutdown: {}", session_id, e);
            }
        }
        
        Ok(())
    }

    fn validate_status_transition(
        current_status: &SessionStatus,
        target_status: &SessionStatus,
    ) -> bool {
        use SessionStatus::*;
        
        match (current_status, target_status) {
            (Created, Active) => true,
            (Active, Paused) => true,
            (Paused, Active) => true,
            (Active, Completed) => true,
            (Active, Ended) => true,
            (status, target) if status == target => true,
            _ => false,
        }
    }
    
    fn create_invalid_transition_error(
        current_status: &SessionStatus,
        target_status: &SessionStatus,
    ) -> SessionError {
        SessionError::InvalidStateTransition {
            current: current_status.to_string(),
            target: target_status.to_string(),
        }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(
        &self,
        description: &str,
        project_id: Option<String>,
    ) -> std::result::Result<Session, SessionError> {
        let session = Session::builder()
            .description(Some(description.to_string()))
            .project_id(project_id.map(|id| uuid::Uuid::parse_str(&id)).transpose().ok().flatten())
            .status(SessionStatus::Created)
            .build();

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.session_id.clone(), session.clone());

        Ok(session)
    }

    async fn start_session(&self, session_id: &SessionId) -> std::result::Result<(), SessionError> {
        // Check if any other session is active
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.is_empty() {
            return Err(SessionError::SessionAlreadyActive {
                session_id: active_sessions.keys().next().unwrap().to_string(),
            });
        }
        drop(active_sessions);

        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if !Self::validate_status_transition(&session.status, &SessionStatus::Active) {
                return Err(Self::create_invalid_transition_error(&session.status, &SessionStatus::Active));
            }
            
            session.status = SessionStatus::Active;
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.insert(session_id.clone(), Utc::now());
            Ok(())
        } else {
            Err(SessionError::SessionNotFound {
                session_id: session_id.to_string(),
            })
        }
    }

    async fn pause_session(&self, session_id: &SessionId) -> std::result::Result<(), SessionError> {
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.contains_key(session_id) {
            return Err(SessionError::SessionNotActive {
                session_id: session_id.to_string(),
            });
        }
        drop(active_sessions);

        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if !Self::validate_status_transition(&session.status, &SessionStatus::Paused) {
                return Err(Self::create_invalid_transition_error(&session.status, &SessionStatus::Paused));
            }
            
            session.status = SessionStatus::Paused;
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.remove(session_id);
            Ok(())
        } else {
            Err(SessionError::SessionNotFound {
                session_id: session_id.to_string(),
            })
        }
    }

    async fn resume_session(&self, session_id: &SessionId) -> std::result::Result<(), SessionError> {
        // Check if any other session is active
        let active_sessions = self.active_sessions.read().await;
        if !active_sessions.is_empty() {
            return Err(SessionError::SessionAlreadyActive {
                session_id: active_sessions.keys().next().unwrap().to_string(),
            });
        }
        drop(active_sessions);

        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if !Self::validate_status_transition(&session.status, &SessionStatus::Active) {
                return Err(Self::create_invalid_transition_error(&session.status, &SessionStatus::Active));
            }
            
            session.status = SessionStatus::Active;
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.insert(session_id.clone(), Utc::now());
            Ok(())
        } else {
            Err(SessionError::SessionNotFound {
                session_id: session_id.to_string(),
            })
        }
    }

    async fn end_session(
        &self,
        session_id: &SessionId,
        summary: Option<&str>,
    ) -> std::result::Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            if !Self::validate_status_transition(&session.status, &SessionStatus::Completed) {
                return Err(Self::create_invalid_transition_error(&session.status, &SessionStatus::Completed));
            }
            
            session.status = SessionStatus::Completed;
            session.ended_at = Some(Utc::now());
            if let Some(summary) = summary {
                session.description = Some(summary.to_string());
            }
            
            let mut active_sessions = self.active_sessions.write().await;
            active_sessions.remove(session_id);
            Ok(())
        } else {
            Err(SessionError::SessionNotFound {
                session_id: session_id.to_string(),
            })
        }
    }

    async fn get_active_session(&self) -> std::result::Result<Option<Session>, SessionError> {
        let active_sessions = self.active_sessions.read().await;
        if let Some(session_id) = active_sessions.keys().next() {
            let sessions = self.sessions.read().await;
            Ok(sessions.get(session_id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn get_session(&self, session_id: &SessionId) -> std::result::Result<Option<Session>, SessionError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(session_id).cloned())
    }

    async fn list_sessions(
        &self,
        _limit: Option<usize>,
        _offset: Option<usize>,
    ) -> std::result::Result<Vec<Session>, SessionError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.values().cloned().collect())
    }

    async fn search_sessions(&self, query: &str) -> std::result::Result<Vec<Session>, SessionError> {
        let sessions = self.sessions.read().await;
        let results: Vec<Session> = sessions
            .values()
            .filter(|session| {
                session
                    .description
                    .as_ref()
                    .map(|desc| desc.contains(query))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        Ok(results)
    }

    async fn get_session_state(&self, _session_id: &SessionId) -> std::result::Result<SessionState, SessionError> {
        Err(SessionError::ConfigurationError {
            message: "State management not implemented in simplified version".to_string(),
        })
    }

    async fn set_session_state(
        &self,
        _session_id: &SessionId,
        _state: SessionState,
    ) -> std::result::Result<(), SessionError> {
        Err(SessionError::ConfigurationError {
            message: "State management not implemented in simplified version".to_string(),
        })
    }

    async fn get_session_context(
        &self,
        _session_id: &SessionId,
    ) -> std::result::Result<SessionContext, SessionError> {
        Err(SessionError::ConfigurationError {
            message: "Context management not implemented in simplified version".to_string(),
        })
    }

    async fn update_session_context(
        &self,
        _session_id: &SessionId,
        _context: SessionContext,
    ) -> std::result::Result<(), SessionError> {
        Err(SessionError::ConfigurationError {
            message: "Context management not implemented in simplified version".to_string(),
        })
    }

    async fn record_action(
        &self,
        _session_id: &SessionId,
        _action: Action,
    ) -> std::result::Result<ActionId, SessionError> {
        Err(SessionError::ConfigurationError {
            message: "Action recording not implemented in simplified version".to_string(),
        })
    }

    async fn get_session_actions(
        &self,
        _session_id: &SessionId,
    ) -> std::result::Result<Vec<Action>, SessionError> {
        Ok(Vec::new())
    }

    async fn remove_action(
        &self,
        _session_id: &SessionId,
        _action_id: &ActionId,
    ) -> std::result::Result<(), SessionError> {
        Err(SessionError::ConfigurationError {
            message: "Action management not implemented in simplified version".to_string(),
        })
    }

    async fn subscribe_to_events(
        &self,
    ) -> std::result::Result<tokio::sync::mpsc::Receiver<SessionEvent>, SessionError> {
        Err(SessionError::ConfigurationError {
            message: "Event system not implemented in simplified version".to_string(),
        })
    }

    async fn get_session_events(
        &self,
        _session_id: &SessionId,
    ) -> std::result::Result<Vec<SessionEvent>, SessionError> {
        Ok(Vec::new())
    }

    async fn recover_interrupted_sessions(&self) -> std::result::Result<Vec<Session>, SessionError> {
        // In this simplified version, recovery is not needed
        Ok(Vec::new())
    }

    async fn needs_recovery(&self) -> std::result::Result<bool, SessionError> {
        Ok(false)
    }

    async fn validate_session(&self, session_id: &SessionId) -> std::result::Result<bool, SessionError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.contains_key(session_id))
    }

    async fn cleanup_orphaned_data(&self) -> std::result::Result<u32, SessionError> {
        Ok(0)
    }

    async fn get_session_stats(&self, _session_id: &SessionId) -> std::result::Result<SessionStats, SessionError> {
        Ok(SessionStats::default())
    }

    async fn get_session_metrics(&self) -> std::result::Result<SessionMetrics, SessionError> {
        let sessions = self.sessions.read().await;
        let active_sessions = self.active_sessions.read().await;
        
        let total_sessions = sessions.len() as u32;
        let active_sessions_count = active_sessions.len() as u32;
        let completed_sessions = sessions
            .values()
            .filter(|s| s.status == SessionStatus::Completed)
            .count() as u32;

        Ok(SessionMetrics {
            total_sessions,
            active_sessions: active_sessions_count,
            completed_sessions,
            average_session_duration: std::time::Duration::from_secs(0),
            total_development_time: std::time::Duration::from_secs(0),
            most_productive_hour: None,
            most_active_day: None,
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
