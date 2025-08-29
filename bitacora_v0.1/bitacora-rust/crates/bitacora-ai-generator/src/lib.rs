//! # BitaFlow AI Template Generator 🤖⚡
//!
//! Plugin embebido para generar templates BFL usando IA
//! Mantiene separación completa del sistema core BitaFlow

pub mod errors;
pub mod providers;
pub mod generator;

pub use errors::{AIGeneratorError, Result, AIGeneratorResultExt};
pub use providers::{AIProvider, ProviderConfig, ProviderLimits};
pub use generator::{AITemplateGenerator, GenerationRequest, GenerationResult, GeneratorConfig};

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

/// Versión del AI Generator
pub const AI_GENERATOR_VERSION: &str = "0.1.0";

/// Plugin principal para generar templates con IA
#[async_trait]
pub trait TemplateGeneratorPlugin: Send + Sync {
    /// Generar template basado en descripción
    async fn generate_template(&self, request: GenerationRequest) -> Result<GenerationResult>;
    
    /// Analizar templates existentes para mejorar generación
    async fn analyze_existing_templates(&self, templates: Vec<String>) -> Result<TemplateAnalysis>;
    
    /// Obtener configuración del provider
    fn get_provider_config(&self) -> &ProviderConfig;
    
    /// Verificar si el provider está disponible
    async fn is_available(&self) -> bool;
}

/// Análisis de templates existentes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateAnalysis {
    /// Patrones encontrados en templates
    pub patterns: Vec<TemplatePattern>,
    /// Dominios más comunes
    pub common_domains: Vec<String>,
    /// Topics frecuentes
    pub frequent_topics: Vec<String>,
    /// Variables más utilizadas
    pub common_variables: Vec<String>,
    /// Estructura típica de templates
    pub typical_structure: TemplateStructure,
}

/// Patrón encontrado en template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePattern {
    /// Tipo de patrón
    pub pattern_type: String,
    /// Frecuencia del patrón
    pub frequency: u32,
    /// Ejemplo del patrón
    pub example: String,
    /// Score de relevancia
    pub relevance: f32,
}

/// Estructura típica de template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStructure {
    /// Secciones comunes
    pub sections: Vec<String>,
    /// Número promedio de steps
    pub avg_steps: u32,
    /// Flujo común de navegación
    pub common_flow: Vec<String>,
}
