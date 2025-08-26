# 🧭 Routier — 2025-08-24 · API Layer MVP
**Resumen corto:** Se definió el endpoint POST /topics y el contrato asociado.

## Decisiones
- Usar Axum para REST por time-to-value.
- Versionar contratos en docs/api/contracts/

## Deltas (qué cambió y por qué)
- Se agregaron pruebas de contrato y manejo de errores 400/409/500.

## Artefactos/Evidencia
- crates/api/src/routes/topics.rs; tests/api_topics.rs

## Flujo
{{> BITA-TPL-FLUJOS-v1 }}

## DoD
{{> BITA-TPL-DOD-v1 }}

## Riesgos
{{> BITA-TPL-RISK-v1 }}

## Próximos pasos + Gate
- Agregar auth y rate limiting — Gate: tests de contrato en verde + cobertura ≥80%