//! Módulo de programación de backups

/// Programador de backups automáticos
pub struct BackupScheduler {
    // TODO: Implementar en Fase 4
}

/// Tipos de triggers para backup
pub enum BackupTrigger {
    /// Backup automático al finalizar sesión
    SessionEnd(String),
    /// Backup periódico programado
    Periodic(std::time::Duration),
    /// Backup manual iniciado por usuario
    Manual(String),
    /// Backup crítico antes de operaciones peligrosas
    Critical(String),
}
