# 0095: Expose task execution progress commands

Status: Accepted

## Context

[Decision 0024](./0024-sparse-task-execution-state.md) fixes the persisted task
execution state: a sparse map holding only `completed` and `blocked` entries,
where an absent key means pending and a blocked entry carries a non-empty
reason. [Decision 0025](./0025-task-read-model.md) makes the CLI projections the
primary reading surface, and the Spec state machine records
`TASK_PROGRESS_RECORDED` as a non-transitioning event.

No public command writes it. Every other mutation of a structured artifact is
CLI-owned, but `specbind-implement` currently has no way to record that a task
finished or is stuck, which leaves hand-editing `tasks.yaml` as the only option.
That defeats the purpose of a structured artifact: a hand edit can renumber keys,
write an unsupported status, contradict the plan, or silently corrupt the
document the whole implementation phase reads.

## Decision

### Commands and ownership

The accepted commands are:

```text
specbind tasks complete <spec> <task-id>
specbind tasks block <spec> <task-id> --reason <reason>
specbind tasks reopen <spec> <task-id>
```

- They live in the `tasks` namespace beside the accepted `tasks list` and
  `tasks show` projections, because they read and write the same task plan. The
  `spec <gate>` namespace stays reserved for lifecycle gate transitions; task
  progress crosses no gate.
- `<spec>` is one canonical Spec identity and `<task-id>` is one executable Task
  ID from the current plan. Neither is inferred from the working directory.
- These commands mutate only `execution.tasks`. They never change the plan, the
  declared state, or any gate evidence.
- Each invocation records exactly one task. There is no bulk form, no range
  form, and no `in_progress` status, which Decision 0024 keeps in the run
  context rather than the artifact.
- A group identifier is rejected rather than expanded to its children.

### One task per invocation

Recording completion is a judgment, not bookkeeping. Decision 0021 makes
completion criteria a per-task question, and the implementer answers it for one
task at a time. A command that accepted several identifiers would let one
invocation record several judgments that were never separately made.

Two further properties depend on it. The prerequisite guard stays meaningful
only while each record is evaluated against the state that actually exists;
resolving a batch in one call would require treating earlier members as complete
mid-call, which diverges from the actionable set the caller read beforehand.
The result contract also stays decidable: a batch mixing already-recorded and
newly recorded tasks has no honest single outcome.

This is a CLI-surface rule, not a limit on how work is requested. A user may ask
a skill to implement a whole group or a set of tasks, and the implementation
skill owns expanding that request into the ordered sequence the plan implies and
invoking this command once per task, stopping where the plan says to stop. That
placement is the ordinary orchestration boundary: convenience belongs to the
skill, and one guarded record per judgment belongs to the CLI.

### Effects

- `complete` writes `status: completed` for the task.
- `block` writes `status: blocked` with the supplied non-empty single-line
  reason. Supplying a new reason for an already-blocked task replaces it.
- `reopen` removes the task's entry, returning it to pending, and removes the
  `execution` container when it becomes empty.

Groups never receive an execution entry under Decision 0024, and group progress
is derived under Decision 0025, so a group identifier resolves to nothing this
command can write.

### Guards

Every command requires a structurally and semantically valid `spec.yaml` and
`tasks.yaml`, an active change in the `implementation` state, and a `<task-id>`
that resolves to an executable task in the current plan.

The `implementation` requirement is what keeps the artifact honest at both ends:
before Tasks approval the plan is not yet the accepted contract, and at
`release_ready` the accepted completion evidence already asserts that every task
finished. Progress in either state would contradict a gate, so it is refused and
the caller uses the applicable explicit transition first.

`complete` additionally requires every effective prerequisite to be complete.

This guard is a safety net, not the primary mechanism. Actionability is already
answerable before any work begins: `tasks show` reports whether a task is
`pending actionable` or `pending waiting` along with its effective
prerequisites, `tasks list` marks every task the same way, and `spec status`
reports the next actionable set. The implementation workflow selects work from
that read model, so a task that reaches `complete` should already have been
actionable when it started.

