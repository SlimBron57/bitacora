```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/10_lip-protocol.md
Versión: 1.0.0
Fecha Creación: 2025-10-26
Autor: Sistema Bitácora - Documentación MTT-DSL
Propósito: Especificación componente LIP - Lens Interface Protocol (Protocolo de lentes para múltiples perspectivas)
Estado: 📋 ESPECIFICACIÓN
Relacionado Con:
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md
  - ROADMAP_V2/02_COMPONENTES/IMPORTANTES/MTT_DSL_TEMPLATES.md
  - cleanup_temp/original_docs/BITA-1_FBCU_v1_Implementation_Spec.md (§6)
Implementa:
  - DA-029: LIP - Lens Interface Protocol
  - DA-030: Quality Bounds Validation
  - BITA-1: Overlay System (Plasticity + Topology)
# === FIN DATOS DE AUDITORÍA ===
```

# 🔮 LIP PROTOCOL - Lens Interface Protocol

---

## 🎯 PROPÓSITO

El **Lens Interface Protocol (LIP)** es el sistema de contratos que permite a **procesadores especializados (lentes)** interactuar con **FBCU Cores** de forma determinista, validable y extensible.

### La Metáfora: Lentes para Ver la Realidad

**Sin lentes (visión directa):**
```
Usuario: "Analiza este FBCU Core"

Sistema sin LIP:
- Procesa FBCU directamente
- Extrae campos genéricos
- Retorna datos raw
- NO hay validación de calidad
- NO hay especialización

Output:
{
  "content": "Usuario escuchó Kaleidoscope",
  "timestamp": 1698345600,
  "semantic_score": 0.75
}

¿Qué significa "semantic_score: 0.75"?
¿Cómo se relaciona con música?
¿Por qué no hay análisis emocional?
❌ INFORMACIÓN PLANA - Sin contexto especializado
```

**Con lentes (LIP):**
```
Usuario: "Analiza este FBCU Core con HarmonyLens"
    ↓
┌─────────────────────────────────────────────────┐
│ LIP PROTOCOL: Validación de Contrato           │
│                                                 │
│ Lente solicitado: HarmonyLens                  │
│ ├─ REQUIRES:                                   │
│ │   ├─ context_tensor.temporal ✅ (presente)   │
│ │   ├─ context_tensor.biographical ✅          │
│ │   └─ embedding opcional ⚠️ (no presente)     │
│ │                                              │
│ ├─ PROVIDES:                                   │
│ │   ├─ musical_context                         │
│ │   ├─ emotional_resonance                     │
│ │   └─ harmony_score                           │
│ │                                              │
│ └─ QUALITY_BOUNDS:                             │
│     ├─ coherence_min: 0.80                     │
│     └─ lens_agreement_min: 0.85                │
│                                                 │
│ Validación: ✅ PASSED (todos los requires OK)  │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ HARMONY LENS: Procesamiento Especializado      │
│                                                 │
│ Analiza FBCU con perspectiva musical:          │
│ ├─ Detecta: "Kaleidoscope" (Orchestra)         │
│ ├─ Extrae: Emotional resonance alta (0.92)     │
│ ├─ Mapea: Temporal context → Momento creativo  │
│ └─ Genera: Harmony score (0.88)                │
└─────────────────────────────────────────────────┘
    ↓
Output validado por LIP:
{
  "musical_context": {
    "genre": "Contemporary Orchestra",
    "composer": "Too Many Zoos",
    "emotional_tone": "Inspirational + Energetic",
    "creative_moment": true
  },
  "emotional_resonance": 0.92,
  "harmony_score": 0.88,
  
  "quality_validation": {
    "coherence": 0.91,        ✅ > 0.80 (passed)
    "lens_agreement": 0.87,   ✅ > 0.85 (passed)
    "validation_status": "PASSED"
  }
}

✅ INFORMACIÓN RICA - Contexto musical especializado
✅ VALIDADA - Quality bounds cumplidos
✅ EXPLICABLE - Sabemos qué midió HarmonyLens
```

