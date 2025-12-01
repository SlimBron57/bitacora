# 🏛️ SISTEMA DUAL DE BASES DE DATOS: TelescopeDB + VoxelDB

**Ubicación:** `ROADMAP_V2/01_ARQUITECTURA/01_sistema-dual-databases.md`  
**Versión:** 1.5 - ESPECIFICACIÓN PIXEL-NATIVE  
**Fecha:** 26 de Octubre, 2025 | Actualizado: Diciembre 2025  
**Tipo:** SPEC (Concepto + Arquitectura)  
**Prerequisitos:** Leer `14_qpx-quantumdao-revolucion.md` para arquitectura v1.5

---

## ⚡ CAMBIOS v1.5 - PIXEL-NATIVE REVOLUTION

> **🎯 Storage unificado:** Todo se almacena en formato `.qpx` (QPX - Quantum Pixel eXchange)  
> **🌊 QuantumDao workflow:** `main` branch = vida personal, `project` branches = proyectos reales  
> **🔷 Pixel-first:** FBCU Cores y Templates ahora son colecciones de pixels con metadata operacional  
> **🎨 Alpha channel:** Multi-purpose (intensidad, relevancia, progreso, prioridad, etc.)  
> **🔍 PXLang queries:** Symbolic queries (🔍 📊 🎯) + natural language

**Lee primero:** `14_qpx-quantumdao-revolucion.md` para entender la arquitectura revolucionaria v1.5

**Documentación relacionada:**
- `15_pxlang-qpx-query-language.md` - Query language design
- `03_pixel-storage-deep-dive.md` - Storage implementation details
- `02_flujo-datos-end-to-end.md` - Complete data flow

---

## 🎯 VISIÓN GENERAL

Bitácora v1.5 usa **DOS bases de datos complementarias** que operan en **geometrías diferentes** pero **sincronizadas semánticamente** a través de **QPX (Quantum Pixel eXchange)**:

```
┌─────────────────────────────────────────────────────────────────────┐
│              BITÁCORA DUAL HELIX v1.5 (PIXEL-NATIVE)                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  🔭 TelescopeDB                      🧊 VoxelDB                     │
│  (Spherical Memory)                  (Cubic Templates)             │
│                                                                     │
│  ┌────────────────────┐              ┌────────────────────┐        │
│  │   MEMORIA          │              │   TEMPLATES        │        │
│  │   BIOGRÁFICA       │ ←──────────→ │   ACCIONABLES      │        │
│  │                    │   QPX sync   │                    │        │
│  │ (Lo viviste)       │              │ (Lo que haces)     │        │
│  │                    │              │                    │        │
│  │ 📦 Formato: .qpx   │              │ 📦 Formato: .qpx   │        │
│  │ 🎨 Pixels + Metadata              │ 🎨 Pixels + Metadata        │
│  │ 🌊 QuantumDao branches             │ 🌊 Project/Job/Task         │
│  └────────────────────┘              └────────────────────┘        │
│                                                                     │
│  Geometría: Esférica                 Geometría: Cúbica            │
│  Unidad: FBCU Core (pixels)          Unidad: Voxel (templates)    │
│  Índice: Spherical (r, θ, φ)         Índice: Octree (x, y, z)     │
│  Query: Contextual + PXLang          Query: Spatial + PXLang      │
│  Branch: main = vida personal        Branch: project = trabajo    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 🔭 TelescopeDB: La Memoria Esférica

### Metáfora

> **"Un telescopio que mira hacia atrás en tu historia personal"**

Cada experiencia biográfica se almacena como un **FBCU Core** (Fractal-Based Compression Unit) en coordenadas esféricas `(r, θ, φ)`:

- **r (radio):** Intensidad emocional de la experiencia
- **θ (theta):** Categoría temática (personal, social, técnica...)
- **φ (phi):** Valencia emocional (positiva, neutral, negativa)

### Por Qué Geometría Esférica

```
Memoria biográfica NO es linear
  ├─ No es solo pasado → presente → futuro
  ├─ Es multidimensional: emocional, contextual, relacional
  └─ Buscamos por "similitud contextual", no por orden temporal

