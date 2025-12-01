```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/09_hubspoke-navigator.md
Versión: 1.0.0
Fecha Creación: 2025-10-26
Autor: Sistema Bitácora - Documentación MTT-DSL
Propósito: Especificación componente HubSpoke Navigator (Routing Multi-LLM inteligente)
Estado: 📋 ESPECIFICACIÓN
Relacionado Con: 
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md
  - FUSION_BAYESIANA/08_DIAGRAMA_SISTEMA.md
Implementa:
  - DA-010: Multi-LLM Orchestration (HubSpoke pattern)
  - DA-002: Context Token 7D Integration
  - BITA-2: ACA-7D Contextual Routing
# === FIN DATOS DE AUDITORÍA ===
```

# 🎯 HUBSPOKE NAVIGATOR - Routing Multi-LLM Inteligente

---

## 🎯 PROPÓSITO

El **HubSpoke Navigator** es el componente central de orquestación multi-LLM de Bitácora. Su responsabilidad es **seleccionar el LLM óptimo** para cada query basándose en análisis contextual 7D, histórico del usuario, y características específicas de cada provider.

### El Problema que Resuelve

**Escenario actual (sin HubSpoke):**
```
Usuario: "Explicame async Rust con ejemplos complejos"

Opciones disponibles:
- GPT-4: Excelente para código, $$$$ (caro)
- Claude 3.5 Sonnet: Mejor razonamiento, $$$$ (muy caro)
- Perplexity Sonar: Bueno para investigación, $ (barato)

¿Cuál elegir? ❌ Usuario decide manualmente
→ Riesgo: Elegir mal = $$$$ gastados + respuesta subóptima
```

**Con HubSpoke Navigator:**
```
Usuario: "Explicame async Rust con ejemplos complejos"
    ↓
HubSpoke analiza:
  ├─ CTX7D: semantic=0.85 (alta complejidad) → prefiere Claude
  ├─ CTX7D: intentional=0.90 (claro: quiere aprender) → OK cualquiera
  ├─ CTX7D: temporal=0.30 (NO urgente) → puede usar modelo lento pero bueno
  ├─ Historia TelescopeDB: Usuario es dev Rust senior → necesita profundidad
  ├─ Costos históricos: Usuario ya gastó $15 hoy → ⚠️ optimizar
  └─ Provider capabilities:
      • GPT-4: code=9/10, reasoning=7/10, cost=$$$$
      • Claude 3.5: code=8/10, reasoning=10/10, cost=$$$$
      • Perplexity: code=6/10, reasoning=6/10, cost=$
    ↓
Decisión: Claude 3.5 Sonnet ✅
Razón: Complejidad alta + NO urgente + necesita profundidad
Estrategia: ContextualBestFit (basada en 7D)
    ↓
Resultado: Respuesta excepcional de 2,500 tokens
Costo: $0.045 (justificado por calidad)
Usuario: "¡Perfecto! Justo lo que necesitaba" 🎯
```

### Por Qué es Crítico

1. **Optimización de Costos:** Evitar usar Claude ($$$) para queries simples que Perplexity ($) puede manejar
2. **Calidad de Respuestas:** Matching perfecto entre características del query y fortalezas del LLM
3. **Experiencia de Usuario:** Usuario NO piensa en "¿qué LLM usar?", solo pregunta
4. **Escalabilidad:** Fácil añadir nuevos providers sin cambiar lógica del sistema
5. **Resiliencia:** Failover automático si un provider está caído o lento

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
FLUJO DE DATOS END-TO-END:

Usuario: "Query en lenguaje natural"
    ↓
┌─────────────────────────────────────────────────────┐
│ SENSORY ENGINE (Procesamiento Multimodal)          │
│ └─> NormalizedInput                                │
└─────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────┐
│ CONTEXT TOKEN 7D (Análisis Dimensional)            │
│ └─> ContextTensor7D {                              │
│       semantic: 0.85,      // Complejidad           │
│       intentional: 0.90,   // Claridad              │
│       temporal: 0.30,      // Urgencia              │
│       emotional: 0.60,     // Tono emocional        │
│       associative: 0.75,   // Relaciones            │
│       evaluative: 0.80,    // Juicio crítico        │
│       metalinguistic: 0.70 // Auto-referencia       │
│     }                                               │
└─────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────┐
│ ★★★ HUBSPOKE NAVIGATOR (TÚ ESTÁS AQUÍ) ★★★         │
│                                                     │
│ Componentes:                                        │
│  ├─ Hub (Central Intelligence)                     │
│  │   ├─ Analiza CTX7D                              │
│  │   ├─ Consulta TelescopeDB (historial)          │
│  │   ├─ Consulta VoxelDB (templates similares)    │
│  │   └─ Aplica estrategia de routing              │
│  │                                                  │
│  └─ Spokes (Specialized Routers)                   │
│      ├─ Spoke 1: OpenAI (GPT-4, GPT-3.5)          │
│      ├─ Spoke 2: Anthropic (Claude 3.5)           │
│      └─ Spoke 3: Perplexity (Sonar, Llama)        │
│                                                     │
│ Estrategias:                                        │
│  ├─ RoundRobin (balanced load)                     │
│  ├─ LeastLoaded (performance)                      │
│  ├─ ContextualBestFit ⭐ (7D-based, RECOMENDADA)   │
│  └─ CostOptimized (budget constraints)             │
│                                                     │
│ Decisión: SelectedProvider + EnhancedPrompt        │
└─────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────┐
│ WIZARD-MULTI-LLM (LLM Execution Layer)             │
│ └─> Ejecuta en provider seleccionado               │
└─────────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────────┐
│ TELESCOPEDB (Almacenar respuesta + metadata)       │
│ - Response text                                     │
│ - Provider usado                                    │
│ - Costo incurrido                                   │
│ - Latencia                                          │
│ - Tokens consumidos                                 │
└─────────────────────────────────────────────────────┘
    ↓
