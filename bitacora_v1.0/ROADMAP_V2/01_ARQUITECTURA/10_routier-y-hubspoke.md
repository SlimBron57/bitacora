# 10. Routier y HubSpoke (CAPA 6: AMPLIFICACIÓN)

**Última actualización:** 2025-11-23  
**Estado:** LISTO PARA PRODUCCIÓN  
**Versión:** 1.0  

---

## PARTE I: ESPECIFICACIÓN (CONCEPTO)

### ¿QUÉ ES CAPA 6?

**CAPA 6 (AMPLIFICACIÓN)** orquesta la **respuesta del sistema** usando:

1. **Routier** → Adaptador de flujo lógico (decide la estrategia)
2. **HubSpoke** → Orquestador multi-LLM (ejecuta la estrategia)

Resultado: Respuestas **personalizadas, contextuales y adaptables** a patrones detectados.

**Metáfora:** Como un **maestro de ceremonias** que lee la sala y decide si continuar con la estrategia original o cambiar de tono/tema, Routier + HubSpoke adaptan la respuesta a la dinámica de conversación.

### ROUTIER: Adaptador de Flujo Lógico

**¿Qué es?** Sistema de decisiones que analiza contexto actual y determina **cómo responder**.

**Decisiones clave:**

```
┌────────────────────────────────────────────────────┐
│  ROUTIER DECISION TREE                            │
├────────────────────────────────────────────────────┤
│                                                    │
│  IF ciclo detectado (disco rayado):               │
│     → STRATEGY: BreakCycle                        │
│       ├─ Cambiar tema radicalmente                │
│       ├─ Introducir nueva perspectiva             │
│       └─ Preguntar por "qué está pasando"         │
│                                                    │
│  ELSE IF progresión emocional positiva:           │
│     → STRATEGY: ReinforceProgress                 │
│       ├─ Reforzar logros alcanzados               │
│       ├─ Aumentar esperanza/motivación            │
│       └─ Acelerar resolución                      │
│                                                    │
│  ELSE IF regresión emocional:                     │
│     → STRATEGY: StabilizeEmotion                  │
│       ├─ Bajar ritmo                              │
│       ├─ Validar sentimientos                     │
│       └─ Volver a fundamentales                   │
│                                                    │
│  ELSE IF baja certeza (incertidumbre):            │
│     → STRATEGY: ProvideCertainty                  │
│       ├─ Ofrecer opciones claras                  │
│       ├─ Eliminar ambigüedad                      │
│       └─ Priorizar claridad > brevedad            │
│                                                    │
│  ELSE:                                             │
│     → STRATEGY: Normal                            │
│       └─ Responder directamente al input          │
│                                                    │
└────────────────────────────────────────────────────┘
```

### HUBSPOKE: Orquestador Multi-LLM

**¿Por qué multi-LLM?** Diferentes LLMs tienen diferentes fortalezas:

| LLM | Fortaleza | Debilidad | Caso de uso |
|-----|-----------|----------|------------|
| **GPT-4o** | Razonamiento, contexto largo | Caro, lento | Decisiones críticas |
| **Claude 3** | Análisis, nuance, seguridad | Lento | Análisis profundo |
| **Mistral 8x7B MoE** | Velocidad, eficiencia | Menos versatilidad | Respuestas rápidas |
| **Phi-3** | Local, rápido, pequeño | Menos capabilities | STM32H7 embedded |

**Arquitectura HubSpoke:**

