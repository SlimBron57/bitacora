```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/13b_shuidao-emotional-space.md
Versión: 1.0.0
Fecha Creación: 2025-11-24
Última Actualización: 2025-11-24
Autor: Sistema Bitácora - En colaboración con Eduardo
Propósito: Especificación técnica del EmotionalSpace - Sistema dinámico de tonos emocionales continuos
Estado: 🔴 CRÍTICO - Refactor arquitectónico DA-033
Template: component_spec.yaml v1.0.0 (MTT-DSL)
Relacionado Con:
  - ROADMAP_V2/00_VISION/DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md (decisión arquitectónica)
  - ROADMAP_V2/01_ARQUITECTURA/12_shuidao-intention-detection.md (IntentionDetector)
  - ROADMAP_V2/07_TEMPLATES/tone_templates/ (MTT-DSL templates)
Fundamentos:
  - Psychology: VAD Model (Valence-Arousal-Dominance) - Russell (1980)
  - NLP: Sentiment Analysis, Emotion Recognition
  - Linguistics: Lexical Semantics, Syntactic Patterns
  - ML: Clustering (K-means), Dimensional Reduction
Implementa: DA-033 (Dynamic Topic & Tone System)
Reemplaza: ToneType enum (5 tonos estáticos)
# === FIN DATOS DE AUDITORÍA ===
```

# 🎭 EmotionalSpace: Sistema Dinámico de Tonos Emocionales

## **Espacio continuo VAD+F** - Beyond discrete emotion categories

---

## 🎯 PROPÓSITO

### ¿Qué es EmotionalSpace?

**EmotionalSpace** es un espacio emocional continuo de 4 dimensiones (Valence, Arousal, Dominance, Formality) que representa los tonos emocionales únicos de cada usuario. En lugar de categorías discretas, usa **ToneClusters** dinámicos que el usuario puede nombrar según su estilo personal.

### Problema que Resuelve

**ToneType enum anterior (❌):**
```rust
pub enum ToneType {
    Pragmatic,     // Solo 5 tonos fijos
    Curious,       // Todos los usuarios = mismos tonos
    Frustrated,    // ¿Y "determinado"? ¿"nostálgico"? ¿"emocionado"?
    Casual,        // Modelo discreto vs realidad continua
    Reflective,
}
```

**Limitaciones fatales:**
1. ❌ Usuario está "determinado" → ¿Pragmatic? ¿NO FIT
2. ❌ Usuario está "nostálgico" → ¿Reflective? INCOMPLETO
3. ❌ Usuario está "emocionado" → ¿Curious? AMBIGUO
4. ❌ No captura matices: "ligeramente frustrado" vs "muy frustrado"
5. ❌ Modelo discreto: realidad emocional es CONTINUA

**EmotionalSpace (✅):**
```rust
// Eduardo: Tono "Determinado"
ToneDimensions {
    valence: 0.3,      // Ligeramente positivo
    arousal: 0.7,      // Alta energía
    dominance: 0.8,    // Muy asertivo
    formality: 0.5,    // Neutral
}
// Sistema: "Detecto que estás determinado. Voy con pasos concretos."

// Eduardo: Tono "Nostálgico"
ToneDimensions {
    valence: 0.1,      // Ligeramente positivo (memorias dulces)
    arousal: -0.4,     // Energía baja (reflexivo)
    dominance: -0.2,   // Pasivo (recordando)
    formality: 0.4,    // Semi-formal (narrativo)
}
// Sistema: "Quieres compartir esos recuerdos con más detalle?"
```

### Principios Fundamentales

#### 1. **Continuous Emotional Space (No Discreto)**

```rust
// ❌ ANTES: enum fijo
let tone = ToneType::Pragmatic; // Solo 5 opciones

// ✅ AHORA: espacio continuo 4D
let tone = ToneDimensions {
    valence: 0.3,    // -1.0 a 1.0
    arousal: 0.7,    // -1.0 a 1.0
    dominance: 0.8,  // -1.0 a 1.0
    formality: 0.5,  // -1.0 a 1.0
};
// Infinitas combinaciones posibles
```

