```yaml
Concepto: HumanRecognition Mini-LLM (BHR-v1)
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 23:52:00
Autor: Eduardo GJ (propuesta original)
Propósito: Concepto de mini-LLM entrenado localmente para identificación de personas
Estado: Diseño (v1.0-v2.0 telemetría) → Implementación (v2.5)
Tags: #ml #cost-optimization #local-inference #v2.5
Decisión: DA-TBD (pending formal approval)
```

# 🧠 HumanRecognition Mini-LLM (BHR-v1)

> **Subtítulo**: *"Bitácora Human Recognition v1 - Mini-LLM entrenado con telemetría propia para identificación local"*

> **Insight Eduardo (2025-11-29)**: "Recolectar parámetros que enviamos a LLM → entrenar nuestro propio mini-LLM → costo casi $0"

---

## ❓ Qué Es

**HumanRecognition Mini-LLM** es un modelo de aprendizaje automático pequeño (~100MB) entrenado específicamente para la tarea de **identificación de personas** en fotografías, usando como datos de entrenamiento la telemetría recolectada durante el uso real de Bitácora.

**NO es:**

- ❌ Vision LLM general (eso es GPT-4o, Gemini)
- ❌ Face recognition genérico (eso es dlib, FaceNet)
- ❌ Cloud API (esto es 100% local)

**SÍ es:**

- ✅ Mini-LLM especializado (single task: "Who is this?")
- ✅ Entrenado con datos reales de usuarios Bitácora (opt-in)
- ✅ Inferencia 100% local (0 cost per match)
- ✅ Fallback a GPT-4o cuando baja confianza

---

## 🤔 Por Qué Existe

### Problema Original

**Eduardo identifica (2025-11-29):**

```
Análisis de costos Bitácora v1.0:

Motor local: $2/month ✅
LLM APIs: $15/month ⚠️

Total: $17/month
Target: $7-9/month

Problema: LLM es 7.5x más caro que motor completo.
GPT-4o: $0.01 per image match (ongoing)
Uso esperado: 1,500 matches/month → $15

¿Solución? Local inference → $0 ongoing
```

**Eduardo pregunta:**

> "¿Por qué no recolectar los parámetros que enviamos al LLM,  
> entrenar nuestro propio mini-LLM,  
> y hacer matching 100% local?"

**Respuesta:** BHR-v1 (Bitácora Human Recognition v1)

### Motivación Económica

**Cost Model Comparison:**

| Strategy | Training Cost | Per-Match Cost | 500k Matches Lifetime | Monthly (1.5k) |
|----------|--------------|----------------|----------------------|----------------|
| **GPT-4o (cloud)** | $0 | $0.01 | $5,000 | $15 |
| **BHR-v1 (local)** | $50 one-time | $0 | $50 | $0 |

**Amortized cost:** $50 / 500,000 matches = **$0.0001 per match**

**Savings:** 98% reduction ($15/month → $0/month after training)

**Break-even:** 5,000 matches (~3 months)

### Motivación Técnica

**Ventajas adicionales:**

1. **Privacy**: 0 data leaves device (face embeddings 100% local)
2. **Speed**: No network latency (~50ms vs 2-3 seconds)
3. **Offline**: Works without internet
4. **Scalability**: Cost doesn't grow with users

---

## 🏗️ Cómo Funciona

### Fase 1: Telemetría (v1.0-v2.0, 2026 Q1-Q2)

**Recolección pasiva de parámetros:**

```rust
// Cada vez que usuario confirma identidad
pub struct TelemetryEntry {
    // Vision LLM analysis (ya pagado)
    face_embedding: Vec<f32>,        // [512 dims]
    age_estimate: u8,
    gender_estimate: Gender,
    emotion_snapshot: Vec<String>,
    clothing_descriptors: Vec<String>,
    scene_context: String,
    
    // Usuario confirma
    confirmed_identity: String,      // "Mamá", "Hermano Carlos"
    confidence_reported: f32,        // 0.0-1.0
    
    // Contexto conversacional
    conversation_themes: Vec<String>,
    emotional_space_label: String,
    
    // Metadata
    timestamp: DateTime<Utc>,
    user_id_anonymous: Hash,         // Anonymized
}

// Opción usuario
let telemetry_enabled = config.human_recognition.telemetry_enabled;
if telemetry_enabled {
    telemetry_db.append(entry)?;
}
```

