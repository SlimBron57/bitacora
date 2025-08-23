use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Information about a Git branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    /// Branch name
    pub name: String,
    
    /// Whether this is the current branch
    pub is_current: bool,
    
    /// Latest commit hash on this branch
    pub commit_hash: String,
    
    /// Upstream branch (if any)
    pub upstream: Option<String>,
}

/// Information about a Git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Commit hash
    pub hash: String,
    
    /// Commit message
    pub message: String,
    
    /// Branch where commit was made
    pub branch: String,
    
    /// Commit timestamp
    pub timestamp: DateTime<Utc>,
}

/// Repository status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryStatus {
    /// Modified files
    pub modified_files: Vec<String>,
    
    /// Staged files
    pub staged_files: Vec<String>,
    
    /// Untracked files
    pub untracked_files: Vec<String>,
    
    /// Current branch name
    pub current_branch: String,
    
    /// Whether working directory is clean
    pub is_clean: bool,
}
