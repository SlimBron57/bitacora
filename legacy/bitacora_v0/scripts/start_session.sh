#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_bitacora.sh"

echo "🚀 Starting bitácora work session..."

# Ensure timestamp daemon is running
echo "📋 Checking timestamp daemon..."

# Check if PID file exists and process is actually running
PID_FILE=".bitacora/timestamp/bitacora_timestamp.pid"
if [ -f "$PID_FILE" ] && kill -0 "$(cat "$PID_FILE")" 2>/dev/null; then
    # Additional check: verify timestamp is recent (less than 120 seconds old)
    current_time=$(date +%s)
    if [ -f ".bitacora/timestamp/timestamp.txt" ]; then
        last_timestamp=$(cat ".bitacora/timestamp/timestamp.txt")
        if [ -n "$last_timestamp" ]; then
            timestamp_time=$(date -d "${last_timestamp:0:4}-${last_timestamp:4:2}-${last_timestamp:6:2} ${last_timestamp:9:2}:${last_timestamp:11:2}" +%s 2>/dev/null || echo "0")
            if [ "$timestamp_time" != "0" ]; then
                age=$((current_time - timestamp_time))
                if [ "$age" -lt 120 ]; then
                    echo "✅ Timestamp daemon already active and working"
                else
                    echo "⚠️  Timestamp daemon appears stale, restarting..."
                    ".bitacora/setup/timestampctl" restart
                fi
            else
                echo "⚠️  Invalid timestamp format, restarting daemon..."
                ".bitacora/setup/timestampctl" restart
            fi
        else
            echo "⚠️  Empty timestamp file, restarting daemon..."
            ".bitacora/setup/timestampctl" restart
        fi
    else
        echo "⚠️  No timestamp file found, restarting daemon..."
        ".bitacora/setup/timestampctl" restart
    fi
else
    echo "Starting timestamp daemon..."
    ".bitacora/setup/timestampctl" start
fi

# Get current timestamp and branch
ts="$(read_timestamp)"
branch="$(git_current_branch)"
branch_desc="$(extract_branch_description "$branch")"

# Validate branch name - should have descriptive content
if [ "$branch" = "main" ] || [ "$branch" = "master" ] || [ "$branch" = "development" ] || [ "$branch" = "dev" ]; then
    echo "❌ Cannot create bitácora records on main/master branch: $branch"
    echo "   Please switch to a feature branch with descriptive name first."
    echo "   Example: git checkout -b feature_name"
    echo "   Example: git checkout -b YYYYMMDD-HHMM_feature_description"
    exit 1
fi

if [ -z "$branch_desc" ] || [ "$branch_desc" = "no-git" ]; then
    echo "❌ Invalid branch for bitácora records: $branch"
    echo "   Please ensure you're on a descriptive feature branch."
    exit 1
fi

echo "📋 Working on branch: $branch → Record naming: $branch_desc"

# Look for the most recent record for this branch (any record, not just current session)
last_record="$(find_active_record_for_branch "$branch_desc")"

if [ -n "$last_record" ] && [ -f "$last_record" ]; then
    # Check session closure status
    if grep -q "_session-end: Work session completed" "$last_record"; then
        echo "✅ Previous session was completed: $(basename "$last_record")"
        echo "🔄 Creating new branch for fresh work session..."
        
        # Create new branch with timestamp and similar description
        new_branch="${ts}_${branch_desc}"
        if git -C "$PROJECT_DIR" show-ref --verify --quiet "refs/heads/$new_branch"; then
            echo "⚠️  Branch $new_branch already exists, switching to it"
            git -C "$PROJECT_DIR" switch "$new_branch"
        else
            git -C "$PROJECT_DIR" switch -c "$new_branch"
            echo "✅ Created new branch: $new_branch"
        fi
        
        # Update branch and branch_desc for new branch
        branch="$new_branch"
        branch_desc="$(extract_branch_description "$branch")"
        
        # Create new record with current timestamp 
        recfile="$RECORDS_DIR/${ts}_${branch_desc}.md"
        cat >"$recfile" <<EOF
# Record ${ts}_${branch_desc}

## DESCRIPTION
(New session initialized on ${ts} - previous session was completed)

## CHECKLIST
- [ ] Define session objectives
- [ ] Complete planned tasks
- [ ] Test and validate changes
- [ ] Document important findings

## ACTIONS
EOF
        echo "📝 Created new record: $(basename "$recfile")"
        
    elif grep -q "_session-end: Work session pending" "$last_record"; then
        echo "⏳ Previous session was marked as pending: $(basename "$last_record")"
        echo "   Work was left in progress to continue later."
        echo ""
        echo "Options:"
        echo "   1) Continue in existing record (resume pending work)"
        echo "   2) Start new record (abandon pending work and start fresh)"
        echo ""
        read -p "Choose option (1/2): " choice
        
        case "$choice" in
            1)
                echo "📝 Continuing in existing record: $(basename "$last_record")"
                recfile="$last_record"
                ;;
            2)
                echo "📝 Starting fresh, marking previous as abandoned..."
                # Create new record
                recfile="$RECORDS_DIR/${ts}_${branch_desc}.md"
                cat >"$recfile" <<EOF
# Record ${ts}_${branch_desc}

## DESCRIPTION
(New session initialized on ${ts} - previous pending work was abandoned)

## CHECKLIST
- [ ] Define session objectives
- [ ] Complete planned tasks
- [ ] Test and validate changes
- [ ] Document important findings

## ACTIONS
EOF
                echo "📝 Created new record: $(basename "$recfile")"
                ;;
            *)
                echo "❌ Invalid choice. Continuing in existing record."
                recfile="$last_record"
                ;;
        esac
        
    else
        echo "⚠️  Previous session was not properly closed: $(basename "$last_record")"
        echo "   Session ended abruptly without END command."
        echo "   Continuing in the last record to preserve work."
        recfile="$last_record"
        echo "📝 Continuing in existing record: $(basename "$recfile")"
    fi
else
    # No previous record found, create new one for this branch
    recfile="$RECORDS_DIR/${ts}_${branch_desc}.md"
    cat >"$recfile" <<EOF
# Record ${ts}_${branch_desc}

## DESCRIPTION
(First session initialized on ${ts})

## CHECKLIST
- [ ] Define session objectives
- [ ] Complete planned tasks
- [ ] Test and validate changes
- [ ] Document important findings

## ACTIONS
EOF
    echo "📝 Created first record for branch: $(basename "$recfile")"
fi

# Register session start
echo "${ts}_session-start: Work session initialized" >> "$recfile"
echo "" >> "$recfile"  # Add empty line for better Markdown readability
echo "✅ Session start registered in $(basename "$recfile")"

# Show current status
echo ""
echo "📊 Current Project Status:"
".bitacora/scripts/status.sh"

echo ""
echo "🎯 Bitácora session ready! Use ACTION, TOPIC, BRANCH commands as needed."
