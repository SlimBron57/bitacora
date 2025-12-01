//! # 🔭 TelescopeDB - Base de Datos Biográfica Esférica
//!
//! TelescopeDB es el componente de memoria episódica del sistema dual-helix de Bitácora v1.0.
//! Almacena y recupera memoria biográfica del usuario en geometría esférica (r, θ, φ).
//!
//! ## Arquitectura
//!
//! ```text
//! TelescopeDB
//! ├── pixel_storage       → Almacenamiento de píxeles con metadata
//! ├── memory_forensics    → Análisis temporal y pattern detection
//! └── snapshot_manager    → Gestión de snapshots y comparaciones
//! ```
//!
//! ## Ejemplo de Uso
//!
//! ```rust,no_run
//! use bitacora::telescopedb::TelescopeDB;
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Crear nueva instancia de TelescopeDB
//!     let mut telescope = TelescopeDB::new(PathBuf::from(".bitacora/telescope"))?;
//!     
//!     // Insertar FBCU Core desde Context Token 7D
//!     let ctx7d = create_context_token();
//!     let id = telescope.insert_from_ctx7d(&ctx7d).await?;
//!     
//!     // Query contextual (búsqueda esférica)
//!     let coords = SphericalCoords { r: 0.8, theta: 1.2, phi: 0.5 };
//!     let results = telescope.query_contextual(coords, 0.3).await?;
//!     
//!     println!("Found {} similar experiences", results.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod pixel_storage;
pub mod memory_forensics;
pub mod snapshot_manager;
pub mod biographical_import;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

// Re-exports
pub use pixel_storage::{PixelData, PixelStore};
pub use memory_forensics::{MemoryForensics, TimelineEvent};
pub use snapshot_manager::{Snapshot, SnapshotManager};
pub use biographical_import::{
    BiographicalImporter, BiographicalRawEntry, ImportResult, 
    SyntheticDataGenerator, import_from_sandbox,
};

/// Errores específicos de TelescopeDB
#[derive(Debug, Error)]
pub enum TelescopeDBError {
    #[error("FBCU Core not found: {0}")]
    CoreNotFound(String),

    #[error("Invalid spherical coordinates: r={r}, theta={theta}, phi={phi}")]
    InvalidCoordinates { r: f64, theta: f64, phi: f64 },

    #[error("Embedding generation failed: {0}")]
    EmbeddingFailed(String),

    #[error("Compression failed: {0}")]
    CompressionError(String),

    #[error("Spherical index error: {0}")]
    SphericalIndexError(String),

    #[error("HNSW index error: {0}")]
    HNSWError(String),

    #[error("VoxelDB not connected")]
    VoxelNotConnected,

    #[error("Import failed: {0}")]
    ImportError(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Context Token 7D conversion failed: {0}")]
    CTX7DConversionError(String),
}

pub type Result<T> = std::result::Result<T, TelescopeDBError>;

/// Coordenadas esféricas (r, θ, φ)
///
/// - `r`: Intensidad [0, ∞) - Calculado de dimensión emocional + intencional
/// - `theta`: Categoría temática [0, 2π) - 0=técnico, π/3=personal, etc.
/// - `phi`: Valencia emocional [0, π] - 0=muy positivo, π/2=neutral, π=muy negativo
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SphericalCoords {
    pub r: f64,
    pub theta: f64,
    pub phi: f64,
}

impl SphericalCoords {
    /// Crea nuevas coordenadas esféricas con validación
    pub fn new(r: f64, theta: f64, phi: f64) -> Result<Self> {
        if r < 0.0 {
            return Err(TelescopeDBError::InvalidCoordinates { r, theta, phi });
        }
        if theta < 0.0 || theta >= 2.0 * std::f64::consts::PI {
            return Err(TelescopeDBError::InvalidCoordinates { r, theta, phi });
        }
        if phi < 0.0 || phi > std::f64::consts::PI {
            return Err(TelescopeDBError::InvalidCoordinates { r, theta, phi });
        }

        Ok(Self { r, theta, phi })
    }

    /// Calcula la distancia euclidiana en espacio esférico
    ///
    /// Fórmula: d = sqrt(r1² + r2² - 2*r1*r2*cos(angle))
    /// donde angle = arccos(sin(φ1)*sin(φ2)*cos(θ1-θ2) + cos(φ1)*cos(φ2))
    pub fn distance(&self, other: &Self) -> f64 {
        let angle = (self.phi.sin() * other.phi.sin() * (self.theta - other.theta).cos()
            + self.phi.cos() * other.phi.cos())
        .acos();

        (self.r.powi(2) + other.r.powi(2) - 2.0 * self.r * other.r * angle.cos()).sqrt()
    }

    /// Convierte a coordenadas cartesianas (x, y, z)
    pub fn to_cartesian(&self) -> (f64, f64, f64) {
        let x = self.r * self.phi.sin() * self.theta.cos();
        let y = self.r * self.phi.sin() * self.theta.sin();
        let z = self.r * self.phi.cos();
        (x, y, z)
    }

