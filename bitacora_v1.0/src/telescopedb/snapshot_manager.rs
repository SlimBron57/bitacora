//! # Snapshot Manager - Gestión de Snapshots de Memoria
//!
//! Sistema de creación, recuperación, comparación y compresión de snapshots.
//! Integración con pixel_storage para almacenamiento visual.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::telescopedb::{Result, TelescopeDBError, FBCUCore};

/// Snapshot de TelescopeDB en un momento específico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// ID único del snapshot
    pub id: String,

    /// Timestamp de creación
    pub created_at: DateTime<Utc>,

    /// Nombre descriptivo
    pub name: String,

    /// Descripción del snapshot
    pub description: String,

    /// IDs de los cores incluidos
    pub core_ids: Vec<String>,

    /// Metadata del snapshot
    pub metadata: SnapshotMetadata,

    /// Compresión aplicada
    pub compression: CompressionInfo,
}

/// Metadata del snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// Número total de cores
    pub total_cores: usize,

    /// Tamaño total sin comprimir (bytes)
    pub uncompressed_size: u64,

    /// Tamaño comprimido (bytes)
    pub compressed_size: u64,

    /// Ratio de compresión
    pub compression_ratio: f64,

    /// Tags del snapshot
    pub tags: Vec<String>,

    /// Usuario que creó el snapshot (opcional)
    pub created_by: Option<String>,
}

/// Información de compresión
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionInfo {
    /// Algoritmo usado
    pub algorithm: CompressionAlgorithm,

    /// Nivel de compresión (0-9)
    pub level: u8,

    /// Tiempo de compresión (ms)
    pub compression_time_ms: u64,
}

/// Algoritmos de compresión soportados
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// Sin compresión
    None,

    /// GZIP (deflate)
    Gzip,

    /// FBCU (Fractal-Based Compression)
    FBCU,

    /// Combinación FBCU + GZIP
    FBCUGzip,
}

/// Comparación entre dos snapshots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotComparison {
    /// ID del snapshot antiguo
    pub old_snapshot_id: String,

    /// ID del snapshot nuevo
    pub new_snapshot_id: String,

    /// Timestamp de la comparación
    pub compared_at: DateTime<Utc>,

    /// Cores añadidos (presentes en new, ausentes en old)
    pub added_cores: Vec<String>,

    /// Cores eliminados (presentes en old, ausentes en new)
    pub deleted_cores: Vec<String>,

    /// Cores modificados
    pub modified_cores: Vec<String>,

    /// Cores sin cambios
    pub unchanged_cores: Vec<String>,

    /// Resumen estadístico
    pub summary: ComparisonSummary,
}

/// Resumen de comparación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonSummary {
    pub total_added: usize,
    pub total_deleted: usize,
    pub total_modified: usize,
    pub total_unchanged: usize,
    pub similarity_score: f64, // 0.0 = totalmente diferente, 1.0 = idéntico
}

/// Gestor de snapshots
pub struct SnapshotManager {
    /// Directorio de almacenamiento
    storage_path: PathBuf,

    /// Snapshots en memoria
    snapshots: HashMap<String, Snapshot>,

    /// Configuración
    config: SnapshotConfig,
}

/// Configuración del snapshot manager
#[derive(Debug, Clone)]
pub struct SnapshotConfig {
    /// Algoritmo de compresión por defecto
    pub default_compression: CompressionAlgorithm,

    /// Nivel de compresión (0-9)
    pub compression_level: u8,

    /// Auto-snapshot habilitado
    pub auto_snapshot_enabled: bool,

    /// Intervalo de auto-snapshot (en segundos)
    pub auto_snapshot_interval_secs: i64,

    /// Máximo número de snapshots a retener
    pub max_snapshots: usize,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        Self {
            default_compression: CompressionAlgorithm::FBCU,
            compression_level: 6,
            auto_snapshot_enabled: false,
            auto_snapshot_interval_secs: 3600, // 1 hora
            max_snapshots: 100,
        }
    }
}

