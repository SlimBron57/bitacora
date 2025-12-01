# 🗺️ FLUJO DE NAVEGACIÓN: LLM → Templates → Documentación

```yaml
# === METADATA ===
Archivo: ROADMAP_V2/07_TEMPLATES/NAVIGATION_FLOW.md
Versión: 1.0
Fecha: 2025-10-26
Propósito: Visualizar el flujo completo desde que un LLM recibe una tarea hasta generar documentación
Entry Point: ROADMAP_V2/GUIA.md
# === FIN METADATA ===
```

---

## 🎯 EL PROBLEMA QUE RESOLVEMOS

**Antes de templates:**
```
LLM recibe: "Crear VOXELDB.md"
   ↓
¿Qué estructura usar? 🤔
¿Qué secciones incluir? 🤔
¿Qué nivel de detalle? 🤔
   ↓
Inventa estructura basándose en conocimiento general
   ↓
Resultado: INCONSISTENTE entre documentos
```

**Con templates:**
```
LLM recibe: "Crear VOXELDB.md"
   ↓
Lee GUIA.md → SECCIÓN 1.6
   ↓
Sigue el flujo de templates
   ↓
Genera documentación CONSISTENTE y COMPLETA
```

---

## 🌊 FLUJO COMPLETO PASO A PASO

### Nivel 1: Entry Point (GUIA.md)

```
┌─────────────────────────────────────────────────┐
│ 1. LLM recibe tarea:                            │
│    "Crear ROADMAP_V2/02_COMPONENTES/            │
│     CRITICOS/VOXELDB.md"                        │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 2. LLM lee: ROADMAP_V2/GUIA.md                  │
│    - Sección 0: El Mapa de Tu Viaje             │
│    - Sección 1.6: Templates MTT-DSL ← CLAVE     │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 3. GUIA.md dice:                                │
│    "Para crear docs ROADMAP_V2, lee:            │
│     07_TEMPLATES/README.md"                     │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
```

### Nivel 2: Master Index (07_TEMPLATES/README.md)

```
┌─────────────────────────────────────────────────┐
│ 4. LLM lee: 07_TEMPLATES/README.md              │
│    - Tabla de templates disponibles             │
│    - Estrategia de selección por path           │
│    - Workflow paso a paso                       │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 5. LLM identifica template correcto:            │
│                                                 │
│    Path: "02_COMPONENTES/CRITICOS/VOXELDB.md"  │
│    ↓                                            │
│    Template: component_spec.yaml                │
│                                                 │
│    Lógica:                                      │
│    if path.contains("02_COMPONENTES/")         │
│       → component_spec.yaml                     │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 6. README.md provee instrucciones:              │
│    "Lee el template YAML completo"              │
│    "Sigue llm_instructions del template"        │
│    "Valida con validations al terminar"         │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
```

### Nivel 3: Template Específico (component_spec.yaml)

```
┌─────────────────────────────────────────────────┐
│ 7. LLM lee: 07_TEMPLATES/component_spec.yaml    │
│                                                 │
│    Secciones clave:                             │
│    - personality: tone, depth, style            │
│    - structure: secciones a incluir             │
│    - validations: checks al terminar            │
│    - llm_instructions: pasos específicos        │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 8. llm_instructions dice:                       │
│                                                 │
│    1. LEE PRIMERO:                              │
│       - 00_VISION/DECISIONES_ARQUITECTONICAS.md │
│       - 01_ARQUITECTURA/*.md                    │
│       - B20250915-data-compressor/src/          │
│                                                 │
│    2. EXTRAE contexto:                          │
│       "VOXELDB.md" → Component: VoxelDB         │
│                                                 │
│    3. BUSCA código existente:                   │
│       grep_search("VoxelDB")                    │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
```

### Nivel 4: Recopilación de Contexto

```
┌─────────────────────────────────────────────────┐
│ 9. LLM ejecuta recopilación:                    │
│                                                 │
│    read_file(00_VISION/DECISIONES_              │
│              ARQUITECTONICAS.md)                │
│    ↓                                            │
│    Identifica DA-XXX relevantes para VoxelDB    │
│    (DA-012: Templates in VoxelDB)               │
│                                                 │
│    read_file(01_ARQUITECTURA/                   │
│              SISTEMA_DUAL_DATABASES.md)         │
│    ↓                                            │
│    Comprende arquitectura dual-helix            │
│                                                 │
│    grep_search("VoxelDB")                       │
│    ↓                                            │
│    Encuentra código de referencia (si existe)   │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
```

