# TOPIC

## Objective
Create a file in `.bitacora/topics/` with name `TOPIC_<slug>.md` to capture ideas outside the current flow without touching the active record.

## Steps
1) Read timestamp from `.bitacora/timestamp/timestamp.txt`.
2) Title slug: lowercase; `[^a-z0-9]+` → `-`; trim `-`.
3) Template:

TOPIC <Title>

Created: YYYYMMDD-hhmm

Active branch: <branch-or-no-git>

Active record: <record-file-or-N/A>

Idea
<free-content>

Quick notes
