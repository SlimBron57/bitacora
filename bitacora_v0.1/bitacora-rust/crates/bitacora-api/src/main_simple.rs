//! Simple main entry point for Bitacora API

use bitacora_api::server_simple;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();
    
    // Server configuration
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;
    
    // Start server
    server_simple::serve(addr).await?;
    
    Ok(())
}
