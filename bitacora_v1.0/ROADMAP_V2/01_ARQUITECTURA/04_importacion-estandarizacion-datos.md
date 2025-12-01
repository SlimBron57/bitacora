# 📥 IMPORTACIÓN Y ESTANDARIZACIÓN DE DATOS

**Sistema Bitácora v1.5**  
**Documento de Arquitectura: DA-004**  
**Estado:** 📋 Especificación Completa  
**Fecha:** 27 Noviembre 2025

---

## 🎯 OBJETIVO

Definir el proceso completo de **importación y estandarización** de datos desde múltiples fuentes heterogéneas hacia el sistema pixel-native de Bitácora, con énfasis en:

1. **Fuentes prioritarias v1.0:** WhatsApp, Telegram, MySQL, Notion, Obsidian, JSON/YAML exports
2. **Trazabilidad completa:** Alpha channel (200-255) para tracking de origen
3. **Preservación de relaciones:** Foreign Keys → Entanglements
4. **Formato unificado:** Todo converge a QPX pixel-native

---

## 📋 FUENTES DE DATOS SOPORTADAS

### Nivel 1: Chat Exports (Alta Prioridad v1.0)

| Fuente | Formato | Complejidad | Alpha Channel | Status |
|--------|---------|-------------|---------------|--------|
| **WhatsApp** | `.txt` + media | Media | `210` | ✅ v1.0 |
| **Telegram** | JSON export | Baja | `215` | ✅ v1.0 |
| **Discord** | JSON export | Media | `220` | 🔄 v1.5 |
| **Slack** | JSON export | Media | `225` | 🔄 v1.5 |

### Nivel 2: Databases (Prioridad v1.0)

| Fuente | Formato | Complejidad | Alpha Channel | Status |
|--------|---------|-------------|---------------|--------|
| **MySQL** | SQL dump | Alta | `200` | ✅ v1.0 |
| **PostgreSQL** | SQL dump | Alta | `202` | 🔄 v1.5 |
| **SQLite** | `.db` file | Media | `205` | 🔄 v1.5 |

### Nivel 3: Note-Taking Apps (Prioridad v1.0)

| Fuente | Formato | Complejidad | Alpha Channel | Status |
|--------|---------|-------------|---------------|--------|
| **Notion** | JSON export | Media | `180` | ✅ v1.0 |
| **Obsidian** | Markdown + frontmatter | Baja | `160` | ✅ v1.0 |
| **Roam Research** | JSON export | Media | `165` | 🔄 v2.0 |
| **Logseq** | Markdown + EDN | Media | `170` | 🔄 v2.0 |

### Nivel 4: LLM Backups & Structured Data

| Fuente | Formato | Complejidad | Alpha Channel | Status |
|--------|---------|-------------|---------------|--------|
| **ChatGPT Export** | JSON | Baja | `185` | ✅ v1.0 |
| **Claude Exports** | JSON | Baja | `188` | ✅ v1.0 |
| **JSON Generic** | `.json` | Baja | `190` | ✅ v1.0 |
| **YAML Generic** | `.yaml` | Baja | `192` | ✅ v1.0 |
| **CSV** | `.csv` | Baja | `195` | ✅ v1.0 |

---

## 🏗️ ARQUITECTURA DE IMPORTACIÓN

```
┌─────────────────────────────────────────────────────────────────┐
│                    IMPORT ORCHESTRATOR                           │
│  (Detecta formato, rutea a adapter correcto, monitorea progreso)│
└────────────────────────┬────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
┌───────▼──────┐  ┌──────▼─────┐  ┌──────▼──────┐
│ Chat Adapters│  │DB Adapters │  │ Note Adapters│
│              │  │            │  │              │
│ - WhatsApp   │  │ - MySQL    │  │ - Notion     │
│ - Telegram   │  │ - Postgres │  │ - Obsidian   │
│ - Discord    │  │ - SQLite   │  │ - Markdown   │
└──────┬───────┘  └──────┬─────┘  └──────┬───────┘
       │                 │                │
       └────────┬────────┴────────┬───────┘
                │                 │
        ┌───────▼─────────────────▼────────┐
        │   STANDARDIZATION PIPELINE        │
        │                                   │
        │  1. Parse → Structured Data      │
        │  2. Deduplicate                   │
        │  3. Extract Entities              │
        │  4. FK → Entanglement mapping    │
        │  5. Alpha channel assignment      │
        └───────────────┬───────────────────┘
                        │
        ┌───────────────▼───────────────────┐
        │      SENSORY ENGINE               │
        │  (Process as normal input)        │
        └───────────────┬───────────────────┘
                        │
        ┌───────────────▼───────────────────┐
        │   QPX ENCODING (Variable-Length)  │
        └───────────────┬───────────────────┘
                        │
        ┌───────────────▼───────────────────┐
        │   TELESCOPEDB + VOXELDB           │
        │   (Stored as native pixels)       │
        └───────────────────────────────────┘
```

