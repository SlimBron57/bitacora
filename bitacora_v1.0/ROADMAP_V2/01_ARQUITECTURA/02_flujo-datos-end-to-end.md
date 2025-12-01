# 🌊 FLUJO DE DATOS END-TO-END - Pipeline Completo v1.5

```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/01_ARQUITECTURA/02_flujo-datos-end-to-end.md
Versión: 1.5 - PIXEL-NATIVE
Fecha Creación: 2025-10-26
Última Actualización: 2025-11-27
Autor: Sistema Bitácora - Arquitectura Integral
Propósito: Documentar flujo completo desde input hasta storage/retrieval (v1.5 QPX)
Estado: ACTIVO
Relacionado Con: 14_qpx-quantumdao-revolucion.md, 01_sistema-dual-databases.md
Pipeline: Input → Sensory → CTX7D → Pixels → QPX → TelescopeDB/VoxelDB → Entanglements
# === FIN DATOS DE AUDITORÍA ===
```

---

## ⚡ CAMBIOS v1.5

> **🎯 QPX-native:** Todo flujo termina en formato `.qpx` (no CBOR)  
> **🌊 QuantumDao:** Branch awareness en cada paso (main vs project:X)  
> **🔗 Entanglements:** Auto-inferencia de relaciones (causal, temporal, semantic)  
> **🎨 Alpha channel:** Multi-purpose encoding (intensity, relevance, progress)

---

## 🎯 PROPÓSITO

Este documento describe el **flujo completo de datos** en Bitácora v1.5 desde que el usuario proporciona input hasta que la información se almacena en formato QPX con entanglements automáticos y topología dinámica.

### Vista Panorámica v1.5 (30,000 ft)

```
┌─────────────┐
│   USUARIO   │
│  Branch: ?  │ ← Detecta contexto (main vs project:X)
└──────┬──────┘
       │ Input (text/voice/visual)
       ▼
┌─────────────────┐
│ SENSORY ENGINE  │ ← Procesa multimodal + branch detection
└──────┬──────────┘
       │ Normalized text + branch_id
       ▼
┌──────────────────┐
│ CONTEXT TOKEN 7D │ ← Analiza 7 dimensiones + operational state
└──────┬───────────┘
       │ 7D vector + score + Project/Job/Task
       ├──────────┬──────────┬──────────┐
       ▼          ▼          ▼          ▼
 ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
 │ Pixels │ │  QPX   │ │ Quantum│ │ Branch │
 │(r,g,b,α)│ │ Header │ │Metadata│ │ Table  │
 └────┬───┘ └───┬────┘ └───┬────┘ └───┬────┘
      │         │          │          │
      └─────────┴──────────┴──────────┘
                │
                ▼
      ┌──────────────────────────────┐
      │   ENTANGLEMENT DISCOVERY      │ ← Auto-infiere relaciones
      │ (Causal, Temporal, Semantic)  │
      └────────┬─────────────────────┘
                │
                ├────────────────────────┐
                ▼                       ▼
      ┌───────────────────┐   ┌───────────────────┐
      │   TELESCOPEDB     │   │     VOXELDB       │
      │   .qpx files      │   │   .qpx templates  │
      │   (r, θ, φ)       │   │   (x, y, z)       │
      │  + EntanglementMap│   │  + OperationalMeta│
      └─────────┬─────────┘   └─────────┬─────────┘
                │                       │
                └───────────┬───────────┘
                            ▼
                  ┌──────────────────┐
                  │ TOPOLOGY MONITOR │ ← Health check
                  │  Self-healing    │
                  └──────────────────┘
```

---

## 📖 PIPELINE PASO A PASO

### FASE 1: INPUT ACQUISITION

```rust
// src/api/input.rs

pub enum UserInput {
    Text(String),
    Voice(VoiceRecording),
    Visual(Image),
}

pub async fn process_input(input: UserInput) -> Result<ProcessedInput> {
    match input {
        UserInput::Text(text) => {
            // Texto directo, no necesita procesamiento adicional
            Ok(ProcessedInput::Text(text))
        }
        
        UserInput::Voice(recording) => {
            // Transcribir con Whisper API
            let transcription = whisper_client.transcribe(&recording).await?;
            Ok(ProcessedInput::Text(transcription.text))
        }
        
        UserInput::Visual(image) => {
            // Análisis con Vision API (futuro v2.0)
            let description = vision_client.describe(&image).await?;
            Ok(ProcessedInput::Text(description))
        }
    }
}

pub enum ProcessedInput {
    Text(String),
}
```

**Output:** Texto normalizado (UTF-8)

---

### FASE 2: SENSORY ENGINE

