//! User domain model

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Representa un usuario del sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    /// Identificador único del usuario
    pub user_id: String,
    /// Nombre completo
    pub name: String,
    /// Email del usuario
    pub email: String,
    /// Nombre de usuario en GitHub (opcional)
    pub github_username: Option<String>,
    /// Configuración del usuario
    pub settings: UserSettings,
    /// Estadísticas de uso
    pub stats: UserStats,
    /// Estado de la cuenta
    pub status: UserStatus,
    /// Rol del usuario
    pub role: UserRole,
    /// Fecha de creación de la cuenta
    pub created_at: DateTime<Utc>,
    /// Fecha de última actividad
    pub last_active_at: DateTime<Utc>,
    /// Fecha de último inicio de sesión
    pub last_login_at: Option<DateTime<Utc>>,
    /// Versión de la configuración (para migraciones)
    pub config_version: u32,
}

/// Configuración personalizable del usuario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSettings {
    /// Zona horaria preferida
    pub timezone: String,
    /// Formato de fecha preferido
    pub date_format: DateFormat,
    /// Configuración de notificaciones
    pub notifications: NotificationSettings,
    /// Configuración de backup automático
    pub auto_backup: BackupSettings,
    /// Herramientas preferidas
    pub preferred_tools: PreferredTools,
    /// Configuración de sesiones
    pub session_config: SessionConfig,
}

/// Configuración de notificaciones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotificationSettings {
    /// Notificar cuando termine una sesión
    pub session_end: bool,
    /// Notificar objetivos completados
    pub goal_completion: bool,
    /// Recordatorios de backup
    pub backup_reminders: bool,
    /// Resúmenes diarios
    pub daily_summary: bool,
}

/// Configuración de backup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BackupSettings {
    /// Backup automático habilitado
    pub enabled: bool,
    /// Frecuencia de backup
    pub frequency: BackupFrequency,
    /// Retención de backups en días
    pub retention_days: u32,
    /// Incluir archivos binarios
    pub include_binaries: bool,
}

/// Herramientas preferidas del usuario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreferredTools {
    /// Editor de código preferido
    pub code_editor: String,
    /// Terminal preferido
    pub terminal: String,
    /// Navegador preferido
    pub browser: String,
    /// Herramientas adicionales
    pub additional_tools: Vec<String>,
}

/// Configuración de sesiones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionConfig {
    /// Duración máxima de sesión en horas
    pub max_session_hours: f32,
    /// Auto-guardar cada X minutos
    pub auto_save_minutes: u32,
    /// Crear backup al finalizar sesión
    pub backup_on_end: bool,
    /// Pausar automáticamente sesiones largas
    pub auto_pause_long_sessions: bool,
}

/// Estadísticas del usuario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStats {
    /// Número total de sesiones
    pub total_sessions: u32,
    /// Total de horas trabajadas
    pub total_work_hours: f32,
    /// Total de proyectos creados
    pub total_projects: u32,
    /// Total de topics completados
    pub completed_topics: u32,
    /// Total de acciones registradas
    pub total_actions: u32,
    /// Racha actual de días trabajados
    pub current_streak: u32,
    /// Mejor racha de días
    pub best_streak: u32,
    /// Fecha de la última sesión
    pub last_session_date: Option<DateTime<Utc>>,
    /// Estadísticas por tipo de acción
    pub action_stats: HashMap<String, u32>,
}

/// Estado de la cuenta de usuario
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserStatus {
    /// Usuario activo
    Active,
    /// Usuario inactivo temporalmente
    Inactive,
    /// Usuario suspendido
    Suspended,
    /// Cuenta eliminada (soft delete)
    Deleted,
}

/// Rol del usuario en el sistema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    /// Usuario básico
    User,
    /// Usuario premium
    Premium,
    /// Administrador
    Admin,
    /// Usuario de prueba
    Trial,
}

/// Formato de fecha preferido
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DateFormat {
    /// ISO 8601: 2024-01-15
    ISO,
    /// US: 01/15/2024
    US,
    /// EU: 15/01/2024
    EU,
    /// Relativo: hace 2 días
    Relative,
}

