#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

ts="$(read_timestamp)"
branch="$(git_current_branch)"
recfile="$(find_current_session_record "$branch")"

echo "Timestamp: $ts"
echo "Rama actual: $branch"

if [ -n "$recfile" ]; then
  echo "Record activo: $(basename "$recfile")"
  count=$(grep -E '^[0-9]{8}-[0-9]{4}_[^:]+' "$recfile" | wc -l | tr -d ' ')
  echo "Acciones registradas: $count"
  echo "Últimas acciones:"
  grep -E '^[0-9]{8}-[0-9]{4}_[^:]+' "$recfile" | tail -n 3 || true
else
  echo "Record activo: N/A"
fi

last_topic=$(ls -1t "$TOPICS_DIR"/TOPIC_*.md 2>/dev/null | head -n1 || true)
if [ -n "$last_topic" ]; then
  echo "Último TOPIC: $(basename "$last_topic")"
else
  echo "Último TOPIC: N/A"
fi
