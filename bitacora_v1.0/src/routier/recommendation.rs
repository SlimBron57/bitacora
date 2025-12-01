//! Next Step Recommendation - Optimal step selection algorithm

use super::error::{Result, RoutierError};
use super::graph::{LearningGraph, LearningStep, StepID};
use super::cognitive_state::CognitiveState;
use super::{LearningPath, RoutierConfig};
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

/// Recommendation for next learning step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStepRecommendation {
    /// Recommended step
    pub step: LearningStep,
    
    /// Score (0.0-1.0)
    pub score: f64,
    
    /// Explanation of why this step
    pub explanation: RecommendationExplanation,
    
    /// Estimated time to complete (minutes)
    pub estimated_time: u64,
    
    /// Estimated difficulty (0.0-1.0)
    pub estimated_difficulty: f64,
}

/// Explanation components for recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationExplanation {
    /// Difficulty match reason
    pub difficulty_reason: String,
    
    /// Interest match reason
    pub interest_reason: String,
    
    /// Momentum reason
    pub momentum_reason: String,
    
    /// Variety reason
    pub variety_reason: String,
    
    /// Overall reasoning
    pub overall: String,
}

/// Select optimal next step
///
/// # Performance Target
/// <50ms
///
/// # Scoring Algorithm
/// ```text
/// score = (0.4 × difficulty_score) +
///         (0.3 × interest_score) +
///         (0.2 × momentum_score) +
///         (0.1 × variety_score)
/// ```
///
/// ## Difficulty Score (40%)
/// - Match user velocity to step difficulty
/// - Fast learners → harder steps
/// - Struggling learners → easier steps
///
/// ## Interest Score (30%)
/// - Match emerging interests to step topics
/// - Higher score for matching topics
///
/// ## Momentum Score (20%)
/// - Prerequisites completed → higher score
/// - Natural progression → higher score
///
/// ## Variety Score (10%)
/// - Different from recent steps → higher score
/// - Prevents monotony
pub fn select_optimal_next_step(
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
    config: &RoutierConfig,
) -> Result<NextStepRecommendation> {
    // Get available steps
    let available = graph.get_available_steps(&path.completed_steps.keys().cloned().collect());
    
    if available.is_empty() {
        return Err(RoutierError::NoAvailableSteps);
    }
    
    // Score each candidate
    let mut candidates: Vec<(StepID, f64)> = available.iter()
        .filter_map(|step_id| {
            let score = calculate_step_score(
                step_id,
                graph,
                path,
                state,
                config,
            ).ok()?;
            
            Some((step_id.clone(), score))
        })
        .collect();
    
    if candidates.is_empty() {
        return Err(RoutierError::NoAvailableSteps);
    }
    
    // Sort by score (descending)
    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    // Select best
    let (best_id, best_score) = candidates.into_iter().next().unwrap();
    
    let step = graph.get_step(&best_id)
        .ok_or_else(|| RoutierError::StepNotFound(best_id.clone()))?
        .clone();
    
    // Generate explanation
    let explanation = generate_explanation(
        &step,
        graph,
        path,
        state,
        config,
    )?;
    
    // Estimate time based on user velocity
    let base_time = step.estimated_hours * 60;
    let adjusted_time = if state.get_velocity() > 0.0 {
        (base_time as f64 / state.get_velocity()) as u64
    } else {
        base_time
    };
    
    Ok(NextStepRecommendation {
        step,
        score: best_score,
        explanation,
        estimated_time: adjusted_time,
        estimated_difficulty: step.difficulty,
    })
}

