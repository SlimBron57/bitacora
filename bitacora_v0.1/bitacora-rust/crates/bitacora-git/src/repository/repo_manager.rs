use crate::{GitError, CommandExecutor};
use std::path::PathBuf;

/// Manages repository operations and validation
pub struct RepositoryManager {
    repository_path: PathBuf,
    executor: CommandExecutor,
}

impl RepositoryManager {
    pub fn new(repository_path: PathBuf) -> Self {
        let executor = CommandExecutor::new(repository_path.clone());
        Self { repository_path, executor }
    }
    
    /// Validate repository state
    pub async fn validate(&self) -> Result<(), GitError> {
        // Check if Git is available
        self.executor.check_git_available().await?;
        
        // Check if we're in a Git repository
        if !self.executor.is_git_repository().await? {
            return Err(GitError::RepositoryNotFound { 
                path: self.repository_path.display().to_string() 
            });
        }
        
        // Check if repository has at least one commit
        match self.executor.execute_check(&["rev-parse", "HEAD"]).await? {
            Some(_) => Ok(()),
            None => {
                // Empty repository is valid, but warn about it
                tracing::warn!("Repository has no commits yet");
                Ok(())
            }
        }
    }
    
    /// Initialize repository if it doesn't exist
    pub async fn ensure_initialized(&self) -> Result<(), GitError> {
        if !self.executor.is_git_repository().await? {
            tracing::info!("Initializing Git repository at {:?}", self.repository_path);
            self.executor.execute(&["init"]).await?;
        }
        Ok(())
    }
    
    /// Get repository root path
    pub async fn get_root_path(&self) -> Result<PathBuf, GitError> {
        self.executor.get_repository_root().await
    }
    
    /// Check if repository has remote
    pub async fn has_remote(&self, remote_name: &str) -> Result<bool, GitError> {
        match self.executor.execute_check(&["remote", "get-url", remote_name]).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    
    /// Add remote to repository
    pub async fn add_remote(&self, name: &str, url: &str) -> Result<(), GitError> {
        self.executor.execute(&["remote", "add", name, url]).await?;
        Ok(())
    }
    
    /// Get remote URL
    pub async fn get_remote_url(&self, remote_name: &str) -> Result<Option<String>, GitError> {
        self.executor.execute_check(&["remote", "get-url", remote_name]).await
    }
    
    /// List all remotes
    pub async fn list_remotes(&self) -> Result<Vec<String>, GitError> {
        let output = self.executor.execute(&["remote"]).await?;
        Ok(output.lines().map(|s| s.to_string()).collect())
    }
    
    /// Check if working directory is inside Git repository
    pub fn is_inside_repository(&self) -> bool {
        // Simple check - more sophisticated validation in validate()
        self.repository_path.join(".git").exists() || 
        self.repository_path.ancestors().any(|p| p.join(".git").exists())
    }
}