---

## 🔧 IMPLEMENTACIÓN: ADAPTERS

### 1. WhatsApp Chat Adapter

```rust
// src/import/adapters/whatsapp.rs

pub struct WhatsAppAdapter {
    alpha_channel: u8,  // 210
    media_processor: MediaProcessor,
}

impl DataAdapter for WhatsAppAdapter {
    async fn import(&self, file_path: &Path) -> Result<Vec<StandardizedMessage>> {
        let raw_content = fs::read_to_string(file_path)?;
        let mut messages = Vec::new();
        
        // WhatsApp format: "DD/MM/YYYY, HH:MM - Contact: Message"
        let regex = Regex::new(
            r"(\d{2}/\d{2}/\d{4}), (\d{2}:\d{2}) - ([^:]+): (.+)"
        )?;
        
        for line in raw_content.lines() {
            if let Some(caps) = regex.captures(line) {
                let date = parse_whatsapp_date(&caps[1], &caps[2])?;
                let contact = caps[3].trim();
                let text = caps[4].trim();
                
                // Detectar media attachments
                let media = if text.contains("<attached:") {
                    self.extract_media_reference(text)?
                } else {
                    None
                };
                
                messages.push(StandardizedMessage {
                    id: generate_id(&format!("wa-{}-{}", date.timestamp(), contact)),
                    source: DataSource::WhatsApp,
                    alpha_channel: self.alpha_channel,
                    timestamp: date,
                    author: contact.to_string(),
                    content: text.to_string(),
                    media_refs: media,
                    metadata: HashMap::from([
                        ("original_format", "whatsapp_txt".into()),
                        ("import_date", Utc::now().to_rfc3339()),
                    ]),
                });
            }
        }
        
        Ok(messages)
    }
}

#[derive(Debug, Clone)]
pub struct StandardizedMessage {
    pub id: String,
    pub source: DataSource,
    pub alpha_channel: u8,
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub content: String,
    pub media_refs: Option<Vec<MediaRef>>,
    pub metadata: HashMap<String, String>,
}
```

### 2. Telegram JSON Adapter

```rust
// src/import/adapters/telegram.rs

pub struct TelegramAdapter {
    alpha_channel: u8,  // 215
}

impl DataAdapter for TelegramAdapter {
    async fn import(&self, file_path: &Path) -> Result<Vec<StandardizedMessage>> {
        let raw_json = fs::read_to_string(file_path)?;
        let telegram_export: TelegramExport = serde_json::from_str(&raw_json)?;
        
        let mut messages = Vec::new();
        
        for msg in telegram_export.messages {
            // Telegram JSON tiene estructura rica
            messages.push(StandardizedMessage {
                id: format!("tg-{}-{}", telegram_export.id, msg.id),
                source: DataSource::Telegram,
                alpha_channel: self.alpha_channel,
                timestamp: msg.date,
                author: msg.from.unwrap_or("Unknown".into()),
                content: self.extract_text(&msg)?,
                media_refs: self.extract_media(&msg)?,
                metadata: HashMap::from([
                    ("chat_name", telegram_export.name.clone()),
                    ("chat_type", telegram_export.type_.clone()),
                    ("message_id", msg.id.to_string()),
                ]),
            });
        }
        
        Ok(messages)
    }
    
    fn extract_text(&self, msg: &TelegramMessage) -> Result<String> {
        match &msg.text {
            TextContent::Plain(s) => Ok(s.clone()),
            TextContent::Rich(parts) => {
                // Telegram puede tener formatting (bold, links, etc)
                Ok(parts.iter().map(|p| &p.text).collect::<String>())
            }
        }
    }
}

#[derive(Deserialize)]
struct TelegramExport {
    name: String,
    type_: String,
    id: u64,
    messages: Vec<TelegramMessage>,
}

#[derive(Deserialize)]
struct TelegramMessage {
    id: u64,
    date: DateTime<Utc>,
    from: Option<String>,
    text: TextContent,
    photo: Option<String>,
    file: Option<String>,
}
```