**La diferencia clave:**
- Sin LIP: Datos genéricos, sin especialización
- Con LIP: **Múltiples perspectivas especializadas con contratos validables**

Como usar diferentes lentes para ver la misma realidad:
- **HarmonyLens:** Ve música y emoción
- **MTT-DSL Lens:** Ve patrones de debugging
- **SemanticLens:** Ve significado profundo
- **Custom Lens:** Ve lo que TÚ defines

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
FLUJO: FBCU Core → LIP Validation → Lens Processing

Usuario: "Analiza mi historia de aprendizaje con SemanticLens"
    ↓
┌─────────────────────────────────────────────────┐
│ TELESCOPEDB: Recuperar FBCUs biográficos        │
│ └─> 1000 FBCUs del usuario (últimos 6 meses)   │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ ★★★ LIP PROTOCOL (TÚ ESTÁS AQUÍ) ★★★           │
│                                                 │
│ FASE 1: Lens Selection                         │
│  ├─ Usuario solicita: SemanticLens             │
│  ├─ Cargar LIP contract de SemanticLens        │
│  └─ Validar disponibilidad del lente           │
│                                                 │
│ FASE 2: Requirement Validation                 │
│  Para cada FBCU:                               │
│  ├─ Verificar REQUIRES del lens:               │
│  │   ├─ fields: ["content", "context_tensor"]  │
│  │   ├─ embedding: { model: "all-MiniLM", dim: 384 } │
│  │   └─ anchors: opcional                      │
│  │                                              │
│  ├─ Validar FBCU tiene los campos:             │
│  │   ✅ content: presente                       │
│  │   ✅ context_tensor: presente                │
│  │   ✅ embedding: presente (384 dims)          │
│  │                                              │
│  └─ Resultado: ✅ FBCU COMPATIBLE con lens      │
│                                                 │
│ FASE 3: Lens Execution                         │
│  ├─ Pasar FBCU validado → SemanticLens.process() │
│  ├─ Lens ejecuta análisis especializado        │
│  └─ Retorna output según PROVIDES del contract │
│                                                 │
│ FASE 4: Quality Validation                     │
│  ├─ Verificar QUALITY_BOUNDS del output:       │
│  │   - coherence: 0.91 ✅ (min: 0.85)          │
│  │   - lens_agreement: 0.88 ✅ (min: 0.80)     │
│  │                                              │
│  └─ Resultado: ✅ OUTPUT VALIDADO               │
│                                                 │
│ FASE 5: Output Enrichment                      │
│  ├─ Agregar metadata de validación:            │
│  │   - lens_used: "SemanticLens v1.0"          │
│  │   - validation_status: "PASSED"             │
│  │   - quality_scores: { coherence, agreement }│
│  │                                              │
│  └─ Retornar: EnrichedLensOutput               │
└─────────────────────────────────────────────────┘
    ↓
Usuario recibe: Análisis semántico validado ✅
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito |
|------------|-----------|-----------|
| **TelescopeDB** | Entrada ← | Provee FBCUs para procesar con lentes |
| **VoxelDB** | Entrada ← | Provee templates para procesar con lentes |
| **MTT-DSL Engine** | Salida → | MTT-DSL Lens procesa templates |
| **Context Token 7D** | Entrada ← | Provee análisis dimensional para lentes |
| **Harmony Engine** | Salida → | HarmonyLens extrae contexto musical |

---

## 📋 RESPONSABILIDADES CORE

El LIP Protocol **DEBE**:

1. **Definir Contratos de Lentes:**
   - **REQUIRES:** Qué necesita el lens del FBCU
   - **PROVIDES:** Qué genera el lens como output
   - **QUALITY_BOUNDS:** Métricas mínimas esperadas
   - **EXPLAIN_HINTS:** Cómo debuggear si falla

2. **Validar Compatibilidad FBCU-Lens:**
   - Verificar que FBCU tiene campos requeridos
   - Validar tipos de datos (string, array, embedding, etc.)
   - Verificar dimensionalidad de embeddings si aplica
   - Rechazar con error claro si incompatible

