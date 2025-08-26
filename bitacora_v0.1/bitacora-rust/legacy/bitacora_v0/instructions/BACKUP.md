# BACKUP

## Objective
Create a complete backup of the current project and register this action in the active record.

## Steps
1) Read timestamp from `.bitacora/timestamp/timestamp.txt`.
2) Execute backup script: `.bitacora/scripts/backup_project.sh`
3) Register backup ACTION in active record: `YYYYMMDD-hhmm_backup: Project backup completed - [backup details]`
4) Display backup completion status.

## Success criteria
- Project backup successfully created
- Backup action registered in active record
- Backup location/details provided to user

## Notes
- This command can be run standalone or automatically as part of END command
- Backup details should include location and timestamp for reference
