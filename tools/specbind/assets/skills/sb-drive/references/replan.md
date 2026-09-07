# Authorized replan routing

Read this procedure only when `--replan` is present or the maintainer explicitly
requests delegated replanning. The option or explicit request is authorization:
do not ask for a second delegation confirmation.

The authority covers Design, Contract, and Tasks revisions within the active
delivery Milestone's existing scope and approved Requirements. This includes
necessary Design/Tasks invalidation, delegated reapproval, independent Design
validation, renewed Contract Review, affected consumers inside scope, and
reimplementation of previously completed work.

It never authorizes changing or invalidating Requirements, changing Milestone
scope or dependencies, changing an out-of-scope Spec, weakening a required
behavior, or Release. An unsettled external compatibility or migration
obligation still needs the user. Reverse adoption and Direct reclassification
are not covered.

On a concrete `REROUTABLE` Design/Contract/Tasks finding, dispatch the complete
`sb-plan` owning Skill with the exact affected Specs, finding, `sb-drive`
delegation workflow, authorized Design/Tasks gates and rewinds, and the
installed `references/replan.md` path from that Skill. This is the explicit
recovery exception to selecting an ordinary status action: status cannot
diagnose a semantic defect in a fresh approved artifact. The planning owner
validates the boundary before mutation. Do not revise artifacts in Drive or
dispatch an internal planner role in place of that Skill.

Carry the original approved Requirements and Milestone scope boundary through
every recovery dispatch; a revised Design cannot expand the delegation. Report
the concrete change and invalidation cost as progress, without waiting for
approval within that boundary. Preserve existing retry budgets and finding
history across dispatches; the same unresolved defect after recovery is parked,
not another fresh replan attempt. A replan is not a way to reset a spent budget.

After the planning owner completes its checkpoints, reread status and resume
implementation and validation in this same Drive run. "The plan is revised;
resume next time" is not a terminal result while safe work remains reachable.
Carry resolved Task blockers and their exact identity mapping to the resumed
`sb-implement` owner, including when its first operation must be `tasks reopen`
before status can expose pending work. This is part of the same recovery.

An attributable partial implementation from this run may be carried through
the same recovery when its exact paths and diff are reported and retained for
the implementation owner. Planning must leave those source changes untouched;
do not switch to unrelated work, reset, stash, discard, or commit them as WIP.
Unrelated, unattributed, or conflicting partial changes still stop the run.
