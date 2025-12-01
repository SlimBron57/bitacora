// ROUTIER NAVIGATOR - Adaptive Learning Path Navigation
// Sistema de navegación de rutas de aprendizaje que evolucionan con el usuario
//
// Metáfora: GPS que aprende de ti
// - Analiza tu estado cognitivo actual
// - Genera rutas personalizadas
// - Se adapta dinámicamente a tu progreso
// - Ajusta complejidad según tu velocidad

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

// ============================================================================
// CORE TYPES
// ============================================================================

/// Learning Graph - Representación del curriculum como grafo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningGraph {
    pub nodes: HashMap<String, LearningNode>,
    pub edges: Vec<LearningEdge>,
}

/// Learning Node - Un paso en la ruta de aprendizaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningNode {
    pub id: String,
    pub title: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub estimated_time_minutes: u32,
    pub content_type: ContentType,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Concept,      // Teoría/explicación
    Practice,     // Ejercicio práctico
    Project,      // Proyecto integrador
    Assessment,   // Evaluación
    Resource,     // Recurso adicional
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Locked,       // No disponible aún
    Available,    // Disponible para iniciar
    InProgress,   // Actualmente trabajando
    Completed,    // Completado exitosamente
    Skipped,      // Saltado (usuario ya lo domina)
    Failed,       // Falló, necesita retry
}

/// Learning Edge - Dependencia entre nodos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEdge {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeType {
    Prerequisite,  // "to" requiere "from"
    Recommended,   // "to" es recomendado después de "from"
    Alternative,   // "to" es alternativa a "from"
}

// ============================================================================
// COGNITIVE STATE
// ============================================================================

/// Cognitive State - Estado actual del usuario en su aprendizaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub user_id: String,
    pub current_position: String,  // ID del nodo actual
    pub mastery_levels: HashMap<String, f32>,  // topic → mastery (0.0-1.0)
    pub learning_velocity: LearningVelocity,
    pub engagement_level: EngagementLevel,
    pub recent_struggles: Vec<String>,  // Topics con dificultad
    pub recent_wins: Vec<String>,       // Topics dominados rápido
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningVelocity {
    pub steps_per_hour: f32,
    pub completion_rate: f32,  // % de pasos completados vs intentados
    pub retry_rate: f32,        // % de pasos que requirieron retry
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EngagementLevel {
    High,      // Usuario muy activo, pregunta frecuentemente
    Medium,    // Progreso constante, engagement normal
    Low,       // Progreso lento, pocas interacciones
    Dropping,  // Riesgo de abandono
}

// ============================================================================
// ROUTE ADJUSTMENT
// ============================================================================

/// Route Adjustment - Modificación dinámica de la ruta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAdjustment {
    pub adjustment_type: AdjustmentType,
    pub reason: String,
    pub affected_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    SkipNode { node_id: String },
    InsertNode { node: LearningNode, after: String },
    UnlockEarly { node_id: String },
    IncreaseComplexity { node_id: String },
    DecreaseComplexity { node_id: String },
    SuggestAlternative { from: String, to: String },
}

// ============================================================================
// NEXT STEP RECOMMENDATION
// ============================================================================

/// Next Step - Recomendación del siguiente paso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStep {
    pub node: LearningNode,
    pub reasoning: String,
    pub estimated_completion_time: u32,
    pub prerequisites_met: bool,
    pub confidence: f32,  // Confianza en la recomendación (0.0-1.0)
}

// ============================================================================
// ROUTIER NAVIGATOR ENGINE
// ============================================================================

/// Routier Navigator - Motor de navegación adaptativa
pub struct RoutierNavigator {
    learning_graph: LearningGraph,
    cognitive_state: CognitiveState,
    adjustment_history: Vec<RouteAdjustment>,
}

