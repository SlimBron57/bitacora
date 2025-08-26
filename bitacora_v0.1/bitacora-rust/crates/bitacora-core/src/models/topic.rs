//! Topic domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa un tema u objetivo de trabajo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Topic {
    /// Identificador único del topic
    pub topic_id: Uuid,
    /// ID del usuario propietario
    pub user_id: String,
    /// ID del proyecto asociado (opcional)
    pub project_id: Option<Uuid>,
    /// Título del topic
    pub title: String,
    /// Descripción detallada
    pub description: String,
    /// Estado actual
    pub status: TopicStatus,
    /// Prioridad
    pub priority: Priority,
    /// Tiempo estimado en horas
    pub estimated_hours: Option<f32>,
    /// Tiempo real invertido en horas
    pub actual_hours: Option<f32>,
    /// Tags para categorización
    pub tags: Vec<String>,
    /// Fecha de creación
    pub created_at: DateTime<Utc>,
    /// Fecha de última actualización
    pub updated_at: DateTime<Utc>,
    /// Fecha de inicio del trabajo
    pub started_at: Option<DateTime<Utc>>,
    /// Fecha de finalización
    pub completed_at: Option<DateTime<Utc>>,
    /// Progreso en porcentaje (0-100)
    pub progress_percentage: Option<u8>,
}

/// Estado del topic
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TopicStatus {
    /// En planificación inicial
    Planning,
    /// Trabajo en progreso
    InProgress,
    /// Completado exitosamente
    Completed,
    /// En pausa temporal
    OnHold,
    /// Cancelado
    Cancelled,
    /// Bloqueado por dependencias
    Blocked,
}

/// Nivel de prioridad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum Priority {
    /// Baja prioridad
    Low,
    /// Prioridad media
    Medium,
    /// Alta prioridad
    High,
    /// Prioridad crítica
    Critical,
}

