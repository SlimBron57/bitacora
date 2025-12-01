```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/01_ARQUITECTURA/12_shuidao-intention-detection.md
Versión: 1.1.0
Fecha Creación: 2025-11-23
Última Actualización: 2025-11-24 16:15:00
Autor: Sistema Bitácora - En colaboración con Eduardo
Propósito: Arquitectura técnica del IntentionDetector - Motor de comprensión de intención ShuiDao
Estado: 🏗️ ARQUITECTURA TÉCNICA (Updated DA-034 separation)
Template: architecture_spec.yaml v1.0.0 (MTT-DSL)
Relacionado Con:
  - ROADMAP_V2/00_VISION/08_shuidao-cognitive-architecture.md v1.0.0 (visión cognitiva)
  - ROADMAP_V2/00_VISION/DA-034_SMALL_WORLD_NETWORKS.md v1.0 (separation of concerns)
  - ROADMAP_V2/02_COMPONENTES/04_flowpacks.md (FlowPacks Phase 3a)
  - ROADMAP_V2/02_COMPONENTES/07_routier-navigator.md v2.0.0 (network navigation)
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/14_shuidao-topic-graph.md v1.1.0 (detection only)
  - ROADMAP_V2/04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md v2.0.1
Fundamentos:
  - NLP: Intent Classification (Rasa, Dialogflow, OpenAI Function Calling)
  - ML: Multi-Factor Decision Trees
  - Cognitive Psychology: Goal-Oriented Behavior Models
Implementa: 
  - DA-032 (ShuiDao - Intention-Oriented Architecture)
  - DA-034 (Small World Networks - Separation of Concerns)
Changelog v1.1.0 (2025-11-24):
  - Añadida sección "SEPARATION OF CONCERNS: DETECTION VS NAVIGATION"
  - Clarificado: TopicGraph (detection <15ms) vs Routier (navigation background)
  - Documentada arquitectura 3-layer (HOT/WARM/COLD paths)
  - Añadidas referencias a DA-034 y documentos relacionados
  - Integration via event bus documentada
# === FIN DATOS DE AUDITORÍA ===
```

# 🎯 IntentionDetector: Technical Architecture

## Multi-Factor Intention Analysis System

---

## 🎯 PROPÓSITO

### ¿Qué es IntentionDetector?

**IntentionDetector** es el motor de análisis semántico que transforma inputs del usuario en **intenciones estructuradas** para activar los 5 Modos Cognitivos de ShuiDao.

### Problema que Resuelve

**FlowPacks Phase 3a** detecta patrones de similitud:
```rust
// FlowPacks piensa en términos de similitud
similarity_score = 0.92 // "Esta pregunta se parece a una anterior"
```

**ShuiDao Phase 3b** detecta intención:
```rust
// IntentionDetector piensa en términos de propósito
DetectedIntention {
    mode: CognitiveMode::Operational,
    submode: Some(Submode::Project),
    confidence: 0.94,
    factors: MultiFactorScore {
        verb_weight: 0.89,    // "instalar", "configurar" → acción
        topic_weight: 0.91,   // "switch", "red" → operacional
        tone_weight: 0.82,    // Pragmático, no curioso
        context_weight: 0.88, // Conversación previa sobre networking
    }
}
```

### ¿Por qué No Usar Alternativas?

| Alternativa | Limitación | ShuiDao Solución |
|-------------|-----------|------------------|
| **OpenAI Function Calling** | Requiere API externa, latencia, costos | Local, 0ms overhead, gratis |
| **Rasa NLU** | Overhead pesado (~100MB), sobre-ingeniería | Ligero (<5MB), específico al dominio |
| **Regex Pattern Matching** | Frágil, no captura semántica | Embeddings semánticos + heurísticas |
| **Pure ML Classifier** | Requiere training data extenso | Híbrido: embeddings + rules |

### Relación con BITA-1/BITA-2

- **BITA-1 (Contexto 7D):** Usa conversación histórica como factor de intención
- **BITA-2 (FBCU + TelescopeDB):** Almacena intenciones detectadas como metadata en entries biográficas

---

## 📖 FUNDAMENTOS TEÓRICOS

### Base Conceptual

**IntentionDetector** combina tres paradigmas:

#### 1. NLP Intent Classification
```
Usuario: "¿Cómo configuro un switch Cisco?"

Pipeline tradicional:
Input → Tokenization → Entity Recognition → Intent Classification → Response

ShuiDao pipeline:
Input → Embedding (MiniLM) → Multi-Factor Analysis → Intention + Mode → Response
```

**Referencias:**
- Rasa NLU: DIETClassifier architecture (Dual Intent Entity Transformer)
- Dialogflow CX: Multi-turn conversation intent tracking
- Papers: "BERT for Intent Classification" (Devlin et al.)

#### 2. Multi-Factor Decision Making
```python
# Analogía: Credit scoring
Credit Score = 0.35*Income + 0.30*History + 0.20*Debt + 0.15*Age

# IntentionDetector
Intention Confidence = w1*Verb + w2*Topic + w3*Tone + w4*Context
```

Cada factor aporta evidencia, el conjunto determina la intención final.

#### 3. Goal-Oriented Behavior (Psicología Cognitiva)

```
Taxonomía de comportamientos humanos:
┌─────────────────────────────────────┐
│ Conversational (social)             │  "¿Cómo estuvo tu día?"
│ Operational (goal-directed)         │  "Necesito configurar X"
│ Procedural (step-by-step)           │  "Paso 3: conectar cable"
│ Learning (knowledge acquisition)    │  "Explícame qué es QEM"
│ Light (casual, low-stake)           │  "¿Quién ganó el mundial 2018?"
└─────────────────────────────────────┘
```

**Referencia:** Miller, Galanter, Pribram - "Plans and the Structure of Behavior" (1960)

### Analogías del Mundo Real

#### Analogía 1: Triaje Médico
```
Paciente llega a emergencias:
→ Enfermera evalúa: síntomas, urgencia, historial
→ Clasifica: Trauma (rojo), Urgente (amarillo), Estable (verde)
→ Ruta al especialista correcto

Usuario envía mensaje:
→ IntentionDetector evalúa: verbo, tema, tono, contexto
→ Clasifica: Operational, Learning, Light, etc.
→ Ruta al CognitiveMode correcto
```

#### Analogía 2: Sistema de Navegación
```
GPS recibe: "Buscar restaurante italiano"
→ Analiza: tipo de lugar, preferencias, ubicación, hora
→ Intención: "Comer cerca, ahora"
→ Activa: búsqueda local + filtros + ruta

IntentionDetector recibe: "¿Cómo instalo Kubernetes?"
→ Analiza: verbo (instalar), tema (DevOps), tono (pragmático)
→ Intención: "Operational Project"
→ Activa: Project tracking + step-by-step + validation
```

---

## 🏗️ VISIÓN GENERAL DE ARQUITECTURA

### Diagrama del Sistema

```
                    USER INPUT
                        │
                        ▼
        ┌───────────────────────────────┐
        │    MessagePreprocessor        │
        │  • Tokenization               │
        │  • Normalization              │
        │  • Entity extraction          │
        └───────────────┬───────────────┘
                        │
                        ▼
        ┌───────────────────────────────┐
        │    EmbeddingGenerator         │
        │  • MiniLM-L6-v2 encode        │
        │  • 384-dim vector             │
        └───────────────┬───────────────┘
                        │
            ┌───────────┴───────────┐
            │                       │
            ▼                       ▼
    ┌───────────────┐       ┌──────────────┐
    │ VerbClassifier│       │TopicAnalyzer │
    │ • Action verbs│       │• Domain match│
    │ • Modality    │       │• Category    │
    └───────┬───────┘       └──────┬───────┘
            │                      │
            │       ┌──────────────┤
            │       │              │
            ▼       ▼              ▼
    ┌───────────────────┐  ┌──────────────┐
    │  ToneDetector     │  │ConversationH.│
    │  • Pragmatic?     │  │• Recent msgs │
    │  • Curious?       │  │• Patterns    │
    │  • Frustrated?    │  │• Projects    │
    └─────────┬─────────┘  └──────┬───────┘
              │                   │
              └─────────┬─────────┘
                        │
                        ▼
            ┌───────────────────────────┐
            │   IntentionScorer         │
            │   • Multi-factor combine  │
            │   • Confidence threshold  │
            │   • Mode selection        │
            └───────────┬───────────────┘
                        │
                        ▼
            ┌───────────────────────────┐
            │   DetectedIntention       │
            │   {                       │
            │     mode: Operational,    │
            │     submode: Project,     │
            │     confidence: 0.94,     │
            │     metadata: {...}       │
            │   }                       │
            └───────────────────────────┘
                        │
                        ▼
                 CognitiveRouter
              (activates correct mode)
```

