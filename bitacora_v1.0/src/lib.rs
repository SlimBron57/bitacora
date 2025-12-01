//! # Bitácora V1.0 - Sistema Integral de Gestión Biográfica
//!
//! Sistema completo que integra:
//! - **TelescopeDB**: Base de datos biográfica con cores y metadata
//! - **VoxelDB**: Motor de consultas cúbicas con Octree espacial
//! - **SENSORY ENGINE**: Procesamiento multimodal (texto, audio, video, imágenes)
//! - **HubSpoke**: Enrutador multi-LLM (OpenAI, Anthropic, Perplexity, etc.)
//! - **FBCU**: Compresor fractal basado en blockchain
//! - **CTX7D**: Token contextual 7D con fusión bayesiana
//! - **Expertise Generation**: Generación automática de expertise personalizado
//! - **LIP Protocol**: Lens Interface Protocol para múltiples perspectivas
//! - **Routier Navigator**: Navegación adaptativa de rutas de aprendizaje

#![allow(dead_code, unused_imports, unused_variables)]

// pub mod cells;  // Temporarily commented to fix compilation
pub mod context_token;
pub mod telescopedb;
pub mod voxeldb;
pub mod sensory_engine;
// pub mod multi_agent;  // Temporarily commented
pub mod fbcu;
pub mod qpx;  // 📦 QPX (Quantum Pixel eXchange) - Native Format v1.5 (2025-11-30)
pub mod expertise_generation;
pub mod lip_protocol;
pub mod routier;
pub mod flowpacks;
pub mod shuidao;
pub mod data_import;
pub mod bstradivarius;  // 🎻 BStradivarius - Meta-Loop System (2025-11-30)

// Re-exports públicos
pub use expertise_generation::{
    ExpertiseGenerator, ExpertiseRequest, ExpertisePackage, ExpertiseLevel,
    Curriculum, CurriculumPhase, GeneratedTemplate, KnowledgeBase,
    CuratedResource, PracticalProject, ExpertiseMetadata,
};

pub use lip_protocol::{
    LipEngine, LipConfig, LensInfo, LensInterface, LensOutput,
    LensRequirements, QualityBounds, QualityMetrics, ValidationStatus,
    HarmonyLens, SemanticLens, MttDslLens,
};

pub use flowpacks::{
    FlowPackEngine, FlowPackConfig, FlowPack, FlowPackEntry, EntryType,
    AdaptiveResponse, ResponseLevel, CompressionEngine, CompressionResult,
    SimilarityIndex, FlowPackError,
};

/// Result type estándar de Bitácora
pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// Version del sistema
pub const VERSION: &str = "1.0.0";

/// Protocolo implementado
pub const PROTOCOL: &str = "BITA-1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "1.0.0");
    }

    #[test]
    fn test_protocol() {
        assert_eq!(PROTOCOL, "BITA-1");
    }
}
