//! Motor de Decisiones AI - Versión Básica
//!
//! Implementa el sistema de toma de decisiones AI para el navegador híbrido.
//! Por ahora es un motor básico sin ML real.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{
    modes::{NavigationStrategy, AIConfig},
    errors::{NavigatorError, Result},
    NavigationContext,
};

/// Motor de decisiones AI principal
#[derive(Debug)]
pub struct AIDecisionEngine {
    /// Configuración del motor AI
    config: AIConfig,
    /// Estadísticas de decisiones anteriores
    decision_history: DecisionHistory,
    /// Modelo de decisión actual
    decision_model: DecisionModel,
}

/// Historial de decisiones para aprendizaje
#[derive(Debug, Clone)]
pub struct DecisionHistory {
    /// Total de decisiones tomadas
    pub total_decisions: u64,
    /// Decisiones por estrategia
    pub strategy_usage: HashMap<String, u64>,
    /// Promedio de confianza
    pub average_confidence: f32,
    /// Decisiones exitosas vs fallidas
    pub success_rate: f32,
}

/// Modelo de decisión básico
#[derive(Debug)]
pub enum DecisionModel {
    /// Modelo básico basado en reglas simples
    RuleBased {
        rules: Vec<DecisionRule>,
    },
    /// Modelo basado en pesos
    WeightBased {
        strategy_weights: HashMap<String, f32>,
    },
    /// Placeholder para ML futuro
    MachineLearning {
        model_path: String,
        parameters: HashMap<String, f32>,
    },
}

/// Regla de decisión simple
#[derive(Debug, Clone)]
pub struct DecisionRule {
    /// Condición que debe cumplirse
    pub condition: RuleCondition,
    /// Estrategia a usar si se cumple
    pub strategy: String,
    /// Confianza de esta regla
    pub confidence: f32,
}

/// Condiciones para reglas de decisión
#[derive(Debug, Clone)]
pub enum RuleCondition {
    /// Query contiene palabras específicas
    QueryContains(Vec<String>),
    /// Contexto tiene project_id
    HasProjectContext,
    /// Contexto tiene topic_id
    HasTopicContext,
    /// Query es corta (menos de N caracteres)
    QueryIsShort(usize),
    /// Query es larga (más de N caracteres)
    QueryIsLong(usize),
    /// Siempre verdadero (regla por defecto)
    Always,
}

/// Resultado de una decisión AI
#[derive(Debug, Clone)]
pub struct AIDecision {
    /// Estrategia recomendada
    pub recommended_strategy: NavigationStrategy,
    /// Nivel de confianza (0.0 - 1.0)
    pub confidence: f32,
    /// Razón de la decisión
    pub reasoning: String,
    /// Estrategias alternativas
    pub alternatives: Vec<(NavigationStrategy, f32)>,
}

impl Default for DecisionHistory {
    fn default() -> Self {
        Self {
            total_decisions: 0,
            strategy_usage: HashMap::new(),
            average_confidence: 0.0,
            success_rate: 0.0,
        }
    }
}

impl AIDecisionEngine {
    /// Crear nuevo motor AI con configuración
    pub fn new(config: AIConfig) -> Result<Self> {
        let decision_model = Self::create_model(&config)?;
        
        Ok(Self {
            config,
            decision_history: DecisionHistory::default(),
            decision_model,
        })
    }

    /// Crear con configuración por defecto
    pub fn default() -> Self {
        let config = AIConfig::default();
        Self::new(config).unwrap()
    }

    /// Crear modelo según configuración
    fn create_model(config: &AIConfig) -> Result<DecisionModel> {
        match config.model_name.as_str() {
            "basic_decision_tree" => Ok(Self::create_basic_rules_model()),
            "weight_based" => Ok(Self::create_weight_based_model()),
            "ml_model" => Ok(DecisionModel::MachineLearning {
                model_path: "models/navigator_decision.pkl".to_string(),
                parameters: HashMap::new(),
            }),
            _ => Err(NavigatorError::ai_engine(
                format!("Modelo no soportado: {}", config.model_name)
            )),
        }
    }

