# 0133: Separate expected phase work from Spec inconsistency

Status: Accepted

## Context

`spec status` combines lifecycle state, gate freshness, traceability, task
progress, and diagnostics. Its current health rule is absolute: any diagnostic
makes the Spec `inconsistent`. A Spec that has just entered Design therefore
reports one `TRACEABILITY_DESIGN_COVERAGE_MISSING` diagnostic per active
Requirement even when no Design has been authored yet.

That output describes expected work as damage. The bundled Design skill has to
explain that the reported inconsistency is the normal starting state. The same
status view reports `Next actionable: none` and `Blockers: none`; both fields
are actually Task-only projections, but their labels appear workflow-wide.

## Decision

`spec status` becomes phase-relative without weakening any gate or standalone
check.

When all of the following hold, missing Design coverage is expected work rather
than a health diagnostic:

- the declared state is `design`;
- the Requirements gate is fresh;
- the Design gate has not been reached.

The status model removes only `TRACEABILITY_DESIGN_COVERAGE_MISSING` from its
diagnostic and health calculation in that condition and reports one aggregate
line:

```text
Expected work: cover <n> active requirement(s) in Design
```

Malformed artifacts, stale gates, semantic contradictions, invalid
traceability mappings, and every other diagnostic still make health
`inconsistent`. `specbind check traceability` is unchanged and continues to
report every uncovered active Requirement until the Design is complete.

The status view adds `Next action`, a workflow-level route derived from the
declared state and prerequisite freshness. Its stable vocabulary is:
`none`, `requirements`, `design`, `contract_review`, `tasks`,
`implementation`, and `release`.

The Task projection labels become explicit:

- `Next actionable` becomes `Next task` in `spec status` only;
- `Blockers` becomes `Task blockers` in `spec status` only.

Task mutation commands retain their existing `Next actionable` result field.

## Consequences

- A newly entered, untouched Design phase reports `Health: consistent`, names
  Design as the next action, and summarizes the coverage still to author.
- A reader no longer interprets absence of a task plan as absence of workflow
  work.
- Strict traceability and approval behavior remain unchanged.
- The text status contract changes labels, so exact-output consumers must move
  to the Task-qualified names.
