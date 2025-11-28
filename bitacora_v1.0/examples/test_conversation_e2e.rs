//! # Test Conversación End-to-End
//!
//! Ejemplo completo del pipeline conversacional de Bitácora:
//!
//! ```text
//! Usuario Input → IntentionDetector → CognitiveRouter → Engine 
//!              → ResponseSynthesizer → Terminal Output
//! ```
//!
//! **Task 7.2:** HubSpoke integración con LLM REAL implementada.
//! Requiere environment variables: OPENAI_API_KEY o ANTHROPIC_API_KEY

use bitacora::shuidao::{
    CognitiveMode, CognitiveRouter, ConversationContext, ConversationalEngine, EngineResponse,
    IceBreakerEngine, IntentionDetector, LearningEngine, LightEngine, MemoryBridge,
    OperationalProjectEngine, ProceduralRecipeEngine, ResponseSynthesizer, UserPreferences,
};
use bitacora::multi_agent::{Hub, HubSpokeConfig, ContextTensor7D};
use std::io::{self, Write};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ⛵️  Bitácora v1.0 - Conversación Interactiva");
    println!("================================================\n");

    // Inicializar componentes
    println!("🔧 Inicializando sistema...");
    let memory_bridge = Arc::new(MemoryBridge::new_stub());
    let synthesizer = Arc::new(ResponseSynthesizer::new(memory_bridge.clone()));

    // Initialize HubSpoke for LLM routing
    let mut hub = Hub::new(HubSpokeConfig::default())?;
    println!("✅ HubSpoke initialized (real LLM integration)");

    // Initialize all 5 engines
    let operational_engine = Arc::new(OperationalProjectEngine::new());
    let procedural_engine = Arc::new(ProceduralRecipeEngine::new());
    let mut learning_engine = LearningEngine::new();
    let mut light_engine = LightEngine::new();
    let mut conversational_engine = ConversationalEngine::new();

    let detector = IntentionDetector::with_config(
        0.70, // Threshold más flexible para conversaciones naturales
        (0.35, 0.35, 0.20, 0.10), // Pesos default: verb, topic, tone, context
    );
    
    let router = CognitiveRouter::new();

    println!("✅ Sistema listo!\n");
    
    // ================================================================
    // ICEBREAKER: Detect first-time user and establish relationship
    // ================================================================
    
    // Simular detección de usuario nuevo (en producción: query biographical data)
    let is_first_time_user = true; // TODO: memory_bridge.query_biographical("user_id").await.is_none()
    
    if is_first_time_user {
        println!("👋 ¡Bienvenido! Parece que es tu primera vez aquí.");
        println!("📋 Vamos a conocernos un poco antes de empezar...\n");
        
        let mut icebreaker = IceBreakerEngine::new(memory_bridge.clone())?;
        let mut conversation_history: Vec<(String, String)> = Vec::new(); // (role, message)
        
        // Ice-breaking loop
        while !icebreaker.is_ice_broken() {
            // Get prompt for current stage
            let base_prompt = icebreaker.get_current_prompt()?;
            
            // Build full prompt with conversation history
            let mut full_prompt = base_prompt.clone();
            if !conversation_history.is_empty() {
                full_prompt.push_str("\n\nHistorial de conversación:\n");
                for (role, msg) in &conversation_history {
                    full_prompt.push_str(&format!("{}: {}\n", role, msg));
                }
                full_prompt.push_str("\nContinúa la conversación de forma natural y coherente con el historial.");
            }
            
            // ✅ Task 7.2: Real LLM integration via HubSpoke
            let ctx7d = ContextTensor7D::from_prompt(&full_prompt);
            let response = hub.query_with_routing(&full_prompt, &ctx7d, "user_demo").await
                .map_err(|e| format!("LLM query failed: {:?}", e))?;
            let llm_response = response.text;
            
            // Store in history
            conversation_history.push(("Bitácora".to_string(), llm_response.clone()));
            
            println!("🤖 Bitácora: {}", llm_response);
            println!("   (via {} | {:.3}s | ${:.4})", 
                     "HubSpoke", 
                     response.latency_ms as f64 / 1000.0,
                     response.cost_usd);
            
            // Get user input
            print!("\n🧑 Tú: ");
            io::stdout().flush()?;
            
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input)?;
            let user_input = user_input.trim();
            
            if user_input.is_empty() {
                continue;
            }
            
            if user_input == "salir" || user_input == "exit" {
                println!("\n👋 ¡Hasta pronto!");
                return Ok(());
            }
            
            // Store user input in history
            conversation_history.push(("Usuario".to_string(), user_input.to_string()));
            
            // Process response and advance stage
            let result = icebreaker.process_user_response(user_input)?;
            
            // Show progress
            println!("\n📍 Stage: {:?} | Ice broken: {}", 
                     result.stage_advanced, result.ice_broken);
            
            if !result.extracted_data.interests.is_empty() {
                println!("💡 Intereses detectados: {:?}", result.extracted_data.interests);
            }
            
            println!();
        }
        
        println!("✅ ¡Genial! Ya nos conocemos mejor.");
        println!("🎯 Ahora puedo ayudarte con lo que necesites.\n");
    }
    
    // ================================================================
    // NORMAL CONVERSATION MODE
    // ================================================================
    
    println!("💡 Prueba los 5 modos cognitivos:");
    println!("   🔧 Operational: 'crear proyecto para migrar base de datos'");
    println!("   📖 Procedural: 'necesito instalar nginx paso a paso'");
    println!("   🎓 Learning: 'quiero aprender Rust'");
    println!("   💬 Conversational: 'cuéntame sobre tu día'");
    println!("   ⚡ Light: '¿cuál es la raíz cuadrada de 144?'");
    println!("   🚪 'salir' para terminar\n");

    // Loop conversacional
    loop {
        print!("\n🧑 Tú: ");
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if user_input.is_empty() {
            continue;
        }

        if user_input == "salir" || user_input == "exit" {
            println!("\n👋 ¡Hasta pronto!");
            break;
        }

        // 1. Detectar intención
        print!("🔍 Detectando intención... ");
        io::stdout().flush()?;

        let intention = detector.detect(user_input)?;
        println!("✓ Modo: {:?} (confianza: {:.0}%)", intention.mode, intention.confidence * 100.0);

        // 2. Guardar en memoria
        let _ = memory_bridge.store_intention(user_input, &intention).await;

        // 2.5 Route to appropriate engine
        let routing_decision = router.route(intention.clone())?;
        println!("🧭 Routing: {:?} (fallback: {})", routing_decision.selected_mode, routing_decision.fallback_used);

        // 3. Procesar según modo
        print!("⚙️  Procesando... ");
        io::stdout().flush()?;

        let engine_response = match routing_decision.selected_mode {
            CognitiveMode::Operational => {
                // Crear proyecto con input del usuario
                let response = operational_engine.create_project(user_input)?;
                EngineResponse::Operational(response)
            }
            CognitiveMode::Procedural => {
                // Por ahora respuesta genérica para procedural
                if user_input.to_lowercase().contains("install") 
                    || user_input.to_lowercase().contains("nginx") 
                    || user_input.to_lowercase().contains("cisco") {
                    EngineResponse::Light(format!(
                        "📖 Encontré una receta para esto. Empezaremos paso a paso con la instalación..."
                    ))
                } else {
                    EngineResponse::Light(format!(
                        "No encontré una receta específica para '{}'. ¿Puedes ser más específico? (Ej: 'instalar nginx', 'configurar cisco')",
                        user_input
                    ))
                }
            }
            CognitiveMode::Light => {
                // Use LightEngine
                let response = light_engine.process(user_input)?;
                EngineResponse::Light(format!(
                    "{}\n(Tipo: {:?}, Confianza: {:.0}%, Tiempo: {:.2}ms)",
                    response.answer,
                    response.response_type,
                    response.confidence * 100.0,
                    response.processing_time_ms
                ))
            }
            CognitiveMode::Learning => {
                // Demo: Create a simple learning path
                EngineResponse::Learning(format!(
                    "Modo aprendizaje: Crearía un learning path sobre '{}'...\n\
                    Módulos sugeridos:\n\
                    1. Fundamentos\n\
                    2. Práctica\n\
                    3. Avanzado",
                    user_input
                ))
            }
            CognitiveMode::Conversational => {
                // Use ConversationalEngine
                let conv_id = conversational_engine.start_conversation(Some("Eduardo".to_string()))?;
                let response = conversational_engine.process_message(&conv_id, user_input.to_string())?;
                EngineResponse::Conversational(format!(
                    "{}\n(Tono: {:?}, Sentiment: {:.2}, Topics: {:?})",
                    response.response_content,
                    response.suggested_tone,
                    response.sentiment.score,
                    response.detected_topics
                ))
            }
        };

        println!("✓");

        // 4. Sintetizar respuesta
        let context = ConversationContext {
            recent_messages: vec![user_input.to_string()],
            active_project: None,
            active_learning_path: None,
            user_preferences: UserPreferences::default(),
        };

        let synthesized = synthesizer
            .synthesize(engine_response, routing_decision.selected_mode, context)
            .await?;

        // 5. Mostrar respuesta
        println!("\n🤖 Bitácora:");
        println!("{}", "─".repeat(60));
        println!("{}", synthesized.content);
        println!("{}", "─".repeat(60));

        // Metadata
        println!(
            "\n📊 Generado en {}ms | Confianza: {:.0}%",
            synthesized.metadata.generation_time_ms,
            synthesized.metadata.confidence * 100.0
        );

        if !synthesized.suggested_actions.is_empty() {
            println!("\n💡 Acciones sugeridas:");
            for (i, action) in synthesized.suggested_actions.iter().enumerate() {
                println!("   {}. {}", i + 1, action);
            }
        }
    }

    Ok(())
}

