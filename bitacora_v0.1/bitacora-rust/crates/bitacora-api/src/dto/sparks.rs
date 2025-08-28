//! DTOs for Spark operations in Bitacora API (Transversal Insights Service)

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Spark data transfer object
/// Sparks capture insights, learnings, and cross-cutting observations at any level
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SparkDto {
    /// Unique spark identifier
    pub id: Uuid,
    
    /// Context where the spark was captured (project, topic, or action)
    pub context: SparkContext,
    
    /// Spark type/category
    pub spark_type: SparkType,
    
    /// Spark title/summary
    pub title: String,
    
    /// Detailed spark content
    pub content: String,
    
    /// Spark insights and learnings
    pub insights: Option<String>,
    
    /// Impact level of this spark
    pub impact: SparkImpact,
    
    /// Spark tags for categorization
    pub tags: Vec<String>,
    
    /// Related entities (projects, topics, actions)
    pub related_entities: Vec<RelatedEntity>,
    
    /// Spark metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Author of the spark
    pub author: Option<String>,
    
    /// Whether this spark has been reviewed/validated
    pub reviewed: bool,
    
    /// Review notes (if reviewed)
    pub review_notes: Option<String>,
    
    /// Actionable items derived from this spark
    pub action_items: Vec<ActionItem>,
}

/// Context where a spark was captured
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SparkContext {
    /// Level where spark was captured
    pub level: ContextLevel,
    
    /// ID of the specific entity (project_id, topic_id, or action_id)
    pub entity_id: Uuid,
    
    /// Human-readable context description
    pub description: String,
}

/// Level in the hierarchy where spark was captured
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ContextLevel {
    /// Spark captured at project level
    Project,
    /// Spark captured at topic level
    Topic,
    /// Spark captured at action level
    Action,
    /// Spark captured across multiple contexts
    Global,
}

/// Type/category of spark
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SparkType {
    /// Learning or insight gained
    Learning,
    /// Idea for improvement
    Idea,
    /// Discovery of best practice
    BestPractice,
    /// Issue or problem identified
    Issue,
    /// Solution or workaround found
    Solution,
    /// Pattern or approach discovered
    Pattern,
    /// Resource or tool discovered
    Resource,
    /// Risk or concern identified
    Risk,
    /// Opportunity identified
    Opportunity,
    /// General observation
    Observation,
}

/// Impact level of the spark
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SparkImpact {
    /// Low impact - minor insight
    Low,
    /// Medium impact - useful insight
    Medium,
    /// High impact - significant insight
    High,
    /// Critical impact - game-changing insight
    Critical,
}

/// Related entity reference
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RelatedEntity {
    /// Type of related entity
    pub entity_type: EntityType,
    /// ID of the related entity
    pub entity_id: Uuid,
    /// Name/title of the related entity
    pub name: String,
    /// Relationship description
    pub relationship: String,
}

/// Type of entity
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EntityType {
    Project,
    Topic,
    Action,
}

/// Actionable item derived from spark
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionItem {
    /// Brief description of the action
    pub description: String,
    /// Priority of this action item
    pub priority: ActionItemPriority,
    /// Whether this has been converted to an actual Action
    pub converted: bool,
    /// ID of the Action if converted
    pub action_id: Option<Uuid>,
}

/// Priority of action items derived from sparks
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionItemPriority {
    Low,
    Medium,
    High,
}

/// Request to create a new spark
#[derive(Debug, Deserialize)]
pub struct CreateSparkRequest {
    /// Context where the spark is captured (required)
    pub context: SparkContext,
    
    /// Spark type (required)
    pub spark_type: SparkType,
    
    /// Spark title/summary (required)
    pub title: String,
    
    /// Detailed spark content (required)
    pub content: String,
    
    /// Spark insights and learnings (optional)
    pub insights: Option<String>,
    
    /// Impact level (defaults to Medium)
    pub impact: Option<SparkImpact>,
    
    /// Spark tags for categorization
    pub tags: Option<Vec<String>>,
    
    /// Related entities
    pub related_entities: Option<Vec<RelatedEntity>>,
    
    /// Spark metadata
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Author of the spark
    pub author: Option<String>,
    
    /// Actionable items derived from this spark
    pub action_items: Option<Vec<ActionItem>>,
}

