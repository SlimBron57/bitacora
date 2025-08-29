//! Threading Especializado - Nivel 0 (Sparks)
//!
//! Implementa el sistema de threading de 4 niveles para el navegador híbrido.
//! Por ahora solo implementamos el Nivel 0 básico.

use std::sync::Arc;
use tokio::sync::{Semaphore, RwLock};
use dashmap::DashMap;
use uuid::Uuid;
use crate::errors::{NavigatorError, Result};

/// Gestor de Threading para el sistema híbrido
#[derive(Debug)]
pub struct ThreadManager {
    /// Semáforo para controlar concurrencia de Sparks (Nivel 0)
    spark_semaphore: Arc<Semaphore>,
    /// Mapa de threads activos por tipo
    active_threads: Arc<DashMap<ThreadType, u32>>,
    /// Configuración de límites
    limits: ThreadLimits,
}

/// Tipos de threads en el sistema
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ThreadType {
    /// Thread procesando Sparks
    Spark,
    /// Thread procesando Projects
    Project,
    /// Thread procesando Topics  
    Topic,
    /// Thread procesando Actions
    Action,
}

/// Límites de concurrencia por tipo de thread
#[derive(Debug, Clone)]
pub struct ThreadLimits {
    /// Máximo sparks simultáneos (Nivel 0)
    pub max_sparks: usize,
    /// Máximo projects simultáneos (Nivel 1) 
    pub max_projects: usize,
    /// Topics son seriales por project (Nivel 2)
    pub topics_serial: bool,
    /// Actions son completamente seriales (Nivel 3)
    pub actions_serial: bool,
}

impl Default for ThreadLimits {
    fn default() -> Self {
        Self {
            max_sparks: 10,    // Nivel 0: múltiples sparks
            max_projects: 5,   // Nivel 1: múltiples projects
            topics_serial: true,   // Nivel 2: serial por project
            actions_serial: true,  // Nivel 3: completamente serial
        }
    }
}

impl ThreadManager {
    /// Crear nuevo gestor de threading
    pub fn new(limits: ThreadLimits) -> Self {
        let spark_semaphore = Arc::new(Semaphore::new(limits.max_sparks));
        let active_threads = Arc::new(DashMap::new());

        // Inicializar contadores
        active_threads.insert(ThreadType::Spark, 0);
        active_threads.insert(ThreadType::Project, 0);
        active_threads.insert(ThreadType::Topic, 0);
        active_threads.insert(ThreadType::Action, 0);

        Self {
            spark_semaphore,
            active_threads,
            limits,
        }
    }

    /// Crear con límites por defecto
    pub fn default() -> Self {
        Self::new(ThreadLimits::default())
    }

    /// Adquirir permiso para ejecutar un Spark (Nivel 0)
    pub async fn acquire_spark_permit(&self) -> Result<SparkPermit> {
        tracing::debug!("Solicitando permiso para Spark...");
        
        let permit = self.spark_semaphore
            .acquire()
            .await
            .map_err(|_| NavigatorError::threading("No se pudo adquirir permiso para Spark"))?;
        
        // Incrementar contador
        self.active_threads.entry(ThreadType::Spark).and_modify(|e| *e += 1);
        
        tracing::debug!("Permiso para Spark adquirido");
        
        Ok(SparkPermit {
            _permit: permit,
            thread_manager: self,
        })
    }

    /// Obtener estadísticas actuales de threads
    pub fn get_stats(&self) -> ThreadStats {
        ThreadStats {
            active_sparks: self.active_threads.get(&ThreadType::Spark)
                .map(|v| *v)
                .unwrap_or(0),
            active_projects: self.active_threads.get(&ThreadType::Project)
                .map(|v| *v)
                .unwrap_or(0),
            active_topics: self.active_threads.get(&ThreadType::Topic)
                .map(|v| *v)
                .unwrap_or(0),
            active_actions: self.active_threads.get(&ThreadType::Action)
                .map(|v| *v)
                .unwrap_or(0),
            max_sparks: self.limits.max_sparks,
            max_projects: self.limits.max_projects,
        }
    }