    /// Convierte desde coordenadas cartesianas
    pub fn from_cartesian(x: f64, y: f64, z: f64) -> Self {
        let r = (x * x + y * y + z * z).sqrt();
        let theta = y.atan2(x);
        let phi = (z / r).acos();

        Self { r, theta, phi }
    }
}

/// Tensor de contexto 7D (del Context Token 7D)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextTensor7D {
    pub semantic: f64,      // Dimensión 1
    pub syntactic: f64,     // Dimensión 2
    pub emotional: f64,     // Dimensión 3
    pub intentional: f64,   // Dimensión 4
    pub contextual: f64,    // Dimensión 5
    pub biographical: f64,  // Dimensión 6 ← TelescopeDB alimenta esta
    pub relational: f64,    // Dimensión 7
}

impl ContextTensor7D {
    /// Convierte tensor 7D a coordenadas esféricas
    pub fn to_spherical_coords(&self) -> SphericalCoords {
        // Intensidad = función de emocional + intencional
        let r = ((self.emotional.powi(2) + self.intentional.powi(2)) / 2.0).sqrt();

        // Categoría = función de semántico + contextual
        let theta = (self.semantic.atan2(self.contextual) + std::f64::consts::PI)
            % (2.0 * std::f64::consts::PI);

        // Valencia = función de emocional normalizada
        let phi = (1.0 - self.emotional).clamp(0.0, 1.0) * std::f64::consts::PI;

        SphericalCoords { r, theta, phi }
    }

    /// Convierte a vector de f64 (para pixel encoding)
    pub fn to_vec(&self) -> Vec<f64> {
        vec![
            self.semantic,
            self.syntactic,
            self.emotional,
            self.intentional,
            self.contextual,
            self.biographical,
            self.relational,
        ]
    }

    /// Crea desde vector de f64
    pub fn from_vec(v: &[f64]) -> Option<Self> {
        if v.len() != 7 {
            return None;
        }

        Some(Self {
            semantic: v[0],
            syntactic: v[1],
            emotional: v[2],
            intentional: v[3],
            contextual: v[4],
            biographical: v[5],
            relational: v[6],
        })
    }
}

/// Embedding vectorial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub vec: Vec<f32>,
    pub model: String, // "text-embedding-ada-002", "local-bge", etc.
}

impl Embedding {
    /// Crea un embedding vacío
    pub fn empty(model: String) -> Self {
        Self {
            vec: Vec::new(),
            model,
        }
    }

    /// Calcula similitud coseno con otro embedding
    pub fn cosine_similarity(&self, other: &Embedding) -> f64 {
        if self.vec.len() != other.vec.len() {
            return 0.0;
        }

        let dot_product: f32 = self
            .vec
            .iter()
            .zip(other.vec.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = self.vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.vec.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        (dot_product / (norm_a * norm_b)) as f64
    }
}

/// Núcleo atómico del FBCU Core
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicCore {
    /// Embedding principal (1536 dims para OpenAI, 768 para local)
    pub embedding: Embedding,

    /// Anchors semánticos (palabras clave)
    pub anchors: Vec<String>,

    /// Timestamp de creación
    pub timestamp: DateTime<Utc>,

    /// Contenido original (comprimido)
    pub content: Vec<u8>,
}

/// FBCU Core - Unidad comprimida de memoria biográfica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FBCUCore {
    /// ID único (SHA-256 del contenido)
    pub id: String,

    /// Núcleo atómico (embeddings + anchors)
    pub atomic_core: AtomicCore,

    /// Tensor de contexto 7D completo
    pub context_tensor: ContextTensor7D,

    /// Coordenadas esféricas
    pub coords: SphericalCoords,

    /// Metadata adicional
    pub metadata: HashMap<String, String>,
}

/// Estructura principal de TelescopeDB
pub struct TelescopeDB {
    /// Directorio raíz de almacenamiento
    storage_path: PathBuf,

    /// Mapa de cores (content-addressable por SHA-256)
    cores: HashMap<String, FBCUCore>,

    /// Métricas de uso
    metrics: TelescopeMetrics,
}

/// Métricas de TelescopeDB
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TelescopeMetrics {
    pub total_cores: usize,
    pub total_queries: usize,
    pub avg_query_time_ms: f64,
    pub compression_ratio: f64,
}