The guard exists because the two are separated in time. Between selecting a task
and finishing it, an earlier task can be reopened or blocked, or the plan can be
revised. Refusing at that point reports a real divergence instead of recording a
completion the derived model cannot explain.

Both use the same derivation — a pending task whose effective prerequisites are
all complete — so the advisory read and the enforcing write can never disagree.
Decision 0019 makes execution order the primary dependency mechanism, so
completing a task ahead of its prerequisites contradicts the approved plan.
There is no force flag; when the order is genuinely wrong, the plan is revised
through the ordinary Tasks rewind and re-approval.

`block` requires the task not to be already complete. Recording a completed task
as blocked is a contradiction, and reopening it first states the intent
explicitly.

### Repository safety

These commands apply Decision 0081 path safety to `tasks.yaml` and replace it
atomically. They require no clean worktree and no commit.

Implementation is the one phase whose worktree is expected to be dirty, and a
task is normally completed while its own code changes are uncommitted. A
cleanliness guard here would make the command unusable exactly when it is
needed. The revision-bound guarantees belong to the Decision 0086 completion
handshake, which independently rechecks the clean revision before accepting
completion evidence.

### The gates stay fresh

Recording progress must never stale an approved gate. Decision 0024 and Decision
0028 exclude execution state from the normalized task-plan projection, so the
tasks-gate fingerprint is unaffected by these commands. This is a property to
preserve rather than to re-derive: a change that let execution state influence
the plan fingerprint would make ordinary implementation work invalidate its own
approval.

### Results

A successful mutation reports the task and the derived progress so the caller
can see where the plan now stands:

```text
OK TASK_COMPLETED: Completed task 1.2 in spec checkout.
  Progress: 2/4 completed, 1 pending, 1 blocked
  Next actionable: 1.3
```

- `block` returns `OK TASK_BLOCKED` and includes the recorded reason;
  `reopen` returns `OK TASK_REOPENED`.
- An already-recorded identical state returns `NO_CHANGE TASK_ALREADY_COMPLETED`
  or `NO_CHANGE TASK_ALREADY_BLOCKED`. Reopening a task with no persisted entry
  returns `NO_CHANGE TASK_NOT_RECORDED`.
- Every guard failure returns `ERROR TASK_COMPLETE_FAILED`, `ERROR
  TASK_BLOCK_FAILED`, or `ERROR TASK_REOPEN_FAILED`, exits nonzero, emits the
  underlying stable diagnostics, and leaves `tasks.yaml` unchanged.

### Agent boundary

Completing a task means the implementer judged the work done, including any
Decision 0021 completion criteria. The CLI validates identity, state, and
prerequisites; it does not and cannot attest that the criteria are satisfied.
`specbind-implement` and `specbind-review-task` own that judgment, and the
Decision 0086 handshake owns the Spec-level validation that follows.

`specbind-implement` also owns the request surface. It accepts a group, a set of
tasks, or a general instruction to continue, resolves that against the current
plan and actionable set, and records each task individually as it is judged
complete. Reporting partial progress when the sequence stops early is part of
that skill's contract; the CLI reports only the single record it made.

## Consequences

- `specbind-implement` can record progress through a public command, so the
  structured task artifact is never hand-edited.
- The derived actionable set, blockers, and milestone progress stay trustworthy
  because every recorded state passed the plan's own prerequisite rules.
- Implementation work does not invalidate the Tasks gate it was approved under.
- Task selection stays a read-model decision made before work starts, while the
  write path only refuses the cases where the plan changed underneath it.
- Out-of-order completion becomes a visible plan revision instead of a silent
  divergence between the plan and what happened.
- Blocked work carries a reason in the artifact, so status and milestone
  reporting can explain a stall without consulting the conversation.

## Implementation status

Not implemented. The task plan loader, derived read model, prerequisite
computation, and guarded atomic writes already exist. Clap routing, the three
guarded mutations, concise rendering, stable exit behavior, and CLI integration
tests remain to be implemented.