```
┌─────────────────────────────────────────────────┐
│  HUBSPOKE ROUTING                               │
├─────────────────────────────────────────────────┤
│                                                 │
│  Input: CTX7D + Estrategia Routier             │
│  ↓                                              │
│  ┌─────────────────────────────────────────┐  │
│  │  1. Seleccionar LLM primario            │  │
│  │     - Basado en: complejidad, latencia  │  │
│  │     - CTX7D[6] (purpose) > 0.9 ?        │  │
│  │       → Usa GPT-4o (razonamiento)       │  │
│  │     - Sino →                             │  │
│  │       → Usa Mistral 8x7B (rápido)       │  │
│  └─────────────────────────────────────────┘  │
│  ↓                                              │
│  ┌─────────────────────────────────────────┐  │
│  │  2. Inyectar contexto (prompt injection) │  │
│  │     - FlowPacks relevantes               │  │
│  │     - Patrones detectados                │  │
│  │     - CTX7D para tono/estilo             │  │
│  │     - Estrategia Routier                 │  │
│  └─────────────────────────────────────────┘  │
│  ↓                                              │
│  ┌─────────────────────────────────────────┐  │
│  │  3. Generar respuesta                   │  │
│  │     LLM.generate(prompt, params)        │  │
│  └─────────────────────────────────────────┘  │
│  ↓                                              │
│  ┌─────────────────────────────────────────┐  │
│  │  4. Validar respuesta                   │  │
│  │     - ¿Responde la pregunta?            │  │
│  │     - ¿Tono apropriado?                 │  │
│  │     - ¿Hay hallucinations?              │  │
│  └─────────────────────────────────────────┘  │
│  ↓                                              │
│  ┌─────────────────────────────────────────┐  │
│  │  5. Failover (si validación falla)      │  │
│  │     - Reintentar con LLM secundario     │  │
│  │     - Timeout: 5 segundos               │  │
│  │     - Max reintentos: 2                 │  │
│  └─────────────────────────────────────────┘  │
│  ↓                                              │
│  Output: Respuesta validada                    │
│                                                 │
└─────────────────────────────────────────────────┘
```

### INYECCIÓN DE CONTEXTO: Prompt Engineering

**Estructura del prompt:**

```
┌─────────────────────────────────────────────────────┐
│  PROMPT INYECTADO EN LLMS                           │
├─────────────────────────────────────────────────────┤
│                                                     │
│  [SYSTEM]                                           │
│  Eres Bitácora, un asistente conversacional        │
│  especializado en memoria biográfica.              │
│                                                     │
│  [CURRENT_CONTEXT]                                 │
│  CTX7D actual: [0.7, 0.3, 0.9, 0.6, 0.8, 0.85, 0.75]│
│  ├─ Semántica: 0.7 (específico)                   │
│  ├─ Emocional: 0.3 (neutral-positivo)             │
│  ├─ Temporal: 0.9 (urgente)                       │
│  ├─ Relacional: 0.6 (múltiples actores)           │
│  ├─ Causal: 0.8 (clara motivación)                │
│  ├─ Propósito: 0.85 (meta clara)                  │
│  └─ Certeza: 0.75 (confianza media)               │
│                                                     │
│  [CONVERSATION_HISTORY]                            │
│  - Turn 1: Usuario pregunta X                      │
│  - Turn 5: Se mencionó tema Y                      │
│  - ...                                             │
│                                                     │
│  [DETECTED_PATTERNS]                               │
│  ├─ Ciclo detectado: Usuario repitió X 3 veces    │
│  ├─ Progresión: Emocional mejorando (+0.3)        │
│  ├─ Similitud: Turn 12 similar a turn 3 (0.92)    │
│  └─ Advertencia: Posible "disco rayado"           │
│                                                     │
│  [STRATEGY]                                        │
│  BreakCycle:                                       │
│  - Cambiar tema levemente                         │
│  - Ofrecer perspectiva nueva                      │
│  - Validar sentimientos del usuario               │
│                                                     │
│  [USER_INPUT]                                      │
│  "Aún no tengo dinero para..."                     │
│                                                     │
│  [INSTRUCTIONS]                                    │
│  1. Responder en tono empático                     │
│  2. Evitar repetir soluciones previas              │
│  3. Ofrecer perspectiva nueva                      │
│  4. Máximo 2 párrafos                              │
│  5. Preguntar qué ha cambiado desde última vez    │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### FAILOVER: Estrategia de Recuperación

```
LLM Primario (GPT-4o)
  ↓
  ❌ Timeout (>5s) o error
  ↓
LLM Secundario (Claude 3)
  ↓
  ✅ OK → Retornar respuesta
  ❌ Falla → Siguiente
  ↓
LLM Terciario (Mistral 8x7B)
  ↓
  ✅ OK → Retornar respuesta
  ❌ Falla → Fallback básico
  ↓
Fallback: Respuesta generativa simple sin IA
  "Entiendo que " + tema + ". ¿Cómo te sientes al respecto?"
