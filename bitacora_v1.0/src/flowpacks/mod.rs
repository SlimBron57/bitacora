// FLOWPACKS :: MAIN MODULE
// Orquestador de compresión contextual anti-disco-rayado
// SPDX-License-Identifier: PROPRIETARY
// Copyright (c) 2024 Eduardo Gil
//! # FlowPacks - Compresión Contextual
//!
//! Sistema de detección de repeticiones y generación de respuestas adaptativas
//! para evitar el "disco rayado" en conversaciones multi-agente.
//!
//! ## Arquitectura
//!
//! - **FlowPack**: Contenedor temporal de mensajes relacionados (~20 msgs, 72h window)
//! - **FBCU Compression**: Compresión baseline 15x usando fractal-bayesiana
//! - **HNSW Index**: Búsqueda de similitud O(log n) con MiniLM-L6-v2
//! - **Adaptive Responses**: 3 niveles (Reference, PartialReference, Full)
//!
//! ## Flujo de Operación
//!
//! ```text
//! Mensaje → Embedding → HNSW Search → Similitud?
//!                                        ├─ >0.95: Reference (link)
//!                                        ├─ 0.85-0.95: PartialReference (recap)
//!                                        └─ <0.85: Full (nueva explicación)
//! ```
//!
//! ## Targets de Performance
//!
//! - Compression ratio: >20x (FBCU 15x + contextual 1.5-3x)
//! - Search latency: <50ms (HNSW k-NN)
//! - Exact detection: >95% accuracy (similarity threshold 0.95)
//! - Temporal window: 72 hours default (configurable)
//!
//! ## Ejemplo de Uso
//!
//! ```rust,ignore
//! use flowpacks::{FlowPackEngine, FlowPackConfig};
//!
//! let config = FlowPackConfig::default();
//! let mut engine = FlowPackEngine::new(config)?;
//!
//! // Agregar mensaje
//! let msg = "¿Cómo funciona el FBCU?";
//! let response = engine.add_message(msg)?;
//!
//! match response {
//!     AdaptiveResponse::Reference { link_text, .. } => {
//!         println!("Ya te expliqué eso: {}", link_text);
//!     }
//!     AdaptiveResponse::Full { full_response } => {
//!         println!("Nueva explicación: {}", full_response);
//!     }
//!     _ => {}
//! }
//! ```

mod error;
mod config;
mod flowpack;
mod similarity;
mod response;
mod compression;

pub use error::{FlowPackError, Result};
pub use config::FlowPackConfig;
pub use flowpack::{FlowPack, FlowPackEntry, EntryType};
pub use similarity::SimilarityIndex;
pub use response::{AdaptiveResponse, ResponseLevel};
pub use compression::{
    CompressionEngine, CompressionStrategy, CompressionResult,
    FBCUCompressor, DeltaCompressor,
};

use lru::LruCache;
use std::sync::{Arc, RwLock};
use std::num::NonZeroUsize;
use uuid::Uuid;

/// Motor principal de FlowPacks
///
/// Coordina todos los componentes: similarity search, compression, adaptive responses
pub struct FlowPackEngine {
    /// Configuración del sistema
    config: FlowPackConfig,
    
    /// Índice de similitud HNSW
    similarity_index: Arc<RwLock<SimilarityIndex>>,
    
    /// Motor de compresión
    compression_engine: Arc<CompressionEngine>,
    

    
    /// Cache LRU de FlowPacks activos
    active_packs: Arc<RwLock<LruCache<Uuid, FlowPack>>>,
    
    /// FlowPack actual (en construcción)
    current_pack: Arc<RwLock<Option<FlowPack>>>,
}

impl FlowPackEngine {
    /// Crea un nuevo motor de FlowPacks
    pub fn new(config: FlowPackConfig) -> Result<Self> {
        config.validate()
            .map_err(|e| FlowPackError::CompressionFailed(format!("Config validation: {}", e)))?;
        
        let similarity_index = Arc::new(RwLock::new(
            SimilarityIndex::new(config.clone())?
        ));
        
        let compression_engine = Arc::new(
            CompressionEngine::new(config.clone())?
        );
        
        let cache_size = NonZeroUsize::new(config.cache_size)
            .ok_or_else(|| FlowPackError::CompressionFailed(
                "cache_size must be > 0".to_string()
            ))?;
        
        let active_packs = Arc::new(RwLock::new(
            LruCache::new(cache_size)
        ));
        
        let current_pack = Arc::new(RwLock::new(None)); // Empieza vacío
        
        Ok(Self {
            config,
            similarity_index,
            compression_engine,
            active_packs,
            current_pack,
        })
    }

