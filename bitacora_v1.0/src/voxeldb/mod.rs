//! # 🧊 VoxelDB - Base de Datos Cúbica de Templates
//!
//! VoxelDB es el segundo componente del sistema dual-helix de Bitácora v1.0,
//! diseñado para almacenar y recuperar templates accionables en geometría cúbica (x, y, z).
//!
//! ## Arquitectura Dual-Helix
//!
//! ```text
//! TelescopeDB (Esférica)    ←→    VoxelDB (Cúbica)
//! Memoria Biográfica               Templates Accionables
//! Lo que VIVISTE                   Lo que HACES
//! ```
//!
//! ## Uso Básico
//!
//! ```rust,no_run
//! use bitacora::voxeldb::{VoxelDB, CubicCoords, TemplateEntry};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Crear nueva instancia de VoxelDB
//!     let mut voxel = VoxelDB::new(PathBuf::from(".bitacora/voxel"))?;
//!     
//!     // Insertar template
//!     let template = TemplateEntry {
//!         name: "debugging_deep_dive".to_string(),
//!         category: TemplateCategory::Technical,
//!         content: "...".to_string(),
//!         coords: CubicCoords { x: 0.8, y: 0.5, z: 0.6 },
//!     };
//!     
//!     let id = voxel.insert_template(template).await?;
//!     
//!     // Query espacial
//!     let coords = CubicCoords { x: 0.75, y: 0.5, z: 0.6 };
//!     let results = voxel.query_spatial(coords, 0.3)?;
//!     
//!     println!("Encontrados {} templates cercanos", results.len());
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// QPX Native Format
use crate::qpx::{QPXEncoder, QPXDecoder, QPXQuantumCore, TemplateMetadata as QPXTemplateMetadata, Pixel};
use crate::fbcu::{FBCUCore, CompressionType, FBCUMetadata};

// Re-exports públicos
pub use crate::voxeldb::octree::{Octree, OctreeNode};

// Submódulos
pub mod octree;

/// Errores específicos de VoxelDB
#[derive(Debug, thiserror::Error)]
pub enum VoxelDBError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("Invalid cubic coordinates: x={x}, y={y}, z={z}")]
    InvalidCoordinates { x: f64, y: f64, z: f64 },
    
    #[error("Spatial index error: {0}")]
    SpatialIndexError(String),
    
    #[error("Invalid template category")]
    InvalidCategory,
    
    #[error("Template validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("TelescopeDB reference not found: {0}")]
    TelescopeRefNotFound(String),
}

/// Alias para Result con VoxelDBError
pub type Result<T> = std::result::Result<T, VoxelDBError>;

/// Coordenadas cúbicas 3D normalizadas [0, 1]
///
/// - **x**: Categoría (0.0 = technical, 0.33 = creative, 0.66 = emotional, 1.0 = meta)
/// - **y**: Complejidad (0.0 = trivial, 0.33 = simple, 0.66 = complex, 1.0 = expert)
/// - **z**: Efectividad (0.0 = no usado, 0.5 = promedio, 1.0 = altamente efectivo)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CubicCoords {
    /// Categoría: [0, 1]
    pub x: f64,
    
    /// Complejidad: [0, 1]
    pub y: f64,
    
    /// Efectividad: [0, 1]
    pub z: f64,
}

impl CubicCoords {
    /// Crear nuevas coordenadas con validación
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&x) {
            return Err(VoxelDBError::InvalidCoordinates { x, y, z });
        }
        if !(0.0..=1.0).contains(&y) {
            return Err(VoxelDBError::InvalidCoordinates { x, y, z });
        }
        if !(0.0..=1.0).contains(&z) {
            return Err(VoxelDBError::InvalidCoordinates { x, y, z });
        }
        
        Ok(Self { x, y, z })
    }
    
    /// Calcular distancia euclidiana entre dos coordenadas
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// Discretizar a grid para Octree (resolución típica: 100)
    pub fn to_grid(&self, resolution: usize) -> (usize, usize, usize) {
        let x_idx = ((self.x * resolution as f64).floor() as usize).min(resolution - 1);
        let y_idx = ((self.y * resolution as f64).floor() as usize).min(resolution - 1);
        let z_idx = ((self.z * resolution as f64).floor() as usize).min(resolution - 1);
        (x_idx, y_idx, z_idx)
    }
}

