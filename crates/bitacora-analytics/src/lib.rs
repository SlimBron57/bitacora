//! # Bitacora Analytics
//! 
//! Intelligent analysis system that combines internal data (sparks, actions, topics)
//! with external documentation sources to provide contextual insights and recommendations.

pub mod analyzers;
pub mod connectors;
pub mod engines;
pub mod services;
pub mod errors;

// Re-exports for easy access
pub use services::{AnalyticsService, AnalyticsServiceImpl};
pub use errors::AnalyticsError;
pub use connectors::{ConnectorManager, ExternalConnector};
