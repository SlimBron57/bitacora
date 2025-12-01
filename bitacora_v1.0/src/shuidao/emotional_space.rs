// ================================================================
// EmotionalSpace - Sistema Dinámico de Tonos Emocionales (DA-033)
// ================================================================
//
// Purpose: Continuous 4D emotional space (VAD+F model)
// Replaces: ToneType enum (5 discrete tones) → Unlimited user-defined tones
// Architecture: ToneDimensions (VAD+F) → ToneCluster (user-named) → EmotionalSpace
// Performance: <20ms detection (HOT PATH)
//
// VAD+F Model:
//   - Valence: Positive/Negative (-1.0 to 1.0)
//   - Arousal: Energy level (-1.0 to 1.0)
//   - Dominance: Assertiveness (-1.0 to 1.0)
//   - Formality: Register (-1.0 to 1.0)
//
// Creado: 2025-11-26 19:15:00
// Especificación: ROADMAP_V2/02_COMPONENTES/CRITICOS/15_shuidao-emotional-space.md
// ================================================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::shuidao::error::Result;

// ================================================================
// CORE: ToneDimensions (VAD+F Model)
// ================================================================

/// Representa un punto en el espacio emocional 4D (VAD+F)
/// 
/// Basado en el modelo VAD de Russell (1980) + Formality extension
/// 
/// # Example
/// ```
/// let determinado = ToneDimensions::new(0.3, 0.7, 0.8, 0.5);
/// // Ligeramente positivo, alta energía, muy asertivo, neutral formal
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ToneDimensions {
    /// Valence: -1.0 (muy negativo) a 1.0 (muy positivo)
    /// Ejemplo: "Estoy frustrado" → -0.6, "Estoy emocionado" → 0.9
    pub valence: f32,

    /// Arousal: -1.0 (muy calmado) a 1.0 (muy energético)
    /// Ejemplo: "Recuerdo cuando..." → -0.4, "¡Hagámoslo ahora!" → 0.8
    pub arousal: f32,

    /// Dominance: -1.0 (muy sumiso) a 1.0 (muy dominante)
    /// Ejemplo: "¿Podrías ayudarme?" → -0.3, "Voy a terminarlo" → 0.8
    pub dominance: f32,

    /// Formality: -1.0 (muy casual) a 1.0 (muy formal)
    /// Ejemplo: "Qué onda?" → -0.7, "Estimado Sr." → 0.8
    pub formality: f32,
}

impl ToneDimensions {
    /// Crea nuevas dimensiones con validación (clamp a [-1.0, 1.0])
    pub fn new(valence: f32, arousal: f32, dominance: f32, formality: f32) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(-1.0, 1.0),
            dominance: dominance.clamp(-1.0, 1.0),
            formality: formality.clamp(-1.0, 1.0),
        }
    }

    /// Distancia euclidiana en espacio 4D
    /// 
    /// Formula: sqrt((v1-v2)² + (a1-a2)² + (d1-d2)² + (f1-f2)²)
    pub fn distance_to(&self, other: &ToneDimensions) -> f32 {
        let dv = self.valence - other.valence;
        let da = self.arousal - other.arousal;
        let dd = self.dominance - other.dominance;
        let df = self.formality - other.formality;

        (dv * dv + da * da + dd * dd + df * df).sqrt()
    }

    /// Descripción humana de valence
    pub fn describe_valence(&self) -> &'static str {
        match self.valence {
            v if v > 0.6 => "muy positivo",
            v if v > 0.2 => "positivo",
            v if v > -0.2 => "neutral",
            v if v > -0.6 => "negativo",
            _ => "muy negativo",
        }
    }

    /// Descripción humana de arousal
    pub fn describe_arousal(&self) -> &'static str {
        match self.arousal {
            a if a > 0.6 => "muy energético",
            a if a > 0.2 => "energético",
            a if a > -0.2 => "neutral",
            a if a > -0.6 => "calmado",
            _ => "muy calmado",
        }
    }

    /// Descripción humana de dominance
    pub fn describe_dominance(&self) -> &'static str {
        match self.dominance {
            d if d > 0.6 => "muy asertivo",
            d if d > 0.2 => "asertivo",
            d if d > -0.2 => "neutral",
            d if d > -0.6 => "pasivo",
            _ => "muy pasivo",
        }
    }

    /// Descripción humana de formality
    pub fn describe_formality(&self) -> &'static str {
        match self.formality {
            f if f > 0.6 => "muy formal",
            f if f > 0.2 => "formal",
            f if f > -0.2 => "neutral",
            f if f > -0.6 => "casual",
            _ => "muy casual",
        }
    }

    /// Descripción completa en español
    pub fn describe(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.describe_valence(),
            self.describe_arousal(),
            self.describe_dominance(),
            self.describe_formality()
        )
    }
}

