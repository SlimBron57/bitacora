//! DTOs for Project operations in Bitacora API

use serde::{Deserialize, Serialize};
// utoipa schema derivations removed
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Project data transfer object
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectDto {
    /// Unique project identifier
    // #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    
    /// Project name
    // #[schema(example = "My Awesome Project")]
    pub name: String,
    
    /// Project description
    // #[schema(example = "A comprehensive project to build amazing software")]
    pub description: Option<String>,
    
    /// Project status
    pub status: ProjectStatus,
    
    /// Project tags for categorization
    // #[schema(example = json!(["development", "rust", "api"]))]
    pub tags: Vec<String>,
    
    /// Project metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Number of topics in this project
    // #[schema(example = 5)]
    pub topic_count: u32,
    
    /// Number of completed actions in this project
    // #[schema(example = 12)]
    pub completed_actions: u32,
    
    /// Total number of actions in this project  
    // #[schema(example = 20)]
    pub total_actions: u32,
    
    /// Project completion percentage (0-100)
    // #[schema(example = 60.0)]
    pub completion_percentage: f32,
}

/// Project status enumeration
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    /// Project is being planned
    Planning,
    /// Project is actively being worked on
    Active,
    /// Project is temporarily paused
    Paused,
    /// Project has been completed
    Completed,
    /// Project has been archived
    Archived,
    /// Project has been cancelled
    Cancelled,
}

/// Request to create a new project
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    /// Project name (required)
    // #[schema(example = "My New Project")]
    pub name: String,
    
    /// Project description (optional)
    // #[schema(example = "Description of what this project aims to achieve")]
    pub description: Option<String>,
    
    /// Initial status (defaults to Planning)
    pub status: Option<ProjectStatus>,
    
    /// Project tags for categorization
    // #[schema(example = json!(["web", "frontend", "react"]))]
    pub tags: Option<Vec<String>>,
    
    /// Project metadata as key-value pairs
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Request to update an existing project
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    /// Updated project name (optional)
    // #[schema(example = "Updated Project Name")]
    pub name: Option<String>,
    
    /// Updated project description (optional)
    // #[schema(example = "Updated description with new goals")]
    pub description: Option<String>,
    
    /// Updated project status (optional)
    pub status: Option<ProjectStatus>,
    
    /// Updated project tags (optional, replaces existing tags)
    // #[schema(example = json!(["web", "backend", "api", "rust"]))]
    pub tags: Option<Vec<String>>,
    
    /// Updated project metadata (optional, merges with existing)
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Project summary for list views
#[derive(Debug, Serialize)]
pub struct ProjectSummaryDto {
    pub id: Uuid,
    pub name: String,
    pub status: ProjectStatus,
    pub topic_count: u32,
    pub completion_percentage: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Project statistics
#[derive(Debug, Serialize)]
pub struct ProjectStatsDto {
    pub total_projects: u32,
    pub active_projects: u32,
    pub completed_projects: u32,
    pub total_topics: u32,
    pub total_actions: u32,
    pub completion_rate: f32,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Planning
    }
}

impl CreateProjectRequest {
    /// Validate the create project request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.name.trim().is_empty() {
            errors.push("Project name cannot be empty".to_string());
        }
        
        if self.name.len() > 200 {
            errors.push("Project name cannot exceed 200 characters".to_string());
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Project description cannot exceed 2000 characters".to_string());
            }
        }
        
        if let Some(tags) = &self.tags {
            if tags.len() > 20 {
                errors.push("Cannot have more than 20 tags".to_string());
            }
            for tag in tags {
                if tag.len() > 50 {
                    errors.push("Tags cannot exceed 50 characters".to_string());
                    break;
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl UpdateProjectRequest {
    /// Check if the update request has any changes
    pub fn has_changes(&self) -> bool {
        self.name.is_some() 
            || self.description.is_some() 
            || self.status.is_some() 
            || self.tags.is_some() 
            || self.metadata.is_some()
    }

    /// Validate the update project request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if let Some(name) = &self.name {
            if name.trim().is_empty() {
                errors.push("Project name cannot be empty".to_string());
            }
            if name.len() > 200 {
                errors.push("Project name cannot exceed 200 characters".to_string());
            }
        }
        
        if let Some(description) = &self.description {
            if description.len() > 2000 {
                errors.push("Project description cannot exceed 2000 characters".to_string());
            }
        }
        
        if let Some(tags) = &self.tags {
            if tags.len() > 20 {
                errors.push("Cannot have more than 20 tags".to_string());
            }
            for tag in tags {
                if tag.len() > 50 {
                    errors.push("Tags cannot exceed 50 characters".to_string());
                    break;
                }
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
    fn test_create_project_request_validation() {
        let valid_request = CreateProjectRequest {
            name: "Valid Project".to_string(),
            description: Some("A valid description".to_string()),
            status: Some(ProjectStatus::Active),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            metadata: None,
        };
        
        assert!(valid_request.validate().is_ok());
        
        let invalid_request = CreateProjectRequest {
            name: "".to_string(), // Empty name
            description: None,
            status: None,
            tags: None,
            metadata: None,
        };
        
        assert!(invalid_request.validate().is_err());
    }

    #[test]
    fn test_update_request_has_changes() {
        let empty_request = UpdateProjectRequest {
            name: None,
            description: None,
            status: None,
            tags: None,
            metadata: None,
        };
        
        assert!(!empty_request.has_changes());
        
        let request_with_changes = UpdateProjectRequest {
            name: Some("New name".to_string()),
            description: None,
            status: None,
            tags: None,
            metadata: None,
        };
        
        assert!(request_with_changes.has_changes());
    }

    #[test]
    fn test_project_status_default() {
        let status = ProjectStatus::default();
        assert!(matches!(status, ProjectStatus::Planning));
    }
}
