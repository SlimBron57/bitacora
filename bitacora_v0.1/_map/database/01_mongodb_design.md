# Diseño Completo MongoDB - Bitacora V1.0

## 🎯 Filosofía de Diseño de Datos

Bitacora V1.0 utiliza **MongoDB** como base de datos principal, diseñada para ser **escalable**, **consultable**, y **optimizada para telemetría**. El diseño sigue principios de **normalización selectiva** para balance entre performance y flexibilidad.

## 🏗️ Arquitectura de Datos

### Principios de Diseño
1. **Multi-tenant por usuario**: Cada documento incluye `user_id` para aislamiento
2. **Telemetría completa**: Timestamps detallados para análisis ML futuro
3. **Auditoría total**: Todo cambio es trazable con historial
4. **Flexibilidad evolutiva**: Esquema puede evolucionar sin breaking changes
5. **Consultas eficientes**: Índices optimizados para queries frecuentes

### Estructura de Base de Datos
```
Database: bitacora_db

Collections:
├── projects              # Proyectos de desarrollo
├── sessions              # Sesiones de trabajo
├── actions               # Acciones individuales dentro de sesiones
├── topics                # Temas de desarrollo a largo plazo
├── sparks                # Insights y observaciones brillantes
├── timestamps            # Log del daemon de timestamps
├── branches              # Tracking de branches de Git
├── users                 # Información de desarrolladores
├── commands              # Comandos disponibles (CRUD admin)
├── instructions          # Instrucciones editables por admin
├── system_config         # Configuración global del sistema
├── database_connectors   # Conectores de DB configurables
└── health_endpoints      # URLs de health checks configurables
```

## 📊 Esquemas de Colecciones Principales

