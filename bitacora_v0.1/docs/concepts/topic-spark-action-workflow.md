# Flujo de Trabajo: PROJECT, TOPIC, SPARK y ACTION en Bitacora

## Visión General

Bitacora utiliza cuatro conceptos fundamentales organizados jerárquicamente para estructurar, rastrear y documentar el trabajo de desarrollo: **PROJECT** → **TOPIC** → **ACTION** y **SPARK**. Estos elementos trabajan en conjunto para crear un sistema completo de gestión del conocimiento y trazabilidad del desarrollo.

## Arquitectura Conceptual

La información fluye en Bitacora a través de dos modalidades principales:

### Flujo Interactivo (Usuario ↔ Bitacora ↔ AI)
- Usuario interactúa directamente con Bitacora
- AI asiste en tiempo real durante el desarrollo
- Captura inmediata de ACTIONs, TOPICs y SPARKs

### Flujo Automático (Bitacora → Compass → AI → Routier → Bitacora)
- Compass analiza datos históricos sin intervención del usuario
- Genera documentación estructurada (Routiers) 
- Alimenta insights de vuelta al sistema
- Produce "cartas de navegación" para futuros desarrollos

## Definiciones Fundamentales

### PROJECT (Proyecto de Desarrollo)
Un **PROJECT** es la unidad organizacional de más alto nivel en Bitacora. Representa un esfuerzo de desarrollo completo con objetivos, recursos, timeline y entregables específicos.

**Características:**
- Almacenado en la colección `projects` de MongoDB
- Identificado por `project_id` único (formato: `user_id + "_" + project_slug`)
- Agrupa múltiples TOPICs, sesiones y acciones bajo un mismo objetivo macro
- Incluye configuración, métricas agregadas, información Git y metadatos del proyecto
- Permite tracking de progreso general, productividad y telemetría

**Estructura jerárquica:**
```
PROJECT "Bitacora Rust Migration"
├── TOPIC "Architecture Design" (completado)
├── TOPIC "Database Implementation" (completado) 
├── TOPIC "API Development" (activo)
├── TOPIC "Performance Optimization" (futuro)
└── TOPIC "Documentation" (en progreso)
```

**Tipos de PROJECT:**
- **PROJECT Activo**: En desarrollo actual, con TOPICs activos
- **PROJECT Completado**: Finalizado, con histórico completo
- **PROJECT Archivado**: Pausado o cancelado
- **PROJECT Ideas**: Pool de ideas para futuros proyectos

### TOPIC (Tema de Desarrollo)
Un **TOPIC** es un tema de desarrollo a largo plazo dentro de un PROJECT. Representa una línea de trabajo, objetivo, área de investigación o feature importante que agrupa múltiples sesiones, acciones y recursos relacionados.

**Características:**
- Almacenado en la colección `topics` de MongoDB
- Identificado por `topic_id` único
- Siempre relacionado con un PROJECT (`project_id`) y usuario (`user_id`)
- Tiene estados: `active`, `completed`, `archived`, `on-hold`, `blocked`
- Incluye métricas de progreso, estimaciones de tiempo y dependencias
- Permite tracking de objetivos, milestones y recursos

**Relación con PROJECT:**
- Un PROJECT puede tener múltiples TOPICs
- Solo un TOPIC puede estar `"active"` por PROJECT a la vez
- Los TOPICs pueden tener dependencias entre sí dentro del mismo PROJECT
- Los TOPICs heredan configuración y contexto del PROJECT padre

**Tipos de TOPIC dentro de un PROJECT:**
- **TOPIC Activo**: En desarrollo actual, estado `"active"`
- **TOPIC del Proyecto**: Relacionado con el proyecto pero no activo (`"on-hold"`, `"archived"`)
- **TOPIC de Funcionalidad Futura**: Para desarrollo posterior dentro del proyecto
- **TOPIC de Ideas**: Capturan ideas relacionadas con el proyecto para evaluación futura

### ACTION (Acción Individual)
Un **ACTION** es una acción individual registrada durante una sesión de trabajo. Representa la unidad mínima de actividad, decisión, cambio o evento relevante que ocurre durante el desarrollo.

**Características:**
- Almacenado en la colección `actions` de MongoDB
- Identificado por `action_id` único
- Siempre relacionado con una sesión (`session_id`)
- Puede estar vinculado a un TOPIC (`topic_id`)
- Incluye timestamp, tipo, descripción y métricas
- Permite trazabilidad granular del trabajo realizado

### SPARK (Insight/Descubrimiento)
Un **SPARK** es un insight, observación brillante, solución elegante o descubrimiento clave generado durante el desarrollo. Captura conocimiento valioso, patrones, optimizaciones y lecciones aprendidas.

**Características:**
- Almacenado en la colección `sparks` de MongoDB
- Identificado por `spark_id` único
- Vinculado a proyectos, sesiones, acciones y topics
- Incluye problema, solución, contexto, validación y aplicabilidad futura
- Categorizado por tipo, prioridad y métricas de impacto
- Facilita la reutilización de conocimiento

## Flujo de Trabajo Detallado

### Escenario 1: ACTION Relacionada con TOPIC Activo

