# Sistema de Administración Completo - Bitacora V1.0

## 🎯 Visión del Sistema Administrativo

El sistema administrativo de Bitacora V1.0 permite **configuración completa sin código**, **gestión dinámica de comandos**, **monitoreo avanzado** y **control total sobre conectores de base de datos**. Todo cambio es auditable y reversible.

## 🏗️ Arquitectura Administrativa

### Principios de Diseño
1. **Configuration as Data**: Toda configuración vive en la base de datos
2. **Hot Reconfiguration**: Cambios sin reiniciar el sistema 
3. **Auditabilidad Total**: Todo cambio queda registrado con quien, cuándo y por qué
4. **Rollback Capability**: Cualquier configuración puede revertirse
5. **Validación Robusta**: Configuración inválida es rechazada antes de aplicarse
6. **Permissions System**: Control granular de quién puede cambiar qué

### Componentes Administrativos
```
Administration System
├── Command Management      # CRUD de comandos disponibles
├── Instruction Management  # Gestión de documentación de comandos
├── System Configuration   # Configuración global del sistema
├── Database Connectors    # Gestión de conectores de BD
├── Health Monitoring      # Sistema de monitoreo y alertas
├── User Management        # Gestión de usuarios y permisos
└── Audit System           # Logging y tracking de cambios
```

## 🛠️ Colecciones Administrativas Detalladas

