#!/usr/bin/env bash
set -euo pipefail

# organize_commits.sh
# Agrupa y crea commits independientes para organizar el historial local.
# Uso:
#   ./scripts/organize_commits.sh [--dry-run] [--push]
# --dry-run: solo muestra qué se haría sin commitear
# --push: tras los commits hace 'git push -u origin master' (desactivado por defecto)

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_DIR"

DRY_RUN=0
PUSH=0

for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=1 ;;
    --push) PUSH=1 ;;
    -h|--help)
      sed -n '1,120p' "$0"
      exit 0 ;;
    *) echo "Unknown arg: $arg"; exit 1 ;;
  esac
done

echo "Repo: $REPO_DIR"
if ! git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  echo "ERROR: Not a git repository: $REPO_DIR" >&2
  exit 2
fi

git --no-pager status --porcelain

commit_if_changes() {
  local msg="$1"
  if [ $DRY_RUN -eq 1 ]; then
    if git diff --staged --quiet; then
      echo "[DRY-RUN] No staged changes for: $msg"
    else
      echo "[DRY-RUN] Would commit: $msg"
      git --no-pager diff --staged --name-status || true
    fi
    return
  fi

  if git diff --staged --quiet; then
    echo "No staged changes for: $msg"
  else
    git commit -m "$msg" && echo "Committed: $msg"
  fi
}

# Commit groups (ajusta las rutas si tu estructura difiere)

# 000-init: scaffold
echo "\n[000-init] staging scaffold files (Cargo.toml, Cargo.lock, top-level src)"
git add Cargo.toml Cargo.lock src || true
commit_if_changes "chore(init): initial project scaffold"

# 001-core: core crates and utilities
echo "\n[001-core] staging core crates..."
git add crates/bitacora-core crates/bitacora-session crates/bitacora-config || true
commit_if_changes "feat(core): domain models, timestamp and session utilities"

# 002-storage: storage crates and config
echo "\n[002-storage] staging storage crates and config..."
git add crates/bitacora-records crates/bitacora-storage config docker docker-compose.yml || true
commit_if_changes "feat(storage): mongodb schemas and repository adapters"

# 003-commands-scripts: CLI and scripts
echo "\n[003-commands-scripts] staging command crate and scripts..."
# include legacy scripts folder if present
git add crates/bitacora-commands scripts bitacora_v0/scripts || true
commit_if_changes "feat(cli/scripts): session and action scripts, command crate scaffold"

# 004-docs: docs and maps
echo "\n[004-docs] staging docs and _map..."
git add docs _map README.md || true
commit_if_changes "docs: architecture, checklist and integration notes"

# 005-fixes: remaining changes
echo "\n[005-fixes] staging everything else..."
# Stage everything left (careful): uses -A
if [ $DRY_RUN -eq 1 ]; then
  echo "[DRY-RUN] Would git add -A (stage all remaining changes)"
  git status --porcelain
else
  git add -A || true
fi
commit_if_changes "chore: misc fixes, formatting and CI setup"

# Summary
echo "\n--- Final status ---"
git --no-pager status --porcelain || true

echo "\n--- Recent commits (last 10) ---"
git --no-pager log --oneline -n 10 || true

if [ $PUSH -eq 1 ]; then
  echo "\n--- Pushing master to origin (will set upstream if needed) ---"
  git push -u origin master
fi

echo "\nDone. Review the commits and push manually if desired."
