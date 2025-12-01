//! Route Adaptation - Dynamic path modification based on cognitive state

use super::error::{Result, RoutierError};
use super::graph::{LearningGraph, LearningStep, StepID};
use super::cognitive_state::CognitiveState;
use super::{LearningPath, RoutierConfig};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Route adjustment applied to learning path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAdjustment {
    /// Type of adjustment
    pub adjustment_type: AdjustmentType,
    
    /// Why this adjustment was made
    pub reason: AdjustmentReason,
    
    /// When adjustment was applied
    pub timestamp: DateTime<Utc>,
    
    /// Steps affected by adjustment
    pub affected_steps: Vec<StepID>,
}

/// Type of route adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    /// Skip steps (user already proficient)
    Skip {
        skipped_steps: Vec<StepID>,
    },
    
    /// Insert prerequisite steps (user confused)
    Insert {
        new_steps: Vec<LearningStep>,
        insert_before: StepID,
    },
    
    /// Unlock advanced step early (user interested)
    Unlock {
        unlocked_step: StepID,
    },
    
    /// Change focus area (user frustrated)
    Pivot {
        from_topic: String,
        to_topic: String,
        new_steps: Vec<StepID>,
    },
    
    /// Add enrichment projects (user engaged)
    Extend {
        extra_projects: Vec<String>,
    },
}

/// Reason for adjustment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentReason {
    /// High success rate, fast completion
    UserTooFast,
    
    /// Confusion patterns detected
    UserConfused,
    
    /// Strong emerging interest
    UserInterested,
    
    /// High frustration level
    UserFrustrated,
    
    /// High engagement, seeking challenge
    UserEngaged,
}

/// Attempt route adjustment based on cognitive state
///
/// # Performance Target
/// <100ms
///
/// # Strategy Selection
/// 1. Check for skip opportunity (fast learner)
/// 2. Check for insert need (confusion)
/// 3. Check for unlock opportunity (interest)
/// 4. Check for pivot need (frustration)
/// 5. Check for extend opportunity (high engagement)
pub fn attempt_route_adjustment(
    graph: &LearningGraph,
    path: &mut LearningPath,
    state: &CognitiveState,
    config: &RoutierConfig,
) -> Result<Option<RouteAdjustment>> {
    // Priority order: confusion > frustration > skip > interest > extend
    
    // 1. Insert prerequisites if confused
    if let Some(adj) = attempt_insert_prerequisite(graph, path, state)? {
        return Ok(Some(adj));
    }
    
    // 2. Pivot if frustrated
    if state.is_frustrated(config.frustration_threshold) {
        if let Some(adj) = attempt_pivot(graph, path, state)? {
            return Ok(Some(adj));
        }
    }
    
    // 3. Skip if too fast
    if state.is_fast_learner(config.skip_threshold) {
        if let Some(adj) = attempt_skip(graph, path, state)? {
            return Ok(Some(adj));
        }
    }
    
    // 4. Unlock advanced if interested
    if let Some(adj) = attempt_unlock(graph, path, state)? {
        return Ok(Some(adj));
    }
    
    // 5. Extend if highly engaged
    if state.is_highly_engaged(0.8) {
        if let Some(adj) = attempt_extend(graph, state)? {
            return Ok(Some(adj));
        }
    }
    
    Ok(None)
}

/// Attempt to skip steps (fast learner)
fn attempt_skip(
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
) -> Result<Option<RouteAdjustment>> {
    if state.success_rate < 0.85 {
        return Ok(None); // Not consistent enough
    }
    
    // Find upcoming steps with similar difficulty to recent wins
    let current_step = path.current_step()
        .ok_or(RoutierError::NoAvailableSteps)?;
    
    let current = graph.get_step(current_step)
        .ok_or_else(|| RoutierError::StepNotFound(current_step.clone()))?;
    
    let mut skippable = Vec::new();
    
    for step_id in &path.steps[path.current_position..] {
        let step = graph.get_step(step_id)
            .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
        
        // Skip if same difficulty and user demonstrated mastery
        if (step.difficulty - current.difficulty).abs() < 0.1 
            && state.velocity > 1.5 
        {
            skippable.push(step_id.clone());
        }
        
        if skippable.len() >= 3 {
            break; // Don't skip too many at once
        }
    }
    
    if skippable.is_empty() {
        return Ok(None);
    }
    
    Ok(Some(RouteAdjustment {
        adjustment_type: AdjustmentType::Skip {
            skipped_steps: skippable.clone(),
        },
        reason: AdjustmentReason::UserTooFast,
        timestamp: Utc::now(),
        affected_steps: skippable,
    }))
}

