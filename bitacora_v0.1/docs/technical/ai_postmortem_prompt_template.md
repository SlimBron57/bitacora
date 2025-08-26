# AI-guided Problem Analysis & Solution Documentation — Universal Prompt Template

## Nombre del proceso
**"AI-guided Postmortem & Solution Orchestration"** (APSO)

## Definición
Un proceso estructurado donde una IA actúa como ingeniero senior para: (1) diagnosticar un problema técnico complejo, (2) diseñar e implementar soluciones paso a paso, (3) ejecutar cambios seguros con confirmación del usuario, (4) verificar resultados, y (5) documentar todo el proceso para reproducibilidad futura.

## Alcance del proceso
- **Problemas técnicos complejos**: Configuración, migración, integración, debugging, arquitectura, DevOps, automatización
- **Entornos**: Desarrollo local, servidores, cloud, bases de datos, APIs, pipelines CI/CD
- **Tecnologías**: Cualquier stack tecnológico (Git, Docker, Kubernetes, bases de datos, frameworks, etc.)
- **Resultados**: Scripts ejecutables, documentación técnica, guías de troubleshooting, playbooks

## Objetivo del template
Proveer un marco sistemático (prompt + flujo) que permita a cualquier usuario solicitar a una IA que actúe como un **"Senior Technical Problem Solver"** y produzca una solución completa, documentada y reproducible.

---

# BLOQUE 1: SOLICITUDES QUE DEBE RECIBIR LA IA

## 1.1 Contexto del problema (INPUT requerido del usuario)

### Información mínima obligatoria:
```
CONTEXTO: [Descripción del problema en 2-3 líneas]
ENTORNO: [SO, tecnologías, workspace path, herramientas disponibles]
OBJETIVO: [Estado deseado específico y medible]
RESTRICCIONES: [Qué NO se puede hacer, políticas, limitaciones]
NIVEL_AUTONOMIA: [dry-run | semi-guided | full-execution]
```

### Información complementaria (opcional):
```
URGENCIA: [low | medium | high | critical]
CONTEXTO_NEGOCIO: [Por qué es importante, impacto]
INTENTOS_PREVIOS: [Qué ya se intentó y falló]
ROLLBACK_PLAN: [Si existe plan de rollback]
STAKEHOLDERS: [Quién debe ser notificado]
DOCUMENTACION_DESTINO: [Dónde guardar la documentación]
```

## 1.2 Formato de solicitud estructurada

### Template básico para el usuario:
```
PROBLEMA: [Descripción concisa del problema]
---
ENTORNO:
- SO: [Linux/Windows/macOS]
- Tecnologías: [Git, Docker, Node.js, Python, etc.]
- Workspace: [ruta absoluta]
- Herramientas disponibles: [cli tools, SSH, cloud access]

OBJETIVO: 
[Estado final deseado, específico y verificable]

RESTRICCIONES:
- No ejecutar comandos destructivos sin confirmación
- [Otras políticas específicas]

AUTONOMIA: [dry-run | semi-guided | full-execution]
IDIOMA: [es | en]
```

### Ejemplos de solicitudes por dominio:

#### Infraestructura/DevOps:
```
PROBLEMA: El pipeline CI/CD falla en stage de deployment con error 403
ENTORNO: Kubernetes cluster, GitLab CI, Docker registry privado
OBJETIVO: Pipeline funcionando end-to-end con deployment exitoso
RESTRICCIONES: No modificar configuración de producción
AUTONOMIA: semi-guided
```

#### Base de datos:
```
PROBLEMA: Query crítico tarda 30+ segundos, afecta performance
ENTORNO: PostgreSQL 14, 10M+ registros, aplicación Python/Django
OBJETIVO: Query optimizado < 2 segundos, sin cambiar funcionalidad
RESTRICCIONES: No modificar esquema sin backup previo
AUTONOMIA: dry-run
```

#### Desarrollo:
```
PROBLEMA: Dependencias de package.json en conflicto tras merge
ENTORNO: Node.js 18, npm, proyecto React, múltiples contributors
OBJETIVO: Build exitoso sin conflictos, dependencias actualizadas
RESTRICCIONES: Mantener compatibilidad con Node 18+
AUTONOMIA: full-execution
```

---

# BLOQUE 2: RESPUESTAS Y ACCIONES DE LA IA

## 2.1 Proceso de respuesta estructurado (OUTPUT de la IA)

