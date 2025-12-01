```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/00_VISION/DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md
Versión: 1.1
Fecha Creación: 2025-11-24
Última Actualización: 2025-11-24
Autor: Eduardo + B (Sistema Bitácora)
Propósito: Decisión Arquitectónica #33 - Sistema Dinámico de Topics y Tonos
Estado: ACTIVO - CRÍTICO para personalización real
Relacionado Con: 
  - DA-032 (ShuiDao)
  - DA-034 (Small World Networks en Routier)
  - MTT-DSL Templates
  - VoxelDB
Impacto: FUNDAMENTAL - Define diferencia entre asistente genérico vs compañero personal
Nota v1.1: Simplificado para enfocarse en detection. Navegación movida a DA-034 (Routier)
# === FIN DATOS DE AUDITORÍA ===
```

# DA-033: Sistema Dinámico de Topics y Tonos Personalizados

> **Decisión Central**: Reemplazar enums estáticos por sistema dinámico que aprende Topics y Tonos únicos de cada usuario.

---

## 🎯 CONTEXTO

### El Problema Detectado (2025-11-24)

Durante implementación de ShuiDao Phase 3b, detectamos HARDCODING crítico:

**TopicAnalyzer actual:**
```rust
pub enum TopicDomain {
    Infrastructure,  // ¿Y si usuario habla de Cerámica?
    Software,        // ¿Y si usuario habla de Armas?
    Learning,        // Muy genérico
    Personal,        // TODO es personal
    Psychology,
    Biography,
    Routine,
}
```

**Problemas:**
1. ❌ Solo 7 categorías fijas para TODA la vida de una persona
2. ❌ No aprende interests únicos del usuario
3. ❌ Igual para todos (Eduardo = Esposa = cualquier usuario)
4. ❌ No se adapta a vocabulario personal
5. ❌ **Contradice filosofía MTT-DSL** (templates dinámicos)

**ToneDetector actual:**
```rust
pub enum ToneType {
    Pragmatic,
    Curious,
    Frustrated,
    Casual,
    Reflective,
}
```

**Problemas similares:**
1. ❌ Solo 5 tonos fijos para TODAS las emociones humanas
2. ❌ ¿Y si usuario está "determinado"? ¿"nostálgico"? ¿"emocionado"?
3. ❌ No captura matices personales de expresión
4. ❌ Modelo discreto vs realidad continua

### La Revelación del Usuario (Eduardo)

> "A mí me gusta aprender de muchas cosas: cocina, cerámica, historia, filosofía, computación, espiritualidad, microprocesadores... Mi esposa: autos, armas, manualidades, escritura, contenido digital... **¿Cómo identifica Bitácora estos topics sin hardcodearlos?**"

> "Juntos pero no revueltos - no mezclar Espiritualidad con Microprocesadores a menos que yo lo conecte explícitamente."

**Insight crítico:** Cada usuario es un **universo único de interests**. Sistema hardcoded = imposible escalar.

---

## 🚀 LA DECISIÓN

### DA-033: Dynamic Topic & Tone System

**ELIMINAR:**
- ❌ `TopicDomain` enum estático
- ❌ `ToneType` enum estático

**REEMPLAZAR CON:**
- ✅ `TopicGraph` - Grafo dinámico personal por usuario
- ✅ `EmotionalSpace` - Espacio continuo de tonos emocionales
- ✅ Templates MTT-DSL para persistencia
- ✅ Aprendizaje incremental de nuevo topics/tones

---

## 📐 ARQUITECTURA

> **⚠️ NOTA IMPORTANTE (v1.1):** Navegación entre topics (shortest paths, hubs, Small World Networks) ha sido movida a **Routier Navigator** (DA-034). TopicGraph se enfoca SOLO en **detection** (<15ms).

### Separación de Responsabilidades

```yaml
TopicGraph (este documento):
  Responsabilidad: "¿De QUÉ habla el usuario?"
  Scope: Topic detection con MiniLM embeddings
  Performance: <15ms (HOT PATH, cada mensaje)
  Algoritmos: Embeddings, cosine similarity, learning
  
Routier Navigator (DA-034):
  Responsabilidad: "¿CÓMO conectar conceptos?"
  Scope: Small World Networks, navegación inteligente
  Performance: Background/offline (NO bloquea detection)
  Algoritmos: Shortest path, PageRank, community detection
```

