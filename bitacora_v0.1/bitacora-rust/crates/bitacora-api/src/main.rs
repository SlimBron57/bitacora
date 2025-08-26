//! Bitacora API Server Binary
//!
//! Main entry point for the Bitacora REST API server.

use bitacora_api::ApiServer;
use std::net::SocketAddr;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::init();

    info!("🚀 Starting Bitacora API Server...");

    // Parse address from env or use default
    let addr = std::env::var("BITACORA_API_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string())
        .parse::<SocketAddr>()
        .expect("Valid socket address");

    info!("📍 Server will bind to: {}", addr);

    // Create and start server
    let server = ApiServer::new(addr);
    
    match server.serve().await {
        Ok(_) => info!("✅ Server stopped gracefully"),
        Err(e) => {
            error!("❌ Server error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
