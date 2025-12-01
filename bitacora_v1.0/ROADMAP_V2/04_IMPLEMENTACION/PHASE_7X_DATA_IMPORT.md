```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/PHASE_7X_DATA_IMPORT_PLAN.md
Versión: 1.0
Fecha Creación: 2025-11-29
Última Actualización: 2025-11-29 11:00:00
Autor: B (Sistema Bitácora)
Propósito: Plan ejecutivo Phase 7.x - Metabolic Digestion System
Estado: ACTIVO - Diseño completo, ready for implementation
Relaciones:
  - ARQUITECTURA: ROADMAP_V2/01_ARQUITECTURA/METABOLIC_DIGESTION_SYSTEM.md (1,800 líneas, detailed design)
  - CHECKLIST: ROADMAP_V2/CHECKLIST_V2.md (43 tasks breakdown)
  - CHECKLIST_TREE: ROADMAP_V2/CHECKLIST_TREE_V2.md (hierarchical structure)
  - GUIA: ROADMAP_V2/GUIA.md (methodology compliance)
Changelog:
  - 2025-11-29 11:00: Documento creado - Plan ejecutivo y quick reference
# === FIN DATOS DE AUDITORÍA ===
```

# 🧬 Phase 7.x: Metabolic Digestion System

**Plan Ejecutivo - Importación de Datos Externa**

---

## 🎯 Executive Summary

### The Vision

> **"No se trata de ingerir, se trata de digerir y extraer nutrientes"** - Eduardo Gil

**Problem:** Manual onboarding takes 30 minutes of Q&A. Users abandon before seeing value.

**Solution:** Import external data (WhatsApp, Telegram, Spotify, etc.) with metabolic digestion pipeline that respects the unique nature of each source.

