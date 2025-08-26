# Bitácora System - Developer Workflow

## Overview

Bitácora is a project-aware development coordination system that automatically tracks work sessions, manages branch workflows, and maintains chronological records of development activities. Think of it as your intelligent development assistant that never forgets what you did or when you did it.

## Core Concepts

### Records & Actions
Every branch has an **active record** (`.bitacora/records/YYYYMMDD-HHMM_<branch>.md`) containing:
- **DESCRIPTION**: What you're working on
- **CHECKLIST**: Task breakdown and success criteria  
- **ACTIONS**: Chronological log of everything you do

### Automatic Timestamping
A background daemon updates `.bitacora/timestamp/timestamp.txt` every minute with the current timestamp (`YYYYMMDD-HHMM` format). All commands read this file for consistent, synchronized time tracking across your development workflow.

### Command-Driven Workflow
Instead of remembering to document your work, you use simple semantic commands that automatically maintain your development history.

## Essential Commands

### Starting Your Work Session
```bash
START
```
- Initializes timestamp daemon
- Creates/updates active record for current branch
- Shows current project status
- Prepares complete development environment

### Managing Development Flow
```bash
BRANCH feature-auth — Implement user authentication system
```
Creates new Git branch and corresponding record with description and checklist.

```bash
ACTION database "Set up PostgreSQL connection - successful"
ACTION tests "Added user auth tests - 3 passing, 1 failing"  
ACTION deploy "Production deployment - failed, investigating"
```
Each action is timestamped and logged in your branch record, creating a complete development history. **NEW**: Actions now automatically commit changes with descriptive messages.

```bash
TOPIC "Performance optimization" "Database queries taking too long in user dashboard"
```
Captures ideas or issues without disrupting current workflow. Topics can be converted to branches later.

### Monitoring Progress
```bash
STATUS
```
Shows current branch, active record, recent actions, and project overview.

### Branch Management from Topics
```bash
BRANCH TOPIC_performance-optimization.md
```
Converts existing topic into new branch with description from topic content.

### Session Management
```bash
BACKUP
```
Creates complete project backup and registers action.

```bash
GITSETUP
```
Configures Git integration for automatic commits and pull requests.

```bash
END  
```
Registers session completion, creates backup, stops services, **automatically creates pull request**, suggests next steps.

## Typical Development Session

1. **Start**: `START` - System ready, daemon running
2. **Setup**: `GITSETUP` - Configure Git integration (one-time)
3. **Work**: `BRANCH new-feature — Add payment processing`
4. **Progress**: Multiple `ACTION` commands as you work (auto-commits)
5. **Ideas**: `TOPIC` commands for side thoughts
6. **Monitor**: `STATUS` to check progress
7. **Finish**: `END` - Clean session close with backup and auto-PR

## File Structure

```
.bitacora/
├── commands/
│   └── index.yml              # Command routing
├── instructions/
│   ├── START.md               # Command definitions
│   ├── BRANCH.md
│   ├── ACTION.md
│   ├── TOPIC.md
│   ├── STATUS.md
│   ├── BACKUP.md
│   └── END.md
├── scripts/
│   ├── lib_bitacora.sh        # Common functions
│   ├── start_session.sh       # Command implementations
│   ├── branch_create.sh
│   ├── action_add.sh
│   ├── topic_create.sh
│   ├── status.sh
│   ├── backup_wrapper.sh
│   └── end_session.sh
├── records/
│   └── YYYYMMDD-HHMM_<branch>.md  # Work records
├── topics/
│   ├── TOPIC_<slug>.md        # Active topics
│   └── archive/               # Completed topics
├── setup/
│   └── timestampctl           # Daemon control
└── timestamp/
    ├── timestamp.txt          # Current timestamp
    ├── bitacora_timestamp.pid # Process ID
    └── bitacora_timestamp.log # Service logs
```

## Benefits

### For Solo Development
- **Never lose context**: Complete chronological history of every decision and action
- **Resume faster**: STATUS command shows exactly where you left off
- **Track progress**: Visual completion through actions and checklists
- **Automatic backup**: Every session ends with project backup
- **Seamless Git integration**: Auto-commits and pull requests without workflow interruption

### For Team Development  
- **Transparent history**: Anyone can see exactly what happened in each branch
- **Consistent documentation**: Standardized format across all developers
- **Handoff clarity**: Clear records for code review and collaboration
- **Decision tracking**: Why and when decisions were made
- **Automatic PRs**: Consistent pull request creation with detailed context

### For Project Management
- **Progress visibility**: Real-time view of development activity
- **Time tracking**: Automatic timestamping of all activities
- **Issue correlation**: Topics and actions linked to specific timeframes
- **Milestone tracking**: Checklist completion and action histories

## Integration Notes

### GitHub Copilot
Bitácora integrates with GitHub Copilot through `.github/copilot-instructions.md`. Simply use the commands in natural language and Copilot executes the appropriate workflows.

### Git Workflow
Works alongside your existing Git practices. Bitácora records complement commit messages with context about the thinking process and development decisions.

### IDE Agnostic
Command-line based system works with any editor or IDE. No plugins required.

## Advanced Usage

### Multi-Project Workflows
Each project maintains independent bitácora systems. Timestamp daemons are project-specific, allowing simultaneous work across multiple repositories.

### Custom Actions
Action descriptions should be meaningful and indicate success/failure states. Examples:
- `ACTION refactor "Extracted user service - tests passing"`
- `ACTION bugfix "Fixed memory leak in audio processor - verified"`
- `ACTION research "Evaluated three OAuth libraries - selected passport.js"`

### Topic Management
Topics serve as a "parking lot" for ideas that emerge during focused work:
- Capture without context switching
- Convert to branches when ready
- Archive completed investigations
- Reference in future planning

## Best Practices

1. **Start every session**: `START` ensures consistent environment
2. **Frequent actions**: Log significant activities as they happen, not at the end
3. **Meaningful descriptions**: Future you will thank current you for clarity
4. **End cleanly**: `END` ensures proper backup and service cleanup
5. **Status checks**: Use `STATUS` when returning to work or context switching

The bitácora system transforms development from ad-hoc activities into a structured, trackable, and reproducible process while maintaining the flexibility and speed you need for creative problem-solving.
