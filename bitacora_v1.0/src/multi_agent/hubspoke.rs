

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use thiserror::Error;

/// Errores del HubSpoke Navigator
#[derive(Error, Debug)]
pub enum HubSpokeError {
    #[error("Ningún provider disponible")]
    NoProvidersAvailable,
    
    #[error("Provider {0:?} no está habilitado")]
    ProviderDisabled(LLMProvider),
    
    #[error("Todos los providers fallaron")]
    AllProvidersFailed,
    
    #[error("Presupuesto diario excedido: ${0:.2} / ${1:.2}")]
    BudgetExceeded(f64, f64),
    
    #[error("Latencia excedida: {0}ms > {1}ms")]
    LatencyExceeded(u64, u64),
    
    #[error("Error de configuración: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, HubSpokeError>;

/// Configuración del HubSpoke Navigator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubSpokeConfig {
    /// Estrategia de routing por defecto
    pub default_strategy: RoutingStrategy,
    
    /// Providers habilitados
    pub enabled_providers: Vec<LLMProvider>,
    
    /// Presupuesto máximo diario (USD)
    pub daily_budget_usd: f64,
    
    /// Latencia máxima aceptable (ms)
    pub max_latency_ms: u64,
    
    /// Failover automático habilitado
    pub enable_failover: bool,
}

impl Default for HubSpokeConfig {
    fn default() -> Self {
        Self {
            default_strategy: RoutingStrategy::ContextualBestFit,
            enabled_providers: vec![
                LLMProvider::OpenAI,
                LLMProvider::Anthropic,
                LLMProvider::Perplexity,
            ],
            daily_budget_usd: 10.0,
            max_latency_ms: 5000,
            enable_failover: true,
        }
    }
}

/// Hub Central - Cerebro del Navigator
pub struct Hub {
    /// Configuración
    config: HubSpokeConfig,
    
    /// Spokes (uno por provider)
    spokes: HashMap<LLMProvider, Spoke>,
    
    /// Registro de decisiones de routing
    routing_history: Vec<RoutingDecision>,
    
    /// Métricas globales
    metrics: HubMetrics,
    
    /// Gasto acumulado hoy (USD)
    daily_spend_usd: f64,
}

impl Hub {
    /// Crear nuevo Hub con configuración
    pub fn new(config: HubSpokeConfig) -> Result<Self> {
        if config.enabled_providers.is_empty() {
            return Err(HubSpokeError::ConfigError(
                "Debe habilitar al menos un provider".to_string()
            ));
        }
        
        // Inicializar spokes
        let mut spokes = HashMap::new();
        for provider in &config.enabled_providers {
            spokes.insert(*provider, Spoke::new(*provider));
        }
        
        Ok(Self {
            config,
            spokes,
            routing_history: Vec::new(),
            metrics: HubMetrics::default(),
            daily_spend_usd: 0.0,
        })
    }
    
    /// Seleccionar provider óptimo basado en contexto
    pub fn route_query(&mut self, query: &str, ctx7d: ContextTensor7D) -> Result<LLMProvider> {
        // Verificar presupuesto
        if self.daily_spend_usd >= self.config.daily_budget_usd {
            return Err(HubSpokeError::BudgetExceeded(
                self.daily_spend_usd,
                self.config.daily_budget_usd
            ));
        }
        
        let start = Instant::now();
        
        // Aplicar estrategia de routing
        let selected = match self.config.default_strategy {
            RoutingStrategy::RoundRobin => self.route_round_robin()?,
            RoutingStrategy::LeastLoaded => self.route_least_loaded()?,
            RoutingStrategy::ContextualBestFit => self.route_contextual(&ctx7d)?,
            RoutingStrategy::CostOptimized => self.route_cost_optimized(&ctx7d)?,
        };
        
        // Registrar decisión
        let decision = RoutingDecision {
            timestamp: chrono::Utc::now().timestamp(),
            query: query.to_string(),
            ctx7d: ctx7d.clone(),
            selected_provider: selected,
            strategy: self.config.default_strategy,
            reasoning: self.generate_reasoning(selected, &ctx7d),
            estimated_cost: self.estimate_cost(selected, query.len()),
            was_failover: false,
        };
        
        self.routing_history.push(decision);
        self.metrics.total_routes += 1;
        self.metrics.routes_by_provider.entry(selected)
            .and_modify(|c| *c += 1)
            .or_insert(1);
        
        let routing_time = start.elapsed().as_millis() as u64;
        self.metrics.avg_routing_time_ms = 
            (self.metrics.avg_routing_time_ms + routing_time) / 2;
        
        Ok(selected)
    }
    