3. **Ejecutar Procesamiento de Lentes:**
   - Pasar FBCU validado al lens especializado
   - Coordinar ejecución (sync o async)
   - Capturar errores de procesamiento
   - Timeout protection (max 30s por lens)

4. **Validar Calidad de Output:**
   - Verificar que output cumple PROVIDES contract
   - Calcular quality metrics (coherence, agreement)
   - Comparar contra QUALITY_BOUNDS (min thresholds)
   - Rechazar output si quality < bounds

5. **Gestión de Lentes Custom:**
   - Registrar nuevos lentes en registry
   - Validar LIP contract de lente custom
   - Hot-reload de lentes sin reiniciar sistema
   - Versionado de lentes (backward compatibility)

6. **Explicabilidad y Debugging:**
   - Generar logs detallados de validación
   - Proveer EXPLAIN_HINTS si validación falla
   - Trace completo: FBCU → Lens → Output
   - Métricas de performance por lens

---

## 🗂️ ESTRUCTURAS DE DATOS

```rust
// src/lip/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait que todos los lentes deben implementar
#[async_trait::async_trait]
pub trait LensInterface: Send + Sync {
    /// Retorna requirements del lens
    fn requires(&self) -> LipRequirements;
    
    /// Retorna outputs que provee el lens
    fn provides(&self) -> LipOutputs;
    
    /// Procesa FBCU con el lens
    async fn process(&self, fbcu: &FBCUCore) -> Result<LensOutput>;
    
    /// Nombre del lens
    fn lens_id(&self) -> &str;
    
    /// Versión del lens
    fn version(&self) -> &str;
}

/// Requirements de un lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipRequirements {
    /// Campos requeridos del FBCU
    pub fields: Vec<String>,
    
    /// Embedding requerido (opcional)
    pub embedding: Option<EmbeddingRequirement>,
    
    /// Anchors requeridos (opcional)
    pub anchors: bool,
    
    /// Triples requeridos (opcional)
    pub triples: bool,
}

/// Requirement de embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequirement {
    /// Modelo de embedding esperado
    pub model: String,
    
    /// Dimensionalidad esperada
    pub dim: usize,
}

/// Outputs que provee un lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipOutputs {
    /// Nombres de outputs generados
    pub outputs: Vec<String>,
    
    /// Quality bounds esperados
    pub quality_bounds: QualityBounds,
    
    /// Hints para debugging (opcional)
    pub explain_hints: Option<Vec<String>>,
}

/// Quality bounds (thresholds mínimos)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityBounds {
    /// Coherencia mínima (0.0-1.0)
    pub coherence_min: f64,
    
    /// Lens agreement mínimo (0.0-1.0)
    pub lens_agreement_min: f64,
}

/// Output de un lens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LensOutput {
    /// ID del lens usado
    pub lens_id: String,
    
    /// Versión del lens
    pub version: String,
    
    /// Datos generados (flexible)
    pub data: serde_json::Value,
    
    /// Quality metrics calculados
    pub quality_metrics: QualityMetrics,
    
    /// Estado de validación
    pub validation_status: ValidationStatus,
    
    /// Timestamp de procesamiento
    pub processed_at: i64,
}

/// Quality metrics calculados
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    /// Coherence score (0.0-1.0)
    pub coherence: f64,
    
    /// Lens agreement score (0.0-1.0)
    pub lens_agreement: f64,
}

/// Estado de validación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Passed,
    Failed { reason: String },
    PartiallyPassed { warnings: Vec<String> },
}

/// Manager de lentes (registry)
pub struct LipManager {
    /// Lentes registrados
    lenses: HashMap<String, Box<dyn LensInterface>>,
    
    /// Configuración
    config: LipConfig,
}

/// Configuración de LIP
#[derive(Debug, Clone)]
pub struct LipConfig {
    /// Timeout máximo por lens (segundos)
    pub max_timeout_secs: u64,
    
    /// Permitir lentes sin embeddings
    pub allow_no_embedding: bool,
    
    /// Strict mode (rechazar si quality < bounds)
    pub strict_mode: bool,
}
```