### 1. TopicGraph (Sistema Dinámico de Topics - DETECTION ONLY)

```rust
pub struct TopicGraph {
    user_id: String,
    
    // Topics definidos dinámicamente por usuario
    root_topics: HashMap<TopicId, RootTopic>,     // Nivel 1: Grandes áreas
    sub_topics: HashMap<TopicId, SubTopic>,       // Nivel 2: Especialidades
    micro_topics: HashMap<TopicId, MicroTopic>,   // Nivel 3: Ultra-específico
    
    // Pesos de interés personalizados
    weights: HashMap<TopicId, InterestWeight>,
    
    // Embeddings para matching semántico (MiniLM-L6-v2, 384D)
    embeddings: HashMap<TopicId, Vec<f32>>,
    
    // Templates MTT-DSL
    templates: HashMap<TopicId, String>,  // Path a template VoxelDB
    
    // Encoder (local, no network)
    embedder: MiniLMEncoder,  // 80MB model, ~10ms per encoding
}

pub struct RootTopic {
    id: TopicId,
    name: String,  // "Cocina", "Tecnología", "Cerámica" - USER DEFINED
    user_defined: bool,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    interaction_count: u32,
    sentiment_average: f32,  // ¿Le gusta o frustra este topic?
    parent: Option<TopicId>,
}

pub struct InterestWeight {
    explicit: f32,    // Usuario dijo "me encanta X" (0.0-1.0)
    implicit: f32,    // Frecuencia de menciones (0.0-1.0)
    temporal: f32,    // Decae con tiempo sin uso (0.0-1.0)
    emotional: f32,   // Sentimiento asociado (0.0-1.0)
    
    pub fn combined(&self) -> f32 {
        (self.explicit * 0.4 + 
         self.implicit * 0.3 + 
         self.temporal * 0.2 + 
         self.emotional * 0.1).clamp(0.0, 1.0)
    }
}
```

**Ejemplo: Eduardo's TopicGraph**

```
Eduardo (user_id: "eduardo_001"):
├─ Cocina (0.85) ← InterestWeight combinado
│  ├─ Recetas Tradicionales (0.90)
│  │  └─ Cocina Mexicana (0.95)
│  ├─ Repostería (0.70)
│  └─ Fermentación (0.60)
│
├─ Artesanía (0.75)
│  └─ Cerámica (0.85)
│     ├─ Técnicas de Torno (0.80)
│     └─ Esmaltes (0.70)
│        └─ [EDGE → Química: 0.85]  ← Conexión detectada
│
├─ Tecnología (0.95)
│  ├─ Programación (0.98)
│  │  ├─ Rust (1.00) 🔥 ← Máximo interés
│  │  ├─ AI/ML (0.90)
│  │  └─ Arquitectura Software (0.95)
│  ├─ Microprocesadores (0.70)
│  └─ Hardware DIY (0.65)
│
├─ Filosofía (0.80)
│  ├─ Filosofía Oriental (0.85)
│  │  └─ [EDGE → Espiritualidad: 0.90]
│  └─ Epistemología (0.75)
│
└─ Espiritualidad (0.70)
   ├─ Meditación (0.75)
   └─ [EDGE → Filosofía Oriental: 0.90]
```

### 2. EmotionalSpace (Sistema Dinámico de Tonos)

```rust
pub struct EmotionalSpace {
    user_id: String,
    
    // Modelo VAD (Valence-Arousal-Dominance) + Formality
    dimensions: ToneDimensions,
    
    // Clusters de tonos descubiertos
    clusters: HashMap<ToneClusterId, ToneCluster>,
    
    // Templates MTT-DSL
    templates: HashMap<ToneClusterId, String>,
}

pub struct ToneDimensions {
    valence: f32,     // -1.0 (negativo) a 1.0 (positivo)
    arousal: f32,     // -1.0 (calmado) a 1.0 (excitado)
    dominance: f32,   // -1.0 (sumiso) a 1.0 (dominante)
    formality: f32,   // -1.0 (casual) a 1.0 (formal)
}

pub struct ToneCluster {
    id: ToneClusterId,
    name: String,  // "Determinado", "Nostálgico" - USER NAMED
    center: ToneDimensions,
    radius: f32,  // Tolerancia para matching
    examples: Vec<String>,
    user_id: String,
    discovered_at: DateTime<Utc>,
}
```

