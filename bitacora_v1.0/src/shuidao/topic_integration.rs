// ================================================================
// TopicIntegration - Persistencia VoxelDB + MTT-DSL Templates
// ================================================================
//
// Purpose: Save/load TopicGraph desde VoxelDB con MTT-DSL templates
// v1.0: Stub implementation (JSON file persistence)
// v1.1: Real VoxelDB integration con CubicCoords mapping
//
// DA-033: Dynamic Topic & Tone System
// Spec: ROADMAP_V2/02_COMPONENTES/CRITICOS/14_shuidao-topic-graph.md v1.1.0
//
// ================================================================

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Deserialize, Serialize};

use super::topic_graph::TopicGraph;
use super::error::{Result, ShuiDaoError};

// ================================================================
// VOXELDB STUB (v1.0)
// ================================================================

/// Storage para TopicGraphs (v1.0: archivo JSON, v1.1: real VoxelDB)
pub struct TopicStorage {
    /// Directorio de almacenamiento
    storage_dir: PathBuf,
}

impl TopicStorage {
    /// Crea storage con directorio por defecto
    pub fn new() -> Self {
        Self {
            storage_dir: PathBuf::from("./data/topic_graphs"),
        }
    }
    
    /// Crea storage con directorio personalizado
    pub fn with_dir(dir: PathBuf) -> Self {
        Self {
            storage_dir: dir,
        }
    }
    
    /// Guarda TopicGraph a archivo JSON (v1.0 stub)
    ///
    /// # Performance
    /// Target: <50ms (I/O bound)
    pub fn save(&self, graph: &TopicGraph) -> Result<()> {
        // Crear directorio si no existe
        fs::create_dir_all(&self.storage_dir)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to create storage dir: {}", e),
            })?;
        
        // Ruta del archivo
        let file_path = self.storage_dir.join(format!("{}.json", graph.user_id));
        
        // Serializar a JSON
        let json = serde_json::to_string_pretty(graph)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to serialize graph: {}", e),
            })?;
        
        // Escribir a archivo
        fs::write(&file_path, json)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to write file: {}", e),
            })?;
        
        Ok(())
    }
    
    /// Carga TopicGraph desde archivo JSON (v1.0 stub)
    ///
    /// # Performance
    /// Target: <50ms (I/O bound)
    pub fn load(&self, user_id: &str) -> Result<TopicGraph> {
        let file_path = self.storage_dir.join(format!("{}.json", user_id));
        
        // Verificar que archivo existe
        if !file_path.exists() {
            return Err(ShuiDaoError::NotFound(format!(
                "TopicGraph for user '{}' not found",
                user_id
            )));
        }
        
        // Leer archivo
        let json = fs::read_to_string(&file_path)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to read file: {}", e),
            })?;
        
        // Deserializar
        let graph = serde_json::from_str(&json)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to parse JSON: {}", e),
            })?;
        
        Ok(graph)
    }
    
    /// Verifica si existe TopicGraph para usuario
    pub fn exists(&self, user_id: &str) -> bool {
        let file_path = self.storage_dir.join(format!("{}.json", user_id));
        file_path.exists()
    }
    
    /// Elimina TopicGraph de usuario
    pub fn delete(&self, user_id: &str) -> Result<()> {
        let file_path = self.storage_dir.join(format!("{}.json", user_id));
        
        if !file_path.exists() {
            return Err(ShuiDaoError::NotFound(format!(
                "TopicGraph for user '{}' not found",
                user_id
            )));
        }
        
        fs::remove_file(&file_path)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to delete file: {}", e),
            })?;
        
        Ok(())
    }
    
    /// Lista todos los user_ids con TopicGraphs
    pub fn list_users(&self) -> Result<Vec<String>> {
        if !self.storage_dir.exists() {
            return Ok(Vec::new());
        }
        
        let entries = fs::read_dir(&self.storage_dir)
            .map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to read directory: {}", e),
            })?;
        
        let mut user_ids = Vec::new();
        
        for entry in entries {
            let entry = entry.map_err(|e| ShuiDaoError::MemoryAccessError {
                component: "TopicStorage".to_string(),
                details: format!("Failed to read entry: {}", e),
            })?;
            
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    user_ids.push(stem.to_string());
                }
            }
        }
        
        Ok(user_ids)
    }
}