#### 2. **VAD+F Model (Dimensional Approach)**

```
Modelo VAD (Russell, 1980):
- Valence: Positivo/Negativo (feliz vs triste)
- Arousal: Energía (excitado vs calmado)
- Dominance: Control (dominante vs sumiso)

+ Formality (extensión Bitácora):
- Formality: Registro (formal vs casual)
```

**Ejemplo mapping:**

```
"Voy a terminar este proyecto cueste lo que cueste"
→ Determinado: V=0.3, A=0.7, D=0.8, F=0.5

"No puedo creer que esto no funcione, es frustrante"
→ Frustrado: V=-0.6, A=0.5, D=-0.3, F=0.3

"¡Estoy tan emocionado por aprender esto!"
→ Emocionado: V=0.9, A=0.8, D=0.5, F=0.2

"Me pregunto por qué funciona así..."
→ Curioso: V=0.2, A=0.3, D=0.1, F=0.4

"Recuerdo cuando construimos aquella casa..."
→ Nostálgico: V=0.1, A=-0.4, D=-0.2, F=0.4
```

#### 3. **User-Named Clusters**

```rust
// Usuario detecta nuevo tono emocional
System: "Detecto un tono nuevo en ti:
         - Muy positivo (valence: 0.9)
         - Alta energía (arousal: 0.8)
         - Moderadamente asertivo (dominance: 0.5)
         ¿Cómo lo describirías?"

User: "Emocionado"

// Sistema crea ToneCluster "Emocionado"
ToneCluster {
    id: "tone_eduardo_emocionado_001",
    name: "Emocionado",  // USER NAMED ✅
    center: ToneDimensions { v:0.9, a:0.8, d:0.5, f:0.2 },
    radius: 0.25,
    user_id: "eduardo_001",
}
```

#### 4. **Lexical + Syntactic Analysis**

```rust
// Análisis léxico
"¡Voy a terminar este proyecto cueste lo que cueste!"

Lexical Markers:
- strong_verbs: ["voy a", "terminar"]
- commitment_phrases: ["cueste lo que cueste"]
- exclamation_count: 1
- uppercase_words: 0

// Análisis sintáctico
Syntactic Patterns:
- pattern: "voy a <verb> <complement> cueste lo que cueste"
- first_person_assertive: true
- imperative_mood: false
- question_form: false

→ ToneDimensions { v:0.3, a:0.7, d:0.8, f:0.5 } → "Determinado"
```

---

## 🏗️ ARQUITECTURA

### Estructuras de Datos

#### 1. EmotionalSpace (Core Structure)

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};

pub struct EmotionalSpace {
    user_id: UserId,
    
    // Clusters de tonos descubiertos
    clusters: HashMap<ToneClusterId, ToneCluster>,
    
    // Templates MTT-DSL (paths en VoxelDB)
    template_paths: HashMap<ToneClusterId, String>,
    
    // Metadata
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl EmotionalSpace {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            clusters: HashMap::new(),
            template_paths: HashMap::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }
    
