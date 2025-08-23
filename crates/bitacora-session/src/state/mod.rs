// Simple state management for now - full implementation later
mod simple_state;
pub use simple_state::{SessionState, SessionStateManager};

/// Different phases a session can be in
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionPhase {
    Initialization,
    Development,
    Testing,
    Documentation,
    Cleanup,
    Finalization,
}

/// Progress markers for tracking session advancement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMarker {
    pub name: String,
    pub completed: bool,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Metadata about the session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMetadata {
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub version: u32,
    pub checksum: String,
}

/// Information about state persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceInfo {
    pub last_saved: DateTime<Utc>,
    pub save_count: u32,
    pub backup_available: bool,
    pub format_version: String,
}

impl SessionState {
    pub fn new(session_id: &SessionId) -> Self {
        let now = Utc::now();
        
        Self {
            session_id: session_id.clone(),
            current_phase: SessionPhase::Initialization,
            checkpoint_data: HashMap::new(),
            progress_markers: vec![],
            state_metadata: StateMetadata {
                created_at: now,
                last_updated: now,
                version: 1,
                checksum: "initial".to_string(),
            },
            persistence_info: PersistenceInfo {
                last_saved: now,
                save_count: 0,
                backup_available: false,
                format_version: "1.0".to_string(),
            },
        }
    }

    pub fn add_progress_marker(&mut self, name: String, notes: Option<String>) {
        self.progress_markers.push(ProgressMarker {
            name,
            completed: true,
            timestamp: Utc::now(),
            notes,
        });
        self.update_metadata();
    }

    pub fn set_checkpoint(&mut self, key: String, value: serde_json::Value) {
        self.checkpoint_data.insert(key, value);
        self.update_metadata();
    }

    pub fn get_checkpoint(&self, key: &str) -> Option<&serde_json::Value> {
        self.checkpoint_data.get(key)
    }

    pub fn advance_phase(&mut self, new_phase: SessionPhase) {
        self.current_phase = new_phase;
        self.update_metadata();
    }

    fn update_metadata(&mut self) {
        self.state_metadata.last_updated = Utc::now();
        self.state_metadata.version += 1;
        // In a real implementation, calculate actual checksum
        self.state_metadata.checksum = format!("v{}", self.state_metadata.version);
    }
}

/// Manager for session state persistence and operations
pub struct SessionStateManager {
    config: StatePersistenceConfig,
}

impl SessionStateManager {
    pub async fn new(config: StatePersistenceConfig) -> Result<Self, SessionError> {
        // Ensure state directory exists
        if let Some(parent) = config.state_dir.parent() {
            fs::create_dir_all(parent).await
                .map_err(|e| SessionError::state_persistence_error(format!("Failed to create state directory: {}", e)))?;
        }

        fs::create_dir_all(&config.state_dir).await
            .map_err(|e| SessionError::state_persistence_error(format!("Failed to create state directory: {}", e)))?;

        Ok(Self { config })
    }

    /// Save session state to persistent storage
    pub async fn save_session_state(
        &self,
        session_id: &SessionId,
        state: &SessionState,
    ) -> Result<(), SessionError> {
        let state_file = self.get_state_file_path(session_id);
        
        // Create backup if enabled
        if self.config.auto_backup && state_file.exists() {
            self.create_backup(session_id).await?;
        }

        // Serialize state
        let serialized = match self.config.format {
            crate::config::StateFormat::Json => {
                serde_json::to_string_pretty(state)
                    .map_err(|e| SessionError::Serialization { source: e })?
            }
            crate::config::StateFormat::Binary => {
                return Err(SessionError::configuration_error("Binary format not yet implemented"));
            }
            crate::config::StateFormat::Yaml => {
                return Err(SessionError::configuration_error("YAML format not yet implemented"));
            }
        };

        // Write to file
        fs::write(&state_file, serialized).await
            .map_err(|e| SessionError::state_persistence_error(format!("Failed to write state file: {}", e)))?;

        Ok(())
    }

