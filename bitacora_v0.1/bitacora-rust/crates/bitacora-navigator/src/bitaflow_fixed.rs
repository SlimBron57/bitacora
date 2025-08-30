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
use crate::errors::NavigatorError;

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

/// Motor de integración BitaFlow + Navigator
pub struct BitaflowNavigatorEngine {
    /// Templates de navegación cargados
    navigator_templates: HashMap<String, NavigatorTemplate>,
    /// Validador de alias
    alias_validator: AliasValidator,
    /// Instancia del HybridNavigator
    hybrid_navigator: HybridNavigator,
}

impl BitaflowNavigatorEngine {
    /// Constructor
    pub fn new(hybrid_navigator: HybridNavigator) -> Self {
        Self {
            navigator_templates: HashMap::new(),
            alias_validator: AliasValidator::new(),
            hybrid_navigator,
        }
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
            .map_err(|e| NavigatorError::ConfigError(format!("Error reading template file: {}", e)))?;
        
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
            .ok_or_else(|| NavigatorError::ConfigError(format!("Template not found: {}", alias)))?;

        // Configurar threading level según template
        self.configure_thread_level(&template.thread_level).await?;

        // Ejecutar navegación con template
        let result = self.execute_navigation_flow(template, context).await?;

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
        // Parsear YAML front-matter
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(NavigatorError::ConfigError("Invalid BFL format".to_string()));
        }

        // Parsear metadatos YAML
        let yaml_content = parts[1];
        let bfl_body = parts[2];

        // TODO: Implementar parsing completo del YAML
        // Por ahora creamos un template básico
        let alias = "BITA-NAV-DEBUG-ERROR-v1".to_string(); // Extraer del YAML real
        
        Ok(NavigatorTemplate {
            alias: alias.clone(),
            name: "Debug Error Navigator".to_string(),
            description: "Autonomous error debugging and resolution".to_string(),
            domain: "debug".to_string(),
            topic: "error".to_string(),
            version: 1,
            autonomy_level: AutonomyLevel::Interactive,
            thread_level: ThreadLevel::ProjectIsolated,
            bfl_content: bfl_body.to_string(),
            variables: HashMap::new(),
            metrics: NavigatorMetrics::default(),
        })
    }

    async fn configure_thread_level(&mut self, level: &ThreadLevel) -> Result<(), NavigatorError> {
        // TODO: Configurar el threading del HybridNavigator según el level
        Ok(())
    }

    async fn execute_navigation_flow(
        &mut self,
        template: &NavigatorTemplate,
        context: HashMap<String, String>
    ) -> Result<NavigationResult, NavigatorError> {
        // TODO: Implementar ejecución del flujo de navegación
        // Interpretar el BFL content y ejecutar con HybridNavigator
        Ok(NavigationResult {
            success: true,
            actions_taken: vec!["template_loaded".to_string()],
            execution_time: 0.0,
            output: "Template executed successfully".to_string(),
        })
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
            Err(NavigatorError::ConfigError(
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
                .map_err(|_| NavigatorError::ConfigError("Invalid version number".to_string()))?;
            
            Ok((domain, topic, version))
        } else {
            Err(NavigatorError::ConfigError(format!("Cannot parse alias: {}", alias)))
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
