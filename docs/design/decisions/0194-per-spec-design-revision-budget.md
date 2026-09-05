# 0194: Bound Design remediation per Spec and finding history

Status: Accepted

Supersedes the bounded Design-revision clause retained by
[Decision 0161](./0161-default-plan-and-phase-skill-namespace.md) from
[Decision 0153](./0153-unified-quick-plan-orchestrator.md) and
[Decision 0120](./0120-quick-and-batch-orchestration-contracts.md).

## Context

Plan sends an unapproved Design through independent validation before Design
approval. The retained orchestration contract permits one Design-owned revision
after `NOT_READY`, then stops when fresh validation is negative again.

That limit prevents unattended rewriting when one objection survives, but it
also stops when the first blocking finding was resolved and the fresh validator
discovers a materially different blocker. The workflow needs to distinguish
semantic continuation from new information without giving the orchestrator
authority to reinterpret review findings or retry without a total bound.

## Decision

### Finding identity belongs to the fresh validator

Every Design-validation finding has a finding ID scoped to that Spec's current
Plan run. On revalidation, the orchestrator gives the fresh validator the
complete accumulated blocking-finding history. The validator must account for
every prior blocking ID exactly once as either `RESOLVED` or still `BLOCKING`,
and assigns a new ID to every materially distinct new finding.

Semantic identity follows the endangered Requirement or boundary and the
missing or conflicting obligation, not incidental wording or a moved document
location. The fresh validator owns that judgment because it independently reads
the complete current Design. The Design author does not declare its own finding
resolved, and the orchestrator only checks the validator's explicit mapping. An
ambiguous or incomplete mapping is not permission to infer identity; Plan stops
with the Design unfinished.

### Two revisions are available per target Spec

The initial Design draft does not consume the budget. During one Plan run, each
target Spec may receive at most two Design-owned revisions after independent
`NOT_READY` verdicts:

1. The initial `NOT_READY` permits the first revision.
2. After fresh revalidation, any prior blocking finding still marked
   `BLOCKING` stops that Spec immediately, even though one revision remains.
3. When every prior blocking finding is `RESOLVED` and the only blockers have
   new finding IDs, Plan may dispatch the second revision.
4. After the second revision, only `READY` proceeds to Design approval. Any
   `NOT_READY` leaves that Spec unfinished, whether the blocker is repeated or
   new.

A requirements rewind or another user-owned decision remains an immediate stop
and consumes no implied permission from this budget. Re-dispatches that request
only a missing status and retries of an environment failure remain governed by
their existing failure rules; they are not Design revisions.

The budget is per target Spec, not per Milestone and not per finding. In
all-Spec scope, one exhausted or repeated-finding branch does not consume
another Spec's budget. Plan continues independent reachable work, while the
existing global Contract Review barrier still waits for current Design approval
from every participating Spec.

### Terminal result is an unfinished Design

Exhausting the revision budget or receiving a repeated blocking finding is an
ordinary unfinished Design result. Plan reports the complete current findings
and next action and does not label the stop `HUMAN_DECISION` unless the Design
phase or validator independently identified a requirements rewind or another
choice that actually belongs to the user.

## Consequences

- Locally repairable new information gets one additional bounded recovery step.
- A persistent objection still stops after its first attempted repair.
- Fresh validation, rather than the author or orchestrator, owns semantic
  finding continuity.
- Parallel Specs retain independent remediation budgets, and Contract Review
  keeps its milestone-global barrier.

## Implementation status

Implemented by the embedded `sb-plan` and `sb-validate-design` Skills, their
focused conformance tests, and Plan forward-test scenario Q3.
