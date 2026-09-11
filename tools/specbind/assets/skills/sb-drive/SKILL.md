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

When `--replan` is present, or the maintainer explicitly requests delegated
replanning, read [Authorized replan routing](references/replan.md) completely
before scheduling work. The option or explicit request is itself the authority;
do not ask for it again. Without that authority, do not load the procedure and
retain the ordinary approval and rewind confirmations below.

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

A pending Direct item is mechanically actionable once any required shared
Contract Review is fresh, even when its summary later
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

### Preserve dispatch-capacity handoffs

When a `handler.kind=skill` receiver cannot start because the host's finite
agent or thread capacity is full:

1. Consume completed Drive-owned receiver results first. If the host exposes a
   safe release operation, release only a completed receiver whose result was
   consumed and which this Drive run will not continue. Never release an
   unrelated receiver or invent a release operation.
2. Continue an addressable receiver only when that already-started run lacks
   its terminal status, and only for the same owning Skill, action, item, and
   handler mode. Never repurpose it as another owner or internal role.
3. If the owner never started and capacity remains unavailable, record
   `EXTERNAL_BLOCK`, reread Git and milestone state once, and do not retry that
   unchanged dispatch in this run. Do not execute owner work in the Drive
   context, change the handler target, or ask the user to manage receiver slots.

For an owner that never started, retain a restart capsule with these exact
fields:

- `Handler target and mode:`
- `Action, command operand, and item:`
- `Supplied authority:`
- `Project cwd, executable, and PATH facts:`
- `Capacity evidence:`
- `Git state:`
- `Resume owner:`

Invent no finding ledger or retry budget for work that did not start.

When an owning Skill returns its own capacity restart handoff, preserve that
handoff **verbatim** as owner-owned continuation state. Append the independent
`git status --short` and milestone-status evidence; do not replace the owner's
exact paths, finding IDs and dispositions, used or remaining budgets, supplied
or omitted authority, resume role, or execution facts with a summary. A
mechanical contradiction blocks the handoff instead of authorizing repair.

If the returned handoff names unapproved or partial paths, the shared worktree
is unsafe: stop after the reread and include the complete owner handoff in the
final report. A clean pre-owner capacity block may continue only to another safe
action whose handler does not need the unavailable receiver capacity. A later
Drive run cannot infer non-durable finding, budget, or authority facts from
status; it needs the prior restart capsule when the owner requires them.

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
- Except for the attributable same-recovery handoff defined by [Authorized
  replan routing](references/replan.md), if the worktree
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
is already covered by [Authorized replan routing](references/replan.md).
Requirements rewinds always retain explicit confirmation; Design and Tasks do
so without replan authority. A Direct item that needs canonical artifacts
returns to Discovery.

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
- every owner-returned capacity restart handoff verbatim, or the complete
  pre-owner restart capsule;
- the next safe action, if one exists; and
- that Release execution did not run.

## Boundaries

- Orchestrate only; author no phase or implementation content.
- One mutating owner at a time in the initial implementation.
- Status is the schedule; retained context is not.
- Do not turn Drive into release authority or a persistent workflow engine.
