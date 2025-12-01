```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/09_relationship-psychology-analyzer.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 18:35:00
Autor: Sistema Bitácora + Eduardo
Propósito: Sistema de análisis psicológico de relaciones interpersonales
Estado: 🎯 ACTIVO - Especificación de componente
Relacionado Con:
  - 00_VISION/DA-035_HYBRID_INTELLIGENCE_ENGINE.md (visión)
  - 01_ARQUITECTURA/13_hybrid-intelligence-engine.md (arquitectura)
  - src/data_import/extraction.rs (implementación)
Categoría: COMPONENT
Prioridad: ALTA
# === FIN DATOS DE AUDITORÍA ===
```

# 🧠 Relationship Psychology Analyzer

> **Component:** RelationshipExtractor (Task 7.x.3.5)  
> **Purpose:** Analizar dinámicas psicológicas y sociales entre usuarios

---

## 📋 TABLA DE CONTENIDOS

1. [Component Overview](#component-overview)
2. [Psychological Models](#psychological-models)
3. [Relationship Classification](#relationship-classification)
4. [Conflict Detection](#conflict-detection)
5. [Evolution Tracking](#evolution-tracking)
6. [Implementation](#implementation)

---

## 🎯 COMPONENT OVERVIEW

### Responsibilities

```yaml
Primary:
  - Classify relationship type (Romantic, Family, Friend, Professional)
  - Detect conflicts & contradictions
  - Track relationship evolution over time
  - Generate psychological profile
  
Secondary:
  - Identify attachment styles
  - Analyze communication patterns
  - Detect power dynamics
  - Predict relationship health
```

### Architecture Position

```
DigestedData
    │
    ├─> EmotionalExtractor     ─┐
    ├─> InterestExtractor       │
    ├─> BiographicalExtractor   ├─> RelationshipExtractor
    └─> TemporalExtractor      ─┘         │
                                           │
                                      ExtractedNutrients
                                      (Social Dimension)
```

---

## 🧠 PSYCHOLOGICAL MODELS

### 1. Attachment Theory (Bowlby)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttachmentStyle {
    /// Secure: "Te amo", consistent communication
    Secure {
        confidence: f64,
        indicators: Vec<String>,
    },
    
    /// Anxious: "¿Por qué no respondes?", frequent validation-seeking
    Anxious {
        confidence: f64,
        indicators: Vec<String>,
    },
    
    /// Avoidant: "No quiero hablar", distant communication
    Avoidant {
        confidence: f64,
        indicators: Vec<String>,
    },
    
    /// Disorganized: Conflicting signals
    Disorganized {
        confidence: f64,
        indicators: Vec<String>,
    },
}

impl AttachmentStyle {
    pub fn detect_from_conversation(
        entries: &[DigestedEntry],
        emotional_nutrients: &[Nutrient],
    ) -> AnalysisResult<Self> {
        let mut secure_score = 0.0;
        let mut anxious_score = 0.0;
        let mut avoidant_score = 0.0;
        
        // Analyze emotional patterns
        for nutrient in emotional_nutrients {
            match nutrient.value.as_str() {
                // Secure indicators
                "love" | "trust" | "support" => secure_score += 1.0,
                
                // Anxious indicators
                "worry" | "doubt" | "fear_abandonment" => anxious_score += 1.0,
                
                // Avoidant indicators
                "distance" | "independence" | "closed_off" => avoidant_score += 1.0,
                
                _ => {}
            }
        }
        
        // Analyze response patterns
        let response_times = calculate_response_times(entries);
        if response_times.iter().any(|&t| t > Duration::hours(24)) {
            avoidant_score += 2.0;
        }
        
        // Frequency analysis
        let message_frequency = calculate_frequency(entries);
        match message_frequency {
            Frequency::Multiple_Daily => secure_score += 1.0,
            Frequency::Sporadic => avoidant_score += 1.0,
            _ => {}
        }
        
        // Determine style
        let total = secure_score + anxious_score + avoidant_score;
        if total == 0.0 {
            return Ok(AnalysisResult {
                result: AttachmentStyle::Secure { 
                    confidence: 0.1, 
                    indicators: vec![] 
                },
                confidence: 0.1,
                source: AnalysisSource::Local { 
                    analyzer: "AttachmentDetector".to_string(),
                    duration_ms: 0,
                },
                low_confidence_reason: Some("Insufficient data".to_string()),
                metadata: HashMap::new(),
            });
        }
        
        let secure_pct = secure_score / total;
        let anxious_pct = anxious_score / total;
        let avoidant_pct = avoidant_score / total;
        
        if secure_pct > 0.5 {
            Ok(AnalysisResult {
                result: AttachmentStyle::Secure { 
                    confidence: secure_pct,
                    indicators: vec![
                        "Consistent communication".to_string(),
                        "Positive emotional expression".to_string(),
                    ],
                },
                confidence: secure_pct,
                source: AnalysisSource::Local { 
                    analyzer: "AttachmentDetector".to_string(),
                    duration_ms: 0,
                },
                low_confidence_reason: None,
                metadata: HashMap::new(),
            })
        } else if anxious_pct > avoidant_pct {
            Ok(AnalysisResult {
                result: AttachmentStyle::Anxious { 
                    confidence: anxious_pct,
                    indicators: vec![
                        "Validation-seeking behavior".to_string(),
                        "Frequent worry expressions".to_string(),
                    ],
                },
                confidence: anxious_pct,
                source: AnalysisSource::Local { 
                    analyzer: "AttachmentDetector".to_string(),
                    duration_ms: 0,
                },
                low_confidence_reason: None,
                metadata: HashMap::new(),
            })
        } else {
            Ok(AnalysisResult {
                result: AttachmentStyle::Avoidant { 
                    confidence: avoidant_pct,
                    indicators: vec![
                        "Sporadic communication".to_string(),
                        "Emotional distance".to_string(),
                    ],
                },
                confidence: avoidant_pct,
                source: AnalysisSource::Local { 
                    analyzer: "AttachmentDetector".to_string(),
                    duration_ms: 0,
                },
                low_confidence_reason: None,
                metadata: HashMap::new(),
            })
        }
    }
}
```

### 2. Communication Patterns (Gottman)

```rust
/// The Four Horsemen of relationship apocalypse (Gottman)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DestructivePattern {
    /// Attacking character, not behavior
    Criticism {
        examples: Vec<String>,
        frequency: f64,  // per week
    },
    
    /// Insulting or psychologically abusive
    Contempt {
        examples: Vec<String>,
        frequency: f64,
    },
    
    /// Denying responsibility, making excuses
    Defensiveness {
        examples: Vec<String>,
        frequency: f64,
    },
    
    /// Withdrawing from conversation
    Stonewalling {
        examples: Vec<String>,
        frequency: f64,
    },
}

/// Positive patterns (Gottman)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstructivePattern {
    /// Gentle startup, soft complaints
    GentleStartup {
        examples: Vec<String>,
        frequency: f64,
    },
    
    /// Making/accepting repair attempts
    RepairAttempts {
        examples: Vec<String>,
        frequency: f64,
    },
    
    /// Accepting influence from partner
    AcceptingInfluence {
        examples: Vec<String>,
        frequency: f64,
    },
    
    /// Compromise and collaboration
    Compromise {
        examples: Vec<String>,
        frequency: f64,
    },
}

pub struct CommunicationAnalysis {
    pub destructive_patterns: Vec<DestructivePattern>,
    pub constructive_patterns: Vec<ConstructivePattern>,
    pub gottman_ratio: f64,  // Positive/Negative (ideal: 5:1)
    pub health_score: f64,   // 0.0-1.0
}
```

### 3. Power Dynamics

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PowerBalance {
    /// Equal power distribution
    Balanced {
        confidence: f64,
        indicators: Vec<String>,
    },
    
    /// User dominates conversation
    UserDominant {
        dominance_ratio: f64,  // >0.7
        indicators: Vec<String>,
    },
    
    /// Other person dominates
    OtherDominant {
        dominance_ratio: f64,
        indicators: Vec<String>,
    },
    
    /// Shifting power dynamics
    Fluctuating {
        pattern: Vec<PowerShift>,
        indicators: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerShift {
    pub timestamp: DateTime<Utc>,
    pub new_balance: PowerBalance,
    pub trigger: Option<String>,  // "conflict", "life_event", etc
}
```

---

## 🔍 RELATIONSHIP CLASSIFICATION

### Relationship Types

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Romantic partner (current or past)
    RomanticPartner {
        status: RomanticStatus,
        duration: Duration,
    },
    
    /// Family member
    Family {
        relationship: FamilyRelation,
        closeness: Closeness,
    },
    
    /// Friend
    Friend {
        closeness: Closeness,
        friendship_type: FriendshipType,
    },
    
    /// Professional contact
    Professional {
        context: ProfessionalContext,
        formality: Formality,
    },
    
    /// Ambiguous or transitioning
    Ambiguous {
        possible_types: Vec<RelationshipType>,
        ambiguity_reason: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RomanticStatus {
    Active,
    Complicated,  // On-off, unclear
    Ending,       // Recent breakup indicators
    Ended,        // Past relationship
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FamilyRelation {
    Parent,
    Child,
    Sibling,
    ExtendedFamily,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Closeness {
    VeryClose,   // Daily contact, high trust
    Close,       // Regular contact
    Moderate,    // Occasional contact
    Distant,     // Rare contact
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendshipType {
    BestFriend,
    CloseGroup,
    Casual,
    Acquaintance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessionalContext {
    Boss,
    Colleague,
    Client,
    Vendor,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Formality {
    Formal,    // "Estimado Sr.", formal language
    Casual,    // "Hola!", informal language
    Mixed,     // Context-dependent
}
```

### Classification Algorithm

```rust
impl RelationshipClassifier {
    pub fn classify(
        &self,
        digested: &DigestedData,
        emotional_nutrients: &[Nutrient],
        interest_nutrients: &[Nutrient],
    ) -> AnalysisResult<RelationshipType> {
        let start = Instant::now();
        
        // Feature extraction
        let features = self.extract_features(digested, emotional_nutrients, interest_nutrients);
        
        // Romantic indicators
        let romantic_score = self.calculate_romantic_score(&features);
        
        // Family indicators
        let family_score = self.calculate_family_score(&features);
        
        // Friend indicators
        let friend_score = self.calculate_friend_score(&features);
        
        // Professional indicators
        let professional_score = self.calculate_professional_score(&features);
        
        // Decision logic
        let scores = vec![
            (romantic_score, "romantic"),
            (family_score, "family"),
            (friend_score, "friend"),
            (professional_score, "professional"),
        ];
        
        let max_score = scores.iter().map(|(s, _)| s).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let top_type = scores.iter().find(|(s, _)| s == max_score).unwrap().1;
        
        let confidence = *max_score;
        
        let result = match top_type {
            "romantic" => {
                let status = self.infer_romantic_status(&features);
                RelationshipType::RomanticPartner {
                    status,
                    duration: features.conversation_duration,
                }
            }
            "family" => {
                let relationship = self.infer_family_relation(&features);
                let closeness = self.infer_closeness(&features);
                RelationshipType::Family { relationship, closeness }
            }
            "friend" => {
                let closeness = self.infer_closeness(&features);
                let friendship_type = self.infer_friendship_type(&features, closeness);
                RelationshipType::Friend { closeness, friendship_type }
            }
            "professional" => {
                let context = self.infer_professional_context(&features);
                let formality = self.infer_formality(&features);
                RelationshipType::Professional { context, formality }
            }
            _ => unreachable!(),
        };
        
        let duration_ms = start.elapsed().as_millis() as u64;
        
        AnalysisResult {
            result,
            confidence,
            source: AnalysisSource::Local {
                analyzer: "RelationshipClassifier".to_string(),
                duration_ms,
            },
            low_confidence_reason: if confidence < 0.7 {
                Some("Multiple relationship types possible".to_string())
            } else {
                None
            },
            metadata: HashMap::new(),
        }
    }
    
    fn calculate_romantic_score(&self, features: &ConversationFeatures) -> f64 {
        let mut score = 0.0;
        
        // Keywords: "amor", "te amo", "mi vida", etc
        if features.romantic_keywords > 5 {
            score += 0.4;
        } else if features.romantic_keywords > 0 {
            score += 0.2;
        }
        
        // Emojis: 😍, ❤️, 💕, etc
        if features.romantic_emojis > 10 {
            score += 0.3;
        } else if features.romantic_emojis > 0 {
            score += 0.15;
        }
        
        // High message frequency
        if features.messages_per_day > 10.0 {
            score += 0.2;
        } else if features.messages_per_day > 5.0 {
            score += 0.1;
        }
        
        // Positive sentiment
        if features.positive_sentiment_pct > 0.3 {
            score += 0.1;
        }
        
        score.min(1.0)
    }
    
    fn calculate_family_score(&self, features: &ConversationFeatures) -> f64 {
        let mut score = 0.0;
        
        // Keywords: "mamá", "papá", "hermano", etc
        if features.family_keywords > 0 {
            score += 0.5;
        }
        
        // Long conversation duration
        if features.conversation_duration.num_days() > 365 {
            score += 0.2;
        }
        
        // Moderate frequency (not daily)
        if features.messages_per_day < 5.0 && features.messages_per_day > 0.5 {
            score += 0.1;
        }
        
        // Supportive language
        if features.supportive_keywords > 5 {
            score += 0.2;
        }
        
        score.min(1.0)
    }
    
    // Similar for calculate_friend_score, calculate_professional_score...
}
```

---

## ⚠️ CONFLICT DETECTION

### Conflict Types

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Contradictory relationship status
    /// Example: "Te amo" followed by "No quiero verte"
    ContradictoryStatus {
        evidence: Vec<ContradictionEvidence>,
        severity: ConflictSeverity,
    },
    
    /// Love triangle or competing relationships
    /// Example: Romantic messages to multiple people
    LoveTriangle {
        participants: Vec<String>,  // Anonymized IDs
        evidence: Vec<String>,
        confidence: f64,
    },
    
    /// Abrupt change in communication pattern
    /// Example: Daily messages → silence for weeks
    AbruptChange {
        before: CommunicationPattern,
        after: CommunicationPattern,
        change_point: DateTime<Utc>,
    },
    
    /// Group conflict (multiple participants)
    GroupConflict {
        factions: Vec<Faction>,
        topic: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContradictionEvidence {
    pub timestamp1: DateTime<Utc>,
    pub message1: String,  // Anonymized
    pub sentiment1: Sentiment,
    
    pub timestamp2: DateTime<Utc>,
    pub message2: String,
    pub sentiment2: Sentiment,
    
    pub contradiction_type: ContradictionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContradictionType {
    EmotionalFlip,      // Positive → Negative
    StatusChange,       // Together → Separated
    IntentionMismatch,  // "Nos vemos mañana" → ghosting
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Low,     // Minor inconsistency
    Medium,  // Clear contradiction
    High,    // Severe conflict, potential relationship end
    Critical, // Relationship terminated
}
```

### Detection Algorithm

```rust
impl ConflictDetector {
    pub fn detect_conflicts(
        &self,
        digested: &DigestedData,
        relationship: &RelationshipType,
        emotional_nutrients: &[Nutrient],
    ) -> Vec<ConflictType> {
        let mut conflicts = Vec::new();
        
        // 1. Detect contradictory status
        if let Some(contradiction) = self.detect_contradictions(digested, emotional_nutrients) {
            conflicts.push(ConflictType::ContradictoryStatus {
                evidence: contradiction.evidence,
                severity: contradiction.severity,
            });
        }
        
        // 2. Detect abrupt changes
        if let Some(change) = self.detect_abrupt_changes(digested) {
            conflicts.push(ConflictType::AbruptChange {
                before: change.before,
                after: change.after,
                change_point: change.timestamp,
            });
        }
        
        // 3. Detect group conflicts (if multi-participant)
        if digested.participant_count > 2 {
            if let Some(group_conflict) = self.detect_group_conflict(digested) {
                conflicts.push(ConflictType::GroupConflict {
                    factions: group_conflict.factions,
                    topic: group_conflict.topic,
                });
            }
        }
        
        conflicts
    }
    
    fn detect_contradictions(
        &self,
        digested: &DigestedData,
        emotional_nutrients: &[Nutrient],
    ) -> Option<Contradiction> {
        let mut evidence = Vec::new();
        
        // Find sentiment flips
        for window in emotional_nutrients.windows(2) {
            let n1 = &window[0];
            let n2 = &window[1];
            
            let sent1 = self.parse_sentiment(&n1.value);
            let sent2 = self.parse_sentiment(&n2.value);
            
            // Detect flip: Positive → Negative within short time
            if sent1 == Sentiment::Positive && sent2 == Sentiment::Negative {
                let time_diff = n2.timestamp - n1.timestamp;
                
                if time_diff < Duration::days(7) {
                    evidence.push(ContradictionEvidence {
                        timestamp1: n1.timestamp,
                        message1: n1.context.clone().unwrap_or_default(),
                        sentiment1: sent1,
                        timestamp2: n2.timestamp,
                        message2: n2.context.clone().unwrap_or_default(),
                        sentiment2: sent2,
                        contradiction_type: ContradictionType::EmotionalFlip,
                    });
                }
            }
        }
        
        if evidence.is_empty() {
            return None;
        }
        
        let severity = if evidence.len() > 5 {
            ConflictSeverity::High
        } else if evidence.len() > 2 {
            ConflictSeverity::Medium
        } else {
            ConflictSeverity::Low
        };
        
        Some(Contradiction { evidence, severity })
    }
}
```

---

## 📈 EVOLUTION TRACKING

### Timeline Representation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipTimeline {
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub phases: Vec<RelationshipPhase>,
    pub key_events: Vec<KeyEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPhase {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub phase_type: PhaseType,
    pub characteristics: PhaseCharacteristics,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseType {
    /// Initial getting-to-know phase
    Honeymoon,
    
    /// Stable, established relationship
    Stable,
    
    /// Conflict or tension
    Turbulent,
    
    /// Declining communication/interest
    Fading,
    
    /// Relationship ended
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseCharacteristics {
    pub avg_messages_per_day: f64,
    pub sentiment_distribution: SentimentDistribution,
    pub communication_pattern: CommunicationPattern,
    pub conflicts: Vec<ConflictType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub description: String,
    pub impact: EventImpact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    FirstContact,
    FirstConflict,
    Reconciliation,
    BreakupIndicator,
    PowerShift,
    PhaseTransition,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventImpact {
    Positive,
    Negative,
    Neutral,
}
```

### Evolution Detection

```rust
impl EvolutionTracker {
    pub fn track_evolution(
        &self,
        digested: &DigestedData,
        emotional_nutrients: &[Nutrient],
    ) -> RelationshipTimeline {
        let start_date = digested.entries.first().unwrap().timestamp;
        let end_date = digested.entries.last().map(|e| e.timestamp);
        
        // Detect phases
        let phases = self.detect_phases(digested, emotional_nutrients);
        
        // Detect key events
        let key_events = self.detect_key_events(digested, &phases);
        
        RelationshipTimeline {
            start_date,
            end_date,
            phases,
            key_events,
        }
    }
    
    fn detect_phases(
        &self,
        digested: &DigestedData,
        emotional_nutrients: &[Nutrient],
    ) -> Vec<RelationshipPhase> {
        let mut phases = Vec::new();
        
        // Use sliding window to detect phase changes
        let window_size = 30; // days
        let stride = 7; // days
        
        let mut current_phase_start = digested.entries.first().unwrap().timestamp;
        let mut current_phase_type = PhaseType::Honeymoon;
        
        for window_start in (0..digested.duration_days).step_by(stride as usize) {
            let window_end = window_start + window_size;
            
            let window_entries: Vec<_> = digested.entries.iter()
                .filter(|e| {
                    let day = (e.timestamp - current_phase_start).num_days();
                    day >= window_start && day < window_end
                })
                .collect();
            
            if window_entries.is_empty() {
                continue;
            }
            
            // Calculate characteristics
            let chars = self.calculate_characteristics(&window_entries, emotional_nutrients);
            
            // Detect phase type
            let detected_phase = self.classify_phase(&chars);
            
            // Check for phase transition
            if detected_phase != current_phase_type {
                // End current phase
                let phase_end = window_entries.first().unwrap().timestamp;
                
                phases.push(RelationshipPhase {
                    start: current_phase_start,
                    end: phase_end,
                    phase_type: current_phase_type,
                    characteristics: chars.clone(),
                });
                
                // Start new phase
                current_phase_start = phase_end;
                current_phase_type = detected_phase;
            }
        }
        
        // Add final phase
        if let Some(last_entry) = digested.entries.last() {
            phases.push(RelationshipPhase {
                start: current_phase_start,
                end: last_entry.timestamp,
                phase_type: current_phase_type,
                characteristics: PhaseCharacteristics {
                    avg_messages_per_day: 0.0,
                    sentiment_distribution: SentimentDistribution::default(),
                    communication_pattern: CommunicationPattern::Unknown,
                    conflicts: vec![],
                },
            });
        }
        
        phases
    }
    
    fn classify_phase(&self, chars: &PhaseCharacteristics) -> PhaseType {
        // High frequency + positive sentiment = Honeymoon
        if chars.avg_messages_per_day > 10.0 
            && chars.sentiment_distribution.positive_pct > 0.5 {
            return PhaseType::Honeymoon;
        }
        
        // Low frequency = Fading
        if chars.avg_messages_per_day < 1.0 {
            return PhaseType::Fading;
        }
        
        // Conflicts present = Turbulent
        if !chars.conflicts.is_empty() {
            return PhaseType::Turbulent;
        }
        
        // Default = Stable
        PhaseType::Stable
    }
}
```

---

## 🛠️ IMPLEMENTATION

### Main Interface

```rust
pub struct RelationshipExtractor {
    classifier: RelationshipClassifier,
    conflict_detector: ConflictDetector,
    evolution_tracker: EvolutionTracker,
    attachment_detector: AttachmentDetector,
    communication_analyzer: CommunicationAnalyzer,
}

#[async_trait]
impl NutrientExtractor for RelationshipExtractor {
    async fn extract(
        &mut self,
        digested: &DigestedData,
        context: &HashMap<String, Vec<Nutrient>>,
    ) -> Result<Vec<Nutrient>> {
        let start = Instant::now();
        let mut nutrients = Vec::new();
        
        // Get dependencies
        let emotional_nutrients = context.get("emotional").unwrap_or(&vec![]);
        let interest_nutrients = context.get("interest").unwrap_or(&vec![]);
        
        // 1. Classify relationship
        let relationship_result = self.classifier.classify(
            digested,
            emotional_nutrients,
            interest_nutrients,
        );
        
        nutrients.push(Nutrient {
            id: Uuid::new_v4().to_string(),
            dimension: "social".to_string(),
            category: "relationship_type".to_string(),
            value: serde_json::to_string(&relationship_result.result)?,
            confidence: relationship_result.confidence,
            source: "RelationshipClassifier".to_string(),
            timestamp: Utc::now(),
            context: None,
        });
        
        // 2. Detect conflicts
        let conflicts = self.conflict_detector.detect_conflicts(
            digested,
            &relationship_result.result,
            emotional_nutrients,
        );
        
        for conflict in conflicts {
            nutrients.push(Nutrient {
                id: Uuid::new_v4().to_string(),
                dimension: "social".to_string(),
                category: "conflict".to_string(),
                value: serde_json::to_string(&conflict)?,
                confidence: 0.8,
                source: "ConflictDetector".to_string(),
                timestamp: Utc::now(),
                context: None,
            });
        }
        
        // 3. Track evolution
        let timeline = self.evolution_tracker.track_evolution(
            digested,
            emotional_nutrients,
        );
        
        nutrients.push(Nutrient {
            id: Uuid::new_v4().to_string(),
            dimension: "social".to_string(),
            category: "relationship_timeline".to_string(),
            value: serde_json::to_string(&timeline)?,
            confidence: 0.9,
            source: "EvolutionTracker".to_string(),
            timestamp: Utc::now(),
            context: None,
        });
        
        // 4. Detect attachment style (if romantic)
        if matches!(relationship_result.result, RelationshipType::RomanticPartner { .. }) {
            let attachment_result = AttachmentStyle::detect_from_conversation(
                &digested.entries,
                emotional_nutrients,
            )?;
            
            nutrients.push(Nutrient {
                id: Uuid::new_v4().to_string(),
                dimension: "social".to_string(),
                category: "attachment_style".to_string(),
                value: serde_json::to_string(&attachment_result.result)?,
                confidence: attachment_result.confidence,
                source: "AttachmentDetector".to_string(),
                timestamp: Utc::now(),
                context: None,
            });
        }
        
        // 5. Analyze communication patterns
        let comm_analysis = self.communication_analyzer.analyze(
            &digested.entries,
            emotional_nutrients,
        );
        
        nutrients.push(Nutrient {
            id: Uuid::new_v4().to_string(),
            dimension: "social".to_string(),
            category: "communication_patterns".to_string(),
            value: serde_json::to_string(&comm_analysis)?,
            confidence: 0.85,
            source: "CommunicationAnalyzer".to_string(),
            timestamp: Utc::now(),
            context: None,
        });
        
        let duration = start.elapsed();
        info!("RelationshipExtractor completed in {:?} ({} nutrients)", 
            duration, nutrients.len());
        
        Ok(nutrients)
    }
    
    fn dimension(&self) -> &str {
        "social"
    }
}
```

---

## 📚 REFERENCES

- Bowlby, J. (1969). *Attachment and Loss* (Attachment Theory)
- Gottman, J. M. (1999). *The Seven Principles for Making Marriage Work*
- Bartholomew, K. & Horowitz, L. M. (1991). *Attachment styles among young adults*

---

**Status:** 🟢 Ready for Implementation  
**Next:** Update CHECKLIST_V2.md with progress