### 1. Collection: `commands`
**Propósito**: Registro y gestión de todos los comandos disponibles

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d6"),
  "command_id": "START",
  "command_name": "START", 
  "display_name": "Start Work Session",
  "description": "Initialize a work session ensuring all bitácora components are ready and functional",
  "version": "2.1.0",
  "category": "session_management",  // session_management, git_operations, project_management, system_operations
  
  // Timestamps de gestión
  "created_at": ISODate("2025-08-21T10:00:00.000Z"),
  "updated_at": ISODate("2025-08-21T16:00:00.000Z"),
  "created_by": "system_migration",
  "updated_by": "admin_edgi",
  
  "status": "active",  // active, deprecated, maintenance, disabled, testing
  "priority": 1,       // 1 = highest, 10 = lowest (for UI ordering)
  
  // Configuración operacional del comando
  "operational_config": {
    "requires_git_repo": true,
    "requires_active_project": true,
    "can_run_parallel": false,      // ¿Puede ejecutarse concurrentemente?
    "timeout_seconds": 300,
    "retry_attempts": 3,
    "requires_confirmation": false,  // ¿Requiere confirmación del usuario?
    "destructive_operation": false,  // ¿Puede causar pérdida de datos?
    "minimum_permissions": ["user"] // Permisos mínimos requeridos
  },
  
  // Parámetros que acepta el comando  
  "parameters": [
    {
      "name": "description",
      "type": "string",
      "required": false,
      "default_value": null,
      "validation_regex": "^[a-zA-Z0-9\\s\\-_]{0,100}$",
      "description": "Optional description for the session",
      "examples": ["rust migration work", "bug fixes", "feature development"]
    },
    {
      "name": "estimated_minutes", 
      "type": "integer",
      "required": false,
      "default_value": 60,
      "min_value": 5,
      "max_value": 480,
      "description": "Estimated duration in minutes"
    }
  ],
  
  // Mapeo a implementación
  "execution": {
    "handler_type": "rust_function",  // rust_function, shell_script, http_endpoint, plugin
    "handler_path": "bitacora_commands::session::start_session_handler",
    "fallback_script": "scripts/start_session.sh",  // Backward compatibility
    "pre_execution_hooks": [
      "validate_timestamp_daemon",
      "check_git_repository",
      "ensure_project_exists"
    ],
    "post_execution_hooks": [
      "update_session_metrics",
      "log_session_start",
      "notify_integrations"
    ]
  },
  
  // Referencias a documentación
  "documentation": {
    "instruction_id": "START_INSTRUCTION",
    "help_text": "Start a new work session. This will initialize the timestamp daemon and create a session record.",
    "examples": [
      {
        "title": "Basic session start",
        "command": "START",
        "description": "Starts a session on current branch"
      },
      {
        "title": "Session with description",
        "command": "START rust migration work",
        "description": "Starts a session with specific description"
      }
    ]
  },
  
  // Configuración de validación
  "validation_rules": [
    {
      "rule_type": "git_repository",
      "error_message": "Command requires active git repository",
      "blocking": true
    },
    {
      "rule_type": "timestamp_daemon",
      "error_message": "Timestamp daemon must be running",
      "blocking": true
    },
    {
      "rule_type": "active_session",
      "condition": "none_exists",
      "error_message": "Another session is already active",
      "blocking": true
    }
  ],
  
  // Permisos y restricciones
  "permissions": {
    "required_roles": ["user", "developer"],
    "forbidden_roles": ["readonly"],
    "rate_limiting": {
      "max_calls_per_hour": 20,
      "max_calls_per_day": 100,
      "burst_limit": 5
    },
    "time_restrictions": {
      "allowed_hours": null,  // null = 24/7, array for specific hours
      "forbidden_days": []    // Days when command is not allowed
    }
  },
  
  // Telemetría del comando
  "usage_statistics": {
    "total_executions": 1247,
    "successful_executions": 1220,
    "failed_executions": 27,
    "success_rate": 0.9783,
    "avg_execution_time_ms": 2340,
    "last_executed_at": ISODate("2025-08-21T15:45:00.000Z"),
    "last_failure_at": ISODate("2025-08-20T14:32:00.000Z"),
    "common_failure_reasons": [
      {"reason": "timestamp_daemon_not_running", "count": 15},
      {"reason": "git_not_initialized", "count": 8},
      {"reason": "invalid_parameters", "count": 4}
    ]
  },
  
  // Configuración de UI/UX
  "ui_config": {
    "icon": "🚀",
    "color": "#28a745",  // Verde para acción positiva
    "keyboard_shortcut": "Ctrl+Shift+S",
    "show_in_quick_actions": true,
    "confirm_dialog": null,  // null or dialog config
    "progress_indicator": true
  },
  
  // Versionado y changelog
  "version_history": [
    {
      "version": "1.0.0",
      "created_at": ISODate("2025-08-20T10:00:00.000Z"),
      "changes": "Initial version migrated from bash script",
      "author": "migration_script",
      "breaking_changes": false
    },
    {
      "version": "2.1.0",
      "created_at": ISODate("2025-08-21T16:00:00.000Z"),
      "changes": "Added database persistence and improved error handling",
      "author": "admin_edgi",
      "breaking_changes": false
    }
  ]
}
```

**Comandos Disponibles por Defecto**:
```javascript
// Lista de comandos estándar de Bitacora
[
  {
    "command_id": "START",
    "display_name": "Start Session",
    "category": "session_management"
  },
  {
    "command_id": "ACTION", 
    "display_name": "Log Action",
    "category": "session_management"
  },
  {
    "command_id": "BRANCH",
    "display_name": "Create Branch", 
    "category": "git_operations"
  },
  {
    "command_id": "TOPIC",
    "display_name": "Create Topic",
    "category": "project_management"
  },
  {
    "command_id": "STATUS",
    "display_name": "Show Status",
    "category": "system_operations"
  },
  {
    "command_id": "BACKUP",
    "display_name": "Backup Project",
    "category": "system_operations"
  },
  {
    "command_id": "END",
    "display_name": "End Session",
    "category": "session_management"
  }
]
```

---

### 2. Collection: `instructions`
**Propósito**: Documentación editable de comandos

```javascript
// Documento de ejemplo - Estructura conceptual
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d7"),
  "instruction_id": "START_INSTRUCTION",
  "command_id": "START",
  "title": "START Command - Complete Guide",
  "version": "3.0.0",
  "language": "en",  // Para futuro soporte i18n
  
  "created_at": ISODate("2025-08-21T10:00:00.000Z"),
  "updated_at": ISODate("2025-08-21T16:00:00.000Z"),
  "created_by": "system_migration",
  "updated_by": "admin_edgi",
  
  "status": "active",  // active, draft, archived, under_review
  "category": "session_management",
  
  // Contenido estructurado de la instrucción
  "content": {
    "objective": "Initialize a work session ensuring all bitácora components are ready and functional, including database connectivity and timestamp synchronization.",
    
    "overview": "The START command is the entry point for any development session in Bitacora. It performs system health checks, initializes required services, and creates a new session record in the database.",
    
    "prerequisites": [
      "Git repository must be initialized",
      "Database connection must be available", 
      "User must have valid authentication",
      "No other active session in current project"
    ],
    
    "steps": [
      {
        "step_number": 1,
        "action": "System Health Check",
        "description": "Verify database connectivity and system components",
        "technical_details": "Checks MongoDB connection, validates user permissions, ensures timestamp service availability"
      },
      {
        "step_number": 2, 
        "action": "Git Repository Validation",
        "description": "Validate git repository state and current branch",
        "technical_details": "Runs 'git rev-parse --is-inside-work-tree' and 'git rev-parse --abbrev-ref HEAD'"
      },
      {
        "step_number": 3,
        "action": "Session Creation",
        "description": "Create new session record in database",
        "technical_details": "Inserts document into sessions collection with session_id, timestamps, and initial checklist"
      },
      {
        "step_number": 4,
        "action": "Initial Action Log", 
        "description": "Register session-start action",
        "technical_details": "Creates action record with type 'session-start' and links to session"
      },
      {
        "step_number": 5,
        "action": "Status Display",
        "description": "Show current project status to user",
        "technical_details": "Displays active session info, current branch, and project statistics"
      }
    ],
    
    "success_criteria": [
      "Session record created in database",
      "Session-start action logged", 
      "User informed of session details",
      "All system components verified as operational"
    ],
    
    "parameters": [
      {
        "name": "description",
        "type": "string",
        "optional": true,
        "description": "Optional description for the work session",
        "examples": ["database migration", "bug fixes", "feature development"],
        "validation": "Must be 1-100 characters, alphanumeric and spaces only"
      }
    ],
    
    "examples": [
      {
        "title": "Basic session start",
        "command": "START",
        "description": "Starts a session with default settings",
        "expected_output": "🚀 Session started for branch: main\nSession ID: edgi_20250821-1530_main\nReady for development!"
      },
      {
        "title": "Session with description", 
        "command": "START database migration work",
        "description": "Starts a session with specific work description",
        "expected_output": "🚀 Session started for branch: main\nDescription: database migration work\nSession ID: edgi_20250821-1530_main"
      }
    ],
    
    "error_scenarios": [
      {
        "error": "Database connection failed",
        "cause": "MongoDB service not running or connection string invalid",
        "solution": "Check database service status and configuration",
        "recovery": "Use fallback SQLite connector or restart database service"
      },
      {
        "error": "Active session already exists",
        "cause": "Another session is currently active for this project",
        "solution": "End current session with END command before starting new one",
        "recovery": "Use 'STATUS' to see current session details"
      },
      {
        "error": "Git repository not found",
        "cause": "Current directory is not a git repository",
        "solution": "Initialize git repository with 'git init' or navigate to valid repository",
        "recovery": "Bitacora requires git repository for session tracking"
      }
    ],
    
    "troubleshooting": [
      {
        "symptom": "Command hangs or times out",
        "possible_causes": ["Database connection slow", "Network issues", "System overload"],
        "diagnostic_steps": ["Check database connectivity", "Verify network status", "Check system resources"],
        "solutions": ["Restart database service", "Check network configuration", "Free up system resources"]
      }
    ],
    
    "related_commands": [
      {"command": "STATUS", "relationship": "Shows current session details"},
      {"command": "END", "relationship": "Complements START by closing session"},
      {"command": "ACTION", "relationship": "Used after START to log work actions"}
    ],
    
    "technical_notes": [
      "Session IDs follow format: {user_id}_{timestamp}_{branch_description}",
      "Timestamp format is YYYYMMDD-HHMM based on system daemon",
      "Database operations are atomic - either all succeed or all rollback",
      "Command execution time typically < 2 seconds in normal conditions"
    ]
  },
  
  // Metadata del editor
  "editor_metadata": {
    "last_editor": "admin_edgi",
    "edit_reason": "Updated to reflect database persistence changes and new error scenarios",
    "review_required": false,
    "review_assignee": null,
    "approved_by": "senior_admin",
    "approved_at": ISODate("2025-08-21T16:00:00.000Z"),
    "editing_time_minutes": 45,
    "word_count": 847
  },
  
  // Configuración de presentación
  "presentation_config": {
    "format": "markdown",  // markdown, html, plain_text
    "syntax_highlighting": true,
    "code_examples_runnable": true,
    "interactive_elements": false,
    "estimated_read_time_minutes": 8
  },
  
  // Métricas de uso de la documentación
  "usage_metrics": {
    "views_total": 156,
    "views_last_30_days": 23,
    "helpful_votes": 18,
    "unhelpful_votes": 2,
    "improvement_suggestions": [
      {"user": "dev_user_1", "suggestion": "Add video tutorial link"},
      {"user": "dev_user_2", "suggestion": "More examples with different scenarios"}
    ]
  },
  
  // Versionado completo
  "version_history": [
    {
      "version": "1.0.0",
      "created_at": ISODate("2025-08-20T10:00:00.000Z"),
      "changes": "Initial version migrated from START.md file",
      "author": "migration_script",
      "content_hash": "abc123...",
      "word_count": 423
    },
    {
      "version": "2.0.0",
      "created_at": ISODate("2025-08-21T14:00:00.000Z"),
      "changes": "Major update for database integration",
      "author": "admin_edgi",
      "content_hash": "def456...",
      "word_count": 672
    },
    {
      "version": "3.0.0",
      "created_at": ISODate("2025-08-21T16:00:00.000Z"), 
      "changes": "Added error scenarios and troubleshooting section",
      "author": "admin_edgi",
      "content_hash": "ghi789...",
      "word_count": 847
    }
  ]
}
```

---

### 3. Collection: `system_config`
**Propósito**: Configuración global administrable del sistema

```javascript
// Documento de ejemplo - Database Connector Configuration
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d8"),
  "config_key": "database_primary_connector",
  "config_category": "infrastructure",
  "config_type": "database_connector",
  "display_name": "Primary Database Connector",
  "description": "Main database connector configuration for Bitacora system",
  
  "created_at": ISODate("2025-08-21T10:00:00.000Z"),
  "updated_at": ISODate("2025-08-21T16:00:00.000Z"),
  "created_by": "system_setup",
  "updated_by": "admin_edgi",
  
  "status": "active",  // active, pending_approval, disabled, deprecated
  "environment": "production",  // development, testing, staging, production
  "critical": true,    // Critical configs require approval workflow
  
  // Valor actual de la configuración
  "current_value": {
    "connector_type": "mongodb",
    "priority": 1,  // 1 = primary, 2 = secondary, etc.
    "connection_config": {
      "host": "localhost",
      "port": 27017,
      "database": "bitacora_db",
      "username": "bitacora_user",
      "password_reference": "BITACORA_MONGO_PASSWORD",  // Reference to secure vault
      "connection_pool": {
        "min_connections": 2,
        "max_connections": 10,
        "idle_timeout_seconds": 300,
        "connection_timeout_seconds": 30
      },
      "ssl_config": {
        "enabled": true,
        "verify_certificates": true,
        "certificate_path": "/etc/bitacora/certs/mongodb.pem"
      },
      "options": {
        "read_preference": "primary",
        "write_concern": "majority",
        "read_timeout_ms": 5000,
        "write_timeout_ms": 5000
      }
    },
    "fallback_config": {
      "connector_type": "sqlite",
      "connection_config": {
        "file_path": "/var/lib/bitacora/fallback.db",
        "journal_mode": "WAL",
        "synchronous": "NORMAL",
        "cache_size": 10000
      },
      "auto_switch_conditions": [
        "primary_connector_failures > 3",
        "primary_response_time > 5000ms"
      ]
    },
    "health_monitoring": {
      "enabled": true,
      "check_interval_seconds": 30,
      "timeout_seconds": 10,
      "failure_threshold": 3,
      "recovery_threshold": 2,
      "alert_on_failure": true
    }
  },
  
  // Validación de configuración
  "validation_schema": {
    "type": "object",
    "required": ["connector_type", "connection_config"],
    "properties": {
      "connector_type": {
        "type": "string",
        "enum": ["mongodb", "postgresql", "sqlite", "mysql"]
      },
      "connection_config": {
        "type": "object",
        "required": ["host", "port", "database"],
        "properties": {
          "host": {"type": "string", "minLength": 1},
          "port": {"type": "integer", "minimum": 1, "maximum": 65535}
        }
      }
    }
  },
  
  "validation_result": {
    "is_valid": true,
    "last_validated_at": ISODate("2025-08-21T16:00:00.000Z"),
    "validation_errors": [],
    "validation_warnings": [
      "SSL verification disabled in development environment"
    ]
  },
  
  // Permisos y workflow de aprobación
  "permissions": {
    "read_roles": ["admin", "operator", "developer"],
    "write_roles": ["admin", "senior_admin"],
    "approval_required": true,
    "approvers": ["senior_admin", "infrastructure_admin"],
    "requires_maintenance_window": true
  },
  
  "approval_workflow": {
    "current_status": "approved",
    "requested_by": "admin_edgi",
    "requested_at": ISODate("2025-08-21T15:30:00.000Z"),
    "approved_by": "senior_admin",
    "approved_at": ISODate("2025-08-21T16:00:00.000Z"),
    "approval_reason": "Migration to production MongoDB cluster",
    "approval_comments": "Validated connection and performance tests passed"
  },
  
  // Historial de cambios
  "change_history": [
    {
      "timestamp": ISODate("2025-08-21T16:00:00.000Z"),
      "changed_by": "admin_edgi",
      "change_type": "update",
      "old_value": {
        "connector_type": "sqlite",
        "connection_config": {"file_path": "/tmp/bitacora.db"}
      },
      "new_value": {
        "connector_type": "mongodb",
        "connection_config": {"host": "localhost", "port": 27017}
      },
      "reason": "Migration to MongoDB for better scalability and features",
      "impact_assessment": "Improved performance and scalability, backward compatible",
      "rollback_plan": "Switch back to SQLite connector if issues occur",
      "tested_in_environment": "staging"
    }
  ],
  
  // Información de rollback
  "rollback_info": {
    "rollback_available": true,
    "previous_version": {
      "config_value": {"connector_type": "sqlite"},
      "version_timestamp": ISODate("2025-08-20T10:00:00.000Z")
    },
    "rollback_procedure": "Switch database_primary_connector back to sqlite configuration",
    "rollback_impact": "Temporary downtime during connection switchover"
  },
  
  // Métricas de uso
  "usage_metrics": {
    "configuration_applied_at": ISODate("2025-08-21T16:00:00.000Z"),
    "last_accessed_at": ISODate("2025-08-21T16:15:00.000Z"),
    "access_count": 23,
    "modification_count": 3,
    "performance_impact": "positive",
    "error_count_since_change": 0
  }
}

