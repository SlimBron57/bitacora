# 🧩 MTT-DSL TEMPLATES EXPERIMENTALES - ROADMAP_V2

```yaml
# === DATOS DE AUDITORÍA ===
Directorio: ROADMAP_V2/07_TEMPLATES/
Versión: 1.1 (Experimental)
Fecha Creación: 2025-10-26
Última Actualización: 2025-11-23 23:25:34
Autor: Eduardo + AI Companion
Propósito: Templates MTT-DSL para generar documentación ROADMAP_V2
Estado: EXPERIMENTAL - Para uso en ROADMAP_V2 solamente
Iteración: Segunda versión - Añadido implementation_plan.yaml
Templates Disponibles: 5 (component_spec, architecture_spec, integration_spec, testing_guide, implementation_plan)
# === FIN DATOS DE AUDITORÍA ===
```

---

## 🎯 PROPÓSITO DE ESTE DIRECTORIO

**Este directorio contiene dos tipos de templates:**

### A. Templates MTT-DSL (Experimentales)

1. ✅ **Generar documentación de ROADMAP_V2** de manera consistente
2. ✅ **Validar la metodología MTT-DSL** en producción real (dogfooding)
3. ✅ **Iterar y optimizar** los templates antes de crear los definitivos
4. ✅ **Guiar a LLMs** en la creación de documentación técnica estructurada

**NO son los templates finales de Bitácora** - esos se crearán en `templates/mtt/` después de completar ROADMAP_V2, basándose en las lecciones aprendidas aquí.

### B. Digester Templates (Phase 7.x) ⭐ NUEVO

Templates YAML para el **Data Import Engine** (Phase 7.x):

- `digesters/base_chat.yaml` - Base genérico para plataformas de chat
- `digesters/whatsapp_v1.yaml` - WhatsApp-specific (emojis, grupos)
- `digesters/telegram_v1.yaml` - Telegram-specific (channels, bots)
- `digesters/email_v1.yaml` - Email-specific (threads, attachments)

**Propósito:** Reglas de extracción evolvables sin recompilar código.

**Documentación relacionada:**
- `00_VISION/09_metabolic-digestion-vision.md` - Filosofía
- `01_ARQUITECTURA/18_metabolic-digestion-system.md` - Arquitectura
- `02_COMPONENTES/17_data-import-engine.md` - Componente
- `04_IMPLEMENTACION/PHASE_7X_DATA_IMPORT.md` - Plan 6 semanas

---

## 📚 TEMPLATES MTT-DSL DISPONIBLES

### 1. `component_spec.yaml` - Especificaciones de Componentes

**Usa este template para:**
- Documentos en `02_COMPONENTES/CRITICOS/`
- Documentos en `02_COMPONENTES/IMPORTANTES/`

**Ejemplos de outputs:**
- `VOXELDB.md`
- `FBCU_CORE.md`
- `SENSORY_ENGINE.md`
- `CONTEXT_TOKEN_7D.md`

**Estructura que genera:**
```markdown
🎯 PROPÓSITO
🏗️ CONTEXTO ARQUITECTÓNICO
📋 RESPONSABILIDADES CORE
🗂️ ESTRUCTURAS DE DATOS
🔌 API PÚBLICA
⚙️ IMPLEMENTACIÓN INTERNA
🔗 DEPENDENCIAS
⚡ OBJETIVOS DE PERFORMANCE
🧪 ESTRATEGIA DE TESTING
⚠️ MANEJO DE ERRORES
📚 REFERENCIAS
🚀 PRÓXIMOS PASOS
```

**Personalidad:**
- Tone: `technical`
- Depth: `implementation-ready`
- Style: `code-focused`
- Approach: `bottom-up`

---

### 2. `architecture_spec.yaml` - Especificaciones de Arquitectura

**Usa este template para:**
- Documentos en `01_ARQUITECTURA/`

**Ejemplos de outputs:**
- `SISTEMA_DUAL_DATABASES.md` ✅ (ya creado manualmente)
- `PIXEL_STORAGE_DEEP_DIVE.md` ✅ (ya creado manualmente)
- Futuros docs arquitectónicos

