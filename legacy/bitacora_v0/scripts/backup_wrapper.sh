#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

echo "💾 Starting project backup..."

# Get current timestamp and branch
ts="$(read_timestamp)"
branch="$(git_current_branch)"
recfile="$(find_active_record_for_branch "$branch")"

# Execute the existing backup script
echo "📦 Running backup process..."
if [ -f ".bitacora/scripts/backup_project.sh" ] && [ -x ".bitacora/scripts/backup_project.sh" ]; then
    backup_result=$(.bitacora/scripts/backup_project.sh 2>&1)
    backup_exit_code=$?
    
    if [ $backup_exit_code -eq 0 ]; then
        # Extract backup filename from output (assumes last line contains filename)
        backup_file=$(echo "$backup_result" | grep -o "[0-9]\{8\}-[0-9]\{4\}_.*\.zip" | tail -n1 || echo "backup completed")
        
        echo "✅ Backup completed successfully"
        echo "$backup_result"
        
        # Register successful backup
        if [ -n "$recfile" ] && [ -f "$recfile" ]; then
            echo "${ts}_backup: Project backup completed - ${backup_file}" >> "$recfile"
            echo "" >> "$recfile"  # Add empty line for better Markdown readability
            echo "📝 Backup action registered in $(basename "$recfile")"
        else
            echo "⚠️  No active record found to register backup action"
        fi
        
        echo "💾 Backup process completed successfully!"
        
    else
        echo "❌ Backup failed with exit code: $backup_exit_code"
        echo "$backup_result"
        
        # Register failed backup
        if [ -n "$recfile" ] && [ -f "$recfile" ]; then
            echo "${ts}_backup: Project backup FAILED - exit code ${backup_exit_code}" >> "$recfile"
            echo "" >> "$recfile"  # Add empty line for better Markdown readability
            echo "📝 Backup failure registered in $(basename "$recfile")"
        fi
        
        exit $backup_exit_code
    fi
    
else
    echo "❌ Backup script not found or not executable: .bitacora/scripts/backup_project.sh"
    
    # Register missing script
    if [ -n "$recfile" ] && [ -f "$recfile" ]; then
        echo "${ts}_backup: Backup FAILED - script not found" >> "$recfile"
        echo "" >> "$recfile"  # Add empty line for better Markdown readability
        echo "📝 Backup failure registered in $(basename "$recfile")"
    fi
    
    exit 1
fi
