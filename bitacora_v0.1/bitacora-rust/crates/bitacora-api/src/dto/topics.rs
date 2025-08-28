//! DTOs for Topic operations in Bitacora API

use serde::{Deserialize, Serialize};
// utoipa schema derivations removed
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Topic data transfer object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicDto {
    /// Unique topic identifier
    pub id: Uuid,
    
    /// Parent project identifier
    pub project_id: Uuid,
    
    /// Topic title
    pub title: String,
    
    /// Topic description
    pub description: Option<String>,
    
    /// Topic status
    pub status: TopicStatus,
    
    /// Topic priority level
    pub priority: TopicPriority,
    
    /// Topic tags for categorization
    pub tags: Vec<String>,
    
    /// Topic metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Estimated completion time in hours
    pub estimated_hours: Option<f32>,
    
    /// Actual time spent in hours
    pub actual_hours: Option<f32>,
    
    /// Number of actions in this topic
    pub action_count: u32,
    
    /// Number of completed actions in this topic
    pub completed_actions: u32,
    
    /// Topic completion percentage (0-100)
    pub completion_percentage: f32,
}

/// Alias for API responses - same as TopicDto
pub type TopicResponse = TopicDto;

/// Topic status enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TopicStatus {
    /// Topic is planned but not started
    Planned,
    /// Topic is in progress
    InProgress,
    /// Topic is blocked by external factors
    Blocked,
    /// Topic is under review
    Review,
    /// Topic has been completed
    Completed,
    /// Topic has been cancelled
    Cancelled,
}

/// Topic priority enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TopicPriority {
    /// Low priority topic
    Low,
    /// Medium priority topic
    Medium,
    /// High priority topic
    High,
    /// Critical priority topic
    Critical,
}

/// Request to create a new topic
#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    /// Parent project identifier (required)
    pub project_id: Uuid,
    
    /// Topic title (required)
    pub title: String,
    
    /// Topic description (optional)
    pub description: Option<String>,
    
    /// Initial status (defaults to Planned)
    pub status: Option<TopicStatus>,
    
    /// Topic priority (defaults to Medium)
    pub priority: Option<TopicPriority>,
    
    /// Topic tags for categorization
    pub tags: Option<Vec<String>>,
    
    /// Topic metadata as key-value pairs
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Estimated completion time in hours
    pub estimated_hours: Option<f32>,
}

/// Request to update an existing topic
#[derive(Debug, Deserialize)]
pub struct UpdateTopicRequest {
    /// Updated topic title (optional)
    pub title: Option<String>,
    
    /// Updated topic description (optional)
    pub description: Option<String>,
    
    /// Updated topic status (optional)
    pub status: Option<TopicStatus>,
    
    /// Updated topic priority (optional)
    pub priority: Option<TopicPriority>,
    
    /// Updated topic tags (optional, replaces existing tags)
    pub tags: Option<Vec<String>>,
    
    /// Updated topic metadata (optional, merges with existing)
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Updated estimated completion time in hours
    pub estimated_hours: Option<f32>,
    
    /// Updated actual time spent in hours
    pub actual_hours: Option<f32>,
}

/// Topic summary for list views
#[derive(Debug, Serialize)]
pub struct TopicSummaryDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub status: TopicStatus,
    pub priority: TopicPriority,
    pub action_count: u32,
    pub completion_percentage: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Topic with project information
#[derive(Debug, Serialize)]
pub struct TopicWithProjectDto {
    #[serde(flatten)]
    pub topic: TopicDto,
    pub project_name: String,
}

impl Default for TopicStatus {
    fn default() -> Self {
        TopicStatus::Planned
    }
}

impl Default for TopicPriority {
    fn default() -> Self {
        TopicPriority::Medium
    }
}

impl CreateTopicRequest {
    /// Validate the create topic request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.title.trim().is_empty() {
            errors.push("Topic title cannot be empty".to_string());
        }
        
        if self.title.len() > 200 {
            errors.push("Topic title cannot exceed 200 characters".to_string());
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Topic description cannot exceed 2000 characters".to_string());
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
            if estimated_hours < 0.0 || estimated_hours > 1000.0 {
                errors.push("Estimated hours must be between 0 and 1000".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl UpdateTopicRequest {
    /// Check if the update request has any changes
    pub fn has_changes(&self) -> bool {
        self.title.is_some() 
            || self.description.is_some() 
            || self.status.is_some() 
            || self.priority.is_some()
            || self.tags.is_some() 
            || self.metadata.is_some()
            || self.estimated_hours.is_some()
            || self.actual_hours.is_some()
    }

    /// Validate the update topic request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if let Some(title) = &self.title {
            if title.trim().is_empty() {
                errors.push("Topic title cannot be empty".to_string());
            }
            if title.len() > 200 {
                errors.push("Topic title cannot exceed 200 characters".to_string());
            }
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Topic description cannot exceed 2000 characters".to_string());
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
            if estimated_hours < 0.0 || estimated_hours > 1000.0 {
                errors.push("Estimated hours must be between 0 and 1000".to_string());
            }
        }
        
        if let Some(actual_hours) = self.actual_hours {
            if actual_hours < 0.0 || actual_hours > 1000.0 {
                errors.push("Actual hours must be between 0 and 1000".to_string());
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
    fn test_create_topic_request_validation() {
        let valid_request = CreateTopicRequest {
            project_id: Uuid::new_v4(),
            title: "Valid Topic".to_string(),
            description: Some("A valid description".to_string()),
            status: Some(TopicStatus::InProgress),
            priority: Some(TopicPriority::High),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            metadata: None,
            estimated_hours: Some(8.5),
        };
        
        assert!(valid_request.validate().is_ok());
        
        let invalid_request = CreateTopicRequest {
            project_id: Uuid::new_v4(),
            title: "".to_string(), // Empty title
            description: None,
            status: None,
            priority: None,
            tags: None,
            metadata: None,
            estimated_hours: Some(-1.0), // Invalid hours
        };
        
        let result = invalid_request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // Empty title and invalid hours
    }

    #[test]
    fn test_topic_defaults() {
        let status = TopicStatus::default();
        let priority = TopicPriority::default();
        
        assert!(matches!(status, TopicStatus::Planned));
        assert!(matches!(priority, TopicPriority::Medium));
    }
}
