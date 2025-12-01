# 📖 ARQUITECTURA - Guía de Lectura

**Módulo:** 01_ARQUITECTURA/  
**Propósito:** Explicar la arquitectura técnica de Bitácora v1.0  
**Estado:** ✅ COMPLETO (7 documentos)

---

## 🎯 ¿QUÉ ENCONTRARÁS AQUÍ?

Este módulo explica **HOW** Bitácora construye su sistema:

- ¿Cómo se almacenan las memorias biográficas?
- ¿Cómo se indexan para búsquedas rápidas?
- ¿Cómo se comprimen sin perder información?
- ¿Cómo se conectan memoria + templates?

---

## 📚 ORDEN DE LECTURA RECOMENDADO

### FASE 1: Comprende la Arquitectura Conceptual

#### **1️⃣ `01_sistema-dual-databases.md` (SPEC)**
**Duración:** 15 min  
**Qué es:** La visión general sin código

- ¿Por qué 2 bases de datos?
- GeometrÍa esférica (TelescopeDB) vs cúbica (VoxelDB)
- Metáforas clave: "Telescopio" + "Cubo de Rubik"
- Sincronización dual-helix

**Pregunta clave:** *"¿Cuál es la diferencia conceptual entre TelescopeDB y VoxelDB?"*

---

#### **2️⃣ `02_flujo-datos-end-to-end.md` (SPEC)**
**Duración:** 20 min  
**Qué es:** El pipeline completo de datos

- Input → Sensory Engine → CTX7D → Databases → Storage
- Ejemplo: Usuario pregunta "Ayúdame con debugging"
- ¿Cómo fluyen datos de extremo a extremo?

**Pregunta clave:** *"¿Qué sucede desde que un usuario escribe algo hasta que se almacena?"*

---

### FASE 2: Aprende la Implementación Técnica

#### **3️⃣ `01a_sistema-dual-databases-implementation.md` (IMPL)**
**Duración:** 20 min  
**Qué es:** Código Rust real + performance targets

- Structs: TelescopeDB, VoxelDB, FBCU Core, Voxel
- Operaciones: Insert, Query, Navigation
- Performance targets (latencia, compresión, storage)
- Milestones de implementación (3 fases, 6 semanas)

**Pregunta clave:** *"¿Cómo se implementa en Rust el concepto dual?"*

---

#### **4️⃣ `03_pixel-storage-deep-dive.md` (IMPL)**
**Duración:** 15 min  
**Qué es:** Cómo se codifica información como píxeles

- Información multidimensional → Arrays de píxeles
- Compresión visual extrema (99.999%)
- Encoding dimensional → RGB mapping
- Reutilización de quantum visual compressor

**Pregunta clave:** *"¿Cómo se puede almacenar información como imágenes?"*

---

#### **5️⃣ `04_content-addressable-ids.md` (IMPL)**
**Duración:** 15 min  
**Qué es:** Sistema de IDs basado en contenido (SHA-256)

- Content-addressable significa: ID = hash(contenido)
- ¿Por qué SHA-256? (deduplicación, verificabilidad, distribuibilidad)
- Cómo se calcula y se valida

**Pregunta clave:** *"¿Por qué el ID debe ser el hash del contenido?"*

---

#### **6️⃣ `05_cbor-serialization.md` (IMPL)**
**Duración:** 15 min  
**Qué es:** Formato binario canónico para serialización

- CBOR vs JSON vs MessagePack (por qué CBOR gana)
- Tipos de datos CBOR (integers, strings, arrays, maps)
- Implementación en Rust con `serde_cbor`
- Por qué es crítico para content-addressable IDs

**Pregunta clave:** *"¿Por qué CBOR es mejor que JSON para Bitácora?"*

---

### FASE 3: Las 7 Capas Arquitectónicas Completas

#### **7️⃣ `06_sensory-engine-y-ctx7d.md` (CAPA 1: CAPTURA)**
**Duración:** 25 min  
**Qué es:** Cómo el sistema captura y normaliza input

