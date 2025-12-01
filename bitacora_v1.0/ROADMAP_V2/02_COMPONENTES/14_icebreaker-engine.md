```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/14_icebreaker-engine.md
Versión: 1.0.0
Fecha Creación: 2025-11-24
Última Actualización: 2025-11-24 12:00:00
Autor: Sistema Bitácora - En colaboración con Eduardo
Propósito: Especificación técnica del IceBreaker Engine - Sistema de construcción progresiva de relación usuario-Bitácora
Estado: 📋 ESPECIFICACIÓN TÉCNICA
Template: component_spec.yaml v1.0.0 (MTT-DSL)
Relacionado Con:
  - ROADMAP_V2/02_COMPONENTES/13_shuidao-cognitive-engine.md v1.0.0 (ShuiDao Cognitive Engine)
  - ROADMAP_V2/02_COMPONENTES/09_hubspoke-navigator.md (HubSpoke LLM routing)
  - ROADMAP_V2/02_COMPONENTES/05_telescopedb.md (TelescopeDB biographical memory)
  - ROADMAP_V2/02_COMPONENTES/06_voxeldb.md (VoxelDB template storage)
  - src/shuidao/memory_bridge.rs v1.0.0 (MemoryBridge implementation)
  - src/shuidao/response_synthesizer.rs v1.0.0 (ResponseSynthesizer implementation)
Implementa:
  - DA-033: IceBreaker - Progressive Relationship Building
  - Phase 3c: Template-driven organic user onboarding
# === FIN DATOS DE AUDITORÍA ===
```

# 🧊 IceBreaker Engine

## Progressive Relationship Building System

---

## 🎯 PROPÓSITO

### Problema que Resuelve

**Bitácora v1.0** puede mantener conversaciones inteligentes usando LLMs, pero **carece de un sistema de onboarding orgánico**:

```
Usuario nuevo: "Hola"

Bitácora responde:
- ResponseSynthesizer: CognitiveMode::Conversational
- HubSpoke → LLM: Generic greeting
- Problema: No hay construcción de relación
- Problema: No aprende sobre el usuario gradualmente
- Problema: Conversación fría e impersonal
```

**IceBreaker Engine** transforma el primer contacto en una **experiencia de construcción progresiva de relación**:

```
Usuario nuevo: "Hola"

IceBreaker piensa:
- RelationshipState: FirstContact
- Stage: Introduction
- Template: icebreaker-intro-001.yaml
- Prompt → LLM: "Genera saludo cálido y pregunta nombre..."

LLM genera: "¡Hola! Soy Bitácora, tu compañera de organización. ¿Cómo te llamas?"

Usuario: "Me llamo Eduardo"

IceBreaker extrae:
- Name: "Eduardo"
- Store → TelescopeDB: BiographicalEntry{name: "Eduardo"}
- Advance stage: NameCollection → InterestProbing
- Next template: icebreaker-interest-001.yaml (with {user_name: "Eduardo"})

LLM genera: "¡Encantada Eduardo! Me imagino que si estás aquí es porque quieres organizar mejor tus proyectos..."
```

### Por Qué Es Crítico

1. **Humanización:** Transforma Bitácora de "herramienta" a "compañera"
2. **Aprendizaje orgánico:** Datos biográficos se acumulan naturalmente, no mediante formularios
3. **Personalización temprana:** Desde la primera conversación, Bitácora se adapta al usuario
4. **Training gradual:** Cada interacción entrena tanto al usuario como a Bitácora
5. **Reducción de fricción:** Onboarding fluido sin pasos artificiales

### Diferenciador Arquitectónico Clave

**IceBreaker NO usa respuestas hardcodeadas:**

```rust
// ❌ ENFOQUE TRADICIONAL (hardcoded)
match stage {
    Introduction => "¡Hola! Soy Bitácora. ¿Cómo te llamas?",
    NameCollection => format!("Encantada {}. ¿En qué trabajas?", user_name),
}

// ✅ ENFOQUE ICEBREAKER (template-driven)
let template = load_template(stage); // "Genera saludo cálido y pregunta nombre..."
let prompt = build_prompt(template, context); // Inject {user_name}, {mood}
let response = hub_spoke.query(prompt).await?; // LLM genera pregunta única
```

**Ventajas:**
- LLM genera preguntas únicas cada vez (no repetitivas)
- Templates evolucionan con contexto acumulado
- Bitácora aprende estilo conversacional del usuario
- Escalable a múltiples idiomas/culturas sin reescribir código

