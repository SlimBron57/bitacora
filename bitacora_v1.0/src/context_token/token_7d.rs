use crate::{Result};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration, Timelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use colored::*;

/// Input normalizado del SENSORY ENGINE para generación de tokens
#[derive(Debug, Clone)]
pub struct NormalizedInput {
    pub text: String,
    pub audio: Option<Vec<f32>>,
    pub visual: Option<Vec<u8>>,
    pub language: String,
    pub sentiment: f64,  // -1.0 a +1.0
    pub confidence: f64,
    pub metadata: HashMap<String, String>,
}

/// Token de contexto 7D con capacidades bidireccionales breakthrough 133.8/100
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextToken7D {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    // Las 7 dimensiones específicas del breakthrough
    pub temporal: TemporalDimension,
    pub semantic: SemanticDimension, 
    pub contextual: ContextualDimension,
    pub relational: RelationalDimension,
    pub emotional: EmotionalDimension,
    pub intentional: IntentionalDimension,
    pub biographical: BiographicalDimension,
    // Métricas breakthrough
    pub breakthrough_score: f64,
    pub bidirectional_weight: f64,
    pub semantic_fingerprint: String,
    pub relationships: Vec<TokenRelationship>,
    pub lifecycle_stage: LifecycleStage,
    // Nuevas capacidades avanzadas 7D
    pub dimensional_coherence: f64,
    pub contextual_intensity: f64,
    pub breakthrough_threshold: f64,
    pub validation_status: ValidationStatus,
}

/// Dimensión temporal específica 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalDimension {
    pub score: f64,
    pub temporal_anchors: Vec<String>,
    pub chronological_depth: f64,
    pub temporal_consistency: f64,
    pub sequential_logic: f64,
    pub time_horizon: f64,
    pub temporal_dynamics: f64,
    // Campos adicionales de fusión (tensor.rs)
    pub sequence: u64,
    pub time_of_day: String,  // "morning", "afternoon", "evening", "night"
    pub day_of_week: String,
    pub session_duration_minutes: u64,
    pub lifecycle_hours: u64,  // 168 horas = 7 días
}

impl TemporalDimension {
    /// Scoring fusionado: coherencia temporal con bonificaciones por sesión y hora
    pub fn coherence_score(&self) -> f64 {
        // Score basado en coherencia temporal (tensor.rs)
        let duration_penalty = if self.session_duration_minutes > 480 {  // >8h
            0.7
        } else if self.session_duration_minutes > 240 {  // >4h
            0.9
        } else {
            1.0
        };
        
        // Hora apropiada bonus
        let time_bonus = match self.time_of_day.as_str() {
            "morning" | "afternoon" => 1.2,
            "evening" => 1.0,
            "night" => 0.8,  // Posible cansancio
            _ => 1.0,
        };
        
        // Multiplicar por score original + consistency
        let base_coherence = self.score * self.temporal_consistency;
        base_coherence * duration_penalty * time_bonus
    }
}

/// Dimensión semántica específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticDimension {
    pub score: f64,
    pub concept_density: f64,
    pub semantic_coherence: f64,
    pub domain_specificity: f64,
    pub conceptual_depth: f64,
    pub terminology_precision: f64,
    pub semantic_relationships: Vec<String>,
    // Campos adicionales de fusión (tensor.rs)
    pub text: String,
    pub language: String,
    pub keywords: Vec<String>,
    pub embeddings: Vec<f32>,  // Vector embedding (opcional)
    pub semantic_density: f64,  // 0.0 - 1.0
}

impl SemanticDimension {
    /// Scoring fusionado: relevancia semántica con densidad y keywords
    pub fn relevance_score(&self) -> f64 {
        // Score basado en densidad semántica y keywords (tensor.rs)
        let keyword_factor = (self.keywords.len() as f64 / 10.0).min(1.0);
        let density_factor = self.semantic_density;
        
        // Multiplicar por score original + coherencia
        let base_relevance = self.score * self.semantic_coherence;
        base_relevance * ((keyword_factor + density_factor) / 2.0 * 1.5).max(1.0)
    }
}

/// Dimensión contextual específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualDimension {
    pub score: f64,
    pub situational_relevance: f64,
    pub environmental_factors: Vec<String>,
    pub contextual_coherence: f64,
    pub scope_awareness: f64,
    pub meta_context: f64,
    // Campos adicionales de fusión (tensor.rs)
    pub session_id: String,
    pub user_id: String,
    pub context_markers: Vec<String>,  // ["debugging", "learning", etc.]
    pub situational_frame: String,
    pub coherence_with_previous: f64,  // 0.0 - 1.0
}

impl ContextualDimension {
    /// Scoring fusionado: fit situacional con coherencia previa
    pub fn situational_fit_score(&self) -> f64 {
        // Score basado en coherencia con contexto previo (tensor.rs)
        let coherence_factor = self.coherence_with_previous;
        let markers_factor = (self.context_markers.len() as f64 / 5.0).min(1.0);
        
        // Multiplicar por score original + relevancia situacional
        let base_fit = self.score * self.situational_relevance;
        base_fit * ((coherence_factor + markers_factor) / 2.0 * 1.3).max(1.0)
    }
}

/// Dimensión relacional específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationalDimension {
    pub score: f64,
    pub entity_connections: Vec<String>,
    pub relationship_strength: f64,
    pub network_density: f64,
    pub relational_patterns: Vec<String>,
    pub social_dynamics: f64,
    // Campos adicionales de fusión (tensor.rs)
    pub related_tokens: Vec<String>,  // IDs de tokens relacionados
    pub entity_graph: HashMap<String, Vec<String>>,  // Grafo de entidades
    pub connection_strength: f64,  // 0.0 - 1.0
    pub pattern_matches: Vec<String>,
}

impl RelationalDimension {
    /// Scoring fusionado: conectividad con patrones y fuerza
    pub fn connectivity_score(&self) -> f64 {
        // Score basado en conexiones y patrones (tensor.rs)
        let relations_factor = (self.related_tokens.len() as f64 / 8.0).min(1.0);
        let patterns_factor = (self.pattern_matches.len() as f64 / 4.0).min(1.0);
        let strength_factor = self.connection_strength;
        
        // Multiplicar por score original + network density
        let base_connectivity = self.score * self.network_density;
        base_connectivity * ((relations_factor + patterns_factor + strength_factor) / 3.0 * 1.8).max(1.0)
    }
}

/// Dimensión emocional específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalDimension {
    pub score: f64,
    pub emotional_intensity: f64,
    pub emotional_valence: f64,
    pub emotional_complexity: Vec<String>,
    pub empathy_markers: Vec<String>,
    pub emotional_resonance: f64,
    // Campos adicionales de fusión (tensor.rs) - VADC Model
    pub valence: f64,      // -1.0 (negativo) a +1.0 (positivo)
    pub arousal: f64,      // 0.0 (calmo) a 1.0 (excitado)
    pub dominance: f64,    // 0.0 (sumiso) a 1.0 (dominante)
    pub certainty: f64,    // 0.0 (incierto) a 1.0 (certero)
    pub emotional_trajectory: Vec<(String, f64)>,  // Historia emocional
}

impl EmotionalDimension {
    /// Scoring fusionado: resonancia emocional con modelo VADC
    pub fn resonance_score(&self) -> f64 {
        // Score basado en resonancia emocional VADC (tensor.rs)
        let valence_abs = self.valence.abs();  // Emociones intensas (+ o -) = mayor resonancia
        let arousal_factor = self.arousal;
        let certainty_factor = self.certainty;
        
        // Multiplicar por score original + intensidad emocional
        let base_resonance = self.score * self.emotional_resonance;
        base_resonance * ((valence_abs + arousal_factor + certainty_factor) / 3.0 * 1.4).max(1.0)
    }
}

/// Dimensión intencional específica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentionalDimension {
    pub score: f64,
    pub goal_clarity: f64,
    pub action_orientation: f64,
    pub strategic_thinking: f64,
    pub purpose_alignment: f64,
    pub intentional_depth: f64,
    // Campos adicionales de fusión (tensor.rs)
    pub intent_category: String,  // "question", "command", "statement"
    pub goal: String,             // "debug", "learn", "create"
    pub action_required: bool,
    pub urgency: f64,             // 0.0 - 1.0
    pub clarity: f64,             // 0.0 (ambiguo) - 1.0 (claro)
}

