//! # Template Repository System 🗃️
//!
//! ## 🎯 Sistema Avanzado de Gestión de Templates
//!
//! Este módulo implementa un sistema completo de repositorio de templates BitaFlow:
//! - **Persistencia**: Storage en filesystem y database
//! - **Discovery**: Búsqueda inteligente con filtros
//! - **Versioning**: Control de versiones automático
//! - **Sharing**: Import/export para ecosistema de templates

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tokio::fs as async_fs;
use chrono::{DateTime, Utc};

use crate::errors::{NavigatorError, Result as NavigatorResult};
use crate::bitaflow::{NavigatorTemplate, NavigatorMetrics};

/// Repositorio de templates BitaFlow
#[derive(Debug, Clone)]
pub struct TemplateRepository {
    /// Ruta base del repositorio
    base_path: PathBuf,
    /// Templates cargados en memoria
    templates: HashMap<String, StoredTemplate>,
    /// Índice de búsqueda
    search_index: TemplateSearchIndex,
}

/// Template almacenado con metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTemplate {
    /// Template original
    pub template: NavigatorTemplate,
    /// Metadata de storage
    pub storage_metadata: TemplateStorageMetadata,
    /// Tags para búsqueda
    pub tags: Vec<String>,
    /// Categorías
    pub categories: Vec<String>,
}

/// Metadata de almacenamiento de template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStorageMetadata {
    /// ID único del template
    pub id: String,
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Fecha de última modificación
    pub updated_at: DateTime<Utc>,
    /// Autor del template
    pub author: String,
    /// Tamaño del archivo BFL
    pub file_size: u64,
    /// Hash del contenido para integridad
    pub content_hash: String,
    /// Path relativo en el repositorio
    pub relative_path: String,
}

/// Índice de búsqueda de templates
#[derive(Debug, Clone, Default)]
pub struct TemplateSearchIndex {
    /// Índice por dominio
    pub by_domain: HashMap<String, Vec<String>>,
    /// Índice por topic
    pub by_topic: HashMap<String, Vec<String>>,
    /// Índice por tags
    pub by_tags: HashMap<String, Vec<String>>,
    /// Índice de texto completo
    pub full_text: HashMap<String, Vec<String>>,
}

/// Filtros para búsqueda de templates
#[derive(Debug, Clone, Default)]
pub struct TemplateSearchFilters {
    /// Filtrar por dominio
    pub domain: Option<String>,
    /// Filtrar por topic
    pub topic: Option<String>,
    /// Filtrar por tags
    pub tags: Vec<String>,
    /// Búsqueda de texto
    pub text_search: Option<String>,
    /// Filtrar por autor
    pub author: Option<String>,
    /// Rango de fechas
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    /// Filtrar por métricas mínimas
    pub min_success_rate: Option<f64>,
}

/// Resultado de búsqueda
#[derive(Debug, Clone)]
pub struct TemplateSearchResult {
    /// Template encontrado
    pub template: StoredTemplate,
    /// Score de relevancia
    pub relevance_score: f64,
    /// Razón del match
    pub match_reason: String,
}

impl TemplateRepository {
    /// Crear nuevo repositorio
    pub async fn new<P: AsRef<Path>>(base_path: P) -> NavigatorResult<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        
        // Crear directorio base si no existe
        if !base_path.exists() {
            async_fs::create_dir_all(&base_path).await
                .map_err(|e| NavigatorError::configuration(format!("Failed to create repository directory: {}", e)))?;
        }
        
        let mut repo = Self {
            base_path,
            templates: HashMap::new(),
            search_index: TemplateSearchIndex::default(),
        };
        
        // Cargar templates existentes
        repo.load_existing_templates().await?;
        