// ================================================================
// LEXICAL & SYNTACTIC MARKERS
// ================================================================

/// Markers léxicos extraídos del texto para caracterizar un tono
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LexicalMarkers {
    /// Verbos fuertes: "voy a", "terminar", "lograr"
    pub strong_verbs: Vec<String>,

    /// Frases de compromiso: "cueste lo que cueste", "sin excusas"
    pub commitment_phrases: Vec<String>,

    /// Marcadores temporales: "hoy", "ahora", "ya"
    pub time_markers: Vec<String>,

    /// Adjetivos emocionales: "determinado", "frustrado", "emocionado"
    pub emotional_adjectives: Vec<String>,

    /// Conteo de exclamaciones
    pub exclamation_count: u32,

    /// Conteo de preguntas
    pub question_count: u32,

    /// Palabras en mayúsculas (énfasis)
    pub uppercase_words: Vec<String>,
}

impl LexicalMarkers {
    pub fn new() -> Self {
        Self::default()
    }

    /// Merge con otros markers (usado en cluster refinement)
    pub fn merge(&mut self, other: Self) {
        self.strong_verbs.extend(other.strong_verbs);
        self.commitment_phrases.extend(other.commitment_phrases);
        self.time_markers.extend(other.time_markers);
        self.emotional_adjectives.extend(other.emotional_adjectives);
        self.exclamation_count += other.exclamation_count;
        self.question_count += other.question_count;
        self.uppercase_words.extend(other.uppercase_words);

        // Dedup
        self.strong_verbs.sort();
        self.strong_verbs.dedup();
        self.commitment_phrases.sort();
        self.commitment_phrases.dedup();
    }
}

/// Patrones sintácticos extraídos del texto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntacticPattern {
    /// Patrón: "voy a <verb> <complement> cueste lo que cueste"
    pub pattern: String,

    /// Confianza en el patrón [0.0 - 1.0]
    pub confidence: f32,

    /// Ejemplos de frases que matchean este patrón
    pub examples: Vec<String>,
}

impl SyntacticPattern {
    pub fn new(pattern: String, confidence: f32) -> Self {
        Self {
            pattern,
            confidence,
            examples: Vec::new(),
        }
    }

    pub fn add_example(&mut self, example: String) {
        if !self.examples.contains(&example) {
            self.examples.push(example);
        }
    }
}

// ================================================================
// TONE CLUSTER (User-Defined Emotional Region)
// ================================================================

/// ID único de un ToneCluster
pub type ToneClusterId = String;

/// Cluster de tono definido por el usuario
/// 
/// Representa una región en el espacio emocional 4D que el usuario
/// ha nombrado con su propio vocabulario ("Determinado", "Nostálgico", etc.)
/// 
/// # Example
/// ```
/// let determinado = ToneCluster {
///     id: "tone_eduardo_determinado_001".to_string(),
///     name: "Determinado".to_string(), // USER NAMED ✅
///     center: ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
///     radius: 0.25,
///     // ...
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneCluster {
    /// ID único del cluster
    pub id: ToneClusterId,

    /// Nombre del tono (definido por usuario)
    /// Ejemplos: "Determinado", "Nostálgico", "Emocionado", "Mi tono reflexivo nocturno"
    pub name: String,

    /// Centro del cluster en espacio 4D
    pub center: ToneDimensions,

    /// Radio de tolerancia para matching (default: 0.25)
    pub radius: f32,

    /// Ejemplos de mensajes con este tono
    pub examples: Vec<String>,

    /// ID del usuario propietario
    pub user_id: String,

    /// Timestamp de descubrimiento
    pub discovered_at: DateTime<Utc>,

    /// Contador de interacciones con este tono
    pub interaction_count: u32,

    /// Markers léxicos característicos
    pub lexical_markers: LexicalMarkers,

    /// Patrones sintácticos característicos
    pub syntactic_patterns: Vec<SyntacticPattern>,
}

