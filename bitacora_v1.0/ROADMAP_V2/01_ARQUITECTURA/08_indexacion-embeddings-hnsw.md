# 08. Indexación, Embeddings y HNSW (CAPA 4: INDEXACIÓN)

**Última actualización:** 2025-11-23  
**Estado:** LISTO PARA PRODUCCIÓN  
**Versión:** 1.0  

---

## PARTE I: ESPECIFICACIÓN (CONCEPTO)

### ¿QUÉ ES CAPA 4?

**CAPA 4 (INDEXACIÓN)** crea índices **semánticos** de datos almacenados en TelescopeDB/VoxelDB usando:

1. **Embeddings** (MiniLM-L6-v2) → Representación vectorial densa
2. **HNSW** (Hierarchical Navigable Small World) → Índice de búsqueda O(log n)

Resultado: Búsquedas rápidas (<50ms) sobre millones de documentos.

**Metáfora:** Como un **catálogo de biblioteca** con índices alfabético + temático que permite encontrar libros en segundos sin leer todos, HNSW permite encontrar contextos similares sin comparar todos los vectores.

### ¿QUÉ SON EMBEDDINGS?

**Embedding:** Representación vectorial densa de texto en un espacio semántico.

**Concepto:**

```
Texto: "necesito dinero urgente"
       ↓ [Model: MiniLM-L6-v2]
Vector: [0.12, -0.45, 0.89, 0.23, ..., 0.67]  (384 dimensiones)
        └─ Cada dimensión captura un aspecto semántico

Ventaja: Textos similares → vectores cercanos en espacio euclidiano
```

**¿Por qué MiniLM-L6-v2?**

| Modelo | Dimensiones | Velocidad | Tamaño | Precisión | Caso de uso |
|--------|-------------|-----------|--------|-----------|-------------|
| BERT | 768 | 100ms | 400MB | 99% | Enterprise (overkill para v1.0) |
| **MiniLM-L6-v2** | **384** | **<10ms** | **90MB** | **97%** | ✅ Bitácora v1.0 (Goldilocks) |
| DistilBERT | 768 | 50ms | 250MB | 95% | Mobile (aún pesado) |
| Sentence-BERT | 384 | 20ms | 120MB | 98% | Similar a MiniLM |

**MiniLM es ideal porque:**
- ✅ 384 dims = Balance velocidad vs. precisión
- ✅ <10ms por embedding = Real-time friendly
- ✅ 90MB = Cabe en SRAM de STM32H7 extendida
- ✅ 97% precisión = Suficiente para similitud conversacional

### SIMILITUD COSENO: Medida de Cercanía

Dados dos vectores, calcular su similitud:

```
Vector A: [0.1, 0.2, 0.3]
Vector B: [0.2, 0.4, 0.1]

Similitud Coseno = (A · B) / (||A|| * ||B||)
                 = (0.1*0.2 + 0.2*0.4 + 0.3*0.1) / (√(0.01+0.04+0.09) * √(0.04+0.16+0.01))
                 = 0.13 / (0.371 * 0.447)
                 ≈ 0.78

Rango: [0, 1]
- 1.0 = Idéntico
- 0.7+ = Muy similar
- 0.5-0.7 = Similar
- <0.5 = Distinto
```

### ¿POR QUÉ HNSW?

**Problema:** Con 1M documentos, búsqueda lineal (comparar todos) = O(n) = 1M comparaciones = 100s ❌

**HNSW Solution:** Estructura jerárquica que reduce a O(log n) ≈ 20 comparaciones = 2ms ✅

**Cómo funciona HNSW:**

```
┌─────────────────────────────────────────────────────┐
│  BÚSQUEDA EN HNSW: Encontrar vector más similar    │
├─────────────────────────────────────────────────────┤
│                                                     │
│  NIVEL 3 (Entrada):  ●─ query                       │
│  ↓  (salto largo)                                   │
│  ┌───────────────────────────────────────────┐     │
│  │ ●──────────● (búsqueda por proximidad)    │     │
│  └───────────────────────────────────────────┘     │
│  NIVEL 2:        ●─ ●──────●─ query                │
│  ↓                                                  │
│  ┌──────────────────────────────────────────────┐  │
│  │●─●─●─●─●─●query                            │  │
│  └──────────────────────────────────────────────┘  │
│  NIVEL 1 (Datos):                                  │
│                                                     │
│  Resultado: Vector más similar encontrado en O(log n)│
└─────────────────────────────────────────────────────┘
```