### Relación con Arquitectura General

```
┌────────────────────────────────────────────────────────────┐
│                    BITÁCORA v1.0                           │
│                                                            │
│  ┌──────────────┐        ┌──────────────────────────┐    │
│  │ ShuiDao      │───────▶│  IceBreaker Engine       │    │
│  │ Cognitive    │ first  │  Phase 3c                │    │
│  │ Engine       │ contact│                          │    │
│  │              │        │ • RelationshipState      │    │
│  │ • Intention  │        │ • IceBreakerTemplate     │    │
│  │ • Cognitive  │        │ • PromptBuilder          │    │
│  │   Modes      │        │ • ResponseProcessor      │    │
│  │ • Memory     │        │ • TemplateEvolution      │    │
│  │   Bridge     │        └──────────┬───────────────┘    │
│  └──────────────┘                   │                     │
│         ↓                           ↓                     │
│  ┌──────────────────────────────────────────────┐        │
│  │     TelescopeDB (biographical memory)        │        │
│  │     - User name, interests, mood patterns    │        │
│  │                                               │        │
│  │     VoxelDB (template storage)               │        │
│  │     - Base templates + evolved templates     │        │
│  └──────────────────────────────────────────────┘        │
└────────────────────────────────────────────────────────────┘
```

IceBreaker es la **capa de construcción de relación** sobre **ShuiDao (cognición) + TelescopeDB/VoxelDB (memoria)**.

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Diagrama de Ubicación

```
Terminal Input
    ↓
┌───────────────────────────────────────────────┐
│ IntentionDetector (ShuiDao)                   │
│   → Detect first-time user                    │
│   → RelationshipState: FirstContact           │
└───────────────────────────────────────────────┘
    ↓
┌───────────────────────────────────────────────┐
│ IceBreaker Engine                             │
│   ┌─────────────────────────────────────┐    │
│   │ 1. Load current stage template      │    │
│   │ 2. Query MemoryBridge for context   │    │
│   │ 3. PromptBuilder: Inject variables  │    │
│   │ 4. HubSpoke → LLM generates question│    │
│   └─────────────────────────────────────┘    │
└───────────────────────────────────────────────┘
    ↓
User Response
    ↓
┌───────────────────────────────────────────────┐
│ ResponseProcessor                             │
│   → Extract: name, interests, sentiment       │
│   → Store → TelescopeDB                       │
│   → Advance stage if criteria met             │
│   → Enrich template with learned data         │
└───────────────────────────────────────────────┘
    ↓
┌───────────────────────────────────────────────┐
│ TemplateEvolutionSystem                       │
│   → Load base template from VoxelDB           │
│   → Inject accumulated context                │
│   → Store evolved template → VoxelDB          │
└───────────────────────────────────────────────┘
    ↓
Next IceBreaker iteration (or transition to normal mode)
```

### Interacciones con Otros Componentes

| Componente | Relación | Flujo de Datos |
|-----------|----------|----------------|
| **IntentionDetector** | Trigger | Detecta primera conversación → Activa IceBreaker |
| **MemoryBridge** | Read/Write | Query biographical data, store learned facts |
| **HubSpoke** | LLM Routing | Envía prompts generados, recibe preguntas del LLM |
| **ResponseSynthesizer** | Post-processing | Formatea respuestas después de IceBreaker |
| **TelescopeDB** | Storage | Almacena datos biográficos extraídos |
| **VoxelDB** | Templates | Lee templates base, escribe templates evolucionados |
| **CognitiveRouter** | Exit condition | Cuando ice broken → Router activa cognitive modes normales |

---

## 📋 RESPONSABILIDADES CORE

**Must Have (v1.0):**

1. **Gestionar estados de relación progresivos:**
   - `FirstContact` → `GettingToKnow` → `Familiar` → `DeepConnection`
   - Transiciones basadas en criterios verificables (nombre revelado, 3+ intereses compartidos, etc.)

2. **Cargar y ejecutar templates de IceBreaker:**
   - Leer templates desde VoxelDB (`icebreaker-intro-001.yaml`)
   - Validar estructura de template (campos requeridos)
   - Manejar templates faltantes/corruptos

3. **Construir prompts contextualizados para LLM:**
   - Inyectar variables: `{user_name}`, `{recent_topic}`, `{mood}`, `{interaction_count}`
   - Manejar variables faltantes gracefully (usar defaults)
   - Generar prompt final optimizado para LLM activo