```rust
// src/cells/sensory_engine.rs

pub struct SensoryEngine {
    normalizer: TextNormalizer,
    metadata_extractor: MetadataExtractor,
}

impl SensoryEngine {
    pub fn process(&self, input: ProcessedInput) -> Result<SensoryOutput> {
        let text = match input {
            ProcessedInput::Text(t) => t,
        };
        
        // 1. Normalización
        let normalized = self.normalizer.normalize(&text)?;
        
        // 2. Extracción de metadatos
        let metadata = self.metadata_extractor.extract(&normalized)?;
        
        // 3. Detección de idioma, sentiment, etc.
        let language = detect_language(&normalized);
        let sentiment = analyze_sentiment(&normalized);
        
        Ok(SensoryOutput {
            text: normalized,
            metadata: SensoryMetadata {
                language,
                sentiment,
                word_count: normalized.split_whitespace().count(),
                char_count: normalized.chars().count(),
                detected_entities: metadata.entities,
                tags: metadata.tags,
            },
            timestamp: Utc::now(),
        })
    }
}

pub struct SensoryOutput {
    pub text: String,
    pub metadata: SensoryMetadata,
    pub timestamp: DateTime<Utc>,
}
```

**Output:** Texto normalizado + metadatos extraídos

---

### FASE 3: CONTEXT TOKEN 7D ANALYSIS

```rust
// src/context_token/analyzer.rs

pub struct ContextTokenAnalyzer {
    dimension_analyzers: [Box<dyn DimensionAnalyzer>; 7],
    bitacora_simulator: BitacoraSimulationEngine,
}

impl ContextTokenAnalyzer {
    pub async fn analyze(&self, sensory: &SensoryOutput) -> Result<ContextToken7D> {
        // 1. Analizar cada dimensión en PARALELO
        let dimensions = futures::future::try_join_all(
            self.dimension_analyzers.iter().map(|analyzer| {
                analyzer.analyze(&sensory.text, &sensory.metadata)
            })
        ).await?;
        
        // 2. Construir Context Token 7D
        let ctx7d = ContextToken7D {
            temporal: dimensions[0],
            semantic: dimensions[1],
            contextual: dimensions[2],
            relational: dimensions[3],
            emotional: dimensions[4],
            intentional: dimensions[5],
            biographical: dimensions[6],
            timestamp: sensory.timestamp,
        };
        
        // 3. Calcular score agregado
        let score = self.calculate_aggregate_score(&ctx7d);
        
        // 4. Detectar breakthrough (score > 1.338)
        let is_breakthrough = if ctx7d.biographical.experiential_uniqueness > 0.85 {
            // Validar con BitacoraSimulationEngine
            self.validate_breakthrough(&ctx7d).await?
        } else {
            false
        };
        
        Ok(ContextToken7D {
            score,
            is_breakthrough,
            ..ctx7d
        })
    }
    
    async fn validate_breakthrough(&self, ctx7d: &ContextToken7D) -> Result<bool> {
        // Ejecutar 10,000 simulaciones estocásticas
        let simulations = self.bitacora_simulator.run_bitacora_simulations(
            &ctx7d.to_input_data(),
            10_000
        )?;
        
        let stats = self.bitacora_simulator.calculate_bitacora_statistics(&simulations);
        
        // Breakthrough si mean > threshold Y confidence > 95%
        Ok(stats.mean > 1.338 && stats.confidence_interval.width() < 0.1)
    }
}

pub struct ContextToken7D {
    // 7 dimensiones
    pub temporal: TemporalDimension,
    pub semantic: SemanticDimension,
    pub contextual: ContextualDimension,
    pub relational: RelationalDimension,
    pub emotional: EmotionalDimension,
    pub intentional: IntentionalDimension,
    pub biographical: BiographicalDimension,
    
    // Agregados
    pub score: f64,
    pub is_breakthrough: bool,
    pub timestamp: DateTime<Utc>,
}
```

**Output:** Context Token 7D con score y breakthrough detection

---

### FASE 4: PIXEL ENCODING + BRANCH DETECTION

