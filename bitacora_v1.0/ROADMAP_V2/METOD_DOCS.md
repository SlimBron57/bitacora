```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/METOD_DOCS.md
Versión: 1.1
Fecha Creación: 2025-11-23
Última Actualización: 2025-11-30 14:10:00
Propósito: Metodología genérica para análisis y reorganización de módulos de documentación
Estado: ACTIVO - Template reutilizable con integración BStradivarius
Autor: Sistema Bitácora + Eduardo
Aplicable A: Cualquier directorio de documentación (00_VISION/, 01_ARQUITECTURA/, 02_COMPONENTES/, etc)
Relación: Precede análisis específicos en TEMP.md de cada módulo
Cambios v1.1: Añadida integración con BStradivarius para auto-documentación continua
BStradivarius: VoxelDB Octree con persistencia espacial, nombres limpios + tags, 6,080+ conceptos indexados
# === FIN DATOS DE AUDITORÍA ===
```

# 🎯 METOD_DOCS v1.0 — Metodología Genérica de Reorganización de Documentación

> **Propósito:** Proporcionar un proceso sistemático y replicable para analizar, limpiar y reorganizar cualquier módulo de documentación en ROADMAP_V2 (o cualquier carpeta de docs).

> **Principio:** "Un análisis, un plan, una ejecución — SIN excepciones."

---

## 📚 TABLA DE CONTENIDOS