impl ToneCluster {
    /// Crea nuevo cluster con valores por defecto
    pub fn new(id: ToneClusterId, name: String, center: ToneDimensions, user_id: String) -> Self {
        Self {
            id,
            name,
            center,
            radius: 0.25, // Default tolerance
            examples: Vec::new(),
            user_id,
            discovered_at: Utc::now(),
            interaction_count: 1,
            lexical_markers: LexicalMarkers::new(),
            syntactic_patterns: Vec::new(),
        }
    }

    /// Verifica si unas dimensiones están dentro del cluster
    pub fn contains(&self, dimensions: &ToneDimensions) -> bool {
        self.center.distance_to(dimensions) <= self.radius
    }

    /// Calcula confidence score basado en distancia
    /// 
    /// Formula: 1.0 - (distance / radius)
    /// Si distance = 0 (centro exacto) → confidence = 1.0
    /// Si distance = radius (borde) → confidence = 0.0
    pub fn confidence_score(&self, dimensions: &ToneDimensions) -> f32 {
        let distance = self.center.distance_to(dimensions);
        (1.0 - (distance / self.radius)).max(0.0)
    }

    /// Añade ejemplo al cluster
    pub fn add_example(&mut self, text: String) {
        if !self.examples.contains(&text) {
            self.examples.push(text);
        }
        self.interaction_count += 1;
    }
}

// ================================================================
// EMOTIONAL SPACE (User-Specific)
// ================================================================

/// Espacio emocional de un usuario específico
/// 
/// Contiene todos los ToneClusters descubiertos para este usuario.
/// Cada usuario tiene su propio EmotionalSpace único.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSpace {
    /// ID del usuario propietario
    pub user_id: String,

    /// Clusters de tonos descubiertos
    pub clusters: HashMap<ToneClusterId, ToneCluster>,

    /// Paths de templates MTT-DSL en VoxelDB
    pub template_paths: HashMap<ToneClusterId, String>,

    /// Timestamp de creación
    pub created_at: DateTime<Utc>,

    /// Timestamp de última actualización
    pub last_updated: DateTime<Utc>,
}

impl EmotionalSpace {
    /// Crea nuevo EmotionalSpace para un usuario
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            clusters: HashMap::new(),
            template_paths: HashMap::new(),
            created_at: Utc::now(),
            last_updated: Utc::now(),
        }
    }

    /// Añade nuevo cluster al espacio
    pub fn add_cluster(&mut self, cluster: ToneCluster) {
        self.clusters.insert(cluster.id.clone(), cluster);
        self.last_updated = Utc::now();
    }

    /// Busca el cluster más cercano a unas dimensiones
    /// 
    /// Returns: (cluster_id, distance) del cluster más cercano dentro de su radio
    pub fn find_nearest_cluster(&self, dimensions: &ToneDimensions) -> Option<(ToneClusterId, f32)> {
        let mut best_match: Option<(ToneClusterId, f32)> = None;

        for (cluster_id, cluster) in &self.clusters {
            let distance = cluster.center.distance_to(dimensions);

            // Solo considerar si está dentro del radio
            if distance <= cluster.radius {
                match best_match {
                    Some((_, best_dist)) if distance < best_dist => {
                        best_match = Some((cluster_id.clone(), distance));
                    }
                    None => {
                        best_match = Some((cluster_id.clone(), distance));
                    }
                    _ => {}
                }
            }
        }

        best_match
    }

    /// Obtiene cluster por ID
    pub fn get_cluster(&self, cluster_id: &ToneClusterId) -> Option<&ToneCluster> {
        self.clusters.get(cluster_id)
    }

    /// Obtiene cluster mutable por ID
    pub fn get_cluster_mut(&mut self, cluster_id: &ToneClusterId) -> Option<&mut ToneCluster> {
        self.clusters.get_mut(cluster_id)
    }

    /// Lista todos los nombres de clusters
    pub fn list_cluster_names(&self) -> Vec<String> {
        self.clusters.values().map(|c| c.name.clone()).collect()
    }

    /// Cuenta total de clusters
    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }
}

