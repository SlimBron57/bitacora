use async_trait::async_trait;
use bitacora_core::prelude::{BitacoraError, Result as BitacoraResult};
use crate::{GitError, BranchInfo, CommitInfo, RepositoryStatus};
use std::path::PathBuf;

/// Service trait for Git operations
#[async_trait]
pub trait GitService: Send + Sync {
    /// Initialize a new Git repository
    async fn init_repository(&self, path: &PathBuf) -> BitacoraResult<()>;
    
    /// Create a new branch
    async fn create_branch(&self, branch_name: &str) -> Result<BranchInfo, GitError>;
    
    /// Switch to an existing branch
    async fn switch_branch(&self, branch_name: &str) -> Result<(), GitError>;
    
    /// Get current branch information
    async fn current_branch(&self) -> Result<BranchInfo, GitError>;
    
    /// List all branches
    async fn list_branches(&self) -> Result<Vec<BranchInfo>, GitError>;
    
    /// Commit changes with a message
    async fn commit(&self, message: &str) -> Result<CommitInfo, GitError>;
    
    /// Push commits to remote
    async fn push(&self) -> Result<(), GitError>;
    
    /// Get repository status
    async fn status(&self) -> Result<RepositoryStatus, GitError>;
    
    /// Check if working directory is clean
    async fn is_working_directory_clean(&self) -> Result<bool, GitError>;
    
    /// Get commit count since last push
    async fn unpushed_commits_count(&self) -> Result<u32, GitError>;
    
    /// Auto-push if threshold is reached
    async fn auto_push_if_needed(&self) -> Result<bool, GitError>;
    
    /// Validate repository state
    async fn validate_repository(&self) -> Result<(), GitError>;
}
