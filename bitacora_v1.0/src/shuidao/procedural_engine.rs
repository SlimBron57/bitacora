//! # Procedural Recipe Engine
//!
//! Motor para ejecución guiada paso a paso (recipes, checklists, tutoriales).
//!
//! ## Responsabilidades
//!
//! 1. **StartRecipe**: Inicia ejecución de receta/procedimiento
//! 2. **NextStep**: Avanza al siguiente paso con validación
//! 3. **ValidateStep**: Verifica completion de paso actual
//! 4. **SkipStep**: Permite saltar pasos opcionales
//! 5. **TrackProgress**: Muestra estado actual de ejecución
//!
//! ## Modo Procedural
//!
//! Cuando el usuario dice:
//! - "¿cómo instalo un switch Cisco?" (paso a paso)
//! - "guíame en la configuración de nginx"
//! - "quiero una checklist para deployment"
//!
//! ShuiDao detecta intención **Procedural** y activa este engine.
//!
//! ## Diferencia con OperationalEngine
//!
//! | OperationalEngine | ProceduralEngine |
//! |-------------------|------------------|
//! | Proyectos largos  | Procedures cortos |
//! | Sub-proyectos libres | Pasos secuenciales |
//! | Usuario define tareas | Sistema define steps |
//! | "hacer X" | "¿cómo hacer X?" |
//!
//! ## Flujo de Ejecución
//!
//! ```text
//! Recipe: "Instalar Switch Cisco"
//!   Step 1: Verificar modelo [Done]
//!   Step 2: Conectar cable consola [Done]
//!   Step 3: Abrir terminal [InProgress] ← current
//!   Step 4: Configurar IP
//!   Step 5: Save config
//! ```
//!
//! **Performance Target**: <5ms next_step(), <50ms start_recipe()
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 2 Days 3-4)

use crate::shuidao::error::{Result, ShuiDaoError};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use uuid::Uuid;

// ============================================================================
// STRUCTURES
// ============================================================================

/// Procedural Recipe Engine
///
/// Gestiona ejecución de recetas paso a paso
pub struct ProceduralRecipeEngine {
    /// Recetas disponibles (recipe_id → Recipe)
    recipes: Arc<RwLock<HashMap<String, Recipe>>>,
    /// Ejecuciones activas (execution_id → RecipeExecution)
    active_executions: Arc<RwLock<HashMap<String, RecipeExecution>>>,
}

/// Receta/procedimiento estructurado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub category: RecipeCategory,
    pub steps: Vec<RecipeStep>,
    pub estimated_duration: Option<Duration>,
    pub difficulty: Difficulty,
    pub prerequisites: Vec<String>,
    pub tags: Vec<String>,
}

/// Paso de receta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub number: usize,
    pub instruction: String,
    pub validation: Option<StepValidation>,
    pub expected_duration: Option<Duration>,
    pub can_skip: bool,
    pub notes: Option<String>,
}

/// Validación de paso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepValidation {
    Manual,                          // Usuario confirma manualmente
    CommandOutput { command: String, expected: String },
    FileExists { path: String },
    ServiceRunning { service: String },
}

/// Categoría de receta
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecipeCategory {
    TechConfiguration,
    MechanicalInstructions,
    Cooking,
    Troubleshooting,
    Learning,
    Other(String),
}

/// Dificultad de receta
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Ejecución activa de receta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeExecution {
    pub execution_id: String,
    pub recipe_id: String,
    pub current_step: usize,
    pub started_at: DateTime<Utc>,
    pub step_history: Vec<StepResult>,
    pub status: ExecutionStatus,
    pub paused_at: Option<DateTime<Utc>>,
}

/// Resultado de paso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_number: usize,
    pub completed_at: DateTime<Utc>,
    pub success: bool,
    pub notes: Option<String>,
    pub duration_seconds: Option<i64>,
}

/// Estado de ejecución
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    InProgress,
    Completed,
    Failed,
    Paused,
}

/// Response del engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProceduralResponse {
    pub execution_id: String,
    pub current_step: RecipeStep,
    pub progress: StepProgress,
    pub next_action: ProceduralAction,
    pub processing_time_ms: f64,
}

/// Progreso de ejecución
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepProgress {
    pub current: usize,
    pub total: usize,
    pub percentage: f32,
    pub estimated_remaining: Option<Duration>,
}

