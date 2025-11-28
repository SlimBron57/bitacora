```yaml
# === DATOS DE AUDITORÍA ===
Archivo: ROADMAP_V2/METODOLOGIA_V1_6_GIT_CHECKLIST.md
Versión: 1.0.0
Fecha Creación: 2025-11-28
Última Actualización: 2025-11-28 11:40:00
Autor: Sistema Bitácora - Metodología Git + Checklist v1.6
Propósito: Metodología integrada de sincronización Git ↔ Checklist para desarrollo organizado
Estado: 🚀 ACTIVO - Metodología oficial desde v1.6
Relacionado Con:
  - CHECKLIST_V2.md (source of truth)
  - CHECKLIST_TREE_V2.md (dependencias)
  - GUIA.md SECCIÓN 2.5 (workflow)
Inspiración: "Branch name = Milestone alcanzable en 2-4 semanas"
# === FIN DATOS DE AUDITORÍA ===
```

# 🔄 METODOLOGÍA v1.6: Git + Checklist Integrado

## 📖 ÍNDICE

1. [Problema que Resolvemos](#problema-que-resolvemos)
2. [Principio Central](#principio-central)
3. [Estructura v1.6](#estructura-v16)
4. [Workflow Completo](#workflow-completo)
5. [Branch Naming Convention](#branch-naming-convention)
6. [Commit Message Template](#commit-message-template)
7. [Checklist Enhanced Format](#checklist-enhanced-format)
8. [Validation Script](#validation-script)
9. [Ejemplos Reales](#ejemplos-reales)
10. [FAQ](#faq)

---

## 🎯 PROBLEMA QUE RESOLVEMOS

### Situación Pre-v1.6

```
❌ PROBLEMA: Sincronización manual Git ↔ Checklist

Branch: feature/v1.5-pixel-native
├─ Nombre prometía: PXLang pixel-native implementation
├─ Work completed: v1.0-beta (ShuiDao + docs)
└─ Gap: Branch name ≠ milestone alcanzado

Checklist: CHECKLIST_V2.md
├─ Tasks marcadas [x] post-facto
├─ No hay commit hash references
└─ No hay branch mapping

Resultado:
- Confusión sobre qué está en qué branch
- Tareas completadas sin trazabilidad
- Branches con nombres no descriptivos
- Merge difícil de justificar
```

### Solución v1.6

```
✅ SOLUCIÓN: Sincronización atómica

Branch: feature/v1.1-rest-api
├─ Nombre = Milestone claro (REST API Layer)
├─ Scope = Phase 7 (14 tasks)
└─ ETA = 2-3 weeks (alcanzable)

Checklist: CHECKLIST_V2.md
├─ Phase 7: REST API Layer
├─ Branch: feature/v1.1-rest-api
├─ Tasks con commit hash: [x] 7.1 (commit: abc123)
└─ Progress tracking: 5/14 (36%)

Git commits:
├─ feat(api): Task 7.1 - POST /biographical/entry
├─ test(api): Task 7.2 - Integration tests API
└─ docs(api): Task 7.3 - API endpoints docs

Resultado:
✅ Branch name = milestone real
✅ Checklist = source of truth
✅ Commits = trazabilidad atómica
✅ Merge = justificación clara
```

---

## 💡 PRINCIPIO CENTRAL

> **"Branch name = Milestone alcanzable en 2-4 semanas"**  
> **"Checklist = Source of truth único"**  
> **"Git commits = Progreso atómico documentado"**

### Reglas de Oro

1. **Branch Names Descriptivos**
   ```bash
   # ✅ BIEN
   feature/v1.1-rest-api
   feature/v1.2-sensory-zoom
   feature/v1.5-pxlang-impl
   
   # ❌ MAL
   feature/new-stuff
   dev-branch
   test
   ```

2. **Milestones Alcanzables**
   ```
   ✅ 2-4 semanas = realistic sprint
   ❌ 3 meses = demasiado largo
   ❌ 3 días = demasiado corto
   ```

3. **Checklist = Truth**
   ```
   Si no está en CHECKLIST_V2.md → no existe
   Si está [x] pero sin commit → inconsistencia
   Si commit sin task → warning
   ```

---

## 🏗️ ESTRUCTURA v1.6

```
┌─────────────────────────────────────────────────────────┐
│ CHECKLIST_V2.md (Source of Truth)                       │
│ ─────────────────────────────────────────────────────   │
│ Versión: Semantic (major.minor.patch)                  │
│ Milestone: v1.X.Y                                       │
│                                                         │
│ ## Phase Z: NOMBRE_MILESTONE (Target: v1.X.Y)          │
│ Estado: 🚧 IN PROGRESS | ✅ COMPLETE                    │
│ Branch: feature/v1.X-milestone-name                    │
│ ETA: YYYY-MM-DD (2-4 weeks)                            │
│ Dependencies: [Phase Y Task Y.Z, ...]                  │
│                                                         │
│ - [x] Z.1 - Task name (commit: abc123) ✅ YYYY-MM-DD   │
│ - [ ] Z.2 - Task name (blocked by: Z.1) ⏸️             │
│ - [ ] Z.3 - Task name 🚧                               │
│                                                         │
│ Progress: Y/Z tasks (P%)                               │
│ Metrics:                                                │
│   - Tests: A/B (C%)                                    │
│   - Performance: <summary>                             │
└─────────────────────────────────────────────────────────┘
         ↓ sync automático
┌─────────────────────────────────────────────────────────┐
│ Git Branch Strategy                                     │
│ ─────────────────────────────────────────────────────   │
│ main (stable releases)                                  │
│ ├─ v1.0.0-beta (tag) ✅                                │
│ └─ v1.1.0 (future tag)                                 │
│                                                         │
│ feature/v1.1-rest-api                                   │
│ ├─ Scope: REST API endpoints (Phase 7)                 │
│ ├─ Duration: 2-3 weeks                                 │
│ ├─ Tasks: 14 (mapped in CHECKLIST Phase 7)            │
│ └─ Merge trigger: Phase 7 complete                    │
│                                                         │
│ feature/v1.2-sensory-zoom                              │
│ ├─ Scope: SENSORY ENGINE + Zoom integration           │
│ ├─ Duration: 3-4 weeks                                 │
│ ├─ Tasks: 8 (mapped in CHECKLIST Phase 8)             │
│ └─ Merge trigger: Phase 8 complete                    │
└─────────────────────────────────────────────────────────┘
         ↓ validation
┌─────────────────────────────────────────────────────────┐
│ scripts/sync_checklist_git.sh (Validation)            │
│ ─────────────────────────────────────────────────────   │
│ ✅ Branch name ↔ milestone coincide                    │
│ ✅ Tasks [x] tienen commit hash                        │
│ ✅ Commits referencian task correcta                   │
│ ⚠️  Inconsistencias detectadas → fix them              │
└─────────────────────────────────────────────────────────┘
```

---

## 🔄 WORKFLOW COMPLETO

### PASO 1: INICIO DE MILESTONE

**a) Definir Milestone en CHECKLIST**

```markdown
## Phase 7: REST API Layer (Target: v1.1.0)

Estado: 🚧 IN PROGRESS
Branch: feature/v1.1-rest-api
ETA: 2025-12-15 (2-3 weeks)
Dependencies: [Phase 6 Complete]

### Tasks

- [ ] 7.1 - POST /biographical/entry endpoint
- [ ] 7.2 - GET /biographical/:id endpoint
- [ ] 7.3 - Integration tests REST API
- [ ] 7.4 - API documentation OpenAPI
- [ ] 7.5 - Rate limiting middleware
...

Progress: 0/14 tasks (0%)
Metrics:
  - Tests: 0/50 (target: 100%)
  - Performance: N/A (target: <200ms p95)
```

**b) Crear Branch desde main**

```bash
# Asegurar main actualizado
git checkout main
git pull origin main

# Crear branch descriptivo
git checkout -b feature/v1.1-rest-api

# Primer commit: Initialize milestone
git add ROADMAP_V2/CHECKLIST_V2.md
git commit -m "docs: Initialize Phase 7 - REST API Layer

- Branch: feature/v1.1-rest-api
- Tasks: 14 total (0 complete)
- ETA: 2025-12-15
- Dependencies: Phase 6 Complete"

# Push branch
git push -u origin feature/v1.1-rest-api
```

### PASO 2: PROGRESO ATÓMICO

**Por cada task completada:**

```bash
# 1. Implementar task (código + tests)
# ... escribir código ...

# 2. Commit con referencia a task
git add src/api/biographical.rs tests/api_test.rs
git commit -m "feat(api): Task 7.1 - POST /biographical/entry

- Implements: CHECKLIST_V2.md Phase 7 Task 7.1
- Endpoint: POST /api/v1/biographical/entry
- Tests: 5/5 passing (test_post_entry, test_validation, ...)
- Performance: 87ms p95 (target: <200ms) ✅
- Dependencies: TelescopeDB v1.0"

# 3. Actualizar CHECKLIST INMEDIATAMENTE (mismo commit o siguiente)
# En CHECKLIST_V2.md:
# - [x] 7.1 - POST /biographical/entry endpoint (commit: abc123) ✅ 2025-11-29

git add ROADMAP_V2/CHECKLIST_V2.md
git commit -m "docs: Mark Task 7.1 complete

- Phase 7 progress: 1/14 tasks (7%)
- Commit reference: abc123
- Next: Task 7.2 (GET endpoint)"

# 4. Push
git push origin feature/v1.1-rest-api
```

**Cada commit debe:**
- Mencionar task number explícitamente
- Incluir métricas (tests, performance)
- Referenciar dependencias si aplica
- Ser atómico (1 task = 1-2 commits)

### PASO 3: PROGRESO INTERMEDIO

**Actualizar progreso periódicamente:**

```markdown
## Phase 7: REST API Layer (Target: v1.1.0)

Estado: 🚧 IN PROGRESS
Branch: feature/v1.1-rest-api
ETA: 2025-12-15 (on track 🎯)
Dependencies: [Phase 6 Complete ✅]

### Tasks

- [x] 7.1 - POST /biographical/entry (commit: abc123) ✅ 2025-11-29
- [x] 7.2 - GET /biographical/:id (commit: def456) ✅ 2025-11-30
- [x] 7.3 - Integration tests REST API (commit: ghi789) ✅ 2025-12-01
- [ ] 7.4 - API documentation OpenAPI 🚧
- [ ] 7.5 - Rate limiting middleware ⏸️ (waiting: 7.4)
...

Progress: 3/14 tasks (21%)
Metrics:
  - Tests: 15/50 (30%)
  - Performance: 92ms p95 (target: <200ms) ✅
  - Coverage: 87% (target: >80%) ✅
```

### PASO 4: CIERRE DE MILESTONE

**Cuando 100% completo:**

```bash
# a) Verificar checklist 100%
echo "Phase 7: 14/14 tasks ✅"

# b) Actualizar CHECKLIST final
# En CHECKLIST_V2.md:
Estado: ✅ COMPLETE
Completion Date: 2025-12-12 (3 days ahead of ETA 🎉)
Final Metrics:
  - Tests: 50/50 (100%) ✅
  - Performance: 134ms p95 (target: <200ms) ✅
  - Coverage: 92% (target: >80%) ✅
  - Bugs fixed: 3
  - Duration: 2 weeks 1 day

Next: Phase 8 (feature/v1.2-sensory-zoom)

git add ROADMAP_V2/CHECKLIST_V2.md
git commit -m "docs: Phase 7 REST API Layer COMPLETE

- All 14 tasks completed ✅
- Ahead of schedule (3 days early)
- All metrics surpassed targets
- Ready for merge to main"

git push origin feature/v1.1-rest-api

# c) Merge a main
git checkout main
git pull origin main
git merge feature/v1.1-rest-api --no-ff -m "Merge feature/v1.1-rest-api

Phase 7 REST API Layer COMPLETE:
- [x] 7.1 - POST /biographical/entry ✅
- [x] 7.2 - GET /biographical/:id ✅
- [x] 7.3 - Integration tests ✅
... (all 14 tasks)

Metrics:
- Tests: 50/50 (100%)
- Performance: 134ms p95
- Coverage: 92%
- Duration: 2 weeks 1 day

Next: Phase 8 SENSORY ENGINE + Zoom (feature/v1.2-sensory-zoom)"

# d) Tag release
git tag -a v1.1.0 -m "Release v1.1.0 - REST API Layer

Phase 7 Complete:
- 14/14 tasks ✅
- 50 tests passing
- Full API documentation
- Rate limiting implemented

Breaking Changes: None
New Features:
- POST /api/v1/biographical/entry
- GET /api/v1/biographical/:id
- OpenAPI 3.0 specification
- JWT authentication
- Rate limiting (100 req/min)

Next: v1.2.0 SENSORY ENGINE + Zoom integration"

# e) Push todo
git push origin main
git push origin v1.1.0

# f) Cleanup branch (opcional, mantener histórico)
# git branch -d feature/v1.1-rest-api
# git push origin --delete feature/v1.1-rest-api
```

---

## 🏷️ BRANCH NAMING CONVENTION

### Template

```bash
feature/v{major}.{minor}-{milestone-slug}

Donde:
- major.minor = Semantic version (v1.1, v1.2, v1.5)
- milestone-slug = Descripción corta kebab-case
```

### Ejemplos Válidos

```bash
feature/v1.1-rest-api          # Phase 7: REST API endpoints
feature/v1.2-sensory-zoom      # Phase 8: SENSORY ENGINE + Zoom
feature/v1.3-mtt-dsl           # Phase 9: MTT-DSL templates
feature/v1.4-icebreaker        # Phase 10: IceBreaker implementation
feature/v1.5-pxlang-impl       # Phase 11: PXLang code implementation
feature/v1.6-routier-network   # Phase 12: Routier network algorithms
feature/v2.0-production        # Major release
```

### Antipatrones

```bash
❌ feature/api           # No version number
❌ dev-branch           # No descriptivo
❌ test                 # Demasiado genérico
❌ fix-bug-123          # Use hotfix/ prefix
❌ feature/long-name-that-describes-everything-we-want-to-do
```

### Branch Types

```bash
# Feature branches (milestone alcanzable)
feature/v{X}.{Y}-{name}

# Hotfix branches (bugs críticos en producción)
hotfix/v{X}.{Y}.{Z}-{bug-description}
Example: hotfix/v1.1.1-memory-leak

# Refactor branches (no cambia funcionalidad)
refactor/{scope}-{description}
Example: refactor/shuidao-error-handling

# Docs branches (solo documentación)
docs/{scope}-{description}
Example: docs/api-endpoints-v1.1
```

---

## 📝 COMMIT MESSAGE TEMPLATE

### Template Estándar

```
{type}({scope}): Task {phase}.{number} - {description}

- Implements: CHECKLIST_V2.md Phase X Task X.Y
- {Details line 1}
- {Details line 2}
- {Optional: Closes #issue_number}

Examples:
feat(api): Task 7.1 - POST /biographical/entry
test(integration): Task 7.3 - REST API integration tests
docs(checklist): Update Phase 7 progress to 21%
```

### Commit Types

```bash
feat      # Nueva funcionalidad
fix       # Bug fix
test      # Añadir/modificar tests
docs      # Documentación
refactor  # Refactoring (no cambia comportamiento)
perf      # Optimización performance
style     # Formatting (no afecta código)
chore     # Tareas mantenimiento
```

### Scope Examples

```bash
(api)           # API endpoints
(shuidao)       # ShuiDao cognitive engine
(telescopedb)   # TelescopeDB
(voxeldb)       # VoxelDB
(tests)         # Testing infrastructure
(checklist)     # CHECKLIST updates
(docs)          # Documentation
(integration)   # Integration between components
```

### Ejemplos Reales

```bash
# Feature implementation
feat(api): Task 7.1 - POST /biographical/entry

- Implements: CHECKLIST_V2.md Phase 7 Task 7.1
- Endpoint: POST /api/v1/biographical/entry
- Request body: BiographicalEntry JSON
- Response: 201 Created with entry_id
- Tests: 5/5 passing
- Performance: 87ms p95 (target: <200ms) ✅
- Dependencies: TelescopeDB v1.0

# Testing
test(integration): Task 7.3 - REST API integration tests

- Implements: CHECKLIST_V2.md Phase 7 Task 7.3
- Tests: 15 integration tests
- Coverage: POST, GET, PUT, DELETE endpoints
- Scenarios: happy path, validation, auth, rate limiting
- All tests passing ✅
- Duration: 2.3s

# Documentation
docs(checklist): Mark Task 7.1-7.3 complete

- Phase 7 progress: 3/14 tasks (21%)
- Commit references added
- Metrics updated
- Next: Task 7.4 (API documentation)

# Bug fix
fix(api): Task 7.5 - Rate limiting off-by-one error

- Fixes: Rate limit was 101 req/min instead of 100
- Root cause: Inclusive comparison (<=) should be (<)
- Tests added: test_rate_limit_exact_boundary
- Closes #42
```

---

## 📊 CHECKLIST ENHANCED FORMAT

### Phase Header Template

```markdown
## Phase {N}: {MILESTONE_NAME} (Target: v{X}.{Y}.{Z})

Estado: 🚧 IN PROGRESS | ✅ COMPLETE | ⏸️ BLOCKED | 🔥 CRITICAL
Branch: feature/v{X}.{Y}-{milestone-slug}
ETA: YYYY-MM-DD ({N} weeks)
Dependencies: [Phase M Task M.K, Phase P Task P.Q, ...]
Related Docs: [ROADMAP_V2/path/to/doc.md, ...]

### Context

{1-2 paragraph description of milestone}

### Success Criteria

- [ ] Metric 1: {value} (target: {threshold})
- [ ] Metric 2: {value} (target: {threshold})
- [ ] All tasks completed
- [ ] Documentation updated
- [ ] Tests passing

### Tasks

- [ ] {N}.1 - Task name 🚧
- [ ] {N}.2 - Task name ⏸️ (blocked by: {N}.1)
- [x] {N}.3 - Task name (commit: abc123) ✅ YYYY-MM-DD
...

### Progress

Progress: {X}/{Y} tasks ({P}%)
Metrics:
  - Tests: {A}/{B} ({C}%)
  - Performance: {metric} (target: {threshold})
  - Coverage: {X}% (target: >{Y}%)
  - Bugs: {fixed}/{total}

### Notes

- {Important note 1}
- {Important note 2}
```

### Task Line Format

```markdown
- {status} {phase}.{number} - {task_name} ({commit_ref}) {completion_emoji} {date}

status:
  [ ]  not started
  [~]  in progress
  [x]  completed
  [!]  blocked

commit_ref (solo si completed):
  (commit: abc123)

completion_emoji (solo si completed):
  ✅

date (solo si completed):
  YYYY-MM-DD

Examples:
- [ ] 7.1 - POST /biographical/entry
- [~] 7.2 - GET /biographical/:id 🚧
- [x] 7.3 - Integration tests (commit: ghi789) ✅ 2025-12-01
- [!] 7.4 - API docs ⏸️ (blocked by: 7.3)
```

### Estado Icons

```
🚧  IN PROGRESS (actively working)
✅  COMPLETE (all tasks done)
⏸️  BLOCKED (waiting for dependency)
🔥  CRITICAL (urgent, needs attention)
🎯  ON TRACK (progressing as planned)
⚠️  AT RISK (behind schedule or issues)
💯  EXCEEDS (surpassing targets)
```

---

## 🔍 VALIDATION SCRIPT

### scripts/sync_checklist_git.sh (Propuesta)

```bash
#!/bin/bash

# ================================================================
# VALIDATION SCRIPT: Git ↔ Checklist Sync
# ================================================================
# Propósito: Validar sincronización entre branch, commits y checklist
# Uso: ./scripts/sync_checklist_git.sh
# Output: ✅ OK | ⚠️ WARNINGS | ❌ ERRORS + acciones requeridas
# ================================================================

set -e

echo "🔍 VALIDACIÓN BRANCH + CHECKLIST"
echo "================================"

# 1. Get current branch
BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo "Branch actual: $BRANCH"

# 2. Extract checklist version
CHECKLIST="ROADMAP_V2/CHECKLIST_V2.md"
if [[ ! -f "$CHECKLIST" ]]; then
    echo "❌ ERROR: CHECKLIST_V2.md no encontrado"
    exit 1
fi

VERSION=$(grep "^Versión:" "$CHECKLIST" | head -1 | sed 's/Versión: //')
echo "Checklist: CHECKLIST_V2.md $VERSION"

# 3. Identify current phase
# Extract phase from checklist based on branch name or Estado field
PHASE=$(grep -A 5 "Estado:.*IN PROGRESS" "$CHECKLIST" | grep "^## Phase" | head -1 || echo "")

if [[ -z "$PHASE" ]]; then
    echo "⚠️  WARNING: No hay Phase IN PROGRESS en checklist"
else
    echo "Phase actual: $PHASE"
fi

# 4. Validate branch name matches milestone
if [[ "$BRANCH" == feature/* ]]; then
    # Extract expected milestone from branch name
    # e.g., feature/v1.1-rest-api → Phase 7 REST API
    
    EXPECTED_PHASE=$(grep "Branch: $BRANCH" "$CHECKLIST" || echo "")
    
    if [[ -z "$EXPECTED_PHASE" ]]; then
        echo "⚠️  WARNING: Branch '$BRANCH' no encontrado en CHECKLIST"
        echo "   Acción: Agregar 'Branch: $BRANCH' al Phase correspondiente"
    else
        echo "✅ Branch name coincide con milestone"
    fi
fi

# 5. Validate completed tasks have commit hash
echo ""
echo "Validando tasks completadas..."

COMPLETED_WITHOUT_COMMIT=$(grep -P '^\s*-\s*\[x\]' "$CHECKLIST" | grep -v "(commit:" || true)

if [[ -n "$COMPLETED_WITHOUT_COMMIT" ]]; then
    echo "⚠️  WARNING: Tasks marcadas [x] sin commit hash:"
    echo "$COMPLETED_WITHOUT_COMMIT"
    echo "   Acción: Agregar (commit: HASH) a cada task [x]"
else
    echo "✅ Todas las tareas [x] tienen commit hash"
fi

# 6. Validate commits reference task numbers
echo ""
echo "Validando commits recientes..."

RECENT_COMMITS=$(git log --oneline -10)

# Check if commits mention Task X.Y
COMMITS_WITHOUT_TASK=$(echo "$RECENT_COMMITS" | grep -v "Task [0-9]\+\.[0-9]\+" || true)

if [[ -n "$COMMITS_WITHOUT_TASK" ]]; then
    echo "⚠️  WARNING: Algunos commits no referencian Task:"
    echo "$COMMITS_WITHOUT_TASK"
    echo "   Acción: Usar formato 'feat(scope): Task X.Y - description'"
else
    echo "✅ Todos los commits recientes referencian tasks"
fi

# 7. Check for inconsistencies
echo ""
echo "Verificando inconsistencias..."

# Tasks with commit but not marked [x]
# (This is complex, would need parsing commit messages and cross-referencing)

echo "✅ Validación completa"

# 8. Summary
echo ""
echo "════════════════════════════════"
echo "RESUMEN"
echo "════════════════════════════════"
echo "Branch: $BRANCH"
echo "Checklist: $VERSION"
echo "Phase: ${PHASE:-N/A}"
echo ""
echo "Próximos pasos sugeridos:"
echo "1. Implementar próxima task pendiente"
echo "2. Commit con formato: 'feat(scope): Task X.Y - description'"
echo "3. Actualizar CHECKLIST_V2.md: marcar [x] + (commit: HASH)"
echo "4. Push: git push origin $BRANCH"
```

### Uso

```bash
# Ejecutar validación
./scripts/sync_checklist_git.sh

# Output esperado:
🔍 VALIDACIÓN BRANCH + CHECKLIST
================================
Branch actual: feature/v1.1-rest-api
Checklist: CHECKLIST_V2.md v2.26
Phase actual: ## Phase 7: REST API Layer

✅ Branch name coincide con milestone
✅ Todas las tareas [x] tienen commit hash
✅ Todos los commits recientes referencian tasks
✅ Validación completa

════════════════════════════════
RESUMEN
════════════════════════════════
Branch: feature/v1.1-rest-api
Checklist: v2.26
Phase: Phase 7: REST API Layer

Próximos pasos sugeridos:
1. Implementar próxima task pendiente
2. Commit con formato: 'feat(scope): Task X.Y - description'
3. Actualizar CHECKLIST_V2.md: marcar [x] + (commit: HASH)
4. Push: git push origin feature/v1.1-rest-api
```

---

## 💎 EJEMPLOS REALES

### Ejemplo 1: Phase 7 REST API Layer

**CHECKLIST_V2.md:**

```markdown
## Phase 7: REST API Layer (Target: v1.1.0)

Estado: ✅ COMPLETE
Branch: feature/v1.1-rest-api
ETA: 2025-12-15 (completed 2025-12-12, 3 days ahead 🎉)
Dependencies: [Phase 6 Complete ✅]
Related Docs: [ROADMAP_V2/06_DOCUMENTACION/API_ENDPOINTS.md]

### Context

Implement RESTful API endpoints for biographical data management.
Includes authentication, rate limiting, and OpenAPI documentation.

### Success Criteria

- [x] All 14 tasks completed ✅
- [x] Tests: 50/50 (100%) ✅
- [x] Performance: <200ms p95 ✅
- [x] Documentation: OpenAPI 3.0 ✅

### Tasks

- [x] 7.1 - POST /biographical/entry (commit: a1b2c3d) ✅ 2025-11-29
- [x] 7.2 - GET /biographical/:id (commit: e4f5g6h) ✅ 2025-11-30
- [x] 7.3 - Integration tests REST API (commit: i7j8k9l) ✅ 2025-12-01
- [x] 7.4 - API documentation OpenAPI (commit: m0n1o2p) ✅ 2025-12-03
- [x] 7.5 - Rate limiting middleware (commit: q3r4s5t) ✅ 2025-12-05
- [x] 7.6 - JWT authentication (commit: u6v7w8x) ✅ 2025-12-06
- [x] 7.7 - Error handling standardized (commit: y9z0a1b) ✅ 2025-12-08
- [x] 7.8 - CORS configuration (commit: c2d3e4f) ✅ 2025-12-09
- [x] 7.9 - Request validation (commit: g5h6i7j) ✅ 2025-12-10
- [x] 7.10 - Response compression (commit: k8l9m0n) ✅ 2025-12-10
- [x] 7.11 - Health check endpoint (commit: o1p2q3r) ✅ 2025-12-11
- [x] 7.12 - Metrics endpoint (commit: s4t5u6v) ✅ 2025-12-11
- [x] 7.13 - Docker deployment config (commit: w7x8y9z) ✅ 2025-12-12
- [x] 7.14 - Load testing (commit: a0b1c2d) ✅ 2025-12-12

### Progress

Progress: 14/14 tasks (100%) ✅
Metrics:
  - Tests: 50/50 (100%) ✅
  - Performance: 134ms p95 (target: <200ms) ✅
  - Coverage: 92% (target: >80%) ✅
  - Load test: 500 req/s sustained ✅

### Notes

- Completed 3 days ahead of schedule
- All metrics exceeded targets
- Ready for production deployment
- Next: Phase 8 SENSORY ENGINE + Zoom
```

**Git History:**

```bash
$ git log --oneline feature/v1.1-rest-api

a0b1c2d feat(api): Task 7.14 - Load testing with wrk
w7x8y9z feat(deploy): Task 7.13 - Docker deployment config
s4t5u6v feat(api): Task 7.12 - Metrics endpoint /api/v1/metrics
o1p2q3r feat(api): Task 7.11 - Health check /api/v1/health
k8l9m0n feat(api): Task 7.10 - Response compression gzip
g5h6i7j feat(api): Task 7.9 - Request validation middleware
c2d3e4f feat(api): Task 7.8 - CORS configuration
y9z0a1b feat(api): Task 7.7 - Error handling standardized
u6v7w8x feat(api): Task 7.6 - JWT authentication
q3r4s5t feat(api): Task 7.5 - Rate limiting middleware
m0n1o2p docs(api): Task 7.4 - OpenAPI 3.0 specification
i7j8k9l test(api): Task 7.3 - Integration tests REST API
e4f5g6h feat(api): Task 7.2 - GET /biographical/:id
a1b2c3d feat(api): Task 7.1 - POST /biographical/entry
xyz9876 docs: Initialize Phase 7 - REST API Layer
```

**Merge Commit:**

```bash
$ git show --format=fuller abc1234

commit abc1234def5678ghi9012jkl3456mno7890pqr
Merge: e01b437 a0b1c2d
Author: Eduardo <edgi@bitacora.dev>
Date:   Wed Dec 12 2025 18:30:00 -0500

    Merge feature/v1.1-rest-api
    
    Phase 7 REST API Layer COMPLETE:
    - [x] 7.1 - POST /biographical/entry ✅
    - [x] 7.2 - GET /biographical/:id ✅
    - [x] 7.3 - Integration tests ✅
    - [x] 7.4 - API documentation OpenAPI ✅
    - [x] 7.5 - Rate limiting middleware ✅
    - [x] 7.6 - JWT authentication ✅
    - [x] 7.7 - Error handling standardized ✅
    - [x] 7.8 - CORS configuration ✅
    - [x] 7.9 - Request validation ✅
    - [x] 7.10 - Response compression ✅
    - [x] 7.11 - Health check endpoint ✅
    - [x] 7.12 - Metrics endpoint ✅
    - [x] 7.13 - Docker deployment ✅
    - [x] 7.14 - Load testing ✅
    
    Metrics:
    - Tests: 50/50 (100%)
    - Performance: 134ms p95 (target: <200ms) ✅
    - Coverage: 92%
    - Load: 500 req/s sustained
    - Duration: 2 weeks 1 day (3 days ahead of schedule)
    
    Breaking Changes: None
    
    New Features:
    - Full REST API for biographical data
    - JWT authentication with refresh tokens
    - Rate limiting (100 req/min per user)
    - OpenAPI 3.0 specification
    - Health check + metrics endpoints
    - Docker deployment ready
    
    Next: Phase 8 SENSORY ENGINE + Zoom (feature/v1.2-sensory-zoom)
```

**Tag:**

```bash
$ git tag -a v1.1.0 -m "Release v1.1.0 - REST API Layer

Phase 7 Complete:
- 14/14 tasks ✅
- 50 tests passing (100%)
- Full API documentation
- Production ready

Breaking Changes: None

New Features:
- POST /api/v1/biographical/entry
- GET /api/v1/biographical/:id
- PUT /api/v1/biographical/:id
- DELETE /api/v1/biographical/:id
- JWT authentication
- Rate limiting (100 req/min)
- OpenAPI 3.0 specification
- Health check /api/v1/health
- Metrics /api/v1/metrics

Performance:
- p50: 45ms
- p95: 134ms
- p99: 187ms
- Max throughput: 500 req/s

Next: v1.2.0 SENSORY ENGINE + Zoom integration"
```

---

## ❓ FAQ

### 1. ¿Qué pasa si un branch toma más de 4 semanas?

**Respuesta:** Re-scope el milestone.

```bash
# Si Phase 7 es muy grande:
# 1. Pausar branch actual
git checkout feature/v1.1-rest-api
git push origin feature/v1.1-rest-api

# 2. Dividir en sub-milestones
# Phase 7a: Core endpoints (1-2 weeks)
# Phase 7b: Auth + security (1-2 weeks)

# 3. Crear nuevos branches
git checkout main
git checkout -b feature/v1.1a-rest-core
git checkout -b feature/v1.1b-rest-auth

# 4. Actualizar CHECKLIST con sub-phases
```

### 2. ¿Puedo trabajar en múltiples branches simultáneamente?

**Respuesta:** Sí, pero con cuidado.

```bash
# OK: Branches independientes (no comparten código)
feature/v1.1-rest-api      # Team A
feature/v1.2-sensory-zoom  # Team B

# RIESGO: Branches dependientes
feature/v1.3-mtt-dsl       # Necesita v1.2 completo
└─ Esperar merge de v1.2 primero
```

### 3. ¿Qué hago con branches antiguos?

**Respuesta:** Merge o delete.

```bash
# Si completado: merge a main
git checkout main
git merge feature/old-branch --no-ff
git push origin main

# Si abandonado: delete
git branch -D feature/old-branch
git push origin --delete feature/old-branch

# Si histórico: mantener pero documentar
# En CHECKLIST: Estado: ARCHIVED
```

### 4. ¿Cómo manejo hotfixes?

**Respuesta:** Branch separado desde main o tag.

```bash
# 1. Desde main o tag
git checkout v1.1.0
git checkout -b hotfix/v1.1.1-memory-leak

# 2. Fix rápido
git commit -m "fix(api): Hotfix v1.1.1 - Memory leak in response caching"

# 3. Merge a main
git checkout main
git merge hotfix/v1.1.1-memory-leak --no-ff

# 4. Tag patch version
git tag -a v1.1.1 -m "Hotfix v1.1.1 - Memory leak fix"

# 5. Push
git push origin main v1.1.1

# 6. Backport a feature branches si necesario
git checkout feature/v1.2-sensory-zoom
git cherry-pick <hotfix-commit>
```

### 5. ¿Cómo sincronizo CHECKLIST si olvidé actualizar?

**Respuesta:** Script de recuperación.

```bash
# 1. Get commit hashes for completed tasks
git log --oneline --grep="Task 7\."

# 2. Manually update CHECKLIST_V2.md
- [x] 7.1 - POST /biographical/entry (commit: a1b2c3d) ✅ 2025-11-29

# 3. Commit update
git commit -m "docs: Sync CHECKLIST with git history

- Added missing commit hashes for Phase 7
- Tasks 7.1-7.5 completed but not marked
- Extracted from git log"
```

### 6. ¿Qué pasa si mi branch diverge de main?

**Respuesta:** Rebase o merge main periódicamente.

```bash
# OPCIÓN 1: Rebase (historia limpia)
git checkout feature/v1.1-rest-api
git fetch origin main
git rebase origin/main

# Resolver conflictos si hay
git rebase --continue

# Push (force con lease para no pisar commits de otros)
git push --force-with-lease origin feature/v1.1-rest-api

# OPCIÓN 2: Merge (historia completa)
git checkout feature/v1.1-rest-api
git fetch origin main
git merge origin/main

# Resolver conflictos si hay
git commit -m "merge: Sync with main"

# Push
git push origin feature/v1.1-rest-api
```

### 7. ¿Cómo documento decisiones técnicas durante el milestone?

**Respuesta:** ADR (Architecture Decision Records) en commits.

```bash
# Commit con decisión técnica
git commit -m "docs: ADR - Use JWT instead of sessions for auth

Decision: JWT tokens for stateless authentication
Context: REST API needs to scale horizontally
Alternatives considered:
- Server-side sessions (requires Redis)
- OAuth2 (overkill for v1.0)
Consequences:
- Stateless API (✅ scales easily)
- Token expiry management (🔄 refresh tokens)
- No centralized session invalidation (⚠️ mitigation: short TTL)

Implements: Task 7.6"

# También documentar en ROADMAP_V2 si es crítico
```

### 8. ¿Cómo valido que mi branch está listo para merge?

**Checklist pre-merge:**

```markdown
Pre-Merge Checklist:
- [ ] Todos los tasks del Phase completados en CHECKLIST
- [ ] Todos los tests passing (unit + integration)
- [ ] Performance metrics cumplen targets
- [ ] Documentación actualizada (API, README, etc)
- [ ] No hay merge conflicts con main
- [ ] Code review aprobado (si aplica)
- [ ] CHANGELOG.md actualizado
- [ ] Branch sincronizado con main reciente
- [ ] CI/CD pipeline green
- [ ] Demo funcional preparado (opcional)
```

---

## 🎯 BENEFICIOS METODOLOGÍA v1.6

### 1. Trazabilidad Perfecta

```
Cada commit → task específica en CHECKLIST
Cada task → commit hash en git
Cada branch → milestone claro

Pregunta: "¿Qué commit implementó Task 7.3?"
Respuesta: grep "7.3" CHECKLIST_V2.md → (commit: i7j8k9l)

Pregunta: "¿Qué branch contiene REST API?"
Respuesta: grep "REST API" CHECKLIST_V2.md → Branch: feature/v1.1-rest-api
```

### 2. Sincronización Automática

```
Script valida inconsistencias:
✅ Branch name ↔ milestone
✅ Tasks [x] ↔ commit hash
✅ Commits ↔ task reference

Error detectado temprano = fix rápido
No más "olvidé actualizar checklist"
```

### 3. Branches Significativos

```
Nombre = milestone alcanzable
Scope = Phase específico
ETA = 2-4 weeks (realistic)

No más branches eternos
Merge frecuente, ciclos cortos
Histórico git legible
```

### 4. Documentación Viva

```
CHECKLIST = historia del proyecto
Git log = narrativa técnica
Ambos se complementan perfectamente

Future developers: entender proyecto en 30 min
AI agents: context completo para asistir
Auditoría: trazabilidad total
```

### 5. Escalabilidad

```
Funciona con:
- 1 desarrollador (solo) ✅
- 5 desarrolladores (equipo) ✅
- 50 desarrolladores (organización) ✅

LLMs pueden:
- Validar automáticamente
- Generar reportes
- Sugerir próximos pasos
```

---

## 🚀 PRÓXIMOS PASOS

### Implementación Inmediata

1. **✅ Crear este documento** (METODOLOGIA_V1_6_GIT_CHECKLIST.md)
2. **✅ Actualizar GUIA.md** con SECCIÓN 2.5 (Git + Checklist Workflow)
3. **✅ Actualizar CHECKLIST_V2.md** con Phase 6.5 + roadmap v1.1-v1.5
4. **🔄 Implementar validation script** (scripts/sync_checklist_git.sh)
5. **🔄 Training session** para nuevos agentes/desarrolladores

### Roadmap v1.6+

```
v1.6.0 (Current):
- Metodología documentada ✅
- CHECKLIST enhanced format ✅
- Branch naming convention ✅
- Commit template ✅

v1.6.1 (Next):
- Validation script automated ✅
- CI/CD integration (validate on PR)
- Pre-commit hooks (validate before commit)

v1.6.2 (Future):
- Dashboard web (visualizar progreso)
- Slack/Discord notifications (task complete)
- AI assistant integration (suggest next task)
```

---

## 📚 REFERENCIAS

- **CHECKLIST_V2.md** - Source of truth único
- **CHECKLIST_TREE_V2.md** - Dependencias jerárquicas
- **GUIA.md SECCIÓN 2.5** - Workflow implementación
- **Git Flow** - Inspiración original (gitflow.github.io)
- **Semantic Versioning** - semver.org
- **Conventional Commits** - conventionalcommits.org

---

**Versión:** 1.0.0  
**Fecha:** 2025-11-28  
**Estado:** 🚀 ACTIVO  
**Próxima Revisión:** v1.6.1 (validation script implementation)  

---

*"Branch name = Milestone alcanzable. Checklist = Truth. Commits = Trazabilidad."*

🎯 Metodología v1.6 - Organización suprema para cualquier modelo ❤️‍🔥