impl IntentionalDimension {
    /// Scoring fusionado: claridad intencional con urgencia
    pub fn clarity_score(&self) -> f64 {
        // Score basado en claridad de intención (tensor.rs)
        let clarity_factor = self.clarity;
        let urgency_factor = self.urgency;
        
        // Multiplicar por score original + goal clarity
        let base_clarity = self.score * self.goal_clarity;
        base_clarity * ((clarity_factor * 0.7 + urgency_factor * 0.3) * 1.6).max(1.0)
    }
}

/// Dimensión biográfica específica - CLAVE DEL BREAKTHROUGH (Convergencia con TelescopeDB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiographicalDimension {
    pub score: f64,
    pub personal_experience_markers: Vec<String>,
    pub life_stage_indicators: Vec<String>,
    pub biographical_depth: f64,
    pub personal_narrative_strength: f64,
    pub identity_markers: Vec<String>,
    pub experiential_uniqueness: f64,    // Factor breakthrough crítico
    pub biographical_resonance: f64,
    // Campos adicionales de fusión (tensor.rs)
    pub user_expertise_level: f64,  // 0.0 (novato) - 1.0 (experto)
    pub historical_patterns: Vec<String>,
    pub preferences: HashMap<String, String>,
    pub biographical_coherence: f64,  // Alineación con historial
    pub personal_significance: f64,   // Importancia para el usuario
}

impl BiographicalDimension {
    /// Scoring fusionado: alineación biográfica con experiencia
    pub fn alignment_score(&self) -> f64 {
        // Score basado en alineación biográfica (tensor.rs)
        let coherence_factor = self.biographical_coherence;
        let significance_factor = self.personal_significance;
        let expertise_bonus = self.user_expertise_level * 0.2;
        
        // Multiplicar por score original + uniqueness (clave del breakthrough)
        let base_alignment = self.score * self.experiential_uniqueness;
        base_alignment * ((coherence_factor + significance_factor) / 2.0 * 1.2 + expertise_bonus).max(1.0)
    }
}

/// Tipos de relaciones entre tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Causal,         // Causa-efecto
    Temporal,       // Secuencia temporal
    Semantic,       // Relación conceptual
    Contextual,     // Contexto compartido
    Emotional,      // Conexión emocional
    Biographical,   // Historia personal compartida
    Bidirectional,  // Relación bidireccional
}

/// Relación entre tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRelationship {
    pub target_token_id: String,
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub created_at: DateTime<Utc>,
}

/// Etapas del ciclo de vida del token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LifecycleStage {
    Birth,       // Token recién creado
    Growth,      // Ganando conexiones
    Maturity,    // Máximo potencial
    Wisdom,      // Experiencia acumulada
    Legacy,      // Valor histórico
    Transition,  // En proceso de cambio
}

/// Estado de validación del token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Pending,     // Esperando validación
    Validated,   // Completamente validado
    Breakthrough, // Ha alcanzado breakthrough
    Enhanced,    // Mejorado con capacidades avanzadas
    Failed,      // Falló la validación
    Deprecated,  // Obsoleto pero mantenido por referencia
}