- CTX7D: Tensor 7-dimensional (semántica, emocional, temporal, relacional, causal, propósito, certeza)
- ¿Por qué 7 dimensiones exactamente? (Validado en AVA)
- Sensory Engine: Análisis multimodal → CTX7D
- Struct ContextToken7D + métodos (from_text, blend, distance)

**Pregunta clave:** *"¿Cómo se captura la esencia de un input en 7 números?"*

---

#### **8️⃣ `07_fbcu-y-flowpacks.md` (CAPA 2: COMPRESIÓN)**
**Duración:** 25 min  
**Qué es:** Compresión fractal extrema (99.999%) + organización contextual

- FBCU: Fractal Binary Compression Unit (IFS, Iterated Function Systems)
- FlowPacks: DAGs de contexto agrupado
- ¿Por qué 99.999%? Breakdown matemático
- Quadtree adaptativo, búsqueda de transformaciones afines

**Pregunta clave:** *"¿Cómo se logra 99.999% de compresión sin perder información?"*

---

#### **9️⃣ `08_indexacion-embeddings-hnsw.md` (CAPA 4: INDEXACIÓN)**
**Duración:** 25 min  
**Qué es:** Indexación semántica para búsquedas rápidas

- Embeddings: MiniLM-L6-v2 (384 dimensiones)
- HNSW: Hierarchical Navigable Small World (búsqueda O(log n))
- Similitud coseno + normalización L2
- ¿Por qué MiniLM? (velocidad vs precisión vs tamaño)

**Pregunta clave:** *"¿Cómo encontrar contextos similares entre millones sin compararlos todos?"*

---

#### **🔟 `09_reconocimiento-patrones.md` (CAPA 5: RECONOCIMIENTO)**
**Duración:** 25 min  
**Qué es:** Detección de patrones, ciclos y evolución emocional

- ConversationGraph: Grafo de contextos + relaciones
- Similitud avanzada: Coseno + temporal + emocional
- Floyd's Cycle Detection: Identifica "disco rayado"
- Estadísticas emocionales: Trend, volatilidad, progresión

**Pregunta clave:** *"¿Cómo sabe Bitácora que el usuario repite la misma pregunta una y otra vez?"*

---

#### **1️⃣1️⃣ `10_routier-y-hubspoke.md` (CAPA 6: AMPLIFICACIÓN)**
**Duración:** 25 min  
**Qué es:** Orquestación inteligente de respuesta multi-LLM

- Routier: Motor de decisiones (BreakCycle, ReinforceProgress, StabilizeEmotion, ProvideCertainty)
- HubSpoke: Orquestador multi-LLM (GPT-4o, Claude, Mistral 8x7B, Phi-3)
- Failover automático con reintentos
- Inyección de contexto en prompts (CAPA 5 + CTX7D → prompt enriquecido)

**Pregunta clave:** *"¿Cómo decide Bitácora qué LLM usar y cómo adaptar el prompt?"*

---

#### **1️⃣2️⃣ `11_respuesta-adaptada-llm.md` (CAPA 7: RESPUESTA)**
**Duración:** 25 min  
**Qué es:** Personalización final de respuesta (voz única)

- PersonalizationEngine: Extrae contexto biográfico + preferencias
- Inyección de hechos/momentos clave (no repite, usa historia)
- Adaptación de tono: Empático → Motivacional basado en emocional
- Ajuste de longitud: Urgente = corto, reflexivo = largo

**Pregunta clave:** *"¿Cómo hace Bitácora que se sienta como SI TE CONOCE?"*

---

#### **1️⃣8️⃣ `18_metabolic-digestion-system.md` (PHASE 7.x: DATA IMPORT)** ⭐ NUEVO
**Duración:** 45 min  
**Qué es:** Sistema de importación de datos externos con digestión metabólica

- 5-phase pipeline: Quarantine → Digest → Extract → Validate → Distribute
- Hybrid architecture: Core (hard-coded) + Logic (templated)
- Source-specific digesters: WhatsApp, Telegram, Email, Spotify, GitHub
- Hyperlink Intelligence: URL analysis para consumption patterns
- Template-driven evolution: YAML rules sin recompilar

