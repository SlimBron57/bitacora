//! # Cognitive Router
//!
//! Router que dirige DetectedIntention hacia el modo cognitivo correcto.
//!
//! ## Responsabilidades
//!
//! 1. **Mode Selection**: Selecciona engine según DetectedIntention
//! 2. **Fallback Chain**: Si confidence < threshold, activa fallback
//! 3. **Integration Point**: Conecta con FlowPacks Phase 3a (pattern check first)
//! 4. **Performance**: Target <5ms routing time (O(1) complexity)
//!
//! ## Arquitectura
//!
//! ```text
//! IntentionDetector → CognitiveRouter → [Mode Engine]
//!                           │
//!                           ├─> OperationalEngine (TODO Week 2)
//!                           ├─> ProceduralEngine (TODO Week 2)
//!                           ├─> LearningEngine (TODO Week 3)
//!                           ├─> ConversationalEngine (TODO Week 3)
//!                           └─> LightEngine (TODO Week 3)
//! ```
//!
//! ## Integration con FlowPacks
//!
//! ```text
//! Input → FlowPacks (similarity ≥0.95?) → Yes: Reference Response
//!                                        → No: ShuiDao Pipeline
//!                                             IntentionDetector → CognitiveRouter → ...
//! ```
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 1 Day 2)

use crate::shuidao::error::{Result, ShuiDaoError};
use crate::shuidao::intention_detector::{CognitiveMode, DetectedIntention};
use std::collections::HashMap;
use std::time::Instant;

// ============================================================================
// STRUCTURES
// ============================================================================

/// CognitiveRouter: Dispatcher to correct cognitive mode
///
/// **Performance Target**: <5ms routing time (O(1) complexity)
#[derive(Debug, Clone)]
pub struct CognitiveRouter {
    /// Minimum confidence to accept routing (default: 0.60)
    pub min_confidence: f32,

    /// Fallback mode if confidence < min_confidence (default: Light)
    pub fallback_mode: CognitiveMode,

    /// Enable fallback chain (default: true)
    pub enable_fallback: bool,
}

/// Routing Decision Result
///
/// Contains selected mode, metadata, and performance metrics
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    /// Selected cognitive mode
    pub selected_mode: CognitiveMode,

    /// Whether fallback was used
    pub fallback_used: bool,

    /// Fallback chain (if confidence < threshold)
    pub fallback_chain: Vec<CognitiveMode>,

    /// Decision confidence (0.0-1.0)
    pub confidence: f64,

    /// Processing metadata
    pub metadata: RoutingMetadata,
}

/// Routing Metadata
#[derive(Debug, Clone)]
pub struct RoutingMetadata {
    /// Routing time in milliseconds
    pub routing_time_ms: f64,

    /// Original detected intention
    pub original_intention: DetectedIntention,

    /// Alternative modes considered
    pub alternatives: Vec<(CognitiveMode, f32)>, // (mode, confidence)