/// Attempt to insert prerequisite steps (confusion detected)
fn attempt_insert_prerequisite(
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
) -> Result<Option<RouteAdjustment>> {
    // Check recent confusion patterns
    if state.confusion_patterns.is_empty() {
        return Ok(None);
    }
    
    let current_step = path.current_step()
        .ok_or(RoutierError::NoAvailableSteps)?;
    
    let current = graph.get_step(current_step)
        .ok_or_else(|| RoutierError::StepNotFound(current_step.clone()))?;
    
    // Generate simpler prerequisite steps
    let new_steps = vec![
        LearningStep {
            id: format!("{}_prereq_review", current_step),
            name: format!("Review: {}", current.name),
            description: "Reinforcement review before proceeding".to_string(),
            phase: current.phase,
            difficulty: current.difficulty * 0.7, // Easier
            estimated_hours: current.estimated_hours / 2,
            concepts: current.concepts.clone(),
            prerequisites: vec![],
        },
    ];
    
    Ok(Some(RouteAdjustment {
        adjustment_type: AdjustmentType::Insert {
            new_steps: new_steps.clone(),
            insert_before: current_step.clone(),
        },
        reason: AdjustmentReason::UserConfused,
        timestamp: Utc::now(),
        affected_steps: new_steps.iter().map(|s| s.id.clone()).collect(),
    }))
}

/// Attempt to unlock advanced step (interest detected)
fn attempt_unlock(
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
) -> Result<Option<RouteAdjustment>> {
    if state.emerging_interests.is_empty() {
        return Ok(None);
    }
    
    // Find most recent strong interest
    let strongest = state.emerging_interests.iter()
        .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
        .unwrap();
    
    // Find advanced step matching this interest
    for (step_id, step) in &graph.nodes {
        if path.completed_steps.contains_key(step_id) {
            continue; // Already done
        }
        
        if step.difficulty > 0.7 
            && step.concepts.iter().any(|c| c.contains(&strongest.topic)) 
        {
            return Ok(Some(RouteAdjustment {
                adjustment_type: AdjustmentType::Unlock {
                    unlocked_step: step_id.clone(),
                },
                reason: AdjustmentReason::UserInterested,
                timestamp: Utc::now(),
                affected_steps: vec![step_id.clone()],
            }));
        }
    }
    
    Ok(None)
}

/// Attempt to pivot to different topic (frustration)
fn attempt_pivot(
    graph: &LearningGraph,
    path: &LearningPath,
    state: &CognitiveState,
) -> Result<Option<RouteAdjustment>> {
    let current_step = path.current_step()
        .ok_or(RoutierError::NoAvailableSteps)?;
    
    let current = graph.get_step(current_step)
        .ok_or_else(|| RoutierError::StepNotFound(current_step.clone()))?;
    
    // Find steps in different topic area with lower difficulty
    let mut alternative_steps = Vec::new();
    
    for (step_id, step) in &graph.nodes {
        if path.completed_steps.contains_key(step_id) {
            continue;
        }
        
        // Different topic, easier
        if step.phase != current.phase 
            && step.difficulty < current.difficulty 
        {
            alternative_steps.push(step_id.clone());
        }
        
        if alternative_steps.len() >= 3 {
            break;
        }
    }
    
    if alternative_steps.is_empty() {
        return Ok(None);
    }
    
    Ok(Some(RouteAdjustment {
        adjustment_type: AdjustmentType::Pivot {
            from_topic: current.concepts.first()
                .unwrap_or(&"current topic".to_string())
                .clone(),
            to_topic: "alternative approach".to_string(),
            new_steps: alternative_steps.clone(),
        },
        reason: AdjustmentReason::UserFrustrated,
        timestamp: Utc::now(),
        affected_steps: alternative_steps,
    }))
}

/// Attempt to extend with projects (high engagement)
fn attempt_extend(
    _graph: &LearningGraph,
    state: &CognitiveState,
) -> Result<Option<RouteAdjustment>> {
    if state.engagement_level < 0.8 {
        return Ok(None);
    }
    
    // Suggest enrichment projects based on interests
    let projects: Vec<String> = state.emerging_interests.iter()
        .take(3)
        .map(|interest| format!("Advanced Project: {}", interest.topic))
        .collect();
    
    if projects.is_empty() {
        return Ok(None);
    }
    
    Ok(Some(RouteAdjustment {
        adjustment_type: AdjustmentType::Extend {
            extra_projects: projects.clone(),
        },
        reason: AdjustmentReason::UserEngaged,
        timestamp: Utc::now(),
        affected_steps: vec![],
    }))
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
                        name: "Phase 1".to_string(),
                        description: "Test phase".to_string(),
                        difficulty: 0.5,
                        concepts: vec!["Concept A".to_string()],
                        estimated_hours: 2,
                        prerequisites: vec![],
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
            estimated_mastery_hours: 2,
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
    fn test_skip_detection() {
        let graph = create_test_graph();
        let path = LearningPath::new(vec!["p0_c0".to_string()]);
        
        let mut state = CognitiveState::new();
        state.success_rate = 0.9;
        state.velocity = 2.0;
        
        let result = attempt_skip(&graph, &path, &state).unwrap();
        assert!(result.is_none()); // No skippable steps in simple graph
    }
    
    #[test]
    fn test_insert_prerequisite() {
        let graph = create_test_graph();
        let path = LearningPath::new(vec!["p0_c0".to_string()]);
        
        let mut state = CognitiveState::new();
        state.confusion_patterns.push(super::super::cognitive_state::ConfusionPattern {
            step_id: "p0_c0".to_string(),
            confusion_type: super::super::cognitive_state::ConfusionType::RepeatedFailure,
            severity: 0.8,
            detected_at: Utc::now(),
        });
        
        let result = attempt_insert_prerequisite(&graph, &path, &state).unwrap();
        assert!(result.is_some());
    }
}
