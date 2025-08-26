//! Minimal main entry point for testing

use std::net::SocketAddr;
use bitacora_api::server_minimal::serve_minimal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Server address
    let addr: SocketAddr = "0.0.0.0:3000".parse()?;

    // Start server
    serve_minimal(addr).await
}
