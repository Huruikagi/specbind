# 0199: Delegate in-scope replanning through Drive

Status: Accepted

Amends the ordinary authority and unsafe-handoff boundaries of
[Decision 0168](./0168-milestone-drive-orchestrator.md) only for explicit replan
delegation, and the confirmation requirements of the Design and Tasks phases.

## Context

A maintainer can delegate most planning approvals and ask Drive to complete
delivery, yet implementation discoveries require another confirmation for
every approved-plan correction. Returning a revised plan as a final answer
also forces the maintainer to restart work they already requested.

## Decision

`sb-drive --replan` is a Skill argument, not a new CLI command or persisted
configuration. Its explicit natural-language equivalent is also accepted. The
request itself delegates in-scope Design, Contract, and Tasks correction;
there is no second delegation confirmation. Without it, current behavior stays.

The boundary is the original approved Requirements and active delivery
Milestone scope, including dependencies. Within that boundary, authority covers
gate invalidation, Design/Contract/Tasks authorship, independent validation,
delegated Design/Tasks reapproval under workflow `sb-drive`, renewed Contract
Review, and reimplementation of previously completed work. Requirements
changes, scope changes, out-of-scope Spec changes, and unsettled external
compatibility or migration obligations require a user decision. Release,
reverse adoption, Direct reclassification, and destructive recovery are not
authorized by this option.

Drive remains a controller. A concrete semantic recovery finding may dispatch
the owning Plan Skill even while status still reports a fresh approved gate;
this is the bounded exception to ordinary actionable-entry scheduling. Plan
loads its recovery reference, validates scope, and dispatches the owning
phases. Implementers never revise plans, and Drive never authors artifacts.
After the recovery handoff Drive rereads state and continues implementation and
validation without requiring another restart request.

Drive loads its own replan-routing reference only when `--replan` or its
explicit natural-language equivalent supplied that authority. The entrypoint
retains the trigger and ordinary no-replan boundary; the reference retains the
scope, delegation, continuity, and partial-worktree procedure.

The existing CLI ordering remains intact. Design invalidation removes the
milestone review; its renewed acceptance still requires every participant at
the Design-approved pre-Tasks barrier. Plan accounts for all participant
rewinds, preserves old plans/progress in recoverable Git revisions, and lets
Tasks owners remove the exact saved plans before review, then reconstruct and
reapprove them afterwards. This removal is authorized reconstruction, not a
reviewer workaround. Tasks-only revisions retain the accepted review.

Progress mapping follows obligations and evidence, never positional IDs alone.
Unchanged proven work can retain its completed records; changed obligations
return to implementation. Old gate or Spec completion evidence is never
restored. Invalidation costs and mappings are reported without another pause.

Attributable partial source from the current implementation may remain through
the same recovery, with exact paths/diff carried between owners. Planning does
not edit or commit it, and Drive does not switch to unrelated work while it
remains. Unrelated, unattributed, or conflicting work still blocks recovery.
Existing approved progress metadata may receive an adapter-permitted narrow
checkpoint to preserve the pre-replan record; partial source and unapproved
replacement plans may not be committed as WIP.

Review budgets and finding history survive dispatches. A repeated unresolved
defect or spent budget stops recovery; new dispatches cannot reset them. No
schema, CLI gate, persistent queue, or authority artifact is added.

## Consequences

- Maintainers opt into meaningful autonomy without weakening verification.
- Design recovery can revalidate more than the initially defective Spec because
  Contract Review is a Milestone-wide barrier; this cost is part of delegation.
- Both installed Agent packages receive the same authority and recovery rules.
- Forward tests cover authorized recovery, default confirmation, and a request
  outside the approved Requirements boundary.
