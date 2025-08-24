# END

## Objective
Properly close a work session by registering final actions, creating summary, and stopping all bitácora services.

## Steps
1) Read timestamp from `.bitacora/timestamp/timestamp.txt`.
2) Register session end ACTION in active record: `YYYYMMDD-hhmm_session-end: Work session completed - [brief summary of work done]`
3) Display final STATUS to show session summary.
4) Stop timestamp daemon with `.bitacora/setup/timestampctl stop`.
5) Suggest creating pull request if on feature branch (not main/master).
6) Archive any completed topics if applicable.
7) Clean up temporary files if any.

## Success criteria
- Final session action recorded
- Timestamp daemon properly stopped
- Developer provided with session summary
- Suggestions for next steps given
- All services cleanly terminated

## Notes
- The session summary should be concise but meaningful
- If significant work was done, recommend PR creation
- Ensure clean state for next session start
