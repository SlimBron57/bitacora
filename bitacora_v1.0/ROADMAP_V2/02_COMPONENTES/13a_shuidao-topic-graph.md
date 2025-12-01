```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/13a_shuidao-topic-graph.md
Versión: 1.1.0
Fecha Creación: 2025-11-24
Última Actualización: 2025-11-24
Autor: Sistema Bitácora - En colaboración con Eduardo
Propósito: Especificación técnica del TopicGraph - Sistema dinámico de topics personalizados (DETECTION ONLY)
Estado: 🔴 CRÍTICO - Refactor arquitectónico DA-033
Template: component_spec.yaml v1.0.0 (MTT-DSL)
Relacionado Con:
  - ROADMAP_V2/00_VISION/DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md (decisión arquitectónica)
  - ROADMAP_V2/00_VISION/DA-034_SMALL_WORLD_NETWORKS.md (navegación en Routier)
  - ROADMAP_V2/01_ARQUITECTURA/12_shuidao-intention-detection.md (IntentionDetector)
  - ROADMAP_V2/02_COMPONENTES/07_routier-navigator.md (navegación)
  - ROADMAP_V2/07_TEMPLATES/topic_templates/ (MTT-DSL templates)
Fundamentos:
  - NLP: Sentence Embeddings (MiniLM-L6-v2), Semantic Similarity
  - Psychology: Personal Interest Models, Topic Isolation
  - VoxelDB: Spatial indexing con CubicCoords
Implementa: DA-033 (Dynamic Topic & Tone System - Detection)
Reemplaza: TopicDomain enum (7 categorías estáticas)
Nota v1.1: Navegación (hubs, paths, communities) movida a Routier Navigator (DA-034)
# === FIN DATOS DE AUDITORÍA ===
```

# 🌐 TopicGraph: Sistema Dinámico de Topics Personalizados

## **"Juntos pero no revueltos"** - Topic isolation & personalization

---

## 🎯 PROPÓSITO

### ¿Qué es TopicGraph?

**TopicGraph** es un grafo dirigido acíclico (DAG) que representa el universo único de intereses de cada usuario. Cada nodo es un **UserDefinedTopic** (definido por el usuario, no hardcoded), y las aristas representan relaciones semánticas.

> **⚠️ RESPONSABILIDAD ENFOCADA:** TopicGraph se encarga SOLO de **detection** ("¿de qué habla?"). Para **navigation** entre topics (shortest paths, hubs, Small World Networks), ver **Routier Navigator** (DA-034).

### Separación de Responsabilidades

```yaml
TopicGraph (este componente):
  Responsabilidad: Topic Detection (<15ms)
  Algoritmos: MiniLM embeddings, cosine similarity
  Scope: Identificar topics del usuario
  
Routier Navigator (02_COMPONENTES/07_routier-navigator.md):
  Responsabilidad: Topic Navigation (background/on-demand)
  Algoritmos: Dijkstra, PageRank, Louvain, BFS
  Scope: Conectar conceptos, sugerir paths, serendipity
```

### Problema que Resuelve

**TopicDomain enum anterior (❌):**
```rust
pub enum TopicDomain {
    Infrastructure,  // Solo 7 categorías
    Software,        // Todos los usuarios = mismo enum
    Learning,        // No aprende nuevos topics
    Personal,        // Demasiado genérico
    Psychology,
    Biography,
    Routine,
}
```

**Limitaciones fatales:**
1. ❌ Usuario interesado en "Cerámica" → ¿Infrastructure? ¿Personal? NO FIT
2. ❌ Usuario habla de "Microprocesadores" → ¿Software? ¿Learning? AMBIGUO
3. ❌ Esposa habla de "Armas" → ¿NO EXISTE categoría
4. ❌ Eduardo habla de "Espiritualidad" Y "Tecnología" → ¡SE MEZCLAN!
5. ❌ No personalización: Eduardo = Esposa = cualquier usuario

