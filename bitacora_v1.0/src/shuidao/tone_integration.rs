// ================================================================
// Tone Integration - Persistence + MTT-DSL Templates (DA-033)
// ================================================================
//
// Purpose: JSON persistence + MTT-DSL YAML template generation for tones
// Storage: ./data/emotional_spaces/{user_id}.json
// Templates: MTT-DSL YAML for tone metadata + response adaptation
//
// Architecture:
//   - ToneStorage: CRUD operations on EmotionalSpace (JSON)
//   - generate_tone_template(): MTT-DSL YAML generation
//   - save_tone_template(): Write template to file
//
// Performance: <50ms I/O (save/load)
//
// Creado: 2025-11-26 19:45:00
// Especificación: ROADMAP_V2/02_COMPONENTES/CRITICOS/15_shuidao-emotional-space.md
// ================================================================

use serde_json;
use std::fs;
use std::path::{Path, PathBuf};

use crate::shuidao::emotional_space::{EmotionalSpace, ToneCluster, ToneClusterId};
use crate::shuidao::error::{Result, ShuiDaoError};

// ================================================================
// TONE STORAGE (JSON Persistence)
// ================================================================

/// Almacenamiento de EmotionalSpaces en disco (JSON)
///
/// Cada usuario tiene su propio archivo: ./data/emotional_spaces/{user_id}.json
pub struct ToneStorage {
    /// Directorio base para almacenamiento
    storage_dir: PathBuf,
}

impl ToneStorage {
    /// Crea nuevo storage con directorio por defecto
    pub fn new() -> Self {
        Self {
            storage_dir: PathBuf::from("./data/emotional_spaces"),
        }
    }

    /// Crea storage con directorio personalizado
    pub fn with_dir<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            storage_dir: dir.as_ref().to_path_buf(),
        }
    }

    /// Guarda EmotionalSpace en disco (JSON)
    ///
    /// Target performance: <50ms
    pub fn save(&self, space: &EmotionalSpace) -> Result<()> {
        // Crear directorio si no existe
        fs::create_dir_all(&self.storage_dir).map_err(|e| {
            ShuiDaoError::IoError(format!("Failed to create directory: {}", e))
        })?;

        // Serializar a JSON
        let json = serde_json::to_string_pretty(space).map_err(|e| {
            ShuiDaoError::SerializationError(format!("Failed to serialize EmotionalSpace: {}", e))
        })?;

        // Escribir archivo
        let file_path = self.get_file_path(&space.user_id);
        fs::write(&file_path, json).map_err(|e| {
            ShuiDaoError::IoError(format!("Failed to write file: {}", e))
        })?;

        Ok(())
    }

    /// Carga EmotionalSpace desde disco (JSON)
    ///
    /// Target performance: <50ms
    pub fn load(&self, user_id: &str) -> Result<EmotionalSpace> {
        let file_path = self.get_file_path(user_id);

        // Verificar que existe
        if !file_path.exists() {
            return Err(ShuiDaoError::TopicNotFound(format!(
                "EmotionalSpace not found for user: {}",
                user_id
            )));
        }

        // Leer archivo
        let json = fs::read_to_string(&file_path).map_err(|e| {
            ShuiDaoError::IoError(format!("Failed to read file: {}", e))
        })?;

        // Deserializar
        let space: EmotionalSpace = serde_json::from_str(&json).map_err(|e| {
            ShuiDaoError::SerializationError(format!("Failed to deserialize EmotionalSpace: {}", e))
        })?;

        Ok(space)
    }

    /// Verifica si existe EmotionalSpace para un usuario
    pub fn exists(&self, user_id: &str) -> bool {
        self.get_file_path(user_id).exists()
    }

    /// Elimina EmotionalSpace de un usuario
    pub fn delete(&self, user_id: &str) -> Result<()> {
        let file_path = self.get_file_path(user_id);

        if file_path.exists() {
            fs::remove_file(&file_path).map_err(|e| {
                ShuiDaoError::IoError(format!("Failed to delete file: {}", e))
            })?;
        }

        Ok(())
    }

    /// Lista todos los user_ids con EmotionalSpace guardado
    pub fn list_users(&self) -> Result<Vec<String>> {
        if !self.storage_dir.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&self.storage_dir).map_err(|e| {
            ShuiDaoError::IoError(format!("Failed to read directory: {}", e))
        })?;

        let mut users = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|e| {
                ShuiDaoError::IoError(format!("Failed to read entry: {}", e))
            })?;

            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    users.push(stem.to_string());
                }
            }
        }

        Ok(users)
    }

    /// Obtiene path del archivo para un usuario
    fn get_file_path(&self, user_id: &str) -> PathBuf {
        self.storage_dir.join(format!("{}.json", user_id))
    }
}

