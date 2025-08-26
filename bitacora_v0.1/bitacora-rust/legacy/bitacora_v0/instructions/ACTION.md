# ACTION

## Objective
Add a chronological entry in the **active record** of the current branch under the "## ACTIONS" section with the format:
`YYYYMMDD-hhmm_<action>: <description>`

This creates a work log of all significant activities performed during development on the current branch.

## Steps
1) Read timestamp from `.bitacora/timestamp/timestamp.txt`.
2) Determine current branch (`git rev-parse --abbrev-ref HEAD`); if no repo, use `no-git`.
3) Active record: most recent file in `.bitacora/records/` whose name ends with `_<branch>.md`.
   - If it doesn't exist, create a new one: `YYYYMMDD-hhmm_<branch>.md` with base template.
4) Add a line exactly under "## ACTIONS" section with:
   `YYYYMMDD-hhmm_<action>: <description>`
   (1 line; concise; indicate whether it was successful or not).
