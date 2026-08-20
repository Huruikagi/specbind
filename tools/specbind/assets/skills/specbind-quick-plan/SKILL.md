---
name: specbind-quick-plan
description: Take one Spec-backed item from its brief through Tasks approval in a single run, using delegated gates. Same artifacts, reviews, and guards as the deliberate flow.
argument-hint: "<spec>"
---

# Take one item through to an approved task plan

Orchestrate the Requirements, Design, and Tasks phases for one Spec-backed item
without stopping at each gate for a confirmation. **You orchestrate. Every
artifact is authored by the phase skill that owns it, and every state change goes
through the CLI.**

The speed comes from removing pauses, not from removing checks. Every review the
deliberate flow runs, this run runs.

## 1. Establish the item and the state

```sh
specbind milestone status
specbind spec status <spec>
```

The item must be a Spec-backed participant in the active milestone. A Direct item
has no Requirements, Design, or task plan to produce — say so and stop.

Read what the state tells you. A Spec already past Requirements does not restart
there; the run picks up at the first phase that is not current.

## 2. Get delegation authorized

Present, before doing anything:

- the milestone and the item
- the gates you will accept without a further pause: requirements, design, tasks
- that each acceptance records **this workflow by name** in the gate's evidence,
  visible afterwards in `specbind spec status`

Take **one confirmation** for the run. Asking again at each gate is the
deliberate flow with extra steps.

**Declining is a legitimate answer and does not end the run.** Without
delegation, sequence the phases exactly as below and pause at each gate for an
explicit approval. The orchestration is still worth something.

Two things this authorization never covers:

- **Invalidating an approved gate.** Delegation authorizes accepting gates, not
  discarding accepted work. If the run discovers it needs a rewind, stop and ask.
- **Accepting the contract review.** It requires no approval authority at all, so
  there is nothing here to extend to it.

## 3. Run the phases

Each phase is a dispatch to the skill that owns it. Give it the Spec identity and
let it read its own inputs; it saw nothing you saw.

| Order | Skill | Produces |
| --- | --- | --- |
| 1 | `specbind-requirements` | Requirements, and the requirements gate |
| 2 | `specbind-design` | Design set, Contract |
| 3 | `specbind-validate-design` | An independent verdict on that design |
| 4 | — | Design gate approval |
| 5 | `specbind-contract-review` | The milestone's accepted contract review |
| 6 | `specbind-tasks` | `tasks.yaml`, and the tasks gate |

**Design validation is not optional here.** With gates delegated, nothing pauses
between authoring and approval, and this is the run's substitute for the reading
a user would otherwise do at the gate. A `NO-GO` stops the run for the design
skill to address; it is not advisory.

Gap analysis is not on this path. Run `specbind-gap-analysis` first if the work
is brownfield and the ground is unfamiliar — this skill does not decide that for
you.

## 4. The contract review is on the path

A single-Spec milestone has a contract review barrier like any other. It is easy
to assume one Spec needs no cross-Spec review; `specbind spec tasks approve` will
refuse regardless, and finding that out from a command failure rather than from
here is how a skill teaches its user to distrust it.

Dispatch `specbind-contract-review` and honor its outcome. It stops when a
finding needs a decision the user owns — that stop is the point of running it.

Do not accept the review yourself, do not compress its findings into a sentence,
and do not proceed to Tasks without an accepted review.

## 5. Read the status block, not the prose

Every dispatch returns a status. **The status decides what happens next.**

| What came back | What you do |
| --- | --- |
| No usable status — missing, ambiguous, or replaced by narrative | Re-dispatch **once**, asking only for the status |
| **Stopped by design** — no authority, a stale upstream gate, a finding needing the user's decision | **Do not retry.** It is the answer. Report it and stop |
| **Failed** — attempted, could not complete | Retry, bounded at **two rounds**, then record it unfinished and stop |

Never infer success because nothing said otherwise.

The middle row is the one to hold. These skills stop deliberately and often.
Retrying a correct refusal either loops or pressures the next run into deciding
something it must not.

## 6. Report

In the project's language:

- what was produced, phase by phase
- which gates were delegated, and that the evidence records it
- what design validation and the contract review found
- where the run stopped, if it did not finish, and what the user can do next
- that implementation has **not** started

## Boundaries

- **Stop after Tasks approval.** Never implement code, never validate completion,
  never touch release.
- Author nothing yourself. If a phase skill would have stopped, this run stops;
  do not finish its work for it.
- No scope changes. No Roadmap items, no new Specs, no removals — if the analysis
  says the boundary is wrong, report it and let discovery own the change.
- Same rules, protocols, and criteria as the deliberate flow. There is no quick
  variant of any of them.