impl TelescopeDB {
    /// Crea nueva instancia de TelescopeDB
    pub fn new(storage_path: PathBuf) -> Result<Self> {
        // Crear directorio si no existe
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            storage_path,
            cores: HashMap::new(),
            metrics: TelescopeMetrics::default(),
        })
    }

    /// Inserta FBCU Core desde Context Token 7D
    pub async fn insert_from_ctx7d(&mut self, token: &ContextTensor7D) -> Result<String> {
        // Calcular coordenadas esféricas
        let coords = token.to_spherical_coords();

        // Generar ID temporal (en implementación real: SHA-256 del contenido)
        let id = uuid::Uuid::new_v4().to_string();

        // Crear FBCU Core
        let fbcu_core = FBCUCore {
            id: id.clone(),
            atomic_core: AtomicCore {
                embedding: Embedding::empty("mock".to_string()),
                anchors: Vec::new(),
                timestamp: Utc::now(),
                content: Vec::new(),
            },
            context_tensor: token.clone(),
            coords,
            metadata: HashMap::new(),
        };

        // Guardar en HashMap
        self.cores.insert(id.clone(), fbcu_core);
        self.metrics.total_cores += 1;

        Ok(id)
    }

    /// Query contextual: buscar experiencias similares por coordenadas
    pub async fn query_contextual(
        &self,
        coords: SphericalCoords,
        radius: f64,
    ) -> Result<Vec<FBCUCore>> {
        let start = std::time::Instant::now();

        let mut results: Vec<(FBCUCore, f64)> = self
            .cores
            .values()
            .filter_map(|core| {
                let distance = coords.distance(&core.coords);
                if distance <= radius {
                    Some((core.clone(), distance))
                } else {
                    None
                }
            })
            .collect();

        // Ordenar por distancia (más cercano primero)
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let elapsed = start.elapsed().as_millis() as f64;
        tracing::debug!("Query contextual completed in {}ms", elapsed);

        // Retornar top-10
        Ok(results.into_iter().take(10).map(|(c, _)| c).collect())
    }

    /// Query semántico: buscar por similaridad de embeddings
    pub async fn query_semantic(&self, query_embedding: &Embedding) -> Result<Vec<(FBCUCore, f64)>> {
        let start = std::time::Instant::now();

        let mut results: Vec<(FBCUCore, f64)> = self
            .cores
            .values()
            .map(|core| {
                let similarity = query_embedding.cosine_similarity(&core.atomic_core.embedding);
                (core.clone(), similarity)
            })
            .collect();

        // Ordenar por similaridad (mayor primero)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let elapsed = start.elapsed().as_millis() as f64;
        tracing::debug!("Query semantic completed in {}ms", elapsed);

        // Retornar top-10 con similarity > 0.7
        Ok(results
            .into_iter()
            .take(10)
            .filter(|(_, sim)| *sim > 0.7)
            .collect())
    }

    /// Obtiene métricas actuales
    pub fn metrics(&self) -> &TelescopeMetrics {
        &self.metrics
    }

    /// Obtiene número total de cores almacenados
    pub fn len(&self) -> usize {
        self.cores.len()
    }

    /// Verifica si está vacío
    pub fn is_empty(&self) -> bool {
        self.cores.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spherical_coords_distance() {
        let c1 = SphericalCoords::new(1.0, 0.0, 0.0).unwrap();
        let c2 = SphericalCoords::new(1.0, std::f64::consts::PI, std::f64::consts::PI).unwrap();

        let distance = c1.distance(&c2);
        assert!(distance > 0.0);
        assert!(distance <= 2.0); // Max distance in unit sphere
    }

    #[test]
    fn test_ctx7d_to_spherical_conversion() {
        let tensor = ContextTensor7D {
            semantic: 0.8,
            syntactic: 0.6,
            emotional: 0.9, // High emotional intensity
            intentional: 0.7,
            contextual: 0.5,
            biographical: 0.4,
            relational: 0.3,
        };

        let coords = tensor.to_spherical_coords();

        // High emotional → high r
        assert!(coords.r > 0.5);

        // Bounds check
        assert!(coords.theta >= 0.0 && coords.theta < 2.0 * std::f64::consts::PI);
        assert!(coords.phi >= 0.0 && coords.phi <= std::f64::consts::PI);
    }

    #[test]
    fn test_cartesian_conversion() {
        let coords = SphericalCoords::new(1.0, std::f64::consts::PI / 4.0, std::f64::consts::PI / 3.0).unwrap();
        let (x, y, z) = coords.to_cartesian();
        let back = SphericalCoords::from_cartesian(x, y, z);

        // Should be approximately equal
        assert!((coords.r - back.r).abs() < 1e-10);
        assert!((coords.theta - back.theta).abs() < 1e-10);
        assert!((coords.phi - back.phi).abs() < 1e-10);
    }

    #[tokio::test]
    async fn test_telescopedb_insert_and_query() {
        let temp_dir = std::env::temp_dir().join("test_telescope");
        let mut db = TelescopeDB::new(temp_dir).unwrap();

        let ctx7d = ContextTensor7D {
            semantic: 0.8,
            syntactic: 0.6,
            emotional: 0.9,
            intentional: 0.7,
            contextual: 0.5,
            biographical: 0.4,
            relational: 0.3,
        };

        let id = db.insert_from_ctx7d(&ctx7d).await.unwrap();
        assert!(!id.is_empty());
        assert_eq!(db.len(), 1);

        let coords = ctx7d.to_spherical_coords();
        let results = db.query_contextual(coords, 0.5).await.unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].id, id);
    }
}
