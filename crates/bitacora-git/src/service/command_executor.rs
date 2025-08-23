use tokio::process::Command;
use crate::GitError;
use std::path::PathBuf;
use tracing::{debug, error, warn};

/// Executes Git commands asynchronously
pub struct CommandExecutor {
    repository_path: PathBuf,
}

impl CommandExecutor {
    pub fn new(repository_path: PathBuf) -> Self {
        Self { repository_path }
    }
    
    /// Execute a Git command with arguments
    pub async fn execute(&self, args: &[&str]) -> Result<String, GitError> {
        debug!("Executing git command: git {}", args.join(" "));
        
        let output = Command::new("git")
            .current_dir(&self.repository_path)
            .args(args)
            .output()
            .await
            .map_err(|e| GitError::Io { source: e })?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            debug!("Git command succeeded: {}", stdout);
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            error!("Git command failed: {}", stderr);
            Err(GitError::CommandFailed { message: stderr })
        }
    }
    
    /// Execute a Git command that may fail silently (for checking conditions)
    pub async fn execute_check(&self, args: &[&str]) -> Result<Option<String>, GitError> {
        debug!("Executing git check command: git {}", args.join(" "));
        
        let output = Command::new("git")
            .current_dir(&self.repository_path)
            .args(args)
            .output()
            .await
            .map_err(|e| GitError::Io { source: e })?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            debug!("Git check command succeeded: {}", stdout);
            Ok(Some(stdout))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            warn!("Git check command failed (expected): {}", stderr);
            Ok(None)
        }
    }
    
    /// Check if git command is available
    pub async fn check_git_available(&self) -> Result<(), GitError> {
        Command::new("git")
            .arg("--version")
            .output()
            .await
            .map_err(|e| GitError::CommandFailed { 
                message: format!("Git not available: {}", e)
            })?;
        Ok(())
    }
    
    /// Check if current directory is a Git repository
    pub async fn is_git_repository(&self) -> Result<bool, GitError> {
        match self.execute_check(&["rev-parse", "--git-dir"]).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    
    /// Get the root directory of the Git repository
    pub async fn get_repository_root(&self) -> Result<PathBuf, GitError> {
        let output = self.execute(&["rev-parse", "--show-toplevel"]).await?;
        Ok(PathBuf::from(output.trim()))
    }
}
