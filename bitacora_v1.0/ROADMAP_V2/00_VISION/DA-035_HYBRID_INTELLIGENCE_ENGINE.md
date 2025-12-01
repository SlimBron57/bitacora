```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/00_VISION/DA-035_HYBRID_INTELLIGENCE_ENGINE.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 18:00:00
Autor: Sistema Bitácora + Eduardo
Propósito: Definir arquitectura híbrida Local + LLM con privacy-first approach
Estado: 🎯 ACTIVO - Visión estratégica para Phase 7.x.3+
Relacionado Con:
  - DA-033_DYNAMIC_TOPIC_TONE_SYSTEM.md (TopicGraph + EmotionalSpace)
  - 01_ARQUITECTURA/13_hybrid-intelligence-engine.md (arquitectura técnica)
  - 02_COMPONENTES/09_relationship-psychology-analyzer.md (componente)
  - CHECKLIST_V2.md v2.36 (Phase 7.x.3 Extraction)
Categoría: CORE ARCHITECTURE
Prioridad: ALTA
# === FIN DATOS DE AUDITORÍA ===
```

# 🧠 DA-035: Hybrid Intelligence Engine

> **"No ser obeso digital — Ser consciente, selectiva, curadora"**
>
> **Filosofía:** Local-first, LLM-assisted cuando necesario, Privacy-preserving siempre

---

## 📋 ÍNDICE

