```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/11_mtt-dsl-templates.md
Versión: 1.0.0
Fecha Creación: 2025-10-26
Autor: Sistema Bitácora - Documentación MTT-DSL
Propósito: Especificación componente MTT-DSL Templates (Sistema de templates estructurales)
Estado: 📋 ESPECIFICACIÓN
Relacionado Con:
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md
  - ROADMAP_V2/07_TEMPLATES/ (Templates experimentales)
  - FUSION_BAYESIANA/05_MTT_DSL_TEMPLATES.md
  - templates/mtt/ (Templates productivos)
Implementa:
  - DA-016: MTT-DSL ≠ Música (Desacoplamiento conceptual)
  - DA-017: Templates como LEGO (Bloques de construcción reutilizables)
  - DA-018: HarmonyEngine opcional (No requerido para MTT-DSL)
# === FIN DATOS DE AUDITORÍA ===
```

# 📋 MTT-DSL TEMPLATES - Sistema de Templates Estructurales

---

## 🎯 PROPÓSITO

El **sistema MTT-DSL** (Meta Template Transformation - Domain Specific Language) es el componente de Bitácora responsable de **estructurar respuestas coherentes** mediante templates configurables que se activan según contexto, intención y tipo de tarea del usuario.

### El Problema que Resuelve

**Escenario sin MTT-DSL:**
```
Usuario: "Ayúdame con debugging de async Rust"

LLM (sin estructura):
"Bueno, el async en Rust puede ser complicado... primero deberías 
revisar... ah, y también asegúrate de... por cierto, los lifetimes...
y no olvides... mmmm, ¿qué más?... bueno, espero que esto ayude..."

Resultado: 
❌ Respuesta desorganizada
❌ Sin metodología clara
❌ Usuario confundido
❌ No reproducible
```

**Con MTT-DSL (template debugging_deep_dive):**
```
Usuario: "Ayúdame con debugging de async Rust"

Sistema detecta:
  ├─ Intención: debugging (technical)
  ├─ Complejidad: alta (async)
  ├─ Trigger: "debugging" keyword
  └─ Selecciona: debugging_deep_dive.yaml

Template genera respuesta estructurada:

╔════════════════════════════════════════════╗
║ 🔍 DEBUGGING SISTEMÁTICO - ASYNC RUST      ║
╠════════════════════════════════════════════╣
║                                            ║
║ 1️⃣  IDENTIFICACIÓN DEL PROBLEMA            ║
║    • Descripción del error                 ║
║    • Contexto de ejecución                 ║
║    • Código relevante                      ║
║                                            ║
║ 2️⃣  ANÁLISIS DE CAUSAS RAÍZ                ║
║    • Hipótesis A: Lifetime issue           ║
║    • Hipótesis B: Pinning problem          ║
║    • Hipótesis C: Executor mismatch        ║
║                                            ║
║ 3️⃣  ESTRATEGIA DE RESOLUCIÓN               ║
║    ✓ Paso 1: Simplificar future            ║
║    ✓ Paso 2: Validar async runtime         ║
║    ✓ Paso 3: Aislar componente             ║
║                                            ║
║ 4️⃣  IMPLEMENTACIÓN + TESTS                 ║
║    ```rust                                 ║
║    // Código corregido                     ║
║    ```                                     ║
║    ✓ Unit test que valida fix              ║
║                                            ║
║ 5️⃣  PREVENCIÓN FUTURA                      ║
║    • Patterns a seguir                     ║
║    • Red flags a evitar                    ║
╚════════════════════════════════════════════╝

Resultado:
✅ Metodología clara y reproducible
✅ Usuario sigue pasos sistemáticos
✅ Aprende el proceso (no solo la solución)
✅ Misma estructura para futuros bugs
```

### Por Qué es Crítico

1. **Consistencia:** Todas las respuestas de debugging siguen la misma estructura probada
2. **Educación:** Usuario aprende METODOLOGÍA, no solo soluciones puntuales
3. **Escalabilidad:** Crear 18 templates cubre 90% de casos de uso
4. **Local-First:** 90% de queries se procesan localmente sin llamar LLM (ahorro de $$$ y latencia)
5. **Personalización:** Templates se adaptan a nivel de expertise del usuario
6. **Medición:** Cada template tiene métricas de efectividad (% de satisfacción)

### Desacoplamiento Crítico: MTT-DSL ≠ Música

⚠️ **IMPORTANTE:** MTT-DSL es un sistema de **TEMPLATES ESTRUCTURALES**, NO un sistema musical.

```
CONCEPTOS SEPARADOS:

MTT-DSL Templates (CORE - ACTIVO v1.0):
  ├─ Estructura de respuestas
  ├─ Bloques de construcción reutilizables
  ├─ Triggers basados en intención
  ├─ Personalidades configurables
  └─ CERO dependencia de música

HarmonyEngine (OPCIONAL - INACTIVO v1.0):
  ├─ Conversión Info ↔ Música
  ├─ Frecuencias Solfeggio
  ├─ Soundtracks asociados
  └─ Feature futuro (v2.0+)
```