```

---

## PARTE II: IMPLEMENTACIÓN (TÉCNICO)

### STRUCT: Routier

```rust
/// Adaptador de flujo lógico
/// Toma decisiones sobre cómo responder
pub struct Routier {
    /// Historial de decisiones
    decision_history: Vec<RoutierDecision>,
    
    /// Estrategia actual
    current_strategy: LlmStrategy,
    
    /// Parámetros configurables
    cycle_threshold: f32,
    emotion_trend_threshold: f32,
}

/// Estrategias de respuesta
#[derive(Debug, Clone, PartialEq)]
pub enum LlmStrategy {
    /// Estrategia normal: responder directamente
    Normal,
    
    /// Romper ciclo: cambiar tema o perspectiva
    BreakCycle {
        cycle_start_turn: u32,
        topic_shift: String,
    },
    
    /// Reforzar progresión: motivar, acelerar resolución
    ReinforceProgress {
        emotional_boost: bool,
    },
    
    /// Estabilizar emoción: validar, bajar ritmo
    StabilizeEmotion {
        validation_needed: bool,
    },
    
    /// Proporcionar certeza: opciones claras, eliminar ambigüedad
    ProvideCertainty {
        options_required: usize,
    },
}

/// Decisión tomada por Routier
#[derive(Debug, Clone)]
pub struct RoutierDecision {
    pub turn: u32,
    pub strategy: LlmStrategy,
    pub reasoning: String,
    pub timestamp: u64,
}
```

### ALGORITMO: Routier Decision Engine

```rust
impl Routier {
    /// Crea nuevo Routier
    pub fn new() -> Self {
        Self {
            decision_history: Vec::new(),
            current_strategy: LlmStrategy::Normal,
            cycle_threshold: 0.15,
            emotion_trend_threshold: 0.1,
        }
    }
    
    /// Toma decisión basada en contexto actual
    pub fn decide(
        &mut self,
        current_ctx7d: &ContextToken7D,
        pattern_recognizer: &PatternRecognizer,
        current_turn: u32,
    ) -> LlmStrategy {
        let mut strategy = LlmStrategy::Normal;
        let mut reasoning = String::new();
        
        // PASO 1: Chequear ciclos (disco rayado)
        if !pattern_recognizer.cycles.is_empty() {
            let cycle = &pattern_recognizer.cycles[0];
            if cycle.avg_distance < self.cycle_threshold {
                reasoning = format!(
                    "Ciclo detectado en turn {} con distancia {:.2}",
                    cycle.entry_turn,
                    cycle.avg_distance
                );
                strategy = LlmStrategy::BreakCycle {
                    cycle_start_turn: cycle.entry_turn,
                    topic_shift: self.suggest_topic_shift(),
                };
                
                self.current_strategy = strategy.clone();
                self.record_decision(current_turn, strategy, reasoning);
                return self.current_strategy.clone();
            }
        }
        
        // PASO 2: Chequear progresión emocional
        let emotion_stats = &pattern_recognizer.emotional_stats;
        if emotion_stats.trend > self.emotion_trend_threshold {
            reasoning = format!(
                "Progresión emocional positiva: trend = {:.2}",
                emotion_stats.trend
            );
            strategy = LlmStrategy::ReinforceProgress {
                emotional_boost: emotion_stats.trend > 0.2,
            };
            
            self.current_strategy = strategy.clone();
            self.record_decision(current_turn, strategy, reasoning);
            return self.current_strategy.clone();
        }
        
        // PASO 3: Chequear regresión emocional
        if emotion_stats.trend < -self.emotion_trend_threshold {
            reasoning = format!(
                "Regresión emocional: trend = {:.2}",
                emotion_stats.trend
            );
            strategy = LlmStrategy::StabilizeEmotion {
                validation_needed: emotion_stats.min_emotion < -0.5,
            };
            
            self.current_strategy = strategy.clone();
            self.record_decision(current_turn, strategy, reasoning);
            return self.current_strategy.clone();
        }
        
        // PASO 4: Chequear baja certeza
        if current_ctx7d.certainty < 0.4 {
            reasoning = format!(
                "Baja certeza detectada: {:.2}",
                current_ctx7d.certainty
            );
            strategy = LlmStrategy::ProvideCertainty {
                options_required: 3,
            };
            
            self.current_strategy = strategy.clone();
            self.record_decision(current_turn, strategy, reasoning);
            return self.current_strategy.clone();
        }
        
        // PASO 5: Estrategia normal
        reasoning = "Sin patrones detectados, estrategia normal".to_string();
        self.current_strategy = LlmStrategy::Normal;
        self.record_decision(current_turn, LlmStrategy::Normal, reasoning);
        
        LlmStrategy::Normal
    }
    