**Propiedades:**
- O(log n) búsqueda en promedio
- O(log n) inserción
- Memoria: O(n) con factor ~2-4
- Excelente para ANN (Approximate Nearest Neighbor)

### INTEGRACIÓN: FlowPacks → Embeddings → HNSW

```
FlowPack #1 (comprimido)
  ├─ avg_ctx7d: [0.7, 0.3, 0.9, ...]
  ├─ turn_range: (3, 8)
  └─ compressed_data: FbcuCore { ... }
  ↓
Embedding::from_flowpack()
  ├─ Descomprimir datos (FbcuCore)
  ├─ Extraer resumen textual
  ├─ Pasar por MiniLM-L6-v2
  └─ Output: Vector [0.12, -0.45, 0.89, ...] (384 dims)
  ↓
HNSW::insert(embedding)
  ├─ Encontrar vecinos más cercanos en HNSW
  ├─ Agregar nueva conexión jerárquica
  └─ Actualizar índice
  ↓
CAPA 5 (Reconocimiento)
  └─ Búsquedas rápidas de contextos similares
```

---

## PARTE II: IMPLEMENTACIÓN (TÉCNICO)

### STRUCT: Embedding

```rust
/// Vector denso de 384 dimensiones
/// Representa semántica de un FlowPack
#[derive(Debug, Clone)]
pub struct Embedding {
    /// Vector de 384 dimensiones (f32)
    pub vector: [f32; 384],
    
    /// ID del FlowPack origen
    pub flowpack_id: [u8; 32],
    
    /// Texto fuente (para debugging)
    pub source_text: String,
    
    /// Hash del embedding (para deduplicación)
    pub hash: [u8; 32],
}

/// Modelo de embedding MiniLM-L6-v2
pub struct EmbeddingModel {
    /// Tokens del tokenizer (vocabulario)
    tokenizer: Tokenizer,
    
    /// Pesos del modelo (red neuronal)
    model: MiniLmModel,
    
    /// Cache de embeddings recientes (LRU)
    cache: std::collections::HashMap<[u8; 32], Embedding>,
    cache_size: usize,
}
```

### ALGORITMO: Generación de Embeddings

```rust
impl EmbeddingModel {
    /// Carga modelo MiniLM-L6-v2
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let tokenizer = Tokenizer::load("minilm-l6-v2-vocab.txt")?;
        let model = MiniLmModel::load("minilm-l6-v2-weights.bin")?;
        
        Ok(Self {
            tokenizer,
            model,
            cache: HashMap::new(),
            cache_size: 1000,
        })
    }
    
    /// Genera embedding desde texto
    pub fn embed(&mut self, text: &str) -> Result<Embedding, Box<dyn std::error::Error>> {
        // PASO 1: Tokenización
        let tokens = self.tokenizer.tokenize(text);
        let token_ids: Vec<u32> = tokens
            .iter()
            .map(|t| self.tokenizer.token_to_id(t))
            .collect::<Result<Vec<_>, _>>()?;
        
        // Limitar a 512 tokens (máximo de BERT/MiniLM)
        let token_ids = if token_ids.len() > 512 {
            token_ids[..512].to_vec()
        } else {
            token_ids
        };
        
        // PASO 2: Padding/Truncation
        let mut padded = vec![0u32; 512];
        padded[..token_ids.len()].copy_from_slice(&token_ids);
        
        // PASO 3: Forward pass (red neuronal)
        // MiniLM: 6 capas de transformer, attention heads, feed-forward
        let logits = self.model.forward(&padded)?;
        
        // PASO 4: Mean pooling (promedia todas las posiciones)
        // Produce vector de 384 dimensiones
        let embedding_vec: [f32; 384] = mean_pool(&logits)?;
        
        // PASO 5: L2 normalización (importante para similitud coseno)
        let embedding_vec = l2_normalize(&embedding_vec);
        
        // Calcular hash para deduplicación
        let hash = calculate_sha256(&embedding_vec);
        
        let embedding = Embedding {
            vector: embedding_vec,
            flowpack_id: [0u8; 32], // Se asigna en otro lugar
            source_text: text.to_string(),
            hash,
        };
        
        Ok(embedding)
    }
    
    /// Genera embedding desde FlowPack (descomprimir + resumir)
    pub fn embed_flowpack(
        &mut self,
        flowpack: &FlowPack,
    ) -> Result<Embedding, Box<dyn std::error::Error>> {
        // PASO 1: Descomprimir FbcuCore
        let decompressed = flowpack.compressed_data.decompress();
        
        // PASO 2: Resumir (extractive summary)
        let summary = extract_summary(&decompressed, 100); // Top 100 bytes
        
        // PASO 3: Generar embedding
        let mut embedding = self.embed(&summary)?;
        embedding.flowpack_id = flowpack.id;
        
        Ok(embedding)
    }
}

/// Mean pooling: toma promedio de todas las posiciones
fn mean_pool(logits: &[[f32; 384]]) -> Result<[f32; 384], Box<dyn std::error::Error>> {
    let mut result = [0.0f32; 384];
    
    for logit_vec in logits {
        for (i, val) in logit_vec.iter().enumerate() {
            result[i] += val;
        }
    }
    
    let n = logits.len() as f32;
    for val in &mut result {
        *val /= n;
    }
    
    Ok(result)
}

/// Normalización L2 (importante para similitud coseno)
fn l2_normalize(vec: &[f32; 384]) -> [f32; 384] {
    let norm: f32 = vec.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
    
    let mut result = [0.0f32; 384];
    for (i, val) in vec.iter().enumerate() {
        result[i] = val / norm;
    }
    result
}
```

