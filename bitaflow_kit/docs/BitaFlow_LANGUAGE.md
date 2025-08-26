# BitaFlow_LANGUAGE.md · Especificación del lenguaje

## Estructura de archivo
1) Front‑matter YAML entre `---`
2) Cuerpo Markdown con directivas

**Front‑matter mínimo:**
```yaml
---
alias: BITA-TPL-COMP-R1R2-v1
name: Comparativo de Iteración (R1↔R2)
slug: comparativo-iteracion
kind: TPL           # TPL|DOC|ADR|SNAP|MAP
version: 1
locale: es
requires: [BITA-TPL-DOD-v1]
---
```

**Directivas**
- Placeholders: `{{nombre}}`
- Includes: `{{> BITA-TPL-DOD-v1 }}`
- Comentarios DSL: líneas que comienzan con `;; `
- Diagramas ASCII en bloques ```
- Operadores: `↦`, `+`, `↘`, `↗`
