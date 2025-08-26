# Analyzer System Integration - Template-Based Analysis Architecture

## Visión General

El sistema `analyzer` representa la evolución hacia un análisis inteligente basado en templates de instrucciones almacenados en base de datos. Esta arquitectura permite análisis adaptativos que evolucionan con el usuario y sus métricas específicas.

## Arquitectura de Templates

### Tipos de Analysis Templates

#### 1. **Primary Analysis Templates** (Análisis Primarios)
- **Propósito**: Templates esenciales que el sistema requiere para funcionamiento básico
- **Características**:
  - Pre-instalados con el sistema
  - No modificables por el usuario
  - Actualizados solo con versiones del sistema
  - Optimizados para patrones fundamentales de desarrollo

**Templates Primarios Identificados:**
```
- record_context_analysis: Análisis contextual del record activo
- productivity_pattern_analysis: Patrones de productividad del usuario
- time_efficiency_analysis: Análisis de eficiencia temporal
- topic_progression_analysis: Progresión y evolución de topics
- action_effectiveness_analysis: Efectividad de acciones tomadas
- sprint_performance_analysis: Rendimiento en sprints/sesiones
```

#### 2. **Secondary Analysis Templates** (Análisis Secundarios)
- **Propósito**: Templates específicos que evolucionan con el usuario
- **Características**:
  - Creados dinámicamente basado en métricas del usuario
  - Modificables y personalizables
  - Evolucionan con patrones de trabajo específicos
  - Permiten análisis ultra-especializados

**Templates Secundarios Potenciales:**
```
- technology_stack_analysis: Análisis específico por stack tecnológico
- collaboration_pattern_analysis: Patrones de colaboración específicos
- domain_expertise_analysis: Análisis por dominio de expertise
- custom_workflow_analysis: Análisis de workflows personalizados
- learning_curve_analysis: Curvas de aprendizaje personalizadas
```

## Database Schema para Templates

### Collection: `analysis_templates`
```mongodb
{
  _id: ObjectId,
  template_id: "record_context_analysis",
  type: "primary" | "secondary",
  version: "1.0.0",
  name: "Record Context Analysis",
  description: "Analiza el contexto del record activo...",
  
  // Template de instrucciones
  instruction_template: {
    system_prompt: "Analiza el contexto del record...",
    user_prompt_template: "Record: {record_data}\nActions: {actions}\n...",
    analysis_framework: {
      dimensions: ["context", "progression", "blockers"],
      metrics: ["time_spent", "actions_count", "topic_changes"],
      output_format: "structured_insights"
    }
  },
  
  // Configuración de datos requeridos
  required_data_sources: [
    "current_record",
    "recent_actions", 
    "active_topics",
    "time_metrics"
  ],
  
  // Configuración de external sources (futuro)
  external_sources_config: {
    enabled: false,
    connectors: [],
    priority: "internal_first"
  },
  
  // Metadata del template
  metadata: {
    created_at: ISODate,
    updated_at: ISODate,
    usage_count: 0,
    avg_execution_time: 0,
    success_rate: 1.0,
    user_rating: null
  },
  
  // Para templates secundarios
  personalization: {
    user_id: ObjectId | null,
    custom_parameters: {},
    learned_patterns: {},
    adaptation_history: []
  }
}
```

### Collection: `analysis_results`
```mongodb
{
  _id: ObjectId,
  analysis_id: "uuid",
  template_id: "record_context_analysis",
  record_id: ObjectId,
  
  // Resultados del análisis
  analysis_results: {
    insights: [
      {
        type: "pattern",
        confidence: 0.85,
        description: "Usuario muestra alta productividad en sesiones matutinas",
        data_points: [...],
        recommendations: [...]
      }
    ],
    metrics: {
      execution_time_ms: 1250,
      data_points_analyzed: 47,
      confidence_score: 0.78
    }
  },
  
  // Contexto de la ejecución
  execution_context: {
    timestamp: ISODate,
    trigger: "manual" | "automatic" | "scheduled",
    data_sources_used: ["records", "actions", "topics"],
    external_sources_used: []
  },
  
  // Feedback loop
  user_feedback: {
    usefulness_rating: null,
    applied_recommendations: [],
    feedback_notes: null
  }
}
```

## Arquitectura del Crate `analyzer`

