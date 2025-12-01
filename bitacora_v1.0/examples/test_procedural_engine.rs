//! # ProceduralEngine Test
//!
//! Test de ejecución paso a paso de recetas.
//!
//! Ejecutar con:
//! ```bash
//! cargo run --example test_procedural_engine
//! ```

use bitacora::shuidao::{
    Difficulty, ExecutionStatus, ProceduralAction, ProceduralRecipeEngine, RecipeCategory,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║     🔧 BITÁCORA - PROCEDURAL ENGINE TEST v1.0.0-beta     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    let engine = ProceduralRecipeEngine::new();

    // Mostrar recetas disponibles
    println!("📚 Available Recipes:\n");
    let recipes = engine.get_all_recipes().await;
    for (i, recipe) in recipes.iter().enumerate() {
        println!(
            "  {}. {} [{:?}] - {:?}",
            i + 1,
            recipe.name,
            recipe.category,
            recipe.difficulty
        );
        println!("     Steps: {} | Duration: ~{:?}\n", recipe.steps.len(), recipe.estimated_duration);
    }

    // Elegir receta
    print!("Choose recipe (1-{}): ", recipes.len());
    io::stdout().flush()?;
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice: usize = choice.trim().parse().unwrap_or(1);

    if choice < 1 || choice > recipes.len() {
        println!("❌ Invalid choice");
        return Ok(());
    }

    let selected_recipe = &recipes[choice - 1];
    println!("\n✅ Selected: {}\n", selected_recipe.name);

    // Iniciar ejecución
    println!("🚀 Starting recipe execution...\n");
    let execution = engine.start_recipe(&selected_recipe.id).await?;
    println!("📝 Execution ID: {}", execution.execution_id);
    println!("⏱️  Started at: {}\n", execution.started_at);

    // Loop de ejecución
    loop {
        let exec_state = engine.get_execution(&execution.execution_id).await?;

        if exec_state.status == ExecutionStatus::Completed {
            println!("\n╔══════════════════════════════════════════════════════════╗");
            println!("║                  ✅ RECIPE COMPLETED! 🎉                  ║");
            println!("╚══════════════════════════════════════════════════════════╝\n");
            println!("Total steps: {}", exec_state.step_history.len());
            println!("Duration: {:?}\n", chrono::Utc::now().signed_duration_since(exec_state.started_at));
            break;
        }

        if exec_state.status == ExecutionStatus::Failed {
            println!("\n❌ Recipe failed.");
            break;
        }

        // Obtener siguiente paso
        let response = engine.next_step(&execution.execution_id).await?;

        // Mostrar progreso
        println!("┌─────────────────────────────────────────────────────────┐");
        println!(
            "│ 📊 Progress: Step {}/{} ({:.1}%)                     ",
            response.progress.current,
            response.progress.total,
            response.progress.percentage
        );
        println!("└─────────────────────────────────────────────────────────┘\n");

        // Mostrar paso actual
        println!("🔹 Step {}: {}", response.current_step.number, response.current_step.instruction);

        if let Some(notes) = &response.current_step.notes {
            println!("   ℹ️  Note: {}", notes);
        }

        println!("\n   Validation: {:?}", response.current_step.validation);
        println!("   Can skip: {}", response.current_step.can_skip);
        println!("   ⏱️  Processing time: {:.2}ms", response.processing_time_ms);

        // Check performance
        if response.processing_time_ms > 5.0 {
            println!("\n   ⚠️  WARNING: Step took {:.2}ms (target <5ms)", response.processing_time_ms);
        }

        // Preguntar acción
        println!("\n   Actions:");
        println!("     [c] Complete step");
        if response.current_step.can_skip {
            println!("     [s] Skip step");
        }
        println!("     [p] Pause execution");
        println!("     [q] Quit");

        print!("\n   Your choice: ");
        io::stdout().flush()?;
        let mut action = String::new();
        io::stdin().read_line(&mut action)?;

        match action.trim().to_lowercase().as_str() {
            "c" => {
                let result = engine.validate_step(&execution.execution_id, true).await?;
                println!("\n   ✅ Step {} completed!", result.step_number + 1);
                println!("   Success: {}", result.success);
                println!();
            }
            "s" if response.current_step.can_skip => {
                let _ = engine.skip_step(&execution.execution_id).await?;
                println!("\n   ⏭️  Step skipped!\n");
            }
            "p" => {
                engine.pause_execution(&execution.execution_id).await?;
                println!("\n   ⏸️  Execution paused.");
                println!("   Press Enter to resume...");
                let mut _resume = String::new();
                io::stdin().read_line(&mut _resume)?;
                let _ = engine.resume_execution(&execution.execution_id).await?;
                println!("   ▶️  Execution resumed!\n");
            }
            "q" => {
                println!("\n   👋 Quitting...\n");
                break;
            }
            _ => {
                println!("\n   ❌ Invalid action. Try again.\n");
            }
        }
    }

    // Resumen final
    let final_exec = engine.get_execution(&execution.execution_id).await?;
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                     EXECUTION SUMMARY                     ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
    println!("Status: {:?}", final_exec.status);
    println!("Steps completed: {}/{}", final_exec.step_history.len(), selected_recipe.steps.len());
    println!("Success rate: {:.1}%", 
        (final_exec.step_history.iter().filter(|s| s.success).count() as f32 
         / final_exec.step_history.len() as f32) * 100.0
    );

    println!("\n📋 Step History:");
    for (i, step_result) in final_exec.step_history.iter().enumerate() {
        let icon = if step_result.success { "✅" } else { "❌" };
        println!(
            "  {}. {} Step {} - {}",
            i + 1,
            icon,
            step_result.step_number + 1,
            step_result.completed_at.format("%H:%M:%S")
        );
        if let Some(notes) = &step_result.notes {
            println!("     Note: {}", notes);
        }
    }

    println!("\n✨ Test completed successfully!\n");

    Ok(())
}