### Flujo de Datos

```
Input: "necesito instalar un switch en la oficina"
  │
  ├─> Preprocess: ["necesitar", "instalar", "switch", "oficina"]
  │
  ├─> Embed: [0.234, -0.891, ..., 0.456] (384-dim)
  │
  ├─> VerbClassifier:
  │     verb="instalar" → ACTION_VERB
  │     modality=NECESITAR → HIGH_COMMITMENT
  │     score=0.92
  │
  ├─> TopicAnalyzer:
  │     topic="switch" → NETWORKING
  │     domain=INFRASTRUCTURE
  │     score=0.89
  │
  ├─> ToneDetector:
  │     tone=PRAGMATIC (no es curiosidad)
  │     urgency=MEDIUM
  │     score=0.85
  │
  ├─> ConversationHistory:
  │     recent_topics=["networking", "oficina"]
  │     active_projects=[]
  │     score=0.78
  │
  └─> IntentionScorer:
        weighted_score = 0.30*0.92 + 0.30*0.89 + 0.25*0.85 + 0.15*0.78
                      = 0.276 + 0.267 + 0.212 + 0.117
                      = 0.872
        
        confidence = 0.87 > threshold(0.75) ✅
        
        DetectedIntention {
          mode: CognitiveMode::Operational,
          submode: Some(Submode::Project),
          confidence: 0.87,
          extracted_entities: {
            "project_name": "instalar switch",
            "location": "oficina",
            "category": "networking"
          }
        }
```

---

## 🔬 DISEÑO DETALLADO

### Componentes Principales

#### 1. MessagePreprocessor

**Responsabilidad:** Normalizar y extraer información básica del input.

```rust
pub struct MessagePreprocessor {
    tokenizer: SimpleTokenizer,
    entity_extractor: EntityExtractor,
}

impl MessagePreprocessor {
    pub fn process(&self, input: &str) -> ProcessedMessage {
        ProcessedMessage {
            raw: input.to_string(),
            tokens: self.tokenizer.tokenize(input),
            entities: self.entity_extractor.extract(input),
            normalized: self.normalize(input),
        }
    }
    
    fn normalize(&self, input: &str) -> String {
        input
            .to_lowercase()
            .trim()
            .replace("á", "a")
            .replace("é", "e")
            // ... más normalizaciones
    }
}

pub struct ProcessedMessage {
    pub raw: String,
    pub tokens: Vec<String>,
    pub entities: HashMap<String, Vec<String>>, // "verbs": ["instalar"], "nouns": ["switch"]
    pub normalized: String,
}
```

#### 2. VerbClassifier

**Responsabilidad:** Clasificar verbos de acción y su modalidad.

```rust
pub struct VerbClassifier {
    action_verbs: HashSet<String>,      // "instalar", "configurar", "crear"
    knowledge_verbs: HashSet<String>,   // "explicar", "entender", "aprender"
    social_verbs: HashSet<String>,      // "conversar", "comentar", "opinar"
}

#[derive(Debug, Clone)]
pub enum VerbCategory {
    Action,      // hacer algo
    Knowledge,   // aprender algo
    Social,      // interactuar
    Question,    // preguntar
    None,
}

#[derive(Debug, Clone)]
pub enum Modality {
    Necessity,   // "necesito", "debo"
    Desire,      // "quiero", "me gustaría"
    Ability,     // "puedo", "sé cómo"
    Question,    // "¿cómo?", "¿qué?"
}

impl VerbClassifier {
    pub fn classify(&self, tokens: &[String]) -> VerbClassification {
        let verb_cat = self.detect_verb_category(tokens);
        let modality = self.detect_modality(tokens);
        let score = self.compute_confidence(&verb_cat, &modality);
        
        VerbClassification {
            category: verb_cat,
            modality,
            confidence: score,
        }
    }
    
    fn detect_verb_category(&self, tokens: &[String]) -> VerbCategory {
        for token in tokens {
            if self.action_verbs.contains(token) {
                return VerbCategory::Action;
            }
            if self.knowledge_verbs.contains(token) {
                return VerbCategory::Knowledge;
            }
            if self.social_verbs.contains(token) {
                return VerbCategory::Social;
            }
        }
        VerbCategory::None
    }
}

pub struct VerbClassification {
    pub category: VerbCategory,
    pub modality: Modality,
    pub confidence: f32, // 0.0-1.0
}
```

#### 3. DynamicTopicAnalyzer (Dynamic System - DA-033)

> **⚠️ REFACTOR CRÍTICO (2025-11-24):** TopicAnalyzer migrado de enum estático a sistema dinámico.
> Ver: `ROADMAP_V2/00_VISION/DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md`

**Responsabilidad:** Detectar y aprender topics únicos de cada usuario (ilimitados, dinámicos).

```rust
pub struct DynamicTopicAnalyzer {
    embedder: MiniLMEmbedder,
    user_graphs: Arc<RwLock<HashMap<UserId, TopicGraph>>>,
    voxeldb: Arc<VoxelDB>,
    template_engine: Arc<MttDslEngine>,
    auto_discovery_threshold: f32, // default: 0.6
}

// TopicGraph: Grafo personal de topics por usuario
pub struct TopicGraph {
    user_id: UserId,
    
    // Jerarquía de topics (ilimitada, user-defined)
    root_topics: HashMap<TopicId, RootTopic>,
    sub_topics: HashMap<TopicId, SubTopic>,
    micro_topics: HashMap<TopicId, MicroTopic>,
    
    // Relaciones entre topics
    edges: Vec<TopicEdge>,
    
    // Pesos de interés personalizados
    weights: HashMap<TopicId, InterestWeight>,
    
    // Embeddings para matching semántico
    embeddings: HashMap<TopicId, Vec<f32>>,
    
    // Templates MTT-DSL (stored in VoxelDB)
    template_paths: HashMap<TopicId, String>,
}

pub struct RootTopic {
    id: TopicId,
    name: String,  // "Cocina", "Cerámica", "Rust" - USER DEFINED ✅
    description: Option<String>,
    user_defined: bool,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    interaction_count: u32,
    sentiment_average: f32,
    parent: Option<TopicId>,
    isolation_mode: IsolationMode,
}

pub struct InterestWeight {
    explicit: f32,   // Usuario dijo "me interesa X" (0.0-1.0)
    implicit: f32,   // Frecuencia de menciones (0.0-1.0)
    temporal: f32,   // Decae con tiempo (0.0-1.0)
    emotional: f32,  // Sentimiento asociado (0.0-1.0)
}

impl InterestWeight {
    pub fn combined(&self) -> f32 {
        (self.explicit * 0.4 + 
         self.implicit * 0.3 + 
         self.temporal * 0.2 + 
         self.emotional * 0.1).clamp(0.0, 1.0)
    }
}

pub enum IsolationMode {
    Strict,      // NUNCA mezclar (Armas ⊥ Cocina)
    Soft,        // Permitir si usuario conecta
    Exploratory, // Sugerir conexiones con confirmación
}

impl DynamicTopicAnalyzer {
    pub fn analyze(&self, message: &ProcessedMessage, user_id: &UserId) 
        -> TopicAnalysis 
    {
        let embedding = self.embedder.encode(&message.normalized);
        
        // 1. Buscar en TopicGraph del usuario
        let graphs = self.user_graphs.read().unwrap();
        let graph = graphs.get(user_id);
        
        if let Some(graph) = graph {
            // Calcular similaridad con todos los topics
            let mut scores: Vec<(TopicId, f32)> = graph.embeddings
                .iter()
                .map(|(topic_id, topic_vec)| {
                    (*topic_id, cosine_similarity(&embedding, topic_vec))
                })
                .collect();
            
            scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            // Si best match > threshold: detectado
            if let Some((topic_id, confidence)) = scores.first() {
                if *confidence > 0.75 {
                    return TopicAnalysis {
                        detected_topics: vec![*topic_id],
                        confidence: *confidence,
                        is_new_topic: false,
                        suggested_name: None,
                        requires_confirmation: false,
                    };
                }
            }
            
            // Si best match < auto_discovery_threshold: nuevo topic
            if scores.first().map(|s| s.1).unwrap_or(0.0) < self.auto_discovery_threshold {
                return TopicAnalysis {
                    detected_topics: vec![],
                    confidence: 0.0,
                    is_new_topic: true,
                    suggested_name: self.suggest_topic_name(&message.normalized),
                    requires_confirmation: true,
                };
            }
        }
        
        // Usuario nuevo: crear primer topic
        TopicAnalysis {
            detected_topics: vec![],
            confidence: 0.0,
            is_new_topic: true,
            suggested_name: self.suggest_topic_name(&message.normalized),
            requires_confirmation: true,
        }
    }
    
    // Auto-discovery: crear nuevo topic con confirmación usuario
    pub async fn learn_new_topic(
        &self,
        user_id: &UserId,
        topic_name: String,
        example_text: &str,
        parent: Option<TopicId>,
    ) -> Result<TopicId> {
        let topic_id = TopicId::new();
        let embedding = self.embedder.encode(example_text);
        
        // Crear RootTopic
        let topic = RootTopic {
            id: topic_id,
            name: topic_name.clone(),
            description: None,
            user_defined: true,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            interaction_count: 1,
            sentiment_average: 0.5,
            parent,
            isolation_mode: IsolationMode::Soft, // default
        };
        
        // Agregar a TopicGraph
        let mut graphs = self.user_graphs.write().unwrap();
        let graph = graphs.entry(user_id.clone())
            .or_insert_with(|| TopicGraph::new(user_id.clone()));
        
        graph.root_topics.insert(topic_id, topic);
        graph.embeddings.insert(topic_id, embedding);
        graph.weights.insert(topic_id, InterestWeight {
            explicit: 0.8,  // Usuario confirmó interés
            implicit: 0.1,  // Primera mención
            temporal: 1.0,  // Reciente
            emotional: 0.5, // Neutral por ahora
        });
        
        // Crear template MTT-DSL
        let template_path = format!("users/{}/topics/{}.yaml", user_id, topic_name);
        self.create_topic_template(user_id, &topic, &template_path).await?;
        graph.template_paths.insert(topic_id, template_path);
        
        // Persistir en VoxelDB
        self.persist_topic_graph(user_id, graph).await?;
        
        Ok(topic_id)
    }
}

pub struct TopicAnalysis {
    pub detected_topics: Vec<TopicId>,
    pub confidence: f32,
    pub is_new_topic: bool,
    pub suggested_name: Option<String>,
    pub requires_confirmation: bool,
}
```

