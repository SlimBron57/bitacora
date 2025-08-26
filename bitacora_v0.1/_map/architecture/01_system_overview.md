# Arquitectura General del Sistema Bitacora V1.0

## 🎯 Visión Arquitectónica

Bitacora V1.0 implementa una **arquitectura modular SOLID** en Rust que reemplaza el sistema de scripts Bash por una solución escalable, mantenible y type-safe con integración MongoDB y API HTTP.

## 🏗️ Principios Arquitectónicos Fundamentales

### 1. SOLID Compliance
- **S**ingle Responsibility: Cada crate tiene una responsabilidad específica
- **O**pen/Closed: Extensible mediante traits sin modificar código existente
- **L**iskov Substitution: Implementaciones intercambiables de traits
- **I**nterface Segregation: Interfaces específicas y cohesivas
- **D**ependency Inversion: Dependencias sobre abstracciones, no implementaciones

### 2. Modularidad por Dominios
- Separación clara entre dominios de negocio
- Comunicación a través de interfaces bien definidas
- Reutilización de componentes entre diferentes contextos

### 3. Asíncrono por Defecto
- Toda I/O es asíncrona usando Tokio
- Non-blocking operations para mejor throughput
- Manejo eficiente de recursos del sistema

## 🧱 Arquitectura de Alto Nivel

```
┌─────────────────────────────────────────────────────────────┐
│                    API Layer (Axum)                        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │   Commands  │ │   Health    │ │   Administration    │   │
│  │   Endpoints │ │   Checks    │ │     Interface       │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Business Logic Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │   Command   │ │   Session   │ │     Telemetry       │   │
│  │   Handler   │ │   Manager   │ │     Collector       │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Service Layer                            │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │  Timestamp  │ │   Records   │ │       Git           │   │
│  │   Service   │ │   Service   │ │     Service         │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Data Access Layer                         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │  Repository │ │  Database   │ │   Configuration     │   │
│  │   Pattern   │ │  Connector  │ │     Manager         │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Infrastructure Layer                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐   │
│  │   MongoDB   │ │  File System│ │    External APIs    │   │
│  │   Database  │ │   Storage   │ │    (Git, Health)    │   │
│  └─────────────┘ └─────────────┘ └─────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Componentes Principales

### API Layer
**Responsabilidad**: Interfaz HTTP para comunicación externa
- **Axum Web Framework**: Manejo de requests/responses
- **Middleware Stack**: Logging, CORS, Authentication, Rate Limiting
- **Error Handling**: Conversión de errores internos a responses HTTP
- **Request Validation**: Validación de payloads y parámetros

### Business Logic Layer
**Responsabilidad**: Lógica de negocio core de Bitacora
- **Command Processing**: Interpretación y ejecución de comandos
- **Session Management**: Lifecycle completo de sesiones de desarrollo
- **Workflow Orchestration**: Coordinación de operaciones complejas
- **Business Rules**: Validaciones y reglas de negocio específicas

### Service Layer
**Responsabilidad**: Servicios de dominio especializados
- **Domain Services**: Operaciones que no pertenecen a una entidad específica
- **External Integration**: Comunicación con sistemas externos (Git)
- **Cross-cutting Concerns**: Logging, métricas, caching

### Data Access Layer
**Responsabilidad**: Abstracción de persistencia de datos
- **Repository Pattern**: Abstracción de acceso a datos
- **Database Abstraction**: Múltiples conectores (MongoDB, SQLite, etc.)
- **Query Building**: Construcción type-safe de queries
- **Transaction Management**: Manejo de transacciones y consistencia

### Infrastructure Layer
**Responsabilidad**: Detalles técnicos de infraestructura
- **Database Drivers**: Conexiones específicas a bases de datos
- **File System**: Operaciones de archivos cuando necesarias
- **Network Communication**: HTTP clients, TCP connections
- **System Integration**: Proceso spawning, signal handling

## 🔄 Flujo de Procesamiento de Comandos

```
1. HTTP Request (Copilot)
           │
           ▼
2. Axum Router & Middleware
           │
           ▼
3. Request Validation & Parsing
           │
           ▼
4. Command Handler (Business Logic)
           │
           ▼
5. Service Layer Orchestration
           │
           ▼
6. Repository Pattern (Data Access)
           │
           ▼
7. Database Connector
           │
           ▼
8. MongoDB Operations
           │
           ▼
9. Response Assembly
           │
           ▼
