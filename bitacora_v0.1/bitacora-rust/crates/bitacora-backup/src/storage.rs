//! Backends de almacenamiento para backups

use async_trait::async_trait;

/// Trait para diferentes backends de almacenamiento
#[async_trait]
#[async_trait]
pub trait BackupStorage: Send + Sync {
    /// Almacenar backup encriptado
    async fn store_backup(&self, backup_id: &str, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;

    /// Recuperar backup
    async fn retrieve_backup(&self, backup_id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>;

    /// Listar backups de usuario
    async fn list_user_backups(&self, user_id: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    /// Eliminar backup
    async fn delete_backup(&self, backup_id: &str) -> Result<(), Box<dyn std::error::Error>>;
}/// Storage local
pub struct LocalBackupStorage {
    base_path: std::path::PathBuf,
}

/// Storage en S3/MinIO
#[cfg(feature = "s3-storage")]
pub struct S3BackupStorage {
    // TODO: Implementar cliente S3
}