### 3. MySQL Database Adapter

```rust
// src/import/adapters/mysql.rs

pub struct MySQLAdapter {
    alpha_channel: u8,  // 200
    connection: MySqlPool,
}

impl DataAdapter for MySQLAdapter {
    async fn import(&self, config: &MySQLConfig) -> Result<Vec<StandardizedRecord>> {
        let mut records = Vec::new();
        
        // 1. Introspect schema
        let schema = self.introspect_schema(&config.database).await?;
        
        // 2. Identificar tablas principales
        let tables = schema.tables;
        
        for table in tables {
            // 3. Extract data
            let rows = sqlx::query(&format!("SELECT * FROM {}", table.name))
                .fetch_all(&self.connection)
                .await?;
            
            // 4. Mapear FKs a entanglements
            let fk_map = self.extract_foreign_keys(&table).await?;
            
            for row in rows {
                let mut entanglements = Vec::new();
                
                // Convertir FK a entanglement
                for (fk_col, ref_table) in &fk_map {
                    if let Some(fk_value) = row.try_get::<i64, _>(fk_col.as_str()).ok() {
                        entanglements.push(PendingEntanglement {
                            target_table: ref_table.clone(),
                            target_id: fk_value,
                            ent_type: EntanglementType::Operational,
                            strength: 1.0,  // FK es relación fuerte
                        });
                    }
                }
                
                records.push(StandardizedRecord {
                    id: format!("mysql-{}-{}", table.name, row.try_get::<i64, _>("id")?),
                    source: DataSource::MySQL,
                    alpha_channel: self.alpha_channel,
                    table_name: table.name.clone(),
                    fields: self.row_to_map(&row, &table.columns)?,
                    entanglements,
                    metadata: HashMap::from([
                        ("database", config.database.clone()),
                        ("original_id", row.try_get::<i64, _>("id")?.to_string()),
                    ]),
                });
            }
        }
        
        Ok(records)
    }
    
    async fn extract_foreign_keys(&self, table: &TableSchema) -> Result<HashMap<String, String>> {
        let query = format!(
            "SELECT COLUMN_NAME, REFERENCED_TABLE_NAME 
             FROM INFORMATION_SCHEMA.KEY_COLUMN_USAGE 
             WHERE TABLE_NAME = '{}' AND REFERENCED_TABLE_NAME IS NOT NULL",
            table.name
        );
        
        let rows = sqlx::query(&query).fetch_all(&self.connection).await?;
        let mut fk_map = HashMap::new();
        
        for row in rows {
            fk_map.insert(
                row.try_get("COLUMN_NAME")?,
                row.try_get("REFERENCED_TABLE_NAME")?,
            );
        }
        
        Ok(fk_map)
    }
}

#[derive(Debug, Clone)]
pub struct StandardizedRecord {
    pub id: String,
    pub source: DataSource,
    pub alpha_channel: u8,
    pub table_name: String,
    pub fields: HashMap<String, FieldValue>,
    pub entanglements: Vec<PendingEntanglement>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PendingEntanglement {
    pub target_table: String,
    pub target_id: i64,
    pub ent_type: EntanglementType,
    pub strength: f64,
}
```

### 4. Notion Export Adapter