**Decisiones Arquitectónicas (DA-016, DA-017, DA-018):**
- **DA-016:** MTT-DSL NO requiere música para funcionar
- **DA-017:** Templates son bloques LEGO, no partituras musicales
- **DA-018:** HarmonyEngine es sistema separado opcional

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
PIPELINE DE PROCESAMIENTO DE QUERIES:

Usuario: "Explicame async Rust"
    ↓
┌─────────────────────────────────────────────────┐
│ SENSORY ENGINE (Input Processing)               │
│ └─> NormalizedInput { text, modality }         │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ CONTEXT TOKEN 7D (Dimensional Analysis)         │
│ └─> ContextTensor7D {                           │
│       semantic: 0.75,    // Complejidad media   │
│       intentional: 0.85, // Claro: aprender     │
│       temporal: 0.40,    // No urgente          │
│       ...                                       │
│     }                                           │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ ★★★ MTT-DSL ENGINE (TÚ ESTÁS AQUÍ) ★★★         │
│                                                 │
│ PASO 1: Template Matching                      │
│  ├─ Analizar keywords: "async", "Rust"         │
│  ├─ Detectar intención: technical/learning     │
│  ├─ Calcular complejidad: media                │
│  └─ Trigger match: "technical_explanation"     │
│                                                 │
│ PASO 2: Template Selection                     │
│  Query VoxelDB:                                 │
│  └─> find_templates_spatial(ctx7d, radius=0.15)│
│       Resultados:                               │
│       1. debugging_deep_dive (score: 0.92) ✅   │
│       2. code_review (score: 0.78)              │
│       3. architectural_design (score: 0.65)     │
│                                                 │
│ PASO 3: Template Loading                       │
│  Load: templates/mtt/technical/debugging.yaml   │
│  Parse YAML → TemplateDefinition                │
│                                                 │
│ PASO 4: Context Injection                      │
│  ├─ Inject: User expertise level (Senior)      │
│  ├─ Inject: Historical context (TelescopeDB)   │
│  ├─ Inject: Similar past solutions             │
│  └─ Apply: Template personality (technical)    │
│                                                 │
│ PASO 5: Response Generation                    │
│  ├─ Fill sections from template structure      │
│  ├─ Generate code examples                     │
│  ├─ Add validation steps                       │
│  └─ Format output                              │
│                                                 │
│ DECISIÓN: ProcessLocally vs OrchestrateLLM     │
│  ├─ Si query simple + template match > 0.8     │
│  │   → ProcessLocally (90% casos) ✅           │
│  │   → Respuesta en <100ms, $0.00              │
│  └─ Si query complejo + no hay template        │
│      → OrchestrateLLM (10% casos)              │
│      → HubSpoke → Wizard → LLM                 │
│      → Respuesta en ~2s, $0.02                 │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ OUTPUT: Structured Response                     │
│ - Formatted según template                      │
│ - Metadata: template_used, effectiveness_score  │
│ - Tracking: latency, cost (si LLM usado)        │
└─────────────────────────────────────────────────┘
    ↓
Usuario: "¡Perfecto! Exactamente lo que necesitaba" 🎯
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito |
|------------|-----------|-----------|
| **VoxelDB** | Consulta ↔ | Buscar templates por similaridad espacial (CTX7D → Vec3) |
| **Context Token 7D** | Entrada ← | Análisis dimensional para template matching |
| **TelescopeDB** | Consulta ↔ | Historial de usuario (expertise, templates usados antes) |
| **HubSpoke Navigator** | Salida → | Enviar query a LLM si template no cubre (10% casos) |
| **SENSORY ENGINE** | Entrada ← | Input normalizado (texto, voz, visual) |

---

## 📋 RESPONSABILIDADES CORE

El sistema MTT-DSL **DEBE**:

1. **Parsear Templates YAML:**
   - Cargar templates desde `templates/mtt/`
   - Validar estructura según schema MTT-DSL
   - Cachear templates parseados en memoria
   - Detectar errores de sintaxis y reportar

2. **Matching Inteligente de Templates:**
   - Analizar keywords del query (`async`, `debugging`, `help`)
   - Detectar intención (technical, creative, analytical, emotional, collaborative)
   - Calcular score de similaridad con cada template (0.0-1.0)
   - Seleccionar template con score más alto (threshold > 0.75)

3. **Procesamiento Local de Queries Simples:**
   - Si template match > 0.8 → procesar localmente (NO llamar LLM)
   - Rellenar secciones del template con contexto del usuario
   - Generar respuesta estructurada coherente
   - **Target:** 90% de queries procesadas localmente

4. **Inyección de Contexto:**
   - Consultar TelescopeDB: historial de usuario, expertise level
   - Consultar VoxelDB: soluciones similares previas
   - Aplicar personalidad del template (tone, style, depth)
   - Adaptar lenguaje según nivel del usuario (beginner, intermediate, senior)

5. **Delegación a LLM cuando Necesario:**
   - Si template match < 0.75 → delegar a HubSpoke
   - Si query es muy complejo/novedoso → usar LLM
   - Registrar decisión (local vs LLM) para métricas
   - **Target:** ≤10% de queries delegadas a LLM

6. **Medición de Efectividad:**
   - Tracking de templates usados (counter por template)
   - User feedback (👍/👎) por respuesta generada
   - Effectiveness score por template:
     ```
     effectiveness = (👍 / (👍 + 👎)) * completeness_factor
     ```
   - Deprecar templates con score < 0.60