**Ejemplo: Eduardo's ToneCluster "Determinado"**

```yaml
cluster_id: "tone_eduardo_determinado_001"
name: "Determinado"
user_id: "eduardo_001"
discovered_from: "Voy a terminar este proyecto cueste lo que cueste"

dimensions:
  valence: 0.3      # Ligeramente positivo
  arousal: 0.7      # Alta energía
  dominance: 0.8    # Muy asertivo
  formality: 0.5    # Neutral
  
radius: 0.2

lexical_markers:
  - "voy a"
  - "cueste lo que cueste"
  - "terminar"
  - "definitivamente"
  - "sin excusas"
  
examples:
  - "Voy a terminar este proyecto cueste lo que cueste"
  - "Lo voy a hacer, no hay alternativa"
  - "Esto se completa hoy sí o sí"
```

### 3. Topic Isolation ("Juntos pero no revueltos")

```rust
pub struct TopicBoundary {
    isolation_mode: IsolationMode,
    cross_references: Vec<TopicCrossRef>,
}

pub enum IsolationMode {
    Strict,      // NUNCA mezclar (Armas ⊥ Cocina)
    Soft,        // Permitir si usuario conecta (Cerámica ↔ Química)
    Exploratory, // Sugerir conexiones pero pedir confirmación
}

pub struct TopicCrossRef {
    topic_a: TopicId,
    topic_b: TopicId,
    connection_type: ConnectionType,
    strength: f32,
    user_approved: bool,
}

pub enum ConnectionType {
    Complementary,  // Cerámica + Química (esmaltes)
    Hierarchical,   // Rust ⊂ Programación ⊂ Tecnología
    Sequential,     // Aprender X antes de Y
    Contrasting,    // Comparar/contrastar
    Forbidden,      // Usuario dijo "nunca mezclar"
}
```

---

## 🔄 FLUJO DE APRENDIZAJE

### Auto-Discovery de Nuevo Topic

```
1. Usuario: "Me gustaría aprender sobre fermentación de vegetales"

2. TopicDetector:
   - Genera embedding del texto
   - Busca en TopicGraph de usuario
   - No encuentra match > 0.6 threshold
   
3. Sistema: "Detecto que hablas de algo nuevo: 'fermentación'. 
            ¿Es un nuevo interés tuyo?"
            
4. Usuario: "Sí, quiero aprender a hacer kimchi"

5. TopicGraph:
   - Crea RootTopic("Fermentación", parent=Some("Cocina"))
   - InterestWeight { explicit: 0.8 (expresó interés), ... }
   - Genera embedding
   - Crea template MTT-DSL
   - Guarda en VoxelDB
   
6. Sistema: "Perfecto, agregué 'Fermentación' a tus topics de Cocina.
            ¿Quieres que sugiera recursos de aprendizaje?"
```

### Auto-Discovery de Nuevo Tone

```
1. Usuario: "¡Estoy tan emocionado por este proyecto! 
            No puedo esperar a empezar"

2. ToneDetector:
   - Calcula dimensions:
     * valence: 0.9 (muy positivo)
     * arousal: 0.8 (alta energía)
     * dominance: 0.6 (moderado)
     * formality: 0.3 (casual)
   - Busca cluster cercano
   - No encuentra (distancia > 0.5)
   
3. Sistema: "Detecto un tono nuevo en ti: muy positivo y energético.
            ¿Cómo lo describirías? (ej: emocionado, entusiasmado)"
            
4. Usuario: "Emocionado"

5. EmotionalSpace:
   - Crea ToneCluster("Emocionado", center=dimensions)
   - Guarda ejemplos de frases
   - Crea template MTT-DSL
   - Guarda en VoxelDB
   
6. Sistema: "Entendido. Cuando detecte ese tono en el futuro,
            ajustaré mis respuestas para mantener tu energía."
```

---

## 🎨 INTEGRACIÓN MTT-DSL

### Template: user_topics/eduardo/ceramica.yaml