    // Buscar cluster más cercano
    pub fn find_nearest_cluster(&self, dimensions: &ToneDimensions) 
        -> Option<(ToneClusterId, f32)> 
    {
        let mut best_match: Option<(ToneClusterId, f32)> = None;
        
        for (cluster_id, cluster) in &self.clusters {
            let distance = euclidean_distance(dimensions, &cluster.center);
            
            // Si está dentro del radio del cluster
            if distance <= cluster.radius {
                if let Some((_, best_dist)) = best_match {
                    if distance < best_dist {
                        best_match = Some((*cluster_id, distance));
                    }
                } else {
                    best_match = Some((*cluster_id, distance));
                }
            }
        }
        
        best_match
    }
}
```

#### 2. ToneDimensions (VAD+F Model)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToneDimensions {
    pub valence: f32,     // -1.0 (negativo) a 1.0 (positivo)
    pub arousal: f32,     // -1.0 (calmado) a 1.0 (excitado)
    pub dominance: f32,   // -1.0 (sumiso) a 1.0 (dominante)
    pub formality: f32,   // -1.0 (casual) a 1.0 (formal)
}

impl ToneDimensions {
    pub fn new(valence: f32, arousal: f32, dominance: f32, formality: f32) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(-1.0, 1.0),
            dominance: dominance.clamp(-1.0, 1.0),
            formality: formality.clamp(-1.0, 1.0),
        }
    }
    
    // Distancia euclidiana en espacio 4D
    pub fn distance_to(&self, other: &ToneDimensions) -> f32 {
        let dv = self.valence - other.valence;
        let da = self.arousal - other.arousal;
        let dd = self.dominance - other.dominance;
        let df = self.formality - other.formality;
        
        (dv*dv + da*da + dd*dd + df*df).sqrt()
    }
    
    // Descripciones humanas
    pub fn describe_valence(&self) -> &'static str {
        match self.valence {
            v if v > 0.6 => "muy positivo",
            v if v > 0.2 => "positivo",
            v if v > -0.2 => "neutral",
            v if v > -0.6 => "negativo",
            _ => "muy negativo",
        }
    }
    
    pub fn describe_arousal(&self) -> &'static str {
        match self.arousal {
            a if a > 0.6 => "muy energético",
            a if a > 0.2 => "energético",
            a if a > -0.2 => "neutral",
            a if a > -0.6 => "calmado",
            _ => "muy calmado",
        }
    }
    
    pub fn describe_dominance(&self) -> &'static str {
        match self.dominance {
            d if d > 0.6 => "muy asertivo",
            d if d > 0.2 => "asertivo",
            d if d > -0.2 => "neutral",
            d if d > -0.6 => "pasivo",
            _ => "muy pasivo",
        }
    }
    
    pub fn describe_formality(&self) -> &'static str {
        match self.formality {
            f if f > 0.6 => "muy formal",
            f if f > 0.2 => "formal",
            f if f > -0.2 => "neutral",
            f if f > -0.6 => "casual",
            _ => "muy casual",
        }
    }
}
```

#### 3. ToneCluster (User-Defined Emotional Region)

```rust
pub struct ToneCluster {
    pub id: ToneClusterId,
    pub name: String,  // "Determinado", "Nostálgico", "Emocionado" - USER NAMED ✅
    pub center: ToneDimensions,
    pub radius: f32,  // Tolerancia para matching
    pub examples: Vec<String>,
    pub user_id: UserId,
    pub discovered_at: DateTime<Utc>,
    pub interaction_count: u32,
    
    // Análisis lingüístico
    pub lexical_markers: LexicalMarkers,
    pub syntactic_patterns: Vec<SyntacticPattern>,
}

pub struct LexicalMarkers {
    pub strong_verbs: Vec<String>,         // "voy a", "terminar", "lograr"
    pub commitment_phrases: Vec<String>,   // "cueste lo que cueste", "sin excusas"
    pub time_markers: Vec<String>,         // "hoy", "ahora", "ya"
    pub emotional_adjectives: Vec<String>, // "determinado", "emocionado", "frustrado"
    pub exclamation_count: u32,
    pub question_count: u32,
    pub uppercase_words: Vec<String>,
}

pub struct SyntacticPattern {
    pub pattern: String,  // "voy a <verb> <complement> cueste lo que cueste"
    pub confidence: f32,
    pub examples: Vec<String>,
}
```

**Ejemplo: Eduardo's ToneCluster "Determinado"**

