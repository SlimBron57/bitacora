//! # Learning Engine
//!
//! Motor para rutas adaptativas de aprendizaje.
//!
//! ## Responsabilidades
//!
//! 1. **LearningPaths**: Crear rutas personalizadas con módulos progresivos
//! 2. **ProgressTracking**: Seguimiento de mastery, tiempo invertido, confusion points
//! 3. **AdaptiveRecommendations**: Sugerir próximo módulo según nivel actual
//! 4. **ConfusionDetection**: Detectar bloqueos y ajustar dificultad
//!
//! ## Modo Learning
//!
//! Cuando el usuario dice:
//! - "quiero aprender Rust"
//! - "necesito entender redes"
//! - "ayúdame a mejorar en Docker"
//!
//! ShuiDao detecta intención **Learning** y activa este engine.
//!
//! ## Jerarquía
//!
//! ```text
//! LearningPath: "Aprende Rust desde 0"
//!   ├─ Module 1: Fundamentos (Mastery: 0.85) ✅
//!   │  ├─ Checkpoint 1.1: Variables (Done)
//!   │  ├─ Checkpoint 1.2: Tipos (Done)
//!   │  └─ Checkpoint 1.3: Control Flow (Done)
//!   ├─ Module 2: Ownership (Mastery: 0.45) 🔄
//!   │  ├─ Checkpoint 2.1: Borrow Checker (InProgress)
//!   │  ├─ Checkpoint 2.2: Lifetimes (Blocked - confusion detected)
//!   │  └─ Checkpoint 2.3: Rc/Arc (Not Started)
//!   └─ Module 3: Concurrency (Not Started)
//! ```
//!
//! **Performance Target**: <180ms engine processing
//!
//! **Versión**: 1.0.0-beta  
//! **Fecha**: 2025-11-24 (Week 3 Day 5)

use crate::shuidao::error::{Result, ShuiDaoError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;
use uuid::Uuid;

// ============================================================================
// STRUCTURES
// ============================================================================

/// Learning Engine - Adaptive learning paths
pub struct LearningEngine {
    /// Learning paths activos (path_id → LearningPath)
    learning_paths: Arc<RwLock<HashMap<String, LearningPath>>>,
    
    /// Confusion threshold (detectar cuando user está bloqueado)
    confusion_threshold: f32, // Default: 0.3 (30% de errores)
}

/// Learning Path completo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub difficulty: Difficulty,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub status: PathStatus,
    
    // Progress
    pub modules: Vec<Module>,
    pub current_module: Option<String>, // Module ID actual
    pub overall_mastery: f32, // 0.0-1.0 promedio de todos los módulos
    
    // Tracking
    pub total_time_minutes: u32,
    pub confusion_points: Vec<ConfusionPoint>,
    
    // Metadata
    pub estimated_hours: Option<f32>,
    pub tags: Vec<String>,
}

/// Módulo de aprendizaje (parte de un LearningPath)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub difficulty: Difficulty,
    pub status: ModuleStatus,
    pub mastery_level: f32, // 0.0-1.0 (mastery actual del user)
    
    // Contenido
    pub checkpoints: Vec<Checkpoint>,
    pub dependencies: Vec<String>, // Module IDs que deben completarse primero
    
    // Tracking
    pub attempts: u32,
    pub time_spent_minutes: u32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Checkpoint dentro de un módulo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub description: String,
    pub status: CheckpointStatus,
    pub mastery_score: f32, // 0.0-1.0
    pub attempts: u32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Punto de confusión detectado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionPoint {
    pub module_id: String,
    pub checkpoint_id: Option<String>,
    pub detected_at: DateTime<Utc>,
    pub error_rate: f32, // 0.0-1.0
    pub attempts_before_success: u32,
    pub notes: String,
}

/// Recomendación adaptativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveRecommendation {
    pub recommendation_type: RecommendationType,
    pub module_id: String,
    pub reason: String,
    pub confidence: f32, // 0.0-1.0
}

/// Tipo de recomendación
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RecommendationType {
    Continue,      // Seguir con próximo checkpoint
    Review,        // Repasar módulo actual (mastery bajo)
    SlowDown,      // Reducir velocidad (confusion detectada)
    Advance,       // Avanzar a siguiente módulo (mastery alto)
    TakeBreak,     // Descansar (demasiado tiempo continuo)
}

/// Dificultad de módulo/path
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Estado del path
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathStatus {
    NotStarted,
    InProgress,
    Completed,
    Paused,
}

