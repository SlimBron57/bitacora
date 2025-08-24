//! File system connector for accessing local documentation

use super::{ExternalConnector, ExternalDocument, ContentType, SourceMetadata};
use async_trait::async_trait;
use bitacora_core::models::analysis::*;
use crate::errors::{AnalyticsError, AnalyticsResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;

pub struct FileSystemConnector;

impl FileSystemConnector {
    pub fn new() -> Self {
        Self
    }
    
    async fn scan_directory(&self, path: &Path) -> AnalyticsResult<Vec<ExternalDocument>> {
        let mut documents = Vec::new();
        let mut entries = fs::read_dir(path).await.map_err(|e| {
            AnalyticsError::external_source_error(
                path.to_string_lossy().to_string(),
                &format!("Failed to read directory: {}", e)
            )
        })?;
        
        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AnalyticsError::external_source_error(
                path.to_string_lossy().to_string(),
                &format!("Failed to read directory entry: {}", e)
            )
        })? {
            let entry_path = entry.path();
            
            if entry_path.is_file() {
                if let Some(doc) = self.process_file(&entry_path).await? {
                    documents.push(doc);
                }
            } else if entry_path.is_dir() {
                // Recursively process subdirectories
                let mut sub_docs = self.scan_directory(&entry_path).await?;
                documents.append(&mut sub_docs);
            }
        }
        
        Ok(documents)
    }
    
    async fn process_file(&self, file_path: &Path) -> AnalyticsResult<Option<ExternalDocument>> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        // Filter for documentation files
        let is_doc_file = matches!(extension.to_lowercase().as_str(), 
            "md" | "rst" | "txt" | "adoc" | "tex" | "org" | "wiki"
        ) || file_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_lowercase())
            .map(|name| name.contains("readme") || name.contains("doc") || name.contains("changelog"))
            .unwrap_or(false);
            
        if !is_doc_file {
            return Ok(None);
        }
        
        let content = fs::read_to_string(file_path).await.map_err(|e| {
            AnalyticsError::external_source_error(
                file_path.to_string_lossy().to_string(),
                &format!("Failed to read file: {}", e)
            )
        })?;
        
        let metadata_result = fs::metadata(file_path).await.map_err(|e| {
            AnalyticsError::external_source_error(
                file_path.to_string_lossy().to_string(),
                &format!("Failed to read file metadata: {}", e)
            )
        })?;
        
        let content_type = match extension.to_lowercase().as_str() {
            "md" => ContentType::Markdown,
            "rst" => ContentType::Other("restructuredtext".to_string()),
            "adoc" => ContentType::Other("asciidoc".to_string()),
            "tex" => ContentType::Other("latex".to_string()),
            "org" => ContentType::Other("org-mode".to_string()),
            "wiki" => ContentType::Other("wiki".to_string()),
            _ => ContentType::PlainText,
        };
        
        let mut metadata = HashMap::new();
        metadata.insert("size".to_string(), serde_json::Value::Number(
            serde_json::Number::from(metadata_result.len())
        ));
        metadata.insert("path".to_string(), serde_json::Value::String(
            file_path.to_string_lossy().to_string()
        ));
        
        let title = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Unknown Document")
            .to_string();
        
        // Generate tags based on file path and content
        let mut tags = vec!["filesystem".to_string(), "documentation".to_string()];
        
        // Add directory-based tags
        if let Some(parent) = file_path.parent() {
            if let Some(dir_name) = parent.file_name().and_then(|n| n.to_str()) {
                tags.push(format!("directory:{}", dir_name));
            }
        }
        
        // Add extension-based tags
        tags.push(format!("type:{}", extension));
        
        // Add content-based tags
        let content_lower = content.to_lowercase();
        if content_lower.contains("api") {
            tags.push("api".to_string());
        }
        if content_lower.contains("tutorial") {
            tags.push("tutorial".to_string());
        }
        if content_lower.contains("guide") {
            tags.push("guide".to_string());
        }
        
        let last_modified = metadata_result
            .modified()
            .ok()
            .and_then(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|duration| {
                        chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                            .unwrap_or_default()
                    })
            });
        
        Ok(Some(ExternalDocument {
            document_id: self.generate_document_id(file_path),
            title,
            content,
            content_type,
            url: Some(format!("file://{}", file_path.to_string_lossy())),
            last_modified,
            author: None, // Could potentially extract from git history
            tags,
            metadata,
            relevance_score: None,
        }))
    }
    
    fn generate_document_id(&self, file_path: &Path) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        file_path.to_string_lossy().hash(&mut hasher);
        format!("filesystem:{:x}", hasher.finish())
    }
}

