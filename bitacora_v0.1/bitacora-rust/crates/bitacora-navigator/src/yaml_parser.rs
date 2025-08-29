//! # YAML Front-matter Parser 📝
//!
//! Parser especializado para extraer metadatos YAML de templates BFL
//! Integrado completamente con el sistema BitaFlow Navigator

use std::collections::HashMap;
use serde_yaml;
use regex::Regex;
use crate::bitaflow::{NavigatorTemplate, AutonomyLevel, ThreadLevel, NavigatorMetrics};
use crate::errors::NavigatorError;

/// Parser mejorado para YAML front-matter de templates BFL
pub fn parse_yaml_frontmatter(yaml_content: &str) -> Result<NavigatorTemplate, NavigatorError> {
    // Parse the YAML content
    let yaml_data: serde_yaml::Value = serde_yaml::from_str(yaml_content)
        .map_err(|e| NavigatorError::configuration(format!("Invalid YAML format: {}", e)))?;
    
    // Extract fields from YAML
    let name = yaml_data.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("Unnamed Template")
        .to_string();
    
    let alias = yaml_data.get("alias")
        .and_then(|v| v.as_str())
        .unwrap_or("BITA-NAV-UNKNOWN-TEMPLATE-v1")
        .to_string();
    
    let description = yaml_data.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("No description provided")
        .to_string();
    
    let domain = yaml_data.get("domain")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    
    let topic = yaml_data.get("topic")
        .and_then(|v| v.as_str())
        .unwrap_or("general")
        .to_string();
    
    let version = yaml_data.get("version")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as u32;
    
    // Parse autonomy level
    let autonomy_level = match yaml_data.get("autonomy_level").and_then(|v| v.as_str()) {
        Some("Full") => AutonomyLevel::Full,
        Some("Restricted") => AutonomyLevel::Restricted,
        Some("Interactive") => AutonomyLevel::Interactive,
        Some("Manual") => AutonomyLevel::Manual,
        _ => AutonomyLevel::Interactive, // Default
    };
    
    // Parse thread level
    let thread_level = match yaml_data.get("thread_level").and_then(|v| v.as_str()) {
        Some("SparkIsolated") => ThreadLevel::SparkIsolated,
        Some("ProjectIsolated") => ThreadLevel::ProjectIsolated,
        Some("TopicSerial") => ThreadLevel::TopicSerial,
        Some("FullSerial") => ThreadLevel::FullSerial,
        _ => ThreadLevel::ProjectIsolated, // Default
    };
    
    // Parse variables if present
    let mut variables = HashMap::new();
    if let Some(vars) = yaml_data.get("variables").and_then(|v| v.as_mapping()) {
        for (key, value) in vars {
            if let (Some(k), Some(v)) = (key.as_str(), value.as_str()) {
                variables.insert(k.to_string(), v.to_string());
            }
        }
    }
    
    Ok(NavigatorTemplate {
        alias,
        name,
        description,
        domain,
        topic,
        version,
        autonomy_level,
        thread_level,
        bfl_content: String::new(), // Will be set later
        variables,
        metrics: NavigatorMetrics::default(),
    })
}

/// Función auxiliar para parsear template BFL completo con YAML front-matter
pub fn parse_bfl_template(content: &str) -> Result<NavigatorTemplate, NavigatorError> {
    // Buscar delimitadores YAML (---)
    let yaml_regex = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$")
        .map_err(|e| NavigatorError::configuration(format!("Regex error: {}", e)))?;
    
    if let Some(captures) = yaml_regex.captures(content) {
        let yaml_content = captures.get(1).map(|m| m.as_str()).unwrap_or("");
        let bfl_content = captures.get(2).map(|m| m.as_str()).unwrap_or("").to_string();
        
        let mut template = parse_yaml_frontmatter(yaml_content)?;
        template.bfl_content = bfl_content;
        
        Ok(template)
    } else {
        // Si no hay YAML front-matter, crear template básico
        Ok(NavigatorTemplate {
            alias: "BITA-NAV-UNKNOWN-TEMPLATE-v1".to_string(),
            name: "Unnamed Template".to_string(),
            description: "Template without YAML front-matter".to_string(),
            domain: "unknown".to_string(),
            topic: "general".to_string(),
            version: 1,
            autonomy_level: AutonomyLevel::Interactive,
            thread_level: ThreadLevel::ProjectIsolated,
            bfl_content: content.to_string(),
            variables: HashMap::new(),
            metrics: NavigatorMetrics::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_parser_basic() {
        let yaml = r#"
name: Debug Error Navigator
alias: BITA-NAV-DEBUG-ERROR-TRACE-v1
description: Specialized navigator for debugging and error tracing
domain: debug
topic: error-trace
version: 1
autonomy_level: Interactive
thread_level: ProjectIsolated
variables:
  error_type: "runtime"
  severity: "high"
"#;
        
        let result = parse_yaml_frontmatter(yaml);
        assert!(result.is_ok());
        
        let template = result.unwrap();
        assert_eq!(template.name, "Debug Error Navigator");
        assert_eq!(template.domain, "debug");
        assert_eq!(template.version, 1);
        assert!(matches!(template.autonomy_level, AutonomyLevel::Interactive));
        assert_eq!(template.variables.len(), 2);
    }

    #[test] 
    fn test_bfl_template_with_frontmatter() {
        let content = r#"---
name: Test Template
alias: BITA-NAV-TEST-SAMPLE-v1
description: Test template with YAML
domain: test
topic: sample
version: 1
autonomy_level: Manual
thread_level: SparkIsolated
---
# Test BFL Content
This is the BFL content of the template.
"#;

        let result = parse_bfl_template(content);
        assert!(result.is_ok());
        
        let template = result.unwrap();
        assert_eq!(template.name, "Test Template");
        assert!(template.bfl_content.contains("Test BFL Content"));
        assert!(matches!(template.autonomy_level, AutonomyLevel::Manual));
        assert!(matches!(template.thread_level, ThreadLevel::SparkIsolated));
    }

    #[test]
    fn test_bfl_template_without_frontmatter() {
        let content = "# Simple BFL Template\nThis is just BFL content.";
        
        let result = parse_bfl_template(content);
        assert!(result.is_ok());
        
        let template = result.unwrap();
        assert_eq!(template.name, "Unnamed Template");
        assert!(template.bfl_content.contains("Simple BFL Template"));
    }
}
