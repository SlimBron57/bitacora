#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

BITADIR="$PROJECT_DIR/.bitacora"
TS_FILE="$BITADIR/timestamp/timestamp.txt"
RECORDS_DIR="$BITADIR/records"
TOPICS_DIR="$BITADIR/topics"
TOPICS_ARCHIVE_DIR="$TOPICS_DIR/archive"

# Crear directorios solo si no existen
[ ! -d "$RECORDS_DIR" ] && mkdir -p "$RECORDS_DIR"
[ ! -d "$TOPICS_DIR" ] && mkdir -p "$TOPICS_DIR"
[ ! -d "$TOPICS_ARCHIVE_DIR" ] && mkdir -p "$TOPICS_ARCHIVE_DIR"

die() { echo "ERROR: $*" >&2; exit 1; }

read_timestamp() {
  [ -f "$TS_FILE" ] || die "No existe $TS_FILE. Inicia el daemon: .bitacora/setup/timestampctl start"
  local ts
  ts="$(tail -n1 "$TS_FILE" | tr -d '\r\n')"
  [[ "$ts" =~ ^[0-9]{8}-[0-9]{4}$ ]] || die "Formato inválido de timestamp: $ts"
  echo "$ts"
}

slugify() {
  echo "$1" | tr '[:upper:]' '[:lower:]' | sed -E 's/[^a-z0-9]+/-/g; s/^-+|-+$//g'
}

git_current_branch() {
  if git -C "$PROJECT_DIR" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git -C "$PROJECT_DIR" rev-parse --abbrev-ref HEAD
  else
    echo "no-git"
  fi
}

# Extract description part from branch name (removes timestamp prefix)
extract_branch_description() {
  local branch="$1"
  if [[ "$branch" =~ ^[0-9]{8}-[0-9]{4}_ ]]; then
    # Remove timestamp prefix (YYYYMMDD-HHMM_)
    echo "${branch#*_}"
  else
    # If no timestamp prefix, return as is
    echo "$branch"
  fi
}

