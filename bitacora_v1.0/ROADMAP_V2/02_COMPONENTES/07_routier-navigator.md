```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/07_routier-navigator.md
Versión: 2.0.0
Fecha Creación: 2025-10-26
Última Actualización: 2025-11-24
Autor: Sistema Bitácora - Documentación MTT-DSL
Propósito: Especificación componente Routier Navigator (Navegación adaptativa + Small World Networks)
Estado: 📋 ESPECIFICACIÓN
Relacionado Con:
  - ROADMAP_V2/00_VISION/DA-034_SMALL_WORLD_NETWORKS.md (NEW)
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/14_shuidao-topic-graph.md
  - ROADMAP_V2/02_COMPONENTES/IMPORTANTES/EXPERTISE_GENERATION.md
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md
  - ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md
  - src/routier/ (Código base futuro)
Implementa:
  - DA-034: Small World Networks (NEW)
  - DA-028: Routier Navigator (Adaptive Learning Paths)
  - DA-010: Context-Aware Routing
  - BITA-2: User Cognitive State Tracking
Cambios v2.0.0:
  - Agregada sección "SMALL WORLD NETWORKS" para navegación inteligente de conocimiento
  - Integración con TopicGraph (ShuiDao) para detection + navigation
  - Arquitectura 3-layer (hot/warm/cold) para mobile-first performance
  - Algoritmos: Dijkstra, PageRank, Betweenness, Louvain
  - Casos de uso: Shortest paths, serendipity, propagation, hubs
# === FIN DATOS DE AUDITORÍA ===
```

# 🧭 ROUTIER NAVIGATOR - Navegación Adaptativa de Rutas de Aprendizaje

---

## 🎯 PROPÓSITO

El **Routier Navigator** es el componente de Bitácora responsable de **navegar rutas de aprendizaje adaptativas** que evolucionan según el progreso real del usuario, NO según planes rígidos predefinidos.

### La Metáfora: GPS que Aprende de Ti

**Sistema tradicional (GPS estático):**
```
Usuario: "Quiero aprender React"

Sistema:
1. Aquí tienes tutorial de 50 pasos
2. Debes completar en orden: 1→2→3→...→50
3. No puedes saltar pasos
4. Todos reciben la misma ruta

Resultado:
❌ Paso 12: Usuario ya sabe esto (aburrido)
❌ Paso 18: Usuario no entendió prerequisito (frustrado)
❌ Paso 30: Usuario abandonó (80% dropout rate)
```

**Con Routier Navigator (GPS adaptativo):**
```
Usuario: "Quiero aprender React"
    ↓
FASE 1: Análisis de Estado Cognitivo
  Routier analiza TelescopeDB (biografía):
  ├─ 250 queries sobre JavaScript → Nivel: Expert
  ├─ 15 queries sobre HTML/CSS → Nivel: Advanced
  ├─ 0 queries sobre componentes → Nivel: None
  └─ Estado detectado: "Dev JS senior, débil en React patterns"
    ↓
FASE 2: Generación de Ruta Personalizada
  Routier crea ruta adaptativa:
  
  ✅ SKIP pasos 1-5: JavaScript basics (ya los dominas)
  ✅ START paso 6: React fundamentos (tu punto de entrada)
  ⚡ FOCUS paso 12: Componentes (tu gap principal)
  🎯 EXTEND paso 18: Hooks avanzados (aquí te quedaste antes)
  🌟 UNLOCK paso 25: Performance optimization (contenido extra)
    ↓
FASE 3: Navegación Dinámica (el GPS que aprende)
  
  Usuario completa paso 12 en 15 min (esperado: 45 min)
    → Routier detecta: "Entiende rápido, aumentar complejidad"
    → Ajuste: Skip pasos 13-14 → Jump to paso 15
  
  Usuario falla paso 18 dos veces
    → Routier detecta: "Gap en async JavaScript"
    → Ajuste: Insert paso 18.1: "Async/Await refresher"
    → Ajuste: Insert paso 18.2: "useEffect deep dive"
  
  Usuario pregunta: "¿Cómo optimizar renders?"
    → Routier detecta: Interés emergente
    → Ajuste: Unlock paso 25 NOW (antes del paso 20)
    ↓
FASE 4: Destino Alcanzado (o Ruta Alternativa)
  
  Usuario domina React en 3 semanas (plan original: 8 semanas)
    → Routier sugiere: "¿Next.js? ¿React Native? ¿State management?"
  
  O... Usuario descubre que prefiere Vue
    → Routier pivotea: "Ruta alternativa detectada, generando Vue path..."
```

**La diferencia clave:**
- Sistema tradicional: **Plan fijo**, usuario se adapta a la ruta
- Routier Navigator: **Ruta adaptativa**, se adapta al usuario

---

## 🏗️ CONTEXTO ARQUITECTÓNICO

### Ubicación en el Sistema

