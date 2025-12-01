// ================================================================
// Tone Learning - Auto-Discovery de Tonos Emocionales (DA-033)
// ================================================================
//
// Purpose: Auto-discover new emotional tones from conversation
// Flow: Detect candidate → User confirms → Add to EmotionalSpace
// User-Named: "Determinado", "Nostálgico", "Mi tono reflexivo nocturno"
//
// Algorithm:
//   1. Analyze message dimensions (VAD+F)
//   2. Check if matches existing clusters
//   3. If novel → create ToneCandidate
//   4. After N mentions → suggest to user
//   5. User confirms with custom name
//   6. Create ToneCluster in EmotionalSpace
//
// Creado: 2025-11-26 19:30:00
// Especificación: ROADMAP_V2/02_COMPONENTES/CRITICOS/15_shuidao-emotional-space.md
// ================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shuidao::error::{Result, ShuiDaoError};
use crate::shuidao::emotional_space::{
    EmotionalSpace, LexicalMarkers, SyntacticPattern, ToneCluster, ToneClusterId, ToneDimensions,
    ToneDetector,
};

// ================================================================
// TONE CANDIDATE (Pending Confirmation)
// ================================================================

/// Candidato a tono emocional (aún no confirmado por usuario)
///
/// Se genera cuando detectamos dimensiones que no matchean ningún cluster existente.
/// Después de N menciones, se sugiere al usuario para confirmación.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneCandidate {
    /// Nombre tentativo (será reemplazado por nombre de usuario)
    pub tentative_name: String,

    /// Dimensiones promedio del candidato
    pub dimensions: ToneDimensions,

    /// Contexto donde se detectó
    pub context: String,

    /// Score de relevancia [0.0 - 1.0]
    pub relevance_score: f32,

    /// Ejemplos de mensajes con este tono
    pub examples: Vec<String>,

    /// Número de veces mencionado
    pub mention_count: usize,

    /// Markers léxicos extraídos
    pub lexical_markers: LexicalMarkers,
}

impl ToneCandidate {
    /// Crea nuevo candidato
    pub fn new(
        tentative_name: String,
        dimensions: ToneDimensions,
        context: String,
        relevance_score: f32,
    ) -> Self {
        Self {
            tentative_name,
            dimensions,
            context,
            relevance_score,
            examples: Vec::new(),
            mention_count: 1,
            lexical_markers: LexicalMarkers::new(),
        }
    }

    /// Añade ejemplo al candidato
    pub fn add_example(&mut self, text: String) {
        if !self.examples.contains(&text) {
            self.examples.push(text);
            self.mention_count += 1;
        }
    }

    /// Actualiza dimensiones con nueva observación (promedio móvil)
    pub fn update_dimensions(&mut self, new_dims: ToneDimensions) {
        let n = self.mention_count as f32;
        let weight_old = (n - 1.0) / n;
        let weight_new = 1.0 / n;

        self.dimensions = ToneDimensions::new(
            self.dimensions.valence * weight_old + new_dims.valence * weight_new,
            self.dimensions.arousal * weight_old + new_dims.arousal * weight_new,
            self.dimensions.dominance * weight_old + new_dims.dominance * weight_new,
            self.dimensions.formality * weight_old + new_dims.formality * weight_new,
        );
    }
}

// ================================================================
// TONE SUGGESTION (For User Confirmation)
// ================================================================

/// Sugerencia de tono para confirmación de usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneSuggestion {
    /// Nombre tentativo del tono
    pub name: String,

    /// Dimensiones detectadas
    pub dimensions: ToneDimensions,

    /// Descripción en lenguaje natural
    pub description: String,

    /// Número de veces detectado
    pub mention_count: usize,

    /// Ejemplos de mensajes
    pub examples: Vec<String>,
}

