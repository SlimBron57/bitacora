//! Template service traits and implementations

use async_trait::async_trait;
use std::collections::HashMap;
use bitacora_core::models::action::{Action, ActionType};
use crate::{Template, TemplateError, TemplateResult, RenderedTemplate};

/// Core template service trait
#[async_trait]
pub trait TemplateService {
    /// Get template by ID
    async fn get_template(&self, template_id: &str) -> TemplateResult<Template>;
    
    /// Get all templates in a category
    async fn get_templates_by_category(&self, category: &str) -> TemplateResult<Vec<Template>>;
    
    /// Detect best template for an action
    async fn detect_template_for_action(&self, action: &Action) -> TemplateResult<Option<String>>;
    
    /// Render template with variables
    async fn render_template(
        &self, 
        template_id: &str, 
        variables: &HashMap<String, serde_json::Value>
    ) -> TemplateResult<RenderedTemplate>;
    
    /// Render action with appropriate template
    async fn render_action_response(&self, action: &Action) -> TemplateResult<String>;
    
    /// Create new template
    async fn create_template(&self, template: &Template) -> TemplateResult<String>;
    
    /// Update existing template  
    async fn update_template(&self, template_id: &str, template: &Template) -> TemplateResult<()>;
    
    /// Delete template
    async fn delete_template(&self, template_id: &str) -> TemplateResult<()>;
    
    /// Record template usage analytics
    async fn record_usage(&self, template_id: &str, user_satisfaction: Option<u8>) -> TemplateResult<()>;
}

/// Template detection service
#[async_trait]
pub trait TemplateDetector {
    /// Detect templates based on action context
    async fn detect_templates(&self, action: &Action) -> TemplateResult<Vec<String>>;
    
    /// Get detection confidence score for template
    async fn get_confidence_score(&self, template_id: &str, action: &Action) -> TemplateResult<f32>;
    
    /// Learn from user feedback to improve detection
    async fn learn_from_feedback(&self, template_id: &str, action: &Action, satisfaction: u8) -> TemplateResult<()>;
}

/// Template repository trait for persistence
#[async_trait]
pub trait TemplateRepository {
    /// Save template
    async fn save(&self, template: &Template) -> TemplateResult<()>;
    
    /// Find template by ID
    async fn find_by_id(&self, template_id: &str) -> TemplateResult<Option<Template>>;
    
    /// Find templates by category
    async fn find_by_category(&self, category: &str) -> TemplateResult<Vec<Template>>;
    
    /// Find templates by tags
    async fn find_by_tags(&self, tags: &[String]) -> TemplateResult<Vec<Template>>;
    
    /// Search templates by trigger conditions
    async fn find_by_triggers(&self, context: &HashMap<String, serde_json::Value>) -> TemplateResult<Vec<Template>>;
    
    /// Update template
    async fn update(&self, template: &Template) -> TemplateResult<()>;
    
    /// Delete template
    async fn delete(&self, template_id: &str) -> TemplateResult<()>;
    
    /// Get usage statistics
    async fn get_usage_stats(&self, template_id: &str) -> TemplateResult<TemplateUsageStats>;
}

/// Template usage statistics
#[derive(Debug, Clone)]
pub struct TemplateUsageStats {
    pub template_id: String,
    pub total_usage: u64,
    pub average_satisfaction: Option<f32>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub success_rate: f32,
}

/// Template validation service
pub trait TemplateValidator {
    /// Validate template structure
    fn validate_template(&self, template: &Template) -> TemplateResult<()>;
    
    /// Validate template variables
    fn validate_variables(&self, template: &Template, variables: &HashMap<String, serde_json::Value>) -> TemplateResult<()>;
    
    /// Check for circular dependencies
    fn check_dependencies(&self, template: &Template) -> TemplateResult<()>;
}

/// Default template service implementation
pub struct DefaultTemplateService<R, D> 
where
    R: TemplateRepository + Send + Sync,
    D: TemplateDetector + Send + Sync,
{
    repository: R,
    detector: D,
    validator: Box<dyn TemplateValidator + Send + Sync>,
}

impl<R, D> DefaultTemplateService<R, D>
where
    R: TemplateRepository + Send + Sync,
    D: TemplateDetector + Send + Sync,
{
    pub fn new(repository: R, detector: D, validator: Box<dyn TemplateValidator + Send + Sync>) -> Self {
        Self {
            repository,
            detector,
            validator,
        }
    }
}