Usuario: Recibe respuesta optimizada ✅
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito |
|------------|-----------|-----------|
| **Context Token 7D** | Entrada ← | Recibe análisis dimensional del query |
| **TelescopeDB** | Consulta ↔ | Lee historial de usuario (costos, preferencias, expertise) |
| **VoxelDB** | Consulta ↔ | Busca templates similares para enriquecer contexto |
| **Wizard-Multi-LLM** | Salida → | Envía provider seleccionado + prompt mejorado |
| **SENSORY ENGINE** | Entrada ← | Recibe input normalizado multimodal |

---

## 📋 RESPONSABILIDADES CORE

El HubSpoke Navigator **DEBE**:

1. **Seleccionar Provider Óptimo** basándose en:
   - Análisis 7D del query (complejidad, urgencia, tono)
   - Fortalezas específicas de cada LLM (código, razonamiento, investigación)
   - Historial del usuario (expertise, presupuesto, preferencias)
   - Restricciones de costo y latencia

2. **Aplicar Estrategias de Routing:**
   - `RoundRobin`: Distribuir carga equitativamente entre providers
   - `LeastLoaded`: Elegir provider con menor latencia actual
   - `ContextualBestFit`: Matching semántico basado en CTX7D ⭐
   - `CostOptimized`: Minimizar costo sin sacrificar calidad crítica

3. **Enriquecer Prompts** antes de enviar al LLM:
   - Inyectar contexto histórico relevante de TelescopeDB
   - Añadir templates MTT-DSL de VoxelDB si aplica
   - Incluir expertise markers del usuario
   - Aplicar dimensiones 7D como hints al LLM

4. **Medir y Reportar** decisiones de routing:
   - Registrar provider seleccionado + razón
   - Tracking de costos por provider
   - Métricas de latencia y calidad
   - Detección de breakthroughs (cuando respuesta supera expectativas)

5. **Failover Automático:**
   - Si provider primario falla → intentar con backup
   - Si provider está saturado (429 rate limit) → reroute
   - Transparente para el usuario

6. **Balanceo de Carga Inteligente:**
   - Evitar saturar un solo provider
   - Respetar rate limits de APIs
   - Distribuir queries pesados vs livianos

7. **Versionado de Modelos:**
   - Mapear `gpt-4` → `gpt-4-0613` (versión específica)
   - Deprecación automática de modelos viejos
   - A/B testing de nuevos modelos

---

## 🗂️ ESTRUCTURAS DE DATOS