---

## 🔌 API PÚBLICA

```rust
// src/lip/mod.rs

impl LipManager {
    /// Crear nuevo manager
    pub fn new(config: LipConfig) -> Self {
        Self {
            lenses: HashMap::new(),
            config,
        }
    }
    
    /// Registrar nuevo lens
    pub fn register_lens(&mut self, lens: Box<dyn LensInterface>) -> Result<()> {
        let lens_id = lens.lens_id().to_string();
        
        // Validar que lens tiene contract válido
        self.validate_lens_contract(&lens)?;
        
        self.lenses.insert(lens_id, lens);
        
        Ok(())
    }
    
    /// Procesar FBCU con lens específico
    pub async fn process_with_lens(
        &self,
        fbcu: &FBCUCore,
        lens_id: &str,
    ) -> Result<LensOutput> {
        // Paso 1: Obtener lens
        let lens = self.lenses.get(lens_id)
            .ok_or(LipError::LensNotFound(lens_id.to_string()))?;
        
        // Paso 2: Validar compatibilidad FBCU-Lens
        self.validate_fbcu_compatibility(fbcu, lens.as_ref())?;
        
        // Paso 3: Ejecutar lens con timeout
        let output = tokio::time::timeout(
            Duration::from_secs(self.config.max_timeout_secs),
            lens.process(fbcu)
        )
        .await
        .map_err(|_| LipError::LensTimeout(lens_id.to_string()))??;
        
        // Paso 4: Validar quality bounds
        let validated_output = self.validate_quality_bounds(output, lens.as_ref())?;
        
        Ok(validated_output)
    }
    
    /// Validar compatibilidad FBCU-Lens
    fn validate_fbcu_compatibility(
        &self,
        fbcu: &FBCUCore,
        lens: &dyn LensInterface,
    ) -> Result<()> {
        let requirements = lens.requires();
        
        // Validar fields
        for field in &requirements.fields {
            if !fbcu.has_field(field) {
                return Err(LipError::MissingField {
                    lens_id: lens.lens_id().to_string(),
                    field: field.clone(),
                });
            }
        }
        
        // Validar embedding si requerido
        if let Some(emb_req) = &requirements.embedding {
            let fbcu_emb = fbcu.embedding.as_ref()
                .ok_or(LipError::MissingEmbedding {
                    lens_id: lens.lens_id().to_string(),
                })?;
            
            // Verificar dimensionalidad
            if fbcu_emb.dimensions != emb_req.dim {
                return Err(LipError::EmbeddingDimensionMismatch {
                    expected: emb_req.dim,
                    found: fbcu_emb.dimensions,
                });
            }
        }
        
        // Validar anchors si requerido
        if requirements.anchors && fbcu.anchors.is_none() {
            return Err(LipError::MissingAnchors {
                lens_id: lens.lens_id().to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Validar quality bounds del output
    fn validate_quality_bounds(
        &self,
        mut output: LensOutput,
        lens: &dyn LensInterface,
    ) -> Result<LensOutput> {
        let bounds = lens.provides().quality_bounds;
        let metrics = &output.quality_metrics;
        
        // Verificar coherence
        if metrics.coherence < bounds.coherence_min {
            if self.config.strict_mode {
                return Err(LipError::QualityBoundViolation {
                    metric: "coherence".to_string(),
                    value: metrics.coherence,
                    min_required: bounds.coherence_min,
                });
            } else {
                output.validation_status = ValidationStatus::PartiallyPassed {
                    warnings: vec![format!(
                        "Coherence {} < min {}",
                        metrics.coherence,
                        bounds.coherence_min
                    )],
                };
            }
        }
        
        // Verificar lens_agreement
        if metrics.lens_agreement < bounds.lens_agreement_min {
            if self.config.strict_mode {
                return Err(LipError::QualityBoundViolation {
                    metric: "lens_agreement".to_string(),
                    value: metrics.lens_agreement,
                    min_required: bounds.lens_agreement_min,
                });
            } else {
                if let ValidationStatus::PartiallyPassed { warnings } = &mut output.validation_status {
                    warnings.push(format!(
                        "Lens agreement {} < min {}",
                        metrics.lens_agreement,
                        bounds.lens_agreement_min
                    ));
                }
            }
        }
        
        Ok(output)
    }
    
    /// Listar lentes disponibles
    pub fn list_lenses(&self) -> Vec<LensInfo> {
        self.lenses.values()
            .map(|lens| LensInfo {
                lens_id: lens.lens_id().to_string(),
                version: lens.version().to_string(),
                requires: lens.requires(),
                provides: lens.provides(),
            })
            .collect()
    }
}

/// Información de un lens
#[derive(Debug, Clone, Serialize)]
pub struct LensInfo {
    pub lens_id: String,
    pub version: String,
    pub requires: LipRequirements,
    pub provides: LipOutputs,
}
```