    /// Crear modelo básico de reglas
    fn create_basic_rules_model() -> DecisionModel {
        let rules = vec![
            // Regla 1: Si tiene contexto de proyecto, usar índice
            DecisionRule {
                condition: RuleCondition::HasProjectContext,
                strategy: "index_based".to_string(),
                confidence: 0.8,
            },
            // Regla 2: Queries cortas usan índice
            DecisionRule {
                condition: RuleCondition::QueryIsShort(20),
                strategy: "index_based".to_string(),
                confidence: 0.7,
            },
            // Regla 3: Queries largas usan query directo
            DecisionRule {
                condition: RuleCondition::QueryIsLong(100),
                strategy: "query_based".to_string(),
                confidence: 0.75,
            },
            // Regla 4: Por defecto usar híbrido
            DecisionRule {
                condition: RuleCondition::Always,
                strategy: "hybrid_optimized".to_string(),
                confidence: 0.6,
            },
        ];

        DecisionModel::RuleBased { rules }
    }

    /// Crear modelo basado en pesos
    fn create_weight_based_model() -> DecisionModel {
        let mut strategy_weights = HashMap::new();
        strategy_weights.insert("index_based".to_string(), 0.4);
        strategy_weights.insert("query_based".to_string(), 0.3);
        strategy_weights.insert("hybrid_optimized".to_string(), 0.3);

        DecisionModel::WeightBased { strategy_weights }
    }

    /// Tomar decisión sobre qué estrategia usar
    pub async fn decide_strategy(&mut self, context: &NavigationContext) -> Result<AIDecision> {
        tracing::debug!("AI decidiendo estrategia para contexto: {:#?}", context);

        let decision = match &self.decision_model {
            DecisionModel::RuleBased { rules } => {
                self.decide_with_rules(context, rules).await?
            }
            DecisionModel::WeightBased { strategy_weights } => {
                self.decide_with_weights(context, strategy_weights).await?
            }
            DecisionModel::MachineLearning { .. } => {
                // Por ahora, fallback a reglas básicas
                tracing::warn!("ML no implementado aún, usando reglas básicas");
                let basic_rules = Self::create_basic_rules_model();
                if let DecisionModel::RuleBased { rules } = basic_rules {
                    self.decide_with_rules(context, &rules).await?
                } else {
                    return Err(NavigatorError::ai_engine("Error en modelo ML fallback"));
                }
            }
        };

        // Actualizar historial
        self.update_decision_history(&decision);

        tracing::debug!("AI decidió usar estrategia: {} (confianza: {:.2})", 
                       decision.recommended_strategy.strategy_name(), 
                       decision.confidence);

        Ok(decision)
    }

    /// Decidir usando reglas
    async fn decide_with_rules(&self, context: &NavigationContext, rules: &[DecisionRule]) -> Result<AIDecision> {
        for rule in rules {
            if self.evaluate_condition(&rule.condition, context) {
                let strategy = self.strategy_from_name(&rule.strategy)?;
                
                return Ok(AIDecision {
                    recommended_strategy: strategy,
                    confidence: rule.confidence,
                    reasoning: format!("Regla aplicada: {:?}", rule.condition),
                    alternatives: vec![], // TODO: generar alternativas
                });
            }
        }

        // Fallback - no debería llegar aquí si hay regla "Always"
        Err(NavigatorError::ai_engine("No se encontró regla aplicable"))
    }

    /// Decidir usando pesos
    async fn decide_with_weights(&self, context: &NavigationContext, weights: &HashMap<String, f32>) -> Result<AIDecision> {
        // Por simplicidad, seleccionar estrategia con mayor peso
        let (strategy_name, confidence) = weights
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(name, weight)| (name.clone(), *weight))
            .unwrap_or(("hybrid_optimized".to_string(), 0.5));

        let strategy = self.strategy_from_name(&strategy_name)?;
        
