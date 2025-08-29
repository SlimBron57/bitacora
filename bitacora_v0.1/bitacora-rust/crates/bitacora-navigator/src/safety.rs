//! Safety Controller - Control de Locks y Detección de Conflictos
//!
//! Implementa el sistema de seguridad para prevenir conflictos de recursos
//! y deadlocks en el navegador híbrido.

use std::sync::Arc;
use std::collections::HashSet;
use dashmap::DashMap;
use parking_lot::{RwLock, Mutex};
use uuid::Uuid;
use crate::errors::{NavigatorError, Result};

/// Controlador de seguridad principal
#[derive(Debug)]
pub struct SafetyController {
    /// Recursos actualmente bloqueados
    locked_resources: Arc<DashMap<ResourceId, LockInfo>>,
    /// Detector de deadlocks
    deadlock_detector: Arc<Mutex<DeadlockDetector>>,
    /// Configuración de seguridad
    config: SafetyConfig,
}

/// ID único para recursos del sistema
pub type ResourceId = String;

/// Información sobre un lock activo
#[derive(Debug, Clone)]
pub struct LockInfo {
    /// ID del thread que posee el lock
    pub owner_thread: String,
    /// Tipo de lock (Read/Write)
    pub lock_type: LockType,
    /// Timestamp cuando se adquirió
    pub acquired_at: std::time::Instant,
    /// Recursos adicionales que requiere este thread
    pub additional_resources: HashSet<ResourceId>,
}

/// Tipos de lock disponibles
#[derive(Debug, Clone, PartialEq)]
pub enum LockType {
    /// Lock de lectura (múltiples permitidos)
    Read,
    /// Lock de escritura (exclusivo)
    Write,
}

/// Detector de deadlocks básico
#[derive(Debug)]
pub struct DeadlockDetector {
    /// Grafo de dependencias entre threads
    dependency_graph: DashMap<String, HashSet<String>>,
    /// Threads actualmente monitoreados
    active_threads: HashSet<String>,
}

/// Configuración del safety controller
#[derive(Debug, Clone)]
pub struct SafetyConfig {
    /// Timeout máximo para adquirir locks (ms)
    pub lock_timeout_ms: u64,
    /// Habilitar detección de deadlocks
    pub enable_deadlock_detection: bool,
    /// Intervalo de verificación de deadlocks (ms)
    pub deadlock_check_interval_ms: u64,
    /// Máximo tiempo que un lock puede mantenerse (ms)
    pub max_lock_hold_time_ms: u64,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            lock_timeout_ms: 5000,
            enable_deadlock_detection: true,
            deadlock_check_interval_ms: 1000,
            max_lock_hold_time_ms: 30000,
        }
    }
}

impl SafetyController {
    /// Crear nuevo controlador de seguridad
    pub fn new(config: SafetyConfig) -> Self {
        Self {
            locked_resources: Arc::new(DashMap::new()),
            deadlock_detector: Arc::new(Mutex::new(DeadlockDetector::new())),
            config,
        }
    }

    /// Crear con configuración por defecto
    pub fn default() -> Self {
        Self::new(SafetyConfig::default())
    }

    /// Adquirir lock sobre un recurso
    pub async fn acquire_lock(
        &self, 
        resource_id: ResourceId, 
        lock_type: LockType,
        thread_id: String
    ) -> Result<ResourceLock> {
        tracing::debug!("Intentando adquirir {:?} lock en recurso: {}", lock_type, resource_id);

        // Verificar conflictos existentes
        if self.has_conflict(&resource_id, &lock_type)? {
            return Err(NavigatorError::resource_conflict(
                format!("Recurso {} ya está bloqueado de forma conflictiva", resource_id)
            ));
        }

        // Verificar deadlock potencial
        if self.config.enable_deadlock_detection {
            self.check_deadlock_potential(&thread_id, &resource_id)?;
        }

        // Crear información del lock
        let lock_info = LockInfo {
            owner_thread: thread_id.clone(),
            lock_type: lock_type.clone(),
            acquired_at: std::time::Instant::now(),
            additional_resources: HashSet::new(),
        };

        // Registrar el lock
        self.locked_resources.insert(resource_id.clone(), lock_info);

        tracing::debug!("Lock {:?} adquirido en recurso: {}", lock_type, resource_id);

        Ok(ResourceLock {
            resource_id,
            thread_id,
            safety_controller: self,
        })
    }

    /// Verificar si hay conflicto para un tipo de lock
    fn has_conflict(&self, resource_id: &ResourceId, requested_lock: &LockType) -> Result<bool> {
        if let Some(existing_lock) = self.locked_resources.get(resource_id) {
            match (&existing_lock.lock_type, requested_lock) {
                // Read-Read no conflictúa
                (LockType::Read, LockType::Read) => Ok(false),
                // Cualquier Write conflictúa con todo
                (LockType::Write, _) | (_, LockType::Write) => Ok(true),
            }
        } else {
            Ok(false)
        }
    }