```yaml
metadata:
  name: "Cerámica"
  created_by: "eduardo_001"
  created_at: "2025-11-24T13:45:00Z"
  version: "1.0.0"
  parent_topic: "Artesanía"
  
detection:
  keywords:
    - "torno"
    - "arcilla"
    - "esmalte"
    - "cocción"
    - "barro"
    - "glaseado"
    
  phrases:
    - "hacer una vasija"
    - "temperatura del horno"
    - "mezclar esmaltes"
    - "centrar arcilla"
    
  embedding_similarity_threshold: 0.75
  
interest_weight:
  explicit: 0.85
  implicit: 0.70
  temporal: 0.90
  emotional: 0.80
  combined: 0.81
  
response_style:
  formality: 0.3         # Casual pero técnico
  detail_level: 0.8      # Mucho detalle
  include_examples: true
  include_visuals: true
  tone_adaptation: "practical_enthusiast"
  
related_topics:
  - topic: "Química"
    connection: "Complementary"
    reason: "Esmaltes requieren conocimiento químico"
    strength: 0.85
    
  - topic: "Arte"
    connection: "Hierarchical"
    reason: "Cerámica es forma de arte"
    strength: 0.70
    
learning_path:
  current_level: "Intermediate"
  topics_completed:
    - "Técnicas básicas de torno"
    - "Preparación de arcilla"
  topics_next:
    - "Esmaltes avanzados"
    - "Cocción de alta temperatura"
```

### Template: user_tones/eduardo/determinado.yaml

```yaml
metadata:
  name: "Determinado"
  user_id: "eduardo_001"
  discovered_at: "2025-11-24T13:50:00Z"
  discovered_from: "Voy a terminar este proyecto cueste lo que cueste"
  version: "1.0.0"
  
dimensions:
  valence: 0.3
  arousal: 0.7
  dominance: 0.8
  formality: 0.5
  
cluster:
  center: [0.3, 0.7, 0.8, 0.5]
  radius: 0.2
  
lexical_markers:
  strong_verbs:
    - "voy a"
    - "terminar"
    - "completar"
    - "lograr"
    
  commitment_phrases:
    - "cueste lo que cueste"
    - "sin excusas"
    - "definitivamente"
    - "sí o sí"
    
  time_markers:
    - "hoy"
    - "ahora"
    - "ya"
    
syntactic_patterns:
  - pattern: "voy a <verb> <complement> cueste lo que cueste"
    confidence: 0.95
  - pattern: "esto se <verb> <time_marker> sí o sí"
    confidence: 0.90
    
response_adaptation:
  style: "direct_supportive"
  energy_level: 0.8
  encouragement: true
  challenge_acceptance: true
  no_excuses: true
  actionable_steps: true
  
  tone_adjustments:
    - "Responder con mismo nivel de energía"
    - "No ofrecer alternativas (usuario ya decidió)"
    - "Proveer pasos concretos inmediatos"
    - "Celebrar progreso sin condescendencia"
```

---

## 💾 PERSISTENCIA EN VOXELDB

### Estructura de Storage

```
VoxelDB/
  users/
    eduardo_001/
      topics/
        cocina.yaml
        ceramica.yaml
        tecnologia.yaml
        rust.yaml
        ...
      tones/
        determinado.yaml
        nostalgico.yaml
        curioso.yaml
        ...
      graph/
        topic_edges.json
        weights.json
        
    esposa_001/
      topics/
        autos.yaml
        armas.yaml
        escritura.yaml
        ...
```

### CubicCoords Mapping

```rust
// Topics mapeados en espacio cúbico [0,1]³
pub fn map_topic_to_voxel(topic: &RootTopic) -> CubicCoords {
    CubicCoords {
        x: hash_domain(&topic.name) / MAX_HASH,  // Dominio
        y: topic.weights.combined,                // Interés
        z: topic.temporal_recency(),              // Recencia
    }
}

// Búsqueda espacial eficiente con Octree
impl VoxelDB {
    pub fn find_similar_topics(&self, query_topic: &str) -> Vec<(TopicId, f32)> {
        let query_coords = self.embed_and_map(query_topic);
        self.octree.nearest_neighbors(query_coords, k=10)
    }
}
```

---

## 🎯 IMPLEMENTACIÓN

### Fase 1: Topic System (8-12h)

