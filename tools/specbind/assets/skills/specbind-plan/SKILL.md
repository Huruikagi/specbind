---
name: specbind-plan
description: Orchestrate planning by default whenever the user asks to plan active work or take it through approved task plans. Supports one named Spec or explicitly all Specs; when neither scope is stated, ask before any phase work.
argument-hint: "[<spec> | --all]"
---

# Take an explicit scope through to approved task plans

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Select this Skill as the default planning entry point before any individual
phase Skill whenever the request asks to plan active work. It owns the complete
route from Requirements through Tasks approval. This routing applies even when
the request names neither a Spec nor all Specs. Do not infer the current
actionable phase and start it directly; establish planning scope first.

Orchestrate planning for either one named Spec or every Spec-backed
participant in the active milestone. **You orchestrate and stay light.** The
phase skills author everything, the CLI owns every state change, and dispatched
runs do the artifact reading.

Delegation changes approval pauses, not checks. Named and all-Spec scope, and
delegated and explicit approval, use the same phases, gates, reviews, protocols,
and guards.

## 1. Establish the scope before doing work

```sh
specbind milestone status
```

There are exactly two scope modes:

- **Named scope** — one named or targeted Spec-backed Roadmap item. Also run
  `specbind spec status <spec>`.
- **All scope** — every Spec-backed participant, selected only by `--all` or an
  equally explicit request for every or all Specs.

If the invocation supplies neither a named target nor explicit all-Spec intent,
use the status only to present the available choices. Ask whether the user wants
one named Spec or all Specs, then **stop for the answer before any phase dispatch,
artifact authoring, or gate approval**. Do not infer all scope from the number
of participants. A milestone containing one Spec still requires the user's
scope choice.

The stopping response itself must name the available Spec choices, name the
all-Spec choice, and explicitly ask the user to select one. A report that merely
says nothing changed or lists the commands run is not the required scope
question and leaves the workflow without a resumable decision.

Scope selection is not delegated-gate authorization. A single answer may do
both only when it explicitly selects the scope and authorizes the gates described
below.

A Direct item has no Requirements, Design, or task plan. In named scope, say so
and stop if the target is Direct. In all scope, exclude Direct items and report
them at the end as remaining work. If no milestone is active, say so and stop;
scope belongs to discovery.

## 2. Get delegation authorized

Once scope is explicit, present before doing anything:

- the milestone and the exact item or items the run will touch;
- the gates it will accept without another pause: requirements, design, tasks;
- that every delegated acceptance records `specbind-plan` in its durable
  gate evidence, visible afterwards through `specbind spec status`.

Take **one confirmation** for the run. The request to run this skill is **not**
that confirmation unless it explicitly authorizes those named gates for the
presented scope. Otherwise stop and wait before dispatching any phase or
approving any gate.

Declining delegation is legitimate and does not end orchestration. Sequence the
same phases and pause at each gate for explicit approval. Explain that all scope
can require many approval pauses.

Delegation never covers invalidating an approved gate. If the run discovers a
rewind is needed, stop and ask. It also does not accept the Contract Review:
that review requires no approval authority.

## 3. Follow the CLI's phase-relative scheduling

The phases do not share one dependency shape:

| Phase | Dependency behavior |
| --- | --- |
| Requirements | **Not gated.** Every in-scope item is immediately available |
| Design | Waits on direct Spec-backed predecessors having current Design approval |
| Contract Review | **One global barrier** across every participating Spec |
| Tasks | **Parallel again.** Roadmap dependencies do not serialize Tasks |

Do not build or persist dependency waves. `specbind milestone status` already
reports `waiting_for` and the authoritative `Actionable` list. The loop is:
read status → dispatch in-scope actionable work → collect → read status again.

In all scope, dispatch every in-scope actionable item, in parallel where the host
supports it. In named scope, dispatch only the selected item. Never expand named
scope to a predecessor or another participant merely to clear a dependency or
the global barrier. If outside-scope work must progress before the selected item
can reach Tasks approval, report that blocker and stop. An explicit all-scope
request or discovery owns scope expansion.

## 4. Run the owned phases

Before the first dispatch, establish the exact project working directory and
confirm `specbind --version` from there. Record the executable resolution and
any project-local `PATH` entry or equivalent environment fact required to
reproduce that same resolution. Do not assume a fresh receiver inherits the
orchestrator's current directory or process environment.

Each phase is a fresh dispatch. Give it the Spec identity, phase, and, when
delegation was accepted, the workflow name `specbind-plan` plus the
authorized gate names. It reads its own artifact inputs; authorization omitted
from the dispatch does not reach it. Also give every phase, Design validator,
and Contract Review receiver the exact project working directory, the
project-local instruction files that apply there, and the confirmed `specbind`
executable, version, and required environment facts. The receiver must start in
that directory and reproduce the same CLI resolution before artifact work.

If the receiver cannot reproduce the confirmed executable and version, treat
that as an environment failure. It must not fall back to another `specbind`,
silently alter `PATH`, install a replacement, or reinterpret missing commands
as a workflow or artifact defect. Correct the dispatch payload or environment
before the bounded retry. These operating facts grant no additional scope,
mutation, or approval authority.

