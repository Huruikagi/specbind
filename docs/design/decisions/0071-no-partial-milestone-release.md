# 0071: Do not represent a partially released milestone

Status: Accepted

## Context

A project adapter may publish to multiple systems, and some external work may succeed before a later Prepare, Publish, or Verify step fails. Core finalization also updates several participating specs and milestone-owned artifacts. Persisting a subset of those specs as released would make roadmap, contract-review, and release-history ownership ambiguous.

## Decision

### Failure before core finalization

- If any applicable project Prepare, Publish, or Verify work fails or remains uncertain, the release skill does not invoke core finalization.
- The milestone and every participating spec remain active. SpecBind does not append release log entries, archive the roadmap or applicable contract review, remove active Brief, Research, or task artifacts, or clear an `active_change`.
- SpecBind does not automatically roll back an external publication that already succeeded. The agent reports the observed partial result and works with the human to inspect actual project-system state, then retry, reconcile manually, or explicitly abandon the unreleased milestone under Decision 0005.
- Decision 0070 applies: partial external progress is not persisted as universal SpecBind release evidence. A project-owned system or adapter-directed artifact may retain it.
- A retry must interpret the adapter against current external state and must not blindly repeat a potentially non-idempotent external operation.

### Core finalization

- `specbind release finalize` has no participating-spec subset option. It always targets the complete spec set and milestone-owned artifacts resolved from the active roadmap.
- Core finalization is one logical all-or-nothing transition. Validation failures occur before mutation; an ordinary failure must not intentionally leave some participating specs released and others active.
- The implementation must use a recoverable mutation plan and must report success only after the complete released-and-idle invariant is verified. Exact crash-consistency and repair mechanics remain an implementation concern, but no partial state is a valid successful outcome.
- Any failed attempt remains retryable. Idempotent log and archive rules prevent a retry from duplicating changes already written by an interrupted attempt.

### Failure after core finalization

- Applicable After finalize instructions run only after the complete core transition succeeds.
- An After finalize failure does not reopen the milestone, remove release history, or roll back participating specs. The release remains finalized and the agent reports the failed work as a follow-up that may be retried independently.

## Consequences

- SpecBind exposes only active or fully finalized milestone lifecycle outcomes, never a supported partially released state.
- External partial success remains visible where it occurred without corrupting SpecBind's internal release boundary.
- Release adapters should make retry and external-state inspection guidance explicit when their operations are not naturally idempotent.
- Core finalization tests must cover failure injection between mutation steps and safe idempotent retry.