```
FLUJO COMPLETO: Expertise Generation → Routier Navigator

Usuario: "Ayúdame a aprender machine learning"
    ↓
┌─────────────────────────────────────────────────┐
│ EXPERTISE GENERATION (Ya implementado)          │
│ └─> ExpertisePackage {                          │
│       curriculum: 6 fases,                      │
│       templates: 18 MTT-DSL,                    │
│       resources: 24 curated,                    │
│       projects: 6 progressive                   │
│     }                                           │
└─────────────────────────────────────────────────┘
    ↓
┌─────────────────────────────────────────────────┐
│ ★★★ ROUTIER NAVIGATOR (TÚ ESTÁS AQUÍ) ★★★      │
│                                                 │
│ RESPONSABILIDAD: Navegar el curriculum         │
│                                                 │
│ INPUT: ExpertisePackage del usuario            │
│ OUTPUT: Next learning step adaptativo          │
│                                                 │
│ FASE 1: Mapear Ruta Inicial                    │
│  ├─ Convertir Curriculum → LearningGraph       │
│  ├─ Nodos = pasos del curriculum               │
│  ├─ Edges = dependencias entre pasos           │
│  └─ Generar: LearningPath inicial              │
│                                                 │
│ FASE 2: Tracking de Estado Cognitivo           │
│  ├─ Monitorear progreso del usuario:           │
│  │   - Tiempo por paso (rápido/lento)          │
│  │   - Intentos por paso (1st try vs retries)  │
│  │   - Queries del usuario (confusión/interés) │
│  │   - Engagement (alto/medio/bajo)            │
│  │                                              │
│  └─ Actualizar: CognitiveState continuo        │
│                                                 │
│ FASE 3: Adaptación Dinámica                    │
│  ├─ Detectar eventos:                          │
│  │   - Usuario avanza rápido → Skip pasos      │
│  │   - Usuario se atora → Insert prerequisitos │
│  │   - Usuario muestra interés → Unlock extras │
│  │   - Usuario se aburre → Change approach     │
│  │                                              │
│  └─ Ejecutar: RouteAdjustment (modificar path) │
│                                                 │
│ FASE 4: Recomendación de Siguiente Paso        │
│  ├─ Analizar:                                  │
│  │   - Current position en LearningGraph       │
│  │   - CognitiveState actual                   │
│  │   - Dependencies (qué puede desbloquearse)  │
│  │   - User preferences (intereses)            │
│  │                                              │
│  └─ Generar: NextStep recommendation           │
│      ├─ Content: Qué hacer                     │
│      ├─ Difficulty: Cuán difícil será          │
│      ├─ Estimated time: Cuánto tomará          │
│      └─ Why: Por qué este paso ahora           │
└─────────────────────────────────────────────────┘
    ↓
Usuario: "¡Perfecto! Este paso es justo lo que necesito" ✅
```

### Interacciones con Otros Componentes

| Componente | Dirección | Propósito |
|------------|-----------|-----------|
| **Expertise Generation** | Entrada ← | Recibe ExpertisePackage (curriculum completo) |
| **TelescopeDB** | Consulta ↔ | Lee biografía para detectar estado cognitivo |
| **Context Token 7D** | Consulta ← | Analiza contexto de queries del usuario |
| **MTT-DSL Engine** | Salida → | Genera templates para el paso actual |
| **VoxelDB** | Salida → | Almacena progreso del usuario en 3D |

---

## 📋 RESPONSABILIDADES CORE

El Routier Navigator **DEBE**:

1. **Generación de Learning Graph:**
   - Parsear Curriculum (de Expertise Generation) → DAG (Directed Acyclic Graph)
   - Identificar dependencias entre pasos
   - Detectar pasos opcionales vs obligatorios
   - Calcular caminos mínimos y máximos

2. **Tracking de Estado Cognitivo:**
   - Monitorear tiempo por paso (velocity)
   - Rastrear intentos por paso (struggle indicators)
   - Analizar queries del usuario (confusion patterns)
   - Medir engagement (attention span)
   - Detectar burnout o aburrimiento

3. **Adaptación Dinámica de Ruta:**
   - **Skip:** Saltar pasos si usuario avanza rápido
   - **Insert:** Agregar prerequisitos si usuario se atora
   - **Unlock:** Habilitar contenido avanzado si muestra interés
   - **Pivot:** Cambiar enfoque si detecta frustración persistente
   - **Extend:** Agregar proyectos extra si domina tema

4. **Recomendación de Siguiente Paso:**
   - Seleccionar próximo paso óptimo según:
     * Estado cognitivo actual
     * Dependencias del grafo
     * Preferencias del usuario
     * Dificultad adaptativa
   - Generar explicación (¿por qué este paso?)
   - Estimar tiempo y dificultad

5. **Persistencia de Progreso:**
   - Guardar LearningPath en VoxelDB
   - Actualizar CognitiveState en TelescopeDB
   - Registrar ajustes de ruta (para análisis)

---

## 🗂️ ESTRUCTURAS DE DATOS