Use the registered `specbind-planner` role when the host provides it; otherwise
use an ordinary fresh subagent. The role changes capability, never the owning
skill, scope, or authority. Fallback is only for an absent role. A configured
role whose model cannot start is an environment failure, not permission to
change models.

Per item, in order:

1. `specbind-plan-requirements` — Requirements and its gate
2. `specbind-plan-design` without Design-gate authority — Design set and Contract;
   stop before approval and checkpoint
3. `specbind-validate-design` — an independent verdict
4. re-dispatch `specbind-plan-design` with delegated authority for Design approval
   and its checkpoint

**Design validation is mandatory.** A `NO-GO` blocks approval. It is a validator
verdict, not a phase status: return its complete findings to that item's
`specbind-plan-design` run for one revision, then dispatch fresh validation. Stop if
Design reports a requirements rewind or another user-owned decision, or if the
fresh validator repeats `NO-GO`. Never approve a rejected draft, repair its
artifacts in the orchestrator, or let remediation change another item's scope.

Do not give the authoring dispatch Design-gate authority. Its expected
stopped-by-design result is an unapproved Design ready for independent review,
not failure. Only `READY` permits the approval dispatch. If Design was approved
before validation, stop; a later verdict cannot retroactively restore the order.

That unapproved Design handoff is the one deliberate exception to the general
clean-checkpoint rule. Before validation, require the dirty set to contain only
the Design artifact paths and that Spec's Contract path reported by its author.
When the author actually recorded a `DEFERRED` finding, the set may also contain
the exact project-relative destination named by the active deferred adapter.
Verify that destination through `adapter list` and `adapter read deferred`; do
not infer it from a conventional filename or admit another adapter output. Pass
the verified path to the validator and approval dispatch as a phase-owned path.

The validator changes no Design, Contract, or lifecycle path. After its verdict,
it may append a deferred finding only to that same verified destination and must
report the write. After `READY`, the approval dispatch owns the checkpoint for
the Design set, Contract, gate state, and verified deferred destination when
present. The normal clean handoff remains mandatory before Contract Review. No
unreported path, `spec.yaml` before approval, unrelated item, generated output,
or earlier-phase artifact may be dirty. Never mix several Specs' drafts in one
dirty validation handoff.

Once **every participating Spec**, not merely every item in named scope, holds
current Design approval:

5. `specbind-contract-review` — once for the milestone

After the review is accepted:

6. `specbind-plan-tasks` — `tasks.yaml` and its gate, for every in-scope item now
   actionable; parallel in all scope

Gap analysis is not on this path. Run `specbind-gap-analysis` first when
brownfield uncertainty requires it; this Skill does not decide that for you.

## 5. Treat the global barrier as global

Contract Review reads the complete participating Contract graph. Dispatch
`specbind-contract-review` once and honor its outcome. Do not accept it yourself,
compress its findings, or proceed to Tasks without an accepted review.

A single-Spec milestone still has this barrier. In a multi-Spec milestone, named
scope does not narrow it: if another participant lacks current Design approval,
report the outside-scope blocker instead of dispatching that participant or
attempting a review that cannot pass.

## 6. Read the returned status, not the prose

Every dispatch returns a status. **The status decides what happens next.**

Except for the bounded unapproved-Design handoff, success covers the owning
phase's whole contract, including its adapter-directed checkpoint. Require the
status to say whether that checkpoint was committed, intentionally
absent/scaffolded, or failed. Before dependent work, independently run
`git status --short` and require no completed-phase paths to remain dirty. A
fresh gate with uncommitted Requirements, Design, Contract, `tasks.yaml`, or
`spec.yaml` is not a clean handoff. Stop and report it; the orchestrator must not
create a checkpoint owned by the dispatched phase.

| Returned outcome | Handling |
| --- | --- |
| No usable status — missing, ambiguous, or narrative | Re-dispatch **once**, asking only for status |
| **Stopped by design** — no authority, stale upstream gate, user-owned decision | **Do not retry.** Report it as the answer |
| **Failed** — attempted and could not complete | Retry, bounded at **two rounds**, then record it unfinished |

Never infer success because nothing said otherwise.

## 7. Continue only within the selected scope

In all scope, continue as far as reachable work permits:

- unfinished Requirements blocks no other Requirements;
- unfinished Design blocks its dependents;
- any participant unfinished at the barrier prevents Contract Review for all.

Finish reachable in-scope work, then report the blocker. Never drop an unfinished
item from milestone scope to get past the barrier. Partial completion needs no
extra bookkeeping; each Spec holds its current state.

In named scope, stop when the selected item completes or reaches an in-scope or
outside-scope blocker. Do not make progress on unselected items.

## 8. Report

In the project's language, report:

- the selected scope mode and exact items;
- per in-scope item, what was produced and its current state;
- which gates were delegated under `specbind-plan`;
- Design validation and Contract Review outcomes;
- unfinished and outside-scope blockers and the next available action;
- in all scope, Direct items not touched;
- that implementation has **not** started.

## Boundaries

- **Stop after Tasks approval.** Never implement, validate completion, or touch
  release.
- Author nothing yourself and never finish work owned by a phase Skill.
- No scope changes: no Roadmap items, new Specs, removals, or silent expansion.
- Same rules, protocols, and criteria under delegated or explicit approval.
  There is no all-Spec variant of them.