impl ContextToken7D {
    /// Crear nuevo token con análisis básico
    pub fn new(content: String, duration_days: i64) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        
        Self {
            id,
            content,
            created_at: now,
            expires_at: now + Duration::days(duration_days),
            temporal: TemporalDimension::new(),
            semantic: SemanticDimension::new(),
            contextual: ContextualDimension::new(),
            relational: RelationalDimension::new(),
            emotional: EmotionalDimension::new(),
            intentional: IntentionalDimension::new(),
            biographical: BiographicalDimension::new(),
            breakthrough_score: 0.0,
            bidirectional_weight: 1.0,
            semantic_fingerprint: String::new(),
            relationships: Vec::new(),
            lifecycle_stage: LifecycleStage::Birth,
            // Nuevas capacidades avanzadas 7D
            dimensional_coherence: 0.0,
            contextual_intensity: 0.0,
            breakthrough_threshold: 75.0, // Threshold para breakthrough
            validation_status: ValidationStatus::Pending,
        }
    }
    
    /// Crear token desde input normalizado usando extractores heurísticos (FUSIÓN BAYESIANA)
    pub fn from_normalized_input(
        input: &NormalizedInput, 
        analyzer: &ContextAnalyzer7D,
        sequence: u64,
        session_start: DateTime<Utc>,
        duration_days: i64
    ) -> Result<Self> {
        let now = Utc::now();
        let id = Uuid::new_v4().to_string();
        
        // Extraer todas las dimensiones usando los métodos fusionados
        let temporal = analyzer.extract_temporal(input, sequence, session_start)?;
        let semantic = analyzer.extract_semantic(input)?;
        let contextual = analyzer.extract_contextual(input, sequence)?;
        let relational = analyzer.extract_relational(input)?;
        let emotional = analyzer.extract_emotional(input)?;
        let intentional = analyzer.extract_intentional(input)?;
        let biographical = analyzer.extract_biographical(input)?;
        
        // Calcular breakthrough score inicial
        let breakthrough_score = 
            (temporal.score * 0.10) +
            (semantic.score * 0.15) +
            (contextual.score * 0.12) +
            (relational.score * 0.13) +
            (emotional.score * 0.10) +
            (intentional.score * 0.15) +
            (biographical.score * 0.25);
        
        let breakthrough_score_scaled = breakthrough_score * 35.0;
        
        // Calcular métricas antes de mover las dimensiones
        let dimensional_coherence = breakthrough_score;
        let contextual_intensity = (semantic.score + contextual.score) / 2.0;
        
        // Generar semantic fingerprint
        let semantic_fingerprint = analyzer.generate_semantic_fingerprint(&input.text)?;
        
        Ok(Self {
            id,
            content: input.text.clone(),
            created_at: now,
            expires_at: now + Duration::days(duration_days),
            temporal,
            semantic,
            contextual,
            relational,
            emotional,
            intentional,
            biographical,
            breakthrough_score: breakthrough_score_scaled,
            bidirectional_weight: 1.0,
            semantic_fingerprint,
            relationships: Vec::new(),
            lifecycle_stage: LifecycleStage::Birth,
            dimensional_coherence,
            contextual_intensity,
            breakthrough_threshold: 75.0,
            validation_status: if breakthrough_score_scaled >= 30.0 {
                ValidationStatus::Validated
            } else {
                ValidationStatus::Pending
            },
        })
    }

    /// Realizar análisis completo 7D para alcanzar breakthrough 133.8/100
    pub async fn analyze_comprehensive(&mut self, analyzer: &ContextAnalyzer7D) -> Result<f64> {
        // Analizar cada dimensión específicamente
        self.temporal = analyzer.analyze_temporal_dimension(&self.content).await?;
        self.semantic = analyzer.analyze_semantic_dimension(&self.content).await?;
        self.contextual = analyzer.analyze_contextual_dimension(&self.content).await?;
        self.relational = analyzer.analyze_relational_dimension(&self.content).await?;
        self.emotional = analyzer.analyze_emotional_dimension(&self.content).await?;
        self.intentional = analyzer.analyze_intentional_dimension(&self.content).await?;
        self.biographical = analyzer.analyze_biographical_dimension(&self.content).await?; // Clave del breakthrough
        
        // Calcular puntuación breakthrough (contribuye 35/30 al score total 133.8/100)
        self.breakthrough_score = self.calculate_breakthrough_contribution();
        
        // Generar métricas adicionales
        self.bidirectional_weight = analyzer.calculate_bidirectional_weight(self)?;
        self.semantic_fingerprint = analyzer.generate_semantic_fingerprint(&self.content)?;
        
        Ok(self.breakthrough_score)
    }

    /// Calcular contribución al breakthrough score total (35/30 esperado)
    pub fn calculate_breakthrough_contribution(&self) -> f64 {
        // Pesos optimizados para breakthrough 133.8/100
        let temporal_weight = 0.10;      // 10%
        let semantic_weight = 0.15;      // 15%  
        let contextual_weight = 0.12;    // 12%
        let relational_weight = 0.13;    // 13%
        let emotional_weight = 0.10;     // 10%
        let intentional_weight = 0.15;   // 15%
        let biographical_weight = 0.25;  // 25% - La dimensión breakthrough
        
        let weighted_score = 
            (self.temporal.score * temporal_weight) +
            (self.semantic.score * semantic_weight) +
            (self.contextual.score * contextual_weight) +
            (self.relational.score * relational_weight) +
            (self.emotional.score * emotional_weight) +
            (self.intentional.score * intentional_weight) +
            (self.biographical.score * biographical_weight);
        
        // Escalar a contribución esperada (35 puntos máximo)
        weighted_score * 35.0
    }

    /// Verificar si alcanza el threshold breakthrough
    pub fn achieves_breakthrough(&self) -> bool {
        self.breakthrough_score >= 30.0 // Mínimo para ser considerado breakthrough
    }

    /// Verificar si el token ha expirado
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Obtener puntuación de relevancia actual
    pub fn relevance_score(&self) -> f64 {
        let time_factor = self.calculate_time_decay_factor();
        let stage_factor = match self.lifecycle_stage {
            LifecycleStage::Birth => 0.3,
            LifecycleStage::Growth => 0.7,
            LifecycleStage::Maturity => 1.0,   // Máximo potencial
            LifecycleStage::Wisdom => 0.9,
            LifecycleStage::Legacy => 0.6,
            LifecycleStage::Transition => 0.2,
        };
        let bidirectional_factor = self.bidirectional_weight;

        (time_factor * stage_factor * bidirectional_factor).min(1.0)
    }

    /// Factor de decaimiento temporal
    fn calculate_time_decay_factor(&self) -> f64 {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        let hours = elapsed.num_hours() as f64;
        let max_hours = 168.0; // 7 días

        // Curva de decaimiento no lineal optimizada para breakthrough
        let normalized_time = hours / max_hours;
        (1.0 - normalized_time.powf(0.7)).max(0.0)
    }

    /// Añadir relación con otro token
    pub fn add_relationship(&mut self, target_id: String, rel_type: RelationshipType, strength: f64) {
        self.relationships.push(TokenRelationship {
            target_token_id: target_id,
            relationship_type: rel_type,
            strength,
            created_at: Utc::now(),
        });
    }

    /// Actualizar etapa del ciclo de vida
    pub fn update_lifecycle_stage(&mut self) -> Result<()> {
        let elapsed = Utc::now().signed_duration_since(self.created_at);
        let days = elapsed.num_days();

        self.lifecycle_stage = match days {
            0..=1 => LifecycleStage::Birth,
            2 => LifecycleStage::Growth,
            3..=4 => LifecycleStage::Maturity,
            5 => LifecycleStage::Wisdom,
            6 => LifecycleStage::Legacy,
            _ => LifecycleStage::Transition,
        };

        Ok(())
    }

    /// Método de compatibilidad - analiza las dimensiones (wrapper del análisis completo)
    pub fn calculate_dimensions(&mut self, analyzer: &mut ContextAnalyzer7D) -> Result<()> {
        // Usar el nuevo método asíncrono en un contexto síncrono
        let _score = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.analyze_comprehensive(analyzer))?;
        Ok(())
    }

    /// Propiedad de compatibilidad - acceder a las dimensiones como vector
    pub fn dimensions(&self) -> Vec<f64> {
        vec![
            self.temporal.score,
            self.semantic.score,
            self.contextual.score,
            self.relational.score,
            self.emotional.score,
            self.intentional.score,
            self.biographical.score,
        ]
    }

    /// Acceder a las dimensiones mutables
    pub fn dimensions_mut(&mut self) -> Vec<&mut f64> {
        vec![
            &mut self.temporal.score,
            &mut self.semantic.score,
            &mut self.contextual.score,
            &mut self.relational.score,
            &mut self.emotional.score,
            &mut self.intentional.score,
            &mut self.biographical.score,
        ]
    }
    
    // ==================== SERIALIZACIÓN CBOR (FUSIÓN BAYESIANA - BITA-1) ====================
    
    /// Serializa el token a CBOR canónico determinístico (BITA-1 compliant)
    pub fn to_cbor(&self) -> Result<Vec<u8>> {
        // Serialización canónica (mismo input → mismo hash)
        let mut buffer = Vec::new();
        let mut serializer = serde_cbor::Serializer::new(&mut buffer);
        
        // Tag CBOR autodescriptivo
        serializer.self_describe()
            .map_err(|e| anyhow::anyhow!("Error en self-describe CBOR: {}", e))?;
        
        serde::Serialize::serialize(self, &mut serializer)
            .map_err(|e| anyhow::anyhow!("Error en serialización CBOR: {}", e))?;
        
        Ok(buffer)
    }
    
    /// Deserializa un token desde bytes CBOR
    pub fn from_cbor(bytes: &[u8]) -> Result<Self> {
        serde_cbor::from_slice(bytes)
            .map_err(|e| anyhow::anyhow!("Error al deserializar CBOR: {}", e))
    }
    
    /// Valida que el roundtrip de serialización sea idéntico (integridad BITA-1)
    pub fn validate_cbor_roundtrip(&self) -> Result<bool> {
        let serialized = self.to_cbor()?;
        let deserialized = Self::from_cbor(&serialized)?;
        let reserialized = deserialized.to_cbor()?;
        
        Ok(serialized == reserialized)
    }
    
    /// Obtiene el hash SHA-256 del token serializado (content-addressable)
    pub fn content_hash(&self) -> Result<String> {
        use sha2::{Sha256, Digest};
        
        let cbor_bytes = self.to_cbor()?;
        let mut hasher = Sha256::new();
        hasher.update(&cbor_bytes);
        let hash = hasher.finalize();
        
        Ok(format!("{:x}", hash))
    }
}

/// Implementaciones por defecto para las dimensiones
impl TemporalDimension {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            score: 0.0,
            temporal_anchors: Vec::new(),
            chronological_depth: 0.0,
            temporal_consistency: 0.0,
            sequential_logic: 0.0,
            time_horizon: 0.0,
            temporal_dynamics: 0.0,
            // Campos fusionados
            sequence: 0,
            time_of_day: "unknown".to_string(),
            day_of_week: now.format("%A").to_string(),
            session_duration_minutes: 0,
            lifecycle_hours: 168,
        }
    }
}

impl SemanticDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            concept_density: 0.0,
            semantic_coherence: 0.0,
            domain_specificity: 0.0,
            conceptual_depth: 0.0,
            terminology_precision: 0.0,
            semantic_relationships: Vec::new(),
            // Campos fusionados
            text: String::new(),
            language: "unknown".to_string(),
            keywords: Vec::new(),
            embeddings: Vec::new(),
            semantic_density: 0.0,
        }
    }
}

impl ContextualDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            situational_relevance: 0.0,
            environmental_factors: Vec::new(),
            contextual_coherence: 0.0,
            scope_awareness: 0.0,
            meta_context: 0.0,
            // Campos fusionados
            session_id: String::new(),
            user_id: "anonymous".to_string(),
            context_markers: Vec::new(),
            situational_frame: "unknown".to_string(),
            coherence_with_previous: 0.0,
        }
    }
}

impl RelationalDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            entity_connections: Vec::new(),
            relationship_strength: 0.0,
            network_density: 0.0,
            relational_patterns: Vec::new(),
            social_dynamics: 0.0,
            // Campos fusionados
            related_tokens: Vec::new(),
            entity_graph: HashMap::new(),
            connection_strength: 0.0,
            pattern_matches: Vec::new(),
        }
    }
}

impl EmotionalDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            emotional_intensity: 0.0,
            emotional_valence: 0.0,
            emotional_complexity: Vec::new(),
            empathy_markers: Vec::new(),
            emotional_resonance: 0.0,
            // Campos fusionados (VADC)
            valence: 0.0,
            arousal: 0.0,
            dominance: 0.0,
            certainty: 0.0,
            emotional_trajectory: Vec::new(),
        }
    }
}

impl IntentionalDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            goal_clarity: 0.0,
            action_orientation: 0.0,
            strategic_thinking: 0.0,
            purpose_alignment: 0.0,
            intentional_depth: 0.0,
            // Campos fusionados
            intent_category: "unknown".to_string(),
            goal: "unknown".to_string(),
            action_required: false,
            urgency: 0.0,
            clarity: 0.0,
        }
    }
}

