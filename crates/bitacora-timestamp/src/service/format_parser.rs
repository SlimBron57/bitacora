//! Format Parser - Time format handling

use chrono::{DateTime, Utc, NaiveDateTime};
use std::collections::HashMap;
use tracing::{debug, error};

use bitacora_core::errors::BitacoraError;
use crate::TimestampResult;

/// Format parser for timestamp formatting
pub struct FormatParser {
    /// Cache for compiled format strings
    format_cache: std::sync::Arc<std::sync::Mutex<HashMap<String, String>>>,
}

impl FormatParser {
    /// Create new format parser
    pub fn new() -> Self {
        Self {
            format_cache: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
    
    /// Format timestamp using specified format
    pub async fn format_timestamp(
        &self,
        datetime: &DateTime<Utc>,
        format: &str,
    ) -> TimestampResult<String> {
        debug!("Formatting timestamp with format: {}", format);
        
        // Check cache first
        if let Ok(cache) = self.format_cache.lock() {
            if let Some(cached_format) = cache.get(format) {
                return Ok(datetime.format(cached_format).to_string());
            }
        }
        
        // Parse and validate format
        let parsed_format = self.parse_format_string(format)?;
        
        // Cache the parsed format
        if let Ok(mut cache) = self.format_cache.lock() {
            cache.insert(format.to_string(), parsed_format.clone());
        }
        
        Ok(datetime.format(&parsed_format).to_string())
    }
    
    /// Parse custom format string to chrono format
    fn parse_format_string(&self, format: &str) -> TimestampResult<String> {
        let mut result = format.to_string();
        
        // Common bitacora timestamp patterns
        let replacements = vec![
            // Date patterns
            ("YYYY", "%Y"),
            ("MM", "%m"),
            ("DD", "%d"),
            ("YY", "%y"),
            
            // Time patterns
            ("HH", "%H"),
            ("mm", "%M"),
            ("SS", "%S"),
            ("hh", "%I"),
            
            // Special patterns
            ("TIMESTAMP", "%Y%m%d-%H%M"),  // Default bitacora format
            ("DATE", "%Y%m%d"),
            ("TIME", "%H%M"),
            ("DATETIME", "%Y%m%d-%H%M%S"),
            
            // Separators (keep as-is)
            ("-", "-"),
            ("_", "_"),
            (":", ":"),
            (".", "."),
        ];
        
        for (pattern, replacement) in replacements {
            result = result.replace(pattern, replacement);
        }
        
        debug!("Parsed format '{}' to '{}'", format, result);
        Ok(result)
    }
    
    /// Validate timestamp against format
    pub async fn validate_timestamp(
        &self,
        timestamp: &str,
        format: &str,
    ) -> TimestampResult<bool> {
        debug!("Validating timestamp '{}' against format '{}'", timestamp, format);
        
        let parsed_format = self.parse_format_string(format)?;
        
        match NaiveDateTime::parse_from_str(timestamp, &parsed_format) {
            Ok(_) => {
                debug!("Timestamp validation successful");
                Ok(true)
            }
            Err(e) => {
                debug!("Timestamp validation failed: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Get current timestamp in bitacora default format
    pub async fn get_default_timestamp(&self) -> TimestampResult<String> {
        let now = Utc::now();
        self.format_timestamp(&now, "TIMESTAMP").await
    }
    
    /// Parse timestamp string back to DateTime
    pub async fn parse_timestamp(
        &self,
        timestamp: &str,
        format: &str,
    ) -> TimestampResult<DateTime<Utc>> {
        let parsed_format = self.parse_format_string(format)?;
        
        let naive_dt = NaiveDateTime::parse_from_str(timestamp, &parsed_format)
            .map_err(|e| BitacoraError::ValidationError(format!("Failed to parse timestamp: {}", e)))?;
        
        Ok(DateTime::from_naive_utc_and_offset(naive_dt, Utc))
    }
    
    /// Get available format patterns
    pub fn get_available_patterns(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("YYYY", "4-digit year"),
            ("YY", "2-digit year"),
            ("MM", "Month (01-12)"),
            ("DD", "Day (01-31)"),
            ("HH", "Hour (00-23)"),
            ("mm", "Minute (00-59)"),
            ("SS", "Second (00-59)"),
            ("hh", "Hour (01-12)"),
            ("TIMESTAMP", "Default format: YYYYMMDD-HHMM"),
            ("DATE", "Date only: YYYYMMDD"),
            ("TIME", "Time only: HHMM"),
            ("DATETIME", "Full datetime: YYYYMMDD-HHMMSS"),
        ]
    }
    
    /// Validate format string
    pub fn validate_format(&self, format: &str) -> TimestampResult<()> {
        if format.is_empty() {
            return Err(BitacoraError::ValidationError("Format string cannot be empty".to_string()));
        }
        
        // Try to parse the format
        let _parsed = self.parse_format_string(format)?;
        
        // Additional validation could be added here
        Ok(())
    }
    
    /// Clear format cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.format_cache.lock() {
            cache.clear();
        }
    }
}

impl Default for FormatParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    #[tokio::test]
    async fn test_format_timestamp() {
        let parser = FormatParser::new();
        let dt = Utc.ymd_opt(2025, 8, 22).unwrap().and_hms_opt(14, 30, 45).unwrap();
        
        let result = parser.format_timestamp(&dt, "TIMESTAMP").await.unwrap();
        assert_eq!(result, "20250822-1430");
        
        let result = parser.format_timestamp(&dt, "YYYY-MM-DD HH:mm:SS").await.unwrap();
        assert_eq!(result, "2025-08-22 14:30:45");
    }
    
    #[tokio::test]
    async fn test_validate_timestamp() {
        let parser = FormatParser::new();
        
        let valid = parser.validate_timestamp("20250822-1430", "TIMESTAMP").await.unwrap();
        assert!(valid);
        
        let invalid = parser.validate_timestamp("invalid", "TIMESTAMP").await.unwrap();
        assert!(!invalid);
    }
}