```rust
ToneCluster {
    id: ToneClusterId("tone_eduardo_determinado_001"),
    name: "Determinado".to_string(),
    center: ToneDimensions {
        valence: 0.3,
        arousal: 0.7,
        dominance: 0.8,
        formality: 0.5,
    },
    radius: 0.25,
    examples: vec![
        "Voy a terminar este proyecto cueste lo que cueste".to_string(),
        "Lo voy a hacer, no hay alternativa".to_string(),
        "Esto se completa hoy sí o sí".to_string(),
    ],
    user_id: UserId("eduardo_001"),
    discovered_at: "2025-11-24T13:50:00Z".parse().unwrap(),
    interaction_count: 42,
    
    lexical_markers: LexicalMarkers {
        strong_verbs: vec!["voy a", "terminar", "completar", "lograr"],
        commitment_phrases: vec!["cueste lo que cueste", "sin excusas", "sí o sí"],
        time_markers: vec!["hoy", "ahora", "ya"],
        emotional_adjectives: vec!["determinado"],
        exclamation_count: 1,
        question_count: 0,
        uppercase_words: vec![],
    },
    
    syntactic_patterns: vec![
        SyntacticPattern {
            pattern: "voy a <verb> <complement> cueste lo que cueste".to_string(),
            confidence: 0.95,
            examples: vec!["Voy a terminar este proyecto cueste lo que cueste"],
        },
        SyntacticPattern {
            pattern: "esto se <verb> <time_marker> sí o sí".to_string(),
            confidence: 0.90,
            examples: vec!["Esto se completa hoy sí o sí"],
        },
    ],
}
```

#### 4. DynamicToneDetector (Main Interface)

```rust
pub struct DynamicToneDetector {
    user_spaces: Arc<RwLock<HashMap<UserId, EmotionalSpace>>>,
    voxeldb: Arc<VoxelDB>,
    template_engine: Arc<MttDslEngine>,
    lexical_analyzer: LexicalAnalyzer,
    syntactic_analyzer: SyntacticAnalyzer,
    sentiment_analyzer: SentimentAnalyzer,
}

impl DynamicToneDetector {
    pub fn detect(&self, message: &ProcessedMessage, user_id: &UserId) 
        -> ToneDetection 
    {
        // 1. Calcular dimensiones VAD+F
        let dimensions = self.compute_dimensions(message);
        
        // 2. Buscar cluster en EmotionalSpace del usuario
        let spaces = self.user_spaces.read().unwrap();
        
        if let Some(space) = spaces.get(user_id) {
            if let Some((cluster_id, distance)) = space.find_nearest_cluster(&dimensions) {
                let cluster = &space.clusters[&cluster_id];
                let confidence = 1.0 - (distance / cluster.radius);
                
                return ToneDetection {
                    dimensions,
                    cluster_id: Some(cluster_id),
                    cluster_name: Some(cluster.name.clone()),
                    confidence,
                    is_new_tone: false,
                    requires_confirmation: false,
                };
            }
        }
        
        // 3. Nuevo tono detectado
        ToneDetection {
            dimensions,
            cluster_id: None,
            cluster_name: None,
            confidence: 0.0,
            is_new_tone: true,
            requires_confirmation: true,
        }
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

---

## 🧠 ALGORITMOS

### 1. Compute Dimensions (VAD+F Analysis)

```rust
impl DynamicToneDetector {
    fn compute_dimensions(&self, message: &ProcessedMessage) -> ToneDimensions {
        // 1. Análisis léxico
        let lexical = self.lexical_analyzer.analyze(&message.normalized);
        
        // 2. Análisis sintáctico
        let syntactic = self.syntactic_analyzer.analyze(&message.normalized);
        
        // 3. Análisis de sentimiento
        let sentiment = self.sentiment_analyzer.analyze(&message.normalized);
        
        // 4. Calcular cada dimensión
        ToneDimensions {
            valence: self.compute_valence(&lexical, &sentiment),
            arousal: self.compute_arousal(&lexical, &syntactic),
            dominance: self.compute_dominance(&lexical, &syntactic),
            formality: self.compute_formality(&lexical, &syntactic),
        }
    }
    
