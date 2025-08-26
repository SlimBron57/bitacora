//! GitHub connector for accessing repositories, issues, and documentation

use super::{ExternalConnector, ExternalDocument, ContentType, SourceMetadata};
use async_trait::async_trait;
use bitacora_core::models::analysis::*;
use crate::errors::{AnalyticsError, AnalyticsResult};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct GitHubConnector {
    client: Client,
}

impl GitHubConnector {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    
    async fn get_repository_info(&self, config: &ConnectorConfig) -> AnalyticsResult<Value> {
        let token = match &config.authentication {
            AuthenticationConfig::Bearer(token) => token,
            AuthenticationConfig::ApiKey(token) => token,
            _ => return Err(AnalyticsError::authentication_failed(config.endpoint.clone())),
        };
        
        let url = format!("https://api.github.com/repos/{}", config.endpoint);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Bitacora-Analytics")
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(AnalyticsError::external_source_error(
                config.endpoint.clone(),
                &format!("GitHub API error: {}", response.status())
            ));
        }
        
        let repo_info: Value = response.json().await?;
        Ok(repo_info)
    }
    
    async fn get_repository_contents(&self, config: &ConnectorConfig, path: &str) -> AnalyticsResult<Vec<ExternalDocument>> {
        let token = match &config.authentication {
            AuthenticationConfig::Bearer(token) => token,
            AuthenticationConfig::ApiKey(token) => token,
            _ => return Err(AnalyticsError::authentication_failed(config.endpoint.clone())),
        };
        
        let url = format!("https://api.github.com/repos/{}/contents/{}", config.endpoint, path);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Bitacora-Analytics")
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(AnalyticsError::external_source_error(
                config.endpoint.clone(),
                &format!("GitHub API error: {}", response.status())
            ));
        }
        
        let contents: Value = response.json().await?;
        let mut documents = Vec::new();
        
        if let Some(items) = contents.as_array() {
            for item in items {
                if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                    if item_type == "file" {
                        if let Some(doc) = self.process_github_file(item, config).await? {
                            documents.push(doc);
                        }
                    } else if item_type == "dir" {
                        // Recursively process directories
                        if let Some(name) = item.get("name").and_then(|n| n.as_str()) {
                            let sub_path = if path.is_empty() { name.to_string() } else { format!("{}/{}", path, name) };
                            let mut sub_docs = self.get_repository_contents(config, &sub_path).await?;
                            documents.append(&mut sub_docs);
                        }
                    }
                }
            }
        }
        
        Ok(documents)
    }
    
    async fn process_github_file(&self, file_info: &Value, config: &ConnectorConfig) -> AnalyticsResult<Option<ExternalDocument>> {
        let name = file_info.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");
        let download_url = file_info.get("download_url").and_then(|u| u.as_str());
        
        // Filter for documentation files
        let is_doc_file = name.ends_with(".md") || 
                         name.ends_with(".rst") || 
                         name.ends_with(".txt") ||
                         name.to_lowercase().contains("readme") ||
                         name.to_lowercase().contains("doc");
                         
        if !is_doc_file {
            return Ok(None);
        }
        
        if let Some(url) = download_url {
            let content_response = self.client.get(url).send().await?;
            let content = content_response.text().await?;
            
            let content_type = if name.ends_with(".md") {
                ContentType::Markdown
            } else if name.ends_with(".rst") {
                ContentType::Other("rst".to_string())
            } else {
                ContentType::PlainText
            };
            
            let mut metadata = HashMap::new();
            if let Some(size) = file_info.get("size") {
                metadata.insert("size".to_string(), size.clone());
            }
            if let Some(path) = file_info.get("path") {
                metadata.insert("path".to_string(), path.clone());
            }
            
            Ok(Some(ExternalDocument {
                document_id: file_info.get("sha").and_then(|s| s.as_str()).unwrap_or("unknown").to_string(),
                title: name.to_string(),
                content,
                content_type,
                url: file_info.get("html_url").and_then(|u| u.as_str()).map(|s| s.to_string()),
                last_modified: None, // GitHub API doesn't provide this directly
                author: None,
                tags: vec!["github".to_string(), "documentation".to_string()],
                metadata,
                relevance_score: None,
            }))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl ExternalConnector for GitHubConnector {
    fn connector_type(&self) -> &str {
        "github"
    }
    
    async fn test_connection(&self, config: &ConnectorConfig) -> AnalyticsResult<bool> {
        match self.get_repository_info(config).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    async fn sync_data(&self, source: &ExternalSource) -> AnalyticsResult<Vec<ExternalDocument>> {
        self.get_repository_contents(&source.connector_config, "").await
    }
    
    async fn search_content(&self, source: &ExternalSource, query: &str) -> AnalyticsResult<Vec<ExternalDocument>> {
        // GitHub search API implementation
        let token = match &source.connector_config.authentication {
            AuthenticationConfig::Bearer(token) => token,
            AuthenticationConfig::ApiKey(token) => token,
            _ => return Err(AnalyticsError::authentication_failed(source.source_id.to_string())),
        };
        
        let search_url = format!(
            "https://api.github.com/search/code?q={}+repo:{}", 
            urlencoding::encode(query), 
            source.connector_config.endpoint
        );
        
        let response = self.client
            .get(&search_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Bitacora-Analytics")
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(AnalyticsError::external_source_error(
                source.source_id.to_string(),
                &format!("GitHub search error: {}", response.status())
            ));
        }
        
        let search_results: Value = response.json().await?;
        let mut documents = Vec::new();
        
        if let Some(items) = search_results.get("items").and_then(|i| i.as_array()) {
            for item in items.iter().take(10) { // Limit results
                if let Some(doc) = self.process_github_file(item, &source.connector_config).await? {
                    documents.push(doc);
                }
            }
        }
        
        Ok(documents)
    }
    
    async fn get_source_metadata(&self, config: &ConnectorConfig) -> AnalyticsResult<SourceMetadata> {
        let repo_info = self.get_repository_info(config).await?;
        
        Ok(SourceMetadata {
            total_documents: None, // Would need to traverse entire repo
            last_updated: repo_info.get("updated_at")
                .and_then(|u| u.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            available_filters: vec![
                super::FilterOption {
                    field: "path".to_string(),
                    display_name: "File Path".to_string(),
                    filter_type: super::FilterType::Text,
                    possible_values: vec![],
                },
                super::FilterOption {
                    field: "extension".to_string(),
                    display_name: "File Extension".to_string(),
                    filter_type: super::FilterType::Select,
                    possible_values: vec![".md".to_string(), ".rst".to_string(), ".txt".to_string()],
                },
            ],
            rate_limits: Some(super::RateLimits {
                requests_per_hour: Some(5000),
                requests_per_day: None,
                concurrent_requests: Some(10),
            }),
            supported_operations: vec![
                super::SupportedOperation::Read,
                super::SupportedOperation::Search,
            ],
        })
    }
}