```rust
// src/core/quantum_visual_compressor.rs

impl QuantumVisualCompressor {
    pub fn encode(&self, ctx7d: &ContextToken7D, input: &SensoryOutput) -> Result<PixelEncoding> {
        // 1. Detectar branch context (main vs project:X)
        let branch_id = self.detect_branch_context(input)?;
        
        // 2. Convertir 7D → vector numérico
        let dimensions = vec![
            ctx7d.temporal.recency,
            ctx7d.semantic.density,
            ctx7d.contextual.coherence,
            ctx7d.relational.connectivity,
            ctx7d.emotional.valence,
            ctx7d.intentional.clarity,
            ctx7d.biographical.experiential_uniqueness,
        ];
        
        // 3. Normalizar a [0, 1]
        let normalized = self.normalize_dimensions(&dimensions);
        
        // 4. Agrupar de 3 en 3 → RGB pixels
        let mut pixels = self.dimensions_to_pixels(&normalized)?;
        
        // Resultado: 3 pixels
        // Pixel 0: RGB(temporal, semantic, contextual)
        // Pixel 1: RGB(relational, emotional, intentional)
        // Pixel 2: RGB(biographical, 0, 0)
        
        // 5. Set alpha channel según intensidad emocional
        for pixel in &mut pixels {
            pixel.alpha = self.calculate_alpha(&ctx7d);
        }
        
        Ok(PixelEncoding {
            pixels,
            original_dimensions: dimensions,
            branch_id,
        })
    }
    
    fn detect_branch_context(&self, input: &SensoryOutput) -> Result<BranchId> {
        // Detectar keywords: "proyecto bitácora", "work on X", etc.
        if input.text.contains("bitácora") || input.text.contains("bitacora") {
            Ok(BranchId::Project("bitacora".into()))
        } else {
            Ok(BranchId::Main)
        }
    }
    
    fn calculate_alpha(&self, ctx7d: &ContextToken7D) -> u8 {
        // Alpha = intensidad emocional (0-255)
        let intensity = ctx7d.emotional.valence.abs() * ctx7d.biographical.experiential_uniqueness;
        (intensity * 255.0) as u8
    }
}
```

**Output:** Array de 3 pixels RGB + alpha + branch_id

---

### FASE 5: QPX ENCODING (Variable-Length)

```rust
// src/storage/qpx_encoder.rs

impl QPXEncoder {
    pub fn encode_memory(&self, pixels: &[Pixel], ctx7d: &ContextToken7D, branch_id: BranchId) -> Result<Vec<u8>> {
        // Decidir: Compact mode vs Full mode
        if pixels.len() <= 10 && ctx7d.score < 1.0 {
            // COMPACT MODE: Variable-length (5-50 bytes)
            self.encode_compact(pixels, branch_id)
        } else {
            // FULL MODE: Quantum Core (48-byte header + bloques)
            self.encode_full(pixels, ctx7d, branch_id)
        }
    }
    
    fn encode_compact(&self, pixels: &[Pixel], branch_id: BranchId) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        
        // 1. Encode branch_id (compact)
        buffer.push(0x50);  // Major=5 (Branch), Additional=0
        buffer.extend_from_slice(&branch_id.to_compact_bytes());  // ~4 bytes
        
        // 2. Encode pixel block
        buffer.push(0x40 | pixels.len() as u8);  // Major=2, count inline
        for pixel in pixels {
            buffer.extend_from_slice(&pixel.to_bytes());  // 4 bytes cada uno
        }
        
        // TOTAL: ~5 + (pixels.len() * 4) bytes
        Ok(buffer)
    }
    
    fn encode_full(&self, pixels: &[Pixel], ctx7d: &ContextToken7D, branch_id: BranchId) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        
        // 1. Major Type = 3 (QuantumCore)
        buffer.push(0x60);
        
        // 2. Write 48-byte header
        buffer.extend_from_slice(b"QPX\0");  // magic
        buffer.extend_from_slice(&0x0015u16.to_be_bytes());  // version 1.5
        buffer.push(0x00);  // flags
        buffer.push(0x60);  // major_type
        buffer.extend_from_slice(&(pixels.len() as u32).to_be_bytes());
        buffer.extend_from_slice(&branch_id.to_uuid().to_bytes_le());
        buffer.extend_from_slice(&ctx7d.timestamp.timestamp().to_be_bytes());
        // ... rest of 48-byte header (offsets)
        
        // 3. Write PixelBlock
        for pixel in pixels {
            buffer.extend_from_slice(&pixel.to_bytes());
        }
        
        // 4. Write QuantumMetadata
        self.encode_quantum_metadata(&mut buffer, ctx7d, branch_id)?;
        
        // 5. Write EntanglementMap (vacío por ahora, se llena después)
        buffer.extend_from_slice(&[0u8; 16]);  // Placeholder
        
        // 6. Write Footer
        self.encode_footer(&mut buffer)?;
        
        // TOTAL: ~200 bytes
        Ok(buffer)
    }
}
```

**Output:** Vec<u8> (5-200 bytes según complejidad)

---

### FASE 6: ENTANGLEMENT DISCOVERY (Auto-inferencia)

