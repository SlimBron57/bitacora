//! Learning Graph - DAG structure for curriculum navigation
//!
//! Converts ExpertisePackage curriculum into directed acyclic graph (DAG)
//! for adaptive path planning.

use super::error::{Result, RoutierError};
use crate::expertise_generation::{ExpertisePackage, CurriculumPhase};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Step identifier (unique within graph)
pub type StepID = String;

/// Learning Graph - DAG representing curriculum structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningGraph {
    /// All learning steps indexed by ID
    pub nodes: HashMap<StepID, LearningStep>,
    
    /// Adjacency list (step → dependencies)
    pub edges: HashMap<StepID, Vec<StepID>>,
    
    /// Entry point (first step)
    pub start_node: StepID,
    
    /// Terminal nodes (completion points)
    pub end_nodes: HashSet<StepID>,
}

/// Learning Step - Single unit in learning path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStep {
    pub id: StepID,
    pub name: String,
    pub description: String,
    pub phase: usize,
    pub difficulty: f64,
    pub estimated_hours: u64,
    pub concepts: Vec<String>,
    pub prerequisites: Vec<StepID>,
}

impl LearningGraph {
    /// Construct learning graph from ExpertisePackage
    ///
    /// # Algorithm
    /// 1. Convert each phase.concepts into LearningStep nodes
    /// 2. Build edges from phase.prerequisites
    /// 3. Detect cycles (must be DAG)
    /// 4. Identify terminal nodes
    ///
    /// # Performance Target
    /// <200ms (CRITICAL - blocks navigator creation)
    pub fn from_expertise_package(package: &ExpertisePackage) -> Result<Self> {
        let mut nodes = HashMap::new();
        let mut edges: HashMap<StepID, Vec<StepID>> = HashMap::new();
        let mut all_step_ids = Vec::new();
        
        // Convert phases to steps
        for (phase_idx, phase) in package.curriculum.phases.iter().enumerate() {
            let step_ids = Self::phase_to_steps(
                phase, 
                phase_idx, 
                &mut nodes,
            )?;
            
            all_step_ids.extend(step_ids.clone());
            
            // Build edges from prerequisites
            Self::build_edges_for_phase(
                &step_ids,
                phase,
                &mut edges,
            );
        }
        
        // Validate no cycles
        if Self::has_cycle(&edges) {
            return Err(RoutierError::GraphConstruction(
                "Cycle detected in curriculum prerequisites".to_string()
            ));
        }
        
        // Identify start node (no prerequisites)
        let start_node = Self::find_start_node(&nodes, &edges)?;
        
        // Identify end nodes (no outgoing edges)
        let end_nodes = Self::find_end_nodes(&edges, &all_step_ids);
        
        Ok(Self {
            nodes,
            edges,
            start_node,
            end_nodes,
        })
    }
    
    /// Convert CurriculumPhase to LearningSteps
    fn phase_to_steps(
        phase: &CurriculumPhase,
        phase_idx: usize,
        nodes: &mut HashMap<StepID, LearningStep>,
    ) -> Result<Vec<StepID>> {
        let mut step_ids = Vec::new();
        
        for (concept_idx, concept) in phase.concepts.iter().enumerate() {
            let step_id = format!("p{}_c{}", phase_idx, concept_idx);
            
            let step = LearningStep {
                id: step_id.clone(),
                name: concept.clone(),
                description: phase.description.clone(),
                phase: phase.phase_number,
                difficulty: phase.difficulty,
                estimated_hours: phase.estimated_hours / phase.concepts.len() as u64,
                concepts: vec![concept.clone()],
                prerequisites: phase.prerequisites.clone(),
            };
            
            nodes.insert(step_id.clone(), step);
            step_ids.push(step_id);
        }
        
        Ok(step_ids)
    }
    
    /// Build edges from phase prerequisites
    fn build_edges_for_phase(
        step_ids: &[StepID],
        phase: &CurriculumPhase,
        edges: &mut HashMap<StepID, Vec<StepID>>,
    ) {
        // Each step in phase depends on prerequisites
        for step_id in step_ids {
            edges.insert(
                step_id.clone(),
                phase.prerequisites.clone(),
            );
        }
    }
    
    /// Detect cycles using DFS
    fn has_cycle(edges: &HashMap<StepID, Vec<StepID>>) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node in edges.keys() {
            if Self::dfs_cycle_check(
                node,
                edges,
                &mut visited,
                &mut rec_stack,
            ) {
                return true;
            }
        }
        