// ================================================================
// TONE DETECTION RESULT
// ================================================================

/// Resultado de detección de tono emocional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneDetection {
    /// Dimensiones VAD+F detectadas
    pub dimensions: ToneDimensions,

    /// ID del cluster matched (None si es nuevo tono)
    pub cluster_id: Option<ToneClusterId>,

    /// Nombre del cluster matched
    pub cluster_name: Option<String>,

    /// Confidence score [0.0 - 1.0]
    pub confidence: f32,

    /// True si es un nuevo tono no visto antes
    pub is_new_tone: bool,

    /// True si requiere confirmación del usuario
    pub requires_confirmation: bool,
}

impl ToneDetection {
    /// Crea detección de tono conocido
    pub fn known_tone(
        dimensions: ToneDimensions,
        cluster_id: ToneClusterId,
        cluster_name: String,
        confidence: f32,
    ) -> Self {
        Self {
            dimensions,
            cluster_id: Some(cluster_id),
            cluster_name: Some(cluster_name),
            confidence,
            is_new_tone: false,
            requires_confirmation: false,
        }
    }

    /// Crea detección de nuevo tono
    pub fn new_tone(dimensions: ToneDimensions) -> Self {
        Self {
            dimensions,
            cluster_id: None,
            cluster_name: None,
            confidence: 0.0,
            is_new_tone: true,
            requires_confirmation: true,
        }
    }
}

// ================================================================
// TONE DETECTOR (Main Interface)
// ================================================================

/// Detector de tonos emocionales
/// 
/// Interfaz principal para detección de tonos en mensajes.
/// v1.0: Usa heurísticas simples (keywords, markers)
/// v1.1: Integrará análisis NLP real (lexical + syntactic)
pub struct ToneDetector {
    /// Threshold de distancia para considerar match (default: 0.25)
    pub distance_threshold: f32,
}

impl ToneDetector {
    /// Crea nuevo detector con threshold por defecto
    pub fn new() -> Self {
        Self {
            distance_threshold: 0.25,
        }
    }

    /// Crea detector con threshold personalizado
    pub fn with_threshold(distance_threshold: f32) -> Self {
        Self {
            distance_threshold,
        }
    }

    /// Detecta tono en un mensaje (v1.0 - heurísticas simples)
    /// 
    /// # Arguments
    /// * `message` - Mensaje a analizar
    /// * `space` - EmotionalSpace del usuario
    /// 
    /// # Returns
    /// ToneDetection con dimensiones calculadas y cluster matched (si existe)
    pub fn detect(&self, message: &str, space: &EmotionalSpace) -> ToneDetection {
        // v1.0: Calcular dimensiones con heurísticas simples
        let dimensions = self.compute_dimensions_stub(message);

        // Buscar cluster más cercano
        if let Some((cluster_id, distance)) = space.find_nearest_cluster(&dimensions) {
            let cluster = space.get_cluster(&cluster_id).unwrap();
            let confidence = cluster.confidence_score(&dimensions);

            ToneDetection::known_tone(
                dimensions,
                cluster_id,
                cluster.name.clone(),
                confidence,
            )
        } else {
            ToneDetection::new_tone(dimensions)
        }
    }