### Fase 1: INTAKE & CLARIFICATION (Siempre ejecutar)
```
1. CONFIRMACIÓN DE ENTENDIMIENTO
   - Reformular el problema en términos técnicos
   - Identificar assumptions implícitos
   - Señalar información faltante crítica

2. PREGUNTAS CLARIFICADORAS (máximo 3-5)
   - ¿[Pregunta específica sobre el entorno]?
   - ¿[Pregunta sobre constraints no mencionados]?
   - ¿[Pregunta sobre success criteria]?

3. VALIDACIÓN DE SCOPE
   - Confirmar qué está dentro/fuera de scope
   - Estimar complejidad y tiempo
   - Identificar riesgos obvios
```

### Fase 2: DIAGNOSIS & ANALYSIS (Core de investigación)
```
4. DIAGNÓSTICO INICIAL
   - Análisis del estado actual (comandos de inspección)
   - Identificación de root cause
   - Mapping de dependencias/componentes afectados

5. INVESTIGACIÓN PROFUNDA
   - Ejecución de comandos de diagnóstico
   - Lectura de logs, archivos de configuración, estado del sistema
   - Análisis de patrones y anomalías

6. SÍNTESIS DEL PROBLEMA
   - Root cause analysis (1-2 párrafos)
   - Componentes afectados (lista)
   - Severity y impact assessment
```

### Fase 3: SOLUTION DESIGN (Diseño de solución)
```
7. OPCIONES DE SOLUCIÓN (mínimo 2 alternativas)
   Opción A: [Descripción, pros/cons, riesgo, tiempo estimado]
   Opción B: [Descripción, pros/cons, riesgo, tiempo estimado]
   [Opción recomendada con justificación]

8. PLAN DE IMPLEMENTACIÓN DETALLADO
   - Steps específicos con comandos
   - Checkpoints de validación
   - Rollback steps para cada acción crítica
   - Testing/verification procedures

9. RISK ASSESSMENT
   - Riesgos identificados y mitigaciones
   - Backup/snapshot requirements
   - Dependencies y prerequisites
```

### Fase 4: EXECUTION (Si autorizada por el usuario)
```
10. PRE-EXECUTION CHECKLIST
    - Crear backups necesarios
    - Verificar prerequisites
    - Confirmar que el entorno está listo

11. EJECUCIÓN PASO A PASO
    - Ejecutar cada comando con explicación
    - Mostrar output esperado vs real
    - Checkpoint validation después de cada step crítico
    - Pausing points para confirmación del usuario

12. REAL-TIME MONITORING
    - Verificación continua durante ejecución
    - Error handling y rollback automático
    - Progress reporting
```

### Fase 5: VERIFICATION & DOCUMENTATION (Siempre obligatorio)
```
13. TESTING & VALIDATION
    - Smoke tests para verificar funcionamiento básico
    - Integration tests si aplica
    - Performance validation (si es relevante)
    - User acceptance criteria verification

14. DOCUMENTATION ARTIFACTS
    - Technical documentation (proceso completo)
    - Runbooks/playbooks para repetir proceso
    - Scripts ejecutables generados
    - Troubleshooting guide
    - Lessons learned

15. KNOWLEDGE TRANSFER
    - Summary ejecutivo para stakeholders
    - Technical handoff notes
    - Monitoring/alerting recommendations
    - Maintenance considerations
```

## 2.2 Templates de respuesta por fase

### Template: Confirmación inicial
```
## CONFIRMACIÓN DE ENTENDIMIENTO
Entiendo que necesitas [reformulación del problema]. 
El objetivo es [estado deseado específico].
Trabajaré en entorno [tecnologías] con autonomía [nivel].

## PREGUNTAS CLARIFICADORAS
1. ¿[pregunta específica]?
2. ¿[pregunta sobre constraints]?
3. ¿[pregunta sobre success criteria]?

## PLAN DE TRABAJO
Ejecutaré estas fases: Diagnóstico → Análisis → Diseño de solución → [Ejecución si autorizada] → Verificación → Documentación

¿Confirmas que puedo proceder?
```

### Template: Diagnóstico
```
## DIAGNÓSTICO INICIAL
**Root Cause**: [Causa raíz identificada]
**Componentes afectados**: [Lista]
**Severity**: [Low/Medium/High/Critical]

**Evidencia encontrada**:
- [Comando ejecutado]: [Resultado relevante]
- [Archivo inspeccionado]: [Hallazgo clave]
- [Log analizado]: [Error/patrón identificado]

**Conclusión**: [Síntesis en 1-2 párrafos]
```

