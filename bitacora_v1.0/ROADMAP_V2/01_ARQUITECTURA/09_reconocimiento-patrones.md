# 09. Reconocimiento de Patrones (CAPA 5: RECONOCIMIENTO)

**Última actualización:** 2025-11-23  
**Estado:** LISTO PARA PRODUCCIÓN  
**Versión:** 1.0  

---

## PARTE I: ESPECIFICACIÓN (CONCEPTO)

### ¿QUÉ ES CAPA 5?

**CAPA 5 (RECONOCIMIENTO)** analiza **patrones** en conversaciones para:

1. **Detectar similitud semántica** entre contextos (via coseno sobre embeddings)
2. **Construir topología de conversación** (grafo de relaciones contextuales)
3. **Identificar ciclos/loops** (evitar "disco rayado")
4. **Mapear progresión emocional** (evolución del sentimiento)

Resultado: Comprensión de **estructura y evolución** de la conversación.

**Metáfora:** Como un **analista de novelas** que identifica patrones narrativos (clímax, anticlimax, círculos viciosos), CAPA 5 reconoce patrones conversacionales.

### SIMILITUD SEMÁNTICA: Coseno Avanzado

**Básico (CAPA 4):**
```
distance(A, B) = 1 - cos(A, B)  ∈ [0, 1]
```

**Avanzado (CAPA 5):** Incluir **contexto temporal + emocional**

```
similarity_avanzada(A, B) = 
    α * cosine(A, B) +
    β * temporal_proximity(A, B) +
    γ * emotional_resonance(A, B)

donde:
  α = 0.6 (similitud semántica es lo principal)
  β = 0.2 (proximidad temporal importa algo)
  γ = 0.2 (resonancia emocional importa)
```

**Ejemplo:**

```
Contexto A: "necesito dinero" (turn 3)
  └─ Embedding: [0.12, -0.45, 0.89, ...]
  └─ CTX7D: [0.75, 0.2, 0.8, 0.6, 0.9, 0.95, 0.7]
  
Contexto B: "me falta plata" (turn 45)
  └─ Embedding: [0.10, -0.42, 0.87, ...]
  └─ CTX7D: [0.73, 0.22, 0.2, 0.58, 0.92, 0.93, 0.71]

Similitud:
  - Coseno: 0.98 (muy similar semánticamente)
  - Temporal: 0.3 (turn 3 vs turn 45 = 42 turns de distancia)
  - Emocional: 0.95 (ambos tienen emotional ≈ 0.2)
  
Resultado: 0.6*0.98 + 0.2*0.3 + 0.2*0.95 = 0.588 + 0.06 + 0.19 = 0.838 (bastante similar, pero no idéntico)
```

### TOPOLOGÍA: Grafo de Conversación

**Estructura:**

```
┌──────────────────────────────────────────────────┐
│  CONVERSACIÓN = GRAFO DIRIGIDO                  │
├──────────────────────────────────────────────────┤
│                                                  │
│  Nodos: FlowPacks / Contextos                  │
│  Aristas: Relaciones de similitud + causalidad │
│                                                  │
│         Turn 1 (Introducción)                  │
│         [CTX7D: baja certeza]                  │
│              ↓ (causalidad)                     │
│         Turn 3 (Problema)                      │
│         [CTX7D: alta urgencia]                 │
│              ↓ (progresión)     ↖ (loop?)      │
│         Turn 8 (Solución 1)   Turn 12 (Re-ask) │
│         [CTX7D: propósito 0.9]                 │
│              ↓ (confirmación)                   │
│         Turn 15 (Cierre)                       │
│         [CTX7D: alta certeza]                  │
│                                                  │
└──────────────────────────────────────────────────┘
```

**Tipos de aristas:**

| Tipo | Condición | Significado |
|------|-----------|------------|
| **Causal** | turn B sigue turn A + similitud > 0.7 | "B es causa de A" |
| **Loop** | A y B similares + turns alejados | "Repetición de tema" |
| **Progresión** | CTX7D[6] (propósito) crece | "Conversación progresa" |
| **Regresión** | CTX7D[6] decrece | "Volvemos a dudas" |

### DETECCIÓN DE CICLOS (Disco Rayado)

**Problema:** Usuario repite la misma pregunta 5 veces → El LLM debe notarlo y cambiar estrategia.

**Solución:** Floyd's cycle detection en topología conversacional

