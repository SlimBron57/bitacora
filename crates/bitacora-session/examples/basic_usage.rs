//! Example usage of the Bitacora Session Management Service

use bitacora_session::{SessionService, SessionServiceImpl, SessionConfig};
use std::path::PathBuf;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Bitacora Session Management Service Example");
    println!("===============================================\n");

    // Configure the session service
    let config = SessionConfig {
        storage_path: PathBuf::from("./example_sessions"),
        max_active_sessions: 2,
        auto_persist: true,
        session_timeout_minutes: 60, // 1 hour
    };

    // Create the service
    let service = SessionServiceImpl::new(config).await?;
    println!("✅ Session service initialized with config");

    // Example 1: Basic session lifecycle
    println!("\n📋 Example 1: Basic Session Lifecycle");
    println!("--------------------------------------");
    
    let session_id = service.create_session("Feature Development Session", None).await?;
    println!("🆕 Created session: {}", session_id);

    service.start_session(&session_id, Some("Starting feature work".to_string())).await?;
    println!("▶️  Started session");

    let session = service.get_session(&session_id).await?;
    println!("📊 Session status: {:?}", session.status);

    service.pause_session(&session_id).await?;
    println!("⏸️  Paused session");

    service.resume_session(&session_id).await?;
    println!("▶️  Resumed session");

    service.end_session(&session_id, Some("Feature implementation completed".to_string())).await?;
    println!("🏁 Ended session");

    // Example 2: Multiple sessions and limits
    println!("\n📋 Example 2: Multiple Sessions and Limits");
    println!("-------------------------------------------");

    let session1 = service.create_session("Bug Fix Session", None).await?;
    let session2 = service.create_session("Code Review Session", None).await?;
    let session3 = service.create_session("Refactoring Session", None).await?;

    println!("🆕 Created 3 sessions");

    service.start_session(&session1, None).await?;
    service.start_session(&session2, None).await?;
    println!("▶️  Started 2 sessions (at limit)");

    // This should fail due to max active sessions limit
    match service.start_session(&session3, None).await {
        Ok(()) => println!("⚠️  Unexpected: Third session started"),
        Err(e) => println!("✅ Expected error: {}", e),
    }

    // Example 3: Session queries and metrics
    println!("\n📋 Example 3: Session Queries and Metrics");
    println!("------------------------------------------");

    let active_sessions = service.list_active_sessions().await?;
    println!("🔍 Active sessions: {}", active_sessions.len());

    let recent_sessions = service.list_recent_sessions(Some(5)).await?;
    println!("📋 Recent sessions: {}", recent_sessions.len());

    let metrics = service.get_session_metrics().await?;
    println!("📊 Session Metrics:");
    println!("   - Total sessions: {}", metrics.total_sessions);
    println!("   - Active sessions: {}", metrics.active_sessions);
    println!("   - Completed sessions: {}", metrics.completed_sessions);
    println!("   - Average duration: {:.1} minutes", metrics.average_duration_minutes);

    // Example 4: Error handling
    println!("\n📋 Example 4: Error Handling");
    println!("-----------------------------");

    let nonexistent_id = Uuid::new_v4();
    match service.get_session(&nonexistent_id).await {
        Ok(_) => println!("⚠️  Unexpected: Found nonexistent session"),
        Err(e) => println!("✅ Expected error: {}", e),
    }

    // Clean up
    service.end_session(&session1, None).await?;
    service.end_session(&session2, None).await?;
    println!("\n🧹 Cleaned up active sessions");

    println!("\n🎉 Session Management Service example completed successfully!");
    Ok(())
}
