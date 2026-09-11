# 0213: Recover Plan dispatch capacity without collapsing roles

Status: Accepted

## Context

Decision 0109 requires fresh dispatch where context separation is part of the
work and permits a main-context compatibility path when a host cannot dispatch.
Decision 0194 then requires fresh Design validation, finding continuity, and a
per-Spec revision budget. Neither contract distinguishes a host that never
supports dispatch from one that supports it but exhausts a finite receiver
limit partway through Plan.

That distinction matters after an author has produced an unapproved Design. The
orchestrator can no longer validate the work in its own context and call the
result independent. Retrying receiver creation without a recovery order can
also lose the author/validator roles, accumulated finding IDs, revision budget,
or the exact dirty handoff that must remain uncommitted.

## Decision

Plan treats dispatch-capacity failure as an environment condition with an
ordered recovery:

1. Consume every completed receiver result first. When the host exposes a safe
   release operation, release only a completed receiver whose result has been
   consumed and which the current Plan run will not continue. A Skill never
   invents a release command or assumes that completion freed capacity.
2. Continue an addressable receiver only for the same role and same phase run.
   This covers a missing-status retry or returning a Design finding ledger to
   its author. It never repurposes an author as an independent validator, a
   validator as an author, or one Spec's receiver for another Spec.
3. When nested dispatch is unavailable but the orchestrator can still dispatch,
   the orchestrator sequences the next receiver itself with the same
   self-contained brief, protocol, authority, and finding history. The child is
   not required to create its successor.
4. When the affected role has not begun and doing the work here does not claim
   independence from work already observed in this context, Plan performs the
   Decision 0109 main-context fallback. It does not stop to ask the user to
   manage host receiver slots. Mandatory independent Design validation never
   uses this fallback after the Design handoff exists.

If the required role still cannot start, Plan stops that branch without
approving, committing, or rewriting its unapproved artifacts. Other independent
all-scope branches may continue. The terminal report names the affected Spec and
phase, the exact unapproved path set and Git state, every accumulated finding ID
and disposition, revisions used and remaining, the delegation authority that
was actually supplied, and the exact role required to resume. A later context
does not infer omitted approval or rewind authority from the report.

Capacity recovery never skips independent validation, changes a receiver's
role, resets a revision budget, discards a finding, checkpoints an unapproved
Design, or treats a host limit as a product finding.

## Consequences

- Hosts with release or continuation controls can recover without relying on a
  universal mechanism name in the product Skill.
- A finite host limit becomes a precise resumable environment stop when
  independence cannot be preserved.
- Nested and top-level orchestration remain behaviorally equivalent because the
  same brief and authority cross the receiver boundary.
- The ordinary no-dispatch compatibility path remains available without making
  an already observed Design self-validating.

## Verification

Skill conformance tests require the ordered recovery, role-preservation rules,
and complete terminal handoff. A fresh Plan forward test saturates receiver
capacity after an unapproved Design exists and verifies that the run either
recovers through a supported host operation or stops without approval or commit
while retaining the full restart information.
