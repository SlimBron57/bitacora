//! Topic Management Service

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use bitacora_core::models::{Topic, TopicStatus, Priority};
use bitacora_core::errors::BitacoraError;
use bitacora_storage::repository::{Repository, TopicRepository};
use bitacora_storage::{StorageResult, StorageError};

/// Service para gestión de Topics
pub struct TopicService<R>
where
    R: Repository<Topic> + TopicRepository,
{
    topic_repo: Arc<R>,
}

impl<R> TopicService<R>
where
    R: Repository<Topic> + TopicRepository,
{
    pub fn new(topic_repo: Arc<R>) -> Self {
        Self { topic_repo }
    }

    /// Crear nuevo topic
    pub async fn create_topic(
        &self,
        title: String,
        description: String,
        user_id: String,
        project_id: Option<Uuid>,
    ) -> Result<Topic, BitacoraError> {
        let topic = Topic::new(title, description, user_id, project_id);
        
        self.topic_repo.create(&topic).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
        
        Ok(topic)
    }

    /// Obtener topic por ID
    pub async fn get_topic(&self, topic_id: &str) -> Result<Option<Topic>, BitacoraError> {
        self.topic_repo.get_by_id(topic_id).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Actualizar topic
    pub async fn update_topic(&self, topic: Topic) -> Result<bool, BitacoraError> {
        self.topic_repo.update(&topic).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Eliminar topic
    pub async fn delete_topic(&self, topic_id: &str) -> Result<bool, BitacoraError> {
        self.topic_repo.delete(topic_id).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Listar todos los topics con paginación
    pub async fn list_topics(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<Topic>, BitacoraError> {
        self.topic_repo.list(limit, offset).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Obtener topics por usuario
    pub async fn get_user_topics(&self, user_id: &str) -> Result<Vec<Topic>, BitacoraError> {
        self.topic_repo.get_by_user_id(user_id).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Obtener topics activos
    pub async fn get_active_topics(&self) -> Result<Vec<Topic>, BitacoraError> {
        self.topic_repo.get_active_topics().await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Obtener topics por estado
    pub async fn get_topics_by_status(&self, status: &str) -> Result<Vec<Topic>, BitacoraError> {
        self.topic_repo.get_by_status(status).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))
    }

    /// Crear topic derivado de otro topic activo
    pub async fn create_derived_topic(
        &self,
        source_topic_id: &str,
        title: String,
        description: String,
        user_id: String,
    ) -> Result<Topic, BitacoraError> {
        // Obtener el topic fuente
        let source_topic = self.get_topic(source_topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Source topic not found".to_string()))?;

        // Crear nuevo topic con mismo project_id que el fuente
        let new_topic = Topic::new(title, description, user_id, source_topic.project_id);
        
        self.topic_repo.create(&new_topic).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
        
        Ok(new_topic)
    }

    /// Obtener topics por proyecto
    pub async fn get_topics_by_project(&self, project_id: &str) -> Result<Vec<Topic>, BitacoraError> {
        // TopicRepository doesn't have find_by_project method, use list and filter
        let all_topics = self.topic_repo.list(None, None).await
            .map_err(|e| BitacoraError::Infrastructure(e.to_string()))?;
        
        let filtered_topics = all_topics.into_iter()
            .filter(|topic| topic.project_id.map(|id| id.to_string()).as_deref() == Some(project_id))
            .collect();
        
        Ok(filtered_topics)
    }

    /// Archivar topic (cambiar estado a Archived)
    pub async fn archive_topic(&self, topic_id: &str) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.archive();
        self.update_topic(topic).await
    }

    /// Activar topic (cambiar estado a Active)
    pub async fn activate_topic(&self, topic_id: &str) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.activate();
        self.update_topic(topic).await
    }

    /// Completar topic (cambiar estado a Completed)
    pub async fn complete_topic(&self, topic_id: &str) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.complete();
        self.update_topic(topic).await
    }

    /// Agregar tag a topic
    pub async fn add_tag_to_topic(&self, topic_id: &str, tag: String) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.add_tag(tag);
        self.update_topic(topic).await
    }

    /// Remover tag de topic
    pub async fn remove_tag_from_topic(&self, topic_id: &str, tag: &str) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.remove_tag(tag);
        self.update_topic(topic).await
    }

    /// Cambiar prioridad del topic
    pub async fn set_topic_priority(&self, topic_id: &str, priority: Priority) -> Result<bool, BitacoraError> {
        let mut topic = self.get_topic(topic_id).await?
            .ok_or_else(|| BitacoraError::NotFound("Topic not found".to_string()))?;
        
        topic.set_priority(priority);
        self.update_topic(topic).await
    }
}