### 1. Collection: `projects`
**Propósito**: Registro maestro de proyectos donde bitacora está activo

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d1"),
  "project_id": "edgi_bitacora-rust-migration",  // user_id + "_" + project_slug
  "user_id": "edgi",
  "name": "Bitacora Rust Migration",
  "description": "Migration from bash to Rust+Axum architecture",
  "path": "/home/edgi/Documents/Development/own/bitacora",
  "git_remote": "https://github.com/edgi/bitacora.git",
  
  // Timestamps BSON para telemetría
  "created_at": ISODate("2025-08-21T10:30:00.000Z"),
  "updated_at": ISODate("2025-08-21T15:45:00.000Z"),
  "last_activity_at": ISODate("2025-08-21T15:45:00.000Z"),
  
  "status": "active",  // active, archived, paused, completed
  
  "config": {
    "auto_commit_enabled": true,
    "push_threshold": 10,
    "timestamp_interval": 60,
    "backup_enabled": true,
    "auto_topic_creation": false
  },
  
  "git_info": {
    // main_branches: Ramas permanentes/estables del repositorio
    "main_branches": ["main", "development", "staging"],
    // current_feature_branches: Ramas temporales con timestamp
    "current_feature_branches": [
      "20250821-1530_rust-migration",
      "20250820-1200_architecture-planning"
    ],
    "default_base_branch": "main"
  },
  
  // Telemetría agregada para ML
  "telemetry": {
    "total_sessions": 25,
    "total_actions": 145,
    "total_sparks": 8,
    "total_topics": 5,
    "avg_session_duration_minutes": 87.5,
    "total_development_hours": 156.3,
    "productivity_trend": "increasing",
    "most_productive_hours": [9, 10, 15, 16],
    "most_used_commands": ["ACTION", "START", "STATUS"]
  },
  
  // Metadatos del proyecto
  "metadata": {
    "primary_language": "rust",
    "framework": "axum",
    "project_type": "migration",
    "team_size": 1,
    "estimated_completion": ISODate("2025-09-15T00:00:00.000Z")
  }
}
```

**Índices de `projects`**:
```javascript
db.projects.createIndex({ "user_id": 1, "project_id": 1 }, { unique: true })
db.projects.createIndex({ "user_id": 1, "status": 1, "updated_at": -1 })
db.projects.createIndex({ "path": 1 })
db.projects.createIndex({ "git_remote": 1 })
db.projects.createIndex({ "telemetry.last_activity": -1 })
```

---

### 2. Collection: `sessions`
**Propósito**: Records de sesiones de trabajo completas

```javascript
// Documento de ejemplo - Estructura conceptual  
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d2"),
  "session_id": "edgi_20250821-1530_rust-migration",
  "project_id": "edgi_bitacora-rust-migration", 
  "user_id": "edgi",
  "branch_name": "20250821-1530_rust-migration",
  "description": "Convert bash scripts to Rust architecture with MongoDB",
  "status": "active",  // active, completed, paused, abandoned
  
  // Timestamps detallados para telemetría
  "created_at": ISODate("2025-08-21T15:30:00.000Z"),
  "updated_at": ISODate("2025-08-21T16:15:00.000Z"), 
  "session_start_at": ISODate("2025-08-21T15:30:00.000Z"),  // START command
  "session_end_at": null,  // END command (null si activa)
  "last_action_at": ISODate("2025-08-21T16:15:00.000Z"),   // Última ACTION
  
  // Telemetría de tiempo para ML futuro
  "time_tracking": {
    "estimated_duration_minutes": 120,
    "actual_duration_minutes": 45,  // Calculado: session_end_at - session_start_at
    "break_time_minutes": 5,        // Tiempo sin actividad > threshold
    "focused_time_minutes": 40,     // Tiempo productivo real
    "interruptions_count": 2,       // Pausas > break_threshold
    "productivity_score": 0.85      // Calculado: focused_time / actual_duration
  },
  
  // Checklist de la sesión
  "checklist": [
    {
      "id": "check_1",
      "text": "Design MongoDB schema",
      "completed": true,
      "created_at": ISODate("2025-08-21T15:30:00.000Z"),
      "completed_at": ISODate("2025-08-21T15:45:00.000Z"),
      "estimated_minutes": 30,
      "actual_minutes": 15
    },
    {
      "id": "check_2",
      "text": "Implement database layer", 
      "completed": false,
      "created_at": ISODate("2025-08-21T15:30:00.000Z"),
      "completed_at": null,
      "estimated_minutes": 60,
      "actual_minutes": null
    }
  ],
  
  // Resumen de sesión (se llena con END command)
  "summary": {
    "completion_percentage": 75,
    "total_actions": 12,
    "actions_by_type": {
      "fix": 4,
      "add": 3,
      "refactor": 2,
      "test": 2,
      "docs": 1
    },
    "completion_notes": "Successfully designed database schema and started implementation",
    "final_status": "completed",  // completed, pending, blocked
    "blockers": [],
    "next_steps": ["Implement repository pattern", "Add unit tests"]
  },
  
  // Información Git relacionada
  "git_info": {
    "initial_commit": "abc123def456",
    "final_commit": "def456abc123", 
    "total_commits": 3,
    "files_changed": ["src/lib.rs", "Cargo.toml", "docs/database.md"],
    "lines_added": 247,
    "lines_removed": 15,
    "auto_commits": 2,
    "manual_commits": 1
  },
  
  // Enlaces a otros elementos
  "linked_topic": "rust-migration",
  "linked_sparks": ["database-design-insight", "performance-optimization-idea"],
  "parent_session": null,  // Para sesiones que continúan trabajo previo
  "child_sessions": [],    // Para sesiones derivadas
  
  // Métricas de calidad
  "quality_metrics": {
    "estimated_accuracy": 0.73,  // estimated vs actual time
    "complexity_average": 6.2,   // Promedio de complexity_score de actions
    "satisfaction_score": 8.5,   // Auto-evaluación del usuario (1-10)
    "learning_progress": "high"  // low, medium, high
  },
  
  "tags": ["architecture", "database", "migration", "mongodb"]
}
```

**Índices de `sessions`**:
```javascript
db.sessions.createIndex({ "user_id": 1, "project_id": 1, "created_at": -1 })
db.sessions.createIndex({ "session_id": 1 }, { unique: true })
db.sessions.createIndex({ "user_id": 1, "branch_name": 1 })
db.sessions.createIndex({ "status": 1, "updated_at": -1 })
db.sessions.createIndex({ "linked_topic": 1 })
db.sessions.createIndex({ "time_tracking.actual_duration_minutes": 1 })
db.sessions.createIndex({ "quality_metrics.estimated_accuracy": -1 })
```

---

### 3. Collection: `actions`
**Propósito**: Log cronológico detallado de todas las acciones

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d3"),
  "action_id": "edgi_20250821-1545_database-schema-design",
  "session_id": "edgi_20250821-1530_rust-migration",
  "project_id": "edgi_bitacora-rust-migration",
  "user_id": "edgi",
  
  // Timestamps precisos para análisis de productividad
  "timestamp": "20250821-1545",  // Formato bitacora original
  "created_at": ISODate("2025-08-21T15:45:00.000Z"),      // Cuando se registró
  "action_start_at": ISODate("2025-08-21T15:40:00.000Z"),  // Cuando empezó tarea
  "action_end_at": ISODate("2025-08-21T15:45:00.000Z"),    // Cuando terminó tarea
  
  // Información de la acción
  "action_type": "design",  // session-start, session-end, fix, add, refactor, test, design, docs, research
  "title": "database-schema-design",
  "description": "Designed complete MongoDB schema with 7 collections for project management",
  
  // Telemetría de tiempo para ML
  "time_metrics": {
    "estimated_effort_minutes": 20,
    "actual_effort_minutes": 5,     // action_end_at - action_start_at
    "complexity_score": 7,          // 1-10 scale (usuario input)
    "satisfaction_score": 9,        // 1-10 usuario rating post-acción
    "difficulty_level": "medium",   // low, medium, high, expert
    "focus_level": 8,              // 1-10 qué tan enfocado estuvo
    "interruption_count": 0         // Cuántas veces se interrumpió
  },
  
  // Context técnico
  "context": {
    "branch": "20250821-1530_rust-migration",
    "files_involved": ["docs/database-schema.md", "src/models.rs"],
    "git_commit": "abc123def456",
    "auto_committed": true,
    "commit_message": "Add: database schema design documentation",
    "lines_changed": 89,
    "files_added": 1,
    "files_modified": 1,
    "files_deleted": 0
  },
  
  // Métricas de secuencia
  "sequence_metrics": {
    "action_sequence": 5,           // Orden dentro de la sesión
    "time_since_last_action": 300,  // Segundos desde acción anterior
    "actions_remaining_estimate": 7, // Cuántas acciones faltan (estimado)
    "session_progress": 0.42        // Progreso estimado de sesión (0-1)
  },
  
  // Enlaces y relaciones
  "related_actions": ["edgi_20250821-1530_session-start", "edgi_20250821-1542_architecture-analysis"],
  "generated_sparks": ["database-design-insight"],  // Si generó insights
  "blocked_by": [],                                  // Acciones que bloquean esta
  "blocks": [],                                     // Acciones que esta bloquea
  
  // Información de aprendizaje
  "learning_info": {
    "new_concepts": ["mongodb-indexing", "telemetry-design"],
    "techniques_used": ["domain-modeling", "performance-optimization"],
    "references_consulted": ["mongodb-docs", "rust-async-patterns"],
    "knowledge_level_before": "intermediate",
    "knowledge_level_after": "advanced"
  },
  
  // Metadata adicional
  "metadata": {
    "environment": "development",
    "tools_used": ["vscode", "mongodb-compass"],
    "mood": "focused",              // focused, tired, frustrated, excited
    "energy_level": 8,              // 1-10
    "confidence": 9                 // 1-10 en la solución implementada
  }
}
```