Geometría esférica permite:
  ✅ Búsqueda radial (por intensidad)
  ✅ Búsqueda angular (por categoría temática)
  ✅ Búsqueda zonal (por valencia emocional)
  ✅ Queries complejas (ej: "experiencias similares en tecnología, positivas")
  ✅ Clustering natural (vecinos esféricos son semánticamente cercanos)
```

### Estructura Conceptual v1.5

**FBCU Core = Unidad mínima de información biográfica (PIXEL-NATIVE)**

```
FBCU Core contiene:
  │
  ├─ 🎨 PixelBlock: Array de pixels (r, g, b, alpha)
  │   └─ r = semántica, g = emocional, h = temporal, alpha = intensidad
  │
  ├─ 📦 QPX Header (48 bytes):
  │   ├─ magic: [0x51, 0x50, 0x58, 0x00] ("QPX\0")
  │   ├─ version: u16
  │   ├─ pixel_count: u32
  │   ├─ branch_id: [u8; 16] (UUID del branch QuantumDao)
  │   └─ timestamp: i64
  │
  ├─ 🌊 QuantumMetadata:
  │   ├─ branch_name: String (e.g., "main", "project:bitacora")
  │   ├─ operational_state: Project/Job/Task metadata
  │   └─ contextual_priority: Priorización dinámica
  │
  ├─ 🔗 Entanglements: Conexiones con otros cores
  │   └─ Relaciones semánticas, temporales, emocionales
  │
  ├─ 📍 Coordenadas esféricas: (r, θ, φ)
  │   └─ Posición en geometría TelescopeDB
  │
  └─ 💾 Storage: archivo .qpx con compresión nativa
```

**Cambios clave v1.5:**
- ✅ FBCU Core ahora es colección de **pixels**, no texto plano
- ✅ Formato `.qpx` unificado (reemplaza CBOR/YAML)
- ✅ QuantumDao `branch_id` integrado (main vs projects)
- ✅ Metadata operacional (Project/Job/Task) en cada core

### Operaciones Conceptuales

#### 1. Insertar una memoria biográfica
```
Usuario dice: "Aprendí Rust en 2025"
     ↓
Bitácora analiza con ContextToken7D
     ↓
Se crea FBCU Core con coordenadas esféricas
     ↓
Se indexa en geometría esférica
     ↓
La memoria es consultable por contexto
```

#### 2. Consultar por contexto
```
Usuario dice: "Ayúdame con Rust"
     ↓
Se analiza intent con ContextToken7D
     ↓
Se buscan FBCU cores cercanos (en espacio esférico)
     ↓
Se retornan memorias relevantes
```

#### 3. Buscar por similaridad
```
Se calcula embedding del input
     ↓
Se buscan cores con embeddings similares (cosine similarity)
     ↓
Se retornan top-N resultados más relevantes
```

---

## 🧊 VoxelDB: El Espacio Cúbico de Templates

### Metáfora

> **"Un cubo de Rubik donde cada posición contiene un patrón de acción"**

Cada template MTT-DSL se almacena en coordenadas cúbicas 3D `(x, y, z)`:

- **x:** Complejidad del template (simple → complicado)
- **y:** Impacto emocional esperado (neutral → profundo)
- **z:** Urgencia temporal (largo plazo → inmediato)

### Por Qué Geometría Cúbica

```
Templates son "formas de hacer" (no memorias)
  ├─ Necesitan buscarse por parámetros clave
  ├─ Tienen jerarquía natural (octree)
  └─ Se aplican a situaciones específicas

Geometría cúbica permite:
  ✅ Búsqueda jerárquica (octree = O(log n))
  ✅ Navegación de vecindad (26 direcciones: caras, aristas, esquinas)
  ✅ Clustering por contexto (voxels cercanos = templates similares)
  ✅ Multi-resolución (ver a diferentes niveles de abstracción)
