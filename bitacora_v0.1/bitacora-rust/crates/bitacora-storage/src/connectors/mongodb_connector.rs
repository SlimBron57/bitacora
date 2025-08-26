//! MongoDB connector implementation

use async_trait::async_trait;
use mongodb::{Client, Database, options::ClientOptions};
use tracing::{info, error, debug};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DatabaseConnector, StorageConfig, StorageResult, StorageError};

/// MongoDB connector
#[derive(Debug)]
pub struct MongoDbConnector {
    client: Arc<RwLock<Option<Client>>>,
    database: Arc<RwLock<Option<Database>>>,
    database_name: Arc<RwLock<String>>,
}

impl MongoDbConnector {
    /// Create new MongoDB connector
    pub fn new() -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
            database: Arc::new(RwLock::new(None)),
            database_name: Arc::new(RwLock::new(String::new())),
        }
    }

    /// Get database instance
    pub async fn database(&self) -> StorageResult<Database> {
        let db_guard = self.database.read().await;
        match db_guard.as_ref() {
            Some(db) => Ok(db.clone()),
            None => Err(StorageError::ConnectionFailed("Not connected to database".to_string())),
        }
    }

    /// Create indexes for all collections
    async fn create_indexes(&self) -> StorageResult<()> {
        let db = self.database().await?;
        
        // Create indexes for sessions collection
        let sessions = db.collection::<mongodb::bson::Document>("sessions");
        sessions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "session_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build()
                )
                .build(),
            None,
        ).await?;

        sessions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "user_id": 1, "start_time": -1 })
                .build(),
            None,
        ).await?;

        // Create indexes for actions collection
        let actions = db.collection::<mongodb::bson::Document>("actions");
        actions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "action_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build()
                )
                .build(),
            None,
        ).await?;

        actions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "session_id": 1, "timestamp": -1 })
                .build(),
            None,
        ).await?;

        actions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "user_id": 1, "timestamp": -1 })
                .build(),
            None,
        ).await?;

        // Create text index for action descriptions
        actions.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { 
                    "description": "text",
                    "tags": "text"
                })
                .build(),
            None,
        ).await?;

        // Create indexes for projects collection
        let projects = db.collection::<mongodb::bson::Document>("projects");
        projects.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "project_id": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build()
                )
                .build(),
            None,
        ).await?;

        projects.create_index(
            mongodb::IndexModel::builder()
                .keys(mongodb::bson::doc! { "user_id": 1, "status": 1 })
                .build(),
            None,
        ).await?;

        info!("Successfully created database indexes");
        Ok(())
    }
}

impl Default for MongoDbConnector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
#[async_trait]
impl DatabaseConnector for MongoDbConnector {
    async fn connect(&self, config: &StorageConfig) -> StorageResult<()> {
        let connection_string = config.connection_string();
        let database_name = &config.database.mongodb.database;
        
        info!("Connecting to MongoDB: {}", connection_string);
        
        let mut client_options = ClientOptions::parse(&connection_string).await
            .map_err(|e| StorageError::ConnectionFailed(format!("Failed to parse connection string: {}", e)))?;
        
        client_options.max_pool_size = Some(10); // Default pool size
        client_options.connect_timeout = Some(std::time::Duration::from_secs(30));

        let client = Client::with_options(client_options)
            .map_err(|e| StorageError::ConnectionFailed(format!("Failed to create client: {}", e)))?;

        // Test the connection
        client
            .database(database_name)
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await
            .map_err(|e| StorageError::ConnectionFailed(format!("Connection test failed: {}", e)))?;

        let database = client.database(database_name);

        // Store client and database
        {
            let mut client_guard = self.client.write().await;
            *client_guard = Some(client);
        }

        {
            let mut db_guard = self.database.write().await;
            *db_guard = Some(database);
        }

        // Update database name for this instance
        {
            let mut name_guard = self.database_name.write().await;
            *name_guard = database_name.to_string();
        }

        // Create indexes
        if let Err(e) = self.create_indexes().await {
            error!("Failed to create indexes: {}", e);
            // Don't fail connection for index issues
        }

        info!("Successfully connected to MongoDB database: {}", database_name);
        Ok(())
    }

    async fn health_check(&self) -> StorageResult<bool> {
        let db = match self.database().await {
            Ok(db) => db,
            Err(_) => return Ok(false),
        };

        match db.run_command(mongodb::bson::doc! {"ping": 1}, None).await {
            Ok(_) => {
                debug!("MongoDB health check passed");
                Ok(true)
            }
            Err(e) => {
                error!("MongoDB health check failed: {}", e);
                Ok(false)
            }
        }
    }

    async fn database_name(&self) -> String {
        let name_guard = self.database_name.read().await;
        name_guard.clone()
    }

    async fn disconnect(&self) -> StorageResult<()> {
        {
            let mut client_guard = self.client.write().await;
            *client_guard = None;
        }

        {
            let mut db_guard = self.database.write().await;
            *db_guard = None;
        }

        info!("Disconnected from MongoDB");
        Ok(())
    }
}