```rust
// src/routier/mod.rs

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::expertise_generation::ExpertisePackage;
use crate::core::context_token::ContextTensor7D;

/// Motor principal de navegación adaptativa
pub struct RoutierNavigator {
    /// Graph de aprendizaje generado
    learning_graph: LearningGraph,
    
    /// Estado cognitivo del usuario
    cognitive_state: CognitiveState,
    
    /// Ruta actual (adaptativa)
    current_path: LearningPath,
    
    /// Historial de ajustes
    adjustment_history: Vec<RouteAdjustment>,
    
    /// Configuración
    config: RoutierConfig,
}

/// Configuración del Routier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutierConfig {
    /// Threshold para skip (velocidad mínima)
    pub skip_velocity_threshold: f64,
    
    /// Threshold para insert (intentos máximos)
    pub insert_retry_threshold: usize,
    
    /// Threshold para unlock (interés mínimo)
    pub unlock_interest_threshold: f64,
    
    /// Threshold para pivot (frustración máxima)
    pub pivot_frustration_threshold: f64,
}

/// Grafo de aprendizaje (DAG)
#[derive(Debug, Clone)]
pub struct LearningGraph {
    /// Nodos = pasos del curriculum
    pub nodes: HashMap<StepID, LearningStep>,
    
    /// Edges = dependencias (step A → step B)
    pub edges: HashMap<StepID, Vec<StepID>>,
    
    /// Nodo inicial (entry point)
    pub start_node: StepID,
    
    /// Nodos finales (posibles endpoints)
    pub end_nodes: HashSet<StepID>,
}

impl LearningGraph {
    /// Genera grafo desde ExpertisePackage
    pub fn from_expertise_package(package: &ExpertisePackage) -> Result<Self> {
        let mut nodes = HashMap::new();
        let mut edges = HashMap::new();
        
        // Convertir cada fase del curriculum en nodos
        for (phase_idx, phase) in package.curriculum.phases.iter().enumerate() {
            for (concept_idx, concept) in phase.concepts.iter().enumerate() {
                let step_id = StepID {
                    phase: phase_idx,
                    concept: concept_idx,
                };
                
                let step = LearningStep {
                    id: step_id.clone(),
                    title: concept.name.clone(),
                    description: concept.description.clone(),
                    difficulty: concept.difficulty,
                    estimated_time: Duration::from_secs(concept.estimated_hours * 3600),
                    prerequisites: concept.prerequisites.clone(),
                    is_optional: false,
                };
                
                nodes.insert(step_id.clone(), step);
                
                // Crear edges desde prerequisites
                let deps: Vec<StepID> = concept.prerequisites.iter()
                    .filter_map(|prereq| Self::find_step_by_name(&nodes, prereq))
                    .collect();
                
                if !deps.is_empty() {
                    edges.insert(step_id, deps);
                }
            }
        }
        
        let start_node = StepID { phase: 0, concept: 0 };
        let end_nodes = Self::find_terminal_nodes(&nodes, &edges);
        
        Ok(LearningGraph {
            nodes,
            edges,
            start_node,
            end_nodes,
        })
    }
    
    /// Encuentra nodos sin dependientes (terminales)
    fn find_terminal_nodes(
        nodes: &HashMap<StepID, LearningStep>,
        edges: &HashMap<StepID, Vec<StepID>>,
    ) -> HashSet<StepID> {
        nodes.keys()
            .filter(|node_id| !edges.values().any(|deps| deps.contains(node_id)))
            .cloned()
            .collect()
    }
}

/// ID de un paso en el grafo
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StepID {
    pub phase: usize,
    pub concept: usize,
}

/// Paso de aprendizaje
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStep {
    pub id: StepID,
    pub title: String,
    pub description: String,
    pub difficulty: f64,
    pub estimated_time: Duration,
    pub prerequisites: Vec<String>,
    pub is_optional: bool,
}

/// Ruta de aprendizaje adaptativa
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    /// Pasos en orden
    pub steps: Vec<StepID>,
    
    /// Paso actual (índice en steps)
    pub current_position: usize,
    
    /// Pasos completados
    pub completed_steps: HashSet<StepID>,
    
    /// Pasos desbloqueados pero no completados
    pub unlocked_steps: HashSet<StepID>,
    
    /// Pasos skipped
    pub skipped_steps: HashSet<StepID>,
}

/// Estado cognitivo del usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    /// Velocidad promedio (pasos por hora)
    pub velocity: f64,
    
    /// Tasa de éxito (0.0-1.0)
    pub success_rate: f64,
    
    /// Nivel de frustración (0.0-1.0)
    pub frustration_level: f64,
    
    /// Nivel de engagement (0.0-1.0)
    pub engagement_level: f64,
    
    /// Intereses emergentes
    pub emerging_interests: Vec<String>,
    
    /// Patterns de confusión detectados
    pub confusion_patterns: Vec<ConfusionPattern>,
    
    /// Última actualización
    pub last_updated: i64,
}

/// Pattern de confusión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfusionPattern {
    pub topic: String,
    pub occurrences: usize,
    pub last_seen: i64,
}

/// Ajuste de ruta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteAdjustment {
    /// Tipo de ajuste
    pub adjustment_type: AdjustmentType,
    
    /// Razón del ajuste
    pub reason: String,
    
    /// Paso afectado
    pub affected_step: StepID,
    
    /// Timestamp
    pub timestamp: i64,
}

/// Tipos de ajuste
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    Skip {
        skipped_steps: Vec<StepID>,
    },
    Insert {
        new_steps: Vec<LearningStep>,
        insert_after: StepID,
    },
    Unlock {
        unlocked_step: StepID,
    },
    Pivot {
        new_focus: String,
        reroute_from: StepID,
    },
    Extend {
        extra_projects: Vec<String>,
    },
}

/// Recomendación de siguiente paso
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStepRecommendation {
    /// Paso recomendado
    pub step: LearningStep,
    
    /// Razón (explicación humana)
    pub why: String,
    
    /// Dificultad estimada (0.0-1.0)
    pub estimated_difficulty: f64,
    
    /// Tiempo estimado
    pub estimated_time: Duration,
    
    /// Confianza de la recomendación (0.0-1.0)
    pub confidence: f64,
}
```

---

## 🔌 API PÚBLICA

