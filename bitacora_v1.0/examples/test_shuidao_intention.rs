// === TEST SHUIDAO INTEGRATION ===
// Ejemplo de uso del IntentionDetector
// Validación de los 5 modos cognitivos + performance
// Creado: 2025-11-24 11:42:28

use bitacora::shuidao::{CognitiveMode, IntentionDetector};

fn main() {
    println!("🌊 SHUIDAO (水道) - Intention Detection Tests\n");
    println!("============================================\n");

    let detector = IntentionDetector::new();

    // Test 1: Operational Mode
    println!("📋 Test 1: OPERATIONAL MODE");
    test_intention(
        &detector,
        "necesito instalar un switch en la oficina",
        CognitiveMode::Operational,
    );

    // Test 2: Learning Mode
    println!("\n📚 Test 2: LEARNING MODE");
    test_intention(
        &detector,
        "explícame qué es el Context Token 7D",
        CognitiveMode::Learning,
    );

    // Test 3: Operational Mode (Kubernetes)
    println!("\n⚙️ Test 3: OPERATIONAL MODE (Technical)");
    test_intention(
        &detector,
        "quiero configurar Kubernetes en el servidor",
        CognitiveMode::Operational,
    );

    // Test 4: Learning Mode (Architecture)
    println!("\n🏗️ Test 4: LEARNING MODE (Architecture)");
    test_intention(
        &detector,
        "cómo funciona TelescopeDB internamente",
        CognitiveMode::Learning,
    );

    // Test 5: Light Mode
    println!("\n⚡ Test 5: LIGHT MODE");
    test_intention(
        &detector,
        "cuántos bytes tiene un megabyte",
        CognitiveMode::Light,
    );

    // Performance test
    println!("\n⏱️ PERFORMANCE TEST");
    println!("Running 100 detections...");
    
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ = detector.detect("necesito instalar un switch");
    }
    let elapsed = start.elapsed();
    let avg_ms = elapsed.as_millis() as f64 / 100.0;
    
    println!("Average: {:.2}ms per detection", avg_ms);
    println!("Target: <15ms ✅");
    
    if avg_ms < 15.0 {
        println!("✅ Performance target MET!");
    } else {
        println!("⚠️ Performance target MISSED ({}ms > 15ms)", avg_ms);
    }

    println!("\n🎉 All tests completed!");
}

fn test_intention(detector: &IntentionDetector, input: &str, expected_mode: CognitiveMode) {
    println!("Input: \"{}\"", input);
    
    match detector.detect(input) {
        Ok(intention) => {
            println!("✅ Detected Mode: {:?}", intention.mode);
            println!("   Submode: {:?}", intention.submode);
            println!("   Confidence: {:.2}", intention.confidence);
            println!("   Verb Score: {:.2}", intention.metadata.verb_score);
            println!("   Topic Score: {:.2}", intention.metadata.topic_score);
            println!("   Tone Score: {:.2}", intention.metadata.tone_score);
            println!("   Context Score: {:.2}", intention.metadata.context_score);
            println!("   Processing Time: {}ms", intention.metadata.processing_time_ms);
            
            // Validate expected mode
            if intention.mode == expected_mode {
                println!("   🎯 Expected mode MATCHED!");
            } else {
                println!("   ⚠️ Expected {:?}, got {:?}", expected_mode, intention.mode);
            }
            
            // Show extracted entities
            if !intention.extracted_entities.is_empty() {
                println!("   Extracted Entities:");
                for (key, value) in &intention.extracted_entities {
                    println!("     - {}: {}", key, value);
                }
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
}