impl Default for ToneStorage {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// MTT-DSL TEMPLATE GENERATION
// ================================================================

/// Genera template MTT-DSL (YAML) para un ToneCluster
///
/// Template incluye:
/// - Metadata (nombre, user_id, discovered_at)
/// - Dimensiones VAD+F
/// - Lexical markers
/// - Syntactic patterns
/// - Response adaptation guidelines
pub fn generate_tone_template(
    space: &EmotionalSpace,
    cluster_id: &ToneClusterId,
) -> Result<String> {
    let cluster = space
        .get_cluster(cluster_id)
        .ok_or_else(|| ShuiDaoError::TopicNotFound(cluster_id.clone()))?;

    let interest_level = match cluster.center.valence {
        v if v > 0.6 => "muy positivo",
        v if v > 0.2 => "positivo",
        v if v > -0.2 => "neutral",
        v if v > -0.6 => "negativo",
        _ => "muy negativo",
    };

    let energy_level = match cluster.center.arousal {
        a if a > 0.6 => "muy energético",
        a if a > 0.2 => "energético",
        a if a > -0.2 => "neutral",
        a if a > -0.6 => "calmado",
        _ => "muy calmado",
    };

    let assertiveness = match cluster.center.dominance {
        d if d > 0.6 => "muy asertivo",
        d if d > 0.2 => "asertivo",
        d if d > -0.2 => "neutral",
        d if d > -0.6 => "pasivo",
        _ => "muy pasivo",
    };

    let formality_level = match cluster.center.formality {
        f if f > 0.6 => "muy formal",
        f if f > 0.2 => "formal",
        f if f > -0.2 => "neutral",
        f if f > -0.6 => "casual",
        _ => "muy casual",
    };

    let template = format!(
        r#"# MTT-DSL Tone Template
# Generated: {}
# User: {}

metadata:
  name: "{}"
  user_id: "{}"
  discovered_at: "{}"
  cluster_id: "{}"
  version: "1.0.0"

dimensions:
  valence: {:.2}      # {}
  arousal: {:.2}      # {}
  dominance: {:.2}    # {}
  formality: {:.2}    # {}

cluster:
  center: [{:.2}, {:.2}, {:.2}, {:.2}]
  radius: {:.2}
  interaction_count: {}

lexical_markers:
  strong_verbs: {}
  commitment_phrases: {}
  time_markers: {}
  emotional_adjectives: {}
  exclamation_count: {}
  question_count: {}

examples:
{}

response_adaptation:
  style: "{}"
  energy_level: {:.2}
  match_user_energy: true
  
  tone_description: |
    El usuario está en tono "{}": {}, {}, {}, {}.
    Ha usado este tono {} veces.

  guidelines:
    - "Responder con nivel de energía similar (arousal: {:.2})"
    - "Ajustar formalidad del lenguaje (formality: {:.2})"
    - "Considerar asertividad del usuario (dominance: {:.2})"
    - "Mantener valencia emocional apropiada (valence: {:.2})"

embedding_dims: 4
similarity_threshold: {:.2}
"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
        space.user_id,
        cluster.name,
        space.user_id,
        cluster.discovered_at.format("%Y-%m-%d %H:%M:%S"),
        cluster.id,
        cluster.center.valence,
        interest_level,
        cluster.center.arousal,
        energy_level,
        cluster.center.dominance,
        assertiveness,
        cluster.center.formality,
        formality_level,
        cluster.center.valence,
        cluster.center.arousal,
        cluster.center.dominance,
        cluster.center.formality,
        cluster.radius,
        cluster.interaction_count,
        format_string_vec(&cluster.lexical_markers.strong_verbs),
        format_string_vec(&cluster.lexical_markers.commitment_phrases),
        format_string_vec(&cluster.lexical_markers.time_markers),
        format_string_vec(&cluster.lexical_markers.emotional_adjectives),
        cluster.lexical_markers.exclamation_count,
        cluster.lexical_markers.question_count,
        format_examples(&cluster.examples),
        determine_style(&cluster.center),
        cluster.center.arousal,
        cluster.name,
        interest_level,
        energy_level,
        assertiveness,
        formality_level,
        cluster.interaction_count,
        cluster.center.arousal,
        cluster.center.formality,
        cluster.center.dominance,
        cluster.center.valence,
        cluster.radius,
    );

    Ok(template)
}

/// Guarda template MTT-DSL en archivo
pub fn save_tone_template(
    template: &str,
    user_id: &str,
    tone_name: &str,
    output_dir: Option<&Path>,
) -> Result<PathBuf> {
    let base_dir = output_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("./templates/tone_templates"));

    let user_dir = base_dir.join(user_id);
    fs::create_dir_all(&user_dir).map_err(|e| {
        ShuiDaoError::IoError(format!("Failed to create directory: {}", e))
    })?;

    let file_path = user_dir.join(format!("{}.yaml", sanitize_filename(tone_name)));
    fs::write(&file_path, template).map_err(|e| {
        ShuiDaoError::IoError(format!("Failed to write template: {}", e))
    })?;

    Ok(file_path)
}

// ================================================================
// HELPERS
// ================================================================

fn format_string_vec(vec: &[String]) -> String {
    if vec.is_empty() {
        return "[]".to_string();
    }

    let items: Vec<String> = vec.iter().map(|s| format!("\"{}\"", s)).collect();
    format!("[{}]", items.join(", "))
}