/// Categorías de templates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateCategory {
    /// Templates técnicos (debugging, arquitectura, código)
    Technical,
    
    /// Templates creativos (brainstorming, diseño, escritura)
    Creative,
    
    /// Templates emocionales (reflexión, mindfulness)
    Emotional,
    
    /// Templates analíticos (análisis de datos, comparación)
    Analytical,
    
    /// Templates colaborativos (reuniones, retrospectivas)
    Collaborative,
    
    /// Meta-templates (templates sobre templates)
    Meta,
}

impl TemplateCategory {
    /// Convertir categoría a coordenada x normalizada
    pub fn to_x_coord(&self) -> f64 {
        match self {
            Self::Technical => 0.0,
            Self::Creative => 0.2,
            Self::Emotional => 0.4,
            Self::Analytical => 0.6,
            Self::Collaborative => 0.8,
            Self::Meta => 1.0,
        }
    }
    
    /// Parsear desde string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "technical" => Ok(Self::Technical),
            "creative" => Ok(Self::Creative),
            "emotional" => Ok(Self::Emotional),
            "analytical" => Ok(Self::Analytical),
            "collaborative" => Ok(Self::Collaborative),
            "meta" => Ok(Self::Meta),
            _ => Err(VoxelDBError::InvalidCategory),
        }
    }
}

/// Entrada de template en VoxelDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateEntry {
    /// ID único (content-addressable SHA-256)
    pub id: String,
    
    /// Nombre del template (ej: "debugging_deep_dive")
    pub name: String,
    
    /// Categoría del template
    pub category: TemplateCategory,
    
    /// Coordenadas cúbicas
    pub coords: CubicCoords,
    
    /// Contenido del template (YAML, Markdown, etc.)
    pub content: String,
    
    /// Tags para clasificación
    pub tags: Vec<String>,
    
    /// Referencias a TelescopeDB (experiencias biográficas relacionadas)
    #[serde(default)]
    pub telescope_refs: Vec<String>,
    
    /// Métricas de efectividad
    pub effectiveness: EffectivenessMetrics,
    
    /// Metadatos adicionales
    pub metadata: TemplateMetadata,
    
    /// Nombre de archivo original (con extensión)
    #[serde(default)]
    pub original_filename: Option<String>,
    
    /// Extensión del archivo original (.md, .rs, .toml, etc.)
    #[serde(default)]
    pub file_extension: Option<String>,
}

impl TemplateEntry {
    /// Crear nuevo template con valores por defecto
    pub fn new(name: String, category: TemplateCategory, content: String) -> Self {
        let coords = CubicCoords {
            x: category.to_x_coord(),
            y: 0.5, // Complejidad media por defecto
            z: 0.5, // Efectividad inicial promedio
        };
        
        Self {
            id: Self::generate_id(&name, &content),
            name,
            category,
            coords,
            content,
            tags: Vec::new(),
            telescope_refs: Vec::new(),
            effectiveness: EffectivenessMetrics::default(),
            metadata: TemplateMetadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                version: "1.0".to_string(),
                author: "bitacora".to_string(),
            },
            original_filename: None,
            file_extension: None,
        }
    }
    
    /// Generar ID content-addressable
    fn generate_id(name: &str, content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        hasher.update(content.as_bytes());
        let result = hasher.finalize();
        format!("vdb_{:x}", result).chars().take(20).collect()
    }
}

/// Métricas de efectividad de un template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectivenessMetrics {
    /// Número de veces usado
    pub usage_count: usize,
    
    /// Tasa de completitud (% de secciones completadas)
    pub completeness_rate: f64,
    
    /// Tasa de validación (% de pasos que pasan validación)
    pub validation_pass_rate: f64,
    
    /// Número promedio de iteraciones hasta éxito
    pub avg_iterations: f64,
    
    /// Feedback del usuario (-1 = malo, 0 = neutral, +1 = bueno)
    pub user_feedback: Vec<i8>,
    
    /// Score agregado calculado
    pub effectiveness_score: f64,
}