// Otro ejemplo - Health Check Configuration
{
  "_id": ObjectId("64f1a2b3c4d5e6f7a8b9c0d9"),
  "config_key": "system_health_monitoring",
  "config_category": "monitoring", 
  "display_name": "System Health Monitoring Configuration",
  
  "current_value": {
    "enabled": true,
    "global_health_endpoint": "https://api.bitacora.internal/health",
    "check_interval_seconds": 30,
    "timeout_seconds": 10,
    "retry_attempts": 3,
    
    "endpoints": [
      {
        "name": "primary_api",
        "url": "https://api.bitacora.internal/health",
        "method": "GET",
        "expected_status": 200,
        "expected_response": {"status": "healthy"},
        "critical": true
      },
      {
        "name": "database_health",
        "url": "https://api.bitacora.internal/health/database",
        "method": "GET", 
        "expected_status": 200,
        "critical": true
      }
    ],
    
    "failure_actions": [
      {
        "action": "log_error",
        "config": {"level": "error", "include_response": true}
      },
      {
        "action": "send_alert",
        "config": {
          "channels": ["email", "slack"],
          "recipients": ["admin@bitacora.com"],
          "escalation_minutes": 15
        }
      },
      {
        "action": "enable_fallback_mode", 
        "config": {"fallback_connector": "sqlite"}
      }
    ],
    
    "recovery_actions": [
      {
        "action": "log_recovery",
        "config": {"level": "info"}
      },
      {
        "action": "send_recovery_notification",
        "config": {"channels": ["slack"]}
      }
    ]
  }
}
```

---

## 🔧 Endpoints Administrativos Completos

### Command Management APIs
```rust
// Ejemplo conceptual de endpoints - NO código funcional