    /// Agrega un mensaje y genera respuesta adaptativa
    ///
    /// # Flujo
    /// 1. Generar embedding del mensaje
    /// 2. Buscar FlowPacks similares (HNSW k-NN)
    /// 3. Decidir tipo de respuesta según similitud
    /// 4. Agregar mensaje al FlowPack actual
    /// 5. Comprimir y rotar si es necesario
    pub fn add_message(&mut self, text: &str) -> Result<AdaptiveResponse> {
        // 1. Generar embedding
        let embedding = {
            let index = self.similarity_index.read()
                .map_err(|e| FlowPackError::SimilaritySearchFailed(e.to_string()))?;
            index.generate_embedding(text)?
        };

        // 2. Buscar similares
        let similar_packs = {
            let index = self.similarity_index.read()
                .map_err(|e| FlowPackError::SimilaritySearchFailed(e.to_string()))?;
            index.search_similar(text, self.config.hnsw_k)?
        };

        // 3. Generar respuesta adaptativa
        let response = AdaptiveResponse::generate(text, &similar_packs, &self.config);

        // 4. Agregar al FlowPack actual
        let entry = FlowPackEntry::new(
            text.to_string(),
            embedding,
            match response.level {
                crate::flowpacks::response::ResponseLevel::Reference => EntryType::Reference,
                crate::flowpacks::response::ResponseLevel::PartialReference => EntryType::Summary,
                crate::flowpacks::response::ResponseLevel::Full => EntryType::FullMessage,
            },
            None, // session_id
        );

        let should_rotate = {
            let mut current = self.current_pack.write()
                .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
            
            if let Some(pack) = current.as_mut() {
                pack.add_entry(entry.clone());
                pack.entries.len() >= self.config.max_pack_size
            } else {
                // Crear primer pack
                *current = Some(FlowPack::new(entry));
                false
            }
        };

        // 4. Rotar si el pack está lleno
        if should_rotate {
            self.rotate_pack()?;
        }

        Ok(response)
    }

    /// Busca FlowPacks similares a un query
    pub fn find_similar(&self, query: &str, k: usize) -> Result<Vec<(FlowPack, f32)>> {
        let index = self.similarity_index.read()
            .map_err(|e| FlowPackError::SimilaritySearchFailed(e.to_string()))?;
        index.search_similar(query, k)
    }

    /// Obtiene un FlowPack por ID
    pub fn get_flowpack(&self, id: Uuid) -> Result<Option<FlowPack>> {
        let cache = self.active_packs.read()
            .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
        Ok(cache.peek(&id).cloned())
    }

    /// Comprime un FlowPack manualmente
    pub fn compress_flowpack(&self, flowpack: &mut FlowPack) -> Result<CompressionResult> {
        self.compression_engine.compress_flowpack(flowpack)
    }

    /// Descomprime un FlowPack
    pub fn decompress_flowpack(&self, flowpack: &FlowPack) -> Result<Vec<String>> {
        self.compression_engine.decompress_flowpack(flowpack)
    }

    /// Rota el FlowPack actual
    ///
    /// 1. Comprimir pack completo
    /// 2. Indexar en HNSW
    /// 3. Mover a cache LRU
    /// 4. Crear nuevo pack vacío
    fn rotate_pack(&mut self) -> Result<()> {
        let mut current = self.current_pack.write()
            .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
        
        if let Some(mut pack) = current.take() {
            // Comprimir
            let result = self.compression_engine.compress_flowpack(&mut pack)?;
            
            // Verificar ratio target
            if !result.meets_target(20.0) && self.config.aggressive_compression {
                eprintln!(
                    "Warning: Compression ratio {:.2}x below target 20x",
                    result.compression_ratio
                );
            }

            // Indexar
            {
                let mut index = self.similarity_index.write()
                    .map_err(|e| FlowPackError::SimilaritySearchFailed(e.to_string()))?;
                index.insert(pack.clone())?;
            }

            // Mover a cache
            {
                let mut cache = self.active_packs.write()
                    .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
                cache.put(pack.id, pack);
            }

            // Nuevo pack (vacío, se crea cuando llegue el siguiente mensaje)
            *current = None;
        }

        Ok(())
    }

    /// Vacuum: elimina FlowPacks expirados del temporal window
    pub fn vacuum(&mut self) -> Result<usize> {
        let mut removed = 0;

        let expired_ids: Vec<Uuid> = {
            let cache = self.active_packs.read()
                .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
            
            cache.iter()
                .filter(|(_, pack)| !pack.is_within_temporal_window(self.config.temporal_window_hours))
                .map(|(id, _)| *id)
                .collect()
        };

        {
            let mut cache = self.active_packs.write()
                .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
            
            for id in expired_ids {
                cache.pop(&id);
                removed += 1;
            }
        }

        Ok(removed)
    }

    /// Fuerza la rotación del pack actual
    pub fn force_rotate(&mut self) -> Result<()> {
        self.rotate_pack()
    }

