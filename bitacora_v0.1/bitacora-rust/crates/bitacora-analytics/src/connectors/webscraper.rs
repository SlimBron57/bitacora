//! Web scraper connector for extracting documentation from websites

use super::{ExternalConnector, ExternalDocument, ContentType, SourceMetadata};
use async_trait::async_trait;
use bitacora_core::models::analysis::*;
use crate::errors::{AnalyticsError, AnalyticsResult};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

pub struct WebScraperConnector {
    client: Client,
}

impl WebScraperConnector {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Bitacora-Analytics-WebScraper/1.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }
    
    async fn scrape_page(&self, url: &str) -> AnalyticsResult<ExternalDocument> {
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(AnalyticsError::external_source_error(
                url.to_string(),
                &format!("HTTP error: {}", response.status())
            ));
        }
        
        let content_type = response.headers()
            .get("content-type")
            .and_then(|ct| ct.to_str().ok())
            .unwrap_or("text/html");
            
        if !content_type.contains("text/html") && !content_type.contains("text/plain") {
            return Err(AnalyticsError::external_source_error(
                url.to_string(),
                "Unsupported content type for web scraping"
            ));
        }
        
        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);
        
        // Extract title
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|element| element.text().collect::<Vec<_>>().join(""))
            .unwrap_or_else(|| {
                // Fallback: extract from URL
                Url::parse(url)
                    .ok()
                    .and_then(|u| u.path_segments())
                    .and_then(|segments| segments.last())
                    .unwrap_or("Unknown Page")
                    .to_string()
            });
        
        // Extract main content using common selectors
        let content_selectors = vec![
            "main", "article", ".content", "#content", ".post-content", 
            ".documentation", ".docs", ".markdown-body", ".wiki-content"
        ];
        
        let mut extracted_content = String::new();
        let mut found_content = false;
        
        for selector_str in content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    if !text.trim().is_empty() && text.len() > 100 {
                        extracted_content = text;
                        found_content = true;
                        break;
                    }
                }
                if found_content {
                    break;
                }
            }
        }
        
        // Fallback: extract from body if no specific content found
        if !found_content {
            if let Ok(body_selector) = Selector::parse("body") {
                if let Some(body) = document.select(&body_selector).next() {
                    // Remove script and style elements
                    let mut body_text = String::new();
                    self.extract_text_recursive(&body, &mut body_text);
                    extracted_content = body_text;
                }
            }
        }
        
        // Clean up the content
        extracted_content = self.clean_extracted_text(&extracted_content);
        
        // Extract metadata
        let mut metadata = HashMap::new();
        
        // Meta description
        if let Ok(meta_selector) = Selector::parse("meta[name='description']") {
            if let Some(meta) = document.select(&meta_selector).next() {
                if let Some(description) = meta.value().attr("content") {
                    metadata.insert("description".to_string(), serde_json::Value::String(description.to_string()));
                }
            }
        }
        
        // Meta keywords
        if let Ok(meta_selector) = Selector::parse("meta[name='keywords']") {
            if let Some(meta) = document.select(&meta_selector).next() {
                if let Some(keywords) = meta.value().attr("content") {
                    metadata.insert("keywords".to_string(), serde_json::Value::String(keywords.to_string()));
                }
            }
        }
        
        // Extract tags from common patterns
        let mut tags = vec!["web".to_string(), "documentation".to_string()];
        
        // Add domain as tag
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(domain) = parsed_url.host_str() {
                tags.push(format!("domain:{}", domain));
            }
        }
        
        // Add content type based on URL patterns
        let url_lower = url.to_lowercase();
        if url_lower.contains("docs") || url_lower.contains("documentation") {
            tags.push("documentation".to_string());
        }
        if url_lower.contains("api") {
            tags.push("api".to_string());
        }
        if url_lower.contains("tutorial") {
            tags.push("tutorial".to_string());
        }
        if url_lower.contains("guide") {
            tags.push("guide".to_string());
        }
        
        Ok(ExternalDocument {
            document_id: self.generate_document_id(url),
            title: title.trim().to_string(),
            content: extracted_content,
            content_type: ContentType::Html,
            url: Some(url.to_string()),
            last_modified: None, // Could extract from Last-Modified header if available
            author: None,
            tags,
            metadata,
            relevance_score: None,
        })
    }
    
    fn extract_text_recursive(&self, element: &scraper::ElementRef, text: &mut String) {
        for child in element.children() {
            match child.value() {
                scraper::Node::Text(t) => {
                    text.push_str(&t.text);
                    text.push(' ');
                }
                scraper::Node::Element(e) => {
                    // Skip script and style elements
                    if e.name() != "script" && e.name() != "style" {
                        if let Some(child_element) = scraper::ElementRef::wrap(child) {
                            self.extract_text_recursive(&child_element, text);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    
    fn clean_extracted_text(&self, text: &str) -> String {
        // Remove excessive whitespace and clean up the text
        let mut cleaned = text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        
        // Remove multiple consecutive newlines
        while cleaned.contains("\n\n\n") {
            cleaned = cleaned.replace("\n\n\n", "\n\n");
        }
        
        cleaned.trim().to_string()
    }
    
    fn generate_document_id(&self, url: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        format!("webscraper:{:x}", hasher.finish())
    }
}

#[async_trait]
impl ExternalConnector for WebScraperConnector {
    fn connector_type(&self) -> &str {
        "webscraper"
    }
    
    async fn test_connection(&self, config: &ConnectorConfig) -> AnalyticsResult<bool> {
        let test_response = self.client
            .head(&config.endpoint)
            .send()
            .await;
            
        match test_response {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    async fn sync_data(&self, source: &ExternalSource) -> AnalyticsResult<Vec<ExternalDocument>> {
        // For single page scraping
        let document = self.scrape_page(&source.connector_config.endpoint).await?;
        Ok(vec![document])
    }
    
    async fn search_content(&self, source: &ExternalSource, query: &str) -> AnalyticsResult<Vec<ExternalDocument>> {
        // For web scraping, we can't really search unless we implement site-specific search
        // For now, we'll scrape the page and filter content
        let document = self.scrape_page(&source.connector_config.endpoint).await?;
        
        // Simple text matching
        let query_lower = query.to_lowercase();
        let content_lower = document.content.to_lowercase();
        
        if content_lower.contains(&query_lower) || 
           document.title.to_lowercase().contains(&query_lower) {
            Ok(vec![document])
        } else {
            Ok(vec![])
        }
    }
    
    async fn get_source_metadata(&self, config: &ConnectorConfig) -> AnalyticsResult<SourceMetadata> {
        // Try to get basic info about the webpage
        let response = self.client
            .head(&config.endpoint)
            .send()
            .await?;
            
        let last_modified = response
            .headers()
            .get("last-modified")
            .and_then(|lm| lm.to_str().ok())
            .and_then(|lm_str| chrono::DateTime::parse_from_rfc2822(lm_str).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc));
        
        Ok(SourceMetadata {
            total_documents: Some(1), // Single page scraping
            last_updated: last_modified,
            available_filters: vec![
                super::FilterOption {
                    field: "domain".to_string(),
                    display_name: "Website Domain".to_string(),
                    filter_type: super::FilterType::Text,
                    possible_values: vec![],
                },
                super::FilterOption {
                    field: "content_type".to_string(),
                    display_name: "Content Type".to_string(),
                    filter_type: super::FilterType::Select,
                    possible_values: vec!["documentation".to_string(), "tutorial".to_string(), "api".to_string()],
                },
            ],
            rate_limits: Some(super::RateLimits {
                requests_per_hour: Some(100), // Conservative rate limiting
                requests_per_day: None,
                concurrent_requests: Some(3),
            }),
            supported_operations: vec![
                super::SupportedOperation::Read,
                super::SupportedOperation::Search,
            ],
        })
    }
}
