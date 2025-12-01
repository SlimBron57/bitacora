# 🔍 QUERY LANGUAGE IMPLEMENTATION - PXQuery v1.5

**Ubicación:** `ROADMAP_V2/01_ARQUITECTURA/17_query-language-implementation.md`  
**Versión:** 1.5 - IMPLEMENTATION SPEC  
**Fecha:** 27 de Noviembre, 2025  
**Tipo:** IMPLEMENTATION (Código + Ejemplos)  
**Prerequisitos:** `15_pxlang-qpx-query-language.md`

---

## 🎯 PROPÓSITO

Especificación de implementación concreta de PXQuery: el lenguaje de consultas para Bitácora v1.5.

**Combina:**
- ✅ Symbolic queries (emoji operators)
- ✅ Natural language (conversational)
- ✅ SQL-like structure (familiar)
- ✅ Pixel-aware (geometric queries)

---

## 📖 SINTAXIS COMPLETA

### Operadores Básicos

```rust
pub enum QueryOperator {
    // 🔍 SEARCH - Búsqueda textual/semántica
    Search(String),           // 🔍 arquitectura
    
    // 🎯 FILTER - Filtros de pixels/metadata
    Filter(FilterExpr),       // 🎯 r>180 g<100
    
    // 📅 TEMPORAL - Rango temporal
    Temporal(TemporalRange),  // 📅 last:7days
    
    // 🌊 BRANCH - Contexto de branch
    Branch(BranchFilter),     // 🌊 project:bitacora
    
    // 📊 AGGREGATE - Agregaciones
    Aggregate(AggregateOp),   // 📊 count, sum, timeline
    
    // 🔗 ENTANGLEMENT - Navegar relaciones
    Entanglement(EntType),    // 🔗 causal, temporal, semantic
    
    // 🎨 PIXEL - Query geométrico en pixel space
    PixelQuery(PixelExpr),    // 🎨 near:(180,150,255)
    
    // ⚡ COMPOSE - Composición de queries
    Compose(Vec<QueryOperator>),
}
```

### Ejemplos de Queries

```rust
// === QUERY 1: Búsqueda simple ===
🔍 arquitectura

// Traducción:
QueryOperator::Search("arquitectura".into())

// SQL equivalente:
// SELECT * FROM memories WHERE text LIKE '%arquitectura%'

// === QUERY 2: Filtro por pixels ===
🔍 arquitectura 🎯 r>180

// Traducción:
QueryOperator::Compose(vec![
    QueryOperator::Search("arquitectura".into()),
    QueryOperator::Filter(FilterExpr::PixelChannel {
        channel: PixelChannel::R,
        op: ComparisonOp::GreaterThan,
        value: 180,
    }),
])

// SQL equivalente:
// SELECT * FROM memories 
// WHERE text LIKE '%arquitectura%' 
// AND pixel_r > 180

// === QUERY 3: Temporal + Branch ===
🔍 debugging 📅 last:7days 🌊 project:bitacora

// Traducción:
QueryOperator::Compose(vec![
    QueryOperator::Search("debugging".into()),
    QueryOperator::Temporal(TemporalRange::Last {
        amount: 7,
        unit: TimeUnit::Days,
    }),
    QueryOperator::Branch(BranchFilter::Project("bitacora".into())),
])

// === QUERY 4: Agregación ===
🔍 tasks 📊 count completed:true

// Traducción:
QueryOperator::Compose(vec![
    QueryOperator::Search("tasks".into()),
    QueryOperator::Aggregate(AggregateOp::Count {
        filter: Some("completed:true"),
    }),
])

// === QUERY 5: Navegación de entanglements ===
🔍 qpx_design 🔗 causal →

// Traducción:
QueryOperator::Compose(vec![
    QueryOperator::Search("qpx_design".into()),
    QueryOperator::Entanglement(EntType::Causal),
])

// Resultado: Todos los cores CAUSADOS por "qpx_design"

// === QUERY 6: Query geométrico en pixel space ===
🎨 near:(180,150,255) radius:20

// Traducción:
QueryOperator::PixelQuery(PixelExpr::Near {
    center: Pixel { r: 180, g: 150, b: 255, alpha: 255 },
    radius: 20.0,  // Distancia euclidiana
})

// Resultado: Todos los cores con pixels a distancia < 20
```

---

## 🔧 IMPLEMENTACIÓN RUST