### Nivel 5: Generación de Documento

```
┌─────────────────────────────────────────────────┐
│ 10. LLM genera documento sección por sección:   │
│                                                 │
│     structure.sections en component_spec.yaml:  │
│                                                 │
│     ┌───────────────────────────────────────┐   │
│     │ Section: "purpose"                    │   │
│     │ Title: "🎯 PROPÓSITO"                 │   │
│     │ Prompt: "Explicar en 2-3 párrafos..." │   │
│     └───────────┬───────────────────────────┘   │
│                 │                               │
│                 ▼                               │
│     LLM genera contenido según prompt           │
│                 │                               │
│                 ▼                               │
│     ┌───────────────────────────────────────┐   │
│     │ Section: "architectural_context"      │   │
│     │ Title: "🏗️ CONTEXTO ARQUITECTÓNICO"   │   │
│     │ Prompt: "Diagrama de ubicación..."    │   │
│     └───────────┬───────────────────────────┘   │
│                 │                               │
│                 ▼                               │
│     ... repite para todas las secciones         │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
```

### Nivel 6: Validación

```
┌─────────────────────────────────────────────────┐
│ 11. LLM ejecuta validaciones del template:      │
│                                                 │
│     validations en component_spec.yaml:         │
│                                                 │
│     ✅ has_rust_code_blocks                     │
│        → Cuenta bloques ```rust```              │
│        → Mínimo: 3                              │
│                                                 │
│     ✅ has_performance_targets                  │
│        → Busca tabla de benchmarks              │
│        → Debe existir                           │
│                                                 │
│     ✅ references_architectural_decisions       │
│        → Busca "DA-XXX" en el doc               │
│        → Mínimo: 1 referencia                   │
│                                                 │
│     ✅ max_document_size_kb: 30                 │
│        → Calcula tamaño del doc                 │
│        → Debe ser < 30 KB                       │
└────────────────┬────────────────────────────────┘
                 │
         ┌───────┴────────┐
         │                │
         ▼                ▼
   ✅ PASA          ❌ FALLA
         │                │
         │                ▼
         │    ┌─────────────────────────┐
         │    │ 12. Identifica problema │
         │    │     - Sección faltante  │
         │    │     - Contenido escaso  │
         │    │     - Doc muy grande    │
         │    └──────────┬──────────────┘
         │               │
         │               ▼
         │    ┌─────────────────────────┐
         │    │ 13. Itera:              │
         │    │     - Añade contenido   │
         │    │     - Mejora secciones  │
         │    │     - Reduce tamaño     │
         │    └──────────┬──────────────┘
         │               │
         │               ▼
         │    ┌─────────────────────────┐
         │    │ 14. Re-valida           │
         │    └──────────┬──────────────┘
         │               │
         └───────────────┘
                 │
                 ▼
```

### Nivel 7: Finalización

```
┌─────────────────────────────────────────────────┐
│ 15. Documento completo y validado:              │
│                                                 │
│     create_file(                                │
│       "ROADMAP_V2/02_COMPONENTES/CRITICOS/      │
│        VOXELDB.md",                             │
│       contenido_generado                        │
│     )                                           │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 16. Actualiza métricas del template:            │
│                                                 │
│     component_spec.yaml:                        │
│       usage_count: 0 → 1                        │
│       effectiveness_score: null → 0.85          │
│                                                 │
│     Cálculo:                                    │
│       - completeness: 100% (todas secciones)    │
│       - quality: 90% (contenido técnico bueno)  │
│       - usability: 85% (fácil de seguir)        │
│       - iteration_count: 1 (primera vez OK)     │
│       - validation_pass: 100% (pasó todo)       │
│                                                 │
│     effectiveness = 0.30*1.0 + 0.30*0.9 +       │
│                     0.20*0.85 + 0.10*(-1) +     │
│                     0.20*1.0                    │
│                   = 0.85 ✅                      │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│ 17. Si effectiveness < 0.8:                     │
│     Documenta en template changelog:            │
│                                                 │
│     changelog:                                  │
│       - version: "1.1"                          │
│         date: "2025-10-27"                      │
│         changes:                                │
│           - "Mejorado prompt X"                 │
│           - "Añadida validación Y"              │
│         reason: "Docs muy largos"               │
│                                                 │
│     Si effectiveness >= 0.8:                    │
│     ✅ Template está funcionando bien           │
└─────────────────────────────────────────────────┘
```

---

## 📊 DIAGRAMA VISUAL COMPLETO