impl Default for EffectivenessMetrics {
    fn default() -> Self {
        Self {
            usage_count: 0,
            completeness_rate: 0.0,
            validation_pass_rate: 0.0,
            avg_iterations: 0.0,
            user_feedback: Vec::new(),
            effectiveness_score: 0.5, // Inicial: promedio
        }
    }
}

impl EffectivenessMetrics {
    /// Calcular effectiveness score según fórmula MTT-DSL
    pub fn calculate_score(&self) -> f64 {
        if self.usage_count == 0 {
            return 0.5; // Sin datos, asumimos promedio
        }
        
        let avg_feedback = if !self.user_feedback.is_empty() {
            self.user_feedback.iter().map(|&f| f as f64).sum::<f64>() 
                / self.user_feedback.len() as f64
        } else {
            0.0
        };
        
        // Normalizar feedback de [-1, 1] a [0, 1]
        let feedback_norm = (avg_feedback + 1.0) / 2.0;
        
        // Penalty por muchas iteraciones
        let iteration_penalty = 1.0 / (self.avg_iterations + 1.0);
        
        // Bonus por uso frecuente
        let usage_bonus = if self.usage_count > 5 { 0.1 } else { 0.0 };
        
        // Fórmula ponderada
        let score = (self.completeness_rate * 0.3)
            + (self.validation_pass_rate * 0.3)
            + (feedback_norm * 0.2)
            + (iteration_penalty * 0.1)
            + usage_bonus;
        
        score.min(1.0).max(0.0)
    }
    
    /// Actualizar métricas después de un uso
    pub fn update_after_use(
        &mut self,
        completeness: f64,
        validation_passed: bool,
        iterations: usize,
        feedback: i8,
    ) {
        self.usage_count += 1;
        
        // Actualizar promedio ponderado de completeness
        self.completeness_rate = (self.completeness_rate * (self.usage_count - 1) as f64 
            + completeness) / self.usage_count as f64;
        
        // Actualizar tasa de validación
        let validation_value = if validation_passed { 1.0 } else { 0.0 };
        self.validation_pass_rate = (self.validation_pass_rate * (self.usage_count - 1) as f64 
            + validation_value) / self.usage_count as f64;
        
        // Actualizar promedio de iteraciones
        self.avg_iterations = (self.avg_iterations * (self.usage_count - 1) as f64 
            + iterations as f64) / self.usage_count as f64;
        
        // Añadir feedback
        self.user_feedback.push(feedback);
        
        // Recalcular score
        self.effectiveness_score = self.calculate_score();
    }
}

/// Metadatos del template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateMetadata {
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
    
    /// Versión del template
    pub version: String,
    
    /// Autor del template
    pub author: String,
}

/// Estructura principal de VoxelDB
pub struct VoxelDB {
    /// Directorio de almacenamiento
    storage_path: PathBuf,
    
    /// Templates en memoria (ID → TemplateEntry)
    templates: HashMap<String, TemplateEntry>,
    
    /// Índice espacial Octree
    spatial_index: Octree<String>, // Almacena IDs de templates
    
    /// Índice por categoría
    category_index: HashMap<TemplateCategory, Vec<String>>,
    
    /// Índice por nombre
    name_index: HashMap<String, String>, // name → id
}

impl VoxelDB {
    /// Crear nueva instancia de VoxelDB
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        // Crear directorio si no existe
        if !storage_path.exists() {
            std::fs::create_dir_all(&storage_path)?;
        }
        
        // Crear índice espacial Octree (resolución 100)
        let spatial_index = Octree::new(100);
        