7. **Gestión del Catálogo de Templates:**
   - Cargar 18 templates de 6 categorías
   - Actualización en caliente (sin reiniciar sistema)
   - Versionado de templates (v1.0, v1.1, etc.)
   - Migración automática a nuevas versiones

---

## 🗂️ ESTRUCTURAS DE DATOS

```rust
// src/templates/mtt_dsl.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Motor principal MTT-DSL
pub struct MTTEngine {
    /// Templates cargados y parseados
    templates: HashMap<String, TemplateDefinition>,
    
    /// Cache de template matching (query hash → template_id)
    match_cache: lru::LruCache<u64, String>,
    
    /// Configuración del motor
    config: MTTConfig,
    
    /// Métricas de uso
    metrics: MTTMetrics,
    
    /// Conexión a VoxelDB (para búsqueda espacial)
    voxel: Arc<VoxelDB>,
    
    /// Conexión a TelescopeDB (para contexto de usuario)
    telescope: Arc<TelescopeDB>,
}

/// Configuración del motor MTT-DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTTConfig {
    /// Directorio raíz de templates
    pub templates_dir: PathBuf,
    
    /// Threshold mínimo para usar template localmente (0.0-1.0)
    pub local_processing_threshold: f64,
    
    /// Tamaño del cache de matching
    pub match_cache_size: usize,
    
    /// Habilitar hot-reloading de templates
    pub enable_hot_reload: bool,
    
    /// Validar templates al cargar
    pub validate_on_load: bool,
}

/// Definición de un template MTT-DSL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateDefinition {
    /// Metadatos del template
    pub metadata: TemplateMetadata,
    
    /// Estructura del template
    pub structure: TemplateStructure,
    
    /// Personalidad del template
    pub personality: TemplatePersonality,
    
    /// Triggers que activan este template
    pub triggers: Vec<TemplateTrigger>,
    
    /// Validaciones que debe cumplir
    pub validations: Vec<TemplateValidation>,
}

/// Metadatos del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// ID único del template
    pub id: String,
    
    /// Nombre humano-legible
    pub name: String,
    
    /// Categoría (technical, creative, analytical, emotional, collaborative, meta)
    pub category: TemplateCategory,
    
    /// Versión del template (semver)
    pub version: String,
    
    /// Descripción breve
    pub description: String,
    
    /// Autor
    pub author: String,
    
    /// Fecha de creación
    pub created_at: String,
    
    /// Fecha de última actualización
    pub updated_at: String,
}

/// Categorías de templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateCategory {
    Technical,
    Creative,
    Analytical,
    Emotional,
    Collaborative,
    Meta,
}

/// Estructura del template (secciones)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStructure {
    /// Secciones del template en orden
    pub sections: Vec<TemplateSection>,
    
    /// Formato de output (markdown, plain, html)
    pub output_format: OutputFormat,
}

/// Sección individual del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Nombre de la sección
    pub name: String,
    
    /// Título mostrado al usuario
    pub title: String,
    
    /// Es obligatoria (si false, se puede omitir)
    pub required: bool,
    
    /// Prompt para generar contenido de esta sección
    pub prompt: String,
    
    /// Placeholder si no hay contenido
    pub placeholder: Option<String>,
}

/// Personalidad del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePersonality {
    /// Tono (technical, casual, empathetic, authoritative)
    pub tone: String,
    
    /// Profundidad (surface, detailed, comprehensive)
    pub depth: String,
    
    /// Estilo (code-focused, theoretical, practical)
    pub style: String,
    
    /// Enfoque (bottom-up, top-down, iterative)
    pub approach: String,
}

/// Trigger que activa el template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTrigger {
    /// Tipo de trigger
    #[serde(rename = "type")]
    pub trigger_type: TriggerType,
    
    /// Patrón de matching
    pub pattern: String,
    
    /// Peso del trigger (0.0-1.0)
    pub weight: f64,
}

/// Tipos de triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerType {
    /// Match por keyword
    Keyword,
    
    /// Match por regex
    Regex,
    
    /// Match por intención (detectado por CTX7D)
    Intent,
    
    /// Match por complejidad
    Complexity,
    
    /// Match por contexto histórico
    Historical,
}

/// Validación del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateValidation {
    /// Tipo de validación
    pub check: String,
    
    /// Mensaje si falla
    pub message: String,
}

/// Métricas del motor MTT-DSL
#[derive(Debug, Clone, Default)]
pub struct MTTMetrics {
    /// Total de queries procesados
    pub total_queries: u64,
    
    /// Queries procesados localmente (sin LLM)
    pub local_queries: u64,
    
    /// Queries delegados a LLM
    pub llm_queries: u64,
    
    /// Uso por template (template_id → count)
    pub template_usage: HashMap<String, u64>,
    
    /// Feedback positivo por template (template_id → count)
    pub positive_feedback: HashMap<String, u64>,
    
    /// Feedback negativo por template (template_id → count)
    pub negative_feedback: HashMap<String, u64>,
    
    /// Tiempo promedio de procesamiento (ms)
    pub avg_processing_ms: f64,
}

/// Resultado de template matching
#[derive(Debug, Clone)]
pub struct TemplateMatch {
    /// ID del template matched
    pub template_id: String,
    
    /// Score de similaridad (0.0-1.0)
    pub score: f64,
    
    /// Razón del match (para debugging)
    pub reasoning: String,
    
    /// Triggers que activaron este match
    pub matched_triggers: Vec<String>,
}

/// Respuesta generada por un template
#[derive(Debug, Clone, Serialize)]
pub struct TemplateResponse {
    /// Template usado
    pub template_id: String,
    
    /// Contenido generado
    pub content: String,
    
    /// Formato del contenido
    pub format: OutputFormat,
    
    /// Metadata de generación
    pub metadata: ResponseMetadata,
}

/// Metadata de respuesta generada
#[derive(Debug, Clone, Serialize)]
pub struct ResponseMetadata {
    /// ¿Se procesó localmente?
    pub processed_locally: bool,
    
    /// Tiempo de procesamiento (ms)
    pub processing_time_ms: u64,
    
    /// Score de efectividad estimado
    pub estimated_effectiveness: f64,
    
    /// Secciones incluidas
    pub sections_included: Vec<String>,
}

/// Formato de output
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Markdown,
    Plain,
    Html,
    Json,
}
```