        Ok(repo)
    }
    
    /// Almacenar template en el repositorio
    pub async fn store_template(&mut self, template: NavigatorTemplate) -> NavigatorResult<String> {
        println!("📦 Storing template: {} ({})", template.name, template.alias);
        
        // Generar metadata
        let storage_metadata = self.generate_storage_metadata(&template).await?;
        let template_id = storage_metadata.id.clone();
        
        // Crear stored template
        let stored_template = StoredTemplate {
            template: template.clone(),
            storage_metadata: storage_metadata.clone(),
            tags: self.extract_tags(&template),
            categories: self.extract_categories(&template),
        };
        
        // Crear path del archivo
        let file_path = self.base_path.join(&storage_metadata.relative_path);
        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent).await
                .map_err(|e| NavigatorError::configuration(format!("Failed to create template directory: {}", e)))?;
        }
        
        // Escribir archivo BFL
        async_fs::write(&file_path, &template.bfl_content).await
            .map_err(|e| NavigatorError::configuration(format!("Failed to write template file: {}", e)))?;
        
        // Escribir metadata
        let metadata_path = file_path.with_extension("meta.json");
        let metadata_json = serde_json::to_string_pretty(&stored_template)
            .map_err(|e| NavigatorError::configuration(format!("Failed to serialize metadata: {}", e)))?;
        async_fs::write(&metadata_path, metadata_json).await
            .map_err(|e| NavigatorError::configuration(format!("Failed to write metadata file: {}", e)))?;
        
        // Actualizar índices
        self.templates.insert(template_id.clone(), stored_template.clone());
        self.update_search_index(&stored_template);
        
        println!("✅ Template stored successfully with ID: {}", template_id);
        Ok(template_id)
    }
    
    /// Buscar templates en el repositorio
    pub async fn search_templates(&self, filters: TemplateSearchFilters) -> NavigatorResult<Vec<TemplateSearchResult>> {
        println!("🔍 Searching templates with filters: {:#?}", filters);
        
        let mut results = Vec::new();
        
        for (template_id, stored_template) in &self.templates {
            if let Some(score) = self.calculate_match_score(stored_template, &filters) {
                let match_reason = self.generate_match_reason(stored_template, &filters);
                
                results.push(TemplateSearchResult {
                    template: stored_template.clone(),
                    relevance_score: score,
                    match_reason,
                });
            }
        }
        
        // Ordenar por relevancia
        results.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
        
        println!("📋 Found {} matching templates", results.len());
        Ok(results)
    }
    
    /// Obtener template por ID
    pub async fn get_template(&self, template_id: &str) -> NavigatorResult<Option<StoredTemplate>> {
        Ok(self.templates.get(template_id).cloned())
    }
    
    /// Listar todos los templates
    pub async fn list_all_templates(&self) -> Vec<&StoredTemplate> {
        self.templates.values().collect()
    }
    
    /// Actualizar métricas de un template
    pub async fn update_template_metrics(&mut self, template_id: &str, metrics: NavigatorMetrics) -> NavigatorResult<()> {
        if let Some(stored_template) = self.templates.get_mut(template_id) {
            stored_template.template.metrics = metrics;
            stored_template.storage_metadata.updated_at = Utc::now();
            
            // Clone the template for persistence to avoid borrowing issues
            let stored_template_clone = stored_template.clone();
            
            println!("📊 Updated metrics for template: {}", template_id);
            
            // Persistir cambios
            self.persist_template_metadata(&stored_template_clone).await?;
        }
        
        Ok(())
    }
    
    // Métodos privados de implementación
    
    async fn load_existing_templates(&mut self) -> NavigatorResult<()> {
        println!("📂 Loading existing templates from: {:?}", self.base_path);
        
        if !self.base_path.exists() {
            return Ok(());
        }
        
        let mut entries = async_fs::read_dir(&self.base_path).await
            .map_err(|e| NavigatorError::configuration(format!("Failed to read repository directory: {}", e)))?;
        
        let mut loaded_count = 0;
        while let Some(entry) = entries.next_entry().await.map_err(|e| NavigatorError::configuration(format!("Failed to read directory entry: {}", e)))? {
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") && path.file_name().map_or(false, |name| name.to_string_lossy().ends_with(".meta.json")) {
                if let Ok(stored_template) = self.load_template_from_metadata(&path).await {
                    let template_id = stored_template.storage_metadata.id.clone();
                    self.templates.insert(template_id, stored_template.clone());
                    self.update_search_index(&stored_template);
                    loaded_count += 1;
                }
            }
        }
        
        println!("✅ Loaded {} existing templates", loaded_count);
        Ok(())
    }
    
    async fn load_template_from_metadata(&self, metadata_path: &Path) -> NavigatorResult<StoredTemplate> {
        let metadata_content = async_fs::read_to_string(metadata_path).await
            .map_err(|e| NavigatorError::configuration(format!("Failed to read metadata file: {}", e)))?;
        
        let stored_template: StoredTemplate = serde_json::from_str(&metadata_content)
            .map_err(|e| NavigatorError::configuration(format!("Failed to parse metadata: {}", e)))?;
        
        Ok(stored_template)
    }
    
    async fn generate_storage_metadata(&self, template: &NavigatorTemplate) -> NavigatorResult<TemplateStorageMetadata> {
        let now = Utc::now();
        let template_id = uuid::Uuid::new_v4().to_string();
        
        // Generar path relativo basado en dominio/topic
        let relative_path = format!("{}/{}/{}.bfl", 
            template.domain, 
            template.topic, 
            template.alias
        );
        
        // Calcular hash del contenido
        let content_hash = self.calculate_content_hash(&template.bfl_content);
        
        Ok(TemplateStorageMetadata {
            id: template_id,
            created_at: now,
            updated_at: now,
            author: std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            file_size: template.bfl_content.len() as u64,
            content_hash,
            relative_path,
        })
    }
    
    fn calculate_content_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    fn extract_tags(&self, template: &NavigatorTemplate) -> Vec<String> {
        let mut tags = vec![
            template.domain.clone(),
            template.topic.clone(),
            format!("v{}", template.version),
            format!("{:?}", template.autonomy_level),
            format!("{:?}", template.thread_level),
        ];
        
        // Extraer tags del contenido BFL
        if template.bfl_content.contains("error") || template.bfl_content.contains("debug") {
            tags.push("debugging".to_string());
        }
        if template.bfl_content.contains("analysis") || template.bfl_content.contains("analyze") {
            tags.push("analysis".to_string());
        }
        if template.bfl_content.contains("test") {
            tags.push("testing".to_string());
        }
        
        tags.sort();
        tags.dedup();
        tags
    }
    
    fn extract_categories(&self, template: &NavigatorTemplate) -> Vec<String> {
        vec![
            "BitaFlow".to_string(),
            template.domain.clone(),
            format!("{:?}", template.autonomy_level),
        ]
    }
    
    fn update_search_index(&mut self, stored_template: &StoredTemplate) {
        let template_id = &stored_template.storage_metadata.id;
        let template = &stored_template.template;
        
        // Índice por dominio
        self.search_index.by_domain
            .entry(template.domain.clone())
            .or_insert_with(Vec::new)
            .push(template_id.clone());
        
        // Índice por topic
        self.search_index.by_topic
            .entry(template.topic.clone())
            .or_insert_with(Vec::new)
            .push(template_id.clone());
        
        // Índice por tags
        for tag in &stored_template.tags {
            self.search_index.by_tags
                .entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(template_id.clone());
        }
        
        // Índice de texto completo
        let full_text = format!("{} {} {}", template.name, template.description, template.bfl_content);
        for word in full_text.split_whitespace() {
            if word.len() > 2 {
                self.search_index.full_text
                    .entry(word.to_lowercase())
                    .or_insert_with(Vec::new)
                    .push(template_id.clone());
            }
        }
    }
    
    fn calculate_match_score(&self, stored_template: &StoredTemplate, filters: &TemplateSearchFilters) -> Option<f64> {
        let mut score = 0.0;
        let template = &stored_template.template;
        
        // Filtro por dominio
        if let Some(ref domain) = filters.domain {
            if template.domain == *domain {
                score += 10.0;
            } else {
                return None; // No match obligatorio
            }
        }
        
        // Filtro por topic
        if let Some(ref topic) = filters.topic {
            if template.topic == *topic {
                score += 8.0;
            } else if template.topic.contains(topic) {
                score += 4.0;
            }
        }
        
        // Filtro por tags
        for filter_tag in &filters.tags {
            if stored_template.tags.contains(filter_tag) {
                score += 5.0;
            }
        }
        
        // Búsqueda de texto
        if let Some(ref text_search) = filters.text_search {
            let search_text = text_search.to_lowercase();
            if template.name.to_lowercase().contains(&search_text) {
                score += 15.0;
            }
            if template.description.to_lowercase().contains(&search_text) {
                score += 10.0;
            }
            if template.bfl_content.to_lowercase().contains(&search_text) {
                score += 5.0;
            }
        }
        
        // Filtro por autor
        if let Some(ref author) = filters.author {
            if stored_template.storage_metadata.author == *author {
                score += 3.0;
            }
        }
        
        // Filtro por métricas
        if let Some(min_success_rate) = filters.min_success_rate {
            if template.metrics.success_rate >= min_success_rate {
                score += template.metrics.success_rate * 2.0;
            } else {
                score -= 5.0; // Penalizar templates con baja success rate
            }
        }
        
        // Bonus por popularidad (usage_count)
        score += (template.metrics.usage_count as f64).ln() * 2.0;
        
        if score > 0.0 {
            Some(score)
        } else {
            None
        }
    }
    
    fn generate_match_reason(&self, stored_template: &StoredTemplate, filters: &TemplateSearchFilters) -> String {
        let mut reasons = Vec::new();
        let template = &stored_template.template;
        
        if filters.domain.as_ref() == Some(&template.domain) {
            reasons.push(format!("Domain match: {}", template.domain));
        }
        
        if filters.topic.as_ref() == Some(&template.topic) {
            reasons.push(format!("Topic match: {}", template.topic));
        }
        
        for filter_tag in &filters.tags {
            if stored_template.tags.contains(filter_tag) {
                reasons.push(format!("Tag match: {}", filter_tag));
            }
        }
        
        if let Some(ref text_search) = filters.text_search {
            if template.name.to_lowercase().contains(&text_search.to_lowercase()) {
                reasons.push("Name contains search term".to_string());
            }
        }
        
        if reasons.is_empty() {
            "General relevance".to_string()
        } else {
            reasons.join(", ")
        }
    }
    
    async fn persist_template_metadata(&self, stored_template: &StoredTemplate) -> NavigatorResult<()> {
        let file_path = self.base_path.join(&stored_template.storage_metadata.relative_path);
        let metadata_path = file_path.with_extension("meta.json");
        
        let metadata_json = serde_json::to_string_pretty(stored_template)
            .map_err(|e| NavigatorError::configuration(format!("Failed to serialize metadata: {}", e)))?;
        
        async_fs::write(&metadata_path, metadata_json).await
            .map_err(|e| NavigatorError::configuration(format!("Failed to write metadata file: {}", e)))?;
        
        Ok(())
    }
}

