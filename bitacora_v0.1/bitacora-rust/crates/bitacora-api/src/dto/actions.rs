//! DTOs for Action operations in Bitacora API

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Action data transfer object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionDto {
    /// Unique action identifier
    pub id: Uuid,
    
    /// Parent topic identifier
    pub topic_id: Uuid,
    
    /// Parent project identifier
    pub project_id: Uuid,
    
    /// Action title
    pub title: String,
    
    /// Action description
    pub description: Option<String>,
    
    /// Action status
    pub status: ActionStatus,
    
    /// Action type/category
    pub action_type: ActionType,
    
    /// Action priority level
    pub priority: ActionPriority,
    
    /// Action tags for categorization
    pub tags: Vec<String>,
    
    /// Action metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// When action was started
    pub started_at: Option<DateTime<Utc>>,
    
    /// When action was completed
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Due date for the action
    pub due_date: Option<DateTime<Utc>>,
    
    /// Estimated completion time in hours
    pub estimated_hours: Option<f32>,
    
    /// Actual time spent in hours
    pub actual_hours: Option<f32>,
    
    /// Action notes and comments
    pub notes: Option<String>,
    
    /// Dependencies - other actions that must be completed first
    pub dependencies: Vec<Uuid>,
    
    /// Assigned user (if any)
    pub assigned_to: Option<String>,
}

/// Alias for API responses - same as ActionDto
pub type ActionResponse = ActionDto;

/// Action status enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionStatus {
    /// Action is planned but not started
    Todo,
    /// Action is currently in progress
    InProgress,
    /// Action is blocked by dependencies or external factors
    Blocked,
    /// Action is waiting for review or approval
    Review,
    /// Action has been completed successfully
    Done,
    /// Action has been cancelled or abandoned
    Cancelled,
}

/// Action type/category enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
    /// Development/coding task
    Development,
    /// Testing task
    Testing,
    /// Documentation task
    Documentation,
    /// Design task
    Design,
    /// Research task
    Research,
    /// Bug fix task
    Bugfix,
    /// Code review task
    Review,
    /// Deployment task
    Deployment,
    /// Meeting or discussion
    Meeting,
    /// Administrative task
    Admin,
    /// Other/miscellaneous task
    Other,
}

/// Action priority enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionPriority {
    /// Low priority action
    Low,
    /// Medium priority action
    Medium,
    /// High priority action
    High,
    /// Critical priority action
    Critical,
}

/// Request to create a new action
#[derive(Debug, Deserialize)]
pub struct CreateActionRequest {
    /// Parent topic identifier (required)
    pub topic_id: Uuid,
    
    /// Action title (required)
    pub title: String,
    
    /// Action description (optional)
    pub description: Option<String>,
    
    /// Initial status (defaults to Todo)
    pub status: Option<ActionStatus>,
    
    /// Action type (defaults to Development)
    pub action_type: Option<ActionType>,
    
    /// Action priority (defaults to Medium)
    pub priority: Option<ActionPriority>,
    
    /// Action tags for categorization
    pub tags: Option<Vec<String>>,
    
    /// Action metadata as key-value pairs
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Due date for the action
    pub due_date: Option<DateTime<Utc>>,
    
    /// Estimated completion time in hours
    pub estimated_hours: Option<f32>,
    
    /// Initial notes
    pub notes: Option<String>,
    
    /// Action dependencies - other actions that must be completed first
    pub dependencies: Option<Vec<Uuid>>,
    
    /// Assigned user (optional)
    pub assigned_to: Option<String>,
}

/// Request to update an existing action
#[derive(Debug, Deserialize)]
pub struct UpdateActionRequest {
    /// Updated action title (optional)
    pub title: Option<String>,
    
    /// Updated action description (optional)
    pub description: Option<String>,
    
    /// Updated action status (optional)
    pub status: Option<ActionStatus>,
    
    /// Updated action type (optional)
    pub action_type: Option<ActionType>,
    
    /// Updated action priority (optional)
    pub priority: Option<ActionPriority>,
    
    /// Updated action tags (optional, replaces existing tags)
    pub tags: Option<Vec<String>>,
    
    /// Updated action metadata (optional, merges with existing)
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Updated due date
    pub due_date: Option<DateTime<Utc>>,
    
    /// Updated estimated completion time in hours
    pub estimated_hours: Option<f32>,
    
    /// Updated actual time spent in hours
    pub actual_hours: Option<f32>,
    
    /// Updated notes
    pub notes: Option<String>,
    
    /// Updated dependencies
    pub dependencies: Option<Vec<Uuid>>,
    
    /// Updated assigned user
    pub assigned_to: Option<String>,
}