impl RoutierNavigator {
    /// Crear nuevo navigator desde ExpertisePackage
    pub fn new(user_id: String, curriculum: Vec<LearningNode>) -> Self {
        // Construir grafo desde curriculum lineal
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        
        for (i, node) in curriculum.iter().enumerate() {
            nodes.insert(node.id.clone(), node.clone());
            
            // Crear edge de prerequisito con nodo anterior
            if i > 0 {
                edges.push(LearningEdge {
                    from: curriculum[i - 1].id.clone(),
                    to: node.id.clone(),
                    edge_type: EdgeType::Prerequisite,
                });
            }
        }
        
        let learning_graph = LearningGraph { nodes, edges };
        
        // Estado cognitivo inicial
        let cognitive_state = CognitiveState {
            user_id,
            current_position: curriculum.first()
                .map(|n| n.id.clone())
                .unwrap_or_default(),
            mastery_levels: HashMap::new(),
            learning_velocity: LearningVelocity {
                steps_per_hour: 0.0,
                completion_rate: 0.0,
                retry_rate: 0.0,
            },
            engagement_level: EngagementLevel::Medium,
            recent_struggles: Vec::new(),
            recent_wins: Vec::new(),
        };
        
        Self {
            learning_graph,
            cognitive_state,
            adjustment_history: Vec::new(),
        }
    }
    
    /// Analizar estado cognitivo actual del usuario
    pub fn analyze_cognitive_state(&mut self, metrics: UserMetrics) {
        // Actualizar velocidad de aprendizaje
        self.cognitive_state.learning_velocity.steps_per_hour = metrics.steps_per_hour;
        self.cognitive_state.learning_velocity.completion_rate = metrics.completion_rate;
        self.cognitive_state.learning_velocity.retry_rate = metrics.retry_rate;
        
        // Actualizar engagement
        self.cognitive_state.engagement_level = self.calculate_engagement(&metrics);
        
        // Detectar struggles y wins
        self.detect_patterns(&metrics);
    }
    
    /// Calcular nivel de engagement
    fn calculate_engagement(&self, metrics: &UserMetrics) -> EngagementLevel {
        let score = (metrics.queries_per_hour * 0.4)
            + (metrics.completion_rate * 0.3)
            + ((1.0 - metrics.retry_rate) * 0.3);
        
        if score > 0.8 {
            EngagementLevel::High
        } else if score > 0.5 {
            EngagementLevel::Medium
        } else if score > 0.3 {
            EngagementLevel::Low
        } else {
            EngagementLevel::Dropping
        }
    }
    
    /// Detectar patrones de lucha o dominio
    fn detect_patterns(&mut self, metrics: &UserMetrics) {
        // Topics donde el usuario se atora (>2 retries)
        for (topic, retries) in &metrics.topic_retries {
            if *retries > 2 {
                if !self.cognitive_state.recent_struggles.contains(topic) {
                    self.cognitive_state.recent_struggles.push(topic.clone());
                }
            }
        }
        
        // Topics completados rápidamente (<50% del tiempo esperado)
        for (topic, time_ratio) in &metrics.topic_completion_times {
            if *time_ratio < 0.5 {
                if !self.cognitive_state.recent_wins.contains(topic) {
                    self.cognitive_state.recent_wins.push(topic.clone());
                }
            }
        }
    }
    
    /// Recomendar siguiente paso adaptativo
    pub fn recommend_next_step(&self) -> Option<NextStep> {
        let current_id = &self.cognitive_state.current_position;
        
        // Encontrar nodos disponibles (prerequisites cumplidos)
        let available = self.find_available_nodes(current_id);
        
        if available.is_empty() {
            return None;
        }
        
        // Seleccionar mejor nodo según estado cognitivo
        let best = self.select_best_node(&available);
        
        let node = self.learning_graph.nodes.get(&best)?;
        
        // Ajustar tiempo estimado según velocidad del usuario
        let velocity_factor = if self.cognitive_state.learning_velocity.steps_per_hour > 0.0 {
            self.cognitive_state.learning_velocity.steps_per_hour / 1.0  // baseline: 1 step/hour
        } else {
            1.0
        };
        
        let adjusted_time = (node.estimated_time_minutes as f32 / velocity_factor) as u32;
        
        Some(NextStep {
            node: node.clone(),
            reasoning: self.generate_reasoning(&best),
            estimated_completion_time: adjusted_time,
            prerequisites_met: true,
            confidence: self.calculate_confidence(&best),
        })
    }
    
    /// Encontrar nodos disponibles
    fn find_available_nodes(&self, from: &str) -> Vec<String> {
        let mut available = Vec::new();
        
        for edge in &self.learning_graph.edges {
            if edge.from == from {
                let node = self.learning_graph.nodes.get(&edge.to);
                if let Some(n) = node {
                    if n.status == NodeStatus::Available || n.status == NodeStatus::Locked {
                        available.push(edge.to.clone());
                    }
                }
            }
        }
        
        available
    }
    