**TopicGraph (✅):**
```rust
// Eduardo's TopicGraph
TopicGraph {
    user_id: "eduardo_001",
    root_topics: {
        "topic_001": RootTopic { name: "Cerámica", weight: 0.85 },
        "topic_002": RootTopic { name: "Rust", weight: 1.00 },
        "topic_003": RootTopic { name: "Filosofía", weight: 0.80 },
        "topic_004": RootTopic { name: "Microprocesadores", weight: 0.70 },
        // ... ILIMITADOS
    },
    edges: [
        ("Cerámica", "Química", Complementary),  // Esmaltes requieren química
        ("Filosofía", "Espiritualidad", Hierarchical),
    ],
    isolation_rules: {
        ("Espiritualidad", "Tecnología"): IsolationMode::Strict, // NUNCA mezclar
    }
}
```

### Principios Fundamentales

#### 1. **User-Defined, Not Hardcoded**

```rust
// ❌ ANTES: enum fijo
let topic = TopicDomain::Infrastructure; // Solo 7 opciones

// ✅ AHORA: ilimitado, user-defined
let topic = graph.learn_topic("eduardo_001", "Fermentación", "Quiero hacer kimchi");
// Sistema aprende topic único del usuario
```

#### 2. **"Juntos pero no revueltos"**

```rust
// Eduardo habla de Espiritualidad
DetectedIntention {
    topics: ["Espiritualidad"],
    isolation: IsolationMode::Strict,
}

// Más tarde habla de Microprocesadores
DetectedIntention {
    topics: ["Microprocesadores"],
    isolation: IsolationMode::Strict,
}

// Sistema NO mezcla automáticamente
// A menos que Eduardo diga: "La espiritualidad de la tecnología"
// → Entonces pide confirmación para crear edge
```

#### 3. **Interest Weights (Pesos Personalizados)**

```rust
InterestWeight {
    explicit: 0.9,   // Eduardo dijo "me apasiona Rust"
    implicit: 0.95,  // Menciona Rust cada día (alta frecuencia)
    temporal: 1.0,   // Hablado recientemente
    emotional: 0.85, // Siempre sentimiento positivo
    
    combined: 0.925  // Rust es su máximo interés
}

vs

InterestWeight {
    explicit: 0.5,   // Eduardo dijo "microprocesadores son interesantes"
    implicit: 0.3,   // Rara vez menciona
    temporal: 0.6,   // Última mención hace 1 mes
    emotional: 0.6,  // Sentimiento neutral
    
    combined: 0.475  // Interés moderado
}
```

#### 4. **Auto-Discovery con User Confirmation**

```
Usuario: "Me gustaría aprender sobre fermentación de vegetales"

TopicGraph:
  - Embedding del texto
  - Buscar en topics existentes
  - Best match: "Cocina" (similarity: 0.62) ← bajo threshold
  - NUEVO TOPIC DETECTADO

Sistema: "Detecto que hablas de algo nuevo relacionado con cocina.
         ¿Es 'Fermentación' un nuevo interés tuyo?"

Usuario: "Sí, quiero hacer kimchi"

TopicGraph:
  - Crea RootTopic("Fermentación", parent="Cocina")
  - InterestWeight { explicit: 0.8, ... }
  - Genera embedding
  - Guarda template MTT-DSL en VoxelDB
  - Ahora detectará "fermentación" en futuros mensajes
```

---

## 🏗️ ARQUITECTURA

### Estructuras de Datos

#### 1. TopicGraph (Core Structure)

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};

pub struct TopicGraph {
    user_id: UserId,
    
    // Jerarquía de topics (3 niveles)
    root_topics: HashMap<TopicId, RootTopic>,      // Nivel 1: Grandes áreas
    sub_topics: HashMap<TopicId, SubTopic>,        // Nivel 2: Especialidades
    micro_topics: HashMap<TopicId, MicroTopic>,    // Nivel 3: Ultra-específico
    
    // Grafo de relaciones
    edges: Vec<TopicEdge>,
    adjacency_list: HashMap<TopicId, Vec<TopicId>>,
    
    // Pesos de interés
    weights: HashMap<TopicId, InterestWeight>,
    
    // Embeddings semánticos (MiniLM)
    embeddings: HashMap<TopicId, Vec<f32>>,
    
    // Templates MTT-DSL (paths en VoxelDB)
    template_paths: HashMap<TopicId, String>,
    
    // Reglas de aislamiento
    isolation_rules: HashMap<(TopicId, TopicId), IsolationMode>,
    