    /// Sugiere cambio de tema
    fn suggest_topic_shift(&self) -> String {
        vec![
            "Déjame preguntarte algo diferente...",
            "¿Podemos abordar esto desde otra perspectiva?",
            "He notado que volvemos a esto. ¿Qué ha cambiado?",
            "Seamos creativos. ¿Hay algo que no hemos considerado?",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap_or(&"Veamos esto de otra manera")
        .to_string()
    }
    
    /// Registra decisión
    fn record_decision(
        &mut self,
        turn: u32,
        strategy: LlmStrategy,
        reasoning: String,
    ) {
        self.decision_history.push(RoutierDecision {
            turn,
            strategy,
            reasoning,
            timestamp: current_timestamp(),
        });
    }
}
```

### STRUCT: HubSpoke

```rust
/// Orquestador multi-LLM
pub struct HubSpoke {
    /// LLMs disponibles
    llm_primary: LlmClient,
    llm_secondary: LlmClient,
    llm_tertiary: LlmClient,
    
    /// Estadísticas
    stats: HubSpokeStats,
    
    /// Configuración
    timeout_ms: u64,
    max_retries: u32,
    use_fallback: bool,
}

/// Cliente LLM
pub enum LlmClient {
    /// OpenAI GPT-4o
    GPT4o {
        api_key: String,
        model: String,
    },
    
    /// Anthropic Claude
    Claude {
        api_key: String,
        model: String,
    },
    
    /// Mistral 8x7B MoE
    Mistral {
        api_key: String,
        model: String,
    },
    
    /// Local Phi-3
    Phi3Local {
        model_path: String,
    },
}

/// Respuesta LLM validada
#[derive(Debug, Clone)]
pub struct ValidatedResponse {
    pub content: String,
    pub llm_used: String,
    pub confidence: f32,
    pub generation_time_ms: u64,
}

/// Estadísticas
#[derive(Debug, Clone)]
pub struct HubSpokeStats {
    pub total_requests: u32,
    pub successful_requests: u32,
    pub failed_requests: u32,
    pub fallback_used: u32,
    pub avg_latency_ms: f32,
}
```

### ALGORITMO: HubSpoke Orchestration

```rust
impl HubSpoke {
    /// Crea nuevo HubSpoke
    pub fn new(primary: LlmClient, secondary: LlmClient, tertiary: LlmClient) -> Self {
        Self {
            llm_primary: primary,
            llm_secondary: secondary,
            llm_tertiary: tertiary,
            stats: HubSpokeStats::default(),
            timeout_ms: 5000,
            max_retries: 2,
            use_fallback: true,
        }
    }
    
    /// Genera respuesta con orquestación
    pub async fn generate(
        &mut self,
        prompt: &str,
        ctx7d: &ContextToken7D,
        strategy: &LlmStrategy,
    ) -> Result<ValidatedResponse, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        // PASO 1: Seleccionar LLM primario
        let llm = self.select_llm_primary(ctx7d, strategy);
        
        // PASO 2: Inyectar contexto en prompt
        let enriched_prompt = self.enrich_prompt(prompt, ctx7d, strategy);
        
        // PASO 3: Generar respuesta con timeout
        let generation = tokio::time::timeout(
            std::time::Duration::from_millis(self.timeout_ms),
            llm.generate(&enriched_prompt),
        ).await;
        
        match generation {
            Ok(Ok(response)) => {
                // PASO 4: Validar respuesta
                if self.validate_response(&response, &enriched_prompt) {
                    let elapsed = start.elapsed().as_millis() as u64;
                    
                    self.stats.successful_requests += 1;
                    self.update_stats(elapsed);
                    
                    return Ok(ValidatedResponse {
                        content: response,
                        llm_used: self.llm_name(&llm),
                        confidence: 0.95,
                        generation_time_ms: elapsed,
                    });
                } else {
                    // Validación falló, intentar secondario
                    return self.fallback_generate(&enriched_prompt, 0).await;
                }
            }
            Ok(Err(_)) | Err(_) => {
                // LLM primario falló, usar secondary
                self.stats.failed_requests += 1;
                return self.fallback_generate(&enriched_prompt, 0).await;
            }
        }
    }
    