```rust
// src/routier/mod.rs

impl RoutierNavigator {
    /// Crear nuevo navigator desde ExpertisePackage
    pub fn new(package: ExpertisePackage, config: RoutierConfig) -> Result<Self> {
        let learning_graph = LearningGraph::from_expertise_package(&package)?;
        
        let initial_path = LearningPath {
            steps: vec![learning_graph.start_node.clone()],
            current_position: 0,
            completed_steps: HashSet::new(),
            unlocked_steps: HashSet::new(),
            skipped_steps: HashSet::new(),
        };
        
        Ok(Self {
            learning_graph,
            cognitive_state: CognitiveState::default(),
            current_path: initial_path,
            adjustment_history: Vec::new(),
            config,
        })
    }
    
    /// Recomendar siguiente paso
    pub fn recommend_next_step(&self) -> Result<NextStepRecommendation> {
        // Obtener paso actual
        let current_step_id = &self.current_path.steps[self.current_path.current_position];
        
        // Encontrar pasos desbloqueados (prerequisites cumplidos)
        let unlocked = self.find_unlocked_steps()?;
        
        // Seleccionar mejor siguiente paso según:
        // 1. Estado cognitivo (velocidad, frustración)
        // 2. Dificultad adaptativa
        // 3. Intereses del usuario
        let next_step = self.select_optimal_next_step(&unlocked)?;
        
        // Calcular dificultad estimada
        let estimated_difficulty = self.estimate_difficulty(&next_step);
        
        // Calcular tiempo estimado
        let estimated_time = self.estimate_time(&next_step);
        
        // Generar explicación
        let why = self.generate_explanation(&next_step);
        
        Ok(NextStepRecommendation {
            step: next_step,
            why,
            estimated_difficulty,
            estimated_time,
            confidence: 0.85,
        })
    }
    
    /// Actualizar estado cognitivo tras completar paso
    pub fn update_cognitive_state(
        &mut self,
        step_id: &StepID,
        completion_time: Duration,
        attempts: usize,
        user_queries: Vec<String>,
    ) -> Result<()> {
        // Calcular nueva velocidad
        let expected_time = self.learning_graph.nodes[step_id].estimated_time;
        let velocity_ratio = expected_time.as_secs_f64() / completion_time.as_secs_f64();
        
        self.cognitive_state.velocity = 
            (self.cognitive_state.velocity * 0.7) + (velocity_ratio * 0.3);
        
        // Calcular nueva tasa de éxito
        let success = if attempts == 1 { 1.0 } else { 1.0 / attempts as f64 };
        self.cognitive_state.success_rate = 
            (self.cognitive_state.success_rate * 0.8) + (success * 0.2);
        
        // Detectar frustración
        if attempts > 3 {
            self.cognitive_state.frustration_level += 0.15;
        } else {
            self.cognitive_state.frustration_level *= 0.9;
        }
        
        // Analizar queries para detectar intereses/confusión
        self.analyze_user_queries(&user_queries)?;
        
        // Marcar paso como completado
        self.current_path.completed_steps.insert(step_id.clone());
        
        // Timestamp
        self.cognitive_state.last_updated = chrono::Utc::now().timestamp();
        
        Ok(())
    }
    
    /// Adaptar ruta dinámicamente
    pub fn adapt_route(&mut self) -> Result<Option<RouteAdjustment>> {
        // Caso 1: Usuario avanza rápido → SKIP pasos
        if self.cognitive_state.velocity > self.config.skip_velocity_threshold {
            return self.attempt_skip();
        }
        
        // Caso 2: Usuario se atora → INSERT prerequisitos
        if self.cognitive_state.frustration_level > self.config.pivot_frustration_threshold {
            return self.attempt_insert_prerequisite();
        }
        
        // Caso 3: Usuario muestra interés → UNLOCK extras
        if !self.cognitive_state.emerging_interests.is_empty() {
            return self.attempt_unlock_advanced();
        }
        
        // Caso 4: Usuario muy frustrado → PIVOT
        if self.cognitive_state.frustration_level > 0.80 {
            return self.attempt_pivot();
        }
        
        Ok(None)
    }
    
    /// Intentar skip de pasos
    fn attempt_skip(&mut self) -> Result<Option<RouteAdjustment>> {
        let current_pos = self.current_path.current_position;
        let next_steps = &self.current_path.steps[current_pos + 1..];
        
        // Encontrar pasos que podemos skip (similares al actual)
        let skippable: Vec<StepID> = next_steps.iter()
            .take(3) // Max 3 pasos adelante
            .filter(|step_id| {
                let step = &self.learning_graph.nodes[step_id];
                step.difficulty < 0.5 // Solo pasos fáciles
            })
            .cloned()
            .collect();
        
        if skippable.is_empty() {
            return Ok(None);
        }
        
        // Marcar como skipped
        for step_id in &skippable {
            self.current_path.skipped_steps.insert(step_id.clone());
        }
        
        let adjustment = RouteAdjustment {
            adjustment_type: AdjustmentType::Skip {
                skipped_steps: skippable.clone(),
            },
            reason: format!(
                "Usuario avanza rápido (velocity: {:.2}), skipping {} pasos fáciles",
                self.cognitive_state.velocity,
                skippable.len()
            ),
            affected_step: skippable[0].clone(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.adjustment_history.push(adjustment.clone());
        
        Ok(Some(adjustment))
    }
    
    /// Intentar insert de prerequisitos
    fn attempt_insert_prerequisite(&mut self) -> Result<Option<RouteAdjustment>> {
        // Analizar confusion patterns
        let confused_topics: Vec<&str> = self.cognitive_state.confusion_patterns.iter()
            .filter(|p| p.occurrences >= 2)
            .map(|p| p.topic.as_str())
            .collect();
        
        if confused_topics.is_empty() {
            return Ok(None);
        }
        
        // Generar pasos de prerequisito
        let new_steps: Vec<LearningStep> = confused_topics.iter()
            .map(|topic| LearningStep {
                id: StepID {
                    phase: 999, // Marca como insertado
                    concept: 0,
                },
                title: format!("{} Refresher", topic),
                description: format!("Repaso de {} antes de continuar", topic),
                difficulty: 0.3,
                estimated_time: Duration::from_secs(1800), // 30 min
                prerequisites: vec![],
                is_optional: false,
            })
            .collect();
        
        let current_step = self.current_path.steps[self.current_path.current_position].clone();
        
        let adjustment = RouteAdjustment {
            adjustment_type: AdjustmentType::Insert {
                new_steps: new_steps.clone(),
                insert_after: current_step.clone(),
            },
            reason: format!(
                "Usuario confundido en {} temas, insertando refreshers",
                confused_topics.len()
            ),
            affected_step: current_step,
            timestamp: chrono::Utc::now().timestamp(),
        };
        
        self.adjustment_history.push(adjustment.clone());
        
        Ok(Some(adjustment))
    }
}
```