    /// Obtiene estadísticas del motor
    pub fn stats(&self) -> Result<FlowPackStats> {
        let cache = self.active_packs.read()
            .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
        
        let current = self.current_pack.read()
            .map_err(|e| FlowPackError::PackNotFound(e.to_string()))?;
        
        let current_entries = current.as_ref()
            .map(|p| p.entries.len())
            .unwrap_or(0);
        
        let total_packs = cache.len();
        let total_entries: usize = cache.iter()
            .map(|(_, pack)| pack.entries.len())
            .sum();
        
        let total_original_size: usize = cache.iter()
            .flat_map(|(_, pack)| &pack.entries)
            .map(|e| e.original_size)
            .sum();
        
        let total_compressed_size: usize = cache.iter()
            .flat_map(|(_, pack)| &pack.entries)
            .filter_map(|e| e.compressed_size)
            .sum();
        
        let avg_compression_ratio = if total_compressed_size > 0 {
            total_original_size as f64 / total_compressed_size as f64
        } else {
            0.0
        };

        Ok(FlowPackStats {
            total_packs,
            total_entries,
            current_pack_entries: current_entries,
            total_original_size,
            total_compressed_size,
            avg_compression_ratio,
            cache_capacity: self.config.cache_size,
        })
    }

    /// Obtiene la configuración actual
    pub fn config(&self) -> &FlowPackConfig {
        &self.config
    }
}

/// Estadísticas del motor FlowPacks
#[derive(Debug, Clone)]
pub struct FlowPackStats {
    /// Total de FlowPacks en cache
    pub total_packs: usize,
    /// Total de entradas en todos los packs
    pub total_entries: usize,
    /// Entradas en el pack actual
    pub current_pack_entries: usize,
    /// Tamaño original total (bytes)
    pub total_original_size: usize,
    /// Tamaño comprimido total (bytes)
    pub total_compressed_size: usize,
    /// Ratio de compresión promedio
    pub avg_compression_ratio: f64,
    /// Capacidad de la cache
    pub cache_capacity: usize,
}

impl FlowPackStats {
    /// Calcula el uso de cache en porcentaje
    pub fn cache_usage(&self) -> f64 {
        if self.cache_capacity > 0 {
            (self.total_packs as f64 / self.cache_capacity as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Verifica si cumple targets de performance
    pub fn meets_targets(&self) -> bool {
        self.avg_compression_ratio >= 20.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let config = FlowPackConfig::default();
        let engine = FlowPackEngine::new(config);
        assert!(engine.is_ok());
    }

    #[test]
    fn test_add_message() {
        let config = FlowPackConfig::default();
        let mut engine = FlowPackEngine::new(config).unwrap();
        
        let msg = "¿Cómo funciona el FBCU?";
        let response = engine.add_message(msg);
        
        assert!(response.is_ok());
        
        // Primera vez debe ser Full (no hay similares)
        let response = response.unwrap();
        assert_eq!(response.level, ResponseLevel::Full);
        assert_eq!(response.tokens_saved, 0);
    }

    #[test]
    fn test_stats() {
        let config = FlowPackConfig::default();
        let engine = FlowPackEngine::new(config).unwrap();
        let stats = engine.stats().unwrap();
        
        assert_eq!(stats.total_packs, 0);
        assert_eq!(stats.current_pack_entries, 0);
        assert_eq!(stats.cache_capacity, 100);
    }

    #[test]
    fn test_vacuum() {
        let config = FlowPackConfig::default();
        let mut engine = FlowPackEngine::new(config).unwrap();
        
        // Agregar algunos mensajes
        for i in 0..5 {
            let msg = format!("Mensaje de prueba {}", i);
            engine.add_message(&msg).unwrap();
        }
        
        // Vacuum no debería eliminar nada reciente
        let removed = engine.vacuum().unwrap();
        assert_eq!(removed, 0);
    }

    #[test]
    fn test_force_rotate() {
        let config = FlowPackConfig::default();
        let mut engine = FlowPackEngine::new(config).unwrap();
        
        // Agregar mensaje
        engine.add_message("Test message").unwrap();
        
        // Forzar rotación
        let result = engine.force_rotate();
        assert!(result.is_ok());
        
        // Stats debería mostrar 1 pack
        let stats = engine.stats().unwrap();
        assert_eq!(stats.total_packs, 1);
        assert_eq!(stats.current_pack_entries, 0);
    }

    #[test]
    fn test_stats_cache_usage() {
        let stats = FlowPackStats {
            total_packs: 50,
            total_entries: 1000,
            current_pack_entries: 10,
            total_original_size: 100000,
            total_compressed_size: 5000,
            avg_compression_ratio: 20.0,
            cache_capacity: 100,
        };
        
        assert_eq!(stats.cache_usage(), 50.0);
        assert!(stats.meets_targets());
    }
}