**Ejemplo: Eduardo's TopicGraph**

```
Eduardo (user_id: "eduardo_001"):
├─ Cocina (InterestWeight: 0.85)
│  ├─ Recetas Tradicionales (0.90)
│  └─ Fermentación (0.60) ← Recién aprendido
│
├─ Artesanía (0.75)
│  └─ Cerámica (0.85)
│     ├─ Técnicas de Torno (0.80)
│     └─ Esmaltes (0.70)
│        └─ [EDGE → Química: 0.85] ← Conexión detectada
│
├─ Tecnología (0.95)
│  ├─ Programación (0.98)
│  │  ├─ Rust (1.00) 🔥 ← Máximo interés
│  │  ├─ AI/ML (0.90)
│  │  └─ Arquitectura Software (0.95)
│  └─ Microprocesadores (0.70)
│
└─ Filosofía (0.80)
   └─ Espiritualidad (0.70)
      └─ [ISOLATION: Strict con Tecnología]
```

**Ventajas vs Enum Estático:**

| Característica | TopicDomain Enum ❌ | DynamicTopicAnalyzer ✅ |
|----------------|---------------------|-------------------------|
| **Número de topics** | 7 fijos | Ilimitados |
| **Personalización** | Igual para todos | Único por usuario |
| **Aprendizaje** | No | Sí, incremental |
| **Jerarquía** | Plana | Multinivel (Root → Sub → Micro) |
| **Aislamiento** | No | Sí ("Juntos pero no revueltos") |
| **Pesos de interés** | No | Sí (combinados: explicit+implicit+temporal+emotional) |
| **MTT-DSL Integration** | No | Sí (templates en VoxelDB) |
```

#### 4. DynamicToneDetector (Dynamic System - DA-033)

> **⚠️ REFACTOR CRÍTICO (2025-11-24):** ToneDetector migrado de enum estático a espacio emocional continuo.
> Ver: `ROADMAP_V2/00_VISION/DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md`

**Responsabilidad:** Detectar y aprender tonos emocionales únicos de cada usuario (continuo, dinámico).

```rust
pub struct DynamicToneDetector {
    user_spaces: Arc<RwLock<HashMap<UserId, EmotionalSpace>>>,
    voxeldb: Arc<VoxelDB>,
    template_engine: Arc<MttDslEngine>,
    lexical_analyzer: LexicalAnalyzer,
    syntactic_analyzer: SyntacticAnalyzer,
}

// EmotionalSpace: Espacio continuo de tonos (modelo VAD + Formality)
pub struct EmotionalSpace {
    user_id: UserId,
    
    // Clusters de tonos descubiertos
    clusters: HashMap<ToneClusterId, ToneCluster>,
    
    // Templates MTT-DSL
    template_paths: HashMap<ToneClusterId, String>,
}

// Modelo VAD (Valence-Arousal-Dominance) + Formality
pub struct ToneDimensions {
    pub valence: f32,     // -1.0 (negativo) a 1.0 (positivo)
    pub arousal: f32,     // -1.0 (calmado) a 1.0 (excitado)
    pub dominance: f32,   // -1.0 (sumiso) a 1.0 (dominante)
    pub formality: f32,   // -1.0 (casual) a 1.0 (formal)
}

pub struct ToneCluster {
    id: ToneClusterId,
    name: String,  // "Determinado", "Nostálgico", "Emocionado" - USER NAMED ✅
    center: ToneDimensions,
    radius: f32,  // Tolerancia para matching
    examples: Vec<String>,
    user_id: UserId,
    discovered_at: DateTime<Utc>,
    lexical_markers: LexicalMarkers,
    syntactic_patterns: Vec<SyntacticPattern>,
}

pub struct LexicalMarkers {
    pub strong_verbs: Vec<String>,      // "voy a", "terminar", "lograr"
    pub commitment_phrases: Vec<String>, // "cueste lo que cueste", "sin excusas"
    pub time_markers: Vec<String>,       // "hoy", "ahora", "ya"
    pub emotional_adjectives: Vec<String>, // "determinado", "emocionado"
}

impl DynamicToneDetector {
    pub fn detect(&self, message: &ProcessedMessage, user_id: &UserId) 
        -> ToneDetection 
    {
        // 1. Calcular dimensiones VAD+F del mensaje
        let dimensions = self.compute_dimensions(message);
        
        // 2. Buscar cluster más cercano en EmotionalSpace del usuario
        let spaces = self.user_spaces.read().unwrap();
        let space = spaces.get(user_id);
        
        if let Some(space) = space {
            for (cluster_id, cluster) in &space.clusters {
                let distance = self.euclidean_distance(&dimensions, &cluster.center);
                
                if distance <= cluster.radius {
                    return ToneDetection {
                        dimensions,
                        cluster_id: Some(*cluster_id),
                        cluster_name: Some(cluster.name.clone()),
                        confidence: 1.0 - (distance / cluster.radius),
                        is_new_tone: false,
                        requires_confirmation: false,
                    };
                }
            }
        }
        
        // 3. Si no hay match: nuevo tono detectado
        ToneDetection {
            dimensions,
            cluster_id: None,
            cluster_name: None,
            confidence: 0.0,
            is_new_tone: true,
            requires_confirmation: true,
        }
    }
    
    fn compute_dimensions(&self, message: &ProcessedMessage) -> ToneDimensions {
        // Análisis léxico
        let lexical_features = self.lexical_analyzer.analyze(&message.normalized);
        
        // Análisis sintáctico
        let syntactic_features = self.syntactic_analyzer.analyze(&message.normalized);
        
        // Calcular dimensiones
        ToneDimensions {
            valence: self.compute_valence(&lexical_features),
            arousal: self.compute_arousal(&lexical_features, &syntactic_features),
            dominance: self.compute_dominance(&lexical_features, &syntactic_features),
            formality: self.compute_formality(&lexical_features),
        }
    }
    
    fn compute_valence(&self, features: &LexicalFeatures) -> f32 {
        // Positive words (+1), Negative words (-1)
        let positive_score = features.positive_words.len() as f32;
        let negative_score = features.negative_words.len() as f32;
        let total = positive_score + negative_score;
        
        if total == 0.0 { return 0.0; }
        ((positive_score - negative_score) / total).clamp(-1.0, 1.0)
    }
    
    fn compute_arousal(&self, lex: &LexicalFeatures, syn: &SyntacticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Exclamation marks (+0.3)
        score += syn.exclamation_count as f32 * 0.3;
        
        // High-energy verbs (+0.2)
        score += lex.high_energy_verbs.len() as f32 * 0.2;
        
        // Uppercase words (+0.1)
        score += lex.uppercase_words.len() as f32 * 0.1;
        
        score.clamp(-1.0, 1.0)
    }
    