    /// Verificar potencial deadlock
    fn check_deadlock_potential(&self, thread_id: &str, resource_id: &ResourceId) -> Result<()> {
        let mut detector = self.deadlock_detector.lock();
        
        // Por ahora, implementación básica
        // TODO: Implementar detección real de ciclos en el grafo
        if detector.active_threads.contains(thread_id) {
            tracing::debug!("Thread {} ya está activo, verificando dependencias...", thread_id);
        }

        detector.active_threads.insert(thread_id.to_string());
        Ok(())
    }

    /// Liberar lock de un recurso
    pub fn release_lock(&self, resource_id: &ResourceId, thread_id: &str) -> Result<()> {
        if let Some((_, lock_info)) = self.locked_resources.remove(resource_id) {
            if lock_info.owner_thread == thread_id {
                tracing::debug!("Lock liberado en recurso: {}", resource_id);
                
                // Limpiar thread del detector de deadlocks
                let mut detector = self.deadlock_detector.lock();
                detector.active_threads.remove(thread_id);
                
                Ok(())
            } else {
                Err(NavigatorError::safety(
                    format!("Thread {} no posee el lock del recurso {}", thread_id, resource_id)
                ))
            }
        } else {
            Err(NavigatorError::safety(
                format!("No existe lock para el recurso {}", resource_id)
            ))
        }
    }

    /// Obtener estadísticas de locks
    pub fn get_lock_stats(&self) -> LockStats {
        let total_locks = self.locked_resources.len();
        let mut read_locks = 0;
        let mut write_locks = 0;
        let mut expired_locks = 0;

        let now = std::time::Instant::now();

        for entry in self.locked_resources.iter() {
            let lock_info = entry.value();
            
            match lock_info.lock_type {
                LockType::Read => read_locks += 1,
                LockType::Write => write_locks += 1,
            }

            if now.duration_since(lock_info.acquired_at).as_millis() 
                > self.config.max_lock_hold_time_ms as u128 {
                expired_locks += 1;
            }
        }

        LockStats {
            total_locks,
            read_locks,
            write_locks,
            expired_locks,
        }
    }

    /// Limpiar locks expirados
    pub fn cleanup_expired_locks(&self) -> Result<u32> {
        let now = std::time::Instant::now();
        let mut cleaned = 0;

        self.locked_resources.retain(|_, lock_info| {
            let expired = now.duration_since(lock_info.acquired_at).as_millis() 
                > self.config.max_lock_hold_time_ms as u128;
            
            if expired {
                tracing::warn!("Limpiando lock expirado del thread: {}", lock_info.owner_thread);
                cleaned += 1;
            }

            !expired
        });

        if cleaned > 0 {
            tracing::info!("Limpiados {} locks expirados", cleaned);
        }

        Ok(cleaned)
    }
}

impl DeadlockDetector {
    fn new() -> Self {
        Self {
            dependency_graph: DashMap::new(),
            active_threads: HashSet::new(),
        }
    }
}

/// Lock sobre un recurso específico
pub struct ResourceLock<'a> {
    resource_id: ResourceId,
    thread_id: String,
    safety_controller: &'a SafetyController,
}

impl<'a> Drop for ResourceLock<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.safety_controller.release_lock(&self.resource_id, &self.thread_id) {
            tracing::error!("Error liberando lock: {}", e);
        }
    }
}

/// Estadísticas de locks del sistema
#[derive(Debug, Clone)]
pub struct LockStats {
    pub total_locks: usize,
    pub read_locks: usize,
    pub write_locks: usize,
    pub expired_locks: usize,
}

impl LockStats {
    /// Verificar si el sistema está saturado de locks
    pub fn is_saturated(&self, threshold: usize) -> bool {
        self.total_locks > threshold
    }

    /// Obtener ratio de locks de escritura
    pub fn write_ratio(&self) -> f32 {
        if self.total_locks == 0 {
            0.0
        } else {
            self.write_locks as f32 / self.total_locks as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_acquire_read_lock() {
        let controller = SafetyController::default();
        let resource_id = "test_resource".to_string();
        let thread_id = "thread_1".to_string();

        let lock = controller.acquire_lock(
            resource_id.clone(), 
            LockType::Read, 
            thread_id
        ).await;
        
        assert!(lock.is_ok());

        let stats = controller.get_lock_stats();
        assert_eq!(stats.total_locks, 1);
        assert_eq!(stats.read_locks, 1);
    }

    #[tokio::test]
    async fn test_conflict_detection() {
        let controller = SafetyController::default();
        let resource_id = "test_resource".to_string();

        // Adquirir write lock
        let _lock1 = controller.acquire_lock(
            resource_id.clone(), 
            LockType::Write, 
            "thread_1".to_string()
        ).await.unwrap();

        // Intentar adquirir otro lock debería fallar
        let result = controller.acquire_lock(
            resource_id, 
            LockType::Read, 
            "thread_2".to_string()
        ).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_lock_stats() {
        let controller = SafetyController::default();
        let stats = controller.get_lock_stats();
        
        assert_eq!(stats.total_locks, 0);
        assert_eq!(stats.write_ratio(), 0.0);
        assert!(!stats.is_saturated(100));
    }
}
