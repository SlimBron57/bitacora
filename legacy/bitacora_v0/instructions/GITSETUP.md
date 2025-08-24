# GITSETUP Command

## Purpose
Configure and verify Git integration for automatic commits and pull request creation in the bitácora system.

## Usage
```bash
GITSETUP
```

## What It Does
1. **Detects Git Remote**: Identifies if using GitHub, GitLab, or other
2. **Checks CLI Tools**: Verifies if `gh` (GitHub CLI) or `glab` (GitLab CLI) are installed
3. **Validates Authentication**: Confirms CLI tools are properly authenticated
4. **Shows Configuration**: Displays current auto-commit and auto-PR settings
5. **Provides Setup Instructions**: Guides for missing dependencies or authentication

## Auto-Commit Feature
- **Trigger**: Every `ACTION` command automatically commits changes
- **Commit Message**: "ACTION: [action-name] - [description]"
- **Scope**: Adds all changes in project (`git add .`)
- **Safety**: Only commits if there are actual changes

## Auto-PR Feature  
- **Trigger**: `END` command automatically creates pull request
- **Title**: Extracted from record description or uses branch name
- **Body**: Generated summary with checklist progress and recent actions
- **Target**: Creates PR from current branch to main/master

## Configuration Options

### Disable Auto-Commit
```bash
export BITACORA_NO_AUTO_COMMIT=1
```

### Disable Auto-PR
```bash
export BITACORA_NO_AUTO_PR=1
```

### Disable Both
```bash
export BITACORA_NO_AUTO_COMMIT=1 BITACORA_NO_AUTO_PR=1
```

### Disable Auto-Push (don't push branches automatically)
```bash
export BITACORA_NO_AUTO_PUSH=1
```

### Control PR Draft Behavior
By default bitácora will create PRs as drafts to avoid accidental merges. To create non-draft PRs set:
```bash
export BITACORA_PR_DRAFT=0
```

Make permanent by adding to `~/.bashrc` or `~/.zshrc`.

## Requirements

### For GitHub Repositories
- Install GitHub CLI: `sudo apt install gh`
- Authenticate: `gh auth login`

### For GitLab Repositories
- Install GitLab CLI: `sudo apt install glab`
- Authenticate: `glab auth login`

## Safety Features
- **Branch Protection**: Won't commit/PR on main/master branches
- **Change Detection**: Only commits when there are actual changes
- **Graceful Fallback**: Shows manual commands if auto-creation fails
- **User Control**: Can be disabled with environment variables

## Integration with Workflow
- Run `GITSETUP` once after implementing bitácora system
- Use `ACTION` commands normally - commits happen automatically
- Use `END` to finish work - PR created automatically
- All existing bitácora functionality remains unchanged
