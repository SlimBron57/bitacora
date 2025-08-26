# Ejemplos Completos de Procesamientos BitaFlow

## 📋 **Ejemplo 1: Plantilla de Proceso Completa**

```bfl
---
alias: BITA-TPL-CODE-REVIEW-v1
name: Proceso de Revisión de Código
slug: code-review-process
kind: TPL
version: 1
requires: [
  "BITA-TPL-GIT-FLOW-v1",
  "BITA-TPL-NOTIFICATIONS-v1"
]
---

# 🔍 Proceso de Revisión de Código

**Objetivo:** Asegurar calidad y consistencia del código antes del merge

;; Este proceso se activa automáticamente en cada PR
;; Requiere al menos 2 aprobaciones antes del merge

## 📝 Información del PR

- **Desarrollador:** {{developer_name}}
- **Rama:** {{feature_branch}}
- **Ticket:** {{jira_ticket}}
- **Reviewers:** {{assigned_reviewers}}

## 🔄 Flujo del Proceso

Crear PR ↦ (Análisis Automático + Revisión Manual) ↦ Aprobación ↦ Merge
         ↘                                      ↗            ↘
           Tests CI/CD ↦ Correcciones Necesarias              Deploy

## 📚 Includes de Plantillas

{{> BITA-TPL-CHECKLIST-QUALITY-v1 }}
{{> BITA-TPL-TESTING-GUIDELINES-v1 }}
{{> BITA-TPL-SECURITY-REVIEW-v1 }}

## ⚡ Validaciones Automáticas

- Linting: {{linter_status}}
- Tests: {{test_coverage}}%
- Seguridad: {{security_scan_result}}
- Performance: {{performance_score}}
```

## 📋 **Ejemplo 2: Documentación con Flujos Complejos**

```bfl
---
alias: PROC-INCIDENT-RESPONSE-v2
name: Respuesta a Incidentes de Producción
slug: incident-response
kind: PROC
version: 2
requires: ["BITA-TPL-NOTIFICATIONS-v1"]
---

# 🚨 Protocolo de Respuesta a Incidentes

**Objetivo:** Minimizar impacto y restaurar servicio lo antes posible

;; Proceso crítico - Disponible 24/7
;; Escalamiento automático según severidad

## 🎯 Clasificación por Severidad

### Severidad 1 - Crítico
```
Detección ↦ Alerta Inmediata ↦ (War Room + Escalamiento) ↦ Resolución
         ↘                   ↘                         ↗           ↘
           Logs + Monitoreo ↦  Comunicación Cliente ↦ Testing ↦    Post-Mortem
```

### Severidad 2 - Alto
```
Detección ↦ Asignación ↦ (Investigación + Fix) ↦ Validación ↦ Deploy
         ↘            ↘                       ↗           ↗
           Monitoreo ↦  Documentación ↦ Testing ↦ Comunicación
```

## 👥 Roles y Responsabilidades

- **Incident Commander:** {{incident_commander}}
- **Technical Lead:** {{tech_lead}}
- **Communications Lead:** {{comms_lead}}
- **Customer Success:** {{customer_success}}

## 📊 Métricas de Seguimiento

- **MTTR:** {{mean_time_to_resolution}}
- **MTBF:** {{mean_time_between_failures}}
- **Impacted Users:** {{affected_users_count}}
- **Revenue Impact:** ${{revenue_impact}}

{{> BITA-TPL-INCIDENT-CHECKLIST-v1 }}
{{> BITA-TPL-COMMUNICATION-TEMPLATE-v1 }}
```

## 📋 **Ejemplo 3: Pipeline de CI/CD**

```bfl
---
alias: BITA-TPL-CICD-PIPELINE-v3
name: Pipeline de Integración y Despliegue Continuo  
slug: cicd-pipeline
kind: TPL
version: 3
requires: [
  "BITA-TPL-DOCKER-BUILD-v1",
  "BITA-TPL-SECURITY-SCAN-v2",
  "BITA-TPL-DEPLOY-STRATEGIES-v1"
]
---

# 🚀 Pipeline CI/CD Completo

**Objetivo:** Automatizar el proceso desde commit hasta producción

;; Pipeline ejecutado en cada push a main
;; Rollback automático en caso de fallo

## 🔧 Configuración del Pipeline

```yaml
environment: {{target_environment}}
docker_image: {{app_name}}:{{version}}
deployment_strategy: {{deploy_strategy}}
```

## 🔄 Etapas del Pipeline

### Fase 1: Preparación y Validación
```
Trigger ↦ (Checkout + Cache) ↦ Dependencies ↦ Lint & Format
       ↘                    ↗              ↘
         Env Setup ↦ Config Validation ↦ Security Pre-check
```

### Fase 2: Testing y Calidad
```
Unit Tests ↦ (Integration Tests + E2E Tests) ↦ Coverage Report
          ↘                               ↗                ↘
            Performance Tests ↦ Security Scan ↦ Quality Gates