    /// Ejecutar query en provider seleccionado con failover automático
    pub fn execute_query(
        &mut self,
        provider: LLMProvider,
        query: &str
    ) -> Result<String> {
        // Intentar con provider seleccionado
        let spoke = self.spokes.get_mut(&provider)
            .ok_or(HubSpokeError::ProviderDisabled(provider))?;
        
        match spoke.execute(query) {
            Ok(response) => {
                self.metrics.successful_requests += 1;
                Ok(response)
            },
            Err(e) if self.config.enable_failover => {
                // Failover automático
                self.metrics.failover_count += 1;
                self.execute_failover(provider, query)
            },
            Err(_) => {
                self.metrics.failed_requests += 1;
                Err(HubSpokeError::AllProvidersFailed)
            }
        }
    }
    
    /// **[NUEVO - Task 7.1]** Ejecutar query con routing inteligente + API real
    ///
    /// Este método integra:
    /// 1. HubSpoke routing (selección óptima de provider)
    /// 2. LLMClient execution (llamada real a API)
    /// 3. Metrics tracking (latencia, costo, tokens)
    /// 4. Failover automático si falla el provider seleccionado
    ///
    /// # Argumentos
    /// - `query`: Prompt del usuario
    /// - `ctx7d`: Tensor de contexto 7D para routing inteligente
    /// - `user_id`: ID del usuario (para histórico)
    ///
    /// # Ejemplo
    /// ```no_run
    /// use bitacora::multi_agent::{Hub, HubSpokeConfig, LLMRequest};
    /// use bitacora::context_token::ContextTensor7D;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut hub = Hub::new(HubSpokeConfig::default())?;
    /// let ctx7d = ContextTensor7D::default();
    /// 
    /// let response = hub.query_with_routing(
    ///     "Explica qué es la compresión fractal",
    ///     &ctx7d,
    ///     "user_123"
    /// ).await?;
    ///
    /// println!("LLM: {}", response.text);
    /// println!("Cost: ${:.4}", response.cost_usd);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn query_with_routing(
        &mut self,
        query: &str,
        ctx7d: &ContextTensor7D,
        user_id: &str,
    ) -> std::result::Result<crate::multi_agent::llm_client::LLMResponse, Box<dyn std::error::Error + Send + Sync>> {
        use crate::multi_agent::llm_client::{LLMClient, LLMRequest, ClientConfig};
        
        // 1. Seleccionar provider óptimo usando routing contextual
        let selected = self.route_query(query, ctx7d.clone())?;
        
        // 2. Obtener API key del entorno
        let api_key = match selected {
            LLMProvider::OpenAI => std::env::var("OPENAI_API_KEY")
                .map_err(|_| "OPENAI_API_KEY no configurada en environment")?,
            LLMProvider::Anthropic => std::env::var("ANTHROPIC_API_KEY")
                .map_err(|_| "ANTHROPIC_API_KEY no configurada en environment")?,
            LLMProvider::Perplexity => std::env::var("PERPLEXITY_API_KEY")
                .map_err(|_| "PERPLEXITY_API_KEY no configurada en environment")?,
        };
        
        // 3. Crear cliente para el provider
        let client = LLMClient::new(selected, api_key, Some(ClientConfig {
            timeout_secs: (self.config.max_latency_ms / 1000).max(30),
            max_retries: if self.config.enable_failover { 3 } else { 1 },
            base_url: None,
        }))?;
        
        // 4. Enriquecer prompt (aquí se puede agregar contexto de TelescopeDB/VoxelDB)
        let enriched_prompt = self.enrich_prompt(query, ctx7d, user_id);
        
        // 5. Ejecutar query
        let request = LLMRequest::new(enriched_prompt)
            .with_max_tokens(1000)
            .with_temperature(0.7);
        
        let response = client.query(&request).await?;
        
        // 6. Actualizar métricas del Hub
        self.update_metrics(
            selected,
            response.latency_ms,
            response.tokens_used,
            response.cost_usd,
        );
        
        // 7. Actualizar gasto diario
        self.daily_spend_usd += response.cost_usd;
        
        Ok(response)
    }
    
    /// Enriquecer prompt con contexto del usuario (placeholder v1.0)
    fn enrich_prompt(&self, query: &str, _ctx7d: &ContextTensor7D, _user_id: &str) -> String {
        // TODO (Phase 8): Integrar TelescopeDB (histórico) + VoxelDB (templates)
        // Por ahora, devolver query original
        query.to_string()
    }
    
    /// Actualizar métricas del Hub después de ejecución
    fn update_metrics(
        &mut self,
        provider: LLMProvider,
        latency_ms: u64,
        tokens_used: usize,
        cost_usd: f64,
    ) {
        self.metrics.successful_requests += 1;
        self.metrics.total_tokens_consumed += tokens_used as u64;
        self.metrics.total_cost_usd += cost_usd;
        
        // Actualizar métricas del spoke
        if let Some(spoke) = self.spokes.get_mut(&provider) {
            spoke.capabilities.avg_latency_ms = 
                ((spoke.capabilities.avg_latency_ms as u64 + latency_ms) / 2) as u32;
        }
    }
    
    /// Routing Round Robin (equitativo)
    fn route_round_robin(&self) -> Result<LLMProvider> {
        let total_routes = self.metrics.total_routes as usize;
        let providers = &self.config.enabled_providers;
        
        if providers.is_empty() {
            return Err(HubSpokeError::NoProvidersAvailable);
        }
        
        Ok(providers[total_routes % providers.len()])
    }
    
    /// Routing Least Loaded (menos cargado)
    fn route_least_loaded(&self) -> Result<LLMProvider> {
        self.spokes.iter()
            .filter(|(p, _)| self.config.enabled_providers.contains(p))
            .min_by_key(|(_, spoke)| spoke.pending_requests.len())
            .map(|(provider, _)| *provider)
            .ok_or(HubSpokeError::NoProvidersAvailable)
    }
    
    /// Routing Contextual (basado en CTX7D) - RECOMENDADO
    fn route_contextual(&self, ctx7d: &ContextTensor7D) -> Result<LLMProvider> {
        let mut scores: Vec<(LLMProvider, f64)> = Vec::new();
        
        for provider in &self.config.enabled_providers {
            let spoke = self.spokes.get(provider).unwrap();
            let capabilities = &spoke.capabilities;
            
            // Calcular score basado en CTX7D
            let mut score = 0.0;
            
            // Semantic (complejidad) → prefiere mejor reasoning
            score += ctx7d.semantic * capabilities.reasoning_quality * 0.3;
            
            // Intentional (claridad) → todos iguales
            score += ctx7d.intentional * 0.1;
            
            // Temporal (urgencia) → prefiere baja latencia
            if ctx7d.temporal > 0.7 {
                score += (1.0 - capabilities.avg_latency_ms as f64 / 5000.0) * 0.2;
            }
            
            // Associative (relaciones) → prefiere research
            score += ctx7d.associative * capabilities.research_quality * 0.2;
            
            // Evaluative (juicio crítico) → prefiere reasoning
            score += ctx7d.evaluative * capabilities.reasoning_quality * 0.2;
            
            scores.push((*provider, score));
        }
        
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        scores.first()
            .map(|(p, _)| *p)
            .ok_or(HubSpokeError::NoProvidersAvailable)
    }
    
    /// Routing Cost Optimized (minimizar costo)
    fn route_cost_optimized(&self, ctx7d: &ContextTensor7D) -> Result<LLMProvider> {
        // Si complejidad baja, usar Perplexity (más barato)
        if ctx7d.semantic < 0.5 && ctx7d.evaluative < 0.5 {
            if self.config.enabled_providers.contains(&LLMProvider::Perplexity) {
                return Ok(LLMProvider::Perplexity);
            }
        }
        
        // Si complejidad alta, usar mejor (aunque caro)
        if ctx7d.semantic > 0.8 || ctx7d.evaluative > 0.8 {
            if self.config.enabled_providers.contains(&LLMProvider::Anthropic) {
                return Ok(LLMProvider::Anthropic);
            }
        }
        
        // Por defecto, OpenAI (balance)
        if self.config.enabled_providers.contains(&LLMProvider::OpenAI) {
            return Ok(LLMProvider::OpenAI);
        }
        
        Err(HubSpokeError::NoProvidersAvailable)
    }
    
    /// Ejecutar failover a otro provider
    fn execute_failover(&mut self, failed_provider: LLMProvider, query: &str) -> Result<String> {
        // Intentar con otros providers
        for provider in &self.config.enabled_providers {
            if *provider != failed_provider {
                if let Some(spoke) = self.spokes.get_mut(provider) {
                    if let Ok(response) = spoke.execute(query) {
                        // Actualizar última decisión como failover
                        if let Some(last) = self.routing_history.last_mut() {
                            last.was_failover = true;
                            last.selected_provider = *provider;
                        }
                        return Ok(response);
                    }
                }
            }
        }
        
        Err(HubSpokeError::AllProvidersFailed)
    }
    
    /// Generar explicación humana del routing
    fn generate_reasoning(&self, provider: LLMProvider, ctx7d: &ContextTensor7D) -> String {
        let strategy = self.config.default_strategy;
        
        match strategy {
            RoutingStrategy::ContextualBestFit => {
                if ctx7d.semantic > 0.8 {
                    format!("{:?} seleccionado por alta complejidad (semantic={:.2})", provider, ctx7d.semantic)
                } else if ctx7d.temporal > 0.7 {
                    format!("{:?} seleccionado por urgencia (temporal={:.2})", provider, ctx7d.temporal)
                } else {
                    format!("{:?} seleccionado por análisis contextual 7D", provider)
                }
            },
            RoutingStrategy::CostOptimized => {
                format!("{:?} seleccionado para optimizar costo", provider)
            },
            RoutingStrategy::RoundRobin => {
                format!("{:?} seleccionado por round-robin", provider)
            },
            RoutingStrategy::LeastLoaded => {
                format!("{:?} seleccionado por menor carga", provider)
            },
        }
    }
    
    /// Estimar costo de un query
    fn estimate_cost(&self, provider: LLMProvider, query_len: usize) -> f64 {
        let spoke = self.spokes.get(&provider).unwrap();
        let caps = &spoke.capabilities;
        
        // Estimar tokens (aprox 4 chars = 1 token)
        let input_tokens = (query_len / 4) as f64;
        let output_tokens = 500.0; // Asumir 500 tokens de respuesta
        
        (input_tokens / 1000.0) * caps.cost_per_1k_input +
        (output_tokens / 1000.0) * caps.cost_per_1k_output
    }
    
    /// Obtener métricas del Hub
    pub fn metrics(&self) -> &HubMetrics {
        &self.metrics
    }
    
    /// Obtener histórico de decisiones
    pub fn routing_history(&self) -> &[RoutingDecision] {
        &self.routing_history
    }
    
    /// Simular gasto diario (solo para testing)
    #[cfg(test)]
    pub fn set_daily_spend(&mut self, amount: f64) {
        self.daily_spend_usd = amount;
    }
}

