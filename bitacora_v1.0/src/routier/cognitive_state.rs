//! Cognitive State Tracking - User learning state analysis

use super::error::{Result, RoutierError};
use super::graph::{LearningGraph, StepID};
use super::RoutierConfig;
use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

/// Cognitive state of user during learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    /// Learning velocity (steps per hour)
    pub velocity: f64,
    
    /// Success rate (% completed on first attempt)
    pub success_rate: f64,
    
    /// Frustration level (0.0 = calm, 1.0 = very frustrated)
    pub frustration_level: f64,
    
    /// Engagement level (0.0 = low, 1.0 = high)
    pub engagement_level: f64,
    
    /// Topics where user shows emerging interest
    pub emerging_interests: Vec<EmergingInterest>,
    
    /// Patterns of confusion detected
    pub confusion_patterns: Vec<ConfusionPattern>,
    
    /// Recent step completion times (for velocity calculation)
    recent_completions: VecDeque<StepCompletion>,
}

/// Step completion record
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StepCompletion {
    step_id: StepID,
    time_spent_minutes: u64,
    attempts: u32,
    timestamp: chrono::DateTime<chrono::Utc>,
}

/// Emerging interest in topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergingInterest {
    pub topic: String,
    pub strength: f64, // 0.0-1.0
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

/// Confusion pattern detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionPattern {
    pub step_id: StepID,
    pub confusion_type: ConfusionType,
    pub severity: f64, // 0.0-1.0
    pub detected_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfusionType {
    /// Multiple attempts needed
    RepeatedFailure,
    
    /// Many questions asked
    ExcessiveQueries,
    
    /// Spent much longer than estimated
    TimeOverrun,
    
    /// Skipped/abandoned
    Abandoned,
}

impl CognitiveState {
    /// Create new cognitive state
    pub fn new() -> Self {
        Self {
            velocity: 0.0,
            success_rate: 1.0,
            frustration_level: 0.0,
            engagement_level: 0.5,
            emerging_interests: Vec::new(),
            confusion_patterns: Vec::new(),
            recent_completions: VecDeque::new(),
        }
    }
    
    /// Update state after step completion
    ///
    /// # Performance Target
    /// <20ms
    pub fn update_from_step_completion(
        &mut self,
        step_id: &StepID,
        graph: &LearningGraph,
        time_spent_minutes: u64,
        attempts_needed: u32,
        queries: &[String],
        config: &RoutierConfig,
    ) -> Result<()> {
        let now = chrono::Utc::now();
        
        // Record completion
        let completion = StepCompletion {
            step_id: step_id.clone(),
            time_spent_minutes,
            attempts: attempts_needed,
            timestamp: now,
        };
        
        self.recent_completions.push_back(completion);
        
        // Maintain window size
        while self.recent_completions.len() > config.velocity_window {
            self.recent_completions.pop_front();
        }
        
        // Recalculate metrics
        self.calculate_velocity();
        self.calculate_success_rate();
        self.calculate_frustration(graph, step_id, time_spent_minutes, attempts_needed)?;
        self.calculate_engagement(queries.len());
        
        // Detect patterns
        self.detect_confusion(step_id, graph, time_spent_minutes, attempts_needed, queries)?;
        self.detect_emerging_interests(step_id, graph, time_spent_minutes)?;
        
        Ok(())
    }
    
    /// Calculate learning velocity
    fn calculate_velocity(&mut self) {
        if self.recent_completions.is_empty() {
            self.velocity = 0.0;
            return;
        }
        
        if self.recent_completions.len() == 1 {
            // Use single completion time
            let time_hours = self.recent_completions[0].time_spent_minutes as f64 / 60.0;
            self.velocity = if time_hours > 0.0 { 1.0 / time_hours } else { 0.0 };
            return;
        }
        
        // Calculate from recent window
        let first = self.recent_completions.front().unwrap();
        let last = self.recent_completions.back().unwrap();
        
        let duration = last.timestamp.signed_duration_since(first.timestamp);
        let hours = duration.num_minutes() as f64 / 60.0;
        
        if hours > 0.0 {
            self.velocity = self.recent_completions.len() as f64 / hours;
        }
    }
    
    /// Calculate success rate
    fn calculate_success_rate(&mut self) {
        if self.recent_completions.is_empty() {
            self.success_rate = 1.0;
            return;
        }
        
        let first_attempt_successes = self.recent_completions.iter()
            .filter(|c| c.attempts == 1)
            .count();
        
        self.success_rate = first_attempt_successes as f64 / self.recent_completions.len() as f64;
    }
    
    /// Calculate frustration level
    fn calculate_frustration(
        &mut self,
        graph: &LearningGraph,
        step_id: &StepID,
        time_spent: u64,
        attempts: u32,
    ) -> Result<()> {
        let step = graph.get_step(step_id)
            .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
        
        // Frustration factors
        let time_ratio = time_spent as f64 / (step.estimated_hours * 60) as f64;
        let time_frustration = if time_ratio > 2.0 {
            (time_ratio - 2.0).min(1.0)
        } else {
            0.0
        };
        
        let attempt_frustration = if attempts > 3 {
            ((attempts - 3) as f64 / 5.0).min(1.0)
        } else {
            0.0
        };
        
        // Weighted average
        self.frustration_level = (time_frustration * 0.6 + attempt_frustration * 0.4)
            .min(1.0);
        
        Ok(())
    }
    
