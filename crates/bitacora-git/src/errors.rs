use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {message}")]
    CommandFailed { message: String },
    
    #[error("Repository not found at path: {path}")]
    RepositoryNotFound { path: String },
    
    #[error("Branch validation failed: {reason}")]
    BranchValidationFailed { reason: String },
    
    #[error("Working directory is dirty")]
    WorkingDirectoryDirty,
    
    #[error("Branch already exists: {branch}")]
    BranchAlreadyExists { branch: String },
    
    #[error("Push failed: {reason}")]
    PushFailed { reason: String },
    
    #[error("IO error: {source}")]
    Io { source: std::io::Error },
    
    #[error("Parse error: {message}")]
    ParseError { message: String },
}
