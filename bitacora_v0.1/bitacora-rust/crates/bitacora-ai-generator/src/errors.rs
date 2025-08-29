//! # AI Generator Error Types 🚨
//!
//! Definición de errores específicos del generador de templates con IA

use std::fmt;

/// Tipo de resultado específico para el AI Generator
pub type Result<T> = std::result::Result<T, AIGeneratorError>;

/// Errores específicos del AI Template Generator
#[derive(Debug, Clone)]
pub enum AIGeneratorError {
    /// Error en la configuración del provider
    ConfigurationError(String),
    
    /// Provider no disponible o no configurado
    ProviderNotAvailable,
    
    /// Error durante la generación
    GenerationFailed(String),
    
    /// Error de autenticación con el provider
    AuthenticationError(String),
    
    /// Rate limit excedido
    RateLimitExceeded(String),
    
    /// Respuesta del provider inválida
    InvalidResponse(String),
    
    /// Template generado inválido
    InvalidTemplate(String),
    
    /// Error de análisis de templates existentes
    AnalysisError(String),
    
    /// Error de red/conectividad
    NetworkError(String),
    
    /// Error de timeout
    TimeoutError(String),
    
    /// Error de formato de entrada
    InputFormatError(String),
    
    /// Error interno del generador
    InternalError(String),
}

impl fmt::Display for AIGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AIGeneratorError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            },
            AIGeneratorError::ProviderNotAvailable => {
                write!(f, "AI provider is not available or not configured")
            },
            AIGeneratorError::GenerationFailed(msg) => {
                write!(f, "Template generation failed: {}", msg)
            },
            AIGeneratorError::AuthenticationError(msg) => {
                write!(f, "Authentication error: {}", msg)
            },
            AIGeneratorError::RateLimitExceeded(msg) => {
                write!(f, "Rate limit exceeded: {}", msg)
            },
            AIGeneratorError::InvalidResponse(msg) => {
                write!(f, "Invalid response from provider: {}", msg)
            },
            AIGeneratorError::InvalidTemplate(msg) => {
                write!(f, "Generated template is invalid: {}", msg)
            },
            AIGeneratorError::AnalysisError(msg) => {
                write!(f, "Template analysis error: {}", msg)
            },
            AIGeneratorError::NetworkError(msg) => {
                write!(f, "Network error: {}", msg)
            },
            AIGeneratorError::TimeoutError(msg) => {
                write!(f, "Operation timed out: {}", msg)
            },
            AIGeneratorError::InputFormatError(msg) => {
                write!(f, "Input format error: {}", msg)
            },
            AIGeneratorError::InternalError(msg) => {
                write!(f, "Internal error: {}", msg)
            },
        }
    }
}

impl std::error::Error for AIGeneratorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// Conversions from common error types

impl From<serde_json::Error> for AIGeneratorError {
    fn from(error: serde_json::Error) -> Self {
        AIGeneratorError::InvalidResponse(format!("JSON parse error: {}", error))
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for AIGeneratorError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            AIGeneratorError::TimeoutError(error.to_string())
        } else if error.is_connect() {
            AIGeneratorError::NetworkError(format!("Connection error: {}", error))
        } else {
            AIGeneratorError::NetworkError(error.to_string())
        }
    }
}

impl From<tokio::time::error::Elapsed> for AIGeneratorError {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        AIGeneratorError::TimeoutError(error.to_string())
    }
}

#[cfg(feature = "openai")]
impl From<openai_api_rs::v1::error::APIError> for AIGeneratorError {
    fn from(error: openai_api_rs::v1::error::APIError) -> Self {
        match error {
            openai_api_rs::v1::error::APIError::InvalidAuthentication => {
                AIGeneratorError::AuthenticationError("Invalid OpenAI API key".to_string())
            },
            openai_api_rs::v1::error::APIError::RateLimitExceeded => {
                AIGeneratorError::RateLimitExceeded("OpenAI rate limit exceeded".to_string())
            },
            _ => AIGeneratorError::GenerationFailed(format!("OpenAI API error: {}", error))
        }
    }
}

/// Extensiones de resultado para mejor UX
pub trait AIGeneratorResultExt<T> {
    /// Convertir error a mensaje de usuario amigable
    fn user_friendly(self) -> std::result::Result<T, String>;
    
    /// Agregar contexto al error
    fn with_context(self, context: &str) -> Result<T>;
    
    /// Mapear error a tipo específico
    fn map_ai_error<F>(self, f: F) -> Result<T>
    where
        F: FnOnce(AIGeneratorError) -> AIGeneratorError;
}

impl<T> AIGeneratorResultExt<T> for Result<T> {
    fn user_friendly(self) -> std::result::Result<T, String> {
        self.map_err(|e| match e {
            AIGeneratorError::ConfigurationError(msg) => {
                format!("Configuration issue: {}", msg)
            },
            AIGeneratorError::ProviderNotAvailable => {
                "AI service is not available. Please check configuration.".to_string()
            },
            AIGeneratorError::GenerationFailed(msg) => {
                format!("Failed to generate template: {}", msg)
            },
            AIGeneratorError::AuthenticationError(_) => {
                "Authentication failed. Please check your API credentials.".to_string()
            },
            AIGeneratorError::RateLimitExceeded(_) => {
                "Request limit exceeded. Please try again later.".to_string()
            },
            AIGeneratorError::NetworkError(_) => {
                "Network connection issue. Please check your internet connection.".to_string()
            },
            AIGeneratorError::TimeoutError(_) => {
                "Request timed out. Please try again.".to_string()
            },
            _ => "An unexpected error occurred".to_string(),
        })
    }
    
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|e| match e {
            AIGeneratorError::InternalError(msg) => {
                AIGeneratorError::InternalError(format!("{}: {}", context, msg))
            },
            _ => AIGeneratorError::InternalError(format!("{}: {}", context, e))
        })
    }
    
    fn map_ai_error<F>(self, f: F) -> Result<T>
    where
        F: FnOnce(AIGeneratorError) -> AIGeneratorError,
    {
        self.map_err(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = AIGeneratorError::GenerationFailed("Test error".to_string());
        assert_eq!(error.to_string(), "Template generation failed: Test error");
    }
    
    #[test]
    fn test_user_friendly_error() {
        let result: Result<()> = Err(AIGeneratorError::ProviderNotAvailable);
        let friendly = result.user_friendly();
        assert!(friendly.is_err());
        assert_eq!(
            friendly.unwrap_err(),
            "AI service is not available. Please check configuration."
        );
    }
    
    #[test]
    fn test_error_context() {
        let result: Result<()> = Err(AIGeneratorError::GenerationFailed("Original".to_string()));
        let with_context = result.with_context("While processing template");
        assert!(with_context.is_err());
        assert!(with_context.unwrap_err().to_string().contains("While processing template"));
    }
}