        false
    }
    
    /// DFS helper for cycle detection
    fn dfs_cycle_check(
        node: &StepID,
        edges: &HashMap<StepID, Vec<StepID>>,
        visited: &mut HashSet<StepID>,
        rec_stack: &mut HashSet<StepID>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true; // Cycle found
        }
        
        if visited.contains(node) {
            return false;
        }
        
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        
        if let Some(deps) = edges.get(node) {
            for dep in deps {
                if Self::dfs_cycle_check(dep, edges, visited, rec_stack) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node);
        false
    }
    
    /// Find start node (no prerequisites)
    fn find_start_node(
        nodes: &HashMap<StepID, LearningStep>,
        edges: &HashMap<StepID, Vec<StepID>>,
    ) -> Result<StepID> {
        for (step_id, deps) in edges {
            if deps.is_empty() {
                return Ok(step_id.clone());
            }
        }
        
        // If all have deps, take first node
        nodes.keys()
            .next()
            .cloned()
            .ok_or_else(|| RoutierError::GraphConstruction(
                "Empty curriculum (no steps)".to_string()
            ))
    }
    
    /// Find terminal nodes (no outgoing edges)
    fn find_end_nodes(
        edges: &HashMap<StepID, Vec<StepID>>,
        all_step_ids: &[StepID],
    ) -> HashSet<StepID> {
        let mut end_nodes = HashSet::new();
        let dependencies: HashSet<_> = edges.values()
            .flat_map(|deps| deps.iter())
            .collect();
        
        for step_id in all_step_ids {
            if !dependencies.contains(&step_id) {
                end_nodes.insert(step_id.clone());
            }
        }
        
        end_nodes
    }
    
    /// Get optimal initial path using topological sort
    pub fn get_optimal_path(&self) -> Result<Vec<StepID>> {
        self.topological_sort()
    }
    
    /// Topological sort (Kahn's algorithm)
    fn topological_sort(&self) -> Result<Vec<StepID>> {
        let mut in_degree: HashMap<StepID, usize> = HashMap::new();
        
        // Calculate in-degrees
        for step_id in self.nodes.keys() {
            in_degree.insert(step_id.clone(), 0);
        }
        
        for deps in self.edges.values() {
            for dep in deps {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
        
        // Queue nodes with in-degree 0
        let mut queue: VecDeque<StepID> = in_degree.iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(id, _)| id.clone())
            .collect();
        
        let mut sorted = Vec::new();
        
        while let Some(step_id) = queue.pop_front() {
            sorted.push(step_id.clone());
            
            // Reduce in-degree of dependents
            if let Some(deps) = self.edges.get(&step_id) {
                for dep in deps {
                    if let Some(deg) = in_degree.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }
        
        if sorted.len() != self.nodes.len() {
            return Err(RoutierError::GraphConstruction(
                "Topological sort failed (cycle or missing nodes)".to_string()
            ));
        }
        
        Ok(sorted)
    }
    
    /// Get available next steps after completing given steps
    pub fn get_available_steps(
        &self,
        completed: &HashSet<StepID>,
    ) -> Vec<StepID> {
        let mut available = Vec::new();
        
        for (step_id, step) in &self.nodes {
            if completed.contains(step_id) {
                continue; // Already done
            }
            
            // Check if all prerequisites met
            let prereqs_met = step.prerequisites.iter()
                .all(|prereq| completed.contains(prereq));
            
            if prereqs_met {
                available.push(step_id.clone());
            }
        }
        
        available
    }
    
    /// Get step by ID
    pub fn get_step(&self, step_id: &StepID) -> Option<&LearningStep> {
        self.nodes.get(step_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expertise_generation::{Curriculum, ExpertiseLevel, ExpertiseMetadata};
    
    fn create_test_package() -> ExpertisePackage {
        ExpertisePackage {
            id: "test_pkg".to_string(),
            domain: "Rust Programming".to_string(),
            current_level: ExpertiseLevel::Beginner,
            target_level: ExpertiseLevel::Expert,
            curriculum: Curriculum {
                name: "Rust Mastery".to_string(),
                phases: vec![
                    CurriculumPhase {
                        phase_number: 1,
                        name: "Basics".to_string(),
                        description: "Fundamentals".to_string(),
                        difficulty: 0.3,
                        concepts: vec![
                            "Variables".to_string(),
                            "Functions".to_string(),
                        ],
                        estimated_hours: 10,
                        prerequisites: vec![],
                    },
                    CurriculumPhase {
                        phase_number: 2,
                        name: "Ownership".to_string(),
                        description: "Memory management".to_string(),
                        difficulty: 0.7,
                        concepts: vec![
                            "Borrowing".to_string(),
                            "Lifetimes".to_string(),
                        ],
                        estimated_hours: 20,
                        prerequisites: vec!["p0_c0".to_string(), "p0_c1".to_string()],
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
            estimated_mastery_hours: 30,
            metadata: ExpertiseMetadata {
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                version: "1.0".to_string(),
                tags: vec![],
            },
        }
    }
    
    #[test]
    fn test_graph_construction() {
        let package = create_test_package();
        let graph = LearningGraph::from_expertise_package(&package).unwrap();
        
        assert_eq!(graph.nodes.len(), 4); // 2 phases × 2 concepts
        assert!(graph.start_node.starts_with("p0_"));
    }
    
    #[test]
    fn test_topological_sort() {
        let package = create_test_package();
        let graph = LearningGraph::from_expertise_package(&package).unwrap();
        
        let path = graph.get_optimal_path().unwrap();
        assert_eq!(path.len(), 4);
        
        // Basics should come before Ownership
        let basics_idx = path.iter().position(|id| id == "p0_c0").unwrap();
        let ownership_idx = path.iter().position(|id| id == "p1_c0").unwrap();
        assert!(basics_idx < ownership_idx);
    }
    
    #[test]
    fn test_available_steps() {
        let package = create_test_package();
        let graph = LearningGraph::from_expertise_package(&package).unwrap();
        
        let mut completed = HashSet::new();
        completed.insert("p0_c0".to_string());
        completed.insert("p0_c1".to_string());
        
        let available = graph.get_available_steps(&completed);
        
        // Ownership steps should be available now
        assert!(available.contains(&"p1_c0".to_string()));
        assert!(available.contains(&"p1_c1".to_string()));
    }
}