```rust
// src/multi_agent/hubspoke.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::core::ctx7d::ContextTensor7D;
use crate::cells::telescopedb::UserHistory;

/// Configuración del HubSpoke Navigator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpokeConfig {
    /// Estrategia de routing por defecto
    pub default_strategy: RoutingStrategy,
    
    /// Providers habilitados (OpenAI, Anthropic, Perplexity)
    pub enabled_providers: Vec<LLMProvider>,
    
    /// Presupuesto máximo diario en USD
    pub daily_budget_usd: f64,
    
    /// Latencia máxima aceptable (ms)
    pub max_latency_ms: u64,
    
    /// Failover automático habilitado
    pub enable_failover: bool,
    
    /// Mapeo de modelos (nombre → versión específica)
    pub model_versions: HashMap<String, String>,
}

/// Hub Central - Cerebro del Navigator
pub struct Hub {
    /// Configuración
    config: HubSpokeConfig,
    
    /// Registro de decisiones de routing
    routing_history: Vec<RoutingDecision>,
    
    /// Métricas por provider
    provider_metrics: HashMap<LLMProvider, ProviderMetrics>,
    
    /// Conexión a TelescopeDB
    telescope: Arc<TelescopeDB>,
    
    /// Conexión a VoxelDB
    voxel: Arc<VoxelDB>,
}

/// Spoke - Especialista en un provider específico
pub struct Spoke {
    /// Proveedor al que representa
    provider: LLMProvider,
    
    /// Configuración específica del provider
    config: ProviderConfig,
    
    /// Capacidades del provider
    capabilities: ProviderCapabilities,
    
    /// Estado actual (healthy, degraded, down)
    health_status: HealthStatus,
    
    /// Cola de requests pendientes
    pending_requests: VecDeque<Request>,
}

/// Estrategias de routing disponibles
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RoutingStrategy {
    /// Distribuir equitativamente entre providers
    RoundRobin,
    
    /// Elegir provider con menor carga actual
    LeastLoaded,
    
    /// Matching semántico basado en CTX7D (RECOMENDADO)
    ContextualBestFit,
    
    /// Minimizar costo respetando calidad
    CostOptimized,
}

/// Providers soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Perplexity,
}

/// Capacidades específicas de un provider
#[derive(Debug, Clone)]
pub struct ProviderCapabilities {
    /// Qué tan bueno es para código (0.0-1.0)
    pub code_quality: f64,
    
    /// Qué tan bueno es para razonamiento complejo (0.0-1.0)
    pub reasoning_quality: f64,
    
    /// Qué tan bueno es para investigación/búsqueda (0.0-1.0)
    pub research_quality: f64,
    
    /// Qué tan bueno es para creatividad/escritura (0.0-1.0)
    pub creative_quality: f64,
    
    /// Contexto máximo en tokens
    pub max_context_tokens: usize,
    
    /// Costo por 1K input tokens (USD)
    pub cost_per_1k_input: f64,
    
    /// Costo por 1K output tokens (USD)
    pub cost_per_1k_output: f64,
    
    /// Latencia promedio (ms)
    pub avg_latency_ms: u64,
}

/// Decisión de routing registrada
#[derive(Debug, Clone, Serialize)]
pub struct RoutingDecision {
    /// Timestamp de la decisión
    pub timestamp: i64,
    
    /// Query original del usuario
    pub query: String,
    
    /// Análisis 7D del query
    pub ctx7d: ContextTensor7D,
    
    /// Provider seleccionado
    pub selected_provider: LLMProvider,
    
    /// Estrategia aplicada
    pub strategy: RoutingStrategy,
    
    /// Razón de la selección (humano-legible)
    pub reasoning: String,
    
    /// Costo estimado (USD)
    pub estimated_cost: f64,
    
    /// ¿Fue un failover?
    pub was_failover: bool,
}

/// Métricas por provider
#[derive(Debug, Clone, Default)]
pub struct ProviderMetrics {
    /// Total de requests enviados
    pub total_requests: u64,
    
    /// Requests exitosos
    pub successful_requests: u64,
    
    /// Requests fallidos
    pub failed_requests: u64,
    
    /// Costo total acumulado (USD)
    pub total_cost_usd: f64,
    
    /// Latencia promedio (ms)
    pub avg_latency_ms: f64,
    
    /// Tokens promedio consumidos
    pub avg_tokens: f64,
    
    /// Breakthroughs detectados
    pub breakthroughs: u64,
}

/// Estado de salud del provider
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}
```

---

## 🔌 API PÚBLICA