```

### Estructura Conceptual v1.5

**Voxel = Celda cúbica con templates (PIXEL-NATIVE)**

```
Voxel contiene:
  │
  ├─ 🎨 PixelBlock: Templates codificados como pixels
  │   └─ Cada template = secuencia de pixels (r, g, b, alpha)
  │
  ├─ 📦 QPX Header (48 bytes):
  │   ├─ magic: [0x51, 0x50, 0x58, 0x00]
  │   ├─ template_count: u32
  │   ├─ branch_id: [u8; 16]
  │   └─ voxel_position: (x, y, z) en espacio cúbico
  │
  ├─ 🌊 OperationalMetadata:
  │   ├─ Project/Job/Task: Clasificación operacional
  │   ├─ template_type: MTT-DSL category
  │   └─ usage_count: Estadísticas de aplicación
  │
  ├─ 📍 Coordenadas 3D: (x, y, z)
  │   ├─ x = complejidad (simple → complicado)
  │   ├─ y = impacto emocional (neutral → profundo)
  │   └─ z = urgencia temporal (largo plazo → inmediato)
  │
  ├─ 🔗 Vecinos (26): Conexiones cúbicas
  │   └─ Caras (6), aristas (12), esquinas (8)
  │
  └─ 💾 Storage: archivo .qpx con índice octree
```

**Cambios clave v1.5:**
- ✅ Templates ahora son **pixels**, no YAML/JSON
- ✅ Formato `.qpx` unificado con TelescopeDB
- ✅ Metadata operacional (Project/Job/Task) explícita
- ✅ Branch awareness (project-specific templates)

### Operaciones Conceptuales

#### 1. Insertar un template
```
Template se define con parámetros
     ↓
Se calcula posición cúbica desde parámetros
     ↓
Se inserta en voxel correspondiente
     ↓
Se actualiza octree
     ↓
Se conectan vecinos (26 direcciones)
```

#### 2. Buscar templates por contexto
```
Usuario dice: "Necesito acción urgente"
     ↓
Se analiza con ContextToken7D
     ↓
Se convierte a coordenadas cúbicas
     ↓
Se busca región cúbica en octree
     ↓
Se retornan templates relevantes (top-5)
```

#### 3. Navegar entre templates similares
```
Template actual: debugging_deep_dive
     ↓
Se exploran voxels vecinos (26 direcciones)
     ↓
Se encuentra: debugging_quick_fix (vecino cercano)
     ↓
Usuario puede navegar a template similar
```

---

## 🔄 SINCRONIZACIÓN DUAL-HELIX v1.5 (QPX-NATIVE)

### Principio Fundamental

> **Cada memoria (TelescopeDB) puede estar conectada a templates (VoxelDB), formando un tejido bidireccional de información a través del formato QPX.**

```
FBCU Core (pixels) ←────────→ Template (pixels)
(memoria vivida)               (patrón de acción)

Ejemplo v1.5:
  Memoria: "Debuggeé un error crítico en Rust"
  → FBCU Core en branch "main" (vida personal)
  → PixelBlock con r=tech, g=stress→calm, b=yesterday
  → Alpha=0.9 (alta intensidad emocional)
  
  Templates conectados (VoxelDB):
    - debugging_deep_dive.qpx (project:bitacora)
    - rust_error_patterns.qpx (main)
    - crisis_resolution.qpx (project:bitacora)
  
  Cuando usuario busca "debugging", obtiene:
    ✅ Memorias biográficas (pixels con contexto)
    ✅ Templates aplicados (pixels con patrones)
    ✅ Branch context (main vs project branches)
    ✅ Metadata operacional (Project/Job/Task)
```

### Flujo de Sincronización v1.5 (QuantumDao)

```
1. Usuario ingresa información (en branch "main" o "project:bitacora")
   ↓
2. Bitácora crea FBCU Core en TelescopeDB
   └─ Formato: .qpx con PixelBlock
   └─ QuantumMetadata: branch_id, operational_state
   ↓
3. Bitácora analiza qué templates aplican
   └─ Query VoxelDB con PXLang
   └─ Busca templates en branch actual Y global
   ↓
4. Bitácora conecta core ←→ templates
   └─ EntanglementMap: referencias entre pixels
   └─ BranchTable: seguimiento de qué branch
   ↓
5. Si templates nuevos, se crean en VoxelDB
   └─ Formato: .qpx con template pixels
   └─ Asociados a branch específico o global
   ↓
6. Dual-Helix totalmente sincronizado en QPX
   └─ TelescopeDB + VoxelDB comparten formato
   └─ QuantumDao branches mantienen coherencia
```

### QuantumDao Workflow Integration

```
main branch (vida personal)
  ├─ Memorias cotidianas
  ├─ Templates generales
  └─ Contexto biográfico base