find_active_record_for_branch() {
  local branch="$1"
  local files
  files=$(ls -1t "$RECORDS_DIR"/*_"$branch".md 2>/dev/null || true)
  if [ -n "$files" ]; then
    echo "$files" | head -n1
  else
    echo ""
  fi
}

find_current_session_record() {
  local branch="$1"
  local branch_desc="$(extract_branch_description "$branch")"
  local current_ts="$(read_timestamp)"
  local current_date="${current_ts%-*}"  # Extract YYYYMMDD part
  
  # Look for record with current date
  local current_record="$RECORDS_DIR/${current_ts}_${branch_desc}.md"
  if [ -f "$current_record" ]; then
    echo "$current_record"
    return
  fi
  
  # If no exact match, look for any record from today
  local today_records
  today_records=$(ls -1t "$RECORDS_DIR/${current_date}-"*"_${branch_desc}.md" 2>/dev/null || true)
  if [ -n "$today_records" ]; then
    echo "$today_records" | head -n1
  else
    echo ""
  fi
}

extract_topic_description() {
  local topic_file="$1"
  if grep -q '^##[[:space:]]\+Idea' "$topic_file"; then
    awk '
      /^##[[:space:]]+Idea/ {in_idea=1; next}
      /^##[[:space:]]+/ {if (in_idea) exit}
      { if (in_idea) print }
    ' "$topic_file"
  else
    awk 'NR>1 {print}' "$topic_file"
  fi
}

# Git automation functions
auto_commit_if_enabled() {
  local commit_msg="$1"
  
  # Check if auto-commit is disabled
  if [ "${BITACORA_NO_AUTO_COMMIT:-}" = "1" ]; then
    return 0
  fi
  
  # Only proceed if we're in a git repository and not on main/master
  local branch="$(git_current_branch)"
  if [ "$branch" = "no-git" ] || [ "$branch" = "main" ] || [ "$branch" = "master" ]; then
    return 0
  fi
  
  # Check if there are changes to commit
  if git -C "$PROJECT_DIR" diff --quiet && git -C "$PROJECT_DIR" diff --staged --quiet; then
    echo "   (no changes to commit)"
    return 0
  fi
  
  echo "   🔄 Auto-committing changes..."
  git -C "$PROJECT_DIR" add . 2>/dev/null || true
  if git -C "$PROJECT_DIR" commit -m "$commit_msg" >/dev/null 2>&1; then
    echo "   ✅ Auto-committed: $commit_msg"
    
    # Smart push: only push every 10 actions to save GitHub API quota
    local push_counter_file="$BITADIR/cache/push_counter.txt"
    local counter=1
    
    # Read existing counter
    if [ -f "$push_counter_file" ]; then
      counter=$(cat "$push_counter_file" 2>/dev/null || echo "1")
      counter=$((counter + 1))
    fi
    
    # Write updated counter
    mkdir -p "$(dirname "$push_counter_file")"
    echo "$counter" > "$push_counter_file"
    
    # Push every 10 actions or if counter reset
    if [ $((counter % 10)) -eq 0 ] || [ "$counter" -eq 1 ]; then
      echo "   🚀 Auto-pushing branch to remote... (action $counter)"
      if git -C "$PROJECT_DIR" push origin "$branch" >/dev/null 2>&1; then
        echo "   ✅ Auto-pushed to origin/$branch"
      else
        echo "   ⚠️  Push failed or no upstream configured"
      fi
    else
      local next_push=$((10 - (counter % 10)))
      echo "   📝 Commit saved locally (will push in $next_push more actions)"
    fi
  else
    echo "   ⚠️  Auto-commit failed (no changes or conflict)"
  fi
}

# Force push any pending commits (useful before END session)
force_push_pending_commits() {
  local branch="$(git_current_branch)"
  if [ "$branch" = "no-git" ] || [ "$branch" = "main" ] || [ "$branch" = "master" ]; then
    return 0
  fi
  
  # Check if there are unpushed commits
  if git -C "$PROJECT_DIR" log origin/"$branch"..HEAD --oneline 2>/dev/null | grep -q .; then
    echo "   🚀 Force-pushing pending commits..."
    if git -C "$PROJECT_DIR" push origin "$branch" >/dev/null 2>&1; then
      echo "   ✅ All commits pushed to origin/$branch"
      # Reset counter after force push
      local push_counter_file="$BITADIR/cache/push_counter.txt"
      echo "0" > "$push_counter_file"
    else
      echo "   ⚠️  Push failed"
    fi
  else
    echo "   ✅ All commits already pushed"
  fi
}

detect_git_remote_type() {
  local remote_url
  remote_url="$(git -C "$PROJECT_DIR" remote get-url origin 2>/dev/null || echo "")"
  
  if [[ "$remote_url" == *"github.com"* ]]; then
    echo "github"
  elif [[ "$remote_url" == *"gitlab.com"* ]] || [[ "$remote_url" == *"gitlab"* ]]; then
    echo "gitlab"
  else
    echo "unknown"
  fi
}

# Check if gh is authenticated
gh_authenticated() {
  if command -v gh >/dev/null 2>&1; then
    if gh auth status >/dev/null 2>&1; then
      return 0
    fi
  fi
  return 1
}

extract_repo_info() {
  local remote_url
  remote_url="$(git -C "$PROJECT_DIR" remote get-url origin 2>/dev/null || echo "")"
  
  if [[ "$remote_url" =~ github\.com[:/]([^/]+)/([^/]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]%.git}"
  elif [[ "$remote_url" =~ gitlab\.com[:/]([^/]+)/([^/]+)(\.git)?$ ]]; then
    echo "${BASH_REMATCH[1]}/${BASH_REMATCH[2]%.git}"
  else
    echo ""
  fi
}

auto_create_pr_if_enabled() {
  local branch="$1"
  local pr_title="$2"
  local pr_body="$3"
  
  # Check if auto-PR is disabled
  if [ "${BITACORA_NO_AUTO_PR:-}" = "1" ]; then
    echo "   💡 Auto-PR disabled. Manually create PR for branch: $branch"
    return 0
  fi
  
  # Skip if on main/master or no-git
  if [ "$branch" = "no-git" ] || [ "$branch" = "main" ] || [ "$branch" = "master" ]; then
    return 0
  fi
  
  local git_type="$(detect_git_remote_type)"
  local repo_info="$(extract_repo_info)"
  
  if [ -z "$repo_info" ]; then
    echo "   ⚠️  Could not extract repository info for auto-PR"
    echo "   💡 Manually create PR for branch: $branch"
    return 0
  fi
  
  echo "   🔄 Auto-creating pull request (draft by default, no merges)..."
  
  # Push branch before attempting PR (optional, can be disabled)
  if [ "${BITACORA_NO_AUTO_PUSH:-}" != "1" ]; then
    echo "   ⬆️  Pushing branch to remote: $branch"
    git -C "$PROJECT_DIR" push origin "$branch" >/dev/null 2>&1 || echo "   ⚠️  Push failed or no upstream configured"
  else
    echo "   ℹ️  Auto-push disabled by BITACORA_NO_AUTO_PUSH=1"
  fi
  
  case "$git_type" in
    "github")
      if command -v gh >/dev/null 2>&1; then
        # Create draft PR by default unless BITACORA_PR_DRAFT=0
        if [ "${BITACORA_PR_DRAFT:-1}" != "0" ]; then
          pr_flags=(--draft)
        else
          pr_flags=()
        fi

        if gh pr create --title "$pr_title" --body "$pr_body" --head "$branch" "${pr_flags[@]}" 2>/dev/null; then
          echo "   ✅ GitHub PR created successfully (draft: ${BITACORA_PR_DRAFT:-1})"
          echo "   ℹ️  Note: bitácora will never perform merges; PRs are for visibility and review only."
        else
          echo "   ⚠️  GitHub PR creation failed"
          echo "   💡 Manually run: gh pr create --title \"$pr_title\" --head \"$branch\""
        fi
      else
        echo "   ⚠️  GitHub CLI (gh) not installed"
        echo "   💡 Install with: sudo apt install gh"
      fi
      ;;
    "gitlab")
      if command -v glab >/dev/null 2>&1; then
        if glab mr create --title "$pr_title" --description "$pr_body" --source-branch "$branch" 2>/dev/null; then
          echo "   ✅ GitLab MR created successfully"
        else
          echo "   ⚠️  GitLab MR creation failed"
          echo "   💡 Manually run: glab mr create --title \"$pr_title\" --source-branch \"$branch\""
        fi
      else
        echo "   ⚠️  GitLab CLI (glab) not installed"
        echo "   💡 Install with: sudo apt install glab"
      fi
      ;;
    *)
      echo "   ⚠️  Unknown git remote type"
      echo "   💡 Manually create PR for branch: $branch"
      ;;
  esac
}

generate_pr_body_from_record() {
  local record_file="$1"
  local branch="$2"
  
  if [ ! -f "$record_file" ]; then
    echo "Work completed on branch: $branch"
    return
  fi
  
  # Extract description
  local description=""
  if grep -q "^## DESCRIPTION" "$record_file"; then
    description="$(awk '/^## DESCRIPTION/,/^## /{if(!/^## DESCRIPTION/ && !/^## [A-Z]/) print}' "$record_file" | sed '/^[[:space:]]*$/d')"
  elif grep -q "^### Descripcion" "$record_file"; then
    description="$(awk '/^### Descripcion/,/^### /{if(!/^### Descripcion/ && !/^### /) print}' "$record_file" | sed '/^[[:space:]]*$/d')"
  fi
  
  # Count completed checklist items
  local total_checks="$(grep -c '- \[.\]' "$record_file" 2>/dev/null || echo "0")"
  local done_checks="$(grep -c '- \[x\]' "$record_file" 2>/dev/null || echo "0")"
  
  # Count actions
  local total_actions="$(grep -c -E '^[0-9]{8}-[0-9]{4}_' "$record_file" 2>/dev/null || echo "0")"
  
  # Build PR body
  cat <<EOF
## Summary
$(echo "$description" | head -3)

## Progress
- ✅ Completed: $done_checks/$total_checks checklist items
- 📝 Actions recorded: $total_actions
- 🔧 Branch: $branch

## Recent Actions
$(grep -E '^[0-9]{8}-[0-9]{4}_' "$record_file" 2>/dev/null | tail -5 | sed 's/^/- /' || echo "- No actions recorded")

---
*Auto-generated by bitácora system*
EOF
}