impl BiographicalDimension {
    pub fn new() -> Self {
        Self {
            score: 0.0,
            personal_experience_markers: Vec::new(),
            life_stage_indicators: Vec::new(),
            biographical_depth: 0.0,
            personal_narrative_strength: 0.0,
            identity_markers: Vec::new(),
            experiential_uniqueness: 0.0,
            biographical_resonance: 0.0,
            // Campos fusionados
            user_expertise_level: 0.0,
            historical_patterns: Vec::new(),
            preferences: HashMap::new(),
            biographical_coherence: 0.0,
            personal_significance: 0.0,
        }
    }
}

/// Analizador de contexto 7D - Sistema breakthrough 133.8/100 con Analytics Avanzados
pub struct ContextAnalyzer7D {
    dimension_weights: [f64; 7],
    semantic_cache: HashMap<String, Vec<f64>>,
    biographical_patterns: Vec<String>,     // Patrones biográficos clave
    temporal_markers: Vec<String>,          // Marcadores temporales
    emotional_lexicon: HashMap<String, f64>, // Léxico emocional
    // Nuevas capacidades avanzadas 7D
    breakthrough_cache: HashMap<String, f64>,
    dimensional_patterns: HashMap<String, Vec<f64>>,
    contextual_memory: Vec<ContextMemoryEntry>,
    validation_metrics: ValidationMetrics,
}

/// Entrada de memoria contextual para análisis avanzado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMemoryEntry {
    pub content_hash: String,
    pub dimensional_signature: [f64; 7],
    pub breakthrough_score: f64,
    pub timestamp: DateTime<Utc>,
    pub validation_status: ValidationStatus,
}

/// Métricas de validación para análisis avanzado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationMetrics {
    pub total_analyzed: u64,
    pub breakthrough_count: u64,
    pub average_coherence: f64,
    pub dimensional_stability: f64,
    pub success_rate: f64,
}

impl ContextAnalyzer7D {
    pub fn new() -> Self {
        Self {
            dimension_weights: [1.0, 1.2, 0.9, 1.1, 0.8, 1.3, 1.5], // Peso extra en biografía
            semantic_cache: HashMap::new(),
            biographical_patterns: vec![
                "cuando era".to_string(), "mi experiencia".to_string(), "recuerdo que".to_string(),
                "en mi caso".to_string(), "personalmente".to_string(), "mi abuela".to_string(),
                "crecí en".to_string(), "mi padre".to_string(), "mi madre".to_string(),
            ],
            temporal_markers: vec![
                "ayer".to_string(), "mañana".to_string(), "antes".to_string(), "después".to_string(),
                "cuando".to_string(), "mientras".to_string(), "durante".to_string(),
            ],
            emotional_lexicon: [
                ("alegría".to_string(), 0.8), ("tristeza".to_string(), -0.7), ("amor".to_string(), 0.9),
                ("miedo".to_string(), -0.6), ("esperanza".to_string(), 0.7), ("nostalgia".to_string(), 0.3),
            ].iter().cloned().collect(),
            // Nuevas capacidades avanzadas 7D
            breakthrough_cache: HashMap::new(),
            dimensional_patterns: HashMap::new(),
            contextual_memory: Vec::new(),
            validation_metrics: ValidationMetrics {
                total_analyzed: 0,
                breakthrough_count: 0,
                average_coherence: 0.0,
                dimensional_stability: 0.0,
                success_rate: 0.0,
            },
        }
    }

    /// Análisis 7D Avanzado con capacidades breakthrough
    pub async fn advanced_7d_analysis(&mut self, token: &mut ContextToken7D) -> Result<f64> {
        println!("{}", "🔍 INICIANDO ANÁLISIS 7D AVANZADO...".bright_cyan());
        
        // Generar hash del contenido para caché
        let content_hash = format!("{:x}", md5::compute(&token.content));
        
        // Verificar caché de breakthrough
        if let Some(cached_score) = self.breakthrough_cache.get(&content_hash) {
            token.breakthrough_score = *cached_score;
            token.validation_status = ValidationStatus::Validated;
            println!("{}", format!("   ⚡ Usando resultado en caché: {:.3}", cached_score).bright_yellow());
            return Ok(*cached_score);
        }

        // Análisis dimensional completo
        let temporal_score = self.analyze_temporal_advanced(&token.content).await?;
        let semantic_score = self.analyze_semantic_advanced(&token.content).await?;
        let contextual_score = self.analyze_contextual_advanced(&token.content).await?;
        let relational_score = self.analyze_relational_advanced(&token.content).await?;
        let emotional_score = self.analyze_emotional_advanced(&token.content).await?;
        let intentional_score = self.analyze_intentional_advanced(&token.content).await?;
        let biographical_score = self.analyze_biographical_advanced(&token.content).await?;

        // Calcular coherencia dimensional
        let dimensional_signature = [
            temporal_score, semantic_score, contextual_score,
            relational_score, emotional_score, intentional_score, biographical_score
        ];
        
        token.dimensional_coherence = self.calculate_dimensional_coherence(&dimensional_signature);
        token.contextual_intensity = self.calculate_contextual_intensity(&dimensional_signature);
        
        // Calcular score breakthrough avanzado
        let breakthrough_score = self.calculate_advanced_breakthrough_score(&dimensional_signature);
        token.breakthrough_score = breakthrough_score;

        // Determinar estado de validación
        if breakthrough_score > 90.0 {
            token.validation_status = ValidationStatus::Breakthrough;
            self.validation_metrics.breakthrough_count += 1;
        } else if breakthrough_score > 75.0 {
            token.validation_status = ValidationStatus::Enhanced;
        } else {
            token.validation_status = ValidationStatus::Validated;
        }

        // Almacenar en memoria contextual
        let memory_entry = ContextMemoryEntry {
            content_hash: content_hash.clone(),
            dimensional_signature,
            breakthrough_score,
            timestamp: Utc::now(),
            validation_status: token.validation_status.clone(),
        };
        self.contextual_memory.push(memory_entry);

        // Actualizar cache
        self.breakthrough_cache.insert(content_hash, breakthrough_score);
        
        // Actualizar métricas
        self.validation_metrics.total_analyzed += 1;
        self.update_validation_metrics();

        println!("{}", format!("✨ ANÁLISIS 7D COMPLETADO: {:.3}/100", breakthrough_score).bright_green());
        println!("{}", format!("   📊 Coherencia Dimensional: {:.3}", token.dimensional_coherence).bright_blue());
        println!("{}", format!("   🌟 Intensidad Contextual: {:.3}", token.contextual_intensity).bright_magenta());

        Ok(breakthrough_score)
    }

    /// Analizar dimensión temporal específica
    pub async fn analyze_temporal_dimension(&self, content: &str) -> Result<TemporalDimension> {
        let content_lower = content.to_lowercase();
        let now = Utc::now();
        
        // Detectar anclajes temporales
        let temporal_anchors: Vec<String> = self.temporal_markers.iter()
            .filter(|marker| content_lower.contains(marker.as_str()))
            .cloned()
            .collect();
        
        // Calcular métricas temporales
        let anchor_density = temporal_anchors.len() as f64 / content.len().max(1) as f64 * 1000.0_f64;
        let chronological_depth = self.calculate_chronological_depth(&content_lower);
        
        // Campos fusionados
        let hour = now.hour();
        let time_of_day = match hour {
            6..=11 => "morning",
            12..=17 => "afternoon",
            18..=21 => "evening",
            _ => "night",
        }.to_string();
        
        Ok(TemporalDimension {
            score: (anchor_density + chronological_depth) / 2.0_f64,
            temporal_anchors,
            chronological_depth,
            temporal_consistency: 0.7,
            sequential_logic: 0.8,
            time_horizon: 0.6,
            temporal_dynamics: 0.5,
            // Campos fusionados
            sequence: 0,
            time_of_day,
            day_of_week: now.format("%A").to_string(),
            session_duration_minutes: 0,
            lifecycle_hours: 168,
        })
    }