4. **Extraer datos de respuestas de usuario:**
   - Detectar nombre (regex + NLP patterns)
   - Extraer keywords de intereses
   - Análisis de sentimiento básico (positivo/neutral/negativo)
   - Detectar tópicos principales

5. **Almacenar datos biográficos en TelescopeDB:**
   - Crear/actualizar `BiographicalEntry`
   - Evitar duplicaciones (merge data si ya existe entrada)
   - Timestamp de cada dato extraído

6. **Detectar ice broken condition:**
   - Criterios: Nombre revelado + 2+ intereses + sentimiento positivo + 3+ interacciones
   - Transición suave a modo conversacional normal
   - Notificar a CognitiveRouter para cambio de modo

7. **Evolucionar templates con contexto acumulado:**
   - Enriquecer template base con datos aprendidos
   - Almacenar evolved template en VoxelDB (versionado)
   - Rollback capability si template evolucionado falla

**Nice to Have (v2.0):**

8. **Multi-idioma support:** Templates en español/inglés/otros
9. **Cultural adaptation:** Templates diferentes según región detectada
10. **Sentiment-based branching:** Ajustar stage progression según humor usuario
11. **Recovery from abandonment:** Si usuario abandona IceBreaker, resumir después

---

## 🗂️ ESTRUCTURAS DE DATOS

### Structs Principales

```rust
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Motor principal del IceBreaker
pub struct IceBreakerEngine {
    /// Estado actual de la relación usuario-Bitácora
    relationship_state: RelationshipState,
    
    /// Stage actual dentro del estado
    current_stage: IceBreakerStage,
    
    /// Bridge a TelescopeDB/VoxelDB
    memory_bridge: Arc<MemoryBridge>,
    
    /// Router a LLMs
    hub_spoke: Arc<HubSpoke>,
    
    /// Templates cargados en memoria
    templates: HashMap<IceBreakerStage, IceBreakerTemplate>,
    
    /// Contador de interacciones en esta sesión
    interaction_count: usize,
    
    /// Datos extraídos en esta sesión
    extracted_data: ExtractedUserData,
}

/// Estados de relación (macro)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipState {
    /// Primera conversación, sin datos biográficos
    FirstContact,
    
    /// 1-5 interacciones, datos básicos conocidos
    GettingToKnow,
    
    /// 5-20 interacciones, preferencias conocidas
    Familiar,
    
    /// 20+ interacciones, confianza establecida
    DeepConnection,
}

/// Stages dentro del IceBreaker (micro)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IceBreakerStage {
    /// Saludo inicial y presentación de Bitácora
    Introduction,
    
    /// Obtener nombre del usuario
    NameCollection,
    
    /// Explorar intereses, proyectos, motivaciones
    InterestProbing,
    
    /// Transición suave a modo conversacional normal
    Transition,
}

/// Template para generar prompts (NO respuestas hardcodeadas)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceBreakerTemplate {
    /// Identificador único
    pub id: String,
    
    /// Stage al que pertenece
    pub stage: IceBreakerStage,
    
    /// Prompt template para LLM (con variables)
    /// Ejemplo: "Genera saludo cálido como Bitácora y pregunta nombre. Usuario previo: {recent_topic}"
    pub prompt_template: String,
    
    /// Variables que este template requiere
    /// Ejemplo: ["user_name", "recent_topic", "mood"]
    pub context_slots: Vec<String>,
    
    /// Criterios para considerar este stage completado
    pub success_criteria: IceBreakerCriteria,
    
    /// Metadata
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
}

/// Criterios de éxito para avanzar de stage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceBreakerCriteria {
    /// Usuario reveló su nombre
    pub user_revealed_name: bool,
    
    /// Mínimo de intereses detectados
    pub min_interests_detected: usize,
    
    /// Sentimiento debe ser al menos neutral
    pub min_sentiment_level: SentimentLevel,
    
    /// Mínimo de interacciones en este stage
    pub min_interactions: usize,
}

/// Datos extraídos de respuestas del usuario
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractedUserData {
    pub name: Option<String>,
    pub interests: Vec<String>,
    pub recent_topics: Vec<String>,
    pub sentiment_history: Vec<SentimentLevel>,
    pub interaction_count: usize,
}

/// Nivel de sentimiento detectado
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentimentLevel {
    VeryNegative,
    Negative,
    Neutral,
    Positive,
    VeryPositive,
}
```

---

## 🔌 API PÚBLICA

