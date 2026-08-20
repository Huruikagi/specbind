---
name: specbind-batch-plan
description: Take every Spec-backed item in the active milestone through to approved plans in one run, ending at Tasks approval. Use only for an all-Spec milestone request; do not use for one named or targeted item.
---

# Take the whole milestone through to approved task plans

Bring every Spec-backed item in the active milestone from its brief to an
approved task plan. **You orchestrate and stay light.** The phase skills author
everything, the CLI owns every state change, and the dispatched runs do the
reading.

This is `specbind-quick-plan` at milestone scale. Same phases, same gates, same
reviews, same guards.

## 1. Establish the scope

```sh
specbind milestone status
```

The Spec-backed participants are your scope. **Direct items are not** — they have
no Requirements, Design, or task plan to produce. Note them and report them at
the end as remaining work, so the milestone does not look finished when it is not.

If no milestone is active, say so and stop. Scope belongs to discovery.

## 2. Get delegation authorized

Present, before doing anything: the milestone, **every item** the run will touch,
the gates it will accept without a further pause — requirements, design, tasks —
and that each acceptance records this workflow by name in the gate's evidence.

Take **one confirmation** for the run.

The request to run this skill is **not** that confirmation. Present the scope
above, stop, and wait for the user's answer before dispatching any phase or
approving any gate.

Declining does not end the run; it means pausing at each gate for an explicit
approval. At milestone scale that is a lot of pauses, so say so before assuming
it is what the user wants.

Delegation never covers invalidating an approved gate. A run that discovers it
needs a rewind stops and asks.

## 3. Do not compute waves

The phases do not share one dependency shape:

| Phase | Dependency behavior |
| --- | --- |
| Requirements | **Not gated at all.** Every item is available immediately |
| Design | Waits only on direct Spec-backed predecessors having current design approval |
| Contract review | **One global barrier** for the whole milestone |
| Tasks | **Parallel again.** Dependencies do not serialize Tasks |

Serializing Requirements behind dependencies throws away parallelism the product
deliberately kept. Treating the contract review as one more per-item step misses
that it is a barrier for everyone.

**Do not build the dependency graph yourself.** `specbind milestone status`
already reports each item's `waiting_for` and an `Actionable` list derived from
current state. Act on what is actionable, then read it again. A graph you compute
independently duplicates a read model that is already authoritative, and drifts
from it the first time the derivation changes.

The loop is: read status → dispatch everything actionable, in parallel → collect
→ read status again.

## 4. Run the phases

Each item's phase is a fresh dispatch. Give it the Spec identity, the phase to
run, and, when delegation was accepted, the workflow name
`specbind-batch-plan` plus the authorized gate names. It reads its own artifact
inputs; authorization omitted from the dispatch does not reach it.

Use the registered `specbind-planner` role when the host provides it; otherwise
use an ordinary fresh subagent. The role changes capability, never the owning
skill, scope, or delegated gate authority carried by the brief.
Fallback is only for an absent role. A configured role whose model cannot start
is a configuration or environment failure, not permission to change models.

Per item, in order:

1. `specbind-requirements` — Requirements and its gate
2. `specbind-design` — Design set and Contract
3. `specbind-validate-design` — an independent verdict
4. design gate approval

**Design validation is not optional, and not something batch skips because it is
running many items.** With gates delegated nothing pauses between authoring and
approval, and this run produces more unreviewed material than any other. A
`NO-GO` stops that item.

Then, once **every** participating Spec holds current design approval:

5. `specbind-contract-review` — once, for the milestone

Then, in parallel across all items:

6. `specbind-tasks` — `tasks.yaml` and its gate

## 5. The barrier is the whole point of the middle

The contract review is where cross-Spec problems surface, and batch is the run
most likely to have created them — many designs authored in parallel by contexts
that never saw each other.

Dispatch `specbind-contract-review` and honor its outcome. Do not accept it
yourself, do not compress its findings, and do not start Tasks without an
accepted review.

If it stops for a decision the user owns, the run stops. That is the check
working.

## 6. Read the status block, not the prose

Every dispatch returns a status. **The status decides what happens next.**

| What came back | What you do |
| --- | --- |
| No usable status — missing, ambiguous, or narrative | Re-dispatch **once**, asking only for the status |
| **Stopped by design** — no authority, stale upstream gate, a finding needing the user's decision | **Do not retry.** It is the answer |
| **Failed** — attempted, could not complete | Retry, bounded at **two rounds**, then record it unfinished |

Never infer success because nothing said otherwise. These skills stop
deliberately and often; retrying a correct refusal either loops or pressures the
next run into deciding what it must not.

## 7. When an item does not finish

Continue as far as the work allows. What that means depends on where it stopped:

- **Requirements unfinished** — blocks nothing else. Other items continue
- **Design unfinished** — blocks that item's dependents, and only those
- **Either one, at the barrier** — the contract review requires current design
  approval across **every** participating Spec, so one unfinished item stops the
  review for all of them

So: finish everything reachable, stop at the barrier, and report which item is
unfinished and why. **Do not attempt a review you know cannot pass.**

**Never drop the unfinished item from scope to get past the barrier.** Scope
belongs to discovery, and the barrier is defined over the participating set —
removing an item to unblock it changes what the milestone is in order to report
success on it.

Partial completion needs no bookkeeping. Each Spec holds its own state, and a
later run picks up from `milestone status`.

## 8. Report

In the project's language:

- per item: what was produced, and its state now
- which gates were delegated
- what design validation found, per item
- what the contract review found
- what is unfinished, why, and what the user can do
- the Direct items this run did not touch
- that implementation has **not** started

## Boundaries

- **Stop after Tasks approval.** Never implement, never validate completion,
  never touch release.
- Stay light. Read `milestone status`; let the dispatched runs read everything
  else. Pulling every artifact into this context defeats the fan-out.
- Author nothing yourself, and never finish a phase skill's work because it
  stopped.
- No scope changes: no Roadmap items, no new Specs, no removals.
- Same rules, protocols, and criteria as the deliberate flow. There is no batch
  variant of any of them.
