# 04 · Plantilla — Proceso Secuencial (prompts y decisiones)

> Para guiar a la AI a decidir el **siguiente paso** usando contexto pasado y objetivo próximo.

## Marco de decisión por paso `t`
- **Contexto (`t-1`)**: Resultado, blockers, estado de roadmap.
- **Objetivo (`t`)**: Qué significa éxito (criterio medible).
- **Opciones**: 2–3 rutas con trade-offs.
- **Decisión**: Ruta elegida + por qué.
- **Verificación**: Checklist de coherencia con Project/Topic.
- **Evidencia**: Artefactos tocados (código/docs/PRs).
- **Siguiente (`t+1`)**: Qué desbloquea y gate.

## Checklist mínima
- ¿Alineado con milestone activo?
- ¿Contrato validado por consumidor?
- ¿Tests/Docs actualizados?
- ¿Nuevo riesgo introducido?

## Ejemplo
```
Contexto: Commands listos; falta API Layer (0%).
Objetivo: Exponer endpoint POST /topics con validación.
Opciones: (A) REST Axum, (B) gRPC tonic, (C) GraphQL async-graphql.
Decisión: (A) por simplicidad y time-to-value.
Verificación: Contract test con TopicService; 3 casos de error.
Evidencia: /crates/api/src/routes/topics.rs, /tests/api_topics.rs
Siguiente: Seguridad (auth) y rate limiting. Gate: 100% tests passing.
```