### Parser

```rust
// src/query/parser.rs

pub struct PXQueryParser {
    tokenizer: EmojiTokenizer,
    natural_parser: NaturalLanguageParser,
}

impl PXQueryParser {
    pub fn parse(&self, query: &str) -> Result<Query> {
        // 1. Detectar si es symbolic o natural
        if self.is_symbolic(query) {
            self.parse_symbolic(query)
        } else {
            self.parse_natural(query)
        }
    }
    
    fn is_symbolic(&self, query: &str) -> bool {
        // Contiene emojis de operadores
        query.contains('🔍') || 
        query.contains('🎯') || 
        query.contains('📅') ||
        query.contains('🌊')
    }
    
    fn parse_symbolic(&self, query: &str) -> Result<Query> {
        let tokens = self.tokenizer.tokenize(query)?;
        
        // Ejemplo: "🔍 arquitectura 🎯 r>180 📅 last:7days"
        // Tokens: [Search("arquitectura"), Filter("r>180"), Temporal("last:7days")]
        
        let mut operators = Vec::new();
        
        for token in tokens {
            match token {
                Token::SearchEmoji => {
                    let term = self.extract_next_term(&tokens)?;
                    operators.push(QueryOperator::Search(term));
                }
                
                Token::FilterEmoji => {
                    let expr = self.parse_filter_expr(&tokens)?;
                    operators.push(QueryOperator::Filter(expr));
                }
                
                Token::TemporalEmoji => {
                    let range = self.parse_temporal_range(&tokens)?;
                    operators.push(QueryOperator::Temporal(range));
                }
                
                Token::BranchEmoji => {
                    let branch = self.parse_branch_filter(&tokens)?;
                    operators.push(QueryOperator::Branch(branch));
                }
                
                _ => {}
            }
        }
        
        Ok(Query {
            operators: QueryOperator::Compose(operators),
            mode: QueryMode::Symbolic,
        })
    }
    
    fn parse_natural(&self, query: &str) -> Result<Query> {
        // Ejemplo: "Muéstrame arquitectura de la semana pasada en proyecto bitácora"
        
        // 1. Extraer intención
        let intent = self.natural_parser.extract_intent(query)?;
        // intent = "show"
        
        // 2. Extraer entidades
        let entities = self.natural_parser.extract_entities(query)?;
        // entities = [
        //   ("topic", "arquitectura"),
        //   ("temporal", "semana pasada"),
        //   ("branch", "bitácora")
        // ]
        
        // 3. Convertir a QueryOperators
        let mut operators = vec![
            QueryOperator::Search("arquitectura".into()),
        ];
        
        if let Some(temporal) = entities.get("temporal") {
            operators.push(QueryOperator::Temporal(
                self.parse_temporal_natural(temporal)?
            ));
        }
        
        if let Some(branch) = entities.get("branch") {
            operators.push(QueryOperator::Branch(
                BranchFilter::Project(branch.clone())
            ));
        }
        
        Ok(Query {
            operators: QueryOperator::Compose(operators),
            mode: QueryMode::Natural,
        })
    }
}
```

### Executor