```rust
// src/import/adapters/notion.rs

pub struct NotionAdapter {
    alpha_channel: u8,  // 180
}

impl DataAdapter for NotionAdapter {
    async fn import(&self, export_dir: &Path) -> Result<Vec<StandardizedNote>> {
        let mut notes = Vec::new();
        
        // Notion exporta como carpeta con:
        // - .md files (contenido)
        // - .csv files (databases)
        // - carpetas (pages with children)
        
        for entry in WalkDir::new(export_dir) {
            let entry = entry?;
            
            if entry.path().extension() == Some(OsStr::new("md")) {
                let note = self.parse_notion_markdown(entry.path()).await?;
                notes.push(note);
            } else if entry.path().extension() == Some(OsStr::new("csv")) {
                let db_notes = self.parse_notion_database(entry.path()).await?;
                notes.extend(db_notes);
            }
        }
        
        Ok(notes)
    }
    
    async fn parse_notion_markdown(&self, path: &Path) -> Result<StandardizedNote> {
        let content = fs::read_to_string(path)?;
        
        // Notion añade metadata al inicio
        let (metadata, body) = self.split_frontmatter(&content)?;
        
        // Extraer links [[Page Name]] → entanglements
        let internal_links = self.extract_notion_links(&body)?;
        
        Ok(StandardizedNote {
            id: format!("notion-{}", path.file_stem().unwrap().to_string_lossy()),
            source: DataSource::Notion,
            alpha_channel: self.alpha_channel,
            title: metadata.get("title").cloned().unwrap_or_default(),
            content: body,
            tags: metadata.get("tags")
                .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default(),
            links: internal_links,
            created_at: metadata.get("created")
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            metadata: metadata,
        })
    }
    
    fn extract_notion_links(&self, content: &str) -> Result<Vec<String>> {
        let link_regex = Regex::new(r"\[\[([^\]]+)\]\]")?;
        Ok(link_regex.captures_iter(content)
            .map(|cap| cap[1].to_string())
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct StandardizedNote {
    pub id: String,
    pub source: DataSource,
    pub alpha_channel: u8,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,  // Internal links → entanglements
    pub created_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}
```

### 5. ChatGPT/Claude Exports Adapter

```rust
// src/import/adapters/llm_exports.rs

pub struct LLMExportAdapter {
    alpha_channel: u8,  // 185 (ChatGPT), 188 (Claude)
    source: LLMSource,
}

impl DataAdapter for LLMExportAdapter {
    async fn import(&self, file_path: &Path) -> Result<Vec<StandardizedConversation>> {
        let raw_json = fs::read_to_string(file_path)?;
        
        match self.source {
            LLMSource::ChatGPT => self.parse_chatgpt_export(&raw_json),
            LLMSource::Claude => self.parse_claude_export(&raw_json),
        }
    }
    
    fn parse_chatgpt_export(&self, json: &str) -> Result<Vec<StandardizedConversation>> {
        let export: ChatGPTExport = serde_json::from_str(json)?;
        let mut conversations = Vec::new();
        
        for conv in export.conversations {
            let mut turns = Vec::new();
            
            for (idx, msg) in conv.messages.iter().enumerate() {
                turns.push(ConversationTurn {
                    turn_id: idx,
                    role: msg.role.clone(),
                    content: msg.content.clone(),
                    timestamp: msg.timestamp,
                });
            }
            
            conversations.push(StandardizedConversation {
                id: format!("chatgpt-{}", conv.id),
                source: DataSource::ChatGPT,
                alpha_channel: self.alpha_channel,
                title: conv.title,
                turns,
                created_at: conv.created_at,
                metadata: HashMap::from([
                    ("model", conv.model.unwrap_or("unknown".into())),
                    ("total_tokens", conv.total_tokens.unwrap_or(0).to_string()),
                ]),
            });
        }
        
        Ok(conversations)
    }
}

#[derive(Debug, Clone)]
pub struct StandardizedConversation {
    pub id: String,
    pub source: DataSource,
    pub alpha_channel: u8,
    pub title: String,
    pub turns: Vec<ConversationTurn>,
    pub created_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub turn_id: usize,
    pub role: String,  // "user" | "assistant" | "system"
    pub content: String,
    pub timestamp: DateTime<Utc>,
}
```

---

## 🔄 STANDARDIZATION PIPELINE