/// Frecuencia de backup
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BackupFrequency {
    /// Después de cada sesión
    AfterSession,
    /// Diario
    Daily,
    /// Semanal
    Weekly,
    /// Manual solamente
    Manual,
}

impl User {
    /// Crear nuevo usuario
    pub fn new(user_id: String, name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            user_id: user_id.clone(),
            name,
            email,
            github_username: None,
            settings: UserSettings::default(),
            stats: UserStats::new(),
            status: UserStatus::Active,
            role: UserRole::User,
            created_at: now,
            last_active_at: now,
            last_login_at: Some(now),
            config_version: 1,
        }
    }

    /// Crear usuario con GitHub
    pub fn with_github(user_id: String, name: String, email: String, github_username: String) -> Self {
        let mut user = Self::new(user_id, name, email);
        user.github_username = Some(github_username);
        user
    }

    /// Actualizar actividad del usuario
    pub fn update_activity(&mut self) {
        self.last_active_at = Utc::now();
    }

    /// Registrar inicio de sesión
    pub fn record_login(&mut self) {
        self.last_login_at = Some(Utc::now());
        self.update_activity();
    }

    /// Incrementar estadística de sesión
    pub fn increment_session_stats(&mut self, work_hours: f32) {
        self.stats.total_sessions += 1;
        self.stats.total_work_hours += work_hours;
        self.stats.last_session_date = Some(Utc::now());
        self.update_streak();
        self.update_activity();
    }

    /// Incrementar proyectos
    pub fn increment_projects(&mut self) {
        self.stats.total_projects += 1;
        self.update_activity();
    }

    /// Incrementar topics completados
    pub fn increment_completed_topics(&mut self) {
        self.stats.completed_topics += 1;
        self.update_activity();
    }

    /// Registrar acción
    pub fn record_action(&mut self, action_type: &str) {
        self.stats.total_actions += 1;
        *self.stats.action_stats.entry(action_type.to_string()).or_insert(0) += 1;
        self.update_activity();
    }

    /// Verificar si el usuario está activo
    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

    /// Verificar si es usuario premium
    pub fn is_premium(&self) -> bool {
        matches!(self.role, UserRole::Premium | UserRole::Admin)
    }

    /// Verificar si es administrador
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }

    /// Obtener días desde la última actividad
    pub fn days_since_last_activity(&self) -> i64 {
        let now = Utc::now();
        (now - self.last_active_at).num_days()
    }

    /// Verificar si el usuario está inactivo
    pub fn is_inactive(&self, days_threshold: i64) -> bool {
        self.days_since_last_activity() > days_threshold
    }

    /// Actualizar configuración
    pub fn update_settings(&mut self, settings: UserSettings) {
        self.settings = settings;
        self.config_version += 1;
        self.update_activity();
    }

    /// Suspender usuario
    pub fn suspend(&mut self) {
        self.status = UserStatus::Suspended;
        self.update_activity();
    }

    /// Reactivar usuario
    pub fn reactivate(&mut self) {
        if matches!(self.status, UserStatus::Inactive | UserStatus::Suspended) {
            self.status = UserStatus::Active;
            self.update_activity();
        }
    }

    /// Marcar como eliminado (soft delete)
    pub fn soft_delete(&mut self) {
        self.status = UserStatus::Deleted;
        self.update_activity();
    }

    /// Obtener configuración de backup
    pub fn backup_settings(&self) -> &BackupSettings {
        &self.settings.auto_backup
    }

    /// Verificar si debe hacer backup automático
    pub fn should_auto_backup(&self) -> bool {
        self.settings.auto_backup.enabled && self.is_active()
    }

    /// Obtener promedio de horas por sesión
    pub fn average_hours_per_session(&self) -> f32 {
        if self.stats.total_sessions > 0 {
            self.stats.total_work_hours / self.stats.total_sessions as f32
        } else {
            0.0
        }
    }

    /// Obtener acción más frecuente
    pub fn most_frequent_action(&self) -> Option<(String, u32)> {
        self.stats.action_stats
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(action, &count)| (action.clone(), count))
    }

    /// Actualizar racha de días trabajados
    fn update_streak(&mut self) {
        if let Some(last_session) = self.stats.last_session_date {
            let days_diff = (Utc::now() - last_session).num_days();
            
            if days_diff <= 1 {
                // Mismo día o día consecutivo
                self.stats.current_streak += 1;
                if self.stats.current_streak > self.stats.best_streak {
                    self.stats.best_streak = self.stats.current_streak;
                }
            } else {
                // Racha rota
                self.stats.current_streak = 1;
            }
        } else {
            self.stats.current_streak = 1;
        }
    }
}

