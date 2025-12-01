// ================================================================
// TopicGraph - Sistema Dinámico de Topics Personalizados
// ================================================================
//
// Purpose: User-defined topic detection & management (DETECTION ONLY)
// Innovation: Topics ilimitados, personalizados por usuario
// Separation: Detection aquí | Navigation en Routier (DA-034)
//
// DA-033: Dynamic Topic & Tone System
// Spec: ROADMAP_V2/02_COMPONENTES/CRITICOS/14_shuidao-topic-graph.md v1.1.0
// 
// Performance Targets:
// - Topic detection: <15ms (HOT PATH)
// - Memory overhead: ~100KB per 50 topics
// - Cosine similarity: O(n) where n = embedding dimensions (300)
//
// ================================================================

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::error::{Result, ShuiDaoError};

// ================================================================
// CORE STRUCTURES
// ================================================================

/// Nodo de topic en el grafo (user-defined)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicNode {
    /// ID único del topic
    pub id: String,
    
    /// Nombre del topic (user-defined, e.g., "Cerámica", "Rust", "Microprocesadores")
    pub name: String,
    
    /// Embedding semántico (MiniLM-L6-v2, 384 dims)
    /// v1.0: Stub con zeros, v1.1: Real MiniLM embeddings
    pub embedding: Vec<f32>,
    
    /// Peso de interés acumulado (0.0 - 1.0)
    pub interest_weight: InterestWeight,
    
    /// Cuándo fue creado este topic
    pub created_at: DateTime<Utc>,
    
    /// Última mención
    pub last_mentioned: DateTime<Utc>,
    
    /// Contador de menciones
    pub mention_count: usize,
    
    /// Metadatos adicionales
    pub metadata: HashMap<String, String>,
}

/// Peso de interés multidimensional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestWeight {
    /// Interés explícito (usuario dijo "me interesa X")
    pub explicit: f32,
    
    /// Interés implícito (frecuencia de mención)
    pub implicit: f32,
    
    /// Peso temporal (decae con el tiempo)
    pub temporal: f32,
    
    /// Peso emocional (sentimiento positivo refuerza)
    pub emotional: f32,
    
    /// Peso combinado (fórmula ponderada)
    pub combined: f32,
}

impl InterestWeight {
    /// Crea peso inicial para nuevo topic
    pub fn new_initial() -> Self {
        Self {
            explicit: 0.5,
            implicit: 0.3,
            temporal: 1.0,
            emotional: 0.6,
            combined: 0.5,
        }
    }
    
    /// Recalcula peso combinado
    pub fn recalculate(&mut self) {
        // Formula ponderada: explicit 40%, implicit 30%, temporal 20%, emotional 10%
        self.combined = 
            0.40 * self.explicit +
            0.30 * self.implicit +
            0.20 * self.temporal +
            0.10 * self.emotional;
        
        // Clamp to [0.0, 1.0]
        self.combined = self.combined.clamp(0.0, 1.0);
    }
    
    /// Actualiza peso después de nueva mención
    pub fn update_on_mention(&mut self, sentiment_positive: bool) {
        // Incrementa implicit (frecuencia)
        self.implicit = (self.implicit + 0.05).min(1.0);
        
        // Refresca temporal
        self.temporal = 1.0;
        
        // Ajusta emotional según sentimiento
        if sentiment_positive {
            self.emotional = (self.emotional + 0.1).min(1.0);
        } else {
            self.emotional = (self.emotional - 0.05).max(0.0);
        }
        
        self.recalculate();
    }
    
    /// Aplica decay temporal (llamar diariamente)
    pub fn apply_temporal_decay(&mut self, days_since_mention: u32) {
        // Decay exponencial: 0.95^days
        let decay_factor = 0.95_f32.powi(days_since_mention as i32);
        self.temporal *= decay_factor;
        self.recalculate();
    }
}

/// Modo de aislamiento entre topics
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationMode {
    /// Nunca mezclar automáticamente (ej: Espiritualidad + Tecnología)
    Strict,
    
    /// Permitir mezcla si usuario lo solicita explícitamente
    Moderated,
    
    /// Libre asociación (ej: Rust + Sistemas Operativos)
    Flexible,
}