```rust
// src/import/standardization.rs

pub struct StandardizationPipeline {
    deduplicator: Deduplicator,
    entity_extractor: EntityExtractor,
    entanglement_mapper: EntanglementMapper,
}

impl StandardizationPipeline {
    pub async fn process<T: StandardizedData>(&self, records: Vec<T>) -> Result<Vec<ProcessedRecord>> {
        let mut processed = Vec::new();
        
        for record in records {
            // 1. Deduplicate (check if already imported)
            if self.deduplicator.is_duplicate(&record).await? {
                log::info!("⏭️  Skipping duplicate: {}", record.id());
                continue;
            }
            
            // 2. Extract entities (people, places, dates, etc)
            let entities = self.entity_extractor.extract(&record).await?;
            
            // 3. Map relationships to entanglements
            let entanglements = self.entanglement_mapper
                .map_relationships(&record, &entities)
                .await?;
            
            // 4. Assign alpha channel from source
            let alpha = record.alpha_channel();
            
            // 5. Create ProcessedRecord
            processed.push(ProcessedRecord {
                original_id: record.id(),
                source: record.source(),
                alpha_channel: alpha,
                content: record.content(),
                entities,
                entanglements,
                timestamp: record.timestamp(),
                metadata: record.metadata(),
            });
        }
        
        Ok(processed)
    }
}

#[derive(Debug, Clone)]
pub struct ProcessedRecord {
    pub original_id: String,
    pub source: DataSource,
    pub alpha_channel: u8,
    pub content: String,
    pub entities: Vec<ExtractedEntity>,
    pub entanglements: Vec<Entanglement>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
}

pub struct Deduplicator {
    telescope_db: Arc<TelescopeDB>,
}

impl Deduplicator {
    pub async fn is_duplicate(&self, record: &impl StandardizedData) -> Result<bool> {
        // Buscar por hash del contenido + timestamp
        let content_hash = self.calculate_hash(record.content());
        
        let existing = self.telescope_db
            .query()
            .filter_by_metadata("content_hash", &content_hash)
            .filter_by_metadata("timestamp", &record.timestamp().to_rfc3339())
            .execute()
            .await?;
        
        Ok(!existing.is_empty())
    }
    
    fn calculate_hash(&self, content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

pub struct EntanglementMapper {
    telescope_db: Arc<TelescopeDB>,
}

impl EntanglementMapper {
    pub async fn map_relationships(
        &self,
        record: &ProcessedRecord,
        entities: &[ExtractedEntity],
    ) -> Result<Vec<Entanglement>> {
        let mut entanglements = Vec::new();
        
        // 1. FK-based relationships (from MySQL, etc)
        for pending in &record.entanglements {
            if let Some(target) = self.resolve_fk_target(pending).await? {
                entanglements.push(Entanglement {
                    target_id: target,
                    strength: pending.strength,
                    ent_type: pending.ent_type,
                    direction: Direction::Bidirectional,
                });
            }
        }
        
        // 2. Entity co-occurrence (people mentioned together → semantic link)
        for entity in entities {
            if entity.entity_type == EntityType::Person {
                if let Some(related) = self.find_related_entity(entity).await? {
                    entanglements.push(Entanglement {
                        target_id: related,
                        strength: 0.6,
                        ent_type: EntanglementType::Semantic,
                        direction: Direction::Bidirectional,
                    });
                }
            }
        }
        
        Ok(entanglements)
    }
    
    async fn resolve_fk_target(&self, pending: &PendingEntanglement) -> Result<Option<String>> {
        // Buscar en TelescopeDB por metadata original_id + table
        self.telescope_db
            .query()
            .filter_by_metadata("original_table", &pending.target_table)
            .filter_by_metadata("original_id", &pending.target_id.to_string())
            .execute()
            .await?
            .first()
            .map(|core| core.id.clone())
            .ok_or_else(|| anyhow!("FK target not found"))
    }
}
```

---

## 📊 IMPORT ORCHESTRATOR