**CONFIG_PARAMETERS:**

```toml
[human_recognition]
telemetry_enabled = true          # Opt-in (default false)
telemetry_retention_days = 365    # 1 year collection
telemetry_anonymous = true        # Hash user IDs
```

**Storage:**

- Local YAML: `~/.bitacora/telemetry/human_recognition.yaml`
- Size: ~500 bytes/entry → 1,500 entries = 750KB
- **NO sale del dispositivo** sin consentimiento explícito

### Fase 2: Entrenamiento (v2.5, 2026 Q3)

**Proceso:**

1. **Aggregation** (Bitácora Corp):
   - Usuarios opt-in envían telemetría (anonymized)
   - Bitácora Corp agrega 100k+ entries
   - Dataset: `{ face_embedding, context } → identity_label`

2. **Training** (Cloud GPUs):
   - Architecture: Lightweight transformer (~100M params)
   - Task: Multi-class classification (top-20 people per user + "unknown")
   - Cost: $50 one-time (4h A100 GPU)

3. **Model Distribution**:
   - `bhr-v1.onnx` (100MB ONNX model)
   - Download via Bitácora updates
   - Stored: `~/.bitacora/models/bhr-v1.onnx`

**Training Pipeline:**

```python
# Pseudocode (not implemented yet)
def train_bhr_v1(telemetry_dataset):
    # Input: face_embedding [512] + context [128]
    # Output: identity probabilities [num_identities]
    
    model = LightweightTransformer(
        input_dims=640,          # 512 face + 128 context
        hidden_dims=256,
        num_layers=4,
        num_heads=8,
        output_classes=num_identities
    )
    
    # Train with cross-entropy loss
    optimizer = AdamW(lr=1e-4)
    train(model, telemetry_dataset, epochs=10)
    
    # Export to ONNX
    export_onnx(model, "bhr-v1.onnx")
```

### Fase 3: Inferencia (v2.5+, 2026 Q3+)

**Hybrid Matching Strategy:**

```rust
pub struct HumanRecognitionEngine {
    bhr_model: Option<BHRModel>,     // Local mini-LLM
    vision_llm: VisionLLMClient,     // GPT-4o fallback
    confidence_threshold: f32,        // 0.85 default
}

impl HumanRecognitionEngine {
    pub async fn identify_person(
        &self,
        face_embedding: &[f32],
        context: &ConversationContext,
    ) -> Result<IdentityMatch> {
        // 1. Try BHR-v1 (local, $0)
        if let Some(model) = &self.bhr_model {
            let prediction = model.predict(face_embedding, context)?;
            
            if prediction.confidence > self.confidence_threshold {
                return Ok(IdentityMatch {
                    identity: prediction.identity,
                    confidence: prediction.confidence,
                    method: MatchMethod::BHRLocal,
                });
            }
        }
        
        // 2. Fallback to Vision LLM (cloud, $0.01)
        let vision_result = self.vision_llm
            .identify_from_embedding(face_embedding, context)
            .await?;
            
        Ok(IdentityMatch {
            identity: vision_result.identity,
            confidence: vision_result.confidence,
            method: MatchMethod::VisionLLMFallback,
        })
    }
}
```

**Performance Target:**

- **BHR-v1 hit rate**: 85% (15% fallback to GPT-4o)
- **Effective cost**: $0 * 0.85 + $0.01 * 0.15 = **$0.0015/match**
- **vs GPT-4o only**: 85% savings even with fallback

---

## 📍 Dónde Aparece

### Documentos Principales

