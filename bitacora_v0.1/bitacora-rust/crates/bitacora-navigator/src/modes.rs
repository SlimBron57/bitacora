//! Modos de Navegación del Sistema Híbrido
//!
//! Define los diferentes modos de operación del navegador híbrido:
//! - Core: Modo básico uni-navegador
//! - Advanced: Modo con múltiples estrategias
//! - AIDriven: Modo con decisiones AI automáticas

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::errors::{NavigatorError, Result};

/// Modos de operación del navegador híbrido
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigatorMode {
    /// Modo básico - Un navegador, una estrategia
    Core {
        /// Estrategia principal a utilizar
        primary_strategy: NavigationStrategy,
        /// Configuraciones específicas del modo core
        config: CoreModeConfig,
    },
    
    /// Modo avanzado - Múltiples estrategias con fallback
    Advanced {
        /// Estrategias ordenadas por prioridad
        strategies: Vec<NavigationStrategy>,
        /// Configuraciones específicas del modo avanzado
        config: AdvancedModeConfig,
    },
    
    /// Modo dirigido por AI - Decisiones automáticas
    AIDriven {
        /// Configuración del motor AI
        ai_config: AIConfig,
        /// Estrategias disponibles para AI
        available_strategies: Vec<NavigationStrategy>,
    },
}

/// Estrategias de navegación disponibles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationStrategy {
    /// Navegación basada en índices pre-construidos
    IndexBased {
        /// Tipo de índice a utilizar
        index_type: IndexType,
        /// Configuraciones específicas del índice
        config: IndexConfig,
    },
    
    /// Navegación basada en queries dinámicas
    QueryBased {
        /// Tipo de query a ejecutar
        query_type: QueryType,
        /// Configuraciones específicas de query
        config: QueryConfig,
    },
    
    /// Navegación híbrida optimizada
    HybridOptimized {
        /// Ratio de uso índice vs query (0.0 = solo query, 1.0 = solo índice)
        index_ratio: f32,
        /// Configuraciones híbridas
        config: HybridConfig,
    },
}

/// Tipos de índice disponibles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IndexType {
    /// Índice de proyectos
    ProjectIndex,
    /// Índice de topics
    TopicIndex,
    /// Índice de acciones
    ActionIndex,
    /// Índice de sparks
    SparkIndex,
    /// Índice de contenido completo
    FullTextIndex,
    /// Índice semántico (embeddings)
    SemanticIndex,
}

/// Tipos de query disponibles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    /// Query directa a MongoDB
    DirectMongo,
    /// Query con agregaciones
    AggregatedMongo,
    /// Query con filtros complejos
    FilteredQuery,
    /// Query semántica
    SemanticQuery,
}

/// Configuración del modo Core
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreModeConfig {
    /// Timeout para operaciones (ms)
    pub operation_timeout_ms: u64,
    /// Usar cache para resultados
    pub enable_cache: bool,
    /// Tamaño máximo de cache
    pub max_cache_size: usize,
    /// TTL del cache (segundos)
    pub cache_ttl_seconds: u64,
}

/// Configuración del modo Advanced
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedModeConfig {
    /// Timeout para cada estrategia (ms)
    pub strategy_timeout_ms: u64,
    /// Máximo número de estrategias a intentar
    pub max_strategy_attempts: usize,
    /// Combinar resultados de múltiples estrategias
    pub combine_results: bool,
    /// Configuraciones específicas por estrategia
    pub strategy_configs: HashMap<String, serde_json::Value>,
}

/// Configuración AI
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AIConfig {
    /// Nivel de confianza mínimo para decisiones AI
    pub confidence_threshold: f32,
    /// Usar aprendizaje automático
    pub enable_ml: bool,
    /// Modelo a utilizar para decisiones
    pub model_name: String,
    /// Configuraciones específicas del modelo
    pub model_config: HashMap<String, serde_json::Value>,
}

/// Configuración de índices
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Actualización automática de índices
    pub auto_refresh: bool,
    /// Intervalo de actualización (segundos)
    pub refresh_interval_seconds: u64,
    /// Usar compresión en índices
    pub enable_compression: bool,
    /// Configuraciones específicas del tipo de índice
    pub type_specific: HashMap<String, serde_json::Value>,
}

/// Configuración de queries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryConfig {
    /// Límite por defecto de resultados
    pub default_limit: usize,
    /// Usar paginación automática
    pub enable_pagination: bool,
    /// Tamaño de página para paginación
    pub page_size: usize,
    /// Configuraciones específicas del tipo de query
    pub type_specific: HashMap<String, serde_json::Value>,
}

/// Configuración híbrida
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridConfig {
    /// Algoritmo de combinación de resultados
    pub combination_algorithm: CombinationAlgorithm,
    /// Pesos para diferentes fuentes de datos
    pub source_weights: HashMap<String, f32>,
    /// Usar scoring adaptivo
    pub adaptive_scoring: bool,
    /// Configuraciones específicas de optimización
    pub optimization_config: HashMap<String, serde_json::Value>,
}

/// Algoritmos de combinación de resultados
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CombinationAlgorithm {
    /// Combinación por peso simple
    WeightedAverage,
    /// Combinación por ranking
    RankFusion,
    /// Combinación por relevancia
    RelevanceScore,
    /// Combinación usando ML
    MachineLearning,
}

impl Default for NavigatorMode {
    fn default() -> Self {
        NavigatorMode::Core {
            primary_strategy: NavigationStrategy::default(),
            config: CoreModeConfig::default(),
        }
    }
}

impl Default for NavigationStrategy {
    fn default() -> Self {
        NavigationStrategy::IndexBased {
            index_type: IndexType::FullTextIndex,
            config: IndexConfig::default(),
        }
    }
}