---

## ⚙️ IMPLEMENTACIÓN INTERNA

### Lentes Implementados

#### 1. HarmonyLens (Contexto Musical)

```rust
// src/lip/lenses/harmony_lens.rs

pub struct HarmonyLens {
    harmony_engine: Arc<HarmonyEngine>,
}

#[async_trait::async_trait]
impl LensInterface for HarmonyLens {
    fn lens_id(&self) -> &str {
        "harmony_lens"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn requires(&self) -> LipRequirements {
        LipRequirements {
            fields: vec![
                "context_tensor.temporal".to_string(),
                "context_tensor.biographical".to_string(),
            ],
            embedding: None, // Opcional
            anchors: false,
            triples: false,
        }
    }
    
    fn provides(&self) -> LipOutputs {
        LipOutputs {
            outputs: vec![
                "musical_context".to_string(),
                "emotional_resonance".to_string(),
                "harmony_score".to_string(),
            ],
            quality_bounds: QualityBounds {
                coherence_min: 0.80,
                lens_agreement_min: 0.85,
            },
            explain_hints: Some(vec![
                "Check if FBCU contains musical references".to_string(),
                "Verify temporal context is present".to_string(),
            ]),
        }
    }
    
    async fn process(&self, fbcu: &FBCUCore) -> Result<LensOutput> {
        // Extraer contexto temporal
        let temporal_ctx = fbcu.context_tensor.temporal;
        
        // Analizar con HarmonyEngine
        let musical_analysis = self.harmony_engine
            .analyze_musical_context(&fbcu.content, temporal_ctx)
            .await?;
        
        // Calcular quality metrics
        let coherence = musical_analysis.coherence_score;
        let lens_agreement = musical_analysis.confidence;
        
        // Generar output
        Ok(LensOutput {
            lens_id: self.lens_id().to_string(),
            version: self.version().to_string(),
            data: serde_json::json!({
                "musical_context": musical_analysis.musical_context,
                "emotional_resonance": musical_analysis.emotional_resonance,
                "harmony_score": musical_analysis.harmony_score,
            }),
            quality_metrics: QualityMetrics {
                coherence,
                lens_agreement,
            },
            validation_status: ValidationStatus::Passed,
            processed_at: chrono::Utc::now().timestamp(),
        })
    }
}
```

#### 2. MTT-DSL Lens (Template Analysis)