#### 1. **18.4_bqm-identity-system-v1.md**
   - **Archivo**: [ROADMAP_V2/02_COMPONENTES/18.4_bqm-identity-system-v1.md](../../ROADMAP_V2/02_COMPONENTES/18.4_bqm-identity-system-v1.md)
   - **Sección**: "Future Evolution: HumanRecognition Mini-LLM"
   - **Rol**: Propuesta completa + cost analysis
   - **Lines**: ~720-780
   - **Contenido**:
     * Telemetry collection strategy
     * Training pipeline description
     * Cost model ($50 training / 500k matches)
     * Hybrid matching (BHR + GPT-4o fallback)

#### 2. **18.5_bqm-quantum-identity-vision-v2.md**
   - **Archivo**: [ROADMAP_V2/00_VISION/18.5_bqm-quantum-identity-vision-v2.md](../../ROADMAP_V2/00_VISION/18.5_bqm-quantum-identity-vision-v2.md)
   - **Sección**: Menciones v2.5
   - **Rol**: Roadmap positioning (v2.5 feature)
   - **Contenido**: BHR-v1 como parte de cost optimization roadmap

### Configuración

#### 3. **CONFIG_PARAMETERS.md**
   - **Archivo**: [CONFIG_PARAMETERS.md](../../CONFIG_PARAMETERS.md)
   - **Sección**: "LLM & AI Services"
   - **Parámetros**:
     * `human_recognition.telemetry_enabled` (bool, default false)
     * `human_recognition.telemetry_retention_days` (int, default 365)
     * `human_recognition.telemetry_anonymous` (bool, default true)
     * `human_recognition.confidence_threshold` (float, default 0.85)
     * `human_recognition.fallback_to_vision_llm` (bool, default true)

### Código (Futuro v2.5)

#### 4. **src/ml/human_recognition.rs** (NOT YET IMPLEMENTED)
   - **Planned path**: `src/ml/human_recognition.rs`
   - **Structs**:
     * `TelemetryEntry`
     * `BHRModel` (ONNX wrapper)
     * `HumanRecognitionEngine`
   - **Implementation**: ~600 lines (v2.5, 2026 Q3)

#### 5. **src/ml/bhr_inference.rs** (NOT YET IMPLEMENTED)
   - **Planned path**: `src/ml/bhr_inference.rs`
   - **Purpose**: ONNX model loading + inference
   - **Dependencies**: `onnxruntime` crate
   - **Implementation**: ~300 lines (v2.5)

---

## 🔗 Conceptos Relacionados

### Identidad & Reconocimiento

- **[[bqm-quantum-masks]]** - Sistema de identidad base que BHR mejora
  * BHR mejora LocalIdentity matching (Q-Soul derivation más rápida)
  * Confidence threshold determina si usar BHR o Vision LLM

- **[[identity-consent-flow]]** - CONSENT-FIRST philosophy
  * Telemetry collection requires explicit opt-in
  * User can revoke telemetry anytime
  * Trained model is public (no privacy risk), but data collection is private

### Optimización & Costos

- **[[cost-optimization]]** (CROSS_REFERENCE) - Estrategia general de costos
  * BHR-v1 es pilar de cost reduction (98% savings)
  * Complementa caching, batching, analysis frequency
  * Target: $17/month → $7-9/month (v2.5)

### LLM & Estrategias

- **[[llm-strategies]]** (CROSS_REFERENCE) - Uso de LLMs en Bitácora
  * Vision LLM (GPT-4o): General image analysis
  * BHR-v1: Specialized task (identity only)
  * Hybrid approach: Local-first + cloud fallback

### Plataforma & Performance

- **[[battery-aware-processing]]** - Mobile considerations
  * BHR-v1 inference: ~50ms (vs 2-3s cloud)
  * Lower battery impact (no network)
  * Offline capability

---

## 📈 Evolución

### Timeline

