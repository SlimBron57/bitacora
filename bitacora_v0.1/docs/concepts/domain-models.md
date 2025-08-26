# Domain Models - Conceptos Fundamentales

## 📋 **Metadatos del Documento**
- **Título**: Domain Models en Bitacora V1.0
- **Descripción Corta**: Explicación de los modelos de dominio y su importancia en la arquitectura
- **Creador**: bitacora (GitHub Copilot)
- **Timestamp Creación**: 20250821-1445
- **Editor**: bitacora (GitHub Copilot)
- **Timestamp Edición**: 20250821-1445

---

## 🏗️ **¿Qué son los Domain Models?**

Los **Domain Models** son representaciones en código de los **conceptos centrales del negocio**. En nuestro caso, representan las "cosas" principales que maneja Bitacora:

```rust
// Ejemplo: Una Sesión de trabajo
pub struct Session {
    pub session_id: Uuid,           // Identificador único
    pub user_id: String,            // A quién pertenece
    pub project_id: Option<Uuid>,   // En qué proyecto trabaja
    pub started_at: DateTime<Utc>,  // Cuándo empezó
    pub ended_at: Option<DateTime<Utc>>, // Cuándo terminó
    pub description: Option<String>, // Descripción opcional
    pub status: SessionStatus,       // Estado actual
}
```

## 🎯 **Domain Models en Bitacora V1.0**

Basándome en el sistema V0.1 existente, estos son nuestros conceptos principales:

### **1. Session (Sesión de Trabajo)**
**Representación**: Una sesión de trabajo completa
**Ejemplo**: "Sesión de 3 horas trabajando en el FFT Analyzer"

```rust
pub struct Session {
    session_id: Uuid,
    user_id: String,               // "edgi"
    project_id: Option<Uuid>,      // Proyecto actual
    started_at: DateTime<Utc>,     // 2025-08-21 14:30:00
    ended_at: Option<DateTime<Utc>>, // 2025-08-21 17:30:00
    status: SessionStatus,         // Active, Ended, Paused
    description: Option<String>,   // Descripción opcional
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,    // Sesión en progreso
    Ended,     // Sesión finalizada
    Paused,    // Sesión pausada temporalmente
}
```

### **2. Action (Acción Individual)**
**Representación**: Una acción específica dentro de una sesión
**Ejemplo**: "Implementé el algoritmo de FFT optimizado"

```rust
pub struct Action {
    action_id: Uuid,
    session_id: Uuid,              // A qué sesión pertenece
    user_id: String,               // Redundante pero útil para queries
    timestamp: DateTime<Utc>,      // Cuándo se hizo
    description: String,           // "Implementé FFT optimizado"
    status: ProjectStatus,         // Active, Completed, Archived
    action_type: ActionType,       // Code, Research, Debug, etc.
    tags: Vec<String>,            // ["rust", "performance", "fft"]
    duration_minutes: Option<u32>, // Duración estimada
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Code,        // Escribir código
    Debug,       // Debugging
    Research,    // Investigación
    Meeting,     // Reuniones
    Documentation, // Documentación
    Testing,     // Testing
    Refactor,    // Refactoring
}
```

### **3. Project (Proyecto)**
**Representación**: Un proyecto de desarrollo
**Ejemplo**: "AVA Audio Processor V2.0"

```rust
pub struct Project {
    project_id: Uuid,
    user_id: String,
    name: String,                  // "AVA Audio Processor"
    description: Option<String>,   // "Procesador de audio en tiempo real"
    repository_url: Option<String>, // Git repo URL
    status: ProjectStatus,         // Active, Completed, Archived
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,   // En planificación
    Active,     // Desarrollo activo
    Completed,  // Proyecto completado
    OnHold,     // En pausa
    Archived,   // Archivado
}
```

### **4. Topic (Tema/Objetivo)**
**Representación**: Un objetivo o tema de trabajo
**Ejemplo**: "Migrar de binnacle a bitacora"

```rust
pub struct Topic {
    topic_id: Uuid,
    user_id: String,
    title: String,                 // "Migración a Bitacora V1.0"
    description: String,           // Descripción detallada
    status: TopicStatus,           // Planning, InProgress, Completed
    priority: Priority,            // High, Medium, Low
    estimated_hours: Option<f32>,  // Estimación de tiempo
    actual_hours: Option<f32>,     // Tiempo real invertido
    created_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopicStatus {
    Planning,     // En planificación
    InProgress,   // En progreso
    Completed,    // Completado
    OnHold,       // En pausa
    Cancelled,    // Cancelado
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    High,    // Alta prioridad
    Medium,  // Prioridad media
    Low,     // Baja prioridad
}
```