```rust
// src/multi_agent/hubspoke.rs

impl Hub {
    /// Crear nuevo Hub con configuración
    pub fn new(config: HubSpokeConfig) -> Result<Self> {
        Ok(Self {
            config,
            routing_history: Vec::new(),
            provider_metrics: HashMap::new(),
            telescope: Arc::new(TelescopeDB::new()?),
            voxel: Arc::new(VoxelDB::new()?),
        })
    }
    
    /// Seleccionar provider óptimo para un query
    /// 
    /// # Argumentos
    /// - `query`: Query original del usuario
    /// - `ctx7d`: Análisis 7D del query
    /// - `user_id`: ID del usuario (para consultar historial)
    /// - `strategy`: Estrategia de routing (opcional, usa default si None)
    /// 
    /// # Retorna
    /// `RoutingDecision` con provider seleccionado y razonamiento
    pub async fn select_provider(
        &mut self,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
        strategy: Option<RoutingStrategy>,
    ) -> Result<RoutingDecision> {
        let strategy = strategy.unwrap_or(self.config.default_strategy);
        
        // Consultar historial del usuario
        let user_history = self.telescope.get_user_history(user_id).await?;
        
        // Aplicar estrategia de routing
        let decision = match strategy {
            RoutingStrategy::RoundRobin => self.route_round_robin(),
            RoutingStrategy::LeastLoaded => self.route_least_loaded().await,
            RoutingStrategy::ContextualBestFit => {
                self.route_contextual_best_fit(query, ctx7d, &user_history).await
            }
            RoutingStrategy::CostOptimized => {
                self.route_cost_optimized(ctx7d, &user_history).await
            }
        }?;
        
        // Registrar decisión
        self.routing_history.push(decision.clone());
        
        Ok(decision)
    }
    
    /// Enriquecer prompt con contexto adicional
    /// 
    /// Inyecta:
    /// - Historial relevante de TelescopeDB
    /// - Templates similares de VoxelDB
    /// - Expertise markers del usuario
    /// - Dimensiones 7D como hints
    pub async fn enrich_prompt(
        &self,
        original_prompt: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
    ) -> Result<String> {
        let mut enriched = original_prompt.to_string();
        
        // Buscar templates similares
        let templates = self.voxel
            .find_templates_spatial(ctx7d, 0.15)
            .await?;
        
        if !templates.is_empty() {
            enriched.push_str("\n\n## Contexto de templates relevantes:\n");
            for tmpl in templates.iter().take(3) {
                enriched.push_str(&format!("- {}\n", tmpl.name));
            }
        }
        
        // Añadir hints dimensionales
        enriched.push_str(&format!(
            "\n\n## Análisis dimensional:\n\
             - Complejidad semántica: {:.2}\n\
             - Claridad intencional: {:.2}\n\
             - Urgencia temporal: {:.2}\n",
            ctx7d.semantic,
            ctx7d.intentional,
            ctx7d.temporal
        ));
        
        Ok(enriched)
    }
    
    /// Obtener métricas actuales de todos los providers
    pub fn get_provider_metrics(&self) -> &HashMap<LLMProvider, ProviderMetrics> {
        &self.provider_metrics
    }
    
    /// Actualizar métricas después de ejecutar un request
    pub fn update_metrics(
        &mut self,
        provider: LLMProvider,
        latency_ms: u64,
        tokens_used: usize,
        cost_usd: f64,
        success: bool,
    ) {
        let metrics = self.provider_metrics
            .entry(provider)
            .or_insert_with(ProviderMetrics::default);
        
        metrics.total_requests += 1;
        
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }
        
        metrics.total_cost_usd += cost_usd;
        
        // Actualizar promedios con media móvil
        let alpha = 0.2; // Factor de suavizado
        metrics.avg_latency_ms = alpha * latency_ms as f64 
            + (1.0 - alpha) * metrics.avg_latency_ms;
        metrics.avg_tokens = alpha * tokens_used as f64 
            + (1.0 - alpha) * metrics.avg_tokens;
    }
}

impl Spoke {
    /// Crear nuevo Spoke para un provider
    pub fn new(provider: LLMProvider, config: ProviderConfig) -> Self {
        let capabilities = Self::default_capabilities(provider);
        
        Self {
            provider,
            config,
            capabilities,
            health_status: HealthStatus::Healthy,
            pending_requests: VecDeque::new(),
        }
    }
    
    /// Verificar salud del provider
    pub async fn health_check(&mut self) -> Result<HealthStatus> {
        // Intentar ping simple al provider
        match self.ping().await {
            Ok(_) => {
                self.health_status = HealthStatus::Healthy;
                Ok(HealthStatus::Healthy)
            }
            Err(e) if e.is_rate_limit() => {
                self.health_status = HealthStatus::Degraded;
                Ok(HealthStatus::Degraded)
            }
            Err(_) => {
                self.health_status = HealthStatus::Down;
                Ok(HealthStatus::Down)
            }
        }
    }
    
    /// Obtener capacidades por defecto de un provider
    fn default_capabilities(provider: LLMProvider) -> ProviderCapabilities {
        match provider {
            LLMProvider::OpenAI => ProviderCapabilities {
                code_quality: 0.95,
                reasoning_quality: 0.80,
                research_quality: 0.70,
                creative_quality: 0.85,
                max_context_tokens: 128_000,
                cost_per_1k_input: 0.01,
                cost_per_1k_output: 0.03,
                avg_latency_ms: 1200,
            },
            LLMProvider::Anthropic => ProviderCapabilities {
                code_quality: 0.90,
                reasoning_quality: 0.95,
                research_quality: 0.75,
                creative_quality: 0.90,
                max_context_tokens: 200_000,
                cost_per_1k_input: 0.015,
                cost_per_1k_output: 0.075,
                avg_latency_ms: 1500,
            },
            LLMProvider::Perplexity => ProviderCapabilities {
                code_quality: 0.70,
                reasoning_quality: 0.75,
                research_quality: 0.95,
                research_quality: 0.70,
                max_context_tokens: 127_000,
                cost_per_1k_input: 0.001,
                cost_per_1k_output: 0.001,
                avg_latency_ms: 800,
            },
        }
    }
}
```

---

## ⚙️ IMPLEMENTACIÓN INTERNA

### Algoritmo: ContextualBestFit (Routing Basado en 7D)

