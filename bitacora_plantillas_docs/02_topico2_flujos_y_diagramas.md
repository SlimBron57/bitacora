# 02 · Flujos y diagramas (Tópico 2)

## Gramática visual
- **Secuencia:** `A ↦ B ↦ C`
- **Paralelo + unión:** `A ↦ (B + C) ↦ D` (join implícito antes de D)
- **Transversal (insight/observador):**
  ```
  A ↦ B ↦ C
     ↘   ↗
       S
  ```
  Donde **S** (Spark) observa/inyecta contexto.

## Bitácora (patrones)
```
Project ↦ Topic ↦ Action
   ↘       ↗
     Spark

Project ↦ Topic ↦ Action
            ↘       ↗
              Spark
```

## Reglas operativas
- **Contratos** por paso: Entrada/Salida, efectos colaterales, errores recuperables.
- **Composición:**
  - Secuencial: política de propagación de errores (fail-fast vs retry/compensación).
  - Paralelo: `A || B` con estrategia de join (AND/OR/quorum/first-wins).
  - Transversal: límites de escritura/lectura para evitar “spaghetti transversal”.

## Alineación con roadmap
Cada paso debe declarar **cómo acerca al milestone** de PROJECT y al objetivo de TOPIC. Añadir `Impacto`/`Riesgo`/`Tiempo`.