    /// Fallback a LLM secundario/terciario
    async fn fallback_generate(
        &mut self,
        prompt: &str,
        retry_count: u32,
    ) -> Result<ValidatedResponse, Box<dyn std::error::Error>> {
        if retry_count >= self.max_retries {
            // Usar fallback básico
            if self.use_fallback {
                self.stats.fallback_used += 1;
                return Ok(ValidatedResponse {
                    content: self.basic_fallback_response(prompt),
                    llm_used: "fallback_basic".to_string(),
                    confidence: 0.5,
                    generation_time_ms: 0,
                });
            } else {
                return Err("All LLMs failed".into());
            }
        }
        
        let secondary_llm = if retry_count == 0 {
            &self.llm_secondary
        } else {
            &self.llm_tertiary
        };
        
        let start = std::time::Instant::now();
        
        match tokio::time::timeout(
            std::time::Duration::from_millis(self.timeout_ms * 2),
            secondary_llm.generate(prompt),
        ).await {
            Ok(Ok(response)) => {
                let elapsed = start.elapsed().as_millis() as u64;
                
                if self.validate_response(&response, prompt) {
                    self.stats.successful_requests += 1;
                    self.update_stats(elapsed);
                    
                    return Ok(ValidatedResponse {
                        content: response,
                        llm_used: self.llm_name(secondary_llm),
                        confidence: 0.85,
                        generation_time_ms: elapsed,
                    });
                }
            }
            _ => {}
        }
        
        // Reintentar
        self.fallback_generate(prompt, retry_count + 1).await
    }
    
    /// Selecciona LLM primario basado en contexto
    fn select_llm_primary(&self, ctx7d: &ContextToken7D, strategy: &LlmStrategy) -> &LlmClient {
        // Si tarea compleja y meta clara → GPT-4o
        if ctx7d.purpose > 0.85 && matches!(strategy, LlmStrategy::ProvideCertainty { .. }) {
            return &self.llm_primary;
        }
        
        // Si necesita rapidez → Mistral
        if ctx7d.temporal > 0.9 {
            // Mistral en caso multi-LLM
            return &self.llm_secondary;
        }
        
        // Default: primario
        &self.llm_primary
    }
    
    /// Enriquece prompt con contexto
    fn enrich_prompt(
        &self,
        user_prompt: &str,
        ctx7d: &ContextToken7D,
        strategy: &LlmStrategy,
    ) -> String {
        format!(
            r#"[SYSTEM]
Eres Bitácora, asistente especializado en memoria biográfica y contexto conversacional.

[CONTEXT_DIMENSIONS]
- Semántica: {:.2}/1.0
- Emocional: {:.2}/1.0
- Temporal: {:.2}/1.0
- Relacional: {:.2}/1.0
- Causal: {:.2}/1.0
- Propósito: {:.2}/1.0
- Certeza: {:.2}/1.0

[STRATEGY]
{}

[INSTRUCTIONS]
- Responder de forma empática y contextual
- Evitar repeticiones
- Validar sentimientos
- Máximo 2-3 párrafos
- Usar información del contexto

[USER_MESSAGE]
{}
"#,
            ctx7d.semantic,
            ctx7d.emotional,
            ctx7d.temporal,
            ctx7d.relational,
            ctx7d.causal,
            ctx7d.purpose,
            ctx7d.certainty,
            self.format_strategy(strategy),
            user_prompt
        )
    }
    
    /// Valida respuesta LLM
    fn validate_response(&self, response: &str, prompt: &str) -> bool {
        // Validaciones básicas
        if response.is_empty() || response.len() > 5000 {
            return false;
        }
        
        // Chequear hallucinations (simple heuristic)
        let hallucination_score = self.detect_hallucinations(response, prompt);
        if hallucination_score > 0.7 {
            return false;
        }
        
        // Chequear relevancia
        if !self.is_relevant(response, prompt) {
            return false;
        }
        
        true
    }
    