/// Estado del módulo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleStatus {
    Locked,       // Dependencias no completadas
    Available,    // Listo para empezar
    InProgress,   // Actualmente trabajando
    Completed,    // Mastery >= 0.7
    Mastered,     // Mastery >= 0.9
}

/// Estado del checkpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckpointStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

/// Response del engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResponse {
    pub path_id: String,
    pub action: LearningAction,
    pub recommendations: Vec<AdaptiveRecommendation>,
    pub progress_summary: String,
    pub processing_time_ms: f64,
}

/// Acción ejecutada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningAction {
    PathCreated { name: String, modules: usize },
    CheckpointCompleted { checkpoint: String, mastery: f32 },
    ModuleCompleted { module: String, mastery: f32 },
    ConfusionDetected { module: String, recommendation: String },
    PathCompleted { name: String, overall_mastery: f32 },
}

// ============================================================================
// IMPLEMENTATION
// ============================================================================

impl LearningEngine {
    /// Crear nuevo LearningEngine
    pub fn new() -> Self {
        Self {
            learning_paths: Arc::new(RwLock::new(HashMap::new())),
            confusion_threshold: 0.3, // 30% error rate triggers confusion
        }
    }
    
    /// Crear nuevo learning path
    pub fn create_learning_path(
        &mut self,
        name: String,
        topic: String,
        difficulty: Difficulty,
        modules: Vec<(String, Difficulty, Vec<String>)>, // (name, difficulty, checkpoints)
    ) -> Result<LearningResponse> {
        let start = Instant::now();
        let path_id = Uuid::new_v4().to_string();
        
        // Crear módulos
        let mut created_modules = Vec::new();
        for (i, (module_name, module_difficulty, checkpoint_descriptions)) in modules.iter().enumerate() {
            let module_id = format!("{}_{}", path_id, i);
            let mut checkpoints = Vec::new();
            
            for (j, desc) in checkpoint_descriptions.iter().enumerate() {
                checkpoints.push(Checkpoint {
                    id: format!("{}_{}", module_id, j),
                    description: desc.clone(),
                    status: CheckpointStatus::NotStarted,
                    mastery_score: 0.0,
                    attempts: 0,
                    created_at: Utc::now(),
                    completed_at: None,
                });
            }
            
            created_modules.push(Module {
                id: module_id,
                name: module_name.clone(),
                difficulty: module_difficulty.clone(),
                status: if i == 0 { ModuleStatus::Available } else { ModuleStatus::Locked },
                mastery_level: 0.0,
                checkpoints,
                dependencies: if i > 0 {
                    vec![format!("{}_{}", path_id, i - 1)]
                } else {
                    vec![]
                },
                attempts: 0,
                time_spent_minutes: 0,
                created_at: Utc::now(),
                completed_at: None,
            });
        }
        
        let path = LearningPath {
            id: path_id.clone(),
            name: name.clone(),
            topic,
            difficulty,
            created_at: Utc::now(),
            last_updated: Utc::now(),
            status: PathStatus::NotStarted,
            modules: created_modules.clone(),
            current_module: Some(created_modules[0].id.clone()),
            overall_mastery: 0.0,
            total_time_minutes: 0,
            confusion_points: Vec::new(),
            estimated_hours: Some((created_modules.len() as f32) * 2.0), // 2h per module estimate
            tags: Vec::new(),
        };
        
        self.learning_paths.write().unwrap().insert(path_id.clone(), path);
        
        Ok(LearningResponse {
            path_id: path_id.clone(),
            action: LearningAction::PathCreated {
                name,
                modules: created_modules.len(),
            },
            recommendations: vec![AdaptiveRecommendation {
                recommendation_type: RecommendationType::Continue,
                module_id: created_modules[0].id.clone(),
                reason: "Start with first module".to_string(),
                confidence: 1.0,
            }],
            progress_summary: format!("Learning path created with {} modules", created_modules.len()),
            processing_time_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }
    
    /// Completar checkpoint
    pub fn complete_checkpoint(
        &mut self,
        path_id: &str,
        checkpoint_id: &str,
        mastery_score: f32,
    ) -> Result<LearningResponse> {
        let start = Instant::now();
        
        let mut paths = self.learning_paths.write().unwrap();
        let path = paths.get_mut(path_id)
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Path {} not found", path_id)))?;
        
        // Find and update checkpoint
        let mut checkpoint_found = false;
        let mut module_id = String::new();
        
        for module in &mut path.modules {
            for checkpoint in &mut module.checkpoints {
                if checkpoint.id == checkpoint_id {
                    checkpoint.status = CheckpointStatus::Completed;
                    checkpoint.mastery_score = mastery_score;
                    checkpoint.completed_at = Some(Utc::now());
                    checkpoint.attempts += 1;
                    checkpoint_found = true;
                    module_id = module.id.clone();
                    break;
                }
            }
            if checkpoint_found { break; }
        }
        
        if !checkpoint_found {
            return Err(ShuiDaoError::NotFound(format!("Checkpoint {} not found", checkpoint_id)));
        }
        
        // Recalculate module mastery
        self.recalculate_module_mastery(path, &module_id);
        
        // Detect confusion if needed
        let recommendations = if mastery_score < self.confusion_threshold {
            self.detect_confusion(path, &module_id, checkpoint_id)?
        } else {
            self.get_adaptive_recommendations(path)?
        };
        
        path.last_updated = Utc::now();
        
        Ok(LearningResponse {
            path_id: path_id.to_string(),
            action: LearningAction::CheckpointCompleted {
                checkpoint: checkpoint_id.to_string(),
                mastery: mastery_score,
            },
            recommendations,
            progress_summary: self.generate_progress_summary(path),
            processing_time_ms: start.elapsed().as_secs_f64() * 1000.0,
        })
    }
    
    /// Recalcular mastery de módulo
    fn recalculate_module_mastery(&self, path: &mut LearningPath, module_id: &str) {
        if let Some(module) = path.modules.iter_mut().find(|m| m.id == module_id) {
            let total_checkpoints = module.checkpoints.len() as f32;
            if total_checkpoints == 0.0 { return; }
            
            let sum_mastery: f32 = module.checkpoints.iter()
                .map(|c| c.mastery_score)
                .sum();
            
            module.mastery_level = sum_mastery / total_checkpoints;
            
            // Update module status based on mastery
            if module.mastery_level >= 0.9 {
                module.status = ModuleStatus::Mastered;
                module.completed_at = Some(Utc::now());
            } else if module.mastery_level >= 0.7 {
                module.status = ModuleStatus::Completed;
                module.completed_at = Some(Utc::now());
            } else if module.mastery_level > 0.0 {
                module.status = ModuleStatus::InProgress;
            }
            
            // Unlock next module if current is completed
            if module.status == ModuleStatus::Completed || module.status == ModuleStatus::Mastered {
                self.unlock_next_module(path, module_id);
            }
        }
        
        // Recalculate overall path mastery
        let total_modules = path.modules.len() as f32;
        if total_modules > 0.0 {
            let sum_mastery: f32 = path.modules.iter()
                .map(|m| m.mastery_level)
                .sum();
            path.overall_mastery = sum_mastery / total_modules;
            
            // Check if path is completed
            if path.modules.iter().all(|m| m.status == ModuleStatus::Completed || m.status == ModuleStatus::Mastered) {
                path.status = PathStatus::Completed;
            } else if path.overall_mastery > 0.0 {
                path.status = PathStatus::InProgress;
            }
        }
    }
    
    /// Desbloquear próximo módulo
    fn unlock_next_module(&self, path: &mut LearningPath, completed_module_id: &str) {
        // First, collect module IDs that should be unlocked
        let mut modules_to_unlock = Vec::new();
        
        for module in path.modules.iter() {
            if module.dependencies.contains(&completed_module_id.to_string()) && module.status == ModuleStatus::Locked {
                // Check if all dependencies are met
                let all_deps_met = module.dependencies.iter().all(|dep_id| {
                    path.modules.iter().any(|m| {
                        m.id == *dep_id && (m.status == ModuleStatus::Completed || m.status == ModuleStatus::Mastered)
                    })
                });
                
                if all_deps_met {
                    modules_to_unlock.push(module.id.clone());
                }
            }
        }
        
        // Now unlock the collected modules
        for module_id in modules_to_unlock {
            if let Some(module) = path.modules.iter_mut().find(|m| m.id == module_id) {
                module.status = ModuleStatus::Available;
            }
        }
    }
    
    /// Detectar confusión
    fn detect_confusion(
        &self,
        path: &mut LearningPath,
        module_id: &str,
        checkpoint_id: &str,
    ) -> Result<Vec<AdaptiveRecommendation>> {
        let confusion = ConfusionPoint {
            module_id: module_id.to_string(),
            checkpoint_id: Some(checkpoint_id.to_string()),
            detected_at: Utc::now(),
            error_rate: 0.7, // High error rate
            attempts_before_success: 3,
            notes: "Low mastery score detected".to_string(),
        };
        
        path.confusion_points.push(confusion);
        
        Ok(vec![AdaptiveRecommendation {
            recommendation_type: RecommendationType::Review,
            module_id: module_id.to_string(),
            reason: "Low mastery detected - review recommended".to_string(),
            confidence: 0.85,
        }])
    }
    
    /// Obtener recomendaciones adaptativas
    fn get_adaptive_recommendations(&self, path: &LearningPath) -> Result<Vec<AdaptiveRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Find current module
        if let Some(current_module_id) = &path.current_module {
            if let Some(module) = path.modules.iter().find(|m| m.id == *current_module_id) {
                match module.status {
                    ModuleStatus::InProgress => {
                        recommendations.push(AdaptiveRecommendation {
                            recommendation_type: RecommendationType::Continue,
                            module_id: module.id.clone(),
                            reason: format!("Continue with current module (mastery: {:.0}%)", module.mastery_level * 100.0),
                            confidence: 0.9,
                        });
                    }
                    ModuleStatus::Completed | ModuleStatus::Mastered => {
                        // Find next available module
                        if let Some(next_module) = path.modules.iter().find(|m| m.status == ModuleStatus::Available) {
                            recommendations.push(AdaptiveRecommendation {
                                recommendation_type: RecommendationType::Advance,
                                module_id: next_module.id.clone(),
                                reason: "Current module completed - ready for next".to_string(),
                                confidence: 0.95,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        
        Ok(recommendations)
    }
    
    /// Generar resumen de progreso
    fn generate_progress_summary(&self, path: &LearningPath) -> String {
        let completed_modules = path.modules.iter()
            .filter(|m| m.status == ModuleStatus::Completed || m.status == ModuleStatus::Mastered)
            .count();
        
        format!(
            "{} - Mastery: {:.0}% - Modules: {}/{}",
            path.name,
            path.overall_mastery * 100.0,
            completed_modules,
            path.modules.len()
        )
    }
    
    /// Obtener learning path
    pub fn get_learning_path(&self, path_id: &str) -> Result<LearningPath> {
        let paths = self.learning_paths.read().unwrap();
        paths.get(path_id)
            .cloned()
            .ok_or_else(|| ShuiDaoError::NotFound(format!("Path {} not found", path_id)))
    }
    
    /// Listar todos los paths
    pub fn list_learning_paths(&self) -> Vec<LearningPath> {
        self.learning_paths.read().unwrap()
            .values()
            .cloned()
            .collect()
    }
}

impl Default for LearningEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_learning_engine_creation() {
        let engine = LearningEngine::new();
        assert_eq!(engine.confusion_threshold, 0.3);
        assert_eq!(engine.list_learning_paths().len(), 0);
    }
    
    #[test]
    fn test_create_learning_path() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Fundamentos".to_string(),
                Difficulty::Beginner,
                vec!["Variables".to_string(), "Tipos".to_string()]
            ),
            (
                "Ownership".to_string(),
                Difficulty::Intermediate,
                vec!["Borrow Checker".to_string(), "Lifetimes".to_string()]
            ),
        ];
        
        let response = engine.create_learning_path(
            "Aprende Rust".to_string(),
            "Rust Programming".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        match response.action {
            LearningAction::PathCreated { name, modules } => {
                assert_eq!(name, "Aprende Rust");
                assert_eq!(modules, 2);
            }
            _ => panic!("Expected PathCreated action"),
        }
        
        assert_eq!(engine.list_learning_paths().len(), 1);
        assert!(response.processing_time_ms < 180.0);
    }
    
    #[test]
    fn test_checkpoint_completion() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["Checkpoint 1".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test Path".to_string(),
            "Testing".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let checkpoint_id = path.modules[0].checkpoints[0].id.clone();
        
        let response = engine.complete_checkpoint(
            &create_response.path_id,
            &checkpoint_id,
            0.85,
        ).unwrap();
        
        match response.action {
            LearningAction::CheckpointCompleted { checkpoint: _, mastery } => {
                assert_eq!(mastery, 0.85);
            }
            _ => panic!("Expected CheckpointCompleted action"),
        }
    }
    
    #[test]
    fn test_mastery_calculation() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["CP1".to_string(), "CP2".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let cp1_id = path.modules[0].checkpoints[0].id.clone();
        let cp2_id = path.modules[0].checkpoints[1].id.clone();
        
        engine.complete_checkpoint(&create_response.path_id, &cp1_id, 0.8).unwrap();
        engine.complete_checkpoint(&create_response.path_id, &cp2_id, 0.9).unwrap();
        
        let updated_path = engine.get_learning_path(&create_response.path_id).unwrap();
        assert!((updated_path.modules[0].mastery_level - 0.85).abs() < 0.01); // (0.8 + 0.9) / 2 = 0.85
    }
    
    #[test]
    fn test_confusion_detection() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Hard Module".to_string(),
                Difficulty::Advanced,
                vec!["Difficult Checkpoint".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Advanced,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let checkpoint_id = path.modules[0].checkpoints[0].id.clone();
        
        // Low mastery score triggers confusion detection
        let response = engine.complete_checkpoint(
            &create_response.path_id,
            &checkpoint_id,
            0.2, // Below threshold (0.3)
        ).unwrap();
        
        // Should recommend review
        assert!(response.recommendations.iter().any(|r| r.recommendation_type == RecommendationType::Review));
        
        let updated_path = engine.get_learning_path(&create_response.path_id).unwrap();
        assert_eq!(updated_path.confusion_points.len(), 1);
    }
    
    #[test]
    fn test_module_completion() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["CP1".to_string(), "CP2".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let cp1_id = path.modules[0].checkpoints[0].id.clone();
        let cp2_id = path.modules[0].checkpoints[1].id.clone();
        
        engine.complete_checkpoint(&create_response.path_id, &cp1_id, 0.8).unwrap();
        engine.complete_checkpoint(&create_response.path_id, &cp2_id, 0.9).unwrap();
        
        let updated_path = engine.get_learning_path(&create_response.path_id).unwrap();
        // Average mastery: (0.8 + 0.9) / 2 = 0.85, which is >= 0.7 but < 0.9
        assert_eq!(updated_path.modules[0].status, ModuleStatus::Completed);
    }
    
    #[test]
    fn test_module_unlock() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["CP1".to_string()]
            ),
            (
                "Module 2".to_string(),
                Difficulty::Intermediate,
                vec!["CP2".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        
        // Module 2 should be locked initially
        assert_eq!(path.modules[1].status, ModuleStatus::Locked);
        
        // Complete Module 1
        let cp1_id = path.modules[0].checkpoints[0].id.clone();
        engine.complete_checkpoint(&create_response.path_id, &cp1_id, 0.8).unwrap();
        
        let updated_path = engine.get_learning_path(&create_response.path_id).unwrap();
        
        // Module 2 should now be available
        assert_eq!(updated_path.modules[1].status, ModuleStatus::Available);
    }
    
    #[test]
    fn test_adaptive_recommendations() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["CP1".to_string()]
            ),
            (
                "Module 2".to_string(),
                Difficulty::Intermediate,
                vec!["CP2".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let cp1_id = path.modules[0].checkpoints[0].id.clone();
        
        // Complete first checkpoint with high mastery
        let response = engine.complete_checkpoint(&create_response.path_id, &cp1_id, 0.95).unwrap();
        
        // Should recommend advancing
        assert!(response.recommendations.iter().any(|r| {
            r.recommendation_type == RecommendationType::Advance ||
            r.recommendation_type == RecommendationType::Continue
        }));
    }
    
    #[test]
    fn test_overall_path_mastery() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module 1".to_string(),
                Difficulty::Beginner,
                vec!["CP1".to_string()]
            ),
            (
                "Module 2".to_string(),
                Difficulty::Beginner,
                vec!["CP2".to_string()]
            ),
        ];
        
        let create_response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        let path = engine.get_learning_path(&create_response.path_id).unwrap();
        let cp1_id = path.modules[0].checkpoints[0].id.clone();
        let cp2_id = path.modules[1].checkpoints[0].id.clone();
        
        engine.complete_checkpoint(&create_response.path_id, &cp1_id, 0.8).unwrap();
        engine.complete_checkpoint(&create_response.path_id, &cp2_id, 0.9).unwrap();
        
        let updated_path = engine.get_learning_path(&create_response.path_id).unwrap();
        
        // Overall mastery should be average of module masteries: (0.8 + 0.9) / 2 = 0.85
        assert!((updated_path.overall_mastery - 0.85).abs() < 0.01);
        assert_eq!(updated_path.status, PathStatus::Completed);
    }
    
    #[test]
    fn test_performance_target() {
        let mut engine = LearningEngine::new();
        
        let modules = vec![
            (
                "Module".to_string(),
                Difficulty::Beginner,
                vec!["CP".to_string()]
            ),
        ];
        
        let response = engine.create_learning_path(
            "Test".to_string(),
            "Test".to_string(),
            Difficulty::Beginner,
            modules,
        ).unwrap();
        
        // Target: <180ms
        assert!(
            response.processing_time_ms < 180.0,
            "LearningEngine took {:.2}ms (target <180ms)",
            response.processing_time_ms
        );
    }
}
