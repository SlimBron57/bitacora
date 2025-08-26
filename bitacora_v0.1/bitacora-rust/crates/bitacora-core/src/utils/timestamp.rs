//! Timestamp utilities
//! 
//! Centralized timestamp generation and formatting functions.
//! Replaces the need for a separate timestamp daemon/crate.

use chrono::{DateTime, Utc, Local};
use serde::{Serialize, Deserialize};
use std::fmt;

/// Bitacora timestamp format patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimestampFormat {
    /// Default bitacora format: YYYYMMDD-HHMM
    Bitacora,
    /// Date only: YYYYMMDD
    DateOnly,
    /// Time only: HHMM
    TimeOnly,
    /// Full datetime: YYYYMMDD-HHMMSS
    Full,
    /// ISO 8601 format
    Iso8601,
    /// RFC 3339 format
    Rfc3339,
    /// Custom format string
    Custom(String),
}

impl Default for TimestampFormat {
    fn default() -> Self {
        TimestampFormat::Bitacora
    }
}

impl fmt::Display for TimestampFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimestampFormat::Bitacora => write!(f, "%Y%m%d-%H%M"),
            TimestampFormat::DateOnly => write!(f, "%Y%m%d"),
            TimestampFormat::TimeOnly => write!(f, "%H%M"),
            TimestampFormat::Full => write!(f, "%Y%m%d-%H%M%S"),
            TimestampFormat::Iso8601 => write!(f, "%Y-%m-%dT%H:%M:%SZ"),
            TimestampFormat::Rfc3339 => write!(f, "%Y-%m-%d %H:%M:%S UTC"),
            TimestampFormat::Custom(format) => write!(f, "{}", format),
        }
    }
}

/// Generate current timestamp in bitacora default format
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::now_bitacora;
/// 
/// let timestamp = now_bitacora();
/// // Returns something like "20250822-1430"
/// ```
pub fn now_bitacora() -> String {
    Utc::now().format("%Y%m%d-%H%M").to_string()
}

/// Generate current timestamp with specified format
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::{now_formatted, TimestampFormat};
/// 
/// let timestamp = now_formatted(TimestampFormat::Full);
/// // Returns something like "20250822-143045"
/// ```
pub fn now_formatted(format: TimestampFormat) -> String {
    let now = Utc::now();
    match format {
        TimestampFormat::Iso8601 => now.to_rfc3339(),
        TimestampFormat::Rfc3339 => now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        _ => now.format(&format.to_string()).to_string(),
    }
}

/// Generate timestamp for specific DateTime
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::{format_datetime, TimestampFormat};
/// use chrono::Utc;
/// 
/// let dt = Utc::now();
/// let timestamp = format_datetime(&dt, TimestampFormat::Bitacora);
/// ```
pub fn format_datetime(dt: &DateTime<Utc>, format: TimestampFormat) -> String {
    match format {
        TimestampFormat::Iso8601 => dt.to_rfc3339(),
        TimestampFormat::Rfc3339 => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        _ => dt.format(&format.to_string()).to_string(),
    }
}

/// Generate local timestamp
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::now_local_bitacora;
/// 
/// let timestamp = now_local_bitacora();
/// // Returns local time in bitacora format
/// ```
pub fn now_local_bitacora() -> String {
    Local::now().format("%Y%m%d-%H%M").to_string()
}

/// Parse bitacora timestamp string back to DateTime
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::parse_bitacora_timestamp;
/// 
/// let dt = parse_bitacora_timestamp("20250822-1430").unwrap();
/// ```
pub fn parse_bitacora_timestamp(timestamp: &str) -> Result<DateTime<Utc>, crate::errors::BitacoraError> {
    use chrono::NaiveDateTime;
    
    let naive_dt = NaiveDateTime::parse_from_str(timestamp, "%Y%m%d-%H%M")
        .map_err(|e| crate::errors::BitacoraError::Validation(format!("Invalid timestamp format: {}", e)))?;
    
    Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc))
}

/// Validate if string is a valid bitacora timestamp
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::is_valid_bitacora_timestamp;
/// 
/// assert!(is_valid_bitacora_timestamp("20250822-1430"));
/// assert!(!is_valid_bitacora_timestamp("invalid"));
/// ```
pub fn is_valid_bitacora_timestamp(timestamp: &str) -> bool {
    parse_bitacora_timestamp(timestamp).is_ok()
}

/// Generate session timestamp (includes seconds for uniqueness)
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::now_session;
/// 
/// let session_timestamp = now_session();
/// // Returns something like "20250822-143045"
/// ```
pub fn now_session() -> String {
    Utc::now().format("%Y%m%d-%H%M%S").to_string()
}

/// Calculate age of timestamp in seconds
/// 
/// # Examples
/// ```
/// use bitacora_core::utils::timestamp::{timestamp_age_seconds, now_bitacora};
/// 
/// let timestamp = now_bitacora();
/// let age = timestamp_age_seconds(&timestamp).unwrap();
/// // Returns age in seconds (should be close to 0 for current timestamp)
/// ```
pub fn timestamp_age_seconds(timestamp: &str) -> Result<i64, crate::errors::BitacoraError> {
    let ts_dt = parse_bitacora_timestamp(timestamp)?;
    let now = Utc::now();
    
    Ok((now - ts_dt).num_seconds())
}

/// Get yesterday's timestamp in bitacora format
pub fn yesterday_bitacora() -> String {
    let yesterday = Utc::now() - chrono::Duration::days(1);
    yesterday.format("%Y%m%d-%H%M").to_string()
}

/// Get tomorrow's timestamp in bitacora format
pub fn tomorrow_bitacora() -> String {
    let tomorrow = Utc::now() + chrono::Duration::days(1);
    tomorrow.format("%Y%m%d-%H%M").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_now_bitacora() {
        let timestamp = now_bitacora();
        assert_eq!(timestamp.len(), 13); // YYYYMMDD-HHMM
        assert!(timestamp.contains('-'));
    }
    
    #[test]
    fn test_parse_bitacora_timestamp() {
        let timestamp = "20250822-1430";
        let dt = parse_bitacora_timestamp(timestamp).unwrap();
        
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 8);
        assert_eq!(dt.day(), 22);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
    }
    
    #[test]
    fn test_is_valid_bitacora_timestamp() {
        assert!(is_valid_bitacora_timestamp("20250822-1430"));
        assert!(!is_valid_bitacora_timestamp("invalid"));
        assert!(!is_valid_bitacora_timestamp("2025-08-22"));
    }
    
    #[test]
    fn test_format_datetime() {
        let dt = Utc::now();
        
        let bitacora = format_datetime(&dt, TimestampFormat::Bitacora);
        assert_eq!(bitacora.len(), 13);
        
        let date_only = format_datetime(&dt, TimestampFormat::DateOnly);
        assert_eq!(date_only.len(), 8);
    }
}
