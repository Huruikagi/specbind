# 0200: Preserve Task blockers in Milestone status

Status: Accepted

## Context

`milestone status --json` is the authoritative scheduler for Drive, while
`sb-status` uses the same Milestone projection for an unscoped status request.
The projection previously reduced an implementing Spec to its declared state
and omitted Task progress and recorded blocker reasons. It could therefore
report an implementation item as actionable even when that Spec had no
actionable Task.

Recording a Task blocker also modifies `tasks.yaml`. The repository-wide dirty
checkout then made every completed Spec-backed predecessor appear incomplete,
so a later blocked Spec could regress to waiting on already checkpointed work.
The resulting view reported `WORKTREE_NOT_CLEAN`, no actionable entries, and no
Task reason. Neither Drive nor Status could explain the actual stop from their
authoritative input.

Decision 0095 already requires milestone reporting to explain a stall from the
recorded blocker without consulting conversation history. The earlier
Milestone design statement that blocked details remain exclusively Spec-local
does not satisfy that scheduler contract.

## Decision

Each Spec-backed item in `milestone status` includes its current Task progress,
next actionable Task IDs, and recorded Task blockers with exact Task IDs and
reasons. Text and JSON expose the same information. Direct items retain their
existing shape.

A Spec-backed implementation action exists only when its Task read model has at
least one actionable Task. A blocked Spec with no actionable Task is not routed
back to implementation. `currentBlockers` includes `TASKS_BLOCKED` whenever a
participating Spec records a Task blocker; independent actionable Milestone
items remain visible.

Dirty state does not erase the current Git `HEAD` revision. Spec-backed
dependency completion is preserved when the Spec's complete Task execution
state is already checkpointed in `HEAD`, even if another item later modifies
its own `tasks.yaml`. An uncheckpointed final Task completion still requires a
clean checkpoint before it can satisfy a Roadmap dependency or enter
Validation. `WORKTREE_NOT_CLEAN` remains phase-relative under Decision 0136 and
is reported only when cleanliness itself would unlock progress.

The CLI projects recorded facts only. It does not infer how to satisfy a
free-form blocker reason, reopen a Task, or grant Git authority. `sb-status`
turns the projection into a read-only handoff that states completed progress,
the blocked Task and reason, independent work that can continue, and the
condition that must be resolved before the owning workflow resumes.

Decision 0158's same-major additive JSON compatibility rule applies to the new
per-item fields.

## Consequences

- Drive can distinguish a genuine Task stop from an implementation action and
  from worktree cleanliness.
- An unscoped Status report can explain the same stop without reading
  `tasks.yaml` directly or depending on prior conversation.
- Completed dependency checkpoints no longer regress merely because a later
  Spec records a blocker.
- Resolving the blocker remains Agent or user judgment; the deterministic CLI
  does not manufacture external inputs or authorization.

## Verification

CLI integration coverage records a Task blocker in a dirty checkout and proves
that Milestone text and JSON retain the revision, Task counts, ID, reason, and
`TASKS_BLOCKED`, with no false implementation action or
`WORKTREE_NOT_CLEAN`. Skill contract coverage proves that named-Spec and
Milestone reporting use separate procedures and that Milestone reporting
preserves the blocked-work handoff.
