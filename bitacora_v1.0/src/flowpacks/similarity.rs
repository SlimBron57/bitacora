// src/flowpacks/similarity.rs
// Similarity Index with HNSW + MiniLM-L6-v2 Embeddings
//
// SimilarityIndex: Índice de búsqueda de vecinos más cercanos (k-NN)
// Usa HNSW (Hierarchical Navigable Small World) para O(log n) search
//
// Design: MiniLM-L6-v2 (384 dims) + HNSW (m=16, ef_construction=200)
//
// TODO: Agregar a Cargo.toml:
//   rust-bert = "0.21"  # Para MiniLM-L6-v2 embeddings
//   hnsw = "0.11"       # Para HNSW index
//   tokenizers = "0.13" # Para tokenization

use std::collections::HashMap;
use uuid::Uuid;
use crate::flowpacks::{
    error::{FlowPackError, Result},
    config::FlowPackConfig,
    flowpack::FlowPack,
};

/// Índice de similitud con HNSW
/// 
/// HNSW (Hierarchical Navigable Small World) es un algoritmo de approximate
/// nearest neighbor search con complejidad O(log n).
///
/// Ventajas vs otros índices:
/// - FAISS: HNSW es Rust-native (no FFI overhead)
/// - Linear scan: O(log n) vs O(n)
/// - Ball tree: Mejor para dims altas (384)
pub struct SimilarityIndex {
    /// Mapa de UUID → FlowPack
    flowpacks: HashMap<Uuid, FlowPack>,
    
    /// Índice HNSW (implementación placeholder)
    /// TODO: Reemplazar con hnsw crate cuando se agregue dependency
    hnsw_index: HnswIndexPlaceholder,
    
    /// Modelo de embeddings (MiniLM-L6-v2)
    /// TODO: Reemplazar con rust-bert cuando se agregue dependency
    embedding_model: EmbeddingModelPlaceholder,
    
    /// Configuración
    config: FlowPackConfig,
}

// ============================================================================
// PLACEHOLDERS (temporal hasta agregar dependencies)
// ============================================================================

/// Placeholder para HNSW index
/// TODO: Reemplazar con `hnsw::Hnsw<f32, DistanceCosine>` del crate hnsw
struct HnswIndexPlaceholder {
    // Simulación simple con linear search (O(n))
    // SOLO PARA DESARROLLO - reemplazar con HNSW real
    vectors: Vec<(Uuid, Vec<f32>)>,
}

impl HnswIndexPlaceholder {
    fn new(_m: usize, _ef_construction: usize, _max_elements: usize) -> Self {
        Self {
            vectors: Vec::new(),
        }
    }
    
    fn insert(&mut self, id: Uuid, embedding: Vec<f32>) {
        self.vectors.push((id, embedding));
    }
    
    fn search(&self, query: &[f32], k: usize, _ef_search: usize) -> Vec<(Uuid, f32)> {
        // Linear search con cosine similarity (PLACEHOLDER)
        let mut results: Vec<(Uuid, f32)> = self.vectors.iter()
            .map(|(id, vec)| {
                let similarity = cosine_similarity(query, vec);
                (*id, similarity)
            })
            .collect();
        
        // Ordenar por similitud descendente
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(k);
        results
    }
}

/// Placeholder para modelo de embeddings
/// TODO: Reemplazar con rust-bert SentenceEmbeddingsModel
struct EmbeddingModelPlaceholder {
    dimension: usize,
}

impl EmbeddingModelPlaceholder {
    fn new(dimension: usize) -> Self {
        Self { dimension }
    }
    
    fn encode(&self, _text: &str) -> Vec<f32> {
        // PLACEHOLDER: Retorna vector aleatorio normalizado
        // TODO: Reemplazar con MiniLM-L6-v2 real
        vec![0.5; self.dimension]
    }
}

// ============================================================================
// SIMILARITY INDEX IMPLEMENTATION
// ============================================================================

impl SimilarityIndex {
    /// Crear índice nuevo
    pub fn new(config: FlowPackConfig) -> Result<Self> {
        // Validar configuración
        config.validate()
            .map_err(|e| FlowPackError::CompressionFailed(format!("Config validation: {}", e)))?;
        
        // Crear índice HNSW
        let hnsw_index = HnswIndexPlaceholder::new(
            config.hnsw_m,
            config.hnsw_ef_construction,
            config.cache_size * 10, // max_elements = cache_size * 10
        );
        
        // Cargar modelo de embeddings
        let embedding_model = EmbeddingModelPlaceholder::new(
            config.embedding_dimension
        );
        
        Ok(Self {
            flowpacks: HashMap::new(),
            hnsw_index,
            embedding_model,
            config,
        })
    }
    
    /// Insertar FlowPack en el índice
    pub fn insert(&mut self, flowpack: FlowPack) -> Result<()> {
        let id = flowpack.id;
        let embedding = flowpack.centroid_embedding.clone();
        
        // Insertar en HNSW index
        self.hnsw_index.insert(id, embedding);
        
        // Guardar FlowPack
        self.flowpacks.insert(id, flowpack);
        
        Ok(())
    }
    