/// Request to update an existing spark
#[derive(Debug, Deserialize)]
pub struct UpdateSparkRequest {
    /// Updated spark title (optional)
    pub title: Option<String>,
    
    /// Updated spark content (optional)
    pub content: Option<String>,
    
    /// Updated insights (optional)
    pub insights: Option<String>,
    
    /// Updated impact level (optional)
    pub impact: Option<SparkImpact>,
    
    /// Updated spark tags (optional, replaces existing)
    pub tags: Option<Vec<String>>,
    
    /// Updated related entities (optional, replaces existing)
    pub related_entities: Option<Vec<RelatedEntity>>,
    
    /// Updated metadata (optional, merges with existing)
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    
    /// Mark as reviewed/unreviewed
    pub reviewed: Option<bool>,
    
    /// Review notes
    pub review_notes: Option<String>,
    
    /// Updated action items
    pub action_items: Option<Vec<ActionItem>>,
}

/// Spark summary for list views
#[derive(Debug, Serialize)]
pub struct SparkSummaryDto {
    pub id: Uuid,
    pub title: String,
    pub spark_type: SparkType,
    pub impact: SparkImpact,
    pub context: SparkContext,
    pub tags: Vec<String>,
    pub reviewed: bool,
    pub created_at: DateTime<Utc>,
    pub author: Option<String>,
}

/// Spark analytics and insights
#[derive(Debug, Serialize)]
pub struct SparkAnalyticsDto {
    pub total_sparks: u32,
    pub sparks_by_type: std::collections::HashMap<String, u32>,
    pub sparks_by_impact: std::collections::HashMap<String, u32>,
    pub sparks_by_context: std::collections::HashMap<String, u32>,
    pub recent_high_impact: Vec<SparkSummaryDto>,
    pub top_tags: Vec<TagCount>,
}

#[derive(Debug, Serialize)]
pub struct TagCount {
    pub tag: String,
    pub count: u32,
}

impl Default for SparkImpact {
    fn default() -> Self {
        SparkImpact::Medium
    }
}

impl Default for ActionItemPriority {
    fn default() -> Self {
        ActionItemPriority::Medium
    }
}

impl CreateSparkRequest {
    /// Validate the create spark request
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.title.trim().is_empty() {
            errors.push("Spark title cannot be empty".to_string());
        }
        
        if self.title.len() > 200 {
            errors.push("Spark title cannot exceed 200 characters".to_string());
        }
        
        if self.content.trim().is_empty() {
            errors.push("Spark content cannot be empty".to_string());
        }
        
        if self.content.len() > 5000 {
            errors.push("Spark content cannot exceed 5000 characters".to_string());
        }
        
        if let Some(insights) = &self.insights {
            if insights.len() > 3000 {
                errors.push("Spark insights cannot exceed 3000 characters".to_string());
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
        
        if let Some(related_entities) = &self.related_entities {
            if related_entities.len() > 10 {
                errors.push("Cannot have more than 10 related entities".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl UpdateSparkRequest {
    /// Check if the update request has any changes
    pub fn has_changes(&self) -> bool {
        self.title.is_some() 
            || self.content.is_some() 
            || self.insights.is_some() 
            || self.impact.is_some()
            || self.tags.is_some() 
            || self.related_entities.is_some()
            || self.metadata.is_some()
            || self.reviewed.is_some()
            || self.review_notes.is_some()
            || self.action_items.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_spark_request_validation() {
        let context = SparkContext {
            level: ContextLevel::Project,
            entity_id: Uuid::new_v4(),
            description: "Test context".to_string(),
        };
        
        let valid_request = CreateSparkRequest {
            context,
            spark_type: SparkType::Learning,
            title: "Valid Spark".to_string(),
            content: "Valid content with insights".to_string(),
            insights: Some("Key insights".to_string()),
            impact: Some(SparkImpact::High),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            related_entities: None,
            metadata: None,
            author: Some("test@example.com".to_string()),
            action_items: None,
        };
        
        assert!(valid_request.validate().is_ok());
    }

    #[test]
    fn test_spark_defaults() {
        let impact = SparkImpact::default();
        let priority = ActionItemPriority::default();
        
        assert!(matches!(impact, SparkImpact::Medium));
        assert!(matches!(priority, ActionItemPriority::Medium));
    }
}
