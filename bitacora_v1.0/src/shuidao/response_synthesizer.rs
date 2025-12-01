//! # ResponseSynthesizer - Sintetizador de Respuestas Adaptativas
//!
//! Componente que transforma respuestas de engines en mensajes finales personalizados,
//! ajustando tono, formato y contexto para cada modo cognitivo.
//!
//! ## Arquitectura
//!
//! ```text
//! EngineResponse → ResponseFormatter → ToneAdjuster → ContextInjection → LLM → SynthesizedResponse
//! ```
//!
//! ## Formatters por Modo
//!
//! - **Operational**: Formato orientado a acción (tasks, progreso, next steps)
//! - **Procedural**: Formato paso-a-paso (recetas, instrucciones, validaciones)
//! - **Light**: Formato directo y conciso (facts, definiciones, cálculos)
//! - **Learning**: Formato educativo (explicaciones, ejemplos, recursos)
//! - **Conversational**: Formato narrativo (historias, reflexiones, contexto biográfico)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::shuidao::error::{Result, ShuiDaoError};
use crate::shuidao::intention_detector::CognitiveMode;
// TODO: Re-enable when memory_bridge is implemented
// use crate::shuidao::memory_bridge::{MemoryBridge, MemoryItem, MemoryQuery, MemoryQueryType};
use crate::shuidao::operational_engine::OperationalResponse;
use crate::shuidao::procedural_engine::ProceduralResponse;

// ============================================================================
// ESTRUCTURAS PRINCIPALES
// ============================================================================

/// Sintetizador de respuestas con formatters por modo
pub struct ResponseSynthesizer {
    /// Formatters específicos por modo cognitivo
    formatters: HashMap<CognitiveMode, Box<dyn ResponseFormatter>>,
    
    /// Ajustador de tono
    tone_adjuster: ToneAdjuster,
    
    /// Bridge a memoria para contexto
    memory_bridge: Arc<MemoryBridge>,
}

/// Respuesta final sintetizada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedResponse {
    /// Contenido de la respuesta
    pub content: String,
    
    /// Modo cognitivo usado
    pub mode: CognitiveMode,
    
    /// Tono aplicado
    pub tone: ResponseTone,
    
    /// Referencias contextuales
    pub context_references: Vec<ContextReference>,
    
    /// Acciones sugeridas
    pub suggested_actions: Vec<String>,
    
    /// Metadata de generación
    pub metadata: ResponseMetadata,
}

/// Tonos de respuesta disponibles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseTone {
    /// Directo, orientado a acción
    Pragmatic,
    
    /// Supportivo, comprensivo
    Empathetic,
    
    /// Informativo, paciente
    Educational,
    
    /// Amigable, ligero
    Casual,
    
    /// Formal, preciso
    Professional,
}

/// Referencia contextual en respuesta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextReference {
    pub reference_type: ReferenceType,
    pub content: String,
    pub timestamp: Option<DateTime<Utc>>,
}

/// Tipos de referencia contextual
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceType {
    PreviousMessage,
    ActiveProject,
    LearningPath,
    BiographicalMemory,
}

/// Metadata de generación de respuesta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub generation_time_ms: u64,
    pub confidence: f32,
    pub sources: Vec<String>,
}

/// Contexto de conversación para síntesis
#[derive(Debug, Clone)]
pub struct ConversationContext {
    pub recent_messages: Vec<String>,
    pub active_project: Option<String>,
    pub active_learning_path: Option<String>,
    pub user_preferences: UserPreferences,
}

/// Preferencias del usuario
#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub preferred_tone: ResponseTone,
    pub verbosity: Verbosity,
    pub include_examples: bool,
}

/// Nivel de verbosidad
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verbosity {
    Concise,
    Standard,
    Verbose,
}

// ============================================================================
// RESPONSE UNIFICADA DE ENGINES
// ============================================================================

