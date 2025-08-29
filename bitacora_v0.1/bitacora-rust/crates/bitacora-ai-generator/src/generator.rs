//! # AI Template Generator Core 🤖
//!
//! Implementación principal del generador de templates con IA

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::providers::{AIProvider, ProviderConfig};
use crate::errors::{AIGeneratorError, Result};
use crate::{TemplateGeneratorPlugin, TemplateAnalysis};

/// Generador principal de templates con IA
#[derive(Debug)]
pub struct AITemplateGenerator {
    /// Provider de IA configurado
    provider: Box<dyn AIProvider>,
    /// Configuración del generador
    config: GeneratorConfig,
    /// Cache de análisis de templates
    analysis_cache: Option<TemplateAnalysis>,
}

/// Configuración del generador
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Creatividad del generador (0.0 - 1.0)
    pub creativity: f32,
    /// Máximo número de steps por template
    pub max_steps: usize,
    /// Incluir variables automáticamente
    pub auto_variables: bool,
    /// Incluir métricas por defecto
    pub include_metrics: bool,
    /// Prefijo de alias personalizado
    pub alias_prefix: Option<String>,
}

/// Request para generación de template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Descripción del problema/objetivo
    pub description: String,
    /// Dominio específico (debug, test, analysis, etc.)
    pub domain: String,
    /// Topic específico
    pub topic: String,
    /// Nivel de autonomía deseado
    pub autonomy_level: String,
    /// Contexto adicional
    pub context: HashMap<String, String>,
    /// Templates de referencia (opcional)
    pub reference_templates: Vec<String>,
    /// Restricciones específicas
    pub constraints: Vec<String>,
}

/// Resultado de generación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    /// Contenido BFL generado
    pub bfl_content: String,
    /// Alias generado
    pub alias: String,
    /// Metadata del template generado
    pub metadata: GeneratedTemplateMetadata,
    /// Score de confianza (0.0 - 1.0)
    pub confidence_score: f32,
    /// Explicación del template generado
    pub explanation: String,
    /// Sugerencias de mejora
    pub improvements: Vec<String>,
}

/// Metadata del template generado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedTemplateMetadata {
    /// Timestamp de generación
    pub generated_at: DateTime<Utc>,
    /// Provider usado
    pub provider: String,
    /// Versión del generador
    pub generator_version: String,
    /// Tokens utilizados (si disponible)
    pub tokens_used: Option<u32>,
    /// Tiempo de generación
    pub generation_time: f64,
    /// Parámetros usados
    pub generation_params: HashMap<String, String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            creativity: 0.7,
            max_steps: 20,
            auto_variables: true,
            include_metrics: true,
            alias_prefix: None,
        }
    }
}

impl AITemplateGenerator {
    /// Crear nuevo generador con provider específico
    pub fn new(provider: Box<dyn AIProvider>) -> Result<Self> {
        Ok(Self {
            provider,
            config: GeneratorConfig::default(),
            analysis_cache: None,
        })
    }
    
    /// Crear generador con configuración personalizada
    pub fn with_config(provider: Box<dyn AIProvider>, config: GeneratorConfig) -> Result<Self> {
        Ok(Self {
            provider,
            config,
            analysis_cache: None,
        })
    }
    
