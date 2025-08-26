//! Service traits

use crate::utils::timestamp::TimestampFormat;

// TODO: Implementar en Día 3-5 del checklist
pub trait SessionService {
    // Se implementará en los próximos días
}

/// Centralized timestamp service trait
/// 
/// Provides consistent timestamp generation across the system.
/// Implementations should use the utilities in `crate::utils::timestamp`
pub trait TimestampService {
    /// Generate current timestamp in default bitacora format
    fn now(&self) -> String {
        crate::utils::timestamp::now_bitacora()
    }
    
    /// Generate timestamp with custom format
    fn now_formatted(&self, format: TimestampFormat) -> String {
        crate::utils::timestamp::now_formatted(format)
    }
    
    /// Generate session timestamp (with seconds for uniqueness)
    fn now_session(&self) -> String {
        crate::utils::timestamp::now_session()
    }
    
    /// Validate timestamp format
    fn is_valid(&self, timestamp: &str) -> bool {
        crate::utils::timestamp::is_valid_bitacora_timestamp(timestamp)
    }
}

pub trait GitService {
    // Se implementará en los próximos días
}
