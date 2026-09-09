# 0207: Explain non-current Task plans in Spec status

Status: Accepted

## Context

Decisions 0201 and 0202 preserve a downstream `tasks.yaml` while Requirements
or Design recovery proceeds through Design validation and renewed Contract
Review. Complete traceability and `spec status` remain strict, so a retained
plan that references only inactive Requirements correctly produces
`TRACEABILITY_TASK_SCOPE_INACTIVE` and `State health: inconsistent`.

That diagnostic is real: the plan is not current and cannot be approved or
used as implementation authority. The same artifact can nevertheless be an
expected recovery input that Decision 0206 assigns to the Tasks owner for
explicit reconciliation. Reporting only `inconsistent` makes that supported
route look indistinguishable from state with no defined recovery.

The current artifact set cannot prove whether a non-current plan was retained
by a guarded rewind or authored prematurely. Git presence, Task execution
records, and file contents are not sufficient provenance. Status must not call
the document retained when it can establish only that the plan exists without
current Tasks authority.

## Decision

`spec status` keeps its existing state health and every diagnostic. It adds a
separate Task-plan authority projection when all of the following hold:

- a structurally and semantically readable `tasks.yaml` is present;
- the Tasks gate is not fresh; and
- the declared state is `requirements`, `design`, or `tasks`.

The text projection is:

```text
Task plan authority: not current; reconcile in Tasks phase
```

The command-specific JSON projection adds this object only when applicable:

```json
"taskPlanAuthority": {
  "status": "not_current",
  "nextAction": "reconcile_in_tasks_phase"
}
```

This is an additive field under Decision 0157. Existing `health`, `tasks`,
`coverage`, and `diagnostics` fields are unchanged. A malformed Task document
does not receive this projection because no trustworthy plan was resolved; its
ordinary diagnostics remain authoritative. An implementation-state plan with
stale evidence also receives no recovery projection because it is outside the
pre-implementation reconciliation route.

The projection does not prove that the plan was retained, declare the Spec
healthy, neutralize any diagnostic, authorize deletion, select an item-level
mapping, or change standalone traceability and gate behavior. It says only
that this readable plan is not current authority and names the phase that owns
its reconciliation.

`sb-status` explains that a non-current plan can be preserved recovery input
or an unapproved draft, that the Tasks owner must reconcile it in the Tasks
phase, and that unrelated diagnostics still require their own interpretation.
Status remains read-only and does not choose retain, revise-and-reset, or
remove under Decision 0206. Tasks is the future reconciliation owner, not an
override of the derived immediate `Next action`: Status preserves lifecycle
order and reports intervening Design and Contract Review work before the plan
can be reconciled.

Milestone status is unchanged. Its aggregate health continues to reflect the
participating Spec's inconsistent status; callers use named Spec status for the
new Task-plan authority detail.

## Consequences

- A normal retained-Tasks recovery is visibly distinguishable from an
  inconsistency with no stated owner or route.
- The CLI does not invent provenance it cannot derive from current state.
- Existing JSON consumers keep the stable two-value health field and may
  ignore the additive authority object.
- Strict diagnostics continue to prevent a non-current plan from being treated
  as approved implementation work.

## Verification

CLI integration coverage constructs a Requirements rewind with a retained
completed plan for the previous active set. Text and JSON retain inconsistent
health and the Task-only diagnostic while reporting the non-current authority
and Tasks-phase reconciliation route. After replacement Tasks approval, the
authority projection disappears. A fresh Status forward test verifies that an
agent explains the supported recovery route in lifecycle order, without
deleting, repairing, or otherwise mutating the fixture.
