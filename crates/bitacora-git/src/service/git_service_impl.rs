use async_trait::async_trait;
use bitacora_core::prelude::{BitacoraError, Result as BitacoraResult};
use bitacora_core::utils::timestamp::now_bitacora;
use crate::{
    GitService, GitError, GitConfig, CommandExecutor,
    BranchInfo, CommitInfo, RepositoryStatus,
    PushCounter, BranchManager, RepositoryManager
};
use std::path::PathBuf;
use tracing::{info, debug, warn};

/// Implementation of GitService using system Git commands
pub struct GitServiceImpl {
    config: GitConfig,
    executor: CommandExecutor,
    push_counter: PushCounter,
    branch_manager: BranchManager,
    repository_manager: RepositoryManager,
}

impl GitServiceImpl {
    pub fn new(config: GitConfig) -> Self {
        let executor = CommandExecutor::new(config.repository_path.clone());
        let push_counter = PushCounter::new(config.auto_push.counter_file_path.clone());
        let branch_manager = BranchManager::new(config.branch_naming.clone());
        let repository_manager = RepositoryManager::new(config.repository_path.clone());
        
        Self {
            config,
            executor,
            push_counter,
            branch_manager,
            repository_manager,
        }
    }
    
    /// Create a branch with timestamp-based naming
    async fn create_timestamped_branch(&self, base_name: &str, branch_type: &str) -> Result<String, GitError> {
        let timestamp = now_bitacora();
        let branch_name = if self.config.branch_naming.use_timestamp {
            format!("{}-{}-{}", branch_type, timestamp, base_name)
        } else {
            format!("{}-{}", branch_type, base_name)
        };
        
        // Truncate if too long
        let final_name = if branch_name.len() > self.config.branch_naming.max_length {
            branch_name[..self.config.branch_naming.max_length].to_string()
        } else {
            branch_name
        };
        
        self.branch_manager.validate_branch_name(&final_name)?;
        Ok(final_name)
    }
}