1. [Paso 1: Inventario Físico](#paso-1-inventario-físico)
2. [Paso 2: Propósito del Módulo](#paso-2-propósito-del-módulo)
3. [Paso 3: Flujo Lógico Ideal](#paso-3-flujo-lógico-ideal)
4. [Paso 4: Mapeo Actual vs Ideal](#paso-4-mapeo-actual-vs-ideal)
5. [Paso 5: Detección de Problemas](#paso-5-detección-de-problemas)
6. [Paso 6: Plan de Acción](#paso-6-plan-de-acción)
7. [Paso 7: Validación Post-Cambio](#paso-7-validación-post-cambio)
8. [Herramientas Automáticas](#herramientas-automáticas)
9. [Estándares Globales](#estándares-globales)
10. [Checklist de Ejecución](#checklist-de-ejecución)

---

## ✅ PASO 1: INVENTARIO FÍSICO

### Objetivo
Entender exactamente QUÉ documentos hay, cuántos son, cuál es su tamaño y si hay duplicados obvios.

### Ejecución
```bash
# 1. Contar archivos
find [MÓDULO]/ -name "*.md" -type f | wc -l

# 2. Listar con tamaños
ls -lh [MÓDULO]/*.md 2>/dev/null | awk '{print $9, $5}'

# 3. Contar líneas totales
find [MÓDULO]/ -name "*.md" -exec wc -l {} + | tail -1

# 4. Detectar archivos de backup (*.backup, *.old, etc)
find [MÓDULO]/ -name "*.backup" -o -name "*.old" -o -name "*.tmp"

# 5. Detectar archivos sin extensión o con extensión rara
find [MÓDULO]/ -type f ! -name "*.md" ! -name "*.yaml" ! -name ".gitkeep"
```

### Salida esperada
```
📊 INVENTARIO DE [MÓDULO]
├─ Total archivos: X
├─ Total líneas: YYYY
├─ Archivos principales: X documentos únicos
├─ Backups detectados: Y
└─ Archivos raros: Z
```

### Plantilla para documentar
```markdown
## PASO 1: INVENTARIO FÍSICO

**Módulo analizado:** [nombre]  
**Fecha:** 2025-11-23  
**Ejecutor:** [tu nombre]

### Archivos encontrados:
- Total: X archivos
- Únicos: Y documentos
- Backups: Z
- Líneas totales: YYYY

### Detalles:
| Archivo | Líneas | Tipo | Estatus |
|---------|--------|------|---------|
| archivo1.md | 500 | Documento | Principal |
| archivo2.md.backup | 500 | Backup | Duplicado |
```

---

## 🎯 PASO 2: PROPÓSITO DEL MÓDULO

### Objetivo
Definir claramente POR QUÉ existe este módulo y QUÉ debe lograr.

### Preguntas Clave
1. **¿PARA QUÉ existe?** (Propósito primario)
   - Ejemplo: "Definir arquitectura del sistema"
   - Ejemplo: "Especificar componentes técnicos"

2. **¿PARA QUIÉN es?** (Audiencia)
   - Ejemplo: "Desarrolladores Rust"
   - Ejemplo: "Eduardo + AI"
   - Ejemplo: "Arquitectos de sistemas"

3. **¿QUÉ obtienen después de leer?** (Resultado)
   - Ejemplo: "Comprensión de cómo funciona el sistema"
   - Ejemplo: "Capacidad de implementar un componente"
   - Ejemplo: "Validación que el diseño es correcto"

### Plantilla para documentar
```markdown
## PASO 2: PROPÓSITO DEL MÓDULO

**Nombre del módulo:** [00_VISION]

### Propósito Primario
Proporcionar una [visión/especificación/guía/diseño] de [qué aspecto del sistema] 
para que [audiencia] pueda [lograr qué].

### Audiencia
- [ ] Eduardo (propietario del proyecto)
- [ ] Desarrolladores Rust
- [ ] Arquitectos de sistemas
- [ ] Otros: ___________

### Resultado Esperado
Después de leer este módulo, la audiencia debería:
1. [Comprensión/Capacidad/Validación]
2. [Comprensión/Capacidad/Validación]
3. [Comprensión/Capacidad/Validación]

### Métrica de Éxito
- [ ] Flujo lógico del módulo es claro
- [ ] Sin contradicciones entre documentos
- [ ] Todos los temas necesarios están cubiertos
- [ ] Nomenclatura coherente
```

---

## 📊 PASO 3: FLUJO LÓGICO IDEAL

### Objetivo
Definir el ORDEN perfecto en que alguien debería leer los documentos para máxima comprensión.

### Estructura Universal (7 Niveles)

```
NIVEL 1: CONCEPTO/FILOSOFÍA
   ↓ Responde: ¿QUÉ es? ¿Para qué existe?
   
NIVEL 2: PRINCIPIOS/REGLAS/METODOLOGÍA
   ↓ Responde: ¿CÓMO se construye? ¿Cuáles son las reglas?
   
NIVEL 3: DISEÑO/DECISIONES
   ↓ Responde: ¿CUÁLES decisiones se tomaron? ¿POR QUÉ?
   
NIVEL 4: ARQUITECTURA/VISIÓN GENERAL
   ↓ Responde: ¿CÓMO funciona el sistema completo?
   
NIVEL 5: ESPECIFICACIÓN/IMPLEMENTACIÓN
   ↓ Responde: ¿Cómo se implementa? ¿Detalles técnicos?
   
NIVEL 6: VALIDACIÓN/TESTING
   ↓ Responde: ¿FUNCIONA? ¿Se alcanzaron objetivos?
   
NIVEL 7: EVOLUCIÓN/FUTURO
   ↓ Responde: ¿A DÓNDE vamos? ¿Próximos pasos?
```

### Personalización por Módulo
No todos los módulos necesitan los 7 niveles. Adapta según propósito:

**Ejemplo 00_VISION:** Niveles 1-7 (completo)  
**Ejemplo 01_ARQUITECTURA:** Niveles 2-5 (sin filosofía, sin futuro)  
**Ejemplo 05_TESTING:** Niveles 3-6 (sin filosofía, sin decisiones)

### Plantilla para documentar
```markdown
## PASO 3: FLUJO LÓGICO IDEAL

**Módulo:** [nombre]

### Flujo propuesto:
```
NIVEL 1: [Categoría] — [Pregunta que responde]
   └─ Objetivo: [qué entienda el lector]

NIVEL 2: [Categoría] — [Pregunta que responde]
   └─ Objetivo: [qué entienda el lector]

[... continuar ...]
```

### Justificación
- NIVEL 1 debe ir primero porque: [razón]
- NIVEL 2 depende de NIVEL 1 porque: [razón]
- [... etc ...]
```

---

## 🔄 PASO 4: MAPEO ACTUAL VS IDEAL

### Objetivo
Comparar la estructura actual con la ideal, identificar dónde van los documentos.

### Ejecución
1. Listar todos los archivos actuales
2. Clasificar cada uno según propósito
3. Asignar a nivel ideal
4. Detectar gaps (falta crear documentos)

### Plantilla para documentar
```markdown
## PASO 4: MAPEO ACTUAL VS IDEAL

### Tabla de Mapeo

| # | Archivo Actual | Propósito | Nivel Ideal | Nuevo Nombre | Acción |
|---|---|---|---|---|---|
| 1 | archivo1.md | [categoría] | 01 | 01_nuevo-nombre.md | RENOMBRAR |
| 2 | archivo2.md | [categoría] | 03 | 03_nuevo-nombre.md | RENOMBRAR |
| 3 | (no existe) | [categoría] | 04 | 04_nuevo-nombre.md | **CREAR** |
| 4 | archivo3.md | [categoría] | 02 | 02_nuevo-nombre.md | RENOMBRAR |

### Análisis
- Documentos en flujo correcto: X
- Documentos que necesitan renombrar: Y
- Documentos nuevos a crear: Z
- Documentos a excluir: W
```

---

## 🚨 PASO 5: DETECCIÓN DE PROBLEMAS

### Objetivo
Identificar duplicidades, contradicciones, gaps y documentos obsoletos.

### Categorías de Problemas

#### A. DUPLICACIONES
```markdown
### Duplicados detectados:
- [ ] Archivos idénticos: [listar]
- [ ] Archivos con contenido muy similar: [listar]
- [ ] Backups sin eliminar: [listar]

**Acción:** Consolidar A+B → documento único
```

#### B. CONTRADICCIONES
```markdown
### Contradicciones detectadas:
- [ ] Documento A dice X, documento B dice Y (incompatibles)
- [ ] Decisión en DA-001 contradice documento Z
- [ ] Flujo en doc A != flujo en doc B

**Acción:** Revisar, consensuar, actualizar docs
```

#### C. GAPS (Falta documentación)
```markdown
### Gaps identificados:
- [ ] No existe documento sobre [tema]
- [ ] No existe especificación de [componente]
- [ ] No existe validación/testing de [aspecto]

**Acción:** Crear documento nuevo
```

#### D. OBSOLESCENCIA
```markdown
### Documentos obsoletos:
- [ ] Documento X ya no aplica (razón)
- [ ] Documento Y está pospuesto a v2.0
- [ ] Documento Z es referencia histórica

**Acción:** Marcar con "_", archivar o eliminar
```

### Plantilla para documentar
```markdown
## PASO 5: DETECCIÓN DE PROBLEMAS

### A. DUPLICACIONES
- [ ] Archivo1 ≈ Archivo2 (diferencia: _____)
  **Decisión:** CONSOLIDAR en nuevo documento

### B. CONTRADICCIONES
- [ ] Documento A (línea X) vs Documento B (línea Y)
  **Decisión:** REVISAR y actualizar

### C. GAPS
- [ ] Falta documento sobre: [tema]
  **Decisión:** CREAR documento nuevo

### D. OBSOLESCENCIA
- [ ] Documento X es histórico
  **Decisión:** MARCAR con "_"

### Resumen
- Problemas encontrados: X
- Problemas resolubles: Y
- Requiere decisión de Eduardo: Z
```

---

## 📋 PASO 6: PLAN DE ACCIÓN

### Objetivo
Definir EXACTAMENTE qué cambios se harán, en qué orden, y quién lo hace.

### Estructura del Plan
```markdown
## PASO 6: PLAN DE ACCIÓN

### A. RENOMBRAMIENTOS
| De | A | Razón |
|----|---|-------|
| archivo1.md | 01_nuevo-nombre.md | Claridad + orden |
| archivo2.md | 02_nuevo-nombre.md | Claridad + orden |

### B. COMBINACIONES
| Archivos | Resultado | Razón |
|----------|-----------|-------|
| A + B | 01_combinado.md | Consolidar contenido |

### C. CREACIONES
| Nombre | Contenido | Líneas est. |
|--------|-----------|------------|
| 04_nuevo.md | [descripción] | ~600 |

### D. EXCLUSIONES
| Archivo | Nuevo nombre | Razón |
|---------|--------------|-------|
| viejo.md | _viejo.md | Histórico |

### E. ELIMINACIONES
| Archivo | Razón |
|---------|-------|
| backup.old | Duplicado innecesario |

### Resumen de Cambios
- Renombramientos: X
- Combinaciones: Y
- Creaciones: Z
- Exclusiones: W
- Eliminaciones: V
```

---

## ✅ PASO 7: VALIDACIÓN POST-CAMBIO

### Objetivo
Verificar que los cambios son correctos y el módulo es ahora coherente.

### Checklist de Validación

```markdown
## PASO 7: VALIDACIÓN POST-CAMBIO

### ✓ Estructura y Nomenclatura
- [ ] Todos los documentos tienen índice numérico (01_, 02_, etc)
- [ ] No hay gaps en numeración
- [ ] Archivos excluidos tienen prefijo "_"
- [ ] No hay archivos sin índice ni prefijo
- [ ] Nombres son descriptivos en minúsculas con guiones

### ✓ Contenido y Coherencia
- [ ] Flujo lógico es claro (01 → 02 → 03...)
- [ ] No hay duplicación de contenido
- [ ] No hay contradicciones entre documentos
- [ ] Cada documento responde su pregunta clave
- [ ] Referencias internas están actualizadas

### ✓ Completitud
- [ ] Todos los temas necesarios están cubiertos
- [ ] No hay gaps de documentación
- [ ] Documentación de transición es clara
- [ ] Nuevos documentos están bien estructurados

### ✓ Integridad de Enlaces
- [ ] Links en otros módulos apuntan a nuevos nombres
- [ ] README.md del módulo está actualizado
- [ ] DOCS_VALIDATION_*.md está actualizado
- [ ] .structure.yaml existe y es correcto

### ✓ Accesibilidad
- [ ] README.md explica el orden de lectura
- [ ] Índice está claro
- [ ] Propósito del módulo es explícito
- [ ] Audiencia objetivo es clara

### Resultado Final
- [ ] TODO VALIDADO ✅ — Módulo listo
- [ ] PARCIAL VALIDADO ⚠️ — Requiere ajustes
- [ ] FALLÓ VALIDACIÓN ❌ — Requiere revisión completa
```

---

## 🛠️ HERRAMIENTAS AUTOMÁTICAS

### Script 1: analyze_docs.sh

```bash
#!/bin/bash
# Analiza estructura de documentación en un módulo
# Uso: ./analyze_docs.sh ROADMAP_V2/00_VISION

MODULE=$1

if [ -z "$MODULE" ]; then
    echo "❌ Uso: $0 <ruta_módulo>"
    exit 1
fi

echo "════════════════════════════════════════════════"
echo "🔍 ANÁLISIS DE DOCUMENTACIÓN: $MODULE"
echo "════════════════════════════════════════════════"
echo ""

# 1. INVENTARIO
echo "📊 PASO 1: INVENTARIO FÍSICO"
echo "────────────────────────────────────────────────"
TOTAL=$(find $MODULE -name "*.md" -type f | wc -l)
LINES=$(find $MODULE -name "*.md" -exec wc -l {} + | tail -1 | awk '{print $1}')
echo "✓ Total archivos: $TOTAL"
echo "✓ Total líneas: $LINES"
echo ""

# 2. DETECTAR PROBLEMAS
echo "🚨 PASO 5: DETECCIÓN DE PROBLEMAS"
echo "────────────────────────────────────────────────"

echo "Backups detectados:"
find $MODULE -name "*.backup" -o -name "*.old" | wc -l
echo ""

echo "Archivos sin indexar:"
find $MODULE -name "*.md" -not -name "_*" -not -regex ".*[0-9][0-9]_.*" | wc -l
echo ""

echo "Tamaños similares (posibles duplicados):"
find $MODULE -name "*.md" -exec wc -l {} \; | awk '{print $1}' | sort -n | \
  awk 'NR>1 && $1 - prev <= 50 {print "Líneas", prev, "y", $1} {prev=$1}'
echo ""

# 3. ESTRUCTURA
echo "📁 ESTRUCTURA ACTUAL"
echo "────────────────────────────────────────────────"
find $MODULE -maxdepth 1 -name "*.md" -o -name "_*" | sort
echo ""
```

### Script 2: generate_structure_yaml.sh

```bash
#!/bin/bash
# Genera template .structure.yaml para un módulo

MODULE=$1
YAML_FILE="$MODULE/.structure.yaml"

cat > "$YAML_FILE" << 'EOF'
# Metadata y estructura del módulo de documentación

module:
  name: "[NOMBRE_MÓDULO]"
  path: "[RUTA]"
  purpose: "[Para qué existe]"
  audience: ["eduardo", "desarrolladores"]
  
flow:
  - level: 1
    category: "[CATEGORÍA]"
    files: ["01_archivo.md"]
    answers: "¿[Pregunta clave]?"
    depends_on: []
    
validation:
  has_readme: false
  no_duplicates: false
  no_contradictions: false
  complete_flow: false
  indexed_files: false
  
last_updated: "2025-11-23"
last_updated_by: "[Tu nombre]"
status: "PLANIFICACIÓN" # PLANIFICACIÓN | EN_PROGRESO | VALIDADO
EOF

echo "✓ Archivo creado: $YAML_FILE"
```

---

## 📖 ESTÁNDARES GLOBALES

### Convenciones de Nomenclatura

```markdown
## ESTÁNDARES DE NOMENCLATURA

### Índices
- `01_` `02_` `03_` ... — Orden de lectura (números con cero)
- Nivel 5 sub-niveles: `05a_` `05b_` `05c_` — Mismo nivel, diferentes aspectos

### Exclusiones
- `_archivo.md` — Prefijo _ indica: excluido del flujo principal
  - Archivo histórico/backup/referencia
  - Relevante pero no esencial
  - Puede existir pero no se debe leer primero

### Nomenclatura de Archivos
- Todos minúsculas
- Palabras separadas por guiones
- Sin caracteres especiales
- Descriptivo pero conciso
- Ejemplo correcto: `01_filosofia-y-proposito.md`
- Ejemplo incorrecto: `01_Filosofia Y Proposito.md`

### Archivos Especiales
- `README.md` — Índice del módulo (sin número)
- `.structure.yaml` — Metadata del módulo (punto inicial)
- `_[nombre].md` — Excluido (punto bajo inicial)
- `TEMP.md` — Temporal (solo para planificación)
```

### Estructura Interna de Documentos

```markdown
## ESTRUCTURA RECOMENDADA DENTRO DE DOCUMENTOS

### Encabezado (Siempre obligatorio)
\`\`\`yaml
# === DATOS DE AUDITORÍA ===
Archivo: [ruta completa]
Versión: [semver]
Fecha Creación: [YYYY-MM-DD]
Última Actualización: [YYYY-MM-DD]
Propósito: [1 línea clara]
Estado: ACTIVO | BORRADOR | POSPUESTO
Autor: [nombre]
Relación: [dependencias con otros docs]
# === FIN DATOS DE AUDITORÍA ===
\`\`\`

### Tabla de Contenidos
\`\`\`
---

## 📚 TABLA DE CONTENIDOS

1. [Sección A](#sección-a)
2. [Sección B](#sección-b)
...
\`\`\`

### Secciones Principales
- Nivel 1: `# [Título]` — Documento
- Nivel 2: `## [Subtítulo]` — Secciones
- Nivel 3: `### [Tema]` — Subtemas
- Nivel 4: `#### [Detalle]` — Detalles

### Pie de Página
\`\`\`markdown
---

*Documento: [archivo]*  
*Versión: [v]*  
*Estado: [ACTIVO/BORRADOR]*  
*Próxima acción: [si aplica]*
\`\`\`
```

---

## 🎯 CHECKLIST DE EJECUCIÓN

### Para Ejecutar Un Análisis Completo

```markdown
## CHECKLIST: ANÁLISIS DE MÓDULO [NOMBRE]

### PREPARACIÓN
- [ ] Seleccionar módulo a analizar
- [ ] Crear TEMP.md en módulo
- [ ] Ejecutar script analyze_docs.sh
- [ ] Revisar output

### ANÁLISIS (7 PASOS)
- [ ] PASO 1: Inventario Físico
  - [ ] Contar archivos
  - [ ] Listar con tamaños
  - [ ] Detectar backups/duplicados
  
- [ ] PASO 2: Propósito del Módulo
  - [ ] Definir propósito primario
  - [ ] Identificar audiencia
  - [ ] Resultado esperado
  
- [ ] PASO 3: Flujo Lógico Ideal
  - [ ] Definir niveles aplicables
  - [ ] Orden de lectura
  - [ ] Dependencias entre docs
  
- [ ] PASO 4: Mapeo Actual vs Ideal
  - [ ] Clasificar cada documento
  - [ ] Asignar a nivel
  - [ ] Detectar gaps
  
- [ ] PASO 5: Detección de Problemas
  - [ ] Detectar duplicaciones
  - [ ] Detectar contradicciones
  - [ ] Detectar gaps
  - [ ] Detectar obsolescencia
  
- [ ] PASO 6: Plan de Acción
  - [ ] Listar renombramientos
  - [ ] Listar combinaciones
  - [ ] Listar creaciones
  - [ ] Listar exclusiones
  
- [ ] PASO 7: Validación
  - [ ] Validar estructura
  - [ ] Validar coherencia
  - [ ] Validar completitud
  - [ ] Validar enlaces

### DOCUMENTACIÓN
- [ ] Completar TEMP.md del módulo
- [ ] Documentar plan en TEMP.md
- [ ] Obtener aprobación de Eduardo
- [ ] Copiar aprobación en TEMP.md

### IMPLEMENTACIÓN
- [ ] Renombrar archivos
- [ ] Combinar archivos
- [ ] Crear nuevos archivos
- [ ] Marcar exclusiones con "_"
- [ ] Actualizar referencias en otros módulos
- [ ] Actualizar DOCS_VALIDATION_*.md
- [ ] Crear .structure.yaml

### VALIDACIÓN FINAL
- [ ] Ejecutar checklist de PASO 7
- [ ] Actualizar README.md del módulo
- [ ] Verificar todos los links funcionan
- [ ] Obtener validación final de Eduardo

### CIERRE
- [ ] Eliminar TEMP.md o renombrarlo a _TEMP.md
- [ ] Documentar conclusiones
- [ ] Marcar como COMPLETO
```

---

## 📊 EJEMPLO COMPLETO: 00_VISION (YA REALIZADO)

Como referencia, el análisis completo de 00_VISION está documentado en:
**`ROADMAP_V2/00_VISION/TEMP.md`**

Muestra cómo se ejecutan todos los 7 pasos aplicados a un módulo real.

---

## 🚀 FLUJO DE USO

### Para analizar nuevo módulo:

```bash
# 1. Crear TEMP.md en el módulo
touch ROADMAP_V2/[MÓDULO]/TEMP.md

# 2. Ejecutar script
./scripts/analyze_docs.sh ROADMAP_V2/[MÓDULO]

# 3. Documentar en TEMP.md siguiendo los 7 pasos
# (Usar ROADMAP_V2/00_VISION/TEMP.md como referencia)

# 4. Presentar a Eduardo para aprobación

# 5. Ejecutar cambios (cuando Eduardo apruebe)

# 6. Validar (PASO 7)

# 7. Cerrar análisis
```

---

## 📝 NOTAS FINALES

- **Esta metodología es ITERATIVA:** Si en PASO 7 encuentras problemas, vuelve al paso correspondiente.
- **Esta metodología es FLEXIBLE:** Adapta los 7 pasos según tu módulo (no todos necesitan todos).
- **Esta metodología es DOCUMENTADA:** Todo queda en TEMP.md del módulo para auditoría.
- **Esta metodología es VALIDABLE:** Checklist final asegura calidad.

---

## 🎻 INTEGRACIÓN CON BSTRADIVARIUS

### Auto-Documentación Continua

BStradivarius (Meta-Loop System) ahora monitorea y indexa toda la documentación ROADMAP_V2 automáticamente:

**Qué hace:**
```bash
# Sync manual (re-indexa todos los archivos)
./target/release/bstradivarius sync

# Watch continuo (auto-indexa cambios)
./target/release/bstradivarius watch

# Query conceptos
./target/release/bstradivarius query "arquitectura"
```

**Beneficios para METOD_DOCS:**
- ✅ **Validación automática**: Detecta conceptos no documentados
- ✅ **Cross-references**: Encuentra relaciones entre módulos
- ✅ **Stats en tiempo real**: Files watched, concepts indexed
- ✅ **Spatial indexing**: VoxelDB octree para búsquedas semánticas

**Integración con Paso 7 (Validación):**
```bash
# Después de cambios, verificar indexación
./target/release/bstradivarius sync
./target/release/bstradivarius metrics

# Ver qué conceptos se extrajeron del módulo
./target/release/bstradivarius query "[nombre_módulo]"
```

**Arquitectura:**
- VoxelDB Octree: Persistencia espacial 3D de conceptos
- Tags: Metadata (file, line, type) queryable
- Nombres limpios: Conceptos indexados por nombre real
- Load on startup: 6,080+ conceptos disponibles instantáneamente

**Referencia:** `BSTRADIVARIUS_COMPLETE.md`, `GUIA.md` (Sección BStradivarius)

---

*Metodología: METOD_DOCS v1.1*  
*Creada: 2025-11-23*  
*Actualizada: 2025-11-30 (Integración BStradivarius)*  
*Aplicable a: Cualquier módulo de documentación ROADMAP_V2*  
*Estado: ACTIVO - Listo para uso con auto-documentación*