#### **2025-11-29 23:00** - Propuesta Original
- **Context**: Eduardo analiza cost model ($2 motor + $15 LLM)
- **Insight**: "Recolectar parámetros LLM → entrenar propio mini-LLM"
- **Decision**: Diseñar HumanRecognition mini-LLM strategy
- **Document**: 18.4 sección "Future Evolution"

#### **v1.0-v1.5** (2026 Q1-Q2) - Telemetry Collection
- **Goal**: Recolectar 100k+ telemetry entries
- **Implementation**: 
  * TelemetryEntry struct
  * Local storage (YAML)
  * Opt-in consent flow
  * CONFIG parameters
- **Output**: Telemetry dataset ready for training

#### **v2.5** (2026 Q3) - BHR-v1 Training & Deployment
- **Goal**: Train first mini-LLM + deploy
- **Implementation**:
  * Aggregate anonymized telemetry (Bitácora Corp)
  * Train BHR-v1 (~100M params)
  * Distribute ONNX model (100MB)
  * Integrate inference pipeline
- **Cost**: $50 one-time training
- **Performance**: 85% hit rate target

#### **v3.0** (2026 Q4) - Hybrid Optimization
- **Goal**: Fine-tune hybrid strategy
- **Improvements**:
  * Per-user fine-tuning (optional)
  * Confidence threshold tuning
  * Fallback optimization (reduce to 10%)
- **Target hit rate**: 90%+

#### **v4.0+** (2027+) - Multi-Modal
- **Goal**: Expand beyond faces
- **Features**:
  * Voice recognition (audio snippets)
  * Handwriting recognition (notes)
  * Behavioral patterns (typing, speech)
- **Vision**: Holistic identity recognition

---

## 🎨 Estado Actual

### Fase

- **Diseño**: ✅ Complete (2025-11-29)
- **Telemetry Collection**: ⏳ Pending (v1.0-v2.0)
- **Training**: ⏳ Pending (v2.5)
- **Deployment**: ⏳ Pending (v2.5)

### Prioridad

- **v1.0-v2.0**: BAJA (telemetry passive, no urgency)
- **v2.5**: ALTA (critical cost optimization)
- **v3.0+**: MEDIA (optimization, not blocker)

### Owner

- **Propuesta**: Eduardo GJ
- **Diseño**: Eduardo + Claude
- **Implementación**: TBD (v2.5 team)

### Blockers

- ✅ None (design complete)
- ⏳ Need telemetry collection (v1.0-v2.0)
- ⏳ Need 100k+ entries for training (12-18 months)

---

## 💡 Preguntas Frecuentes

### P1: ¿Por qué el costo no es $0 si es local?

**R:** El costo de inferencia es $0 (100% local). El "$0.0001/match" es el costo **amortizado de entrenamiento**:

- Training cost: $50 one-time
- Expected lifetime: 500k matches
- Amortized: $50 / 500,000 = $0.0001 per match

Después de 5,000 matches (~3 meses), BHR-v1 se paga a sí mismo vs GPT-4o.

### P2: ¿Qué pasa si BHR-v1 falla?

**R:** Hybrid fallback strategy:

```
1. Try BHR-v1 (local, $0)
   - Confidence > 0.85? → Use result
   
2. Fallback to GPT-4o (cloud, $0.01)
   - Confidence < 0.85? → Use Vision LLM
```

User never notices. Worst case: 15% matches use GPT-4o ($0.0015 average vs $0.01 pure cloud).

### P3: ¿Qué datos se envían a Bitácora Corp?

**R:** NADA sin consentimiento explícito.

- Telemetry collection: **Opt-in** (default OFF)
- Data sent: **Anonymized** (hashed user IDs)
- Face embeddings: **Vectors only** (no raw images)
- Trained model: **Public** (no privacy risk)

User controls todo desde CONFIG_PARAMETERS.md.

### P4: ¿Funciona offline?

**R:** Sí (después de model download):

- BHR-v1 model: 100MB one-time download
- Inference: 100% local (no network)
- Fallback: Requires internet (GPT-4o)

User puede desactivar fallback → 100% offline con degraded accuracy.

