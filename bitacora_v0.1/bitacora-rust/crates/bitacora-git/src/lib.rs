pub mod service;
pub mod branch;
pub mod commit;
pub mod repository;
pub mod config;
pub mod errors;

pub use service::*;
pub use branch::*;
pub use commit::*;
pub use repository::*;
pub use config::*;
pub use errors::*;

// Re-export core types that are commonly used
pub use bitacora_core::prelude::{BitacoraError, Result as BitacoraResult};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_branch_manager_validation() {
        let config = BranchNamingConfig::default();
        let manager = BranchManager::new(config);
        
        // Valid names
        assert!(manager.validate_branch_name("feature-test").is_ok());
        assert!(manager.validate_branch_name("20250822-1234-feature").is_ok());
        
        // Invalid names
        assert!(manager.validate_branch_name("").is_err());
        assert!(manager.validate_branch_name("master").is_err());
        assert!(manager.validate_branch_name("branch with spaces").is_err());
    }

    #[tokio::test]
    async fn test_push_counter() {
        let temp_dir = TempDir::new().unwrap();
        let counter_file = temp_dir.path().join("counter.txt");
        let counter = PushCounter::new(counter_file);
        
        // Initial state
        assert_eq!(counter.current_count().await.unwrap(), 0);
        
        // Increment
        assert_eq!(counter.increment().await.unwrap(), 1);
        assert_eq!(counter.increment().await.unwrap(), 2);
        
        // Reset
        counter.reset().await.unwrap();
        assert_eq!(counter.current_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_message_builder() {
        let template = CommitTemplate::default();
        let builder = MessageBuilder::new(template);
        
        // Test simple message
        let message = MessageBuilder::create_simple_message("Test", "content");
        assert_eq!(message, "Test: content");
        
        // Test truncation
        let long_message = "a".repeat(100);
        let truncated = builder.truncate_subject(&long_message, 50);
        assert!(truncated.len() <= 50);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_git_config_default() {
        let config = GitConfig::default();
        assert_eq!(config.default_branch, "master");
        assert!(config.auto_push.enabled);
        assert_eq!(config.auto_push.push_threshold, 5);
    }
}
