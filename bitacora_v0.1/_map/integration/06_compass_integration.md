# Compass Integration - Propuesta de Sistema Analítico Independiente

## Visión General

Compass es un sistema analítico independiente diseñado para complementar a Bitacora. Su propósito es procesar, correlacionar y generar insights y recomendaciones a partir de múltiples orígenes de datos (incluyendo Bitacora) sin acoplarse al ciclo de vida de la aplicación core.

Esta propuesta documenta la integración planificada dentro del mapa (`_map`) como una futura versión/feature.

## Resumen Ejecutivo

- Nombre del crate/proyecto: `compass`
- Naturaleza: Servicio independiente (microservicio/crate) con API propia
- Rol: Motor de análisis basado en templates y conectores multi-fuente
- Prioridad: Enfocado a v0.3+ (después de estabilizar Bitacora core)

## ¿Por qué independiente?

- Permite evolución autónoma del motor analítico.
- Facilita reuso en otros proyectos o ecosistemas.
- Reduce riesgo de regresiones en la rama principal de Bitacora.
- Habilita escalado y despliegue diferenciado (recursos, seguridad, versiones).

## Objetivos funcionales

1. Analizar sesiones, acciones y topics de Bitacora y otras fuentes.
2. Generar insights estructurados (patterns, riesgos, recomendaciones).
3. Exponer API para consultas y triggers de análisis.
4. Permitir templates versionados que definen tipos de análisis.
5. Mantener trazabilidad y feedback loop con Bitacora.

## Contracto de Datos (Data Contract)

Compass y Bitacora compartirán modelos básicos (ejemplos):

- Session
  - id, user_id, start_ts, end_ts, metadata
- Action
  - id, session_id, type, timestamp, payload
- Topic
  - id, name, related_records

Compass ingestará estos objetos vía API o extractores ETL y mantendrá sus propias colecciones de `insights`, `patterns` y `analysis_results`.

## Templates: Diseño y Ejemplo (MongoDB)

Collection: `analysis_templates`
```json
{
  "_id": "ObjectId",
  "template_id": "record_context_v1",
  "type": "primary|secondary",
  "name": "Record Context Analysis",
  "version": "1.0.0",
  "description": "Instrucciones para análisis contextual del record",
  "instruction_template": {
    "system_prompt": "...",
    "user_prompt_template": "..."
  },
  "analysis_pipeline": [
    { "step": "collect", "sources": ["bitacora.records"] },
    { "step": "transform", "operation": "normalize_timestamps" },
    { "step": "analyze", "algorithm": "sequence_mining", "params": {} },
    { "step": "generate_insight", "format": "structured" }
  ],
  "required_data_sources": ["records","actions","topics"],
  "personalization": { "user_id": null, "params": {} },
  "metadata": { "created_at": "ISODate", "updated_at": "ISODate" }
}
```

Collection: `analysis_results`
```json
{
  "_id": "ObjectId",
  "analysis_id": "uuid",
  "template_id": "record_context_v1",
  "context": { "record_id": "..." },
  "insights": [ { "type": "pattern", "confidence": 0.9, "description": "..." } ],
  "metrics": { "execution_ms": 1234, "data_points": 42 },
  "execution_context": { "trigger": "manual|scheduled|event" },
  "feedback": {}
}
```

## Arquitectura del Proyecto `compass`

```
compass/
├── src/
│   ├── core/                # Modelos, tipos y contratos
│   ├── templates/           # Manager de templates y storage
│   ├── engines/             # Motores de análisis (pluggable)
│   ├── connectors/          # Conectores a Bitacora, GitHub, web
│   ├── api/                 # HTTP/gRPC endpoints
│   ├── persistence/         # Acceso a MongoDB y storage
│   └── utils/               # Logging, metrics, auth helpers
├── templates/               # Templates preinstalados
└── infra/                   # Docker/Helm/k8s manifests
```

### Componentes clave

- Template Manager: carga, versión y cache de templates.
- Data Collector: ingesta desde Bitacora (push o pull) y otras fuentes.
- Analysis Engine: ejecuta pipelines declarativos definidos por templates.
- Insight Store: persistencia de resultados y métricas.
- API: endpoints para triggers, consultas y feedback.

## Integración con Bitacora (flujo)

1. Bitacora envía eventos/objetos a Compass via HTTP POST o Compass los extrae mediante connectors.
2. Compass ejecuta templates relevantes y persiste `analysis_results`.
3. Compass notifica a Bitacora (webhook) o Bitacora consulta resultados vía API.
4. Usuario recibe recomendaciones en Bitacora UI o CLI y puede enviar feedback que alimenta `compass`.

## Seguridad y Gobernanza

- Autenticación: JWT/OAuth para APIs internas
- Autorización: roles para modificar templates, ejecutar análisis, ver resultados
- Auditoría: trazabilidad de ejecuciones y cambios en templates
- Sandbox: ejecución de pasos de análisis en entornos limitados para evitar a ejecución de código arbitrario

## Roadmap - Fases

- Fase 0: Diseño/Documentación (este documento) - prioridad alta
- Fase 1: PoC ligero (ingesta desde Bitacora + 2 templates primarios)
- Fase 2: Motor de templates y panel de insights básico
- Fase 3: Conectores externos (GitHub, webscraper)
- Fase 4: Personalization & learning loop
- Fase 5: LLM augmentation y recomendaciones en lenguaje natural

## Métricas de éxito

- Tiempo medio para producir un insight < 2s para queries simples
- Tasa de adopción en usuarios de Bitacora > 30% en el primer mes de beta
- Feedback positivo (útil/no útil) > 70% en resultados iniciales

## Consideraciones operacionales

- Desplegar Compass en entorno separado (k8s/VM) con control de recursos
- Versionado estricto de templates y migraciones de schema
- Backups de `analysis_results` y `analysis_templates`

## Próximos pasos inmediatos

- Confirmar nombre `compass` y alinearlo en `_map`
- Añadir `compass` al README del _map en sección de integración como "Propuesta futura"
- Priorizar Fase 1: PoC (definir 2 templates primarios y endpoints mínimos)

---

**Documento añadido a**: `_map/integration/06_compass_integration.md`
**Estado**: Propuesta futura - listo para revisión y aprobación