**Pregunta clave:** *"¿Cómo Bitácora logra onboarding de 30s importando datos externos?"*

---

## 🎓 CASOS DE USO POR PERFIL

### Si Eres: **Arquitecto**
Lectura mínima: `01_` + `02_` (30 min)
Luego: Saltear a detalles que te interesen

### Si Eres: **Developer Implementando TelescopeDB**
Lectura: `01_` + `01a_` + `04_` + `05_` + `03_` (60 min)
Luego: Código en src/telescope_db/

### Si Eres: **Developer Implementando VoxelDB**
Lectura: `01_` + `01a_` (35 min)
Luego: Código en src/voxel_db/

### Si Eres: **LLM Futuro analizando Bitácora**
Lectura: TODO, en orden (80 min)
Entenderás arquitectura completa

---

## ✅ CHECKLIST DE COMPRENSIÓN

Después de leer este módulo, deberías poder explicar:

- [ ] **Concepto:** ¿Qué problema resuelve la dual-DB?
- [ ] **Geometría:** ¿Por qué esférica vs cúbica?
- [ ] **Operaciones:** ¿Cómo se inserta y consulta?
- [ ] **Performance:** ¿Cuáles son los targets de latencia?
- [ ] **Sincronización:** ¿Cómo se conectan TelescopeDB y VoxelDB?
- [ ] **Serialización:** ¿Por qué CBOR + SHA-256?
- [ ] **Implementación:** ¿En qué orden se implementan los 3 milestones?

**Si respondes SÍ a todos:** ✅ Dominas 01_ARQUITECTURA

---

## 🔗 REFERENCIAS CRUZADAS

**Prerequisitos (lee antes de aquí):**
- `00_VISION/03_decisiones-arquitectonicas.md` - DA que gobiernan
- `00_VISION/05a_bita-1-fbcu-specification.md` - FBCU spec
- `00_VISION/05b_bita-2-aca-7d-specification.md` - CTX7D spec

**Continuación (lee después):**
- `02_COMPONENTES/` - Componentes específicos
- `03_INTEGRACION/` - Cómo se integra con sensory engine
- Código en `src/` - Implementación real

---

## 📊 ESTRUCTURA ACTUAL

```
01_ARQUITECTURA/
├─ 01_sistema-dual-databases.md (250 líneas, SPEC)
├─ 01a_sistema-dual-databases-implementation.md (350 líneas, IMPL)
├─ 02_flujo-datos-end-to-end.md (786 líneas, SPEC)
├─ 03_pixel-storage-deep-dive.md (590 líneas, IMPL)
├─ 04_content-addressable-ids.md (806 líneas, IMPL)
├─ 05_cbor-serialization.md (784 líneas, IMPL)
└─ README.md (este archivo)

Total: 4,166 líneas de documentación arquitectónica
```

---

## 🎯 DECISIONES ARQUITECTÓNICAS CLAVE

Todos los documentos en este módulo respetan:

- **DA-001:** Local-First (sin cloud)
- **DA-003:** CBOR serialization (no JSON)
- **DA-005:** Content-addressable IDs (SHA-256)
- **DA-007:** TelescopeDB es brecha crítica
- **DA-008:** VoxelDB complementa TelescopeDB

---

## 🚀 CÓMO USAR ESTA DOCUMENTACIÓN

1. **Primera vez:** Lee TODO en orden (80 min)
2. **Después:** Usa como referencia por tema:
   - Buscas algoritmo de query → `01a_`
   - Buscas performance targets → `01a_`
   - Buscas explicación conceptual → `01_` + `02_`
   - Buscas formato serialización → `05_`
   - Buscas encoding visual → `03_`

---

*"Dos geometrías, una inteligencia: memoria biográfica + templates accionables"*

*Última actualización: 2025-11-23*  
*Estado: ✅ COMPLETO*