**Estructura que genera:**
```markdown
🎯 PROPÓSITO
📖 FUNDAMENTOS TEÓRICOS
🏗️ VISIÓN GENERAL DE ARQUITECTURA
🔬 DISEÑO DETALLADO
⚙️ ESTRATEGIA DE IMPLEMENTACIÓN
🗂️ FORMATOS DE DATOS
⚡ CARACTERÍSTICAS DE PERFORMANCE
📈 ESCALABILIDAD (opcional)
🔐 CONSIDERACIONES DE SEGURIDAD (opcional)
⚖️ COMPARACIÓN CON ALTERNATIVAS
🔗 PUNTOS DE INTEGRACIÓN
🧪 TESTING Y VALIDACIÓN
📚 REFERENCIAS
🚀 MEJORAS FUTURAS
```

**Personalidad:**
- Tone: `technical_explanatory`
- Depth: `deep_theoretical_and_practical`
- Style: `educational_but_precise`
- Approach: `top-down`

---

### 3. `integration_spec.yaml` - Especificaciones de Integración

**Usa este template para:**
- Documentos en `03_INTEGRACION/`

**Ejemplos de outputs:**
- `SENSORY_TO_TELESCOPEDB.md`
- `CTX7D_TO_VOXELDB.md`
- `HUBSPOKE_ROUTING.md`
- `BREAKTHROUGH_DETECTION.md`
- `FBCU_LIFECYCLE.md`

**Estructura que genera:**
```markdown
🎯 PROPÓSITO DE LA INTEGRACIÓN
🔄 VISIÓN GENERAL DEL FLUJO
📋 CONTRATOS DE DATOS
⚙️ LÓGICA DE TRANSFORMACIÓN
⚠️ MANEJO DE ERRORES
🔄 GESTIÓN DE ESTADO
⏱️ PATRONES ASYNC/SYNC
⚡ OPTIMIZACIÓN DE PERFORMANCE
🧪 TESTING DE INTEGRACIÓN
📊 MONITOREO Y OBSERVABILIDAD
🔙 ESTRATEGIA DE ROLLBACK (opcional)
📚 REFERENCIAS
🚀 MEJORAS FUTURAS
```

**Personalidad:**
- Tone: `systematic_clear`
- Depth: `integration-focused`
- Style: `flow-oriented`
- Approach: `pipeline-thinking`

---

### 4. `testing_guide.yaml` - Guías de Testing

**Usa este template para:**
- Documentos en `05_TESTING/`

**Ejemplos de outputs:**
- `UNIT_TESTS_GUIDE.md`
- `INTEGRATION_TESTS.md`
- `PERFORMANCE_BENCHMARKS.md`
- `GOLDEN_TESTS.md`
- `METAMORPHIC_TESTS.md`

---

### 5. `implementation_plan.yaml` - Planes de Implementación 🆕

**Usa este template para:**
- Documentos en `04_IMPLEMENTACION/*_IMPLEMENTATION_PLAN.md`
- Documentos en `04_IMPLEMENTACION/*_PLAN.md`

**Ejemplos de outputs:**
- `FLOWPACKS_IMPLEMENTATION_PLAN.md` ✅ (ya creado, ahora validado con template)
- `TELESCOPEDB_IMPLEMENTATION_PLAN.md`
- Futuros planes de componentes complejos

**Estructura que genera:**
```markdown
🌊 VISIÓN Y FILOSOFÍA
🧭 CONTEXTO EVOLUTIVO (si aplica)
🎯 PROBLEMA Y SOLUCIÓN
🏗️ ARQUITECTURA CORE
📦 COMPONENTES DETALLADOS (8-12 componentes)
🔗 PUNTOS DE INTEGRACIÓN
🌊 FLUJO DE DATOS
🚀 FASES DE IMPLEMENTACIÓN (3-9 fases)
📊 MÉTRICAS DE ÉXITO
🧪 ESTRATEGIA DE TESTING
🔗 DEPENDENCIAS Y RESTRICCIONES
🔌 PUNTOS DE EXTENSIBILIDAD (opcional)
⚠️ RIESGOS Y MITIGACIONES
🚀 MEJORAS FUTURAS
📚 REFERENCIAS
✅ CHECKLIST DE IMPLEMENTACIÓN
```

**Personalidad:**
- Tone: `strategic_and_technical`
- Depth: `comprehensive`
- Style: `narrative_with_code`
- Approach: `top-down`

