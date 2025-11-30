// bitacora_v1.0/src/watcher/indexer.rs
//! 📇 VoxelDB Indexer
//!
//! **Core**: Uses VoxelDB to index concepts from documentation.
//! **Philosophy**: Bitácora indexing itself in real-time.

use super::*;
use crate::voxeldb::{VoxelDB, TemplateEntry, TemplateCategory, CubicCoords};
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use anyhow::{Context, Result};

/// Concept indexer using VoxelDB
pub struct ConceptIndexer {
    voxel_db: VoxelDB,
    concept_patterns: Vec<ConceptPattern>,
}

/// Pattern to detect concepts in markdown
#[derive(Debug, Clone)]
struct ConceptPattern {
    name: String,
    pattern: Regex,
}

impl ConceptIndexer {
    /// Create new indexer
    /// 🎻 Loads existing concepts from VoxelDB on startup
    pub fn new(db_path: &Path) -> Result<Self> {
        let mut voxel_db = VoxelDB::new(db_path.to_path_buf())?;
        
        // 🏎️ Load all persisted templates from disk into memory
        let loaded = voxel_db.load_all_from_disk()?;
        if loaded > 0 {
            eprintln!("   💾 Loaded {} concepts from VoxelDB", loaded);
        }
        
        let concept_patterns = Self::build_patterns();
        
        Ok(Self {
            voxel_db,
            concept_patterns,
        })
    }
    
    /// Build regex patterns for concept detection
    fn build_patterns() -> Vec<ConceptPattern> {
        vec![
            // Markdown headings
            ConceptPattern {
                name: "heading".to_string(),
                pattern: Regex::new(r"^#+\s+(.+)$").unwrap(),
            },
            // [[wikilinks]]
            ConceptPattern {
                name: "wikilink".to_string(),
                pattern: Regex::new(r"\[\[([^\]]+)\]\]").unwrap(),
            },
            // File references [text](file.md)
            ConceptPattern {
                name: "fileref".to_string(),
                pattern: Regex::new(r"\[([^\]]+)\]\(([^)]+\.md)\)").unwrap(),
            },
            // Code blocks with language
            ConceptPattern {
                name: "codeblock".to_string(),
                pattern: Regex::new(r"```(\w+)").unwrap(),
            },
            // Tags #tag
            ConceptPattern {
                name: "tag".to_string(),
                pattern: Regex::new(r"#(\w+)").unwrap(),
            },
            // Decision Architecture references DA-XXX
            ConceptPattern {
                name: "da_ref".to_string(),
                pattern: Regex::new(r"DA-(\d{3})").unwrap(),
            },
        ]
    }
    
    /// Index a file
    /// 🏎️ Memory-conscious: reads file once, processes line-by-line (no big allocations)
    pub fn index_file(&mut self, path: &Path) -> Result<IndexResult> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {:?}", path))?;
        
        let mut result = IndexResult::default();
        