    fn compute_dominance(&self, lex: &LexicalFeatures, syn: &SyntacticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Imperative verbs (+0.4)
        score += syn.imperative_verbs.len() as f32 * 0.4;
        
        // First person assertive (+0.3): "voy a", "haré", "lograré"
        score += lex.assertive_first_person.len() as f32 * 0.3;
        
        // Questions (-0.2): "¿podrías?", "¿me ayudas?"
        score -= syn.question_marks as f32 * 0.2;
        
        score.clamp(-1.0, 1.0)
    }
    
    fn compute_formality(&self, features: &LexicalFeatures) -> f32 {
        let mut score = 0.0;
        
        // Formal vocabulary (+0.3)
        score += features.formal_words.len() as f32 * 0.3;
        
        // Slang/colloquialisms (-0.3)
        score -= features.slang_words.len() as f32 * 0.3;
        
        // Contractions (-0.2): "no sé" vs "no lo sé"
        score -= features.contractions.len() as f32 * 0.2;
        
        score.clamp(-1.0, 1.0)
    }
    
    fn euclidean_distance(&self, a: &ToneDimensions, b: &ToneDimensions) -> f32 {
        let dv = a.valence - b.valence;
        let da = a.arousal - b.arousal;
        let dd = a.dominance - b.dominance;
        let df = a.formality - b.formality;
        
        (dv*dv + da*da + dd*dd + df*df).sqrt()
    }
    
    // Auto-discovery: crear nuevo tone cluster con confirmación usuario
    pub async fn learn_new_tone(
        &self,
        user_id: &UserId,
        tone_name: String,
        dimensions: ToneDimensions,
        example_text: &str,
    ) -> Result<ToneClusterId> {
        let cluster_id = ToneClusterId::new();
        
        // Analizar lexical markers del ejemplo
        let lexical_markers = self.lexical_analyzer.extract_markers(example_text);
        let syntactic_patterns = self.syntactic_analyzer.extract_patterns(example_text);
        
        // Crear ToneCluster
        let cluster = ToneCluster {
            id: cluster_id,
            name: tone_name.clone(),
            center: dimensions,
            radius: 0.25,  // default tolerance
            examples: vec![example_text.to_string()],
            user_id: user_id.clone(),
            discovered_at: Utc::now(),
            lexical_markers,
            syntactic_patterns,
        };
        
        // Agregar a EmotionalSpace
        let mut spaces = self.user_spaces.write().unwrap();
        let space = spaces.entry(user_id.clone())
            .or_insert_with(|| EmotionalSpace::new(user_id.clone()));
        
        space.clusters.insert(cluster_id, cluster);
        
        // Crear template MTT-DSL
        let template_path = format!("users/{}/tones/{}.yaml", user_id, tone_name);
        self.create_tone_template(user_id, &cluster, &template_path).await?;
        space.template_paths.insert(cluster_id, template_path);
        
        // Persistir en VoxelDB
        self.persist_emotional_space(user_id, space).await?;
        
        Ok(cluster_id)
    }
}

pub struct ToneDetection {
    pub dimensions: ToneDimensions,
    pub cluster_id: Option<ToneClusterId>,
    pub cluster_name: Option<String>,
    pub confidence: f32,
    pub is_new_tone: bool,
    pub requires_confirmation: bool,
}
```

**Ejemplo: Eduardo's ToneCluster "Determinado"**

```yaml
# users/eduardo_001/tones/determinado.yaml
metadata:
  name: "Determinado"
  user_id: "eduardo_001"
  discovered_at: "2025-11-24T13:50:00Z"
  discovered_from: "Voy a terminar este proyecto cueste lo que cueste"
  version: "1.0.0"
  
dimensions:
  valence: 0.3      # Ligeramente positivo
  arousal: 0.7      # Alta energía
  dominance: 0.8    # Muy asertivo
  formality: 0.5    # Neutral
  
cluster:
  center: [0.3, 0.7, 0.8, 0.5]
  radius: 0.25
  
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
```

**Ventajas vs Enum Estático:**

| Característica | ToneType Enum ❌ | DynamicToneDetector ✅ |
|----------------|------------------|------------------------|
| **Número de tonos** | 5 fijos | Ilimitados |
| **Representación** | Discreto (5 opciones) | Continuo (espacio 4D) |
| **Personalización** | Igual para todos | Único por usuario |
| **Aprendizaje** | No | Sí, incremental |
| **Matices** | Pragmatic OR Curious | valence:0.3, arousal:0.7, etc. |
| **User naming** | No | Sí ("Determinado", "Nostálgico") |
| **Response adaptation** | Genérica | Basada en dimensiones VAD+F |
| **MTT-DSL Integration** | No | Sí (templates en VoxelDB) |
```

#### 5. ConversationHistory

**Responsabilidad:** Analizar contexto de conversaciones previas.

```rust
pub struct ConversationHistory {
    recent_messages: VecDeque<HistoricalMessage>,
    active_projects: Vec<ProjectContext>,
    topic_frequency: HashMap<String, usize>,
}

pub struct HistoricalMessage {
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub detected_mode: Option<CognitiveMode>,
}

pub struct ProjectContext {
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub completion: f32, // 0.0-1.0
}

impl ConversationHistory {
    pub fn analyze_context(&self, current_message: &ProcessedMessage) -> ContextScore {
        let recency_score = self.compute_recency_score(current_message);
        let project_score = self.compute_project_continuation(current_message);
        let topic_score = self.compute_topic_consistency(current_message);
        
        ContextScore {
            overall: (recency_score + project_score + topic_score) / 3.0,
            recency: recency_score,
            project_continuation: project_score,
            topic_consistency: topic_score,
        }
    }
    
    fn compute_project_continuation(&self, message: &ProcessedMessage) -> f32 {
        // Si hay un proyecto activo y el mensaje menciona términos relacionados
        if let Some(project) = self.active_projects.last() {
            if message.normalized.contains(&project.name.to_lowercase()) {
                return 0.9;
            }
        }
        0.3 // baseline
    }
}

pub struct ContextScore {
    pub overall: f32,
    pub recency: f32,
    pub project_continuation: f32,
    pub topic_consistency: f32,
}
```

#### 6. IntentionScorer

**Responsabilidad:** Combinar todos los factores y determinar la intención final.

```rust
pub struct IntentionScorer {
    weights: ScoringWeights,
    thresholds: ModeThresholds,
}

pub struct ScoringWeights {
    pub verb: f32,      // default: 0.30
    pub topic: f32,     // default: 0.30
    pub tone: f32,      // default: 0.25
    pub context: f32,   // default: 0.15
}

pub struct ModeThresholds {
    pub operational: f32,    // 0.75 - alta confianza requerida
    pub procedural: f32,     // 0.80 - muy específico
    pub learning: f32,       // 0.70 - más flexible
    pub conversational: f32, // 0.60 - default catch-all
    pub light: f32,          // 0.65 - trivia rápida
}

impl IntentionScorer {
    pub fn score_and_select(
        &self,
        verb: &VerbClassification,
        topic: &TopicAnalysis,
        tone: &ToneDetection,
        context: &ContextScore,
    ) -> DetectedIntention {
        // 1. Compute weighted score
        let weighted = 
            self.weights.verb * verb.confidence +
            self.weights.topic * topic.confidence +
            self.weights.tone * tone.confidence +
            self.weights.context * context.overall;
        
        // 2. Mode selection logic
        let mode = self.select_mode(verb, topic, tone, context);
        
        // 3. Submode refinement
        let submode = self.select_submode(&mode, verb, topic);
        
        DetectedIntention {
            mode,
            submode,
            confidence: weighted,
            factors: MultiFactorScore {
                verb_weight: verb.confidence,
                topic_weight: topic.confidence,
                tone_weight: tone.confidence,
                context_weight: context.overall,
            },
            metadata: self.extract_metadata(verb, topic, tone),
        }
    }
    
    fn select_mode(
        &self,
        verb: &VerbClassification,
        topic: &TopicAnalysis,
        tone: &ToneDetection,
        context: &ContextScore,
    ) -> CognitiveMode {
        // Logic tree para determinar modo
        
        // Si hay proyecto activo y mensaje relacionado → Operational
        if context.project_continuation > 0.8 {
            return CognitiveMode::Operational;
        }
        
        // Si verbo de acción + tono pragmático → Operational
        if matches!(verb.category, VerbCategory::Action) 
           && tone.tone == Tone::Pragmatic {
            return CognitiveMode::Operational;
        }
        
        // Si verbo de conocimiento + tono curioso → Learning
        if matches!(verb.category, VerbCategory::Knowledge)
           && tone.tone == Tone::Curious {
            return CognitiveMode::Learning;
        }
        
        // Si topic = Biography o Psychology → Conversational
        if matches!(topic.primary_domain, TopicDomain::Biography | TopicDomain::Psychology) {
            return CognitiveMode::Conversational;
        }
        
        // Si es pregunta trivial → Light
        if self.is_trivia_question(verb, topic) {
            return CognitiveMode::Light;
        }
        
        // Default: Conversational
        CognitiveMode::Conversational
    }
}

#[derive(Debug, Clone)]
pub struct DetectedIntention {
    pub mode: CognitiveMode,
    pub submode: Option<Submode>,
    pub confidence: f32,
    pub factors: MultiFactorScore,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MultiFactorScore {
    pub verb_weight: f32,
    pub topic_weight: f32,
    pub tone_weight: f32,
    pub context_weight: f32,
}
```