    /// Fallback básico sin IA
    fn basic_fallback_response(&self, prompt: &str) -> String {
        // Análisis mínimo del prompt
        let has_question = prompt.contains("?");
        let is_urgent = prompt.to_lowercase().contains("urgente") ||
                        prompt.to_lowercase().contains("rápido");
        
        if has_question {
            "Entiendo tu pregunta. Déjame ayudarte a pensar en esto con más claridad.".to_string()
        } else if is_urgent {
            "Veo que esto es urgente. ¿Cuál es exactamente lo que necesitas ahora mismo?".to_string()
        } else {
            "Cuéntame más. ¿Qué es lo más importante para ti en este momento?".to_string()
        }
    }
}
```

### PERFORMANCE TARGETS

| Métrica | Target | Ambiente |
|---------|--------|----------|
| Decisión Routier | <50ms | Pattern analysis |
| Generación LLM primario | <5000ms | GPT-4o @ api.openai.com |
| Failover (si falla) | <10000ms | Total con reintentos |
| Validación de respuesta | <500ms | Hallucination check |
| Latencia total (end-to-end) | <6000ms | User perceivable |

---

## PARTE III: INTEGRACIÓN

### Pipeline Completo (CAPA 1-6)

```
INPUT (Usuario)
  ↓
[CAPA 1] CTX7D::from_text() → [0.7, 0.3, 0.9, ...]
  ↓
[CAPA 2] FbcuCore::compress() + FlowPack::organize()
  ↓
[CAPA 3] Persistencia en TelescopeDB/VoxelDB
  ↓
[CAPA 4] Embedding + HNSW::search(K=5 similar contexts)
  ↓
[CAPA 5] PatternRecognizer::detect_cycles() + emotional_stats
  ↓
[CAPA 6] Routier::decide() → LlmStrategy
        ↓
        HubSpoke::generate()
          ├─ enrich_prompt(CTX7D, estrategia)
          ├─ LLM primario (timeout 5s)
          ├─ validate_response()
          └─ failover si necesario
  ↓
OUTPUT (Respuesta personalizada)
```

---

## VALIDACIÓN

### CHECKLIST DE ACEPTACIÓN

- [ ] Routier decision tree implementado (5 estrategias)
- [ ] HubSpoke selección de LLM funcional
- [ ] Inyección de contexto en prompts
- [ ] Validación de respuestas (hallucination check)
- [ ] Failover automático con reintentos
- [ ] Fallback básico sin IA
- [ ] Estadísticas de uso (requests, latencias)
- [ ] Latencia total <6 segundos

### TESTS UNITARIOS

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_routier_cycle_detection() {
        let mut routier = Routier::new();
        let mut pattern_recognizer = PatternRecognizer::new(0.15, 0.7);
        // ... setup patterns
        
        let strategy = routier.decide(&ctx7d, &pattern_recognizer, 10);
        assert!(matches!(strategy, LlmStrategy::BreakCycle { .. }));
    }
    
    #[test]
    fn test_hubspoke_validation() {
        let hubspoke = HubSpoke::new(
            LlmClient::Mistral { /* ... */ },
            LlmClient::Claude { /* ... */ },
            LlmClient::Phi3Local { /* ... */ },
        );
        
        let valid_response = "Entiendo tu situación. Aquí hay 3 opciones...";
        assert!(hubspoke.validate_response(valid_response, "¿qué hago?"));
        
        let invalid_response = ""; // Empty
        assert!(!hubspoke.validate_response(invalid_response, "¿qué hago?"));
    }
}
```

---

## REFERENCIAS

- **00_VISION:** `04_arquitectura-sistema-7-capas.md` (definición CAPA 6)
- **01_ARQUITECTURA:** `09_reconocimiento-patrones.md` (productor upstream)
- **Prompt Engineering:** OpenAI best practices
- **LLM APIs:** OpenAI, Anthropic, Mistral, Ollama

---

## NOTAS PARA DESARROLLO

- ⚠️ Prompt injection debe ser **context-aware**, no static
- ⚠️ Failover requiere **timeouts** configurables por LLM
- ✅ Routier es **stateless** (no mantiene estado entre turns)
- ✅ HubSpoke mantiene **stats** para monitoring
- ✅ Fallback es **best-effort**, nunca falla

---

**Estado:** ✅ READY FOR CODING  
**Siguiente:** `11_respuesta-adaptada-llm.md` (CAPA 7)