    // Metadata
    created_at: DateTime<Utc>,
    last_updated: DateTime<Utc>,
}

impl TopicGraph {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            root_topics: HashMap::new(),
            sub_topics: HashMap::new(),
            micro_topics: HashMap::new(),
            edges: Vec::new(),
            adjacency_list: HashMap::new(),
            weights: HashMap::new(),
            embeddings: HashMap::new(),
            template_paths: HashMap::new(),
            isolation_rules: HashMap::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }
    
    // Buscar topics similares
    pub fn find_similar(&self, text: &str, threshold: f32) -> Vec<(TopicId, f32)> {
        let embedder = MiniLMEmbedder::global();
        let query_embedding = embedder.encode(text);
        
        let mut similarities: Vec<(TopicId, f32)> = self.embeddings
            .iter()
            .map(|(topic_id, topic_embedding)| {
                let sim = cosine_similarity(&query_embedding, topic_embedding);
                (*topic_id, sim)
            })
            .filter(|(_, sim)| *sim >= threshold)
            .collect();
        
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities
    }
    
    // Verificar si dos topics deben estar aislados
    pub fn should_isolate(&self, topic_a: TopicId, topic_b: TopicId) -> bool {
        if let Some(mode) = self.isolation_rules.get(&(topic_a, topic_b))
            .or_else(|| self.isolation_rules.get(&(topic_b, topic_a))) 
        {
            matches!(mode, IsolationMode::Strict)
        } else {
            false // default: permitir mezcla con confirmación
        }
    }
}
```

#### 2. RootTopic (Grandes Áreas)

```rust
pub struct RootTopic {
    pub id: TopicId,
    pub name: String,  // "Cocina", "Cerámica", "Rust" - USER DEFINED ✅
    pub description: Option<String>,
    
    // Metadata
    pub user_defined: bool,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub interaction_count: u32,
    pub sentiment_average: f32,  // -1.0 (negativo) a 1.0 (positivo)
    
    // Jerarquía
    pub parent: Option<TopicId>,
    pub children: Vec<TopicId>,
    
    // Aislamiento
    pub isolation_mode: IsolationMode,
    
    // VoxelDB mapping
    pub voxel_coords: Option<CubicCoords>,
}

pub enum IsolationMode {
    Strict,      // NUNCA mezclar con otros topics (ej: Armas ⊥ Cocina)
    Soft,        // Permitir si usuario conecta explícitamente
    Exploratory, // Sugerir conexiones pero pedir confirmación
}
```

**Ejemplos:**

```rust
// Eduardo: Rust (máximo interés)
RootTopic {
    id: TopicId("topic_rust_001"),
    name: "Rust".to_string(),
    description: Some("Lenguaje de programación Rust".to_string()),
    user_defined: true,
    created_at: "2025-10-15T10:00:00Z",
    last_accessed: "2025-11-24T14:00:00Z",
    interaction_count: 1250,  // ¡Habla mucho de Rust!
    sentiment_average: 0.92,  // Siempre positivo
    parent: Some(TopicId("topic_programacion_001")),
    children: vec![
        TopicId("topic_async_rust_001"),
        TopicId("topic_macros_rust_001"),
    ],
    isolation_mode: IsolationMode::Exploratory,
    voxel_coords: Some(CubicCoords::new(0.85, 1.00, 0.95)),
}

// Eduardo: Espiritualidad (aislamiento estricto)
RootTopic {
    id: TopicId("topic_espiritualidad_001"),
    name: "Espiritualidad".to_string(),
    description: Some("Prácticas espirituales y reflexión".to_string()),
    user_defined: true,
    created_at: "2025-11-01T08:00:00Z",
    last_accessed: "2025-11-23T19:00:00Z",
    interaction_count: 85,
    sentiment_average: 0.75,
    parent: None,
    children: vec![TopicId("topic_meditacion_001")],
    isolation_mode: IsolationMode::Strict,  // NO mezclar con Tecnología
    voxel_coords: Some(CubicCoords::new(0.25, 0.70, 0.15)),
}
```

#### 3. SubTopic (Especialidades)

```rust
pub struct SubTopic {
    pub id: TopicId,
    pub name: String,  // "Técnicas de Torno", "Async Rust"
    pub parent: TopicId,
    pub keywords: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub interaction_count: u32,
}
```

#### 4. MicroTopic (Ultra-específico)

```rust
pub struct MicroTopic {
    pub id: TopicId,
    pub name: String,  // "Centrado de arcilla", "Tokio runtime"
    pub parent: TopicId,
    pub examples: Vec<String>,
    pub created_at: DateTime<Utc>,
}
```

#### 5. TopicEdge (Relaciones)

```rust
pub struct TopicEdge {
    pub from: TopicId,
    pub to: TopicId,
    pub connection_type: ConnectionType,
    pub strength: f32,  // 0.0-1.0
    pub user_approved: bool,
    pub created_at: DateTime<Utc>,
}