/// Acción procedural
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProceduralAction {
    ShowCurrentStep,
    WaitForValidation,
    AdvanceToNext,
    RecipeCompleted,
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl ProceduralRecipeEngine {
    /// Crea nuevo engine con recetas demo
    pub fn new() -> Self {
        let mut recipes = HashMap::new();
        
        // Receta demo: Instalar Switch Cisco
        recipes.insert(
            "recipe_switch_cisco".to_string(),
            Self::create_demo_recipe_switch(),
        );
        
        // Receta demo: Configurar Nginx
        recipes.insert(
            "recipe_nginx_basic".to_string(),
            Self::create_demo_recipe_nginx(),
        );

        Self {
            recipes: Arc::new(RwLock::new(recipes)),
            active_executions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Inicia ejecución de receta
    ///
    /// # Performance
    /// Target: <50ms
    ///
    /// # Ejemplo
    /// ```rust
    /// let execution = engine.start_recipe("recipe_switch_cisco").await?;
    /// println!("Started: {}", execution.execution_id);
    /// ```
    pub async fn start_recipe(&self, recipe_id: &str) -> Result<RecipeExecution> {
        let start = Instant::now();

        // Validar que receta existe
        let recipes = self.recipes.read().unwrap();
        let recipe = recipes
            .get(recipe_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Recipe {} not found", recipe_id)))?;

        if recipe.steps.is_empty() {
            return Err(ShuiDaoError::InvalidInput(
                "Recipe has no steps".to_string(),
            ));
        }

        // Crear ejecución
        let execution = RecipeExecution {
            execution_id: Uuid::new_v4().to_string(),
            recipe_id: recipe_id.to_string(),
            current_step: 0, // Empezamos en step 0
            started_at: Utc::now(),
            step_history: Vec::new(),
            status: ExecutionStatus::InProgress,
            paused_at: None,
        };

        // Guardar ejecución
        let mut executions = self.active_executions.write().unwrap();
        executions.insert(execution.execution_id.clone(), execution.clone());

        let duration = start.elapsed().as_secs_f64() * 1000.0;
        if duration > 50.0 {
            eprintln!(
                "⚠️  ProceduralEngine::start_recipe() took {:.2}ms (target <50ms)",
                duration
            );
        }

        Ok(execution)
    }

    /// Avanza al siguiente paso
    ///
    /// # Performance
    /// Target: <5ms
    ///
    /// # Ejemplo
    /// ```rust
    /// let response = engine.next_step(&execution_id).await?;
    /// println!("Step {}: {}", response.current_step.number, response.current_step.instruction);
    /// ```
    pub async fn next_step(&self, execution_id: &str) -> Result<ProceduralResponse> {
        let start = Instant::now();

        // Obtener ejecución
        let mut executions = self.active_executions.write().unwrap();
        let execution = executions
            .get_mut(execution_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))?;

        // Validar estado
        if execution.status != ExecutionStatus::InProgress {
            return Err(ShuiDaoError::InvalidState(format!(
                "Execution is {:?}, cannot advance",
                execution.status
            )));
        }

        // Obtener receta
        let recipes = self.recipes.read().unwrap();
        let recipe = recipes
            .get(&execution.recipe_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Recipe {} not found", execution.recipe_id)))?;

        // Verificar si hay más pasos
        if execution.current_step >= recipe.steps.len() {
            return Err(ShuiDaoError::InvalidState(
                "No more steps available".to_string(),
            ));
        }

        let current_step = recipe.steps[execution.current_step].clone();

        // Calcular progreso
        let progress = StepProgress {
            current: execution.current_step + 1, // 1-indexed for display
            total: recipe.steps.len(),
            percentage: ((execution.current_step + 1) as f32 / recipe.steps.len() as f32) * 100.0,
            estimated_remaining: None, // TODO: Calculate based on step durations
        };

        // Determinar próxima acción
        let next_action = if current_step.validation.is_some() {
            ProceduralAction::WaitForValidation
        } else if execution.current_step + 1 < recipe.steps.len() {
            ProceduralAction::AdvanceToNext
        } else {
            ProceduralAction::RecipeCompleted
        };

        let duration = start.elapsed().as_secs_f64() * 1000.0;
        if duration > 5.0 {
            eprintln!(
                "⚠️  ProceduralEngine::next_step() took {:.2}ms (target <5ms)",
                duration
            );
        }

        Ok(ProceduralResponse {
            execution_id: execution_id.to_string(),
            current_step,
            progress,
            next_action,
            processing_time_ms: duration,
        })
    }

    /// Valida paso actual y avanza
    ///
    /// # Ejemplo
    /// ```rust
    /// let result = engine.validate_step(&execution_id, true).await?;
    /// println!("Step {} completed: {}", result.step_number, result.success);
    /// ```
    pub async fn validate_step(
        &self,
        execution_id: &str,
        validation_result: bool,
    ) -> Result<StepResult> {
        let mut executions = self.active_executions.write().unwrap();
        let execution = executions
            .get_mut(execution_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))?;

        // Crear resultado de paso
        let step_result = StepResult {
            step_number: execution.current_step,
            completed_at: Utc::now(),
            success: validation_result,
            notes: None,
            duration_seconds: None,
        };

        // Añadir a historial
        execution.step_history.push(step_result.clone());

        // Avanzar al siguiente paso si validación exitosa
        if validation_result {
            execution.current_step += 1;

            // Obtener receta para verificar si terminamos
            let recipes = self.recipes.read().unwrap();
            let recipe = recipes
                .get(&execution.recipe_id)
                .ok_or_else(|| ShuiDaoError::NotFound(format!("Recipe {} not found", execution.recipe_id)))?;

            if execution.current_step >= recipe.steps.len() {
                execution.status = ExecutionStatus::Completed;
            }
        } else {
            execution.status = ExecutionStatus::Failed;
        }

        Ok(step_result)
    }

    /// Salta paso actual (si permitido)
    ///
    /// # Ejemplo
    /// ```rust
    /// let response = engine.skip_step(&execution_id).await?;
    /// ```
    pub async fn skip_step(&self, execution_id: &str) -> Result<ProceduralResponse> {
        let mut executions = self.active_executions.write().unwrap();
        let execution = executions
            .get_mut(execution_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))?;

        // Obtener receta y paso actual
        let recipes = self.recipes.read().unwrap();
        let recipe = recipes
            .get(&execution.recipe_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Recipe {} not found", execution.recipe_id)))?;

        let current_step = &recipe.steps[execution.current_step];

        // Verificar si se puede saltar
        if !current_step.can_skip {
            return Err(ShuiDaoError::InvalidInput(
                "Current step cannot be skipped".to_string(),
            ));
        }

        // Añadir a historial como skipped
        let step_result = StepResult {
            step_number: execution.current_step,
            completed_at: Utc::now(),
            success: false,
            notes: Some("Skipped by user".to_string()),
            duration_seconds: None,
        };
        execution.step_history.push(step_result);

        // Avanzar
        execution.current_step += 1;

        // Verificar si terminamos
        if execution.current_step >= recipe.steps.len() {
            execution.status = ExecutionStatus::Completed;
        }

        // Return next step
        drop(executions); // Release lock before calling next_step
        self.next_step(execution_id).await
    }

    /// Pausa ejecución
    pub async fn pause_execution(&self, execution_id: &str) -> Result<()> {
        let mut executions = self.active_executions.write().unwrap();
        let execution = executions
            .get_mut(execution_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))?;

        execution.status = ExecutionStatus::Paused;
        execution.paused_at = Some(Utc::now());

        Ok(())
    }

    /// Resume ejecución pausada
    pub async fn resume_execution(&self, execution_id: &str) -> Result<ProceduralResponse> {
        let mut executions = self.active_executions.write().unwrap();
        let execution = executions
            .get_mut(execution_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))?;

        if execution.status != ExecutionStatus::Paused {
            return Err(ShuiDaoError::InvalidState(
                "Execution is not paused".to_string(),
            ));
        }

        execution.status = ExecutionStatus::InProgress;
        execution.paused_at = None;

        drop(executions); // Release lock
        self.next_step(execution_id).await
    }

    /// Busca recetas por categoría
    pub async fn find_recipes(&self, category: RecipeCategory) -> Vec<Recipe> {
        let recipes = self.recipes.read().unwrap();
        recipes
            .values()
            .filter(|r| r.category == category)
            .cloned()
            .collect()
    }

    /// Obtiene todas las recetas disponibles
    pub async fn get_all_recipes(&self) -> Vec<Recipe> {
        let recipes = self.recipes.read().unwrap();
        recipes.values().cloned().collect()
    }

    /// Obtiene ejecución por ID
    pub async fn get_execution(&self, execution_id: &str) -> Result<RecipeExecution> {
        let executions = self.active_executions.read().unwrap();
        executions
            .get(execution_id)
            .cloned()
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Execution {} not found", execution_id)))
    }
}

// ============================================================================
// DEMO RECIPES
// ============================================================================

impl ProceduralRecipeEngine {
    /// Crea receta demo: Instalar Switch Cisco
    fn create_demo_recipe_switch() -> Recipe {
        Recipe {
            id: "recipe_switch_cisco".to_string(),
            name: "Instalar y configurar switch Cisco".to_string(),
            category: RecipeCategory::TechConfiguration,
            steps: vec![
                RecipeStep {
                    number: 1,
                    instruction: "Verificar modelo del switch (buscar etiqueta en parte trasera)".to_string(),
                    validation: Some(StepValidation::Manual),
                    expected_duration: Some(Duration::minutes(2)),
                    can_skip: false,
                    notes: Some("Ejemplos: Catalyst 2960, 3560, etc.".to_string()),
                },
                RecipeStep {
                    number: 2,
                    instruction: "Conectar cable consola (RJ45 a USB) desde switch a laptop".to_string(),
                    validation: Some(StepValidation::Manual),
                    expected_duration: Some(Duration::minutes(3)),
                    can_skip: false,
                    notes: Some("Cable consola normalmente es azul claro".to_string()),
                },
                RecipeStep {
                    number: 3,
                    instruction: "Abrir terminal serie (PuTTY en Windows, screen en Linux/Mac)".to_string(),
                    validation: Some(StepValidation::Manual),
                    expected_duration: Some(Duration::minutes(2)),
                    can_skip: false,
                    notes: Some("Configuración: 9600 baud, 8N1, sin control de flujo".to_string()),
                },
                RecipeStep {
                    number: 4,
                    instruction: "Configurar IP de management: config t, interface vlan 1, ip address X.X.X.X 255.255.255.0".to_string(),
                    validation: Some(StepValidation::Manual),
                    expected_duration: Some(Duration::minutes(5)),
                    can_skip: false,
                    notes: Some("Usar IP de tu red local".to_string()),
                },
                RecipeStep {
                    number: 5,
                    instruction: "Guardar configuración: write memory o copy running-config startup-config".to_string(),
                    validation: Some(StepValidation::Manual),
                    expected_duration: Some(Duration::minutes(1)),
                    can_skip: false,
                    notes: Some("Confirmar con 'show running-config'".to_string()),
                },
            ],
            estimated_duration: Some(Duration::minutes(15)),
            difficulty: Difficulty::Intermediate,
            prerequisites: vec![
                "Cable consola RJ45-USB".to_string(),
                "Software terminal (PuTTY/screen)".to_string(),
                "Switch Cisco".to_string(),
            ],
            tags: vec!["cisco".to_string(), "networking".to_string(), "switch".to_string()],
        }
    }

    /// Crea receta demo: Configurar Nginx
    fn create_demo_recipe_nginx() -> Recipe {
        Recipe {
            id: "recipe_nginx_basic".to_string(),
            name: "Configurar Nginx básico".to_string(),
            category: RecipeCategory::TechConfiguration,
            steps: vec![
                RecipeStep {
                    number: 1,
                    instruction: "Instalar nginx: sudo apt install nginx (Ubuntu/Debian) o sudo yum install nginx (CentOS/RHEL)".to_string(),
                    validation: Some(StepValidation::CommandOutput {
                        command: "nginx -v".to_string(),
                        expected: "nginx version".to_string(),
                    }),
                    expected_duration: Some(Duration::minutes(5)),
                    can_skip: false,
                    notes: Some("Verificar con: nginx -v".to_string()),
                },
                RecipeStep {
                    number: 2,
                    instruction: "Crear archivo de configuración en /etc/nginx/sites-available/mi-sitio".to_string(),
                    validation: Some(StepValidation::FileExists {
                        path: "/etc/nginx/sites-available/mi-sitio".to_string(),
                    }),
                    expected_duration: Some(Duration::minutes(3)),
                    can_skip: false,
                    notes: None,
                },
                RecipeStep {
                    number: 3,
                    instruction: "Crear symlink: sudo ln -s /etc/nginx/sites-available/mi-sitio /etc/nginx/sites-enabled/".to_string(),
                    validation: Some(StepValidation::FileExists {
                        path: "/etc/nginx/sites-enabled/mi-sitio".to_string(),
                    }),
                    expected_duration: Some(Duration::minutes(1)),
                    can_skip: false,
                    notes: None,
                },
                RecipeStep {
                    number: 4,
                    instruction: "Validar configuración: sudo nginx -t".to_string(),
                    validation: Some(StepValidation::CommandOutput {
                        command: "sudo nginx -t".to_string(),
                        expected: "syntax is ok".to_string(),
                    }),
                    expected_duration: Some(Duration::minutes(1)),
                    can_skip: false,
                    notes: Some("Debe decir 'syntax is ok' y 'test is successful'".to_string()),
                },
                RecipeStep {
                    number: 5,
                    instruction: "Recargar nginx: sudo systemctl reload nginx".to_string(),
                    validation: Some(StepValidation::ServiceRunning {
                        service: "nginx".to_string(),
                    }),
                    expected_duration: Some(Duration::minutes(1)),
                    can_skip: false,
                    notes: Some("Verificar con: sudo systemctl status nginx".to_string()),
                },
            ],
            estimated_duration: Some(Duration::minutes(15)),
            difficulty: Difficulty::Beginner,
            prerequisites: vec![
                "Servidor Linux".to_string(),
                "Acceso sudo".to_string(),
            ],
            tags: vec!["nginx".to_string(), "web-server".to_string(), "linux".to_string()],
        }
    }
}

impl Default for ProceduralRecipeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_recipe() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_switch_cisco").await.unwrap();

        assert_eq!(execution.recipe_id, "recipe_switch_cisco");
        assert_eq!(execution.current_step, 0);
        assert_eq!(execution.status, ExecutionStatus::InProgress);
    }

    #[tokio::test]
    async fn test_next_step() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_switch_cisco").await.unwrap();
        let response = engine.next_step(&execution.execution_id).await.unwrap();

        assert_eq!(response.current_step.number, 1);
        assert_eq!(response.progress.current, 1);
        assert_eq!(response.progress.total, 5);
        assert!(response.processing_time_ms < 5.0);
    }

    #[tokio::test]
    async fn test_validate_step() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_switch_cisco").await.unwrap();

        // Get first step
        let _ = engine.next_step(&execution.execution_id).await.unwrap();

        // Validate it
        let result = engine.validate_step(&execution.execution_id, true).await.unwrap();
        assert_eq!(result.step_number, 0);
        assert!(result.success);

        // Check that current_step advanced
        let updated = engine.get_execution(&execution.execution_id).await.unwrap();
        assert_eq!(updated.current_step, 1);
    }

    #[tokio::test]
    async fn test_skip_step() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_nginx_basic").await.unwrap();

        // Get first step
        let _ = engine.next_step(&execution.execution_id).await.unwrap();

        // Try to skip (note: demo recipes have can_skip=false, so this will fail)
        let result = engine.skip_step(&execution.execution_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pause_resume() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_switch_cisco").await.unwrap();

        // Pause
        engine.pause_execution(&execution.execution_id).await.unwrap();
        let paused = engine.get_execution(&execution.execution_id).await.unwrap();
        assert_eq!(paused.status, ExecutionStatus::Paused);

        // Resume
        let _ = engine.resume_execution(&execution.execution_id).await.unwrap();
        let resumed = engine.get_execution(&execution.execution_id).await.unwrap();
        assert_eq!(resumed.status, ExecutionStatus::InProgress);
    }

    #[tokio::test]
    async fn test_find_recipes() {
        let engine = ProceduralRecipeEngine::new();
        let recipes = engine.find_recipes(RecipeCategory::TechConfiguration).await;

        assert!(recipes.len() >= 2); // At least switch and nginx
        assert!(recipes.iter().any(|r| r.id == "recipe_switch_cisco"));
        assert!(recipes.iter().any(|r| r.id == "recipe_nginx_basic"));
    }

    #[tokio::test]
    async fn test_recipe_completion() {
        let engine = ProceduralRecipeEngine::new();
        let execution = engine.start_recipe("recipe_switch_cisco").await.unwrap();

        // Complete all steps
        for _ in 0..5 {
            let _ = engine.next_step(&execution.execution_id).await.unwrap();
            engine.validate_step(&execution.execution_id, true).await.unwrap();
        }

        let completed = engine.get_execution(&execution.execution_id).await.unwrap();
        assert_eq!(completed.status, ExecutionStatus::Completed);
        assert_eq!(completed.step_history.len(), 5);
    }
}