        Ok(AIDecision {
            recommended_strategy: strategy,
            confidence,
            reasoning: "Seleccionado por peso máximo".to_string(),
            alternatives: vec![], // TODO: generar alternativas
        })
    }

    /// Evaluar condición de regla
    fn evaluate_condition(&self, condition: &RuleCondition, context: &NavigationContext) -> bool {
        match condition {
            RuleCondition::QueryContains(words) => {
                let query_lower = context.user_query.to_lowercase();
                words.iter().any(|word| query_lower.contains(&word.to_lowercase()))
            }
            RuleCondition::HasProjectContext => context.project_id.is_some(),
            RuleCondition::HasTopicContext => context.topic_id.is_some(),
            RuleCondition::QueryIsShort(max_len) => context.user_query.len() < *max_len,
            RuleCondition::QueryIsLong(min_len) => context.user_query.len() > *min_len,
            RuleCondition::Always => true,
        }
    }

    /// Convertir nombre de estrategia a enum
    fn strategy_from_name(&self, name: &str) -> Result<NavigationStrategy> {
        match name {
            "index_based" => Ok(NavigationStrategy::IndexBased {
                index_type: crate::modes::IndexType::FullTextIndex,
                config: crate::modes::IndexConfig::default(),
            }),
            "query_based" => Ok(NavigationStrategy::QueryBased {
                query_type: crate::modes::QueryType::DirectMongo,
                config: crate::modes::QueryConfig::default(),
            }),
            "hybrid_optimized" => Ok(NavigationStrategy::HybridOptimized {
                index_ratio: 0.6,
                config: crate::modes::HybridConfig::default(),
            }),
            _ => Err(NavigatorError::ai_engine(
                format!("Estrategia no reconocida: {}", name)
            )),
        }
    }

    /// Actualizar historial de decisiones
    fn update_decision_history(&mut self, decision: &AIDecision) {
        self.decision_history.total_decisions += 1;
        
        let strategy_name = decision.recommended_strategy.strategy_name();
        *self.decision_history.strategy_usage.entry(strategy_name.to_string()).or_insert(0) += 1;
        
        // Actualizar promedio de confianza
        let total = self.decision_history.total_decisions as f32;
        let current_avg = self.decision_history.average_confidence;
        self.decision_history.average_confidence = 
            (current_avg * (total - 1.0) + decision.confidence) / total;
    }

    /// Obtener estadísticas del motor AI
    pub fn get_stats(&self) -> &DecisionHistory {
        &self.decision_history
    }

    /// Retroalimentar resultado de decisión (para aprendizaje futuro)
    pub fn feedback(&mut self, decision_id: Option<String>, success: bool) {
        // TODO: Implementar sistema de feedback para mejorar decisiones
        tracing::debug!("Feedback recibido - éxito: {}", success);
        
        // Actualizar success_rate simplificado
        let current_rate = self.decision_history.success_rate;
        let total = self.decision_history.total_decisions as f32;
        
        if total > 0.0 {
            let success_value = if success { 1.0 } else { 0.0 };
            self.decision_history.success_rate = 
                (current_rate * (total - 1.0) + success_value) / total;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AIAssistanceLevel;

    #[tokio::test]
    async fn test_create_ai_engine() {
        let engine = AIDecisionEngine::default();
        let stats = engine.get_stats();
        
        assert_eq!(stats.total_decisions, 0);
        assert_eq!(stats.average_confidence, 0.0);
    }

    #[tokio::test]
    async fn test_decide_strategy_with_project_context() {
        let mut engine = AIDecisionEngine::default();
        
        let context = NavigationContext {
            project_id: Some(uuid::Uuid::new_v4()),
            user_query: "test query".to_string(),
            ..Default::default()
        };

        let decision = engine.decide_strategy(&context).await;
        assert!(decision.is_ok());

        let decision = decision.unwrap();
        assert!(decision.confidence > 0.0);
        assert!(!decision.reasoning.is_empty());
    }

    #[tokio::test]
    async fn test_rule_evaluation() {
        let engine = AIDecisionEngine::default();
        
        let context = NavigationContext {
            project_id: Some(uuid::Uuid::new_v4()),
            user_query: "short".to_string(),
            ..Default::default()
        };

        assert!(engine.evaluate_condition(&RuleCondition::HasProjectContext, &context));
        assert!(engine.evaluate_condition(&RuleCondition::QueryIsShort(10), &context));
        assert!(!engine.evaluate_condition(&RuleCondition::QueryIsLong(100), &context));
    }

    #[test]
    fn test_feedback_system() {
        let mut engine = AIDecisionEngine::default();
        
        // Simular una decisión
        engine.decision_history.total_decisions = 1;
        engine.decision_history.success_rate = 0.0;
        
        // Dar feedback positivo
        engine.feedback(None, true);
        
        assert_eq!(engine.decision_history.success_rate, 1.0);
    }
}