    fn compute_valence(&self, lex: &LexicalFeatures, sent: &SentimentScore) -> f32 {
        let mut score = 0.0;
        
        // Sentiment analysis (base)
        score += sent.polarity;  // -1.0 a 1.0
        
        // Positive words (+)
        score += lex.positive_words.len() as f32 * 0.1;
        
        // Negative words (-)
        score -= lex.negative_words.len() as f32 * 0.1;
        
        // Emotional adjectives
        for adj in &lex.emotional_adjectives {
            score += self.adjective_valence_score(adj);
        }
        
        score.clamp(-1.0, 1.0)
    }
    
    fn compute_arousal(&self, lex: &LexicalFeatures, syn: &SyntacticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Exclamation marks (+0.3)
        score += syn.exclamation_count as f32 * 0.3;
        
        // High-energy verbs (+0.2): "correr", "saltar", "explotar"
        score += lex.high_energy_verbs.len() as f32 * 0.2;
        
        // Uppercase words (+0.15)
        score += lex.uppercase_words.len() as f32 * 0.15;
        
        // Repetition (!!!, ???) (+0.2)
        score += syn.repeated_punctuation as f32 * 0.2;
        
        // Low-energy verbs (-0.2): "dormir", "descansar", "reflexionar"
        score -= lex.low_energy_verbs.len() as f32 * 0.2;
        
        // Long pauses (...) (-0.1)
        score -= syn.ellipsis_count as f32 * 0.1;
        
        score.clamp(-1.0, 1.0)
    }
    
    fn compute_dominance(&self, lex: &LexicalFeatures, syn: &SyntacticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Imperative verbs (+0.4): "haz", "debes", "tienes que"
        score += syn.imperative_verbs.len() as f32 * 0.4;
        
        // First-person assertive (+0.3): "voy a", "haré", "lograré"
        score += lex.assertive_first_person.len() as f32 * 0.3;
        
        // Modal verbs of certainty (+0.2): "definitivamente", "seguro"
        score += lex.certainty_modals.len() as f32 * 0.2;
        
        // Questions (-0.3): "¿podrías?", "¿me ayudas?"
        score -= syn.question_count as f32 * 0.3;
        
        // Conditional mood (-0.2): "si pudiera", "ojalá"
        score -= syn.conditional_count as f32 * 0.2;
        
        // Hedging words (-0.15): "quizás", "tal vez", "creo que"
        score -= lex.hedging_words.len() as f32 * 0.15;
        
        score.clamp(-1.0, 1.0)
    }
    
    fn compute_formality(&self, lex: &LexicalFeatures, syn: &SyntacticFeatures) -> f32 {
        let mut score = 0.0;
        
        // Formal vocabulary (+0.3): "usted", "estimado", "por favor"
        score += lex.formal_words.len() as f32 * 0.3;
        
        // Technical terminology (+0.2)
        score += lex.technical_terms.len() as f32 * 0.2;
        
        // Complete sentences (+0.1)
        score += syn.complete_sentences as f32 * 0.1;
        
        // Slang/colloquialisms (-0.3): "qué onda", "chido", "wey"
        score -= lex.slang_words.len() as f32 * 0.3;
        
        // Contractions (-0.2): "pa'", "'tá", "q"
        score -= lex.contractions.len() as f32 * 0.2;
        
        // Emoticons/emoji (-0.2)
        score -= lex.emoticons.len() as f32 * 0.2;
        
        // Fragments (-0.1): oraciones incompletas
        score -= syn.fragments as f32 * 0.1;
        
        score.clamp(-1.0, 1.0)
    }
}
```

### 2. Tone Learning (Auto-Discovery)

```rust
impl DynamicToneDetector {
    pub async fn learn_new_tone(
        &self,
        user_id: &UserId,
        tone_name: String,
        dimensions: ToneDimensions,
        example_text: &str,
    ) -> Result<ToneClusterId> {
        let cluster_id = ToneClusterId::new();
        
        // 1. Extraer markers lingüísticos del ejemplo
        let lexical_markers = self.lexical_analyzer.extract_markers(example_text);
        let syntactic_patterns = self.syntactic_analyzer.extract_patterns(example_text);
        
        // 2. Crear ToneCluster
        let cluster = ToneCluster {
            id: cluster_id,
            name: tone_name.clone(),
            center: dimensions,
            radius: 0.25,  // default tolerance
            examples: vec![example_text.to_string()],
            user_id: user_id.clone(),
            discovered_at: Utc::now(),
            interaction_count: 1,
            lexical_markers,
            syntactic_patterns,
        };
        
        // 3. Agregar a EmotionalSpace
        let mut spaces = self.user_spaces.write().unwrap();
        let space = spaces.entry(user_id.clone())
            .or_insert_with(|| EmotionalSpace::new(user_id.clone()));
        
        space.clusters.insert(cluster_id, cluster.clone());
        
        // 4. Crear template MTT-DSL
        let template_path = format!("users/{}/tones/{}.yaml", user_id, tone_name);
        self.create_tone_template(user_id, &cluster, &template_path).await?;
        space.template_paths.insert(cluster_id, template_path);
        
        // 5. Persistir en VoxelDB
        self.persist_emotional_space(user_id, space).await?;
        
        Ok(cluster_id)
    }
    