impl SnapshotManager {
    /// Crea un nuevo gestor de snapshots
    pub fn new(storage_path: PathBuf, config: SnapshotConfig) -> Result<Self> {
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            storage_path,
            snapshots: HashMap::new(),
            config,
        })
    }

    /// Crea un nuevo snapshot
    pub fn create_snapshot(
        &mut self,
        name: String,
        description: String,
        cores: &[FBCUCore],
    ) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();

        // Calcular tamaños
        let uncompressed_size = self.calculate_uncompressed_size(cores);
        let (compressed_data, compressed_size, compression_time_ms) =
            self.compress_cores(cores)?;

        let compression_ratio = if compressed_size > 0 {
            uncompressed_size as f64 / compressed_size as f64
        } else {
            1.0
        };

        let snapshot = Snapshot {
            id: id.clone(),
            created_at: Utc::now(),
            name,
            description,
            core_ids: cores.iter().map(|c| c.id.clone()).collect(),
            metadata: SnapshotMetadata {
                total_cores: cores.len(),
                uncompressed_size,
                compressed_size,
                compression_ratio,
                tags: Vec::new(),
                created_by: None,
            },
            compression: CompressionInfo {
                algorithm: self.config.default_compression,
                level: self.config.compression_level,
                compression_time_ms,
            },
        };

        // Guardar snapshot a disco
        self.save_snapshot(&snapshot, &compressed_data)?;

        // Guardar en memoria
        self.snapshots.insert(id.clone(), snapshot);

        // Limitar número de snapshots
        self.prune_old_snapshots()?;

        Ok(id)
    }

    /// Recupera un snapshot por ID
    pub fn get_snapshot(&self, id: &str) -> Result<Option<Snapshot>> {
        Ok(self.snapshots.get(id).cloned())
    }

    /// Elimina un snapshot
    pub fn delete_snapshot(&mut self, id: &str) -> Result<()> {
        self.snapshots.remove(id);

        // Eliminar de disco
        let snapshot_path = self.storage_path.join(format!("{}.snapshot", id));
        if snapshot_path.exists() {
            std::fs::remove_file(snapshot_path)?;
        }

        Ok(())
    }

    /// Compara dos snapshots
    pub fn compare_snapshots(&self, old_id: &str, new_id: &str) -> Result<SnapshotComparison> {
        let old_snapshot = self
            .snapshots
            .get(old_id)
            .ok_or_else(|| TelescopeDBError::CoreNotFound(old_id.to_string()))?;

        let new_snapshot = self
            .snapshots
            .get(new_id)
            .ok_or_else(|| TelescopeDBError::CoreNotFound(new_id.to_string()))?;

        let old_cores: std::collections::HashSet<_> =
            old_snapshot.core_ids.iter().collect();
        let new_cores: std::collections::HashSet<_> =
            new_snapshot.core_ids.iter().collect();

        let added_cores: Vec<String> = new_cores
            .difference(&old_cores)
            .map(|s| s.to_string())
            .collect();

        let deleted_cores: Vec<String> = old_cores
            .difference(&new_cores)
            .map(|s| s.to_string())
            .collect();

        let common_cores: Vec<String> = old_cores
            .intersection(&new_cores)
            .map(|s| s.to_string())
            .collect();

        // TODO: Detectar modificaciones reales comparando contenido
        let modified_cores = Vec::new();
        let unchanged_cores = common_cores;

        let total_added = added_cores.len();
        let total_deleted = deleted_cores.len();
        let total_modified = modified_cores.len();
        let total_unchanged = unchanged_cores.len();

        // Calcular similitud
        let total_cores = (old_snapshot.core_ids.len() + new_snapshot.core_ids.len()) as f64 / 2.0;
        let similarity_score = if total_cores > 0.0 {
            total_unchanged as f64 / total_cores
        } else {
            1.0
        };

        Ok(SnapshotComparison {
            old_snapshot_id: old_id.to_string(),
            new_snapshot_id: new_id.to_string(),
            compared_at: Utc::now(),
            added_cores,
            deleted_cores,
            modified_cores,
            unchanged_cores,
            summary: ComparisonSummary {
                total_added,
                total_deleted,
                total_modified,
                total_unchanged,
                similarity_score,
            },
        })
    }

    /// Lista todos los snapshots
    pub fn list_snapshots(&self) -> Vec<Snapshot> {
        let mut snapshots: Vec<_> = self.snapshots.values().cloned().collect();
        snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        snapshots
    }

    /// Obtiene el snapshot más reciente
    pub fn get_latest_snapshot(&self) -> Option<Snapshot> {
        self.list_snapshots().into_iter().next()
    }

    // === Métodos privados ===

    fn calculate_uncompressed_size(&self, cores: &[FBCUCore]) -> u64 {
        // Estimación simple: serializar a JSON y medir tamaño
        cores
            .iter()
            .map(|core| {
                serde_json::to_string(core)
                    .map(|s| s.len() as u64)
                    .unwrap_or(0)
            })
            .sum()
    }

    fn compress_cores(&self, cores: &[FBCUCore]) -> Result<(Vec<u8>, u64, u64)> {
        let start = std::time::Instant::now();

        // Serializar cores
        let serialized = serde_json::to_vec(cores)?;

        // Aplicar compresión según algoritmo
        let compressed = match self.config.default_compression {
            CompressionAlgorithm::None => serialized.clone(),
            CompressionAlgorithm::Gzip => {
                // TODO: Implementar GZIP
                serialized.clone()
            }
            CompressionAlgorithm::FBCU => {
                // TODO: Implementar FBCU compression
                serialized.clone()
            }
            CompressionAlgorithm::FBCUGzip => {
                // TODO: Implementar FBCU + GZIP
                serialized.clone()
            }
        };

        let elapsed = start.elapsed().as_millis() as u64;
        let size = compressed.len() as u64;

        Ok((compressed, size, elapsed))
    }

    fn save_snapshot(&self, snapshot: &Snapshot, data: &[u8]) -> Result<()> {
        let snapshot_path = self.storage_path.join(format!("{}.snapshot", snapshot.id));

        // Guardar metadata
        let metadata_path = self
            .storage_path
            .join(format!("{}.metadata.json", snapshot.id));
        let metadata_json = serde_json::to_string_pretty(snapshot)?;
        std::fs::write(metadata_path, metadata_json)?;

        // Guardar datos comprimidos
        std::fs::write(snapshot_path, data)?;

        Ok(())
    }

    fn prune_old_snapshots(&mut self) -> Result<()> {
        if self.snapshots.len() <= self.config.max_snapshots {
            return Ok(());
        }

        // Ordenar por fecha (más antiguo primero)
        let mut sorted: Vec<_> = self.snapshots.values().cloned().collect();
        sorted.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        // Eliminar los más antiguos
        let to_delete = self.snapshots.len() - self.config.max_snapshots;
        for snapshot in sorted.iter().take(to_delete) {
            self.delete_snapshot(&snapshot.id)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescopedb::{ContextTensor7D, SphericalCoords, AtomicCore, Embedding};

    fn create_test_core(id: String) -> FBCUCore {
        FBCUCore {
            id,
            atomic_core: AtomicCore {
                embedding: Embedding::empty("test".to_string()),
                anchors: Vec::new(),
                timestamp: Utc::now(),
                content: Vec::new(),
            },
            context_tensor: ContextTensor7D {
                semantic: 0.8,
                syntactic: 0.6,
                emotional: 0.9,
                intentional: 0.7,
                contextual: 0.5,
                biographical: 0.4,
                relational: 0.3,
            },
            coords: SphericalCoords::new(1.0, 0.5, 1.0).unwrap(),
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_snapshot_manager_creation() {
        let temp_dir = std::env::temp_dir().join("test_snapshots");
        let manager = SnapshotManager::new(temp_dir, SnapshotConfig::default());
        assert!(manager.is_ok());
    }

    #[test]
    fn test_create_snapshot() {
        let temp_dir = std::env::temp_dir().join("test_snapshots_create");
        let mut manager = SnapshotManager::new(temp_dir, SnapshotConfig::default()).unwrap();

        let cores = vec![
            create_test_core("core-1".to_string()),
            create_test_core("core-2".to_string()),
        ];

        let snapshot_id = manager
            .create_snapshot(
                "Test Snapshot".to_string(),
                "A test snapshot".to_string(),
                &cores,
            )
            .unwrap();

        assert!(!snapshot_id.is_empty());

        let snapshot = manager.get_snapshot(&snapshot_id).unwrap();
        assert!(snapshot.is_some());

        let snapshot = snapshot.unwrap();
        assert_eq!(snapshot.core_ids.len(), 2);
        assert_eq!(snapshot.metadata.total_cores, 2);
    }

    #[test]
    fn test_compare_snapshots() {
        let temp_dir = std::env::temp_dir().join("test_snapshots_compare");
        let mut manager = SnapshotManager::new(temp_dir, SnapshotConfig::default()).unwrap();

        let cores1 = vec![
            create_test_core("core-1".to_string()),
            create_test_core("core-2".to_string()),
        ];

        let cores2 = vec![
            create_test_core("core-2".to_string()),
            create_test_core("core-3".to_string()),
        ];

        let snapshot1_id = manager
            .create_snapshot("Snapshot 1".to_string(), "First".to_string(), &cores1)
            .unwrap();

        let snapshot2_id = manager
            .create_snapshot("Snapshot 2".to_string(), "Second".to_string(), &cores2)
            .unwrap();

        let comparison = manager
            .compare_snapshots(&snapshot1_id, &snapshot2_id)
            .unwrap();

        assert_eq!(comparison.summary.total_added, 1); // core-3
        assert_eq!(comparison.summary.total_deleted, 1); // core-1
        assert_eq!(comparison.summary.total_unchanged, 1); // core-2
    }

    #[test]
    fn test_list_snapshots() {
        let temp_dir = std::env::temp_dir().join("test_snapshots_list");
        let mut manager = SnapshotManager::new(temp_dir, SnapshotConfig::default()).unwrap();

        let cores = vec![create_test_core("core-1".to_string())];

        manager
            .create_snapshot("Snapshot 1".to_string(), "First".to_string(), &cores)
            .unwrap();

        manager
            .create_snapshot("Snapshot 2".to_string(), "Second".to_string(), &cores)
            .unwrap();

        let snapshots = manager.list_snapshots();
        assert_eq!(snapshots.len(), 2);
    }

    #[test]
    fn test_delete_snapshot() {
        let temp_dir = std::env::temp_dir().join("test_snapshots_delete");
        let mut manager = SnapshotManager::new(temp_dir, SnapshotConfig::default()).unwrap();

        let cores = vec![create_test_core("core-1".to_string())];

        let snapshot_id = manager
            .create_snapshot("Test".to_string(), "Test".to_string(), &cores)
            .unwrap();

        assert!(manager.get_snapshot(&snapshot_id).unwrap().is_some());

        manager.delete_snapshot(&snapshot_id).unwrap();

        assert!(manager.get_snapshot(&snapshot_id).unwrap().is_none());
    }

    #[test]
    fn test_prune_old_snapshots() {
        let temp_dir = std::env::temp_dir().join("test_snapshots_prune");
        let config = SnapshotConfig {
            max_snapshots: 2,
            ..Default::default()
        };
        let mut manager = SnapshotManager::new(temp_dir, config).unwrap();

        let cores = vec![create_test_core("core-1".to_string())];

        // Crear 3 snapshots (max es 2)
        manager
            .create_snapshot("Snapshot 1".to_string(), "First".to_string(), &cores)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10));

        manager
            .create_snapshot("Snapshot 2".to_string(), "Second".to_string(), &cores)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10));

        manager
            .create_snapshot("Snapshot 3".to_string(), "Third".to_string(), &cores)
            .unwrap();

        // Debería haber solo 2 snapshots (el más antiguo fue eliminado)
        assert_eq!(manager.list_snapshots().len(), 2);
    }
}
