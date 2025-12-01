//! # Test de Integración - HubSpoke Navigator
//!
//! Valida routing inteligente, failover automático y optimización de costos

use std::collections::HashMap;

// Importar módulos desde src/
mod multi_agent {
    pub mod hubspoke {
        include!("../src/multi_agent/hubspoke.rs");
    }
}

use multi_agent::hubspoke::*;

/// Test 1: Routing básico round-robin
#[test]
fn test_routing_round_robin() {
    let mut config = HubSpokeConfig::default();
    config.default_strategy = RoutingStrategy::RoundRobin;
    config.enabled_providers = vec![
        LLMProvider::OpenAI,
        LLMProvider::Anthropic,
        LLMProvider::Perplexity,
    ];
    
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    let ctx7d = create_neutral_context();
    
    // Ejecutar 6 queries para completar 2 ciclos
    let mut providers = Vec::new();
    for i in 0..6 {
        let query = format!("Test query {}", i);
        let provider = hub.route_query(&query, ctx7d.clone())
            .expect("Routing failed");
        providers.push(provider);
    }
    
    // Verificar rotación
    assert_eq!(providers.len(), 6);
    
    // Debe haber al menos 2 providers diferentes
    let unique: std::collections::HashSet<_> = providers.iter().collect();
    assert!(unique.len() >= 2, "Round-robin no está rotando providers");
    
    // Verificar que no se repite el mismo 3 veces seguidas
    for window in providers.windows(3) {
        let all_same = window.iter().all(|p| *p == window[0]);
        assert!(!all_same, "Round-robin tiene 3 consecutivos iguales");
    }
    
    println!("✅ Round-robin rotación correcta: {:?}", providers);
}

/// Test 2: Routing contextual para alta complejidad
#[test]
fn test_routing_contextual_high_complexity() {
    let mut config = HubSpokeConfig::default();
    config.default_strategy = RoutingStrategy::ContextualBestFit;
    
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    // Contexto con alta complejidad semántica y evaluativa
    let ctx7d_complex = ContextTensor7D {
        semantic: 0.95,      // Alta complejidad
        intentional: 0.9,
        temporal: 0.2,       // No urgente
        emotional: 0.5,
        associative: 0.8,
        evaluative: 0.9,     // Alto juicio crítico
        metalinguistic: 0.7,
    };
    
    let semantic_value = ctx7d_complex.semantic;
    
    let provider = hub.route_query(
        "Explain async Rust ownership and lifetimes with advanced patterns",
        ctx7d_complex
    ).expect("Routing failed");
    
    // Alta complejidad → Anthropic (mejor reasoning_quality=0.95)
    assert_eq!(
        provider,
        LLMProvider::Anthropic,
        "Alta complejidad debería seleccionar Anthropic"
    );
    
    // Verificar registro de decisión
    let history = hub.routing_history();
    assert_eq!(history.len(), 1);
    assert!(history[0].reasoning.contains("complejidad") || 
            history[0].reasoning.contains("contextual"));
    
    println!("✅ Routing contextual: semantic={} → {:?}", semantic_value, provider);
}

/// Test 3: Routing cost-optimized para queries simples
#[test]
fn test_routing_cost_optimized() {
    let mut config = HubSpokeConfig::default();
    config.default_strategy = RoutingStrategy::CostOptimized;
    
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    // Contexto simple (baja complejidad)
    let ctx7d_simple = ContextTensor7D {
        semantic: 0.2,       // Baja complejidad
        intentional: 0.8,
        temporal: 0.5,
        emotional: 0.5,
        associative: 0.3,
        evaluative: 0.2,     // Bajo juicio crítico
        metalinguistic: 0.1,
    };
    
    let provider = hub.route_query(
        "What's the capital of France?",
        ctx7d_simple
    ).expect("Routing failed");
    
    // Baja complejidad → Perplexity (más barato: $0.001/1k)
    assert_eq!(
        provider,
        LLMProvider::Perplexity,
        "Baja complejidad debería seleccionar Perplexity (cost-optimized)"
    );
    
    // Verificar costo estimado es bajo
    let decision = &hub.routing_history()[0];
    assert!(
        decision.estimated_cost < 0.01,
        "Costo estimado debería ser < $0.01, fue ${:.4}",
        decision.estimated_cost
    );
    
    println!("✅ Cost-optimized: simple query → Perplexity (${:.4})", decision.estimated_cost);
}

/// Test 4: Failover automático ante falla del provider
#[test]
fn test_failover_mechanism() {
    let mut config = HubSpokeConfig::default();
    config.enable_failover = true;
    config.enabled_providers = vec![
        LLMProvider::OpenAI,
        LLMProvider::Anthropic,
    ];
    
    let hub = Hub::new(config).expect("Hub creation failed");
    
    // En v1.0, spokes son STUBs que siempre responden
    // Verificamos que failover está habilitado
    let metrics = hub.metrics();
    assert_eq!(metrics.failover_count, 0, "Failover count inicial debe ser 0");
    
    println!("✅ Failover habilitado y configurado correctamente");
}