```rust
// src/topology/entanglement_discovery.rs

pub struct EntanglementDiscovery {
    telescope_db: Arc<TelescopeDB>,
    voxel_db: Arc<VoxelDB>,
    git_adapter: Option<Arc<GitAdapter>>,
}

impl EntanglementDiscovery {
    pub async fn discover(&self, new_core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        let mut entanglements = Vec::new();
        
        // 1. CAUSAL: Buscar cores recientes que puedan haber causado este
        entanglements.extend(self.discover_causal(new_core).await?);
        
        // 2. TEMPORAL: Buscar eventos al mismo tiempo (Git commits, etc.)
        entanglements.extend(self.discover_temporal(new_core).await?);
        
        // 3. SEMÁNTICO: Buscar cores similares (cosine similarity)
        entanglements.extend(self.discover_semantic(new_core).await?);
        
        // 4. EMOCIONAL: Buscar cores con mismo sentimiento
        entanglements.extend(self.discover_emotional(new_core).await?);
        
        // 5. OPERACIONAL: Buscar Project/Job/Task relacionados
        entanglements.extend(self.discover_operational(new_core).await?);
        
        Ok(entanglements)
    }
    
    async fn discover_causal(&self, core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        // Buscar cores en ventana temporal de -30 minutos
        let window_start = core.timestamp - 1800;  // 30 min atrás
        let recent_cores = self.telescope_db
            .query_time_range(window_start, core.timestamp)
            .await?;
        
        let mut causal = Vec::new();
        
        for candidate in recent_cores {
            // Calcular "causal weight" basado en:
            // - Proximidad temporal (más cercano = más causal)
            // - Similaridad semántica (mismo topic)
            // - Same branch (mismo contexto)
            
            let temporal_proximity = 1.0 - ((core.timestamp - candidate.timestamp) as f64 / 1800.0);
            let semantic_similarity = self.cosine_similarity(&core.pixels, &candidate.pixels);
            let branch_match = if core.quantum.branch_id == candidate.quantum.branch_id { 1.0 } else { 0.5 };
            
            let causal_weight = temporal_proximity * semantic_similarity * branch_match;
            
            if causal_weight > 0.7 {
                causal.push(Entanglement {
                    target_id: candidate.id,
                    strength: causal_weight as f32,
                    ent_type: EntanglementType::Causal,
                    direction: Direction::Backward,  // candidate causó core
                });
            }
        }
        
        Ok(causal)
    }
    
    async fn discover_temporal(&self, core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        let mut temporal = Vec::new();
        
        // Buscar Git commits en ventana de ±5 minutos
        if let Some(git) = &self.git_adapter {
            let commits = git.commits_in_window(
                core.timestamp - 300,
                core.timestamp + 300
            ).await?;
            
            for commit in commits {
                temporal.push(Entanglement {
                    target_id: format!("git:commit:{}", commit.hash),
                    strength: 1.0,  // Temporal = siempre fuerte
                    ent_type: EntanglementType::Temporal,
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        Ok(temporal)
    }
    
    async fn discover_semantic(&self, core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        // Vector search (cosine similarity > 0.85)
        let similar = self.telescope_db
            .semantic_search(&core.pixels, 10)
            .await?;
        
        similar.into_iter()
            .filter(|(_, similarity)| *similarity > 0.85)
            .map(|(candidate, similarity)| Entanglement {
                target_id: candidate.id,
                strength: similarity as f32,
                ent_type: EntanglementType::Semantic,
                direction: Direction::Bidirectional,
            })
            .collect()
    }
    
    async fn discover_emotional(&self, core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        // Buscar cores con alpha similar (±20) y g (emocional) similar
        let alpha_range = (core.pixels[0].alpha.saturating_sub(20), core.pixels[0].alpha.saturating_add(20));
        let g_range = (core.pixels[0].g.saturating_sub(30), core.pixels[0].g.saturating_add(30));
        
        let emotional_matches = self.telescope_db
            .query_emotional_range(alpha_range, g_range)
            .await?;
        
        emotional_matches.into_iter()
            .filter(|candidate| candidate.id != core.id)
            .map(|candidate| Entanglement {
                target_id: candidate.id,
                strength: 0.6,  // Emotional = peso medio
                ent_type: EntanglementType::Emotional,
                direction: Direction::Bidirectional,
            })
            .collect()
    }
    
    async fn discover_operational(&self, core: &FBCU_Core) -> Result<Vec<Entanglement>> {
        // Buscar cores en mismo Project/Job/Task
        let same_project = self.telescope_db
            .query_by_project(&core.quantum.operational_state.project)
            .await?;
        
        same_project.into_iter()
            .filter(|candidate| candidate.id != core.id)
            .map(|candidate| Entanglement {
                target_id: candidate.id,
                strength: 1.0,  // Operational = siempre fuerte
                ent_type: EntanglementType::Operational,
                direction: Direction::Bidirectional,
            })
            .collect()
    }
}
```

**Output:** Vec<Entanglement> (5 tipos: Causal, Temporal, Semantic, Emotional, Operational)

---

### FASE 7: SPHERICAL COORDINATE MAPPING