        Ok(Self {
            storage_path,
            templates: HashMap::new(),
            spatial_index,
            category_index: HashMap::new(),
            name_index: HashMap::new(),
        })
    }
    
    /// Insertar template en VoxelDB
    pub fn insert_template(&mut self, mut template: TemplateEntry) -> Result<String> {
        // Validar coordenadas
        if template.coords.x < 0.0 || template.coords.x > 1.0 ||
           template.coords.y < 0.0 || template.coords.y > 1.0 ||
           template.coords.z < 0.0 || template.coords.z > 1.0 {
            return Err(VoxelDBError::InvalidCoordinates {
                x: template.coords.x,
                y: template.coords.y,
                z: template.coords.z,
            });
        }
        
        // Note: Nombre puede duplicarse (ej: conceptos repetidos en múltiples archivos)
        // El ID (hash) garantiza unicidad real
        // name_index almacena solo la primera ocurrencia para búsqueda rápida
        
        // Actualizar metadata
        template.metadata.updated_at = Utc::now();
        
        let id = template.id.clone();
        
        // Guardar en disco
        self.save_template_to_disk(&template)?;
        
        // Indexar en Octree
        self.spatial_index.insert(template.coords, id.clone());
        
        // Indexar por categoría
        self.category_index
            .entry(template.category)
            .or_insert_with(Vec::new)
            .push(id.clone());
        
        // Indexar por nombre (solo primera ocurrencia para búsqueda rápida)
        self.name_index.entry(template.name.clone())
            .or_insert_with(|| id.clone());
        
        // Guardar en memoria
        self.templates.insert(id.clone(), template);
        
        Ok(id)
    }
    
    /// Query espacial: templates en vecindad de coordenadas
    pub fn query_spatial(&self, coords: CubicCoords, radius: f64) -> Result<Vec<TemplateEntry>> {
        // Buscar IDs en Octree
        let ids = self.spatial_index.query_sphere(coords, radius);
        
        // Cargar templates y ordenar por distancia
        let mut results: Vec<(TemplateEntry, f64)> = ids
            .iter()
            .filter_map(|id| {
                self.templates.get(id).map(|template| {
                    let distance = coords.distance(&template.coords);
                    (template.clone(), distance)
                })
            })
            .collect();
        
        // Ordenar por distancia (más cercano primero)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        Ok(results.into_iter().map(|(t, _)| t).collect())
    }
    
    /// Obtener template por ID
    pub fn get_by_id(&self, id: &str) -> Result<TemplateEntry> {
        self.templates
            .get(id)
            .cloned()
            .ok_or_else(|| VoxelDBError::TemplateNotFound(id.to_string()))
    }
    
    /// Obtener template por nombre
    pub fn get_by_name(&self, name: &str) -> Result<TemplateEntry> {
        let id = self.name_index
            .get(name)
            .ok_or_else(|| VoxelDBError::TemplateNotFound(name.to_string()))?;
        self.get_by_id(id)
    }
    
    /// Query por categoría
    pub fn query_by_category(&self, category: TemplateCategory) -> Result<Vec<TemplateEntry>> {
        let ids = self.category_index
            .get(&category)
            .ok_or_else(|| VoxelDBError::TemplateNotFound(format!("{:?}", category)))?;
        
        Ok(ids
            .iter()
            .filter_map(|id| self.templates.get(id).cloned())
            .collect())
    }
    
    /// Actualizar efectividad de un template
    pub fn update_effectiveness(
        &mut self,
        template_id: &str,
        completeness: f64,
        validation_passed: bool,
        iterations: usize,
        feedback: i8,
    ) -> Result<()> {
        let template = self.templates
            .get_mut(template_id)
            .ok_or_else(|| VoxelDBError::TemplateNotFound(template_id.to_string()))?;
        
        // Actualizar métricas
        template.effectiveness.update_after_use(
            completeness,
            validation_passed,
            iterations,
            feedback,
        );
        
        // Actualizar coordenada z (efectividad)
        let old_coords = template.coords;
        template.coords.z = template.effectiveness.effectiveness_score;
        
        // Re-indexar en Octree (coordenadas cambiaron)
        let template_id_string = template_id.to_string();
        self.spatial_index.remove(old_coords, &template_id_string);
        self.spatial_index.insert(template.coords, template_id_string);
        
        // Actualizar metadata
        template.metadata.updated_at = Utc::now();
        
        // Clonar template para guardar (evitar conflicto de borrow)
        let template_clone = template.clone();
        
        // Guardar cambios en disco
        self.save_template_to_disk(&template_clone)?;
        
        Ok(())
    }
    
    /// Vincular template con entrada biográfica de TelescopeDB
    pub fn link_to_telescope(&mut self, template_id: &str, telescope_id: &str) -> Result<()> {
        let template = self.templates
            .get_mut(template_id)
            .ok_or_else(|| VoxelDBError::TemplateNotFound(template_id.to_string()))?;
        
        // Añadir referencia si no existe ya
        if !template.telescope_refs.contains(&telescope_id.to_string()) {
            template.telescope_refs.push(telescope_id.to_string());
        }
        
        // Actualizar metadata
        template.metadata.updated_at = Utc::now();
        
        // Clonar template para guardar (evitar conflicto de borrow)
        let template_clone = template.clone();
        
        // Guardar cambios
        self.save_template_to_disk(&template_clone)?;
        
        Ok(())
    }
    
    /// Obtener templates más efectivos (top-k)
    pub fn get_top_effective(&self, k: usize) -> Vec<TemplateEntry> {
        let mut templates: Vec<_> = self.templates.values().cloned().collect();
        
        // Ordenar por effectiveness_score descendente
        templates.sort_by(|a, b| {
            b.effectiveness.effectiveness_score
                .partial_cmp(&a.effectiveness.effectiveness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        templates.into_iter().take(k).collect()
    }
    
    /// Obtener estadísticas de VoxelDB
    pub fn stats(&self) -> VoxelDBStats {
        VoxelDBStats {
            total_templates: self.templates.len(),
            by_category: self.category_index
                .iter()
                .map(|(cat, ids)| (*cat, ids.len()))
                .collect(),
            avg_effectiveness: if self.templates.is_empty() {
                0.0
            } else {
                self.templates.values()
                    .map(|t| t.effectiveness.effectiveness_score)
                    .sum::<f64>() / self.templates.len() as f64
            },
            total_usages: self.templates.values()
                .map(|t| t.effectiveness.usage_count)
                .sum(),
        }
    }
    
    /// Get storage path (for disk usage calculation)
    pub fn storage_path(&self) -> &Path {
        &self.storage_path
    }
    
    // === Operaciones de persistencia ===
    
    /// Guardar template en disco
    fn save_template_to_disk(&self, template: &TemplateEntry) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", template.id));
        let json = serde_json::to_string_pretty(template)
            .map_err(|e| VoxelDBError::SerializationError(e.to_string()))?;
        std::fs::write(file_path, json)?;
        Ok(())
    }
    
    /// Cargar template desde disco
    fn load_template_from_disk(&self, id: &str) -> Result<TemplateEntry> {
        let file_path = self.storage_path.join(format!("{}.json", id));
        let json = std::fs::read_to_string(file_path)?;
        let template = serde_json::from_str(&json)
            .map_err(|e| VoxelDBError::SerializationError(e.to_string()))?;
        Ok(template)
    }

    // ========================================================================
    // QPX NATIVE FORMAT METHODS (v1.5+)
    // ========================================================================

    /// Generate QPX storage path with year/month partitioning
    /// 
    /// Example: data/voxel/templates/2025/11/202511_abc123def456.qpxf
    pub fn get_template_qpx_path(&self, template_id: &str, timestamp: DateTime<Utc>) -> PathBuf {
        let year = timestamp.format("%Y").to_string();
        let month = timestamp.format("%m").to_string();
        let month_prefix = timestamp.format("%Y%m").to_string();
        
        self.storage_path
            .join("templates")
            .join(&year)
            .join(&month)
            .join(format!("{}_{}.qpxf", month_prefix, template_id))
    }

    /// Write template as QPX binary format
    /// 
    /// Converts TemplateEntry → QPXQuantumCore → .qpxf binary file
    /// 
    /// # Arguments
    /// * `template` - Template to encode
    /// * `fbcu_core` - Compressed FBCU data
    /// * `pixels` - 384D embedding (128 pixels × 3 RGB)
    /// 
    /// # Returns
    /// * Path where .qpxt file was written
    pub fn write_template_qpx(
        &self,
        template: &TemplateEntry,
        fbcu_core: FBCUCore,
        pixels: Vec<Pixel>,
    ) -> Result<PathBuf> {
        // Create QPX metadata
        let qpx_metadata = QPXTemplateMetadata {
            concept_name: template.name.clone(),
            category: format!("{:?}", template.category),
            tags: template.tags.clone(),
            original_path: String::new(), // Not stored in QPX (only in VoxelDB index)
            original_filename: template.original_filename.clone(),
            file_extension: template.file_extension.clone(),
        };

        // Determine alpha based on effectiveness
        let alpha = if template.effectiveness.effectiveness_score > 0.8 {
            255 // Core template (highly effective)
        } else if template.effectiveness.effectiveness_score > 0.5 {
            128 // Helper template (moderately effective)
        } else {
            50  // Deprecated template (low effectiveness)
        };

        // Create quantum core
        let quantum_core = QPXQuantumCore {
            fbcu_core,
            pixels,
            alpha,
            metadata: qpx_metadata,
            timestamp: template.metadata.created_at,
            checksum: self.compute_template_checksum(template),
        };

        // Encode to QPX binary
        let qpx_bytes = QPXEncoder::encode_quantum_core(&quantum_core)
            .map_err(|e| VoxelDBError::SerializationError(format!("QPX encoding failed: {}", e)))?;

        // Generate path with year/month partitioning
        let qpx_path = self.get_template_qpx_path(&template.id, template.metadata.created_at);

        // Create directory structure
        if let Some(parent) = qpx_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write to disk
        std::fs::write(&qpx_path, qpx_bytes)?;

        Ok(qpx_path)
    }

    /// Read template from QPX binary format
    /// 
    /// # Arguments
    /// * `qpx_path` - Path to .qpxt file
    /// 
    /// # Returns
    /// * Decoded QPXQuantumCore with all template data
    pub fn read_template_qpx(&self, qpx_path: &Path) -> Result<QPXQuantumCore> {
        let qpx_bytes = std::fs::read(qpx_path)?;
        
        QPXDecoder::decode_quantum_core(&qpx_bytes)
            .map_err(|e| VoxelDBError::SerializationError(format!("QPX decoding failed: {}", e)))
    }

    /// Find QPX file for a template ID
    /// 
    /// Searches in year/month directories for the template's .qpxt file
    pub fn find_template_qpx(&self, template_id: &str) -> Result<PathBuf> {
        let templates_dir = self.storage_path.join("templates");
        
        if !templates_dir.exists() {
            return Err(VoxelDBError::TemplateNotFound(
                format!("QPX templates directory not found: {:?}", templates_dir)
            ));
        }

        // Search in year directories
        for year_entry in std::fs::read_dir(&templates_dir)? {
            let year_entry = year_entry?;
            if !year_entry.file_type()?.is_dir() {
                continue;
            }

            // Search in month directories
            for month_entry in std::fs::read_dir(year_entry.path())? {
                let month_entry = month_entry?;
                if !month_entry.file_type()?.is_dir() {
                    continue;
                }

                // Search for .qpxf files matching template_id
                for file_entry in std::fs::read_dir(month_entry.path())? {
                    let file_entry = file_entry?;
                    let path = file_entry.path();
                    
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        if filename.contains(template_id) && filename.ends_with(".qpxf") {
                            return Ok(path);
                        }
                    }
                }
            }
        }

        Err(VoxelDBError::TemplateNotFound(
            format!("QPX file not found for template: {}", template_id)
        ))
    }

    /// Compute SHA256 checksum for template content
    fn compute_template_checksum(&self, template: &TemplateEntry) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(template.content.as_bytes());
        
        let result = hasher.finalize();
        let mut checksum = [0u8; 32];
        checksum.copy_from_slice(&result);
        checksum
    }
    
    /// Cargar todos los templates desde disco
    pub fn load_all_from_disk(&mut self) -> Result<usize> {
        let mut count = 0;
        
        for entry in std::fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    match self.load_template_from_disk(stem) {
                        Ok(template) => {
                            let id = template.id.clone();
                            
                            // Re-indexar
                            self.spatial_index.insert(template.coords, id.clone());
                            self.category_index
                                .entry(template.category)
                                .or_insert_with(Vec::new)
                                .push(id.clone());
                            self.name_index.insert(template.name.clone(), id.clone());
                            
                            // Guardar en memoria
                            self.templates.insert(id, template);
                            count += 1;
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to load template {}: {}", stem, e);
                        }
                    }
                }
            }
        }
        
        Ok(count)
    }
}