/// Respuesta unificada de cualquier engine
#[derive(Debug, Clone)]
pub enum EngineResponse {
    Operational(OperationalResponse),
    Procedural(ProceduralResponse),
    Light(String),
    Learning(String),
    Conversational(String),
}

// ============================================================================
// TRAIT RESPONSEFORMATTER
// ============================================================================

/// Trait para formatters específicos por modo cognitivo
pub trait ResponseFormatter: Send + Sync {
    /// Formatea respuesta de engine según su modo
    fn format(&self, engine_response: &EngineResponse) -> Result<String>;
}

// ============================================================================
// FORMATTERS CONCRETOS
// ============================================================================

/// Formatter para modo Operational
struct OperationalFormatter;

impl ResponseFormatter for OperationalFormatter {
    fn format(&self, engine_response: &EngineResponse) -> Result<String> {
        match engine_response {
            EngineResponse::Operational(response) => {
                let mut output = String::new();
                
                // Header con acción ejecutada
                output.push_str(&format!("📋 **Acción:** {:?}\n\n", response.action));
                
                // Progress summary
                output.push_str(&format!("📊 {}\n\n", response.progress_summary));
                
                // Next steps recomendados
                if !response.next_steps.is_empty() {
                    output.push_str("🎯 **Próximos pasos:**\n");
                    for (i, action) in response.next_steps.iter().enumerate() {
                        let time_str = action.estimated_time.as_ref()
                            .map(|t| format!(" ({})", t))
                            .unwrap_or_default();
                        output.push_str(&format!(
                            "{}. {} - Prioridad: {:?}{}\n",
                            i + 1,
                            action.description,
                            action.priority,
                            time_str
                        ));
                    }
                }
                
                Ok(output)
            }
            _ => Err(ShuiDaoError::InvalidInput(
                "Expected Operational response".to_string(),
            )),
        }
    }
}

/// Formatter para modo Procedural
struct ProceduralFormatter;

impl ResponseFormatter for ProceduralFormatter {
    fn format(&self, engine_response: &EngineResponse) -> Result<String> {
        match engine_response {
            EngineResponse::Procedural(response) => {
                let mut output = String::new();
                
                // Header con step actual
                output.push_str(&format!("📖 **Paso {}:** {}\n\n", 
                    response.current_step.number, 
                    response.current_step.instruction
                ));
                
                // Progress
                output.push_str(&format!(
                    "📊 Progreso: {}/{} ({:.0}%)\n\n",
                    response.progress.current,
                    response.progress.total,
                    response.progress.percentage
                ));
                
                // Validation notes si existen
                if let Some(validation) = &response.current_step.validation {
                    output.push_str(&format!("✓ **Validación:** {:?}\n\n", validation));
                }
                
                // Step notes
                if let Some(notes) = &response.current_step.notes {
                    output.push_str(&format!("💡 **Nota:** {}\n\n", notes));
                }
                
                // Next action
                output.push_str(&format!("⏭️  **Siguiente:** {:?}\n", response.next_action));
                
                Ok(output)
            }
            _ => Err(ShuiDaoError::InvalidInput(
                "Expected Procedural response".to_string(),
            )),
        }
    }
}

/// Formatter para modo Light
struct LightFormatter;

impl ResponseFormatter for LightFormatter {
    fn format(&self, engine_response: &EngineResponse) -> Result<String> {
        match engine_response {
            EngineResponse::Light(content) => {
                // Light mode: respuesta directa, sin formateo complejo
                Ok(content.clone())
            }
            _ => Err(ShuiDaoError::InvalidInput(
                "Expected Light response".to_string(),
            )),
        }
    }
}

/// Formatter para modo Learning
struct LearningFormatter;

impl ResponseFormatter for LearningFormatter {
    fn format(&self, engine_response: &EngineResponse) -> Result<String> {
        match engine_response {
            EngineResponse::Learning(content) => {
                let mut output = String::new();
                
                // Header educativo
                output.push_str("📚 **Explicación:**\n\n");
                output.push_str(content);
                output.push_str("\n\n💡 **Recuerda:** Practica estos conceptos para mejor retención.\n");
                
                Ok(output)
            }
            _ => Err(ShuiDaoError::InvalidInput(
                "Expected Learning response".to_string(),
            )),
        }
    }
}