impl ToneSuggestion {
    /// Crea sugerencia desde candidato
    pub fn from_candidate(candidate: &ToneCandidate) -> Self {
        let description = format!(
            "Tono {}: {}, {}, {}",
            candidate.tentative_name,
            candidate.dimensions.describe_valence(),
            candidate.dimensions.describe_arousal(),
            candidate.dimensions.describe_dominance()
        );

        Self {
            name: candidate.tentative_name.clone(),
            dimensions: candidate.dimensions,
            description,
            mention_count: candidate.mention_count,
            examples: candidate.examples.clone(),
        }
    }
}

// ================================================================
// TONE LEARNER (Main Interface)
// ================================================================

/// Aprendiz de tonos emocionales
///
/// Analiza conversación y propone nuevos clusters de tonos cuando detecta
/// patrones emocionales consistentes que no matchean clusters existentes.
pub struct ToneLearner {
    /// Threshold de relevancia para crear candidato [0.0 - 1.0]
    pub relevance_threshold: f32,

    /// Mínimo de menciones antes de sugerir al usuario
    pub min_mentions_to_suggest: usize,

    /// Candidatos pendientes de confirmación
    pub pending_candidates: HashMap<String, ToneCandidate>,

    /// Detector de tonos (para verificar novedad)
    tone_detector: ToneDetector,

    /// Threshold de distancia para considerar "similar" (default: 0.5)
    similarity_threshold: f32,
}

impl ToneLearner {
    /// Crea nuevo learner con configuración por defecto
    pub fn new() -> Self {
        Self {
            relevance_threshold: 0.6,
            min_mentions_to_suggest: 2,
            pending_candidates: HashMap::new(),
            tone_detector: ToneDetector::new(),
            similarity_threshold: 0.5,
        }
    }

    /// Crea learner con configuración personalizada
    pub fn with_config(
        relevance_threshold: f32,
        min_mentions_to_suggest: usize,
        similarity_threshold: f32,
    ) -> Self {
        Self {
            relevance_threshold,
            min_mentions_to_suggest,
            pending_candidates: HashMap::new(),
            tone_detector: ToneDetector::new(),
            similarity_threshold,
        }
    }

    /// Extrae candidatos de un mensaje
    ///
    /// Analiza si el tono del mensaje es novedoso (no matchea clusters existentes).
    /// Si es novedoso y relevante, crea o actualiza ToneCandidate.
    pub fn extract_candidates(
        &mut self,
        message: &str,
        space: &EmotionalSpace,
    ) -> Vec<ToneCandidate> {
        // 1. Detectar dimensiones del mensaje
        let detection = self.tone_detector.detect(message, space);

        // 2. Si matchea cluster existente, no es candidato
        if !detection.is_new_tone {
            return Vec::new();
        }

        // 3. Calcular score de novedad (distancia a clusters existentes)
        let novelty_score = self.compute_novelty_score(&detection.dimensions, space);

        // 4. Si no es suficientemente novedoso, ignorar
        if novelty_score < self.relevance_threshold {
            return Vec::new();
        }

        // 5. Generar nombre tentativo basado en dimensiones
        let tentative_name = self.generate_tentative_name(&detection.dimensions);

        // 6. Verificar si ya existe candidato similar
        if let Some(candidate) = self.find_similar_candidate(&detection.dimensions) {
            // Actualizar candidato existente
            candidate.add_example(message.to_string());
            candidate.update_dimensions(detection.dimensions);
            vec![candidate.clone()]
        } else {
            // Crear nuevo candidato
            let mut candidate = ToneCandidate::new(
                tentative_name.clone(),
                detection.dimensions,
                message.to_string(),
                novelty_score,
            );
            candidate.add_example(message.to_string());

            self.pending_candidates
                .insert(tentative_name.clone(), candidate.clone());

            vec![candidate]
        }
    }

