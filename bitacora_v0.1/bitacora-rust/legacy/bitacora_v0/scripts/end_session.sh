#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

echo "🏁 Ending bitácora work session..."

# Get current timestamp and branch
ts="$(read_timestamp)"
branch="$(git_current_branch)"
recfile="$(find_current_session_record "$branch")"

if [ -n "$recfile" ] && [ -f "$recfile" ]; then
    # Count actions in this session (today)
    today=$(echo "$ts" | cut -d'-' -f1)
    session_actions=$(grep -E "^${today}-[0-9]{4}_" "$recfile" | wc -l)
    
    # Ask user about session completion status
    echo ""
    echo "📋 How do you want to close this session?"
    echo "   1) Completed - All planned work finished"
    echo "   2) Pending - Work in progress, will continue later"
    echo ""
    read -p "Choose option (1/2): " completion_choice
    
    case "$completion_choice" in
        1)
            summary="Work session completed"
            status_marker="completed"
            ;;
        2)
            summary="Work session pending"
            status_marker="pending"
            ;;
        *)
            echo "❌ Invalid choice. Defaulting to 'pending' status."
            summary="Work session pending"
            status_marker="pending"
            ;;
    esac
    
    # Register session end with chosen status
    echo "${ts}_session-end: ${summary}" >> "$recfile"
    echo "" >> "$recfile"  # Add empty line for better Markdown readability
    echo "✅ Session end registered: $summary (${session_actions} actions recorded)"
else
    echo "⚠️  No active record found for current branch"
fi

# Show final status
echo ""
echo "📊 Final Session Status:"
".bitacora/scripts/status.sh"

# Execute backup
echo ""
echo "💾 Creating project backup..."
if ".bitacora/scripts/backup_wrapper.sh"; then
    echo "✅ Backup completed as part of session end"
else
    echo "⚠️  Backup failed, but continuing with session end"
fi

# Stop timestamp daemon
echo ""
echo "🛑 Stopping timestamp daemon..."
".bitacora/setup/timestampctl" stop

# Auto-create PR if on feature branch
echo ""
if [ "$branch" != "main" ] && [ "$branch" != "master" ] && [ "$branch" != "no-git" ]; then
    echo "🚀 Creating pull request for branch: $branch (will be created as draft by default; no merges performed)"
    
    # Extract PR title from record description
    pr_title="$branch"
    if [ -n "$recfile" ] && [ -f "$recfile" ]; then
        # Try to get a better title from description
        if grep -q "^## DESCRIPTION" "$recfile"; then
            pr_title="$(awk '/^## DESCRIPTION/,/^## /{if(!/^## DESCRIPTION/ && !/^## [A-Z]/) print}' "$recfile" | sed '/^[[:space:]]*$/d' | head -1 | sed 's/^[[:space:]]*//')"
        elif grep -q "^### Descripcion" "$recfile"; then
            pr_title="$(awk '/^### Descripcion/,/^### /{if(!/^### Descripcion/ && !/^### /) print}' "$recfile" | sed '/^[[:space:]]*$/d' | head -1 | sed 's/^[[:space:]]*//')"
        fi
        
        # If title is empty or too short, use branch name
        if [ -z "$pr_title" ] || [ ${#pr_title} -lt 10 ]; then
            pr_title="$branch"
        fi
        
        # Generate PR body from record
        pr_body="$(generate_pr_body_from_record "$recfile" "$branch")"
        
    # Ensure all commits are pushed before creating PR
    force_push_pending_commits
        
    # Create the PR (draft by default)
    auto_create_pr_if_enabled "$branch" "$pr_title" "$pr_body"
    else
        echo "   ⚠️  No record found, creating basic PR"
        # Ensure all commits are pushed before creating PR
        force_push_pending_commits
        auto_create_pr_if_enabled "$branch" "$branch" "Work completed on branch: $branch"
    fi
else
    echo "💡 Next Steps Suggestions:"
fi

echo "   • Review completed work in record: $(basename "$recfile" 2>/dev/null || echo "N/A")"
echo "   • Use START command when resuming work"

echo ""
echo "✨ Session ended successfully!"