#[async_trait]
impl ExternalConnector for FileSystemConnector {
    fn connector_type(&self) -> &str {
        "filesystem"
    }
    
    async fn test_connection(&self, config: &ConnectorConfig) -> AnalyticsResult<bool> {
        let path = Path::new(&config.endpoint);
        Ok(path.exists())
    }
    
    async fn sync_data(&self, source: &ExternalSource) -> AnalyticsResult<Vec<ExternalDocument>> {
        let path = Path::new(&source.connector_config.endpoint);
        
        if path.is_file() {
            // Single file
            if let Some(doc) = self.process_file(path).await? {
                Ok(vec![doc])
            } else {
                Ok(vec![])
            }
        } else if path.is_dir() {
            // Directory
            self.scan_directory(path).await
        } else {
            Err(AnalyticsError::external_source_error(
                source.source_id.to_string(),
                "Path does not exist or is not accessible"
            ))
        }
    }
    
    async fn search_content(&self, source: &ExternalSource, query: &str) -> AnalyticsResult<Vec<ExternalDocument>> {
        let documents = self.sync_data(source).await?;
        let query_lower = query.to_lowercase();
        
        let filtered_documents: Vec<ExternalDocument> = documents
            .into_iter()
            .filter(|doc| {
                doc.content.to_lowercase().contains(&query_lower) ||
                doc.title.to_lowercase().contains(&query_lower) ||
                doc.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .collect();
        
        Ok(filtered_documents)
    }
    
    async fn get_source_metadata(&self, config: &ConnectorConfig) -> AnalyticsResult<SourceMetadata> {
        let path = Path::new(&config.endpoint);
        
        if !path.exists() {
            return Err(AnalyticsError::external_source_error(
                config.endpoint.clone(),
                "Path does not exist"
            ));
        }
        
        let metadata_result = fs::metadata(path).await.map_err(|e| {
            AnalyticsError::external_source_error(
                config.endpoint.clone(),
                &format!("Failed to read metadata: {}", e)
            )
        })?;
        
        let last_modified = metadata_result
            .modified()
            .ok()
            .and_then(|time| {
                time.duration_since(std::time::UNIX_EPOCH)
                    .ok()
                    .map(|duration| {
                        chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                            .unwrap_or_default()
                    })
            });
        
        // Count documents if it's a directory
        let total_documents = if path.is_dir() {
            // This is a rough estimate - we'd need to traverse to get exact count
            None
        } else {
            Some(1)
        };
        
        Ok(SourceMetadata {
            total_documents,
            last_updated: last_modified,
            available_filters: vec![
                super::FilterOption {
                    field: "extension".to_string(),
                    display_name: "File Extension".to_string(),
                    filter_type: super::FilterType::Select,
                    possible_values: vec![
                        ".md".to_string(), ".rst".to_string(), ".txt".to_string(),
                        ".adoc".to_string(), ".tex".to_string(), ".org".to_string()
                    ],
                },
                super::FilterOption {
                    field: "directory".to_string(),
                    display_name: "Directory".to_string(),
                    filter_type: super::FilterType::Text,
                    possible_values: vec![],
                },
                super::FilterOption {
                    field: "size".to_string(),
                    display_name: "File Size".to_string(),
                    filter_type: super::FilterType::Range,
                    possible_values: vec![],
                },
            ],
            rate_limits: None, // No rate limits for filesystem
            supported_operations: vec![
                super::SupportedOperation::Read,
                super::SupportedOperation::Search,
            ],
        })
    }
}