---

## ⚙️ ESTRATEGIA DE IMPLEMENTACIÓN

### Rust Implementation Roadmap

```rust
// Crates necesarios
[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokenizers = "0.15"           // Tokenización eficiente
rust-bert = "0.22"            // MiniLM embeddings
hnsw_rs = "0.3"               // Similarity search (si necesario)
chrono = "0.4"                // Timestamps
```

### Estructura de Módulos

```
src/shuidao/
├── intention_detector.rs      # API principal
├── preprocessor.rs            # MessagePreprocessor
├── classifiers/
│   ├── mod.rs
│   ├── verb_classifier.rs     # VerbClassifier
│   ├── topic_analyzer.rs      # TopicAnalyzer + embeddings
│   └── tone_detector.rs       # ToneDetector
├── context/
│   ├── mod.rs
│   ├── history.rs             # ConversationHistory
│   └── project_tracker.rs     # ProjectContext tracking
├── scoring.rs                 # IntentionScorer
└── types.rs                   # Shared types (DetectedIntention, etc.)
```

### API Principal

```rust
use crate::shuidao::intention_detector::IntentionDetector;

pub struct IntentionDetector {
    preprocessor: MessagePreprocessor,
    verb_classifier: VerbClassifier,
    topic_analyzer: TopicAnalyzer,
    tone_detector: ToneDetector,
    conversation_history: Arc<RwLock<ConversationHistory>>,
    scorer: IntentionScorer,
}

impl IntentionDetector {
    pub fn new() -> Result<Self> {
        Ok(Self {
            preprocessor: MessagePreprocessor::new(),
            verb_classifier: VerbClassifier::from_defaults(),
            topic_analyzer: TopicAnalyzer::load_embeddings()?,
            tone_detector: ToneDetector::new(),
            conversation_history: Arc::new(RwLock::new(ConversationHistory::new())),
            scorer: IntentionScorer::with_default_weights(),
        })
    }
    
    /// Detecta la intención del mensaje del usuario
    pub async fn detect(&self, user_input: &str) -> Result<DetectedIntention> {
        // 1. Preprocess
        let message = self.preprocessor.process(user_input);
        
        // 2. Factor analysis (paralelo)
        let (verb, topic, tone, context) = tokio::join!(
            async { self.verb_classifier.classify(&message.tokens) },
            async { self.topic_analyzer.analyze(&message) },
            async { self.tone_detector.detect(&message) },
            async { 
                self.conversation_history
                    .read()
                    .await
                    .analyze_context(&message)
            }
        );
        
        // 3. Score and select mode
        let intention = self.scorer.score_and_select(
            &verb,
            &topic,
            &tone,
            &context,
        );
        
        // 4. Update history
        self.conversation_history
            .write()
            .await
            .add_message(HistoricalMessage {
                content: user_input.to_string(),
                timestamp: Utc::now(),
                detected_mode: Some(intention.mode.clone()),
            });
        
        Ok(intention)
    }
    
    /// Update weights para tuning
    pub fn update_weights(&mut self, weights: ScoringWeights) {
        self.scorer.weights = weights;
    }
    
    /// Obtener estadísticas de detección
    pub async fn get_stats(&self) -> DetectionStats {
        let history = self.conversation_history.read().await;
        DetectionStats {
            total_messages: history.recent_messages.len(),
            mode_distribution: history.compute_mode_distribution(),
            avg_confidence: history.compute_avg_confidence(),
        }
    }
}
```

### Patterns de Diseño

1. **Builder Pattern** para configuración:
```rust
let detector = IntentionDetector::builder()
    .with_weights(ScoringWeights { verb: 0.35, .. })
    .with_threshold(ModeThresholds { operational: 0.80, .. })
    .with_history_size(100)
    .build()?;
```

2. **Strategy Pattern** para diferentes clasificadores:
```rust
pub trait Classifier {
    fn classify(&self, input: &ProcessedMessage) -> ClassificationResult;
}

impl Classifier for VerbClassifier { ... }
impl Classifier for TopicAnalyzer { ... }
```

3. **Observer Pattern** para tracking:
```rust
pub trait IntentionObserver {
    fn on_intention_detected(&self, intention: &DetectedIntention);
}

// Para logging, metrics, debugging
```

### Consideraciones de Concurrencia

```rust
// ConversationHistory es compartido entre requests
Arc<RwLock<ConversationHistory>>

// Read-heavy workload → RwLock perfecto
// Writes: solo al agregar nuevos mensajes
// Reads: en cada detección

// Alternative: usar tokio::sync::RwLock para async
```

---

## 🗂️ FORMATOS DE DATOS

### Serialización

**CBOR** para almacenamiento eficiente en TelescopeDB:

```rust
use serde::{Deserialize, Serialize};
use serde_cbor;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredIntention {
    pub timestamp: i64,
    pub mode: String,              // "Operational", "Learning", etc.
    pub submode: Option<String>,
    pub confidence: f32,
    pub factors: [f32; 4],         // [verb, topic, tone, context]
    pub metadata: Vec<(String, String)>,
}

impl From<DetectedIntention> for StoredIntention {
    fn from(intention: DetectedIntention) -> Self {
        StoredIntention {
            timestamp: Utc::now().timestamp(),
            mode: format!("{:?}", intention.mode),
            submode: intention.submode.map(|s| format!("{:?}", s)),
            confidence: intention.confidence,
            factors: [
                intention.factors.verb_weight,
                intention.factors.topic_weight,
                intention.factors.tone_weight,
                intention.factors.context_weight,
            ],
            metadata: intention.metadata.into_iter().collect(),
        }
    }
}

// Serialization
let stored = StoredIntention::from(intention);
let bytes = serde_cbor::to_vec(&stored)?;

// Deserialization
let restored: StoredIntention = serde_cbor::from_slice(&bytes)?;
```

### Schema Evolution

**Versionado de formato:**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredIntentionV2 {
    pub version: u8,  // Schema version
    pub timestamp: i64,
    pub mode: String,
    pub submode: Option<String>,
    pub confidence: f32,
    pub factors: FactorsV2,  // New nested struct
    pub metadata: HashMap<String, String>,
}

// Compatibilidad hacia atrás
impl From<StoredIntention> for StoredIntentionV2 {
    fn from(v1: StoredIntention) -> Self {
        StoredIntentionV2 {
            version: 2,
            timestamp: v1.timestamp,
            mode: v1.mode,
            submode: v1.submode,
            confidence: v1.confidence,
            factors: FactorsV2 {
                verb: v1.factors[0],
                topic: v1.factors[1],
                tone: v1.factors[2],
                context: v1.factors[3],
            },
            metadata: v1.metadata.into_iter().collect(),
        }
    }
}
```

### Integración con TelescopeDB

```rust
use crate::telescopedb::{TelescopeDB, BiographicalEntry};

impl IntentionDetector {
    pub async fn store_in_telescopedb(
        &self,
        telescope: &mut TelescopeDB,
        user_input: &str,
        intention: &DetectedIntention,
    ) -> Result<String> {
        let stored = StoredIntention::from(intention.clone());
        let cbor_data = serde_cbor::to_vec(&stored)?;
        
        let entry = BiographicalEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            content: user_input.to_string(),
            ctx7d: ContextToken7D::default(), // Populate as needed
            metadata: hashmap! {
                "intention_mode".to_string() => intention.mode.to_string(),
                "confidence".to_string() => intention.confidence.to_string(),
                "cbor_intention".to_string() => base64::encode(&cbor_data),
            },
        };
        