```
┌─────────────────────────────────────────────┐
│  DETECCIÓN DE CICLO                         │
├─────────────────────────────────────────────┤
│                                             │
│  Turn 3: "Necesito dinero"                  │
│           CTX7D: [0.7, 0.2, 0.9, ...]       │
│           ↓                                  │
│  Turn 8:  "Sigo necesitando dinero"         │
│           CTX7D: [0.71, 0.21, 0.88, ...]    │
│           distance = 0.05 (muy cercano)     │
│           ↓                                  │
│  Turn 13: "Aún necesito dinero"             │
│           CTX7D: [0.72, 0.22, 0.87, ...]    │
│           distance = 0.04 (MUY cercano)     │
│           ↓                                  │
│  CICLO DETECTADO ✓                          │
│  Acción: Cambiar estrategia de respuesta    │
│                                             │
└─────────────────────────────────────────────┘
```

**Algoritmo:** Floyd's Cycle Detection (tortuga + liebre)
- tortuga = avanza 1 turn
- liebre = avanza 2 turns
- Si se encuentran = ciclo

**Parámetro crítico:** `loop_threshold` = distancia máxima para considerar "repetición"
- Recomendación: 0.15 (15% de diferencia en CTX7D es "prácticamente igual")

### MAPEO EMOCIONAL: Progresión del Sentimiento

```
┌────────────────────────────────────────┐
│  EVOLUCIÓN EMOCIONAL DE CONVERSACIÓN   │
├────────────────────────────────────────┤
│                                        │
│  Emocional (CTX7D[2]):                 │
│  -1.0 (negativo) ←────→ 1.0 (positivo) │
│                                        │
│   Turn  3: -0.2 (frustración)          │
│       \                                │
│   Turn  8:  0.0 (neutral)              │
│        \                               │
│   Turn 13:  0.3 (esperanza)            │
│         \                              │
│   Turn 15:  0.7 (satisfacción)         │
│                                        │
│   Tendencia: ASCENDENTE (+0.9)         │
│   Volatilidad: BAJA (swings < 0.3)     │
│   Patrón: Lineal growth (predecible)   │
│                                        │
└────────────────────────────────────────┘
```

---

## PARTE II: IMPLEMENTACIÓN (TÉCNICO)

### STRUCT: PatternRecognizer

```rust
/// Analiza patrones en conversaciones
pub struct PatternRecognizer {
    /// Grafo dirigido de contextos
    graph: ConversationGraph,
    
    /// Memoria de ciclos detectados
    cycles: Vec<CycleDetection>,
    
    /// Estadísticas emocionales
    emotional_stats: EmotionalStats,
    
    /// Parámetros de configuración
    loop_threshold: f32,
    similarity_threshold: f32,
}

/// Grafo de conversación
#[derive(Debug, Clone)]
pub struct ConversationGraph {
    /// Nodos: [flowpack_id] → Node
    nodes: std::collections::HashMap<[u8; 32], GraphNode>,
    
    /// Aristas: (source, target) → Edge
    edges: std::collections::HashMap<
        ([u8; 32], [u8; 32]),
        GraphEdge,
    >,
}

/// Nodo en el grafo
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub flowpack_id: [u8; 32],
    pub ctx7d: ContextToken7D,
    pub embedding: Embedding,
    pub turn_index: u32,
}

/// Arista en el grafo
#[derive(Debug, Clone)]
pub enum GraphEdge {
    Causal { similarity: f32, confidence: f32 },
    Loop { repetitions: u32, severity: f32 },
    Progression { direction: i32 }, // +1 = progreso, -1 = retroceso
    Resonance { emotional_alignment: f32 },
}

/// Detección de ciclo
#[derive(Debug, Clone)]
pub struct CycleDetection {
    pub cycle_nodes: Vec<[u8; 32]>,
    pub entry_turn: u32,
    pub cycle_length: u32,
    pub avg_distance: f32, // Distancia promedio entre nodos del ciclo
}

/// Estadísticas emocionales
#[derive(Debug, Clone)]
pub struct EmotionalStats {
    pub trend: f32, // pendiente emocional
    pub volatility: f32, // desviación estándar
    pub avg_emotion: f32, // promedio
    pub min_emotion: f32,
    pub max_emotion: f32,
}
```

### ALGORITMO: Construcción del Grafo

