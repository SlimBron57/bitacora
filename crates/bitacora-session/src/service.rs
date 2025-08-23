//! Session service implementation

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

use crate::config::SessionConfig;
use crate::errors::SessionError;
use bitacora_core::models::{Session, SessionStatus};

/// Session metrics for monitoring and reporting
#[derive(Debug, Clone)]
pub struct SessionMetrics {
    pub total_sessions: u32,
    pub active_sessions: u32,
    pub completed_sessions: u32,
    pub average_duration_minutes: f64,
}

/// Session service trait defining core session management operations
#[async_trait]
pub trait SessionService: Send + Sync {
    // Session lifecycle operations
    async fn create_session(&self, description: &str, project_id: Option<Uuid>) -> Result<Uuid, SessionError>;
    async fn start_session(&self, session_id: &Uuid, context: Option<String>) -> Result<(), SessionError>;
    async fn pause_session(&self, session_id: &Uuid) -> Result<(), SessionError>;
    async fn resume_session(&self, session_id: &Uuid) -> Result<(), SessionError>;
    async fn end_session(&self, session_id: &Uuid, summary: Option<String>) -> Result<(), SessionError>;

    // Session query operations
    async fn get_session(&self, session_id: &Uuid) -> Result<Session, SessionError>;
    async fn list_active_sessions(&self) -> Result<Vec<Session>, SessionError>;
    async fn list_recent_sessions(&self, limit: Option<u32>) -> Result<Vec<Session>, SessionError>;

    // Session management
    async fn validate_session_transition(&self, session_id: &Uuid, target_status: &SessionStatus) -> Result<bool, SessionError>;
    async fn get_session_metrics(&self) -> Result<SessionMetrics, SessionError>;
}

/// Simple implementation of SessionService for basic session management
#[derive(Debug)]
pub struct SessionServiceImpl {
    config: SessionConfig,
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
}

