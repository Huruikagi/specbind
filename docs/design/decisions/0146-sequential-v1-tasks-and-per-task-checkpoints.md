# 0146: Keep v1 Task execution sequential and checkpoint each completed Task

Status: Accepted

## Context

The inherited cc-sdd plan marked some Tasks `(P)`, but its hardened executor
ultimately processed every Task sequentially so that review could use the actual
shared-worktree diff and each Task could end in its own selective commit.
SpecBind carried the marker into `tasks.yaml` as `parallel: true`, made it remove
an implicit ordering dependency, and allowed `specbind-implement` to dispatch
such Tasks concurrently.

That contract has a larger safety surface than v1 needs. Concurrent Task work in
one worktree makes task-local diff ownership, independent review, remediation,
generated-output attribution, and selective commits interdependent. The marker
also creates a wire and scheduling contract even when an agent chooses not to
run concurrently.

Forward-test journey HP1 exposed the adjacent checkpoint defect: the default Git
adapter names each completed implementation Task as one workflow unit, while the
implement skill reached its checkpoint only after all requested Task outcomes
were recorded. Two completed Tasks could therefore be combined into one commit.

## Decision

### The v1 Task plan is fully ordered

- `tasks/v1` has no `parallel` field or parallel Task variant. An occurrence is
  an unknown-field schema error.
- Every top-level executable Task depends on the immediately preceding top-level
  item. Every child Task depends on its preceding sibling, and every child in a
  group inherits the preceding top-level prerequisites.
- `depends_on` remains a sparse way to add non-obvious prerequisites. It never
  removes the dependency established by plan order.
- `boundaries` remains optional task scope used for implementation and review;
  it carries no scheduling meaning.
- The task-plan fingerprint no longer has a parallel marker to normalize or
  compare.

### Implementation and acceptance are one sequential cycle

`specbind-implement` selects and finishes one actionable Task at a time in plan
order. It does not concurrently dispatch implementation Tasks into a shared
worktree. A Task completes its implementation, review, guarded progress write,
durable Implementation Notes, and checkpoint decision before the skill re-reads
the task model and selects another Task.

Only a Task recorded `completed` is an eligible implementation checkpoint. With
the installed default Git adapter, the skill creates one local commit for that
Task containing only:

- the deliberate implementation and test paths produced for the Task;
- the Task's CLI-owned `tasks.yaml` execution-state transition; and
- Implementation Notes created or revised from that Task's durable finding.

The checkpoint stays inside the per-Task cycle and is never deferred until all
requested Tasks finish. Another Task, unrelated work, Spec completion metadata,
rejected work, and partial implementation are excluded. Project-owned adapter
guidance may opt out or choose another grouping, but the skill still reads and
resolves that policy after each completed Task rather than silently batching the
default units.

A blocked Task stops the sequential run. It is not a completed-Task checkpoint,
and partial implementation is never committed to manufacture a clean handoff.
Spec-level completion validation and its metadata checkpoint remain separate.

## Stabilization and migration

This removes a prerelease v1 wire field before stable `1.0.0`, where Decision
0144's same-major compatibility promise begins. A prerelease `tasks.yaml` that
contains `parallel` must be regenerated and reapproved; the field is not ignored
because silently changing its dependency semantics would preserve the bytes but
not the plan.

An inherited cc-sdd `(P)` marker is not carried into `tasks.yaml`. Guided
migration preserves the Task's position, text, provable progress, boundaries,
and explicit dependencies, while the target plan uses conservative sequential
ordering.

## Consequences

- Task readiness, review, remediation, and Git ownership have one deterministic
  order in v1.
- A later Task cannot bypass a blocked earlier Task merely because their work
  appears independent.
- Projects give up Task-level implementation concurrency in exchange for a
  smaller and mechanically testable workflow contract.
- Planning across Specs, read-only research dispatches, and independent
  validation may still run in parallel where their owning Decisions allow it.
- A focused implementation forward test must prove that two requested Tasks
  become two default local commits with one progress transition in each.

## Superseded clauses

This Decision supersedes only the Task-level parallel clauses of Decisions
0013, 0018, 0019, 0023, 0080, 0093, 0094, 0105, and 0110. It clarifies the
implementation checkpoint timing required by Decision 0137. Other uses of
parallel authoring, research, validation, and cross-Spec orchestration remain
unchanged.