impl Topic {
    /// Crear nuevo topic
    pub fn new(user_id: String, title: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            topic_id: Uuid::new_v4(),
            user_id,
            project_id: None,
            title,
            description,
            status: TopicStatus::Planning,
            priority: Priority::Medium,
            estimated_hours: None,
            actual_hours: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
            progress_percentage: Some(0),
        }
    }

    /// Asociar con un proyecto
    pub fn with_project(mut self, project_id: Uuid) -> Self {
        self.project_id = Some(project_id);
        self.touch();
        self
    }

    /// Establecer prioridad
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self.touch();
        self
    }

    /// Establecer estimación de horas
    pub fn with_estimation(mut self, hours: f32) -> Self {
        self.estimated_hours = Some(hours);
        self.touch();
        self
    }

    /// Agregar tags
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self.touch();
        self
    }

    /// Iniciar trabajo en el topic
    pub fn start(&mut self) -> Result<(), String> {
        if !self.can_start() {
            return Err("Topic cannot be started from current status".to_string());
        }
        
        self.status = TopicStatus::InProgress;
        self.started_at = Some(Utc::now());
        self.touch();
        Ok(())
    }

    /// Completar el topic
    pub fn complete(&mut self) -> Result<(), String> {
        if !self.can_complete() {
            return Err("Topic cannot be completed from current status".to_string());
        }
        
        self.status = TopicStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress_percentage = Some(100);
        self.touch();
        Ok(())
    }

    /// Poner en pausa
    pub fn pause(&mut self) -> Result<(), String> {
        if !matches!(self.status, TopicStatus::InProgress) {
            return Err("Only in-progress topics can be paused".to_string());
        }
        
        self.status = TopicStatus::OnHold;
        self.touch();
        Ok(())
    }

    /// Marcar como bloqueado
    pub fn block(&mut self, _reason: Option<String>) -> Result<(), String> {
        if matches!(self.status, TopicStatus::Completed | TopicStatus::Cancelled) {
            return Err("Cannot block completed or cancelled topics".to_string());
        }
        
        self.status = TopicStatus::Blocked;
        // TODO: Store block reason when we add that field
        self.touch();
        Ok(())
    }

    /// Actualizar progreso
    pub fn update_progress(&mut self, percentage: u8) -> Result<(), String> {
        if percentage > 100 {
            return Err("Progress cannot exceed 100%".to_string());
        }
        
        self.progress_percentage = Some(percentage);
        
        // Auto-complete if progress reaches 100%
        if percentage == 100 && self.status == TopicStatus::InProgress {
            let _ = self.complete(); // Ignore error for now
        }
        
        self.touch();
        Ok(())
    }

    /// Registrar tiempo trabajado
    pub fn add_work_time(&mut self, hours: f32) {
        self.actual_hours = Some(self.actual_hours.unwrap_or(0.0) + hours);
        self.touch();
    }

    /// Verificar si puede iniciarse
    pub fn can_start(&self) -> bool {
        matches!(self.status, TopicStatus::Planning | TopicStatus::OnHold)
    }

    /// Verificar si puede completarse
    pub fn can_complete(&self) -> bool {
        matches!(self.status, TopicStatus::InProgress | TopicStatus::OnHold)
    }

    /// Verificar si está activo (en progreso)
    pub fn is_active(&self) -> bool {
        matches!(self.status, TopicStatus::InProgress)
    }

    /// Verificar si está completado
    pub fn is_completed(&self) -> bool {
        matches!(self.status, TopicStatus::Completed)
    }

    /// Verificar si está bloqueado
    pub fn is_blocked(&self) -> bool {
        matches!(self.status, TopicStatus::Blocked)
    }

    /// Obtener duración total del trabajo
    pub fn total_duration(&self) -> Option<chrono::Duration> {
        if let (Some(started), Some(completed)) = (self.started_at, self.completed_at) {
            Some(completed - started)
        } else if let Some(started) = self.started_at {
            Some(Utc::now() - started)
        } else {
            None
        }
    }

    /// Obtener eficiencia (horas reales vs estimadas)
    pub fn efficiency_ratio(&self) -> Option<f32> {
        if let (Some(estimated), Some(actual)) = (self.estimated_hours, self.actual_hours) {
            if estimated > 0.0 {
                Some(estimated / actual)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Verificar si tiene un tag específico
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t.eq_ignore_ascii_case(tag))
    }

    /// Obtener progreso actual
    pub fn progress(&self) -> u8 {
        self.progress_percentage.unwrap_or(0)
    }

    /// Verificar si está atrasado (si hay estimación)
    pub fn is_overdue(&self) -> bool {
        if let (Some(estimated), Some(actual)) = (self.estimated_hours, self.actual_hours) {
            actual > estimated * 1.2 // 20% tolerance
        } else {
            false
        }
    }

    /// Actualizar timestamp de modificación
    fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl TopicStatus {
    /// Obtener todas las variantes
    pub fn all() -> Vec<TopicStatus> {
        vec![
            TopicStatus::Planning,
            TopicStatus::InProgress,
            TopicStatus::Completed,
            TopicStatus::OnHold,
            TopicStatus::Cancelled,
            TopicStatus::Blocked,
        ]
    }

    /// Convertir a string
    pub fn as_str(&self) -> &'static str {
        match self {
            TopicStatus::Planning => "planning",
            TopicStatus::InProgress => "in_progress",
            TopicStatus::Completed => "completed",
            TopicStatus::OnHold => "on_hold",
            TopicStatus::Cancelled => "cancelled",
            TopicStatus::Blocked => "blocked",
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Option<TopicStatus> {
        match s.to_lowercase().as_str() {
            "planning" | "plan" => Some(TopicStatus::Planning),
            "in_progress" | "progress" | "active" => Some(TopicStatus::InProgress),
            "completed" | "done" | "finished" => Some(TopicStatus::Completed),
            "on_hold" | "hold" | "paused" => Some(TopicStatus::OnHold),
            "cancelled" | "canceled" => Some(TopicStatus::Cancelled),
            "blocked" | "block" => Some(TopicStatus::Blocked),
            _ => None,
        }
    }

    /// Verificar si es estado final
    pub fn is_final(&self) -> bool {
        matches!(self, TopicStatus::Completed | TopicStatus::Cancelled)
    }
}

impl Priority {
    /// Convertir a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Critical => "critical",
        }
    }

    /// Crear desde string
    pub fn from_str(s: &str) -> Option<Priority> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Some(Priority::Low),
            "medium" | "med" | "m" => Some(Priority::Medium),
            "high" | "h" => Some(Priority::High),
            "critical" | "crit" | "c" => Some(Priority::Critical),
            _ => None,
        }
    }

    /// Obtener valor numérico para ordenamiento
    pub fn value(&self) -> u8 {
        match self {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Critical => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topic_creation() {
        let topic = Topic::new(
            "user123".to_string(),
            "Implement new feature".to_string(),
            "Add awesome functionality".to_string(),
        );
        
        assert_eq!(topic.user_id, "user123");
        assert_eq!(topic.title, "Implement new feature");
        assert_eq!(topic.status, TopicStatus::Planning);
        assert_eq!(topic.priority, Priority::Medium);
        assert_eq!(topic.progress(), 0);
    }

    #[test]
    fn test_topic_lifecycle() {
        let mut topic = Topic::new(
            "user".to_string(),
            "Test Topic".to_string(),
            "Description".to_string(),
        );
        
        // Start topic
        assert!(topic.start().is_ok());
        assert!(topic.is_active());
        assert!(topic.started_at.is_some());
        
        // Update progress
        assert!(topic.update_progress(50).is_ok());
        assert_eq!(topic.progress(), 50);
        
        // Complete topic
        assert!(topic.complete().is_ok());
        assert!(topic.is_completed());
        assert!(topic.completed_at.is_some());
        assert_eq!(topic.progress(), 100);
    }

    #[test]
    fn test_topic_work_time_tracking() {
        let mut topic = Topic::new(
            "user".to_string(),
            "Time Tracking Test".to_string(),
            "Test time tracking".to_string(),
        ).with_estimation(5.0);
        
        topic.add_work_time(2.5);
        topic.add_work_time(4.0); // Total: 6.5 hours
        
        assert_eq!(topic.actual_hours, Some(6.5));
        // 6.5 > 5.0 * 1.2 = 6.0, so should be overdue
        assert!(topic.is_overdue());
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        
        assert_eq!(Priority::Critical.value(), 4);
        assert_eq!(Priority::Low.value(), 1);
    }
}