```rust
// src/import/orchestrator.rs

pub struct ImportOrchestrator {
    adapters: HashMap<DataSource, Box<dyn DataAdapter>>,
    standardization: StandardizationPipeline,
    sensory_engine: Arc<SensoryEngine>,
}

impl ImportOrchestrator {
    pub async fn import_from_source(&self, config: ImportConfig) -> Result<ImportReport> {
        let start_time = Utc::now();
        
        // 1. Detect source type
        let source = self.detect_source(&config)?;
        
        // 2. Get appropriate adapter
        let adapter = self.adapters.get(&source)
            .ok_or_else(|| anyhow!("No adapter for source: {:?}", source))?;
        
        log::info!("🚀 Starting import from {:?}", source);
        
        // 3. Run adapter
        let raw_records = adapter.import(&config).await?;
        log::info!("📥 Imported {} raw records", raw_records.len());
        
        // 4. Standardize
        let processed = self.standardization.process(raw_records).await?;
        log::info!("🔄 Processed {} records", processed.len());
        
        // 5. Feed to Sensory Engine (as if they were real-time inputs)
        let mut imported_ids = Vec::new();
        for record in processed {
            let quantum_core = self.sensory_engine.process(record).await?;
            imported_ids.push(quantum_core.id.clone());
        }
        
        log::info!("✅ Import complete: {} cores created", imported_ids.len());
        
        Ok(ImportReport {
            source,
            total_records: imported_ids.len(),
            duration: Utc::now() - start_time,
            imported_ids,
        })
    }
    
    fn detect_source(&self, config: &ImportConfig) -> Result<DataSource> {
        // Auto-detect based on file extension or content
        if let Some(path) = &config.file_path {
            if path.extension() == Some(OsStr::new("txt")) {
                // Peek first line to distinguish WhatsApp
                let first_line = BufReader::new(File::open(path)?)
                    .lines()
                    .next()
                    .ok_or_else(|| anyhow!("Empty file"))??;
                
                if first_line.contains("WhatsApp") || first_line.contains("/") && first_line.contains(":") {
                    return Ok(DataSource::WhatsApp);
                }
            } else if path.extension() == Some(OsStr::new("json")) {
                let content = fs::read_to_string(path)?;
                if content.contains("\"type\": \"personal_chat\"") {
                    return Ok(DataSource::Telegram);
                } else if content.contains("\"conversations\"") {
                    return Ok(DataSource::ChatGPT);
                }
            }
        }
        
        // Fallback to explicit config
        config.source.ok_or_else(|| anyhow!("Cannot detect source"))
    }
}

#[derive(Debug, Clone)]
pub struct ImportConfig {
    pub source: Option<DataSource>,
    pub file_path: Option<PathBuf>,
    pub connection_string: Option<String>,  // For databases
    pub alpha_channel_override: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct ImportReport {
    pub source: DataSource,
    pub total_records: usize,
    pub duration: chrono::Duration,
    pub imported_ids: Vec<String>,
}
```

---

## 🎯 EJEMPLO COMPLETO: IMPORTAR WHATSAPP

```rust
// examples/import_whatsapp.rs

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup orchestrator
    let orchestrator = ImportOrchestrator::new(
        vec![
            Box::new(WhatsAppAdapter::new(210)),
            Box::new(TelegramAdapter::new(215)),
            Box::new(MySQLAdapter::new(200)),
        ],
        StandardizationPipeline::new(),
        Arc::new(SensoryEngine::new()),
    );
    
    // 2. Import WhatsApp chat
    let report = orchestrator.import_from_source(ImportConfig {
        source: Some(DataSource::WhatsApp),
        file_path: Some(PathBuf::from("./data/whatsapp_export.txt")),
        connection_string: None,
        alpha_channel_override: None,
    }).await?;
    
    println!("✅ Import completed!");
    println!("📊 Source: {:?}", report.source);
    println!("📥 Records imported: {}", report.total_records);
    println!("⏱️  Duration: {:?}", report.duration);
    
    // 3. Query imported data (con alpha channel filter)
    let telescope_db = TelescopeDB::new("./data/telescope.db")?;
    
    let whatsapp_cores = telescope_db
        .query()
        .filter_by_alpha_channel(210)  // WhatsApp
        .execute()
        .await?;
    
    println!("🔍 Found {} WhatsApp cores", whatsapp_cores.len());
    
    // 4. Example: Find conversation threads (via entanglements)
    for core in whatsapp_cores.iter().take(5) {
        println!("\n💬 Message from {}", 
            core.metadata.get("author").unwrap_or(&"Unknown".into()));
        println!("   Content: {}", core.text_summary());
        println!("   Entanglements: {}", core.entanglements.len());
    }
    
    Ok(())
}
```