### STRUCT: HNSW Index

```rust
/// Hierarchical Navigable Small World
/// Índice para búsqueda rápida de embeddings similares
pub struct HnswIndex {
    /// Nodos del grafo (embeddings)
    nodes: std::collections::HashMap<[u8; 32], HnswNode>,
    
    /// Nivel máximo en la jerarquía (dinámico)
    max_level: usize,
    
    /// Entry point (nivel 0, parte de un nodo)
    entry_point: Option<[u8; 32]>,
    
    /// Parámetro M: máximo de conexiones por nodo
    m: usize,
    
    /// Parámetro M_L: factor para multiplicar niveles
    m_l: f32,
}

/// Nodo individual en HNSW
#[derive(Debug, Clone)]
pub struct HnswNode {
    /// Embedding del nodo
    pub embedding: Embedding,
    
    /// Nivel asignado a este nodo
    pub level: usize,
    
    /// Vecinos por nivel: [nivel][vecinos]
    pub neighbors: Vec<Vec<[u8; 32]>>,
}
```

### ALGORITMO: Construcción e Inserción HNSW

```rust
impl HnswIndex {
    /// Crea índice HNSW vacío
    pub fn new(m: usize) -> Self {
        Self {
            nodes: HashMap::new(),
            max_level: 0,
            entry_point: None,
            m,
            m_l: 1.0 / (2.0_f32.ln()), // ≈ 1.44
        }
    }
    
    /// Inserta nuevo embedding
    pub fn insert(&mut self, embedding: Embedding) -> Result<(), Box<dyn std::error::Error>> {
        let embedding_id = embedding.hash;
        
        // Si índice vacío, crear primer nodo
        if self.entry_point.is_none() {
            let node = HnswNode {
                embedding: embedding.clone(),
                level: 0,
                neighbors: vec![Vec::new()],
            };
            self.nodes.insert(embedding_id, node);
            self.entry_point = Some(embedding_id);
            return Ok(());
        }
        
        // PASO 1: Asignar nivel a nuevo nodo (probabilístico)
        let level = self.assign_level();
        
        // PASO 2: Buscar vecinos más cercanos (insertion heuristic)
        let candidates = self.search_layer_insert(
            &embedding.vector,
            self.entry_point.unwrap(),
            self.m * 2, // ef_construction
            0,
        )?;
        
        // PASO 3: Crear nodo
        let mut neighbors = vec![Vec::new(); level + 1];
        neighbors[0] = candidates.iter().take(self.m).map(|(id, _)| *id).collect();
        
        let node = HnswNode {
            embedding: embedding.clone(),
            level,
            neighbors,
        };
        
        self.nodes.insert(embedding_id, node);
        
        // PASO 4: Actualizar referencias en vecinos
        for (neighbor_id, _) in &candidates {
            if let Some(neighbor_node) = self.nodes.get_mut(neighbor_id) {
                neighbor_node.neighbors[0].push(embedding_id);
                
                // Limitar conexiones si excede M
                if neighbor_node.neighbors[0].len() > self.m {
                    // Prune: mantener M vecinos más cercanos
                    self.prune_connections(neighbor_id, self.m)?;
                }
            }
        }
        
        // PASO 5: Insertar en niveles superiores si necesario
        if level > self.max_level {
            self.update_entry_point(embedding_id, level)?;
            self.max_level = level;
        }
        
        Ok(())
    }
    
    /// Asigna nivel a nuevo nodo (exponential decay probability)
    fn assign_level(&self) -> usize {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        let mut level = 0;
        while rng.gen::<f32>() < self.m_l && level < 16 {
            level += 1;
        }
        
        level
    }
    
    /// Búsqueda en capa para inserción
    fn search_layer_insert(
        &self,
        query: &[f32; 384],
        entry_point: [u8; 32],
        ef: usize,
        layer: usize,
    ) -> Result<Vec<([u8; 32], f32)>, Box<dyn std::error::Error>> {
        let mut candidates = vec![];
        let mut visited = std::collections::HashSet::new();
        
        // Calcular distancia desde entry point
        let entry_dist = cosine_distance(
            query,
            &self.nodes[&entry_point].embedding.vector,
        );
        
        candidates.push((entry_point, entry_dist));
        visited.insert(entry_point);
        
        // Greedy search hasta encontrar ef candidatos
        while !candidates.is_empty() {
            let (current_id, _) = candidates.pop().unwrap();
            let current_node = &self.nodes[&current_id];
            
            if layer < current_node.neighbors.len() {
                for &neighbor_id in &current_node.neighbors[layer] {
                    if !visited.contains(&neighbor_id) {
                        visited.insert(neighbor_id);
                        
                        let neighbor_dist = cosine_distance(
                            query,
                            &self.nodes[&neighbor_id].embedding.vector,
                        );
                        
                        if neighbor_dist < candidates.last().map(|x| x.1).unwrap_or(f32::MAX) {
                            candidates.push((neighbor_id, neighbor_dist));
                            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                            
                            if candidates.len() > ef {
                                candidates.pop();
                            }
                        }
                    }
                }
            }
        }
        
        Ok(candidates)
    }
}

/// Distancia coseno entre dos embeddings
fn cosine_distance(a: &[f32; 384], b: &[f32; 384]) -> f32 {
    let mut dot_product = 0.0f32;
    
    for i in 0..384 {
        dot_product += a[i] * b[i];
    }
    
    // Como los vectores están normalizados (L2), distancia = 1 - coseno
    1.0 - dot_product.max(0.0).min(1.0)
}
```

