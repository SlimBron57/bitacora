//! # AI Generator Demo 🤖⚡
//!
//! Demo completo del AI Template Generator con múltiples providers

use bitacora_ai_generator::{
    AITemplateGenerator, 
    GenerationRequest, 
    TemplateGeneratorPlugin,
    providers::MockProvider,
};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖⚡ AI Template Generator Demo");
    println!("================================");
    
    // Demo with Mock Provider
    demo_mock_provider().await?;
    
    // Demo with error generation
    demo_error_log_generation().await?;
    
    // Demo with multiple variations
    demo_multiple_generations().await?;
    
    // Demo template improvement
    demo_template_improvement().await?;
    
    println!("\n🎉 All AI Generator demos completed successfully!");
    
    Ok(())
}

async fn demo_mock_provider() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 DEMO 1: Mock Provider Basic Generation");
    println!("------------------------------------------");
    
    let mock_provider = Box::new(MockProvider::new());
    let generator = AITemplateGenerator::new(mock_provider)?;
    
    println!("✓ AI Generator initialized with Mock provider");
    
    // Check availability
    let available = generator.is_available().await;
    println!("✓ Provider available: {}", available);
    
    // Basic generation request
    let request = GenerationRequest {
        description: "Debug a memory leak in Rust application".to_string(),
        domain: "debug".to_string(),
        topic: "memory_leak".to_string(),
        autonomy_level: "Interactive".to_string(),
        context: HashMap::from([
            ("language".to_string(), "Rust".to_string()),
            ("issue_type".to_string(), "Memory Management".to_string()),
            ("severity".to_string(), "High".to_string()),
        ]),
        reference_templates: vec![],
        constraints: vec![
            "Focus on Rust-specific memory patterns".to_string(),
            "Include valgrind integration".to_string(),
            "Provide heap analysis steps".to_string(),
        ],
    };
    
    let result = generator.generate_template(request).await?;
    
    println!("✅ Template generated successfully!");
    println!("   📝 Alias: {}", result.alias);
    println!("   🎯 Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("   ⏱️  Generation time: {:.3}s", result.metadata.generation_time);
    println!("   🔧 Provider: {}", result.metadata.provider);
    
    println!("\n📄 Generated Template (first 200 chars):");
    println!("{}", result.bfl_content.chars().take(200).collect::<String>());
    if result.bfl_content.len() > 200 {
        println!("... (truncated)");
    }
    
    if !result.improvements.is_empty() {
        println!("\n💡 Suggested improvements:");
        for improvement in &result.improvements {
            println!("   - {}", improvement);
        }
    }
    
    Ok(())
}

async fn demo_error_log_generation() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔥 DEMO 2: Error Log Analysis Generation");
    println!("------------------------------------------");
    
    let mock_provider = Box::new(MockProvider::new());
    let generator = AITemplateGenerator::new(mock_provider)?;
    
    let error_logs = r#"
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 5', src/main.rs:42:9
stack backtrace:
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic_bounds_check
   3: bitacora_core::process_records
   4: bitacora_core::main
"#;
    
    let project_context = "BitaFlow Navigator - Rust application for workflow automation";
    
    let result = generator.generate_from_error_logs(error_logs, project_context).await?;
    
    println!("✅ Error-specific template generated!");
    println!("   📝 Alias: {}", result.alias);
    println!("   🎯 Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("   🔍 Explanation: {}", result.explanation);
    
    Ok(())
}

async fn demo_multiple_generations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎨 DEMO 3: Multiple Template Variations (A/B Testing)");
    println!("------------------------------------------------------");
    
    let mock_provider = Box::new(MockProvider::new());
    let generator = AITemplateGenerator::new(mock_provider)?;
    
    let request = GenerationRequest {
        description: "Setup testing environment for API endpoints".to_string(),
        domain: "test".to_string(),
        topic: "api_testing".to_string(),
        autonomy_level: "Autonomous".to_string(),
        context: HashMap::from([
            ("framework".to_string(), "Rust + Axum".to_string()),
            ("test_type".to_string(), "Integration".to_string()),
        ]),
        reference_templates: vec![],
        constraints: vec!["Include setup and teardown steps".to_string()],
    };
    
    let variations = generator.generate_multiple_templates(request, 3).await?;
    
    println!("✅ Generated {} template variations:", variations.len());
    
    for (i, variation) in variations.iter().enumerate() {
        println!("   📋 Variation {}: {} (confidence: {:.1}%)", 
                 i + 1, 
                 variation.alias,
                 variation.confidence_score * 100.0);
    }
    
    // Find best variation
    let best = variations.iter()
        .max_by(|a, b| a.confidence_score.partial_cmp(&b.confidence_score).unwrap())
        .unwrap();
    
    println!("🏆 Best variation: {} with {:.1}% confidence", 
             best.alias, 
             best.confidence_score * 100.0);
    
    Ok(())
}

async fn demo_template_improvement() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚡ DEMO 4: Template Improvement");
    println!("--------------------------------");
    
    let mock_provider = Box::new(MockProvider::new());
    let generator = AITemplateGenerator::new(mock_provider)?;
    
    let existing_template = r#"
alias: BASIC-DEBUG-v1

## Steps
1. Look at error
2. Fix error
3. Test fix
"#;
    
    let improvements = vec![
        "Add systematic debugging approach".to_string(),
        "Include logging and monitoring steps".to_string(),
        "Add rollback procedures".to_string(),
    ];
    
    let result = generator.improve_template(existing_template, improvements).await?;
    
    println!("✅ Template improved successfully!");
    println!("   📝 New alias: {}", result.alias);
    println!("   🎯 Confidence: {:.1}%", result.confidence_score * 100.0);
    println!("   🔧 Explanation: {}", result.explanation);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitacora_ai_generator::providers::MockProvider;
    
    #[tokio::test]
    async fn test_basic_generation() {
        let mock_provider = Box::new(MockProvider::new());
        let generator = AITemplateGenerator::new(mock_provider).unwrap();
        
        let request = GenerationRequest {
            description: "Test template".to_string(),
            domain: "test".to_string(),
            topic: "basic".to_string(),
            autonomy_level: "Interactive".to_string(),
            context: HashMap::new(),
            reference_templates: vec![],
            constraints: vec![],
        };
        
        let result = generator.generate_template(request).await;
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(!result.bfl_content.is_empty());
        assert!(!result.alias.is_empty());
        assert!(result.confidence_score > 0.0);
        assert!(result.confidence_score <= 1.0);
    }
    
    #[tokio::test]
    async fn test_provider_availability() {
        let mock_provider = Box::new(MockProvider::new());
        let generator = AITemplateGenerator::new(mock_provider).unwrap();
        
        let available = generator.is_available().await;
        assert!(available);
    }
    
    #[tokio::test]
    async fn test_multiple_generations() {
        let mock_provider = Box::new(MockProvider::new());
        let generator = AITemplateGenerator::new(mock_provider).unwrap();
        
        let request = GenerationRequest {
            description: "Multi test".to_string(),
            domain: "test".to_string(),
            topic: "multi".to_string(),
            autonomy_level: "Interactive".to_string(),
            context: HashMap::new(),
            reference_templates: vec![],
            constraints: vec![],
        };
        
        let results = generator.generate_multiple_templates(request, 2).await;
        assert!(results.is_ok());
        
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
    }
}
