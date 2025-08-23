# Bitacora Session Management Service

This crate provides comprehensive session management functionality for the Bitacora development workflow tracking system. It handles the complete lifecycle of development sessions with a focus on simplicity and reliability.

## Features

- **Session Lifecycle Management**: Create, start, pause, resume, and end sessions
- **State Transitions**: Proper validation of session state transitions
- **Resource Management**: Configurable limits for maximum active sessions
- **Metrics and Monitoring**: Built-in metrics collection for session analytics
- **Async-First Design**: Full async/await support with tokio integration

## Architecture

The Session Management Service follows a clean architecture pattern:

```
SessionService (Trait)
    ↓
SessionServiceImpl (Implementation)
    ├── SessionConfig (Configuration)
    ├── SessionError (Error Handling)
    └── In-Memory Storage (HashMap<Uuid, Session>)
```

## Usage

### Basic Session Management

```rust
use bitacora_session::{SessionService, SessionServiceImpl, SessionConfig};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service with default configuration
    let config = SessionConfig::default();
    let service = SessionServiceImpl::new(config).await?;

    // Create a new session
    let session_id = service.create_session("My Development Session", None).await?;
    println!("Created session: {}", session_id);

    // Start the session
    service.start_session(&session_id, None).await?;
    println!("Session started");

    // Pause the session
    service.pause_session(&session_id).await?;
    println!("Session paused");

    // Resume the session
    service.resume_session(&session_id).await?;
    println!("Session resumed");

    // End the session
    service.end_session(&session_id, Some("Completed feature implementation")).await?;
    println!("Session ended");

    Ok(())
}
```

### Configuration

```rust
use bitacora_session::SessionConfig;
use std::path::PathBuf;

let config = SessionConfig {
    storage_path: PathBuf::from("./my_sessions"),
    max_active_sessions: 3,
    auto_persist: true,
    session_timeout_minutes: 120, // 2 hours
};
```

### Session Queries and Metrics

```rust
// Get session details
let session = service.get_session(&session_id).await?;
println!("Session status: {:?}", session.status);

// List active sessions
let active_sessions = service.list_active_sessions().await?;
println!("Active sessions: {}", active_sessions.len());

// Get metrics
let metrics = service.get_session_metrics().await?;
println!("Total sessions: {}", metrics.total_sessions);
println!("Average duration: {:.2} minutes", metrics.average_duration_minutes);
```

## Session States and Transitions

Sessions follow a well-defined state machine:

```
Created (Paused) → Active → Paused → Active → ... → Ended
       ↓                   ↗    ↓      ↗
       └─────── Active ────┘    └──────┘
       └─────────────────────────── Ended
```

### Valid Transitions

- **Paused → Active**: Start or resume a session
- **Active → Paused**: Pause an active session
- **Active → Ended**: End an active session
- **Paused → Ended**: End a paused session

### Invalid Transitions

- **Ended → Any**: Ended sessions cannot be reactivated
- **Active → Active**: Sessions are already active

## Error Handling

The service provides comprehensive error handling through the `SessionError` enum:

```rust
match service.start_session(&session_id, None).await {
    Ok(()) => println!("Session started successfully"),
    Err(SessionError::SessionNotFound(id)) => {
        eprintln!("Session {} not found", id);
    }
    Err(SessionError::ConfigurationError(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Testing

The crate includes comprehensive unit tests covering:

- Session lifecycle operations
- State transition validation
- Resource limit enforcement
- Error conditions
- Metrics calculation

Run tests with:

```bash
cargo test --package bitacora-session
```

## Integration with Bitacora Core

This service integrates seamlessly with the Bitacora core models:

- Uses `bitacora_core::models::Session` for session data
- Compatible with `bitacora_core::models::SessionStatus` enum
- Follows Bitacora's async patterns and error handling

## Performance Characteristics

- **Memory Usage**: O(n) where n is the number of sessions
- **Operation Complexity**: O(1) for most operations, O(n) for listing and metrics
- **Concurrency**: Full async support with tokio RwLock for thread safety
- **Scalability**: Suitable for applications with hundreds of concurrent sessions

## Future Enhancements

The current implementation provides a solid foundation that can be extended with:

- Persistent storage backends (PostgreSQL, SQLite, etc.)
- Session context preservation and restoration
- Integration with Git, Timestamp, and Storage services
- Advanced session analytics and reporting
- Session templates and presets
- Automatic session timeout handling

## Contributing

When contributing to this crate:

1. Ensure all tests pass
2. Add tests for new functionality
3. Follow the existing async patterns
4. Update documentation for public APIs
5. Consider backward compatibility

## License

This crate is part of the Bitacora project and follows the same licensing terms.
