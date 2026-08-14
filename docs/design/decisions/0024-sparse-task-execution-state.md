# 0024: Persist only completed and blocked task states

Status: Accepted

## Context

The inherited cc-sdd artifact adds progress over time by changing checkboxes and appending `_Blocked:_` annotations. A structured execution model should preserve those durable facts without requiring an entry for every pending task or leaving stale `in_progress` markers after an agent run is interrupted.

Tasks may also be added during an active milestone. Treating absence as pending lets a guarded plan revision add new tasks without generating redundant execution records.

## Decision

- `execution` is omitted while no durable execution state exists.
- `execution.tasks` is a sparse map keyed by executable Task ID.
- An executable task absent from the map is `pending`.
- The only persisted status values are `completed` and `blocked`.
- `in_progress` belongs to the active workflow run context and is never persisted in `tasks.yaml`.
- `skipped` is not supported; Decision 0022 requires every task remaining in the active plan to complete.
- A `completed` entry contains only `status: completed`.
- A `blocked` entry requires a non-empty `blocked_reason`.
- `blocked_reason` is forbidden for completed entries.
- Returning a blocked task to pending removes its map entry rather than writing `status: pending`.
- Groups never receive execution entries.
- Execution map keys must resolve to current executable tasks. Plan mutations add, remove, or renumber execution keys atomically where applicable.
- `completed_at`, `blocked_at`, commit hashes, and review transcripts are not stored in v1 task execution state. Git history and gate evidence retain their respective audit responsibilities.
- Execution state is excluded from the task-plan fingerprint under Decision 0018. Persistent `SpecBind Implementation Notes` artifacts accepted by Decisions 0026 and 0057 are separate and are not task-gate inputs.

## Consequences

- A newly added task is pending without an execution-file update.
- Interrupted workflows do not leave a persistent task claiming to be in progress.
- Status output derives pending tasks by subtracting completed and blocked IDs from the executable plan.
- Completion requires every executable task to have a completed entry and no blocked entries.
- JSON Schema validates state shape and Task ID syntax; Rust semantic validation resolves keys against executable plan tasks.