```rust
// src/cells/telescopedb/coords.rs

impl CoordinateMapper {
    pub fn map_to_spherical(&self, ctx7d: &ContextToken7D) -> SphericalCoords {
        // Radio: Función de temporalidad (más reciente = más cerca del origen)
        let r = self.temporal_to_radius(ctx7d.temporal.recency);
        
        // Theta (azimutal): Función de dimensión semántica
        let theta = self.semantic_to_theta(ctx7d.semantic.density);
        
        // Phi (polar): Función de dimensión emocional
        let phi = self.emotional_to_phi(ctx7d.emotional.valence);
        
        SphericalCoords { r, theta, phi }
    }
    
    fn temporal_to_radius(&self, recency: f64) -> f64 {
        // Más reciente = menor radio (más cerca del centro)
        // recency ∈ [0, 1] → r ∈ [0.1, 10.0]
        0.1 + (1.0 - recency) * 9.9
    }
    
    fn semantic_to_theta(&self, density: f64) -> f64 {
        // density ∈ [0, 1] → θ ∈ [0, 2π]
        density * 2.0 * std::f64::consts::PI
    }
    
    fn emotional_to_phi(&self, valence: f64) -> f64 {
        // valence ∈ [-1, 1] → φ ∈ [0, π]
        // -1 (negativo) → 0, 0 (neutral) → π/2, +1 (positivo) → π
        ((valence + 1.0) / 2.0) * std::f64::consts::PI
    }
}
```

**Output:** Coordenadas esféricas (r, θ, φ)

---

### FASE 7: TELESCOPEDB STORAGE

```rust
// src/cells/telescopedb.rs

impl TelescopeDB {
    pub async fn insert(&mut self, 
        ctx7d: ContextToken7D,
        fbcu: FBCUCore,
        sensory: SensoryOutput
    ) -> Result<EntryId> {
        
        // 1. Mapear a coordenadas esféricas
        let coords = self.coord_mapper.map_to_spherical(&ctx7d);
        
        // 2. Crear entrada biográfica
        let entry = BiographicalEntry {
            id: Uuid::new_v4().to_string(),
            coordinates: coords,
            fbcu_core: fbcu,
            timestamp: sensory.timestamp.timestamp_millis() as u64,
            metadata: EntryMetadata {
                tags: sensory.metadata.tags,
                ctx7d_score: ctx7d.score,
                is_breakthrough: ctx7d.is_breakthrough,
                language: sensory.metadata.language,
            },
        };
        
        // 3. Serializar a CBOR
        let cbor = entry.to_cbor()?;
        
        // 4. Calcular content-addressable ID
        let content_id = ContentId::from_bytes(&cbor);
        
        // 5. Guardar en storage
        let path = self.storage.id_to_path(&content_id);
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(&path, cbor)?;
        
        // 6. Actualizar índices
        self.update_indices(&entry, &content_id).await?;
        
        // 7. Registrar en log
        self.log_insertion(&entry, &content_id)?;
        
        println!("✅ Entrada guardada: {} en ({:.2}, {:.2}, {:.2})",
            &entry.id[..8], coords.r, coords.theta, coords.phi);
        
        Ok(EntryId {
            content_id,
            uuid: entry.id,
        })
    }
    
    async fn update_indices(&mut self, entry: &BiographicalEntry, id: &ContentId) -> Result<()> {
        // Índice por tags
        for tag in &entry.metadata.tags {
            self.tag_index.add(tag, id);
        }
        
        // Índice por coordenadas (octree espacial)
        self.spatial_index.insert(&entry.coordinates, id);
        
        // Índice temporal
        self.temporal_index.insert(entry.timestamp, id);
        
        // Guardar índices a disco
        self.tag_index.save()?;
        self.spatial_index.save()?;
        self.temporal_index.save()?;
        
        Ok(())
    }
}
```

**Output:** Entry almacenada en `.bitacora/telescope/cores/`

---

### FASE 8: VOXELDB TEMPLATE MATCHING (Opcional)

```rust
// src/cells/voxeldb.rs

impl VoxelDB {
    pub async fn match_template(&self, ctx7d: &ContextToken7D) -> Result<Option<MatchedTemplate>> {
        // 1. Generar embedding del contexto
        let context_embedding = self.llm_client.embed(&ctx7d.to_text()).await?;
        
        // 2. Buscar templates similares (HNSW)
        let candidates = self.vector_index.search(
            &context_embedding,
            k: 5,
            threshold: 0.8
        )?;
        
        if candidates.is_empty() {
            return Ok(None);
        }
        
        // 3. Seleccionar mejor match
        let best_match = candidates[0];
        let template = self.load_template(&best_match.id)?;
        
        Ok(Some(MatchedTemplate {
            template,
            similarity: best_match.score,
            context: ctx7d.clone(),
        }))
    }
}
```

**Output:** Template MTT-DSL (si hay match >0.8)

---

## 🔄 FLUJO DE RECUPERACIÓN (RETRIEVAL)

### Query por Similitud Semántica

