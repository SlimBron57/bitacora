use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// Path to the Git repository
    pub repository_path: PathBuf,
    
    /// Default branch name (usually "master" or "main")
    pub default_branch: String,
    
    /// Auto-push settings
    pub auto_push: AutoPushConfig,
    
    /// Commit message template
    pub commit_template: CommitTemplate,
    
    /// Branch naming strategy
    pub branch_naming: BranchNamingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoPushConfig {
    /// Enable automatic pushing
    pub enabled: bool,
    
    /// Number of commits before auto-push is triggered
    pub push_threshold: u32,
    
    /// Path to push counter file
    pub counter_file_path: PathBuf,
}

impl Default for AutoPushConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            push_threshold: 5,
            counter_file_path: PathBuf::from("cache/push_counter.txt"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitTemplate {
    /// Template for session commits
    pub session_template: String,
    
    /// Template for action commits
    pub action_template: String,
    
    /// Template for branch commits
    pub branch_template: String,
}

impl Default for CommitTemplate {
    fn default() -> Self {
        Self {
            session_template: "Session: {session_id} - {message}".to_string(),
            action_template: "Action: {action_type} - {message}".to_string(),
            branch_template: "Branch: {branch_name} - {message}".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNamingConfig {
    /// Use timestamp in branch names
    pub use_timestamp: bool,
    
    /// Branch prefix for session branches
    pub session_prefix: String,
    
    /// Branch prefix for feature branches
    pub feature_prefix: String,
    
    /// Maximum branch name length
    pub max_length: usize,
}

impl Default for BranchNamingConfig {
    fn default() -> Self {
        Self {
            use_timestamp: true,
            session_prefix: "session".to_string(),
            feature_prefix: "feature".to_string(),
            max_length: 64,
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            repository_path: PathBuf::from("."),
            default_branch: "master".to_string(),
            auto_push: AutoPushConfig::default(),
            commit_template: CommitTemplate::default(),
            branch_naming: BranchNamingConfig::default(),
        }
    }
}