**Índices de `actions`**:
```javascript
db.actions.createIndex({ "user_id": 1, "session_id": 1, "created_at": 1 })
db.actions.createIndex({ "action_id": 1 }, { unique: true })
db.actions.createIndex({ "user_id": 1, "project_id": 1, "timestamp": 1 })
db.actions.createIndex({ "action_type": 1, "created_at": -1 })
db.actions.createIndex({ "time_metrics.complexity_score": 1 })
db.actions.createIndex({ "time_metrics.actual_effort_minutes": 1 })
db.actions.createIndex({ "sequence_metrics.action_sequence": 1 })
```

---

### 4. Collection: `topics`
**Propósito**: Temas de desarrollo a largo plazo

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d4"),
  "topic_id": "edgi_rust-migration",
  "project_id": "edgi_bitacora-rust-migration",
  "user_id": "edgi",
  "name": "Rust Migration", 
  "description": "Complete migration from bash scripts to Rust+Axum+MongoDB architecture",
  "status": "active",  // active, completed, archived, on-hold, blocked
  "priority": "high",  // low, medium, high, critical
  
  "created_at": ISODate("2025-08-21T10:00:00.000Z"),
  "updated_at": ISODate("2025-08-21T15:45:00.000Z"),
  "due_date": ISODate("2025-09-15T00:00:00.000Z"),
  "completed_at": null,
  
  // Contenido del topic
  "content": {
    "idea": "Replace bash-based bitacora with modern Rust architecture for better performance, type safety, and API integration",
    "motivation": "Current bash system works but lacks scalability, type safety, and modern integration capabilities",
    "objectives": [
      "Design SOLID architecture with crates",
      "Implement MongoDB persistence",
      "Create Axum HTTP API", 
      "Maintain backward compatibility",
      "Add comprehensive testing",
      "Document everything thoroughly"
    ],
    "acceptance_criteria": [
      "All bash functionality replicated in Rust",
      "API responds to curl commands", 
      "Data persisted in MongoDB",
      "Performance improved 10x",
      "Test coverage > 90%",
      "Documentation complete"
    ],
    "technical_requirements": [
      "Rust 1.75+",
      "MongoDB 7.0+", 
      "Axum web framework",
      "Tokio async runtime",
      "Docker for development"
    ]
  },
  
  // Tracking de progreso
  "progress": {
    "overall_percentage": 35,
    "objectives_completed": 2,
    "objectives_total": 6,
    "milestones": [
      {
        "name": "Architecture Design",
        "completed": true,
        "completed_at": ISODate("2025-08-21T15:45:00.000Z"),
        "weight": 20
      },
      {
        "name": "Database Schema", 
        "completed": true,
        "completed_at": ISODate("2025-08-21T15:45:00.000Z"),
        "weight": 15
      },
      {
        "name": "Core Implementation",
        "completed": false,
        "target_date": ISODate("2025-08-30T00:00:00.000Z"),
        "weight": 40
      }
    ]
  },
  
  // Estimaciones vs realidad
  "time_estimates": {
    "initial_estimate_hours": 40,
    "current_estimate_hours": 45,  // Se ajusta conforme progresa
    "actual_hours_spent": 12.5,
    "remaining_estimate_hours": 32.5,
    "accuracy_trend": "underestimating",  // underestimating, overestimating, accurate
    "adjustment_factor": 1.12  // Factor de ajuste para futuras estimaciones
  },
  
  // Relaciones con sesiones y acciones
  "sessions": [
    "edgi_20250821-1530_rust-migration",
    "edgi_20250820-1200_architecture-planning",
    "edgi_20250819-0900_research-phase"
  ],
  "total_actions": 47,
  "actions_by_type": {
    "research": 8,
    "design": 12, 
    "implement": 15,
    "test": 7,
    "docs": 5
  },
  
  // Información de dependencias
  "dependencies": {
    "blocks": ["api-design", "performance-optimization"],  // Topics que esperan este
    "blocked_by": [],                                      // Topics que bloquean este
    "related": ["database-optimization", "testing-strategy"],
    "prerequisites": ["rust-learning", "mongodb-basics"]
  },
  
  // Recursos y referencias
  "resources": {
    "documentation": [
      "https://doc.rust-lang.org/book/",
      "https://docs.rs/axum/latest/axum/",
      "https://www.mongodb.com/docs/"
    ],
    "repositories": [
      "https://github.com/tokio-rs/axum",
      "https://github.com/mongodb/mongo-rust-driver"
    ],
    "tutorials": ["Rust async programming", "MongoDB with Rust"],
    "experts": ["@rust-community", "@mongodb-developers"]
  },
  
  // Métricas de aprendizaje
  "learning_metrics": {
    "skill_level_start": "intermediate",
    "skill_level_current": "advanced", 
    "new_skills_acquired": ["async-rust", "mongodb-design", "axum-framework"],
    "knowledge_gaps": ["advanced-mongodb-operations", "rust-macros"],
    "confidence_level": 8,  // 1-10
    "learning_velocity": "high"
  },
  
  "tags": ["architecture", "migration", "rust", "mongodb", "api", "backend"],
  "category": "technical-upgrade"
}
```

**Índices de `topics`**:
```javascript
db.topics.createIndex({ "user_id": 1, "project_id": 1, "status": 1 })
db.topics.createIndex({ "topic_id": 1 }, { unique: true })
db.topics.createIndex({ "priority": 1, "due_date": 1 })
db.topics.createIndex({ "progress.overall_percentage": 1 })
db.topics.createIndex({ "tags": 1 })
```

---

### 5. Collection: `sparks`
**Propósito**: Insights brillantes, soluciones elegantes, observaciones clave

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d5"),
  "spark_id": "edgi_database-design-insight",
  "project_id": "edgi_bitacora-rust-migration", 
  "user_id": "edgi",
  "title": "MongoDB Schema Normalization Strategy",
  "description": "Discovered optimal balance between normalization and denormalization for bitacora data model",
  
  "created_at": ISODate("2025-08-21T15:45:00.000Z"),
  "updated_at": ISODate("2025-08-21T15:45:00.000Z"),
  
  "spark_type": "solution",  // solution, observation, pattern, optimization, question, discovery
  "category": "architecture", // architecture, performance, ux, algorithm, debugging, process
  "priority": "high",        // low, medium, high, game-changing
  "confidence": 9,           // 1-10 confidence in this insight
  
  // Contenido detallado del insight
  "content": {
    "problem": "How to structure MongoDB collections for bitacora without excessive data duplication while maintaining query performance",
    "solution": "Normalize by separating sessions and actions into different collections with references, but denormalize frequently accessed fields",
    "key_insight": "Actions reference sessions by session_id, but include denormalized project_id and user_id for efficient querying without joins",
    "impact": "Enables complex analytics queries, reduces storage by 40%, improves write performance, maintains read performance",
    "implementation": "Use separate collections with strategic denormalization of high-frequency query fields",
    "code_example": `
      // Actions collection includes denormalized fields
      {
        "action_id": "...",
        "session_id": "...",     // Reference to sessions
        "project_id": "...",     // Denormalized for direct queries
        "user_id": "...",        // Denormalized for user filtering
        // ... rest of action data
      }
    `,
    "lessons_learned": [
      "MongoDB normalization is not the same as SQL normalization",
      "Query patterns should drive schema design decisions", 
      "Strategic denormalization improves performance without significant storage cost",
      "Index design is crucial for query performance"
    ]
  },
  
  // Contexto donde se generó
  "origin": {
    "session_id": "edgi_20250821-1530_rust-migration",
    "action_id": "edgi_20250821-1545_database-schema-design", 
    "triggered_by": "user",  // user, ai, collaboration, research
    "trigger_event": "designing mongodb collections",
    "discussion_context": "Analyzing how to replace file-based storage with MongoDB while optimizing for analytics",
    "time_to_insight_minutes": 15  // Cuánto tomó llegar al insight
  },
  
  // Validación y aplicación
  "validation": {
    "tested": true,
    "test_results": "Query performance improved 3x, storage reduced 40%",
    "peer_reviewed": false,
    "reviewed_by": [],
    "feedback": [],
    "production_ready": true
  },
  
  // Aplicabilidad futura
  "applicability": {
    "reusable": true,
    "similar_contexts": [
      "any-mongodb-project",
      "microservices-data-design", 
      "analytics-optimized-schemas"
    ],
    "transferable_to": ["postgresql", "other-document-databases"],
    "limitations": ["High write volume scenarios", "Complex transaction requirements"],
    "adaptations_needed": "Adjust denormalization strategy based on specific query patterns"
  },
  
  // Métricas de impacto
  "impact_metrics": {
    "time_saved_hours": 8,        // Tiempo ahorrado por este insight
    "complexity_reduced": "high", // low, medium, high
    "performance_improvement": "3x faster queries",
    "cost_impact": "reduced storage costs by 40%",
    "team_impact": "improved development velocity",
    "future_applications": 3      // Cuántas veces se puede reusar
  },
  
  // Enlaces a elementos relacionados
  "related_sparks": ["performance-optimization-insight", "indexing-strategy-discovery"],
  "influenced_actions": ["database-implementation", "schema-optimization"],
  "influenced_topics": ["database-optimization", "performance-tuning"],
  
  // Categorización para ML futuro
  "ml_features": {
    "domain": "database-design",
    "complexity_level": "intermediate",
    "novelty_score": 7,           // 1-10 qué tan novel es el insight
    "utility_score": 9,           // 1-10 qué tan útil es
    "generalizability": 8,        // 1-10 qué tan generalizable es
    "keywords": ["mongodb", "schema-design", "normalization", "performance", "analytics"]
  },
  
  "status": "validated",  // draft, validated, applied, archived, superseded
  "tags": ["mongodb", "schema-design", "normalization", "performance", "best-practices"]
}
```