---

## 🔌 API PÚBLICA

```rust
// src/templates/mtt_dsl.rs

impl MTTEngine {
    /// Crear nuevo motor MTT-DSL
    pub fn new(config: MTTConfig) -> Result<Self> {
        let mut engine = Self {
            templates: HashMap::new(),
            match_cache: lru::LruCache::new(config.match_cache_size),
            config,
            metrics: MTTMetrics::default(),
            voxel: Arc::new(VoxelDB::new()?),
            telescope: Arc::new(TelescopeDB::new()?),
        };
        
        // Cargar templates del directorio
        engine.load_templates()?;
        
        Ok(engine)
    }
    
    /// Cargar todos los templates desde directorio
    pub fn load_templates(&mut self) -> Result<()> {
        let templates_dir = &self.config.templates_dir;
        
        for category in &["technical", "creative", "analytical", 
                          "emotional", "collaborative", "meta"] {
            let category_dir = templates_dir.join(category);
            
            if !category_dir.exists() {
                continue;
            }
            
            for entry in std::fs::read_dir(category_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension() != Some(std::ffi::OsStr::new("yaml")) {
                    continue;
                }
                
                // Leer y parsear template
                let content = std::fs::read_to_string(&path)?;
                let template: TemplateDefinition = serde_yaml::from_str(&content)?;
                
                // Validar si configurado
                if self.config.validate_on_load {
                    self.validate_template(&template)?;
                }
                
                // Registrar template
                tracing::info!(
                    "Template cargado: {} (v{})",
                    template.metadata.name,
                    template.metadata.version
                );
                
                self.templates.insert(template.metadata.id.clone(), template);
            }
        }
        
        tracing::info!("Total templates cargados: {}", self.templates.len());
        
        Ok(())
    }
    
    /// Procesar query con MTT-DSL
    /// 
    /// # Argumentos
    /// - `query`: Query del usuario
    /// - `ctx7d`: Análisis dimensional del query
    /// - `user_id`: ID del usuario
    /// 
    /// # Retorna
    /// `TemplateResponse` si template match >= threshold
    /// `None` si debe delegarse a LLM
    pub async fn process_query(
        &mut self,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
    ) -> Result<Option<TemplateResponse>> {
        let start = Instant::now();
        
        // PASO 1: Template matching
        let template_match = self.find_best_template(query, ctx7d).await?;
        
        // Si score bajo, delegar a LLM
        if template_match.score < self.config.local_processing_threshold {
            tracing::info!(
                "Template match score bajo ({:.2}) - delegando a LLM",
                template_match.score
            );
            
            self.metrics.llm_queries += 1;
            return Ok(None);
        }
        
        // PASO 2: Cargar template
        let template = self.templates
            .get(&template_match.template_id)
            .ok_or(Error::TemplateNotFound(template_match.template_id.clone()))?;
        
        // PASO 3: Generar respuesta
        let content = self.generate_response(template, query, ctx7d, user_id).await?;
        
        let elapsed = start.elapsed();
        
        // PASO 4: Actualizar métricas
        self.metrics.total_queries += 1;
        self.metrics.local_queries += 1;
        *self.metrics.template_usage
            .entry(template_match.template_id.clone())
            .or_insert(0) += 1;
        
        // Actualizar promedio de procesamiento
        let alpha = 0.1;
        self.metrics.avg_processing_ms = alpha * elapsed.as_millis() as f64
            + (1.0 - alpha) * self.metrics.avg_processing_ms;
        
        Ok(Some(TemplateResponse {
            template_id: template_match.template_id,
            content,
            format: template.structure.output_format,
            metadata: ResponseMetadata {
                processed_locally: true,
                processing_time_ms: elapsed.as_millis() as u64,
                estimated_effectiveness: template_match.score,
                sections_included: template.structure.sections
                    .iter()
                    .map(|s| s.name.clone())
                    .collect(),
            },
        }))
    }
    
    /// Encontrar el mejor template para un query
    async fn find_best_template(
        &mut self,
        query: &str,
        ctx7d: &ContextTensor7D,
    ) -> Result<TemplateMatch> {
        // Revisar cache primero
        let query_hash = Self::hash_query(query);
        
        if let Some(cached_template_id) = self.match_cache.get(&query_hash) {
            return Ok(TemplateMatch {
                template_id: cached_template_id.clone(),
                score: 1.0, // Cache hit = perfect match
                reasoning: "Cached match".to_string(),
                matched_triggers: vec![],
            });
        }
        
        // Calcular score para cada template
        let mut matches = Vec::new();
        
        for (template_id, template) in &self.templates {
            let score = self.calculate_match_score(query, ctx7d, template);
            let matched_triggers = self.get_matched_triggers(query, ctx7d, template);
            
            matches.push(TemplateMatch {
                template_id: template_id.clone(),
                score,
                reasoning: format!(
                    "Triggers matched: {} | Score: {:.2}",
                    matched_triggers.len(),
                    score
                ),
                matched_triggers,
            });
        }
        
        // Ordenar por score descendente
        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        // Mejor match
        let best_match = matches
            .into_iter()
            .next()
            .ok_or(Error::NoTemplateMatched)?;
        
        // Cachear resultado
        self.match_cache.put(query_hash, best_match.template_id.clone());
        
        Ok(best_match)
    }
    
    /// Calcular score de matching para un template
    fn calculate_match_score(
        &self,
        query: &str,
        ctx7d: &ContextTensor7D,
        template: &TemplateDefinition,
    ) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;
        
        for trigger in &template.triggers {
            let trigger_score = match trigger.trigger_type {
                TriggerType::Keyword => {
                    if query.to_lowercase().contains(&trigger.pattern.to_lowercase()) {
                        1.0
                    } else {
                        0.0
                    }
                }
                
                TriggerType::Regex => {
                    let re = regex::Regex::new(&trigger.pattern).unwrap();
                    if re.is_match(query) { 1.0 } else { 0.0 }
                }
                
                TriggerType::Intent => {
                    // Detectar intención basado en CTX7D
                    self.detect_intent_match(&trigger.pattern, ctx7d)
                }
                
                TriggerType::Complexity => {
                    // Match basado en complejidad semántica
                    let expected_complexity: f64 = trigger.pattern.parse().unwrap_or(0.5);
                    1.0 - (ctx7d.semantic - expected_complexity).abs()
                }
                
                TriggerType::Historical => {
                    // TODO: consultar TelescopeDB para match histórico
                    0.5 // Placeholder
                }
            };
            
            score += trigger_score * trigger.weight;
            total_weight += trigger.weight;
        }
        
        // Normalizar por total de pesos
        if total_weight > 0.0 {
            score / total_weight
        } else {
            0.0
        }
    }
    
    /// Generar respuesta a partir de template
    async fn generate_response(
        &self,
        template: &TemplateDefinition,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
    ) -> Result<String> {
        let mut response = String::new();
        
        // Header del template
        response.push_str(&format!(
            "# {}\n\n",
            template.metadata.name
        ));
        
        // Generar cada sección
        for section in &template.structure.sections {
            // Título de la sección
            response.push_str(&format!("\n## {}\n\n", section.title));
            
            // Generar contenido de la sección
            let section_content = self.generate_section_content(
                section,
                query,
                ctx7d,
                user_id,
            ).await?;
            
            response.push_str(&section_content);
            response.push_str("\n");
        }
        
        Ok(response)
    }
    
    /// Generar contenido de una sección específica
    async fn generate_section_content(
        &self,
        section: &TemplateSection,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
    ) -> Result<String> {
        // Consultar TelescopeDB para contexto
        let user_history = self.telescope.get_user_history(user_id).await?;
        
        // Buscar ejemplos similares en VoxelDB
        let similar_entries = self.voxel
            .find_templates_spatial(ctx7d, 0.15)
            .await?;
        
        // Rellenar prompt de la sección con contexto
        let filled_prompt = section.prompt
            .replace("{query}", query)
            .replace("{user_expertise}", &user_history.expertise_level.to_string())
            .replace("{similar_count}", &similar_entries.len().to_string());
        
        // Para v1.0, usamos lógica simple de relleno
        // v2.0 puede integrar LLM para secciones complejas
        let content = if similar_entries.is_empty() {
            section.placeholder
                .clone()
                .unwrap_or_else(|| "[Contenido generado aquí]".to_string())
        } else {
            format!(
                "Basado en {} ejemplos similares previos:\n{}",
                similar_entries.len(),
                similar_entries.iter()
                    .take(3)
                    .map(|e| format!("- {}", e.name))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };
        
        Ok(content)
    }
    
    /// Registrar feedback de usuario
    pub fn record_feedback(
        &mut self,
        template_id: &str,
        positive: bool,
    ) {
        if positive {
            *self.metrics.positive_feedback
                .entry(template_id.to_string())
                .or_insert(0) += 1;
        } else {
            *self.metrics.negative_feedback
                .entry(template_id.to_string())
                .or_insert(0) += 1;
        }
    }
    
    /// Calcular effectiveness score de un template
    pub fn calculate_effectiveness(&self, template_id: &str) -> f64 {
        let positive = self.metrics.positive_feedback
            .get(template_id)
            .copied()
            .unwrap_or(0) as f64;
        
        let negative = self.metrics.negative_feedback
            .get(template_id)
            .copied()
            .unwrap_or(0) as f64;
        
        let usage = self.metrics.template_usage
            .get(template_id)
            .copied()
            .unwrap_or(0) as f64;
        
        if positive + negative == 0.0 {
            return 0.5; // Neutral si no hay feedback
        }
        
        // Fórmula: satisfaction_rate * completeness_factor
        let satisfaction_rate = positive / (positive + negative);
        let completeness_factor = (usage / (usage + 10.0)).min(1.0);
        
        satisfaction_rate * completeness_factor
    }
    
    /// Obtener métricas del motor
    pub fn get_metrics(&self) -> &MTTMetrics {
        &self.metrics
    }
}
```

