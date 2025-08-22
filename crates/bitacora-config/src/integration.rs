use serde::{Deserialize, Serialize};
use crate::{ConfigError, ConfigResult};

/// External integrations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    /// Git integration configuration
    #[serde(default)]
    pub git: GitIntegrationConfig,
    
    /// GitHub integration configuration
    pub github: Option<GitHubConfig>,
    
    /// Backup integrations
    #[serde(default)]
    pub backup: BackupIntegrationConfig,
    
    /// Notification integrations
    #[serde(default)]
    pub notifications: NotificationConfig,
    
    /// Monitoring integrations
    #[serde(default)]
    pub monitoring: MonitoringConfig,
}

/// Git integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitIntegrationConfig {
    /// Enable automatic git operations
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Auto-commit threshold (number of actions)
    #[serde(default = "default_auto_commit_threshold")]
    pub auto_commit_threshold: u32,
    
    /// Auto-push threshold (number of commits)
    #[serde(default = "default_auto_push_threshold")]
    pub auto_push_threshold: u32,
    
    /// Default branch naming pattern
    #[serde(default = "default_branch_pattern")]
    pub branch_pattern: String,
    
    /// Default commit message template
    #[serde(default = "default_commit_template")]
    pub commit_message_template: String,
    
    /// Git user name (if different from system)
    pub user_name: Option<String>,
    
    /// Git user email (if different from system)
    pub user_email: Option<String>,
}

/// GitHub integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    /// GitHub personal access token (provide via environment variable)
    pub token: String,
    
    /// GitHub API base URL (for enterprise)
    #[serde(default = "default_github_api")]
    pub api_url: String,
    
    /// Default repository owner/name
    pub default_repo: Option<String>,
    
    /// Enable automatic PR creation
    #[serde(default)]
    pub auto_pr: bool,
    
    /// PR template
    pub pr_template: Option<String>,
    
    /// Enable issue integration
    #[serde(default)]
    pub issue_integration: bool,
}

/// Backup integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupIntegrationConfig {
    /// Enable automatic backups
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Backup frequency in hours
    #[serde(default = "default_backup_frequency")]
    pub frequency_hours: u32,
    
    /// Local backup configuration
    #[serde(default)]
    pub local: LocalBackupConfig,
    
    /// S3-compatible storage configuration
    pub s3: Option<S3BackupConfig>,
    
    /// Google Cloud Storage configuration
    pub gcs: Option<GcsBackupConfig>,
    
    /// Encryption configuration
    #[serde(default)]
    pub encryption: BackupEncryptionConfig,
}

/// Local backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalBackupConfig {
    /// Enable local backups
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Local backup directory
    #[serde(default = "default_backup_path")]
    pub path: String,
    
    /// Retention policy (days)
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
    
    /// Enable compression
    #[serde(default = "default_true")]
    pub compress: bool,
}

/// S3-compatible backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3BackupConfig {
    /// S3 endpoint URL
    pub endpoint: String,
    
    /// AWS region
    #[serde(default = "default_s3_region")]
    pub region: String,
    
    /// S3 bucket name
    pub bucket: String,
    
    /// S3 access key
    pub access_key: String,
    
    /// S3 secret key (provide via environment variable)
    pub secret_key: String,
    
    /// Path prefix in bucket
    #[serde(default = "default_s3_prefix")]
    pub prefix: String,
}

/// Google Cloud Storage backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcsBackupConfig {
    /// GCS project ID
    pub project_id: String,
    
    /// GCS bucket name
    pub bucket: String,
    
    /// Service account key JSON (base64 encoded, provide via environment variable)
    pub service_account_key: String,
    
    /// Path prefix in bucket
    #[serde(default = "default_gcs_prefix")]
    pub prefix: String,
}

/// Backup encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEncryptionConfig {
    /// Enable encryption
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Encryption key (base64 encoded, provide via environment variable)
    pub key: Option<String>,
    
    /// Key derivation method
    #[serde(default = "default_kdf")]
    pub key_derivation: String,
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enable notifications
    #[serde(default)]
    pub enabled: bool,
    
    /// Email notifications
    pub email: Option<EmailNotificationConfig>,
    
    /// Slack notifications
    pub slack: Option<SlackNotificationConfig>,
    
    /// Discord notifications
    pub discord: Option<DiscordNotificationConfig>,
}

/// Email notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailNotificationConfig {
    /// SMTP server
    pub smtp_server: String,
    
    /// SMTP port
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    
    /// SMTP username
    pub username: String,
    
    /// SMTP password (provide via environment variable)
    pub password: String,
    
    /// From email address
    pub from: String,
    
    /// To email addresses
    pub to: Vec<String>,
    
    /// Enable TLS
    #[serde(default = "default_true")]
    pub tls: bool,
}

/// Slack notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackNotificationConfig {
    /// Slack webhook URL (provide via environment variable)
    pub webhook_url: String,
    
    /// Default channel
    pub channel: Option<String>,
    
    /// Bot username
    pub username: Option<String>,
}

/// Discord notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordNotificationConfig {
    /// Discord webhook URL (provide via environment variable)
    pub webhook_url: String,
    
    /// Bot username
    pub username: Option<String>,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable monitoring
    #[serde(default)]
    pub enabled: bool,
    
    /// Prometheus metrics
    pub prometheus: Option<PrometheusConfig>,
    
    /// OpenTelemetry configuration
    pub opentelemetry: Option<OpenTelemetryConfig>,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Prometheus endpoint
    #[serde(default = "default_prometheus_endpoint")]
    pub endpoint: String,
    
    /// Metrics prefix
    #[serde(default = "default_metrics_prefix")]
    pub prefix: String,
    
    /// Collection interval in seconds
    #[serde(default = "default_collection_interval")]
    pub interval: u64,
}