/// Extrae nombre de proyecto de input (heurística simple)
fn extract_project_name(input: &str) -> String {
    let lower = input.to_lowercase();

    if lower.contains("para") {
        // "crear proyecto para X" → X
        if let Some(pos) = lower.find("para") {
            return input[pos + 4..].trim().to_string();
        }
    }

    if lower.contains("llamado") || lower.contains("llamada") {
        // "proyecto llamado X" → X
        if let Some(pos) = lower.find("llamad") {
            return input[pos + 7..].trim().to_string();
        }
    }

    // Default: usar parte relevante
    input
        .replace("crear", "")
        .replace("proyecto", "")
        .replace("nuevo", "")
        .trim()
        .to_string()
}

/// Respuestas simples para modo Light (sin LLM)
fn simple_light_response(input: &str) -> String {
    let lower = input.to_lowercase();

    // Matemáticas básicas
    if lower.contains("raíz cuadrada de 144") || lower.contains("sqrt(144)") {
        return "12".to_string();
    }
    if lower.contains("2 + 2") || lower.contains("2+2") {
        return "4".to_string();
    }

    // Definiciones simples
    if lower.contains("qué es rust") || lower.contains("que es rust") {
        return "Rust es un lenguaje de programación de sistemas enfocado en seguridad, velocidad y concurrencia.".to_string();
    }

    // Default
    format!("Respuesta directa para: {}", input)
}

// ✅ Task 7.2 COMPLETE: simulate_llm_response() eliminada
// Ahora usamos hub.query_with_routing() con LLM real