// CRUD de comandos
POST   /admin/api/v1/commands                    
// Create new command
// Body: { "command_id": "CUSTOM", "display_name": "...", ... }

GET    /admin/api/v1/commands                    
// List all commands with filtering
// Query: ?status=active&category=session_management

GET    /admin/api/v1/commands/{command_id}       
// Get specific command details

PUT    /admin/api/v1/commands/{command_id}       
// Update command configuration
// Body: { "version": "1.1.0", "status": "active", ... }

DELETE /admin/api/v1/commands/{command_id}      
// Disable/archive command (soft delete)

POST   /admin/api/v1/commands/{command_id}/test  
// Test command execution in safe mode

GET    /admin/api/v1/commands/{command_id}/usage-stats
// Get command usage statistics and performance metrics
```

### System Configuration APIs  
```rust
// Ejemplo conceptual de endpoints - NO código funcional

GET    /admin/api/v1/config                      
// List all configuration keys

GET    /admin/api/v1/config/{config_key}         
// Get specific configuration

PUT    /admin/api/v1/config/{config_key}         
// Update configuration value
// Body: { "value": {...}, "reason": "...", "tested_in": "staging" }

POST   /admin/api/v1/config/{config_key}/validate
// Validate configuration without applying
// Body: { "value": {...} }