/// Calculate score for a step
fn calculate_step_score(
    step_id: &StepID,
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
    config: &RoutierConfig,
) -> Result<f64> {
    let step = graph.get_step(step_id)
        .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
    
    // 1. Difficulty score (40%)
    let difficulty_score = calculate_difficulty_score(step, state);
    
    // 2. Interest score (30%)
    let interest_score = calculate_interest_score(step, state);
    
    // 3. Momentum score (20%)
    let momentum_score = calculate_momentum_score(step_id, graph, path)?;
    
    // 4. Variety score (10%)
    let variety_score = calculate_variety_score(step, path);
    
    // Weighted sum
    let total = (difficulty_score * config.difficulty_weight) +
                (interest_score * config.interest_weight) +
                (momentum_score * config.momentum_weight) +
                (variety_score * config.variety_weight);
    
    Ok(total.min(1.0).max(0.0))
}

/// Calculate difficulty match score
///
/// Match step difficulty to user's current capability:
/// - Fast learners (velocity > 1.5) → prefer harder steps
/// - Slow learners (velocity < 0.5) → prefer easier steps
/// - Average learners → prefer medium difficulty
fn calculate_difficulty_score(step: &LearningStep, state: &CognitiveState) -> f64 {
    let velocity = state.get_velocity();
    
    // Determine ideal difficulty based on velocity
    let ideal_difficulty = if velocity > 1.5 {
        0.8 // Hard steps for fast learners
    } else if velocity < 0.5 {
        0.3 // Easy steps for slow learners
    } else {
        0.5 // Medium for average
    };
    
    // Score based on how close to ideal
    let diff = (step.difficulty - ideal_difficulty).abs();
    1.0 - diff
}

/// Calculate interest match score
///
/// Higher score for steps matching emerging interests
fn calculate_interest_score(step: &LearningStep, state: &CognitiveState) -> f64 {
    if state.emerging_interests.is_empty() {
        return 0.5; // Neutral score
    }
    
    // Check if step concepts match any interests
    let mut max_match = 0.0;
    
    for interest in &state.emerging_interests {
        for concept in &step.concepts {
            if concept.to_lowercase().contains(&interest.topic.to_lowercase()) {
                max_match = max_match.max(interest.strength);
            }
        }
    }
    
    max_match
}

/// Calculate momentum score
///
/// Higher score for steps that:
/// - Have all prerequisites completed
/// - Are next in natural sequence
/// - Continue current learning thread
fn calculate_momentum_score(
    step_id: &StepID,
    graph: &LearningGraph,
    path: &LearningPath,
) -> Result<f64> {
    let step = graph.get_step(step_id)
        .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
    
    // Count completed prerequisites
    let total_prereqs = step.prerequisites.len();
    if total_prereqs == 0 {
        return Ok(1.0); // No prerequisites = full momentum
    }
    
    let completed_prereqs = step.prerequisites.iter()
        .filter(|prereq| path.completed_steps.contains_key(*prereq))
        .count();
    
    // Score = % of prerequisites completed
    Ok(completed_prereqs as f64 / total_prereqs as f64)
}

/// Calculate variety score
///
/// Higher score for steps that differ from recent completions
/// to prevent monotony
fn calculate_variety_score(step: &LearningStep, path: &LearningPath) -> f64 {
    // Get last 3 completed steps
    let recent: HashSet<_> = path.completed_steps.keys()
        .rev()
        .take(3)
        .collect();
    
    if recent.is_empty() {
        return 1.0; // First step = maximum variety
    }
    
    // Score = 1.0 if different phase, 0.5 if same phase
    let same_phase = recent.iter()
        .any(|id| id.starts_with(&format!("p{}_", step.phase)));
    
    if same_phase {
        0.5
    } else {
        1.0
    }
}

