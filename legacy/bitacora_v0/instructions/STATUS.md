# STATUS

## Objective
Provide a quick project status:
- Current timestamp (last read)
- Current branch
- Active record and number of actions
- Last 3 actions
- Last modified TOPIC

## Steps
1) Read timestamp.
2) Determine current branch.
3) Identify active record and count lines matching `^[0-9]{8}-[0-9]{4}_[^:]+:`.
4) Show last 3 (if they exist).
5) Show last file in `.bitacora/topics/` (not `archive/`), if it exists.