impl Default for CoreModeConfig {
    fn default() -> Self {
        Self {
            operation_timeout_ms: 5000,
            enable_cache: true,
            max_cache_size: 1000,
            cache_ttl_seconds: 300,
        }
    }
}

impl Default for AdvancedModeConfig {
    fn default() -> Self {
        Self {
            strategy_timeout_ms: 3000,
            max_strategy_attempts: 3,
            combine_results: true,
            strategy_configs: HashMap::new(),
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.7,
            enable_ml: false,
            model_name: "basic_decision_tree".to_string(),
            model_config: HashMap::new(),
        }
    }
}

impl Default for IndexConfig {
    fn default() -> Self {
        Self {
            auto_refresh: true,
            refresh_interval_seconds: 300,
            enable_compression: true,
            type_specific: HashMap::new(),
        }
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            default_limit: 50,
            enable_pagination: true,
            page_size: 20,
            type_specific: HashMap::new(),
        }
    }
}

impl Default for HybridConfig {
    fn default() -> Self {
        let mut source_weights = HashMap::new();
        source_weights.insert("index".to_string(), 0.6);
        source_weights.insert("query".to_string(), 0.4);

        Self {
            combination_algorithm: CombinationAlgorithm::WeightedAverage,
            source_weights,
            adaptive_scoring: true,
            optimization_config: HashMap::new(),
        }
    }
}

impl NavigatorMode {
    /// Crear modo Core básico
    pub fn core() -> Self {
        NavigatorMode::Core {
            primary_strategy: NavigationStrategy::default(),
            config: CoreModeConfig::default(),
        }
    }

    /// Crear modo Advanced con estrategias por defecto
    pub fn advanced() -> Self {
        let strategies = vec![
            NavigationStrategy::HybridOptimized {
                index_ratio: 0.7,
                config: HybridConfig::default(),
            },
            NavigationStrategy::IndexBased {
                index_type: IndexType::FullTextIndex,
                config: IndexConfig::default(),
            },
            NavigationStrategy::QueryBased {
                query_type: QueryType::DirectMongo,
                config: QueryConfig::default(),
            },
        ];

        NavigatorMode::Advanced {
            strategies,
            config: AdvancedModeConfig::default(),
        }
    }

    /// Crear modo AI-Driven básico
    pub fn ai_driven() -> Self {
        let available_strategies = vec![
            NavigationStrategy::HybridOptimized {
                index_ratio: 0.5,
                config: HybridConfig::default(),
            },
            NavigationStrategy::IndexBased {
                index_type: IndexType::SemanticIndex,
                config: IndexConfig::default(),
            },
            NavigationStrategy::QueryBased {
                query_type: QueryType::SemanticQuery,
                config: QueryConfig::default(),
            },
        ];

        NavigatorMode::AIDriven {
            ai_config: AIConfig::default(),
            available_strategies,
        }
    }

    /// Validar configuración del modo
    pub fn validate(&self) -> Result<()> {
        match self {
            NavigatorMode::Core { primary_strategy, config } => {
                if config.operation_timeout_ms == 0 {
                    return Err(NavigatorError::validation("Timeout no puede ser 0"));
                }
                // Validar estrategia primaria
                primary_strategy.validate()?;
            }
            NavigatorMode::Advanced { strategies, config } => {
                if strategies.is_empty() {
                    return Err(NavigatorError::validation("Debe haber al menos una estrategia"));
                }
                if config.max_strategy_attempts == 0 {
                    return Err(NavigatorError::validation("max_strategy_attempts no puede ser 0"));
                }
                // Validar todas las estrategias
                for strategy in strategies {
                    strategy.validate()?;
                }
            }
            NavigatorMode::AIDriven { ai_config, available_strategies } => {
                if available_strategies.is_empty() {
                    return Err(NavigatorError::validation("AI debe tener estrategias disponibles"));
                }
                if ai_config.confidence_threshold < 0.0 || ai_config.confidence_threshold > 1.0 {
                    return Err(NavigatorError::validation("confidence_threshold debe estar entre 0.0 y 1.0"));
                }
                // Validar estrategias disponibles
                for strategy in available_strategies {
                    strategy.validate()?;
                }
            }
        }
        Ok(())
    }

    /// Obtener nombre del modo como string
    pub fn mode_name(&self) -> &'static str {
        match self {
            NavigatorMode::Core { .. } => "Core",
            NavigatorMode::Advanced { .. } => "Advanced",
            NavigatorMode::AIDriven { .. } => "AI-Driven",
        }
    }
}

impl NavigationStrategy {
    /// Validar configuración de la estrategia
    pub fn validate(&self) -> Result<()> {
        match self {
            NavigationStrategy::IndexBased { config, .. } => {
                if config.refresh_interval_seconds == 0 {
                    return Err(NavigatorError::validation("refresh_interval_seconds no puede ser 0"));
                }
            }
            NavigationStrategy::QueryBased { config, .. } => {
                if config.default_limit == 0 {
                    return Err(NavigatorError::validation("default_limit no puede ser 0"));
                }
            }
            NavigationStrategy::HybridOptimized { index_ratio, .. } => {
                if *index_ratio < 0.0 || *index_ratio > 1.0 {
                    return Err(NavigatorError::validation("index_ratio debe estar entre 0.0 y 1.0"));
                }
            }
        }
        Ok(())
    }

    /// Obtener nombre de la estrategia como string
    pub fn strategy_name(&self) -> &'static str {
        match self {
            NavigationStrategy::IndexBased { .. } => "Index-Based",
            NavigationStrategy::QueryBased { .. } => "Query-Based",
            NavigationStrategy::HybridOptimized { .. } => "Hybrid-Optimized",
        }
    }
}