    /// Generar múltiples templates para A/B testing
    pub async fn generate_multiple_templates(
        &self, 
        request: GenerationRequest, 
        count: usize
    ) -> Result<Vec<GenerationResult>> {
        let mut results = Vec::new();
        
        for i in 0..count {
            let mut modified_request = request.clone();
            modified_request.context.insert(
                "variation".to_string(), 
                format!("v{}", i + 1)
            );
            
            let result = self.generate_template(modified_request).await?;
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// Mejorar template existente
    pub async fn improve_template(
        &self, 
        existing_bfl: &str, 
        improvement_goals: Vec<String>
    ) -> Result<GenerationResult> {
        let request = GenerationRequest {
            description: format!(
                "Improve this existing template:\n{}\n\nImprovement goals: {}",
                existing_bfl,
                improvement_goals.join(", ")
            ),
            domain: "improvement".to_string(),
            topic: "optimization".to_string(),
            autonomy_level: "Interactive".to_string(),
            context: HashMap::new(),
            reference_templates: vec![existing_bfl.to_string()],
            constraints: improvement_goals,
        };
        
        self.generate_template(request).await
    }
    
    /// Generar template basado en error logs
    pub async fn generate_from_error_logs(
        &self, 
        error_logs: &str, 
        project_context: &str
    ) -> Result<GenerationResult> {
        let request = GenerationRequest {
            description: format!(
                "Create a debugging template for these error logs:\n{}\n\nProject context: {}",
                error_logs, project_context
            ),
            domain: "debug".to_string(),
            topic: "error_analysis".to_string(),
            autonomy_level: "Interactive".to_string(),
            context: HashMap::from([
                ("error_logs".to_string(), error_logs.to_string()),
                ("project_context".to_string(), project_context.to_string()),
            ]),
            reference_templates: vec![],
            constraints: vec![
                "Focus on systematic debugging approach".to_string(),
                "Include log analysis steps".to_string(),
                "Provide error pattern detection".to_string(),
            ],
        };
        
        self.generate_template(request).await
    }
    
    /// Configurar creatividad del generador
    pub fn set_creativity(&mut self, creativity: f32) {
        self.config.creativity = creativity.clamp(0.0, 1.0);
    }
    
    /// Obtener estadísticas del generador
    pub async fn get_generation_stats(&self) -> GeneratorStats {
        GeneratorStats {
            provider_name: self.provider.name(),
            total_generated: 0, // TODO: Implementar tracking
            avg_confidence: 0.0, // TODO: Implementar tracking
            most_common_domain: "debug".to_string(), // TODO: Implementar tracking
            cache_status: self.analysis_cache.is_some(),
        }
    }
    
    // Métodos privados de implementación
    
    fn generate_alias(&self, domain: &str, topic: &str) -> String {
        let prefix = self.config.alias_prefix.as_deref().unwrap_or("BITA-NAV");
        let domain_clean = domain.to_uppercase().replace(" ", "-");
        let topic_clean = topic.to_uppercase().replace(" ", "-");
        format!("{}-{}-{}-AI-v1", prefix, domain_clean, topic_clean)
    }
    
    fn build_generation_prompt(&self, request: &GenerationRequest) -> String {
        let mut prompt = format!(
            "Generate a BitaFlow template for the following requirement:\n\n\
            Description: {}\n\
            Domain: {}\n\
            Topic: {}\n\
            Autonomy Level: {}\n\n",
            request.description, request.domain, request.topic, request.autonomy_level
        );
        
        if !request.context.is_empty() {
            prompt.push_str("Context:\n");
            for (key, value) in &request.context {
                prompt.push_str(&format!("- {}: {}\n", key, value));
            }
            prompt.push('\n');
        }
        
        if !request.constraints.is_empty() {
            prompt.push_str("Constraints:\n");
            for constraint in &request.constraints {
                prompt.push_str(&format!("- {}\n", constraint));
            }
            prompt.push('\n');
        }
        
        prompt.push_str(
            "Generate a complete BitaFlow template (.bfl) with:\n\
            1. Clear alias following BITA-NAV-DOMAIN-TOPIC-v1 pattern\n\
            2. Context variables section\n\
            3. Navigation steps with {{variable}} substitution\n\
            4. Interactive questions if applicable\n\
            5. Metrics section\n\
            6. Proper BFL formatting\n\n\
            Focus on practical, actionable steps that solve the specific problem described."
        );
        
        prompt
    }
}

#[async_trait::async_trait]
impl TemplateGeneratorPlugin for AITemplateGenerator {
    async fn generate_template(&self, request: GenerationRequest) -> Result<GenerationResult> {
        println!("🤖 Generating template with AI for: {}/{}", request.domain, request.topic);
        
        let start_time = std::time::Instant::now();
        let prompt = self.build_generation_prompt(&request);
        
        // Generate with AI provider
        let ai_response = self.provider.generate_text(&prompt, self.config.creativity).await?;
        let generation_time = start_time.elapsed().as_secs_f64();
        
        // Generate alias
        let alias = self.generate_alias(&request.domain, &request.topic);
        
        // Build metadata
        let metadata = GeneratedTemplateMetadata {
            generated_at: Utc::now(),
            provider: self.provider.name(),
            generator_version: crate::AI_GENERATOR_VERSION.to_string(),
            tokens_used: None, // TODO: Get from provider if available
            generation_time,
            generation_params: HashMap::from([
                ("creativity".to_string(), self.config.creativity.to_string()),
                ("max_steps".to_string(), self.config.max_steps.to_string()),
            ]),
        };
        
        // Calculate confidence based on response quality
        let confidence_score = self.calculate_confidence(&ai_response, &request);
        
        // Generate improvements suggestions
        let improvements = self.generate_improvement_suggestions(&ai_response, &request);
        
        let result = GenerationResult {
            bfl_content: ai_response,
            alias: alias.clone(),
            metadata,
            confidence_score,
            explanation: format!(
                "Generated template for {} in domain '{}' with topic '{}' using {} provider",
                request.description, request.domain, request.topic, self.provider.name()
            ),
            improvements,
        };
        
        println!("✅ Template generated with alias: {}", alias);
        println!("   Confidence: {:.1}%", confidence_score * 100.0);
        println!("   Generation time: {:.2}s", generation_time);
        
        Ok(result)
    }
    
    async fn analyze_existing_templates(&self, _templates: Vec<String>) -> Result<TemplateAnalysis> {
        // TODO: Implement template analysis
        // This would analyze patterns, common structures, frequently used variables, etc.
        
        Ok(TemplateAnalysis {
            patterns: vec![],
            common_domains: vec!["debug".to_string(), "test".to_string()],
            frequent_topics: vec!["error".to_string(), "analysis".to_string()],
            common_variables: vec![
                "error_message".to_string(),
                "stack_trace".to_string(),
                "project_context".to_string(),
            ],
            typical_structure: crate::TemplateStructure {
                sections: vec![
                    "Context Variables".to_string(),
                    "Navigation Steps".to_string(),
                    "Interactive Questions".to_string(),
                    "Metrics".to_string(),
                ],
                avg_steps: 8,
                common_flow: vec![
                    "Analysis".to_string(),
                    "Investigation".to_string(),
                    "Resolution".to_string(),
                ],
            },
        })
    }
    
    fn get_provider_config(&self) -> &ProviderConfig {
        self.provider.config()
    }
    
    async fn is_available(&self) -> bool {
        self.provider.is_available().await
    }
}

impl AITemplateGenerator {
    fn calculate_confidence(&self, response: &str, request: &GenerationRequest) -> f32 {
        let mut score: f32 = 0.5; // Base score
        
        // Check for alias presence
        if response.contains("alias:") || response.contains("BITA-NAV") {
            score += 0.1;
        }
        
        // Check for variable substitution
        if response.contains("{{") && response.contains("}}") {
            score += 0.1;
        }
        
        // Check for navigation steps
        if response.contains("1.") || response.contains("-") {
            score += 0.1;
        }
        
        // Check for domain/topic relevance
        if response.to_lowercase().contains(&request.domain.to_lowercase()) {
            score += 0.1;
        }
        
        if response.to_lowercase().contains(&request.topic.to_lowercase()) {
            score += 0.1;
        }
        
        // Length check (reasonable template size)
        if response.len() > 500 && response.len() < 5000 {
            score += 0.1;
        }
        
        score.min(1.0)
    }
    
    fn generate_improvement_suggestions(&self, response: &str, _request: &GenerationRequest) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if !response.contains("{{") {
            suggestions.push("Consider adding variable substitution with {{variable}} syntax".to_string());
        }
        
        if !response.contains("?") {
            suggestions.push("Consider adding interactive questions for user engagement".to_string());
        }
        
        if response.lines().count() < 10 {
            suggestions.push("Template could be more detailed with additional steps".to_string());
        }
        
        if !response.contains("metric") && !response.contains("measure") {
            suggestions.push("Consider adding metrics section for effectiveness tracking".to_string());
        }
        
        suggestions
    }
}

/// Estadísticas del generador
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorStats {
    pub provider_name: String,
    pub total_generated: u32,
    pub avg_confidence: f32,
    pub most_common_domain: String,
    pub cache_status: bool,
}