### Constructor y Lifecycle

```rust
impl IceBreakerEngine {
    /// Crea nuevo IceBreaker engine
    pub fn new(
        memory_bridge: Arc<MemoryBridge>,
        hub_spoke: Arc<HubSpoke>,
    ) -> Result<Self, IceBreakerError> {
        let templates = Self::load_templates(&memory_bridge)?;
        
        Ok(Self {
            relationship_state: RelationshipState::FirstContact,
            current_stage: IceBreakerStage::Introduction,
            memory_bridge,
            hub_spoke,
            templates,
            interaction_count: 0,
            extracted_data: ExtractedUserData::default(),
        })
    }
    
    /// Carga templates desde VoxelDB
    fn load_templates(
        memory_bridge: &Arc<MemoryBridge>,
    ) -> Result<HashMap<IceBreakerStage, IceBreakerTemplate>, IceBreakerError> {
        // Query VoxelDB for templates with prefix "icebreaker-"
        // Parse YAML into IceBreakerTemplate structs
        // Validate required fields
        todo!()
    }
}
```

### Core Methods

```rust
impl IceBreakerEngine {
    /// Genera el próximo prompt para el LLM basado en stage actual + contexto
    pub async fn get_current_prompt(&self) -> Result<String, IceBreakerError> {
        let template = self.templates.get(&self.current_stage)
            .ok_or(IceBreakerError::TemplateNotFound(self.current_stage.clone()))?;
        
        // Build context from MemoryBridge
        let context = self.build_context().await?;
        
        // Inject variables into template
        let prompt = PromptBuilder::new()
            .template(&template.prompt_template)
            .context(context)
            .build()?;
        
        Ok(prompt)
    }
    
    /// Procesa respuesta del usuario: extrae datos + avanza stage si criterios cumplidos
    pub async fn process_user_response(
        &mut self,
        user_input: &str,
    ) -> Result<ProcessResult, IceBreakerError> {
        // Extract data from user input
        let extracted = ResponseProcessor::new()
            .extract_name(user_input)
            .extract_interests(user_input)
            .analyze_sentiment(user_input)
            .extract_topics(user_input)
            .process()?;
        
        // Merge with existing data
        self.extracted_data.merge(extracted);
        self.interaction_count += 1;
        
        // Store in TelescopeDB
        self.store_extracted_data().await?;
        
        // Check if stage criteria met
        if self.is_stage_complete() {
            self.advance_stage()?;
        }
        
        // Check if ice broken
        let ice_broken = self.is_ice_broken();
        
        Ok(ProcessResult {
            stage_advanced: self.current_stage,
            ice_broken,
            extracted_data: self.extracted_data.clone(),
        })
    }
    
    /// Verifica si los criterios del stage actual están cumplidos
    fn is_stage_complete(&self) -> bool {
        let template = match self.templates.get(&self.current_stage) {
            Some(t) => t,
            None => return false,
        };
        
        let criteria = &template.success_criteria;
        
        // Check all criteria
        let name_ok = !criteria.user_revealed_name || self.extracted_data.name.is_some();
        let interests_ok = self.extracted_data.interests.len() >= criteria.min_interests_detected;
        let sentiment_ok = self.check_sentiment_level(&criteria.min_sentiment_level);
        let interactions_ok = self.interaction_count >= criteria.min_interactions;
        
        name_ok && interests_ok && sentiment_ok && interactions_ok
    }
    
    /// Avanza al siguiente stage
    fn advance_stage(&mut self) -> Result<(), IceBreakerError> {
        use IceBreakerStage::*;
        
        self.current_stage = match self.current_stage {
            Introduction => NameCollection,
            NameCollection => InterestProbing,
            InterestProbing => Transition,
            Transition => return Err(IceBreakerError::AlreadyTransitioned),
        };
        
        // Update relationship state if needed
        self.update_relationship_state();
        
        Ok(())
    }
    
    /// Verifica si la relación está suficientemente establecida (ice broken)
    pub fn is_ice_broken(&self) -> bool {
        // Criteria: Name + 2+ interests + positive sentiment + 3+ interactions
        let has_name = self.extracted_data.name.is_some();
        let enough_interests = self.extracted_data.interests.len() >= 2;
        let positive_sentiment = self.check_sentiment_level(&SentimentLevel::Positive);
        let enough_interactions = self.interaction_count >= 3;
        
        has_name && enough_interests && positive_sentiment && enough_interactions
    }
}
```

### Template Evolution