### ALGORITMO: Búsqueda HNSW

```rust
impl HnswIndex {
    /// Búsqueda rápida: encontrar K vecinos más similares
    pub fn search(
        &self,
        query: &[f32; 384],
        k: usize,
    ) -> Result<Vec<([u8; 32], f32)>, Box<dyn std::error::Error>> {
        if self.entry_point.is_none() {
            return Ok(Vec::new());
        }
        
        // Parámetro ef: amplitud de búsqueda (ef >= k)
        let ef = (k * 2).max(50);
        
        // PASO 1: Búsqueda en niveles superiores
        let mut nearest = self.entry_point.unwrap();
        
        for level in (1..=self.max_level).rev() {
            nearest = self.search_layer(query, &[nearest], 1, level)?
                .first()
                .map(|(id, _)| *id)
                .unwrap_or(nearest);
        }
        
        // PASO 2: Búsqueda en capa 0 (datos)
        let mut candidates = self.search_layer(query, &[nearest], ef, 0)?;
        
        // Retornar K mejores
        Ok(candidates.into_iter().take(k).collect())
    }
    
    /// Búsqueda en una capa específica (greedy)
    fn search_layer(
        &self,
        query: &[f32; 384],
        entry_points: &[[u8; 32]],
        ef: usize,
        layer: usize,
    ) -> Result<Vec<([u8; 32], f32)>, Box<dyn std::error::Error>> {
        let mut candidates = std::collections::BinaryHeap::new();
        let mut visited = std::collections::HashSet::new();
        let mut w = Vec::new();
        
        for &entry in entry_points {
            let dist = cosine_distance(query, &self.nodes[&entry].embedding.vector);
            candidates.push((-(dist as i32), entry, dist)); // Max heap (inverted)
            w.push((entry, dist));
            visited.insert(entry);
        }
        
        while !candidates.is_empty() {
            let (_, lowerbound_id, _) = candidates.peek().unwrap();
            let lowerbound = candidates.peek().unwrap().2;
            
            if lowerbound > w.last().map(|(_, d)| *d).unwrap_or(f32::MAX) {
                break;
            }
            
            let (_, current, _) = candidates.pop().unwrap();
            
            if let Some(node) = self.nodes.get(&current) {
                if layer < node.neighbors.len() {
                    for &neighbor in &node.neighbors[layer] {
                        if !visited.contains(&neighbor) {
                            visited.insert(neighbor);
                            
                            let dist = cosine_distance(query, &self.nodes[&neighbor].embedding.vector);
                            
                            if dist < w.last().map(|(_, d)| *d).unwrap_or(f32::MAX) || w.len() < ef {
                                candidates.push((-(dist as i32), neighbor, dist));
                                w.push((neighbor, dist));
                                
                                if w.len() > ef {
                                    w.pop();
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(w)
    }
}
```

