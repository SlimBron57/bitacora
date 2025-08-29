//! # Template Repository Demo 🗃️
//!
//! Demo completo del Template Repository System integrado con BitaFlow Navigator

use std::collections::HashMap;
use bitacora_navigator::{
    HybridNavigator, BitaflowNavigatorEngine, TemplateRepository,
    template_repository::TemplateSearchFilters
};
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗃️  Template Repository System Demo!");
    println!();
    
    // Step 1: Crear directorio temporal para el repositorio
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let repo_path = temp_dir.path().join("bitaflow_templates");
    
    // Step 2: Inicializar repositorio
    println!("📂 Initializing template repository at: {:?}", repo_path);
    let mut repo = TemplateRepository::new(&repo_path).await?;
    
    // Step 3: Crear BitaFlow engine con Navigator
    let navigator = HybridNavigator::new_core()?;
    let mut bitaflow_engine = BitaflowNavigatorEngine::new(navigator)?;
    
    // Step 4: Cargar template de prueba
    println!("📋 Loading test template...");
    let test_bfl_content = r#"
# Error Debugging Template
alias: BITA-NAV-DEBUG-ERROR-TRACE-v2

## Context Variables
- error_message: Main error message
- stack_trace: Complete stack trace  
- error_file: Primary file where error occurred
- project_context: Current project context

## Navigation Steps
1. **Extract stack trace files** → `{{stack_trace_files}}`
2. **Identify error line** → `{{error_line}}`
3. **Map file dependencies** → `{{file_dependencies}}`
4. **Search for recent changes** → `{{recent_changes}}`

## Analysis Phase  
1. **Read error context** (±20 lines)
2. **Identify involved functions** → `{{error_functions}}`
3. **Find similar patterns** → `{{similar_patterns}}`
4. **Analyze related logs** → `{{related_logs}}`

## Interactive Questions
- Search closed similar issues? `{{ask_issue_history}}`
- Analyze commits touching these lines? `{{ask_commit_analysis}}`
- Run regression analysis? `{{ask_regression_analysis}}`

## Metrics
- execution_time: Total execution time
- files_analyzed: Number of files analyzed  
- patterns_found: Similar patterns detected
- suggestions_generated: Number of suggestions
- success_rate: Resolution success rate
"#;

    let template_alias = bitaflow_engine.load_template(test_bfl_content)?;
    println!("✅ Template loaded with alias: {}", template_alias);
    
    // Step 5: Obtener el template del BitaFlow engine para almacenarlo en el repo
    let templates = bitaflow_engine.list_templates();
    if let Some(template) = templates.first() {
        println!("📦 Storing template in repository...");
        
        // Clone the template to avoid borrowing issues
        let template_clone = (*template).clone();
        let stored_id = repo.store_template(template_clone.clone()).await?;
        println!("✅ Template stored with ID: {}", stored_id);
        
        // Step 6: Buscar templates en el repositorio
        println!();
        println!("🔍 Testing repository search functionality...");
        
        // Búsqueda por dominio
        println!("📋 Search by domain 'debug':");
        let search_filters = TemplateSearchFilters::new().domain("debug".to_string());
        let results = repo.search_templates(search_filters).await?;
        for result in &results {
            println!("  • {} (score: {:.2}) - {}", 
                result.template.template.name, 
                result.relevance_score,
                result.match_reason
            );
        }
        
        // Búsqueda de texto
        println!("📋 Search for 'error' text:");
        let search_filters = TemplateSearchFilters::new().text_search("error".to_string());
        let results = repo.search_templates(search_filters).await?;
        for result in &results {
            println!("  • {} (score: {:.2}) - {}", 
                result.template.template.name, 
                result.relevance_score,
                result.match_reason
            );
        }
        
        // Búsqueda por tags
        println!("📋 Search by tag 'debugging':");
        let search_filters = TemplateSearchFilters::new().tag("debugging".to_string());
        let results = repo.search_templates(search_filters).await?;
        for result in &results {
            println!("  • {} (score: {:.2}) - {}", 
                result.template.template.name, 
                result.relevance_score,
                result.match_reason
            );
        }
        
        // Step 7: Listar todos los templates
        println!();
        println!("📋 All templates in repository:");
        let all_templates = repo.list_all_templates().await;
        for template in all_templates {
            let tmpl = &template.template;
            let metadata = &template.storage_metadata;
            println!("  📄 {}", tmpl.name);
            println!("      Alias: {}", tmpl.alias);
            println!("      Domain/Topic: {}/{}", tmpl.domain, tmpl.topic);
            println!("      Author: {}", metadata.author);
            println!("      Created: {}", metadata.created_at.format("%Y-%m-%d %H:%M"));
            println!("      Tags: {:?}", template.tags);
            println!("      Success Rate: {:.1}%", tmpl.metrics.success_rate * 100.0);
            println!("      Usage Count: {}", tmpl.metrics.usage_count);
            println!();
        }
        
        // Step 8: Ejecutar template desde repositorio
        println!("🚀 Executing template from repository...");
        let context = HashMap::new();
        let execution_result = bitaflow_engine.execute_template(&template_clone.alias, context).await?;
        
        println!("✅ Template execution completed!");
        println!("   Success: {}", execution_result.success);
        println!("   Execution time: {:.3}s", execution_result.execution_time);
        println!("   Actions taken: {}", execution_result.actions_taken.len());
        
        // Step 9: Simular actualización de métricas
        println!("📊 Updating template metrics...");
        let mut new_metrics = template_clone.metrics.clone();
        new_metrics.usage_count += 1;
        new_metrics.success_rate = 0.95; // Simular alta tasa de éxito
        new_metrics.avg_execution_time = 1.2;
        
        repo.update_template_metrics(&stored_id, new_metrics).await?;
        println!("✅ Metrics updated successfully!");
        
        // Step 10: Verificar persistencia  
        println!("💾 Verifying persistence...");
        if let Some(updated_template) = repo.get_template(&stored_id).await? {
            println!("✅ Template persisted correctly:");
            println!("   Usage count: {}", updated_template.template.metrics.usage_count);
            println!("   Success rate: {:.1}%", updated_template.template.metrics.success_rate * 100.0);
            println!("   Avg execution time: {:.2}s", updated_template.template.metrics.avg_execution_time);
        }
    }
    
    println!();
    println!("🎉 Template Repository System demo completed successfully!");
    println!("📂 Repository created at: {:?}", repo_path);
    println!("📊 Features demonstrated:");
    println!("   ✅ Template storage and persistence");
    println!("   ✅ Advanced search with filters and scoring");
    println!("   ✅ Metadata management and indexing");
    println!("   ✅ Integration with BitaFlow execution");
    println!("   ✅ Metrics tracking and updates");
    println!("   ✅ Full filesystem persistence");
    
    Ok(())
}