fn format_examples(examples: &[String]) -> String {
    if examples.is_empty() {
        return "  - \"(no examples yet)\"".to_string();
    }

    examples
        .iter()
        .take(5)
        .map(|e| format!("  - \"{}\"", e.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join("\n")
}

fn determine_style(dims: &crate::shuidao::emotional_space::ToneDimensions) -> &'static str {
    // Heurística simple para determinar estilo de respuesta
    if dims.arousal > 0.5 && dims.dominance > 0.5 {
        "direct_energetic"
    } else if dims.arousal < -0.3 && dims.dominance < 0.0 {
        "reflective_gentle"
    } else if dims.valence > 0.6 {
        "enthusiastic_supportive"
    } else if dims.valence < -0.5 {
        "empathetic_careful"
    } else {
        "balanced_neutral"
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// ================================================================
// TESTS
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shuidao::emotional_space::{LexicalMarkers, ToneCluster, ToneDimensions};
    use tempfile::TempDir;

    fn create_test_space() -> EmotionalSpace {
        let mut space = EmotionalSpace::new("eduardo_001".to_string());

        let mut cluster = ToneCluster::new(
            "tone_001".to_string(),
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "eduardo_001".to_string(),
        );

        cluster.examples = vec![
            "Voy a terminarlo".to_string(),
            "Sin excusas".to_string(),
        ];

        let mut markers = LexicalMarkers::new();
        markers.strong_verbs = vec!["voy a".to_string(), "terminar".to_string()];
        markers.exclamation_count = 1;
        cluster.lexical_markers = markers;

        space.add_cluster(cluster);
        space
    }

    #[test]
    fn test_tone_storage_creation() {
        let storage = ToneStorage::new();
        assert_eq!(storage.storage_dir, PathBuf::from("./data/emotional_spaces"));
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ToneStorage::with_dir(temp_dir.path());

        let space = create_test_space();
        storage.save(&space).unwrap();

        let loaded = storage.load("eduardo_001").unwrap();
        assert_eq!(loaded.user_id, "eduardo_001");
        assert_eq!(loaded.clusters.len(), 1);
    }

    #[test]
    fn test_load_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ToneStorage::with_dir(temp_dir.path());

        let result = storage.load("nonexistent_001");
        assert!(result.is_err());
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ToneStorage::with_dir(temp_dir.path());

        assert!(!storage.exists("eduardo_001"));

        let space = create_test_space();
        storage.save(&space).unwrap();

        assert!(storage.exists("eduardo_001"));
    }

    #[test]
    fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ToneStorage::with_dir(temp_dir.path());

        let space = create_test_space();
        storage.save(&space).unwrap();
        assert!(storage.exists("eduardo_001"));

        storage.delete("eduardo_001").unwrap();
        assert!(!storage.exists("eduardo_001"));
    }

    #[test]
    fn test_list_users() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ToneStorage::with_dir(temp_dir.path());

        let space1 = create_test_space();
        storage.save(&space1).unwrap();

        let mut space2 = EmotionalSpace::new("esposa_001".to_string());
        let cluster = ToneCluster::new(
            "tone_002".to_string(),
            "Test".to_string(),
            ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
            "esposa_001".to_string(),
        );
        space2.add_cluster(cluster);
        storage.save(&space2).unwrap();

        let users = storage.list_users().unwrap();
        assert_eq!(users.len(), 2);
        assert!(users.contains(&"eduardo_001".to_string()));
        assert!(users.contains(&"esposa_001".to_string()));
    }

    #[test]
    fn test_generate_tone_template() {
        let space = create_test_space();
        let template = generate_tone_template(&space, &"tone_001".to_string()).unwrap();

        assert!(template.contains("MTT-DSL Tone Template"));
        assert!(template.contains("name: \"Determinado\""));
        assert!(template.contains("user_id: \"eduardo_001\""));
        assert!(template.contains("dimensions:"));
        assert!(template.contains("valence:"));
        assert!(template.contains("arousal:"));
        assert!(template.contains("dominance:"));
        assert!(template.contains("formality:"));
    }

    #[test]
    fn test_save_tone_template() {
        let temp_dir = TempDir::new().unwrap();
        let space = create_test_space();
        let template = generate_tone_template(&space, &"tone_001".to_string()).unwrap();

        let path = save_tone_template(&template, "eduardo_001", "Determinado", Some(temp_dir.path()))
            .unwrap();

        assert!(path.exists());
        assert!(path.to_str().unwrap().contains("Determinado.yaml"));

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("MTT-DSL"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Normal"), "Normal");
        assert_eq!(sanitize_filename("With Spaces"), "With_Spaces");
        assert_eq!(sanitize_filename("Special!@#$"), "Special____");
        // Rust is_alphanumeric() supports Unicode (ú, á, ñ, etc)
        assert_eq!(sanitize_filename("números-123"), "números-123");
    }

    #[test]
    fn test_format_string_vec() {
        let empty: Vec<String> = vec![];
        assert_eq!(format_string_vec(&empty), "[]");

        let items = vec!["one".to_string(), "two".to_string()];
        assert_eq!(format_string_vec(&items), "[\"one\", \"two\"]");
    }
}
