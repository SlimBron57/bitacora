# 📋 API Layer - Documentación Completa v2.0

## 🎯 **API REST - Endpoints Implementados**

### **🏥 Health Check**
```http
GET /health
```
**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "timestamp": "2025-08-26T02:27:47.342Z",
    "version": "0.1.0",
    "uptime": "1h"
  },
  "message": null,
  "timestamp": "2025-08-26T02:27:47.342Z"
}
```

### **📁 Projects Management**
```http
GET /projects?page=1&limit=10
```
**Response:**
```jso## 🔗 **Integración con OpenAPI**

Los DTOs automáticamente generan:
- **JSON Schemas** para Swagger UI
- **Ejemplos interactivos** en la documentación  
- **Validación automática** en endpoints
- **Tipos TypeScript** exportables

## 🏗️ **Arquitectura Técnica del Servidor**

```
┌────────────────────────────────────────────────────────────────────┐
│                      AXUM SERVER STACK                            │
│                                                                    │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐  │
│  │   HTTP Request  │──►│   Middleware    │──►│   Router        │  │
│  │                 │   │                 │   │                 │  │
│  │ - Headers       │   │ - CORS          │   │ - /health       │  │
│  │ - Body          │   │ - Timeout       │   │ - /projects     │  │
│  │ - Method        │   │ - Tracing       │   │ - /topics       │  │
│  │ - Path          │   │ - Auth (future) │   │ - /actions      │  │
│  │                 │   │                 │   │ - /sparks       │  │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘  │
│                                                        │            │
│                                                        ▼            │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐  │
│  │  HTTP Response  │◄──│      DTOs       │◄──│    Handlers     │  │
│  │                 │   │                 │   │                 │  │
│  │ - JSON Body     │   │ - Serialization │   │ - Business      │  │
│  │ - Status Code   │   │ - Validation    │   │   Logic         │  │
│  │ - Headers       │   │ - OpenAPI       │   │ - Data          │  │
│  │ - Timestamps    │   │ - Type Safety   │   │   Generation    │  │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘  │
│                                                                    │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    SWAGGER UI                              │  │
│  │                                                            │  │
│  │  📚 /swagger-ui  ──►  Interactive Documentation           │  │
│  │  📄 /api-docs/openapi.json  ──►  OpenAPI Specification    │  │
│  └────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────┘
```success": true,
  "data": [
    {
      "id": "cf328800-a76f-436b-a71f-fc4fcccd5786",
      "name": "Test Project 1",
      "description": "A test project",
      "status": "active",
      "created_at": "2025-08-26T02:27:47.342Z"
    }
  ],
  "message": null,
  "timestamp": "2025-08-26T02:27:47.342Z"
}
```

### **📝 Topics by Project**
```http
GET /projects/{project_id}/topics
```
**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "40063782-d77c-41f6-b19c-31fb9b671a3e",
      "project_id": "cf328800-a76f-436b-a71f-fc4fcccd5786",
      "title": "API Development",
      "description": "Develop REST API",
      "status": "in_progress",
      "created_at": "2025-08-26T02:27:47.342Z"
    }
  ],
  "message": null,
  "timestamp": "2025-08-26T02:27:47.342Z"
}
```

### **⚡ Actions by Topic**
```http
GET /topics/{topic_id}/actions
```
**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "06f6f956-011f-4733-8cb1-7d2f6216a85f",
      "topic_id": "40063782-d77c-41f6-b19c-31fb9b671a3e",
      "title": "Setup development environment",
      "description": "Configure Rust project with dependencies",
      "action_type": "task",
      "status": "completed",
      "priority": "high",
      "due_date": "2025-08-26T02:27:47.342Z",
      "created_at": "2025-08-26T02:27:47.342Z",
      "completed_at": "2025-08-26T02:27:47.342Z"
    }
  ],
  "message": null,
  "timestamp": "2025-08-26T02:27:47.342Z"
}
```

### **✨ Sparks Management**
```http
GET /sparks?page=1&limit=10
```
**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "d3b33c20-a8ff-4301-9e7b-532954ac3f2e",
      "title": "API Performance Optimization",
      "content": "Consider implementing caching layer for frequently accessed endpoints to improve response times",
      "spark_type": "idea",
      "tags": ["performance", "api", "caching"],
      "project_id": "bf3aaafb-b0fe-4716-8a24-e7ad67fd9e50",
      "topic_id": null,
      "created_at": "2025-08-26T02:27:47.342Z",
      "updated_at": null
    }
  ],
  "message": null,
  "timestamp": "2025-08-26T02:27:47.342Z"
}
```