/// Spoke - Especialista en un provider específico
pub struct Spoke {
    /// Provider al que representa
    provider: LLMProvider,
    
    /// Capacidades del provider
    capabilities: ProviderCapabilities,
    
    /// Estado actual
    health_status: HealthStatus,
    
    /// Cola de requests pendientes
    pending_requests: VecDeque<String>,
}

impl Spoke {
    fn new(provider: LLMProvider) -> Self {
        Self {
            provider,
            capabilities: ProviderCapabilities::for_provider(provider),
            health_status: HealthStatus::Healthy,
            pending_requests: VecDeque::new(),
        }
    }
    
    /// Ejecutar query (STUB para v1.0)
    fn execute(&mut self, query: &str) -> Result<String> {
        // STUB: En v1.0, simulamos ejecución
        self.pending_requests.push_back(query.to_string());
        
        let response = format!(
            "[{:?} RESPONSE STUB] Query procesada: '{}...'. \
            En v2.0 se integrará API real del provider.",
            self.provider,
            &query[..query.len().min(50)]
        );
        
        self.pending_requests.pop_front();
        Ok(response)
    }
}

/// Estrategias de routing
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RoutingStrategy {
    RoundRobin,
    LeastLoaded,
    ContextualBestFit,
    CostOptimized,
}