        telescope.insert_entry(entry).await
    }
}
```

---

## ⚡ CARACTERÍSTICAS DE PERFORMANCE

### Benchmarks Esperados

| Operación | Complejidad | Target | Notas |
|-----------|-------------|--------|-------|
| Preprocess | O(n) | <1ms | n = longitud mensaje (~100 chars) |
| Embedding (MiniLM) | O(1) | <5ms | 384-dim vector, cached model |
| Verb Classification | O(1) | <0.5ms | HashSet lookup |
| Topic Analysis | O(k) | <3ms | k = número de dominios (~7) |
| Tone Detection | O(m) | <1ms | m = número de indicadores (~20) |
| Context Scoring | O(h) | <2ms | h = tamaño history (~100 msgs) |
| Score & Select | O(1) | <0.5ms | Weighted sum + if/else tree |
| **TOTAL** | **O(n+k+h)** | **<15ms** | ✅ Target cumplido |

### Análisis de Complejidad

```
n = longitud mensaje del usuario (100-500 chars típico)
k = número de dominios de topic (7 fijo)
h = tamaño de conversation history (100 mensajes default)

Total: O(n + k + h)
     ≈ O(n) en práctica (n dominante)

Peor caso: 
  n=1000, k=7, h=100
  ~15ms total (embedding es bottleneck)
```

### Bottlenecks Conocidos

1. **Embedding Generation (MiniLM):**
   - 5ms por mensaje
   - **Mitigation:** Cache embeddings para mensajes repetidos
   ```rust
   use lru::LruCache;
   
   pub struct CachedTopicAnalyzer {
       embedder: MiniLMEmbedder,
       cache: LruCache<String, Vec<f32>>,
   }
   
   impl CachedTopicAnalyzer {
       pub fn analyze(&mut self, message: &ProcessedMessage) -> TopicAnalysis {
           if let Some(embedding) = self.cache.get(&message.normalized) {
               return self.analyze_from_embedding(embedding);
           }
           
           let embedding = self.embedder.encode(&message.normalized);
           self.cache.put(message.normalized.clone(), embedding.clone());
           self.analyze_from_embedding(&embedding)
       }
   }
   ```

2. **Context History Scan:**
   - O(h) scan sobre 100 mensajes
   - **Mitigation:** Usar VecDeque con ventana deslizante, indexar por timestamp

3. **Topic Similarity (7 dominios):**
   - 7 cosine similarities por mensaje
   - **Mitigation:** Ya es óptimo, considerar early-stopping si score > 0.95

### Estrategias de Optimización

**v1.0 (Baseline):**
```rust
// Sequential processing
let verb = verb_classifier.classify(&tokens);
let topic = topic_analyzer.analyze(&message);
let tone = tone_detector.detect(&message);
let context = history.analyze_context(&message);
// ~15ms total
```

**v1.1 (Parallel):**
```rust
// Parallel processing con tokio::join!
let (verb, topic, tone, context) = tokio::join!(
    async { verb_classifier.classify(&tokens) },
    async { topic_analyzer.analyze(&message) },
    async { tone_detector.detect(&message) },
    async { history.analyze_context(&message) }
);
// ~8ms total (embedding bottleneck queda)
```

**v2.0 (Speculative + Cache):**
```rust
// Pre-compute topic embeddings async
// Cache recent embeddings (LRU)
// Speculative mode selection (si confidence > 0.95 en verb, skip tone?)
// ~5ms total
```

---

## ⚖️ COMPARACIÓN CON ALTERNATIVAS

| Aspecto | IntentionDetector (ShuiDao) | OpenAI Function Calling | Rasa NLU | Simple Regex |
|---------|----------------------------|------------------------|----------|--------------|
| **Latencia** | <15ms local | 200-500ms API | 50-100ms local | <1ms |
| **Costo** | $0 | $0.01-0.10/request | $0 | $0 |
| **Precisión** | 90-95% (dominio específico) | 95-98% (general) | 85-90% | 60-70% |
| **Setup** | Embeddings pre-trained | API key | Training data (1000+ ejemplos) | Patterns manuales |
| **Memoria** | ~50MB (model) | 0 (API) | ~100MB (model + pipeline) | <1MB |
| **Offline** | ✅ Sí | ❌ No | ✅ Sí | ✅ Sí |
| **Extensibilidad** | ✅ Fácil (agregar dominios) | ❌ Limitado a API | ⚠️ Requiere re-training | ❌ Difícil (regex hell) |
| **Caso de uso** | Personal assistant local | Production apps con budget | Chatbots corporativos | Prototipos rápidos |

### Por Qué IntentionDetector

**vs OpenAI Function Calling:**
- ✅ Privacidad: datos no salen del sistema
- ✅ Costo: $0 vs $100+/mes
- ✅ Latencia: 15ms vs 300ms
- ❌ Precisión ligeramente menor (pero aceptable para dominio específico)

**vs Rasa NLU:**
- ✅ Simplicidad: no requiere training data extenso
- ✅ Ligereza: 50MB vs 100MB
- ✅ Setup: plug-and-play vs días de training
- ❌ Generalización menor (pero no necesitamos generalizar fuera de Bitácora)

**vs Regex:**
- ✅ Robustez: captura variaciones semánticas ("instalar" ≈ "configurar")
- ✅ Mantenibilidad: agregar dominios sin regex hell
- ✅ Contextual: usa conversación histórica
- ❌ Más complejo, pero vale la pena

**Decisión:** IntentionDetector híbrido (embeddings + heurísticas) es óptimo para Bitácora.

---

## 🔗 PUNTOS DE INTEGRACIÓN

### Con FlowPacks Phase 3a

```rust
// FlowPacks provee metadata
pub struct FlowPack {
    pub id: String,
    pub pattern_type: PatternType,
    pub similarity_score: f32,
    // ... otros campos
    
    // NEW: ShuiDao metadata
    pub intention_mode: Option<CognitiveMode>,
    pub intention_confidence: Option<f32>,
}

// Pipeline integrado
async fn process_message(
    user_input: &str,
    flowpacks: &FlowPackEngine,
    intention_detector: &IntentionDetector,
) -> Response {
    // 1. Detect intention (ShuiDao)
    let intention = intention_detector.detect(user_input).await?;
    
    // 2. Check FlowPacks similarity (Phase 3a)
    let similar_pack = flowpacks.find_similar(user_input).await?;
    
    // 3. Route based on intention + similarity
    match (intention.mode, similar_pack) {
        (CognitiveMode::Operational, None) => {
            // New operational project
            operational_engine.create_project(intention).await
        },
        (CognitiveMode::Operational, Some(pack)) if pack.similarity > 0.90 => {
            // Continue existing project
            operational_engine.update_project(pack.id, intention).await
        },
        (CognitiveMode::Learning, _) => {
            // Learning mode (similarity informa estilo de respuesta)
            learning_engine.handle(intention, similar_pack).await
        },
        _ => {
            conversational_engine.handle(intention).await
        }
    }
}
```

### Con TelescopeDB (Memoria Biográfica)

```rust
// Almacenar intenciones como parte de entries biográficas
impl IntentionDetector {
    pub async fn enrich_biographical_entry(
        &self,
        entry: &mut BiographicalEntry,
        user_input: &str,
    ) -> Result<()> {
        let intention = self.detect(user_input).await?;
        
        entry.metadata.insert(
            "intention_mode".to_string(),
            format!("{:?}", intention.mode),
        );
        entry.metadata.insert(
            "intention_confidence".to_string(),
            intention.confidence.to_string(),
        );
        
        // CBOR serializado para análisis posterior
        let stored = StoredIntention::from(intention);
        let cbor = serde_cbor::to_vec(&stored)?;
        entry.metadata.insert(
            "intention_cbor".to_string(),
            base64::encode(&cbor),
        );
        
        Ok(())
    }
}

// Query: "¿Qué proyectos operacionales tengo activos?"
async fn query_operational_projects(
    telescope: &TelescopeDB,
) -> Result<Vec<BiographicalEntry>> {
    telescope.query_by_metadata(
        "intention_mode",
        "Operational",
    ).await
}
```

### Con VoxelDB (Templates)

```rust
// Almacenar patrones de intención en VoxelDB
pub struct IntentionTemplate {
    pub id: String,
    pub mode: CognitiveMode,
    pub example_inputs: Vec<String>,
    pub typical_factors: MultiFactorScore,
    pub voxel_coord: CubicCoordinate,
}

impl IntentionDetector {
    pub async fn store_as_template(
        &self,
        voxel: &mut VoxelDB,
        intention: &DetectedIntention,
        user_input: &str,
    ) -> Result<()> {
        let template = IntentionTemplate {
            id: uuid::Uuid::new_v4().to_string(),
            mode: intention.mode.clone(),
            example_inputs: vec![user_input.to_string()],
            typical_factors: intention.factors.clone(),
            voxel_coord: self.compute_voxel_coordinate(intention),
        };
        
        voxel.insert_template(template).await
    }
    
