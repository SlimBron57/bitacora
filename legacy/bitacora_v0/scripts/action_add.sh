#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

if [ $# -lt 2 ]; then
  echo "Uso: $0 <accion> <descripcion breve>"
  exit 1
fi

accion="$1"
shift
desc="$*"

ts="$(read_timestamp)"
branch="$(git_current_branch)"
branch_desc="$(extract_branch_description "$branch")"

# Validate branch name - should have descriptive content
if [ "$branch" = "main" ] || [ "$branch" = "master" ] || [ "$branch" = "development" ] || [ "$branch" = "dev" ]; then
    echo "❌ Cannot add actions on main/master branch: $branch"
    echo "   Please switch to a feature branch with descriptive name first."
    exit 1
fi

if [ -z "$branch_desc" ] || [ "$branch_desc" = "no-git" ]; then
    echo "❌ Invalid branch for bitácora actions: $branch"
    echo "   Please ensure you're on a descriptive feature branch."
    exit 1
fi

recfile="$(find_current_session_record "$branch")"

# Critical validation: Check if the current active record is completed
# Only prevent actions if the CURRENT record is completed, not other records with same description
if [ -n "$recfile" ] && [ -f "$recfile" ]; then
    if grep -q "_session-end: Work session completed" "$recfile"; then
        echo "❌ CRITICAL: Current session record is already completed!"
        echo "   Completed Record: $(basename "$recfile")"
        echo "   Current Branch: $branch"
        echo "   Cannot add actions to completed current session."
        echo "   Please run START command to begin a new session."
        exit 1
    fi
fi

if [ -z "$recfile" ]; then
  # Crear record nuevo si no existía
  recfile="$RECORDS_DIR/${ts}_${branch_desc}.md"
  cat >"$recfile" <<EOF
# Record ${ts}_${branch_desc}

## Seccion 1
### Descripcion
(Registro inicial automático para rama ${branch_desc}.)

### Checklist
- [ ] Definir criterios de éxito
- [ ] Implementar tareas clave
- [ ] Probar y validar
- [ ] Documentar cambios

---

## Seccion 2 (Acciones importantes)
# Formato: YYYYMMDD-hhmm_<accion>: <descripcion breve y si fue exitoso o no>
EOF
fi

echo "${ts}_${accion}: ${desc}" >> "$recfile"
echo "" >> "$recfile"  # Add empty line for better Markdown readability
echo "✅ Action registered in $(basename "$recfile"): ${accion}"

# Auto-commit the changes
auto_commit_if_enabled "ACTION: ${accion} - ${desc}"
