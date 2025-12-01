```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/02_COMPONENTES/15_pxlang-symbolic-engine.md
Versión: 1.5
Fecha Creación: 2025-11-27
Última Actualización: 2025-11-27
Autor: Sistema Bitácora - Arquitectura QPX v1.5 (documento reescrito desde cero)
Propósito: PXLang como lenguaje de query simbólico 100% integrado con TelescopeDB + VoxelDB
Estado: 📋 ESPECIFICACIÓN v1.5 - Symbolic Revolution
Relacionado Con: 14_qpx-quantumdao-revolucion.md, 05_telescopedb.md, 06_voxeldb.md, 13_shuidao-cognitive-engine.md
Implementa: DA-035 (PXLang query engine), DA-036 (Symbolic query optimization)
Backup: 15_pxlang-symbolic-engine.md.backup_v1.0 (arquitectura anterior para referencia)
# === FIN DATOS DE AUDITORÍA ===
```

# 🔮 PXLANG v1.5 - Motor Simbólico Cuántico

> **"No es un query language. Es una forma de conversar con tu memoria cuántica."**

---

## 📚 TABLA DE CONTENIDOS

1. [Propósito](#propósito)
2. [PXQuery: 3 Modos de Query](#pxquery-3-modos-de-query)
3. [Symbolic Mode](#symbolic-mode)
4. [Natural Mode](#natural-mode)
5. [Programmatic Mode](#programmatic-mode)
6. [Query Optimizer](#query-optimizer)
7. [Integration con TelescopeDB + VoxelDB](#integration-con-telescopedb--voxeldb)
8. [API Principal](#api-principal)
9. [Casos de Uso](#casos-de-uso)
10. [Performance Targets](#performance-targets)

---

## 🎯 PROPÓSITO

### ¿Qué es PXLang v1.5?

**PXLang** es el **lenguaje de query simbólico** de Bitácora que permite consultar memoria biográfica y semántica de forma natural:

1. **3 modos de query** (Symbolic, Natural, Programmatic)
2. **Query optimization** automática
3. **Integración dual** (TelescopeDB + VoxelDB)
4. **Symbolic reasoning** sobre QuantumCores
5. **Entanglement traversal** inteligente
6. **Context-aware** (usa CTX7D para enriquecer queries)

### ¿Qué Problema Resuelve?

**Problema clásico:**
```
Usuario: "Busca cuando estaba frustrado debuggeando Rust ownership hace 2 semanas"

SQL tradicional:
❌ SELECT * FROM memories WHERE text LIKE '%ownership%' AND date > '2025-11-13'
❌ No captura "frustrado"
❌ No entiende "debuggeando"
❌ No relaciona conceptos semánticos
```

**Solución PXLang v1.5:**
```rust
// MODO 1: Symbolic Query
let query = pxlang.parse_symbolic(r#"
    FIND cores
    WHERE emotional.valence < 0 AND emotional.frustration > 0.6
    AND topic IN ["Rust", "ownership", "debugging"]
    AND temporal.age < 14.days()
"#)?;

// MODO 2: Natural Query
let query = pxlang.parse_natural(
    "cuando estaba frustrado debuggeando Rust ownership hace 2 semanas"
)?;

// MODO 3: Programmatic Query
let query = PXQuery::builder()
    .emotional_state(EmotionalFilter {
        valence_range: (-1.0, 0.0),
        frustration_min: 0.6,
    })
    .topics(vec!["Rust", "ownership", "debugging"])
    .time_range(14.days())
    .build();

// Ejecutar (todos los modos producen mismo resultado)
let results = pxlang.execute(&query).await?;

// Resultado:
// ✅ Encuentra cores con estado emocional frustrado
// ✅ Filtra por topics semánticos (no keywords literales)
// ✅ Range temporal inteligente
// ✅ <100ms execution time
```

### Los 5 Imposibles que PXLang v1.5 Logra

```rust
/// IMPOSIBLE #1: Query Multi-Modal (3 sintaxis → mismo resultado)
pub enum PXQuery {
    Symbolic(SymbolicQuery),
    Natural(NaturalQuery),
    Programmatic(ProgrammaticQuery),
}

/// IMPOSIBLE #2: Query Optimization Automática
pub struct QueryOptimizer {
    pub rewrite_rules: Vec<RewriteRule>,
    pub cost_estimator: CostEstimator,
    pub execution_planner: ExecutionPlanner,
}

/// IMPOSIBLE #3: Dual-DB Execution (TelescopeDB + VoxelDB)
pub struct QueryExecutor {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    query_router: QueryRouter,  // Decide qué DB usar
}

/// IMPOSIBLE #4: Symbolic Reasoning sobre QuantumCores
pub struct SymbolicReasoner {
    pub entanglement_traversal: EntanglementTraverser,
    pub topic_inference: TopicInferencer,
    pub emotional_analysis: EmotionalAnalyzer,
}

/// IMPOSIBLE #5: Context-Aware Query Enhancement
pub struct ContextEnhancer {
    pub ctx7d: Arc<ContextToken7D>,
    pub query_expander: QueryExpander,
}
```

---

## 🔮 PXQUERY: 3 MODOS DE QUERY

### Overview de Modos

```rust
/// PXQuery = Union de 3 modos
pub enum PXQuery {
    Symbolic(SymbolicQuery),      // SQL-like
    Natural(NaturalQuery),         // Natural language
    Programmatic(ProgrammaticQuery), // Builder pattern
}

impl PXQuery {
    /// Parse desde cualquier modo
    pub fn parse(input: &str, mode: QueryMode) -> Result<Self> {
        match mode {
            QueryMode::Symbolic => Self::parse_symbolic(input),
            QueryMode::Natural => Self::parse_natural(input),
            QueryMode::Programmatic => Err(PXError::UseBuilder), // Use builder()
        }
    }
    
    /// Auto-detect mode
    pub fn parse_auto(input: &str) -> Result<Self> {
        // Si empieza con FIND/SELECT → Symbolic
        if input.trim_start().to_uppercase().starts_with("FIND") ||
           input.trim_start().to_uppercase().starts_with("SELECT") {
            return Self::parse_symbolic(input);
        }
        
        // Default: Natural
        Self::parse_natural(input)
    }
}
```

---

## 📜 SYMBOLIC MODE

### Sintaxis SQL-like para Memory Queries

```rust
/// SymbolicQuery = SQL-like syntax
pub struct SymbolicQuery {
    pub select: SelectClause,
    pub where_clause: Option<WhereClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<usize>,
}

/// Parser para Symbolic queries
pub struct SymbolicQueryParser {
    lexer: Lexer,
}

impl SymbolicQueryParser {
    pub fn parse(&mut self, input: &str) -> Result<SymbolicQuery> {
        // 1. Tokenize
        let tokens = self.lexer.tokenize(input)?;
        
        // 2. Parse SELECT/FIND
        let select = self.parse_select(&tokens)?;
        
        // 3. Parse WHERE
        let where_clause = self.parse_where(&tokens)?;
        
        // 4. Parse ORDER BY
        let order_by = self.parse_order_by(&tokens)?;
        
        // 5. Parse LIMIT
        let limit = self.parse_limit(&tokens)?;
        
        Ok(SymbolicQuery {
            select,
            where_clause,
            order_by,
            limit,
        })
    }
}
```

### Ejemplos de Symbolic Queries

```rust
// Ejemplo 1: Buscar por estado emocional
let query = pxlang.parse_symbolic(r#"
    FIND cores
    WHERE emotional.frustration > 0.7
    AND temporal.age < 30.days()
    ORDER BY timestamp DESC
    LIMIT 10
"#)?;

// Ejemplo 2: Buscar por proyecto
let query = pxlang.parse_symbolic(r#"
    FIND cores
    WHERE branch = "Implementar TelescopeDB"
    AND progress < 1.0
"#)?;

// Ejemplo 3: Buscar por topic + emotional state
let query = pxlang.parse_symbolic(r#"
    FIND cores
    WHERE topic IN ["Rust", "ownership"]
    AND emotional.valence < 0
    AND spherical.r > 0.8
"#)?;

// Ejemplo 4: Entanglement traversal
let query = pxlang.parse_symbolic(r#"
    FIND cores
    WHERE core_id = "abc123"
    TRAVERSE entanglements(type=Causal, depth=2)
"#)?;

// Ejemplo 5: Dual-DB query (TelescopeDB + VoxelDB)
let query = pxlang.parse_symbolic(r#"
    FIND cores
    SIMILAR TO "concurrency en Rust"
    WHERE alpha = 255
    LIMIT 5
"#)?;
```

### Sintaxis Completa

```
SYMBOLIC QUERY GRAMMAR:

query := SELECT_CLAUSE [WHERE_CLAUSE] [ORDER_BY_CLAUSE] [LIMIT_CLAUSE]

SELECT_CLAUSE :=
    | FIND cores
    | FIND voxels
    | FIND projects
    | FIND topics

WHERE_CLAUSE :=
    | WHERE condition
    | WHERE condition AND condition
    | WHERE condition OR condition

condition :=
    | field_path comparator value
    | field_path IN list
    | SIMILAR TO string
    | TRAVERSE entanglements(params)

field_path :=
    | emotional.valence
    | emotional.frustration
    | temporal.age
    | temporal.timestamp
    | spherical.r / .theta / .phi
    | topic
    | branch
    | progress
    | alpha
    | core_id

comparator := = | != | < | > | <= | >=

ORDER_BY_CLAUSE := ORDER BY field_path [ASC|DESC]

LIMIT_CLAUSE := LIMIT integer

TRAVERSE_CLAUSE := TRAVERSE entanglements(type=EnumType, depth=integer)
```

---

## 💬 NATURAL MODE

### Natural Language Queries con Symbol Table

```rust
/// NaturalQuery = Parsed from natural language
pub struct NaturalQuery {
    pub intent: QueryIntent,
    pub filters: Vec<QueryFilter>,
    pub constraints: Vec<QueryConstraint>,
}

pub enum QueryIntent {
    FindMemories,
    FindSimilar,
    FindProjects,
    FindTopics,
    TraverseRelations,
}

pub enum QueryFilter {
    EmotionalState(EmotionalFilter),
    TimeRange(TimeFilter),
    Topic(TopicFilter),
    Branch(BranchFilter),
    Similarity(SimilarityFilter),
}

/// Parser para Natural queries CON SYMBOL TABLE
pub struct NaturalQueryParser {
    symbol_table: Arc<SymbolTable>,  // PX-Core-256 + user symbols
    embedding_service: Arc<EmbeddingService>,  // Fallback para queries complejos
}

impl NaturalQueryParser {
    pub async fn parse(&self, input: &str) -> Result<NaturalQuery> {
        // FAST PATH: Pattern matching con symbol table (80% queries)
        if let Some(query) = self.try_symbol_table_patterns(input) {
            return Ok(query);  // <20ms ✅
        }
        
        // SLOW PATH: Embedding similarity para queries complejos (20%)
        self.parse_with_embeddings(input).await  // <150ms
    }
    
    /// FAST PATH: Pattern matching con Symbol Table
    fn try_symbol_table_patterns(&self, input: &str) -> Option<NaturalQuery> {
        let input_lower = input.to_lowercase();
        let mut filters = Vec::new();
        
        // 1. EMOTIONAL entities (usando symbol table)
        if let Some(symbols) = self.symbol_table.match_emotional_keywords(&input_lower) {
            // "frustrado" → 😔 (symbol ID: 0x01) → EmotionalFilter
            filters.push(QueryFilter::EmotionalState(EmotionalFilter {
                valence_range: symbols.valence_range,
                frustration_min: symbols.frustration,
            }));
        }
        
        // 2. TEMPORAL entities (regex patterns cacheados)
        if let Some(time_filter) = self.extract_time_fast(&input_lower) {
            filters.push(QueryFilter::TimeRange(time_filter));
        }
        
        // 3. TOPIC entities (usando symbol table)
        if let Some(topics) = self.symbol_table.match_topic_keywords(&input_lower) {
            // "trabajo" → 💼 (symbol ID: 0x02) → TopicFilter
            filters.push(QueryFilter::Topic(TopicFilter {
                topics: topics.symbol_ids,
            }));
        }
        
        // 4. Si encontramos filtros, construir query
        if !filters.is_empty() {
            return Some(NaturalQuery {
                intent: QueryIntent::FindMemories,
                filters,
                constraints: vec![],
            });
        }
        
        None
    }
    
    /// SLOW PATH: Embeddings para queries complejos
    async fn parse_with_embeddings(&self, input: &str) -> Result<NaturalQuery> {
        // Solo para queries que no matchean symbol table
        let embedding = self.embedding_service.embed(input).await?;
        
        // Buscar símbolos similares en VoxelDB
        let similar_symbols = self.voxel_db.query_symbols(&embedding).await?;
        
        // Construir filters desde símbolos similares
        let filters = self.symbols_to_filters(similar_symbols)?;
        
        Ok(NaturalQuery {
            intent: QueryIntent::FindMemories,
            filters,
            constraints: vec![],
        })
    }
    
    /// Extraer tiempo FAST (regex precompilados)
    fn extract_time_fast(&self, text: &str) -> Option<TimeFilter> {
        // Patterns precompilados (lazy_static)
        static TEMPORAL_PATTERNS: Lazy<Vec<(Regex, TimeExtractor)>> = Lazy::new(|| {
            vec![
                (Regex::new(r"hace (\d+) (día|días)").unwrap(), TimeExtractor::Days),
                (Regex::new(r"hace (\d+) (semana|semanas)").unwrap(), TimeExtractor::Weeks),
                (Regex::new(r"hace (\d+) (mes|meses)").unwrap(), TimeExtractor::Months),
                (Regex::new(r"esta semana").unwrap(), TimeExtractor::ThisWeek),
                (Regex::new(r"hoy").unwrap(), TimeExtractor::Today),
            ]
        });
        
        for (pattern, extractor) in TEMPORAL_PATTERNS.iter() {
            if let Some(caps) = pattern.captures(text) {
                return Some(extractor.extract(&caps));
            }
        }
        
        None
    }
}

/// Symbol Table = PX-Core-256 + user-specific symbols
pub struct SymbolTable {
    // Core 256 símbolos universales
    core_symbols: HashMap<String, Symbol>,
    
    // Símbolos aprendidos del usuario
    user_symbols: HashMap<String, Symbol>,
    
    // Cache de keywords → symbols
    keyword_to_symbol: HashMap<String, Vec<SymbolId>>,
}

pub struct Symbol {
    pub id: SymbolId,              // 0x01, 0x02, ...
    pub unicode: char,             // 😔, 💼, ...
    pub semantic_category: SymbolCategory,
    pub keywords: Vec<String>,      // ["frustrado", "molesto", "enojado"]
    pub emotional_valence: Option<(f64, f64)>, // (-1.0, -0.4)
}

pub enum SymbolCategory {
    Emotion,      // 😔, 😊, 😰
    Context,      // 💼 (trabajo), 🏠 (casa), 🎓 (estudio)
    Action,       // 🚶‍♂️ (reflexionar), ⚡ (decisión)
    Transition,   // →, ↔, ↺
    Resolution,   // 😌, 🎉, 💔
}

impl SymbolTable {
    /// Match emotional keywords → symbols
    pub fn match_emotional_keywords(&self, text: &str) -> Option<EmotionalMatch> {
        for keyword in ["frustrado", "molesto", "enojado", "triste", "feliz", "contento"] {
            if text.contains(keyword) {
                if let Some(symbols) = self.keyword_to_symbol.get(keyword) {
                    return Some(self.build_emotional_match(symbols));
                }
            }
        }
        None
    }
    
    /// Match topic keywords → symbols
    pub fn match_topic_keywords(&self, text: &str) -> Option<TopicMatch> {
        for keyword in ["trabajo", "casa", "estudio", "familia", "proyecto"] {
            if text.contains(keyword) {
                if let Some(symbols) = self.keyword_to_symbol.get(keyword) {
                    return Some(TopicMatch {
                        symbol_ids: symbols.clone(),
                    });
                }
            }
        }
        None
    }
}

pub struct EmotionalMatch {
    pub valence_range: (f64, f64),
    pub frustration: f64,
}

pub struct TopicMatch {
    pub symbol_ids: Vec<SymbolId>,
}

pub type SymbolId = u16;

enum TimeExtractor {
    Days,
    Weeks,
    Months,
    ThisWeek,
    Today,
}

impl TimeExtractor {
    fn extract(&self, caps: &regex::Captures) -> TimeFilter {
        match self {
            TimeExtractor::Days => {
                let days: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                TimeFilter::Relative(Duration::days(days))
            }
            TimeExtractor::Weeks => {
                let weeks: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                TimeFilter::Relative(Duration::weeks(weeks))
            }
            TimeExtractor::Months => {
                let months: i64 = caps.get(1).unwrap().as_str().parse().unwrap();
                TimeFilter::Relative(Duration::days(months * 30))
            }
            TimeExtractor::ThisWeek => TimeFilter::Relative(Duration::weeks(1)),
            TimeExtractor::Today => TimeFilter::Relative(Duration::days(1)),
        }
    }
}
```

### Ejemplos de Natural Queries

```rust
// Ejemplo 1: Temporal + emotional
let query = pxlang.parse_natural(
    "busca cuando estaba frustrado hace 2 semanas"
).await?;

// Se traduce a:
// WHERE emotional.frustration > 0.6
// AND temporal.age < 14.days()

// Ejemplo 2: Topic-based
let query = pxlang.parse_natural(
    "encuentra conversaciones sobre Rust ownership"
).await?;

// Se traduce a:
// SIMILAR TO "Rust ownership"
// (usa VoxelDB para búsqueda semántica)

// Ejemplo 3: Project-based
let query = pxlang.parse_natural(
    "muéstrame el progreso del proyecto de instalar switch"
).await?;

// Se traduce a:
// WHERE branch LIKE "%instalar switch%"

// Ejemplo 4: Relational traversal
let query = pxlang.parse_natural(
    "qué está relacionado con mi última conversación sobre debugging"
).await?;

// Se traduce a:
// FIND cores WHERE topic = "debugging" ORDER BY timestamp DESC LIMIT 1
// TRAVERSE entanglements(type=Causal, depth=2)
```

---

## 💻 PROGRAMMATIC MODE

### Builder Pattern para Query Construction

```rust
/// ProgrammaticQuery = Builder pattern
pub struct ProgrammaticQuery {
    filters: Vec<Filter>,
    order: Option<OrderBy>,
    limit: Option<usize>,
}

impl ProgrammaticQuery {
    pub fn builder() -> ProgrammaticQueryBuilder {
        ProgrammaticQueryBuilder::new()
    }
}

pub struct ProgrammaticQueryBuilder {
    filters: Vec<Filter>,
    order: Option<OrderBy>,
    limit: Option<usize>,
}

impl ProgrammaticQueryBuilder {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            order: None,
            limit: None,
        }
    }
    
    /// Filtrar por estado emocional
    pub fn emotional_state(mut self, filter: EmotionalFilter) -> Self {
        self.filters.push(Filter::Emotional(filter));
        self
    }
    
    /// Filtrar por range temporal
    pub fn time_range(mut self, duration: Duration) -> Self {
        self.filters.push(Filter::TimeRange(duration));
        self
    }
    
    /// Filtrar por topics
    pub fn topics(mut self, topics: Vec<&str>) -> Self {
        self.filters.push(Filter::Topics(topics.iter().map(|s| s.to_string()).collect()));
        self
    }
    
    /// Filtrar por branch (proyecto)
    pub fn branch(mut self, branch_id: BranchId) -> Self {
        self.filters.push(Filter::Branch(branch_id));
        self
    }
    
    /// Filtrar por alpha channel (origen)
    pub fn alpha(mut self, alpha: u8) -> Self {
        self.filters.push(Filter::Alpha(alpha));
        self
    }
    
    /// Filtrar por coordenadas esféricas
    pub fn spherical(mut self, center: SphericalCoords, radius: f64) -> Self {
        self.filters.push(Filter::Spherical { center, radius });
        self
    }
    
    /// Búsqueda semántica por similitud
    pub fn similar_to(mut self, text: &str) -> Self {
        self.filters.push(Filter::SimilarTo(text.to_string()));
        self
    }
    
    /// Ordenar resultados
    pub fn order_by(mut self, field: OrderField, direction: OrderDirection) -> Self {
        self.order = Some(OrderBy { field, direction });
        self
    }
    
    /// Limitar resultados
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Build final query
    pub fn build(self) -> ProgrammaticQuery {
        ProgrammaticQuery {
            filters: self.filters,
            order: self.order,
            limit: self.limit,
        }
    }
}

pub enum Filter {
    Emotional(EmotionalFilter),
    TimeRange(Duration),
    Topics(Vec<String>),
    Branch(BranchId),
    Alpha(u8),
    Spherical { center: SphericalCoords, radius: f64 },
    SimilarTo(String),
}

pub struct EmotionalFilter {
    pub valence_range: (f64, f64),
    pub frustration_min: f64,
}

pub struct OrderBy {
    pub field: OrderField,
    pub direction: OrderDirection,
}

pub enum OrderField {
    Timestamp,
    Intensity,
    Similarity,
}

pub enum OrderDirection {
    Asc,
    Desc,
}
```

### Ejemplos de Programmatic Queries

```rust
// Ejemplo 1: Emotional + temporal
let query = PXQuery::builder()
    .emotional_state(EmotionalFilter {
        valence_range: (-1.0, 0.0),
        frustration_min: 0.6,
    })
    .time_range(Duration::days(14))
    .order_by(OrderField::Timestamp, OrderDirection::Desc)
    .limit(10)
    .build();

// Ejemplo 2: Topic-based semantic search
let query = PXQuery::builder()
    .similar_to("Rust ownership debugging")
    .alpha(255)  // Solo native
    .limit(5)
    .build();

// Ejemplo 3: Project progress
let query = PXQuery::builder()
    .branch(branch_id)
    .order_by(OrderField::Timestamp, OrderDirection::Asc)
    .build();

// Ejemplo 4: Spherical search
let query = PXQuery::builder()
    .spherical(
        SphericalCoords { r: 0.8, theta: 0.3, phi: 1.2 },
        0.2,  // radius
    )
    .time_range(Duration::days(30))
    .limit(20)
    .build();
```

---

## ⚡ QUERY OPTIMIZER

### Automatic Query Optimization

```rust
/// QueryOptimizer = Optimiza queries antes de ejecución
pub struct QueryOptimizer {
    rewrite_rules: Vec<RewriteRule>,
    cost_estimator: CostEstimator,
}

impl QueryOptimizer {
    pub fn optimize(&self, query: PXQuery) -> OptimizedQuery {
        // 1. Rewrite query (aplicar reglas de optimización)
        let rewritten = self.apply_rewrite_rules(query);
        
        // 2. Estimate cost de diferentes planes
        let plans = self.generate_execution_plans(&rewritten);
        let costs: Vec<_> = plans.iter()
            .map(|plan| self.cost_estimator.estimate(plan))
            .collect();
        
        // 3. Seleccionar plan con menor costo
        let best_idx = costs.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        
        OptimizedQuery {
            original: rewritten,
            execution_plan: plans[best_idx].clone(),
            estimated_cost: costs[best_idx],
        }
    }
    
    /// Aplicar reglas de rewrite
    fn apply_rewrite_rules(&self, mut query: PXQuery) -> PXQuery {
        for rule in &self.rewrite_rules {
            query = rule.apply(query);
        }
        query
    }
}

/// Reglas de optimización
pub enum RewriteRule {
    /// Combinar filtros redundantes
    MergeFilters,
    
    /// Pushdown de filtros (aplicar filtros antes de joins)
    FilterPushdown,
    
    /// Convertir SIMILAR TO en búsqueda HNSW
    SimilarityToHNSW,
    
    /// Usar índice esférico cuando sea posible
    UseSphericalIndex,
    
    /// Limitar traversal depth
    LimitTraversalDepth,
}

impl RewriteRule {
    pub fn apply(&self, query: PXQuery) -> PXQuery {
        match self {
            RewriteRule::MergeFilters => {
                // Combinar filtros temporales: hace 1 semana + hace 2 días → hace 1 semana
                query
            }
            RewriteRule::FilterPushdown => {
                // Aplicar filtros baratos (alpha, branch) antes de caros (similarity)
                query
            }
            RewriteRule::SimilarityToHNSW => {
                // Convertir SIMILAR TO en búsqueda HNSW optimizada
                query
            }
            RewriteRule::UseSphericalIndex => {
                // Si hay filtro emocional, usar índice esférico
                query
            }
            RewriteRule::LimitTraversalDepth => {
                // Limitar depth de entanglement traversal a 3 por defecto
                query
            }
        }
    }
}

/// Cost estimator
pub struct CostEstimator {
    telescope_stats: Arc<TelescopeDBStats>,
    voxel_stats: Arc<VoxelDBStats>,
}

impl CostEstimator {
    pub fn estimate(&self, plan: &ExecutionPlan) -> f64 {
        let mut cost = 0.0;
        
        for step in &plan.steps {
            cost += match step {
                ExecutionStep::FilterByAlpha => 10.0,  // Muy barato
                ExecutionStep::FilterByBranch => 20.0,
                ExecutionStep::FilterByTimeRange => 50.0,
                ExecutionStep::SphericalSearch => 100.0,
                ExecutionStep::HNSWSearch => 150.0,  // Moderado
                ExecutionStep::EntanglementTraversal(depth) => 200.0 * (*depth as f64),
                ExecutionStep::FullScan => 1000.0,  // Muy caro
            };
        }
        
        cost
    }
}

pub struct OptimizedQuery {
    pub original: PXQuery,
    pub execution_plan: ExecutionPlan,
    pub estimated_cost: f64,
}

pub struct ExecutionPlan {
    pub steps: Vec<ExecutionStep>,
}

pub enum ExecutionStep {
    FilterByAlpha,
    FilterByBranch,
    FilterByTimeRange,
    SphericalSearch,
    HNSWSearch,
    EntanglementTraversal(usize),  // depth
    FullScan,
}
```

---

## 🔗 INTEGRATION CON TELESCOPEDB + VOXELDB

### Dual-DB Query Execution

```rust
/// QueryExecutor = Ejecuta queries optimizadas
pub struct QueryExecutor {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    optimizer: QueryOptimizer,
}

impl QueryExecutor {
    pub async fn execute(&self, query: PXQuery) -> Result<QueryResult> {
        // 1. Optimize query
        let optimized = self.optimizer.optimize(query);
        
        // 2. Route a DB apropiado
        let results = match &optimized.original {
            PXQuery::Symbolic(sq) => self.execute_symbolic(sq, &optimized.execution_plan).await?,
            PXQuery::Natural(nq) => self.execute_natural(nq, &optimized.execution_plan).await?,
            PXQuery::Programmatic(pq) => self.execute_programmatic(pq, &optimized.execution_plan).await?,
        };
        
        Ok(QueryResult {
            cores: results,
            execution_time: optimized.estimated_cost,
        })
    }
    
    /// Ejecutar symbolic query
    async fn execute_symbolic(
        &self,
        query: &SymbolicQuery,
        plan: &ExecutionPlan,
    ) -> Result<Vec<QuantumCore>> {
        let mut candidates = Vec::new();
        
        for step in &plan.steps {
            match step {
                ExecutionStep::FilterByAlpha => {
                    // Filtrar por alpha en TelescopeDB
                    let alpha = self.extract_alpha_filter(query)?;
                    candidates = self.telescope_db.query_by_alpha(alpha).await?;
                }
                ExecutionStep::SphericalSearch => {
                    // Búsqueda esférica en TelescopeDB
                    let (center, radius) = self.extract_spherical_filter(query)?;
                    candidates = self.telescope_db.query_by_spherical(center, radius).await?;
                }
                ExecutionStep::HNSWSearch => {
                    // Búsqueda semántica en VoxelDB
                    let text = self.extract_similarity_filter(query)?;
                    let embedding = self.embedding_service.embed(&text).await?;
                    let voxels = self.voxel_db.query(&VoxelQuery {
                        embedding,
                        k: 20,
                        spherical_context: None,
                        alpha_filter: None,
                        time_range: None,
                        core_filter: None,
                    }).await?;
                    
                    // Dual-helix: Recuperar cores desde voxels
                    for voxel in voxels {
                        if let Ok(core) = self.telescope_db.get(&voxel.core_id).await {
                            candidates.push(core);
                        }
                    }
                }
                ExecutionStep::EntanglementTraversal(depth) => {
                    // Traversar entanglements
                    let mut traversed = Vec::new();
                    for core in &candidates {
                        let related = self.traverse_entanglements(&core.id, *depth).await?;
                        traversed.extend(related);
                    }
                    candidates = traversed;
                }
                _ => {}
            }
        }
        
        Ok(candidates)
    }
    
    /// Traversar entanglements recursivamente
    async fn traverse_entanglements(
        &self,
        core_id: &CoreId,
        depth: usize,
    ) -> Result<Vec<QuantumCore>> {
        if depth == 0 {
            return Ok(vec![]);
        }
        
        let core = self.telescope_db.get(core_id).await?;
        let mut results = vec![core.clone()];
        
        for entanglement in &core.entanglements {
            let related = self.traverse_entanglements(&entanglement.target_id, depth - 1).await?;
            results.extend(related);
        }
        
        Ok(results)
    }
}

pub struct QueryResult {
    pub cores: Vec<QuantumCore>,
    pub execution_time: f64,
}
```

---

## 🔌 API PRINCIPAL

### High-Level API

```rust
impl PXLang {
    /// Parse query (auto-detect mode)
    pub async fn parse(&self, input: &str) -> Result<PXQuery> {
        PXQuery::parse_auto(input)
    }
    
    /// Parse query (explicit mode)
    pub async fn parse_with_mode(&self, input: &str, mode: QueryMode) -> Result<PXQuery> {
        PXQuery::parse(input, mode)
    }
    
    /// Execute query
    pub async fn execute(&self, query: &PXQuery) -> Result<QueryResult> {
        self.executor.execute(query.clone()).await
    }
    
    /// Parse + Execute (convenience)
    pub async fn query(&self, input: &str) -> Result<QueryResult> {
        let query = self.parse(input).await?;
        self.execute(&query).await
    }
}

pub enum QueryMode {
    Symbolic,
    Natural,
    Programmatic,
}
```

---

## 🎯 CASOS DE USO

### Caso 1: Symbolic Query con Emotional Filter

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let pxlang = PXLang::new("./data")?;
    
    let results = pxlang.query(r#"
        FIND cores
        WHERE emotional.frustration > 0.7
        AND temporal.age < 14.days()
        ORDER BY timestamp DESC
        LIMIT 5
    "#).await?;
    
    println!("Encontrados {} cores frustrados", results.cores.len());
    for core in results.cores {
        println!("  - {}: {:?}", core.timestamp, core.id);
    }
    
    Ok(())
}
```

### Caso 2: Natural Query con Semantic Search

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let pxlang = PXLang::new("./data")?;
    
    let results = pxlang.query(
        "busca conversaciones sobre Rust ownership cuando estaba frustrado"
    ).await?;
    
    println!("Encontrados {} cores", results.cores.len());
    println!("Execution time: {:.2}ms", results.execution_time);
    
    Ok(())
}
```

### Caso 3: Programmatic Query con Builder

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let pxlang = PXLang::new("./data")?;
    
    let query = PXQuery::builder()
        .emotional_state(EmotionalFilter {
            valence_range: (-1.0, 0.0),
            frustration_min: 0.6,
        })
        .topics(vec!["Rust", "ownership"])
        .time_range(Duration::days(14))
        .order_by(OrderField::Timestamp, OrderDirection::Desc)
        .limit(5)
        .build();
    
    let results = pxlang.execute(&query).await?;
    
    println!("Encontrados {} cores", results.cores.len());
    
    Ok(())
}
```

### Caso 4: Entanglement Traversal

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let pxlang = PXLang::new("./data")?;
    
    // Encontrar última conversación sobre debugging
    let last = pxlang.query(r#"
        FIND cores
        WHERE topic = "debugging"
        ORDER BY timestamp DESC
        LIMIT 1
    "#).await?;
    
    let core_id = last.cores[0].id;
    
    // Traversar relaciones causales
    let related = pxlang.query(&format!(r#"
        FIND cores
        WHERE core_id = "{}"
        TRAVERSE entanglements(type=Causal, depth=2)
    "#, core_id)).await?;
    
    println!("Core inicial + {} relacionados", related.cores.len() - 1);
    
    Ok(())
}
```

---

## ⚡ PERFORMANCE TARGETS

### Objetivos v1.5 (con Symbol Table)

| Operación | Target | Estrategia |
|-----------|--------|------------|
| **parse_symbolic()** | <10ms | Lexer + parser simple (sin I/O) |
| **parse_natural() fast** | <20ms | **Symbol Table lookup** (80% queries) ✅ |
| **parse_natural() slow** | <150ms | Embedding similarity fallback (20% queries complejos) |
| **optimize()** | <20ms | Rewrite rules cached + cost estimation |
| **execute() simple** | <50ms | Single DB index lookup |
| **execute() complex** | <200ms | Multi-step + early termination |
| **execute() HNSW** | <80ms | VoxelDB HNSW (k≤20, ef_search=100) |

### Symbol Table = O(1) Lookup

```rust
/// Symbol Table con HashMap para lookup instantáneo
pub struct SymbolTable {
    // PX-Core-256: Símbolos universales precomputados
    core_symbols: HashMap<String, Symbol>,  // O(1) lookup
    
    // Keywords precomputados: "frustrado" → [😔]
    keyword_to_symbol: HashMap<String, Vec<SymbolId>>,  // O(1) lookup
}

impl SymbolTable {
    /// Carga inicial desde archivo JSON
    pub fn load_px_core_256() -> Self {
        // Precompute en build time
        include_str!("../data/px_core_256.json")
            .parse()
            .unwrap()
    }
    
    /// Match keywords instantáneo
    pub fn match_emotional_keywords(&self, text: &str) -> Option<EmotionalMatch> {
        // O(1) lookup per keyword
        for keyword in EMOTIONAL_KEYWORDS.iter() {
            if text.contains(keyword) {
                // HashMap lookup: O(1)
                if let Some(symbols) = self.keyword_to_symbol.get(keyword) {
                    return Some(self.build_match(symbols));
                }
            }
        }
        None
    }
}

/// Benchmark: Symbol Table vs LLM
///
/// Query: "cuando estaba frustrado hace 2 semanas"
///
/// CON Symbol Table:
/// 1. Keyword scan: 2ms (loop sobre 10 keywords)
/// 2. HashMap lookup: 0.1ms (O(1) × 2 hits)
/// 3. Build filters: 1ms
/// Total: 3.1ms ✅
///
/// SIN Symbol Table (LLM):
/// 1. Tokenize: 2ms
/// 2. Embedding generation: 50ms
/// 3. LLM inference: 100ms
/// 4. Parse response: 5ms
/// Total: 157ms ❌
///
/// Mejora: 50x más rápido con Symbol Table
```

### Fast Path Implementation

```rust
/// Fast path para natural queries comunes
impl NaturalQueryParser {
    async fn parse(&self, input: &str) -> Result<NaturalQuery> {
        // FAST PATH: Pattern matching (80% de queries)
        if let Some(query) = self.try_fast_patterns(input) {
            return Ok(query);  // <20ms
        }
        
        // SLOW PATH: LLM (queries complejos)
        self.llm_parse(input).await  // <150ms
    }
    
    fn try_fast_patterns(&self, input: &str) -> Option<NaturalQuery> {
        let lower = input.to_lowercase();
        
        // Pattern: "hace X días/semanas sobre Y"
        if let Some(caps) = TEMPORAL_TOPIC_PATTERN.captures(&lower) {
            return Some(self.build_temporal_topic_query(&caps));
        }
        
        // Pattern: "cuando estaba [emoción]"
        if let Some(caps) = EMOTIONAL_PATTERN.captures(&lower) {
            return Some(self.build_emotional_query(&caps));
        }
        
        None
    }
}

/// Early termination para traversals
async fn traverse_entanglements(
    &self,
    core_id: &CoreId,
    depth: usize,
    limit: usize,
) -> Result<Vec<QuantumCore>> {
    let mut results = Vec::with_capacity(limit);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((core_id.clone(), 0));
    
    while let Some((id, current_depth)) = queue.pop_front() {
        // Early termination
        if results.len() >= limit {
            break;
        }
        
        if current_depth > depth || visited.contains(&id) {
            continue;
        }
        
        visited.insert(id.clone());
        let core = self.telescope_db.get(&id).await?;
        results.push(core.clone());
        
        if current_depth < depth {
            for ent in &core.entanglements {
                queue.push_back((ent.target_id.clone(), current_depth + 1));
            }
        }
    }
    
    Ok(results)
}
```

### Query Optimization Examples

```
Query original:
  FIND cores SIMILAR TO "Rust ownership"
  WHERE emotional.frustration > 0.6

Plan sin optimizar (costo: 1150ms):
  1. FullScan (1000ms)
  2. FilterByEmotion (100ms)
  3. FilterBySimilarity (50ms)

Plan optimizado (costo: 200ms):
  1. HNSWSearch "Rust ownership" (150ms)  ← Primero (reduce candidates)
  2. FilterByEmotion (50ms)               ← Segundo (sobre resultado pequeño)
```

---

## 🚀 PRÓXIMOS PASOS

### Implementación v1.5 (Prioridad ALTA)

1. ✅ **Especificación completa** (este documento)
2. 🔄 **Implementar SymbolicQueryParser** (src/pxlang/symbolic/)
3. 🔄 **Implementar NaturalQueryParser** (src/pxlang/natural/)
4. 🔄 **Implementar ProgrammaticQueryBuilder** (src/pxlang/programmatic/)
5. 🔄 **Implementar QueryOptimizer** (src/pxlang/optimizer/)
6. 🔄 **Implementar QueryExecutor** (src/pxlang/executor/)
7. 🔄 **Tests end-to-end** (examples/test_pxlang.rs)

### Integración con Ecosistema

- **TelescopeDB** → PXLang (biographical queries)
- **VoxelDB** → PXLang (semantic queries)
- **ShuiDao** → PXLang (intention-to-query translation)
- **CTX7D** → PXLang (context-aware query enhancement)

---

**Estado:** 📋 ESPECIFICACIÓN v1.5 COMPLETA  
**Complejidad:** ⚠️ ALTA - Query engine central  
**Prioridad:** 🟡 ALTA - Interface principal con memoria

---

*Generado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Pixel-Native Revolution*  
*"No es un query language. Es una forma de conversar con tu memoria cuántica."* 🔮✨