Este es el algoritmo **ESTRELLA** del HubSpoke - matching semántico entre características del query y fortalezas del provider:

```rust
impl Hub {
    /// Routing basado en análisis contextual 7D
    async fn route_contextual_best_fit(
        &self,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_history: &UserHistory,
    ) -> Result<RoutingDecision> {
        // PASO 1: Calcular score de cada provider basado en dimensiones 7D
        let mut provider_scores: Vec<(LLMProvider, f64, String)> = Vec::new();
        
        for provider in &self.config.enabled_providers {
            let spoke = self.get_spoke(*provider)?;
            let caps = &spoke.capabilities;
            
            // Score inicial = 0
            let mut score = 0.0;
            let mut reasoning_parts = Vec::new();
            
            // DIMENSIÓN 1: Semantic (complejidad semántica)
            // Alta complejidad → preferir Anthropic (razonamiento)
            if ctx7d.semantic > 0.75 {
                score += caps.reasoning_quality * 2.0;
                reasoning_parts.push(format!(
                    "Alta complejidad ({:.2}) → +{:.2} ({:?} reasoning)",
                    ctx7d.semantic,
                    caps.reasoning_quality * 2.0,
                    provider
                ));
            }
            
            // DIMENSIÓN 2: Intentional (claridad intencional)
            // Si intención es código → preferir OpenAI
            if query.contains("código") || query.contains("code") {
                score += caps.code_quality * 1.5;
                reasoning_parts.push(format!(
                    "Query de código → +{:.2} ({:?} code)",
                    caps.code_quality * 1.5,
                    provider
                ));
            }
            
            // Si intención es investigación → preferir Perplexity
            if query.contains("investiga") || query.contains("busca") {
                score += caps.research_quality * 2.0;
                reasoning_parts.push(format!(
                    "Query de investigación → +{:.2} ({:?} research)",
                    caps.research_quality * 2.0,
                    provider
                ));
            }
            
            // DIMENSIÓN 3: Temporal (urgencia)
            // Alta urgencia → preferir provider más rápido
            if ctx7d.temporal > 0.75 {
                let speed_score = 1.0 - (caps.avg_latency_ms as f64 / 2000.0);
                score += speed_score * 1.5;
                reasoning_parts.push(format!(
                    "Alta urgencia ({:.2}) → +{:.2} (latencia {}ms)",
                    ctx7d.temporal,
                    speed_score * 1.5,
                    caps.avg_latency_ms
                ));
            }
            
            // DIMENSIÓN 4: Emotional (tono emocional)
            // Tono empático → preferir Claude (mejor en empatía)
            if ctx7d.emotional > 0.70 {
                let empathy_bonus = if *provider == LLMProvider::Anthropic {
                    1.0
                } else {
                    0.3
                };
                score += empathy_bonus;
                reasoning_parts.push(format!(
                    "Tono empático ({:.2}) → +{:.2}",
                    ctx7d.emotional,
                    empathy_bonus
                ));
            }
            
            // DIMENSIÓN 5: Evaluative (juicio crítico)
            // Alto juicio → preferir razonamiento complejo
            if ctx7d.evaluative > 0.75 {
                score += caps.reasoning_quality * 1.2;
                reasoning_parts.push(format!(
                    "Alto juicio crítico ({:.2}) → +{:.2}",
                    ctx7d.evaluative,
                    caps.reasoning_quality * 1.2
                ));
            }
            
            // FACTOR: Restricciones de presupuesto
            if user_history.budget_remaining_usd < 5.0 {
                // Penalizar providers caros si presupuesto bajo
                let cost_penalty = caps.cost_per_1k_output * 10.0;
                score -= cost_penalty;
                reasoning_parts.push(format!(
                    "Presupuesto bajo (${:.2}) → -{:.2}",
                    user_history.budget_remaining_usd,
                    cost_penalty
                ));
            }
            
            // FACTOR: Experiencia del usuario
            if user_history.expertise_level == ExpertiseLevel::Senior {
                // Usuarios senior prefieren profundidad (Anthropic, GPT-4)
                if *provider == LLMProvider::Anthropic 
                    || *provider == LLMProvider::OpenAI {
                    score += 0.5;
                    reasoning_parts.push("Usuario senior → +0.5".to_string());
                }
            }
            
            let reasoning = reasoning_parts.join("; ");
            provider_scores.push((*provider, score, reasoning));
        }
        
        // PASO 2: Ordenar providers por score descendente
        provider_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // PASO 3: Seleccionar top provider (con failover si no está healthy)
        for (provider, score, reasoning) in provider_scores {
            let spoke = self.get_spoke(provider)?;
            
            if spoke.health_status == HealthStatus::Healthy {
                return Ok(RoutingDecision {
                    timestamp: chrono::Utc::now().timestamp(),
                    query: query.to_string(),
                    ctx7d: ctx7d.clone(),
                    selected_provider: provider,
                    strategy: RoutingStrategy::ContextualBestFit,
                    reasoning: format!(
                        "Score: {:.2} | {}",
                        score,
                        reasoning
                    ),
                    estimated_cost: self.estimate_cost(provider, query)?,
                    was_failover: false,
                });
            }
        }
        
        Err(Error::AllProvidersDown)
    }
    
    /// Estimar costo de un query para un provider
    fn estimate_cost(&self, provider: LLMProvider, query: &str) -> Result<f64> {
        let spoke = self.get_spoke(provider)?;
        let caps = &spoke.capabilities;
        
        // Estimación simplificada: ~4 chars = 1 token
        let input_tokens = query.len() / 4;
        
        // Estimación output: 2x input (promedio)
        let output_tokens = input_tokens * 2;
        
        let cost = (input_tokens as f64 / 1000.0) * caps.cost_per_1k_input
                 + (output_tokens as f64 / 1000.0) * caps.cost_per_1k_output;
        
        Ok(cost)
    }
}
```