#[async_trait]
impl GitService for GitServiceImpl {
    async fn init_repository(&self, path: &PathBuf) -> BitacoraResult<()> {
        info!("Initializing Git repository at: {:?}", path);
        
        let executor = CommandExecutor::new(path.clone());
        executor.check_git_available().await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
        
        if !executor.is_git_repository().await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))? {
            
            executor.execute(&["init"]).await
                .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
            info!("Git repository initialized successfully");
        } else {
            info!("Git repository already exists");
        }
        
        Ok(())
    }
    
    async fn create_branch(&self, branch_name: &str) -> Result<BranchInfo, GitError> {
        info!("Creating branch: {}", branch_name);
        
        // Check if branch already exists
        if let Ok(branches) = self.list_branches().await {
            if branches.iter().any(|b| b.name == branch_name) {
                return Err(GitError::BranchAlreadyExists { 
                    branch: branch_name.to_string() 
                });
            }
        }
        
        // Create and switch to the new branch
        self.executor.execute(&["checkout", "-b", branch_name]).await?;
        
        let branch_info = BranchInfo {
            name: branch_name.to_string(),
            is_current: true,
            commit_hash: self.executor.execute(&["rev-parse", "HEAD"]).await.unwrap_or_default(),
            upstream: None,
        };
        
        info!("Branch '{}' created successfully", branch_name);
        Ok(branch_info)
    }
    
    async fn switch_branch(&self, branch_name: &str) -> Result<(), GitError> {
        info!("Switching to branch: {}", branch_name);
        
        self.executor.execute(&["checkout", branch_name]).await?;
        info!("Switched to branch '{}'", branch_name);
        Ok(())
    }
    
    async fn current_branch(&self) -> Result<BranchInfo, GitError> {
        let branch_name = self.executor.execute(&["branch", "--show-current"]).await?;
        let commit_hash = self.executor.execute(&["rev-parse", "HEAD"]).await?;
        
        // Try to get upstream branch
        let upstream = self.executor.execute_check(&["rev-parse", "--abbrev-ref", "@{u}"]).await?;
        
        Ok(BranchInfo {
            name: branch_name,
            is_current: true,
            commit_hash,
            upstream,
        })
    }
    
    async fn list_branches(&self) -> Result<Vec<BranchInfo>, GitError> {
        let output = self.executor.execute(&["branch", "-v"]).await?;
        let _current_branch = self.current_branch().await?;
        
        let branches: Result<Vec<BranchInfo>, GitError> = output
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| {
                let line = line.trim();
                let is_current = line.starts_with('*');
                
                let parts: Vec<&str> = if is_current {
                    line[1..].trim().split_whitespace().collect()
                } else {
                    line.split_whitespace().collect()
                };
                
                if parts.len() >= 2 {
                    Ok(BranchInfo {
                        name: parts[0].to_string(),
                        is_current,
                        commit_hash: parts[1].to_string(),
                        upstream: None, // Would need separate call to get upstream
                    })
                } else {
                    Err(GitError::ParseError { 
                        message: format!("Could not parse branch info: {}", line) 
                    })
                }
            })
            .collect();
        
        branches
    }
    
    async fn commit(&self, message: &str) -> Result<CommitInfo, GitError> {
        info!("Creating commit with message: {}", message);
        
        // Check if there are changes to commit
        let status = self.status().await?;
        if status.staged_files.is_empty() && status.modified_files.is_empty() {
            warn!("No changes to commit");
            return Err(GitError::CommandFailed { 
                message: "No changes to commit".to_string() 
            });
        }
        
        // Stage all changes if nothing is staged
        if status.staged_files.is_empty() {
            self.executor.execute(&["add", "."]).await?;
        }
        
        // Create commit
        self.executor.execute(&["commit", "-m", message]).await?;
        
        let commit_hash = self.executor.execute(&["rev-parse", "HEAD"]).await?;
        let branch = self.current_branch().await?;
        
        // Increment push counter
        self.push_counter.increment().await?;
        
        let commit_info = CommitInfo {
            hash: commit_hash,
            message: message.to_string(),
            branch: branch.name,
            timestamp: chrono::Utc::now(),
        };
        
        info!("Commit created: {}", commit_info.hash);
        Ok(commit_info)
    }
    
    async fn push(&self) -> Result<(), GitError> {
        info!("Pushing commits to remote");
        
        let branch = self.current_branch().await?;
        
        // Try to push to upstream, fallback to origin
        let push_result = if let Some(_upstream) = &branch.upstream {
            self.executor.execute(&["push"]).await
        } else {
            self.executor.execute(&["push", "origin", &branch.name]).await
        };
        
        match push_result {
            Ok(_) => {
                // Reset push counter after successful push
                self.push_counter.reset().await?;
                info!("Push completed successfully");
                Ok(())
            }
            Err(e) => {
                warn!("Push failed: {}", e);
                Err(GitError::PushFailed { reason: e.to_string() })
            }
        }
    }
    
    async fn status(&self) -> Result<RepositoryStatus, GitError> {
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
                    if status_chars.chars().nth(0) != Some(' ') {
                        staged_files.push(file_path.clone());
                    }
                    if status_chars.chars().nth(1) != Some(' ') {
                        modified_files.push(file_path);
                    }
                }
            }
        }
        
        Ok(RepositoryStatus {
            modified_files,
            staged_files,
            untracked_files,
            current_branch: self.current_branch().await?.name,
            is_clean: output.trim().is_empty(),
        })
    }
    
    async fn is_working_directory_clean(&self) -> Result<bool, GitError> {
        let status = self.status().await?;
        Ok(status.is_clean)
    }
    
    async fn unpushed_commits_count(&self) -> Result<u32, GitError> {
        let branch = self.current_branch().await?;
        
        if let Some(upstream) = &branch.upstream {
            let output = self.executor.execute_check(&[
                "rev-list", 
                "--count", 
                &format!("{}..HEAD", upstream)
            ]).await?;
            
            if let Some(count_str) = output {
                count_str.parse::<u32>()
                    .map_err(|e| GitError::ParseError { 
                        message: format!("Could not parse commit count: {}", e) 
                    })
            } else {
                Ok(0)
            }
        } else {
            // If no upstream, use push counter
            self.push_counter.current_count().await
                .map_err(|e| GitError::CommandFailed { message: e.to_string() })
        }
    }
    
    async fn auto_push_if_needed(&self) -> Result<bool, GitError> {
        if !self.config.auto_push.enabled {
            return Ok(false);
        }
        
        let unpushed_count = self.unpushed_commits_count().await?;
        
        if unpushed_count >= self.config.auto_push.push_threshold {
            info!("Auto-push threshold reached ({} commits), pushing...", unpushed_count);
            self.push().await?;
            Ok(true)
        } else {
            debug!("Auto-push not needed ({}/{} commits)", unpushed_count, self.config.auto_push.push_threshold);
            Ok(false)
        }
    }
    
    async fn validate_repository(&self) -> Result<(), GitError> {
        // Check if Git is available
        self.executor.check_git_available().await?;
        
        // Check if we're in a Git repository
        if !self.executor.is_git_repository().await? {
            return Err(GitError::RepositoryNotFound { 
                path: self.config.repository_path.display().to_string() 
            });
        }
        
        // Validate repository structure
        self.repository_manager.validate().await?;
        
        Ok(())
    }
}