pub enum ConnectionType {
    Complementary,  // "Cerámica" + "Química" (esmaltes requieren química)
    Hierarchical,   // "Rust" ⊂ "Programación" ⊂ "Tecnología"
    Sequential,     // Aprender X antes de Y
    Contrasting,    // Comparar/contrastar dos topics
    Forbidden,      // Usuario dijo "NUNCA mezclar"
}
```

**Ejemplo:**

```rust
// Eduardo: Cerámica ↔ Química (complementary)
TopicEdge {
    from: TopicId("topic_ceramica_001"),
    to: TopicId("topic_quimica_001"),
    connection_type: ConnectionType::Complementary,
    strength: 0.85,
    user_approved: true,  // Eduardo confirmó
    created_at: "2025-11-10T16:00:00Z",
}

// Eduardo: Espiritualidad ⊥ Tecnología (forbidden)
TopicEdge {
    from: TopicId("topic_espiritualidad_001"),
    to: TopicId("topic_tecnologia_001"),
    connection_type: ConnectionType::Forbidden,
    strength: 0.0,
    user_approved: true,
    created_at: "2025-11-15T09:00:00Z",
}
```

#### 6. InterestWeight (Pesos Multi-Factor)

```rust
pub struct InterestWeight {
    pub explicit: f32,   // Usuario expresó interés explícito (0.0-1.0)
    pub implicit: f32,   // Frecuencia de menciones (0.0-1.0)
    pub temporal: f32,   // Decae con tiempo sin uso (0.0-1.0)
    pub emotional: f32,  // Sentimiento asociado (0.0-1.0)
}

impl InterestWeight {
    pub fn combined(&self) -> f32 {
        (self.explicit * 0.4 + 
         self.implicit * 0.3 + 
         self.temporal * 0.2 + 
         self.emotional * 0.1).clamp(0.0, 1.0)
    }
    
    // Decay temporal (llamar periódicamente)
    pub fn decay_temporal(&mut self, days_since_last: u32) {
        // Decaimiento exponencial: 50% cada 30 días
        let decay_rate = 0.5_f32.powf(days_since_last as f32 / 30.0);
        self.temporal *= decay_rate;
    }
    
    // Update implicit con nueva mención
    pub fn update_implicit(&mut self, total_mentions: u32) {
        // Normalizar: 100+ menciones = 1.0
        self.implicit = (total_mentions as f32 / 100.0).min(1.0);
    }
}
```

**Ejemplo cálculo:**

```rust
// Eduardo habla de Rust constantemente
let rust_weight = InterestWeight {
    explicit: 0.95,  // Dijo "Rust es mi lenguaje favorito"
    implicit: 1.00,  // 1250 menciones (>100)
    temporal: 1.00,  // Habló hoy
    emotional: 0.92, // Sentimiento muy positivo
};

println!("Rust combined weight: {}", rust_weight.combined());
// Output: 0.964 (máximo interés)

// Eduardo mencionó Microprocesadores hace 1 mes
let mut micro_weight = InterestWeight {
    explicit: 0.5,   // "Interesante pero no prioritario"
    implicit: 0.15,  // 15 menciones
    temporal: 0.6,   // Hace 1 mes
    emotional: 0.6,  // Neutral
};