/// Estadísticas de VoxelDB
#[derive(Debug, Clone)]
pub struct VoxelDBStats {
    pub total_templates: usize,
    pub by_category: HashMap<TemplateCategory, usize>,
    pub avg_effectiveness: f64,
    pub total_usages: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cubic_coords_distance() {
        let c1 = CubicCoords::new(0.0, 0.0, 0.0).unwrap();
        let c2 = CubicCoords::new(1.0, 1.0, 1.0).unwrap();
        
        let distance = c1.distance(&c2);
        assert!((distance - 1.732).abs() < 0.01); // √3 ≈ 1.732
    }
    
    #[test]
    fn test_cubic_coords_validation() {
        assert!(CubicCoords::new(0.5, 0.5, 0.5).is_ok());
        assert!(CubicCoords::new(-0.1, 0.5, 0.5).is_err());
        assert!(CubicCoords::new(0.5, 1.5, 0.5).is_err());
        assert!(CubicCoords::new(0.5, 0.5, -0.1).is_err());
    }
    
    #[test]
    fn test_template_category_to_coord() {
        assert_eq!(TemplateCategory::Technical.to_x_coord(), 0.0);
        assert_eq!(TemplateCategory::Creative.to_x_coord(), 0.2);
        assert_eq!(TemplateCategory::Meta.to_x_coord(), 1.0);
    }
    
