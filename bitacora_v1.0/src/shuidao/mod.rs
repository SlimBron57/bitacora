// === SHUIDAO (水道) - INTENTION-ORIENTED COGNITIVE ENGINE ===
// Sistema cognitivo que evoluciona de detección de patrones a comprensión de intención
// Implementa: DA-032 (ShuiDao - Intention-Oriented Cognitive Architecture)
// Evolución: FlowPacks Phase 3a (patrones) → ShuiDao Phase 3b (intención)
// Creado: 2025-11-24 11:42:28
// Autor: Sistema Bitácora v1.0

//! # ShuiDao (水道) - The Water's Way
//!
//! > "El agua no fuerza su camino. Encuentra el cauce natural." — Filosofía ShuiDao
//!
//! ## Filosofía
//!
//! ShuiDao representa la evolución de Bitácora de un **asistente que detecta patrones**
//! (FlowPacks Phase 3a) a un **compañero que comprende intenciones** (Phase 3b).
//!
//! ## Arquitectura de 5 Modos Cognitivos
//!
//! 1. **Operational** - Usuario quiere HACER algo (proyectos, tareas, progreso)
//! 2. **Procedural** - Usuario sigue PASOS (recetas, guías, tutorials)
//! 3. **Learning** - Usuario quiere APRENDER (conceptos, conocimiento)
//! 4. **Conversational** - Usuario quiere DIALOGAR (narrativa, biografía)
//! 5. **Light** - Usuario quiere RESPUESTA RÁPIDA (facts, cálculos)
//!
//! ## Componentes Principales
//!
//! ```text
//! UserInput
//!     ↓
//! IntentionDetector (multi-factor: verb/topic/tone/context)
//!     ↓
//! DetectedIntention { mode, confidence, context }
//!     ↓
//! CognitiveRouter (enruta según modo)
//!     ↓
//! OperationalEngine | ProceduralEngine | LearningEngine | ConversationalEngine | LightEngine
//!     ↓
//! EngineResponse { content, metadata, next_action }
//!     ↓
//! ResponseSynthesizer (combina con FlowPacks/FBCU)
//!     ↓
//! SynthesizedResponse (respuesta final al usuario)
//! ```
//!
//! ## Diferencia con FlowPacks
//!
//! ```text
//! FlowPacks Phase 3a (✅ COMPLETADO):
//! - Detecta similitud semántica (cosine similarity)
//! - Responde con referencias a conversaciones previas
//! - Ahorra tokens (~0.85x compression)
//! - Pregunta: "Eduardo preguntó sobre CTX7D de nuevo"
//!
//! ShuiDao Phase 3b (🚧 IMPLEMENTANDO):
//! - Detecta INTENCIÓN del usuario
//! - Clasifica en 5 modos cognitivos
//! - Crea estructuras adaptadas (proyectos, recetas, learning paths)
//! - Pregunta: "¿Qué quiere HACER Eduardo con esta información?"
//! ```
//!
//! ## Performance Targets
//!
//! - IntentionDetector: <15ms (multi-factor scoring)
//! - CognitiveRouter: <5ms (dispatch)
//! - Engine processing: <180ms (mode-specific)
//! - End-to-end: <200ms (95th percentile)
//! - Throughput: 50+ messages/second
//!
//! ## Integración con FlowPacks
//!
//! ShuiDao NO reemplaza FlowPacks, lo EXTIENDE:
//!
//! ```rust
//! // FlowPacks maneja similitud semántica
//! let similarity = flowpacks.check_similarity(user_input);
//!
//! if similarity.score > 0.95 {
//!     // Respuesta por referencia (FlowPacks)
//!     return flowpacks.generate_reference_response(similarity.pack_id);
//! } else {
//!     // Detección de intención (ShuiDao)
//!     let intention = shuidao.detect_intention(user_input);
//!     let response = shuidao.route_and_process(intention);
//!     
//!     // Combinar respuesta + actualizar FlowPacks
//!     flowpacks.add_to_pack(user_input, response);
//!     return response;
//! }
//! ```

// ========================================
// MÓDULOS
// ========================================