    // Actualizar cluster existente con nuevo ejemplo
    pub fn update_cluster(&self, user_id: &UserId, cluster_id: ToneClusterId, example: &str) {
        let mut spaces = self.user_spaces.write().unwrap();
        
        if let Some(space) = spaces.get_mut(user_id) {
            if let Some(cluster) = space.clusters.get_mut(&cluster_id) {
                cluster.examples.push(example.to_string());
                cluster.interaction_count += 1;
                
                // Recalcular markers (promedio con nuevos ejemplos)
                let new_markers = self.lexical_analyzer.extract_markers(example);
                cluster.lexical_markers.merge(new_markers);
            }
        }
    }
}
```

### 3. Cluster Refinement (K-means Adaptation)

```rust
impl EmotionalSpace {
    // Refinar clusters periódicamente con K-means
    pub fn refine_clusters(&mut self) {
        let mut all_samples: Vec<(ToneClusterId, ToneDimensions)> = Vec::new();
        
        // Recopilar todas las detecciones históricas
        for (cluster_id, cluster) in &self.clusters {
            for example in &cluster.examples {
                // Recompute dimensions de cada ejemplo
                let dimensions = recompute_dimensions(example);
                all_samples.push((*cluster_id, dimensions));
            }
        }
        
        // K-means: recalcular centros de clusters
        let k = self.clusters.len();
        let new_centers = kmeans_4d(&all_samples, k, max_iterations=100);
        
        // Actualizar centers
        for (cluster_id, new_center) in new_centers {
            if let Some(cluster) = self.clusters.get_mut(&cluster_id) {
                cluster.center = new_center;
            }
        }
    }
}

fn kmeans_4d(samples: &[(ToneClusterId, ToneDimensions)], k: usize, max_iterations: usize) 
    -> HashMap<ToneClusterId, ToneDimensions> 
{
    // Implementación K-means para espacio 4D
    // (simplificado, usar librería rkm en producción)
    let mut centers = HashMap::new();
    
    for _ in 0..max_iterations {
        // Assign samples to nearest center
        let mut assignments: HashMap<ToneClusterId, Vec<ToneDimensions>> = HashMap::new();
        
        for (cluster_id, dimensions) in samples {
            assignments.entry(*cluster_id).or_default().push(*dimensions);
        }
        
        // Recompute centers (mean)
        for (cluster_id, dims_vec) in assignments {
            let center = compute_mean_dimensions(&dims_vec);
            centers.insert(cluster_id, center);
        }
    }
    
    centers
}

