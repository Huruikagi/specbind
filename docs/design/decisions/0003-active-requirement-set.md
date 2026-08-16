# 0003: Store the active requirement set in per-spec lifecycle metadata

Status: Accepted

## Context

The singleton `SpecBind Requirements` artifact represents the complete currently valid requirement set. Decision 0057 discovers it by OKF type; `requirements.md` is only the default path. Decision 0060 derives canonical Requirement IDs from an explicit mapped Requirement heading number and Acceptance Criteria list position. A milestone may change or revalidate only a subset of those requirements, so downstream design, tasks, and completion verification need an explicit active requirement set.

The set is current lifecycle state rather than prose or release history. It must also be machine-readable so task coverage can be checked without inferring scope from document diffs.

## Decision

Store the active requirement set inside each spec's active-change lifecycle metadata. [Decision 0014](./0014-structured-spec-metadata.md) defines the target serialized artifact as `spec.yaml`.

Conceptual shape before requirements approval:

```yaml
active_change:
  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
  requirement_ids: null
```

After requirements approval:

```yaml
active_change:
  milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
  requirement_ids:
    - "1.1"
    - "1.2"
    - "3.1"
```

After release finalization:

```yaml
active_change: null
```

`null` means the requirements phase has not established the set. An array means the set has been explicitly established and approved. The final schema may contain additional lifecycle fields, but it must preserve this distinction.

## Invariants

- Requirement IDs use the canonical IDs from the discovered requirements artifact.
- The array contains unique IDs in deterministic order.
- The approved array is non-empty for every spec-backed active change.
- Every stored ID must exist in the current requirements artifact.
- Requirements approval writes and freezes the array.
- Design and tasks read the stored array; they do not independently infer or expand it.
- Tasks must provide machine-checkable 100% coverage of the stored array.
- Changing the array returns the workflow to requirements and invalidates affected downstream approvals.
- Release finalization clears the active change rather than retaining current-state flags as release history.

## Consequences

- `spec.yaml` becomes the source of truth for current milestone requirement scope under Decision 0014.
- The singleton requirements artifact remains the source of truth for requirement definitions.
- The per-spec `log.md` records the released coverage summary, not current active state, under Decision 0048.
- Migration must reconstruct the active set for an in-progress milestone before the project can claim tasks coverage.
- Status and validation skills can distinguish an unestablished set from an approved set without parsing prose.

Decision 0023 resolves task-to-Requirement traceability through each executable Task's non-empty `requirement_ids` set. The Tasks gate and read model validate complete active-set coverage.

## Implementation status

The Rust traceability read model now validates every active ID against the current Requirements catalog and requires complete coverage through executable Task `requirement_ids` once the workflow reaches `tasks`. It allows `tasks.yaml` to remain absent in earlier states, validates unknown Task references whenever a valid plan is present, and does not require inactive current Requirements to appear in the milestone plan. CLI rendering and gate-transition enforcement remain subsequent work.
