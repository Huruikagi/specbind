# 0206: Retained Task progress must reconcile to active obligations

Status: Accepted

## Context

Decision 0202 preserves a stale Task plan and execution records through Design
repair and renewed Contract Review so they remain visible repair input. It then
requires the Tasks owner to preserve execution only where obligations and
evidence still apply.

A fresh driver interpreted a request to preserve that input through review as a
requirement to keep an inactive completed Task in the replacement plan. It added
a new active Task beside it, then correctly stopped because complete traceability
rejects a Task that references no active Requirement. The record was not lost,
but the proposed mapping could never be approved.

## Decision

Preservation through Contract Review ends at the Tasks owner's explicit
before/after reconciliation. Every retained Task with execution state receives
exactly one disposition:

- retain the Task and its state when its active obligation and evidence still
  apply;
- revise the Task and reset it to pending when changed active work remains; or
- remove the Task from the active plan and remove its keyed execution entry when
  it serves no active Requirement.

A historical-only Task that references no active Requirement must not remain in
the replacement active plan. Its removal is not silent: the owner presents the
old identity, completed or blocked state, reason it no longer applies, and the
exact removal before obtaining confirmation. The retained bytes remain in Git
history and in the renewed-review checkpoint; version 1 creates no sidecar
archive or synthetic retired Task.

The owner never relabels old execution as evidence for a new obligation. A
request to preserve the old plan and progress through review means “do not
delete before the Tasks mapping,” not “retain every item after reconciliation.”

This Decision clarifies Decision 0202 and the Tasks revision contract. It does
not weaken traceability or authorize unconfirmed progress removal.

## Consequences

- A Requirements replacement can reach an approvable Task plan without inactive
  references or inherited completion for changed work.
- Completed and blocked history stays visible until its owner explicitly maps
  it, and Git retains the pre-repair record afterwards.
- No user-operated deletion, manual lifecycle edit, or archive artifact is
  introduced.

## Verification

Skill tests require the three dispositions and forbid historical-only inactive
Tasks. RR1 fresh forward testing verifies preservation through renewed review,
explicit mapping, active-only replacement, fresh Tasks approval, and clean Git
history.
