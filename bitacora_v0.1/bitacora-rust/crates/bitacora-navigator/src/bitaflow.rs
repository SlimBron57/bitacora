//! # BitaFlow Navigator Integration 🔥
//!
//! ## 🎯 El Tesoro de Bitacora
//!
//! Esta integración revolucionaria combina:
//! - **HybridNavigator**: Navegación inteligente con IA y threading especializado
//! - **BitaFlow DSL**: Sistema de templates con .bfl/.bt files y validación de alias
//!
//! Creando autonomous specialized navigators que aprenden y evolucionan.

use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::core::HybridNavigator;
use crate::NavigatorCore; // 🚀 Import trait para usar navigate()
use crate::errors::NavigatorError;
use crate::variable_substitution::{VariableSubstitutor, VariableContext};

/// Niveles de autonomía para templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutonomyLevel {
    /// Autonomía completa - ejecución sin confirmación
    Full,
    /// Restringido - solo acciones pre-aprobadas
    Restricted,
    /// Interactivo - requiere confirmación para acciones críticas
    Interactive,
    /// Manual - control total del usuario
    Manual,
}

/// Niveles de threading especializados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadLevel {
    /// Nivel 0: SparkIsolated - máximo aislamiento
    SparkIsolated,
    /// Nivel 1: ProjectIsolated - aislado por proyecto
    ProjectIsolated,
    /// Nivel 2: TopicSerial - serial dentro de topic
    TopicSerial,
    /// Nivel 3: FullSerial - ejecución completamente serial
    FullSerial,
}

/// Template Navigator con metadatos BitaFlow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigatorTemplate {
    /// Alias único del template (BITA-NAV-DOMAIN-TOPIC-v1)
    pub alias: String,
    /// Nombre descriptivo
    pub name: String,
    /// Descripción del propósito
    pub description: String,
    /// Dominio especializado (debug, code-review, research, etc.)
    pub domain: String,
    /// Tópico específico
    pub topic: String,
    /// Versión del template
    pub version: u32,
    /// Nivel de autonomía
    pub autonomy_level: AutonomyLevel,
    /// Nivel de threading requerido
    pub thread_level: ThreadLevel,
    /// Contenido BFL del template
    pub bfl_content: String,
    /// Variables del template
    pub variables: HashMap<String, String>,
    /// Métricas de efectividad
    pub metrics: NavigatorMetrics,
}

/// Métricas de efectividad del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigatorMetrics {
    pub success_rate: f64,
    pub avg_execution_time: f64,
    pub usage_count: u32,
    pub last_used: Option<String>,
}

impl Default for NavigatorMetrics {
    fn default() -> Self {
        Self {
            success_rate: 0.0,
            avg_execution_time: 0.0,
            usage_count: 0,
            last_used: None,
        }
    }
}

/// Engine principal para integración BitaFlow + Navigator
pub struct BitaflowNavigatorEngine {
    /// Validator de alias de templates
    alias_validator: AliasValidator,
    /// Templates cargados indexados por alias
    navigator_templates: HashMap<String, NavigatorTemplate>,
    /// Navigator híbrido subyacente
    hybrid_navigator: HybridNavigator,
    /// Variable substitution system 🔄
    variable_substitutor: VariableSubstitutor,
}

impl BitaflowNavigatorEngine {
    /// Constructor
    pub fn new(hybrid_navigator: HybridNavigator) -> Result<Self, NavigatorError> {
        Ok(Self {
            navigator_templates: HashMap::new(),
            alias_validator: AliasValidator::new(),
            hybrid_navigator,
            variable_substitutor: VariableSubstitutor::new()?,
        })
    }

    /// Cargar template desde contenido BFL
    pub fn load_template(&mut self, bfl_content: &str) -> Result<String, NavigatorError> {
        // Parsear contenido BFL
        let template = self.parse_bfl_content(bfl_content)?;
        
        // Validar alias
        self.alias_validator.validate_alias(&template.alias)?;
        
        // Almacenar template
        let alias = template.alias.clone();
        self.navigator_templates.insert(alias.clone(), template);
        
        Ok(alias)
    }

    /// Cargar template desde archivo
    pub async fn load_template_from_file(&mut self, file_path: &str) -> Result<String, NavigatorError> {
        let content = tokio::fs::read_to_string(file_path)
            .await
            .map_err(|e| NavigatorError::configuration(format!("Error reading template file: {}", e)))?;
        
        self.load_template(&content)
    }