### P5: ¿Qué tan preciso es vs GPT-4o?

**R:** Target (v2.5):

- **BHR-v1 alone**: 85% accuracy (specialized task)
- **GPT-4o**: 95% accuracy (general vision)
- **Hybrid (BHR + GPT fallback)**: 94% accuracy (best of both)

Trade-off: -1% accuracy for 98% cost savings.

---

## 📊 Métricas de Éxito (v2.5)

### Telemetry Collection (v1.0-v2.0)

- ✅ 100k+ telemetry entries collected
- ✅ Opt-in rate: >30% users
- ✅ Dataset quality: >90% valid entries

### Training (v2.5)

- ✅ Model size: <150MB
- ✅ Training cost: <$100
- ✅ Inference latency: <100ms

### Deployment (v2.5+)

- ✅ Hit rate: >85% (BHR-v1 alone)
- ✅ Hybrid accuracy: >94%
- ✅ Cost reduction: >95% vs GPT-4o
- ✅ User satisfaction: >90% (no perceived degradation)

---

## 🚀 Next Steps

### Immediate (v1.0, Week 3-4)

1. ✅ Design complete (this document)
2. ⏳ Create `TelemetryEntry` struct (Phase 7.x.3)
3. ⏳ Add CONFIG parameters
4. ⏳ Implement telemetry collection (opt-in flow)
5. ⏳ Local storage (YAML append)

### Short-term (v1.5-v2.0, Q1-Q2 2026)

6. ⏳ Monitor telemetry collection
7. ⏳ Analyze data quality
8. ⏳ Estimate training timeline (need 100k entries)

### Mid-term (v2.5, Q3 2026)

9. ⏳ Aggregate telemetry (Bitácora Corp)
10. ⏳ Train BHR-v1 (cloud GPUs)
11. ⏳ Distribute model (ONNX 100MB)
12. ⏳ Implement inference pipeline
13. ⏳ Test hybrid strategy
14. ⏳ Deploy to users

### Long-term (v3.0+, Q4 2026+)

15. ⏳ Optimize hit rate (>90%)
16. ⏳ Per-user fine-tuning
17. ⏳ Multi-modal expansion (voice, handwriting)

---

## 🧩 Integration Points

### Input Dependencies

- **ImageAnalyzer** (18.2): Face embeddings extraction
- **VisionLLMClient**: Fallback matching
- **ConversationContext**: Context for inference
- **ConsentTracker** (18.4): Telemetry opt-in

### Output Dependencies

- **IdentityManager** (18.4): Consumes IdentityMatch results
- **IdentityLinker** (18.4): Uses confidence scores
- **CONFIG_PARAMETERS**: User controls telemetry + thresholds

---

## 📚 Referencias

### Documentos

- [18.4 BQM Identity System v1.0](../../ROADMAP_V2/02_COMPONENTES/18.4_bqm-identity-system-v1.md#future-evolution-humanrecognition-mini-llm)
- [18.5 BQM Quantum Vision v2.0](../../ROADMAP_V2/00_VISION/18.5_bqm-quantum-identity-vision-v2.md)
- [CONFIG_PARAMETERS.md](../../CONFIG_PARAMETERS.md#llm-ai-services)
- [CROSS_REFERENCES/cost-optimization.md](../CROSS_REFERENCES/cost-optimization.md)

### Papers & Tech

- FaceNet (face embeddings): [Schroff et al. 2015]
- Lightweight Transformers: [MobileViT, EfficientFormer]
- ONNX Runtime: [Microsoft ONNX docs]

### Decisiones

- **DA-TBD**: HumanRecognition Mini-LLM Architecture (pending formal approval)

---

**Tags**: `#ml` `#cost-optimization` `#local-inference` `#v2.5` `#eduardo-insight`  
**Decisión**: DA-TBD  
**Última Actualización**: 2025-11-29 23:52:00  
**Mantenedores**: Eduardo GJ + Claude  

🧠✨💎