```rust
// src/lip/lenses/mtt_dsl_lens.rs

pub struct MttDslLens {
    mtt_engine: Arc<MTTEngine>,
}

#[async_trait::async_trait]
impl LensInterface for MttDslLens {
    fn lens_id(&self) -> &str {
        "mtt_dsl_lens"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn requires(&self) -> LipRequirements {
        LipRequirements {
            fields: vec!["content".to_string()],
            embedding: None,
            anchors: false,
            triples: false,
        }
    }
    
    fn provides(&self) -> LipOutputs {
        LipOutputs {
            outputs: vec![
                "template_match".to_string(),
                "confidence_score".to_string(),
            ],
            quality_bounds: QualityBounds {
                coherence_min: 0.75,
                lens_agreement_min: 0.80,
            },
            explain_hints: None,
        }
    }
    
    async fn process(&self, fbcu: &FBCUCore) -> Result<LensOutput> {
        // Analizar contenido con MTT engine
        let template_match = self.mtt_engine
            .match_template(&fbcu.content)
            .await?;
        
        Ok(LensOutput {
            lens_id: self.lens_id().to_string(),
            version: self.version().to_string(),
            data: serde_json::json!({
                "template_match": template_match.template_id,
                "confidence_score": template_match.confidence,
            }),
            quality_metrics: QualityMetrics {
                coherence: template_match.coherence,
                lens_agreement: template_match.confidence,
            },
            validation_status: ValidationStatus::Passed,
            processed_at: chrono::Utc::now().timestamp(),
        })
    }
}
```

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito |
|------------|---------|-----------|
| **TelescopeDB** | v1.0 | Provee FBCUs para procesar con lentes |
| **VoxelDB** | v1.0 | Provee templates para procesar |
| **MTT-DSL Engine** | v1.0 | MTT-DSL Lens integration |
| **Harmony Engine** | v1.0 | HarmonyLens integration |

### Crates Externos

```toml
[dependencies]
# Async
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
thiserror = "1.0"
anyhow = "1.0"
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

| Operación | Target | Medición | Status |
|-----------|--------|----------|--------|
| `validate_fbcu_compatibility()` | <5ms | Verificar fields, embedding, anchors | ⏸️ TBD |
| `process_with_lens()` | <30s | Ejecutar lens + validar quality | 🎯 CRÍTICO |
| `validate_quality_bounds()` | <10ms | Verificar coherence + agreement | ⏸️ TBD |
| **Registry lookup** | **<1ms** | **Encontrar lens en HashMap** | ⏸️ TBD |

---

## 🧪 ESTRATEGIA DE TESTING

```rust
// tests/lip_test.rs

#[tokio::test]
async fn test_harmony_lens_processing() {
    let lip_manager = create_test_lip_manager();
    
    // Mock FBCU con contexto musical
    let fbcu = create_mock_fbcu_with_music();
    
    let output = lip_manager
        .process_with_lens(&fbcu, "harmony_lens")
        .await
        .unwrap();
    
    assert_eq!(output.lens_id, "harmony_lens");
    assert!(matches!(output.validation_status, ValidationStatus::Passed));
    assert!(output.quality_metrics.coherence >= 0.80);
}
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/lip/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LipError {
    #[error("Lens not found: {0}")]
    LensNotFound(String),
    
    #[error("Missing field '{field}' required by lens '{lens_id}'")]
    MissingField { lens_id: String, field: String },
    
    #[error("Lens '{0}' timed out after {1}s")]
    LensTimeout(String, u64),
    
    #[error("Quality bound violation: {metric} = {value} < {min_required}")]
    QualityBoundViolation {
        metric: String,
        value: f64,
        min_required: f64,
    },
}

pub type Result<T> = std::result::Result<T, LipError>;
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md** - FBCUs para procesar
- **ROADMAP_V2/02_COMPONENTES/IMPORTANTES/MTT_DSL_TEMPLATES.md** - MTT-DSL Lens

### Decisiones Arquitectónicas

- **DA-029:** LIP - Lens Interface Protocol
- **DA-030:** Quality Bounds Validation
- **BITA-1:** Overlay System (§6)

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Semanas 15-16)

1. **Implementar LensInterface trait**
2. **Implementar LipManager (registry)**
3. **Implementar HarmonyLens**
4. **Implementar MTT-DSL Lens**
5. **Validación de quality bounds**

---

**Estado:** 📋 ESPECIFICACIÓN  
**Complejidad:** 🟡 MEDIA (Contract validation + Registry management)  
**Prioridad:** 🟡 MEDIA (Fase 3)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec*