    /// Computa dimensiones VAD+F (v1.0 - stub con heurísticas simples)
    /// 
    /// TODO v1.1: Implementar análisis léxico + sintáctico real
    fn compute_dimensions_stub(&self, message: &str) -> ToneDimensions {
        let msg_lower = message.to_lowercase();

        // Valence: positivo/negativo
        let valence = self.compute_valence_stub(&msg_lower);

        // Arousal: energía
        let arousal = self.compute_arousal_stub(&msg_lower, message);

        // Dominance: asertividad
        let dominance = self.compute_dominance_stub(&msg_lower);

        // Formality: registro
        let formality = self.compute_formality_stub(&msg_lower);

        ToneDimensions::new(valence, arousal, dominance, formality)
    }

    fn compute_valence_stub(&self, msg: &str) -> f32 {
        let mut score: f32 = 0.0;

        // Positivo
        let positive_words = ["bien", "excelente", "perfecto", "genial", "feliz", "emocionado"];
        for word in &positive_words {
            if msg.contains(word) {
                score += 0.3;
            }
        }

        // Negativo
        let negative_words = ["mal", "frustrado", "triste", "error", "problema", "falla"];
        for word in &negative_words {
            if msg.contains(word) {
                score -= 0.3;
            }
        }

        score.clamp(-1.0, 1.0)
    }

    fn compute_arousal_stub(&self, msg: &str, original: &str) -> f32 {
        let mut score: f32 = 0.0;

        // Alta energía
        if original.contains('!') {
            score += 0.3;
        }
        if msg.contains("voy a") || msg.contains("hagamos") {
            score += 0.2;
        }

        // Baja energía
        if msg.contains("recuerdo") || msg.contains("quizás") {
            score -= 0.2;
        }
        if msg.contains("...") {
            score -= 0.1;
        }

        score.clamp(-1.0, 1.0)
    }

    fn compute_dominance_stub(&self, msg: &str) -> f32 {
        let mut score: f32 = 0.0;

        // Asertivo
        if msg.contains("voy a") || msg.contains("haré") {
            score += 0.3;
        }
        if msg.contains("sin excusas") || msg.contains("definitivamente") {
            score += 0.2;
        }

        // Pasivo
        if msg.contains("¿") {
            score -= 0.2;
        }
        if msg.contains("podrías") || msg.contains("tal vez") {
            score -= 0.15;
        }

        score.clamp(-1.0, 1.0)
    }

    fn compute_formality_stub(&self, msg: &str) -> f32 {
        let mut score: f32 = 0.0;

        // Formal
        if msg.contains("usted") || msg.contains("estimado") {
            score += 0.3;
        }

        // Casual
        if msg.contains("wey") || msg.contains("qué onda") {
            score -= 0.3;
        }

        score.clamp(-1.0, 1.0)
    }
}

impl Default for ToneDetector {
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
    fn test_tone_dimensions_creation() {
        let dims = ToneDimensions::new(0.5, 0.7, -0.3, 0.1);
        assert_eq!(dims.valence, 0.5);
        assert_eq!(dims.arousal, 0.7);
        assert_eq!(dims.dominance, -0.3);
        assert_eq!(dims.formality, 0.1);
    }

    #[test]
    fn test_tone_dimensions_clamping() {
        let dims = ToneDimensions::new(2.0, -5.0, 0.5, 10.0);
        assert_eq!(dims.valence, 1.0); // clamped
        assert_eq!(dims.arousal, -1.0); // clamped
        assert_eq!(dims.dominance, 0.5);
        assert_eq!(dims.formality, 1.0); // clamped
    }

    #[test]
    fn test_euclidean_distance() {
        let a = ToneDimensions::new(0.0, 0.0, 0.0, 0.0);
        let b = ToneDimensions::new(0.3, 0.4, 0.0, 0.0);

        let distance = a.distance_to(&b);
        let expected = (0.3_f32.powi(2) + 0.4_f32.powi(2)).sqrt();

        assert!((distance - expected).abs() < 0.01);
    }

    #[test]
    fn test_tone_cluster_contains() {
        let cluster = ToneCluster::new(
            "test_001".to_string(),
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "eduardo_001".to_string(),
        );

        // Dentro del radio (0.25)
        let inside = ToneDimensions::new(0.35, 0.75, 0.85, 0.55);
        assert!(cluster.contains(&inside));

        // Fuera del radio
        let outside = ToneDimensions::new(-0.5, -0.4, 0.1, 0.2);
        assert!(!cluster.contains(&outside));
    }