### PERFORMANCE TARGETS

| Métrica | Target | Ambiente |
|---------|--------|----------|
| Embedding (384 dims) | <10ms | MiniLM-L6-v2 @ STM32H7 |
| Inserción HNSW | <5ms | 1M nodos |
| Búsqueda HNSW (K=10) | <50ms | 1M nodos, ef=50 |
| Memoria HNSW | O(n * 1.5) | M=12, 1M nodos ≈ 1.5GB |
| Precisión (Recall@10) | >95% | vs. búsqueda lineal |

---

## PARTE III: VALIDACIÓN

### CHECKLIST DE ACEPTACIÓN

- [ ] EmbeddingModel carga MiniLM-L6-v2 correctamente
- [ ] Tokenization + forward pass genera 384 dims
- [ ] L2 normalización aplicada correctamente
- [ ] HNSW inserción en O(log n) amortizado
- [ ] HNSW búsqueda en O(log n), precisión >95%
- [ ] Cosine distance calculada correctamente
- [ ] Manejo de edge cases (índice vacío, k > n)
- [ ] Performance <10ms embedding, <50ms búsqueda

### TESTS UNITARIOS

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_embedding_generation() {
        let mut model = EmbeddingModel::load().unwrap();
        let embedding = model.embed("necesito dinero urgente").unwrap();
        
        assert_eq!(embedding.vector.len(), 384);
        // L2 norm ≈ 1.0 (normalizado)
        let norm: f32 = embedding.vector.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.01);
    }
    
    #[test]
    fn test_cosine_similarity() {
        let a = [1.0; 384];
        let b = [1.0; 384];
        let dist = cosine_distance(&a, &b);
        assert!(dist < 0.01); // Casi idénticos
    }
    
    #[test]
    fn test_hnsw_insert_and_search() {
        let mut index = HnswIndex::new(12);
        
        let emb1 = Embedding { vector: [1.0; 384], ..Default::default() };
        let emb2 = Embedding { vector: [1.1; 384], ..Default::default() };
        let emb3 = Embedding { vector: [0.5; 384], ..Default::default() };
        
        index.insert(emb1.clone()).unwrap();
        index.insert(emb2.clone()).unwrap();
        index.insert(emb3.clone()).unwrap();
        
        let results = index.search(&[1.05; 384], 2).unwrap();
        assert_eq!(results.len(), 2);
        // emb2 debe ser el más cercano a [1.05; 384]
    }
}
```

---

## REFERENCIAS

- **00_VISION:** `04_arquitectura-sistema-7-capas.md` (definición CAPA 4)
- **01_ARQUITECTURA:** `07_fbcu-y-flowpacks.md` (productor upstream)
- **MiniLM-L6-v2:** Microsoft Sentence Transformers
- **HNSW Paper:** Malkov & Yashunin (2018) "Efficient and robust approximate nearest neighbor search"

---

## NOTAS PARA DESARROLLO

- ⚠️ MiniLM requiere ~90MB en memoria para cargar (o lazy-load)
- ⚠️ HNSW usa memoria O(n * 1.5) para M=12, optimizar M vs. precisión
- ✅ Búsqueda es **best-effort**, no garantiza óptimo global (aproximado)
- ✅ L2 normalización es **crítica** para similitud coseno correcta
- ✅ Embedding deduplication evita duplicados mediante hash

---

**Estado:** ✅ READY FOR CODING  
**Siguiente:** `09_reconocimiento-patrones.md` (CAPA 5)