    /// Decision reason
    pub reason: String,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl CognitiveRouter {
    /// Create new CognitiveRouter with default configuration
    ///
    /// **Defaults**:
    /// - min_confidence: 0.60 (60%)
    /// - fallback_mode: Light (quick answers)
    /// - enable_fallback: true
    pub fn new() -> Self {
        Self {
            min_confidence: 0.60,
            fallback_mode: CognitiveMode::Light,
            enable_fallback: true,
        }
    }

    /// Create CognitiveRouter with custom configuration
    pub fn with_config(min_confidence: f32, fallback_mode: CognitiveMode) -> Self {
        Self {
            min_confidence,
            fallback_mode,
            enable_fallback: true,
        }
    }

    /// Route detected intention to appropriate cognitive mode
    ///
    /// **Algorithm**:
    /// 1. Check confidence against threshold
    /// 2. If confidence ≥ min_confidence: Use detected mode
    /// 3. If confidence < min_confidence && enable_fallback: Use fallback chain
    /// 4. Otherwise: Return error
    ///
    /// **Fallback Chain**:
    /// - Operational → Procedural → Light
    /// - Learning → Conversational → Light
    /// - Conversational → Light
    /// - Procedural → Light
    /// - Light → (no fallback)
    ///
    /// **Performance**: Target <5ms (O(1) complexity)
    pub fn route(&self, intention: DetectedIntention) -> Result<RoutingDecision> {
        let start = Instant::now();

        // Validate intention confidence
        if intention.confidence < 0.0 || intention.confidence > 1.0 {
            return Err(ShuiDaoError::RoutingFailed {
                detected_mode: format!("{:?}", intention.mode),
                reason: format!("Invalid confidence: {}", intention.confidence),
            });
        }

        // Decision logic
        let (selected_mode, fallback_used, fallback_chain) = if intention.confidence
            >= self.min_confidence as f64
        {
            // High confidence: use detected mode
            (intention.mode.clone(), false, Vec::new())
        } else if self.enable_fallback {
            // Low confidence: use fallback chain
            let fallback_chain = self.build_fallback_chain(&intention.mode);
            let selected_mode = fallback_chain.first().unwrap_or(&self.fallback_mode).clone();
            (selected_mode, true, fallback_chain)
        } else {
            // Fallback disabled: error
            return Err(ShuiDaoError::RoutingFailed {
                detected_mode: format!("{:?}", intention.mode),
                reason: format!(
                    "Confidence {} below threshold {} and fallback disabled",
                    intention.confidence, self.min_confidence
                ),
            });
        };

        // Build alternatives list
        let alternatives = self.build_alternatives(&intention);

        // Build decision reason
        let reason = if fallback_used {
            format!(
                "Low confidence ({:.2}), using fallback: {:?}",
                intention.confidence, selected_mode
            )
        } else {
            format!(
                "High confidence ({:.2}), routing to: {:?}",
                intention.confidence, selected_mode
            )
        };

        // Calculate routing time
        let routing_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(RoutingDecision {
            selected_mode,
            fallback_used,
            fallback_chain,
            confidence: intention.confidence,
            metadata: RoutingMetadata {
                routing_time_ms,
                original_intention: intention,
                alternatives,
                reason,
            },
        })
    }

    /// Build fallback chain for a given mode
    ///
    /// **Chains**:
    /// - Operational → Procedural → Light
    /// - Learning → Conversational → Light
    /// - Conversational → Light
    /// - Procedural → Light
    /// - Light → (empty)
    fn build_fallback_chain(&self, mode: &CognitiveMode) -> Vec<CognitiveMode> {
        match mode {
            CognitiveMode::Operational => {
                vec![CognitiveMode::Procedural, CognitiveMode::Light]
            }
            CognitiveMode::Learning => {
                vec![CognitiveMode::Conversational, CognitiveMode::Light]
            }
            CognitiveMode::Conversational => {
                vec![CognitiveMode::Light]
            }
            CognitiveMode::Procedural => {
                vec![CognitiveMode::Light]
            }
            CognitiveMode::Light => {
                // Light has no fallback (it's the ultimate fallback)
                Vec::new()
            }
        }
    }

    /// Build alternatives list (modes with score > 0.5)
    ///
    /// For now, returns only the detected mode. Future: multi-mode scoring.
    fn build_alternatives(&self, intention: &DetectedIntention) -> Vec<(CognitiveMode, f32)> {
        // TODO Week 3: Implement multi-mode scoring
        // For now, return detected mode only
        vec![(intention.mode.clone(), intention.confidence as f32)]
    }

    /// Get default routing decision (emergency fallback)
    ///
    /// Used when routing fails catastrophically
    pub fn default_decision(&self) -> RoutingDecision {
        RoutingDecision {
            selected_mode: self.fallback_mode.clone(),
            fallback_used: true,
            fallback_chain: Vec::new(),
            confidence: 0.0,
            metadata: RoutingMetadata {
                routing_time_ms: 0.0,
                original_intention: DetectedIntention {
                    mode: self.fallback_mode.clone(),
                    submode: None,
                    confidence: 0.0,
                    extracted_entities: HashMap::new(),
                    metadata: crate::shuidao::intention_detector::IntentionMetadata {
                        verb_score: 0.0,
                        topic_score: 0.0,
                        tone_score: 0.0,
                        context_score: 0.0,
                        processing_time_ms: 0,
                        alternative_mode: None,
                    },
                },
                alternatives: Vec::new(),
                reason: "Emergency fallback (routing failed)".to_string(),
            },
        }
    }
}

impl Default for CognitiveRouter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shuidao::intention_detector::{IntentionMetadata, Submode};

