use crate::{CommitTemplate, GitError};
use bitacora_core::models::{Session, Action};
use std::collections::HashMap;

/// Builder for creating commit messages from templates
pub struct MessageBuilder {
    template: CommitTemplate,
}

impl MessageBuilder {
    pub fn new(template: CommitTemplate) -> Self {
        Self { template }
    }
    
    /// Build session commit message
    pub fn build_session_message(&self, session: &Session, custom_message: Option<&str>) -> Result<String, GitError> {
        let mut variables = HashMap::new();
        variables.insert("session_id".to_string(), session.session_id.to_string());
        variables.insert("project".to_string(), session.project_id.as_ref().map(|id| id.to_string()).unwrap_or_else(|| "unknown".to_string()));
        variables.insert("objective".to_string(), session.description.clone().unwrap_or_else(|| "session work".to_string()));
        
        if let Some(msg) = custom_message {
            variables.insert("message".to_string(), msg.to_string());
        } else {
            variables.insert("message".to_string(), format!("Working on {}", session.description.clone().unwrap_or_else(|| "session".to_string())));
        }
        
        self.substitute_variables(&self.template.session_template, &variables)
    }
    
    /// Build action commit message
    pub fn build_action_message(&self, action: &Action, custom_message: Option<&str>) -> Result<String, GitError> {
        let mut variables = HashMap::new();
        variables.insert("action_type".to_string(), format!("{:?}", action.action_type));
        variables.insert("target".to_string(), action.description.clone());
        
        if let Some(msg) = custom_message {
            variables.insert("message".to_string(), msg.to_string());
        } else {
            variables.insert("message".to_string(), format!("{:?} on {}", action.action_type, action.description));
        }
        
        self.substitute_variables(&self.template.action_template, &variables)
    }
    
    /// Build branch commit message
    pub fn build_branch_message(&self, branch_name: &str, custom_message: Option<&str>) -> Result<String, GitError> {
        let mut variables = HashMap::new();
        variables.insert("branch_name".to_string(), branch_name.to_string());
        
        if let Some(msg) = custom_message {
            variables.insert("message".to_string(), msg.to_string());
        } else {
            variables.insert("message".to_string(), format!("Created branch {}", branch_name));
        }
        
        self.substitute_variables(&self.template.branch_template, &variables)
    }
    
    /// Build generic commit message with custom variables
    pub fn build_custom_message(&self, template: &str, variables: &HashMap<String, String>) -> Result<String, GitError> {
        self.substitute_variables(template, variables)
    }
    
    /// Substitute variables in template string
    fn substitute_variables(&self, template: &str, variables: &HashMap<String, String>) -> Result<String, GitError> {
        let mut result = template.to_string();
        
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        // Check for unsubstituted placeholders
        if result.contains('{') && result.contains('}') {
            let remaining: Vec<&str> = result.matches('{')
                .zip(result.matches('}'))
                .take(5) // Limit to first 5 unsubstituted
                .map(|(_, _)| "placeholder")
                .collect();
            
            if !remaining.is_empty() {
                return Err(GitError::ParseError { 
                    message: format!("Unsubstituted placeholders found in template: {}", template) 
                });
            }
        }
        
        Ok(result)
    }
    
    /// Create a simple message with prefix
    pub fn create_simple_message(prefix: &str, content: &str) -> String {
        format!("{}: {}", prefix, content)
    }
    
    /// Truncate message to Git's recommended length (50 chars for first line)
    pub fn truncate_subject(&self, message: &str, max_length: usize) -> String {
        if message.len() <= max_length {
            message.to_string()
        } else {
            let truncated = &message[..max_length.saturating_sub(3)];
            format!("{}...", truncated)
        }
    }
}