/// OpenTelemetry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenTelemetryConfig {
    /// OTLP endpoint
    pub endpoint: String,
    
    /// Service name
    #[serde(default = "default_service_name")]
    pub service_name: String,
    
    /// Headers for authentication
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

impl IntegrationConfig {
    /// Validate integration configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate Git config
        self.git.validate()?;
        
        // Validate GitHub config if present
        if let Some(github) = &self.github {
            github.validate()?;
        }
        
        // Validate backup config
        self.backup.validate()?;
        
        // Validate notification config
        self.notifications.validate()?;
        
        Ok(())
    }
}

impl GitIntegrationConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.auto_commit_threshold == 0 {
            return Err(ConfigError::ValidationError("Auto commit threshold must be greater than 0".to_string()));
        }
        
        if self.auto_push_threshold == 0 {
            return Err(ConfigError::ValidationError("Auto push threshold must be greater than 0".to_string()));
        }
        
        Ok(())
    }
}

impl GitHubConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.token.is_empty() {
            return Err(ConfigError::ValidationError("GitHub token cannot be empty".to_string()));
        }
        
        if self.api_url.is_empty() {
            return Err(ConfigError::ValidationError("GitHub API URL cannot be empty".to_string()));
        }
        
        Ok(())
    }
}

impl BackupIntegrationConfig {
    fn validate(&self) -> ConfigResult<()> {
        if self.frequency_hours == 0 {
            return Err(ConfigError::ValidationError("Backup frequency must be greater than 0".to_string()));
        }
        
        // Validate S3 config if present
        if let Some(s3) = &self.s3 {
            if s3.endpoint.is_empty() || s3.bucket.is_empty() {
                return Err(ConfigError::ValidationError("S3 endpoint and bucket cannot be empty".to_string()));
            }
        }
        
        // Validate GCS config if present
        if let Some(gcs) = &self.gcs {
            if gcs.project_id.is_empty() || gcs.bucket.is_empty() {
                return Err(ConfigError::ValidationError("GCS project ID and bucket cannot be empty".to_string()));
            }
        }
        
        Ok(())
    }
}

impl NotificationConfig {
    fn validate(&self) -> ConfigResult<()> {
        // Validate email config if present
        if let Some(email) = &self.email {
            if email.smtp_server.is_empty() || email.from.is_empty() {
                return Err(ConfigError::ValidationError("Email SMTP server and from address cannot be empty".to_string()));
            }
            
            if email.to.is_empty() {
                return Err(ConfigError::ValidationError("Email to addresses cannot be empty".to_string()));
            }
        }
        
        Ok(())
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            git: GitIntegrationConfig::default(),
            github: None,
            backup: BackupIntegrationConfig::default(),
            notifications: NotificationConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for GitIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            auto_commit_threshold: default_auto_commit_threshold(),
            auto_push_threshold: default_auto_push_threshold(),
            branch_pattern: default_branch_pattern(),
            commit_message_template: default_commit_template(),
            user_name: None,
            user_email: None,
        }
    }
}

impl Default for BackupIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            frequency_hours: default_backup_frequency(),
            local: LocalBackupConfig::default(),
            s3: None,
            gcs: None,
            encryption: BackupEncryptionConfig::default(),
        }
    }
}

impl Default for LocalBackupConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            path: default_backup_path(),
            retention_days: default_retention_days(),
            compress: default_true(),
        }
    }
}

impl Default for BackupEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            key: None,
            key_derivation: default_kdf(),
        }
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            email: None,
            slack: None,
            discord: None,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            prometheus: None,
            opentelemetry: None,
        }
    }
}

// Default value functions
fn default_true() -> bool { true }
fn default_auto_commit_threshold() -> u32 { 5 }
fn default_auto_push_threshold() -> u32 { 3 }
fn default_branch_pattern() -> String { "YYYYMMDD-HHMM_{topic}".to_string() }
fn default_commit_template() -> String { "Bitacora session: {session_id} - {summary}".to_string() }
fn default_github_api() -> String { "https://api.github.com".to_string() }
fn default_backup_frequency() -> u32 { 24 }
fn default_backup_path() -> String { "./backups".to_string() }
fn default_retention_days() -> u32 { 30 }
fn default_s3_region() -> String { "us-east-1".to_string() }
fn default_s3_prefix() -> String { "bitacora/".to_string() }
fn default_gcs_prefix() -> String { "bitacora/".to_string() }
fn default_kdf() -> String { "pbkdf2".to_string() }
fn default_smtp_port() -> u16 { 587 }
fn default_prometheus_endpoint() -> String { "/metrics".to_string() }
fn default_metrics_prefix() -> String { "bitacora_".to_string() }
fn default_collection_interval() -> u64 { 30 }
fn default_service_name() -> String { "bitacora".to_string() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_integration_config() {
        let config = IntegrationConfig::default();
        assert!(config.git.enabled);
        assert!(config.backup.enabled);
        assert!(!config.notifications.enabled);
        assert!(!config.monitoring.enabled);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_git_integration_validation() {
        let config = GitIntegrationConfig {
            auto_commit_threshold: 0,
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_github_config_validation() {
        let config = GitHubConfig {
            token: "".to_string(),
            api_url: "https://api.github.com".to_string(),
            default_repo: None,
            auto_pr: false,
            pr_template: None,
            issue_integration: false,
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_backup_config_validation() {
        let config = BackupIntegrationConfig {
            frequency_hours: 0,
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }
}