**Proceso Actual (Sin Cambios):**
1. Usuario realiza una actividad dentro del contexto del TOPIC activo
2. Se registra el ACTION con:
   - `session_id`: Sesión actual
   - `topic_id`: TOPIC activo
   - `type`, `description`, `timestamp`: Detalles de la acción
3. El ACTION se vincula automáticamente al TOPIC activo
4. Se actualiza el progreso y métricas del TOPIC activo
5. Si se genera un insight, se puede crear un SPARK vinculado

**Ejemplo:**
```
PROJECT Activo: "Bitacora Rust Migration" (edgi_bitacora-rust-migration)
└── TOPIC Activo: "API Development" (edgi_api-development)
    └── ACTION: "Implemented Axum authentication middleware"
        → Se registra directamente en el TOPIC activo del PROJECT activo
```

### Escenario 2: TOPIC Relacionado con Proyecto pero No con TOPIC Activo

**Nuevo Proceso Propuesto:**

#### Paso 1: Detección de Nuevo TOPIC
- Durante el trabajo en el TOPIC activo, surge la necesidad de un nuevo tema
- Puede ser una funcionalidad futura, optimización, o área de investigación relacionada con el proyecto actual

#### Paso 2: Creación del Nuevo TOPIC
- Se crea un nuevo documento en la colección `topics` con:
  - `project_id`: Mismo proyecto actual (mantiene contexto del PROJECT)
  - `status`: `"on-hold"` o `"archived"` (no activo inmediatamente)
  - `priority`: Según relevancia (`"low"`, `"medium"`, `"high"`)
  - `created_at`: Timestamp actual
  - `content.idea`: Descripción del nuevo tema
  - `origin_context`: Referencia al TOPIC activo donde surgió
  - `dependencies.blocked_by`: Puede incluir el TOPIC activo si es prerequisito

#### Paso 3: ACTION de Auditoría en TOPIC Activo
- Se registra un ACTION en el TOPIC activo con:
  - `type`: `"topic_creation"`
  - `description`: "Creado nuevo TOPIC: [nombre] - [breve descripción]"
  - `metadata.new_topic_id`: ID del TOPIC recién creado
  - `metadata.reason`: "Surgió durante desarrollo de [contexto actual]"

**Ejemplo de ACTION de auditoría:**
```json
{
  "action_id": "edgi_20250824-1545_topic-creation-audit",
  "session_id": "edgi_20250824-1530_api-development",
  "project_id": "edgi_bitacora-rust-migration",
  "topic_id": "edgi_api-development",
  "type": "topic_creation",
  "description": "Creado nuevo TOPIC: 'API Performance Optimization' - Identificada necesidad de optimizar rendimiento de endpoints durante implementación de middleware",
  "metadata": {
    "new_topic_id": "edgi_api-performance-optimization",
    "reason": "Surgió durante implementación de authentication middleware",
    "context": "Notamos latencia >200ms en endpoints con autenticación que requieren optimización futura"
  }
}
```

#### Paso 4: ACTION Inicial en Nuevo TOPIC
- Se registra el primer ACTION en el nuevo TOPIC con:
  - `type`: `"topic_origin"`
  - `description`: Contexto de origen y motivación
  - `metadata.origin_topic_id`: TOPIC donde se estaba trabajando
  - `metadata.origin_context`: Descripción de lo que estaba sucediendo

**Ejemplo de ACTION inicial:**
```json
{
  "action_id": "edgi_20250824-1546_topic-origin",
  "session_id": "edgi_20250824-1530_api-development", 
  "project_id": "edgi_bitacora-rust-migration",
  "topic_id": "edgi_api-performance-optimization",
  "type": "topic_origin",
  "description": "TOPIC originado durante trabajo en 'API Development'. Mientras implementábamos authentication middleware, identificamos latencia >200ms en endpoints protegidos. Necesario optimizar queries, caching y middleware chain para mejorar performance de autenticación.",
  "metadata": {
    "origin_topic_id": "edgi_api-development",
    "origin_context": "Implementación de Axum authentication middleware",
    "trigger_event": "Performance analysis durante testing",
    "initial_priority": "high"
  }
}
```

#### Paso 5: Continuación con TOPIC Activo
- El sistema continúa con el TOPIC activo original
- El nuevo TOPIC queda registrado pero no activo
- Se mantiene la trazabilidad completa entre ambos TOPICs

#### Paso 6: Confirmación para Cambio de TOPIC (Opcional)
- La AI pregunta al usuario si desea cambiar al nuevo TOPIC
- Si el usuario confirma:
  - El TOPIC actual pasa a `"on-hold"`
  - El nuevo TOPIC pasa a `"active"`
  - Se registra ACTION de cambio de contexto en ambos TOPICs
- Si el usuario no confirma:
  - Se continúa con el TOPIC activo
  - El nuevo TOPIC permanece para trabajo futuro

## Relaciones y Complementariedad