---

## 🌐 SMALL WORLD NETWORKS - Navegación Inteligente de Conocimiento

> **Nueva capacidad (DA-034):** Routier ahora implementa teoría de grafos de Small World Networks para navegación inteligente entre conceptos del usuario.

### 🎯 Propósito

Mientras **TopicGraph (ShuiDao)** detecta "¿de qué habla el usuario?" (<15ms), **Routier** navega "¿cómo conectar conceptos?" usando:
- 6 Grados de Separación (Watts-Strogatz)
- Redes sin Escala (Barabási-Albert)
- Lazos Débiles (Granovetter)

### 📐 Arquitectura 3-Layer

```yaml
HOT PATH (cada mensaje):
  - TopicGraph.detect(): 12ms
  - Routier: NO ejecutado (evita bloqueo)
  - Total: 12ms ✅

WARM PATH (usuario pide conexión):
  - find_shortest_path(): 5ms (Dijkstra)
  - Total: 17ms ✅

COLD PATH (background, 1x día):
  - PageRank: 12ms
  - Betweenness: 180ms
  - Community Detection: 25ms
  - Total: 217ms (offline, cero impacto UX)
```

### 🗂️ Estructuras de Datos Adicionales

```rust
// ============================================
// NETWORK TOPOLOGY
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// Nodos (topics del usuario)
    nodes: HashMap<TopicId, TopicNode>,
    
    /// Aristas (conexiones entre topics)
    edges: Vec<TopicEdge>,
    
    /// Hubs detectados (top 5% más conectados)
    hubs: Vec<TopicHub>,
    
    /// Comunidades (clusters de Louvain)
    communities: Vec<TopicCommunity>,
    
    /// Atajos (lazos débiles de Granovetter)
    shortcuts: Vec<TopicShortcut>,
    
    /// Métricas globales
    metrics: NetworkMetrics,
}

#[derive(Debug, Clone)]
pub struct TopicNode {
    id: TopicId,
    name: String,
    
    // Métricas de centralidad
    degree: usize,           // Conexiones directas
    betweenness: f32,        // % paths que pasan por aquí
    pagerank: f32,           // Importancia iterativa
    closeness: f32,          // Cercanía al resto
}

#[derive(Debug, Clone)]
pub struct TopicHub {
    topic_id: TopicId,
    name: String,
    degree: usize,
    is_critical: bool,       // ¿Eliminar fragmenta red?
    connections: Vec<(TopicId, f32)>,  // (id, weight)
}

#[derive(Debug, Clone)]
pub struct TopicCommunity {
    id: CommunityId,
    topics: Vec<TopicId>,
    density: f32,            // Densidad intra-cluster
    modularity: f32,         // Modularidad (Louvain)
    representative: TopicId, // Topic más central
}

#[derive(Debug, Clone)]
pub struct TopicShortcut {
    from: TopicId,
    to: TopicId,
    from_cluster: CommunityId,
    to_cluster: CommunityId,
    
    // Valor del atajo
    path_reduction: usize,   // Saltos ahorrados
    serendipity: f32,        // Sorpresa (0.0-1.0)
}

#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    /// Grados de separación promedio
    avg_path_length: f32,
    
    /// Coeficiente de clustering (0.0-1.0)
    clustering_coefficient: f32,
    
    /// Ratio Small World (L_actual / L_random)
    small_world_ratio: f32,
    
    /// Sigma (C/C_random) / (L/L_random)
    sigma: f32,
    
    /// Distribución Power Law (γ)
    power_law_exponent: f32,
}
```

### 🔌 API Pública Extendida

```rust
impl RoutierNavigator {
    // ============================================
    // WARM PATH: Usuario pide explícitamente
    // ============================================
    
    /// Encuentra path más corto entre dos topics
    pub fn find_shortest_path(
        &self,
        from: TopicId,
        to: TopicId,
    ) -> Result<Path> {
        // Dijkstra: O(E + V log V)
        // Performance: <5ms para 500 topics
    }
    
    /// Sugiere conexiones serendípicas
    pub fn suggest_serendipitous_connections(
        &self,
        current_topic: TopicId,
    ) -> Vec<Insight> {
        // Filtra shortcuts con serendipity > 0.7
        // Genera insights creativos
    }
    
    /// Recomienda siguiente aprendizaje basado en hubs
    pub fn suggest_next_learning(
        &self,
        user_id: &UserId,
    ) -> Vec<LearningPath> {
        // Explora vecinos no visitados de hubs
        // Ordena por dificultad
    }
    
    /// Simula propagación de idea
    pub fn simulate_idea_propagation(
        &self,
        seed: TopicId,
        idea: &str,
    ) -> PropagationResult {
        // BFS con decay
        // Watts-Strogatz model
    }
    
    // ============================================
    // COLD PATH: Background processing
    // ============================================
    
    /// Recalcula métricas de red (ejecuta 1x día, offline)
    pub async fn refresh_network_topology(&mut self) -> Result<()> {
        if !self.is_optimal_time() {
            return Ok(());  // Espera condiciones óptimas
        }
        
        tokio::spawn(async move {
            self.calculate_pagerank();
            self.calculate_betweenness();
            self.detect_communities();
            self.find_shortcuts();
            self.update_metrics();
        });
        
        Ok(())
    }
    
    /// Detecta hubs críticos (cuyo removal fragmenta red)
    pub fn find_critical_hubs(&self) -> Vec<(TopicId, f32)> {
        // Simula remoción y mide impacto
    }
}
```