micro_weight.decay_temporal(30); // 30 días sin mencionar
println!("Microprocesadores combined weight: {}", micro_weight.combined());
// Output: ~0.38 (interés moderado-bajo)
```

---

## 🧠 ALGORITMOS

### 1. Topic Detection (Semantic Matching)

```rust
impl DynamicTopicAnalyzer {
    pub fn detect_topics(&self, text: &str, user_id: &UserId) 
        -> TopicDetectionResult 
    {
        // 1. Generar embedding
        let query_embedding = self.embedder.encode(text);
        
        // 2. Obtener TopicGraph del usuario
        let graphs = self.user_graphs.read().unwrap();
        let graph = match graphs.get(user_id) {
            Some(g) => g,
            None => return TopicDetectionResult::new_user(), // Primera vez
        };
        
        // 3. Calcular similitudes con todos los topics
        let mut scores: Vec<(TopicId, f32)> = graph.embeddings
            .iter()
            .map(|(topic_id, topic_embedding)| {
                let sim = cosine_similarity(&query_embedding, topic_embedding);
                (*topic_id, sim)
            })
            .collect();
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // 4. Clasificar resultado
        if let Some((best_topic, best_score)) = scores.first() {
            if *best_score >= 0.75 {
                // MATCH FUERTE: topic detectado
                return TopicDetectionResult {
                    detected_topics: vec![*best_topic],
                    confidence: *best_score,
                    is_new_topic: false,
                    suggested_name: None,
                    requires_confirmation: false,
                };
            } else if *best_score >= 0.5 && *best_score < 0.75 {
                // MATCH DÉBIL: posible topic existente o variación
                return TopicDetectionResult {
                    detected_topics: vec![*best_topic],
                    confidence: *best_score,
                    is_new_topic: false,
                    suggested_name: None,
                    requires_confirmation: true, // Pedir confirmación
                };
            }
        }
        
        // 5. NUEVO TOPIC: score < 0.5
        TopicDetectionResult {
            detected_topics: vec![],
            confidence: 0.0,
            is_new_topic: true,
            suggested_name: self.suggest_topic_name(text),
            requires_confirmation: true,
        }
    }
    
    // Sugerir nombre para nuevo topic (extractive keywords)
    fn suggest_topic_name(&self, text: &str) -> Option<String> {
        // RAKE (Rapid Automatic Keyword Extraction)
        let keywords = self.extract_keywords(text);
        keywords.first().cloned()
    }
}
```

### 2. Topic Learning (Auto-Discovery)

```rust
impl DynamicTopicAnalyzer {
    pub async fn learn_new_topic(
        &self,
        user_id: &UserId,
        topic_name: String,
        example_text: &str,
        parent: Option<TopicId>,
        isolation_mode: IsolationMode,
    ) -> Result<TopicId> {
        let topic_id = TopicId::new();
        let embedding = self.embedder.encode(example_text);
        
        // 1. Crear RootTopic
        let topic = RootTopic {
            id: topic_id,
            name: topic_name.clone(),
            description: None,
            user_defined: true,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            interaction_count: 1,
            sentiment_average: 0.5, // neutral por ahora
            parent,
            children: vec![],
            isolation_mode,
            voxel_coords: None, // Se calcula después
        };
        
        // 2. InterestWeight inicial
        let weight = InterestWeight {
            explicit: 0.8,  // Usuario confirmó interés
            implicit: 0.1,  // Primera mención
            temporal: 1.0,  // Justo ahora
            emotional: 0.5, // Neutral por ahora
        };
        
        // 3. Agregar a TopicGraph
        let mut graphs = self.user_graphs.write().unwrap();
        let graph = graphs.entry(user_id.clone())
            .or_insert_with(|| TopicGraph::new(user_id.clone()));
        
        graph.root_topics.insert(topic_id, topic.clone());
        graph.embeddings.insert(topic_id, embedding);
        graph.weights.insert(topic_id, weight);
        
        // 4. VoxelDB mapping (CubicCoords)
        let coords = self.map_to_voxel(&topic, &weight);
        graph.root_topics.get_mut(&topic_id).unwrap().voxel_coords = Some(coords);
        
        // 5. Crear template MTT-DSL
        let template_path = format!("users/{}/topics/{}.yaml", user_id, topic_name);
        self.create_topic_template(user_id, &topic, &template_path).await?;
        graph.template_paths.insert(topic_id, template_path);
        
        // 6. Persistir en VoxelDB
        self.persist_topic_graph(user_id, graph).await?;
        
        Ok(topic_id)
    }
}
```

### 3. Topic Isolation Check

```rust
impl TopicGraph {
    pub fn check_isolation(
        &self,
        current_topics: &[TopicId],
        incoming_topic: TopicId,
    ) -> IsolationCheckResult {
        for existing_topic in current_topics {
            // Verificar si hay regla de aislamiento
            let key1 = (*existing_topic, incoming_topic);
            let key2 = (incoming_topic, *existing_topic);
            
            if let Some(mode) = self.isolation_rules.get(&key1)
                .or_else(|| self.isolation_rules.get(&key2)) 
            {
                match mode {
                    IsolationMode::Strict => {
                        return IsolationCheckResult::Forbidden {
                            topic_a: *existing_topic,
                            topic_b: incoming_topic,
                            reason: "User enforced strict isolation".to_string(),
                        };
                    },
                    IsolationMode::Soft => {
                        // Permitir pero registrar
                        return IsolationCheckResult::Allowed {
                            requires_confirmation: false,
                        };
                    },
                    IsolationMode::Exploratory => {
                        return IsolationCheckResult::Allowed {
                            requires_confirmation: true,
                        };
                    },
                }
            }
        }
        
        // No hay regla: permitir con confirmación (default)
        IsolationCheckResult::Allowed {
            requires_confirmation: true,
        }
    }
}