    /// Calculate engagement level
    fn calculate_engagement(&mut self, queries_count: usize) {
        // Base engagement from velocity
        let velocity_engagement = (self.velocity / 2.0).min(1.0);
        
        // Engagement from queries (shows curiosity)
        let query_engagement = (queries_count as f64 / 5.0).min(1.0);
        
        // Engagement from success
        let success_engagement = self.success_rate;
        
        // Weighted average
        self.engagement_level = (
            velocity_engagement * 0.3 +
            query_engagement * 0.4 +
            success_engagement * 0.3
        ).min(1.0);
    }
    
    /// Detect confusion patterns
    fn detect_confusion(
        &mut self,
        step_id: &StepID,
        graph: &LearningGraph,
        time_spent: u64,
        attempts: u32,
        queries: &[String],
    ) -> Result<()> {
        let step = graph.get_step(step_id)
            .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
        
        let now = chrono::Utc::now();
        
        // Repeated failure
        if attempts > 3 {
            self.confusion_patterns.push(ConfusionPattern {
                step_id: step_id.clone(),
                confusion_type: ConfusionType::RepeatedFailure,
                severity: ((attempts - 3) as f64 / 5.0).min(1.0),
                detected_at: now,
            });
        }
        
        // Excessive queries
        if queries.len() > 5 {
            self.confusion_patterns.push(ConfusionPattern {
                step_id: step_id.clone(),
                confusion_type: ConfusionType::ExcessiveQueries,
                severity: ((queries.len() - 5) as f64 / 10.0).min(1.0),
                detected_at: now,
            });
        }
        
        // Time overrun
        let expected_minutes = step.estimated_hours * 60;
        if time_spent > expected_minutes * 2 {
            self.confusion_patterns.push(ConfusionPattern {
                step_id: step_id.clone(),
                confusion_type: ConfusionType::TimeOverrun,
                severity: ((time_spent as f64 / expected_minutes as f64) - 2.0).min(1.0),
                detected_at: now,
            });
        }
        
        Ok(())
    }
    
    /// Detect emerging interests
    fn detect_emerging_interests(
        &mut self,
        step_id: &StepID,
        graph: &LearningGraph,
        time_spent: u64,
    ) -> Result<()> {
        let step = graph.get_step(step_id)
            .ok_or_else(|| RoutierError::StepNotFound(step_id.clone()))?;
        
        // Interest = completed faster than expected + high difficulty
        let expected_minutes = step.estimated_hours * 60;
        let time_ratio = time_spent as f64 / expected_minutes as f64;
        
        if time_ratio < 0.7 && step.difficulty > 0.6 {
            // Fast completion on difficult topic = strong interest
            let strength = (1.0 - time_ratio) * step.difficulty;
            
            for concept in &step.concepts {
                self.emerging_interests.push(EmergingInterest {
                    topic: concept.clone(),
                    strength,
                    detected_at: chrono::Utc::now(),
                });
            }
        }
        
        Ok(())
    }
    
    /// Get average velocity over recent window
    pub fn get_velocity(&self) -> f64 {
        self.velocity
    }
    
    /// Check if user is frustrated
    pub fn is_frustrated(&self, threshold: f64) -> bool {
        self.frustration_level > threshold
    }
    
    /// Check if user is highly engaged
    pub fn is_highly_engaged(&self, threshold: f64) -> bool {
        self.engagement_level > threshold
    }
    
    /// Check if user is fast (for skip detection)
    pub fn is_fast_learner(&self, threshold: f64) -> bool {
        self.success_rate > threshold && self.velocity > 1.0
    }
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self::new()
    }
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
                name: "Test Curriculum".to_string(),
                phases: vec![
                    CurriculumPhase {
                        phase_number: 1,
                        name: "Basics".to_string(),
                        description: "Test basics".to_string(),
                        difficulty: 0.3,
                        concepts: vec!["Concept 1".to_string()],
                        estimated_hours: 2,
                        prerequisites: vec![],
                    },
                ],
                complexity_score: 0.3,
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
    fn test_cognitive_state_creation() {
        let state = CognitiveState::new();
        assert_eq!(state.velocity, 0.0);
        assert_eq!(state.success_rate, 1.0);
    }
    
    #[test]
    fn test_update_from_completion() {
        let mut state = CognitiveState::new();
        let graph = create_test_graph();
        let config = RoutierConfig::default();
        
        state.update_from_step_completion(
            &"p0_c0".to_string(),
            &graph,
            60, // 1 hour
            1,  // 1 attempt
            &[],
            &config,
        ).unwrap();
        
        assert!(state.velocity > 0.0);
        assert_eq!(state.success_rate, 1.0);
    }
    
    #[test]
    fn test_frustration_detection() {
        let mut state = CognitiveState::new();
        let graph = create_test_graph();
        let config = RoutierConfig::default();
        
        // Spend 5 hours on 2-hour task with 5 attempts
        state.update_from_step_completion(
            &"p0_c0".to_string(),
            &graph,
            300, // 5 hours
            5,   // 5 attempts
            &[],
            &config,
        ).unwrap();
        
        assert!(state.frustration_level > 0.5);
    }
    
    #[test]
    fn test_interest_detection() {
        let mut state = CognitiveState::new();
        let graph = create_test_graph();
        let config = RoutierConfig::default();
        
        // Complete quickly (30 min on 2-hour task)
        state.update_from_step_completion(
            &"p0_c0".to_string(),
            &graph,
            30,
            1,
            &[],
            &config,
        ).unwrap();
        
        assert!(!state.emerging_interests.is_empty());
    }
}
