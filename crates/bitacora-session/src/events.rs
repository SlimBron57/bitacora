//! Session events module

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Events that occur during session lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionEvent {
    /// Session was created
    SessionCreated {
        session_id: Uuid,
        user_id: String,
        timestamp: DateTime<Utc>,
    },
    /// Session was started
    SessionStarted {
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Session was paused
    SessionPaused {
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Session was resumed
    SessionResumed {
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    /// Session was ended
    SessionEnded {
        session_id: Uuid,
        timestamp: DateTime<Utc>,
        duration: std::time::Duration,
    },
    /// Action was recorded in session
    ActionRecorded {
        session_id: Uuid,
        action_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}

impl SessionEvent {
    pub fn session_id(&self) -> Uuid {
        match self {
            SessionEvent::SessionCreated { session_id, .. } => *session_id,
            SessionEvent::SessionStarted { session_id, .. } => *session_id,
            SessionEvent::SessionPaused { session_id, .. } => *session_id,
            SessionEvent::SessionResumed { session_id, .. } => *session_id,
            SessionEvent::SessionEnded { session_id, .. } => *session_id,
            SessionEvent::ActionRecorded { session_id, .. } => *session_id,
        }
    }
    
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            SessionEvent::SessionCreated { timestamp, .. } => *timestamp,
            SessionEvent::SessionStarted { timestamp, .. } => *timestamp,
            SessionEvent::SessionPaused { timestamp, .. } => *timestamp,
            SessionEvent::SessionResumed { timestamp, .. } => *timestamp,
            SessionEvent::SessionEnded { timestamp, .. } => *timestamp,
            SessionEvent::ActionRecorded { timestamp, .. } => *timestamp,
        }
    }
}
