//! Session domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Type alias for session identifiers
pub type SessionId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: Uuid,
    pub user_id: String,
    pub project_id: Option<Uuid>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub status: SessionStatus,
}

impl Session {
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }
}

pub struct SessionBuilder {
    session_id: Option<Uuid>,
    user_id: Option<String>,
    project_id: Option<Uuid>,
    started_at: Option<DateTime<Utc>>,
    ended_at: Option<DateTime<Utc>>,
    description: Option<String>,
    status: Option<SessionStatus>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        Self {
            session_id: None,
            user_id: None,
            project_id: None,
            started_at: None,
            ended_at: None,
            description: None,
            status: None,
        }
    }
    
    pub fn session_id(mut self, session_id: Uuid) -> Self {
        self.session_id = Some(session_id);
        self
    }
    
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn project_id(mut self, project_id: Option<Uuid>) -> Self {
        self.project_id = project_id;
        self
    }
    
    pub fn started_at(mut self, started_at: DateTime<Utc>) -> Self {
        self.started_at = Some(started_at);
        self
    }
    
    pub fn ended_at(mut self, ended_at: Option<DateTime<Utc>>) -> Self {
        self.ended_at = ended_at;
        self
    }
    
    pub fn description(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }
    
    pub fn status(mut self, status: SessionStatus) -> Self {
        self.status = Some(status);
        self
    }
    
    pub fn build(self) -> Session {
        Session {
            session_id: self.session_id.unwrap_or_else(Uuid::new_v4),
            user_id: self.user_id.unwrap_or_else(|| "unknown".to_string()),
            project_id: self.project_id,
            started_at: self.started_at.unwrap_or_else(Utc::now),
            ended_at: self.ended_at,
            description: self.description,
            status: self.status.unwrap_or(SessionStatus::Created),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Ended,
    Paused,
    Created,
    Completed,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Active => write!(f, "Active"),
            SessionStatus::Ended => write!(f, "Ended"),
            SessionStatus::Paused => write!(f, "Paused"),
            SessionStatus::Created => write!(f, "Created"),
            SessionStatus::Completed => write!(f, "Completed"),
        }
    }
}