1. [Contexto](#contexto)
2. [Problema](#problema)
3. [Solución](#solución)
4. [Arquitectura Conceptual](#arquitectura-conceptual)
5. [Principios de Diseño](#principios-de-diseño)
6. [Flujo de Decisión](#flujo-de-decisión)
7. [Economía del Sistema](#economía-del-sistema)
8. [Privacy & Security](#privacy--security)
9. [Casos de Uso](#casos-de-uso)
10. [Roadmap de Implementación](#roadmap-de-implementación)
11. [Referencias](#referencias)

---

## 🎯 CONTEXTO

### Estado Actual (Phase 7.x.3)

**Bitácora v1.0** ha alcanzado un milestone crítico:

```
✅ Ingestion completa (QuarantineZone)
   • 1,354 mensajes procesados (100% success)
   • Performance: 26ms (2,308x faster than target)
   
✅ Digestion completa (WhatsAppDigester)
   • Parser production-ready
   • Multiline, attachments, real formats
   
✅ Extraction parcial (2/7 dimensiones)
   • InterestExtractor: 348 nutrients (keywords + URLs)
   • EmotionalExtractor: 826 nutrients (sentiment)
   • Performance: 71ms total (141x faster than target)
```

### El Dilema

Al analizar 839 mensajes de texto reales, encontramos:

```
📊 Confidence Distribution:
   • Alta confianza (≥0.7): 785 mensajes (93.6%)
     - "Te amo" → Positive (0.95) ✅
     - "😍😍😍" → Positive (0.90) ✅
   
   • Baja confianza (<0.7): 54 mensajes (6.4%)
     - "Nos vemos luego" → Neutral (0.35) ⚠️
     - "Ciao" → ??? (0.20) ⚠️
     - Sarcasmo, ironía, contexto cultural
```

**Pregunta crítica:** ¿Qué hacemos con el 6.4% ambiguo?

---

## 🚨 PROBLEMA

### Opción A: Solo Local (Status Quo)

```rust
Pros:
✅ 100% privado
✅ $0.00 costo
✅ Latencia ultra-baja (<100ms)
✅ Offline-capable

Cons:
❌ 6.4% de mensajes mal clasificados
❌ No maneja sarcasmo, ironía, contexto cultural
❌ Lexicon-based limitado
❌ Sin aprendizaje continuo
```

**Caso real:**
```
Mensaje: "Claro que sí campeón 🙄"
Local analysis: Positive (0.6) ← "claro que sí" + emoji
Reality: Sarcastic/Negative ← 🙄 es sarcasmo
```

### Opción B: Solo LLM

```rust
Pros:
✅ Alta precisión (>95%)
✅ Entiende contexto, sarcasmo, cultura
✅ Aprendizaje continuo (modelos actualizados)

Cons:
❌ PRIVACY NIGHTMARE (datos a cloud)
❌ Costo alto ($4.50 per 839 msgs)
❌ Latencia alta (30-60s)
❌ Requiere internet
❌ Vendor lock-in
```

**Costo proyectado:**
```
Usuario promedio: 10 chats × 800 msgs = 8,000 msgs/mes
GPT-4 API: $45.00/mes
Claude 3: $4.50/mes
Llama 3 (cloud): $0.80/mes
```

### Opción C: Híbrido Ingenuo

```rust
Usar LLM para todo el 6.4% ambiguo sin optimización:

54 queries × $0.001 = $0.054/chat
10 chats/mes = $0.54/mes

Problema:
❌ Aún expone datos privados
❌ No reutiliza aprendizajes
❌ Latencia variable
```

---

## 💡 SOLUCIÓN

### Hybrid Intelligence Engine

**3-Layer Architecture:**

```
┌──────────────────────────────────────────────────┐
│ LAYER 1: Local Processing (Default)             │
│ • 93.6% de mensajes                              │
│ • <100ms latency                                 │
│ • $0.00 cost                                     │
│ • 100% private                                   │
└────────────┬─────────────────────────────────────┘
             │ (confidence < 0.7)
             ▼
┌──────────────────────────────────────────────────┐
│ LAYER 2: Anonymization Shield                   │
│ • Remove: names, locations, identifiers         │
│ • Preserve: structure, context, patterns        │
│ • Output: Anonymous query                       │
└────────────┬─────────────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────────────┐
│ LAYER 3: LLM Query (Fallback)                   │
│ • 6.4% de mensajes ambiguos                     │
│ • ~500ms latency (acceptable)                   │
│ • ~$0.001/query                                  │
│ • Anonymous data only                           │
└────────────┬─────────────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────────────┐
│ LAYER 4: Cache & Learning                       │
│ • Pattern: "nos vemos luego" → Neutral          │
│ • Next occurrence → Use cached (no LLM)         │
│ • Build local knowledge base                    │
│ • Reduce LLM dependency over time               │
└──────────────────────────────────────────────────┘
```

### Key Innovations

1. **Confidence Scoring**
   - Cada análisis local retorna confidence (0.0-1.0)
   - Threshold dinámico (default: 0.7)
   - Usuario puede ajustar (privacy vs accuracy trade-off)

2. **Privacy-Preserving Anonymization**
   - Remove: `"Paula"` → `"Person A"`
   - Remove: `"Gainesville"` → `"[LOCATION]"`
   - Preserve: Linguistic structure, sentiment signals
   - Preserve: Aggregated metadata (non-identifying)

3. **Intelligent Caching**
   - LLM response para `"ciao"` → Cache
   - Próxima vez `"ciao"` → Local cache (no LLM)
   - Cache expiry (30 days)
   - Cache sharing (opt-in, anonymized)

4. **Progressive Learning**
   - LLM responses → Expand local lexicon
   - User corrections → Update confidence weights
   - A/B testing (local vs LLM accuracy)

---

## 🏗️ ARQUITECTURA CONCEPTUAL

### Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                  NutrientExtractor                      │
│                  (Base Interface)                       │
└────────────┬────────────────────────────────────────────┘
             │
             ├─────────────────┬──────────────────┬────────
             ▼                 ▼                  ▼
    ┌─────────────────┐ ┌──────────────┐ ┌─────────────┐
    │ InterestExtract │ │ EmotionalExt │ │ Biography   │
    │ (Keywords/URLs) │ │ (Sentiment)  │ │ (Identity)  │
    └────────┬────────┘ └──────┬───────┘ └──────┬──────┘
             │                  │                 │
             ▼                  ▼                 ▼
    ┌────────────────────────────────────────────────────┐
    │      HybridIntelligenceEngine (NEW)                │
    ├────────────────────────────────────────────────────┤
    │  • Local analyzer (lexicon-based)                  │
    │  • Confidence scorer                               │
    │  • Anonymizer                                      │
    │  • LLM client (optional)                           │
    │  • Cache manager                                   │
    │  • Learning engine                                 │
    └────────────────────────────────────────────────────┘
             │
             ├───────────────────┬──────────────────
             ▼                   ▼
    ┌─────────────────┐  ┌─────────────────┐
    │  Local Lexicon  │  │  LLM Providers  │
    │  • Stopwords    │  │  • OpenAI       │
    │  • Sentiment    │  │  • Anthropic    │
    │  • Patterns     │  │  • Local LLaMA  │
    │  • Learned      │  │  • Groq         │
    └─────────────────┘  └─────────────────┘
```

### Data Flow

```
Input: DigestedEntry
     │
     ▼
┌─────────────────────────────────────┐
│ 1. Local Analysis                   │
│    analyze_sentiment_with_confidence│
│    → {sentiment, confidence}        │
└────────────┬────────────────────────┘
             │
             ├─ confidence ≥ 0.7 ────▶ Return (Local)
             │
             └─ confidence < 0.7
                     │
                     ▼
            ┌─────────────────────────┐
            │ 2. Check Cache          │
            │    cache_key = hash(msg)│
            └────────┬────────────────┘
                     │
                     ├─ Cache HIT ────▶ Return (Cached)
                     │
                     └─ Cache MISS
                             │
                             ▼
                    ┌────────────────────┐
                    │ 3. Anonymize       │
                    │    remove_pii()    │
                    └────────┬───────────┘
                             │
                             ▼
                    ┌────────────────────┐
                    │ 4. Query LLM       │
                    │    (anonymous)     │
                    └────────┬───────────┘
                             │
                             ▼
                    ┌────────────────────┐
                    │ 5. Cache Result    │
                    │    + Learn         │
                    └────────┬───────────┘
                             │
                             ▼
                        Return (LLM)
```

---

## 🎨 PRINCIPIOS DE DISEÑO

### 1. **Privacy-First**

```
NO NEGOCIABLE:
• Datos privados NUNCA salen sin anonimización
• Usuario controla si habilita LLM (opt-in)
• Modo "offline-only" siempre disponible
• Transparencia total en qué se envía
```

### 2. **Cost-Conscious**

```
Budget Control:
• Max queries/session (default: 10)
• Max queries/mes (default: 100)
• Cost tracking en tiempo real
• Alertas cuando se acerca al límite
```

### 3. **Graceful Degradation**

```
Si LLM no disponible:
• Sistema funciona 100% local
• Marca resultados como "lower confidence"
• Usuario puede corregir manualmente
• Correcciones → Learning engine
```

### 4. **Progressive Enhancement**

```
Mejora continua:
• Cada corrección usuario → Update weights
• Cada respuesta LLM → Expand lexicon
• Cache compartido (opt-in) → Collective learning
• A/B testing → Optimizar threshold
```

### 5. **Transparent Intelligence**

```
Usuario siempre sabe:
• ¿Análisis local o LLM?
• Confidence score (0.0-1.0)
• Cost por query ($0.001)
• Cache hit/miss
• Privacy level (anonymization)
```

---

## 🔀 FLUJO DE DECISIÓN

### Decision Tree

```
                ┌─────────────────┐
                │ New Message     │
                └────────┬────────┘
                         │
                         ▼
            ┌────────────────────────┐
            │ Local Analyzer         │
            │ (lexicon + patterns)   │
            └────────┬───────────────┘
                     │
         ┌───────────┴───────────┐
         │                       │
    confidence ≥ 0.7        confidence < 0.7
         │                       │
         ▼                       ▼
    ┌─────────┐         ┌───────────────┐
    │ USE     │         │ LLM Enabled?  │
    │ LOCAL   │         └───────┬───────┘
    └─────────┘                 │
                     ┌──────────┴──────────┐
                     │                     │
                   YES                    NO
                     │                     │
                     ▼                     ▼
            ┌─────────────────┐    ┌──────────┐
            │ Check Budget    │    │ USE      │
            └────────┬────────┘    │ LOCAL    │
                     │             │ (warn)   │
         ┌───────────┴────────┐   └──────────┘
         │                    │
    Budget OK          Budget Exceeded
         │                    │
         ▼                    ▼
   ┌──────────┐        ┌──────────┐
   │ Check    │        │ USE      │
   │ Cache    │        │ LOCAL    │
   └────┬─────┘        └──────────┘
        │
    ┌───┴────┐
    │        │
  Hit      Miss
    │        │
    ▼        ▼
 ┌─────┐  ┌──────────┐
 │ USE │  │ Anonymize│
 │CACHE│  │ + Query  │
 └─────┘  │ LLM      │
          └────┬─────┘
               │
               ▼
          ┌─────────┐
          │ Cache + │
          │ Return  │
          └─────────┘
```

### Threshold Tuning

```yaml
Privacy Mode (threshold: 0.9):
  description: Máxima privacidad
  local_usage: ~99%
  llm_usage: ~1%
  accuracy: ~85%
  cost: <$0.01/chat

Balanced Mode (threshold: 0.7):
  description: Default - balance óptimo
  local_usage: ~93%
  llm_usage: ~7%
  accuracy: ~94%
  cost: ~$0.05/chat

Accuracy Mode (threshold: 0.5):
  description: Máxima precisión
  local_usage: ~80%
  llm_usage: ~20%
  accuracy: ~98%
  cost: ~$0.20/chat
```

---

## 💰 ECONOMÍA DEL SISTEMA

### Cost Analysis (839 messages)

```
┌─────────────────────────────────────────────────────┐
│ SCENARIO A: 100% Local (Current)                    │
├─────────────────────────────────────────────────────┤
│ • Queries: 839                                      │
│ • LLM calls: 0                                      │
│ • Cost: $0.00                                       │
│ • Accuracy: ~87% (estimated)                        │
│ • Latency: 71ms total                               │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│ SCENARIO B: 100% LLM                                │
├─────────────────────────────────────────────────────┤
│ • Queries: 839                                      │
│ • LLM calls: 839                                    │
│ • Cost: $4.50 (GPT-4) / $0.45 (Claude-3)           │
│ • Accuracy: ~98%                                    │
│ • Latency: ~40s total                               │
└─────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────┐
│ SCENARIO C: Hybrid (Threshold 0.7)                  │
├─────────────────────────────────────────────────────┤
│ • Local high-conf: 785 (93.6%) → $0.00             │
│ • LLM queries: 54 (6.4%)                            │
│   - First time: 54 × $0.001 = $0.054               │
│   - Cached: 15 (27%) → $0.00                        │
│   - Actual LLM: 39 × $0.001 = $0.039                │
│ • Total cost: $0.039/chat                           │
│ • Accuracy: ~94%                                    │
│ • Latency: 71ms (local) + 500ms (LLM avg)          │
│ • Savings vs full-LLM: 98.8%                        │
└─────────────────────────────────────────────────────┘
```

### Scaling Projection

```
Mensual (10 chats × 800 msgs):
──────────────────────────────
100% Local:   $0.00/mes
Hybrid:       $0.39/mes  (↓99% vs GPT-4)
100% GPT-4:   $45.00/mes
100% Claude:  $4.50/mes

Anual (120 chats × 800 msgs):
──────────────────────────────
100% Local:   $0.00/año
Hybrid:       $4.68/año
100% GPT-4:   $540.00/año
100% Claude:  $54.00/año

1M usuarios (projected):
──────────────────────────────
100% Local:   $0
Hybrid:       $4.68M/año (manageable con freemium)
100% GPT-4:   $540M/año (insostenible)
```

---

## 🔐 PRIVACY & SECURITY

### Anonymization Strategy

#### Level 1: Maximum (Default)

```
Original:
"Paula, nos vemos mañana en el café de Gainesville a las 3pm. 
Mi número es 352-555-1234. Avísame a edgi@example.com"

Anonymized:
"Person A says: nos vemos mañana en el café de [LOCATION] a las [TIME].
Mi número es [PHONE]. Avísame a [EMAIL]"

Context (non-identifying):
- Message #145 in 6-month conversation
- 2 participants
- Previous sentiment: 75% positive
- Frequency: Daily
```

#### Level 2: High

```
Anonymized + Temporal context:
"Person A says (Day 145 of 178): nos vemos mañana..."

Additional context:
- Current week sentiment: 80% positive
- Recent topic cluster: "social plans"
- Activity pattern: Afternoon peak
```

#### Level 3: Medium (Opt-in)

```
Anonymized + Cultural context:
"Spanish-speaking Person A says: nos vemos mañana..."

Additional context:
- Language: Spanish (Spain/LatAm)
- Time zone: EST
- Platform: WhatsApp
```

### PII Removal

```rust
Removed Always:
• Full names (Person A, Person B)
• Phone numbers ([PHONE])
• Email addresses ([EMAIL])
• Physical addresses ([ADDRESS])
• Credit card numbers ([CC])
• Social security numbers ([SSN])
• URLs with tokens ([URL_TOKENIZED])
• Geo-coordinates ([GEO])

Preserved (Safe):
• Linguistic structure
• Sentiment indicators
• Common nouns
• Temporal patterns (aggregated)
• Conversation metadata (counts, not content)
```

### Audit Trail

```yaml
LLM Query Log:
  timestamp: 2025-11-29T18:30:00Z
  query_id: "abc123"
  original_msg_id: "msg-456" (hashed)
  anonymization_level: "Maximum"
  llm_provider: "claude-3-sonnet"
  query_text: "[ANONYMIZED]"
  response: "Neutral"
  confidence: 0.85
  cost_usd: 0.001
  user_consent: true
  cache_stored: true
```

---

## 🎯 CASOS DE USO

### Caso 1: Conversación Simple (93% local)

```
Chat: Pareja romántica (Paula & Eduardo)
Mensajes: 839 text

Resultados:
• Local: 785 msgs (93.6%)
  - "Te amo" → Positive (0.95)
  - "😍😍" → Positive (0.90)
  - "Buenos días amor" → Positive (0.88)
  
• LLM: 54 msgs (6.4%)
  - "Nos vemos luego" → Neutral (cache)
  - "Claro campeón 🙄" → Sarcastic/Negative
  - "jajaja ntp" → Positive/Casual (slang)

Cost: $0.039 (vs $4.50 full-LLM)
Accuracy: 94% (vs 87% local-only)
```

### Caso 2: Conversación Profesional

```
Chat: Cliente & Contractor
Mensajes: 450 text

Características:
• Lenguaje formal
• Sin emojis
• Contexto técnico

Resultados:
• Local: 280 msgs (62%)
  - "Excelente trabajo" → Positive (0.85)
  - "Necesito revisión" → Neutral (0.75)
  
• LLM: 170 msgs (38%)
  - Sarcasmo implícito
  - Ironía profesional
  - Contexto técnico ambiguo

Cost: $0.17
Accuracy: 96%
```

### Caso 3: Grupo Familiar

```
Chat: 5 participantes
Mensajes: 2,400 text

Características:
• Multilingüe (ES/EN)
• Generacional (diferentes edades)
• Cultural context fuerte

Resultados:
• Local: 1,800 msgs (75%)
• LLM: 600 msgs (25%)
  - Slang generacional
  - Code-switching ES/EN
  - Referencias culturales

Cost: $0.60
Accuracy: 95%
```

---

## 🗺️ ROADMAP DE IMPLEMENTACIÓN

### Phase 1: Foundation (Week 1-2) ✅ CURRENT

```yaml
Tasks:
  - [x] NutrientExtractor trait
  - [x] InterestExtractor (local-only)
  - [x] EmotionalExtractor (local-only)
  - [x] Confidence scoring base
  - [ ] AnalysisResult<T> structure
```

### Phase 2: Hybrid Core (Week 3-4)

```yaml
Tasks:
  - [ ] HybridIntelligenceEngine struct
  - [ ] Confidence threshold system
  - [ ] Anonymization engine
  - [ ] PII detection & removal
  - [ ] LLM client abstraction (multi-provider)
  - [ ] Cache layer (SQLite)
  - [ ] Budget tracking
  - [ ] Tests con datos reales
```

### Phase 3: LLM Integration (Week 5-6)

```yaml
Providers:
  - [ ] OpenAI GPT-4 client
  - [ ] Anthropic Claude-3 client
  - [ ] Groq (fast inference) client
  - [ ] Local LLaMA (Ollama) client
  - [ ] Fallback chain (primary → backup)
  - [ ] Rate limiting
  - [ ] Error handling & retry
```

### Phase 4: Learning Engine (Week 7-8)

```yaml
Tasks:
  - [ ] User corrections API
  - [ ] Weight updating (lexicon + patterns)
  - [ ] A/B testing framework
  - [ ] Cache optimization
  - [ ] Pattern extraction from LLM responses
  - [ ] Local lexicon expansion
  - [ ] Performance monitoring
```

### Phase 5: UI & UX (Week 9-10)

```yaml
Tasks:
  - [ ] Settings UI (threshold, budget, providers)
  - [ ] Transparency dashboard
  - [ ] Cost tracking UI
  - [ ] Confidence visualization
  - [ ] Manual correction interface
  - [ ] Privacy report
  - [ ] Audit log viewer
```

---

## 📚 REFERENCIAS

### Academic

- **Federated Learning** (Google, 2016)
  - Privacy-preserving ML
  - On-device training
  
- **Differential Privacy** (Apple, Microsoft)
  - Anonymous data aggregation
  - Privacy guarantees

### Industry

- **Apple Intelligence** (2024)
  - On-device + cloud hybrid
  - Private Cloud Compute
  
- **Google Gemini Nano** (2024)
  - On-device LLM
  - Hybrid orchestration

### Related Work

- DA-033: Dynamic Topic/Tone System
- DA-034: Small World Networks
- Phase 7.x.3: Nutrient Extraction

---

## 🎯 SUCCESS METRICS

### Technical

```yaml
Performance:
  local_latency: <100ms (p95)
  llm_latency: <1s (p95)
  cache_hit_rate: >70% (after 1 week)
  
Accuracy:
  sentiment_f1: >0.92
  topic_precision: >0.88
  user_satisfaction: >4.2/5
  
Cost:
  cost_per_chat: <$0.10
  cost_per_user_month: <$1.00
  vs_full_llm_savings: >95%
  
Privacy:
  pii_leakage: 0 incidents
  anonymization_audit: 100% pass
  user_consent: opt-in required
```

### Business

```yaml
Adoption:
  users_enable_llm: >30%
  users_upgrade_budget: >10%
  nps_score: >50
  
Economics:
  cac_payback: <6 months
  ltv_cac_ratio: >3:1
  gross_margin: >70%
```

---

## 🚀 NEXT STEPS

1. ✅ **Review & Approval** (this doc)
2. 📝 **Create Architecture Doc** (`13_hybrid-intelligence-engine.md`)
3. 🏗️ **Create Component Doc** (`09_relationship-psychology-analyzer.md`)
4. 💻 **Implement Phase 2** (HybridIntelligenceEngine)
5. 🧪 **Validate with Real Data** (Paula Roque chat)
6. 📊 **Measure & Iterate**

---

## 📝 CHANGELOG

```yaml
v1.0 (2025-11-29):
  - Initial vision document
  - 3-layer hybrid architecture
  - Privacy-first approach
  - Economic analysis
  - Anonymization strategy
  - Implementation roadmap
```

---

**Status:** 🟢 Ready for Implementation  
**Owner:** Eduardo + Bitácora Team  
**Next Review:** 2025-12-15