    /// Calcula score de novedad (qué tan diferente es de clusters existentes)
    ///
    /// Formula: 1.0 - (similarity_to_nearest_cluster)
    /// Si muy diferente → score alto (cercano a 1.0)
    /// Si muy similar → score bajo (cercano a 0.0)
    fn compute_novelty_score(&self, dimensions: &ToneDimensions, space: &EmotionalSpace) -> f32 {
        if space.clusters.is_empty() {
            return 1.0; // Primer cluster, máxima novedad
        }

        // Encontrar distancia al cluster más cercano
        let min_distance = space
            .clusters
            .values()
            .map(|cluster| cluster.center.distance_to(dimensions))
            .fold(f32::INFINITY, f32::min);

        // Normalizar: distancia → novelty score
        // Distancia 0 → novelty 0
        // Distancia alta → novelty alta
        let max_distance = 2.0_f32.sqrt() * 2.0; // Máxima distancia posible en espacio 4D [-1,1]
        (min_distance / max_distance).min(1.0)
    }

    /// Genera nombre tentativo basado en dimensiones dominantes
    fn generate_tentative_name(&self, dims: &ToneDimensions) -> String {
        // Identificar dimensión dominante
        let abs_valence = dims.valence.abs();
        let abs_arousal = dims.arousal.abs();
        let abs_dominance = dims.dominance.abs();
        let abs_formality = dims.formality.abs();

        let max_val = abs_valence.max(abs_arousal).max(abs_dominance).max(abs_formality);

        if max_val == abs_valence {
            if dims.valence > 0.0 {
                "Positivo".to_string()
            } else {
                "Negativo".to_string()
            }
        } else if max_val == abs_arousal {
            if dims.arousal > 0.0 {
                "Energético".to_string()
            } else {
                "Calmado".to_string()
            }
        } else if max_val == abs_dominance {
            if dims.dominance > 0.0 {
                "Asertivo".to_string()
            } else {
                "Reflexivo".to_string()
            }
        } else {
            if dims.formality > 0.0 {
                "Formal".to_string()
            } else {
                "Casual".to_string()
            }
        }
    }

    /// Busca candidato similar existente
    fn find_similar_candidate(&mut self, dimensions: &ToneDimensions) -> Option<&mut ToneCandidate> {
        for candidate in self.pending_candidates.values_mut() {
            let distance = candidate.dimensions.distance_to(dimensions);
            if distance < self.similarity_threshold {
                return Some(candidate);
            }
        }
        None
    }

    /// Obtiene sugerencias listas para confirmación
    ///
    /// Returns: Lista de candidatos con mention_count >= min_mentions_to_suggest
    pub fn get_suggestions(&self) -> Vec<ToneSuggestion> {
        self.pending_candidates
            .values()
            .filter(|c| c.mention_count >= self.min_mentions_to_suggest)
            .map(|c| ToneSuggestion::from_candidate(c))
            .collect()
    }

    /// Confirma candidato y crea cluster en EmotionalSpace
    ///
    /// # Arguments
    /// * `candidate_name` - Nombre tentativo del candidato
    /// * `user_name` - Nombre que el usuario quiere darle al tono
    /// * `space` - EmotionalSpace donde agregar el cluster
    ///
    /// # Returns
    /// ToneClusterId del nuevo cluster creado
    pub fn confirm_candidate(
        &mut self,
        candidate_name: &str,
        user_name: String,
        space: &mut EmotionalSpace,
    ) -> Result<ToneClusterId> {
        // Buscar candidato
        let candidate = self
            .pending_candidates
            .remove(candidate_name)
            .ok_or_else(|| ShuiDaoError::TopicNotFound(candidate_name.to_string()))?;

        // Crear ToneClusterId único
        let cluster_id = format!("tone_{}_{:03}", space.user_id, space.clusters.len() + 1);

        // Crear ToneCluster
        let mut cluster = ToneCluster::new(
            cluster_id.clone(),
            user_name,
            candidate.dimensions,
            space.user_id.clone(),
        );

        // Copiar ejemplos y markers
        cluster.examples = candidate.examples;
        cluster.lexical_markers = candidate.lexical_markers;
        cluster.interaction_count = candidate.mention_count as u32;

        // Agregar a EmotionalSpace
        space.add_cluster(cluster);

        Ok(cluster_id)
    }

    /// Rechaza candidato (no crear cluster)
    pub fn reject_candidate(&mut self, candidate_name: &str) -> Result<()> {
        self.pending_candidates
            .remove(candidate_name)
            .ok_or_else(|| ShuiDaoError::TopicNotFound(candidate_name.to_string()))?;
        Ok(())
    }