### Algoritmo: RoundRobin (Balanceo Simple)

```rust
impl Hub {
    fn route_round_robin(&mut self) -> Result<RoutingDecision> {
        // Contador interno (no mostrado arriba)
        let idx = self.round_robin_counter % self.config.enabled_providers.len();
        let provider = self.config.enabled_providers[idx];
        self.round_robin_counter += 1;
        
        Ok(RoutingDecision {
            timestamp: chrono::Utc::now().timestamp(),
            selected_provider: provider,
            strategy: RoutingStrategy::RoundRobin,
            reasoning: format!("Turno #{} → {:?}", idx + 1, provider),
            estimated_cost: 0.0, // Calcular después
            was_failover: false,
            // ... otros campos
        })
    }
}
```

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito |
|------------|---------|-----------|
| **Context Token 7D** | v1.0 | Análisis dimensional del query para routing inteligente |
| **TelescopeDB** | v1.0 | Consulta historial de usuario (presupuesto, expertise, preferencias) |
| **VoxelDB** | v1.0 | Búsqueda de templates similares para enriquecer prompts |
| **Wizard-Multi-LLM** | v1.0 | Ejecución del request en el provider seleccionado |

### Crates Externos

```toml
[dependencies]
# Core async runtime
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP client para APIs de LLMs
reqwest = { version = "0.11", features = ["json", "stream"] }

# Manejo de errores
thiserror = "1.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Fecha y hora
chrono = { version = "0.4", features = ["serde"] }

# Collections
im = "15.1"  # Estructuras de datos persistentes

# Testing
mockito = "1.2"  # Mocks de APIs
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

| Operación | Target | Medición | Status |
|-----------|--------|----------|--------|
| `select_provider()` | <50ms | Tiempo desde CTX7D hasta RoutingDecision | ⏸️ TBD |
| `enrich_prompt()` | <100ms | Tiempo de consulta TelescopeDB + VoxelDB + construcción | ⏸️ TBD |
| `health_check()` (por provider) | <200ms | Ping al API del provider | ⏸️ TBD |
| **Routing overhead total** | **<200ms** | **Desde query hasta envío al LLM** | **🎯 CRÍTICO** |
| Memoria RAM (Hub) | <50 MB | RSS después de 1000 decisiones | ⏸️ TBD |
| Accuracy de ContextualBestFit | >85% | % de veces que usuario está satisfecho con provider elegido | ⏸️ TBD |

**Nota crítica:** El routing overhead de <200ms es ESENCIAL porque se suma a la latencia del LLM (1-2 segundos). Si routing demora >500ms, UX se degrada.

### Benchmarks Esperados

```rust
// tests/benchmarks/hubspoke_bench.rs

#[bench]
fn bench_contextual_best_fit_routing(b: &mut Bencher) {
    let hub = Hub::new(default_config()).unwrap();
    let ctx7d = ContextTensor7D::example();
    let user_history = UserHistory::example();
    
    b.iter(|| {
        hub.route_contextual_best_fit("test query", &ctx7d, &user_history)
    });
}