**Archivos a crear:**

1. `src/shuidao/topic_graph.rs` (~500 líneas)
   - TopicGraph struct
   - TopicDetector
   - TopicBoundary
   - InterestWeight calculations

2. `src/shuidao/topic_learning.rs` (~400 líneas)
   - Auto-discovery logic
   - User confirmation flow
   - Template generation

3. `src/shuidao/topic_integration.rs` (~300 líneas)
   - VoxelDB persistence
   - MTT-DSL template parsing
   - TelescopeDB biographical links

**Refactors necesarios:**

1. `src/shuidao/intention_detector.rs`
   - ELIMINAR `TopicDomain` enum
   - REEMPLAZAR con `DynamicTopicAnalyzer`
   - Integration con TopicGraph

### Fase 2: Tone System (8-12h)

**Archivos a crear:**

1. `src/shuidao/emotional_space.rs` (~450 líneas)
   - EmotionalSpace struct
   - ToneDimensions (VAD+F model)
   - ToneCluster management

2. `src/shuidao/tone_learning.rs` (~400 líneas)
   - Auto-discovery logic
   - Dimensional analysis
   - Template generation

**Refactors necesarios:**

1. `src/shuidao/intention_detector.rs`
   - ELIMINAR `ToneType` enum
   - REEMPLAZAR con `DynamicToneDetector`
   - Integration con EmotionalSpace

### Fase 3: Integration & Testing (6-8h)

1. VoxelDB persistence layer
2. MTT-DSL template system
3. User confirmation UI/API
4. Migration de datos existentes
5. Tests unitarios (50+ tests)
6. Tests de integración (10+ scenarios)

**Total estimado:** 22-32h (3-4 días de trabajo efectivo)

---

## 📊 MÉTRICAS DE ÉXITO

### Quantitativas

1. **Topic Coverage**: Usuario puede definir Topics ILIMITADOS
2. **Learning Accuracy**: >90% de nuevos topics detectados correctamente
3. **Tone Recognition**: >85% accuracy en dimensiones VAD+F
4. **Response Time**: <50ms para topic/tone detection
5. **Storage Efficiency**: <10KB por topic template

### Qualitativas

1. **Personalización**: Respuestas adaptadas a interest weights
2. **Isolation**: "Juntos pero no revueltos" validado por usuario
3. **Learning**: Sistema mejora con cada interacción
4. **UX**: Usuario siente que Bitácora "lo entiende"
5. **Escalabilidad**: N usuarios con M topics c/u = O(1) performance

---

## 🔗 DECISIONES RELACIONADAS

- **DA-032**: ShuiDao - Intention-Oriented Cognitive Architecture
- **DA-016**: MTT-DSL - Templates como bloques LEGO
- **DA-011**: Local-first architecture (VoxelDB storage)
- **DA-001**: Sistema dual TelescopeDB + VoxelDB

---

## ✅ VALIDACIÓN

### Criterios de Aceptación

1. ✅ Usuario puede crear topic "Cerámica" dinámicamente
2. ✅ Sistema detecta topic "Cerámica" en mensajes futuros (>0.75 confidence)
3. ✅ Respuestas adaptadas a interest_weight (Rust 1.0 vs Microprocesadores 0.7)
4. ✅ Topics aislados (Espiritualidad ⊥ Microprocesadores) unless connected
5. ✅ Usuario puede nombrar tono "Determinado" con sus características
6. ✅ Sistema adapta respuestas según dimensiones VAD+F
7. ✅ Templates persistidos en VoxelDB y recuperables
8. ✅ Performance <50ms detection end-to-end

### Test Cases

