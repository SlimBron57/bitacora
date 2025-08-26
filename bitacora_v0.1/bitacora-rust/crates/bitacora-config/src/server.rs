use serde::{Deserialize, Serialize};
use crate::{ConfigError, ConfigResult};

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host/address to bind to
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Server port to bind to
    #[serde(default = "default_port")]
    pub port: u16,
    
    /// Maximum request body size in bytes
    #[serde(default = "default_max_body_size")]
    pub max_body_size: usize,
    
    /// Request timeout in seconds
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,
    
    /// Enable CORS
    #[serde(default = "default_true")]
    pub cors_enabled: bool,
    
    /// CORS allowed origins
    #[serde(default)]
    pub cors_origins: Vec<String>,
    
    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    
    /// TLS/SSL configuration
    pub tls: Option<TlsConfig>,
    
    /// Server workers (0 = auto-detect)
    #[serde(default)]
    pub workers: usize,
    
    /// Keep-alive timeout in seconds
    #[serde(default = "default_keepalive")]
    pub keepalive: u64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Requests per minute per IP
    #[serde(default = "default_rpm")]
    pub requests_per_minute: u32,
    
    /// Burst capacity (requests allowed above rate limit)
    #[serde(default = "default_burst")]
    pub burst: u32,
    
    /// Rate limit window in seconds
    #[serde(default = "default_window")]
    pub window_seconds: u64,
}

/// TLS/SSL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Path to certificate file
    pub cert_path: String,
    
    /// Path to private key file
    pub key_path: String,
    
    /// TLS protocol versions to support
    #[serde(default)]
    pub protocols: Vec<String>,
    
    /// Cipher suites to support
    #[serde(default)]
    pub cipher_suites: Vec<String>,
}

impl ServerConfig {
    /// Get full server bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    
    /// Check if TLS is enabled
    pub fn is_tls_enabled(&self) -> bool {
        self.tls.is_some()
    }
    
    /// Check if CORS is enabled
    pub fn is_cors_enabled(&self) -> bool {
        self.cors_enabled
    }
    
    /// Check if rate limiting is enabled
    pub fn is_rate_limit_enabled(&self) -> bool {
        self.rate_limit.enabled
    }
    
    /// Validate server configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate port
        if self.port == 0 {
            return Err(ConfigError::ValidationError("Server port must be greater than 0".to_string()));
        }
        
        // Validate host
        if self.host.is_empty() {
            return Err(ConfigError::ValidationError("Server host cannot be empty".to_string()));
        }
        
        // Validate timeouts
        if self.request_timeout == 0 {
            return Err(ConfigError::ValidationError("Request timeout must be greater than 0".to_string()));
        }
        
        if self.keepalive == 0 {
            return Err(ConfigError::ValidationError("Keepalive timeout must be greater than 0".to_string()));
        }
        
        // Validate max body size
        if self.max_body_size == 0 {
            return Err(ConfigError::ValidationError("Max body size must be greater than 0".to_string()));
        }
        
        // Validate rate limit config
        if self.rate_limit.enabled {
            if self.rate_limit.requests_per_minute == 0 {
                return Err(ConfigError::ValidationError("Requests per minute must be greater than 0 when rate limiting is enabled".to_string()));
            }
            
            if self.rate_limit.window_seconds == 0 {
                return Err(ConfigError::ValidationError("Rate limit window must be greater than 0".to_string()));
            }
        }
        
        // Validate TLS config if present
        if let Some(tls) = &self.tls {
            if tls.cert_path.is_empty() {
                return Err(ConfigError::ValidationError("TLS certificate path cannot be empty".to_string()));
            }
            
            if tls.key_path.is_empty() {
                return Err(ConfigError::ValidationError("TLS private key path cannot be empty".to_string()));
            }
        }
        
        // Validate CORS origins format
        for origin in &self.cors_origins {
            if origin.is_empty() {
                return Err(ConfigError::ValidationError("CORS origin cannot be empty".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Get CORS origins or default to allow all in development
    pub fn effective_cors_origins(&self) -> Vec<String> {
        if self.cors_origins.is_empty() {
            vec!["*".to_string()]
        } else {
            self.cors_origins.clone()
        }
    }
    
    /// Get effective worker count (auto-detect if 0)
    pub fn effective_workers(&self) -> usize {
        if self.workers == 0 {
            num_cpus::get()
        } else {
            self.workers
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            max_body_size: default_max_body_size(),
            request_timeout: default_request_timeout(),
            cors_enabled: default_true(),
            cors_origins: vec![],
            rate_limit: RateLimitConfig::default(),
            tls: None,
            workers: 0,
            keepalive: default_keepalive(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            requests_per_minute: default_rpm(),
            burst: default_burst(),
            window_seconds: default_window(),
        }
    }
}

// Default value functions
fn default_host() -> String { "127.0.0.1".to_string() }
fn default_port() -> u16 { 3000 }
fn default_max_body_size() -> usize { 16 * 1024 * 1024 } // 16MB
fn default_request_timeout() -> u64 { 30 }
fn default_keepalive() -> u64 { 60 }
fn default_true() -> bool { true }
fn default_rpm() -> u32 { 100 }
fn default_burst() -> u32 { 20 }
fn default_window() -> u64 { 60 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.is_cors_enabled());
        assert!(config.is_rate_limit_enabled());
        assert!(!config.is_tls_enabled());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_bind_address() {
        let config = ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            ..Default::default()
        };
        
        assert_eq!(config.bind_address(), "0.0.0.0:8080");
    }

    #[test]
    fn test_effective_cors_origins_default() {
        let config = ServerConfig::default();
        assert_eq!(config.effective_cors_origins(), vec!["*"]);
    }

    #[test]
    fn test_effective_cors_origins_custom() {
        let config = ServerConfig {
            cors_origins: vec!["http://localhost:3000".to_string(), "https://example.com".to_string()],
            ..Default::default()
        };
        
        assert_eq!(config.effective_cors_origins().len(), 2);
        assert!(config.effective_cors_origins().contains(&"http://localhost:3000".to_string()));
    }

    #[test]
    fn test_invalid_port() {
        let config = ServerConfig {
            port: 0,
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_empty_host() {
        let config = ServerConfig {
            host: "".to_string(),
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_tls_config_validation() {
        let config = ServerConfig {
            tls: Some(TlsConfig {
                cert_path: "".to_string(),
                key_path: "/path/to/key.pem".to_string(),
                protocols: vec![],
                cipher_suites: vec![],
            }),
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }
}