    /// Genera mensaje de sugerencia para el usuario
    pub fn format_suggestion_message(&self, suggestion: &ToneSuggestion) -> String {
        format!(
            "Detecto un nuevo tono en ti:\n\
             - {}\n\
             - Mencionado {} veces\n\
             - Ejemplos:\n{}\n\
             ¿Cómo lo describirías?",
            suggestion.description,
            suggestion.mention_count,
            suggestion
                .examples
                .iter()
                .take(3)
                .map(|e| format!("  • \"{}\"", e))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Cuenta candidatos pendientes
    pub fn pending_count(&self) -> usize {
        self.pending_candidates.len()
    }

    /// Limpia candidatos con mention_count bajo (housekeeping)
    pub fn cleanup_low_mention_candidates(&mut self, max_age_mentions: usize) {
        self.pending_candidates
            .retain(|_, c| c.mention_count >= max_age_mentions || c.mention_count > 0);
    }
}

impl Default for ToneLearner {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// TESTS
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tone_candidate_creation() {
        let candidate = ToneCandidate::new(
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "Voy a terminar esto".to_string(),
            0.85,
        );

        assert_eq!(candidate.tentative_name, "Determinado");
        assert_eq!(candidate.mention_count, 1);
        assert!((candidate.relevance_score - 0.85).abs() < 0.01);
    }

    #[test]
    fn test_candidate_add_example() {
        let mut candidate = ToneCandidate::new(
            "Test".to_string(),
            ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
            "context".to_string(),
            0.5,
        );

        candidate.add_example("example 1".to_string());
        candidate.add_example("example 2".to_string());
        candidate.add_example("example 1".to_string()); // Duplicate

        assert_eq!(candidate.examples.len(), 2);
        assert_eq!(candidate.mention_count, 3);
    }

    #[test]
    fn test_candidate_update_dimensions() {
        let mut candidate = ToneCandidate::new(
            "Test".to_string(),
            ToneDimensions::new(0.5, 0.5, 0.5, 0.5),
            "context".to_string(),
            0.5,
        );

        // Primera actualización (n=2)
        candidate.add_example("msg".to_string());
        candidate.update_dimensions(ToneDimensions::new(1.0, 1.0, 1.0, 1.0));

        // Promedio: (0.5 + 1.0) / 2 = 0.75
        assert!((candidate.dimensions.valence - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_tone_learner_creation() {
        let learner = ToneLearner::new();

        assert_eq!(learner.relevance_threshold, 0.6);
        assert_eq!(learner.min_mentions_to_suggest, 2);
        assert_eq!(learner.pending_count(), 0);
    }

    #[test]
    fn test_novelty_score_empty_space() {
        let learner = ToneLearner::new();
        let space = EmotionalSpace::new("test_001".to_string());
        let dims = ToneDimensions::new(0.5, 0.5, 0.5, 0.5);

        let score = learner.compute_novelty_score(&dims, &space);

        assert_eq!(score, 1.0); // Primer cluster, máxima novedad
    }

    #[test]
    fn test_novelty_score_similar_cluster() {
        let learner = ToneLearner::new();
        let mut space = EmotionalSpace::new("test_001".to_string());

        // Añadir cluster existente
        let cluster = ToneCluster::new(
            "tone_001".to_string(),
            "Test".to_string(),
            ToneDimensions::new(0.5, 0.5, 0.5, 0.5),
            "test_001".to_string(),
        );
        space.add_cluster(cluster);

        // Dimensiones muy similares
        let similar = ToneDimensions::new(0.52, 0.48, 0.51, 0.49);
        let score = learner.compute_novelty_score(&similar, &space);

        assert!(score < 0.3); // Baja novedad (muy similar)
    }

    #[test]
    fn test_generate_tentative_name() {
        let learner = ToneLearner::new();

        // Alta valencia positiva
        let positive = ToneDimensions::new(0.9, 0.1, 0.1, 0.1);
        assert_eq!(learner.generate_tentative_name(&positive), "Positivo");

        // Alto arousal
        let energetic = ToneDimensions::new(0.1, 0.9, 0.1, 0.1);
        assert_eq!(learner.generate_tentative_name(&energetic), "Energético");

        // Alta dominancia
        let assertive = ToneDimensions::new(0.1, 0.1, 0.9, 0.1);
        assert_eq!(learner.generate_tentative_name(&assertive), "Asertivo");

        // Alta formalidad
        let formal = ToneDimensions::new(0.1, 0.1, 0.1, 0.9);
        assert_eq!(learner.generate_tentative_name(&formal), "Formal");
    }

    #[test]
    fn test_extract_candidates_new_tone() {
        let mut learner = ToneLearner::new();
        let space = EmotionalSpace::new("eduardo_001".to_string());

        // Primer mensaje → nuevo candidato
        let candidates = learner.extract_candidates("¡Estoy muy emocionado!", &space);

        assert_eq!(candidates.len(), 1);
        assert_eq!(learner.pending_count(), 1);
    }

    #[test]
    fn test_get_suggestions() {
        let mut learner = ToneLearner::new();
        learner.min_mentions_to_suggest = 2;

        // Candidato con 1 mención (no suficiente)
        let candidate1 = ToneCandidate::new(
            "Test1".to_string(),
            ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
            "ctx".to_string(),
            0.7,
        );
        learner
            .pending_candidates
            .insert("Test1".to_string(), candidate1);

        // Candidato con 3 menciones (suficiente)
        let mut candidate2 = ToneCandidate::new(
            "Test2".to_string(),
            ToneDimensions::new(0.5, 0.5, 0.5, 0.5),
            "ctx".to_string(),
            0.8,
        );
        candidate2.add_example("msg1".to_string());
        candidate2.add_example("msg2".to_string());
        learner
            .pending_candidates
            .insert("Test2".to_string(), candidate2);

        let suggestions = learner.get_suggestions();

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].name, "Test2");
    }

    #[test]
    fn test_confirm_candidate() {
        let mut learner = ToneLearner::new();
        let mut space = EmotionalSpace::new("eduardo_001".to_string());

        // Crear candidato
        let candidate = ToneCandidate::new(
            "Positivo".to_string(),
            ToneDimensions::new(0.8, 0.5, 0.5, 0.3),
            "context".to_string(),
            0.85,
        );
        learner
            .pending_candidates
            .insert("Positivo".to_string(), candidate);

        // Confirmar con nombre de usuario
        let cluster_id = learner
            .confirm_candidate("Positivo", "Emocionado".to_string(), &mut space)
            .unwrap();

        assert_eq!(space.cluster_count(), 1);
        assert!(learner.pending_candidates.is_empty());

        let cluster = space.get_cluster(&cluster_id).unwrap();
        assert_eq!(cluster.name, "Emocionado"); // Nombre de usuario
    }

    #[test]
    fn test_reject_candidate() {
        let mut learner = ToneLearner::new();

        let candidate = ToneCandidate::new(
            "Test".to_string(),
            ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
            "ctx".to_string(),
            0.5,
        );
        learner
            .pending_candidates
            .insert("Test".to_string(), candidate);

        assert_eq!(learner.pending_count(), 1);

        learner.reject_candidate("Test").unwrap();

        assert_eq!(learner.pending_count(), 0);
    }

    #[test]
    fn test_format_suggestion_message() {
        let learner = ToneLearner::new();
        let suggestion = ToneSuggestion {
            name: "Determinado".to_string(),
            dimensions: ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            description: "Tono asertivo con alta energía".to_string(),
            mention_count: 5,
            examples: vec![
                "Voy a terminarlo".to_string(),
                "Sin excusas".to_string(),
                "Definitivamente lo haré".to_string(),
            ],
        };

        let message = learner.format_suggestion_message(&suggestion);

        assert!(message.contains("nuevo tono"));
        assert!(message.contains("5 veces"));
        assert!(message.contains("Voy a terminarlo"));
    }
}