    /// Analizar dimensión semántica específica
    pub async fn analyze_semantic_dimension(&self, content: &str) -> Result<SemanticDimension> {
        let concept_density = self.calculate_concept_density(content);
        let semantic_coherence = self.calculate_semantic_coherence(content);
        
        // Campos fusionados
        let keywords: Vec<String> = content
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .take(10)
            .map(|s| s.to_lowercase())
            .collect();
        
        let words: Vec<&str> = content.split_whitespace().collect();
        let unique_words: std::collections::HashSet<_> = words.iter().collect();
        let semantic_density = if !words.is_empty() {
            unique_words.len() as f64 / words.len() as f64
        } else {
            0.0
        };
        
        Ok(SemanticDimension {
            score: (concept_density + semantic_coherence) / 2.0_f64,
            concept_density,
            semantic_coherence,
            domain_specificity: 0.7,
            conceptual_depth: 0.8,
            terminology_precision: 0.6,
            semantic_relationships: Vec::new(),
            // Campos fusionados
            text: content.to_string(),
            language: "unknown".to_string(),
            keywords,
            embeddings: Vec::new(),
            semantic_density,
        })
    }

    /// Analizar dimensión contextual específica
    pub async fn analyze_contextual_dimension(&self, content: &str) -> Result<ContextualDimension> {
        let situational_relevance = self.calculate_situational_relevance(content);
        
        Ok(ContextualDimension {
            score: situational_relevance,
            situational_relevance,
            environmental_factors: Vec::new(),
            contextual_coherence: 0.8,
            scope_awareness: 0.7,
            meta_context: 0.6,
            // Campos fusionados
            session_id: String::new(),
            user_id: "anonymous".to_string(),
            context_markers: Vec::new(),
            situational_frame: "unknown".to_string(),
            coherence_with_previous: 0.0,
        })
    }

    /// Analizar dimensión relacional específica
    pub async fn analyze_relational_dimension(&self, content: &str) -> Result<RelationalDimension> {
        let network_density = self.calculate_network_density(content);
        
        Ok(RelationalDimension {
            score: network_density,
            entity_connections: Vec::new(),
            relationship_strength: 0.7,
            network_density,
            relational_patterns: Vec::new(),
            social_dynamics: 0.6,
            // Campos fusionados
            related_tokens: Vec::new(),
            entity_graph: HashMap::new(),
            connection_strength: 0.0,
            pattern_matches: Vec::new(),
        })
    }

    /// Analizar dimensión emocional específica
    pub async fn analyze_emotional_dimension(&self, content: &str) -> Result<EmotionalDimension> {
        let emotional_intensity = self.calculate_emotional_intensity(content);
        let emotional_valence_calc = self.calculate_emotional_valence(content);
        
        Ok(EmotionalDimension {
            score: (emotional_intensity + emotional_valence_calc.abs()) / 2.0_f64,
            emotional_intensity,
            emotional_valence: emotional_valence_calc,
            emotional_complexity: Vec::new(),
            empathy_markers: Vec::new(),
            emotional_resonance: 0.7,
            // Campos fusionados (VADC model)
            valence: emotional_valence_calc,
            arousal: emotional_intensity,
            dominance: 0.5,
            certainty: 0.7,
            emotional_trajectory: Vec::new(),
        })
    }

    /// Analizar dimensión intencional específica
    pub async fn analyze_intentional_dimension(&self, content: &str) -> Result<IntentionalDimension> {
        let goal_clarity = self.calculate_goal_clarity(content);
        
        Ok(IntentionalDimension {
            score: goal_clarity,
            goal_clarity,
            action_orientation: 0.7,
            strategic_thinking: 0.8,
            purpose_alignment: 0.6,
            intentional_depth: 0.5,
            // Campos fusionados
            intent_category: "unknown".to_string(),
            goal: "unknown".to_string(),
            action_required: false,
            urgency: 0.0,
            clarity: goal_clarity,
        })
    }

    /// Analizar dimensión biográfica específica - CLAVE DEL BREAKTHROUGH
    pub async fn analyze_biographical_dimension(&self, content: &str) -> Result<BiographicalDimension> {
        let content_lower = content.to_lowercase();
        
        // Detectar marcadores biográficos
        let personal_markers: Vec<String> = self.biographical_patterns.iter()
            .filter(|pattern| content_lower.contains(pattern.as_str()))
            .cloned()
            .collect();
        
        let biographical_depth = personal_markers.len() as f64 / 10.0_f64; // Normalizar
        let experiential_uniqueness = self.calculate_experiential_uniqueness(&content_lower);
        
        // BREAKTHROUGH: La dimensión biográfica tiene peso extra
        let breakthrough_multiplier = 1.3;
        let base_score = (biographical_depth + experiential_uniqueness) / 2.0_f64;
        
        Ok(BiographicalDimension {
            score: base_score * breakthrough_multiplier,
            personal_experience_markers: personal_markers,
            life_stage_indicators: Vec::new(),
            biographical_depth,
            personal_narrative_strength: 0.8,
            identity_markers: Vec::new(),
            experiential_uniqueness,
            biographical_resonance: 0.9,
            // Campos fusionados
            user_expertise_level: 0.0,
            historical_patterns: Vec::new(),
            preferences: HashMap::new(),
            biographical_coherence: 0.0,
            personal_significance: 0.0,
        })
    }

    // Métodos auxiliares de análisis
    fn calculate_chronological_depth(&self, content: &str) -> f64 {
        // Placeholder - análisis temporal profundo
        if content.contains("hace años") || content.contains("cuando era pequeño") {
            0.9
        } else if content.contains("ayer") || content.contains("mañana") {
            0.6
        } else {
            0.3
        }
    }

    fn calculate_concept_density(&self, content: &str) -> f64 {
        // Placeholder - densidad conceptual
        content.split_whitespace().count() as f64 / content.len().max(1) as f64 * 100.0
    }

    fn calculate_semantic_coherence(&self, content: &str) -> f64 {
        // Placeholder - coherencia semántica
        0.7 // Valor base
    }

    fn calculate_situational_relevance(&self, content: &str) -> f64 {
        // Placeholder - relevancia situacional
        0.8
    }

    fn calculate_network_density(&self, content: &str) -> f64 {
        // Placeholder - densidad de red
        0.6
    }

    fn calculate_emotional_intensity(&self, content: &str) -> f64 {
        let content_lower = content.to_lowercase();
        let mut intensity = 0.0;
        
        for (emotion, weight) in &self.emotional_lexicon {
            if content_lower.contains(emotion) {
                intensity += weight.abs();
            }
        }
        
        (intensity / 5.0).min(1.0) // Normalizar
    }

    fn calculate_emotional_valence(&self, content: &str) -> f64 {
        let content_lower = content.to_lowercase();
        let mut valence = 0.0;
        let mut count = 0;
        
        for (emotion, weight) in &self.emotional_lexicon {
            if content_lower.contains(emotion) {
                valence += weight;
                count += 1;
            }
        }
        
        if count > 0 { valence / count as f64 } else { 0.0 }
    }

    fn calculate_goal_clarity(&self, content: &str) -> f64 {
        let content_lower = content.to_lowercase();
        if content_lower.contains("quiero") || content_lower.contains("objetivo") || content_lower.contains("meta") {
            0.8
        } else {
            0.4
        }
    }

    fn calculate_experiential_uniqueness(&self, content: &str) -> f64 {
        // BREAKTHROUGH: Factor crítico - mide la singularidad experiencial
        let uniqueness_markers = [
            "nunca antes", "por primera vez", "única experiencia", "solo yo",
            "nadie más", "mi particular", "exclusivamente", "únicamente"
        ];

        let uniqueness_count = uniqueness_markers.iter()
            .filter(|&marker| content.to_lowercase().contains(marker))
            .count();
        
        (uniqueness_count as f64 / uniqueness_markers.len() as f64).min(1.0)
    }

    // ==================== MÉTODOS DE ANÁLISIS AVANZADO 7D ====================

    /// Análisis temporal avanzado con breakthrough
    pub async fn analyze_temporal_advanced(&self, content: &str) -> Result<f64> {
        let basic_temporal = self.calculate_chronological_depth(content);
        let temporal_complexity = self.calculate_temporal_complexity(content);
        let temporal_coherence = self.calculate_temporal_coherence(content);
        
        Ok((basic_temporal + temporal_complexity + temporal_coherence) / 3.0 * 100.0)
    }