**Características especiales:**
- ✅ Incluye narrativa filosófica + metáforas
- ✅ Contexto evolutivo (Phase 1 → Phase 2)
- ✅ Validación arquitectura dual database (TelescopeDB + VoxelDB)
- ✅ Fases temporales con estimación de horas
- ✅ Riesgos y contingencias
- ✅ Checklist ejecutivo para CHECKLIST_V2.md

**Validaciones críticas:**
- ❌ NO mencionar PostgreSQL, MongoDB, Redis, MySQL, SQLite
- ✅ SOLO TelescopeDB (memoria biográfica) + VoxelDB (templates)
- ✅ Al menos 5 bloques de código Rust
- ✅ Referencias a 3+ DAs (DA-XXX)
- ✅ Métricas cuantificables
- ✅ Timestamps actualizados (./scripts/timestamp.sh)

**Estructura que genera:**
```markdown
🎯 PROPÓSITO DEL TESTING
💡 FILOSOFÍA DE TESTING
📁 CATEGORÍAS DE TESTS
🏗️ ESTRUCTURA DE TESTS
📝 EJEMPLOS DE TESTS
🗄️ FIXTURES Y TEST DATA
🎭 MOCKING Y STUBBING
✅ ESTRATEGIAS DE ASSERTIONS
📊 MÉTRICAS DE COVERAGE
🔄 INTEGRACIÓN CI/CD
🐛 DEBUGGING DE TESTS FALLIDOS
⚡ BENCHMARKS DE PERFORMANCE
📚 REFERENCIAS
🚀 PRÓXIMOS PASOS
```

**Personalidad:**
- Tone: `rigorous_methodical`
- Depth: `comprehensive`
- Style: `example-driven`
- Approach: `quality-first`

---

### 5. `debugging_deep_dive.yaml` - Debugging Profundo (Legacy)

**Template original de FUSION_BAYESIANA** - incluido como referencia.

**NO usar para ROADMAP_V2** - este es para la metodología MTT-DSL original con componentes musicales.

---

## 🤖 GUÍA PARA LLMs: ¿Cómo Seleccionar el Template Correcto?

### Estrategia de Selección por Path

```rust
fn select_template(file_path: &str) -> Template {
    match file_path {
        // Componentes
        path if path.contains("02_COMPONENTES/CRITICOS/") => Template::ComponentSpec,
        path if path.contains("02_COMPONENTES/IMPORTANTES/") => Template::ComponentSpec,
        
        // Arquitectura
        path if path.contains("01_ARQUITECTURA/") => Template::ArchitectureSpec,
        
        // Integración
        path if path.contains("03_INTEGRACION/") => Template::IntegrationSpec,
        
        // Implementación (planes complejos)
        path if path.contains("04_IMPLEMENTACION/") && path.ends_with("_PLAN.md") => {
            Template::ImplementationPlan
        }
        
        // Testing
        path if path.contains("05_TESTING/") => Template::TestingGuide,
        
        // Default
        _ => Template::None,
    }
}
```

### Ejemplos de Selección

| Archivo a Crear | Template Seleccionado | Razón |
|-----------------|----------------------|-------|
| `02_COMPONENTES/CRITICOS/VOXELDB.md` | `component_spec.yaml` | Path contiene `02_COMPONENTES/` |
| `01_ARQUITECTURA/VOXEL_CUBIC_STORAGE.md` | `architecture_spec.yaml` | Path contiene `01_ARQUITECTURA/` |
| `03_INTEGRACION/SENSORY_TO_TELESCOPEDB.md` | `integration_spec.yaml` | Path contiene `03_INTEGRACION/` |
| `04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md` | `implementation_plan.yaml` | Path contiene `04_IMPLEMENTACION/` + termina en `_PLAN.md` |
| `05_TESTING/UNIT_TESTS_GUIDE.md` | `testing_guide.yaml` | Path contiene `05_TESTING/` |

---

## 🔄 WORKFLOW DE USO DE TEMPLATES

### Paso 1: Identificar el Documento a Crear

```
Task: Crear ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md
```

### Paso 2: Seleccionar Template Apropiado

```
Path contiene "02_COMPONENTES/" → Template: component_spec.yaml
```

### Paso 3: Leer el Template

```yaml
# Leer ROADMAP_V2/07_TEMPLATES/component_spec.yaml
# Revisar:
# - structure.sections (qué secciones incluir)
# - personality (tono y profundidad)
# - llm_instructions (instrucciones específicas)
# - validations (qué validar al terminar)
```

### Paso 4: Recopilar Contexto Necesario