### Módulos Principales

```
analyzer/
├── src/
│   ├── lib.rs
│   ├── templates/
│   │   ├── mod.rs
│   │   ├── manager.rs           // Template Manager
│   │   ├── primary.rs           // Primary templates logic
│   │   ├── secondary.rs         // Secondary templates logic
│   │   └── evolution.rs         // Template evolution engine
│   ├── engines/
│   │   ├── mod.rs
│   │   ├── analysis_engine.rs   // Core analysis execution
│   │   ├── data_collector.rs    // Internal data collection
│   │   └── insight_generator.rs // Insights generation
│   ├── models/
│   │   ├── mod.rs
│   │   ├── template.rs          // Template data models
│   │   ├── analysis.rs          // Analysis models
│   │   └── insights.rs          // Insights models
│   └── services/
│       ├── mod.rs
│       ├── analyzer_service.rs  // Main service interface
│       └── personalization.rs   // User personalization
```

## Flujo de Análisis Propuesto

### 1. **Template Loading Phase**
```
1. Load primary templates from database
2. Load user-specific secondary templates
3. Initialize analysis engines with templates
4. Cache frequently used templates
```

### 2. **Data Collection Phase**
```
1. Identify required data sources per template
2. Collect internal Bitacora data (records, actions, topics, metrics)
3. [Future] Collect external data based on connectors
4. Validate data completeness and quality
```

### 3. **Analysis Execution Phase**
```
1. Execute template instructions against collected data
2. Apply analysis framework defined in template
3. Generate structured insights with confidence scores
4. Create actionable recommendations
```

### 4. **Personalization Learning Phase**
```
1. Track analysis effectiveness
2. Learn from user feedback
3. Adapt secondary templates based on patterns
4. Suggest new secondary templates
```

## Integración con Sistema Existente

### Commands Integration
- Nuevo comando: `bitacora analyze [template_id]`
- Análisis automático en `bitacora status`
- Análisis programado en `bitacora start/end`

### Database Integration
- Nuevas collections para templates y resultados
- Integración con existing records/actions/topics
- Índices optimizados para queries de análisis

### API Integration
- REST endpoints para template management
- WebSocket para análisis en tiempo real
- Webhook support para análisis triggered

## Roadmap de Implementación

### Fase 1: Template Foundation (v0.2)
- [ ] Basic template system with primary templates
- [ ] Database schema implementation
- [ ] Core analysis engine
- [ ] Integration with existing commands

### Fase 2: Intelligence Layer (v0.3)
- [ ] Advanced insight generation
- [ ] User pattern learning
- [ ] Secondary template creation
- [ ] Performance optimization

### Fase 3: External Integration (v0.4)
- [ ] External connectors framework
- [ ] Documentation source integration
- [ ] Multi-source analysis fusion
- [ ] Advanced recommendation system

### Fase 4: AI Enhancement (v0.5)
- [ ] LLM-powered analysis
- [ ] Natural language insights
- [ ] Predictive analytics
- [ ] Automated optimization suggestions

## Beneficios de esta Arquitectura

### 1. **Escalabilidad**
- Templates pueden crecer independientemente del código core
- Nuevos tipos de análisis sin cambios de código
- Personalización sin fragmentación del sistema

### 2. **Mantenibilidad**
- Lógica de análisis separada del código de aplicación
- Templates versionados y actualizables
- Testing independiente de templates

### 3. **Flexibilidad**
- Análisis adaptables a diferentes workflows
- Customización por usuario o equipo
- Evolución basada en uso real

### 4. **Performance**
- Templates compilados y cacheados
- Análisis paralelo de múltiples templates
- Optimización basada en métricas de uso

## Consideraciones Técnicas

### Database Performance
- Índices optimizados para template queries
- Caching strategy para templates frecuentes
- Partitioning por user_id para secondary templates

### Memory Management
- Template compilation y caching inteligente
- Lazy loading de templates no utilizados
- Garbage collection de analysis results antiguos

### Security
- Template validation para prevenir code injection
- Sandboxed execution environment
- User permission system para template modification

---

**Nota**: Esta documentación representa el diseño conceptual del sistema analyzer. La implementación comenzará después de completar el sistema de commands y validar esta arquitectura con el equipo de desarrollo.