**Resultado:**
```
✅ Import completed!
📊 Source: WhatsApp
📥 Records imported: 1247
⏱️  Duration: 3.2s

🔍 Found 1247 WhatsApp cores

💬 Message from Juan Pérez
   Content: "Hey! Nos vemos mañana?"
   Entanglements: 3

💬 Message from María García
   Content: "Perfecto! A las 5pm en el café"
   Entanglements: 2
```

---

## 🔍 ALPHA CHANNEL TRACKING

### Cómo usar el alpha channel para trazabilidad

```rust
// Query all data from specific source
let mysql_data = telescope_db
    .query()
    .filter_by_alpha_channel(200)  // MySQL
    .execute()
    .await?;

// Query multiple sources
let chat_data = telescope_db
    .query()
    .filter_by_alpha_range(210, 225)  // WhatsApp (210) to Slack (225)
    .execute()
    .await?;

// Find native Bitácora data only
let native_data = telescope_db
    .query()
    .filter_by_alpha_channel(255)  // Native
    .execute()
    .await?;

// Find all imported data (exclude native)
let imported_data = telescope_db
    .query()
    .filter_by_alpha_range(100, 254)  // Everything except 255
    .execute()
    .await?;
```

### Metadata adicional por fuente

Cada core importado incluye metadata de origen:

```rust
StandardizedRecord {
    metadata: HashMap::from([
        ("source_system", "whatsapp"),
        ("alpha_channel", "210"),
        ("import_date", "2025-11-27T14:30:00Z"),
        ("original_id", "msg_12847"),
        ("original_format", "txt"),
        ("author", "Juan Pérez"),
        ("chat_name", "Familia"),
    ]),
}
```

---

## 📈 MÉTRICAS DE IMPORTACIÓN

```rust
pub struct ImportMetrics {
    pub source: DataSource,
    pub total_records: usize,
    pub duplicates_skipped: usize,
    pub errors: usize,
    pub entanglements_created: usize,
    pub duration: Duration,
    pub throughput: f64,  // records/second
}

impl ImportOrchestrator {
    pub async fn get_import_stats(&self) -> ImportMetrics {
        // Track during import for monitoring
    }
}
```

---

## 🚀 ROADMAP DE IMPORTACIÓN

### v1.0 (Inmediato)
- ✅ WhatsApp `.txt` export
- ✅ Telegram JSON export
- ✅ MySQL database dumps
- ✅ Notion export folders
- ✅ Obsidian markdown vaults
- ✅ ChatGPT/Claude JSON exports
- ✅ Generic JSON/YAML files

### v1.5 (Próximo)
- 🔄 Discord JSON export
- 🔄 Slack workspace export
- 🔄 PostgreSQL dumps
- 🔄 SQLite databases
- 🔄 Roam Research export
- 🔄 Logseq graphs

### v2.0 (Futuro)
- 🔮 Live API connectors (Notion API, Google Drive API)
- 🔮 Email imports (Gmail, Outlook)
- 🔮 Calendar imports (Google Calendar, iCal)
- 🔮 Git repository history
- 🔮 Continuous sync (real-time import)

---

## 🎯 CONCLUSIÓN

El sistema de importación de Bitácora v1.5 permite:

1. ✅ **Importar desde múltiples fuentes** con adapters especializados
2. ✅ **Preservar trazabilidad** via alpha channel (200-255)
3. ✅ **Mantener relaciones** FK → Entanglements
4. ✅ **Estandarizar todo a QPX** pixel-native
5. ✅ **Deduplicación automática** para re-imports
6. ✅ **Detección de fuente** automática (auto-detect format)

**Todo converge al mismo sistema pixel-native**, permitiendo consultas unificadas independiente del origen de datos.

---

**Estado:** 📋 Especificación completa - Lista para implementación  
**Prioridad:** 🟡 ALTA - Crítico para onboarding de usuarios  
**Complejidad:** ⚠️ MEDIA-ALTA

---

*Generado: 27 Noviembre 2025*  
*Sistema Bitácora v1.5 - Import & Standardization*  
*"De la heterogeneidad al pixel. Un solo formato para gobernarlos a todos."* 📥✨