### **5. Spark (Insight/Idea)**
**Representación**: Una idea, insight o aprendizaje
**Ejemplo**: "El problema de performance era el buffer size"

```rust
pub struct Spark {
    spark_id: Uuid,
    user_id: String,
    session_id: Option<Uuid>,      // Puede estar asociado a una sesión
    content: String,               // "Buffer size era el cuello de botella"
    spark_type: SparkType,         // Insight, Bug, Solution, etc.
    tags: Vec<String>,            // Para categorización
    created_at: DateTime<Utc>,
    related_actions: Vec<Uuid>,   // Acciones relacionadas
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SparkType {
    Insight,     // Comprensión o aprendizaje
    Idea,        // Nueva idea
    Bug,         // Problema identificado
    Solution,    // Solución propuesta
    Question,    // Pregunta para investigar
    Note,        // Nota general
}
```

### **6. User (Usuario)**
**Representación**: Usuario del sistema
**Ejemplo**: Desarrollador que usa Bitacora

```rust
pub struct User {
    user_id: String,               // Identificador único (username)
    display_name: String,          // "Edgar"
    email: Option<String>,         // Email opcional
    timezone: String,              // "America/Mexico_City"
    preferences: UserPreferences,  // Configuraciones personales
    created_at: DateTime<Utc>,
    last_active: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    default_session_reminder: Option<u32>, // Minutos
    preferred_time_format: TimeFormat,
    auto_backup_enabled: bool,
    notification_settings: NotificationSettings,
}
```

## 🧩 **¿Por qué son Importantes los Domain Models?**

### **1. Separación de Responsabilidades**
Los domain models mantienen la lógica de negocio separada de la infraestructura:

```rust
// ❌ MAL: Lógica mezclada con base de datos
fn crear_sesion_mongodb(user: &str, desc: &str) {
    let doc = doc! {
        "user": user,
        "desc": desc,
        "started": Utc::now()  // ¿Qué pasa si cambiamos a PostgreSQL?
    };
    collection.insert_one(doc);
}

// ✅ BIEN: Domain model independiente
impl Session {
    pub fn new(user_id: String, description: Option<String>) -> Self {
        Self {
            session_id: Uuid::new_v4(),
            user_id,
            description,
            started_at: Utc::now(),
            ended_at: None,
            status: SessionStatus::Active,
        }
    }
    
    pub fn end_session(&mut self) -> Duration {
        self.ended_at = Some(Utc::now());
        self.status = SessionStatus::Ended;
        self.duration().unwrap_or_default()
    }
    
    pub fn duration(&self) -> Option<Duration> {
        self.ended_at.map(|end| {
            (end - self.started_at).to_std().unwrap_or_default()
        })
    }
}
```

### **2. Facilitan el Testing**
Los domain models se pueden testear sin infraestructura:

```rust
#[test]
fn test_session_lifecycle() {
    let mut session = Session::new("test_user".to_string(), None);
    assert_eq!(session.status, SessionStatus::Active);
    
    // Simular paso del tiempo
    thread::sleep(Duration::from_millis(100));
    let duration = session.end_session();
    
    assert_eq!(session.status, SessionStatus::Ended);
    assert!(duration >= Duration::from_millis(100));
    // ¡No necesita base de datos para testear!
}
```

### **3. Código Más Legible y Mantenible**
```rust
// En lugar de diccionarios o structs genéricos:
let mut session_data = HashMap::new();
session_data.insert("user", "edgi");
session_data.insert("status", "active");
session_data.insert("started", "2025-08-21T14:30:00Z");

// Tenemos tipos específicos con métodos semánticos:
let session = Session::new("edgi".to_string(), None);
if session.is_active() {
    println!("Sesión activa desde: {}", session.started_at);
}
```

### **4. Evolución Controlada**
Los domain models permiten evolucionar el sistema de manera segura:

```rust
// Versión 1.0
pub struct Session {
    session_id: Uuid,
    user_id: String,
    started_at: DateTime<Utc>,
}

// Versión 1.1 - Agregar campos sin romper código existente
pub struct Session {
    session_id: Uuid,
    user_id: String,
    started_at: DateTime<Utc>,
    
    // Nuevos campos opcionales
    pub productivity_score: Option<f32>,  // Para métricas futuras
    pub ai_suggestions: Vec<String>,      // Para integración con IA
    pub mood_rating: Option<u8>,          // 1-10 rating del estado de ánimo
}
```

## 🔄 **Relación con el Sistema V0.1 Actual**

El sistema actual maneja estos conceptos, pero usando archivos:

```bash
# Sistema V0.1 (Basado en archivos)
records/20250821-1430_session_trabajo.md     → Session
topics/TOPIC_fft_analyzer.md                 → Topic  
scripts/action_add.sh                        → Action
cache/push_counter.txt                       → Métricas
```

```rust
// Sistema V1.0 (Domain Models + Base de Datos)
Session { session_id, started_at, ... }      → MongoDB Collection
Topic { topic_id, title, status, ... }       → MongoDB Collection  
Action { action_id, description, ... }       → MongoDB Collection
UserMetrics { user_id, stats, ... }         → MongoDB Collection
```

## 🎯 **Beneficios para Bitacora V1.0**

### **1. Validación Automática**
```rust
impl Session {
    pub fn new(user_id: String) -> Result<Self, ValidationError> {
        if user_id.trim().is_empty() {
            return Err(ValidationError::EmptyUserId);
        }
        
        if user_id.len() > 50 {
            return Err(ValidationError::UserIdTooLong);
        }
        
        Ok(Self {
            session_id: Uuid::new_v4(),
            user_id,
            started_at: Utc::now(),
            ended_at: None,
            status: SessionStatus::Active,
        })
    }
}
```

### **2. Integración Perfecta con APIs**
```rust
// Los domain models se serializan automáticamente para APIs
#[derive(Serialize, Deserialize)]
pub struct Session { ... }

// En el handler de la API:
async fn get_session(session_id: Uuid) -> Json<Session> {
    let session = session_service.get_session(session_id).await?;
    Json(session) // Automáticamente convierte a JSON
}

// Copilot recibe: 
// {"session_id": "...", "user_id": "edgi", "started_at": "2025-08-21T14:30:00Z"}
```

### **3. Consultas Type-Safe**
```rust
// En lugar de queries strings peligrosos:
let query = "SELECT * FROM sessions WHERE user_id = 'edgi'"; // ❌ SQL injection risk

// Usamos métodos type-safe:
let sessions = session_repository
    .find_by_user(&user_id)  // ✅ Type-safe
    .with_status(SessionStatus::Active)
    .order_by_date_desc()
    .limit(10)
    .execute()
    .await?;
```

### **4. Business Rules Centralizadas**
```rust
impl Topic {
    pub fn can_be_completed(&self) -> bool {
        matches!(self.status, TopicStatus::InProgress) && 
        self.estimated_hours.is_some()
    }
    
    pub fn mark_completed(&mut self) -> Result<(), BusinessRuleError> {
        if !self.can_be_completed() {
            return Err(BusinessRuleError::TopicNotReadyForCompletion);
        }
        
        self.status = TopicStatus::Completed;
        self.completed_at = Some(Utc::now());
        Ok(())
    }
}
```

## 🚀 **Implementación en el Checklist**

En **Día 3-5: Core Domain Types** implementaremos:

1. **✅ Todos estos domain models** con sus campos, validaciones y métodos
2. **✅ Service traits** que definen cómo interactuar con ellos  
3. **✅ Error types** para manejar todos los casos especiales
4. **✅ Validation logic** para asegurar consistencia de datos
5. **✅ Unit tests** para verificar el comportamiento

## 📚 **Próximos Pasos**

1. **Implementar structs básicos** en `bitacora-core/src/models/`
2. **Agregar validaciones** y métodos de negocio
3. **Crear service traits** para abstraer operaciones
4. **Escribir tests unitarios** para cada modelo
5. **Integrar con el sistema de persistencia** (Phase 2)

---

**💡 Nota**: Los domain models son el corazón de Bitacora V1.0. Una vez implementados correctamente, el resto del sistema (APIs, base de datos, interfaces) se construye naturalmente alrededor de ellos.
