# 08 · Ejemplos aplicados a Bitácora

## Diagrama 1 — Descubrimiento temprano
```
Project ↦ Topic ↦ Action
   ↘       ↗
     Spark
```
**Uso:** Spark valida supuestos entre Project y Topic (p.ej., alcance de la API).

## Diagrama 2 — Afinamiento operativo
```
Project ↦ Topic ↦ Action
            ↘       ↗
              Spark
```
**Uso:** Spark captura optimizaciones durante la ejecución de acciones (p.ej., cacheo de consultas).

## Mini‑respuesta con Plantilla (03)
```
🎯 OBJETIVO: Implementar `POST /topics` con contrato estable.
🎛 ÁMBITO: crates api, services; modelos Topic.
🔗 TRAZABILIDAD: Project(Bitácora)/Topic(API Layer)/Action(Endpoint create).

Delta: Ruta POST con validación y mapeo de errores.

Artefactos: /crates/api/src/routes/topics.rs, /docs/api/topics_contract.md

Contratos: Request {title, tags?} → Response {id, status}; errores 400/409/500.
Decisiones: REST (Axum) por simplicidad.
Trazabilidad: Command(CreateTopic) ↔ TopicService.create ↔ TopicRepo.
Estado: build OK; tests 12/12; cobertura 78%.
Riesgos: tasas altas de escritura → rate limit; mitigación: middleware.
Próximos pasos: Auth y quotas. Gate: tests de contrato en verde.
```