```rust
impl IceBreakerEngine {
    /// Evoluciona template con contexto acumulado y lo guarda en VoxelDB
    pub async fn evolve_template(
        &self,
        stage: IceBreakerStage,
    ) -> Result<IceBreakerTemplate, IceBreakerError> {
        let base_template = self.templates.get(&stage)
            .ok_or(IceBreakerError::TemplateNotFound(stage.clone()))?;
        
        // Enrich prompt with learned context
        let evolved_prompt = self.enrich_prompt_with_context(&base_template.prompt_template);
        
        let evolved = IceBreakerTemplate {
            id: format!("{}-evolved-{}", base_template.id, Utc::now().timestamp()),
            prompt_template: evolved_prompt,
            version: format!("{}.1", base_template.version),
            last_used: Some(Utc::now()),
            ..base_template.clone()
        };
        
        // Store in VoxelDB
        self.memory_bridge.store_template(evolved.clone()).await?;
        
        Ok(evolved)
    }
    
    /// Enriquece prompt con contexto aprendido
    fn enrich_prompt_with_context(&self, base_prompt: &str) -> String {
        let mut enriched = base_prompt.to_string();
        
        // Add user-specific context
        if let Some(name) = &self.extracted_data.name {
            enriched.push_str(&format!("\nUsuario conocido: {}", name));
        }
        
        if !self.extracted_data.interests.is_empty() {
            enriched.push_str(&format!(
                "\nIntereses previos: {}",
                self.extracted_data.interests.join(", ")
            ));
        }
        
        enriched
    }
}
```

### Helper Types

```rust
/// Resultado del procesamiento de respuesta
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub stage_advanced: IceBreakerStage,
    pub ice_broken: bool,
    pub extracted_data: ExtractedUserData,
}

/// Builder para construir prompts con inyección de variables
pub struct PromptBuilder {
    template: String,
    context: HashMap<String, String>,
}

impl PromptBuilder {
    pub fn new() -> Self {
        Self {
            template: String::new(),
            context: HashMap::new(),
        }
    }
    
    pub fn template(mut self, template: &str) -> Self {
        self.template = template.to_string();
        self
    }
    
    pub fn context(mut self, context: HashMap<String, String>) -> Self {
        self.context = context;
        self
    }
    
    /// Construye prompt final reemplazando variables
    pub fn build(self) -> Result<String, IceBreakerError> {
        let mut result = self.template;
        
        for (key, value) in self.context {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
        }
        
        // Check for unreplaced variables
        if result.contains('{') && result.contains('}') {
            return Err(IceBreakerError::UnresolvedVariable);
        }
        
        Ok(result)
    }
}

/// Procesador de respuestas de usuario
pub struct ResponseProcessor {
    input: String,
}

impl ResponseProcessor {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
    
    pub fn extract_name(mut self, input: &str) -> Self {
        self.input = input.to_string();
        self
    }
    
    pub fn extract_interests(self, _input: &str) -> Self {
        // TODO: Implement NLP interest extraction
        self
    }
    
    pub fn analyze_sentiment(self, _input: &str) -> Self {
        // TODO: Implement sentiment analysis
        self
    }
    
    pub fn extract_topics(self, _input: &str) -> Self {
        // TODO: Implement topic extraction
        self
    }
    
    pub fn process(self) -> Result<ExtractedUserData, IceBreakerError> {
        // TODO: Combine all extractions
        Ok(ExtractedUserData::default())
    }
}
```

---

## ⚙️ IMPLEMENTACIÓN INTERNA

### Algoritmos Clave

#### 1. Name Detection

```rust
fn detect_name(input: &str) -> Option<String> {
    // Pattern 1: "Me llamo X" / "Mi nombre es X" / "Soy X"
    let patterns = [
        regex::Regex::new(r"(?i)me llamo\s+([A-ZÁÉÍÓÚ][a-záéíóúñ]+)").unwrap(),
        regex::Regex::new(r"(?i)mi nombre es\s+([A-ZÁÉÍÓÚ][a-záéíóúñ]+)").unwrap(),
        regex::Regex::new(r"(?i)soy\s+([A-ZÁÉÍÓÚ][a-záéíóúñ]+)").unwrap(),
    ];
    
    for pattern in &patterns {
        if let Some(captures) = pattern.captures(input) {
            if let Some(name_match) = captures.get(1) {
                return Some(name_match.as_str().to_string());
            }
        }
    }
    
    None
}
```