---

## ⚙️ IMPLEMENTACIÓN INTERNA

### Catálogo de Templates (18 Total)

#### 1. Technical Templates (3)

**debugging_deep_dive.yaml** ✅ IMPLEMENTADO
```yaml
metadata:
  id: "debugging_deep_dive"
  name: "Debugging Sistemático"
  category: "technical"
  version: "1.0.0"

structure:
  sections:
    - name: "problem_identification"
      title: "1️⃣ IDENTIFICACIÓN DEL PROBLEMA"
      required: true
      prompt: "Describe el error y contexto de ejecución"
    
    - name: "root_cause_analysis"
      title: "2️⃣ ANÁLISIS DE CAUSAS RAÍZ"
      required: true
      prompt: "Formula hipótesis sobre causas potenciales"
    
    - name: "resolution_strategy"
      title: "3️⃣ ESTRATEGIA DE RESOLUCIÓN"
      required: true
      prompt: "Define pasos metodológicos para resolver"
    
    - name: "implementation"
      title: "4️⃣ IMPLEMENTACIÓN + TESTS"
      required: true
      prompt: "Código corregido + unit tests de validación"
    
    - name: "prevention"
      title: "5️⃣ PREVENCIÓN FUTURA"
      required: false
      prompt: "Patterns a seguir y red flags a evitar"

personality:
  tone: "technical"
  depth: "comprehensive"
  style: "code-focused"
  approach: "systematic"

triggers:
  - type: "keyword"
    pattern: "debugging|debug|bug|error|fix"
    weight: 1.0
  
  - type: "intent"
    pattern: "troubleshooting"
    weight: 0.8
  
  - type: "complexity"
    pattern: "0.6"  # Media-alta complejidad
    weight: 0.5
```