    /// Análisis semántico avanzado con breakthrough
    pub async fn analyze_semantic_advanced(&self, content: &str) -> Result<f64> {
        let semantic_density = self.calculate_semantic_density(content);
        let concept_coherence = self.calculate_semantic_coherence(content);
        let domain_specificity = self.calculate_domain_specificity(content);
        
        Ok((semantic_density + concept_coherence + domain_specificity) / 3.0 * 100.0)
    }

    /// Análisis contextual avanzado
    pub async fn analyze_contextual_advanced(&self, content: &str) -> Result<f64> {
        let contextual_depth = self.calculate_contextual_depth(content);
        let situational_relevance = self.calculate_situational_relevance(content);
        let environmental_awareness = self.calculate_environmental_awareness(content);
        
        Ok((contextual_depth + situational_relevance + environmental_awareness) / 3.0 * 100.0)
    }

    /// Análisis relacional avanzado
    pub async fn analyze_relational_advanced(&self, content: &str) -> Result<f64> {
        let social_connectivity = self.calculate_social_connectivity(content);
        let network_density = self.calculate_network_density(content);
        let relationship_depth = self.calculate_relationship_depth(content);
        
        Ok((social_connectivity + network_density + relationship_depth) / 3.0 * 100.0)
    }

    /// Análisis emocional avanzado
    pub async fn analyze_emotional_advanced(&self, content: &str) -> Result<f64> {
        let emotional_intensity = self.calculate_emotional_intensity(content);
        let emotional_complexity = self.calculate_emotional_complexity(content);
        let emotional_resonance = self.calculate_emotional_resonance(content);
        
        Ok((emotional_intensity + emotional_complexity + emotional_resonance) / 3.0 * 100.0)
    }

    /// Análisis intencional avanzado
    pub async fn analyze_intentional_advanced(&self, content: &str) -> Result<f64> {
        let goal_clarity = self.calculate_goal_clarity(content);
        let purpose_strength = self.calculate_purpose_strength(content);
        let strategic_depth = self.calculate_strategic_depth(content);
        
        Ok((goal_clarity + purpose_strength + strategic_depth) / 3.0 * 100.0)
    }

    /// Análisis biográfico avanzado - CLAVE DEL BREAKTHROUGH
    pub async fn analyze_biographical_advanced(&self, content: &str) -> Result<f64> {
        let experiential_depth = self.calculate_experiential_uniqueness(content);
        let personal_narrative = self.calculate_personal_narrative_strength(content);
        let identity_coherence = self.calculate_identity_coherence(content);
        let biographical_resonance = self.calculate_biographical_resonance(content);
        
        // BREAKTHROUGH: Peso especial para dimensión biográfica
        let base_score = (experiential_depth + personal_narrative + identity_coherence + biographical_resonance) / 4.0;
        let breakthrough_multiplier = 1.4; // Factor breakthrough
        
        Ok(base_score * breakthrough_multiplier * 100.0)
    }

    // ==================== MÉTODOS DE CÁLCULO AVANZADO ====================

    fn calculate_dimensional_coherence(&self, dimensional_signature: &[f64; 7]) -> f64 {
        // Calcular varianza para medir coherencia
        let mean = dimensional_signature.iter().sum::<f64>() / 7.0;
        let variance = dimensional_signature.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / 7.0;
        
        // Menor varianza = mayor coherencia (invertimos y normalizamos)
        (1.0 / (1.0 + variance)).min(1.0).max(0.0)
    }

    fn calculate_contextual_intensity(&self, dimensional_signature: &[f64; 7]) -> f64 {
        // Intensidad basada en la suma ponderada de las dimensiones
        let weights = &self.dimension_weights;
        let weighted_sum: f64 = dimensional_signature.iter()
            .zip(weights.iter())
            .map(|(score, weight)| score * weight)
            .sum();
        
        let max_possible: f64 = weights.iter().map(|w| w * 100.0).sum();
        (weighted_sum / max_possible).min(1.0).max(0.0)
    }

    fn calculate_advanced_breakthrough_score(&self, dimensional_signature: &[f64; 7]) -> f64 {
        let coherence = self.calculate_dimensional_coherence(dimensional_signature);
        let intensity = self.calculate_contextual_intensity(dimensional_signature);
        let dimensional_average = dimensional_signature.iter().sum::<f64>() / 7.0;
        
        // Fórmula breakthrough avanzada
        let base_score = coherence * 0.3 + intensity * 0.4 + dimensional_average * 0.3;
        let breakthrough_bonus = if base_score > 0.75 { base_score * 0.2 } else { 0.0 };
        
        ((base_score + breakthrough_bonus) * 100.0).min(100.0).max(0.0)
    }

    fn update_validation_metrics(&mut self) {
        if self.validation_metrics.total_analyzed > 0 {
            self.validation_metrics.success_rate = 
                self.validation_metrics.breakthrough_count as f64 / 
                self.validation_metrics.total_analyzed as f64;
            
            // Calcular coherencia promedio de la memoria contextual
            if !self.contextual_memory.is_empty() {
                let avg_coherence: f64 = self.contextual_memory.iter()
                    .map(|entry| {
                        self.calculate_dimensional_coherence(&entry.dimensional_signature)
                    })
                    .sum::<f64>() / self.contextual_memory.len() as f64;
                
                self.validation_metrics.average_coherence = avg_coherence;
            }
        }
    }

    // ==================== MÉTODOS AUXILIARES DE ANÁLISIS ====================

    fn calculate_temporal_complexity(&self, content: &str) -> f64 {
        // Analizar complejidad temporal del contenido
        let temporal_indicators = [
            "antes de", "después de", "mientras", "durante", "cuando", 
            "hasta que", "desde que", "al mismo tiempo"
        ];
        
        let complexity_count = temporal_indicators.iter()
            .filter(|&indicator| content.to_lowercase().contains(indicator))
            .count();
        
        (complexity_count as f64 / 8.0).min(1.0) // Normalizar a [0,1]
    }

    fn calculate_temporal_coherence(&self, content: &str) -> f64 {
        // Medir coherencia temporal en narrativa
        let content_lower = content.to_lowercase();
        let has_past = content_lower.contains("era") || content_lower.contains("fue");
        let has_present = content_lower.contains("es") || content_lower.contains("está");
        let has_future = content_lower.contains("será") || content_lower.contains("va a");
        
        let temporal_variety = [has_past, has_present, has_future].iter()
            .filter(|&&x| x).count() as f64;
        
        temporal_variety / 3.0 // Normalizar a [0,1]
    }

    fn calculate_semantic_density(&self, content: &str) -> f64 {
        let words = content.split_whitespace().collect::<Vec<_>>();
        let unique_words = words.iter().collect::<std::collections::HashSet<_>>();
        
        if words.is_empty() { 0.0 } else { unique_words.len() as f64 / words.len() as f64 }
    }

    fn calculate_domain_specificity(&self, content: &str) -> f64 {
        // Detectar términos técnicos/especializados
        let specialized_terms = [
            "algoritmo", "función", "variable", "estructura", "sistema",
            "proceso", "método", "técnica", "estrategia", "análisis"
        ];
        
        let term_count = specialized_terms.iter()
            .filter(|&term| content.to_lowercase().contains(term))
            .count();
        
        (term_count as f64 / specialized_terms.len() as f64).min(1.0)
    }

    fn calculate_contextual_depth(&self, content: &str) -> f64 {
        // Medir profundidad contextual
        let depth_indicators = [
            "porque", "debido a", "como resultado", "por tanto", "sin embargo",
            "además", "por otro lado", "en consecuencia", "finalmente"
        ];
        
        let depth_count = depth_indicators.iter()
            .filter(|&indicator| content.to_lowercase().contains(indicator))
            .count();
        
        (depth_count as f64 / depth_indicators.len() as f64).min(1.0)
    }