    #[test]
    fn test_effectiveness_calculation() {
        let mut metrics = EffectivenessMetrics::default();
        
        // Simular uso exitoso
        metrics.update_after_use(0.9, true, 2, 1);
        metrics.update_after_use(0.85, true, 1, 1);
        metrics.update_after_use(0.95, true, 1, 1);
        
        assert!(metrics.effectiveness_score > 0.7);
        assert_eq!(metrics.usage_count, 3);
    }
    
    #[test]
    fn test_template_entry_creation() {
        let template = TemplateEntry::new(
            "test_template".to_string(),
            TemplateCategory::Technical,
            "content".to_string(),
        );
        
        assert_eq!(template.name, "test_template");
        assert_eq!(template.category, TemplateCategory::Technical);
        assert_eq!(template.coords.x, 0.0); // Technical = 0.0
        assert_eq!(template.coords.y, 0.5); // Default complejidad
        assert_eq!(template.coords.z, 0.5); // Default efectividad
    }

    #[test]
    fn test_qpx_path_generation() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let voxel = VoxelDB::new(temp_dir.path().to_path_buf()).unwrap();
        
        let timestamp = DateTime::parse_from_rfc3339("2025-11-30T15:30:00Z")
            .unwrap()
            .with_timezone(&Utc);
        