impl SessionServiceImpl {
    pub async fn new(config: SessionConfig) -> Result<Self, SessionError> {
        Ok(Self {
            config,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    async fn can_transition_to(&self, current: &SessionStatus, target: &SessionStatus) -> bool {
        match (current, target) {
            (SessionStatus::Active, SessionStatus::Paused) => true,
            (SessionStatus::Active, SessionStatus::Ended) => true,
            (SessionStatus::Paused, SessionStatus::Active) => true,
            (SessionStatus::Paused, SessionStatus::Ended) => true,
            _ => false,
        }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(&self, description: &str, project_id: Option<Uuid>) -> Result<Uuid, SessionError> {
        let session_id = Uuid::new_v4();
        
        let session = Session {
            session_id,
            user_id: "default_user".to_string(), // TODO: Get from context
            project_id,
            started_at: Utc::now(),
            ended_at: None,
            description: Some(description.to_string()),
            status: SessionStatus::Paused, // Created but not started
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session);
        
        Ok(session_id)
    }

    async fn start_session(&self, session_id: &Uuid, _context: Option<String>) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        
        // Check if we would exceed max active sessions
        let active_count = sessions.values()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .count() as u32;
        
        if active_count >= self.config.max_active_sessions {
            return Err(SessionError::configuration_error("Maximum active sessions limit reached"));
        }

        if let Some(session) = sessions.get_mut(session_id) {
            if !self.can_transition_to(&session.status, &SessionStatus::Active).await {
                return Err(SessionError::invalid_state_transition(
                    format!("{:?}", session.status),
                    format!("{:?}", SessionStatus::Active),
                ));
            }
            
            session.status = SessionStatus::Active;
            Ok(())
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    async fn pause_session(&self, session_id: &Uuid) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if !matches!(session.status, SessionStatus::Active) {
                return Err(SessionError::session_not_active(session_id.to_string()));
            }
            
            session.status = SessionStatus::Paused;
            Ok(())
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    async fn resume_session(&self, session_id: &Uuid) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        
        // Check if we would exceed max active sessions
        let active_count = sessions.values()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .count() as u32;
        
        if active_count >= self.config.max_active_sessions {
            return Err(SessionError::configuration_error("Maximum active sessions limit reached"));
        }

        if let Some(session) = sessions.get_mut(session_id) {
            if !matches!(session.status, SessionStatus::Paused) {
                return Err(SessionError::invalid_state_transition(
                    format!("{:?}", session.status),
                    format!("{:?}", SessionStatus::Active),
                ));
            }
            
            session.status = SessionStatus::Active;
            Ok(())
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    async fn end_session(&self, session_id: &Uuid, _summary: Option<String>) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = SessionStatus::Ended;
            session.ended_at = Some(Utc::now());
            Ok(())
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    async fn get_session(&self, session_id: &Uuid) -> Result<Session, SessionError> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id)
            .cloned()
            .ok_or_else(|| SessionError::session_not_found(session_id.to_string()))
    }

    async fn list_active_sessions(&self) -> Result<Vec<Session>, SessionError> {
        let sessions = self.sessions.read().await;
        let active_sessions = sessions.values()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .cloned()
            .collect();
        Ok(active_sessions)
    }

    async fn list_recent_sessions(&self, limit: Option<u32>) -> Result<Vec<Session>, SessionError> {
        let sessions = self.sessions.read().await;
        let limit = limit.unwrap_or(10) as usize;
        
        let mut recent_sessions: Vec<_> = sessions.values().cloned().collect();
        recent_sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        recent_sessions.truncate(limit);
        
        Ok(recent_sessions)
    }

    async fn validate_session_transition(&self, session_id: &Uuid, target_status: &SessionStatus) -> Result<bool, SessionError> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            Ok(self.can_transition_to(&session.status, target_status).await)
        } else {
            Err(SessionError::session_not_found(session_id.to_string()))
        }
    }

    async fn get_session_metrics(&self) -> Result<SessionMetrics, SessionError> {
        let sessions = self.sessions.read().await;
        
        let total_sessions = sessions.len() as u32;
        let active_sessions = sessions.values()
            .filter(|s| matches!(s.status, SessionStatus::Active))
            .count() as u32;
        let completed_sessions = sessions.values()
            .filter(|s| matches!(s.status, SessionStatus::Ended))
            .count() as u32;

        // Calculate average duration for completed sessions
        let completed_durations: Vec<_> = sessions.values()
            .filter_map(|s| {
                if let Some(ended_at) = s.ended_at {
                    Some((ended_at - s.started_at).num_minutes() as f64)
                } else {
                    None
                }
            })
            .collect();

        let average_duration_minutes = if completed_durations.is_empty() {
            0.0
        } else {
            completed_durations.iter().sum::<f64>() / completed_durations.len() as f64
        };

        Ok(SessionMetrics {
            total_sessions,
            active_sessions,
            completed_sessions,
            average_duration_minutes,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_session() {
        let config = SessionConfig::default();
        let service = SessionServiceImpl::new(config).await.unwrap();
        
        let session_id = service.create_session("Test session", None).await.unwrap();
        
        let session = service.get_session(&session_id).await.unwrap();
        assert_eq!(session.description, Some("Test session".to_string()));
        assert!(matches!(session.status, SessionStatus::Paused));
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let config = SessionConfig::default();
        let service = SessionServiceImpl::new(config).await.unwrap();
        
        // Create session
        let session_id = service.create_session("Test session", None).await.unwrap();
        
        // Start session
        service.start_session(&session_id, None).await.unwrap();
        let session = service.get_session(&session_id).await.unwrap();
        assert!(matches!(session.status, SessionStatus::Active));
        
        // Pause session
        service.pause_session(&session_id).await.unwrap();
        let session = service.get_session(&session_id).await.unwrap();
        assert!(matches!(session.status, SessionStatus::Paused));
        
        // Resume session
        service.resume_session(&session_id).await.unwrap();
        let session = service.get_session(&session_id).await.unwrap();
        assert!(matches!(session.status, SessionStatus::Active));
        
        // End session
        service.end_session(&session_id, Some("Test completed".to_string())).await.unwrap();
        let session = service.get_session(&session_id).await.unwrap();
        assert!(matches!(session.status, SessionStatus::Ended));
        assert!(session.ended_at.is_some());
    }

    #[tokio::test]
    async fn test_max_active_sessions() {
        let mut config = SessionConfig::default();
        config.max_active_sessions = 1;
        let service = SessionServiceImpl::new(config).await.unwrap();
        
        // Create and start first session
        let session_id1 = service.create_session("Session 1", None).await.unwrap();
        service.start_session(&session_id1, None).await.unwrap();
        
        // Create second session
        let session_id2 = service.create_session("Session 2", None).await.unwrap();
        
        // Try to start second session - should fail
        let result = service.start_session(&session_id2, None).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SessionError::ConfigurationError(_)));
    }

    #[tokio::test]
    async fn test_session_metrics() {
        let config = SessionConfig::default();
        let service = SessionServiceImpl::new(config).await.unwrap();
        
        // Create and complete a session
        let session_id = service.create_session("Test session", None).await.unwrap();
        service.start_session(&session_id, None).await.unwrap();
        service.end_session(&session_id, None).await.unwrap();
        
        let metrics = service.get_session_metrics().await.unwrap();
        assert_eq!(metrics.total_sessions, 1);
        assert_eq!(metrics.active_sessions, 0);
        assert_eq!(metrics.completed_sessions, 1);
    }

    #[tokio::test]
    async fn test_invalid_transitions() {
        let config = SessionConfig::default();
        let service = SessionServiceImpl::new(config).await.unwrap();
        
        let session_id = service.create_session("Test session", None).await.unwrap();
        
        // Try to pause a non-active session
        let result = service.pause_session(&session_id).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SessionError::SessionNotActive(_)));
    }
}