// Target: ~30-50ms por decisión de routing
```

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
// tests/hubspoke_test.rs

#[tokio::test]
async fn test_contextual_best_fit_selects_anthropic_for_complex_reasoning() {
    let hub = Hub::new(default_config()).unwrap();
    
    let ctx7d = ContextTensor7D {
        semantic: 0.95,      // Muy complejo
        intentional: 0.85,   // Claro
        temporal: 0.30,      // NO urgente
        emotional: 0.70,     // Empático
        associative: 0.75,
        evaluative: 0.90,    // Alto juicio crítico
        metalinguistic: 0.80,
    };
    
    let decision = hub.select_provider(
        "Explicame la paradoja EPR de mecánica cuántica",
        &ctx7d,
        "user_123",
        Some(RoutingStrategy::ContextualBestFit),
    ).await.unwrap();
    
    assert_eq!(decision.selected_provider, LLMProvider::Anthropic);
    assert!(decision.reasoning.contains("Alta complejidad"));
}

#[tokio::test]
async fn test_cost_optimized_selects_perplexity_when_budget_low() {
    let mut hub = Hub::new(default_config()).unwrap();
    
    let user_history = UserHistory {
        budget_remaining_usd: 2.0,  // Muy bajo
        ..Default::default()
    };
    
    let ctx7d = ContextTensor7D::default();
    
    let decision = hub.route_cost_optimized(&ctx7d, &user_history)
        .await.unwrap();
    
    assert_eq!(decision.selected_provider, LLMProvider::Perplexity);
}

#[tokio::test]
async fn test_failover_when_primary_provider_down() {
    let mut hub = Hub::new(default_config()).unwrap();
    
    // Marcar OpenAI como down
    hub.get_spoke_mut(LLMProvider::OpenAI)
        .unwrap()
        .health_status = HealthStatus::Down;
    
    let ctx7d = ContextTensor7D::default();
    let decision = hub.select_provider(
        "test",
        &ctx7d,
        "user_123",
        Some(RoutingStrategy::RoundRobin),
    ).await.unwrap();
    
    // Debe seleccionar Anthropic o Perplexity (NO OpenAI)
    assert_ne!(decision.selected_provider, LLMProvider::OpenAI);
    assert!(decision.was_failover || decision.selected_provider != LLMProvider::OpenAI);
}
```

### Integration Tests

```rust
// tests/integration/hubspoke_wizard_integration.rs

#[tokio::test]
async fn test_full_pipeline_hubspoke_to_wizard() {
    // Setup: Hub + Wizard
    let hub = Hub::new(default_config()).unwrap();
    let wizard = Wizard::new(wizard_config()).unwrap();
    
    // Query de usuario
    let query = "Qué es async/await en Rust?";
    let ctx7d = ContextToken7D::analyze(query).await.unwrap();
    
    // Routing
    let decision = hub.select_provider(query, &ctx7d, "user_123", None)
        .await.unwrap();
    
    // Enriquecimiento
    let enriched_prompt = hub.enrich_prompt(query, &ctx7d, "user_123")
        .await.unwrap();
    
    // Ejecución en Wizard
    let response = wizard.execute(
        decision.selected_provider,
        &enriched_prompt,
    ).await.unwrap();
    
    // Validaciones
    assert!(response.text.len() > 100);
    assert!(response.metadata.provider == decision.selected_provider);
    assert!(response.metadata.cost_usd > 0.0);
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_routing_always_selects_valid_provider(
        semantic in 0.0..1.0f64,
        intentional in 0.0..1.0f64,
        temporal in 0.0..1.0f64,
    ) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let hub = Hub::new(default_config()).unwrap();
        
        let ctx7d = ContextTensor7D {
            semantic,
            intentional,
            temporal,
            ..Default::default()
        };
        
        let decision = rt.block_on(async {
            hub.select_provider("test", &ctx7d, "user_123", None).await
        }).unwrap();
        
        // PROPIEDAD: Provider seleccionado SIEMPRE debe estar en enabled_providers
        prop_assert!(hub.config.enabled_providers.contains(&decision.selected_provider));
    }
}
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/multi_agent/hubspoke/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HubSpokeError {
    #[error("Ningún provider está disponible (todos down)")]
    AllProvidersDown,
    
    #[error("Provider {0:?} no está configurado")]
    ProviderNotConfigured(LLMProvider),
    
    #[error("Presupuesto excedido: ${0:.2} / ${1:.2}")]
    BudgetExceeded(f64, f64),
    
    #[error("Rate limit excedido para provider {0:?}")]
    RateLimitExceeded(LLMProvider),
    
    #[error("Health check falló: {0}")]
    HealthCheckFailed(String),
    
    #[error("Error de TelescopeDB: {0}")]
    TelescopeDBError(#[from] crate::cells::telescopedb::Error),
    
    #[error("Error de VoxelDB: {0}")]
    VoxelDBError(#[from] crate::cells::voxeldb::Error),
    
    #[error("Error de Wizard: {0}")]
    WizardError(#[from] crate::llm::wizard::Error),
    
    #[error("Error de red: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, HubSpokeError>;
```

### Manejo de Errores en Routing

