use bitacora_navigator::bitaflow::{BitaflowNavigatorEngine, AliasValidator};
use bitacora_navigator::core::HybridNavigator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 Probando BitaFlow Navigator Integration!");
    
    // Crear navigator
    let navigator = HybridNavigator::new_core()?;
    let mut bitaflow_engine = BitaflowNavigatorEngine::new(navigator)?;
    
    // Probar AliasValidator
    println!("\n📋 Testing AliasValidator:");
    let validator = AliasValidator::new();
    
    // Generar alias
    let alias = validator.generate_navigator_alias("debug", "error")?;
    println!("✅ Generated alias: {}", alias);
    
    // Parsear alias
    let (domain, topic, version) = validator.parse_alias(&alias)?;
    println!("✅ Parsed - Domain: {}, Topic: {}, Version: {}", domain, topic, version);
    
    // Incrementar versión
    let next_alias = validator.increment_version(&alias)?;
    println!("✅ Next version: {}", next_alias);
    
    // Cargar template desde archivo
    println!("\n🎯 Testing Template Loading:");
    let template_path = "/home/edgi/Documents/Development/own/bitacora/bitacora_v0.1/bitacora-rust/crates/bitacora-navigator/templates/debug-error-trace.bfl";
    
    match bitaflow_engine.load_template_from_file(template_path).await {
        Ok(loaded_alias) => {
            println!("✅ Template loaded successfully with alias: {}", loaded_alias);
            
            // Listar templates
            let templates = bitaflow_engine.list_templates();
            println!("📊 Total templates loaded: {}", templates.len());
            
            for template in templates {
                println!("  - {}: {} (v{})", template.alias, template.name, template.version);
            }
            
            // 🚀 PROBAR EJECUCIÓN DE TEMPLATE
            println!("\n🚀 Testing Template Execution:");
            let mut context = std::collections::HashMap::new();
            context.insert("error_type".to_string(), "NullPointerException".to_string());
            context.insert("severity".to_string(), "HIGH".to_string());
            
            match bitaflow_engine.execute_template(&loaded_alias, context).await {
                Ok(result) => {
                    println!("✅ Template execution successful!");
                    println!("   Success: {}", result.success);
                    println!("   Execution time: {:.3}s", result.execution_time);
                    println!("   Actions taken: {}", result.actions_taken.len());
                    for action in &result.actions_taken {
                        println!("     • {}", action);
                    }
                    println!("   Output: {}", result.output);
                },
                Err(e) => {
                    println!("❌ Template execution failed: {}", e);
                }
            }
        },
        Err(e) => {
            println!("⚠️  Template loading failed: {}", e);
            println!("   Creating template programmatically...");
            
            // Crear template simple para probar
            let simple_template = r#"---
name: "Simple Debug Navigator"
alias: "BITA-NAV-DEBUG-SIMPLE-v1"
domain: "debug"
topic: "simple"
version: 1
autonomy_level: "Interactive"
thread_level: "ProjectIsolated"
---

# Simple Debug Template
## Variables
- {{error_type}}: The type of error to debug
- {{severity}}: Error severity level

## Navigation Flow
1. Analyze error context
2. Identify potential causes  
3. Suggest resolution steps

## Metrics
- success_rate: 0.0
- avg_execution_time: 0.0
"#;
            
            let alias = bitaflow_engine.load_template(simple_template)?;
            println!("✅ Created simple template with alias: {}", alias);
        }
    }
    
    println!("\n🎉 BitaFlow Navigator Integration test completed!");
    Ok(())
}