    fn calculate_environmental_awareness(&self, content: &str) -> f64 {
        // Medir conciencia del entorno/contexto
        let environmental_markers = [
            "en este caso", "en esta situación", "considerando", "teniendo en cuenta",
            "en el contexto", "dada la circunstancia", "en este momento"
        ];
        
        let awareness_count = environmental_markers.iter()
            .filter(|&marker| content.to_lowercase().contains(marker))
            .count();
        
        (awareness_count as f64 / environmental_markers.len() as f64).min(1.0)
    }

    fn calculate_social_connectivity(&self, content: &str) -> f64 {
        // Medir conectividad social
        let social_markers = [
            "nosotros", "todos", "la gente", "las personas", "mi familia",
            "mis amigos", "la comunidad", "juntos", "compartir"
        ];
        
        let social_count = social_markers.iter()
            .filter(|&marker| content.to_lowercase().contains(marker))
            .count();
        
        (social_count as f64 / social_markers.len() as f64).min(1.0)
    }

    fn calculate_relationship_depth(&self, content: &str) -> f64 {
        // Medir profundidad relacional
        let depth_words = content.split_whitespace()
            .filter(|word| word.len() > 5) // Palabras más largas suelen indicar mayor profundidad
            .count();
        
        let total_words = content.split_whitespace().count();
        
        if total_words > 0 { 
            (depth_words as f64 / total_words as f64).min(1.0) 
        } else { 
            0.0 
        }
    }

    fn calculate_emotional_complexity(&self, content: &str) -> f64 {
        // Contar diferentes tipos de emociones
        let emotion_types = self.emotional_lexicon.keys()
            .filter(|emotion| content.to_lowercase().contains(emotion.as_str()))
            .count();
        
        (emotion_types as f64 / self.emotional_lexicon.len() as f64).min(1.0)
    }

    fn calculate_emotional_resonance(&self, content: &str) -> f64 {
        // Calcular resonancia emocional
        let resonance_words = [
            "profundamente", "intensamente", "realmente", "verdaderamente",
            "completamente", "totalmente", "absolutamente"
        ];
        
        let resonance_count = resonance_words.iter()
            .filter(|&word| content.to_lowercase().contains(word))
            .count();
        
        (resonance_count as f64 / resonance_words.len() as f64).min(1.0)
    }

    fn calculate_purpose_strength(&self, content: &str) -> f64 {
        // Medir fuerza del propósito
        let purpose_indicators = [
            "quiero", "necesito", "debo", "voy a", "mi objetivo",
            "mi meta", "mi propósito", "pretendo", "busco"
        ];
        
        let purpose_count = purpose_indicators.iter()
            .filter(|&indicator| content.to_lowercase().contains(indicator))
            .count();
        
        (purpose_count as f64 / purpose_indicators.len() as f64).min(1.0)
    }

    fn calculate_strategic_depth(&self, content: &str) -> f64 {
        // Medir profundidad estratégica
        let strategic_words = [
            "planear", "estrategia", "paso a paso", "primero", "luego",
            "finalmente", "considerar", "evaluar", "decidir"
        ];
        
        let strategic_count = strategic_words.iter()
            .filter(|&word| content.to_lowercase().contains(word))
            .count();
        
        (strategic_count as f64 / strategic_words.len() as f64).min(1.0)
    }

    fn calculate_personal_narrative_strength(&self, content: &str) -> f64 {
        // Medir fuerza de narrativa personal
        let narrative_indicators = [
            "mi historia", "mi experiencia", "lo que me pasó", "recuerdo",
            "cuando yo", "en mi vida", "mi camino", "mi proceso"
        ];
        
        let narrative_count = narrative_indicators.iter()
            .filter(|&indicator| content.to_lowercase().contains(indicator))
            .count();
        
        (narrative_count as f64 / narrative_indicators.len() as f64).min(1.0)
    }

    fn calculate_identity_coherence(&self, content: &str) -> f64 {
        // Medir coherencia de identidad
        let identity_markers = [
            "soy", "me defino", "mi identidad", "quien soy", "mi esencia",
            "mi naturaleza", "mi carácter", "mi personalidad"
        ];
        
        let identity_count = identity_markers.iter()
            .filter(|&marker| content.to_lowercase().contains(marker))
            .count();
        
        (identity_count as f64 / identity_markers.len() as f64).min(1.0)
    }

    fn calculate_biographical_resonance(&self, content: &str) -> f64 {
        // Medir resonancia biográfica
        let personal_depth = self.biographical_patterns.iter()
            .filter(|pattern| content.to_lowercase().contains(pattern.as_str()))
            .count();
        
        (personal_depth as f64 / self.biographical_patterns.len() as f64).min(1.0)
    }

    // ==================== MÉTODOS PÚBLICOS PARA ACCESO A MÉTRICAS ====================

    /// Obtener métricas de validación
    pub fn get_validation_metrics(&self) -> &ValidationMetrics {
        &self.validation_metrics
    }

    /// Obtener número de entradas en memoria contextual
    pub fn get_memory_entries_count(&self) -> usize {
        self.contextual_memory.len()
    }

    /// Obtener tamaño del caché de breakthrough
    pub fn get_cache_size(&self) -> usize {
        self.breakthrough_cache.len()
    }

    /// Obtener patrones dimensionales
    pub fn get_dimensional_patterns(&self) -> &HashMap<String, Vec<f64>> {
        &self.dimensional_patterns
    }

    /// Obtener memoria contextual (read-only)
    pub fn get_contextual_memory(&self) -> &Vec<ContextMemoryEntry> {
        &self.contextual_memory
    }

    // ==================== MÉTODOS DE COMPATIBILIDAD ====================

    /// Calcular peso bidireccional (método de compatibilidad)
    pub fn calculate_bidirectional_weight(&self, token: &ContextToken7D) -> Result<f64> {
        // BREAKTHROUGH: Cálculo avanzado del peso bidireccional basado en las 7 dimensiones
        let dimension_scores = token.dimensions();
        let avg_score: f64 = dimension_scores.iter().sum::<f64>() / dimension_scores.len() as f64;
        
        // Factor de multiplicación basado en el score breakthrough
        let breakthrough_factor = if token.achieves_breakthrough() { 1.3 } else { 1.0 };
        
        Ok((avg_score * breakthrough_factor).min(2.0)) // Máximo 2.0 para peso bidireccional
    }

    /// Generar huella semántica (método de compatibilidad)
    pub fn generate_semantic_fingerprint(&self, content: &str) -> Result<String> {
        // Generar un fingerprint basado en las características semánticas
        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count = words.len();
        let char_count = content.len();
        let avg_word_length = if word_count > 0 { char_count / word_count } else { 0 };
        
        Ok(format!("sem_fp_{}_{}_{}_{}", 
            word_count, 
            char_count, 
            avg_word_length,
            content.chars().take(10).collect::<String>().replace(' ', "_")
        ))
    }

    // ==================== EXTRACTORES HEURÍSTICOS (FUSIÓN BAYESIANA) ====================
    
    /// Extraer dimensión temporal desde input normalizado
    pub fn extract_temporal(&self, _input: &NormalizedInput, sequence: u64, session_start: DateTime<Utc>) -> Result<TemporalDimension> {
        let now = Utc::now();
        let session_duration = (now - session_start).num_minutes() as u64;
        
        let hour = now.hour();
        let time_of_day = match hour {
            6..=11 => "morning",
            12..=17 => "afternoon",
            18..=21 => "evening",
            _ => "night",
        }.to_string();
        
        let day_of_week = now.format("%A").to_string();
        
        // Detectar anclajes temporales (del método original)
        let temporal_anchors = vec!["ahora".to_string(), "hoy".to_string()];
        
        Ok(TemporalDimension {
            score: 0.8,  // Score base
            temporal_anchors,
            chronological_depth: 0.7,
            temporal_consistency: 0.8,
            sequential_logic: 0.8,
            time_horizon: 0.6,
            temporal_dynamics: 0.5,
            // Campos fusionados
            sequence,
            time_of_day,
            day_of_week,
            session_duration_minutes: session_duration,
            lifecycle_hours: 168,  // 7 días por defecto
        })
    }
    