    /// Buscar FlowPacks similares a un query
    /// 
    /// Retorna: Vec<(FlowPack, similarity_score)> ordenado por similitud
    pub fn search_similar(
        &self,
        query_text: &str,
        k: usize,
    ) -> Result<Vec<(FlowPack, f32)>> {
        // 1. Generar embedding del query
        let query_embedding = self.embedding_model.encode(query_text);
        
        // 2. Buscar k vecinos más cercanos en HNSW
        let neighbors = self.hnsw_index.search(
            &query_embedding,
            k,
            self.config.hnsw_ef_search,
        );
        
        // 3. Aplicar temporal decay y thresholds
        let mut results = Vec::new();
        
        for (id, base_similarity) in neighbors {
            // Obtener FlowPack
            let flowpack = self.flowpacks.get(&id)
                .ok_or_else(|| FlowPackError::PackNotFound(id.to_string()))?;
            
            // Aplicar temporal decay
            let decay_factor = flowpack.temporal_decay_factor(
                self.config.temporal_window_hours as f32
            );
            let adjusted_similarity = base_similarity * decay_factor;
            
            // Filtrar por threshold
            if adjusted_similarity >= self.config.similarity_threshold as f32 {
                results.push((flowpack.clone(), adjusted_similarity));
            }
        }
        
        // Ordenar por similitud ajustada
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(results)
    }
    
    /// Generar embedding de texto
    pub fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        Ok(self.embedding_model.encode(text))
    }
    
    /// Obtener FlowPack por ID
    pub fn get(&self, id: &Uuid) -> Option<&FlowPack> {
        self.flowpacks.get(id)
    }
    
    /// Obtener FlowPack mutable por ID
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut FlowPack> {
        self.flowpacks.get_mut(id)
    }
    
    /// Número de FlowPacks en el índice
    pub fn len(&self) -> usize {
        self.flowpacks.len()
    }
    
    /// Verificar si índice está vacío
    pub fn is_empty(&self) -> bool {
        self.flowpacks.is_empty()
    }
    
    /// Limpiar FlowPacks fuera de ventana temporal
    pub fn prune_old_flowpacks(&mut self) -> usize {
        let window_hours = self.config.temporal_window_hours;
        
        let old_ids: Vec<Uuid> = self.flowpacks.iter()
            .filter(|(_, fp)| !fp.is_within_temporal_window(window_hours))
            .map(|(id, _)| *id)
            .collect();
        
        let count = old_ids.len();
        
        for id in old_ids {
            self.flowpacks.remove(&id);
            // TODO: También remover del HNSW index cuando usemos crate real
        }
        
        count
    }
}

// ============================================================================
// SIMILARITY METRICS
// ============================================================================

/// Cosine similarity entre dos vectores
/// 
/// Formula: cos(θ) = (A · B) / (||A|| × ||B||)
/// Rango: [-1, 1] (normalmente [0, 1] para embeddings)
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimension");
    
    let dot_product: f32 = a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum();
    
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    
    dot_product / (norm_a * norm_b)
}

/// Euclidean distance entre dos vectores
/// 
/// Formula: d = sqrt(Σ(ai - bi)²)
/// Rango: [0, ∞)
#[allow(dead_code)]
pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimension");
    
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flowpacks::flowpack::{FlowPackEntry, EntryType};
    
    fn create_test_flowpack() -> FlowPack {
        let entry = FlowPackEntry::new(
            "Test message".to_string(),
            vec![0.5; 384],
            EntryType::FullMessage,
            None,
        );
        FlowPack::new(entry)
    }
    
    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 0.001);
    }
    
    #[test]
    fn test_cosine_similarity_opposite() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim + 1.0).abs() < 0.001);
    }
    
    #[test]
    fn test_index_creation() {
        let config = FlowPackConfig::default();
        let index = SimilarityIndex::new(config);
        assert!(index.is_ok());
    }
    
    #[test]
    fn test_insert_flowpack() {
        let config = FlowPackConfig::default();
        let mut index = SimilarityIndex::new(config).unwrap();
        
        let fp = create_test_flowpack();
        let result = index.insert(fp);
        
        assert!(result.is_ok());
        assert_eq!(index.len(), 1);
    }
    
    #[test]
    fn test_search_similar() {
        let config = FlowPackConfig::default();
        let mut index = SimilarityIndex::new(config).unwrap();
        
        // Insertar FlowPack
        let fp = create_test_flowpack();
        index.insert(fp).unwrap();
        
        // Buscar similar
        let results = index.search_similar("Test query", 5).unwrap();
        
        // Con placeholder, debería encontrar el FlowPack insertado
        assert!(!results.is_empty());
    }
    
    #[test]
    fn test_prune_old() {
        let mut config = FlowPackConfig::default();
        config.temporal_window_hours = 0; // Ventana de 0 horas (todo es viejo)
        
        let mut index = SimilarityIndex::new(config).unwrap();
        
        // Insertar FlowPack
        let fp = create_test_flowpack();
        index.insert(fp).unwrap();
        
        assert_eq!(index.len(), 1);
        
        // Pruning debería eliminar FlowPack (fuera de ventana)
        let pruned = index.prune_old_flowpacks();
        assert_eq!(pruned, 1);
        assert_eq!(index.len(), 0);
    }
    
    #[test]
    fn test_euclidean_distance() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let dist = euclidean_distance(&a, &b);
        assert!((dist - 2.0_f32.sqrt()).abs() < 0.001);
    }
}