fn compute_mean_dimensions(dims: &[ToneDimensions]) -> ToneDimensions {
    let n = dims.len() as f32;
    let sum = dims.iter().fold(
        ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
        |acc, d| ToneDimensions {
            valence: acc.valence + d.valence,
            arousal: acc.arousal + d.arousal,
            dominance: acc.dominance + d.dominance,
            formality: acc.formality + d.formality,
        }
    );
    
    ToneDimensions {
        valence: sum.valence / n,
        arousal: sum.arousal / n,
        dominance: sum.dominance / n,
        formality: sum.formality / n,
    }
}
```

---

## 🗄️ PERSISTENCIA (VoxelDB)

### Template MTT-DSL

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
    
  emotional_adjectives:
    - "determinado"
    
syntactic_patterns:
  - pattern: "voy a <verb> <complement> cueste lo que cueste"
    confidence: 0.95
    examples:
      - "Voy a terminar este proyecto cueste lo que cueste"
      
  - pattern: "esto se <verb> <time_marker> sí o sí"
    confidence: 0.90
    examples:
      - "Esto se completa hoy sí o sí"
      
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

### VoxelDB Structure

```
VoxelDB/
  users/
    eduardo_001/
      emotional_space.json       ← EmotionalSpace serialized
      tones/
        determinado.yaml
        nostalgico.yaml
        curioso.yaml
        emocionado.yaml
        ...
        
    esposa_001/
      emotional_space.json
      tones/
        entusiasmada.yaml
        reflexiva.yaml
        directa.yaml
        ...
```

---

## 📊 TESTING

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dimensions_computation() {
        let detector = DynamicToneDetector::new();
        let message = ProcessedMessage {
            normalized: "¡Voy a terminar este proyecto cueste lo que cueste!".to_string(),
        };
        
        let dimensions = detector.compute_dimensions(&message);
        
        // Determinado: positivo, alta energía, asertivo
        assert!(dimensions.valence > 0.2);
        assert!(dimensions.arousal > 0.6);
        assert!(dimensions.dominance > 0.7);
    }
    
    #[test]
    fn test_tone_detection_match() {
        let detector = DynamicToneDetector::new();
        let user_id = UserId("eduardo_001".to_string());
        
        // Setup: Eduardo tiene cluster "Determinado"
        detector.learn_new_tone(
            &user_id,
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "Voy a terminar este proyecto cueste lo que cueste",
        ).await.unwrap();
        
        // Test: detectar tono similar
        let message = ProcessedMessage {
            normalized: "Lo voy a hacer sin excusas".to_string(),
        };
        let detection = detector.detect(&message, &user_id);
        
        assert!(!detection.is_new_tone);
        assert_eq!(detection.cluster_name, Some("Determinado".to_string()));
        assert!(detection.confidence > 0.7);
    }
    
    #[test]
    fn test_tone_detection_new() {
        let detector = DynamicToneDetector::new();
        let user_id = UserId("eduardo_001".to_string());
        
        // Test: tono nunca visto
        let message = ProcessedMessage {
            normalized: "Recuerdo cuando construimos aquella casa...".to_string(),
        };
        let detection = detector.detect(&message, &user_id);
        
        assert!(detection.is_new_tone);
        assert!(detection.requires_confirmation);
        
        // Dimensions esperadas: Nostálgico
        assert!(detection.dimensions.valence > -0.2 && detection.dimensions.valence < 0.3);
        assert!(detection.dimensions.arousal < 0.0); // Baja energía
    }
    
    #[test]
    fn test_cluster_distance() {
        let center = ToneDimensions::new(0.3, 0.7, 0.8, 0.5);
        
        // Dentro del cluster (radius=0.25)
        let close = ToneDimensions::new(0.35, 0.75, 0.85, 0.55);
        assert!(center.distance_to(&close) < 0.25);
        
        // Fuera del cluster
        let far = ToneDimensions::new(-0.5, -0.4, 0.1, 0.2);
        assert!(center.distance_to(&far) > 0.25);
    }
}
```

---

