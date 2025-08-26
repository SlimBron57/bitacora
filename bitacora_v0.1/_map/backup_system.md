# Sistema de Respaldos Automáticos - Bitacora V1.0

## 🎯 Objetivo
Implementar un sistema de respaldos automáticos por usuario que se ejecute al finalizar cada sesión de trabajo, garantizando la preservación de datos y permitiendo recuperación punto en el tiempo.

## 🏗️ Arquitectura del Sistema

### **Crate: bitacora-backup**
```
bitacora-backup/
├── src/
│   ├── scheduler/          # Programación de backups
│   ├── storage/           # Backends de almacenamiento  
│   ├── compression/       # Compresión de datos
│   ├── encryption/        # Encriptación por usuario
│   └── restore/           # Sistema de restauración
```

## 🔄 Flujo de Backup Automático

### **Trigger: Comando END**
```rust
// En bitacora-commands/src/handler/end_handler.rs
pub async fn handle_end_session(
    session_id: &str,
    user_id: &str,
    backup_service: &BackupService,
) -> Result<()> {
    // 1. Finalizar sesión
    session_service.end_session(session_id).await?;
    
    // 2. Trigger backup automático
    backup_service.backup_user_session(user_id, session_id).await?;
    
    // 3. Limpieza de backups antiguos
    backup_service.cleanup_old_backups(user_id).await?;
    
    Ok(())
}
```

### **Backup Scheduler**
```rust
// Tipos de backup disponibles
pub enum BackupTrigger {
    SessionEnd(SessionId),      // Al terminar sesión
    Periodic(Duration),         // Backup periódico
    Manual(UserId),            // Backup manual
    Critical(String),          // Backup crítico (antes de operaciones peligrosas)
}

pub struct BackupScheduler {
    triggers: Vec<BackupTrigger>,
    storage_backends: Vec<Box<dyn BackupStorage>>,
    encryption_manager: EncryptionManager,
}
```

## 🗂️ Estructura de Datos del Backup

### **Formato del Backup**
```json
{
  "backup_id": "uuid",
  "user_id": "string", 
  "timestamp": "ISO8601",
  "session_id": "string",
  "backup_type": "session_end|periodic|manual|critical",
  "compression": "gzip|none",
  "encryption": {
    "algorithm": "AES-256-GCM",
    "key_id": "user_specific_key_id"
  },
  "data": {
    "sessions": [...],
    "actions": [...],
    "topics": [...],
    "sparks": [...],
    "user_config": {...}
  },
  "integrity_hash": "sha256_hash",
  "size_compressed": 12345,
  "size_uncompressed": 45678
}
```

## 🔐 Encriptación por Usuario

### **Key Management**
```rust
pub struct UserKeyManager {
    master_key: SecretKey,
    user_keys: HashMap<UserId, UserKey>,
}

pub struct UserKey {
    user_id: UserId,
    key_id: String,
    encryption_key: SecretKey,
    created_at: DateTime<Utc>,
    last_rotated: DateTime<Utc>,
}

impl UserKeyManager {
    // Generar clave única por usuario
    pub fn generate_user_key(&mut self, user_id: &UserId) -> Result<UserKey>;
    
    // Rotar clave (para seguridad)
    pub fn rotate_user_key(&mut self, user_id: &UserId) -> Result<UserKey>;
    
    // Encriptar datos específicos del usuario
    pub fn encrypt_user_data(&self, user_id: &UserId, data: &[u8]) -> Result<Vec<u8>>;
}
```

## 📦 Storage Backends

### **Local Storage**
```rust
pub struct LocalBackupStorage {
    base_path: PathBuf,
    retention_policy: RetentionPolicy,
}

// Estructura de directorios:
// backups/
// ├── users/
// │   └── {user_id}/
// │       ├── daily/
// │       ├── weekly/ 
// │       └── sessions/
// │           └── {session_id}_{timestamp}.backup.gz.enc
```

### **Cloud Storage (S3/MinIO)**
```rust
pub struct S3BackupStorage {
    client: S3Client,
    bucket: String,
    prefix: String,  // users/{user_id}/backups/
}
```