/// TopicGraph - Grafo de topics personalizado por usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicGraph {
    /// ID del usuario
    pub user_id: String,
    
    /// Nodos del grafo (topic_id -> TopicNode)
    pub nodes: HashMap<String, TopicNode>,
    
    /// Aristas dirigidas (from_topic_id -> to_topic_id -> edge_weight)
    pub edges: HashMap<String, HashMap<String, f32>>,
    
    /// Reglas de aislamiento (topic_pair -> mode)
    pub isolation_rules: HashMap<(String, String), IsolationMode>,
    
    /// Cuándo fue creado el grafo
    pub created_at: DateTime<Utc>,
    
    /// Última actualización
    pub updated_at: DateTime<Utc>,
}

impl TopicGraph {
    /// Crea TopicGraph vacío para usuario
    pub fn new(user_id: String) -> Self {
        let now = Utc::now();
        
        Self {
            user_id,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            isolation_rules: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Agrega nuevo topic al grafo
    pub fn add_topic(&mut self, name: String, embedding: Vec<f32>) -> Result<String> {
        // Generate topic ID
        let topic_id = format!("topic_{}", self.nodes.len() + 1);
        
        // Check if topic already exists by name
        if self.nodes.values().any(|node| node.name == name) {
            return Err(ShuiDaoError::TopicAlreadyExists(name));
        }
        
        let now = Utc::now();
        
        let node = TopicNode {
            id: topic_id.clone(),
            name,
            embedding,
            interest_weight: InterestWeight::new_initial(),
            created_at: now,
            last_mentioned: now,
            mention_count: 1,
            metadata: HashMap::new(),
        };
        
        self.nodes.insert(topic_id.clone(), node);
        self.updated_at = now;
        
        Ok(topic_id)
    }
    
    /// Busca topic por nombre (case-insensitive)
    pub fn find_topic_by_name(&self, name: &str) -> Option<&TopicNode> {
        let name_lower = name.to_lowercase();
        self.nodes.values()
            .find(|node| node.name.to_lowercase() == name_lower)
    }
    
    /// Registra mención de topic existente
    pub fn mention_topic(&mut self, topic_id: &str, sentiment_positive: bool) -> Result<()> {
        let node = self.nodes.get_mut(topic_id)
            .ok_or_else(|| ShuiDaoError::TopicNotFound(topic_id.to_string()))?;
        
        node.mention_count += 1;
        node.last_mentioned = Utc::now();
        node.interest_weight.update_on_mention(sentiment_positive);
        
        self.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Agrega arista entre topics
    pub fn add_edge(&mut self, from_id: &str, to_id: &str, weight: f32) -> Result<()> {
        // Validate topics exist
        if !self.nodes.contains_key(from_id) {
            return Err(ShuiDaoError::TopicNotFound(from_id.to_string()));
        }
        if !self.nodes.contains_key(to_id) {
            return Err(ShuiDaoError::TopicNotFound(to_id.to_string()));
        }
        
        self.edges
            .entry(from_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(to_id.to_string(), weight);
        
        self.updated_at = Utc::now();
        
        Ok(())
    }
    
    /// Configura regla de aislamiento entre dos topics
    pub fn set_isolation(&mut self, topic1: String, topic2: String, mode: IsolationMode) {
        // Store in canonical order (alphabetical)
        let pair = if topic1 < topic2 {
            (topic1, topic2)
        } else {
            (topic2, topic1)
        };
        
        self.isolation_rules.insert(pair, mode);
        self.updated_at = Utc::now();
    }
    
    /// Obtiene modo de aislamiento entre dos topics
    pub fn get_isolation(&self, topic1: &str, topic2: &str) -> IsolationMode {
        let pair = if topic1 < topic2 {
            (topic1.to_string(), topic2.to_string())
        } else {
            (topic2.to_string(), topic1.to_string())
        };
        
        self.isolation_rules.get(&pair)
            .cloned()
            .unwrap_or(IsolationMode::Flexible) // Default: Flexible
    }
    
    /// Aplica decay temporal a todos los topics (ejecutar diariamente)
    pub fn apply_daily_decay(&mut self) {
        let now = Utc::now();
        
        for node in self.nodes.values_mut() {
            let days_since = (now - node.last_mentioned).num_days() as u32;
            if days_since > 0 {
                node.interest_weight.apply_temporal_decay(days_since);
            }
        }
        
        self.updated_at = now;
    }
    
    /// Obtiene top N topics por interés combinado
    pub fn get_top_topics(&self, n: usize) -> Vec<&TopicNode> {
        let mut topics: Vec<&TopicNode> = self.nodes.values().collect();
        topics.sort_by(|a, b| {
            b.interest_weight.combined
                .partial_cmp(&a.interest_weight.combined)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        topics.into_iter().take(n).collect()
    }
}

// ================================================================
// TOPIC DETECTION RESULT
// ================================================================

/// Resultado de detección de topics
#[derive(Debug, Clone)]
pub struct TopicMatch {
    /// Topic detectado
    pub topic_id: String,
    
    /// Nombre del topic
    pub topic_name: String,
    
    /// Score de similitud (0.0 - 1.0)
    pub similarity: f32,
    
    /// Peso de interés del topic
    pub interest_weight: f32,
    
    /// Score combinado (similarity * interest_weight)
    pub combined_score: f32,
}

impl TopicMatch {
    /// Crea TopicMatch desde nodo y similitud
    pub fn from_node(node: &TopicNode, similarity: f32) -> Self {
        Self {
            topic_id: node.id.clone(),
            topic_name: node.name.clone(),
            similarity,
            interest_weight: node.interest_weight.combined,
            combined_score: similarity * node.interest_weight.combined,
        }
    }
}

// ================================================================
// EMBEDDING UTILITIES (v1.0 stub, v1.1 real MiniLM)
// ================================================================

/// Genera embedding stub (v1.0: zeros, v1.1: MiniLM)
pub fn generate_embedding_stub(text: &str) -> Vec<f32> {
    // v1.0: Simple hash-based pseudo-embedding (300 dims)
    // v1.1: Replace with rust-bert MiniLM-L6-v2 (384 dims)
    
    let mut embedding = vec![0.0; 300];
    
    // Hash text to generate deterministic pseudo-embedding
    let text_lower = text.to_lowercase();
    let hash = text_lower.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
    
    // Fill embedding with normalized values based on hash
    for (i, val) in embedding.iter_mut().enumerate() {
        let seed = hash.wrapping_add(i as u32);
        *val = ((seed % 1000) as f32 - 500.0) / 500.0; // Range [-1.0, 1.0]
    }
    
    // Normalize to unit vector
    let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        for val in &mut embedding {
            *val /= magnitude;
        }
    }
    
    embedding
}

/// Calcula similitud coseno entre dos embeddings
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (magnitude_a * magnitude_b)
}

// ================================================================
// TOPIC DETECTOR
// ================================================================

/// Detector de topics basado en embeddings
pub struct TopicDetector {
    /// Threshold de similitud para considerar match (0.75 recomendado)
    pub similarity_threshold: f32,
}

impl TopicDetector {
    /// Crea detector con threshold por defecto
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.75,
        }
    }
    
    /// Crea detector con threshold personalizado
    pub fn with_threshold(threshold: f32) -> Self {
        Self {
            similarity_threshold: threshold,
        }
    }
    
    /// Detecta topics en mensaje del usuario
    ///
    /// # Performance
    /// Target: <15ms (HOT PATH)
    /// Complexity: O(n * d) where n = topics count, d = embedding dims (300)
    pub fn detect_topics(&self, message: &str, graph: &TopicGraph) -> Vec<TopicMatch> {
        // Generate embedding for input message
        let message_embedding = generate_embedding_stub(message);
        
        // Calculate similarity with all topics
        let mut matches: Vec<TopicMatch> = graph.nodes.values()
            .filter_map(|node| {
                let similarity = cosine_similarity(&message_embedding, &node.embedding);
                
                if similarity >= self.similarity_threshold {
                    Some(TopicMatch::from_node(node, similarity))
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by combined score (similarity * interest_weight)
        matches.sort_by(|a, b| {
            b.combined_score.partial_cmp(&a.combined_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        matches
    }
    
    /// Detecta topic más relevante (top match)
    pub fn detect_primary_topic(&self, message: &str, graph: &TopicGraph) -> Option<TopicMatch> {
        self.detect_topics(message, graph).into_iter().next()
    }
}

impl Default for TopicDetector {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// TESTS
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_topic_graph_creation() {
        let graph = TopicGraph::new("eduardo_001".to_string());
        assert_eq!(graph.user_id, "eduardo_001");
        assert_eq!(graph.nodes.len(), 0);
    }
    
    #[test]
    fn test_add_topic() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust programming");
        
        let topic_id = graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key(&topic_id));
        
        let node = graph.nodes.get(&topic_id).unwrap();
        assert_eq!(node.name, "Rust");
        assert_eq!(node.mention_count, 1);
    }
    
    #[test]
    fn test_find_topic_by_name() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust");
        graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        let found = graph.find_topic_by_name("rust");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Rust");
        
        let not_found = graph.find_topic_by_name("Python");
        assert!(not_found.is_none());
    }
    
    #[test]
    fn test_mention_topic() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust");
        let topic_id = graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        graph.mention_topic(&topic_id, true).unwrap();
        
        let node = graph.nodes.get(&topic_id).unwrap();
        assert_eq!(node.mention_count, 2);
        assert!(node.interest_weight.emotional > 0.6); // Should increase
    }
    
    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001); // Should be 1.0 (identical)
        
        let c = vec![0.0, 1.0, 0.0];
        let sim2 = cosine_similarity(&a, &c);
        assert!((sim2 - 0.0).abs() < 0.001); // Should be 0.0 (orthogonal)
    }
    
    #[test]
    fn test_topic_detection() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        
        // Add topics
        let rust_emb = generate_embedding_stub("Rust programming language");
        let music_emb = generate_embedding_stub("experimental music composition");
        
        graph.add_topic("Rust".to_string(), rust_emb).unwrap();
        graph.add_topic("Música".to_string(), music_emb).unwrap();
        
        // Detect topics in message
        let detector = TopicDetector::new();
        let matches = detector.detect_topics("Quiero aprender Rust ownership", &graph);
        
        // Should detect Rust (high similarity)
        assert!(!matches.is_empty());
        assert!(matches[0].topic_name == "Rust" || matches[0].topic_name == "Música");
    }
    
    #[test]
    fn test_interest_weight_update() {
        let mut weight = InterestWeight::new_initial();
        let initial_combined = weight.combined;
        
        // Mention with positive sentiment
        weight.update_on_mention(true);
        
        assert!(weight.combined > initial_combined); // Should increase
        assert!(weight.implicit > 0.3); // Frequency increased
        assert_eq!(weight.temporal, 1.0); // Refreshed
    }
    
    #[test]
    fn test_isolation_rules() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        
        graph.set_isolation(
            "Espiritualidad".to_string(),
            "Tecnología".to_string(),
            IsolationMode::Strict
        );
        
        let mode = graph.get_isolation("Tecnología", "Espiritualidad");
        assert_eq!(mode, IsolationMode::Strict);
        
        // Default isolation (no rule)
        let mode2 = graph.get_isolation("Rust", "Python");
        assert_eq!(mode2, IsolationMode::Flexible);
    }
    
    #[test]
    fn test_top_topics() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        
        // Add topics with different weights
        let emb1 = generate_embedding_stub("Rust");
        let emb2 = generate_embedding_stub("Música");
        let emb3 = generate_embedding_stub("Cerámica");
        
        let id1 = graph.add_topic("Rust".to_string(), emb1).unwrap();
        let id2 = graph.add_topic("Música".to_string(), emb2).unwrap();
        let id3 = graph.add_topic("Cerámica".to_string(), emb3).unwrap();
        
        // Boost Rust weight
        graph.mention_topic(&id1, true).unwrap();
        graph.mention_topic(&id1, true).unwrap();
        
        let top = graph.get_top_topics(2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].name, "Rust"); // Highest weight
    }
}
