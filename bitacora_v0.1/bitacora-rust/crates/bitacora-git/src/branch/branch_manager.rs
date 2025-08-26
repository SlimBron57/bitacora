use crate::{GitError, BranchNamingConfig};
use regex::Regex;
use std::sync::OnceLock;

static VALID_BRANCH_NAME_REGEX: OnceLock<Regex> = OnceLock::new();

/// Manages branch operations
pub struct BranchManager {
    naming_config: BranchNamingConfig,
}

impl BranchManager {
    pub fn new(naming_config: BranchNamingConfig) -> Self {
        Self { naming_config }
    }
    
    /// Validate a branch name according to Git rules
    pub fn validate_branch_name(&self, name: &str) -> Result<(), GitError> {
        if name.is_empty() {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name cannot be empty".to_string() 
            });
        }
        
        if name.len() > self.naming_config.max_length {
            return Err(GitError::BranchValidationFailed { 
                reason: format!("Branch name too long (max: {})", self.naming_config.max_length) 
            });
        }
        
        // Git branch name rules
        let regex = VALID_BRANCH_NAME_REGEX.get_or_init(|| {
            // Git branch names cannot contain:
            // - spaces, ~, ^, :, ?, *, [, \, ..
            // - start with . or end with .lock
            // - contain double dots ..
            Regex::new(r"^[a-zA-Z0-9._/-]+$").unwrap()
        });
        
        if !regex.is_match(name) {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name contains invalid characters".to_string() 
            });
        }
        
        // Check for Git-specific invalid patterns
        if name.starts_with('.') || name.ends_with(".lock") || name.contains("..") {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name violates Git naming rules".to_string() 
            });
        }
        
        // Check for reserved names
        if matches!(name, "HEAD" | "master" | "main" | "origin" | "upstream") {
            return Err(GitError::BranchValidationFailed { 
                reason: "Branch name is reserved".to_string() 
            });
        }
        
        Ok(())
    }
    
    /// Sanitize a branch name to make it valid
    pub fn sanitize_branch_name(&self, name: &str) -> String {
        let mut sanitized = name.to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' | '.' | '_' | '-' | '/' => c,
                ' ' | '\t' | '\n' => '-',
                _ => '_',
            })
            .collect::<String>();
        
        // Remove consecutive special characters
        while sanitized.contains("--") || sanitized.contains("__") || sanitized.contains("..") {
            sanitized = sanitized
                .replace("--", "-")
                .replace("__", "_")
                .replace("..", ".");
        }
        
        // Trim special characters from start and end
        sanitized = sanitized.trim_matches(&['.', '_', '-'][..]).to_string();
        
        // Ensure not empty
        if sanitized.is_empty() {
            sanitized = "branch".to_string();
        }
        
        // Truncate if too long
        if sanitized.len() > self.naming_config.max_length {
            sanitized.truncate(self.naming_config.max_length);
            // Make sure we don't end with a special character after truncation
            sanitized = sanitized.trim_end_matches(&['.', '_', '-'][..]).to_string();
        }
        
        sanitized
    }
    
    /// Generate a session branch name
    pub fn generate_session_branch(&self, session_id: &str, description: Option<&str>) -> String {
        let mut name = format!("{}-{}", self.naming_config.session_prefix, session_id);
        
        if let Some(desc) = description {
            let clean_desc = self.sanitize_branch_name(desc);
            if !clean_desc.is_empty() {
                name = format!("{}-{}", name, clean_desc);
            }
        }
        
        self.sanitize_branch_name(&name)
    }
    
    /// Generate a feature branch name
    pub fn generate_feature_branch(&self, feature_name: &str, timestamp: Option<&str>) -> String {
        let mut name = format!("{}-{}", self.naming_config.feature_prefix, feature_name);
        
        if self.naming_config.use_timestamp {
            if let Some(ts) = timestamp {
                name = format!("{}-{}", ts, name);
            }
        }
        
        self.sanitize_branch_name(&name)
    }
    
    /// Check if a branch name follows the expected pattern
    pub fn matches_pattern(&self, name: &str, pattern_type: BranchPatternType) -> bool {
        match pattern_type {
            BranchPatternType::Session => name.starts_with(&self.naming_config.session_prefix),
            BranchPatternType::Feature => name.starts_with(&self.naming_config.feature_prefix),
            BranchPatternType::Any => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BranchPatternType {
    Session,
    Feature,
    Any,
}