pub enum IsolationCheckResult {
    Allowed { requires_confirmation: bool },
    Forbidden { topic_a: TopicId, topic_b: TopicId, reason: String },
}
```

### 4. Interest Weight Updates

```rust
impl TopicGraph {
    pub fn update_weights(&mut self, topic_id: TopicId, sentiment: f32) {
        if let Some(weight) = self.weights.get_mut(&topic_id) {
            // Update implicit (frecuencia)
            if let Some(topic) = self.root_topics.get_mut(&topic_id) {
                topic.interaction_count += 1;
                topic.last_accessed = Utc::now();
                weight.update_implicit(topic.interaction_count);
                
                // Update emotional (promedio sentimiento)
                let old_avg = topic.sentiment_average;
                let n = topic.interaction_count as f32;
                topic.sentiment_average = (old_avg * (n - 1.0) + sentiment) / n;
                weight.emotional = (topic.sentiment_average + 1.0) / 2.0; // map [-1,1] → [0,1]
            }
            
            // Reset temporal (acaba de ser mencionado)
            weight.temporal = 1.0;
        }
    }
    
    // Decay periódico (ejecutar diariamente)
    pub fn decay_all_weights(&mut self) {
        let now = Utc::now();
        for (topic_id, weight) in &mut self.weights {
            if let Some(topic) = self.root_topics.get(topic_id) {
                let days_since = (now - topic.last_accessed).num_days() as u32;
                weight.decay_temporal(days_since);
            }
        }
    }
}
```

---

## 🗄️ PERSISTENCIA (VoxelDB)

### VoxelDB Mapping

```rust
impl DynamicTopicAnalyzer {
    fn map_to_voxel(&self, topic: &RootTopic, weight: &InterestWeight) -> CubicCoords {
        // Mapear topic a espacio cúbico [0,1]³
        
        // X: Domain hash (determinista por nombre)
        let x = self.hash_topic_name(&topic.name);
        
        // Y: Interest weight (0.0-1.0)
        let y = weight.combined();
        
        // Z: Temporal recency (0.0-1.0)
        let z = weight.temporal;
        
        CubicCoords::new(x, y, z)
    }
    
    fn hash_topic_name(&self, name: &str) -> f32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let hash = hasher.finish();
        