```rust
impl PatternRecognizer {
    /// Crea nuevo PatternRecognizer
    pub fn new(loop_threshold: f32, similarity_threshold: f32) -> Self {
        Self {
            graph: ConversationGraph {
                nodes: HashMap::new(),
                edges: HashMap::new(),
            },
            cycles: Vec::new(),
            emotional_stats: EmotionalStats::default(),
            loop_threshold,
            similarity_threshold,
        }
    }
    
    /// Construye grafo desde secuencia de FlowPacks
    pub fn build_graph(
        &mut self,
        flowpacks: &[FlowPack],
        embeddings: &[Embedding],
        ctx7ds: &[ContextToken7D],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // PASO 1: Agregar nodos
        for (i, (flowpack, embedding, ctx7d)) in izip!(flowpacks, embeddings, ctx7ds).enumerate() {
            let node = GraphNode {
                flowpack_id: flowpack.id,
                ctx7d: *ctx7d,
                embedding: embedding.clone(),
                turn_index: i as u32,
            };
            self.graph.nodes.insert(flowpack.id, node);
        }
        
        // PASO 2: Agregar aristas (basado en similitud)
        for i in 0..flowpacks.len() {
            for j in (i+1)..flowpacks.len() {
                let similarity = self.calculate_advanced_similarity(
                    &embeddings[i],
                    &embeddings[j],
                    &ctx7ds[i],
                    &ctx7ds[j],
                    (j - i) as u32,
                );
                
                if similarity > self.similarity_threshold {
                    let edge_type = self.classify_edge(
                        &ctx7ds[i],
                        &ctx7ds[j],
                        (j - i) as u32,
                        similarity,
                    );
                    
                    self.graph.edges.insert(
                        (flowpacks[i].id, flowpacks[j].id),
                        edge_type,
                    );
                }
            }
        }
        
        // PASO 3: Detectar ciclos
        self.detect_cycles()?;
        
        // PASO 4: Calcular estadísticas emocionales
        self.calculate_emotional_stats(ctx7ds)?;
        
        Ok(())
    }
    
    /// Calcula similitud avanzada con factores temporales
    fn calculate_advanced_similarity(
        &self,
        emb_a: &Embedding,
        emb_b: &Embedding,
        ctx7d_a: &ContextToken7D,
        ctx7d_b: &ContextToken7D,
        turn_distance: u32,
    ) -> f32 {
        // Similitud semántica (coseno)
        let cosine_sim = self.cosine_similarity(&emb_a.vector, &emb_b.vector);
        
        // Proximidad temporal (decay exponencial)
        // Max proximity en turns cercanos, decae con distancia
        let temporal_proximity = (-0.05 * turn_distance as f32).exp();
        
        // Resonancia emocional (similitud en emocional)
        let emotional_resonance = 1.0 - (ctx7d_a.emotional - ctx7d_b.emotional).abs();
        
        // Fórmula ponderada
        let weights = (0.6, 0.2, 0.2);
        weights.0 * cosine_sim + weights.1 * temporal_proximity + weights.2 * emotional_resonance
    }
    
    /// Coseno entre dos embeddings
    fn cosine_similarity(&self, a: &[f32; 384], b: &[f32; 384]) -> f32 {
        let mut dot_product = 0.0f32;
        for i in 0..384 {
            dot_product += a[i] * b[i];
        }
        dot_product.max(0.0).min(1.0) // Clamp a [0, 1]
    }
    
    /// Clasifica tipo de arista
    fn classify_edge(
        &self,
        ctx7d_a: &ContextToken7D,
        ctx7d_b: &ContextToken7D,
        turn_distance: u32,
        similarity: f32,
    ) -> GraphEdge {
        // Si muy similar y turnos cercanos → Causal
        if similarity > 0.85 && turn_distance < 10 {
            return GraphEdge::Causal {
                similarity,
                confidence: 0.9,
            };
        }
        
        // Si similar pero turnos lejanos → Posible Loop
        if similarity > 0.75 && turn_distance > 15 {
            return GraphEdge::Loop {
                repetitions: 1,
                severity: similarity,
            };
        }
        
        // Si Purpose crece → Progresión
        if ctx7d_b.purpose > ctx7d_a.purpose + 0.1 {
            return GraphEdge::Progression { direction: 1 };
        }
        
        // Si Purpose cae → Regresión
        if ctx7d_b.purpose < ctx7d_a.purpose - 0.1 {
            return GraphEdge::Progression { direction: -1 };
        }
        
        // Default: Resonancia emocional
        let emotional_alignment = 1.0 - (ctx7d_a.emotional - ctx7d_b.emotional).abs();
        GraphEdge::Resonance { emotional_alignment }
    }
}
```