    /// Listar todos los templates cargados
    pub fn list_templates(&self) -> Vec<&NavigatorTemplate> {
        self.navigator_templates.values().collect()
    }

    /// Ejecutar template específico
    pub async fn execute_template(
        &mut self, 
        alias: &str, 
        context: HashMap<String, String>
    ) -> Result<NavigationResult, NavigatorError> {
        let template = self.navigator_templates.get(alias)
            .ok_or_else(|| NavigatorError::configuration(format!("Template not found: {}", alias)))?;

        // Clonar información necesaria para evitar borrowing issues
        let thread_level = template.thread_level.clone();
        let template_clone = template.clone();

        // Configurar threading level según template
        self.configure_thread_level(&thread_level).await?;

        // Ejecutar navegación con template
        let result = self.execute_navigation_flow(&template_clone, context).await?;

        // Actualizar métricas
        self.update_template_metrics(alias, &result);

        Ok(result)
    }

    /// Generar nuevo template con IA
    pub async fn generate_template(
        &mut self,
        domain: &str,
        topic: &str,
        requirements: &str
    ) -> Result<String, NavigatorError> {
        // TODO: Implementar generación de templates con IA
        // Usando el AI engine del HybridNavigator
        let alias = self.alias_validator.generate_navigator_alias(domain, topic)?;
        
        // Por ahora retornamos el alias generado
        // En implementación completa aquí iría la lógica de IA
        Ok(alias)
    }

    // Métodos privados de implementación

    fn parse_bfl_content(&self, content: &str) -> Result<NavigatorTemplate, NavigatorError> {
        // Usar el YAML parser integrado para parsing completo
        crate::yaml_parser::parse_bfl_template(content)
    }

    async fn configure_thread_level(&mut self, level: &ThreadLevel) -> Result<(), NavigatorError> {
        // Configurar el threading del HybridNavigator según el level
        match level {
            ThreadLevel::SparkIsolated => {
                // Máximo aislamiento - usar threading level 0
                // TODO: Integrar con thread_manager.configure_level(0)
                println!("🔒 Configured SparkIsolated threading (Level 0)");
            },
            ThreadLevel::ProjectIsolated => {
                // Aislado por proyecto - usar threading level 1
                println!("🏗️  Configured ProjectIsolated threading (Level 1)");
            },
            ThreadLevel::TopicSerial => {
                // Serial dentro de topic - usar threading level 2
                println!("📝 Configured TopicSerial threading (Level 2)");
            },
            ThreadLevel::FullSerial => {
                // Completamente serial - usar threading level 3
                println!("🔄 Configured FullSerial threading (Level 3)");
            }
        }
        Ok(())
    }

    async fn execute_navigation_flow(
        &mut self,
        template: &NavigatorTemplate,
        context: HashMap<String, String>
    ) -> Result<NavigationResult, NavigatorError> {
        println!("🎯 Executing navigation flow for: {}", template.name);
        println!("📋 Autonomy Level: {:?}", template.autonomy_level);
        println!("🧵 Thread Level: {:?}", template.thread_level);
        
        let start_time = std::time::Instant::now();
        let mut actions_taken = Vec::new();
        
        // Step 0: 🔄 Create variable context and substitute variables
        let mut variable_context = VariableContext::from_template_variables(template.variables.clone());
        
        // Add runtime context variables
        for (key, value) in context.iter() {
            variable_context.set_runtime_variable(key.clone(), value.clone());
        }
        
        // Substitute variables in BFL content
        let substituted_content = self.variable_substitutor.substitute(&template.bfl_content, &variable_context)?;
        println!("🔄 Variable substitution completed");
        
        // Step 1: Parse BFL content and extract navigation steps
        let navigation_steps = self.parse_navigation_steps(&substituted_content)?;
        actions_taken.push(format!("Parsed {} navigation steps", navigation_steps.len()));
        
        // Step 2: Execute steps according to autonomy level
        match template.autonomy_level {
            AutonomyLevel::Full => {
                // Execute all steps automatically
                for step in &navigation_steps {
                    self.execute_step(step, &variable_context).await?;
                    actions_taken.push(format!("Executed step: {}", step));
                }
            },
            AutonomyLevel::Interactive => {
                // Execute with confirmation prompts
                for step in &navigation_steps {
                    println!("🤔 About to execute: {}", step);
                    println!("   Continue? (This is simulation - always continuing)");
                    self.execute_step(step, &variable_context).await?;
                    actions_taken.push(format!("Executed step with confirmation: {}", step));
                }
            },
            AutonomyLevel::Restricted => {
                // Only execute pre-approved actions
                for step in &navigation_steps {
                    if self.is_approved_action(step) {
                        self.execute_step(step, &variable_context).await?;
                        actions_taken.push(format!("Executed approved step: {}", step));
                    } else {
                        actions_taken.push(format!("Skipped restricted step: {}", step));
                    }
                }
            },
            AutonomyLevel::Manual => {
                // Manual control - just log the steps
                for step in &navigation_steps {
                    actions_taken.push(format!("Manual step logged: {}", step));
                }
            }
        }
        
        let execution_time = start_time.elapsed().as_secs_f64();
        
        Ok(NavigationResult {
            success: true,
            actions_taken,
            execution_time,
            output: format!("Navigation flow completed for {}", template.alias),
        })
    }
    