project:bitacora branch
  ├─ Memorias de desarrollo Bitácora
  ├─ Templates específicos del proyecto
  └─ Metadata operacional: tasks, bugs, features

project:renovacion-casa branch
  ├─ Memorias de renovación
  ├─ Templates de construcción
  └─ Timeline de progreso

Operaciones QuantumDao:
  - project create "bitacora" → nuevo branch
  - project merge "bitacora" → integra aprendizajes a main
  - project diff "bitacora" → compara estados
  - project abandon "proyecto-fallido" → archiva branch
```

---

## 📊 COMPARACIÓN CONCEPTUAL v1.5

| Aspecto | TelescopeDB | VoxelDB |
|---------|-------------|---------|
| **Propósito** | Memoria (lo que fue) | Templates (lo que hacer) |
| **Unidad** | FBCU Core (pixels) | Voxel + Templates (pixels) |
| **Formato** | `.qpx` (QPX) | `.qpx` (QPX) |
| **Geometría** | Esférica (r, θ, φ) | Cúbica (x, y, z) |
| **Índice** | Spherical (búsqueda radial) | Octree (búsqueda jerárquica) |
| **Query principal** | "Contexto similar" | "Acción aplicable" |
| **Query language** | PXLang (symbolic + natural) | PXLang (symbolic + natural) |
| **Mutabilidad** | Inmutable (memoria histórica) | Mutable (templates evolucionan) |
| **Escalabilidad** | Millones de cores | Cientos de templates |
| **Búsqueda típica** | ¿Qué he vivido? | ¿Qué debo hacer? |
| **QuantumDao** | branch "main" + "project:X" | branch-aware templates |
| **Metadata** | QuantumMetadata + Project/Job/Task | OperationalMetadata + usage stats |
| **Pixel alpha** | Intensidad emocional | Relevancia contextual |

---

## 🔀 FLUJO CONCEPTUAL END-TO-END

### Escenario v1.5: Usuario pregunta "Ayúdame con debugging"

```
┌─ ENTRADA ───────────────────────────────────────────────────┐
│ "Ayúdame con debugging"                                     │
│ Branch context: project:bitacora                            │
└─────────────────────────────────────────────────────────────┘
                    ↓
┌─ ANÁLISIS (CTX7D + PXLang) ─────────────────────────────────┐
│ ContextToken7D analiza:                                     │
│ - Semántica: "debugging" (r=tech pixel)                    │
│ - Emocional: stress level (g=stress→calm)                  │
│ - Temporal: urgente (b=now)                                │
│ - Alpha: 0.8 (alta intensidad)                             │
│ - Branch: project:bitacora (context)                       │
│                                                             │
│ PXLang query generada:                                     │
│ 🔍 debugging 🎯 r>200 g<100 📊 last:7days 🌊 project:bitacora│
└─────────────────────────────────────────────────────────────┘
         ↙                                    ↘
    [TelescopeDB - QPX]                [VoxelDB - QPX]
         │                                    │
         ↓                                    ↓
    FBCU Cores (pixels):            Templates (pixels):
    - "Debuggeé Rust bug            - debugging_deep_dive.qpx
      crítico" (alpha=0.9)            (project:bitacora)
    - "Fixed panic en TelescopeDB"  - rust_error_patterns.qpx
      (alpha=0.85)                    (main branch, global)
    - "Error de lifetime resuelto"  - crisis_resolution.qpx
      (alpha=0.7)                     (project:bitacora)
         │                                    │
         │  EntanglementMap: cores ←→ templates
         │  BranchTable: project:bitacora context
         └──────────┬─────────────────────────┘
                    ↓
         ┌────────────────────────────────────┐
         │ RESPUESTA INTEGRADA (QPX):         │
         │ - Pixel memories (3 cores)         │
         │ - Pixel templates (2 aplicables)   │
         │ - Branch context (project specific)│
         │ - Alpha weighting (priorización)   │
         │ - PXLang visualization: 📊 graph   │
         └────────────────────────────────────┘
                    ↓
         ┌────────────────────────────────────┐
         │ GUARDAR NUEVA MEMORIA (QPX):       │
         │                                    │
         │ FBCU Core nuevo:                   │
         │ - PixelBlock: r=tech, g=calm,      │
         │   b=2025-12-XX, alpha=0.6          │
         │ - QuantumMetadata:                 │
         │   branch_id: project:bitacora      │
         │   operational_state: Job resolved  │
         │ - Entanglement: → debugging_deep   │
         │ - Storage: memory_debug_help.qpx   │
         │                                    │
         │ Timeline actualizado:              │
         │ - project:bitacora avanza          │
         │ - Stats: debugging attempts++      │
         └────────────────────────────────────┘