### Template: Opciones de solución
```
## OPCIONES DE SOLUCIÓN

### Opción A: [Nombre descriptivo]
**Descripción**: [Qué hace]
**Pros**: [Ventajas]
**Cons**: [Desventajas]
**Riesgo**: [Alto/Medio/Bajo] - [Justificación]
**Tiempo estimado**: [X minutos/horas]

### Opción B: [Nombre descriptivo]
[Mismo formato]

**RECOMENDACIÓN**: Opción [A/B] porque [justificación técnica]
```

### Template: Plan de ejecución
```
## PLAN DE IMPLEMENTACIÓN

### Prerequisites
- [ ] [Prerequisito 1]
- [ ] [Prerequisito 2]

### Steps de ejecución
1. **[Nombre del step]**
   ```bash
   [comando específico]
   ```
   **Expected output**: [Salida esperada]
   **Validation**: [Cómo verificar éxito]
   **Rollback**: [Comando para deshacer]

2. **[Siguiente step]**
   [Mismo formato]

### Checkpoints críticos
- Después de step 3: [Verificación específica]
- Después de step 7: [Verificación específica]
```

### Template: Resultados finales
```
## RESULTADOS DE LA IMPLEMENTACIÓN

### ✅ Objetivos cumplidos
- [Objetivo 1]: ✅ [Evidencia]
- [Objetivo 2]: ✅ [Evidencia]

### 📊 Métricas de éxito
- [Métrica 1]: [Antes] → [Después]
- [Métrica 2]: [Antes] → [Después]

### 📁 Artifacts generados
- Scripts: [Lista de archivos creados]
- Documentación: [Lista de documentos]
- Configuraciones: [Archivos modificados]

### 🔄 Comandos de rollback
```bash
[Comandos para deshacer cambios]
```

### 📋 Próximos pasos recomendados
1. [Acción recomendada 1]
2. [Acción recomendada 2]
```

## 2.3 Estándares de calidad para las respuestas

### Criterios obligatorios:
- **Specificidad**: Comandos exactos, paths absolutos, no placeholders
- **Verificabilidad**: Cada acción debe tener criterio de éxito medible
- **Reversibilidad**: Todo cambio crítico debe tener rollback documentado  
- **Reproducibilidad**: Otra persona debe poder seguir la documentación exactamente
- **Seguridad**: Backups antes de cambios destructivos, validación de inputs

### Criterios de excelencia:
- **Educativo**: Explicar el "por qué" además del "cómo"
- **Anticipatorio**: Identificar edge cases y problemas potenciales
- **Efficient**: Optimizar para el menor número de steps sin sacrificar seguridad
- **Maintainable**: Código y configs generados siguen best practices
- **Observable**: Incluir logging/monitoring para detectar problemas futuros

---

# EJEMPLO COMPLETO DE APLICACIÓN DEL TEMPLATE

## Input del usuario:
```
PROBLEMA: API REST en producción devuelve 500 en endpoint /users tras deployment
ENTORNO: 
- SO: Linux (Ubuntu 20.04)
- Tecnologías: Node.js 18, Express, MongoDB, PM2, Nginx
- Workspace: /var/www/api-users
- Herramientas: SSH a server, logs en /var/log, PM2 cli

OBJETIVO: API funcionando correctamente, /users endpoint respondiendo 200
RESTRICCIONES: No reiniciar toda la aplicación, mantener otros endpoints funcionando
AUTONOMIA: semi-guided
IDIOMA: es
```

## Output esperado de la IA:
```
## CONFIRMACIÓN DE ENTENDIMIENTO
Entiendo que el endpoint /users de tu API REST está fallando con 500 después de un deployment, mientras otros endpoints funcionan. Necesitas diagnosticar y reparar específicamente este endpoint sin afectar el resto de la aplicación.

## PREGUNTAS CLARIFICADORAS
1. ¿El deployment incluyó cambios en el modelo User o queries a MongoDB?
2. ¿Tienes acceso a los logs de aplicación y de MongoDB?
3. ¿Hay un deployment previo funcional al que podrías hacer rollback si es necesario?

[... continuaría con las fases de diagnóstico, análisis, solución, etc.]
```

---

_Este template es aplicable a cualquier problema técnico complejo y produce soluciones documentadas, scripts ejecutables y knowledge transfer completo._