    fn compute_voxel_coordinate(&self, intention: &DetectedIntention) -> CubicCoordinate {
        // Map intention to 3D space
        CubicCoordinate {
            q: (intention.factors.verb_weight * 100.0) as i32,
            r: (intention.factors.topic_weight * 100.0) as i32,
            s: (intention.factors.tone_weight * 100.0) as i32,
        }
    }
}
```

### Con CognitiveRouter

```rust
// CognitiveRouter recibe DetectedIntention y activa modo correcto
pub struct CognitiveRouter {
    operational_engine: OperationalProjectEngine,
    learning_engine: LearningAdaptivityEngine,
    conversational_engine: ConversationalEngine,
    // ... otros engines
}

impl CognitiveRouter {
    pub async fn route(
        &self,
        intention: DetectedIntention,
        user_input: &str,
    ) -> Result<Response> {
        match intention.mode {
            CognitiveMode::Operational => {
                self.operational_engine.handle(intention, user_input).await
            },
            CognitiveMode::Procedural => {
                self.procedural_engine.handle(intention, user_input).await
            },
            CognitiveMode::Learning => {
                self.learning_engine.handle(intention, user_input).await
            },
            CognitiveMode::Conversational => {
                self.conversational_engine.handle(intention, user_input).await
            },
            CognitiveMode::Light => {
                self.light_engine.handle(intention, user_input).await
            },
        }
    }
}
```

---

## 🧪 TESTING Y VALIDACIÓN

### Tests Arquitectónicos

#### 1. Property-Based Testing (Invariantes)

```rust
use proptest::prelude::*;

#[cfg(test)]
mod intention_tests {
    use super::*;
    
    // INVARIANTE 1: Confidence siempre en [0.0, 1.0]
    proptest! {
        #[test]
        fn confidence_always_in_range(
            input in "\\PC{100,500}"  // Cualquier string 100-500 chars
        ) {
            let detector = IntentionDetector::new().unwrap();
            let intention = detector.detect(&input).await.unwrap();
            
            prop_assert!(intention.confidence >= 0.0);
            prop_assert!(intention.confidence <= 1.0);
        }
    }
    
    // INVARIANTE 2: Suma de weights = 1.0
    proptest! {
        #[test]
        fn weights_sum_to_one(
            w1 in 0.0f32..1.0,
            w2 in 0.0f32..1.0,
            w3 in 0.0f32..1.0,
            w4 in 0.0f32..1.0,
        ) {
            let total = w1 + w2 + w3 + w4;
            let weights = ScoringWeights {
                verb: w1 / total,
                topic: w2 / total,
                tone: w3 / total,
                context: w4 / total,
            };
            
            let sum = weights.verb + weights.topic + weights.tone + weights.context;
            prop_assert!((sum - 1.0).abs() < 0.001); // Float tolerance
        }
    }
    
    // INVARIANTE 3: Mismo input → mismo output (determinismo)
    #[test]
    fn deterministic_detection() {
        let detector = IntentionDetector::new().unwrap();
        let input = "necesito instalar un switch cisco";
        
        let intention1 = detector.detect(input).await.unwrap();
        let intention2 = detector.detect(input).await.unwrap();
        
        assert_eq!(intention1.mode, intention2.mode);
        assert_eq!(intention1.confidence, intention2.confidence);
    }
}
```

#### 2. Golden Tests (Casos Conocidos)

```rust
#[cfg(test)]
mod golden_tests {
    use super::*;
    
    #[tokio::test]
    async fn operational_project_detection() {
        let detector = IntentionDetector::new().unwrap();
        
        let inputs = vec![
            "necesito instalar un switch en la oficina",
            "quiero configurar un servidor nginx",
            "debo armar una PC nueva",
        ];
        
        for input in inputs {
            let intention = detector.detect(input).await.unwrap();
            assert_eq!(intention.mode, CognitiveMode::Operational);
            assert!(intention.confidence > 0.75);
        }
    }
    
    #[tokio::test]
    async fn learning_mode_detection() {
        let detector = IntentionDetector::new().unwrap();
        
        let inputs = vec![
            "explícame qué es quantum entanglement memory",
            "quiero entender cómo funciona FBCU",
            "por qué usamos coordenadas cúbicas en VoxelDB",
        ];
        
        for input in inputs {
            let intention = detector.detect(input).await.unwrap();
            assert_eq!(intention.mode, CognitiveMode::Learning);
            assert!(intention.confidence > 0.70);
        }
    }
    
    #[tokio::test]
    async fn light_mode_detection() {
        let detector = IntentionDetector::new().unwrap();
        
        let inputs = vec![
            "¿quién ganó el mundial 2018?",
            "¿cuántos bytes en un megabyte?",
            "¿qué hora es en tokyo?",
        ];
        
        for input in inputs {
            let intention = detector.detect(input).await.unwrap();
            assert_eq!(intention.mode, CognitiveMode::Light);
            assert!(intention.confidence > 0.65);
        }
    }
}
```

#### 3. Benchmarks de Performance

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_intention_detection(c: &mut Criterion) {
    let detector = IntentionDetector::new().unwrap();
    
    c.bench_function("detect_short_message", |b| {
        b.iter(|| {
            let input = black_box("¿cómo instalo nginx?");
            detector.detect(input).await
        });
    });
    
    c.bench_function("detect_long_message", |b| {
        b.iter(|| {
            let input = black_box(
                "necesito instalar y configurar un servidor nginx en ubuntu 22.04 \
                 con SSL certificates de let's encrypt y configuración de reverse proxy \
                 para múltiples subdominios, además de logging estructurado en JSON"
            );
            detector.detect(input).await
        });
    });
    
    c.bench_function("detect_with_history", |b| {
        // Pre-populate history
        for i in 0..50 {
            detector.detect(&format!("mensaje {}", i)).await;
        }
        
        b.iter(|| {
            let input = black_box("siguiente mensaje");
            detector.detect(input).await
        });
    });
}

criterion_group!(benches, benchmark_intention_detection);
criterion_main!(benches);

// Expected results:
// detect_short_message:   8-12ms
// detect_long_message:    12-18ms
// detect_with_history:    10-15ms
```

#### 4. Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn full_pipeline_operational_project() {
        let detector = IntentionDetector::new().unwrap();
        let mut telescope = TelescopeDB::new_in_memory().await.unwrap();
        
        // 1. User starts project
        let input1 = "necesito instalar kubernetes";
        let intention1 = detector.detect(input1).await.unwrap();
        
        assert_eq!(intention1.mode, CognitiveMode::Operational);
        
        // 2. Store in TelescopeDB
        let entry_id = detector.store_in_telescopedb(
            &mut telescope,
            input1,
            &intention1,
        ).await.unwrap();
        
        // 3. User continues project
        let input2 = "ya instalé minikube, ¿qué sigue?";
        let intention2 = detector.detect(input2).await.unwrap();
        
        // Should detect project continuation
        assert_eq!(intention2.mode, CognitiveMode::Operational);
        assert!(intention2.factors.context_weight > 0.7);
        
        // 4. Verify stored in TelescopeDB
        let entries = telescope.query_by_metadata(
            "intention_mode",
            "Operational",
        ).await.unwrap();
        
        assert_eq!(entries.len(), 2);
    }
}
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

1. **00_VISION/08_shuidao-cognitive-architecture.md v1.0.0**
   - Visión cognitiva completa
   - 5 Cognitive Modes, 12 Submodes
   - Filosofía ShuiDao (水道)

2. **02_COMPONENTES/04_flowpacks.md**
   - FlowPacks Phase 3a (pattern detection)
   - Evolución hacia intention detection

3. **04_IMPLEMENTACION/FLOWPACKS_IMPLEMENTATION_PLAN.md v2.0.1**
   - Plan de implementación completo
   - Timeline: Phase 3a (✅) → Phase 3b (🚧)

4. **03_INTEGRACION/06_flowpacks-compression.md**
   - Integración FlowPacks + FBCU
   - Context preservation durante detección

### Decisiones Arquitectónicas

- **DA-032:** ShuiDao - Intention-Oriented Cognitive Architecture
- **DA-011:** Dual Database (TelescopeDB + VoxelDB ONLY)
- **DA-001:** ContextToken7D como foundation

### Especificaciones

- **BITA-1:** Context Token 7D (memoria 7-dimensional)
- **BITA-2:** FBCU + TelescopeDB (compresión biográfica)

### Papers Académicos

1. **"BERT for Sequence Classification"** - Devlin et al., 2019
   - Base teórica para embeddings semánticos
   - Transformer architecture aplicada a NLP tasks

2. **"DIET: Lightweight Language Understanding for Dialogue Systems"** - Bunk et al., 2020
   - Rasa's approach: Dual Intent Entity Transformer
   - Multi-task learning para intent + entity recognition