impl Default for TopicStorage {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// MTT-DSL TEMPLATE GENERATION (v1.0 simple YAML)
// ================================================================

/// Genera template MTT-DSL desde TopicNode
///
/// # Format (YAML)
/// ```yaml
/// id: topic_rust_001
/// name: Rust
/// type: topic
/// metadata:
///   user_defined: true
///   interest_weight: 0.92
///   mention_count: 125
/// prompt_context: |
///   El usuario tiene alto interés en Rust (weight: 0.92).
///   Ha mencionado este tema 125 veces.
///   Últi ma mención: 2025-11-26.
/// ```
pub fn generate_topic_template(graph: &TopicGraph, topic_id: &str) -> Result<String> {
    let node = graph.nodes.get(topic_id)
        .ok_or_else(|| ShuiDaoError::TopicNotFound(topic_id.to_string()))?;
    
    let interest_level = if node.interest_weight.combined > 0.8 {
        "alto"
    } else if node.interest_weight.combined > 0.5 {
        "moderado"
    } else {
        "bajo"
    };
    
    let template = format!(
        r#"# MTT-DSL Topic Template
# Generated: {}
# User: {}

id: {}
name: {}
type: topic
category: user_defined

metadata:
  user_defined: true
  interest_weight: {:.2}
  mention_count: {}
  created_at: {}
  last_mentioned: {}

prompt_context: |
  El usuario tiene {} interés en {} (peso: {:.2}).
  Ha mencionado este tema {} veces.
  Última mención: {}.
  
  Este topic fue aprendido dinámicamente desde la conversación.
  Ajusta las respuestas considerando este nivel de interés.

embedding_dims: {}
similarity_threshold: 0.75
"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        graph.user_id,
        node.id,
        node.name,
        node.interest_weight.combined,
        node.mention_count,
        node.created_at.format("%Y-%m-%d %H:%M:%S"),
        node.last_mentioned.format("%Y-%m-%d %H:%M:%S"),
        interest_level,
        node.name,
        node.interest_weight.combined,
        node.mention_count,
        node.last_mentioned.format("%Y-%m-%d"),
        node.embedding.len(),
    );
    
    Ok(template)
}

/// Guarda template MTT-DSL a archivo
pub fn save_topic_template(
    graph: &TopicGraph,
    topic_id: &str,
    output_dir: &Path,
) -> Result<PathBuf> {
    let template = generate_topic_template(graph, topic_id)?;
    
    // Crear directorio si no existe
    fs::create_dir_all(output_dir)
        .map_err(|e| ShuiDaoError::MemoryAccessError {
            component: "TopicIntegration".to_string(),
            details: format!("Failed to create template dir: {}", e),
        })?;
    
    // Ruta del archivo
    let file_path = output_dir.join(format!("{}.yaml", topic_id));
    
    // Escribir template
    fs::write(&file_path, template)
        .map_err(|e| ShuiDaoError::MemoryAccessError {
            component: "TopicIntegration".to_string(),
            details: format!("Failed to write template: {}", e),
        })?;
    
    Ok(file_path)
}

// ================================================================
// TESTS
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::topic_graph::generate_embedding_stub;
    use tempfile::TempDir;
    
    #[test]
    fn test_topic_storage_creation() {
        let storage = TopicStorage::new();
        assert!(storage.storage_dir.to_string_lossy().contains("topic_graphs"));
    }
    
    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TopicStorage::with_dir(temp_dir.path().to_path_buf());
        
        // Create graph
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust");
        graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        // Save
        storage.save(&graph).unwrap();
        
        // Verify file exists
        assert!(storage.exists("eduardo_001"));
        
        // Load
        let loaded = storage.load("eduardo_001").unwrap();
        
        // Verify content
        assert_eq!(loaded.user_id, "eduardo_001");
        assert_eq!(loaded.nodes.len(), 1);
        assert!(loaded.nodes.values().any(|n| n.name == "Rust"));
    }
    
    #[test]
    fn test_load_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TopicStorage::with_dir(temp_dir.path().to_path_buf());
        
        let result = storage.load("nonexistent_user");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TopicStorage::with_dir(temp_dir.path().to_path_buf());
        
        // Create and save
        let graph = TopicGraph::new("eduardo_001".to_string());
        storage.save(&graph).unwrap();
        
        assert!(storage.exists("eduardo_001"));
        
        // Delete
        storage.delete("eduardo_001").unwrap();
        
        assert!(!storage.exists("eduardo_001"));
    }
    
    #[test]
    fn test_list_users() {
        let temp_dir = TempDir::new().unwrap();
        let storage = TopicStorage::with_dir(temp_dir.path().to_path_buf());
        
        // Create multiple graphs
        let graph1 = TopicGraph::new("user1".to_string());
        let graph2 = TopicGraph::new("user2".to_string());
        let graph3 = TopicGraph::new("user3".to_string());
        
        storage.save(&graph1).unwrap();
        storage.save(&graph2).unwrap();
        storage.save(&graph3).unwrap();
        
        // List
        let users = storage.list_users().unwrap();
        
        assert_eq!(users.len(), 3);
        assert!(users.contains(&"user1".to_string()));
        assert!(users.contains(&"user2".to_string()));
        assert!(users.contains(&"user3".to_string()));
    }
    
    #[test]
    fn test_generate_topic_template() {
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust programming");
        let topic_id = graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        // Mention para aumentar weight
        graph.mention_topic(&topic_id, true).unwrap();
        
        let template = generate_topic_template(&graph, &topic_id).unwrap();
        
        // Verify content
        assert!(template.contains("MTT-DSL Topic Template"));
        assert!(template.contains("name: Rust"));
        assert!(template.contains("user_defined: true"));
        assert!(template.contains(&topic_id));
    }
    
    #[test]
    fn test_save_topic_template() {
        let temp_dir = TempDir::new().unwrap();
        let mut graph = TopicGraph::new("eduardo_001".to_string());
        let embedding = generate_embedding_stub("Rust");
        let topic_id = graph.add_topic("Rust".to_string(), embedding).unwrap();
        
        let file_path = save_topic_template(&graph, &topic_id, temp_dir.path()).unwrap();
        
        // Verify file exists
        assert!(file_path.exists());
        assert!(file_path.extension().unwrap() == "yaml");
        
        // Read and verify content
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("MTT-DSL Topic Template"));
        assert!(content.contains("name: Rust"));
    }
}
