// LIP Protocol - Lens Interface Protocol
// Sistema de contratos para aplicar lentes especializadas a FBCUs

pub mod types;
pub mod validation;
pub mod engine;
pub mod lenses;

pub use types::*;
pub use validation::Validator;
pub use engine::{LipEngine, LipConfig, LensInfo};
pub use lenses::{HarmonyLens, SemanticLens, MttDslLens};

pub mod prelude {
    pub use super::types::*;
    pub use super::validation::Validator;
    pub use super::engine::{LipEngine, LipConfig};
    pub use super::lenses::{HarmonyLens, SemanticLens, MttDslLens};
}