```

### Fase 3: Build y Packaging
```
Build ↦ (Docker Build + Asset Optimization) ↦ Registry Push
     ↘                                     ↗              ↘
       Artifact Creation ↦ Vulnerability Scan ↦ Signing & Verification
```

### Fase 4: Deployment
```
Deploy Staging ↦ (Smoke Tests + Health Checks) ↦ Deploy Prod
              ↘                              ↗             ↘
                Database Migration ↦ Config Update ↦ Traffic Routing
```

## 📈 Variables del Pipeline

- **Build ID:** {{build_id}}
- **Commit SHA:** {{commit_sha}}
- **Branch:** {{branch_name}}
- **Triggered By:** {{triggered_by}}
- **Environment:** {{environment}}
- **Version:** {{app_version}}

## 🛠️ Herramientas Integradas

{{> BITA-TPL-DOCKER-CONFIG-v1 }}
{{> BITA-TPL-KUBERNETES-DEPLOY-v1 }}
{{> BITA-TPL-MONITORING-SETUP-v1 }}

;; Notificaciones configuradas para Slack y email
;; Métricas enviadas a DataDog y Grafana
```

## 📋 **Ejemplo 4: Onboarding de Empleados**

```bfl
---
alias: PROC-EMPLOYEE-ONBOARDING-v1
name: Proceso de Incorporación de Empleados
slug: employee-onboarding
kind: PROC
version: 1
requires: ["BITA-TPL-HR-FORMS-v1"]
---

# 👋 Proceso de Onboarding

**Objetivo:** Integrar efectivamente nuevos empleados al equipo

;; Duración estimada: 2 semanas
;; Seguimiento automático por HR y manager directo

## 👤 Información del Nuevo Empleado

- **Nombre:** {{employee_name}}
- **Posición:** {{job_title}}
- **Departamento:** {{department}}
- **Manager:** {{direct_manager}}
- **Buddy:** {{assigned_buddy}}
- **Fecha Inicio:** {{start_date}}

## 📅 Timeline de Actividades

### Semana 1: Fundamentos
```
Día 1: Bienvenida ↦ (Setup Técnico + Documentos HR) ↦ Tour Oficina
                 ↘                                 ↗            ↘
                   Entrega Equipos ↦ Cuentas & Accesos ↦ Lunch Team
```

```  
Días 2-3: Capacitación ↦ (Producto + Procesos) ↦ Shadowing
                      ↘                       ↗          ↘
                        Cultura Company ↦ Tools Training ↦ 1:1 Manager
```

```
Días 4-5: Práctica ↦ (Primeros Tasks + Code Review) ↦ Feedback Session
                   ↘                              ↗                  ↘
                     Pair Programming ↦ Team Meetings ↦ Check-in HR
```

### Semana 2: Integración
```
Completa Onboarding ↦ (Proyecto Real + Responsabilidades) ↦ Evaluación
                    ↘                                    ↗             ↘
                      Networking ↦ Training Específico ↦ Plan 90 días
```

## ✅ Checklist de Completado

- [ ] **IT Setup:** {{it_setup_complete}}
- [ ] **HR Documentation:** {{hr_docs_complete}}  
- [ ] **Training Modules:** {{training_progress}}%
- [ ] **Access Granted:** {{access_granted}}
- [ ] **Equipment Delivered:** {{equipment_status}}
- [ ] **Buddy Assigned:** {{buddy_introduction}}

{{> BITA-TPL-WELCOME-PACKAGE-v1 }}
{{> BITA-TPL-TRAINING-CHECKLIST-v1 }}
{{> BITA-TPL-FEEDBACK-FORM-v1 }}

;; Evaluación automática a los 30, 60 y 90 días
;; Métricas de satisfacción y time-to-productivity
```

## 🔍 **Análisis de los Procesamientos**

### **Procesamientos de Sintaxis Detectados:**

1. **Front-matter YAML** - Metadatos estructurados
2. **Encabezados Markdown** - Estructura jerárquica (H1-H6)
3. **Comentarios** - Documentación inline con `;;`
4. **Operadores de Flujo** - Visualización de procesos con `↦ ↘ ↗ +`
5. **Placeholders** - Variables dinámicas con `{{variable}}`
6. **Includes** - Modularidad con `{{> TEMPLATE }}`
7. **Bloques de Código** - Syntax highlighting para YAML embebido
8. **Listas y Checkboxes** - Elementos de Markdown estándar

### **Funcionalidades de Edición Disponibles:**

- ✅ Auto-completado con snippets
- ✅ Comentado rápido con Ctrl+/  
- ✅ Navegación por brackets
- ✅ Auto-cierre de pares
- ✅ Plegado de código (folding)
- ✅ Find/Replace con sintaxis específica
- ✅ Indentación automática

Estos ejemplos muestran la versatilidad de BitaFlow para documentar procesos complejos, desde desarrollo de software hasta procedimientos organizacionales, con una sintaxis rica que combina lo mejor de YAML, Markdown y diagramas de flujo textuales.
