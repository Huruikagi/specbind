# 0019: Use ordered tasks with sparse dependency exceptions

Status: Accepted

## Context

The inherited cc-sdd task plan uses document order as its conservative dependency default. `(P)` marks a reviewed exception that may run in parallel, while `_Depends:_` records non-obvious prerequisites. Requiring an AI author to replace that model with a complete dependency graph would create precise-looking but potentially incomplete scheduling data and make ordinary task generation harder to review.

Structured `tasks.yaml` still needs deterministic status and next-task calculation without treating an AI-generated graph as perfectly complete.

## Decision

- The order of plan items is semantically significant and is part of the approved plan projection.
- A top-level item depends on completion of the preceding top-level item by default.
- A group forms an execution barrier: its child tasks inherit the group's preceding top-level prerequisites.
- Child tasks within a group depend on the preceding sibling task by default.
- Executable tasks omit `parallel` by default; when present its only valid value is `true` under Decision 0023.
- `parallel: true` removes only that task's immediate implicit ordering dependency. It does not remove inherited group prerequisites or any explicit dependency.
- `depends_on` is a sparse list of additional, non-obvious task prerequisites. It is not required to restate dependencies already implied by ordering or group barriers.
- Container-only groups are not executable and do not carry `parallel` or `depends_on` fields.
- A top-level executable task may use `parallel: true`; when it must retain a prerequisite that the flag would otherwise remove, that prerequisite must appear in `depends_on`.
- The CLI derives an effective dependency graph from order, group barriers, `parallel`, and `depends_on`, then rejects missing references, self-dependencies, and cycles.
- `parallel: true` permits but never requires concurrent execution. An implementation workflow may execute every task sequentially when concurrency is unsupported or uncertain.

## Consequences

- AI authors provide only reviewed exceptions instead of reconstructing a complete DAG.
- Missing optional optimization data falls back to safe sequential execution.
- Array order and group structure cannot be changed without changing the task-plan fingerprint.
- Parallel execution remains opt-in and conservative while status tooling can still derive a deterministic set of actionable tasks.
- Cross-group or other non-obvious dependencies remain explicit and machine-checkable.

## Open questions

- Exact migration diagnostics when inherited numbering violates the positional ID rules accepted by Decision 0020.
- How migration expands ambiguous inherited `(P)` and `_Depends:_` combinations.