POST   /admin/api/v1/config/{config_key}/rollback
// Rollback to previous configuration version

GET    /admin/api/v1/config/{config_key}/history 
// Get configuration change history
```

### Database Connector Management
```rust 
// Ejemplo conceptual de endpoints - NO código funcional

GET    /admin/api/v1/database/connectors         
// List all configured connectors

POST   /admin/api/v1/database/connectors         
// Add new database connector
// Body: { "connector_id": "backup_postgres", ... }

PUT    /admin/api/v1/database/connectors/{id}/activate
// Switch active database connector
// Body: { "reason": "...", "maintenance_window": true }

GET    /admin/api/v1/database/health             
// Get health status of all connectors

POST   /admin/api/v1/database/connectors/{id}/test
// Test connector connection without switching

GET    /admin/api/v1/database/performance        
// Get performance metrics of active connector
```

### Health Monitoring Management
```rust
// Ejemplo conceptual de endpoints - NO código funcional

GET    /admin/api/v1/health/endpoints            
// List all health check endpoints

POST   /admin/api/v1/health/endpoints            
// Add new health check endpoint
// Body: { "url": "...", "method": "GET", ... }

PUT    /admin/api/v1/health/endpoints/{id}       
// Update health check configuration

POST   /admin/api/v1/health/endpoints/{id}/check 
// Execute manual health check

