//! Template Engine - Carga y aplica templates MTT-DSL para guiar documentación
//!
//! Este módulo es el corazón del "Maestro de Ceremonias":
//! - Carga templates YAML (MTT-DSL format)
//! - Presenta prompts al LLM/usuario
//! - Guía el proceso de documentación paso a paso
//!
//! # Convención de Directorios
//!
//! - **`templates/dev/doc/`**: Templates para documentación de código (ej: code_documentation.yaml)
//! - **`templates/dev/code/`**: Código comprimido (.qpxf) generado por `bstradivarius compress`
//! - **`templates/mtt/`**: Templates MTT-DSL generales (fallback)
//!
//! El engine busca templates en `templates/dev/doc/` primero, luego en `templates/mtt/`.
//!
//! # Filosofía
//!
//! Los templates no son solo estructuras - son **guías conversacionales**.
//! Cada template tiene una personalidad, un tono, un estilo.
//! Usa narrativa motivacional para que los LLMs generen mejor documentación.

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

/// Template MTT-DSL completo
#[derive(Debug, Clone, Deserialize)]
pub struct MTTTemplate {
    pub name: String,
    pub category: String,
    pub version: String,
    pub description: String,
    
    #[serde(default)]
    pub trigger_patterns: Vec<String>,
    
    pub personality: Personality,
    pub structure: HashMap<String, TemplateSection>,
    
    #[serde(default)]
    pub validation: Option<Validation>,
    
    #[serde(default)]
    pub output_format: Option<OutputFormat>,
}

/// Personalidad del template (tono, estilo narrativo)
#[derive(Debug, Clone, Deserialize)]
pub struct Personality {
    pub tone: String,
    pub depth: String,
    pub style: String,
    
    #[serde(default)]
    pub approach: Option<String>,
}

/// Sección del template con prompts y outputs esperados
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateSection {
    pub name: String,
    pub description: String,
    pub prompts: Vec<TemplatePrompt>,
    pub outputs: Vec<String>,
}

/// Prompt individual con guía contextual
#[derive(Debug, Clone, Deserialize)]
pub struct TemplatePrompt {
    pub question: String,
    pub guidance: String,
}

/// Validación de completitud del documento generado
#[derive(Debug, Clone, Deserialize)]
pub struct Validation {
    #[serde(default)]
    pub completeness_check: Vec<String>,
    
    #[serde(default)]
    pub quality_check: Vec<String>,
}

/// Formato de output esperado
#[derive(Debug, Clone, Deserialize)]
pub struct OutputFormat {
    pub template: String,
}

/// Motor de templates que carga y procesa templates MTT-DSL
pub struct TemplateEngine {
    templates_dir: PathBuf,
    loaded_templates: HashMap<String, MTTTemplate>,
}

impl TemplateEngine {
    /// Crear nuevo motor de templates
    pub fn new(templates_dir: PathBuf) -> Result<Self> {
        let mut engine = Self {
            templates_dir,
            loaded_templates: HashMap::new(),
        };
        
        engine.load_all_templates()?;
        Ok(engine)
    }
    
    /// Cargar todos los templates desde templates/dev/doc/*.yaml
    /// 
    /// **Convención de directorios**:
    /// - `templates/dev/doc/`: Templates para documentación de código (ej: code_documentation.yaml)
    /// - `templates/dev/code/`: Archivos de código comprimido (.qpxf)
    /// - `templates/mtt/`: Templates MTT-DSL generales (otros casos de uso)
    fn load_all_templates(&mut self) -> Result<()> {
        // Buscar templates en templates/dev/doc/ (prioridad para desarrollo)
        let dev_doc_dir = self.templates_dir.join("dev").join("doc");
        
        if dev_doc_dir.exists() {
            self.load_templates_from_dir(&dev_doc_dir)?;
        }
        
        // Fallback: buscar en templates/mtt/ (compatibilidad con templates antiguos)
        let mtt_dir = self.templates_dir.join("mtt");
        if mtt_dir.exists() {
            self.load_templates_from_dir(&mtt_dir)?;
        }
        
        if self.loaded_templates.is_empty() {
            return Err(anyhow::anyhow!(
                "No templates found in templates/dev/doc/ or templates/mtt/"
            ));
        }
        
        Ok(())
    }
    
