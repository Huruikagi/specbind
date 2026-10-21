# 0230: CLI-owned Task plan revision and execution reconciliation

Status: Accepted

## Decision

Replace the manual execution-key editing exception in Decisions 0105 and 0206
with a guarded revision operation. The agent authors the replacement plan and
judges whether prior obligations and evidence still apply; the CLI copies,
resets or removes existing execution records. It never accepts caller-authored
completed/blocked records or blocker reasons.

`specbind tasks revision preflight <spec>` reads the existing valid Tasks
document and reports a source fingerprint and its current plan/execution as
JSON. `specbind tasks revision apply <spec> --input <file|-> [--dry-run]`
accepts strict JSON described by `task-revision/v1`: `schemaVersion: 1`,
`sourceFingerprint`, a replacement `plan` in the existing Tasks v1 shape, and
`progress`, an array of explicit dispositions. Each disposition names `from`,
`action` (`preserve`, `reset`, `remove`), and a nonblank `reason`; preserve/reset
also require `to`. Remove forbids `to`.

Every existing execution entry must have exactly one disposition. Source IDs
must have recorded execution, target IDs must be executable Tasks in the new
plan, and no two dispositions may target the same Task. Preserve copies the
complete existing record unchanged, including a blocked reason. Reset/remove
copy no execution. New Tasks start pending. Splits/merges cannot duplicate or
combine old execution: reset/remove affected records and prove new work afresh.
Preserve remains an agent assertion about unchanged obligations and applicable
evidence, not a semantic conclusion the CLI can establish from matching IDs.

Both operations require an active delivery participant in `tasks` state with
no Tasks/completion evidence, fresh Requirements and Design gates, and fresh
Contract Review. A stale retained plan is read as repair input; its old active
coverage is not required. The proposed replacement must pass Tasks schema,
positional/dependency semantics and complete current traceability before any
write. The CLI does not invalidate or approve gates, commit, or change scope.

The fingerprint binds the exact Tasks bytes (including execution), target
identity, Spec state bytes and active Roadmap bytes. Apply re-resolves all
prerequisites and source immediately before persistence and rejects a changed
fingerprint. Dry-run performs the same checks and reports the explicit mapping
and resulting plan without writing. Apply atomically replaces only the existing
regular `tasks.yaml`. As with other guarded operations, this is not a filesystem
lock against arbitrary simultaneous writers. Failed validation leaves it intact.
Input files must be outside the project worktree, or supplied on stdin.

The Tasks Skill uses this route for every existing-plan revision, including a
plan with no execution. First authoring without an existing plan remains direct
file authoring with no execution. Confirmation of the exact reconciliation is
still required unless supplied Drive replan authority covers it. Gate approval
is separate. Missing, malformed or link-like retained input is a stop, not
permission to replace it by hand. No durable revision ledger or new tasks.yaml
wire version is introduced.

## Verification

CLI tests cover preserve/reset/remove, renumbering and blocked reasons, empty
execution, stale sources, incomplete/duplicate/invalid mappings, injected
execution, lifecycle/review/traceability failures, dry-run and byte preservation
on failure. Schema generation and conformance cover the transient wire shape.
Skill tests and fresh Tasks-revision forward tests verify that the agent uses
the guarded operation, preserves approval boundaries and never hand-edits
execution state.