/// Formatter para modo Conversational
struct ConversationalFormatter;

impl ResponseFormatter for ConversationalFormatter {
    fn format(&self, engine_response: &EngineResponse) -> Result<String> {
        match engine_response {
            EngineResponse::Conversational(content) => {
                // Conversational: formato natural, sin iconos excesivos
                Ok(content.clone())
            }
            _ => Err(ShuiDaoError::InvalidInput(
                "Expected Conversational response".to_string(),
            )),
        }
    }
}

// ============================================================================
// TONE ADJUSTER
// ============================================================================

/// Ajustador de tono de respuestas
pub struct ToneAdjuster {
    // TODO: Integrar sentiment analyzer en v1.1
}

impl ToneAdjuster {
    fn new() -> Self {
        Self {}
    }
    
    /// Ajusta tono del contenido según target
    fn adjust_tone(&self, content: &str, target_tone: ResponseTone) -> String {
        // v1.0: Prefijos simples por tono
        match target_tone {
            ResponseTone::Pragmatic => {
                format!("Aquí está lo que necesitas:\n\n{}", content)
            }
            ResponseTone::Empathetic => {
                format!("Entiendo lo que buscas. {}", content)
            }
            ResponseTone::Educational => {
                format!("Déjame explicarte paso a paso:\n\n{}", content)
            }
            ResponseTone::Casual => {
                format!("¡Hola! {}", content)
            }
            ResponseTone::Professional => {
                content.to_string() // Sin prefijo, formal directo
            }
        }
    }
}

// ============================================================================
// IMPLEMENTACIÓN RESPONSESYNTHESIZER
// ============================================================================

impl ResponseSynthesizer {
    /// Crea nuevo synthesizer con formatters y memoria
    pub fn new(memory_bridge: Arc<MemoryBridge>) -> Self {
        let mut formatters: HashMap<CognitiveMode, Box<dyn ResponseFormatter>> = HashMap::new();
        
        // Registrar formatters por modo
        formatters.insert(
            CognitiveMode::Operational,
            Box::new(OperationalFormatter),
        );
        formatters.insert(
            CognitiveMode::Procedural,
            Box::new(ProceduralFormatter),
        );
        formatters.insert(CognitiveMode::Light, Box::new(LightFormatter));
        formatters.insert(CognitiveMode::Learning, Box::new(LearningFormatter));
        formatters.insert(
            CognitiveMode::Conversational,
            Box::new(ConversationalFormatter),
        );
        
        Self {
            formatters,
            tone_adjuster: ToneAdjuster::new(),
            memory_bridge,
        }
    }
    
