# 0135: Treat absent Requirements as expected phase work

Status: Accepted

## Context

Discovery creates a new Spec with `spec.yaml` and a Brief, then deliberately
stops before Requirements authoring. `spec status` nevertheless folds
`TRACEABILITY_REQUIREMENTS_UNAVAILABLE` into health immediately, so the normal
entry to the Requirements phase reports `State health: inconsistent`. `milestone
status` inherits that result as `MILESTONE_SPEC_INCONSISTENT` while also routing
the same Spec to `action=requirements`.

Decision 0133 made absent Design coverage phase-relative without weakening the
strict traceability check. Requirements need the same expected-work distinction.

## Decision

When the declared state is `requirements` and its gate has not been reached,
`spec status` treats `TRACEABILITY_REQUIREMENTS_UNAVAILABLE` as expected work
rather than a health diagnostic. It reports:

```text
Expected work: author Requirements
```

Every other diagnostic remains. A malformed Requirements draft therefore stays
inconsistent through its parsing or profile diagnostics; suppressing the
redundant unavailable summary does not make damaged content healthy.

`specbind check traceability` remains strict and continues to fail until one
valid Requirements artifact exists. Approval guards are unchanged.

Because milestone health composes phase-relative Spec health, an otherwise
valid Discovery result is consistent and continues to expose the Requirements
action. No milestone-specific suppression is added.

## Consequences

- Discovery output reads as healthy incomplete work rather than damaged state.
- The status view names the missing authoring work explicitly.
- Invalid drafts, strict checks, and guarded approval retain their existing
  failure behavior.

## Implementation status

Implemented in the Spec status read model and text renderer. CLI coverage proves
the phase-relative Spec and milestone views while retaining strict traceability.