### ALGORITMO: Detección de Ciclos (Floyd)

```rust
impl PatternRecognizer {
    /// Detecta ciclos usando algoritmo de Floyd (tortuga + liebre)
    pub fn detect_cycles(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let node_ids: Vec<_> = self.graph.nodes.keys().copied().collect();
        
        for &start_id in &node_ids {
            let mut tortuga = start_id;
            let mut liebre = start_id;
            
            loop {
                // Tortuga avanza 1
                tortuga = self.next_similar_node(&tortuga, self.loop_threshold)?;
                if tortuga == [0u8; 32] {
                    break; // Sin siguiente similar
                }
                
                // Liebre avanza 2
                liebre = self.next_similar_node(&liebre, self.loop_threshold)?;
                if liebre == [0u8; 32] {
                    break;
                }
                liebre = self.next_similar_node(&liebre, self.loop_threshold)?;
                if liebre == [0u8; 32] {
                    break;
                }
                
                // Si se encuentran → ciclo encontrado
                if tortuga == liebre {
                    let cycle = self.extract_cycle(&tortuga)?;
                    self.cycles.push(cycle);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Encuentra nodo más similar al actual (siguiente en ciclo)
    fn next_similar_node(
        &self,
        current_id: &[u8; 32],
        threshold: f32,
    ) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        let current_node = self.graph.nodes.get(current_id)
            .ok_or("Nodo no encontrado")?;
        
        let mut best = [0u8; 32];
        let mut best_distance = f32::INFINITY;
        
        for (&candidate_id, candidate_node) in &self.graph.nodes {
            if candidate_id == *current_id {
                continue;
            }
            
            let distance = current_node.ctx7d.distance(&candidate_node.ctx7d);
            
            if distance < threshold && distance < best_distance {
                best = candidate_id;
                best_distance = distance;
            }
        }
        
        Ok(best)
    }
    
    /// Extrae ciclo encontrado
    fn extract_cycle(&self, start_id: &[u8; 32]) -> Result<CycleDetection, Box<dyn std::error::Error>> {
        let mut cycle_nodes = vec![*start_id];
        let mut current = *start_id;
        let mut total_distance = 0.0f32;
        
        loop {
            current = self.next_similar_node(&current, self.loop_threshold)?;
            if current == [0u8; 32] || current == *start_id {
                break;
            }
            
            let dist = self.graph.nodes[&cycle_nodes[cycle_nodes.len()-1]]
                .ctx7d
                .distance(&self.graph.nodes[&current].ctx7d);
            
            total_distance += dist;
            cycle_nodes.push(current);
            
            if cycle_nodes.len() > 20 {
                break; // Protección contra ciclos infinitos
            }
        }
        
        let avg_distance = total_distance / cycle_nodes.len() as f32;
        let entry_turn = self.graph.nodes[&cycle_nodes[0]].turn_index;
        let cycle_length = cycle_nodes.len() as u32;
        
        Ok(CycleDetection {
            cycle_nodes,
            entry_turn,
            cycle_length,
            avg_distance,
        })
    }
}
```

### ALGORITMO: Estadísticas Emocionales

```rust
impl PatternRecognizer {
    /// Calcula progresión emocional
    pub fn calculate_emotional_stats(
        &mut self,
        ctx7ds: &[ContextToken7D],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if ctx7ds.is_empty() {
            return Ok(());
        }
        
        let emotions: Vec<f32> = ctx7ds.iter().map(|c| c.emotional).collect();
        
        // Mínimo, máximo, promedio
        let min = emotions.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = emotions.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let avg = emotions.iter().sum::<f32>() / emotions.len() as f32;
        
        // Tendencia (regresión lineal)
        let trend = self.linear_regression(&emotions);
        
        // Volatilidad (desviación estándar)
        let variance = emotions.iter()
            .map(|e| (e - avg).powi(2))
            .sum::<f32>() / emotions.len() as f32;
        let volatility = variance.sqrt();
        
        self.emotional_stats = EmotionalStats {
            trend,
            volatility,
            avg_emotion: avg,
            min_emotion: min,
            max_emotion: max,
        };
        
        Ok(())
    }
    
    /// Regresión lineal simple (pendiente)
    fn linear_regression(&self, values: &[f32]) -> f32 {
        let n = values.len() as f32;
        let sum_x = (0..values.len()).map(|i| i as f32).sum::<f32>();
        let sum_y = values.iter().sum::<f32>();
        let sum_xy = values.iter()
            .enumerate()
            .map(|(i, &y)| i as f32 * y)
            .sum::<f32>();
        let sum_x2 = (0..values.len()).map(|i| (i as f32).powi(2)).sum::<f32>();
        
        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = n * sum_x2 - sum_x.powi(2);
        
        if denominator.abs() < 1e-6 {
            0.0
        } else {
            numerator / denominator
        }
    }
}
```