### ⚙️ Algoritmos Implementados

```rust
// 1. Shortest Path (Dijkstra)
fn dijkstra_shortest_path(
    &self,
    from: TopicId,
    to: TopicId,
) -> Vec<TopicId> {
    // Complejidad: O(E + V log V)
    // Performance: <5ms para 500 topics
    
    use std::collections::BinaryHeap;
    
    let mut dist: HashMap<TopicId, f32> = HashMap::new();
    let mut prev: HashMap<TopicId, TopicId> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    dist.insert(from, 0.0);
    heap.push((OrderedFloat(0.0), from));
    
    while let Some((OrderedFloat(cost), node)) = heap.pop() {
        if node == to { break; }
        
        for edge in self.topology.get_edges(node) {
            let new_cost = cost + (1.0 - edge.strength);
            
            if new_cost < *dist.get(&edge.to).unwrap_or(&f32::MAX) {
                dist.insert(edge.to, new_cost);
                prev.insert(edge.to, node);
                heap.push((OrderedFloat(new_cost), edge.to));
            }
        }
    }
    
    // Reconstruir path
    self.reconstruct_path(&prev, to)
}

// 2. PageRank (Iterativo)
fn calculate_pagerank(&mut self, iterations: usize) {
    // Complejidad: O(k·E) donde k=iterations
    // Performance: ~12ms para 1000 topics, k=20
    
    let damping = 0.85;
    let n = self.topology.nodes.len() as f32;
    let mut ranks: HashMap<TopicId, f32> = HashMap::new();
    
    // Inicializar
    for id in self.topology.nodes.keys() {
        ranks.insert(*id, 1.0 / n);
    }
    
    // Iterar
    for _ in 0..iterations {
        let mut new_ranks = HashMap::new();
        
        for id in self.topology.nodes.keys() {
            let mut rank = (1.0 - damping) / n;
            
            // Sumar contribuciones de vecinos
            for edge in self.topology.get_incoming_edges(*id) {
                let neighbor_rank = ranks[&edge.from];
                let neighbor_out_degree = self.topology.out_degree(edge.from);
                rank += damping * neighbor_rank / neighbor_out_degree as f32;
            }
            
            new_ranks.insert(*id, rank);
        }
        
        ranks = new_ranks;
    }
    
    // Actualizar nodos
    for (id, rank) in ranks {
        if let Some(node) = self.topology.nodes.get_mut(&id) {
            node.pagerank = rank;
        }
    }
}

// 3. Betweenness Centrality (Brandes algorithm)
fn calculate_betweenness(&mut self) {
    // Complejidad: O(V·E)
    // Performance: ~180ms para 1000 topics
    // ⚠️ SOLO ejecutar en Cold Path (offline)
    
    let mut betweenness: HashMap<TopicId, f32> = HashMap::new();
    
    for s in self.topology.nodes.keys() {
        let mut stack = Vec::new();
        let mut paths: HashMap<TopicId, Vec<TopicId>> = HashMap::new();
        let mut sigma: HashMap<TopicId, usize> = HashMap::new();
        let mut dist: HashMap<TopicId, i32> = HashMap::new();
        
        // BFS desde s
        // ... (algoritmo Brandes completo)
    }
    
    // Normalizar y actualizar
    let n = self.topology.nodes.len() as f32;
    for (id, score) in betweenness {
        let normalized = score * 2.0 / ((n - 1.0) * (n - 2.0));
        if let Some(node) = self.topology.nodes.get_mut(&id) {
            node.betweenness = normalized;
        }
    }
}

// 4. Community Detection (Louvain algorithm)
fn detect_communities(&mut self) -> Vec<TopicCommunity> {
    // Complejidad: O(V log V)
    // Performance: ~25ms para 1000 topics
    
    // Fase 1: Asignar cada nodo a su propia comunidad
    let mut communities: HashMap<TopicId, CommunityId> = HashMap::new();
    
    // Fase 2: Iterar hasta convergencia
    loop {
        let mut improved = false;
        
        for node in self.topology.nodes.keys() {
            // Calcular ganancia de modularidad al mover a cada vecino
            let best_move = self.find_best_community_move(*node, &communities);
            
            if best_move.gain > 0.0 {
                communities.insert(*node, best_move.community);
                improved = true;
            }
        }
        
        if !improved { break; }
    }
    
    // Construir estructuras TopicCommunity
    self.build_communities_from_partition(communities)
}
```

### 🎯 Casos de Uso

#### 1. Usuario pregunta conexión explícita

```rust
// Usuario: "¿Qué relación hay entre Rust y Cerámica?"

let from = topic_graph.find("Rust");
let to = topic_graph.find("Cerámica");

let path = routier.find_shortest_path(from, to)?;
// Path: Rust → Optimización → Química → Esmaltes → Cerámica

let insights = routier.generate_insights_from_path(&path);
// "Ambos requieren optimización: 
//  Rust optimiza código, Cerámica optimiza temperatura hornos"
```

#### 2. Sugerencia de siguiente aprendizaje