        // Normalizar a [0,1]
        (hash % 10000) as f32 / 10000.0
    }
    
    pub async fn persist_topic_graph(&self, user_id: &UserId, graph: &TopicGraph) 
        -> Result<()> 
    {
        // Serializar TopicGraph
        let json = serde_json::to_string(graph)?;
        
        // Guardar en VoxelDB
        let path = format!("users/{}/topic_graph.json", user_id);
        self.voxeldb.write(&path, json.as_bytes()).await?;
        
        Ok(())
    }
    
    pub async fn load_topic_graph(&self, user_id: &UserId) -> Result<TopicGraph> {
        let path = format!("users/{}/topic_graph.json", user_id);
        let data = self.voxeldb.read(&path).await?;
        
        let graph: TopicGraph = serde_json::from_slice(&data)?;
        Ok(graph)
    }
}
```

### VoxelDB Structure

```
VoxelDB/
  users/
    eduardo_001/
      topic_graph.json           ← Full TopicGraph serialized
      topics/
        cocina.yaml              ← MTT-DSL template
        ceramica.yaml
        rust.yaml
        filosofia.yaml
        espiritualidad.yaml
        ...
      embeddings/
        topic_001.bin            ← Vec<f32> embeddings (binary)
        topic_002.bin
        ...
        
    esposa_001/
      topic_graph.json
      topics/
        autos.yaml
        armas.yaml
        escritura.yaml
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
    fn test_topic_detection_match() {
        let analyzer = DynamicTopicAnalyzer::new();
        let user_id = UserId("eduardo_001".to_string());
        
        // Setup: Eduardo tiene topic "Rust"
        let rust_topic = analyzer.learn_new_topic(
            &user_id,
            "Rust".to_string(),
            "Me encanta programar en Rust",
            None,
            IsolationMode::Exploratory,
        ).await.unwrap();
        
        // Test: detectar Rust en nuevo mensaje
        let result = analyzer.detect_topics(
            "Cómo implementar async/await en Rust",
            &user_id,
        );
        
        assert!(!result.is_new_topic);
        assert_eq!(result.detected_topics[0], rust_topic);
        assert!(result.confidence > 0.75);
    }
    
    #[test]
    fn test_topic_detection_new() {
        let analyzer = DynamicTopicAnalyzer::new();
        let user_id = UserId("eduardo_001".to_string());
        
        // Test: nuevo topic nunca visto
        let result = analyzer.detect_topics(
            "Quiero aprender sobre fermentación de vegetales",
            &user_id,
        );
        
        assert!(result.is_new_topic);
        assert!(result.requires_confirmation);
        assert_eq!(result.suggested_name, Some("Fermentación".to_string()));
    }
    
    #[test]
    fn test_topic_isolation_strict() {
        let mut graph = TopicGraph::new(UserId("eduardo_001".to_string()));
        
        let espiritualidad = TopicId("topic_esp_001");
        let tecnologia = TopicId("topic_tech_001");
        
        // Eduardo establece aislamiento estricto
        graph.isolation_rules.insert(
            (espiritualidad, tecnologia),
            IsolationMode::Strict,
        );
        
        // Test: intentar mezclar
        let result = graph.check_isolation(&[espiritualidad], tecnologia);
        
        match result {
            IsolationCheckResult::Forbidden { topic_a, topic_b, .. } => {
                assert!(topic_a == espiritualidad || topic_b == espiritualidad);
            },
            _ => panic!("Should be forbidden!"),
        }
    }
    
    #[test]
    fn test_interest_weight_decay() {
        let mut weight = InterestWeight {
            explicit: 0.8,
            implicit: 0.6,
            temporal: 1.0,
            emotional: 0.7,
        };
        
        assert_eq!(weight.combined(), 0.74); // Initial
        
        // 30 días sin mención
        weight.decay_temporal(30);
        
        assert_eq!(weight.temporal, 0.5);
        assert!(weight.combined() < 0.74); // Decayó
    }
}
```

---

## 🎯 MÉTRICAS DE ÉXITO

| Métrica | Target | Verificación |
|---------|--------|--------------|
| **Detection Accuracy** | >85% | Topics detectados correctamente |
| **False Positives** | <10% | Nuevo topic mal clasificado como existente |
| **User Confirmation Rate** | <20% | Confirmaciones necesarias vs automáticas |
| **Response Time** | <30ms | Detección + lookup en TopicGraph |
| **Storage per Topic** | <5KB | Template YAML + embedding |
| **Scalability** | 1000+ topics/user | Performance con graphs grandes |
| **Isolation Violations** | 0% | Nunca mezclar topics Strict |

---

## 🔗 INTEGRACIÓN

### Con IntentionDetector

```rust
// src/shuidao/intention_detector.rs

