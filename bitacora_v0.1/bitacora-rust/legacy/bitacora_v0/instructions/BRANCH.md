# BRANCH

## Objective
- Register a final ACTION in the **active record** of the current branch indicating the branch change.
- Create/switch to a **new Git branch**.
- Create a **new record**: `.bitacora/records/YYYYMMDD-hhmm_<branch-name>.md`
  - ## DESCRIPTION: from the prompt or, if coming from TOPIC, from the TOPIC content.
  - ## CHECKLIST: initial and actionable.
  - ## ACTIONS: chronological log where all ACTION commands will be recorded during work on this branch.
- If coming from `TOPIC_<name>.md`, move the topic to `.bitacora/topics/archive/`.

## Rules
- Timestamp: read last line from `.bitacora/timestamp/timestamp.txt` (format `YYYYMMDD-hhmm`).
- Active record (previous branch): the most recent file in `.bitacora/records/` whose name ends with `_<previous-branch>.md`. If it doesn't exist, don't write previous ACTION.
- Branch slug: lowercase; `[^a-z0-9]+` → `-`; trim `-`.

## Formats
- Branch change ACTION (in previous branch active record):
  `YYYYMMDD-hhmm_branch-change: branch change to YYYYMMDD-hhmm_<branch-name>`
- New record:
  - Name: `YYYYMMDD-hhmm_<branch-name>.md`
  - Base content:
    ```
    # Record YYYYMMDD-hhmm_<branch-name>

    ## DESCRIPTION
    <description>

    ## CHECKLIST
    - [ ] Define success criteria
    - [ ] Implement key tasks
    - [ ] Test and validate
    - [ ] Document changes

    ## ACTIONS
    ```

## Invocation variants
1) `BRANCH <branch-name> — <description>`
   - Use `<description>` as Description.
2) `BRANCH TOPIC_<topic-name>.md`
   - Description = `## Idea` section from topic (if doesn't exist, use file body).
   - Branch name = slug of `<topic-name>`.
   - Move to `topics/archive/`.