GET    /admin/api/v1/health/status               
// Get overall system health status

GET    /admin/api/v1/health/history              
// Get health check history and trends
```

---

## 🔐 Sistema de Permisos y Roles

### Roles Administrativos
```javascript
// Roles del sistema con permisos granulares
{
  "roles": [
    {
      "role_id": "super_admin",
      "display_name": "Super Administrator", 
      "permissions": ["*"],  // All permissions
      "description": "Full system access including user management"
    },
    {
      "role_id": "admin",
      "display_name": "System Administrator",
      "permissions": [
        "commands.read", "commands.write", "commands.delete",
        "config.read", "config.write", 
        "database.read", "database.write",
        "health.read", "health.write",
        "audit.read"
      ]
    },
    {
      "role_id": "operator",
      "display_name": "System Operator",
      "permissions": [
        "commands.read", 
        "config.read",
        "database.read", "database.health", 
        "health.read", "health.check"
      ]
    },
    {
      "role_id": "developer",
      "display_name": "Developer", 
      "permissions": [
        "commands.read",
        "config.read",
        "health.read"
      ]
    }
  ]
}
```

### Workflow de Aprobación
```javascript
// Sistema de aprobación para cambios críticos
{
  "approval_workflows": [
    {
      "workflow_id": "critical_config_change",
      "triggers": [
        "config.database_connector.*",
        "config.system_security.*", 
        "commands.*.execution.handler_path"
      ],
      "approval_chain": [
        {"role": "admin", "required_approvals": 1},
        {"role": "senior_admin", "required_approvals": 1}
      ],
      "timeout_hours": 24,
      "auto_rollback_on_timeout": true
    }
  ]
}
```

---

## 📊 Sistema de Auditoría

### Audit Log Structure
```javascript
// Collection: audit_logs
{
  "_id": ObjectId("..."),
  "event_id": "edgi_20250821-1600_config_change",
  "timestamp": ISODate("2025-08-21T16:00:00.000Z"),
  "user_id": "admin_edgi",
  "action": "update_configuration",
  "resource_type": "system_config",
  "resource_id": "database_primary_connector",
  
  "details": {
    "old_value": {"connector_type": "sqlite"},
    "new_value": {"connector_type": "mongodb"},
    "reason": "Migration to production database",
    "approval_id": "approval_20250821_001"
  },
  
  "context": {
    "ip_address": "192.168.1.100",
    "user_agent": "Mozilla/5.0...",
    "api_endpoint": "/admin/api/v1/config/database_primary_connector",
    "session_id": "admin_session_123"
  },
  
  "impact": {
    "affected_users": ["all"],
    "system_restart_required": false,
    "data_migration_required": false,
    "estimated_downtime_seconds": 30
  }
}
```

---

**Próximo documento**: `02_commands_crud.md` - Detalles del sistema CRUD de comandos