/// Providers soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Perplexity,
}

/// Capacidades de un provider
#[derive(Debug, Clone)]
pub struct ProviderCapabilities {
    pub code_quality: f64,
    pub reasoning_quality: f64,
    pub research_quality: f64,
    pub creative_quality: f64,
    pub max_context_tokens: usize,
    pub cost_per_1k_input: f64,
    pub cost_per_1k_output: f64,
    pub avg_latency_ms: u32,
}

impl ProviderCapabilities {
    fn for_provider(provider: LLMProvider) -> Self {
        match provider {
            LLMProvider::OpenAI => Self {
                code_quality: 0.9,
                reasoning_quality: 0.85,
                research_quality: 0.80,
                creative_quality: 0.90,
                max_context_tokens: 128_000,
                cost_per_1k_input: 0.01,
                cost_per_1k_output: 0.03,
                avg_latency_ms: 1500,
            },
            LLMProvider::Anthropic => Self {
                code_quality: 0.88,
                reasoning_quality: 0.95,
                research_quality: 0.85,
                creative_quality: 0.92,
                max_context_tokens: 200_000,
                cost_per_1k_input: 0.015,
                cost_per_1k_output: 0.075,
                avg_latency_ms: 2000,
            },
            LLMProvider::Perplexity => Self {
                code_quality: 0.70,
                reasoning_quality: 0.75,
                research_quality: 0.95,
                creative_quality: 0.65,
                max_context_tokens: 4_000,
                cost_per_1k_input: 0.001,
                cost_per_1k_output: 0.001,
                avg_latency_ms: 800,
            },
        }
    }
}