/// Generate explanation for recommendation
fn generate_explanation(
    step: &LearningStep,
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
    config: &RoutierConfig,
) -> Result<RecommendationExplanation> {
    let difficulty_reason = if state.get_velocity() > 1.5 {
        format!("Your fast pace ({:.1} steps/hour) indicates you're ready for this challenging step", state.get_velocity())
    } else if state.get_velocity() < 0.5 {
        "This step's moderate difficulty matches your current learning pace".to_string()
    } else {
        "Difficulty level aligns well with your progress".to_string()
    };
    
    let interest_reason = if let Some(interest) = state.emerging_interests.first() {
        if step.concepts.iter().any(|c| c.contains(&interest.topic)) {
            format!("Matches your interest in '{}'", interest.topic)
        } else {
            "Complements your current learning interests".to_string()
        }
    } else {
        "Core topic in your learning path".to_string()
    };
    
    let momentum_reason = {
        let completed_prereqs = step.prerequisites.iter()
            .filter(|p| path.completed_steps.contains_key(*p))
            .count();
        
        if step.prerequisites.is_empty() {
            "No prerequisites required - you can start immediately".to_string()
        } else if completed_prereqs == step.prerequisites.len() {
            "All prerequisites completed - natural next step".to_string()
        } else {
            format!("{}/{} prerequisites completed", completed_prereqs, step.prerequisites.len())
        }
    };
    
    let variety_reason = if path.completed_steps.is_empty() {
        "Starting point of your learning journey".to_string()
    } else {
        "Provides good variety from recent steps".to_string()
    };
    
    let overall = format!(
        "Recommended: '{}' - {} This step balances challenge, interest, and progression.",
        step.name,
        if step.difficulty > 0.7 { "Advanced topic." } else { "Core concept." }
    );
    
    Ok(RecommendationExplanation {
        difficulty_reason,
        interest_reason,
        momentum_reason,
        variety_reason,
        overall,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expertise_generation::{ExpertisePackage, Curriculum, CurriculumPhase, ExpertiseLevel, ExpertiseMetadata};
    
    fn create_test_graph() -> LearningGraph {
        let package = ExpertisePackage {
            id: "test".to_string(),
            domain: "Test".to_string(),
            current_level: ExpertiseLevel::Beginner,
            target_level: ExpertiseLevel::Expert,
            curriculum: Curriculum {
                name: "Test".to_string(),
                phases: vec![
                    CurriculumPhase {
                        phase_number: 1,
                        name: "Basics".to_string(),
                        description: "Basic concepts".to_string(),
                        difficulty: 0.3,
                        concepts: vec!["Concept A".to_string()],
                        estimated_hours: 2,
                        prerequisites: vec![],
                    },
                    CurriculumPhase {
                        phase_number: 2,
                        name: "Advanced".to_string(),
                        description: "Advanced topics".to_string(),
                        difficulty: 0.8,
                        concepts: vec!["Concept B".to_string()],
                        estimated_hours: 5,
                        prerequisites: vec!["p0_c0".to_string()],
                    },
                ],
                complexity_score: 0.5,
                prerequisites: vec![],
            },
            templates: vec![],
            knowledge_base: crate::expertise_generation::KnowledgeBase {
                facts: vec![],
                relationships: vec![],
                sources: vec![],
            },
            resources: vec![],
            projects: vec![],
            estimated_mastery_hours: 7,
            metadata: ExpertiseMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                version: "1.0".to_string(),
                tags: vec![],
            },
        };
        
        LearningGraph::from_expertise_package(&package).unwrap()
    }
    
    #[test]
    fn test_difficulty_score() {
        let step = LearningStep {
            id: "test".to_string(),
            name: "Test".to_string(),
            description: "Test".to_string(),
            phase: 1,
            difficulty: 0.5,
            estimated_hours: 2,
            concepts: vec![],
            prerequisites: vec![],
        };
        
        let mut state = CognitiveState::new();
        state.velocity = 1.0;
        
        let score = calculate_difficulty_score(&step, &state);
        assert!(score > 0.0);
    }
    
    #[test]
    fn test_select_optimal_step() {
        let graph = create_test_graph();
        let path = LearningPath::new(vec!["p0_c0".to_string()]);
        let state = CognitiveState::new();
        let config = RoutierConfig::default();
        
        let rec = select_optimal_next_step(&graph, &path, &state, &config).unwrap();
        
        assert_eq!(rec.step.id, "p0_c0"); // Should recommend first step
        assert!(rec.score > 0.0);
    }
}
