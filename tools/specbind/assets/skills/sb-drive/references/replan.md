# Authorized replan routing

Read this procedure only when Drive resolved replan to true from the current
request or project default. The Drive execution request with that resolved
setting is authorization within the boundary below:
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
dispatch an internal planner role in place of that Skill. Use the entrypoint's
isolated-owner dispatch: a new receiver with no inherited conversation and a
self-contained recovery brief. Never reuse the prior implementation receiver
as Plan, or Plan's completed receiver for the subsequent implementation.

Before Plan starts, run `specbind milestone replan capture`. Retain its exact
`baselineRevision` and report the `freshCompletions` the repair can stale.
The CLI verifies the clean committed baseline and historical gate state.
Keep the output with the original authority as run-local handoff evidence.
If capture fails, do not invent baseline facts or clean up partial work to
qualify: automatic completion aftercare is unavailable. Independently authorized
Plan repair may proceed only within its own prerequisites.

Carry the original approved Requirements and Milestone scope boundary through
every recovery dispatch; a revised Design cannot expand the delegation. Report
the concrete change and invalidation cost as progress, without waiting for
approval within that boundary. Preserve existing retry budgets and finding
history across dispatches; the same unresolved defect after recovery is parked,
not another fresh replan attempt. A replan is not a way to reset a spent budget.

For a Spec-backed Task recovery, carry the implementation owner's full ledger:
recovery identity and Task mapping, allowance identity and user authorization,
recovery mode and its selection provenance/scope, current cycles used/remaining
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

Before stopping on inconsistent health, retain Plan's terminal checkpoint as
the fixed `recoveryRevision`. Precheck the entire affected set:

```sh
specbind milestone replan check --baseline <baselineRevision> --recovery-revision <recoveryRevision>
```

The CLI reconstructs committed states and checks scope, Requirements authority,
fresh earlier gates/review, completion staleness solely under
`FRESHNESS_COMPLETION_PROJECT_CHANGED`, and all milestone inconsistencies. It
rejects missing history, pre-existing staleness, dirty state and unexpected
post-recovery commits. Errors stop this route; never fall back to manually
assembling an eligibility verdict or advance either fixed revision to hide work.

The result is mechanical evidence. Independently verify original replan
authority, no live owner, and the actual baseline-to-recovery Git diff and
commit history against the authorized repair, using `recoveryChangedPaths` as
an index. Paths or the planning owner's narrative alone do not prove cause.
`requiresAuthorityAndDeltaReview` must not be treated as an approval. Unattributed
work stops automatic recovery. This exception does not permit implementation
through inconsistent health.

For each target in the returned `remaining` order, dispatch a new isolated owner
with no inherited conversation for the complete
`sb-validate-implementation` Skill with **completion-withdrawal-only authority**,
the original replan authority, fixed baseline/recovery revisions and check output,
exact target path, authorized delta, and the remaining eligible set. Supply the
ordinary cwd, PATH and instruction facts. The owner independently checks the
boundary, invalidates only completion and closes its own narrow checkpoint.
Do not ask for another confirmation, perform that work in Drive, or dispatch
final validation while implementation remains pending.

After each terminal handoff, rerun the same check. Its `withdrawn` entries must
prove the returned target and checkpoint; its `remaining` set must advance by
exactly that target. The CLI validates the narrow committed transition and
preserved earlier gates. A failure, missing checkpoint or unchanged target stops
this route; do not retry unchanged evidence, roll back completed withdrawals,
or switch to implementation while any inconsistency remains.

Retain both fixed revisions, authority, completed checkpoints and remaining set
across an interrupted handoff. Resume by running the same check and matching
its proven withdrawals with the handoff; status alone cannot reconstruct history.
These withdrawals do not add or reset the Task recovery budget.

When the set is cleared, resume the ordinary CLI schedule in this same turn:
remaining implementation, whole-milestone convergence, then fresh validation.
The withdrawal owner's `WAITING` is a handoff to this schedule, not a request
for the user to restart Drive. Never restore previous completion evidence.
