//! Template engine implementations

use async_trait::async_trait;
use std::collections::HashMap;
use crate::{Template, TemplateResult, TemplateError, RenderedTemplate, RenderMetadata};

/// Template engine trait
#[async_trait]
pub trait TemplateEngine {
    /// Engine name
    fn name(&self) -> &'static str;
    
    /// Render template with variables
    async fn render(
        &self,
        template: &Template,
        variables: &HashMap<String, serde_json::Value>,
    ) -> TemplateResult<RenderedTemplate>;
    
    /// Check if engine can handle template
    fn can_handle(&self, template: &Template) -> bool;
    
    /// Validate template syntax
    fn validate_syntax(&self, template_content: &str) -> TemplateResult<()>;
}

/// Handlebars template engine
#[cfg(feature = "handlebars")]
pub struct HandlebarsEngine {
    handlebars: handlebars::Handlebars<'static>,
}

#[cfg(feature = "handlebars")]
impl HandlebarsEngine {
    pub fn new() -> TemplateResult<Self> {
        let mut handlebars = handlebars::Handlebars::new();
        
        // Register helpers
        handlebars.register_helper("uppercase", Box::new(uppercase_helper));
        handlebars.register_helper("format_date", Box::new(format_date_helper));
        handlebars.register_helper("truncate", Box::new(truncate_helper));
        
        Ok(Self { handlebars })
    }

    /// Register template sections
    fn register_template_sections(&mut self, template: &Template) -> TemplateResult<()> {
        for section in &template.structure.sections {
            self.handlebars
                .register_template_string(&section.section_id, &section.template)
                .map_err(|e| TemplateError::EngineError {
                    engine: "handlebars".to_string(),
                    reason: e.to_string(),
                })?;
        }
        Ok(())
    }
}

#[cfg(feature = "handlebars")]
#[async_trait]
impl TemplateEngine for HandlebarsEngine {
    fn name(&self) -> &'static str {
        "handlebars"
    }

    async fn render(
        &self,
        template: &Template,
        variables: &HashMap<String, serde_json::Value>,
    ) -> TemplateResult<RenderedTemplate> {
        let start_time = std::time::Instant::now();
        let mut rendered_sections = Vec::new();
        let mut warnings = Vec::new();
        let mut full_content = String::new();

        // Clone handlebars to avoid mutable reference issues
        let mut engine = self.handlebars.clone();
        
        // Register template sections
        for section in &template.structure.sections {
            engine
                .register_template_string(&section.section_id, &section.template)
                .map_err(|e| TemplateError::EngineError {
                    engine: "handlebars".to_string(),
                    reason: e.to_string(),
                })?;
        }

        // Render each section
        for section in &template.structure.sections {
            // Check if section should be rendered
            if !section.should_render(variables) {
                warnings.push(format!("Skipped section '{}' due to unmet conditions", section.section_id));
                continue;
            }

            let rendered_section = engine
                .render(&section.section_id, variables)
                .map_err(|e| TemplateError::RenderError {
                    reason: format!("Failed to render section '{}': {}", section.section_id, e),
                })?;

            rendered_sections.push(section.section_id.clone());
            full_content.push_str(&rendered_section);
            
            // Add section separator if not the last section
            if let Some(separator) = &template.structure.section_separator {
                if section != template.structure.sections.last().unwrap() {
                    full_content.push_str(separator);
                }
            }
        }

        // Add footer if present
        if let Some(footer) = &template.structure.footer {
            full_content.push_str(footer);
        }

        let render_time = start_time.elapsed().as_millis() as u64;

        Ok(RenderedTemplate {
            template_id: template.template_id.clone(),
            content: full_content,
            variables: variables.clone(),
            render_metadata: RenderMetadata {
                rendered_at: chrono::Utc::now(),
                render_time_ms: render_time,
                engine_used: self.name().to_string(),
                sections_rendered: rendered_sections,
                warnings,
            },
        })
    }

    fn can_handle(&self, _template: &Template) -> bool {
        // Handlebars can handle most templates
        true
    }

    fn validate_syntax(&self, template_content: &str) -> TemplateResult<()> {
        // Try to compile the template
        handlebars::Handlebars::new()
            .register_template_string("test", template_content)
            .map_err(|e| TemplateError::InvalidFormat {
                reason: format!("Invalid handlebars syntax: {}", e),
            })?;
        Ok(())
    }
}