**code_review.yaml** 🌱 NO IMPLEMENTADO
```yaml
metadata:
  id: "code_review"
  name: "Code Review Estructurado"
  category: "technical"
  version: "1.0.0"

structure:
  sections:
    - name: "overview"
      title: "📋 OVERVIEW DEL CÓDIGO"
    - name: "correctness"
      title: "✅ CORRECTITUD"
    - name: "performance"
      title: "⚡ PERFORMANCE"
    - name: "security"
      title: "🔒 SEGURIDAD"
    - name: "maintainability"
      title: "🛠️ MANTENIBILIDAD"
    - name: "recommendations"
      title: "💡 RECOMENDACIONES"

triggers:
  - type: "keyword"
    pattern: "review|revisar|código"
    weight: 1.0
```

**architectural_design.yaml** 🌱 NO IMPLEMENTADO
```yaml
metadata:
  id: "architectural_design"
  name: "Diseño Arquitectónico"
  category: "technical"
  version: "1.0.0"

triggers:
  - type: "keyword"
    pattern: "architecture|diseño|system design"
    weight: 1.0
```

#### 2. Creative Templates (3)

**creative_flow.yaml, storytelling.yaml, design_thinking.yaml** 🌱 NO IMPLEMENTADOS

#### 3. Analytical Templates (3)

**systematic_analysis.yaml, data_analysis.yaml, decision_making.yaml** 🌱 NO IMPLEMENTADOS

#### 4. Emotional Templates (3)

**emotional_processing.yaml, empathy.yaml, stress_management.yaml** 🌱 NO IMPLEMENTADOS

#### 5. Collaborative Templates (3)

**multi_agent_collaboration.yaml, team_dynamics.yaml, conflict_resolution.yaml** 🌱 NO IMPLEMENTADOS

#### 6. Meta Templates (3)

**meta_reflection.yaml, learning_path.yaml, session_retrospective.yaml** 🌱 NO IMPLEMENTADOS

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito |
|------------|---------|-----------|
| **VoxelDB** | v1.0 | Indexación espacial de templates para búsqueda por similaridad |
| **Context Token 7D** | v1.0 | Análisis dimensional para template matching inteligente |
| **TelescopeDB** | v1.0 | Historial de usuario (expertise, templates usados, feedback) |
| **HubSpoke Navigator** | v1.0 | Delegación a LLM cuando template match < threshold |

### Crates Externos

