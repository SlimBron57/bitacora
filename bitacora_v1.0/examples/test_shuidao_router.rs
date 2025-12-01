//! # ShuiDao CognitiveRouter Integration Tests
//!
//! Tests para validar routing decisions y fallback chains
//!
//! **Completado**: 2025-11-24 Week 1 Day 2

use bitacora::shuidao::{
    CognitiveMode, CognitiveRouter, DetectedIntention, IntentionDetector, IntentionMetadata,
};
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== SHUIDAO COGNITIVE ROUTER - INTEGRATION TESTS ===\n");

    let router = CognitiveRouter::new();
    let detector = IntentionDetector::new();

    println!("📋 Configuration:");
    println!("   Min Confidence: {}", router.min_confidence);
    println!("   Fallback Mode: {:?}", router.fallback_mode);
    println!("   Fallback Enabled: {}\n", router.enable_fallback);

    // Test 1: High confidence Operational
    println!("Test 1: High Confidence Operational");
    let intention1 = detector
        .detect("necesito crear un proyecto de networking con switches")
        .unwrap();
    println!("   Input: \"necesito crear un proyecto de networking...\"");
    println!("   Detected: {:?} (confidence: {:.2})", intention1.mode, intention1.confidence);

    let decision1 = router.route(intention1).unwrap();
    println!("   ✅ Routed to: {:?}", decision1.selected_mode);
    println!("   Fallback used: {}", decision1.fallback_used);
    println!("   Routing time: {:.3}ms\n", decision1.metadata.routing_time_ms);

    assert_eq!(
        decision1.selected_mode,
        CognitiveMode::Operational,
        "Expected Operational mode"
    );
    assert!(!decision1.fallback_used, "Should not use fallback");

    // Test 2: High confidence Learning
    println!("Test 2: High Confidence Learning");
    let intention2 = detector.detect("explícame cómo funciona CTX7D en detalle").unwrap();
    println!("   Input: \"explícame cómo funciona CTX7D...\"");
    println!("   Detected: {:?} (confidence: {:.2})", intention2.mode, intention2.confidence);

    let decision2 = router.route(intention2).unwrap();
    println!("   ✅ Routed to: {:?}", decision2.selected_mode);
    println!("   Fallback used: {}", decision2.fallback_used);
    println!("   Routing time: {:.3}ms\n", decision2.metadata.routing_time_ms);

    assert_eq!(
        decision2.selected_mode,
        CognitiveMode::Learning,
        "Expected Learning mode"
    );

    // Test 3: Low confidence (simulated)
    println!("Test 3: Low Confidence with Fallback");
    let low_confidence_intention = DetectedIntention {
        mode: CognitiveMode::Operational,
        submode: None,
        confidence: 0.45, // Below 0.60 threshold
        extracted_entities: HashMap::new(),
        metadata: IntentionMetadata {
            verb_score: 0.5,
            topic_score: 0.4,
            tone_score: 0.3,
            context_score: 0.6,
            processing_time_ms: 10,
            alternative_mode: None,
        },
    };
    println!("   Input: [simulated low confidence]");
    println!("   Detected: {:?} (confidence: {:.2})", low_confidence_intention.mode, low_confidence_intention.confidence);

    let decision3 = router.route(low_confidence_intention).unwrap();
    println!("   ✅ Fallback activated!");
    println!("   Routed to: {:?}", decision3.selected_mode);
    println!("   Fallback chain: {:?}", decision3.fallback_chain);
    println!("   Routing time: {:.3}ms\n", decision3.metadata.routing_time_ms);

    assert!(decision3.fallback_used, "Should use fallback");
    assert!(!decision3.fallback_chain.is_empty(), "Should have fallback chain");

    // Test 4: Fallback chain validation (using low confidence scenarios)
    println!("Test 4: Fallback Chain Logic");
    
    // Test Operational fallback
    let operational_low = DetectedIntention {
        mode: CognitiveMode::Operational,
        submode: None,
        confidence: 0.40,
        extracted_entities: HashMap::new(),
        metadata: IntentionMetadata {
            verb_score: 0.4,
            topic_score: 0.4,
            tone_score: 0.4,
            context_score: 0.4,
            processing_time_ms: 10,
            alternative_mode: None,
        },
    };
    let decision_op = router.route(operational_low).unwrap();
    println!("   Operational → {:?}", decision_op.fallback_chain);
    assert_eq!(decision_op.fallback_chain, vec![CognitiveMode::Procedural, CognitiveMode::Light]);

    // Test Learning fallback
    let learning_low = DetectedIntention {
        mode: CognitiveMode::Learning,
        submode: None,
        confidence: 0.40,
        extracted_entities: HashMap::new(),
        metadata: IntentionMetadata {
            verb_score: 0.4,
            topic_score: 0.4,
            tone_score: 0.4,
            context_score: 0.4,
            processing_time_ms: 10,
            alternative_mode: None,
        },
    };
    let decision_learn = router.route(learning_low).unwrap();
    println!("   Learning → {:?}", decision_learn.fallback_chain);
    assert_eq!(decision_learn.fallback_chain, vec![CognitiveMode::Conversational, CognitiveMode::Light]);

    // Test Light (should have no fallback - use high confidence to avoid fallback activation)
    let light_high = DetectedIntention {
        mode: CognitiveMode::Light,
        submode: None,
        confidence: 0.80,
        extracted_entities: HashMap::new(),
        metadata: IntentionMetadata {
            verb_score: 0.8,
            topic_score: 0.8,
            tone_score: 0.8,
            context_score: 0.8,
            processing_time_ms: 10,
            alternative_mode: None,
        },
    };
    let decision_light = router.route(light_high).unwrap();
    println!("   Light → {:?} (no fallback)\n", decision_light.fallback_chain);
    assert!(decision_light.fallback_chain.is_empty());

    // Test 5: Performance benchmark
    println!("Test 5: Performance Benchmark (100 routing decisions)");
    let test_intention = detector.detect("configurar kubernetes").unwrap();

    let start = Instant::now();
    for _ in 0..100 {
        let _ = router.route(test_intention.clone()).unwrap();
    }
    let elapsed = start.elapsed();
    let avg_ms = elapsed.as_secs_f64() * 1000.0 / 100.0;

    println!("   100 routing decisions");
    println!("   Total time: {:.2}ms", elapsed.as_secs_f64() * 1000.0);
    println!("   Average: {:.3}ms per decision", avg_ms);
    println!("   Target: <5ms ✅\n");

    assert!(avg_ms < 5.0, "Average routing time should be < 5ms");

    // Test 6: Custom threshold
    println!("Test 6: Custom Threshold (0.80)");
    let strict_router = CognitiveRouter::with_config(0.80, CognitiveMode::Light);
    let medium_confidence = DetectedIntention {
        mode: CognitiveMode::Learning,
        submode: None,
        confidence: 0.75, // Between 0.60 and 0.80
        extracted_entities: HashMap::new(),
        metadata: IntentionMetadata {
            verb_score: 0.7,
            topic_score: 0.8,
            tone_score: 0.7,
            context_score: 0.8,
            processing_time_ms: 10,
            alternative_mode: None,
        },
    };

    println!("   Confidence: 0.75 vs Threshold: 0.80");
    let decision6 = strict_router.route(medium_confidence).unwrap();
    println!("   ✅ Fallback to: {:?}", decision6.selected_mode);
    println!("   Reason: {}\n", decision6.metadata.reason);

    assert!(decision6.fallback_used, "Should use fallback with strict threshold");

    // Test 7: Default decision (emergency fallback)
    println!("Test 7: Emergency Fallback (Default Decision)");
    let emergency = router.default_decision();
    println!("   Emergency mode: {:?}", emergency.selected_mode);
    println!("   Confidence: {}", emergency.confidence);
    println!("   Reason: {}\n", emergency.metadata.reason);

    assert_eq!(emergency.selected_mode, CognitiveMode::Light);
    assert_eq!(emergency.confidence, 0.0);

    // Summary
    println!("=== SUMMARY ===");
    println!("✅ All 7 integration tests passed!");
    println!("✅ CognitiveRouter working correctly");
    println!("✅ Fallback chains validated");
    println!("✅ Performance target met (<5ms)");
    println!("\n📊 Metrics:");
    println!("   Average routing time: {:.3}ms", avg_ms);
    println!("   Target: <5ms ✅");
    println!("   Speedup vs target: {:.1}x\n", 5.0 / avg_ms);
}
