# Authorized replan routing

Read this procedure only when `--replan` is present or the maintainer explicitly
requests delegated replanning. The option or explicit request is authorization:
do not ask for a second delegation confirmation.

The authority covers Design, Contract, and Tasks revisions within the active
delivery Milestone's existing scope and approved Requirements. This includes
necessary Design/Tasks invalidation, delegated reapproval, independent Design
validation, renewed Contract Review, affected consumers inside scope, and
reimplementation of previously completed work.
It also covers the completion-only withdrawal procedure below when this same
replan stales previously fresh accepted completion in participating Specs.

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

Before Plan starts, capture a clean committed baseline, milestone identity and
scope, participant states, completion freshness and implementation revisions,
and earlier gate freshness. Name the accepted completions the write will stale.
Keep these as run-local handoff facts. If no clean baseline can be established
without disturbing partial work, do not invent one: this automatic completion
aftercare will need a separately authorized recovery.

Carry the original approved Requirements and Milestone scope boundary through
every recovery dispatch; a revised Design cannot expand the delegation. Report
the concrete change and invalidation cost as progress, without waiting for
approval within that boundary. Preserve existing retry budgets and finding
history across dispatches; the same unresolved defect after recovery is parked,
not another fresh replan attempt. A replan is not a way to reset a spent budget.

For a Spec-backed Task recovery, carry the implementation owner's full ledger:
recovery identity and Task mapping, allowance identity and user authorization,
recovery mode and its explicit selection/scope, current cycles used/remaining
(out of three for bounded, `uncapped` remaining for progress), historical counts separately, all
finding IDs/dispositions/evidence, latest assessment, partial paths, authority,
and next owner. Reserve one Task cycle when the planning owner starts; the whole
Plan repair consumes that cycle, while Plan's own phase bounds still apply.
Preserve whether a cycle is already reserved so a resumed owner never counts it
twice. Return the same ledger and Plan's evidence to implementation for independent
reassessment. Drive cannot mark findings resolved or infer missing counters from
status. A repeated finding, missing history, or exhausted budget cannot gain an
allowance from a new dispatch or remapped Task ID.
Only a later qualifying human resumption after a reported recovery-limit stop
can authorize a new allowance through the implementation owner. Transport that
instruction and the resulting allowance unchanged; never reuse the instruction
to renew it again. Preserve failed approaches and unknown historical totals.
Preserve progress mode through every owner return and restart; do not replace it
with a three-cycle cap or apply it to Plan's independent phase budgets.

After the planning owner completes its checkpoints, reread status, apply the
completion withdrawal below when eligible, and resume
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

## Withdraw completion made stale by this replan

Before stopping on inconsistent health, compare the retained baseline with
current `milestone status --json`, each affected `spec status <spec> --json`,
Git status, and the actual intervening commit diff. This is an explicit recovery
exception to actionable-entry scheduling, not permission to run implementation
through inconsistent health.

Precheck the entire affected set before dispatching any withdrawal:

- Same active delivery milestone, original scope and supplied replan authority.
- Each target had fresh accepted completion at the clean recorded baseline.
- Intervening changes belong only to this authorized recovery, not an unrelated
  write. The planning owner's narrative alone is insufficient evidence.
- Each target is `release_ready`, with fresh Requirements/Design/Tasks and stale
  completion whose only cause is `FRESHNESS_COMPLETION_PROJECT_CHANGED`.
- Every `MILESTONE_SPEC_INCONSISTENT` is explained by those targets; no other
  gate, Contract Review, scope, or integrity inconsistency exists.
- All target `spec.yaml` files are clean and unstaged; no live owner or unsafe
  partial work remains. Never clean up the worktree to qualify.

Missing history, pre-existing staleness, another diagnostic, unattributed edits,
or a dirty target stops automatic recovery. Report the ordinary owning workflow
and needed evidence/authority; `--replan` alone cannot waive these conditions.

For each eligible target in returned milestone order, dispatch the complete
`sb-validate-implementation` Skill with **completion-withdrawal-only authority**,
the original replan authority, baseline/current revisions and status evidence,
exact target path, authorized delta, and the remaining eligible set. Supply the
ordinary cwd, PATH and instruction facts. The owner independently checks the
boundary, invalidates only completion and closes its own narrow checkpoint.
Do not ask for another confirmation, perform that work in Drive, or dispatch
final validation while implementation remains pending.

After each terminal handoff, reread Git and milestone/Spec status. Require the
exact withdrawal checkpoint, target `implementation` with no completion, fresh
earlier gates, and no new inconsistency. Other prechecked targets may still be
stale until their turn. A failure, missing checkpoint or unchanged target stops
this route; do not retry unchanged evidence, roll back completed withdrawals,
or switch to implementation while any inconsistency remains.

Retain the completed checkpoints and remaining set across an interrupted
handoff. Skip a completed target only when that handoff and the current Git
delta prove its exact withdrawal; status alone cannot reconstruct the history.
These withdrawals do not add or reset the Task recovery budget.

When the set is cleared, resume the ordinary CLI schedule in this same turn:
remaining implementation, whole-milestone convergence, then fresh validation.
The withdrawal owner's `WAITING` is a handoff to this schedule, not a request
for the user to restart Drive. Never restore previous completion evidence.