```

**Ventajas v1.5:**
- ✅ Todo en formato `.qpx` (unificado)
- ✅ Branch awareness (project:bitacora context)
- ✅ Alpha channel prioriza resultados
- ✅ PXLang query simbólico + natural
- ✅ Metadata operacional (Job resolved)
- ✅ Timeline divergence tracking

---

## 🎯 DECISIONES ARQUITECTÓNICAS v1.5

**Decisiones fundamentales que gobiernan v1.5:**

- **PIXEL-NATIVE:** Todo se almacena como pixels en formato `.qpx` (NO CBOR, NO YAML)
  - ✅ TelescopeDB: FBCU Cores = colecciones de pixels
  - ✅ VoxelDB: Templates = colecciones de pixels
  - ✅ Formato unificado: `.qpx` (QPX - Quantum Pixel eXchange)

- **QUANTUMDAO WORKFLOW:**
  - ✅ `main` branch = vida personal (cotidiana, memoria biográfica)
  - ✅ `project:X` branches = proyectos reales (Bitácora, renovación, etc.)
  - ✅ Project/Job/Task: Metadata operacional integrada
  - ✅ Branches NO son realidades alternativas, son proyectos reales

- **METADATA OPERACIONAL:**
  - ✅ Cada FBCU Core tiene `QuantumMetadata` con branch_id
  - ✅ Cada Template tiene `OperationalMetadata` con Project/Job/Task
  - ✅ Alpha channel: intensidad/relevancia/progreso (contextual)

- **PXLANG INTEGRATION:**
  - ✅ PXLang: Compression + Query + Visualization
  - ✅ Symbolic queries con emojis: 🔍 📊 🎯 ⚡
  - ✅ Complementario a ShuiDao (natural language)

**Decisiones heredadas v1.0 (aún válidas):**
- **DA-007:** TelescopeDB es brecha crítica #1 (prioridad máxima)
- **DA-008:** VoxelDB complementa TelescopeDB (no reemplaza)
- **DA-005:** IDs content-addressable (SHA-256)
- **DA-001:** Local-First (sin dependencia cloud)

**Decisiones obsoletas v1.0:**
- ❌ **DA-003:** CBOR serialización → **REEMPLAZADA por QPX v1.5**

---

## 📚 PRÓXIMA LECTURA

Para entender la arquitectura completa v1.5:

**`14_qpx-quantumdao-revolucion.md`** (MASTER DOCUMENT)
  → Especificación completa de QPX format
  → QuantumDao workflow (branches, projects)
  → Alpha channel 7 use cases
  → Implementación roadmap 8 semanas

**`15_pxlang-qpx-query-language.md`**
  → PXLang: Compression + Query
  → Symbolic queries (🔍 📊 🎯)
  → Integración con ShuiDao

**`03_pixel-storage-deep-dive.md`**
  → Detalles técnicos de pixel storage
  → Algoritmos de compresión
  → Performance optimization

**`02_flujo-datos-end-to-end.md`**
  → Flujo completo desde input hasta storage
  → Integración TelescopeDB ↔ VoxelDB
  → QuantumDao operations

---

## 🔄 CHANGELOG v1.5

**Cambios revolucionarios:**
- ✅ CBOR → QPX (formato pixel-native)
- ✅ YAML templates → Pixel templates
- ✅ QuantumDao workflow (main + project branches)
- ✅ Metadata operacional (Project/Job/Task)
- ✅ PXLang query language
- ✅ Alpha channel multi-purpose
- ✅ `.qpx` formato unificado

**Preservado de v1.0:**
- ✅ Geometría dual (esférica + cúbica)
- ✅ Sincronización Dual-Helix
- ✅ Local-First architecture
- ✅ Content-addressable IDs (SHA-256)

---

*Especificación de arquitectura dual de Bitácora v1.5*  
*"Memoria esférica + Templates cúbicos + Pixels everywhere = Revolución cognitiva"*