```rust
// Usuario domina Rust (hub con 47 conexiones)

let suggestions = routier.suggest_next_learning(&user_id);
// [
//   LearningPath { from: "Rust", to: "WebAssembly", difficulty: 5/10 },
//   LearningPath { from: "Rust", to: "LLVM", difficulty: 7/10 },
//   LearningPath { from: "Rust", to: "Embedded", difficulty: 6/10 },
// ]
```

#### 3. Insights serendípicos (lazos débiles)

```rust
// Sistema detecta shortcuts inesperados

let insights = routier.suggest_serendipitous_connections(current_topic);
// [
//   Insight {
//     connection: "Fermentación ↔ Compiladores",
//     rationale: "Ambos transforman en etapas: 
//                 Bacteria→Ácido→Kimchi, 
//                 Source→AST→IR→Binary",
//     serendipity: 0.91,
//   }
// ]
```

### 📊 Métricas de Éxito

```yaml
Small World Properties:
  avg_path_length: < 6 saltos ✅
  clustering_coefficient: > 0.5 ✅
  small_world_ratio: < 1.5 ✅
  sigma: > 1.0 ✅

Scale-Free Properties:
  power_law_exponent: [2, 3] ✅
  hub_concentration: Top 5% tiene >50% conexiones ✅

Performance:
  hot_path: 0ms (Routier no ejecutado) ✅
  warm_path: <10ms (find_path) ✅
  cold_path: <300ms (background, 1x día) ✅
  
Mobile:
  memory: +28MB (grafo 500 topics) ✅
  battery: <1% por hora ✅
```

### 🔗 Integración con TopicGraph

```rust
// TopicGraph notifica nuevos topics
impl TopicGraph {
    pub fn learn_topic(&mut self, topic: Topic) -> TopicId {
        let id = self.insert(topic);
        
        // Notifica a Routier
        self.event_bus.publish(Event::NewTopic {
            topic_id: id,
            embedding: self.embeddings[&id].clone(),
        });
        
        id
    }
}

// Routier actualiza grafo (preferential attachment)
impl RoutierNavigator {
    pub fn on_new_topic(&mut self, event: Event::NewTopic) {
        self.add_node(event.topic_id);
        
        // Conecta a hubs existentes (Barabási-Albert)
        let edges = self.calculate_edges_with_preferential_attachment(
            event.embedding
        );
        
        for edge in edges {
            self.add_edge(edge);
        }
        
        // Marca para recálculo (en próximo Cold Path)
        self.mark_metrics_stale();
    }
}
```

### 📚 Referencias Científicas

```yaml
Fundamentos:
  - Watts & Strogatz (1998): Small World Networks
  - Barabási & Albert (1999): Scale-Free Networks
  - Granovetter (1973): Strength of Weak Ties
  - Newman (2003): Structure of Complex Networks
  - Blondel et al. (2008): Louvain Algorithm

Para más detalles ver:
  - ROADMAP_V2/00_VISION/DA-034_SMALL_WORLD_NETWORKS.md
```

---

## ⚙️ IMPLEMENTACIÓN INTERNA

### Algoritmo: Selección Óptima de Siguiente Paso

```rust
impl RoutierNavigator {
    /// Selecciona siguiente paso óptimo
    fn select_optimal_next_step(
        &self,
        unlocked: &[StepID],
    ) -> Result<LearningStep> {
        // Scoring de cada paso desbloqueado
        let mut scores: Vec<(StepID, f64)> = unlocked.iter()
            .map(|step_id| {
                let step = &self.learning_graph.nodes[step_id];
                let score = self.calculate_step_score(step);
                (step_id.clone(), score)
            })
            .collect();
        
        // Ordenar por score descendente
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Seleccionar mejor
        let best_step_id = &scores[0].0;
        let best_step = self.learning_graph.nodes[best_step_id].clone();
        
        Ok(best_step)
    }
    
    /// Calcula score de un paso
    fn calculate_step_score(&self, step: &LearningStep) -> f64 {
        let mut score = 0.0;
        
        // Factor 1: Dificultad adaptativa (40%)
        // - Si usuario rápido → preferir pasos difíciles
        // - Si usuario lento → preferir pasos fáciles
        let difficulty_match = if self.cognitive_state.velocity > 1.2 {
            step.difficulty // Usuario rápido, dar difíciles
        } else {
            1.0 - step.difficulty // Usuario lento, dar fáciles
        };
        score += difficulty_match * 0.4;
        
        // Factor 2: Interés del usuario (30%)
        let interest_match = self.cognitive_state.emerging_interests.iter()
            .any(|interest| step.title.contains(interest)) as i32 as f64;
        score += interest_match * 0.3;
        
        // Factor 3: Momentum (20%)
        // - Pasos con muchos prerequisitos completados = alto momentum
        let completed_prereqs = step.prerequisites.iter()
            .filter(|prereq| {
                self.current_path.completed_steps.iter()
                    .any(|s| self.learning_graph.nodes[s].title.contains(prereq.as_str()))
            })
            .count();
        let momentum = completed_prereqs as f64 / step.prerequisites.len().max(1) as f64;
        score += momentum * 0.2;
        
        // Factor 4: Variedad (10%)
        // - Evitar repetir temas similares consecutivamente
        let variety = 0.5; // Simplified
        score += variety * 0.1;
        
        score
    }
}
```

---

## 🔗 DEPENDENCIAS

### Componentes de Bitácora

| Componente | Versión | Propósito |
|------------|---------|-----------|
| **Expertise Generation** | v1.0 | Provee ExpertisePackage (curriculum inicial) |
| **TelescopeDB** | v1.0 | Consultar biografía para estado cognitivo |
| **Context Token 7D** | v1.0 | Analizar contexto de queries del usuario |
| **VoxelDB** | v1.0 | Almacenar progreso y rutas en 3D |
| **MTT-DSL Engine** | v1.0 | Generar templates para cada paso |