### PERFORMANCE TARGETS

| Métrica | Target | Ambiente |
|---------|--------|----------|
| Construcción de grafo (1000 nodos) | <500ms | STM32H7 @ 400MHz |
| Detección de ciclos | <100ms | Floyd algorithm |
| Cálculo de similitud avanzada | <5ms | Por par de contextos |
| Estadísticas emocionales | <50ms | Regresión lineal |

---

## PARTE III: INTEGRACIÓN

### Uso en CAPA 6 (Amplificación)

```rust
// En HubSpoke::route()
pub fn route(
    &self,
    current_ctx7d: &ContextToken7D,
    pattern_recognizer: &PatternRecognizer,
) -> LlmStrategy {
    // Si se detectó un ciclo
    if let Some(cycle) = pattern_recognizer.cycles.first() {
        if cycle.avg_distance < 0.15 {
            // Disco rayado detectado
            return LlmStrategy::BreakCycle {
                cycle_start: cycle.entry_turn,
                suggested_topic_shift: self.suggest_topic_shift(),
            };
        }
    }
    
    // Si progresión emocional positiva
    if pattern_recognizer.emotional_stats.trend > 0.1 {
        return LlmStrategy::ReinforceProgress {
            emotional_boost: true,
        };
    }
    
    // Default: estrategia normal
    LlmStrategy::Normal
}
```

---

## VALIDACIÓN

### CHECKLIST DE ACEPTACIÓN

- [ ] ConversationGraph construido correctamente
- [ ] Similitud avanzada con factores temporales
- [ ] Floyd cycle detection implementado
- [ ] Extracción de ciclos funcional
- [ ] Estadísticas emocionales (regresión lineal)
- [ ] Detección de loops con threshold configurable
- [ ] Performance <500ms para 1000 nodos

### TESTS UNITARIOS

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_advanced_similarity() {
        let recognizer = PatternRecognizer::new(0.15, 0.7);
        let emb_a = Embedding { vector: [0.1; 384], ..Default::default() };
        let emb_b = Embedding { vector: [0.11; 384], ..Default::default() };
        let ctx7d_a = ContextToken7D::neutral();
        let ctx7d_b = ContextToken7D::neutral();
        
        let sim = recognizer.calculate_advanced_similarity(
            &emb_a, &emb_b, &ctx7d_a, &ctx7d_b, 5
        );
        
        assert!(sim > 0.7); // Muy similar
    }
    
    #[test]
    fn test_cycle_detection() {
        let mut recognizer = PatternRecognizer::new(0.15, 0.7);
        
        // Crear FlowPacks repetitivos
        let flowpacks = vec![/* ... */];
        let embeddings = vec![/* ... */];
        let ctx7ds = vec![/* ... */];
        
        recognizer.build_graph(&flowpacks, &embeddings, &ctx7ds).unwrap();
        
        assert!(!recognizer.cycles.is_empty());
    }
}
```

---

## REFERENCIAS

- **00_VISION:** `04_arquitectura-sistema-7-capas.md` (definición CAPA 5)
- **01_ARQUITECTURA:** `08_indexacion-embeddings-hnsw.md` (productor upstream)
- **Floyd's Cycle Detection:** Algoritmo clásico de detección de ciclos en grafos

---

## NOTAS PARA DESARROLLO

- ⚠️ Loop threshold debe ser **adaptativo** según tipo de conversación
- ⚠️ Similitud avanzada usa **3 factores**; considerar agregar más
- ✅ Ciclos son **inmutables** una vez detectados
- ✅ Detección es **best-effort**, no garantiza encontrar TODOS los ciclos
- ✅ Estadísticas emocionales son **agregadas**, usar para contexto global

---

**Estado:** ✅ READY FOR CODING  
**Siguiente:** `10_routier-y-hubspoke.md` (CAPA 6)