```rust
impl Hub {
    pub async fn select_provider_with_retry(
        &mut self,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
        max_retries: u32,
    ) -> Result<RoutingDecision> {
        let mut attempts = 0;
        
        loop {
            match self.select_provider(query, ctx7d, user_id, None).await {
                Ok(decision) => return Ok(decision),
                
                Err(HubSpokeError::RateLimitExceeded(provider)) => {
                    attempts += 1;
                    
                    if attempts >= max_retries {
                        return Err(HubSpokeError::AllProvidersDown);
                    }
                    
                    tracing::warn!(
                        "Rate limit en {:?}, reintentando con otro provider...",
                        provider
                    );
                    
                    // Marcar provider como degradado temporalmente
                    self.get_spoke_mut(provider)?
                        .health_status = HealthStatus::Degraded;
                    
                    // Esperar antes de reintentar
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                
                Err(e) => return Err(e),
            }
        }
    }
}
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **ROADMAP_V2/02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md** - Análisis dimensional que alimenta ContextualBestFit
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md** - Historial de usuario para decisiones de routing
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/VOXELDB.md** - Templates para enriquecimiento de prompts
- **ROADMAP_V2/01_ARQUITECTURA/FLUJO_DATOS_END_TO_END.md** - Ubicación de HubSpoke en el pipeline

### Decisiones Arquitectónicas

- **DA-010:** Multi-LLM Orchestration - HubSpoke pattern es la arquitectura aprobada
- **DA-002:** Context Token 7D Integration - ContextualBestFit usa las 7 dimensiones
- **DA-001:** Local-First Architecture - HubSpoke almacena decisiones localmente
- **BITA-2:** ACA-7D Specification - Framework teórico del routing contextual

### FUSION_BAYESIANA

- **FUSION_BAYESIANA/08_DIAGRAMA_SISTEMA.md** (Líneas 290-320, 794-870) - Diagramas detallados de HubSpoke + Wizard
- **FUSION_BAYESIANA/04_SANDBOX_INTEGRATION.md** - Validación multi-LLM en sandbox
- **FUSION_BAYESIANA/06_HARMONY_CTX7D.md** - Integración de 7D con HubSpoke

### Código de Referencia

- `B20250915-data-compressor/src/llm/wizard.rs` - Implementación actual de Wizard (punto de partida)
- `B20250915-data-compressor/src/llm/providers/*.rs` - Clients de OpenAI, Anthropic, Perplexity
- `B20250915-data-compressor/src/core/context_token.rs` - Generación de CTX7D

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Semanas 7-8)

1. **Crear estructura base:**
   ```bash
   mkdir -p src/multi_agent/hubspoke
   touch src/multi_agent/hubspoke/{mod.rs,hub.rs,spoke.rs,routing.rs,error.rs}
   ```

2. **Implementar structs core:**
   - `Hub` con `HubSpokeConfig`
   - `Spoke` con `ProviderCapabilities`
   - `RoutingDecision` con metadatos

3. **Implementar estrategias de routing:**
   - ✅ `RoundRobin` (simple, empezar aquí)
   - ✅ `LeastLoaded` (requiere métricas en tiempo real)
   - ⭐ `ContextualBestFit` (algoritmo estrella, implementar con cuidado)
   - ✅ `CostOptimized` (requiere integración con TelescopeDB)

4. **Integrar con Wizard-Multi-LLM:**
   - Modificar `Wizard::execute()` para recibir `RoutingDecision`
   - Pasar provider seleccionado + prompt enriquecido

5. **Testing básico:**
   - Unit tests de cada estrategia
   - Integration test con Wizard
   - Property test de validez de decisiones

### Mejoras v1.5 (Semanas 9-10)

6. **Health checks automáticos:**
   - Tarea async que hace ping cada 30 segundos
   - Actualiza `HealthStatus` de cada Spoke
   - Alertas si provider está down >5 minutos

7. **Dashboard de métricas:**
   - Endpoint `/api/hubspoke/metrics` con:
     * Distribución de requests por provider
     * Costos acumulados
     * Latencias promedio
     * Tasa de failover

8. **A/B testing de modelos:**
   - Permitir routing a `gpt-4-0613` vs `gpt-4-1106-preview`
   - Comparar calidad de respuestas
   - Deprecar modelos viejos automáticamente

### Mejoras v2.0 (Futuro)

9. **Machine Learning en routing:**
   - Entrenar modelo que predice satisfacción del usuario
   - Usar historial de feedback (👍/👎) como training data
   - Superar ContextualBestFit con aprendizaje continuo

10. **Multimodal routing:**
    - Diferentes estrategias para texto vs voz vs imagen
    - GPT-4V para queries visuales
    - Claude para conversaciones empáticas

11. **Cost budgeting avanzado:**
    - Presupuesto por usuario
    - Alertas cuando usuario gasta >80% del presupuesto
    - Auto-switch a providers baratos cuando quedan $2

---

**Estado:** 📋 ESPECIFICACIÓN  
**Complejidad:** 🟡 MEDIA (routing simple) → 🔴 ALTA (ContextualBestFit completo)  
**Prioridad:** 🔴 CRÍTICA (Fase 1, Semanas 7-8)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec*