impl UserSettings {
    /// Configuración por defecto
    pub fn default() -> Self {
        Self {
            timezone: "UTC".to_string(),
            date_format: DateFormat::ISO,
            notifications: NotificationSettings::default(),
            auto_backup: BackupSettings::default(),
            preferred_tools: PreferredTools::default(),
            session_config: SessionConfig::default(),
        }
    }
}

impl NotificationSettings {
    pub fn default() -> Self {
        Self {
            session_end: true,
            goal_completion: true,
            backup_reminders: true,
            daily_summary: false,
        }
    }
}

impl BackupSettings {
    pub fn default() -> Self {
        Self {
            enabled: true,
            frequency: BackupFrequency::AfterSession,
            retention_days: 30,
            include_binaries: false,
        }
    }
}

impl PreferredTools {
    pub fn default() -> Self {
        Self {
            code_editor: "VS Code".to_string(),
            terminal: "bash".to_string(),
            browser: "default".to_string(),
            additional_tools: Vec::new(),
        }
    }
}

impl SessionConfig {
    pub fn default() -> Self {
        Self {
            max_session_hours: 8.0,
            auto_save_minutes: 10,
            backup_on_end: true,
            auto_pause_long_sessions: true,
        }
    }
}

impl UserStats {
    /// Estadísticas iniciales
    pub fn new() -> Self {
        Self {
            total_sessions: 0,
            total_work_hours: 0.0,
            total_projects: 0,
            completed_topics: 0,
            total_actions: 0,
            current_streak: 0,
            best_streak: 0,
            last_session_date: None,
            action_stats: HashMap::new(),
        }
    }
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::Suspended => "suspended",
            UserStatus::Deleted => "deleted",
        }
    }
}

impl UserRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRole::User => "user",
            UserRole::Premium => "premium",
            UserRole::Admin => "admin",
            UserRole::Trial => "trial",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "user123".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        
        assert_eq!(user.user_id, "user123");
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert!(user.is_active());
        assert!(!user.is_premium());
        assert_eq!(user.stats.total_sessions, 0);
    }

    #[test]
    fn test_user_activity_tracking() {
        let mut user = User::new(
            "user".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        
        user.increment_session_stats(2.5);
        assert_eq!(user.stats.total_sessions, 1);
        assert_eq!(user.stats.total_work_hours, 2.5);
        assert_eq!(user.stats.current_streak, 1);
        
        user.record_action("git_commit");
        user.record_action("git_commit");
        user.record_action("file_edit");
        
        assert_eq!(user.stats.total_actions, 3);
        assert_eq!(user.stats.action_stats.get("git_commit"), Some(&2));
        assert_eq!(user.most_frequent_action(), Some(("git_commit".to_string(), 2)));
    }

    #[test]
    fn test_user_settings_update() {
        let mut user = User::new(
            "user".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        
        let initial_version = user.config_version;
        
        let mut new_settings = UserSettings::default();
        new_settings.timezone = "Europe/Madrid".to_string();
        new_settings.date_format = DateFormat::EU;
        
        user.update_settings(new_settings);
        
        assert_eq!(user.settings.timezone, "Europe/Madrid");
        assert_eq!(user.settings.date_format, DateFormat::EU);
        assert_eq!(user.config_version, initial_version + 1);
    }

    #[test]
    fn test_user_status_changes() {
        let mut user = User::new(
            "user".to_string(),
            "Test User".to_string(),
            "test@example.com".to_string(),
        );
        
        assert!(user.is_active());
        
        user.suspend();
        assert!(!user.is_active());
        assert_eq!(user.status, UserStatus::Suspended);
        
        user.reactivate();
        assert!(user.is_active());
        assert_eq!(user.status, UserStatus::Active);
    }
}
