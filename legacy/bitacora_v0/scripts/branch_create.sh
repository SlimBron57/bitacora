#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

# Uso:
#   branch_create.sh "<nombre-rama>" "<descripcion>"
#   branch_create.sh "TOPIC_<nombre-topic>.md"

[ $# -ge 1 ] || { echo 'Uso: branch_create.sh "<nombre-rama>" "<descripcion>" | branch_create.sh "TOPIC_<topic>.md"'; exit 1; }

ts="$(read_timestamp)"

current_branch="$(git_current_branch)"
active_record=""
if [ "$current_branch" != "no-git" ]; then
  active_record="$(find_active_record_for_branch "$current_branch")"
fi

arg1="$1"

if [[ "$arg1" == TOPIC_*.md ]]; then
  topic_file="$TOPICS_DIR/$arg1"
  [ -f "$topic_file" ] || die "No existe topic: $topic_file"
  
  topic_name="${arg1#TOPIC_}"
  topic_name="${topic_name%.md}"
  branch_name="$(slugify "$topic_name")"
  description="$(extract_topic_description "$topic_file")"
  use_topic="1"
else
  [ $# -ge 2 ] || { echo 'Falta descripcion. Uso: branch_create.sh "<nombre-rama>" "<descripcion>"'; exit 1; }
  branch_name="$(slugify "$arg1")"
  shift
  description="$*"
  use_topic="0"
fi

if [ -n "$active_record" ] && [ -f "$active_record" ]; then
  echo "${ts}_branch-change: cambio de branch a ${ts}_${branch_name}" >> "$active_record"
fi

if [ "$current_branch" = "no-git" ]; then
  die "No estás en un repositorio Git."
fi

if git -C "$PROJECT_DIR" show-ref --verify --quiet "refs/heads/$branch_name"; then
  git -C "$PROJECT_DIR" switch "$branch_name"
else
  git -C "$PROJECT_DIR" switch -c "$branch_name"
fi

record_file="$RECORDS_DIR/${ts}_${branch_name}.md"
if [ ! -f "$record_file" ]; then
  cat >"$record_file" <<EOF
# Record ${ts}_${branch_name}

## Seccion 1
### Descripcion
$(echo "$description" | sed 's/\r$//')

### Checklist
- [ ] Definir criterios de éxito
- [ ] Implementar tareas clave
- [ ] Probar y validar
- [ ] Documentar cambios

---

## Seccion 2 (Acciones importantes)
EOF
  echo "Creado record: $(basename "$record_file")"
else
  echo "Record ya existe: $(basename "$record_file")"
fi

if [ "$use_topic" = "1" ]; then
  if [ ! -d "$TOPICS_ARCHIVE_DIR" ]; then
    mkdir -p "$TOPICS_ARCHIVE_DIR"
  fi
  mv -f "$topic_file" "$TOPICS_ARCHIVE_DIR/"
  echo "Topic archivado: $TOPICS_ARCHIVE_DIR/$(basename "$topic_file")"
fi

echo "OK: branch actual -> $branch_name"
echo "Record nuevo -> $(basename "$record_file")"