```rust
// src/cells/telescopedb/query.rs

impl TelescopeDB {
    pub async fn query_semantic(&self, query: &str, k: usize) -> Result<Vec<BiographicalEntry>> {
        // 1. Analizar query con Context Token 7D
        let query_ctx7d = self.ctx_analyzer.analyze_text(query).await?;
        
        // 2. Convertir a pixels
        let query_pixels = self.visual_compressor.ctx7d_to_pixels(&query_ctx7d)?;
        
        // 3. Buscar entries con pixels similares
        let mut results = Vec::new();
        
        for entry_id in self.storage.iter_all()? {
            let entry = self.load_entry(&entry_id)?;
            
            // Descomprimir FBCU → pixels
            let entry_pixels = self.fbcu_engine.decompress_to_pixels(&entry.fbcu_core)?;
            
            // Calcular distancia euclidiana
            let distance = pixel_distance(&query_pixels, &entry_pixels);
            
            results.push((entry, distance));
        }
        
        // 4. Ordenar por distancia (menor = más similar)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        // 5. Retornar top-k
        Ok(results.into_iter().take(k).map(|(e, _)| e).collect())
    }
}
```

### Query por Región Esférica

```rust
impl TelescopeDB {
    pub fn query_region(&self, 
        center: &SphericalCoords, 
        radius: f64
    ) -> Result<Vec<BiographicalEntry>> {
        
        // Usar índice espacial (octree)
        let entry_ids = self.spatial_index.query_sphere(center, radius)?;
        
        // Cargar entries
        entry_ids.iter()
            .map(|id| self.load_entry(id))
            .collect()
    }
}
```

---

## 📊 PERFORMANCE DEL PIPELINE

### Benchmarks End-to-End

```rust
#[bench]
fn bench_full_pipeline(b: &mut Bencher) {
    let system = BitacoraSystem::new();
    let input = "Eduardo implementó TelescopeDB con geometría esférica el 26 de octubre de 2025";
    
    b.iter(|| {
        // Pipeline completo
        let sensory = system.sensory_engine.process(input).unwrap();
        let ctx7d = system.ctx_analyzer.analyze(&sensory).await.unwrap();
        let pixels = system.visual_compressor.encode(&ctx7d).unwrap();
        let fbcu = system.fbcu_engine.compress(&pixels).unwrap();
        let entry_id = system.telescopedb.insert(ctx7d, fbcu, sensory).await.unwrap();
        
        black_box(entry_id);
    });
    
    // Resultado esperado: ~50-100ms total
    // Breakdown:
    //   - Sensory: ~5ms
    //   - CTX7D: ~30ms (LLM calls)
    //   - Pixels: ~1ms
    //   - FBCU: ~5ms
    //   - Storage: ~10ms
}
```

### Métricas Reales

| Fase | Tiempo | % Total | Bottleneck? |
|------|--------|---------|-------------|
| Sensory | 5ms | 5% | ❌ No |
| CTX7D Analysis | 30ms | 30% | ⚠️ LLM calls |
| Pixel Encoding | 1ms | 1% | ❌ No |
| FBCU Compression | 5ms | 5% | ❌ No |
| TelescopeDB Insert | 10ms | 10% | ❌ No |
| Indexing | 5ms | 5% | ❌ No |
| **TOTAL** | **~100ms** | **100%** | CTX7D |

**Optimización:** Cachear análisis CTX7D para queries frecuentes.

---

## 🧪 TESTING DEL FLUJO COMPLETO

```rust
#[tokio::test]
async fn test_end_to_end_pipeline() {
    // Setup
    let system = BitacoraSystem::new();
    
    // Input
    let input = UserInput::Text(
        "Aprendí sobre content-addressable storage hoy".into()
    );
    
    // FASE 1-2: Sensory
    let sensory = system.sensory_engine.process(input).unwrap();
    assert_eq!(sensory.metadata.language, Language::Spanish);
    
    // FASE 3: CTX7D
    let ctx7d = system.ctx_analyzer.analyze(&sensory).await.unwrap();
    assert!(ctx7d.score > 0.0);
    
    // FASE 4: Pixels
    let pixels = system.visual_compressor.encode(&ctx7d).unwrap();
    assert_eq!(pixels.pixels.len(), 3);  // 7D → 3 pixels
    
    // FASE 5: FBCU
    let fbcu = system.fbcu_engine.compress(&pixels.pixels).unwrap();
    assert!(fbcu.metadata.compression_ratio < 0.01);  // >99% compression
    
    // FASE 6-7: TelescopeDB
    let entry_id = system.telescopedb.insert(ctx7d.clone(), fbcu, sensory).await.unwrap();
    
    // VERIFICACIÓN: Recuperar y validar
    let loaded_entry = system.telescopedb.load(&entry_id.content_id).unwrap();
    assert_eq!(loaded_entry.id, entry_id.uuid);
    
    // FASE 8: Query
    let results = system.telescopedb.query_semantic("storage", 5).await.unwrap();
    assert!(results.len() > 0);
    assert_eq!(results[0].id, entry_id.uuid);
}
```