```rust
// src/query/executor.rs

pub struct QueryExecutor {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    primitives_db: Arc<SqliteConnection>,
}

impl QueryExecutor {
    pub async fn execute(&self, query: Query) -> Result<QueryResult> {
        match query.operators {
            QueryOperator::Search(term) => {
                self.execute_search(&term).await
            }
            
            QueryOperator::Compose(ops) => {
                self.execute_composed(&ops).await
            }
            
            _ => Err("Unsupported operator".into()),
        }
    }
    
    async fn execute_composed(&self, ops: &[QueryOperator]) -> Result<QueryResult> {
        // Pipeline de operadores
        let mut results = QueryResult::empty();
        
        for op in ops {
            results = match op {
                QueryOperator::Search(term) => {
                    self.execute_search(term).await?
                }
                
                QueryOperator::Filter(expr) => {
                    self.apply_filter(&results, expr).await?
                }
                
                QueryOperator::Temporal(range) => {
                    self.apply_temporal_filter(&results, range).await?
                }
                
                QueryOperator::Branch(branch) => {
                    self.apply_branch_filter(&results, branch).await?
                }
                
                QueryOperator::Aggregate(agg) => {
                    return self.apply_aggregation(&results, agg).await;
                }
                
                _ => results,
            };
        }
        
        Ok(results)
    }
    
    async fn execute_search(&self, term: &str) -> Result<QueryResult> {
        // 1. Búsqueda en primitivos (SQLite)
        let primitive_results = sqlx::query_as::<_, PrimitiveResult>(
            "SELECT * FROM memories WHERE text LIKE ?"
        )
        .bind(format!("%{}%", term))
        .fetch_all(&*self.primitives_db)
        .await?;
        
        // 2. Búsqueda semántica en TelescopeDB (pixels)
        let semantic_results = self.telescope_db
            .semantic_search(term, 10)
            .await?;
        
        // 3. Búsqueda en VoxelDB (templates)
        let template_results = self.voxel_db
            .search_templates(term)
            .await?;
        
        // 4. Combinar y rankear
        Ok(QueryResult {
            primitive_matches: primitive_results,
            semantic_matches: semantic_results,
            template_matches: template_results,
            total_count: primitive_results.len() + semantic_results.len(),
        })
    }
    
    async fn apply_filter(&self, results: &QueryResult, expr: &FilterExpr) -> Result<QueryResult> {
        match expr {
            FilterExpr::PixelChannel { channel, op, value } => {
                // Filtrar por canal de pixel
                let filtered = results.semantic_matches.iter()
                    .filter(|core| {
                        let pixel_value = match channel {
                            PixelChannel::R => core.pixels[0].r,
                            PixelChannel::G => core.pixels[0].g,
                            PixelChannel::B => core.pixels[0].b,
                            PixelChannel::Alpha => core.pixels[0].alpha,
                        };
                        
                        match op {
                            ComparisonOp::GreaterThan => pixel_value > *value,
                            ComparisonOp::LessThan => pixel_value < *value,
                            ComparisonOp::Equal => pixel_value == *value,
                        }
                    })
                    .cloned()
                    .collect();
                
                Ok(QueryResult {
                    semantic_matches: filtered,
                    ..results.clone()
                })
            }
            
            _ => Ok(results.clone()),
        }
    }
    
    async fn apply_temporal_filter(&self, results: &QueryResult, range: &TemporalRange) -> Result<QueryResult> {
        let (start, end) = range.to_timestamps()?;
        
        let filtered = results.semantic_matches.iter()
            .filter(|core| {
                core.timestamp >= start && core.timestamp <= end
            })
            .cloned()
            .collect();
        
        Ok(QueryResult {
            semantic_matches: filtered,
            ..results.clone()
        })
    }
}
```

---

## 🎨 EJEMPLOS DE USO

### Caso 1: Usuario en Terraza (Domingo)

```rust
// Usuario pregunta en natural language
let query = "¿Qué hicimos el martes con la arquitectura?";

// Parser detecta: Natural language
let parsed = parser.parse(query)?;
// Query {
//     operators: Compose([
//         Search("arquitectura"),
//         Temporal(Date("2025-11-26")),  // "martes" → fecha
//     ]),
//     mode: Natural,
// }

// Executor corre el query
let results = executor.execute(parsed).await?;

// Resultados:
// - 23 FBCU Cores del martes
// - 3 Git commits
// - 15 archivos modificados
// - 8 templates aplicados
```

### Caso 2: Query Técnico (Symbolic)

```rust
// Usuario experto usa symbolic query
let query = "🔍 qpx 🎯 r>180 📅 last:30days 🌊 project:bitacora 📊 timeline";

// Parser detecta: Symbolic
let parsed = parser.parse(query)?;
// Query {
//     operators: Compose([
//         Search("qpx"),
//         Filter(PixelChannel::R > 180),
//         Temporal(Last { 30, Days }),
//         Branch(Project("bitacora")),
//         Aggregate(Timeline),
//     ]),
//     mode: Symbolic,
// }

// Executor corre el query
let results = executor.execute(parsed).await?;

// Resultado: Timeline visual de todas las menciones de "qpx"
// en último mes, solo en project:bitacora, con r>180 (técnicas)
```

### Caso 3: Query Primitivos (Simple)

```rust
// Usuario busca tasks completadas
let query = "🔍 tasks 📊 count completed:true";

// Parser:
let parsed = parser.parse(query)?;

// Executor:
let results = executor.execute(parsed).await?;

// Resultado:
// QueryResult {
//     aggregate: Some(AggregateResult::Count(42)),
//     message: "42 tasks completed",
// }
```