    /// Cargar templates desde un directorio específico
    fn load_templates_from_dir(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir)
            .context(format!("Failed to read directory: {:?}", dir))?
        {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
                let content = fs::read_to_string(&path)
                    .context(format!("Failed to read template: {:?}", path))?;
                    
                let template: MTTTemplate = serde_yaml::from_str(&content)
                    .context(format!("Failed to parse template: {:?}", path))?;
                
                self.loaded_templates.insert(template.name.clone(), template);
            }
        }
        
        Ok(())
    }
    
    /// Obtener template por nombre
    pub fn get_template(&self, name: &str) -> Option<&MTTTemplate> {
        self.loaded_templates.get(name)
    }
    
    /// Buscar template por trigger pattern
    pub fn find_template_by_trigger(&self, query: &str) -> Option<&MTTTemplate> {
        let query_lower = query.to_lowercase();
        
        self.loaded_templates.values()
            .find(|t| {
                t.trigger_patterns.iter()
                    .any(|p| query_lower.contains(&p.to_lowercase()))
            })
    }
    
    /// Listar todos los templates disponibles
    pub fn list_templates(&self) -> Vec<String> {
        self.loaded_templates.keys().cloned().collect()
    }
    
    /// Generar documento guiado por template
    /// 
    /// Retorna markdown con prompts estructurados para que el LLM/usuario complete
    pub fn generate_documentation_guide(
        &self,
        template_name: &str,
        context: &DocumentationContext,
    ) -> Result<String> {
        let template = self.get_template(template_name)
            .context(format!("Template not found: {}", template_name))?;
        
        let mut output = String::new();
        
        // Header con metadata
        output.push_str(&format!("# Documentation: {}\n\n", context.module_name));
        output.push_str(&format!("> **Template**: {}\n", template.name));
        output.push_str(&format!("> **Version**: {}\n", template.version));
        output.push_str(&format!("> **File**: {}\n\n", context.file_path.display()));
        
        // Descripción del template
        output.push_str(&format!("{}\n\n", template.description));
        
        output.push_str("---\n\n");
        
        // Generar secciones en orden
        let section_order = vec![
            "module_purpose",
            "architecture_overview",
            "dependencies",
            "design_decisions",
            "implementation_details",
            "testing_strategy",
            "git_traceability",
            "future_work",
        ];
        
        for section_key in section_order {
            if let Some(section) = template.structure.get(section_key) {
                output.push_str(&format!("## {}\n\n", section.name));
                output.push_str(&format!("*{}*\n\n", section.description));
                
                // Prompts de la sección
                for (i, prompt) in section.prompts.iter().enumerate() {
                    output.push_str(&format!("### {} Pregunta {}\n\n", 
                        if i == 0 { "📝" } else { "  " }, i + 1));
                    output.push_str(&format!("**{}**\n\n", prompt.question));
                    
                    // Guía contextual
                    if !prompt.guidance.is_empty() {
                        output.push_str(&format!("*Guía*: {}\n\n", prompt.guidance));
                    }
                    
                    // Espacio para respuesta
                    output.push_str("**Respuesta**:\n\n");
                    output.push_str("```\n[TODO - Completar aquí]\n```\n\n");
                }
                
                // Outputs esperados
                output.push_str(&format!("**Outputs esperados**: {}\n\n", 
                    section.outputs.join(", ")));
                
                output.push_str("---\n\n");
            }
        }
        
        // Validación
        if let Some(validation) = &template.validation {
            output.push_str("## ✅ Checklist de Validación\n\n");
            
            if !validation.completeness_check.is_empty() {
                output.push_str("### Completitud\n\n");
                for check in &validation.completeness_check {
                    output.push_str(&format!("- [ ] {}\n", check));
                }
                output.push_str("\n");
            }
            
            if !validation.quality_check.is_empty() {
                output.push_str("### Calidad\n\n");
                for check in &validation.quality_check {
                    output.push_str(&format!("- [ ] {}\n", check));
                }
                output.push_str("\n");
            }
        }
        
        Ok(output)
    }
}

/// Contexto para generar documentación
#[derive(Debug, Clone)]
pub struct DocumentationContext {
    pub file_path: PathBuf,
    pub module_name: String,
    pub content: String,
    
    #[allow(dead_code)]
    pub git_commits: Vec<String>,
}

impl DocumentationContext {
    pub fn new(file_path: PathBuf, content: String) -> Self {
        let module_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Self {
            file_path,
            module_name,
            content,
            git_commits: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_load_templates() {
        let root = env::current_dir().unwrap();
        let templates_dir = root.join("templates");
        
        if !templates_dir.exists() {
            println!("Skipping test: templates/ directory not found");
            return;
        }
        
        let engine = TemplateEngine::new(templates_dir);
        assert!(engine.is_ok(), "Failed to load templates");
        
        let engine = engine.unwrap();
        let templates = engine.list_templates();
        assert!(!templates.is_empty(), "No templates loaded");
    }
    
    #[test]
    fn test_generate_guide() {
        let root = env::current_dir().unwrap();
        let templates_dir = root.join("templates");
        
        if !templates_dir.exists() {
            println!("Skipping test: templates/ directory not found");
            return;
        }
        
        let engine = TemplateEngine::new(templates_dir).unwrap();
        
        let context = DocumentationContext::new(
            PathBuf::from("src/fbcu/mod.rs"),
            "// test content".to_string(),
        );
        
        if let Some(_) = engine.get_template("code_documentation") {
            let guide = engine.generate_documentation_guide("code_documentation", &context);
            assert!(guide.is_ok(), "Failed to generate guide");
            
            let guide = guide.unwrap();
            assert!(guide.contains("Documentation:"));
            assert!(guide.contains("Template:"));
            assert!(guide.contains("Pregunta"));
        }
    }
}