Según `llm_instructions` del template:

```
1. LEE PRIMERO:
   - 00_VISION/DECISIONES_ARQUITECTONICAS.md (DA-XXX relevantes)
   - 01_ARQUITECTURA/*.md (diseño del sistema)
   - B20250915-data-compressor/src/ (código de referencia)

2. EXTRAE contexto del nombre:
   - "VOXELDB.md" → Component: VoxelDB, Category: Databases

3. BUSCA código existente:
   - grep_search para encontrar structs/impls relacionados
```

### Paso 5: Generar Documento Siguiendo Estructura

```markdown
# Seguir sections del template en orden:
1. audit_header
2. purpose
3. architectural_context
4. core_responsibilities
5. data_structures
6. public_api
7. internal_implementation (opcional)
8. dependencies
9. performance_targets
10. testing_strategy
11. error_handling
12. references
13. next_steps
14. footer
```

### Paso 6: Validar Output

```
Ejecutar validations del template:
✅ has_rust_code_blocks → Mínimo 3 bloques de código Rust
✅ has_performance_targets → Tabla de benchmarks incluida
✅ references_architectural_decisions → Al menos una DA-XXX
✅ max_document_size_kb: 30 → Tamaño < 30 KB
```

### Paso 7: Iterar si Necesario

```
Si validación falla:
- Añadir secciones faltantes
- Expandir ejemplos de código
- Mejorar referencias
- Reducir tamaño si excede límite
```

---

## 📊 MÉTRICAS DE CALIDAD DE TEMPLATES

### Por Template

| Template | Usos | Effectiveness Score | Última Actualización |
|----------|------|---------------------|----------------------|
| `component_spec.yaml` | 0 | TBD | 2025-10-26 |
| `architecture_spec.yaml` | 0 | TBD | 2025-10-26 |
| `integration_spec.yaml` | 0 | TBD | 2025-10-26 |
| `testing_guide.yaml` | 0 | TBD | 2025-10-26 |

**Effectiveness Score** se calculará como:
```
score = (
    completeness * 0.3 +      # ¿Todas las secciones completadas?
    quality * 0.3 +            # ¿Calidad del contenido generado?
    usability * 0.2 +          # ¿Fácil de seguir para LLMs?
    iteration_count * -0.1 +   # ¿Cuántas iteraciones necesitó?
    validation_pass_rate * 0.2 # ¿Pasó validaciones a la primera?
)
```

### Objetivos de Calidad

- **Completeness:** 100% de secciones requeridas
- **Quality:** Contenido técnico preciso, ejemplos funcionales
- **Usability:** LLM puede seguir instrucciones sin ambigüedad
- **Iteration Count:** Máximo 2 iteraciones para docs de calidad
- **Validation Pass Rate:** 80%+ de documentos pasan validación primera vez

---

## 🔬 PROCESO DE ITERACIÓN Y MEJORA

### Ciclo de Feedback

```
┌─────────────────────────────────────────────┐
│ 1. Usar template para generar doc          │
└────────────────┬────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────┐
│ 2. Validar output con validations          │
└────────────────┬────────────────────────────┘
                 │
         ┌───────┴────────┐
         │ ¿Pasa?         │
         └───────┬────────┘
                 │
        ┌────────┴────────┐
        │ SÍ              │ NO
        │                 │
        ▼                 ▼
┌────────────┐   ┌───────────────────┐
│ 3. Marcar  │   │ 3. Identificar    │
│ template   │   │ problema:         │
│ como       │   │ - Sección falta   │
│ exitoso    │   │ - Prompt ambiguo  │
│            │   │ - Validación mala │
└──────┬─────┘   └─────────┬─────────┘
       │                   │
       │                   ▼
       │         ┌───────────────────┐
       │         │ 4. Actualizar     │
       │         │ template:         │
       │         │ - Mejorar prompt  │
       │         │ - Ajustar validac.│
       │         │ - Añadir ejemplos │
       │         └─────────┬─────────┘
       │                   │
       │                   ▼
       │         ┌───────────────────┐
       │         │ 5. Regenerar doc  │
       │         └─────────┬─────────┘
       │                   │
       └───────────────────┘
                 │
                 ▼
       ┌───────────────────┐
       │ 6. Documentar     │
       │ lección aprendida │
       └───────────────────┘
```

### Registro de Mejoras

**Al actualizar un template, documentar:**