        // Collect matches first to avoid borrow checker issues
        // Vec capacity hint: most files have < 50 concepts (avoid reallocs)
        let mut matches = Vec::with_capacity(50);
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.concept_patterns {
                if let Some(captures) = pattern.pattern.captures(line) {
                    let concept = captures.get(1)
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default();
                    
                    if !concept.is_empty() {
                        matches.push((concept, line_num + 1, pattern.name.clone()));
                    }
                }
            }
        }
        
        // Now store all matches
        for (concept, line_num, pattern_name) in matches {
            self.store_concept(path, &concept, line_num, &pattern_name)?;
            result.concepts_found += 1;
        }
        
        // Extract metadata
        result.file_size = content.len();
        result.line_count = content.lines().count();
        
        // 🏎️ Drop content explicitly (free memory immediately, don't wait for scope end)
        drop(content);
        
        Ok(result)
    }
    
    /// Store concept in VoxelDB
    /// 🎻 Maps (file, line, concept) → 3D spatial coordinates in octree
    fn store_concept(
        &mut self,
        file: &Path,
        concept: &str,
        line: usize,
        pattern_type: &str,
    ) -> Result<()> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // 🏎️ Convert (file, line) → 3D coordinates
        // X: file hash (normalized 0.0-1.0)
        // Y: line number (normalized by max typical file length 10000)
        // Z: concept hash (normalized 0.0-1.0)
        
        let file_str = file.to_string_lossy();
        
        // Hash file path
        let mut hasher = DefaultHasher::new();
        file_str.hash(&mut hasher);
        let file_hash = hasher.finish();
        let x = (file_hash % 1000) as f64 / 1000.0;
        
        // Normalize line number (0-10000 → 0.0-1.0)
        let y = (line.min(10000) as f64) / 10000.0;
        
        // Hash concept
        let mut hasher = DefaultHasher::new();
        concept.hash(&mut hasher);
        let concept_hash = hasher.finish();
        let z = (concept_hash % 1000) as f64 / 1000.0;
        
        // Create VoxelDB template entry
        let coords = CubicCoords::new(x, y, z)?;
        
        // 🎻 Use clean concept name (VoxelDB now allows duplicates)
        let mut template = TemplateEntry::new(
            concept.to_string(),  // Clean name: "rust", "yaml", etc.
            TemplateCategory::Technical,
            String::new(), // Content not needed, using tags
        );
        
        // 🏎️ Use tags for metadata (cleaner + queryable)
        template.tags = vec![
            format!("file:{}", file_str),
            format!("line:{}", line),
            format!("type:{}", pattern_type),
        ];
        
        // Override coordinates with our calculated spatial position
        template.coords = coords;
        
        // Insert into VoxelDB octree (this persists to disk + indexes spatially)
        self.voxel_db.insert_template(template)?;
        
        Ok(())
    }
    
    /// Query concepts by pattern
    /// 🎻 Searches VoxelDB using name matching + spatial proximity
    pub fn query_concepts(&self, pattern: &str) -> Result<Vec<ConceptMatch>> {
        let mut matches = Vec::new();
        
        // Get all concepts (stored as Technical category)
        // Handle case where category might be empty
        let all_templates = self.voxel_db.query_by_category(TemplateCategory::Technical)
            .unwrap_or_default();
        
        if pattern.is_empty() {
            // Return all concepts
            for template in all_templates {
                matches.push(self.template_to_match(&template));
            }
        } else {
            // Search by name pattern (case-insensitive contains)
            let pattern_lower = pattern.to_lowercase();
            
            for template in all_templates {
                if template.name.to_lowercase().contains(&pattern_lower) {
                    matches.push(self.template_to_match(&template));
                }
            }
        }
        
        Ok(matches)
    }
    
    /// Convert TemplateEntry to ConceptMatch
    /// 🎻 Now using tags instead of content for metadata
    fn template_to_match(&self, template: &crate::voxeldb::TemplateEntry) -> ConceptMatch {
        // Parse tags: ["file:X", "line:Y", "type:Z"]
        let mut file = PathBuf::new();
        let mut line = 0;
        let mut context = String::from("unknown");
        
        for tag in &template.tags {
            if let Some(path) = tag.strip_prefix("file:") {
                file = PathBuf::from(path);
            } else if let Some(line_str) = tag.strip_prefix("line:") {
                line = line_str.parse().unwrap_or(0);
            } else if let Some(type_str) = tag.strip_prefix("type:") {
                context = type_str.to_string();
            }
        }
        
        ConceptMatch {
            concept: template.name.clone(),
            file,
            line,
            context,
        }
    }
    
    /// Get stats
    pub fn get_stats(&self) -> IndexerStats {
        let voxel_stats = self.voxel_db.stats();
        
        // Extract unique files from templates (Technical category)
        let all_templates = self.voxel_db.query_by_category(TemplateCategory::Technical)
            .unwrap_or_default();
        
        let unique_files: std::collections::HashSet<String> = all_templates
            .iter()
            .filter_map(|t| {
                t.content.split(';')
                    .next()
                    .and_then(|s| s.strip_prefix("file="))
                    .map(String::from)
            })
            .collect();
        
        // Calculate disk usage (approximate: count .json files in storage)
        let disk_usage = if let Ok(entries) = std::fs::read_dir(self.voxel_db.storage_path()) {
            entries.filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum()
        } else {
            0
        };
        
        IndexerStats {
            total_concepts: voxel_stats.total_templates,
            total_files: unique_files.len(),
            index_size_bytes: disk_usage,
        }
    }
}

/// Result of indexing a file
#[derive(Debug, Default)]
pub struct IndexResult {
    pub concepts_found: usize,
    pub file_size: usize,
    pub line_count: usize,
}

/// Concept match from query
#[derive(Debug, Clone)]
pub struct ConceptMatch {
    pub concept: String,
    pub file: PathBuf,
    pub line: usize,
    pub context: String,
}

/// Indexer statistics
#[derive(Debug, Default)]
pub struct IndexerStats {
    pub total_concepts: usize,
    pub total_files: usize,
    pub index_size_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_concept_patterns() {
        let patterns = ConceptIndexer::build_patterns();
        assert!(patterns.len() >= 5);
        
        // Test wikilink pattern
        let wikilink = patterns.iter()
            .find(|p| p.name == "wikilink")
            .unwrap();
        
        assert!(wikilink.pattern.is_match("[[concept-name]]"));
        assert!(!wikilink.pattern.is_match("[concept-name]"));
    }
    
    #[test]
    fn test_da_ref_pattern() {
        let patterns = ConceptIndexer::build_patterns();
        let da_ref = patterns.iter()
            .find(|p| p.name == "da_ref")
            .unwrap();
        
        assert!(da_ref.pattern.is_match("DA-036"));
        assert!(da_ref.pattern.is_match("Decision DA-042 approved"));
        assert!(!da_ref.pattern.is_match("DA-42")); // Must be 3 digits
    }
}
