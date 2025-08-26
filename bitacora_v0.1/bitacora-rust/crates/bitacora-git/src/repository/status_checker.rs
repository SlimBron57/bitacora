use crate::{GitError, CommandExecutor, RepositoryStatus};
use std::path::PathBuf;

/// Checks repository status and health
pub struct StatusChecker {
    executor: CommandExecutor,
}

impl StatusChecker {
    pub fn new(repository_path: PathBuf) -> Self {
        let executor = CommandExecutor::new(repository_path);
        Self { executor }
    }
    
    /// Get detailed repository status
    pub async fn get_status(&self) -> Result<RepositoryStatus, GitError> {
        let output = self.executor.execute(&["status", "--porcelain"]).await?;
        
        let mut modified_files = Vec::new();
        let mut staged_files = Vec::new();
        let mut untracked_files = Vec::new();
        
        for line in output.lines() {
            if line.len() < 3 {
                continue;
            }
            
            let status_chars = &line[0..2];
            let file_path = line[3..].to_string();
            
            match status_chars {
                "??" => untracked_files.push(file_path),
                _ => {
                    // First character is staged status
                    if status_chars.chars().nth(0) != Some(' ') {
                        staged_files.push(file_path.clone());
                    }
                    // Second character is working tree status
                    if status_chars.chars().nth(1) != Some(' ') {
                        modified_files.push(file_path);
                    }
                }
            }
        }
        
        let current_branch = self.executor.execute(&["branch", "--show-current"]).await
            .unwrap_or_else(|_| "unknown".to_string());
        
        Ok(RepositoryStatus {
            modified_files,
            staged_files,
            untracked_files,
            current_branch,
            is_clean: output.trim().is_empty(),
        })
    }
    
    /// Check if repository is healthy
    pub async fn health_check(&self) -> Result<HealthStatus, GitError> {
        let mut issues = Vec::new();
        
        // Check if Git is available
        if let Err(e) = self.executor.check_git_available().await {
            issues.push(format!("Git not available: {}", e));
        }
        
        // Check if we're in a repository
        if !self.executor.is_git_repository().await? {
            issues.push("Not in a Git repository".to_string());
        } else {
            // Additional checks for existing repository
            
            // Check if repository has commits
            if self.executor.execute_check(&["rev-parse", "HEAD"]).await?.is_none() {
                issues.push("Repository has no commits".to_string());
            }
            
            // Check for untracked files
            let status = self.get_status().await?;
            if !status.untracked_files.is_empty() {
                issues.push(format!("{} untracked files", status.untracked_files.len()));
            }
            
            // Check for uncommitted changes
            if !status.modified_files.is_empty() {
                issues.push(format!("{} modified files", status.modified_files.len()));
            }
        }
        
        Ok(HealthStatus {
            is_healthy: issues.is_empty(),
            issues,
        })
    }
    
    /// Count files by status
    pub async fn count_files_by_status(&self) -> Result<FileStatusCounts, GitError> {
        let status = self.get_status().await?;
        
        Ok(FileStatusCounts {
            modified: status.modified_files.len(),
            staged: status.staged_files.len(),
            untracked: status.untracked_files.len(),
        })
    }
}

#[derive(Debug)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub issues: Vec<String>,
}

#[derive(Debug)]
pub struct FileStatusCounts {
    pub modified: usize,
    pub staged: usize,
    pub untracked: usize,
}