#[async_trait]
impl<R, D> TemplateService for DefaultTemplateService<R, D>
where
    R: TemplateRepository + Send + Sync,
    D: TemplateDetector + Send + Sync,
{
    async fn get_template(&self, template_id: &str) -> TemplateResult<Template> {
        self.repository.find_by_id(template_id)
            .await?
            .ok_or_else(|| TemplateError::NotFound { 
                template_id: template_id.to_string() 
            })
    }

    async fn get_templates_by_category(&self, category: &str) -> TemplateResult<Vec<Template>> {
        self.repository.find_by_category(category).await
    }

    async fn detect_template_for_action(&self, action: &Action) -> TemplateResult<Option<String>> {
        let templates = self.detector.detect_templates(action).await?;
        
        if templates.is_empty() {
            return Ok(None);
        }

        // Get the template with highest confidence score
        let mut best_template = None;
        let mut best_score = 0.0;

        for template_id in templates {
            let score = self.detector.get_confidence_score(&template_id, action).await?;
            if score > best_score {
                best_score = score;
                best_template = Some(template_id);
            }
        }

        Ok(best_template)
    }

    async fn render_template(
        &self, 
        template_id: &str, 
        variables: &HashMap<String, serde_json::Value>
    ) -> TemplateResult<RenderedTemplate> {
        let template = self.get_template(template_id).await?;
        
        // Validate variables
        self.validator.validate_variables(&template, variables)?;
        
        // TODO: Implement actual rendering with template engine
        // For now, return placeholder
        Ok(RenderedTemplate {
            template_id: template_id.to_string(),
            content: format!("Rendered content for template: {}", template_id),
            variables: variables.clone(),
            render_metadata: crate::models::RenderMetadata {
                rendered_at: chrono::Utc::now(),
                render_time_ms: 10,
                engine_used: "placeholder".to_string(),
                sections_rendered: vec!["all".to_string()],
                warnings: Vec::new(),
            },
        })
    }

    async fn render_action_response(&self, action: &Action) -> TemplateResult<String> {
        // Check if action already has template metadata
        if let Some(template_meta) = &action.template_metadata {
            let rendered = self.render_template(&template_meta.template_id, &template_meta.variables).await?;
            return Ok(rendered.content);
        }

        // Auto-detect template
        if let Some(template_id) = self.detect_template_for_action(action).await? {
            // Create variables from action context
            let variables = self.create_variables_from_action(action);
            let rendered = self.render_template(&template_id, &variables).await?;
            Ok(rendered.content)
        } else {
            // Fallback to default action format
            Ok(format!("Action: {} - {}", action.action_type.as_str(), action.description))
        }
    }

    async fn create_template(&self, template: &Template) -> TemplateResult<String> {
        self.validator.validate_template(template)?;
        self.repository.save(template).await?;
        Ok(template.template_id.clone())
    }

    async fn update_template(&self, template_id: &str, template: &Template) -> TemplateResult<()> {
        self.validator.validate_template(template)?;
        self.repository.update(template).await
    }

    async fn delete_template(&self, template_id: &str) -> TemplateResult<()> {
        self.repository.delete(template_id).await
    }

    async fn record_usage(&self, template_id: &str, user_satisfaction: Option<u8>) -> TemplateResult<()> {
        // TODO: Implement usage analytics
        Ok(())
    }
}

impl<R, D> DefaultTemplateService<R, D>
where
    R: TemplateRepository + Send + Sync,
    D: TemplateDetector + Send + Sync,
{
    /// Create template variables from action
    fn create_variables_from_action(&self, action: &Action) -> HashMap<String, serde_json::Value> {
        let mut variables = HashMap::new();
        
        variables.insert("action_type".to_string(), serde_json::Value::String(action.action_type.as_str().to_string()));
        variables.insert("description".to_string(), serde_json::Value::String(action.description.clone()));
        variables.insert("timestamp".to_string(), serde_json::Value::String(action.timestamp.to_rfc3339()));
        
        if !action.tags.is_empty() {
            let tags: Vec<serde_json::Value> = action.tags.iter()
                .map(|t| serde_json::Value::String(t.clone()))
                .collect();
            variables.insert("tags".to_string(), serde_json::Value::Array(tags));
        }

        // Add context variables
        if let Some(git_branch) = &action.context.git_branch {
            variables.insert("git_branch".to_string(), serde_json::Value::String(git_branch.clone()));
        }
        
        if !action.context.files_affected.is_empty() {
            let files: Vec<serde_json::Value> = action.context.files_affected.iter()
                .map(|f| serde_json::Value::String(f.clone()))
                .collect();
            variables.insert("files_affected".to_string(), serde_json::Value::Array(files));
        }

        variables
    }
}