#### 2. Interest Extraction (Basic Keyword Matching)

```rust
fn extract_interests(input: &str) -> Vec<String> {
    let keywords = [
        "programación", "música", "arte", "deporte", "ciencia",
        "tecnología", "lectura", "cine", "viajes", "cocina",
    ];
    
    keywords.iter()
        .filter(|keyword| input.to_lowercase().contains(*keyword))
        .map(|s| s.to_string())
        .collect()
}
```

#### 3. Sentiment Analysis (Simple Rule-Based)

```rust
fn analyze_sentiment(input: &str) -> SentimentLevel {
    let positive_words = ["bien", "genial", "excelente", "perfecto", "encantado"];
    let negative_words = ["mal", "terrible", "horrible", "frustrado", "molesto"];
    
    let positive_count = positive_words.iter()
        .filter(|w| input.to_lowercase().contains(*w))
        .count();
    
    let negative_count = negative_words.iter()
        .filter(|w| input.to_lowercase().contains(*w))
        .count();
    
    match (positive_count, negative_count) {
        (p, n) if p > n && p >= 2 => SentimentLevel::VeryPositive,
        (p, n) if p > n => SentimentLevel::Positive,
        (p, n) if n > p && n >= 2 => SentimentLevel::VeryNegative,
        (p, n) if n > p => SentimentLevel::Negative,
        _ => SentimentLevel::Neutral,
    }
}
```

### Trade-offs de Diseño

| Decisión | Pro | Con | Justificación |
|----------|-----|-----|---------------|
| **Templates generan prompts (no respuestas)** | Flexibilidad infinita del LLM, no repetitivo | Mayor latencia (LLM call), costo API | User experience > cost, v1.0 usa LLMs pequeños/locales |
| **Stage progression criteria hardcoded** | Predecible, testable | No adaptativo | v1.0 simple, v2.0 puede usar ML para criterios dinámicos |
| **Basic NLP (regex + keywords)** | Rápido (<1ms), sin dependencias pesadas | Menos preciso que spaCy/transformers | Suficiente para v1.0, optimizable después |
| **In-memory template cache** | Rápido acceso | No persiste entre reinicios | VoxelDB es source of truth, cache rebuild es barato |

---

## 🔗 DEPENDENCIAS

### Componentes Bitácora

| Componente | Versión | Uso |
|-----------|---------|-----|
| **MemoryBridge** | v1.0.0 | Query biographical data, store extracted facts |
| **HubSpoke** | v1.0.0 | Routing prompts to active LLM |
| **TelescopeDB** | v1.0.0 | Store BiographicalEntry with user data |
| **VoxelDB** | v1.0.0 | Load base templates, store evolved templates |
| **ResponseSynthesizer** | v1.0.0 | Format IceBreaker responses (post-LLM) |

### Crates Externos