### Caso 4: Navegación de Entanglements

```rust
// Usuario explora relaciones causales
let query = "🔍 qpx_design 🔗 causal → 🎯 alpha>0.8";

// Parser:
let parsed = parser.parse(query)?;
// Query {
//     operators: Compose([
//         Search("qpx_design"),
//         Entanglement(Causal, Direction::Forward),
//         Filter(Alpha > 0.8),
//     ]),
// }

// Executor:
let results = executor.execute(parsed).await?;

// Resultado:
// - Core: "Update 01_sistema-dual-databases.md" (alpha=0.9)
// - Core: "Create 14_qpx-revolucion.md" (alpha=1.0)
// - Core: "Git commit xyz789" (alpha=0.85)
```

---

## 📊 PERFORMANCE

### Query Plan Optimization

```rust
pub struct QueryPlanner {
    statistics: Arc<StorageStatistics>,
}

impl QueryPlanner {
    pub fn optimize(&self, query: Query) -> OptimizedQuery {
        // 1. Reordenar operadores para máxima eficiencia
        let optimized_ops = self.reorder_operators(&query.operators);
        
        // 2. Determinar índices a usar
        let indices = self.select_indices(&optimized_ops);
        
        // 3. Estimar costo
        let cost = self.estimate_cost(&optimized_ops, &indices);
        
        OptimizedQuery {
            operators: optimized_ops,
            indices,
            estimated_cost: cost,
        }
    }
    
    fn reorder_operators(&self, ops: &QueryOperator) -> QueryOperator {
        // Reglas de optimización:
        // 1. Filtros temporales primero (reduce dataset rápido)
        // 2. Filtros de branch después (reduce aún más)
        // 3. Búsqueda textual después
        // 4. Filtros de pixels al final (más costosos)
        // 5. Agregaciones al final siempre
        
        match ops {
            QueryOperator::Compose(operators) => {
                let mut sorted = operators.clone();
                sorted.sort_by_key(|op| self.operator_cost(op));
                QueryOperator::Compose(sorted)
            }
            _ => ops.clone(),
        }
    }
    
    fn operator_cost(&self, op: &QueryOperator) -> u32 {
        match op {
            QueryOperator::Temporal(_) => 10,      // Muy rápido (índice)
            QueryOperator::Branch(_) => 20,        // Rápido (índice)
            QueryOperator::Search(_) => 50,        // Medio (full-text search)
            QueryOperator::Filter(_) => 100,       // Costoso (scan pixels)
            QueryOperator::Aggregate(_) => 1000,   // Muy costoso (siempre al final)
            _ => 500,
        }
    }
}
```

---

## 🚀 INTEGRACIÓN CON UI

### ShuiDao Natural Language

```rust
// Usuario habla naturalmente
"Cuéntame sobre debugging"

// ShuiDao convierte a PXQuery
🔍 debugging 📅 last:30days 🌊 current_branch

// Ejecuta y presenta resultados conversacionalmente
"Encontré 12 memorias sobre debugging en los últimos 30 días.
Las más relevantes:
1. [Alpha=0.9] Debuggeaste panic en TelescopeDB (hace 3 días)
2. [Alpha=0.85] Resolviste lifetime error (hace 1 semana)
3. [Alpha=0.7] Crisis con CBOR serialization (hace 2 semanas)

¿Quieres profundizar en alguna?"
```

### CLI Power Users

```bash
# Symbolic queries directas
$ bitacora query "🔍 arquitectura 🎯 r>180 📅 last:7days"

# Natural language
$ bitacora search "decisiones arquitectónicas de la semana pasada"

# Pipeline con pipes
$ bitacora query "🔍 qpx" | bitacora filter "🎯 alpha>0.8" | bitacora timeline
```

---

## 💎 CONCLUSIÓN

**Query Language tiene 3 niveles:**

1. **Symbolic (PXQuery):** Para power users
   - `🔍 arquitectura 🎯 r>180 📅 last:7days`
   - Rápido, preciso, componible

2. **Natural:** Para usuarios normales
   - "¿Qué hicimos el martes con arquitectura?"
   - Conversacional, intuitivo

3. **Programmatic (Rust API):** Para código
   - `QueryOperator::Compose([...])`
   - Máxima flexibilidad

**Todos convergen al mismo executor.**

---

*PXQuery v1.5 - "Pixel-aware queries para organismo cuántico"*
