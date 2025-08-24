# Session State Management - Sistema Bitácora

## Resumen

El sistema bitácora implementa gestión inteligente de estados de sesión que permite:
- **Completed Sessions**: Sesiones completadas exitosamente
- **Pending Sessions**: Trabajo en progreso que se reanudará después  
- **Unclosed Sessions**: Sesiones que terminaron abruptamente sin END comando

## Funcionamiento del Sistema

### Comando END - Finalización de Sesión

Cuando ejecutas el comando `END`, el sistema:

1. **Pregunta por el estado de finalización:**
   ```
   📋 How do you want to close this session?
      1) Completed - All planned work finished
      2) Pending - Work in progress, will continue later
   ```

2. **Registra el marcador correspondiente:**
   - **Opción 1 (Completed)**: `{timestamp}_session-end: Work session completed`
   - **Opción 2 (Pending)**: `{timestamp}_session-end: Work session pending`

3. **Ejecuta el flujo completo de END:**
   - Backup del proyecto
   - Git push (si está habilitado)
   - Creación de PR draft (si está habilitado)
   - Parada del timestamp daemon

### Comando START - Validación de Estados

Cuando ejecutas el comando `START`, el sistema detecta automáticamente el estado de la sesión anterior:

#### 🟢 Sesión Completada (Completed)
```
✅ Previous session was completed: {record_name}
```
- **Comportamiento**: Crea automáticamente un nuevo record
- **Descripción del nuevo record**: `(New session initialized on {timestamp} - previous session was completed)`
- **Sin prompts al usuario**: El flujo es automático

#### 🟡 Sesión Pendiente (Pending) 
```
⏳ Previous session was marked as pending: {record_name}
   Work was left in progress to continue later.

Options:
   1) Continue in existing record (resume pending work)
   2) Start new record (abandon pending work and start fresh)
```
- **Opción 1**: Continúa en el record existente preservando el trabajo anterior
- **Opción 2**: Crea un nuevo record y marca la descripción como `(New session initialized on {timestamp} - previous pending work was abandoned)`

#### 🔴 Sesión No Cerrada (Unclosed)
```
⚠️  Previous session was not properly closed: {record_name}
   Session ended abruptly without END command.
   Continuing in the last record to preserve work.
```
- **Comportamiento**: Continúa automáticamente en el record existente
- **Sin prompts al usuario**: Preserva el trabajo para evitar pérdida de datos

## Marcadores en Records

Los records contienen marcadores específicos que permiten al sistema detectar el estado:

### Session Start
```
{timestamp}_session-start: Work session initialized
```

### Session End - Completed  
```
{timestamp}_session-end: Work session completed
```

### Session End - Pending
```
{timestamp}_session-end: Work session pending
```

### Sin Marcador de End
Ausencia de cualquier marcador `_session-end:` indica que la sesión terminó abruptamente.

## Lógica de Detección

El sistema utiliza la función `find_active_record_for_branch()` que:

1. **Busca records** que coincidan con el nombre de la rama actual
2. **Ordena por timestamp** para encontrar el más reciente  
3. **Analiza el contenido** buscando marcadores `_session-end:`
4. **Determina el estado** basado en la presencia y tipo de marcador

## Flujos de Trabajo Recomendados

### Trabajo Completado
```bash
START    # Inicia nueva sesión
ACTION "task_1" "Implemented feature X"  
ACTION "task_2" "Added tests for feature X"
END      # Opción 1: Completed
```

### Trabajo en Progreso
```bash  
START    # Inicia nueva sesión
ACTION "investigation" "Researching implementation approach"
END      # Opción 2: Pending

# Más tarde...
START    # Opción 1: Continue in existing record
ACTION "implementation" "Started coding feature"
END      # Opción 1: Completed
```

### Recuperación de Sesión Interrumpida
```bash
START    # Si la sesión anterior terminó abruptamente
         # El sistema continúa automáticamente en el record existente
ACTION "recovery" "Resuming interrupted work"
END      # Opción 1 o 2 según corresponda
```

## Beneficios del Sistema

- **🔄 Continuidad**: Permite reanudar trabajo interrumpido
- **📝 Historial Completo**: Mantiene trazabilidad de todas las sesiones  
- **🛡️ Prevención de Pérdidas**: Recupera automáticamente sesiones abruptas
- **🎯 Flexibilidad**: Adapta el flujo según el contexto del trabajo
- **📊 Visibilidad**: Estado claro de cada sesión en los records

## Configuración y Variables de Entorno

El sistema respeta las variables de entorno estándar de bitácora:
- `BITACORA_NO_AUTO_COMMIT=true`: Desactiva auto-commits
- `BITACORA_NO_AUTO_PR=true`: Desactiva auto-creación de PRs
- `BITACORA_NO_AUTO_PUSH=true`: Desactiva auto-push
- `BITACORA_PR_DRAFT=false`: Crea PRs como ready en lugar de draft

## Integración con Git

La gestión de estados de sesión se integra completamente con:
- **Auto-commits**: Después de cada ACTION
- **Auto-push**: Al finalizar con END
- **Auto-PR creation**: Como draft al finalizar con END
- **Branch management**: Records vinculados específicamente a cada branch

Este sistema proporciona un flujo de trabajo robusto y flexible que se adapta a diferentes patrones de desarrollo y permite gestión eficiente de sesiones de trabajo interrumpidas o extendidas.