    /// Verificar si se puede ejecutar un tipo de thread
    pub fn can_execute(&self, thread_type: &ThreadType) -> bool {
        match thread_type {
            ThreadType::Spark => {
                self.spark_semaphore.available_permits() > 0
            }
            ThreadType::Project => {
                let active = self.active_threads.get(thread_type)
                    .map(|v| *v)
                    .unwrap_or(0);
                active < self.limits.max_projects as u32
            }
            ThreadType::Topic => {
                // En Nivel 2, topics son seriales por project
                !self.limits.topics_serial || 
                self.active_threads.get(thread_type)
                    .map(|v| *v)
                    .unwrap_or(0) == 0
            }
            ThreadType::Action => {
                // En Nivel 3, actions son completamente seriales
                !self.limits.actions_serial || 
                self.active_threads.get(thread_type)
                    .map(|v| *v)
                    .unwrap_or(0) == 0
            }
        }
    }

    /// Obtener límites actuales
    pub fn limits(&self) -> &ThreadLimits {
        &self.limits
    }

    /// Actualizar límites (requiere reinicio para aplicar)
    pub fn update_limits(&mut self, limits: ThreadLimits) {
        self.limits = limits;
        // TODO: Aplicar nuevos límites a semáforos activos
    }
}

/// Permiso para ejecutar un Spark
pub struct SparkPermit<'a> {
    _permit: tokio::sync::SemaphorePermit<'a>,
    thread_manager: &'a ThreadManager,
}

impl<'a> Drop for SparkPermit<'a> {
    fn drop(&mut self) {
        // Decrementar contador al liberar
        self.thread_manager.active_threads.entry(ThreadType::Spark).and_modify(|e| *e -= 1);
        tracing::debug!("Permiso para Spark liberado");
    }
}

/// Estadísticas de threads activos
#[derive(Debug, Clone)]
pub struct ThreadStats {
    pub active_sparks: u32,
    pub active_projects: u32,
    pub active_topics: u32,
    pub active_actions: u32,
    pub max_sparks: usize,
    pub max_projects: usize,
}

impl ThreadStats {
    /// Verificar si el sistema está saturado
    pub fn is_saturated(&self) -> bool {
        self.active_sparks >= self.max_sparks as u32 || 
        self.active_projects >= self.max_projects as u32
    }

    /// Obtener porcentaje de utilización
    pub fn utilization_percent(&self) -> f32 {
        let spark_util = self.active_sparks as f32 / self.max_sparks as f32;
        let project_util = self.active_projects as f32 / self.max_projects as f32;
        spark_util.max(project_util) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_manager_creation() {
        let limits = ThreadLimits::default();
        let manager = ThreadManager::new(limits.clone());
        
        assert_eq!(manager.limits().max_sparks, limits.max_sparks);
        assert_eq!(manager.limits().max_projects, limits.max_projects);
    }

    #[tokio::test]
    async fn test_spark_permit_acquisition() {
        let manager = ThreadManager::default();
        
        // Verificar que podemos adquirir un permiso
        assert!(manager.can_execute(&ThreadType::Spark));
        
        let permit = manager.acquire_spark_permit().await;
        assert!(permit.is_ok());
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_sparks, 1);
    }

    #[tokio::test]
    async fn test_spark_permit_release() {
        let manager = ThreadManager::default();
        
        {
            let _permit = manager.acquire_spark_permit().await.unwrap();
            let stats = manager.get_stats();
            assert_eq!(stats.active_sparks, 1);
        } // permit se libera aquí
        
        // Pequeña espera para que se procese el drop
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        let stats = manager.get_stats();
        assert_eq!(stats.active_sparks, 0);
    }

    #[test]
    fn test_thread_stats() {
        let limits = ThreadLimits {
            max_sparks: 5,
            max_projects: 3,
            topics_serial: true,
            actions_serial: true,
        };
        let manager = ThreadManager::new(limits);
        
        let stats = manager.get_stats();
        assert!(!stats.is_saturated());
        assert_eq!(stats.utilization_percent(), 0.0);
    }
}