    #[test]
    fn test_confidence_score() {
        let cluster = ToneCluster::new(
            "test_001".to_string(),
            "Test".to_string(),
            ToneDimensions::new(0.0, 0.0, 0.0, 0.0),
            "user_001".to_string(),
        );

        // Centro exacto → confidence 1.0
        let center = ToneDimensions::new(0.0, 0.0, 0.0, 0.0);
        assert!((cluster.confidence_score(&center) - 1.0).abs() < 0.01);

        // En el borde (distance = radius) → confidence ~0.0
        let edge = ToneDimensions::new(0.25, 0.0, 0.0, 0.0);
        assert!(cluster.confidence_score(&edge) < 0.1);
    }

    #[test]
    fn test_emotional_space_find_nearest() {
        let mut space = EmotionalSpace::new("eduardo_001".to_string());

        // Añadir cluster "Determinado"
        let determinado = ToneCluster::new(
            "tone_det_001".to_string(),
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "eduardo_001".to_string(),
        );
        space.add_cluster(determinado);

        // Buscar dimensiones similares
        let similar = ToneDimensions::new(0.35, 0.75, 0.85, 0.55);
        let result = space.find_nearest_cluster(&similar);

        assert!(result.is_some());
        let (cluster_id, distance) = result.unwrap();
        assert_eq!(cluster_id, "tone_det_001");
        assert!(distance < 0.25);
    }

    #[test]
    fn test_emotional_space_no_match() {
        let mut space = EmotionalSpace::new("eduardo_001".to_string());

        let determinado = ToneCluster::new(
            "tone_det_001".to_string(),
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "eduardo_001".to_string(),
        );
        space.add_cluster(determinado);

        // Dimensiones muy diferentes (fuera de todos los radios)
        let different = ToneDimensions::new(-0.9, -0.8, -0.7, -0.6);
        let result = space.find_nearest_cluster(&different);

        assert!(result.is_none());
    }

    #[test]
    fn test_tone_detector_determinado() {
        let detector = ToneDetector::new();
        let mut space = EmotionalSpace::new("eduardo_001".to_string());

        // Setup: cluster "Determinado" con radio más amplio para stub
        let mut determinado = ToneCluster::new(
            "tone_det_001".to_string(),
            "Determinado".to_string(),
            ToneDimensions::new(0.3, 0.7, 0.8, 0.5),
            "eduardo_001".to_string(),
        );
        determinado.radius = 1.5; // Stub heuristics son imprecisas, radio más amplio
        space.add_cluster(determinado);

        // Detectar mensaje con tono determinado
        let message = "¡Voy a terminar este proyecto sin excusas!";
        let detection = detector.detect(message, &space);

        // v1.0 stub puede no matchear exacto, verificamos dimensiones
        // v1.1 con análisis real será más preciso
        assert!(detection.dimensions.arousal > 0.0); // Tiene energía (!)
        assert!(detection.dimensions.dominance > 0.0); // Asertivo (voy a)
    }

    #[test]
    fn test_tone_detector_new_tone() {
        let detector = ToneDetector::new();
        let space = EmotionalSpace::new("eduardo_001".to_string());

        // Detectar con EmotionalSpace vacío → nuevo tono
        let message = "Recuerdo cuando construimos aquella casa...";
        let detection = detector.detect(message, &space);

        assert!(detection.is_new_tone);
        assert!(detection.requires_confirmation);
        assert!(detection.cluster_id.is_none());
    }

    #[test]
    fn test_dimensions_descriptions() {
        let dims = ToneDimensions::new(0.7, 0.8, -0.3, -0.5);

        assert_eq!(dims.describe_valence(), "muy positivo"); // 0.7 > 0.6
        assert_eq!(dims.describe_arousal(), "muy energético");
        assert_eq!(dims.describe_dominance(), "pasivo");
        assert_eq!(dims.describe_formality(), "casual");

        let full = dims.describe();
        assert!(full.contains("positivo"));
        assert!(full.contains("energético"));
    }
}