    /// Load session state from persistent storage
    pub async fn load_session_state(&self, session_id: &SessionId) -> Result<SessionState, SessionError> {
        let state_file = self.get_state_file_path(session_id);
        
        if !state_file.exists() {
            return Err(SessionError::session_not_found(session_id.to_string()));
        }

        // Read file contents
        let contents = fs::read_to_string(&state_file).await
            .map_err(|e| SessionError::state_persistence_error(format!("Failed to read state file: {}", e)))?;

        // Deserialize based on format
        let state = match self.config.format {
            crate::config::StateFormat::Json => {
                serde_json::from_str(&contents)
                    .map_err(|e| SessionError::Serialization { source: e })?
            }
            crate::config::StateFormat::Binary => {
                return Err(SessionError::configuration_error("Binary format not yet implemented"));
            }
            crate::config::StateFormat::Yaml => {
                return Err(SessionError::configuration_error("YAML format not yet implemented"));
            }
        };

        Ok(state)
    }

    /// Find sessions that were interrupted (not properly closed)
    pub async fn find_interrupted_sessions(&self) -> Result<Vec<SessionId>, SessionError> {
        let mut interrupted_sessions = Vec::new();

        let mut entries = fs::read_dir(&self.config.state_dir).await
            .map_err(|e| SessionError::Io { source: e })?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| SessionError::Io { source: e })? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".state.json") {
                    let session_id_str = file_name.replace(".state.json", "");
                    let session_id = SessionId::from(session_id_str);
                    
                    // Check if session was properly closed
                    if let Ok(state) = self.load_session_state(&session_id).await {
                        // Check if session indicates it was interrupted
                        // This is a simplified check - in reality, you'd have more sophisticated logic
                        if matches!(state.current_phase, SessionPhase::Development | SessionPhase::Testing) {
                            interrupted_sessions.push(session_id);
                        }
                    }
                }
            }
        }

        Ok(interrupted_sessions)
    }

    /// Validate session state consistency
    pub async fn validate_session_state(&self, session_id: &SessionId) -> Result<bool, SessionError> {
        let state_file = self.get_state_file_path(session_id);
        
        if !state_file.exists() {
            return Ok(false);
        }

        // Try to load and parse the state
        match self.load_session_state(session_id).await {
            Ok(_state) => {
                // Additional validation logic could go here
                // For now, just check if it can be loaded
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    /// Clean up orphaned state files
    pub async fn cleanup_orphaned_states(&self) -> Result<u32, SessionError> {
        let mut cleanup_count = 0;

        let mut entries = fs::read_dir(&self.config.state_dir).await
            .map_err(|e| SessionError::Io { source: e })?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| SessionError::Io { source: e })? {
            
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".state.json") {
                    let session_id_str = file_name.replace(".state.json", "");
                    let session_id = SessionId::from(session_id_str);
                    
                    // Check if state is valid
                    if !self.validate_session_state(&session_id).await? {
                        // Remove invalid state file
                        if let Err(e) = fs::remove_file(entry.path()).await {
                            tracing::warn!("Failed to remove orphaned state file: {}", e);
                        } else {
                            cleanup_count += 1;
                        }
                    }
                }
            }
        }

        Ok(cleanup_count)
    }

    /// Create a backup of the current state file
    async fn create_backup(&self, session_id: &SessionId) -> Result<(), SessionError> {
        let state_file = self.get_state_file_path(session_id);
        let backup_file = self.get_backup_file_path(session_id);

        if state_file.exists() {
            fs::copy(&state_file, &backup_file).await
                .map_err(|e| SessionError::state_persistence_error(format!("Failed to create backup: {}", e)))?;
        }

        Ok(())
    }

    /// Get the file path for a session's state
    fn get_state_file_path(&self, session_id: &SessionId) -> PathBuf {
        self.config.state_dir.join(format!("{}.state.json", session_id))
    }

    /// Get the backup file path for a session's state
    fn get_backup_file_path(&self, session_id: &SessionId) -> PathBuf {
        self.config.state_dir.join(format!("{}.state.backup.json", session_id))
    }
}