impl IntentionDetector {
    pub fn detect(&self, message: &str, user_id: &UserId) -> DetectedIntention {
        let processed = self.preprocessor.process(message);
        
        // NUEVO: DynamicTopicAnalyzer
        let topic_analysis = self.topic_analyzer.detect_topics(&processed.normalized, user_id);
        
        // Si es nuevo topic: pausar y pedir confirmación
        if topic_analysis.is_new_topic {
            return DetectedIntention::new_topic_confirmation(
                topic_analysis.suggested_name.unwrap_or("Nuevo tema".to_string())
            );
        }
        
        // Si requiere confirmación de match débil
        if topic_analysis.requires_confirmation {
            return DetectedIntention::confirm_topic_match(
                topic_analysis.detected_topics[0],
                topic_analysis.confidence,
            );
        }
        
        // Normal flow con topics detectados
        let verb = self.verb_classifier.classify(&processed);
        let tone = self.tone_detector.detect(&processed, user_id);
        let context = self.conversation_history.analyze_context(&processed);
        
        self.intention_scorer.score_and_select(verb, topic_analysis, tone, context)
    }
}
```

### Con MTT-DSL Templates

```yaml
# users/eduardo_001/topics/ceramica.yaml
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
    
  embedding_similarity_threshold: 0.75
  
interest_weight:
  explicit: 0.85
  implicit: 0.70
  temporal: 0.90
  emotional: 0.80
  combined: 0.81
  
response_style:
  formality: 0.3
  detail_level: 0.8
  include_examples: true
  tone_adaptation: "practical_enthusiast"
```

---

## ⚡ OPTIMIZACIONES

### 1. Embedding Cache

```rust
pub struct EmbeddingCache {
    cache: Arc<RwLock<HashMap<String, Vec<f32>>>>,
    max_size: usize,
}

impl EmbeddingCache {
    pub fn get_or_compute(&self, text: &str, embedder: &MiniLMEmbedder) -> Vec<f32> {
        {
            let cache = self.cache.read().unwrap();
            if let Some(embedding) = cache.get(text) {
                return embedding.clone();
            }
        }
        
        let embedding = embedder.encode(text);
        
        let mut cache = self.cache.write().unwrap();
        if cache.len() < self.max_size {
            cache.insert(text.to_string(), embedding.clone());
        }
        
        embedding
    }
}
```

### 2. Incremental Similarity Search

```rust
impl TopicGraph {
    // En lugar de calcular similitud con TODOS los topics,
    // usar Octree espacial en VoxelDB
    pub fn find_similar_fast(&self, query_embedding: &[f32]) -> Vec<(TopicId, f32)> {
        // Mapear query a CubicCoords
        let query_coords = self.embedding_to_coords(query_embedding);
        
        // Buscar vecinos más cercanos en VoxelDB Octree
        let neighbors = self.voxeldb.octree().nearest_neighbors(query_coords, k=10);
        
        // Calcular similitud solo con esos 10 candidatos
        neighbors
            .iter()
            .map(|topic_id| {
                let topic_embedding = &self.embeddings[topic_id];
                let sim = cosine_similarity(query_embedding, topic_embedding);
                (*topic_id, sim)
            })
            .collect()
    }
}
```

---

## 🎓 CONCLUSIÓN

TopicGraph es la **diferencia fundamental** entre:

❌ **Asistente genérico** con 7 categorías hardcoded  
✅ **Compañero personal** que conoce TUS interests únicos

**Sin TopicGraph:**
- Usuario habla de "cerámica" → ¿Infrastructure? ¿NO FIT
- Sistema trata a Eduardo = Esposa = cualquier persona
- No aprende nuevos interests
- Mezcla topics involuntariamente

**Con TopicGraph:**
- Eduardo habla de "cerámica" → detecta topic "Cerámica" (0.85 weight)
- Respuestas adaptadas a interest_weight (Rust 1.0 > Microprocesadores 0.7)
- Aprende nuevos topics dinámicamente con confirmación
- "Juntos pero no revueltos": respeta boundaries cognitivos

**Esto es personalización real.**

---

**Estado:** 🔴 CRÍTICO - Implementación prioritaria  
**Owner:** B (Sistema Bitácora) + Eduardo  
**Next:** Implementación src/shuidao/topic_graph.rs (~800 líneas)