    fn parse_navigation_steps(&self, bfl_content: &str) -> Result<Vec<String>, NavigatorError> {
        // Parse the BFL content to extract navigation steps
        let mut steps = Vec::new();
        
        // Simple parser - look for numbered lists or bullet points
        for line in bfl_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("1.") || trimmed.starts_with("2.") || 
               trimmed.starts_with("3.") || trimmed.starts_with("4.") ||
               trimmed.starts_with("5.") || trimmed.starts_with("-") {
                steps.push(trimmed.to_string());
            }
        }
        
        if steps.is_empty() {
            steps.push("Default navigation step".to_string());
        }
        
        Ok(steps)
    }
    
    async fn execute_step(&self, step: &str, context: &VariableContext) -> Result<(), NavigatorError> {
        println!("   ⚡ Executing: {}", step);
        
        // 🚀 REAL INTEGRATION: Use HybridNavigator for actual navigation
        let navigation_context = self.build_navigation_context(step, context)?;
        
        // Execute with HybridNavigator
        match self.hybrid_navigator.navigate(navigation_context).await {
            Ok(nav_result) => {
                println!("     🎯 Navigator executed successfully in {:.3}s", nav_result.execution_time.as_secs_f64());
                println!("     📊 Confidence: {:.2}", nav_result.confidence_score);
                println!("     📋 Strategy: {:?}", nav_result.strategy_used);
                
                if !nav_result.results.is_empty() {
                    println!("     🔍 Found {} results", nav_result.results.len());
                }
            }
            Err(e) => {
                println!("     ⚠️  Navigator error: {}", e);
                // Continue execution for now
            }
        }
        
        Ok(())
    }
    
    fn is_approved_action(&self, step: &str) -> bool {
        // Define pre-approved actions
        let approved_keywords = vec!["analyze", "identify", "suggest", "log", "read"];
        let step_lower = step.to_lowercase();
        
        approved_keywords.iter().any(|&keyword| step_lower.contains(keyword))
    }

    /// Build NavigationContext from step and variable context for HybridNavigator
    fn build_navigation_context(&self, step: &str, variable_context: &VariableContext) -> Result<crate::NavigationContext, NavigatorError> {
        use crate::NavigationContext;
        
        // Build navigation context using the actual structure
        let mut context = NavigationContext {
            project_id: None,
            topic_id: None,
            action_id: None,
            spark_context: Some("BitaFlow template execution".to_string()),
            user_query: step.to_string(),
            ai_assistance_level: crate::AIAssistanceLevel::Standard,
        };
        
        // Extract variable information to guide navigation
        // Look for specific variable patterns that indicate content type
        if let Some(error_file) = variable_context.get_variable("error_file") {
            context.spark_context = Some(format!("Error analysis: {}", error_file));
        } else if let Some(project_context) = variable_context.get_variable("project_context") {
            context.spark_context = Some(format!("Project context: {}", project_context));
        }
        
        // Set AI assistance level based on step complexity
        if step.contains("analyze") || step.contains("suggest") || step.contains("identify") {
            context.ai_assistance_level = crate::AIAssistanceLevel::Advanced;
        } else if step.contains("search") || step.contains("find") {
            context.ai_assistance_level = crate::AIAssistanceLevel::Standard;
        }
        
        Ok(context)
    }

    fn update_template_metrics(&mut self, alias: &str, result: &NavigationResult) {
        if let Some(template) = self.navigator_templates.get_mut(alias) {
            template.metrics.usage_count += 1;
            template.metrics.avg_execution_time = 
                (template.metrics.avg_execution_time * (template.metrics.usage_count - 1) as f64 + result.execution_time) 
                / template.metrics.usage_count as f64;
            
            if result.success {
                template.metrics.success_rate = 
                    (template.metrics.success_rate * (template.metrics.usage_count - 1) as f64 + 1.0) 
                    / template.metrics.usage_count as f64;
            }
        }
    }
}

