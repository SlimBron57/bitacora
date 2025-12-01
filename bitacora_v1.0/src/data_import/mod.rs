//! # 🍽️ Data Import - Sistema de Digestión Metabólica
//!
//! **Filosofía:** "No ingestión, digestión con respeto por la fuente"
//!
//! Este módulo implementa el sistema de importación de datos externos de Bitácora v1.1,
//! inspirado en la digestión biológica: datos crudos → nutrientes específicos → distribución inteligente.
//!
//! ## Arquitectura (5 Phases)
//!
//! ```text
//! Data Import Pipeline
//! ├── Phase 1: Quarantine       → Safety layer (inspect, approve/reject)
//! ├── Phase 2: Digestion        → Source-specific processing (WhatsApp ≠ Email)
//! ├── Phase 3: Extraction       → 7D parallel nutrient extraction
//! ├── Phase 4: Validation       → Coherence checking (detect conflicts)
//! └── Phase 5: Distribution     → Route to TelescopeDB, TopicGraph, EmotionalSpace
//! ```
//!
//! ## Componentes Principales
//!
//! - **QuarantineZone**: Safety layer con state machine (Pending → Safe/Suspicious/Rejected)
//! - **HybridDigester**: Core (hard-coded) + Logic (templated YAML)
//! - **NutrientExtractor**: 7D parallel extraction (tokio::join!)
//! - **CoherenceValidator**: Conflict detection entre sources
//! - **NutrientDistributor**: Parallel routing a subsistemas
//!
//! ## Diseño Híbrido
//!
//! - **Layer 1 (Core):** Hard-coded, estable, rápido
//! - **Layer 2 (Logic):** YAML templates, evolvable sin recompilar
//! - **Layer 3 (Distribution):** Hard-coded, parallel, confiable
//!
//! ## Performance Targets (v1.0)
//!
//! - Quarantine: <500ms per file
//! - Digestion: <30s for 1000 messages  
//! - Extraction: <10s (parallel 7D)
//! - Validation: <2s for 500 nutrients
//! - Distribution: <3s (parallel routing)
//! - **Total Pipeline: <45s end-to-end** (60x faster than 30min manual)
//!
//! ## Documentación Relacionada
//!
//! - Vision: `ROADMAP_V2/00_VISION/09_metabolic-digestion-vision.md`
//! - Architecture: `ROADMAP_V2/01_ARQUITECTURA/18_metabolic-digestion-system.md`
//! - Spec: `ROADMAP_V2/02_COMPONENTES/17_data-import-engine.md`
//! - Plan: `ROADMAP_V2/04_IMPLEMENTACION/PHASE_7X_DATA_IMPORT.md`
//! - Templates: `ROADMAP_V2/07_TEMPLATES/digesters/`
//!
//! ## Ejemplo de Uso
//!
//! ```rust,no_run
//! use bitacora::data_import::{QuarantineZone, DataSource};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Phase 1: Quarantine
//!     let raw_data = std::fs::read("whatsapp_export.txt")?;
//!     let mut quarantine = QuarantineZone::inspect(raw_data, DataSource::WhatsApp).await?;
//!     
//!     // Approve after inspection
//!     quarantine.approve();
//!     
//!     if quarantine.is_ready_for_digestion() {
//!         println!("✅ Safe to proceed with digestion");
//!         // Continuar con Phase 2: Digestion...
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Changelog
//!
//! - **v1.0.0** (2025-11-29): Módulo creado, QuarantineZone implementado
//! - **v1.1.0** (planned): HybridDigester + template system
//! - **v1.2.0** (planned): NutrientExtractor 7D parallel
//! - **v1.3.0** (planned): CoherenceValidator + NutrientDistributor

pub mod quarantine;
pub mod digestion;
pub mod extraction;
pub mod validation;
pub mod distribution;
pub mod error;

// Re-exports for convenience
pub use quarantine::{QuarantineZone, QuarantineState, QuarantineMetadata, DataSource};
pub use error::{DataImportError, Result};
pub use extraction::{
    NutrientExtractor, NutrientDimension, 
    BiographicalExtractor, InterestExtractor, EmotionalExtractor, TemporalExtractor, BehavioralExtractor, RelationshipExtractor,
};

/// Versión del Data Import Engine
pub const VERSION: &str = "1.0.0";

/// Performance budgets (microsegundos)
pub mod performance {
    pub const QUARANTINE_BUDGET_MS: u64 = 500;
    pub const DIGESTION_BUDGET_MS: u64 = 30_000;
    pub const EXTRACTION_BUDGET_MS: u64 = 10_000;
    pub const VALIDATION_BUDGET_MS: u64 = 2_000;
    pub const DISTRIBUTION_BUDGET_MS: u64 = 3_000;
    pub const TOTAL_PIPELINE_BUDGET_MS: u64 = 45_000;
}
