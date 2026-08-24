# 0120: Fix the quick-plan and batch-plan orchestration contracts

Status: Accepted

## Context

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) fixes what these two
skills are: thin orchestrators over the same artifacts, reviews, approvals, and
CLI guards as the deliberate flow, stopping after Tasks approval and never
implementing. [Decision 0012](./0012-delegated-approval.md) fixes the
authorization that makes them faster than running the phases by hand, and
[Decision 0116](./0116-spec-status-delegated-gates.md) makes the result
observable afterwards.

What none of them fixes is the shape of a run. The inherited `kiro-spec-quick`
and `kiro-spec-batch` describe one, but it was built against a workflow with no
contract review and a uniform dependency model, and SpecBind has neither.

They are treated together because they are one contract at two scales, in the
same way [Decision 0111](./0111-review-task-and-debug-skill-contracts.md) treats
the review and debug skills together.

## One contract, two scales

`specbind-quick-plan` orchestrates one Spec-backed item. `specbind-batch-plan` orchestrates
every Spec-backed item in the active milestone. The phases, gates, protocols, and
guards are identical; Decisions 0093 and 0094 already refuse quick-plan-specific
or batch-plan-specific rule and protocol variants.

Direct items are in neither scope. They have no Requirements, Design, or task
plan to produce. Both skills report them as remaining work rather than silently
implying the milestone is finished.

## Delegation is authorized once, and bounded

Decision 0012 binds delegation to an identified active milestone and to named
gate types, holds it in the run context only, and records the workflow name in
each gate's evidence.

The skill therefore presents, before doing anything: the milestone, the items,
the gates it will accept without a further pause, and that each acceptance will
record this workflow by name. **One confirmation covers the run.** Prompting per
gate is the deliberate flow with extra steps.

Declining delegation is a legitimate answer and does not end the run. The
orchestration is still worth something without it: the skill sequences the
phases, dispatches the work, and pauses at each gate for an explicit approval.

Each delegated phase dispatch carries the workflow name and the gate names the
user authorized. A fresh phase run inherits no conversational context from the
orchestrator; omitting that handoff leaves it with no authority and correctly
stops at the gate.

Two things delegation never covers:

- **Invalidating an approved gate.** Decision 0100 already states it: delegation
  authorizes accepting gates, not discarding accepted work. A run that discovers
  it needs a rewind stops and asks.
- **Accepting the contract review.** Not because authority is withheld, but
  because [Decision 0108](./0108-contract-review-skill-contract.md) requires
  none — the accepted record has no `approval_mode` and no
  `delegation_workflow`. There is nothing for delegation to reach.

## The phases do not share one dependency shape

The inherited batch groups every feature into uniform dependency waves and runs
all phases inside them. [Decision 0082](./0082-derived-milestone-state-machine.md)
gives each phase its own semantics, and applying one wave model to all of them is
wrong in both directions at once:

| Phase | Dependency behavior |
| --- | --- |
| Requirements | Not dependency-gated at all. Every item is available immediately |
| Design | Waits only on direct Spec-backed predecessors having current Design approval |
| Contract review | One global barrier after every participating Spec has current Design approval |
| Tasks | Parallel again. Roadmap dependencies do not serialize Tasks |

Serializing Requirements behind dependencies wastes the parallelism Decision 0082
deliberately kept, and treating the contract review as one more per-item step
misses that it is a barrier for the whole milestone.

**The skill does not compute waves.** Decision 0082 states that wave numbers are
never persisted, and `specbind milestone status` expresses the same information
as each item's `waiting_for` and an `Actionable` list derived from current state.
Batch reads that, acts on what is actionable, and reads it again. Computing the
graph independently re-implements a CLI read model that is already authoritative
and will drift from it the first time the derivation changes.

## The contract review is run, not skipped and not handed back

Both skills reach the barrier: batch obviously, and quick because a
single-Spec milestone has one too. [Decision 0107](./0107-spec-status-contract-review-barrier.md)
observes that the single-Spec case is the one most likely to be misjudged as not
needing a review at all, which makes stating this here worth its space.

The skill dispatches `specbind-contract-review` and honors its outcome. Decision
0075's "same reviews as the deliberate flow" means running them, not routing
around them, and Decision 0108's stop conditions do the work: a finding needing a
decision the user owns stops the run, exactly as it would in the deliberate flow.

The orchestrator does not accept the review itself, does not summarize its
findings into a sentence, and does not proceed to Tasks on an unaccepted review —
`tasks approve` would refuse anyway, and discovering that from a command failure
rather than from the contract is how a skill teaches its user to distrust it.

## Design validation stays on the path