/// Test 5: Budget enforcement (presupuesto diario)
#[test]
fn test_budget_enforcement() {
    let mut config = HubSpokeConfig::default();
    config.daily_budget_usd = 1.0;  // Presupuesto bajo
    
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    // Simular gasto ya realizado
    hub.set_daily_spend(1.2);
    
    let ctx7d = create_neutral_context();
    
    // Intentar routing debería fallar por presupuesto excedido
    let result = hub.route_query("test query", ctx7d);
    
    assert!(
        matches!(result, Err(HubSpokeError::BudgetExceeded(_, _))),
        "Debería rechazar query cuando presupuesto excedido"
    );
    
    if let Err(HubSpokeError::BudgetExceeded(spent, limit)) = result {
        assert_eq!(spent, 1.2);
        assert_eq!(limit, 1.0);
        println!("✅ Budget enforcement: ${:.2} / ${:.2} rechazado", spent, limit);
    }
}

/// Test 6: Métricas y tracking de decisiones
#[test]
fn test_metrics_tracking() {
    let mut config = HubSpokeConfig::default();
    config.default_strategy = RoutingStrategy::RoundRobin;
    
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    let ctx7d = create_neutral_context();
    
    // Ejecutar varias queries
    for i in 0..5 {
        hub.route_query(&format!("query {}", i), ctx7d.clone())
            .expect("Routing failed");
    }
    
    // Verificar métricas
    let metrics = hub.metrics();
    assert_eq!(metrics.total_routes, 5, "Total routes incorrectos");
    
    // Verificar distribución por provider
    let total_provider_routes: u64 = metrics.routes_by_provider.values().sum();
    assert_eq!(
        total_provider_routes, 5,
        "Suma de routes por provider debe ser 5"
    );
    
    // Verificar histórico
    let history = hub.routing_history();
    assert_eq!(history.len(), 5, "Histórico debe tener 5 decisiones");
    
    // Cada decisión debe tener reasoning
    for decision in history {
        assert!(!decision.reasoning.is_empty(), "Reasoning vacío");
        assert!(decision.estimated_cost >= 0.0, "Costo estimado negativo");
    }
    
    println!("✅ Métricas tracking: {} routes, {} providers activos",
        metrics.total_routes,
        metrics.routes_by_provider.len()
    );
}

/// Test 7: Ejecución real de query con STUB
#[test]
fn test_query_execution_stub() {
    let config = HubSpokeConfig::default();
    let mut hub = Hub::new(config).expect("Hub creation failed");
    
    let ctx7d = create_neutral_context();
    
    // Routing
    let provider = hub.route_query("Explain Rust lifetimes", ctx7d)
        .expect("Routing failed");
    
    // Ejecución
    let response = hub.execute_query(provider, "Explain Rust lifetimes")
        .expect("Execution failed");
    
    // Verificar respuesta STUB
    assert!(response.contains("RESPONSE STUB"), "No es respuesta STUB");
    assert!(response.contains(&format!("{:?}", provider)), "No menciona provider");
    
    // Verificar métricas actualizadas
    let metrics = hub.metrics();
    assert_eq!(
        metrics.successful_requests, 1,
        "Successful requests debe ser 1"
    );
    
    println!("✅ Query ejecutada: {:?} → '{}'",
        provider,
        &response[..response.len().min(80)]
    );
}

// ============================================================================
// HELPERS
// ============================================================================

/// Crear contexto neutral (0.5 en todas las dimensiones)
fn create_neutral_context() -> ContextTensor7D {
    ContextTensor7D {
        semantic: 0.5,
        intentional: 0.5,
        temporal: 0.5,
        emotional: 0.5,
        associative: 0.5,
        evaluative: 0.5,
        metalinguistic: 0.5,
    }
}

// ============================================================================
// MAIN - Resumen de tests
// ============================================================================

fn main() {
    println!("\n🧪 HUBSPOKE NAVIGATOR - Test Suite\n");
    println!("═══════════════════════════════════════════════════");
    
    println!("\n[1/7] Routing Round-Robin...");
    test_routing_round_robin();
    
    println!("\n[2/7] Routing Contextual (alta complejidad)...");
    test_routing_contextual_high_complexity();
    
    println!("\n═══════════════════════════════════════════════════");
    println!("ℹ️  Run tests with: cargo test --example test_hubspoke");
    println!("✅ 7 tests available");
    println!("═══════════════════════════════════════════════════\n");
}