## 🎯 MÉTRICAS DE ÉXITO

| Métrica | Target | Verificación |
|---------|--------|--------------|
| **Tone Detection Accuracy** | >80% | Tonos detectados correctamente |
| **Dimension Accuracy** | RMSE <0.2 | Error en cálculo VAD+F |
| **False Positives** | <15% | Nuevo tono mal clasificado como existente |
| **User Confirmation Rate** | <25% | Confirmaciones vs automáticas |
| **Response Time** | <20ms | Cálculo dimensions + cluster lookup |
| **Cluster Stability** | >90% | Clusters no cambian drásticamente |
| **User Satisfaction** | >85% | Respuestas adaptadas correctamente |

---

## 🔗 INTEGRACIÓN

### Con IntentionDetector

```rust
impl IntentionDetector {
    pub fn detect(&self, message: &str, user_id: &UserId) -> DetectedIntention {
        let processed = self.preprocessor.process(message);
        
        // NUEVO: DynamicToneDetector
        let tone_detection = self.tone_detector.detect(&processed, user_id);
        
        // Si es nuevo tono: pausar y pedir confirmación
        if tone_detection.is_new_tone {
            return DetectedIntention::new_tone_confirmation(
                tone_detection.dimensions
            );
        }
        
        // Normal flow
        let verb = self.verb_classifier.classify(&processed);
        let topic = self.topic_analyzer.detect_topics(&processed, user_id);
        let context = self.conversation_history.analyze_context(&processed);
        
        self.intention_scorer.score_and_select(verb, topic, tone_detection, context)
    }
}
```

### Con Response Synthesis

```rust
impl ResponseSynthesizer {
    pub fn adapt_to_tone(&self, response: &str, tone: &ToneDetection) -> String {
        if let Some(cluster_name) = &tone.cluster_name {
            // Cargar template de respuesta para este tono
            let template = self.load_tone_template(&cluster_name);
            
            // Adaptar respuesta según dimensiones
            let adapted = match template.style.as_str() {
                "direct_supportive" => self.make_direct_supportive(response, &tone.dimensions),
                "reflective_gentle" => self.make_reflective_gentle(response, &tone.dimensions),
                "energetic_enthusiastic" => self.make_energetic(response, &tone.dimensions),
                _ => response.to_string(),
            };
            
            adapted
        } else {
            response.to_string()
        }
    }
    
    fn make_direct_supportive(&self, response: &str, dims: &ToneDimensions) -> String {
        // Ajustar energía de respuesta según arousal del usuario
        let energy_multiplier = (dims.arousal + 1.0) / 2.0; // map [-1,1] → [0,1]
        
        if energy_multiplier > 0.7 {
            format!("¡Perfecto! {}", response) // Alta energía
        } else {
            response.to_string() // Energía normal
        }
    }
}
```

---

## 🎓 CONCLUSIÓN

EmotionalSpace es la **diferencia fundamental** entre:

❌ **Asistente con 5 tonos fijos** (Pragmatic, Curious, etc.)  
✅ **Compañero que entiende TU estilo emocional único**

**Sin EmotionalSpace:**
- Usuario está "determinado" → ¿Pragmatic? NO FIT
- Sistema responde igual a "frustrado" que a "emocionado"
- No aprende matices personales
- Modelo discreto (5 opciones)

**Con EmotionalSpace:**
- Eduardo está "determinado" → detecta cluster (V:0.3, A:0.7, D:0.8, F:0.5)
- Respuestas adaptadas: "¡Perfecto! Aquí los pasos concretos para terminar hoy"
- Aprende nuevos tonos dinámicamente con confirmación
- Modelo continuo (infinitas combinaciones)

**Esto es adaptación emocional real.**

---

**Estado:** 🔴 CRÍTICO - Implementación prioritaria  
**Owner:** B (Sistema Bitácora) + Eduardo  
**Next:** Implementación src/shuidao/emotional_space.rs (~600 líneas)