**Impact:** 
- ⚡ 60x improvement: 30 minutes → 30 seconds onboarding
- 🎯 Killer feature vs ChatGPT/Claude (they don't import external data)
- 🔗 Hyperlink Intelligence (unique URL analysis)
- 🎨 Template-driven evolution (no recompilation needed)

### The Numbers

| Metric | Value |
|--------|-------|
| **Duration** | 6 weeks (Nov 29 - Jan 10) |
| **Effort** | 80-100 hours (part-time) |
| **Tasks** | 43 total (8 groups) |
| **Code** | 5,000-7,000 lines (Rust + tests) |
| **Templates** | ~2,000 lines (YAML) |
| **Documentation** | ~3,000 lines |
| **Target** | v1.1.x (minor version bump) |

### Success Criteria

- ✅ Onboarding speed: <30s (vs 30min manual)
- ✅ Extraction accuracy: >90%
- ✅ User satisfaction: >8.0/10
- ✅ Performance: <30s per 1000 messages
- ✅ Template evolution: 2-3 updates/week without recompiling
- ✅ Zero data loss: 100% preserved
- ✅ Support 5+ platforms (WhatsApp, Telegram, Email, Spotify, GitHub)

---

## 🏗️ Architecture Overview

### The Pipeline: 5 Phases

```
┌─────────────────────────────────────────────────────────────┐
│                    METABOLIC DIGESTION                       │
│                                                              │
│  Data Sources → [1. Quarantine] → [2. Digestion] →          │
│                 [3. Extraction] → [4. Validation] →          │
│                 [5. Distribution] → Subsystems               │
└─────────────────────────────────────────────────────────────┘
```

### Phase 1: Quarantine Layer 🛡️
**Safety first - inspect before processing**

- Integrity verification (SHA-256 hashes)
- Format detection (auto-detect JSON, CSV, XML, text)
- Threat scanning (malware, exploits)
- Quality assessment (completeness, coherence)
- PII detection and warnings
- User approval required before digestion

**CLI Commands:**
```bash
/import whatsapp chat_backup.txt
/quarantine list
/quarantine inspect abc123
/quarantine approve abc123
```

### Phase 2: Source-Specific Digestion 🍽️
**Each source deserves unique understanding**

**Philosophy:** WhatsApp ≠ Email ≠ Spotify. Each platform has unique characteristics:

- **WhatsApp:** Groups, multimedia, informal tone, high frequency, emoji context
- **Telegram:** Channels, bots, stickers, forwards
- **Email:** Threads, attachments, formal tone, low frequency, professional context
- **Calendar:** Events, recurrence, invitees, time commitments
- **Spotify:** Playlists, genres, listening patterns, mood-music correlation
- **GitHub:** Commits, PRs, repos, languages, technical expertise
- **Twitter:** Threads, mentions, hashtags, brevity

**Hybrid Architecture:**
```rust
pub struct HybridDigester {
    core: CoreDigester,              // Hard-coded (stable, fast)
    template_engine: TemplateEngine,  // YAML-based (flexible, evolvable)
    template: DigestionTemplate,
}
```

**Why Hybrid?**
- **Performance:** Core compiled (fast parsing, validation)
- **Flexibility:** Templates YAML (edit without recompiling)
- **Experimentation:** A/B test multiple templates
- **Evolution:** "Estamos inventando desde cero" - need rapid iteration

### Phase 3: Nutrient Extraction 🧬
**Multi-dimensional parallel extraction**

Extract 7 dimensions simultaneously:

1. **Biographical:** Name, age, location, occupation, identity
2. **Emotional:** VADC baseline (Valence, Arousal, Dominance, Certainty)
3. **Behavioral:** Communication style, activity patterns, decision-making
4. **Relational:** Social graph, relationship strengths, group memberships
5. **Temporal:** Life events timeline, daily/weekly routines
6. **Interests:** Topics, categories, expertise levels from content
7. **Hyperlink Intelligence** 🆕: URL analysis for consumption patterns

**Hyperlink Intelligence** (Competitive Advantage):
- Extract URLs from messages (expand short links)
- Classify platform (YouTube, Spotify, GitHub, etc.)
- Fetch metadata (title, description, duration via Open Graph)
- Infer intent (self-reference, recommendation, question)
- Calculate efficiency score (deep work vs entertainment time)
- Determine social role (Curator, Learner, Entertainer)

**Performance:** All extractors run in parallel using `tokio::join!`

### Phase 4: Validation & Coherence ✅
**Detect conflicts and ensure consistency**

- **Conflict Detection:**
  - Temporal inconsistencies (events in wrong order)
  - Identity mismatches ("Eduardo" vs "Edgi" - same person?)
  - Interest contradictions ("I hate X" but mentions X positively)

- **Cross-Source Validation:**
  - Compare WhatsApp data vs Telegram data
  - Verify against existing TelescopeDB biographical data

- **Interactive Resolution:**
  - Present conflicts to user with options
  - User selects resolution (or accepts both as context-dependent)

- **Confidence Scoring:**
  - Each nutrient gets confidence score (0.0-1.0)
  - Low-confidence data flagged for review

### Phase 5: Distribution to Subsystems 📡
**Route nutrients to correct destinations**

Parallel distribution to all subsystems:

| Nutrient | Destination Subsystems |
|----------|------------------------|
| **Identity** | TelescopeDB (biographical entries), IceBreaker (boost confidence) |
| **Interests** | TopicGraph (add/update topics + edges), IceBreaker (personalization) |
| **Emotional** | EmotionalSpace (baseline profile + patterns) |
| **Behavioral** | MemoryBridge (communication style, activity patterns) |
| **Relational** | SocialGraph (relationships + strength, group memberships) |
| **Temporal** | MemoryBridge (timeline of events, routines) |
| **Hyperlinks** | TopicGraph (interests from links), BiographicalProfile (consumption patterns) |

**Performance:** All distributions run in parallel using `tokio::try_join!`

**Error Handling:** Partial failures handled gracefully (continue if one subsystem fails)

---

## 🎨 Template System

### Why Templates?

**Eduardo's Insight:** "Estamos inventando esto desde cero y muy seguramente tendrá muchas correcciones"

**Problem:** Hard-coded extraction rules require recompilation for every tweak.

**Solution:** YAML templates for extraction logic.

**Benefits:**
- ✏️ Edit template → Save → Auto-reload (no recompilation)
- 🧪 A/B testing (control vs variant templates)
- 📊 Metrics comparison (which template extracts better?)
- 🔄 Version evolution (v1, v2, v3 without code changes)

### Template Structure

```yaml
# templates/digesters/whatsapp_v1.yaml
version: "1.0"
source: "WhatsApp"
author: "Eduardo Gil"
date: "2025-11-29"

extends: "base_chat.yaml"  # Inheritance

extraction:
  identity:
    patterns:
      - field: "sender"
        regex: "^[A-Z][a-z]+ [A-Z][a-z]+$"
        confidence: 0.9
  
  interests:
    keywords:
      technology: ["rust", "AI", "🦀", "crab"]
      music: ["canción", "🎵", "playlist"]
    context_boost:
      - condition: "interest + emoji in same message"
        boost: 0.2
  
  relationships:
    strength_indicators:
      very_strong:
        - daily_messages: "> 10"
        - emoji_usage: "> 5"
      strong:
        - daily_messages: "> 5"

hyperlinks:
  enabled: true
  expand_short_urls: true
  fetch_metadata: true
  platforms_priority:
    - YouTube
    - Spotify
    - GitHub
  intent_inference:
    self_reference:
      patterns: ["shared to myself", "para mi", "revisar luego"]
      confidence: 0.95
    recommendation:
      patterns: ["deberías", "te recomiendo", "check this out"]
      confidence: 0.85

distribution:
  biographical: "TelescopeDB"
  interests: "TopicGraph"
  emotional: "EmotionalSpace"
  relationships: "SocialGraph"
```

### Template Evolution

**Version History:**
```
whatsapp_v1.yaml  → Initial (basic keyword extraction)
whatsapp_v2.yaml  → Improved (emoji context, relationship inference)
whatsapp_v3.yaml  → Advanced (hyperlink intelligence, intent inference)
```

**A/B Testing:**
```rust
let experiment = DigestExperiment::new(
    control: load_template("whatsapp_v2.yaml"),
    variant: load_template("whatsapp_v3.yaml"),
);

let comparison = experiment.run(sample_data).await?;
// ComparisonReport {
//   accuracy_delta: +12%,
//   confidence_delta: +8%,
//   recommendation: "Deploy variant to production"
// }
```

### CLI Commands

```bash
# Reload templates without restarting
/reload templates

# Test template on sample data
/test template whatsapp_v3.yaml sample_chat.txt

# Compare templates (A/B test)
/compare whatsapp_v2.yaml whatsapp_v3.yaml sample_chat.txt

# Show template diff
/diff whatsapp_v2.yaml whatsapp_v3.yaml
```

---

## 📊 Implementation Plan

### 6-Week Timeline

| Week | Focus | Tasks | Hours |
|------|-------|-------|-------|
| **Week 1** | Quarantine + Core Digester | 7.x.1 | 8h |
| **Week 2** | Source Digesters + Templates Start | 7.x.2, 7.x.6 (parallel) | 12h |
| **Week 3** | Nutrient Extraction + Validation | 7.x.3, 7.x.4 | 14h + 10h |
| **Week 4** | Distribution + Templates Complete | 7.x.5, 7.x.6 (parallel) | 12h + 8h |
| **Week 5** | Hyperlink Intelligence | 7.x.7 | 10h |
| **Week 6** | Integration + Testing + User Feedback | 7.x.8 | 12h |

### Task Breakdown (43 tasks)

#### 7.x.1 - Quarantine Layer (Week 1, 8h)
- [ ] 7.x.1.1 - QuarantineZone struct (state machine)
- [ ] 7.x.1.2 - Inspection engine (integrity, format, threats, quality)
- [ ] 7.x.1.3 - CLI commands (/quarantine)
- [ ] 7.x.1.4 - Dashboard visual (status indicators)
- [ ] 7.x.1.5 - Tests (corruption, PII, format detection)

#### 7.x.2 - Source-Specific Digesters (Week 2, 12h)
- [ ] 7.x.2.1 - DigestionPipeline trait
- [ ] 7.x.2.2 - WhatsAppDigester
- [ ] 7.x.2.3 - TelegramDigester
- [ ] 7.x.2.4 - EmailDigester
- [ ] 7.x.2.5 - CalendarDigester
- [ ] 7.x.2.6 - SpotifyDigester
- [ ] 7.x.2.7 - GitHubDigester
- [ ] 7.x.2.8 - Factory pattern (auto-select)
- [ ] 7.x.2.9 - Tests per digester

#### 7.x.3 - Nutrient Extraction (Week 2-3, 14h)
- [ ] 7.x.3.1 - NutrientExtractor struct (orchestrator)
- [ ] 7.x.3.2 - BiographicalExtractor
- [ ] 7.x.3.3 - EmotionalExtractor
- [ ] 7.x.3.4 - BehavioralExtractor
- [ ] 7.x.3.5 - RelationalExtractor
- [ ] 7.x.3.6 - TemporalExtractor
- [ ] 7.x.3.7 - HyperlinkExtractor 🆕
- [ ] 7.x.3.8 - Parallel extraction optimization
- [ ] 7.x.3.9 - Tests (>90% accuracy)

#### 7.x.4 - Validation & Coherence (Week 3, 10h)
- [ ] 7.x.4.1 - CoherenceValidator struct
- [ ] 7.x.4.2 - ConflictDetector (temporal, identity, interests)
- [ ] 7.x.4.3 - ConsistencyChecker (cross-source)
- [ ] 7.x.4.4 - TruthEstimator (confidence scoring)
- [ ] 7.x.4.5 - Interactive conflict resolution CLI
- [ ] 7.x.4.6 - ValidationReport generation
- [ ] 7.x.4.7 - Tests (conflict scenarios)

#### 7.x.5 - Distribution to Subsystems (Week 3-4, 12h)
- [ ] 7.x.5.1 - NutrientDistributor struct
- [ ] 7.x.5.2 - distribute_identity()
- [ ] 7.x.5.3 - distribute_interests()
- [ ] 7.x.5.4 - distribute_emotional()
- [ ] 7.x.5.5 - distribute_behavioral()
- [ ] 7.x.5.6 - distribute_relational()
- [ ] 7.x.5.7 - distribute_temporal()
- [ ] 7.x.5.8 - distribute_hyperlinks()
- [ ] 7.x.5.9 - DistributionReport + visualization
- [ ] 7.x.5.10 - Tests (parallel distribution, error handling)

#### 7.x.6 - Template System (Week 2-4 parallel, 16h)
- [ ] 7.x.6.1 - DigestionTemplate struct
- [ ] 7.x.6.2 - TemplateEngine (load, execute, validate)
- [ ] 7.x.6.3 - YAML loader + validator (serde_yaml)
- [ ] 7.x.6.4 - Template inheritance (extends)
- [ ] 7.x.6.5 - Template versioning (v1, v2, v3)
- [ ] 7.x.6.6 - A/B testing framework
- [ ] 7.x.6.7 - Hot reload (/reload templates)
- [ ] 7.x.6.8 - Initial templates YAML (4 templates)
- [ ] 7.x.6.9 - Tests (loading, inheritance, validation)

#### 7.x.7 - Hyperlink Intelligence (Week 5, 10h) 🆕
- [ ] 7.x.7.1 - URL extraction (regex, short URLs)
- [ ] 7.x.7.2 - URL expansion service (redirects, cache)
- [ ] 7.x.7.3 - Platform classification (domain parsing)
- [ ] 7.x.7.4 - Metadata fetching (Open Graph, APIs)
- [ ] 7.x.7.5 - Intent inference (self-share, recommendation)
- [ ] 7.x.7.6 - Intelligence analysis (consumption, efficiency)
- [ ] 7.x.7.7 - CLI visualization (/insights links)
- [ ] 7.x.7.8 - Template integration (hyperlinks: section)
- [ ] 7.x.7.9 - Tests (extraction, classification, scoring)

#### 7.x.8 - End-to-End Integration (Week 6, 12h)
- [ ] 7.x.8.1 - Full pipeline test (quarantine → distribution)
- [ ] 7.x.8.2 - Performance benchmarks (<30s for 1000 messages)
- [ ] 7.x.8.3 - Memory profiling (<500MB peak)
- [ ] 7.x.8.4 - Error recovery tests (partial failures)
- [ ] 7.x.8.5 - User testing (3-5 users, satisfaction >8.0/10)
- [ ] 7.x.8.6 - Iteration based on feedback (adjust templates)
- [ ] 7.x.8.7 - Documentation (user, developer, template guides)

---

## 🎯 Quick Start Guide

### For Developers

**1. Read Architecture Document First:**
```bash
# Comprehensive 1,800-line design document
cat ROADMAP_V2/01_ARQUITECTURA/METABOLIC_DIGESTION_SYSTEM.md
```

**2. Create Feature Branch:**
```bash
git checkout -b feature/v1.2-data-import
```

**3. Start with Quarantine Layer:**
```bash
# Create module structure
mkdir -p src/import/quarantine
touch src/import/quarantine/mod.rs
touch src/import/quarantine/inspection.rs
touch src/import/quarantine/state.rs
```

**4. Follow Task Order:**
- Start with 7.x.1.1 (QuarantineZone struct)
- Test after each subtask
- Commit after each completed task
- Update CHECKLIST_V2.md progress

**5. Run Tests Continuously:**
```bash
cargo test --package bitacora --lib import::quarantine
```

### For Template Authors

**1. Study Existing Templates:**
```bash
cat templates/digesters/base_chat.yaml
cat templates/digesters/whatsapp_v1.yaml
```

**2. Create New Template:**
```bash
cp templates/digesters/base_chat.yaml templates/digesters/telegram_v1.yaml
# Edit telegram_v1.yaml with Telegram-specific rules
```

**3. Validate Template:**
```bash
cargo run -- /test template telegram_v1.yaml sample_data.json
```

**4. Run A/B Test:**
```bash
cargo run -- /compare telegram_v1.yaml telegram_v2.yaml sample_data.json
```

**5. Deploy to Production:**
```bash
# Hot reload without restart
cargo run -- /reload templates
```

### For Users

**1. Import Your Data:**
```bash
# WhatsApp export (from phone: Settings → Chats → Export Chat)
cargo run -- /import whatsapp chat_backup.txt

# Or auto-detect from Downloads folder
cargo run -- /import auto ~/Downloads
```

**2. Review Quarantine:**
```bash
# List all quarantined items
cargo run -- /quarantine list

# Inspect specific item
cargo run -- /quarantine inspect abc123

# Approve for digestion
cargo run -- /quarantine approve abc123
```

**3. Wait for Digestion:**
```
⏳ Processing... (30s for 1000 messages)
✅ Extracted: 250 biographical facts
✅ Extracted: 45 interest topics
✅ Extracted: 120 relationships
✅ Extracted: 80 hyperlinks
```

**4. View Insights:**
```bash
# Hyperlink intelligence
cargo run -- /insights links

# Memory recall (see imported data in action)
cargo run -- /memory
```

**5. Provide Feedback:**
```bash
# Rate accuracy (1-10)
cargo run -- /feedback accuracy 9

# Report incorrect extraction
cargo run -- /feedback incorrect "Thought I was 25, but I'm 26"
```

---

## 🔗 Related Documents

### Architecture & Design
- **[METABOLIC_DIGESTION_SYSTEM.md](01_ARQUITECTURA/METABOLIC_DIGESTION_SYSTEM.md)** (1,800 lines)
  - Complete architecture document
  - Detailed component designs
  - Code examples and diagrams
  - Success metrics and KPIs

### Implementation Tracking
- **[CHECKLIST_V2.md](CHECKLIST_V2.md)** - Detailed task list (43 tasks)
- **[CHECKLIST_TREE_V2.md](CHECKLIST_TREE_V2.md)** - Hierarchical tree view
- **[GUIA.md](GUIA.md)** - Methodology and philosophy

### Context & History
- **[SESION_EPICA_NOV28_2025.md](SESION_EPICA_NOV28_2025.md)** - Phase 7 completion session (9 hours)
- **[METOD_DOCS.md](METOD_DOCS.md)** - Documentation methodology

### Code Examples (Future)
- **examples/test_quarantine.rs** - Quarantine layer tests
- **examples/test_digestion.rs** - Source-specific digesters
- **examples/test_extraction.rs** - Nutrient extraction
- **examples/test_hyperlinks.rs** - Hyperlink intelligence
- **examples/test_data_import_e2e.rs** - End-to-end pipeline

---

## 🎉 Why This Matters

### Competitive Advantage

**ChatGPT/Claude:**
- ❌ No data import from external platforms
- ❌ Start from zero every conversation
- ❌ No hyperlink intelligence
- ❌ No consumption pattern analysis

**Bitácora with Phase 7.x:**
- ✅ Import WhatsApp, Telegram, Spotify, GitHub, etc.
- ✅ Instant deep understanding from day 1
- ✅ Hyperlink intelligence (unique insights)
- ✅ Efficiency scoring (how you use your time)
- ✅ Template-driven evolution (rapid iteration)

### User Experience

**Before Phase 7.x:**
```
User: Hi Bitácora
Bitácora: Tell me about yourself...
User: [30 minutes of Q&A] 😫
```

**After Phase 7.x:**
```
User: /import whatsapp chat_backup.txt
Bitácora: ⏳ Processing... [30 seconds]
Bitácora: ✅ Done! I understand you now:
  - Software engineer, loves Rust 🦀
  - Active at 6-10am, night owl 🌙
  - Shares educational content (Curator 📚)
  - 85% efficiency score (impressive!) ⚡
User: Let's chat! 😊
```

### Strategic Value

1. **Invisible Onboarding:** Users see value in 30 seconds (vs 30 minutes)
2. **Data-Driven Insights:** Hyperlink intelligence reveals hidden patterns
3. **Rapid Evolution:** Templates enable quick iteration without code changes
4. **Competitive Moat:** Unique feature that competitors don't have
5. **User Retention:** Deep understanding from day 1 = users stay

---

## 📞 Support & Contribution

### Need Help?

- **Architecture Questions:** Read [METABOLIC_DIGESTION_SYSTEM.md](01_ARQUITECTURA/METABOLIC_DIGESTION_SYSTEM.md)
- **Task Tracking:** Check [CHECKLIST_V2.md](CHECKLIST_V2.md)
- **Methodology:** Review [GUIA.md](GUIA.md)
- **Bug Reports:** Create issue with `[Phase 7.x]` tag

### Contributing

1. Read architecture document first
2. Pick task from CHECKLIST_V2.md
3. Create feature branch (`feature/7.x.N-description`)
4. Write tests first (TDD)
5. Implement feature
6. Update checklist progress
7. Submit PR with task number in title

### Testing Your Changes

```bash
# Unit tests
cargo test --package bitacora --lib import

# Integration tests
cargo test --test test_data_import_e2e

# Performance benchmarks
cargo bench --bench digestion_pipeline

# Template validation
cargo run -- /test template your_template.yaml sample_data.json
```

---

## 🚀 Let's Build This!

Phase 7.x is ambitious but achievable. The architecture is solid, the tasks are defined, and the vision is clear.

**Week 1 starts now. Let's make onboarding invisible! 🎯**

---

**Generated:** 2025-11-29 11:00:00  
**Version:** 1.0  
**Status:** ✅ Ready for Implementation  
**Next Steps:** Create feature branch → Start 7.x.1.1 (QuarantineZone struct)