// Handlebars helpers
#[cfg(feature = "handlebars")]
fn uppercase_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&value.to_uppercase())?;
        }
    }
    Ok(())
}

#[cfg(feature = "handlebars")]
fn format_date_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(date_str) = param.value().as_str() {
            // Try to parse and format date
            if let Ok(datetime) = chrono::DateTime::parse_from_rfc3339(date_str) {
                out.write(&datetime.format("%Y-%m-%d %H:%M").to_string())?;
            } else {
                out.write(date_str)?;
            }
        }
    }
    Ok(())
}

#[cfg(feature = "handlebars")]
fn truncate_helper(
    h: &handlebars::Helper,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    if let (Some(text_param), Some(length_param)) = (h.param(0), h.param(1)) {
        if let (Some(text), Some(length)) = (text_param.value().as_str(), length_param.value().as_u64()) {
            if text.len() > length as usize {
                out.write(&format!("{}...", &text[..length as usize]))?;
            } else {
                out.write(text)?;
            }
        }
    }
    Ok(())
}

/// Template engine registry
pub struct TemplateEngineRegistry {
    engines: Vec<Box<dyn TemplateEngine + Send + Sync>>,
}

impl TemplateEngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: Vec::new(),
        }
    }

    /// Add engine to registry
    pub fn register<T>(&mut self, engine: T) 
    where
        T: TemplateEngine + Send + Sync + 'static,
    {
        self.engines.push(Box::new(engine));
    }

    /// Get best engine for template
    pub fn get_engine_for_template(&self, template: &Template) -> Option<&(dyn TemplateEngine + Send + Sync)> {
        self.engines
            .iter()
            .find(|engine| engine.can_handle(template))
            .map(|engine| engine.as_ref())
    }

    /// Get engine by name
    pub fn get_engine_by_name(&self, name: &str) -> Option<&(dyn TemplateEngine + Send + Sync)> {
        self.engines
            .iter()
            .find(|engine| engine.name() == name)
            .map(|engine| engine.as_ref())
    }
}

impl Default for TemplateEngineRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        #[cfg(feature = "handlebars")]
        {
            if let Ok(handlebars_engine) = HandlebarsEngine::new() {
                registry.register(handlebars_engine);
            }
        }
        
        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Template, TemplateCategory, TemplateSection, SectionType};

    #[cfg(feature = "handlebars")]
    #[tokio::test]
    async fn test_handlebars_engine() {
        let engine = HandlebarsEngine::new().unwrap();
        
        let mut template = Template::new(
            "test".to_string(),
            "Test Template".to_string(),
            TemplateCategory::General,
        );
        
        let section = TemplateSection::new(
            "header".to_string(),
            SectionType::Header,
            "Hello {{name}}!".to_string(),
        );
        template.add_section(section);
        
        let mut variables = HashMap::new();
        variables.insert("name".to_string(), serde_json::Value::String("World".to_string()));
        
        let result = engine.render(&template, &variables).await;
        assert!(result.is_ok());
        
        let rendered = result.unwrap();
        assert!(rendered.content.contains("Hello World!"));
    }

    #[test]
    fn test_engine_registry() {
        let registry = TemplateEngineRegistry::default();
        
        #[cfg(feature = "handlebars")]
        {
            let engine = registry.get_engine_by_name("handlebars");
            assert!(engine.is_some());
        }
    }
}
