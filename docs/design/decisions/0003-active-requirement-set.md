# 0003: Store the active requirement set in spec.json

Status: Accepted

## Context

`requirements.md` represents the complete currently valid requirement set. A milestone may change or revalidate only a subset of those requirements, so downstream design, tasks, and completion verification need an explicit active requirement set.

The set is current lifecycle state rather than prose or release history. It must also be machine-readable so task coverage can be checked without inferring scope from document diffs.

## Decision

Store the active requirement set in each spec's `spec.json`, inside its active-change state.

Conceptual shape before requirements approval:

```json
{
  "active_change": {
    "milestone_id": "<generated-id>",
    "change_id": "<generated-or-stable-id>",
    "requirement_ids": null
  }
}
```

After requirements approval:

```json
{
  "active_change": {
    "milestone_id": "<generated-id>",
    "change_id": "<generated-or-stable-id>",
    "requirement_ids": ["1.1", "1.2", "3.1"]
  }
}
```

After release finalization:

```json
{
  "active_change": null
}
```

`null` means the requirements phase has not established the set. An array means the set has been explicitly established and approved. The final schema may contain additional lifecycle fields, but it must preserve this distinction.

## Invariants

- Requirement IDs use the canonical IDs from `requirements.md`.
- The array contains unique IDs in deterministic order.
- Every stored ID must exist in the current `requirements.md`.
- Requirements approval writes and freezes the array.
- Design and tasks read the stored array; they do not independently infer or expand it.
- Tasks must provide machine-checkable 100% coverage of the stored array.
- Changing the array returns the workflow to requirements and invalidates affected downstream approvals.
- Release finalization clears the active change rather than retaining current-state flags as release history.

## Consequences

- `spec.json` becomes the source of truth for current milestone requirement scope.
- `requirements.md` remains the source of truth for requirement definitions.
- `changelog.md` records the released coverage summary, not current active state.
- Migration must reconstruct the active set for an in-progress milestone before the project can claim tasks coverage.
- Status and validation skills can distinguish an unestablished set from an approved set without parsing prose.

## Open schema details

- The final names and placement of other `active_change` lifecycle fields.
- Whether an approved active set may ever be empty; the default expectation is at least one Requirement ID for spec-backed work.
- The exact task-to-requirement trace representation used to prove coverage.