### Crates Externos

```toml
[dependencies]
# Graph algorithms
petgraph = "0.6"          # DAG manipulation

# Core async
tokio = { version = "1.35", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Datetime
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1.0"
anyhow = "1.0"
```

---

## ⚡ OBJETIVOS DE PERFORMANCE

| Operación | Target | Medición | Status |
|-----------|--------|----------|--------|
| `recommend_next_step()` | <50ms | Selección óptima de siguiente paso | ⏸️ TBD |
| `update_cognitive_state()` | <20ms | Actualizar velocidad, frustración, etc. | ⏸️ TBD |
| `adapt_route()` | <100ms | Detectar y ejecutar ajuste (skip/insert/etc.) | ⏸️ TBD |
| **Grafo completo** | **<200ms** | **Generar LearningGraph desde curriculum** | **🎯 CRÍTICO** |
| Memoria RAM | <50 MB | RSS para navegador + grafo | ⏸️ TBD |

---

## 🧪 ESTRATEGIA DE TESTING

### Unit Tests

```rust
// tests/routier_test.rs

#[test]
fn test_skip_detection_fast_user() {
    let mut navigator = create_mock_navigator();
    
    // Simular usuario rápido (velocity > threshold)
    navigator.cognitive_state.velocity = 2.5; // Muy rápido
    
    let adjustment = navigator.adapt_route().unwrap();
    
    assert!(matches!(
        adjustment,
        Some(RouteAdjustment {
            adjustment_type: AdjustmentType::Skip { .. },
            ..
        })
    ));
}

#[test]
fn test_insert_prerequisite_confused_user() {
    let mut navigator = create_mock_navigator();
    
    // Simular usuario confundido
    navigator.cognitive_state.frustration_level = 0.85;
    navigator.cognitive_state.confusion_patterns.push(ConfusionPattern {
        topic: "async/await".to_string(),
        occurrences: 3,
        last_seen: chrono::Utc::now().timestamp(),
    });
    
    let adjustment = navigator.adapt_route().unwrap();
    
    assert!(matches!(
        adjustment,
        Some(RouteAdjustment {
            adjustment_type: AdjustmentType::Insert { .. },
            ..
        })
    ));
}
```

---

## ⚠️ MANEJO DE ERRORES

```rust
// src/routier/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoutierError {
    #[error("Grafo inválido: ciclo detectado")]
    CyclicGraphDetected,
    
    #[error("Paso no encontrado: {0:?}")]
    StepNotFound(StepID),
    
    #[error("Prerequisitos no cumplidos para paso {0:?}")]
    PrerequisitesNotMet(StepID),
    
    #[error("Estado cognitivo inválido: {0}")]
    InvalidCognitiveState(String),
    
    #[error("Curriculum vacío")]
    EmptyCurriculum,
}

pub type Result<T> = std::result::Result<T, RoutierError>;
```

---

## 📚 REFERENCIAS

### Documentos ROADMAP_V2

- **ROADMAP_V2/02_COMPONENTES/IMPORTANTES/EXPERTISE_GENERATION.md** - Genera curriculum inicial
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/TELESCOPEDB.md** - Biografía para estado cognitivo
- **ROADMAP_V2/02_COMPONENTES/CRITICOS/CONTEXT_TOKEN_7D.md** - Análisis contextual

### Decisiones Arquitectónicas

- **DA-028:** Routier Navigator (Adaptive Learning Paths)
- **DA-010:** Context-Aware Routing
- **BITA-2:** User Cognitive State Tracking

### FUSION_BAYESIANA

- **FUSION_BAYESIANA/02_GAP_ANALYSIS.md** (Brecha #7) - Routier Navigator como brecha ALTA
- **FUSION_BAYESIANA/07_PLAN_IMPLEMENTACION.md** (Semanas 13-14) - Plan de implementación

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata (Semanas 13-14)

1. **Implementar LearningGraph:**
   - Parser de Curriculum → DAG
   - Detección de ciclos
   - Cálculo de caminos mínimos
   - Unit tests con mock curriculums

2. **Implementar CognitiveState tracking:**
   - Calcular velocity adaptativa
   - Detectar frustration patterns
   - Analizar confusion topics
   - Integration con TelescopeDB

3. **Implementar Route Adaptation:**
   - Skip logic (fast users)
   - Insert logic (confused users)
   - Unlock logic (interested users)
   - Pivot logic (frustrated users)

4. **Implementar Next Step Recommendation:**
   - Step scoring algorithm
   - Difficulty estimation
   - Time estimation
   - Explanation generation

5. **Persistence:**
   - Guardar LearningPath en VoxelDB
   - Actualizar CognitiveState en TelescopeDB
   - Logs de RouteAdjustment

### Mejoras v1.5 (Semanas 15-16)

6. **ML-based adaptation:**
   - Entrenar modelo predictivo de frustración
   - Personalización continua (feedback loop)
   - Transfer learning entre usuarios

7. **Collaborative learning:**
   - Comparar rutas entre usuarios
   - Detectar mejores prácticas
   - Compartir ajustes exitosos

8. **Gamification:**
   - XP por paso completado
   - Achievements por milestones
   - Leaderboards adaptativos

---

**Estado:** 📋 ESPECIFICACIÓN  
**Complejidad:** 🟡 MEDIA-ALTA (Graph algorithms + ML adaptativo)  
**Prioridad:** 🟡 ALTA (Fase 2, Semanas 13-14)

---

*Generado: 2025-10-26*  
*Sistema Bitácora v1.0 - MTT-DSL Template: component_spec*