/// Action summary for list views
#[derive(Debug, Serialize)]
pub struct ActionSummaryDto {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub status: ActionStatus,
    pub action_type: ActionType,
    pub priority: ActionPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub assigned_to: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Action with context information
#[derive(Debug, Serialize)]
pub struct ActionWithContextDto {
    #[serde(flatten)]
    pub action: ActionDto,
    pub topic_title: String,
    pub project_name: String,
}

impl Default for ActionStatus {
    fn default() -> Self {
        ActionStatus::Todo
    }
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::Development
    }
}

impl Default for ActionPriority {
    fn default() -> Self {
        ActionPriority::Medium
    }
}

impl CreateActionRequest {
    /// Validate the create action request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.title.trim().is_empty() {
            errors.push("Action title cannot be empty".to_string());
        }
        
        if self.title.len() > 200 {
            errors.push("Action title cannot exceed 200 characters".to_string());
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Action description cannot exceed 2000 characters".to_string());
            }
        }
        
        if let Some(tags) = &self.tags {
            if tags.len() > 15 {
                errors.push("Cannot have more than 15 tags".to_string());
            }
            for tag in tags {
                if tag.len() > 50 {
                    errors.push("Tags cannot exceed 50 characters".to_string());
                    break;
                }
            }
        }
        
        if let Some(estimated_hours) = self.estimated_hours {
            if estimated_hours < 0.0 || estimated_hours > 100.0 {
                errors.push("Estimated hours must be between 0 and 100".to_string());
            }
        }
        
        if let Some(notes) = &self.notes {
            if notes.len() > 5000 {
                errors.push("Notes cannot exceed 5000 characters".to_string());
            }
        }
        
        if let Some(dependencies) = &self.dependencies {
            if dependencies.len() > 20 {
                errors.push("Cannot have more than 20 dependencies".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl UpdateActionRequest {
    /// Check if the update request has any changes
    pub fn has_changes(&self) -> bool {
        self.title.is_some() 
            || self.description.is_some() 
            || self.status.is_some() 
            || self.action_type.is_some()
            || self.priority.is_some()
            || self.tags.is_some() 
            || self.metadata.is_some()
            || self.due_date.is_some()
            || self.estimated_hours.is_some()
            || self.actual_hours.is_some()
            || self.notes.is_some()
            || self.dependencies.is_some()
            || self.assigned_to.is_some()
    }

    /// Validate the update action request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                errors.push("Action title cannot be empty".to_string());
            }
            if title.len() > 200 {
                errors.push("Action title cannot exceed 200 characters".to_string());
            }
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Action description cannot exceed 2000 characters".to_string());
            }
        }
        
        if let Some(tags) = &self.tags {
            if tags.len() > 15 {
                errors.push("Cannot have more than 15 tags".to_string());
            }
            for tag in tags {
                if tag.len() > 50 {
                    errors.push("Tags cannot exceed 50 characters".to_string());
                    break;
                }
            }
        }
        
        if let Some(estimated_hours) = self.estimated_hours {
            if estimated_hours < 0.0 || estimated_hours > 100.0 {
                errors.push("Estimated hours must be between 0 and 100".to_string());
            }
        }
        
        if let Some(actual_hours) = self.actual_hours {
            if actual_hours < 0.0 || actual_hours > 100.0 {
                errors.push("Actual hours must be between 0 and 100".to_string());
            }
        }
        
        if let Some(notes) = &self.notes {
            if notes.len() > 5000 {
                errors.push("Notes cannot exceed 5000 characters".to_string());
            }
        }
        
        if let Some(dependencies) = &self.dependencies {
            if dependencies.len() > 20 {
                errors.push("Cannot have more than 20 dependencies".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_action_request_validation() {
        let valid_request = CreateActionRequest {
            topic_id: Uuid::new_v4(),
            title: "Valid Action".to_string(),
            description: Some("A valid description".to_string()),
            status: Some(ActionStatus::InProgress),
            action_type: Some(ActionType::Development),
            priority: Some(ActionPriority::High),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            metadata: None,
            due_date: None,
            estimated_hours: Some(4.5),
            notes: Some("Initial notes".to_string()),
            dependencies: Some(vec![]),
            assigned_to: Some("developer@example.com".to_string()),
        };
        
        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_action_defaults() {
        let status = ActionStatus::default();
        let action_type = ActionType::default();
        let priority = ActionPriority::default();
        
        assert!(matches!(status, ActionStatus::Todo));
        assert!(matches!(action_type, ActionType::Development));
        assert!(matches!(priority, ActionPriority::Medium));
    }
}
