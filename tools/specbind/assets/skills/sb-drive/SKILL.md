---
name: sb-drive
description: Drive the active milestone through every safe reachable planning, implementation, and validation action. Park branch-local attention and continue elsewhere; stop before release execution.
argument-hint: "[--replan] [--target-release <version>]"
---

# Drive the active milestone to its next real boundary

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

Use this Skill when the user asks to drive, continue, or advance the active
milestone as far as possible. A request for one Roadmap item's implementation
still belongs to `sb-implement`.

You are a thin controller. Existing Skills own all artifacts, judgments,
progress, and checkpoints. The CLI owns lifecycle state. Never author their work
or retain a competing schedule.

## Optional replan authority

`--replan`, or an explicit natural-language request to delegate replanning,
authorizes Design, Contract, and Tasks revisions within the active delivery
Milestone's existing scope and approved Requirements. The option itself is
authorization: do not ask for a second delegation confirmation. Without it,
the ordinary approval and rewind confirmations below remain unchanged.

This includes necessary Design/Tasks invalidation, delegated reapproval,
independent Design validation, renewed Contract Review, affected consumers
inside scope, and reimplementation of previously completed work. It never
authorizes changing or invalidating Requirements, changing Milestone scope or
dependencies, changing an out-of-scope Spec, weakening a required behavior, or
Release. An unsettled external compatibility or migration obligation still
needs the user. Reverse adoption and Direct reclassification are not covered.

On a concrete `REROUTABLE` Design/Contract/Tasks finding, dispatch the complete
`sb-plan` owning Skill with the exact affected Specs, finding, `sb-drive`
delegation workflow, authorized Design/Tasks gates and rewinds, and the
installed `references/replan.md` path. This is the explicit recovery exception
to selecting an ordinary status action: status cannot diagnose a semantic
defect in a fresh approved artifact. The planning owner validates the boundary
before mutation. Do not revise artifacts in Drive or dispatch an internal
planner role in place of that Skill.

Carry the original approved Requirements and Milestone scope boundary through
every recovery dispatch; a revised Design cannot expand the delegation.
Report the concrete change and invalidation cost as progress, without waiting
for approval within that boundary. Preserve existing retry budgets and finding
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

## 1. Read the only scheduler

```sh
specbind milestone status --json
```

If there is no active milestone, route to `sb-discovery` and stop. If
state health is inconsistent, report its diagnostics. Do not guess a repair.

Use only the returned ordered `actionable` entries, their typed `handler`,
`action`, and exact `commandOperand`, plus `waitingFor`, current blockers, and
release blockers.
Never parse the Roadmap to compute waves. Keep a run-local set of action keys
already attempted at unchanged state: `<action>:<commandOperand-or-milestone>`.

A pending Direct item is mechanically actionable even when its summary later
proves it was classified too narrowly. Dispatch its exact item and summary to
`sb-implement`; do not pre-classify Roadmap prose in Drive or treat a clean
status as semantic approval of the Direct kind.

## 2. Dispatch one owning workflow at a time

Choose the first safe actionable entry not parked at the current state. Never
reconstruct an owner from `action` or maintain a local action-to-Skill table.

- `handler.kind=skill` names the installed owning Skill in `handler.target`.
  Dispatch a fresh subagent with the exact item, action, handler mode,
  `commandOperand`, applicable authority, project working directory,
  project-local executable and PATH facts, and exact applicable project
  instruction paths. Require that owner to read those instructions and load the
  exact `handler.target` Skill before it delegates any internal role. The
  dispatched workflow then reads its own domain inputs.
- A Skill target is the complete owning workflow package, not a similarly named
  registered internal agent role. Tell the fresh subagent to execute the exact
  `handler.target` Skill through its terminal handoff. For
  `handler.target=sb-implement`, never dispatch `specbind-implementer` or
  `specbind-reviewer` directly: those roles are internal to `sb-implement`,
  which owns review, Task progress, and the checkpoint. An internal
  `READY_FOR_REVIEW` result is not an owning-workflow result and must not return
  control to Drive.
- `handler.kind=guarded_cli` names the accepted guarded command. Run it only
  when this Skill's existing authority boundary permits that exact action.
- `handler.kind=boundary` names the next explicit workflow. Report it and stop;
  never invoke that workflow from Drive.
- An unknown kind, target, or mode is an incompatible product surface. Stop
  rather than guessing a route.