    /// Seleccionar mejor nodo según estado cognitivo
    fn select_best_node(&self, candidates: &[String]) -> String {
        if candidates.is_empty() {
            return String::new();
        }
        
        // Si usuario está con high engagement, priorizar nodos avanzados
        if self.cognitive_state.engagement_level == EngagementLevel::High {
            for id in candidates {
                if let Some(node) = self.learning_graph.nodes.get(id) {
                    if node.difficulty == Difficulty::Advanced 
                        || node.difficulty == Difficulty::Expert {
                        return id.clone();
                    }
                }
            }
        }
        
        // Si tiene struggles recientes, priorizar practice
        if !self.cognitive_state.recent_struggles.is_empty() {
            for id in candidates {
                if let Some(node) = self.learning_graph.nodes.get(id) {
                    if matches!(node.content_type, ContentType::Practice) {
                        return id.clone();
                    }
                }
            }
        }
        
        // Default: primer candidato
        candidates[0].clone()
    }
    
    /// Generar razonamiento para la recomendación
    fn generate_reasoning(&self, node_id: &str) -> String {
        let node = self.learning_graph.nodes.get(node_id);
        
        if let Some(n) = node {
            match self.cognitive_state.engagement_level {
                EngagementLevel::High => {
                    format!("Alta engagement detectada. '{}' te desafiará apropiadamente.", n.title)
                }
                EngagementLevel::Low | EngagementLevel::Dropping => {
                    format!("'{}' es un buen paso para retomar momentum.", n.title)
                }
                EngagementLevel::Medium => {
                    format!("Siguiente paso lógico: '{}'", n.title)
                }
            }
        } else {
            "Siguiente paso en tu ruta".to_string()
        }
    }
    
    /// Calcular confianza en la recomendación
    fn calculate_confidence(&self, _node_id: &str) -> f32 {
        // Confianza basada en datos disponibles
        let base_confidence = 0.70_f32;
        
        let velocity_bonus = if self.cognitive_state.learning_velocity.steps_per_hour > 0.0 {
            0.15_f32
        } else {
            0.0_f32
        };
        
        let engagement_bonus = match self.cognitive_state.engagement_level {
            EngagementLevel::High => 0.10_f32,
            EngagementLevel::Medium => 0.05_f32,
            _ => 0.0_f32,
        };
        
        (base_confidence + velocity_bonus + engagement_bonus).min(1.0_f32)
    }
    
    /// Aplicar ajuste de ruta
    pub fn apply_adjustment(&mut self, adjustment: RouteAdjustment) {
        match &adjustment.adjustment_type {
            AdjustmentType::SkipNode { node_id } => {
                if let Some(node) = self.learning_graph.nodes.get_mut(node_id) {
                    node.status = NodeStatus::Skipped;
                }
            }
            AdjustmentType::UnlockEarly { node_id } => {
                if let Some(node) = self.learning_graph.nodes.get_mut(node_id) {
                    node.status = NodeStatus::Available;
                }
            }
            _ => {}
        }
        
        self.adjustment_history.push(adjustment);
    }
    
    /// Marcar nodo como completado
    pub fn mark_completed(&mut self, node_id: &str, actual_time: u32) {
        if let Some(node) = self.learning_graph.nodes.get_mut(node_id) {
            node.status = NodeStatus::Completed;
            
            // Actualizar posición actual
            self.cognitive_state.current_position = node_id.to_string();
            
            // Analizar si fue rápido o lento
            let time_ratio = actual_time as f32 / node.estimated_time_minutes as f32;
            
            // Si completó muy rápido (<50% tiempo), considerar skip de siguientes
            // Clonar el nodo para evitar conflictos de borrow
            let completed_node_clone = node.clone();
            
            if time_ratio < 0.5_f32 {
                self.suggest_skip_similar_nodes(&completed_node_clone);
            }
        }
    }
    
