#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

if [ $# -lt 1 ]; then
  echo "Uso: $0 <titulo_del_topic> [contenido_inicial]"
  exit 1
fi

title="$1"
shift
content="${*:-}"

ts="$(read_timestamp)"
branch="$(git_current_branch)"
active_record="$(find_active_record_for_branch "$branch")"
recbase="$( [ -n "$active_record" ] && basename "$active_record" || echo 'N/A' )"
slug="$(slugify "$title")"
file="$TOPICS_DIR/TOPIC_${slug}.md"

if [ -f "$file" ]; then
  echo "Ya existe: $file"
  exit 0
fi

cat >"$file" <<EOF
# TOPIC ${title}

- **Creado:** ${ts}
- **Rama activa:** ${branch}
- **Record activo:** ${recbase}

## Idea
${content}

## Notas rápidas
- 
EOF

echo "Topic creado: $(basename "$file")"