```yaml
# En el template .yaml

changelog:
  - version: "1.1"
    date: "2025-10-27"
    changes:
      - "Mejorado prompt de 'core_responsibilities' con ejemplos"
      - "Añadida validación has_async_examples"
      - "Reducido max_document_size_kb de 30 a 25"
    reason: "Docs generados eran demasiado largos y faltaban ejemplos async"
    effectiveness_improvement: "+15% validation pass rate"
```

---

## 🚀 PRÓXIMOS PASOS

### Esta Semana (Validación de Templates)

1. ✅ **Crear templates base** (component_spec, architecture_spec, integration_spec, testing_guide)
2. 🔄 **Probar component_spec.yaml** generando `VOXELDB.md`
3. 🔄 **Iterar basado en feedback** - ajustar prompts, validaciones, estructura
4. 🔄 **Probar integration_spec.yaml** generando `SENSORY_TO_TELESCOPEDB.md`
5. 🔄 **Optimizar templates** hasta tener effectiveness score > 0.8

### Próximas Semanas (Uso en Producción)

6. Generar todos los documentos de `02_COMPONENTES/` usando `component_spec.yaml`
7. Generar todos los documentos de `03_INTEGRACION/` usando `integration_spec.yaml`
8. Generar todos los documentos de `05_TESTING/` usando `testing_guide.yaml`
9. Documentar lecciones aprendidas después de cada lote
10. Actualizar templates basándose en experiencia acumulada

### Al Completar ROADMAP_V2 (Templates Finales)

11. Crear templates definitivos en `templates/mtt/` basados en estos experimentales
12. Los templates definitivos serán para **uso de Bitácora en producción** (usuarios finales)
13. Incluir componentes musicales (HarmonyEngine) si están implementados
14. Documentar diferencias entre templates experimentales vs. finales

---

## 📚 REFERENCIAS

### Metodología MTT-DSL Original
- `FUSION_BAYESIANA/05_MTT_DSL_TEMPLATES.md` - Análisis completo de MTT-DSL
- `templates/mtt/README.md` - Templates originales (con música)
- `templates/mtt/technical/debugging_deep_dive.yaml` - Template de referencia

### Documentos ROADMAP_V2
- `00_VISION/DECISIONES_ARQUITECTONICAS.md` - DA-001 a DA-027
- `01_ARQUITECTURA/*.md` - Diseños arquitectónicos
- `CHECKLIST_V2.md` - Lista completa de tareas
- `CHECKLIST_TREE_V2.md` - Vista jerárquica

### Código de Referencia
- `B20250915-data-compressor/src/` - Implementación proof-of-concept
- `src/harmony_engine/` - HarmonyEngine (inactivo)
- `src/context_intelligence/` - Context Intelligence

---

## ❓ PREGUNTAS FRECUENTES

### ¿Por qué templates experimentales separados?

**R:** Para validar MTT-DSL en producción real antes de crear los templates finales de Bitácora. Es "dogfooding" - usar nuestra propia metodología para documentarnos a nosotros mismos.

### ¿Estos templates incluyen música (HarmonyEngine)?

**R:** NO. Son puramente estructurales. La música es feature opcional de MTT-DSL que se añadirá en templates finales de `templates/mtt/` si HarmonyEngine se implementa.

### ¿Qué pasa si un template no funciona bien?

**R:** Iteramos. El propósito de ser "experimentales" es poder ajustarlos sin romper nada. Cada uso mejora el template.

### ¿Cuándo creamos los templates finales de Bitácora?

**R:** Después de completar ROADMAP_V2 (38 documentos). Para ese entonces tendremos:
- Templates optimizados y validados
- Lecciones aprendidas documentadas
- Effectiveness scores > 0.8
- Patrones claros de qué funciona y qué no

### ¿Los templates finales serán diferentes?

**R:** Sí. Los templates finales en `templates/mtt/` serán para:
- Usuarios de Bitácora (no solo documentación interna)
- Incluirán componentes musicales (HarmonyEngine)
- Basados en especificaciones BITA-1/BITA-2 completas
- Optimizados basados en experiencia de estos experimentales

---

**Última Actualización:** 26 Octubre 2025  
**Estado:** ACTIVO - Experimentación  
**Próxima Revisión:** Después de generar 5 documentos usando templates

---

*"Los mejores templates nacen de la iteración, no de la especulación."* 🧩✨