/// Estado de salud del spoke
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}

/// Context Tensor 7D (mock para compilación)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTensor7D {
    pub semantic: f64,
    pub intentional: f64,
    pub temporal: f64,
    pub emotional: f64,
    pub associative: f64,
    pub evaluative: f64,
    pub metalinguistic: f64,
}

impl ContextTensor7D {
    /// Create from text prompt (simple heuristic analysis)
    pub fn from_prompt(prompt: &str) -> Self {
        let lower = prompt.to_lowercase();
        let len = prompt.len() as f64;
        
        // Simple heuristics for dimensional analysis
        let semantic = (len / 500.0).clamp(0.3, 0.9);
        let intentional = if lower.contains('?') { 0.8 } else { 0.5 };
        let temporal = if lower.contains("ahora") || lower.contains("urgente") { 0.8 } else { 0.4 };
        let emotional = if lower.contains('!') { 0.7 } else { 0.5 };
        let associative = (prompt.split_whitespace().count() as f64 / 50.0).clamp(0.3, 0.8);
        let evaluative = if lower.contains("por qué") || lower.contains("cómo") { 0.7 } else { 0.5 };
        let metalinguistic = if lower.contains("significa") || lower.contains("define") { 0.8 } else { 0.5 };
        
        Self {
            semantic,
            intentional,
            temporal,
            emotional,
            associative,
            evaluative,
            metalinguistic,
        }
    }
}

/// Decisión de routing registrada
#[derive(Debug, Clone, Serialize)]
pub struct RoutingDecision {
    pub timestamp: i64,
    pub query: String,
    pub ctx7d: ContextTensor7D,
    pub selected_provider: LLMProvider,
    pub strategy: RoutingStrategy,
    pub reasoning: String,
    pub estimated_cost: f64,
    pub was_failover: bool,
}