    /// Sintetiza respuesta final con formato, tono y contexto
    pub async fn synthesize(
        &self,
        engine_response: EngineResponse,
        mode: CognitiveMode,
        context: ConversationContext,
    ) -> Result<SynthesizedResponse> {
        let start = std::time::Instant::now();
        
        // 1. Formatear según modo
        let formatter = self
            .formatters
            .get(&mode)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("No formatter for mode {:?}", mode)))?;
        
        let formatted_content = formatter.format(&engine_response)?;
        
        // 2. Ajustar tono
        let toned_content = self
            .tone_adjuster
            .adjust_tone(&formatted_content, context.user_preferences.preferred_tone.clone());
        
        // 3. Agregar referencias contextuales
        let context_refs = self.gather_context_references(&context).await?;
        
        // 4. Generar acciones sugeridas
        let suggested_actions = self.generate_suggested_actions(&engine_response);
        
        let generation_time = start.elapsed().as_millis() as u64;
        
        Ok(SynthesizedResponse {
            content: toned_content,
            mode,
            tone: context.user_preferences.preferred_tone.clone(),
            context_references: context_refs,
            suggested_actions,
            metadata: ResponseMetadata {
                generation_time_ms: generation_time,
                confidence: 0.85, // TODO: calcular confidence real
                sources: vec!["ResponseSynthesizer".to_string()],
            },
        })
    }
    
    /// Recopila referencias contextuales de memoria
    async fn gather_context_references(
        &self,
        context: &ConversationContext,
    ) -> Result<Vec<ContextReference>> {
        let mut refs = Vec::new();
        
        // Agregar proyecto activo si existe
        if let Some(project_name) = &context.active_project {
            refs.push(ContextReference {
                reference_type: ReferenceType::ActiveProject,
                content: format!("Proyecto activo: {}", project_name),
                timestamp: Some(Utc::now()),
            });
        }
        
        // Agregar learning path activo si existe
        if let Some(path_topic) = &context.active_learning_path {
            refs.push(ContextReference {
                reference_type: ReferenceType::LearningPath,
                content: format!("Aprendiendo: {}", path_topic),
                timestamp: Some(Utc::now()),
            });
        }
        
        // Query mensajes previos recientes
        if !context.recent_messages.is_empty() {
            let query = MemoryQuery {
                query_type: MemoryQueryType::BiographicalEntries,
                filters: HashMap::new(),
                limit: Some(3),
                time_range: None,
            };
            
            if let Ok(result) = self.memory_bridge.query(query).await {
                for item in result.items.iter().take(2) {
                    if let MemoryItem::BiographicalEntry { content, timestamp, .. } = item {
                        refs.push(ContextReference {
                            reference_type: ReferenceType::PreviousMessage,
                            content: content.clone(),
                            timestamp: Some(*timestamp),
                        });
                    }
                }
            }
        }
        
        Ok(refs)
    }
    
    /// Genera acciones sugeridas según respuesta de engine
    fn generate_suggested_actions(&self, engine_response: &EngineResponse) -> Vec<String> {
        match engine_response {
            EngineResponse::Operational(resp) => {
                resp.next_steps
                    .iter()
                    .map(|a| a.description.clone())
                    .collect()
            }
            EngineResponse::Procedural(resp) => {
                vec![format!("{:?}", resp.next_action)]
            }
            EngineResponse::Light(_) => vec![],
            EngineResponse::Learning(_) => vec!["Practica este concepto".to_string()],
            EngineResponse::Conversational(_) => vec!["Continúa la conversación".to_string()],
        }
    }
    
    /// Ajusta tono público (wrapper)
    pub fn adjust_tone(&self, content: &str, target_tone: ResponseTone) -> String {
        self.tone_adjuster.adjust_tone(content, target_tone)
    }
}

impl Default for ConversationContext {
    fn default() -> Self {
        Self {
            recent_messages: vec![],
            active_project: None,
            active_learning_path: None,
            user_preferences: UserPreferences::default(),
        }
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            preferred_tone: ResponseTone::Pragmatic,
            verbosity: Verbosity::Standard,
            include_examples: true,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shuidao::operational_engine::{
        ActionRecommendation, OperationalAction, Priority, ProjectStatus, Task, TaskStatus,
    };

    fn create_test_memory_bridge() -> Arc<MemoryBridge> {
        Arc::new(MemoryBridge::new_stub())
    }

    #[tokio::test]
    async fn test_synthesizer_creation() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        assert_eq!(synthesizer.formatters.len(), 5);
    }

    #[tokio::test]
    async fn test_operational_formatting() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        let op_response = OperationalResponse {
            project_id: "proj-123".to_string(),
            action: OperationalAction::ProjectCreated {
                name: "Test Project".to_string(),
                sub_projects: 3,
            },
            next_steps: vec![ActionRecommendation {
                priority: Priority::High,
                description: "Continue with implementation".to_string(),
                estimated_time: Some("2 hours".to_string()),
            }],
            progress_summary: "Proyecto creado con 3 sub-proyectos".to_string(),
            processing_time_ms: 15.0,
        };
        