```toml
[dependencies]
# Serialization (YAML templates)
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"

# Template parsing
regex = "1.10"

# Caching
lru = "0.12"

# Hashing para cache
seahash = "4.1"

# Filesystem watching (hot-reload)
notify = "6.1"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"

# Async runtime
tokio = { version = "1.35", features = ["full"] }
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

| Operación | Target | Medición | Status |
|-----------|--------|----------|--------|
| `load_templates()` | <200ms | Tiempo de carga inicial de 18 templates | ⏸️ TBD |
| `find_best_template()` | <50ms | Matching contra 18 templates | ⏸️ TBD |
| `process_query()` (local) | <100ms | Query simple procesado localmente | 🎯 CRÍTICO |
| `generate_response()` | <80ms | Generación de respuesta desde template | ⏸️ TBD |
| **Pipeline total (local)** | **<200ms** | **Desde query hasta respuesta (sin LLM)** | **🎯 CRÍTICO** |
| Memoria RAM (MTTEngine) | <100 MB | RSS con 18 templates cargados | ⏸️ TBD |
| **Local processing rate** | **≥90%** | **% de queries procesadas sin LLM** | **🎯 CRÍTICO** |
| Template effectiveness | >0.75 | Promedio de satisfaction rate | ⏸️ TBD |

### Benchmarks Esperados

```rust
// tests/benchmarks/mtt_dsl_bench.rs

#[bench]
fn bench_template_matching(b: &mut Bencher) {
    let engine = MTTEngine::new(default_config()).unwrap();
    let ctx7d = ContextTensor7D::example();
    
    b.iter(|| {
        engine.find_best_template("help debugging async Rust", &ctx7d)
    });
}

// Target: ~30-50ms por matching
```

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
// tests/mtt_dsl_test.rs

#[tokio::test]
async fn test_debugging_template_matches_bug_query() {
    let mut engine = MTTEngine::new(default_config()).unwrap();
    
    let query = "I have a bug in my async Rust code";
    let ctx7d = ContextTensor7D {
        semantic: 0.70,
        intentional: 0.85,
        ..Default::default()
    };
    
    let template_match = engine.find_best_template(query, &ctx7d)
        .await.unwrap();
    
    assert_eq!(template_match.template_id, "debugging_deep_dive");
    assert!(template_match.score > 0.80);
}

#[tokio::test]
async fn test_local_processing_generates_structured_response() {
    let mut engine = MTTEngine::new(default_config()).unwrap();
    
    let query = "debugging async issue";
    let ctx7d = ContextTensor7D::default();
    
    let response = engine.process_query(query, &ctx7d, "user_123")
        .await.unwrap()
        .expect("Should process locally");
    
    assert!(response.metadata.processed_locally);
    assert!(response.metadata.processing_time_ms < 200);
    assert!(response.content.contains("IDENTIFICACIÓN DEL PROBLEMA"));
}

#[test]
fn test_effectiveness_calculation() {
    let mut engine = MTTEngine::new(default_config()).unwrap();
    
    // Simular feedback
    engine.record_feedback("debugging_deep_dive", true);
    engine.record_feedback("debugging_deep_dive", true);
    engine.record_feedback("debugging_deep_dive", true);
    engine.record_feedback("debugging_deep_dive", false);
    
    let effectiveness = engine.calculate_effectiveness("debugging_deep_dive");
    
    // 3 positivos / 4 total = 0.75
    assert!((effectiveness - 0.75).abs() < 0.1);
}
```

### Integration Tests

```rust
// tests/integration/mtt_dsl_voxeldb_integration.rs

#[tokio::test]
async fn test_mtt_engine_queries_voxeldb_for_templates() {
    let mut engine = MTTEngine::new(default_config()).unwrap();
    let voxel = VoxelDB::new().unwrap();
    
    // Insertar template en VoxelDB
    voxel.insert_template(Template {
        name: "debugging_deep_dive".to_string(),
        coords: CubicCoords { x: 0.5, y: 0.6, z: 0.7 },
        ..Default::default()
    }).await.unwrap();
    
    let ctx7d = ContextTensor7D {
        semantic: 0.5,
        intentional: 0.6,
        temporal: 0.7,
        ..Default::default()
    };
    
    // MTT Engine debe encontrar el template vía VoxelDB
    let response = engine.process_query("debug", &ctx7d, "user_123")
        .await.unwrap()
        .expect("Should find template");
    
    assert_eq!(response.template_id, "debugging_deep_dive");
}
```

### Golden Tests (Template Validation)