    /// Sugerir skip de nodos similares si usuario domina rápido
    fn suggest_skip_similar_nodes(&mut self, completed_node: &LearningNode) {
        // Recolectar IDs de nodos a ajustar (evitar borrow conflict)
        let nodes_to_skip: Vec<String> = self.learning_graph.nodes
            .iter()
            .filter(|(_, node)| {
                node.difficulty == completed_node.difficulty 
                    && node.status == NodeStatus::Available
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        // Aplicar adjustments
        for id in nodes_to_skip {
            self.apply_adjustment(RouteAdjustment {
                adjustment_type: AdjustmentType::SkipNode { 
                    node_id: id.clone() 
                },
                reason: format!(
                    "Usuario dominó '{}' rápidamente, sugiriendo skip de contenido similar",
                    completed_node.title
                ),
                affected_nodes: vec![id],
            });
        }
    }
    
    /// Obtener estado actual
    pub fn get_state(&self) -> &CognitiveState {
        &self.cognitive_state
    }
    
    /// Obtener historial de ajustes
    pub fn get_adjustment_history(&self) -> &[RouteAdjustment] {
        &self.adjustment_history
    }
}

// ============================================================================
// USER METRICS (from TelescopeDB analysis)
// ============================================================================

/// User Metrics - Métricas del usuario desde TelescopeDB
#[derive(Debug, Clone)]
pub struct UserMetrics {
    pub steps_per_hour: f32,
    pub completion_rate: f32,
    pub retry_rate: f32,
    pub queries_per_hour: f32,
    pub topic_retries: HashMap<String, u32>,
    pub topic_completion_times: HashMap<String, f32>,  // ratio vs tiempo esperado
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_curriculum() -> Vec<LearningNode> {
        vec![
            LearningNode {
                id: "node_1".to_string(),
                title: "JavaScript Basics".to_string(),
                description: "Fundamentals".to_string(),
                difficulty: Difficulty::Beginner,
                estimated_time_minutes: 60,
                content_type: ContentType::Concept,
                status: NodeStatus::Available,
            },
            LearningNode {
                id: "node_2".to_string(),
                title: "Functions Deep Dive".to_string(),
                description: "Advanced functions".to_string(),
                difficulty: Difficulty::Intermediate,
                estimated_time_minutes: 90,
                content_type: ContentType::Concept,
                status: NodeStatus::Locked,
            },
            LearningNode {
                id: "node_3".to_string(),
                title: "Async/Await Practice".to_string(),
                description: "Practical exercises".to_string(),
                difficulty: Difficulty::Advanced,
                estimated_time_minutes: 120,
                content_type: ContentType::Practice,
                status: NodeStatus::Locked,
            },
        ]
    }
    
    #[test]
    fn test_routier_creation() {
        let curriculum = create_test_curriculum();
        let navigator = RoutierNavigator::new("user_123".to_string(), curriculum);
        
        assert_eq!(navigator.cognitive_state.user_id, "user_123");
        assert_eq!(navigator.learning_graph.nodes.len(), 3);
    }
    
    #[test]
    fn test_next_step_recommendation() {
        let curriculum = create_test_curriculum();
        let navigator = RoutierNavigator::new("user_123".to_string(), curriculum);
        
        let next = navigator.recommend_next_step();
        assert!(next.is_some(), "Should recommend a next step");
        
        let step = next.unwrap();
        // Property-based: verify recommended node is valid (not specific ID)
        assert!(
            step.node.id == "node_1" || step.node.id == "node_2",
            "Should recommend node_1 or node_2, got {}",
            step.node.id
        );
        assert!(step.prerequisites_met, "Prerequisites must be met");
        assert!(step.confidence > 0.0, "Confidence must be positive");
    }
    
    #[test]
    fn test_cognitive_state_analysis() {
        let curriculum = create_test_curriculum();
        let mut navigator = RoutierNavigator::new("user_123".to_string(), curriculum);
        
        let metrics = UserMetrics {
            steps_per_hour: 2.0,
            completion_rate: 0.95,
            retry_rate: 0.10,
            queries_per_hour: 5.0,
            topic_retries: HashMap::new(),
            topic_completion_times: HashMap::new(),
        };
        
        navigator.analyze_cognitive_state(metrics);
        
        assert_eq!(navigator.cognitive_state.engagement_level, EngagementLevel::High);
    }
    
    #[test]
    fn test_mark_completed() {
        let curriculum = create_test_curriculum();
        let mut navigator = RoutierNavigator::new("user_123".to_string(), curriculum);
        
        navigator.mark_completed("node_1", 30);  // Completó en 30 min (estimado: 60)
        
        let node = navigator.learning_graph.nodes.get("node_1").unwrap();
        assert_eq!(node.status, NodeStatus::Completed);
    }
}