pub mod error;
pub mod intention_detector;
pub mod cognitive_router; // ✅ Week 1 Day 2
pub mod operational_engine; // ✅ Week 2 Days 1-2
pub mod procedural_engine; // ✅ Week 2 Days 3-4
// pub mod memory_bridge;     // TODO: Week 3 Day 1
// pub mod response_synthesizer; // TODO: Week 3 Day 1 (depends on memory_bridge)
// pub mod icebreaker_engine; // TODO: Week 3 Day 5 (in conversational_engine)
pub mod light_engine;      // ✅ Week 3 Day 5
pub mod learning_engine;   // ✅ Week 3 Day 5
pub mod conversational_engine; // ✅ Week 3 Day 5
pub mod topic_graph;       // ✅ DA-033 TopicGraph System
pub mod topic_learning;    // ✅ DA-033 Auto-Discovery
pub mod topic_integration; // ✅ DA-033 VoxelDB Persistence
pub mod emotional_space;   // ✅ DA-033 EmotionalSpace System (VAD+F)
pub mod tone_learning;     // ✅ DA-033 Tone Auto-Discovery (User-Named)
pub mod tone_integration;  // ✅ DA-033 Tone Persistence + MTT-DSL

// ========================================
// RE-EXPORTS PÚBLICOS
// ========================================

pub use error::{Result, ShuiDaoError};

// Exportar IntentionDetector (✅ Implementado Week 1 Day 1)
pub use intention_detector::{
    CognitiveMode, DetectedIntention, IntentionDetector, IntentionMetadata, Submode,
};

// Exportar CognitiveRouter (✅ Implementado Week 1 Day 2)
pub use cognitive_router::{CognitiveRouter, RoutingDecision, RoutingMetadata};

// Exportar OperationalEngine (✅ Implementado Week 2 Days 1-2)
pub use operational_engine::{
    ActionRecommendation, OperationalAction, OperationalProject, OperationalProjectEngine,
    OperationalResponse, Priority, ProjectCategory, ProjectStatus, SubProject, Task, TaskStatus,
};

// Exportar ProceduralEngine (✅ Implementado Week 2 Days 3-4)
pub use procedural_engine::{
    Difficulty, ExecutionStatus, ProceduralAction, ProceduralRecipeEngine, ProceduralResponse,
    Recipe, RecipeCategory, RecipeExecution, RecipeStep, StepProgress, StepResult,
    StepValidation,
};

// TODO: Exportar MemoryBridge (Pendiente Week 3 Day 1)
// pub use memory_bridge::{
//     BiographicalEntry, IntentionTemplate, LearningPath, MasteryLevel, MemoryBridge, MemoryItem,
//     MemoryQuery, MemoryQueryType, MemoryResult, MemorySource, Project, 
//     ProjectStatus as MemoryProjectStatus, TimeRange,
// };

// TODO: Exportar ResponseSynthesizer (Pendiente - depende de MemoryBridge)
// pub use response_synthesizer::{
//     ConversationContext, ContextReference, EngineResponse, ReferenceType, ResponseMetadata,
//     ResponseSynthesizer, ResponseTone, SynthesizedResponse, UserPreferences, Verbosity,
// };

// TODO: Exportar IceBreakerEngine (Pendiente - migrar de conversational_engine)
// pub use icebreaker_engine::{
//     ExtractedUserData, IceBreakerCriteria, IceBreakerEngine, IceBreakerError, IceBreakerResult,
//     IceBreakerStage, IceBreakerTemplate, ProcessResult, RelationshipState, SentimentLevel,
// };

// Exportar LightEngine (✅ Implementado Week 3 Day 5)
pub use light_engine::{LightEngine, LightResponse, LightResponseType};

// Exportar LearningEngine (✅ Implementado Week 3 Day 5)
pub use learning_engine::{
    AdaptiveRecommendation, Checkpoint, CheckpointStatus, ConfusionPoint,
    LearningAction, LearningEngine, LearningResponse, Module, ModuleStatus,
    PathStatus, RecommendationType,
};
// Note: Difficulty and LearningPath have naming conflicts with ProceduralEngine and MemoryBridge
// Use learning_engine::Difficulty and learning_engine::LearningPath explicitly when needed

// Exportar ConversationalEngine (✅ Implementado Week 3 Day 5)
pub use conversational_engine::{
    Conversation, ConversationalEngine, ConversationalResponse, ConversationalTone,
    ConversationMessage, ConversationStatus, SentimentScore, Speaker,
};

// ========================================
// VERSIÓN Y METADATA
// ========================================

/// Versión actual de ShuiDao
pub const VERSION: &str = "1.0.0-beta";

/// Fecha de inicio de implementación
pub const IMPLEMENTATION_START: &str = "2025-11-24";

/// Estado de implementación
pub const STATUS: &str = "🚧 IN PROGRESS - Week 1 Day 2";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_format() {
        assert!(VERSION.contains("1.0.0"));
    }

    #[test]
    fn test_implementation_date() {
        assert_eq!(IMPLEMENTATION_START, "2025-11-24");
    }
}
