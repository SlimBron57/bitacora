# 03 · Plantilla — Arquitectura Secuencial (crates)

> Para documentar módulos/crates autónomos que se componen en secuencia, paralelo y transversal.

## Encabezado
- **🎯 OBJETIVO:** (una línea)
- **🎛 ÁMBITO:** (crates/tipos tocados)
- **🔗 TRAZABILIDAD:** Project/Topic/Action relacionados

## Delta de esta iteración
- Qué cambió y **por qué aporta** al roadmap.

## Artefactos
- Código (rutas), docs, scripts, migraciones, ejemplos.

## Contratos y composición
- **Input/Output** por crate
- **Idempotencia** y **Side effects**
- **Errores** y recuperación
- **Modos:** Secuencial / Paralelo (join) / Transversal (Spark/Observer)

## Decisiones de arquitectura
- Alternativas consideradas y trade-offs.

## Trazabilidad capas
- Comando ↔ Servicio(s) ↔ Repos/Modelos (tabla breve).

## Estado y calidad
- Build/tests/cobertura/lints, benchmark si aplica.

## Riesgos y mitigación
- Lista breve con plan de contención.

## Próximos pasos + Gate
- Acciones priorizadas y condición para avanzar.

---

### Mini‑ejemplo (rellenar)
```
🎯 OBJETIVO: Exponer crate `bitacora-workflow` con contratos explícitos.
🎛 ÁMBITO: crates workflow, core; modelos Topic/Spark.
🔗 TRAZABILIDAD: Project(Bitácora) / Topic(Workflow API) / Action(Definir contratos).

Delta: Se definieron Input/Output de `promote_spark_to_topic` y se añadió retry con backoff.

Artefactos: /crates/workflow/src/..., /docs/architecture/workflow_contracts.md

Contratos: ...
Decisiones: ...
Trazabilidad: ...
Estado: build OK, 28 tests verdes, cobertura 81%.
Riesgos: dependencia de storage - mitigar con adapter.
Próximos pasos: endpoint REST + pruebas de integración. Gate: tests de contrato en verde.
```