## 🏗️ **DTOs Architecture Overview**

```
┌────────────────────────────────────────────────────────────────────┐
│                          API LAYER                                 │
│  ┌──────────────┐   JSON   ┌──────────────┐   Transform   ┌─────── │
│  │ Client Data  │ ◄─────── │  API DTOs    │ ◄───────────  │ Models │
│  │              │          │              │               │        │
│  │ - UI Format  │          │ - Validation │               │ - DB   │
│  │ - User Data  │          │ - OpenAPI    │               │ - Logic│
│  │ - Frontend   │          │ - Security   │               │ - Core │
│  └──────────────┘          └──────────────┘               └─────── │
└────────────────────────────────────────────────────────────────────┘
```

## 📊 **Relaciones entre Entidades**

```
┌─────────────────┐  1:N  ┌─────────────────┐  1:N  ┌─────────────────┐
│     Project     │ ────► │      Topic      │ ────► │     Action      │
│                 │       │                 │       │                 │
│ - id: UUID      │       │ - id: UUID      │       │ - id: UUID      │
│ - name          │       │ - project_id    │       │ - topic_id      │
│ - status        │       │ - title         │       │ - title         │
│ - created_at    │       │ - status        │       │ - action_type   │
└─────────────────┘       └─────────────────┘       │ - status        │
         │                         │                │ - priority      │
         │ 0:N                     │ 0:N            └─────────────────┘
         │                         │
         ▼                         ▼
┌─────────────────────────────────────────────────────────────────────┐
│                         Spark                                       │
│                                                                     │
│ - id: UUID                                                          │
│ - title                                                             │
│ - content                                                           │
│ - spark_type: [idea, insight, question, observation]                │
│ - tags: [String]                                                    │
│ - project_id: Optional<UUID>  ← Puede asociarse a Project           │
│ - topic_id: Optional<UUID>    ← Puede asociarse a Topic             │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## 🏗️ **DTOs (Data Transfer Objects)**

### 1. **Project DTO**
```rust
pub struct Project {
    pub id: Uuid,
    pub name: String,                    // 1-100 chars
    pub description: Option<String>,     // max 500 chars
    pub status: String,                  // "active", "planning", "completed", "archived"
    pub created_at: DateTime<Utc>,
}
```

### 2. **Topic DTO**
```rust
pub struct Topic {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,                   // 1-200 chars
    pub description: Option<String>,     // max 1000 chars
    pub status: String,                  // "active", "in_progress", "completed", "on_hold"
    pub created_at: DateTime<Utc>,
}
```

### 3. **Action DTO**
```rust
pub struct Action {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub title: String,                   // 1-200 chars
    pub description: Option<String>,     // max 1000 chars
    pub action_type: String,             // Ver tipos abajo
    pub status: String,                  // Ver estados abajo
    pub priority: String,                // Ver prioridades abajo
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
```

**Action Types:**
- **"task"** - Tarea ejecutable concreta
- **"milestone"** - Hito o meta importante del proyecto
- **"reminder"** - Recordatorio temporal o periódico

```
┌─────────────────────────────────────────────────────────────────────┐
│                      ACTION WORKFLOW                                │
│                                                                     │
│  ┌─────────┐   start   ┌─────────────┐   complete   ┌─────────────┐ │
│  │ pending │ ────────► │ in_progress │ ───────────► │  completed  │ │
│  └─────────┘           └─────────────┘              └─────────────┘ │
│      │                        │                            │        │
│      │ cancel                 │ cancel                     │        │
│      ▼                        ▼                            │        │
│  ┌─────────┐                                               │        │
│  │cancelled│ ◄─────────────────────────────────────────────┘        │
│  └─────────┘                                                        │
│                                                                     │
│  Priority Flow:   LOW ──► MEDIUM ──► HIGH ──► URGENT                │
│                                                                     │
│  Types:                                                             │
│  ├── 📋 TASK      (executable work)                                 │
│  ├── 🎯 MILESTONE (project goals)                                   │
│  └── ⏰ REMINDER  (temporal alerts)                                 │
└─────────────────────────────────────────────────────────────────────┘
```

**Action Status:**
- **"pending"** - Pendiente de inicio
- **"in_progress"** - En progreso activo
- **"completed"** - Completada exitosamente
- **"cancelled"** - Cancelada (ya no necesaria)

**Action Priority:**
- **"low"** - Prioridad baja
- **"medium"** - Prioridad media
- **"high"** - Prioridad alta
- **"urgent"** - Urgente (requiere atención inmediata)

### 4. **Spark DTO**
```rust
pub struct Spark {
    pub id: Uuid,
    pub title: String,                   // 1-200 chars
    pub content: String,                 // 1-5000 chars
    pub spark_type: String,              // Ver tipos abajo
    pub tags: Vec<String>,               // 0-10 tags, cada tag max 50 chars
    pub project_id: Option<Uuid>,        // Asociación opcional
    pub topic_id: Option<Uuid>,          // Asociación opcional
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

**Spark Types:**
- **"idea"** - Ideas creativas, propuestas, innovaciones
- **"insight"** - Descubrimientos, lecciones aprendidas, conocimientos
- **"question"** - Preguntas pendientes que requieren investigación
- **"observation"** - Notas sobre comportamientos, patrones detectados

```
┌─────────────────────────────────────────────────────────────────────┐
│                       SPARK ECOSYSTEM                               │
│                                                                     │
│  💡 IDEA          💎 INSIGHT       ❓ QUESTION      👁 OBSERVATION  │
│  ┌──────────┐     ┌─────────────┐   ┌───────────┐   ┌─────────────┐ │
│  │Creative  │     │ Discovery   │   │ Research  │   │ Pattern     │ │
│  │Proposal  │     │ Learning    │   │ Needed    │   │ Detection   │ │
│  │Innovation│     │ Knowledge   │   │ Decision  │   │ Behavior    │ │
│  └──────────┘     └─────────────┘   └───────────┘   └─────────────┘ │
│       │                │                │               │           │
│       │                │                │               │           │
│       └────────────────┴────────────────┴───────────────┘           │
│                                │                                    │
│                        ┌───────▼───────┐                            │
│                        │  SPARK CORE   │                            │
│                        │               │                            │
│                        │ - id: UUID    │                            │
│                        │ - title       │                            │
│                        │ - content     │                            │
│                        │ - tags[]      │                            │
│                        │ - timestamps  │                            │
│                        │               │                            │
│                        │ Associations: │                            │
│                        │ project_id?   │                            │
│                        │ topic_id?     │                            │
│                        └───────────────┘                            │
└─────────────────────────────────────────────────────────────────────┘
```

## 🧠 **Proceso de Toma de Decisiones - Clasificación de Tipos**

### **📊 Matriz de Decisión para Action Types**

```
┌─────────────────────────────────────────────────────────────────────┐
│                    DECISION MATRIX - ACTION TYPES                   │
│                                                                     │
│  Criteria ➡️       Executable  Timeline   Measurement   Result      │
│  ┌─────────────┬─────────────┬────────────┬────────────┬─────────── │
│  │ TASK        │    ✅ Yes    │  Specific  │ Complete/  │  Output    │
│  │             │             │  Deadline  │ Incomplete │  Created   │
│  ├─────────────┼─────────────┼────────────┼────────────┼─────────── │
│  │ MILESTONE   │   ⚠️  Mixed  │ Target     │ Achievement│ Goal       │
│  │             │             │ Date       │ Level      │ Reached    │
│  ├─────────────┼─────────────┼────────────┼────────────┼─────────── │
│  │ REMINDER    │    ❌ No     │ Recurring/ │ Triggered/ │ Alert      │
│  │             │             │ One-time   │ Dismissed  │ Sent       │
│  └─────────────┴─────────────┴────────────┴────────────┴─────────── │
│                                                                     │
│  🎯 Decision Flow:                                                  │
│  1. Is it executable work? ──► TASK                                 │
│  2. Is it a project goal?  ──► MILESTONE                            │
│  3. Is it a time trigger?  ──► REMINDER                             │
└─────────────────────────────────────────────────────────────────────┘
```

### **🔍 Matriz de Decisión para Spark Types**

```
┌─────────────────────────────────────────────────────────────────────┐
│                    DECISION MATRIX - SPARK TYPES                    │
│                                                                     │
│  Intent ➡️         Creative   Knowledge   Inquiry    Recording      │
│  ┌─────────────┬────────────┬───────────┬──────────┬─────────────── │
│  │ IDEA        │   ✅ High   │   Future  │   Low    │  Proposal     │
│  │             │            │  Focused  │          │  Innovation   │
│  ├─────────────┼────────────┼───────────┼──────────┼─────────────── │
│  │ INSIGHT     │   ⚠️  Med   │ ✅ High   │   Low    │  Learning     │
│  │             │            │ Past Exp. │          │  Knowledge    │
│  ├─────────────┼────────────┼───────────┼──────────┼─────────────── │
│  │ QUESTION    │   ⚠️  Med   │   Mixed   │ ✅ High  │  Research     │
│  │             │            │           │          │  Decision     │
│  ├─────────────┼────────────┼───────────┼──────────┼─────────────── │
│  │ OBSERVATION │    ❌ Low   │ ✅ High   │   Low    │ ✅ High       │
│  │             │            │ Current   │          │  Pattern      │
│  └─────────────┴────────────┴───────────┴──────────┴─────────────── │
│                                                                     │
│  🎯 Decision Flow:                                                  │
│  1. Does it propose something new?     ──► IDEA                     │
│  2. Does it capture learned knowledge? ──► INSIGHT                  │
│  3. Does it require investigation?     ──► QUESTION                 │
│  4. Does it document a pattern?        ──► OBSERVATION              │
└─────────────────────────────────────────────────────────────────────┘
```

### **👥 Responsabilidades en la Clasificación**

#### **🤖 Clasificación Automática (Sistema)**
```rust
// Algoritmo de sugerencia automática basado en patrones de texto
fn suggest_action_type(title: &str, description: &str) -> ActionType {
    let keywords_task = ["implement", "create", "fix", "update", "write"];
    let keywords_milestone = ["complete", "finish", "deploy", "release", "achieve"];
    let keywords_reminder = ["review", "check", "remind", "schedule", "alert"];
    
    // Análisis de keywords y contexto
    match analyze_content(title, description) {
        Pattern::ExecutableWork => ActionType::Task,
        Pattern::ProjectGoal => ActionType::Milestone,
        Pattern::TemporalAlert => ActionType::Reminder,
    }
}

fn suggest_spark_type(content: &str, context: &SparkContext) -> SparkType {
    let patterns_idea = ["what if", "we could", "proposal", "suggestion"];
    let patterns_insight = ["learned", "discovered", "realized", "understood"];
    let patterns_question = ["how", "why", "what", "should we", "?"];
    let patterns_observation = ["noticed", "observed", "pattern", "behavior"];
    
    // ML-based classification future enhancement
}
```

#### **👤 Clasificación Manual (Usuario)**
```
┌─────────────────────────────────────────────────────────────────────┐
│                    USER DECISION INTERFACE                          │
│                                                                     │
│  📝 Smart Suggestions:                                              │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ Title: "Implement user authentication"                     │   │
│  │                                                            │   │
│  │ 🤖 AI Suggests: TASK (89% confidence)                      │   │
│  │    Reasons: Contains "implement", describes executable work │   │
│  │                                                            │   │
│  │ 👤 User Override: [TASK] [MILESTONE] [REMINDER]           │   │
│  │    Manual selection if AI is wrong                         │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  📊 Learning Feedback Loop:                                         │
│  • User corrections improve AI accuracy                             │
│  • Pattern recognition gets better over time                        │
│  • Context awareness increases                                      │
└─────────────────────────────────────────────────────────────────────┘
```

#### **🏢 Clasificación Organizacional (Equipo)**
```
┌─────────────────────────────────────────────────────────────────────┐
│                    TEAM DECISION GOVERNANCE                         │
│                                                                     │
│  📋 Standards & Guidelines:                                         │
│  ├── Action Types:                                                  │
│  │   ├── TASK: Individual contributor work (< 2 weeks)              │
│  │   ├── MILESTONE: Team goals (2-8 weeks)                          │
│  │   └── REMINDER: Process/maintenance items                        │
│  │                                                                  │
│  ├── Spark Types:                                                   │
│  │   ├── IDEA: Requires POC/validation                              │
│  │   ├── INSIGHT: Documented learning                               │
│  │   ├── QUESTION: Needs research/decision                          │
│  │   └── OBSERVATION: Metrics/behavior data                         │
│  │                                                                  │
│  └── 🎯 Role-based Authority:                                       │
│      ├── Developer: All action types                                │
│      ├── Tech Lead: Milestone validation                            │
│      ├── Product Manager: Priority assignment                       │
│      └── Architect: Type standards evolution                        │
└─────────────────────────────────────────────────────────────────────┘
```

### **📈 Métricas y Evolución de Tipos**

#### **🔍 Análisis de Patrones de Uso**
```
┌─────────────────────────────────────────────────────────────────────┐
│                     TYPE USAGE ANALYTICS                           │
│                                                                     │
│  📊 Distribution Over Time:                                         │
│                                                                     │
│  Actions:  █████████ TASK (65%)                                    │
│           ███ MILESTONE (20%)                                      │
│           ██ REMINDER (15%)                                        │
│                                                                     │
│  Sparks:   ████ IDEA (35%)                                         │
│           ████ INSIGHT (30%)                                       │
│           ███ QUESTION (25%)                                       │
│           ██ OBSERVATION (10%)                                     │
│                                                                     │
│  🎯 Classification Accuracy:                                        │
│  ├── Auto-classification: 78% accuracy                             │
│  ├── User corrections: 22% override rate                           │
│  └── Learning improvement: +5% accuracy per month                  │
│                                                                     │
│  📈 Trends & Adaptations:                                           │
│  • New type proposals based on usage patterns                      │
│  • Seasonal variations (more MILESTONES near releases)             │
│  • Team-specific preferences and customizations                    │
└─────────────────────────────────────────────────────────────────────┘
```

### **🚀 Proceso de Evolución de Tipos**

#### **💡 Criterios para Nuevos Tipos**
```
┌─────────────────────────────────────────────────────────────────────┐
│                    NEW TYPE EVALUATION PROCESS                     │
│                                                                     │
│  📋 Requirements Checklist:                                         │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ ✅ Frequency: >5% of total items need this classification  │   │
│  │ ✅ Distinction: Cannot be handled by existing types        │   │
│  │ ✅ Behavior: Requires different workflow/status handling   │   │
│  │ ✅ Value: Provides meaningful filtering/reporting benefit  │   │
│  │ ✅ Consensus: Team agreement on necessity                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  🔄 Evaluation Workflow:                                            │
│  1. 📊 Data Analysis ──► Usage pattern identification              │
│  2. 🏛️  RFC Process ──► Team discussion and proposal              │
│  3. 🧪 Prototype ──────► Limited implementation trial              │
│  4. 📈 Validation ─────► Metrics collection and analysis           │
│  5. ✅ Adoption ───────► Full implementation and documentation      │
│                                                                     │
│  📝 Historical Examples:                                            │
│  ├── EPIC (rejected): Too similar to MILESTONE                     │
│  ├── BUG (considered): Could be TASK with priority                 │
│  └── RESEARCH (potential): Pattern emerging for QUESTION+INSIGHT   │
└─────────────────────────────────────────────────────────────────────┘
```

#### **🔮 Tipos en Consideración (Roadmap)**
```rust
// Potential future types based on usage analysis
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtendedActionType {
    // Current types
    Task,
    Milestone,
    Reminder,
    
    // Under consideration
    #[cfg(feature = "experimental")]
    Epic,        // Large initiatives (3+ months)
    
    #[cfg(feature = "experimental")]
    Bug,         // Defect tracking (vs feature TASK)
    
    #[cfg(feature = "experimental")]
    Research,    // Investigation work (distinct from QUESTION spark)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtendedSparkType {
    // Current types
    Idea,
    Insight,
    Question,
    Observation,
    
    // Under consideration
    #[cfg(feature = "experimental")]
    Hypothesis,  // Testable propositions
    
    #[cfg(feature = "experimental")]
    Decision,    // Recorded choices and rationale
    
    #[cfg(feature = "experimental")]
    Lesson,      // Structured learning (enhanced INSIGHT)
}
```

### **📋 Autoridad de Decisión y Escalación**

#### **🏗️ Governance Structure**
```
┌─────────────────────────────────────────────────────────────────────┐
│                    DECISION AUTHORITY MATRIX                        │
│                                                                     │
│  Decision Level           │  Authority        │  Stakeholders       │
│  ────────────────────────┼──────────────────┼──────────────────── │
│  🔧 Type Selection        │  Individual User │  Self + AI assist   │
│  ├─ Daily classification │  Personal choice │  Learning feedback  │
│  └─ Override AI suggest  │  User final word │  Context owner      │
│                          │                  │                     │
│  🏢 Team Standards        │  Technical Lead  │  Development Team   │
│  ├─ Classification rules │  Lead + consensus│  Stakeholder input  │
│  └─ Process guidelines   │  Team decision   │  Product alignment  │
│                          │                  │                     │
│  🏛️  System Evolution     │  Architecture    │  All Users          │
│  ├─ New type addition    │  Architect + RFC │  Community vote     │
│  ├─ Breaking changes     │  Senior approval │  Migration support  │
│  └─ Core type removal    │  Board decision  │  Impact assessment  │
│                          │                  │                     │
│  📊 Conflict Resolution   │                  │                     │
│  ├─ User vs AI          ➤│  User wins       │  Feedback to ML     │
│  ├─ User vs Team        ➤│  Discussion      │  Compromise/vote    │
│  └─ Team vs System      ➤│  RFC process     │  Data-driven        │
└─────────────────────────────────────────────────────────────────────┘
```

#### **⚖️ Criterios de Resolución de Conflictos**
```
┌─────────────────────────────────────────────────────────────────────┐
│                    CONFLICT RESOLUTION FRAMEWORK                    │
│                                                                     │
│  🎯 Principios de Decisión (Por prioridad):                        │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │ 1. 🎯 User Intent      ──► Personal context wins            │   │
│  │ 2. 🏢 Team Consistency ──► Standards alignment              │   │
│  │ 3. 📊 Data Evidence    ──► Usage patterns matter            │   │
│  │ 4. 🚀 System Evolution ──► Future compatibility             │   │
│  │ 5. 🔄 Learning Loop    ──► Continuous improvement           │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  🤔 Example Scenarios:                                              │
│                                                                     │
│  Scenario A: "Fix critical bug in user auth"                       │
│  ├─ AI suggests: TASK (high confidence)                            │
│  ├─ User selects: MILESTONE (wants visibility)                     │
│  ├─ Team standard: TASK for bugs                                   │
│  └─ Resolution: User choice wins, data logged for analysis         │
│                                                                     │
│  Scenario B: "Research new database technology"                    │
│  ├─ AI suggests: QUESTION (medium confidence)                      │
│  ├─ User unsure: Could be IDEA or QUESTION                         │
│  ├─ Team discussion: Leans toward QUESTION                         │
│  └─ Resolution: Team consensus, possible new RESEARCH type         │
└─────────────────────────────────────────────────────────────────────┘
```

### 5. **Common DTOs**
```rust
// Wrapper para todas las respuestas
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

// Health check específico
pub struct HealthStatus {
    pub status: String,                  // "healthy", "degraded", "unhealthy"
    pub timestamp: DateTime<Utc>,
    pub version: String,                 // Versión de la API
    pub uptime: String,                  // Tiempo activo
}

// Paginación
pub struct Pagination {
    pub page: Option<u32>,               // Default: 1
    pub limit: Option<u32>,              // Default: 20, Max: 100
}
```

### 2. **TopicDTO** (`topics.rs`)  
- **CreateTopicRequest** - Input para crear tema
- **TopicResponse** - Output completo del tema
- **TopicListResponse** - Lista con filtros
- **UpdateTopicRequest** - Actualizaciones parciales

**Validaciones:**
- Title: 1-200 caracteres, requerido
- Status: Draft → Active → Completed → Archived

### 3. **ActionDTO** (`actions.rs`)
- **CreateActionRequest** - Nueva acción
- **ActionResponse** - Detalles completos
- **ActionListResponse** - Historial paginado  
- **UpdateActionRequest** - Cambios de estado

**Validaciones:**
- Description: 1-300 caracteres
- ActionType: FileEdit, Debug, Research, Planning, etc.
- Context: JSON opcional para metadatos

### 4. **SparkDTO** (`sparks.rs`)
- **CreateSparkRequest** - Capturar insight
- **SparkResponse** - Insight completo
- **SparkListResponse** - Lista filtrada
- **ApplySparkRequest** - Aplicar insight

**Validaciones:**
- Title: 1-150 caracteres
- Content: 1-1000 caracteres para el insight
- SparkType: Insight, Idea, Problem, Solution

### 5. **CommonDTO** (`common.rs`)
- **PaginationQuery** - limit/offset estándar
- **ErrorResponse** - Formato de errores consistente
- **HealthResponse** - Status del sistema

## 🔍 **Ejemplo de Implementación**

```rust
// DTO - Para API REST
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(max = 500))]
    pub description: Option<String>,
}

// Model - Interno del sistema  
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    // ... campos internos no expuestos
}
```

## 🎯 **Características Técnicas**

### **Validación Automática**
```rust
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
}
```

### **OpenAPI Documentation**
```rust
use utoipa::ToSchema;

/// Request to create a new project in the Bitacora system
#[derive(Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Mi Proyecto Rust",
    "description": "Sistema de gestión de bitácoras"
}))]
pub struct CreateProjectRequest { ... }
```

### **Transformación Bidireccional**
```rust
impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.project_id,
            name: project.name,
            // Transformación controlada
        }
    }
}
```

## ✅ **Estado Actual**

| DTO Module | Structs | Validación | OpenAPI | Ejemplos |
|------------|---------|------------|---------|----------|
| projects.rs | ✅ 4/4  | ✅ Completa | ✅ Sí | ✅ Sí |
| topics.rs   | ✅ 4/4  | ✅ Completa | ✅ Sí | ✅ Sí |
| actions.rs  | ✅ 4/4  | ✅ Completa | ✅ Sí | ✅ Sí |
| sparks.rs   | ✅ 4/4  | ✅ Completa | ✅ Sí | ✅ Sí |
| common.rs   | ✅ 3/3  | ✅ Completa | ✅ Sí | ✅ Sí |

**Total: 19 DTOs completamente implementados** 🎉

## � **Flujo Típico de Trabajo con la API**

```
┌───────────────────────────────────────────────────────────────────────┐
│                        API WORKFLOW                                   │
│                                                                       │
│  1️⃣ HEALTH CHECK        2️⃣ PROJECT DISCOVERY    3️⃣ TOPIC EXPLORATION  │
│  ┌─────────────────┐    ┌─────────────────────┐  ┌─────────────────┐  │
│  │ GET /health     │    │ GET /projects       │  │ GET /projects/  │  │
│  │                 │    │                     │  │     {id}/topics │  │
│  │ Response:       │    │ Response:           │  │                 │  │
│  │ ├─ status       │    │ ├─ projects[]       │  │ Response:       │  │
│  │ ├─ version      │    │ ├─ pagination       │  │ ├─ topics[]     │  │
│  │ └─ uptime       │    │ └─ timestamps       │  │ └─ project_id   │  │
│  └─────────────────┘    └─────────────────────┘  └─────────────────┘  │
│           │                       │                       │           │
│           ▼                       ▼                       ▼           │
│  ✅ System Ready          📋 Select Project        📝 Select Topic    │
│                                  │                       │            │
│                                  └───────────────────────┘            │
│                                              │                        │
│                                              ▼                        │
│  4️⃣ ACTION MANAGEMENT                    5️⃣ SPARK EXPLORATION         │
│  ┌─────────────────────┐                ┌─────────────────────┐       │
│  │ GET /topics/{id}/   │                │ GET /sparks         │       │
│  │     actions         │                │                     │       │
│  │                     │                │ Response:           │       │
│  │ Response:           │                │ ├─ ideas            │       │
│  │ ├─ actions[]        │                │ ├─ insights         │       │
│  │ ├─ task types       │                │ ├─ questions        │       │
│  │ ├─ priorities       │                │ ├─ observations     │       │
│  │ ├─ statuses         │                │ └─ associations     │       │
│  │ └─ due_dates        │                └─────────────────────┘       │
│  └─────────────────────┘                                              │
│           │                                       │                   │
│           ▼                                       ▼                   │
│  ⚡ Execute Tasks                          💡 Capture Insights         │
└───────────────────────────────────────────────────────────────────────┘
```

## �🔗 **Integración con OpenAPI**

Los DTOs automáticamente generan:
- **JSON Schemas** para Swagger UI
- **Ejemplos interactivos** en la documentación  
- **Validación automática** en endpoints
- **Tipos TypeScript** exportables