        let engine_response = EngineResponse::Operational(op_response);
        let context = ConversationContext::default();
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Operational, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.content.contains("Test Project"));
        assert_eq!(response.mode, CognitiveMode::Operational);
    }

    #[tokio::test]
    async fn test_procedural_formatting() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        use crate::shuidao::procedural_engine::{RecipeStep, StepProgress, StepValidation, ProceduralAction};
        
        let proc_response = ProceduralResponse {
            execution_id: "exec-123".to_string(),
            current_step: RecipeStep {
                number: 2,
                instruction: "Run: sudo apt install nginx".to_string(),
                validation: Some(StepValidation::CommandOutput {
                    command: "apt list nginx".to_string(),
                    expected: "installed".to_string(),
                }),
                can_skip: false,
                notes: Some("Package will be installed".to_string()),
                expected_duration: Some(chrono::Duration::seconds(60)),
            },
            progress: StepProgress {
                current: 2,
                total: 5,
                percentage: 40.0,
                estimated_remaining: None,
            },
            next_action: ProceduralAction::WaitForValidation,
            processing_time_ms: 10.0,
        };
        
        let engine_response = EngineResponse::Procedural(proc_response);
        let context = ConversationContext::default();
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Procedural, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.content.contains("sudo apt install nginx"));
        assert!(response.content.contains("2/5"));
    }

    #[tokio::test]
    async fn test_light_formatting() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        let engine_response = EngineResponse::Light("42".to_string());
        let context = ConversationContext::default();
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Light, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.content.contains("42"));
    }

    #[tokio::test]
    async fn test_tone_adjustment() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        let content = "This is a test response";
        
        let pragmatic = synthesizer.adjust_tone(content, ResponseTone::Pragmatic);
        assert!(pragmatic.contains("necesitas"));
        
        let empathetic = synthesizer.adjust_tone(content, ResponseTone::Empathetic);
        assert!(empathetic.contains("Entiendo"));
        
        let educational = synthesizer.adjust_tone(content, ResponseTone::Educational);
        assert!(educational.contains("paso a paso"));
    }

    #[tokio::test]
    async fn test_context_references() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        let mut context = ConversationContext::default();
        context.active_project = Some("Mi Proyecto".to_string());
        context.active_learning_path = Some("Rust Avanzado".to_string());
        
        let engine_response = EngineResponse::Light("test".to_string());
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Light, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.context_references.len() >= 2);
        
        let has_project = response
            .context_references
            .iter()
            .any(|r| r.reference_type == ReferenceType::ActiveProject);
        assert!(has_project);
    }

    #[tokio::test]
    async fn test_suggested_actions_generation() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        use crate::shuidao::procedural_engine::{RecipeStep, StepProgress, ProceduralAction};
        
        let proc_response = ProceduralResponse {
            execution_id: "exec-456".to_string(),
            current_step: RecipeStep {
                number: 1,
                instruction: "Step 1".to_string(),
                validation: None,
                can_skip: false,
                notes: None,
                expected_duration: None,
            },
            progress: StepProgress {
                current: 1,
                total: 3,
                percentage: 33.0,
                estimated_remaining: None,
            },
            next_action: ProceduralAction::AdvanceToNext,
            processing_time_ms: 5.0,
        };
        
        let engine_response = EngineResponse::Procedural(proc_response);
        let context = ConversationContext::default();
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Procedural, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.suggested_actions.len(), 1);
        assert!(response.suggested_actions[0].contains("AdvanceToNext"));
    }

    #[tokio::test]
    async fn test_performance_target() {
        let bridge = create_test_memory_bridge();
        let synthesizer = ResponseSynthesizer::new(bridge);
        
        let engine_response = EngineResponse::Light("Quick response".to_string());
        let context = ConversationContext::default();
        
        let result = synthesizer.synthesize(engine_response, CognitiveMode::Light, context).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        
        // Target: <30ms según spec
        assert!(response.metadata.generation_time_ms < 30);
    }
}