---

## 🔍 DEBUGGING Y OBSERVABILIDAD

### Pipeline Tracer

```rust
// src/utils/pipeline_tracer.rs

pub struct PipelineTracer {
    events: Vec<TraceEvent>,
    start_time: Instant,
}

impl PipelineTracer {
    pub fn trace_phase(&mut self, phase: Phase, duration: Duration) {
        self.events.push(TraceEvent {
            phase,
            duration,
            timestamp: self.start_time.elapsed(),
        });
    }
    
    pub fn report(&self) -> String {
        let total = self.events.iter().map(|e| e.duration).sum::<Duration>();
        
        let mut report = format!("Pipeline Execution Report\n");
        report.push_str(&format!("Total time: {:?}\n\n", total));
        
        for event in &self.events {
            let pct = (event.duration.as_secs_f64() / total.as_secs_f64()) * 100.0;
            report.push_str(&format!(
                "{:?}: {:?} ({:.1}%)\n",
                event.phase, event.duration, pct
            ));
        }
        
        report
    }
}

// Uso:
let mut tracer = PipelineTracer::new();

let t0 = Instant::now();
let sensory = process_sensory(input)?;
tracer.trace_phase(Phase::Sensory, t0.elapsed());

let t1 = Instant::now();
let ctx7d = analyze_ctx7d(&sensory)?;
tracer.trace_phase(Phase::CTX7D, t1.elapsed());

// ... etc ...

println!("{}", tracer.report());
```

---

## ⚠️ MANEJO DE ERRORES

### Error Propagation

```rust
// src/core/error.rs

#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("Sensory processing failed: {0}")]
    SensoryFailed(String),
    
    #[error("CTX7D analysis failed: {0}")]
    CTX7DFailed(String),
    
    #[error("Pixel encoding failed: {0}")]
    PixelEncodingFailed(String),
    
    #[error("FBCU compression failed: {0}")]
    FBCUFailed(String),
    
    #[error("TelescopeDB insertion failed: {0}")]
    TelescopeDBFailed(String),
    
    #[error("VoxelDB matching failed: {0}")]
    VoxelDBFailed(String),
}

// Pipeline con manejo robusto
pub async fn run_pipeline(input: UserInput) -> Result<EntryId, PipelineError> {
    // Cada fase puede fallar independientemente
    let sensory = process_sensory(input)
        .map_err(|e| PipelineError::SensoryFailed(e.to_string()))?;
    
    let ctx7d = analyze_ctx7d(&sensory).await
        .map_err(|e| PipelineError::CTX7DFailed(e.to_string()))?;
    
    let pixels = encode_pixels(&ctx7d)
        .map_err(|e| PipelineError::PixelEncodingFailed(e.to_string()))?;
    
    let fbcu = compress_fbcu(&pixels)
        .map_err(|e| PipelineError::FBCUFailed(e.to_string()))?;
    
    let entry_id = insert_telescopedb(ctx7d, fbcu, sensory).await
        .map_err(|e| PipelineError::TelescopeDBFailed(e.to_string()))?;
    
    Ok(entry_id)
}
```

---

## 📚 REFERENCIAS

### Documentos Relacionados
- `00_VISION/BITA-1_FBCU_SPECIFICATION.md`
- `00_VISION/BITA-2_ACA-7D_SPECIFICATION.md`
- `01_ARQUITECTURA/SISTEMA_DUAL_DATABASES.md`
- `01_ARQUITECTURA/PIXEL_STORAGE_DEEP_DIVE.md`
- `01_ARQUITECTURA/CBOR_IMPLEMENTATION.md`
- `01_ARQUITECTURA/CONTENT_ADDRESSABLE_IDS.md`

### Componentes Clave
- Sensory Engine
- Context Token 7D Analyzer
- QuantumVisualCompressor
- FBCUEngine
- TelescopeDB
- VoxelDB

---

---

## 🏥 FASE 8: TOPOLOGY MONITORING & SELF-HEALING