### PROJECT → TOPIC → ACTION (Jerarquía Principal)
```
PROJECT: "Bitacora Rust Migration"
└── TOPIC: "API Development" (activo)
    ├── ACTION: "Design REST endpoints"
    ├── ACTION: "Implement Axum handlers" 
    ├── ACTION: "Add authentication middleware"
    └── ACTION: "Write integration tests"
```

### PROJECT ↔ TOPIC
- Un PROJECT contiene múltiples TOPICs organizados por área temática
- Solo un TOPIC está activo por PROJECT para mantener foco
- Los TOPICs heredan configuración y contexto del PROJECT
- El progreso de TOPICs se agrega al nivel PROJECT para métricas generales

### TOPIC ↔ ACTION
- Un TOPIC agrupa múltiples ACTIONs relacionadas temáticamente
- Cada ACTION puede pertenecer a un TOPIC específico (`topic_id`)
- Las ACTIONs actualizan el progreso y métricas del TOPIC
- Las ACTIONs de auditoría mantienen trazabilidad entre TOPICs

### TOPIC ↔ SPARK (Transversal)
- Los SPARKs pueden surgir durante el trabajo en cualquier TOPIC
- Un SPARK puede influir en la evolución de un TOPIC existente
- Un SPARK puede inspirar la creación de nuevos TOPICs
- Los SPARKs se vinculan a TOPICs para contexto y aplicabilidad

### ACTION ↔ SPARK (Generativo)
- Los SPARKs pueden originarse de ACTIONs específicas
- Las ACTIONs pueden aplicar conocimiento de SPARKs existentes
- Ambos mantienen trazabilidad temporal y contextual

## Beneficios del Flujo Propuesto

### Trazabilidad Completa
- Cada nuevo TOPIC tiene registro de su origen
- Se mantiene contexto de por qué y cuándo surgió la necesidad
- Auditoría completa de decisiones y bifurcaciones de desarrollo

### Gestión de Interrupciones
- Las ideas no se pierden, se capturan inmediatamente
- El flujo principal no se interrumpe innecesariamente
- Decisión consciente sobre cambios de contexto

### Análisis y Métricas
- Permite analizar patrones de generación de TOPICs
- Identifica áreas que generan múltiples bifurcaciones
- Facilita estimaciones futuras basadas en histórico

### Flexibilidad Organizacional
- TOPICs pueden ser del proyecto actual, ideas futuras o relacionados con otros proyectos
- Sistema adaptable a diferentes metodologías de trabajo
- Soporte para desarrollo iterativo y exploratorio

## Implementación Técnica

### Campos Adicionales Sugeridos

**En colección `projects`:**
```json
{
  "project_id": "edgi_bitacora-rust-migration",
  "active_topic_id": "edgi_api-development",
  "topic_history": [
    {
      "topic_id": "edgi_architecture-design",
      "status": "completed",
      "duration_days": 3
    },
    {
      "topic_id": "edgi_api-development", 
      "status": "active",
      "started_at": "2025-08-24T10:00:00.000Z"
    }
  ]
}
```

**En colección `topics`:**
```json
{
  "origin_context": {
    "origin_topic_id": "topic_id_origen",
    "created_during_session": "session_id",
    "trigger_context": "descripción_del_contexto",
    "created_at": "timestamp"
  }
}
```

**En colección `actions`:**
```json
{
  "metadata": {
    "action_subtype": "topic_creation|topic_origin|topic_switch",
    "related_topic_id": "topic_relacionado",
    "context_description": "descripción_del_contexto"
  }
}
```

### Queries de Ejemplo

**Buscar PROJECT activo con su TOPIC activo:**
```javascript
db.projects.findOne({"user_id": "edgi", "status": "active"}, 
                   {"project_id": 1, "active_topic_id": 1, "name": 1})
```

**Buscar todos los TOPICs de un PROJECT:**
```javascript
db.topics.find({"project_id": "edgi_bitacora-rust-migration"})
           .sort({"status": 1, "priority": -1})
```

**Buscar origen de un TOPIC:**
```javascript
db.topics.findOne({"topic_id": "edgi_api-performance-optimization"}, 
                  {"origin_context": 1})
```

**Buscar TOPICs creados durante otro TOPIC:**
```javascript
db.topics.find({"origin_context.origin_topic_id": "edgi_api-development"})
```

**Auditoría de creación de TOPICs en un PROJECT:**
```javascript
db.actions.find({
  "project_id": "edgi_bitacora-rust-migration",
  "type": "topic_creation"
}).sort({"created_at": -1})
```

**Historial de TOPICs activos en un PROJECT:**
```javascript
db.projects.findOne({"project_id": "edgi_bitacora-rust-migration"}, 
                   {"topic_history": 1})
```

## Conclusión

Este flujo de trabajo proporciona un marco completo para la gestión inteligente de PROJECTs, TOPICs, ACTIONs y SPARKs en Bitacora. Permite capturar ideas sin interrumpir el flujo principal, mantiene trazabilidad completa y facilita la organización flexible del trabajo de desarrollo.

La combinación de estos cuatro conceptos crea un sistema robusto de gestión del conocimiento que evoluciona naturalmente con las necesidades del proyecto y del desarrollador, desde la planificación macro (PROJECT) hasta la captura de insights granulares (SPARK).