    fn create_test_intention(mode: CognitiveMode, confidence: f32) -> DetectedIntention {
        DetectedIntention {
            mode,
            submode: None,
            confidence: confidence as f64,
            extracted_entities: HashMap::new(),
            metadata: IntentionMetadata {
                verb_score: 0.8,
                topic_score: 0.7,
                tone_score: 0.6,
                context_score: 0.5,
                processing_time_ms: 10,
                alternative_mode: None,
            },
        }
    }

    #[test]
    fn test_router_creation() {
        let router = CognitiveRouter::new();
        assert_eq!(router.min_confidence, 0.60);
        assert_eq!(router.fallback_mode, CognitiveMode::Light);
        assert!(router.enable_fallback);
    }

    #[test]
    fn test_high_confidence_routing() {
        let router = CognitiveRouter::new();
        let intention = create_test_intention(CognitiveMode::Operational, 0.85);

        let decision = router.route(intention).unwrap();

        assert_eq!(decision.selected_mode, CognitiveMode::Operational);
        assert!(!decision.fallback_used);
        assert!(decision.fallback_chain.is_empty());
        assert!((decision.confidence - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_low_confidence_fallback() {
        let router = CognitiveRouter::new();
        let intention = create_test_intention(CognitiveMode::Operational, 0.45);

        let decision = router.route(intention).unwrap();

        assert!(decision.fallback_used);
        assert!(!decision.fallback_chain.is_empty());
        assert_eq!(decision.fallback_chain[0], CognitiveMode::Procedural);
        assert_eq!(decision.fallback_chain[1], CognitiveMode::Light);
    }

    #[test]
    fn test_fallback_chain_operational() {
        let router = CognitiveRouter::new();
        let chain = router.build_fallback_chain(&CognitiveMode::Operational);

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0], CognitiveMode::Procedural);
        assert_eq!(chain[1], CognitiveMode::Light);
    }

    #[test]
    fn test_fallback_chain_learning() {
        let router = CognitiveRouter::new();
        let chain = router.build_fallback_chain(&CognitiveMode::Learning);

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0], CognitiveMode::Conversational);
        assert_eq!(chain[1], CognitiveMode::Light);
    }

    #[test]
    fn test_fallback_chain_light() {
        let router = CognitiveRouter::new();
        let chain = router.build_fallback_chain(&CognitiveMode::Light);

        assert!(chain.is_empty()); // Light has no fallback
    }

    #[test]
    fn test_routing_performance() {
        let router = CognitiveRouter::new();
        let intention = create_test_intention(CognitiveMode::Learning, 0.80);

        let decision = router.route(intention).unwrap();

        // Target: <5ms routing time
        assert!(
            decision.metadata.routing_time_ms < 5.0,
            "Routing took {:.2}ms (target: <5ms)",
            decision.metadata.routing_time_ms
        );
    }

    #[test]
    fn test_invalid_confidence() {
        let router = CognitiveRouter::new();
        let mut intention = create_test_intention(CognitiveMode::Operational, 1.5);

        let result = router.route(intention);
        assert!(result.is_err());
    }

    #[test]
    fn test_fallback_disabled() {
        let mut router = CognitiveRouter::new();
        router.enable_fallback = false;

        let intention = create_test_intention(CognitiveMode::Operational, 0.45);

        let result = router.route(intention);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_threshold() {
        let router = CognitiveRouter::with_config(0.80, CognitiveMode::Conversational);
        let intention = create_test_intention(CognitiveMode::Learning, 0.75);

        let decision = router.route(intention).unwrap();

        // confidence 0.75 < 0.80 → fallback
        assert!(decision.fallback_used);
    }

    #[test]
    fn test_default_decision() {
        let router = CognitiveRouter::new();
        let decision = router.default_decision();

        assert_eq!(decision.selected_mode, CognitiveMode::Light);
        assert!(decision.fallback_used);
        assert!((decision.confidence - 0.0).abs() < 0.01);
    }
}