**Índices de `sparks`**:
```javascript
db.sparks.createIndex({ "user_id": 1, "category": 1, "created_at": -1 })
db.sparks.createIndex({ "spark_id": 1 }, { unique: true })
db.sparks.createIndex({ "spark_type": 1, "priority": 1 })
db.sparks.createIndex({ "tags": 1 })
db.sparks.createIndex({ "applicability.reusable": 1, "impact_metrics.utility_score": -1 })
db.sparks.createIndex({ "ml_features.domain": 1, "ml_features.utility_score": -1 })
```

---

## 🚀 Consultas de Ejemplo Frecuentes

### Analytics de Productividad
```javascript
// Productividad por usuario en último mes
db.sessions.aggregate([
  { $match: { 
    "user_id": "edgi",
    "created_at": { $gte: ISODate("2025-07-21T00:00:00.000Z") }
  }},
  { $group: {
    "_id": "$user_id",
    "total_sessions": { $sum: 1 },
    "total_hours": { $sum: "$time_tracking.focused_time_minutes" },
    "avg_productivity": { $avg: "$time_tracking.productivity_score" }
  }}
])
```

### Sesiones por proyecto
```javascript
// Últimas sesiones de un proyecto
db.sessions.find({
  "project_id": "edgi_bitacora-rust-migration",
  "status": "completed"
}).sort({ "created_at": -1 }).limit(10)
```

### Insights más valiosos
```javascript
// Sparks con mayor impacto
db.sparks.find({
  "user_id": "edgi",
  "status": "validated"
}).sort({ 
  "impact_metrics.utility_score": -1,
  "ml_features.generalizability": -1 
}).limit(5)
```

---

**Próximo documento**: `02_collections_schema.md` - Esquemas detallados de colecciones administrativas