`handler.target=sb-adopt` with `handler.mode=reverse_resume` does not turn a
generic Drive request into reverse Gate authority. Unless the maintainer
explicitly authorized reverse continuation, park it as `HUMAN_DECISION` and
name `sb-adopt` as the continuation owner.

Use one mutating dispatch at a time. Do not launch dependency-wave items in
parallel and do not predict path conflicts.

Planning still owns its delegation confirmation. Invoking Drive without
`--replan` does not grant gate approvals. Pass through only authority the user
explicitly supplied. A missing confirmation is attention, not permission to approve or a reason to
interrupt while unrelated implementation remains reachable.

If `sb-implement` reports that a Direct summary requires canonical artifacts or
an unsettled product or architecture decision, normalize the handoff as
`REROUTABLE` plus `HUMAN_DECISION`. Park it for Discovery and continue every
independent safe action; do not expect milestone status to infer that semantic
classification from the summary.

## 3. Re-read after every handoff

After every dispatch, run:

```sh
git status --short
specbind milestone status --json
```

The fresh status decides what happened. Do not accept the subagent's narrative
as lifecycle evidence.

- If authoritative state advanced and the handoff is safe, clear obsolete
  waits and select again.
- If the attempted action remains unchanged, add it to attention. Do not
  re-dispatch it in this run unless the owning workflow explicitly returned
  `RETRYABLE` within an unspent retry bound.
- Preserve every owning workflow's retry and remediation limit. Drive never
  resets one.
- Except for the attributable same-recovery handoff above, if the worktree
  contains partial, rejected, unrelated, or unattributed work,
  stop the run. Never reset, stash, revert, clean, or manufacture a WIP commit
  to switch items.

## 4. Separate the cause from stopping the run

Normalize a returned or re-read condition to one cause:

- `RETRYABLE` — the same owner may repeat within its existing bound
- `REPAIRABLE` — the owner may apply concrete review findings within its bound
- `REROUTABLE` — an earlier owning phase is required
- `WAITING` — a dependency or global barrier is not satisfied
- `BLOCKED` — the owning surface established that the item cannot progress
- `HUMAN_DECISION` — meaning, scope, authority, or irreversible consequence
  needs the maintainer
- `EXTERNAL_BLOCK` — the environment cannot satisfy a prerequisite
- `COMPLETE` — the delegated boundary is complete

Then make a separate scheduler decision:

- `CONTINUE_ELSEWHERE` when status exposes another independent safe action;
- `STOP_RUN` when no safe reachable action remains or the shared worktree makes
  switching unsafe; or
- `COMPLETE` when status reaches `release_ready`.

`BLOCKED` and `HUMAN_DECISION` do not mean immediate interruption. Park the
affected item, let its descendants wait, and continue another branch. An
unfinished Design prevents Contract Review but not other reachable Design. An
unfinished implementation prevents its descendants and milestone validation
but not independent implementation.

## 5. Keep attention run-local

For every parked condition retain only this run's report data:

- owner and affected item or milestone barrier;
- cause and concise evidence;
- action or decision needed to resume;
- descendants or global barriers it prevents; and
- whether work continued elsewhere.

Create no queue, checkpoint, batch status, or authority artifact. Durable Task
blocks and lifecycle progress remain in their existing owning surfaces. A later
Drive invocation reconstructs everything from fresh status.

Never silently invalidate an approved gate, change Roadmap scope, choose a
release version, reclassify Direct work, accept breaking Contract consequences,
use credentials, perform destructive recovery, or take external action. Those
become `HUMAN_DECISION` attention unless the Design/Contract/Tasks consequence
is already covered by the explicit replan authority above. Requirements
rewinds always retain explicit confirmation; Design and Tasks do so without
replan authority. A Direct item that needs canonical
artifacts returns to Discovery.

## 6. Stop before Release

The successful delivery boundary is `release_ready`. Never dispatch
`sb-release`, execute adapter Prepare/Publish/Verify instructions, or call
release finalization. If the target release is unbound and none was explicitly
supplied, park `bind_release` as a human decision and finish other reachable
work first.

## 7. Report one accumulated handoff

Report in the project's language:

- the milestone and boundary reached;
- owning workflows completed and authoritative state gained;
- every attention item, its cause, and affected descendants or barrier;
- decisions now required, grouped after reachable work is exhausted;
- external blocks and unsafe-worktree details;
- the next safe action, if one exists; and
- that Release execution did not run.

## Boundaries

- Orchestrate only; author no phase or implementation content.
- One mutating owner at a time in the initial implementation.
- Status is the schedule; retained context is not.
- Do not turn Drive into release authority or a persistent workflow engine.
