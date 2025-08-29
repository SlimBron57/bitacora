//! Errores del Sistema de Navegación Híbrido

use thiserror::Error;

/// Errores específicos del navegador híbrido
#[derive(Error, Debug)]
pub enum NavigatorError {
    #[error("Error de inicialización del navegador: {0}")]
    InitializationError(String),

    #[error("Error de configuración: {0}")]
    ConfigurationError(String),

    #[error("Error de threading: {0}")]
    ThreadingError(String),

    #[error("Error de safety controller: {0}")]
    SafetyError(String),

    #[error("Error del motor AI: {0}")]
    AIEngineError(String),

    #[error("Error de navegación: {0}")]
    NavigationError(String),

    #[error("Error de índice: {0}")]
    IndexError(String),

    #[error("Error de query: {0}")]
    QueryError(String),

    #[error("Conflicto de recursos detectado: {0}")]
    ResourceConflict(String),

    #[error("Timeout en operación: {0}ms")]
    OperationTimeout(u64),

    #[error("Límite de concurrencia excedido: {current}/{max}")]
    ConcurrencyLimitExceeded { current: usize, max: usize },

    #[error("Modo de navegador no soportado: {0}")]
    UnsupportedMode(String),

    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Error de IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Error de sistema: {0}")]
    SystemError(String),
}

/// Tipo de resultado estándar para el navegador
pub type Result<T> = std::result::Result<T, NavigatorError>;

impl NavigatorError {
    /// Crear error de inicialización
    pub fn initialization<S: Into<String>>(msg: S) -> Self {
        NavigatorError::InitializationError(msg.into())
    }

    /// Crear error de configuración
    pub fn configuration<S: Into<String>>(msg: S) -> Self {
        NavigatorError::ConfigurationError(msg.into())
    }

    /// Crear error de threading
    pub fn threading<S: Into<String>>(msg: S) -> Self {
        NavigatorError::ThreadingError(msg.into())
    }

    /// Crear error de safety
    pub fn safety<S: Into<String>>(msg: S) -> Self {
        NavigatorError::SafetyError(msg.into())
    }

    /// Crear error de motor AI
    pub fn ai_engine<S: Into<String>>(msg: S) -> Self {
        NavigatorError::AIEngineError(msg.into())
    }

    /// Crear error de navegación
    pub fn navigation<S: Into<String>>(msg: S) -> Self {
        NavigatorError::NavigationError(msg.into())
    }

    /// Crear error de índice
    pub fn index<S: Into<String>>(msg: S) -> Self {
        NavigatorError::IndexError(msg.into())
    }

    /// Crear error de query
    pub fn query<S: Into<String>>(msg: S) -> Self {
        NavigatorError::QueryError(msg.into())
    }

    /// Crear error de conflicto de recursos
    pub fn resource_conflict<S: Into<String>>(msg: S) -> Self {
        NavigatorError::ResourceConflict(msg.into())
    }

    /// Crear error de timeout
    pub fn timeout(ms: u64) -> Self {
        NavigatorError::OperationTimeout(ms)
    }

    /// Crear error de límite de concurrencia
    pub fn concurrency_limit(current: usize, max: usize) -> Self {
        NavigatorError::ConcurrencyLimitExceeded { current, max }
    }

    /// Crear error de modo no soportado
    pub fn unsupported_mode<S: Into<String>>(mode: S) -> Self {
        NavigatorError::UnsupportedMode(mode.into())
    }

    /// Crear error de validación
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        NavigatorError::ValidationError(msg.into())
    }

    /// Crear error de sistema
    pub fn system<S: Into<String>>(msg: S) -> Self {
        NavigatorError::SystemError(msg.into())
    }

    /// Verificar si es un error recuperable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            NavigatorError::NavigationError(_)
                | NavigatorError::QueryError(_)
                | NavigatorError::OperationTimeout(_)
                | NavigatorError::ConcurrencyLimitExceeded { .. }
        )
    }

    /// Obtener código de error numérico
    pub fn error_code(&self) -> u32 {
        match self {
            NavigatorError::InitializationError(_) => 1001,
            NavigatorError::ConfigurationError(_) => 1002,
            NavigatorError::ThreadingError(_) => 1003,
            NavigatorError::SafetyError(_) => 1004,
            NavigatorError::AIEngineError(_) => 1005,
            NavigatorError::NavigationError(_) => 1006,
            NavigatorError::IndexError(_) => 1007,
            NavigatorError::QueryError(_) => 1008,
            NavigatorError::ResourceConflict(_) => 1009,
            NavigatorError::OperationTimeout(_) => 1010,
            NavigatorError::ConcurrencyLimitExceeded { .. } => 1011,
            NavigatorError::UnsupportedMode(_) => 1012,
            NavigatorError::ValidationError(_) => 1013,
            NavigatorError::SerializationError(_) => 1014,
            NavigatorError::IoError(_) => 1015,
            NavigatorError::SystemError(_) => 1099,
        }
    }
}