```rust
// tests/golden/templates_validation.rs

#[test]
fn test_all_18_templates_are_valid() {
    let templates_dir = PathBuf::from("templates/mtt");
    
    let expected_templates = [
        "debugging_deep_dive",
        "code_review",
        "architectural_design",
        "creative_flow",
        "storytelling",
        "design_thinking",
        "emotional_processing",
        "empathy",
        "stress_management",
        "systematic_analysis",
        "data_analysis",
        "decision_making",
        "multi_agent_collaboration",
        "team_dynamics",
        "conflict_resolution",
        "meta_reflection",
        "learning_path",
        "session_retrospective",
    ];
    
    for template_id in &expected_templates {
        // Buscar archivo
        let yaml_path = templates_dir
            .join("**")
            .join(format!("{}.yaml", template_id));
        
        // Verificar que existe
        assert!(
            yaml_path.exists(),
            "Template {} no encontrado en {}",
            template_id,
            yaml_path.display()
        );
        
        // Parsear y validar
        let content = std::fs::read_to_string(&yaml_path).unwrap();
        let template: TemplateDefinition = serde_yaml::from_str(&content)
            .expect(&format!("Template {} tiene sintaxis YAML inválida", template_id));
        
        // Validaciones básicas
        assert_eq!(template.metadata.id, *template_id);
        assert!(!template.structure.sections.is_empty());
        assert!(!template.triggers.is_empty());
    }
}
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/templates/mtt_dsl/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MTTError {
    #[error("Template no encontrado: {0}")]
    TemplateNotFound(String),
    
    #[error("Sintaxis YAML inválida en template {0}: {1}")]
    InvalidYAML(String, String),
    
    #[error("Template {0} falló validación: {1}")]
    ValidationFailed(String, String),
    
    #[error("Ningún template matched el query")]
    NoTemplateMatched,
    
    #[error("Directorio de templates no existe: {0}")]
    TemplatesDirNotFound(PathBuf),
    
    #[error("Error de IO: {0}")]
    IOError(#[from] std::io::Error),
    
    #[error("Error de VoxelDB: {0}")]
    VoxelDBError(#[from] crate::cells::voxeldb::Error),
    
    #[error("Error de TelescopeDB: {0}")]
    TelescopeDBError(#[from] crate::cells::telescopedb::Error),
    
    #[error("Error de regex: {0}")]
    RegexError(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, MTTError>;
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md** - Indexación espacial de templates
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md** - Análisis dimensional para matching
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md** - Historial de usuario
- **ROADMAP_V2/07_TEMPLATES/** - Templates experimentales de documentación (dogfooding)

### Decisiones Arquitectónicas

- **DA-016:** MTT-DSL ≠ Música - Sistema de templates NO requiere componente musical
- **DA-017:** Templates como LEGO - Bloques de construcción reutilizables
- **DA-018:** HarmonyEngine opcional - Música es feature separado, no core

### FUSION_BAYESIANA

- **FUSION_BAYESIANA/05_MTT_DSL_TEMPLATES.md** - Análisis completo de metodología MTT-DSL
- **FUSION_BAYESIANA/08_DIAGRAMA_SISTEMA.md** - Ubicación de MTT-DSL en arquitectura

### Código Existente

- `templates/mtt/` - Directorio de templates productivos (18 templates)
- `templates/mtt/README.md` - Documentación de estructura de templates
- `templates/mtt/technical/debugging_deep_dive.yaml` - Template implementado de referencia

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Semanas 7-9)

1. **Crear estructura base:**
   ```bash
   mkdir -p src/templates/mtt_dsl
   touch src/templates/mtt_dsl/{mod.rs,engine.rs,parser.rs,matcher.rs,error.rs}
   ```

2. **Implementar parser YAML:**
   - Cargar templates desde `templates/mtt/`
   - Parsear estructura YAML → `TemplateDefinition`
   - Validar schema (secciones, triggers, metadata)
   - Unit tests de parsing

3. **Implementar template matching:**
   - Algoritmo de scoring por triggers
   - Cache LRU de matches (query_hash → template_id)
   - Integración con CTX7D para intent detection
   - Benchmarks de matching (<50ms target)

4. **Implementar procesamiento local:**
   - Generación de respuesta desde template
   - Inyección de contexto (TelescopeDB, VoxelDB)
   - Formato de output (Markdown, Plain, HTML)
   - Testing con debugging_deep_dive

5. **Métricas y feedback:**
   - Tracking de uso por template
   - Sistema de feedback (👍/👎)
   - Cálculo de effectiveness score
   - Dashboard de métricas

### Mejoras v1.5 (Semanas 10-12)

6. **Implementar 17 templates restantes:**
   - 2 technical (code_review, architectural_design)
   - 3 creative
   - 3 analytical
   - 3 emotional
   - 3 collaborative
   - 3 meta

7. **Hot-reloading de templates:**
   - Watcher de filesystem (`notify` crate)
   - Recarga automática cuando .yaml cambia
   - Validación antes de aplicar cambios
   - Rollback si template inválido

8. **Optimizaciones de performance:**
   - Paralelización de matching (Rayon)
   - Compilación de regex en cache
   - Reducir allocaciones en hot path
   - Target: <100ms procesamiento local total

### Mejoras v2.0 (Futuro)

9. **LLM-assisted section generation:**
   - Para secciones complejas, usar LLM con mini-prompt
   - Mantener estructura del template
   - Reducir latencia con prompts optimizados

10. **Template personalization:**
    - A/B testing de variantes de template
    - Personalización por usuario (beginner vs expert)
    - Aprendizaje de preferencias (histórico)

11. **HarmonyEngine integration (OPCIONAL):**
    - Asociar soundtracks a templates (si HarmonyEngine se implementa)
    - Frecuencias Solfeggio por categoría
    - Feature flag para habilitar/deshabilitar música

---

**Estado:** 📋 ESPECIFICACIÓN  
**Complejidad:** 🟡 MEDIA (parsing + matching) → 🔴 ALTA (18 templates completos)  
**Prioridad:** 🟡 ALTA (Fase 2, Semanas 7-12)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec*