```toml
[dependencies]
# Core
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"

# Date/Time
chrono = "0.4"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# NLP básico
regex = "1.10"

# Optional (v2.0)
# spacy-rs = "0.1"  # Para NLP avanzado
# sentencepiece = "0.1"  # Tokenization
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

| Operación | Target | Actual | Status | Notas |
|-----------|--------|--------|--------|-------|
| `get_current_prompt()` | <10ms | TBD | ⏸️ | Sin LLM call, solo template loading + injection |
| `process_user_response()` | <50ms | TBD | ⏸️ | Incluye NLP extraction + TelescopeDB write |
| `extract_name()` | <5ms | TBD | ⏸️ | Regex-based, debe ser instant |
| `analyze_sentiment()` | <5ms | TBD | ⏸️ | Rule-based, no ML |
| `evolve_template()` | <100ms | TBD | ⏸️ | VoxelDB write puede ser async |
| **End-to-End IceBreaker cycle** | <300ms | TBD | ⏸️ | Excluding LLM latency (HubSpoke responsibility) |

**Performance Crítica:**
- IceBreaker debe sentirse **instantáneo** para el usuario
- LLM latency es inevitable, pero pre/post processing debe ser <100ms total
- Template caching crítico (cargar una vez al inicio)

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_name_detection_spanish() {
        assert_eq!(detect_name("Me llamo Eduardo"), Some("Eduardo".to_string()));
        assert_eq!(detect_name("Mi nombre es María"), Some("María".to_string()));
        assert_eq!(detect_name("Soy Carlos"), Some("Carlos".to_string()));
        assert_eq!(detect_name("Hola"), None);
    }
    
    #[test]
    fn test_interest_extraction() {
        let interests = extract_interests("Me gusta la programación y la música");
        assert!(interests.contains(&"programación".to_string()));
        assert!(interests.contains(&"música".to_string()));
    }
    
    #[test]
    fn test_sentiment_analysis() {
        assert_eq!(analyze_sentiment("Esto está genial y perfecto"), SentimentLevel::VeryPositive);
        assert_eq!(analyze_sentiment("Estoy bien"), SentimentLevel::Positive);
        assert_eq!(analyze_sentiment("Esto es terrible"), SentimentLevel::Negative);
    }
    
    #[test]
    fn test_prompt_builder() {
        let prompt = PromptBuilder::new()
            .template("Hola {user_name}, hablemos de {topic}")
            .context(HashMap::from([
                ("user_name".to_string(), "Eduardo".to_string()),
                ("topic".to_string(), "Rust".to_string()),
            ]))
            .build()
            .unwrap();
        
        assert_eq!(prompt, "Hola Eduardo, hablemos de Rust");
    }
    
    #[test]
    fn test_stage_progression() {
        let mut engine = IceBreakerEngine::new_mock();
        
        assert_eq!(engine.current_stage, IceBreakerStage::Introduction);
        engine.advance_stage().unwrap();
        assert_eq!(engine.current_stage, IceBreakerStage::NameCollection);
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_complete_icebreaker_flow() {
    let memory_bridge = Arc::new(MemoryBridge::new_stub());
    let hub_spoke = Arc::new(HubSpoke::new_mock());
    let mut engine = IceBreakerEngine::new(memory_bridge, hub_spoke).unwrap();
    
    // Stage 1: Introduction
    let prompt1 = engine.get_current_prompt().await.unwrap();
    assert!(prompt1.contains("Bitácora"));
    
    // User reveals name
    engine.process_user_response("Me llamo Eduardo").await.unwrap();
    assert_eq!(engine.extracted_data.name, Some("Eduardo".to_string()));
    assert_eq!(engine.current_stage, IceBreakerStage::NameCollection);
    
    // Stage 2: Interest probing
    let prompt2 = engine.get_current_prompt().await.unwrap();
    assert!(prompt2.contains("Eduardo")); // Context injected
    
    // User shares interests
    engine.process_user_response("Me gusta programar y la música").await.unwrap();
    assert!(engine.extracted_data.interests.contains(&"programación".to_string()));
    
    // Ice broken?
    if engine.interaction_count >= 3 {
        assert!(engine.is_ice_broken());
    }
}
```

### Property-Based Tests (v2.0)

```rust
#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn name_detection_never_panics(input in "\\PC*") {
            let _ = detect_name(&input);
        }
        
        #[test]
        fn sentiment_analysis_always_returns_valid_level(input in "\\PC*") {
            let level = analyze_sentiment(&input);
            assert!(matches!(level, SentimentLevel::_));
        }
    }
}
```

### Golden Tests

- **Golden template files:** `tests/golden/icebreaker-intro-001.yaml`
- **Golden conversation flows:** `tests/golden/conversation-flow-happy-path.txt`
- Compare generated prompts against expected outputs

---