```rust
#[test]
fn test_dynamic_topic_creation() {
    let mut graph = TopicGraph::new("eduardo_001");
    
    // Usuario menciona nuevo topic
    let text = "Me gustaría aprender sobre fermentación de kimchi";
    let detection = graph.detect_topics(text);
    
    assert!(detection.is_new_topic);
    assert_eq!(detection.suggested_name, Some("Fermentación"));
    
    // Usuario confirma
    graph.learn_new_topic("Fermentación", text, parent=Some("Cocina"));
    
    // Verificar futuras detecciones
    let text2 = "Cómo hacer kimchi en casa";
    let detection2 = graph.detect_topics(text2);
    
    assert!(detection2.matched_topics.contains("Fermentación"));
    assert!(detection2.confidence > 0.75);
}

#[test]
fn test_topic_isolation() {
    let graph = TopicGraph::new("eduardo_001");
    graph.add_topic("Espiritualidad", isolation=IsolationMode::Strict);
    graph.add_topic("Microprocesadores", isolation=IsolationMode::Strict);
    
    let text = "La espiritualidad de los microprocesadores"; // Forzado
    let should_mix = graph.boundary.should_mix("Espiritualidad", "Microprocesadores");
    
    assert_eq!(should_mix, false);
    // Sistema debería pedir confirmación o separar topics
}

#[test]
fn test_tone_dimensions_detection() {
    let detector = DynamicToneDetector::new();
    
    let text = "¡Voy a terminar este proyecto cueste lo que cueste!";
    let tone = detector.detect(text, "eduardo_001");
    
    assert!(tone.dimensions.valence > 0.2);   // Positivo
    assert!(tone.dimensions.arousal > 0.6);   // Alta energía
    assert!(tone.dimensions.dominance > 0.7); // Asertivo
    
    // Usuario nombra el tono
    detector.learn_tone("Determinado", tone.dimensions, "eduardo_001");
    
    // Verificar futuras detecciones
    let text2 = "Lo voy a hacer sin excusas";
    let tone2 = detector.detect(text2, "eduardo_001");
    
    assert_eq!(tone2.cluster_name, Some("Determinado"));
}
```

---

## 🎓 LECCIONES APRENDIDAS

### Por Qué Esto Es Crítico

1. **Hardcoding = Death of Personalization**
   - 7 topics fijos NO pueden capturar vida humana compleja
   - Cada persona es un universo único de interests
   
2. **Enums vs Graphs**
   - Enum: Closed world assumption (sabemos todo)
   - Graph: Open world (aprendemos constantemente)
   
3. **MTT-DSL Philosophy**
   - Templates dinámicos > Código estático
   - Sistema debe EVOLUCIONAR con usuario
   
4. **"Juntos pero no revueltos"**
   - Respeto por boundaries cognitivos
   - No mezclar contexts a menos que usuario conecte

### Por Qué Ahora

- ShuiDao implementación en progreso (40% complete)
- Detectado ANTES de escalar (buena arquitectura)
- Refactor ahora = 20h vs 200h después
- Diferencia entre asistente genérico vs compañero real

---

## 📚 REFERENCIAS

### Documentos Relacionados

1. `ROADMAP_V2/01_ARQUITECTURA/12_shuidao-intention-detection.md`
2. `ROADMAP_V2/02_COMPONENTES/13_shuidao-cognitive-engine.md`
3. `ROADMAP_V2/07_TEMPLATES/README.md`
4. `ROADMAP_V2/00_VISION/BITA-1_FBCU_SPECIFICATION.md` (personalización)

### Papers & Research

1. **VAD Model**: Russell (1980) - Valence-Arousal-Dominance
2. **Topic Modeling**: LDA, CTM, dynamic topic models
3. **Personalization**: Collaborative filtering, user modeling
4. **Embeddings**: Sentence-BERT for semantic similarity

---

## 🏁 CONCLUSIÓN

Esta decisión arquitectónica es **FUNDAMENTAL** para diferencia entre:

- ❌ Asistente genérico con 7 categorías hardcoded
- ✅ Compañero cognitivo que aprende TUS interests únicos

Sin DA-033, ShuiDao sería solo otro chatbot inteligente.

Con DA-033, ShuiDao se convierte en **TU** asistente personal que entiende que Rust (1.0) te apasiona más que Microprocesadores (0.7), que no debes mezclar Espiritualidad con Tecnología a menos que TÚ lo conectes, y que cuando estás "Determinado" necesitas pasos concretos sin excusas.

**Esto es lo que significa verdadera personalización.**

---

**Estado:** ACTIVO - CRÍTICO  
**Prioridad:** MÁXIMA  
**Siguiente paso:** Implementación Fase 1 (Topic System)  
**Owner:** B (Sistema Bitácora) + Eduardo  
**Fecha límite:** Week 2-3 de ShuiDao Phase 3b
