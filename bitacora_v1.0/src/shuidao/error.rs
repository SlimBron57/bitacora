// === SHUIDAO ERROR HANDLING ===
// Tipos de errores para el sistema ShuiDao (水道)
// Implementa: DA-032 (ShuiDao - Intention-Oriented Cognitive Architecture)
// Creado: 2025-11-24 11:42:28
// Autor: Sistema Bitácora v1.0

use std::fmt;

/// Errores del sistema ShuiDao
#[derive(Debug, Clone)]
pub enum ShuiDaoError {
    /// Error al detectar intención del usuario
    IntentionDetectionFailed {
        input: String,
        reason: String,
    },

    /// Error en el enrutamiento cognitivo
    RoutingFailed {
        detected_mode: String,
        reason: String,
    },

    /// Error en motor operacional (proyectos, tareas)
    OperationalEngineError {
        operation: String,
        details: String,
    },

    /// Error en motor procedural (recetas, steps)
    ProceduralEngineError {
        operation: String,
        details: String,
    },

    /// Error en motor de aprendizaje (knowledge paths)
    LearningEngineError {
        operation: String,
        details: String,
    },

    /// Error en motor conversacional (diálogos, narrativa)
    ConversationalEngineError {
        operation: String,
        details: String,
    },

    /// Error en motor ligero (respuestas directas)
    LightEngineError {
        operation: String,
        details: String,
    },

    /// Error en síntesis de respuesta final
    ResponseSynthesisError {
        reason: String,
    },

    /// Error al acceder a memoria (TelescopeDB/VoxelDB)
    MemoryAccessError {
        component: String,
        details: String,
    },

    /// Error de validación de entrada
    ValidationError {
        field: String,
        reason: String,
    },

    /// Error de configuración
    ConfigurationError {
        parameter: String,
        reason: String,
    },

    /// Error de timeout (operación tardó demasiado)
    TimeoutError {
        operation: String,
        elapsed_ms: u64,
        limit_ms: u64,
    },

    /// Recurso no encontrado (recipe, execution, project, etc.)
    NotFound(String),

    /// Estado inválido (operation no permitida en estado actual)
    InvalidState(String),

    /// Input inválido (parámetros incorrectos)
    InvalidInput(String),
    
    /// Topic ya existe (intento de agregar duplicado)
    TopicAlreadyExists(String),
    
    /// Topic no encontrado
    TopicNotFound(String),
    
    /// IceBreaker stage ya completado (AlreadyTransitioned)
    AlreadyTransitioned,
    
    /// Error de I/O (filesystem, network, etc)
    IoError(String),
    
    /// Error de serialización/deserialización
    SerializationError(String),
}

impl fmt::Display for ShuiDaoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShuiDaoError::IntentionDetectionFailed { input, reason } => {
                write!(
                    f,
                    "Failed to detect intention for input: '{}'. Reason: {}",
                    input, reason
                )
            }
            ShuiDaoError::RoutingFailed { detected_mode, reason } => {
                write!(
                    f,
                    "Routing failed for mode '{}'. Reason: {}",
                    detected_mode, reason
                )
            }
            ShuiDaoError::OperationalEngineError { operation, details } => {
                write!(
                    f,
                    "OperationalEngine error during '{}': {}",
                    operation, details
                )
            }
            ShuiDaoError::ProceduralEngineError { operation, details } => {
                write!(
                    f,
                    "ProceduralEngine error during '{}': {}",
                    operation, details
                )
            }
            ShuiDaoError::LearningEngineError { operation, details } => {
                write!(
                    f,
                    "LearningEngine error during '{}': {}",
                    operation, details
                )
            }
            ShuiDaoError::ConversationalEngineError { operation, details } => {
                write!(
                    f,
                    "ConversationalEngine error during '{}': {}",
                    operation, details
                )
            }
            ShuiDaoError::LightEngineError { operation, details } => {
                write!(
                    f,
                    "LightEngine error during '{}': {}",
                    operation, details
                )
            }
            ShuiDaoError::ResponseSynthesisError { reason } => {
                write!(f, "Failed to synthesize response: {}", reason)
            }
            ShuiDaoError::MemoryAccessError { component, details } => {
                write!(f, "Memory access error in {}: {}", component, details)
            }
            ShuiDaoError::ValidationError { field, reason } => {
                write!(f, "Validation failed for '{}': {}", field, reason)
            }
            ShuiDaoError::ConfigurationError { parameter, reason } => {
                write!(f, "Configuration error in '{}': {}", parameter, reason)
            }
            ShuiDaoError::TimeoutError {
                operation,
                elapsed_ms,
                limit_ms,
            } => {
                write!(
                    f,
                    "Timeout in '{}': {}ms elapsed (limit: {}ms)",
                    operation, elapsed_ms, limit_ms
                )
            }
            ShuiDaoError::NotFound(msg) => {
                write!(f, "Not found: {}", msg)
            }
            ShuiDaoError::InvalidState(msg) => {
                write!(f, "Invalid state: {}", msg)
            }
            ShuiDaoError::InvalidInput(msg) => {
                write!(f, "Invalid input: {}", msg)
            }
            ShuiDaoError::TopicAlreadyExists(name) => {
                write!(f, "Topic '{}' already exists", name)
            }
            ShuiDaoError::TopicNotFound(id) => {
                write!(f, "Topic '{}' not found", id)
            }
            ShuiDaoError::AlreadyTransitioned => {
                write!(f, "IceBreaker stage already transitioned (cannot advance further)")
            }
            ShuiDaoError::IoError(msg) => {
                write!(f, "I/O error: {}", msg)
            }
            ShuiDaoError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
        }
    }
}

impl std::error::Error for ShuiDaoError {}

/// Alias de Result para ShuiDao
pub type Result<T> = std::result::Result<T, ShuiDaoError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_formats() {
        let error = ShuiDaoError::IntentionDetectionFailed {
            input: "test input".to_string(),
            reason: "insufficient confidence".to_string(),
        };
        
        let display = format!("{}", error);
        assert!(display.contains("Failed to detect intention"));
        assert!(display.contains("test input"));
    }

    #[test]
    fn test_timeout_error() {
        let error = ShuiDaoError::TimeoutError {
            operation: "intention_detection".to_string(),
            elapsed_ms: 250,
            limit_ms: 200,
        };
        
        let display = format!("{}", error);
        assert!(display.contains("Timeout"));
        assert!(display.contains("250ms"));
    }
}