## ⚠️ MANEJO DE ERRORES

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IceBreakerError {
    #[error("Template not found for stage: {0:?}")]
    TemplateNotFound(IceBreakerStage),
    
    #[error("Template missing required field: {0}")]
    TemplateMissingField(String),
    
    #[error("Unresolved variable in prompt template")]
    UnresolvedVariable,
    
    #[error("Already transitioned to normal mode")]
    AlreadyTransitioned,
    
    #[error("Failed to extract data from user input")]
    ExtractionFailed,
    
    #[error("Memory bridge error: {0}")]
    MemoryBridge(#[from] crate::shuidao::memory_bridge::MemoryBridgeError),
    
    #[error("HubSpoke error: {0}")]
    HubSpoke(#[from] crate::hubspoke::HubSpokeError),
    
    #[error("Template parsing error: {0}")]
    TemplateParse(#[from] serde_yaml::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type IceBreakerResult<T> = Result<T, IceBreakerError>;
```

### Error Recovery Strategies

| Error | Recovery | User Impact |
|-------|----------|-------------|
| `TemplateNotFound` | Use fallback hardcoded prompt | Conversación continúa (degraded) |
| `UnresolvedVariable` | Replace with placeholder "..." | LLM puede manejar context incompleto |
| `ExtractionFailed` | Continue without storing data | No impacta flow, solo data quality |
| `MemoryBridge` error | Log + continue | No bloquea conversación |
| `HubSpoke` error | Propagate up (crítico) | Usuario ve error LLM |

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **00_VISION/08_shuidao-cognitive-architecture.md** - Visión cognitiva que justifica IceBreaker
- **01_ARQUITECTURA/12_shuidao-intention-detection.md** - Cómo IntentionDetector activa IceBreaker
- **02_COMPONENTES/13_shuidao-cognitive-engine.md** - ShuiDao parent system
- **02_COMPONENTES/05_telescopedb.md** - Biographical storage spec
- **02_COMPONENTES/06_voxeldb.md** - Template storage spec
- **02_COMPONENTES/09_hubspoke-navigator.md** - LLM routing

### Decisiones Arquitectónicas

- **DA-033:** IceBreaker - Progressive Relationship Building (este documento)
- **DA-032:** ShuiDao - Intention-Oriented Cognitive Architecture
- **DA-015:** TelescopeDB - Biographical Memory Storage
- **DA-016:** VoxelDB - Template and Pattern Storage

### Código de Referencia

- **src/shuidao/memory_bridge.rs** - Interface to TelescopeDB/VoxelDB
- **src/shuidao/response_synthesizer.rs** - Post-processing de responses
- **src/hubspoke/navigator.rs** - LLM routing implementation
- **examples/test_conversation_e2e.rs** - E2E conversation flow

### Papers/Artículos

- "Progressive Disclosure in UX Design" - Nielsen Norman Group
- "Conversational AI: Principles and Practices" - Stanford NLP
- "Template-based vs Generation-based Dialogue Systems" - ACL 2023

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Esta Semana)

**Prioridad 1: Core IceBreaker (2-3h)**
- [ ] Crear `src/shuidao/icebreaker_engine.rs`
- [ ] Implementar structs: `IceBreakerEngine`, `IceBreakerTemplate`, estados/stages
- [ ] Implementar `get_current_prompt()` y `process_user_response()`
- [ ] Tests unitarios: stage progression, name detection, sentiment

**Prioridad 2: Template Files (1h)**
- [ ] Crear `templates/icebreaker-intro-001.yaml`
- [ ] Crear `templates/icebreaker-name-001.yaml`
- [ ] Crear `templates/icebreaker-interest-001.yaml`
- [ ] Crear `templates/icebreaker-transition-001.yaml`
- [ ] Validar YAML parsing

**Prioridad 3: Integration (1-2h)**
- [ ] Integrar IceBreaker en `examples/test_conversation_e2e.rs`
- [ ] Detectar first-time user (no profile in TelescopeDB)
- [ ] If first-time → IceBreakerEngine.start()
- [ ] Display stage progression in terminal
- [ ] Test complete flow

**Prioridad 4: Documentation (30min)**
- [ ] Update `02_COMPONENTES/README.md` (add component 14)
- [ ] Update `CHECKLIST_V2.md` (mark MemoryBridge/ResponseSynthesizer done, add IceBreaker tasks)
- [ ] Update `CHECKLIST_TREE_V2.md` (visual progress)
- [ ] Execute `./scripts/timestamp.sh` and `./scripts/backup.sh`

### Mejoras v2.0 (Futuro)

**Enhanced NLP (Complejidad: 🟡 MEDIA)**
- Replace regex name detection with spaCy NER
- Use BERT embeddings for interest classification
- Sentiment analysis con transformers (RoBERTa-sentiment)

**Multi-Idioma (Complejidad: 🟡 MEDIA)**
- Template variants: `icebreaker-intro-001-es.yaml`, `icebreaker-intro-001-en.yaml`
- Language detection from first user message
- Dynamic template loading por idioma

**Adaptive Criteria (Complejidad: 🔴 ALTA)**
- ML model learns optimal stage progression per user type
- Dynamic criteria adjustment based on conversation flow
- A/B testing different stage sequences

**Recovery System (Complejidad: 🟢 BAJA)**
- If user abandons IceBreaker mid-flow, store state
- Next session: "Hola de nuevo Eduardo, ¿continuamos nuestra conversación?"
- Resume from last stage

---

**Estado:** 📋 ESPECIFICACIÓN TÉCNICA  
**Complejidad:** 🟡 MEDIA  
**Prioridad:** 🟡 ALTA (Post-MVP, pre-v1.1)

---

*Generado: 2025-11-24*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec v1.0.0*