10. HTTP Response (JSON)
```

### Ejemplo de Flujo - Comando START

```
POST /commands/start
│
├─ Axum Handler: handle_start_command()
│  ├─ Validate request payload
│  ├─ Extract user context
│  └─ Call CommandHandler::execute(Command::Start)
│
├─ CommandHandler (Business Logic)
│  ├─ Check if session already active
│  ├─ Validate git repository state
│  ├─ Orchestrate session start workflow
│  └─ Call multiple services
│
├─ Service Orchestration
│  ├─ TimestampService::start_daemon()
│  ├─ GitService::current_branch()
│  ├─ RecordService::create_or_resume_session()
│  └─ ActionService::add_session_start_action()
│
├─ Repository Layer
│  ├─ ProjectRepository::find_by_path()
│  ├─ SessionRepository::create_session()
│  └─ ActionRepository::create_action()
│
├─ Database Operations
│  ├─ MongoDB: Insert into sessions collection
│  ├─ MongoDB: Insert into actions collection
│  └─ MongoDB: Update projects collection
│
└─ Response Assembly
   └─ JSON: {"success": true, "session_id": "...", "output": "🚀 Session started"}
```

## 🎨 Patrones de Diseño Implementados

### 1. Repository Pattern
**Propósito**: Abstracción del acceso a datos
```rust
// Ejemplo conceptual - NO código funcional
trait SessionRepository {
    async fn find_current_session(&self, project_id: &str) -> Result<Option<Session>>;
    async fn create_session(&self, session: &Session) -> Result<String>;
    async fn update_session(&self, session: &Session) -> Result<()>;
}
```

### 2. Dependency Injection
**Propósito**: Inversión de control y testabilidad
```rust
// Ejemplo conceptual - NO código funcional
struct CommandHandler {
    session_repo: Arc<dyn SessionRepository>,
    git_service: Arc<dyn GitService>,
    timestamp_service: Arc<dyn TimestampService>,
}
```

### 3. Strategy Pattern
**Propósito**: Múltiples implementaciones intercambiables
```rust
// Ejemplo conceptual - NO código funcional  
enum DatabaseConnector {
    MongoDB(MongoConnector),
    SQLite(SqliteConnector),
    PostgreSQL(PostgresConnector),
}
```

### 4. Observer Pattern
**Propósito**: Notificaciones de eventos del sistema
```rust
// Ejemplo conceptual - NO código funcional
trait EventSubscriber {
    async fn handle_event(&self, event: SystemEvent);
}
```

### 5. Command Pattern
**Propósito**: Encapsulación de operaciones como objetos
```rust
// Ejemplo conceptual - NO código funcional
enum Command {
    Start { description: Option<String> },
    Action { action_type: String, description: String },
    Branch { name: String },
}
```

## 🔐 Principios de Seguridad

### 1. Input Validation
- Validación exhaustiva de todos los inputs
- Sanitización de datos antes de procesamiento
- Rate limiting para prevenir abuse

### 2. Database Security
- Prepared statements/queries parametrizadas
- Principio de menor privilegio para conexiones DB
- Encryption at rest y in transit

### 3. Configuration Security
- Secrets management separado de configuración
- Environment-based configuration
- Validación de configuración al startup

### 4. API Security
- Authentication y authorization cuando requerido
- CORS policy restrictiva
- Request/response logging para auditoria

## 📊 Observabilidad y Monitoreo

### 1. Structured Logging
- Logs en formato JSON para facilitar parsing
- Correlation IDs para tracing de requests
- Diferentes niveles de logging configurables

### 2. Métricas y Telemetría
- Prometheus-compatible metrics
- Custom business metrics (sesiones, acciones, etc.)
- Performance metrics (latencia, throughput, errores)

### 3. Health Checks
- Health endpoints configurables
- Dependency health monitoring
- Graceful degradation cuando sea posible

### 4. Distributed Tracing
- Tracing de requests end-to-end
- Identificación de bottlenecks
- Error correlation across services

## 🚀 Escalabilidad y Performance

### 1. Async-First Architecture
- Non-blocking I/O para mejor throughput
- Connection pooling para databases
- Efficient resource utilization

### 2. Caching Strategy
- In-memory caching para data frequently accessed
- Database query result caching
- Configuration caching con invalidation

### 3. Database Optimization
- Proper indexing strategy
- Query optimization
- Connection pooling y management

### 4. Horizontal Scalability
- Stateless service design
- Load balancer ready
- Database scaling considerations

---

**Próximo documento**: `02_crates_structure.md` - Estructura detallada de crates modulares