/// Builder para TemplateSearchFilters
impl TemplateSearchFilters {
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }
    
    pub fn topic(mut self, topic: String) -> Self {
        self.topic = Some(topic);
        self
    }
    
    pub fn tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    pub fn text_search(mut self, text: String) -> Self {
        self.text_search = Some(text);
        self
    }
    
    pub fn author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }
    
    pub fn min_success_rate(mut self, rate: f64) -> Self {
        self.min_success_rate = Some(rate);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[tokio::test]
    async fn test_repository_creation() {
        let temp_dir = tempdir().unwrap();
        let repo = TemplateRepository::new(temp_dir.path()).await;
        assert!(repo.is_ok());
    }
    
    #[tokio::test] 
    async fn test_template_storage_and_retrieval() {
        let temp_dir = tempdir().unwrap();
        let mut repo = TemplateRepository::new(temp_dir.path()).await.unwrap();
        
        // Crear template de prueba
        let template = NavigatorTemplate {
            alias: "BITA-NAV-TEST-DEMO-v1".to_string(),
            name: "Test Demo Template".to_string(),
            description: "Template for testing".to_string(),
            domain: "test".to_string(),
            topic: "demo".to_string(),
            version: 1,
            autonomy_level: crate::bitaflow::AutonomyLevel::Interactive,
            thread_level: crate::bitaflow::ThreadLevel::ProjectIsolated,
            bfl_content: "# Test BFL Content\n1. Test step\n".to_string(),
            variables: HashMap::new(),
            metrics: NavigatorMetrics::default(),
        };
        
        // Almacenar template
        let template_id = repo.store_template(template).await.unwrap();
        
        // Recuperar template
        let retrieved = repo.get_template(&template_id).await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_template = retrieved.unwrap();
        assert_eq!(retrieved_template.template.alias, "BITA-NAV-TEST-DEMO-v1");
        assert_eq!(retrieved_template.template.domain, "test");
    }
}