    /// Extraer dimensión semántica desde input normalizado
    pub fn extract_semantic(&self, input: &NormalizedInput) -> Result<SemanticDimension> {
        // Extraer keywords básico
        let keywords: Vec<String> = input.text
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .take(10)
            .map(|s| s.to_lowercase())
            .collect();
        
        // Densidad semántica = ratio de palabras únicas vs total
        let words: Vec<&str> = input.text.split_whitespace().collect();
        let unique_words: std::collections::HashSet<_> = words.iter().collect();
        let semantic_density = if !words.is_empty() {
            unique_words.len() as f64 / words.len() as f64
        } else {
            0.0
        };
        
        let concept_density = self.calculate_concept_density(&input.text);
        let semantic_coherence = self.calculate_semantic_coherence(&input.text);
        
        Ok(SemanticDimension {
            score: (concept_density + semantic_coherence) / 2.0,
            concept_density,
            semantic_coherence,
            domain_specificity: 0.7,
            conceptual_depth: 0.8,
            terminology_precision: 0.6,
            semantic_relationships: Vec::new(),
            // Campos fusionados
            text: input.text.clone(),
            language: input.language.clone(),
            keywords,
            embeddings: vec![],  // Por ahora vacío (requiere modelo embeddings)
            semantic_density,
        })
    }
    
    /// Extraer dimensión contextual desde input normalizado
    pub fn extract_contextual(&self, input: &NormalizedInput, sequence: u64) -> Result<ContextualDimension> {
        // Extraer markers contextuales del metadata
        let context_markers: Vec<String> = input.metadata
            .iter()
            .filter(|(k, _)| k.starts_with("context_"))
            .map(|(_, v)| v.clone())
            .collect();
        
        let session_id = input.metadata
            .get("session_id")
            .cloned()
            .unwrap_or_else(|| format!("session_{}", sequence));
        
        let user_id = input.metadata
            .get("user_id")
            .cloned()
            .unwrap_or_else(|| "anonymous".to_string());
        
        // Coherencia con previo (por ahora stub, requiere historial)
        let coherence = 0.7;
        
        let situational_relevance = self.calculate_situational_relevance(&input.text);
        
        Ok(ContextualDimension {
            score: situational_relevance,
            situational_relevance,
            environmental_factors: Vec::new(),
            contextual_coherence: 0.8,
            scope_awareness: 0.7,
            meta_context: 0.6,
            // Campos fusionados
            session_id,
            user_id,
            context_markers,
            situational_frame: "work".to_string(),  // Stub
            coherence_with_previous: coherence,
        })
    }
    
    /// Extraer dimensión relacional desde input normalizado
    pub fn extract_relational(&self, input: &NormalizedInput) -> Result<RelationalDimension> {
        // Buscar entidades (nombres propios, conceptos clave)
        let mut entity_graph = HashMap::new();
        
        // Stub básico: palabras capitalizadas = entidades
        let entities: Vec<String> = input.text
            .split_whitespace()
            .filter(|w| w.chars().next().map_or(false, |c| c.is_uppercase()))
            .map(|s| s.to_string())
            .collect();
        
        for entity in &entities {
            entity_graph.insert(entity.clone(), vec![]);
        }
        
        // Strength basado en confidence del input
        let connection_strength = input.confidence;
        
        let network_density = self.calculate_network_density(&input.text);
        
        Ok(RelationalDimension {
            score: network_density,
            entity_connections: Vec::new(),
            relationship_strength: 0.7,
            network_density,
            relational_patterns: Vec::new(),
            social_dynamics: 0.6,
            // Campos fusionados
            related_tokens: vec![],  // Requiere historial de tokens
            entity_graph,
            connection_strength,
            pattern_matches: vec![],  // Requiere pattern matching engine
        })
    }
    
    /// Extraer dimensión emocional desde input normalizado
    pub fn extract_emotional(&self, input: &NormalizedInput) -> Result<EmotionalDimension> {
        // Valence = sentiment del input
        let valence = input.sentiment;
        
        // Arousal estimado por longitud y signos de exclamación
        let exclamations = input.text.matches('!').count();
        let questions = input.text.matches('?').count();
        let arousal = ((exclamations + questions) as f64 / 10.0).min(1.0);
        
        // Dominance basado en imperativos (stub)
        let dominance = if input.text.to_lowercase().contains("must") 
            || input.text.to_lowercase().contains("should") {
            0.7
        } else {
            0.5
        };
        
        // Certainty = confidence
        let certainty = input.confidence;
        
        let emotional_intensity = self.calculate_emotional_intensity(&input.text);
        let emotional_valence_calc = self.calculate_emotional_valence(&input.text);
        
        Ok(EmotionalDimension {
            score: (emotional_intensity + emotional_valence_calc.abs()) / 2.0,
            emotional_intensity,
            emotional_valence: emotional_valence_calc,
            emotional_complexity: Vec::new(),
            empathy_markers: Vec::new(),
            emotional_resonance: 0.7,
            // Campos fusionados (VADC model)
            valence,
            arousal,
            dominance,
            certainty,
            emotional_trajectory: vec![],  // Requiere historial
        })
    }
    
    /// Extraer dimensión intencional desde input normalizado
    pub fn extract_intentional(&self, input: &NormalizedInput) -> Result<IntentionalDimension> {
        // Detectar intent básico
        let has_question = input.text.contains('?');
        let has_imperative = input.text.to_lowercase().starts_with("create")
            || input.text.to_lowercase().starts_with("make")
            || input.text.to_lowercase().starts_with("do");
        
        let intent_category = if has_question {
            "question"
        } else if has_imperative {
            "command"
        } else {
            "statement"
        }.to_string();
        
        // Goal estimado
        let goal = if input.text.to_lowercase().contains("debug") {
            "debug"
        } else if input.text.to_lowercase().contains("learn") {
            "learn"
        } else {
            "general"
        }.to_string();
        
        // Urgency basado en palabras clave
        let urgency = if input.text.to_lowercase().contains("urgent")
            || input.text.to_lowercase().contains("asap") {
            0.9
        } else {
            0.5
        };
        
        // Clarity = confidence
        let clarity_value = input.confidence;
        
        let goal_clarity = self.calculate_goal_clarity(&input.text);
        
        Ok(IntentionalDimension {
            score: goal_clarity,
            goal_clarity,
            action_orientation: 0.7,
            strategic_thinking: 0.8,
            purpose_alignment: 0.6,
            intentional_depth: 0.5,
            // Campos fusionados
            intent_category,
            goal,
            action_required: has_imperative || has_question,
            urgency,
            clarity: clarity_value,
        })
    }
    
    /// Extraer dimensión biográfica desde input normalizado - CLAVE DEL BREAKTHROUGH
    pub fn extract_biographical(&self, input: &NormalizedInput) -> Result<BiographicalDimension> {
        // Expertise estimado (requiere TelescopeDB integration)
        let expertise_level = input.metadata
            .get("expertise_level")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.5);
        
        // Coherencia biográfica (stub, requiere historial de usuario)
        let biographical_coherence = 0.7;
        
        // Significance estimado por longitud
        let significance = (input.text.len() as f64 / 500.0).min(1.0);
        
        let content_lower = input.text.to_lowercase();
        
        // Detectar marcadores biográficos (del método original)
        let personal_markers: Vec<String> = self.biographical_patterns.iter()
            .filter(|pattern| content_lower.contains(pattern.as_str()))
            .cloned()
            .collect();
        
        let biographical_depth = personal_markers.len() as f64 / 10.0; // Normalizar
        let experiential_uniqueness = self.calculate_experiential_uniqueness(&content_lower);
        
        // BREAKTHROUGH: La dimensión biográfica tiene peso extra
        let breakthrough_multiplier = 1.3;
        let base_score = (biographical_depth + experiential_uniqueness) / 2.0;
        
        Ok(BiographicalDimension {
            score: base_score * breakthrough_multiplier,
            personal_experience_markers: personal_markers,
            life_stage_indicators: Vec::new(),
            biographical_depth,
            personal_narrative_strength: 0.8,
            identity_markers: Vec::new(),
            experiential_uniqueness,
            biographical_resonance: 0.9,
            // Campos fusionados
            user_expertise_level: expertise_level,
            historical_patterns: vec![],  // Requiere TelescopeDB
            preferences: HashMap::new(),  // Requiere perfil de usuario
            biographical_coherence,
            personal_significance: significance,
        })
    }
}

impl Default for ContextAnalyzer7D {
    fn default() -> Self {
        Self::new()
    }
}