3. **"Plans and the Structure of Behavior"** - Miller, Galanter, Pribram, 1960
   - Psicología cognitiva: goal-oriented behavior
   - TOTE units (Test-Operate-Test-Exit)

4. **"Semantic Similarity Measures in WordNet"** - Pedersen et al., 2004
   - Fundamentos de similarity scoring
   - Aplicado a topic analysis

### Código de Referencia

- **B20250915-data-compressor:** FBCU implementation (compresión base)
- **rust-bert crate:** Pre-trained models (MiniLM-L6-v2)
- **hnsw_rs crate:** Similarity search eficiente (si necesario v2.0)

### RFCs y Estándares

- **CBOR (RFC 8949):** Concise Binary Object Representation
  - Serialización eficiente para TelescopeDB storage
  - https://www.rfc-editor.org/rfc/rfc8949.html

---

## 🚀 MEJORAS FUTURAS

### v1.0: Implementación Base (POST-BETA Phase 3b)

**Scope:**
- ✅ 4 factores: Verb, Topic, Tone, Context
- ✅ 5 Cognitive Modes: Operational, Procedural, Learning, Conversational, Light
- ✅ MiniLM-L6-v2 embeddings
- ✅ Integración TelescopeDB + VoxelDB
- ✅ <15ms latency target

**Timeline:** 76 horas (12.4 - 12.12)

### v1.1: Optimizaciones (v1.1 Q1 2026)

**Optimizations:**
1. **Parallel Factor Analysis**
   ```rust
   let (verb, topic, tone, context) = tokio::join!(...);
   // 8ms latency (down from 15ms)
   ```

2. **Embedding Cache (LRU)**
   ```rust
   cache: LruCache<String, Vec<f32>>  // 1000 entries
   // Cache hit: 1ms vs 5ms
   ```

3. **Adaptive Thresholds**
   ```rust
   // Learn optimal thresholds per user
   thresholds.operational = learn_from_history(user_id);
   ```

4. **Confidence Calibration**
   ```rust
   // Improve accuracy: 90% → 93%
   calibrated_confidence = platt_scaling(raw_confidence);
   ```

### v2.0: Advanced Features (v2.0 Q2 2026)

**New Capabilities:**

1. **Multi-Language Support**
   ```rust
   pub enum Language {
       Spanish,
       English,
       Mixed,  // Spanglish
   }
   
   // Detect language, switch embeddings
   let lang = detector.detect_language(input);
   let embedding = match lang {
       Language::Spanish => spanish_embedder.encode(input),
       Language::English => english_embedder.encode(input),
       Language::Mixed => multilingual_embedder.encode(input),
   };
   ```

2. **Contextual Submodes**
   ```rust
   // Auto-detect submode based on deeper analysis
   if mode == Operational {
       let submode = detect_operational_submode(intention);
       // Project vs Job vs Task classification
   }
   ```

3. **Sentiment Analysis Integration**
   ```rust
   pub struct SentimentFactor {
       valence: f32,    // -1.0 (negative) to 1.0 (positive)
       arousal: f32,    // 0.0 (calm) to 1.0 (excited)
       dominance: f32,  // 0.0 (submissive) to 1.0 (dominant)
   }
   
   // Adjust response style based on sentiment
   if sentiment.valence < -0.5 {
       // User frustrated → empathetic response
   }
   ```

4. **Active Learning**
   ```rust
   // Ask user for feedback when confidence < threshold
   if intention.confidence < 0.70 {
       return Response::AskForClarification {
           options: vec![
               CognitiveMode::Operational,
               CognitiveMode::Learning,
           ],
           question: "¿Quieres que te ayude a hacer esto o a entenderlo?",
       };
   }
   ```

5. **Transfer Learning**
   ```rust
   // Fine-tune embeddings específicos al dominio
   let custom_embedder = MiniLMEmbedder::fine_tune(
       base_model,
       bitacora_corpus,  // Conversaciones de Eduardo
   );
   // Improved accuracy: 90% → 95%
   ```

### Research Ideas (Experimental)

1. **Temporal Attention**
   - Weight recent context more than old context
   - Decay factor: `w(t) = e^(-λt)`

2. **Graph-Based Context**
   - Represent conversation as knowledge graph
   - Use PageRank for topic importance

3. **Hybrid Neural + Symbolic**
   - Neural: embeddings, similarity
   - Symbolic: rules, logic constraints
   - Best of both worlds

4. **Federated Learning**
   - Aggregate patterns across multiple users (privacy-preserving)
   - Improve general model without sharing raw data

---

## 🌐 SEPARATION OF CONCERNS: DETECTION VS NAVIGATION

### Architectural Decision (DA-034)

**Principio:** "Distribución de cargas" - cada componente hace UNA cosa bien.

#### TopicGraph Responsibility (IntentionDetector)

**¿Qué hace?**
- "¿De QUÉ habla el usuario?" → **DETECTION**

**Performance:**
- <15ms (HOT PATH, every message)

**Technology:**
- MiniLM-L6-v2 embeddings (384D)
- Cosine similarity
- Auto-learning from user interactions

**Output:**
- `DetectedIntention` with topics + confidence
- Topic clusters learned from conversations

**Scope:**
- Detection ONLY
- Learning user-specific topic patterns
- Real-time semantic analysis

#### Routier Navigator Responsibility

**¿Qué hace?**
- "¿CÓMO conectar conceptos?" → **NAVIGATION**

**Performance:**
- Background (COLD PATH) or on-demand (WARM PATH <10ms)

**Technology:**
- Small World Networks (Watts-Strogatz, Barabási-Albert)
- Graph algorithms: Dijkstra, PageRank, Betweenness, Louvain
- Weak ties (Granovetter 1973)

**Output:**
- Shortest paths between topics
- Hub detection (highly connected concepts)
- Community clusters (related topics)
- Serendipitous connections (weak ties across domains)
- Learning recommendations (explore new areas)

**Scope:**
- Navigation, routing, discovery
- Background topology refresh (1x/day)
- Insight generation (cross-domain connections)

### Why This Separation?

**Performance:**
```
HOT PATH (cada mensaje):
  TopicGraph.detect() → 12ms ✅
  Routier NOT executed → 0ms overhead ✅
  Total: 12ms (target <15ms) ✅

WARM PATH (usuario pregunta explícitamente "¿cómo se relaciona X con Y?"):
  TopicGraph.detect() → 12ms
  Routier.find_shortest_path() → 5ms
  Total: 17ms (<50ms target) ✅

COLD PATH (background 1x/día):
  PageRank → 12ms
  Betweenness → 180ms
  Louvain → 25ms
  Total: 217ms (offline, zero UX impact) ✅
```

**Separation of Concerns:**
- TopicGraph: Pure detection, fast, always-on
- Routier: Navigation, expensive, on-demand or background
- NO mixing responsibilities (maintainability, testability)

**Mobile-First:**
```yaml
HOT PATH Memory: +0MB (no network topology loaded)
WARM PATH Memory: +5MB (cache shortcuts for 100 topics)
COLD PATH Memory: +28MB (full topology 500 topics)
Battery: <1% per hour (HOT 0%, WARM 0.1%, COLD 0.5%)
```

### Integration via Event Bus

```rust
// TopicGraph notifica cuando detecta nuevo topic
topic_graph.on_new_topic_detected(|topic| {
    event_bus.publish(TopicEvent::NewTopicDetected {
        topic_id: topic.id,
        name: topic.name,
        first_seen: timestamp,
        user_id: user.id,
    });
});

// Routier escucha y actualiza topología (background)
event_bus.subscribe::<TopicEvent>(|event| {
    match event {
        TopicEvent::NewTopicDetected { topic_id, .. } => {
            routier.attach_to_network(
                topic_id,
                mechanism: PreferentialAttachment, // Barabási-Albert
            );
        }
    }
});
```

### See Also

- **DA-034:** `00_VISION/DA-034_SMALL_WORLD_NETWORKS.md` (complete architectural decision)
- **Routier v2.0.0:** `02_COMPONENTES/07_routier-navigator.md` (network algorithms)
- **TopicGraph v1.1.0:** `02_COMPONENTES/CRITICOS/14_shuidao-topic-graph.md` (detection only)

---

**Estado:** 🏗️ ARQUITECTURA TÉCNICA COMPLETA  
**Complejidad:** 🟡 MEDIA  
**Prioridad:** 🔴 ALTA (POST-BETA Phase 3b)

---

*Generado: 2025-11-23 23:46:27*  
*Última Actualización: 2025-11-24 16:15:00 (DA-034 separation note added)*  
*Sistema Bitácora v1.0 - MTT-DSL Template: architecture_spec v1.0.0*