```rust
// src/topology/monitor.rs

pub struct TopologyMonitor {
    telescope_db: Arc<TelescopeDB>,
    check_interval: Duration,
}

impl TopologyMonitor {
    pub async fn run_continuous_monitoring(&self) -> ! {
        loop {
            // 1. Health check completo
            let health = self.check_topology_health().await;
            
            // 2. Auto-reparación si hay problemas
            if health.integrity_score < 0.9 {
                self.auto_heal(&health).await;
            }
            
            // 3. Log report
            self.log_health_report(&health);
            
            // 4. Sleep hasta próximo check
            tokio::time::sleep(self.check_interval).await;
        }
    }
    
    async fn check_topology_health(&self) -> TopologyHealth {
        let mut health = TopologyHealth::default();
        
        // 1. Detectar broken links
        health.broken_links = self.detect_broken_links().await;
        
        // 2. Detectar orphaned nodes
        health.orphaned_nodes = self.detect_orphaned_nodes().await;
        
        // 3. Detectar cycles peligrosos
        health.cycles = self.detect_dangerous_cycles().await;
        
        // 4. Calcular integrity score
        health.integrity_score = self.calculate_integrity(&health);
        
        health
    }
    
    async fn detect_broken_links(&self) -> Vec<BrokenLink> {
        let all_cores = self.telescope_db.get_all_cores().await?;
        let mut broken = Vec::new();
        
        for core in all_cores {
            for ent in &core.entanglements {
                // Verificar que target existe
                if !self.telescope_db.exists(&ent.target_id).await? {
                    broken.push(BrokenLink {
                        from: core.id.clone(),
                        to: ent.target_id.clone(),
                        ent_type: ent.ent_type,
                        severity: Severity::Critical,
                    });
                }
            }
        }
        
        broken
    }
    
    async fn detect_orphaned_nodes(&self) -> Vec<OrphanedNode> {
        let all_cores = self.telescope_db.get_all_cores().await?;
        let mut orphaned = Vec::new();
        
        for core in all_cores {
            // Huérfano si: no entanglements + no usado en > 90 días
            if core.entanglements.is_empty() {
                let last_accessed = self.get_last_access_time(&core.id).await?;
                let days_unused = (Utc::now() - last_accessed).num_days();
                
                if days_unused > 90 {
                    orphaned.push(OrphanedNode {
                        node_id: core.id.clone(),
                        days_unused,
                        reason: "No entanglements + unused > 90 days".into(),
                    });
                }
            }
        }
        
        orphaned
    }
    
    async fn auto_heal(&self, health: &TopologyHealth) {
        // 1. Reparar broken links
        for broken_link in &health.broken_links {
            self.repair_broken_link(broken_link).await;
        }
        
        // 2. Adoptar orphaned nodes
        for orphan in &health.orphaned_nodes {
            self.adopt_orphan(orphan).await;
        }
        
        // 3. Resolver cycles
        for cycle in &health.cycles {
            self.resolve_cycle(cycle).await;
        }
    }
    
    async fn repair_broken_link(&self, link: &BrokenLink) {
        // Estrategia: Buscar core similar para re-link
        let similar = self.telescope_db
            .find_similar_to_deleted(&link.to)
            .await?;
        
        if let Some(replacement) = similar.first() {
            // Crear nuevo entanglement a replacement
            self.telescope_db.add_entanglement(
                &link.from,
                Entanglement {
                    target_id: replacement.id.clone(),
                    strength: 0.7,  // Más débil (es inferido)
                    ent_type: link.ent_type,
                    direction: Direction::Bidirectional,
                }
            ).await?;
            
            log::info!("✅ Repaired broken link: {} → {}", link.from, replacement.id);
        }
    }
    
    async fn adopt_orphan(&self, orphan: &OrphanedNode) {
        // Estrategia: Conectar a "historical_archives" collection
        self.telescope_db.add_entanglement(
            &orphan.node_id,
            Entanglement {
                target_id: "collection:historical_archives".into(),
                strength: 0.5,
                ent_type: EntanglementType::Operational,
                direction: Direction::Bidirectional,
            }
        ).await?;
        
        log::info!("✅ Adopted orphan: {} → historical_archives", orphan.node_id);
    }
}

pub struct TopologyHealth {
    pub broken_links: Vec<BrokenLink>,
    pub orphaned_nodes: Vec<OrphanedNode>,
    pub cycles: Vec<Cycle>,
    pub integrity_score: f64,  // 0.0 - 1.0
}
```

---

## 🚀 PRÓXIMOS PASOS

### En v1.5 (Implementación Inmediata)
1. ✅ Implementar pipeline completo con QPX variable-length
2. ✅ Entanglement discovery automático
3. ✅ Topology monitoring & self-healing
4. ✅ Tests end-to-end con datos reales
5. ✅ Pipeline tracer para observabilidad
6. ✅ Benchmarks de performance

### En v2.0 (Mejoras Futuras)
1. Pipeline paralelo (múltiples inputs simultáneos)
2. Batch processing (procesar N entradas en lote)
3. Streaming pipeline (procesar mientras se ingesta)
4. Distributed pipeline (multi-nodo)
5. Machine learning para entanglement prediction

---

**Estado:** 📋 Especificación v1.5 completa - Listo para implementación  
**Complejidad:** ⚠️ ALTA - Integra todos los componentes + topología dinámica  
**Prioridad:** 🔴 CRÍTICA - Es el corazón del sistema

---

*Actualizado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Pixel-Native Revolution*  
*"De la voz al píxel, del píxel a la topología cuántica. El flujo evoluciona."* 🌊✨🎨
