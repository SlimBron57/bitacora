# 09 · Riesgos y mitigaciones

- **Spaghetti transversal (Spark omnipresente):**
  - *Mitigación:* límites claros de lectura/escritura; eventos inmutables; revisiones de acople.
- **Paralelismo sin join definido:**
  - *Mitigación:* documentar estrategia (AND/OR/quorum/first‑wins) y tiempos tope.
- **Deriva de roadmap (acciones de poco impacto):**
  - *Mitigación:* exigir “Impacto/Riesgo/Tiempo” y Gate por milestone.
- **Estados inconsistentes entre capas (R1 vs R2):**
  - *Mitigación:* STATUS_SNAPSHOT.md generado por CI a partir de artefactos verificables.