## ⏰ Políticas de Retención

### **Configuración**
```toml
[backup]
enabled = true
auto_backup_on_session_end = true
compression = "gzip"
encryption = true

[backup.retention]
# Retener backups por sesión: 30 días
session_backups_days = 30

# Backups diarios: 90 días  
daily_backups_days = 90

# Backups semanales: 1 año
weekly_backups_days = 365

# Backup mensual: 5 años
monthly_backups_days = 1825

[backup.storage]
primary = "local"
secondary = "s3"  # Backup de backup

[backup.storage.local]
path = "./data/backups"
max_size_gb = 10

[backup.storage.s3]
bucket = "bitacora-backups"
region = "us-east-1"
```

## 🔄 Sistema de Restauración

### **Point-in-Time Recovery**
```rust
pub struct RestoreService {
    backup_storage: Box<dyn BackupStorage>,
    encryption_manager: EncryptionManager,
}

impl RestoreService {
    // Restaurar sesión específica
    pub async fn restore_session(
        &self, 
        user_id: &UserId, 
        session_id: &SessionId
    ) -> Result<SessionData>;
    
    // Restaurar datos de usuario a fecha específica
    pub async fn restore_user_data_at_time(
        &self,
        user_id: &UserId,
        target_time: DateTime<Utc>
    ) -> Result<UserData>;
    
    // Restaurar elementos específicos
    pub async fn restore_selective(
        &self,
        user_id: &UserId,
        items: Vec<RestoreItem>
    ) -> Result<()>;
}
```

## 🚀 Integración con Comandos

### **Comando END (Automático)**
```bash
# Usuario ejecuta:
curl -X POST http://localhost:8080/api/commands/end \
  -H "Content-Type: application/json" \
  -d '{"session_id": "session_123", "user_id": "user_456"}'

# Sistema automáticamente:
# 1. Finaliza sesión
# 2. Crea backup encriptado
# 3. Almacena en storage backends
# 4. Limpia backups antiguos
# 5. Retorna confirmación
```

### **Comando BACKUP (Manual)**
```bash
# Backup manual inmediato
curl -X POST http://localhost:8080/api/commands/backup \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user_456", "type": "manual"}'
```

### **Comando RESTORE**
```bash
# Listar backups disponibles
curl -X GET http://localhost:8080/api/backups/user_456

# Restaurar sesión específica
curl -X POST http://localhost:8080/api/restore/session \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user_456", "session_id": "session_123"}'
```

## 📊 Métricas y Monitoreo

### **Health Checks**
- ✅ Espacio disponible en storage
- ✅ Tiempo promedio de backup
- ✅ Éxito/fallo de backups
- ✅ Integridad de backups existentes

### **Alertas**
- 🚨 Fallo en backup automático
- 🚨 Espacio insuficiente
- 🚨 Corrupción detectada en backup
- 🚨 Clave de encriptación comprometida

## 🧪 Testing del Sistema

### **Test Cases**
- [ ] Backup automático al finalizar sesión
- [ ] Encriptación/desencriptación correcta
- [ ] Compresión efectiva de datos
- [ ] Limpieza de backups antiguos
- [ ] Restore point-in-time funcional
- [ ] Failover a storage secundario
- [ ] Verificación de integridad

### **Performance Tests**
- [ ] Backup de sesión < 5 segundos
- [ ] Compresión > 70% de reducción
- [ ] Restore < 10 segundos
- [ ] Concurrent backups support

## 🔧 Implementación por Fases

### **Fase 1: Básico** (Día 29)
- [ ] Local backup storage
- [ ] Backup al finalizar sesión
- [ ] Compresión GZIP

### **Fase 2: Seguridad** (Día 30)  
- [ ] Encriptación AES-256-GCM
- [ ] Key management por usuario
- [ ] Verificación de integridad

### **Fase 3: Avanzado** (Futuro)
- [ ] Cloud storage backends
- [ ] Backup incremental
- [ ] Deduplicación de datos
