use crate::{GitService, GitError, CommitInfo, PushCounter};
use std::sync::Arc;
use tracing::{info, warn};

/// Auto-commit functionality
pub struct AutoCommit {
    git_service: Arc<dyn GitService>,
    push_counter: PushCounter,
    auto_push_threshold: u32,
}

impl AutoCommit {
    pub fn new(git_service: Arc<dyn GitService>, push_counter: PushCounter, auto_push_threshold: u32) -> Self {
        Self {
            git_service,
            push_counter,
            auto_push_threshold,
        }
    }
    
    /// Auto-commit with optional push
    pub async fn auto_commit(&self, message: &str) -> Result<CommitInfo, GitError> {
        info!("Performing auto-commit: {}", message);
        
        // Check if there are changes to commit
        if self.git_service.is_working_directory_clean().await? {
            warn!("No changes to commit for auto-commit");
            return Err(GitError::CommandFailed { 
                message: "No changes to auto-commit".to_string() 
            });
        }
        
        // Create commit
        let commit_info = self.git_service.commit(message).await?;
        
        // Check if auto-push should be triggered
        let should_push = self.push_counter.is_threshold_reached(self.auto_push_threshold).await?;
        
        if should_push {
            info!("Auto-push threshold reached, pushing commits...");
            match self.git_service.push().await {
                Ok(_) => info!("Auto-push completed successfully"),
                Err(e) => warn!("Auto-push failed: {}", e),
            }
        }
        
        Ok(commit_info)
    }
    
    /// Auto-commit session start
    pub async fn commit_session_start(&self, session_id: &str, project: &str) -> Result<CommitInfo, GitError> {
        let message = format!("Session start: {} - {}", session_id, project);
        self.auto_commit(&message).await
    }
    
    /// Auto-commit session end
    pub async fn commit_session_end(&self, session_id: &str) -> Result<CommitInfo, GitError> {
        let message = format!("Session end: {}", session_id);
        self.auto_commit(&message).await
    }
    
    /// Auto-commit action
    pub async fn commit_action(&self, action_type: &str, target: &str) -> Result<CommitInfo, GitError> {
        let message = format!("Action: {} - {}", action_type, target);
        self.auto_commit(&message).await
    }
    
    /// Auto-commit with force push (ignores threshold)
    pub async fn auto_commit_and_push(&self, message: &str) -> Result<CommitInfo, GitError> {
        let commit_info = self.git_service.commit(message).await?;
        
        info!("Force pushing after commit...");
        self.git_service.push().await?;
        
        Ok(commit_info)
    }
}