/// Métricas del Hub
#[derive(Debug, Clone, Default)]
pub struct HubMetrics {
    pub total_routes: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub failover_count: u64,
    pub avg_routing_time_ms: u64,
    pub routes_by_provider: HashMap<LLMProvider, u64>,
    pub total_tokens_consumed: u64,
    pub total_cost_usd: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hub_creation() {
        let hub = Hub::new(HubSpokeConfig::default()).unwrap();
        assert_eq!(hub.spokes.len(), 3);
        assert_eq!(hub.daily_spend_usd, 0.0);
    }
    
    #[test]
    fn test_round_robin_routing() {
        let mut config = HubSpokeConfig::default();
        config.default_strategy = RoutingStrategy::RoundRobin;
        let mut hub = Hub::new(config).unwrap();
        
        let ctx7d = ContextTensor7D {
            semantic: 0.5, intentional: 0.5, temporal: 0.5,
            emotional: 0.5, associative: 0.5, evaluative: 0.5,
            metalinguistic: 0.5,
        };
        
        let p1 = hub.route_query("test1", ctx7d.clone()).unwrap();
        let p2 = hub.route_query("test2", ctx7d.clone()).unwrap();
        let p3 = hub.route_query("test3", ctx7d.clone()).unwrap();
        
        // Debe rotar entre providers
        assert_ne!(p1, p2);
        assert_ne!(p2, p3);
    }
    
    #[test]
    fn test_contextual_routing_high_complexity() {
        let mut config = HubSpokeConfig::default();
        config.default_strategy = RoutingStrategy::ContextualBestFit;
        let mut hub = Hub::new(config).unwrap();
        
        let ctx7d_complex = ContextTensor7D {
            semantic: 0.95,      // Alta complejidad
            intentional: 0.9,
            temporal: 0.2,       // No urgente
            emotional: 0.5,
            associative: 0.8,
            evaluative: 0.9,     // Alto juicio crítico
            metalinguistic: 0.7,
        };
        
        let provider = hub.route_query("Explain async Rust", ctx7d_complex).unwrap();
        
        // Alta complejidad debería preferir Anthropic (mejor reasoning)
        assert_eq!(provider, LLMProvider::Anthropic);
    }
    
    #[test]
    fn test_cost_optimized_routing() {
        let mut config = HubSpokeConfig::default();
        config.default_strategy = RoutingStrategy::CostOptimized;
        let mut hub = Hub::new(config).unwrap();
        
        let ctx7d_simple = ContextTensor7D {
            semantic: 0.3,       // Baja complejidad
            intentional: 0.8,
            temporal: 0.5,
            emotional: 0.5,
            associative: 0.4,
            evaluative: 0.3,     // Bajo juicio crítico
            metalinguistic: 0.2,
        };
        
        let provider = hub.route_query("What's 2+2?", ctx7d_simple).unwrap();
        
        // Baja complejidad debería usar Perplexity (más barato)
        assert_eq!(provider, LLMProvider::Perplexity);
    }
    
    #[test]
    fn test_budget_enforcement() {
        let mut config = HubSpokeConfig::default();
        config.daily_budget_usd = 1.0;
        let mut hub = Hub::new(config).unwrap();
        
        hub.daily_spend_usd = 1.5; // Simular gasto excedido
        
        let ctx7d = ContextTensor7D {
            semantic: 0.5, intentional: 0.5, temporal: 0.5,
            emotional: 0.5, associative: 0.5, evaluative: 0.5,
            metalinguistic: 0.5,
        };
        
        let result = hub.route_query("test", ctx7d);
        assert!(matches!(result, Err(HubSpokeError::BudgetExceeded(_, _))));
    }
    
    #[test]
    fn test_failover_execution() {
        let hub = Hub::new(HubSpokeConfig::default()).unwrap();
        let metrics = hub.metrics();
        
        assert_eq!(metrics.total_routes, 0);
        assert_eq!(metrics.failover_count, 0);
    }
}
