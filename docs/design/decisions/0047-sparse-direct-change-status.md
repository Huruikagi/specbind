# 0047: Persist only completed direct-change status

Status: Accepted

## Context

Spec-backed roadmap items derive progress from `spec.yaml`, `tasks.yaml`, and accepted gate evidence. Direct changes intentionally have no spec-local lifecycle artifact, but release preflight still needs one durable indication that each direct item was performed.

The inherited roadmap represented this distinction with an unchecked or checked one-line item. Reproducing a multi-state workflow, timestamps, revisions, or a second evidence model would make direct work disproportionate to its purpose.

## Decision

- Only items in `work_items.direct_changes` may carry a `status` property.
- The only persisted value is `status: completed`.
- Absence of `status` means pending. `status: pending` and boolean checkbox equivalents are invalid.
- `in_progress`, `blocked`, `skipped`, and other direct-change states are not persisted in v1.
- A completed direct item stores no `completed_at`, implementation revision, approver, or evidence object.
- The CLI owns the mechanical roadmap mutation that adds or removes `status: completed`; the agent workflow owns the judgment that the direct change has been performed.
- Reopening a direct item removes `status` rather than writing another value.
- Release preflight requires every direct-change item in the active roadmap to have `status: completed`.
- The roadmap's overall progress or readiness remains derived. It is not copied into a top-level `status` field.

## Consequences

Pending:

```yaml
direct_changes:
  - id: update-ci
    summary: Add the new validation command to CI
```

Completed:

```yaml
direct_changes:
  - id: update-ci
    summary: Add the new validation command to CI
    status: completed
```

- The persisted information is equivalent to the inherited unchecked/checked roadmap item without retaining Markdown checkbox parsing.
- Status rendering combines derived spec-backed progress with this sparse direct-change state.
- If a direct change later needs blocked reasons, revision-bound validation, or richer evidence, discovery should first reconsider whether it still belongs on the direct route.
