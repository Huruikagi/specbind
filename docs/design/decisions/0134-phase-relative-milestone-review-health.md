# 0134: Treat an absent milestone review as expected workflow work

Status: Accepted

## Context

`milestone status` derives both the current delivery stage and aggregate health.
For every Spec-backed milestone it evaluates the contract review immediately and
adds `CONTRACT_REVIEW_MISSING` to diagnostics when no accepted review exists.
That makes a newly created milestone report `State health: inconsistent` during
Requirements and Design, even though the review cannot run until every
participating Spec has current Design approval.

The same view still reports an actionable Requirements or Design item. Absence
therefore means expected future work, while `inconsistent` suggests damaged
state. Decision 0107 already keeps an absent review from affecting per-Spec
health, and Decision 0133 separates expected phase work from Spec inconsistency.
The milestone projection needs the same distinction.

## Decision

An absent contract review does not contribute `CONTRACT_REVIEW_MISSING` to
`milestone status` diagnostics or health at any delivery stage.

The projection continues to report:

- `Contract review: absent`;
- `contract_review` as actionable once every participating Spec has current
  Design approval; and
- `CONTRACT_REVIEW_NOT_FRESH` as a release blocker.

Every guard remains unchanged. Tasks approval, implementation validation, and
release still require a fresh accepted review.

This suppression applies only to absence. A stale or invalid accepted review is
evidence that recorded state disagrees with its authoritative inputs and remains
a milestone inconsistency with diagnostics. Current Tasks authored before review
acceptance also remain inconsistent through
`MILESTONE_TASKS_BEFORE_REVIEW`; removing the generic missing-review diagnostic
does not hide that concrete ordering violation.

## Consequences

- Requirements, Design, and the contract-review entry state report healthy
  incomplete work as `consistent`.
- The review field, actionable list, and release blockers still expose exactly
  what remains to do.
- A malformed or stale accepted review and premature Tasks remain distinguishable
  from an ordinary review that has not been authored yet.

## Implementation status

Implemented in the milestone read model. Review issues are folded into aggregate
diagnostics for stale and invalid states, while the missing state is represented
only by review freshness, actions, and release readiness.