With every gate delegated, nothing pauses between authoring and approval. The
contract review is a real check but the wrong one for this gap: it looks across
Spec boundaries, not inside one Spec's design.

Both skills therefore route through `specbind-validate-design` before design
approval. This is the accelerated flow's substitute for the reading a user would
otherwise do at the gate, and it reuses an existing contract rather than
inventing a quick-plan-specific sanity review the way the inherited skill did.

A validator `NO-GO` is not itself a phase status and does not make an
independently fixable draft a user-owned decision. The orchestrator returns the
complete findings to the owning Design skill for one revision, then validates
the revised draft in a fresh context. Approval remains blocked throughout. A
Design result that identifies a requirements rewind or another user-owned
decision is a deliberate stop, as is a repeated `NO-GO` after that bounded
revision. The orchestrator never repairs the artifacts itself.

It applies to batch as much as to quick. The hole is identical, and batch is the
higher-volume path, so exempting it would put the weaker check on the run that
produces more.

## Failure is classified by what the run returned

A dispatched phase run returns a status from a closed set, and **the status
decides what happens next — never the shape of the prose around it**. This is the
convention `specbind-implement` already uses under
[Decision 0109](./0109-subagent-dispatch-contract.md).

| Outcome | Handling |
| --- | --- |
| No usable result — the status is missing, ambiguous, or replaced by narrative | Re-dispatch once, asking only for the status |
| **Stopped by design** — no approval authority, an upstream gate is stale, a finding needs a decision the user owns | **Not retried.** It is the answer, and retrying it either loops or pressures the run into deciding what it must not |
| **Failed** — attempted and could not complete | Bounded at two rounds under Decision 0075, then recorded unfinished |

The middle row is the one that matters. SpecBind phase skills stop deliberately
and often, and an orchestrator that treats every non-success as something to try
again converts correct refusals into pressure to proceed.

## What an unfinished item leaves behind

The run continues as far as it can. What that means depends on where the item
stopped, and follows from the dependency table above: an unfinished Requirements
phase blocks nothing else, an unfinished Design blocks that item's dependents,
and **one unfinished Spec stops the contract review for every participant**,
because the barrier requires current Design approval across the whole set.

So a failed run finishes everything reachable, stops at the barrier, and reports
which item is unfinished and why. It does not attempt the review it knows cannot
pass.

**It never drops the unfinished item from scope to proceed.** Scope belongs to
discovery under Decision 0097, and the barrier is defined over the participating
set, so removing an item to unblock the barrier would change what the milestone
is in order to report success on it.

Partial completion needs no representation. Each Spec already holds its own
state, and there is no "batch in progress" to record.

## Where the run stops

After Tasks approval for every item it completed. Neither skill implements code,
under Decision 0075, and neither runs completion validation or release work.

The closing report names what was produced, what was delegated, what the contract
review found, what remains unfinished and why, and the Direct items neither skill
touched.

## Boundary

- Orchestrate only. Every artifact is authored by the phase skill that owns it,
  and every state change goes through the CLI.
- No quick-plan-specific or batch-plan-specific rules, protocols, or review criteria.
- No scope changes: no Roadmap items, no new Specs, no removals.
- No gate invalidation without asking, and no rewind on delegated authority.
- Writing while a milestone holds an accepted completion carries the cost stated
  by the `okf-authoring` protocol under
  [Decision 0119](./0119-writing-while-a-completion-stands.md). Neither skill
  restates it.

## Consequences

- Batch follows the dependency semantics SpecBind actually accepted, and reads
  them from the command that derives them rather than recomputing them.
- The contract review is reached by the accelerated paths, including the
  single-Spec case where its absence would be least noticed.
- Delegation stays one bounded, named authorization with a recorded trace, rather
  than a flag that means something different in each phase.
- A run that hits a deliberate stop reports it as an answer instead of retrying
  it, which is what makes the phase skills' stop conditions worth having.
- An accelerated run is checked by design validation, so the speed comes from
  removing pauses rather than from removing review.
- A correctable validation finding gets one Design-owned revision and a fresh
  independent verdict instead of being mistaken for a user confirmation gate.

## Implementation status

Implemented. `specbind-quick-plan` and `specbind-batch-plan` are embedded and installed,
completing the Decision 0075 v1 skill set at seventeen.

Both take one bounded delegation confirmation, route through
`specbind-validate-design` before design approval, dispatch
`specbind-contract-review` at the barrier without accepting it themselves, and
classify a dispatched run's outcome by its returned status rather than by its
prose. Batch drives its rounds from `specbind milestone status` rather than
computing a dependency graph, and stops at the barrier with the unfinished item
named instead of removing it from scope.

Forward-test scenarios Q1 through Q5 and B1 through B6 remain outstanding,
pending a run against the fixture project.
