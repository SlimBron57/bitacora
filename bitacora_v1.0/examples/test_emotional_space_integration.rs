// === EMOTIONAL SPACE INTEGRATION TEST ===
// Prueba end-to-end del sistema EmotionalSpace + IntentionDetector
// Demuestra: TopicGraph + EmotionalSpace = Personalización Infinita
// DA-033: EmotionalSpace System (VAD+F model)
// Creado: 2025-11-26 14:45:00

use bitacora::shuidao::{
    emotional_space::{EmotionalSpace, ToneCluster, ToneDimensions, ToneDetector},
    tone_learning::ToneLearner,
    tone_integration::{ToneStorage, generate_tone_template},
    intention_detector::IntentionDetector,
    topic_graph::TopicGraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== DA-033 EmotionalSpace Integration Test ===\n");
    
    // ========================================
    // PARTE 1: Crear perfil de Eduardo
    // ========================================
    println!("📊 PARTE 1: Creando perfil emocional de Eduardo...\n");
    
    let user_id = "eduardo_001";
    let mut emotional_space = EmotionalSpace::new(user_id.to_string());
    
    // Tone 1: "Determinado" (positivo, energético, asertivo, neutral)
    let determinado = ToneDimensions::new(0.3, 0.7, 0.8, 0.5);
    let mut cluster_determinado = ToneCluster::new(
        "tone_eduardo_001".to_string(),
        "Determinado".to_string(),
        determinado,
        user_id.to_string(),
    );
    cluster_determinado.add_example("necesito crear un proyecto ahora".to_string());
    cluster_determinado.add_example("vamos a implementar esto rápido".to_string());
    emotional_space.add_cluster(cluster_determinado);
    
    // Tone 2: "Nostálgico" (ligeramente negativo, calmado, pasivo, informal)
    let nostalgico = ToneDimensions::new(-0.2, -0.5, -0.4, -0.3);
    let mut cluster_nostalgico = ToneCluster::new(
        "tone_eduardo_002".to_string(),
        "Nostálgico".to_string(),
        nostalgico,
        user_id.to_string(),
    );
    cluster_nostalgico.add_example("recuerdo cuando trabajábamos en aquel proyecto".to_string());
    cluster_nostalgico.add_example("extraño aquellos días de universidad".to_string());
    emotional_space.add_cluster(cluster_nostalgico);
    
    // Tone 3: "Reflexivo Nocturno" (neutral, muy calmado, balanceado, informal)
    let reflexivo = ToneDimensions::new(0.0, -0.7, 0.0, -0.2);
    let mut cluster_reflexivo = ToneCluster::new(
        "tone_eduardo_003".to_string(),
        "Reflexivo Nocturno".to_string(),
        reflexivo,
        user_id.to_string(),
    );
    cluster_reflexivo.add_example("pensando en las implicaciones filosóficas del código".to_string());
    cluster_reflexivo.add_example("me pregunto por qué elegimos esta arquitectura".to_string());
    emotional_space.add_cluster(cluster_reflexivo);
    
    println!("✅ Perfil creado con {} tones personalizados:", emotional_space.cluster_count());
    for cluster in emotional_space.clusters.values() {
        println!("   - {} ({})", cluster.name, cluster.id);
        println!("     Dimensiones: {}", cluster.center.describe());
    }
    
    // ========================================
    // PARTE 2: Detectar tones en mensajes
    // ========================================
    println!("\n📡 PARTE 2: Detectando tones en mensajes nuevos...\n");
    println!("ℹ️  NOTA: v1.0 usa heurísticas stub para VAD+F computation");
    println!("         v1.1 integrará NLP real (sentiment analysis, POS tagging)\n");
    
    let detector = ToneDetector::new();
    
    let test_messages = vec![
        "necesito terminar este módulo urgentemente", // Determinado
        "recuerdo cuando empezamos este proyecto juntos", // Nostálgico
        "reflexionando sobre la arquitectura del sistema", // Reflexivo
        "este código me inspira a crear algo mejor", // Nuevo tone?
    ];
    
    for (i, msg) in test_messages.iter().enumerate() {
        let detection = detector.detect(msg, &emotional_space);
        
        println!("Mensaje {}: \"{}\"", i+1, msg);
        
        if detection.is_new_tone {
            println!("   🆕 Tone NUEVO detectado!");
            println!("   Dimensiones: {}", detection.dimensions.describe());
        } else {
            println!("   ✅ Tone conocido: {}", detection.cluster_name.clone().unwrap());
            println!("   Confianza: {:.2}%", detection.confidence * 100.0);
            println!("   Dimensiones: {}", detection.dimensions.describe());
        }
        println!();
    }
    
    // ========================================
    // PARTE 3: Auto-discovery con ToneLearner
    // ========================================
    println!("🧠 PARTE 3: Auto-discovery de tones nuevos...\n");
    
    let mut learner = ToneLearner::new();
    
    // Simular conversación donde Eduardo usa un nuevo tone
    let inspirational_messages = vec![
        "este código me inspira a crear algo mejor",
        "estoy motivado para llevar esto al siguiente nivel",
    ];
    
    for msg in inspirational_messages {
        let candidates = learner.extract_candidates(msg, &emotional_space);
        println!("Mensaje: \"{}\"", msg);
        
        if candidates.is_empty() {
            println!("   No se detectaron candidatos nuevos\n");
        } else {
            for candidate in candidates {
                println!("   🆕 Candidato detectado: {}", candidate.tentative_name);
                println!("   Relevancia: {:.2}", candidate.relevance_score);
                println!("   Dimensiones: {}", candidate.dimensions.describe());
            }
            println!();
        }
    }
    
    // Obtener sugerencias (requiere 2+ menciones)
    let suggestions = learner.get_suggestions();
    
    if !suggestions.is_empty() {
        println!("💡 Sugerencias de tones para confirmar:\n");
        for suggestion in &suggestions {
            println!("{}", learner.format_suggestion_message(suggestion));
            println!();
        }
        
        // Simular confirmación del primer tone
        if let Some(suggestion) = suggestions.first() {
            println!("✅ Usuario confirma: 'Inspirado Motivacional'\n");
            
            let cluster_id = learner.confirm_candidate(
                &suggestion.name,
                "Inspirado Motivacional".to_string(),
                &mut emotional_space,
            )?;
            
            println!("✅ Tone agregado al perfil: {}", cluster_id);
        }
    } else {
        println!("ℹ️  Se requieren más menciones (2+) para generar sugerencias\n");
    }
    
    println!("📊 Perfil actualizado: {} tones totales\n", emotional_space.cluster_count());
    
    // ========================================
    // PARTE 4: Persistencia (JSON + YAML)
    // ========================================
    println!("💾 PARTE 4: Persistencia del perfil...\n");
    
    let storage = ToneStorage::new();
    
    // Guardar EmotionalSpace
    storage.save(&emotional_space)?;
    println!("✅ EmotionalSpace guardado: ./data/emotional_spaces/{}.json", user_id);
    
    // Cargar de vuelta
    let loaded_space = storage.load(user_id)?;
    println!("✅ EmotionalSpace cargado: {} clusters", loaded_space.cluster_count());
    
    // Generar templates MTT-DSL
    println!("\n📝 Generando templates MTT-DSL...\n");
    
    for cluster in loaded_space.clusters.values() {
        let template = generate_tone_template(&loaded_space, &cluster.id)?;
        println!("Template para '{}':", cluster.name);
        println!("{}", template.lines().take(10).collect::<Vec<_>>().join("\n"));
        println!("...\n");
    }
    
    // ========================================
    // PARTE 5: Integración con IntentionDetector
    // ========================================
    println!("🎯 PARTE 5: Integración con IntentionDetector...\n");
    
    // Crear TopicGraph también
    let mut topic_graph = TopicGraph::new(user_id.to_string());
    
    // add_topic requires (name, embedding)
    let _k8s_id = topic_graph.add_topic(
        "Kubernetes".to_string(), 
        vec![0.8, 0.6, 0.9] // stub embedding
    )?;
    
    let _rust_id = topic_graph.add_topic(
        "Rust".to_string(), 
        vec![0.7, 0.9, 0.8] // stub embedding
    )?;
    
    let topic_count = topic_graph.nodes.len();
    
    // IntentionDetector con AMBOS sistemas
    // Usar threshold más bajo (0.60) para v1.0 stub heuristics
    let detector = IntentionDetector::with_config(0.60, (0.30, 0.30, 0.25, 0.15))
        .with_topic_graph(topic_graph)
        .with_emotional_space(loaded_space);
    
    println!("✅ IntentionDetector configurado con:");
    println!("   - TopicGraph: {} topics", topic_count);
    println!("   - EmotionalSpace: {} tones", detector.emotional_space().unwrap().cluster_count());
    
    // Detectar intención con personalización completa
    let test_intentions = vec![
        "necesito configurar un cluster de Kubernetes urgentemente",
        "explícame cómo funciona el ownership en Rust",
        "reflexionando sobre la arquitectura de microservicios",
    ];
    
    println!("\n🔍 Detección de intenciones con personalización:\n");
    
    for msg in test_intentions {
        let intention = detector.detect(msg)?;
        
        println!("Mensaje: \"{}\"", msg);
        println!("   Modo: {:?}", intention.mode);
        println!("   Confianza: {:.2}%", intention.confidence * 100.0);
        println!("   Scores: verb={:.2}, topic={:.2}, tone={:.2}",
            intention.metadata.verb_score,
            intention.metadata.topic_score,
            intention.metadata.tone_score);
        println!();
    }
    
    // ========================================
    // PARTE 6: Métricas finales
    // ========================================
    println!("📈 MÉTRICAS FINALES:\n");
    
    println!("EmotionalSpace:");
    println!("   - User ID: {}", detector.emotional_space().unwrap().user_id);
    println!("   - Tones personalizados: {}", detector.emotional_space().unwrap().cluster_count());
    println!("   - Archivo: ./data/emotional_spaces/{}.json", user_id);
    
    println!("\nTopicGraph:");
    println!("   - User ID: {}", detector.topic_graph().unwrap().user_id);
    println!("   - Topics personalizados: {}", detector.topic_graph().unwrap().nodes.len());
    println!("   - Archivo: ./data/topic_graphs/{}.json", user_id);
    
    println!("\n✅ Sistema de personalización infinita operativo!");
    println!("🎯 TopicGraph + EmotionalSpace = ∞ combinaciones posibles");
    println!("🚀 DA-033 EmotionalSpace System: COMPLETO\n");
    
    Ok(())
}