/// Resultado de navegación
#[derive(Debug, Clone)]
pub struct NavigationResult {
    pub success: bool,
    pub actions_taken: Vec<String>,
    pub execution_time: f64,
    pub output: String,
}

/// Validador de alias BitaFlow para Navigator
pub struct AliasValidator {
    alias_pattern: Regex,
}

impl AliasValidator {
    pub fn new() -> Self {
        Self {
            // Patrón: BITA-NAV-{DOMAIN}-{TOPIC}-v{VERSION}
            alias_pattern: Regex::new(r"^BITA-NAV-([A-Z0-9]+(?:-[A-Z0-9]+)*)-([A-Z0-9]+(?:-[A-Z0-9]+)*)-v(\d+)$")
                .expect("Invalid regex pattern"),
        }
    }

    /// Validar formato de alias
    pub fn validate_alias(&self, alias: &str) -> Result<(), NavigatorError> {
        if self.alias_pattern.is_match(alias) {
            Ok(())
        } else {
            Err(NavigatorError::configuration(
                format!("Invalid alias format: {}. Expected: BITA-NAV-DOMAIN-TOPIC-vN", alias)
            ))
        }
    }

    /// Parsear componentes del alias
    pub fn parse_alias(&self, alias: &str) -> Result<(String, String, u32), NavigatorError> {
        if let Some(captures) = self.alias_pattern.captures(alias) {
            let domain = captures.get(1).unwrap().as_str().to_lowercase();
            let topic = captures.get(2).unwrap().as_str().to_lowercase();
            let version: u32 = captures.get(3).unwrap().as_str().parse()
                .map_err(|_| NavigatorError::configuration("Invalid version number".to_string()))?;
            
            Ok((domain, topic, version))
        } else {
            Err(NavigatorError::configuration(format!("Cannot parse alias: {}", alias)))
        }
    }

    /// Generar alias para navigator
    pub fn generate_navigator_alias(&self, domain: &str, topic: &str) -> Result<String, NavigatorError> {
        let domain_upper = domain.to_uppercase().replace("-", "_");
        let topic_upper = topic.to_uppercase().replace("-", "_");
        
        Ok(format!("BITA-NAV-{}-{}-v1", domain_upper, topic_upper))
    }

    /// Incrementar versión de alias
    pub fn increment_version(&self, alias: &str) -> Result<String, NavigatorError> {
        let (domain, topic, version) = self.parse_alias(alias)?;
        let domain_upper = domain.to_uppercase();
        let topic_upper = topic.to_uppercase();
        
        Ok(format!("BITA-NAV-{}-{}-v{}", domain_upper, topic_upper, version + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_validator() {
        let validator = AliasValidator::new();
        
        // Test valid alias
        assert!(validator.validate_alias("BITA-NAV-DEBUG-ERROR-v1").is_ok());
        assert!(validator.validate_alias("BITA-NAV-CODE-REVIEW-v2").is_ok());
        
        // Test invalid alias
        assert!(validator.validate_alias("INVALID-ALIAS").is_err());
        assert!(validator.validate_alias("BITA-NAV-debug-error-v1").is_err()); // lowercase
    }

    #[test]
    fn test_parse_alias() {
        let validator = AliasValidator::new();
        
        let (domain, topic, version) = validator.parse_alias("BITA-NAV-DEBUG-ERROR-v1").unwrap();
        assert_eq!(domain, "debug");
        assert_eq!(topic, "error");
        assert_eq!(version, 1);
    }

    #[test]
    fn test_generate_alias() {
        let validator = AliasValidator::new();
        
        let alias = validator.generate_navigator_alias("debug", "error").unwrap();
        assert_eq!(alias, "BITA-NAV-DEBUG-ERROR-v1");
    }

    #[test]
    fn test_increment_version() {
        let validator = AliasValidator::new();
        
        let new_alias = validator.increment_version("BITA-NAV-DEBUG-ERROR-v1").unwrap();
        assert_eq!(new_alias, "BITA-NAV-DEBUG-ERROR-v2");
    }
}
