# Configuración de Base de Datos - Bitacora V1.0

## Variables de Entorno para Docker

### Desarrollo Local
```bash
# .env.development
DATABASE_TYPE=mongodb
MONGODB_URI=mongodb://bitacora_app:app_password_2025@localhost:27017/bitacora_db
MONGODB_DATABASE=bitacora_db

# SQLite Fallback
SQLITE_PATH=./data/bitacora.db

# Configuración de Conexión
DB_MAX_CONNECTIONS=100
DB_CONNECT_TIMEOUT=30
DB_IDLE_TIMEOUT=600
```

### Producción
```bash
# .env.production
DATABASE_TYPE=mongodb
MONGODB_URI=${MONGODB_CONNECTION_STRING}
MONGODB_DATABASE=bitacora_prod

# Configuración adicional
DB_MAX_CONNECTIONS=200
DB_CONNECT_TIMEOUT=10
DB_IDLE_TIMEOUT=300
```

## Archivo de Configuración (config.toml)

### Estructura Flexible
```toml
[database]
# Tipo de base de datos: "mongodb", "sqlite", "postgresql"
type = "mongodb"

# Configuración específica por tipo
[database.mongodb]
uri = "${MONGODB_URI}"
database_name = "${MONGODB_DATABASE}"
max_connections = 100
connect_timeout = 30
idle_timeout = 600

[database.sqlite]
path = "${SQLITE_PATH:-./data/bitacora.db}"
max_connections = 10

[database.postgresql]
host = "${POSTGRES_HOST:-localhost}"
port = "${POSTGRES_PORT:-5432}"
database = "${POSTGRES_DB:-bitacora}"
username = "${POSTGRES_USER:-bitacora}"
password = "${POSTGRES_PASSWORD}"
```

## Docker Compose con Variables

### docker-compose.yml actualizado
```yaml
version: '3.8'

services:
  mongodb:
    image: mongo:7.0
    container_name: bitacora_mongo_dev
    environment:
      MONGO_INITDB_ROOT_USERNAME: ${MONGO_ROOT_USER:-bitacora}
      MONGO_INITDB_ROOT_PASSWORD: ${MONGO_ROOT_PASSWORD:-dev_password_2025}
      MONGO_INITDB_DATABASE: ${MONGO_DATABASE:-bitacora_db}
    ports:
      - "${MONGO_PORT:-27017}:27017"
    volumes:
      - mongodb_data:/data/db

  bitacora-api:
    build: .
    environment:
      DATABASE_TYPE: ${DATABASE_TYPE:-mongodb}
      MONGODB_URI: ${MONGODB_URI:-mongodb://bitacora_app:app_password_2025@mongodb:27017/bitacora_db}
      RUST_LOG: ${RUST_LOG:-info}
    depends_on:
      - mongodb
    ports:
      - "${API_PORT:-8080}:8080"

volumes:
  mongodb_data:
```

## Implementación en Rust

### bitacora-config/src/database.rs
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub mongodb: Option<MongoDBConfig>,
    pub sqlite: Option<SqliteConfig>,
    pub postgresql: Option<PostgreSQLConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseType {
    MongoDB,
    Sqlite,
    PostgreSQL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDBConfig {
    pub uri: String,
    pub database_name: String,
    pub max_connections: u32,
    pub connect_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub path: String,
    pub max_connections: u32,
}
```

### bitacora-storage/src/connectors/connector_manager.rs
```rust
pub struct ConnectorManager {
    active_connector: Box<dyn DatabaseConnector>,
    fallback_connector: Option<Box<dyn DatabaseConnector>>,
}

impl ConnectorManager {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let active_connector = match config.db_type {
            DatabaseType::MongoDB => {
                Box::new(MongoDBConnector::new(&config.mongodb.as_ref().unwrap()).await?)
            }
            DatabaseType::Sqlite => {
                Box::new(SqliteConnector::new(&config.sqlite.as_ref().unwrap()).await?)
            }
            DatabaseType::PostgreSQL => {
                Box::new(PostgreSQLConnector::new(&config.postgresql.as_ref().unwrap()).await?)
            }
        };

        // Crear fallback automático a SQLite si la conexión principal falla
        let fallback_connector = Some(Box::new(
            SqliteConnector::new(&SqliteConfig { 
                path: "./data/bitacora_fallback.db".to_string(),
                max_connections: 10 
            }).await?
        ));

        Ok(Self {
            active_connector,
            fallback_connector,
        })
    }
}
```

¿Te parece bien esta arquitectura de configuración flexible?