        let path = voxel.get_template_qpx_path("abc123def456", timestamp);
        
        assert!(path.to_string_lossy().contains("templates"));
        assert!(path.to_string_lossy().contains("2025"));
        assert!(path.to_string_lossy().contains("11"));
        assert!(path.to_string_lossy().contains("202511_abc123def456.qpxf"));
    }

    #[test]
    fn test_qpx_write_read_roundtrip() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let voxel = VoxelDB::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Create test template
        let template = TemplateEntry::new(
            "test_qpx_template".to_string(),
            TemplateCategory::Technical,
            "Test content for QPX".to_string(),
        );

        // Create FBCU core (compressed data)
        let fbcu_core = FBCUCore {
            id: "qpx_test_123".into(),
            compression_type: CompressionType::Hybrid,
            compressed_data: vec![1, 2, 3, 4, 5],
            original_size: 1000,
            compression_ratio: 0.005,
            metadata: FBCUMetadata {
                compressed_at: Utc::now().to_rfc3339(),
                compression_time_ms: 10,
                original_hash: "test_hash".into(),
                wavelet_level: Some(3),
                fractal_level: None,
            },
        };

        // Create pixels (128 pixels for 384D embedding)
        // Alpha should match template effectiveness: 0.5 (default) -> alpha=128
        let pixels = vec![Pixel::new(100, 150, 200, 128); 128];

        // Write QPX
        let qpx_path = voxel.write_template_qpx(&template, fbcu_core, pixels).unwrap();
        
        // Verify file exists
        assert!(qpx_path.exists());
        assert!(qpx_path.to_string_lossy().ends_with(".qpxf"));

        // Read QPX back
        let decoded = voxel.read_template_qpx(&qpx_path).unwrap();
        
        // Verify data
        assert_eq!(decoded.metadata.concept_name, "test_qpx_template");
        assert_eq!(decoded.metadata.category, "Technical");
        assert_eq!(decoded.pixels.len(), 128);
        assert_eq!(decoded.alpha, 128); // Default effectiveness (0.5) = helper template
        assert_eq!(decoded.fbcu_core.compressed_data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_qpx_alpha_determination() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let voxel = VoxelDB::new(temp_dir.path().to_path_buf()).unwrap();
        
        // High effectiveness template
        let mut template_high = TemplateEntry::new(
            "high_effectiveness".to_string(),
            TemplateCategory::Technical,
            "content".to_string(),
        );
        template_high.effectiveness.effectiveness_score = 0.9;

        let fbcu = FBCUCore {
            id: "test".into(),
            compression_type: CompressionType::Hybrid,
            compressed_data: vec![1],
            original_size: 100,
            compression_ratio: 0.01,
            metadata: FBCUMetadata {
                compressed_at: Utc::now().to_rfc3339(),
                compression_time_ms: 5,
                original_hash: "hash".into(),
                wavelet_level: None,
                fractal_level: None,
            },
        };

        let pixels = vec![Pixel::new(50, 50, 50, 255); 64];
        let path = voxel.write_template_qpx(&template_high, fbcu, pixels).unwrap();
        let decoded = voxel.read_template_qpx(&path).unwrap();
        
        assert_eq!(decoded.alpha, 255); // Core template
    }

    #[test]
    fn test_find_template_qpx() {
        use tempfile::tempdir;
        
        let temp_dir = tempdir().unwrap();
        let voxel = VoxelDB::new(temp_dir.path().to_path_buf()).unwrap();
        
        // Create and write template
        let template = TemplateEntry::new(
            "findable_template".to_string(),
            TemplateCategory::Creative,
            "content".to_string(),
        );

        let fbcu = FBCUCore {
            id: "find_test".into(),
            compression_type: CompressionType::Hybrid,
            compressed_data: vec![1, 2, 3],
            original_size: 500,
            compression_ratio: 0.006,
            metadata: FBCUMetadata {
                compressed_at: Utc::now().to_rfc3339(),
                compression_time_ms: 8,
                original_hash: "xyz".into(),
                wavelet_level: Some(2),
                fractal_level: None,
            },
        };

        let pixels = vec![Pixel::new(75, 100, 125, 128); 96];
        voxel.write_template_qpx(&template, fbcu, pixels).unwrap();

        // Find the template
        let found_path = voxel.find_template_qpx(&template.id).unwrap();
        
        assert!(found_path.exists());
        assert!(found_path.to_string_lossy().contains(&template.id));
    }
}