```
┌─────────────────────────────────────────────────────────────┐
│                    ENTRADA: Tarea del Usuario               │
│        "Crear ROADMAP_V2/02_COMPONENTES/CRITICOS/           │
│         VOXELDB.md"                                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
        ┌────────────────────────────────────┐
        │  🗺️ NIVEL 1: ENTRY POINT          │
        │  ROADMAP_V2/GUIA.md                │
        │  → Sección 1.6: Templates MTT-DSL  │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  📚 NIVEL 2: MASTER INDEX          │
        │  07_TEMPLATES/README.md            │
        │  → Selección de template           │
        │  → Workflow detallado              │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  🧩 NIVEL 3: TEMPLATE ESPECÍFICO   │
        │  07_TEMPLATES/component_spec.yaml  │
        │  → Estructura (sections)           │
        │  → Instrucciones (llm_instructions)│
        │  → Validaciones (validations)      │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  📖 NIVEL 4: CONTEXTO              │
        │  - DECISIONES_ARQUITECTONICAS.md   │
        │  - ARQUITECTURA/*.md               │
        │  - Código de referencia            │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  ✍️ NIVEL 5: GENERACIÓN            │
        │  Crear doc sección por sección     │
        │  siguiendo prompts del template    │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  ✅ NIVEL 6: VALIDACIÓN            │
        │  Ejecutar checks del template      │
        │  Iterar si falla (máx 2 veces)     │
        └────────────┬───────────────────────┘
                     │
                     ▼
        ┌────────────────────────────────────┐
        │  💾 NIVEL 7: FINALIZACIÓN          │
        │  - Guardar documento               │
        │  - Actualizar métricas template    │
        │  - Feedback para mejora continua   │
        └────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    SALIDA: Documento ROADMAP_V2             │
│        ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md        │
│        ✅ Consistente                                       │
│        ✅ Completo                                          │
│        ✅ Validado                                          │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔑 PUNTOS CLAVE DEL FLUJO

### 1. **Entry Point Único y Claro**
- **GUIA.md** es el **master document**
- Sección 1.6 apunta a templates
- LLM siempre empieza aquí

### 2. **Selección Automática de Template**
- Por path del archivo (`02_COMPONENTES/` → `component_spec.yaml`)
- No requiere intervención humana
- Lógica simple y predecible

### 3. **Instrucciones Explícitas**
- Cada template tiene `llm_instructions`
- Pasos numerados y específicos
- Sin ambigüedad

### 4. **Validación Automática**
- Checks programáticos en el template
- Feedback inmediato si falla
- Iteración guiada

### 5. **Mejora Continua**
- Effectiveness score por template
- Changelog documenta iteraciones
- Cada uso mejora el sistema

---

## 🎯 PREGUNTAS FRECUENTES

### ¿Por qué GUIA.md como entry point?

**R:** Porque es el documento que los LLMs ya conocen y leen primero. Es la "puerta de entrada" natural al proyecto.

### ¿Qué pasa si el LLM no lee GUIA.md?

**R:** La estructura de directorios + nombres de templates hacen que sea **inferible**:
- Path contiene `02_COMPONENTES/` → buscar template con "component" en el nombre
- Pero es **más confiable** si el LLM sigue el flujo oficial

### ¿Los templates son obligatorios?

**R:** Para documentación ROADMAP_V2, **SÍ**. Son la metodología oficial. Para otros documentos (fuera de ROADMAP_V2), no necesariamente.

### ¿Qué pasa si un template no funciona bien?

**R:** Se itera. El template se actualiza en su `changelog` y se mejora para el siguiente uso. Es parte del proceso experimental.

---

## 📈 MÉTRICAS DE ÉXITO

### Por Documento Individual
- ✅ Todas las secciones requeridas completadas
- ✅ Pasa todas las validaciones del template
- ✅ Generado en máximo 2 iteraciones
- ✅ Tamaño dentro de límites

### Por Template
- ✅ Effectiveness score > 0.8
- ✅ Usage count > 5 (validado en múltiples docs)
- ✅ Changelog documenta mejoras
- ✅ LLMs pueden seguir instrucciones sin ambigüedad

### Por Sistema Completo
- ✅ 38 documentos ROADMAP_V2 generados consistentemente
- ✅ Templates optimizados y listos para templates finales
- ✅ Lecciones aprendidas documentadas
- ✅ Metodología MTT-DSL validada en producción

---

**Última Actualización:** 26 Octubre 2025  
**Versión:** 1.0  
**Estado:** ACTIVO - Guía de referencia

---

*"El flujo no es lineal. Es una danza."* 🌊✨
