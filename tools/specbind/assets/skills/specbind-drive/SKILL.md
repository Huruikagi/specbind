---
name: specbind-drive
description: Drive the active milestone through every safe reachable planning, implementation, and validation action. Park branch-local attention and continue elsewhere; stop before release execution.
argument-hint: "[--target-release <version>]"
---

# Drive the active milestone to its next real boundary

Use this Skill when the user asks to drive, continue, or advance the active
milestone as far as possible. A request for one Roadmap item's implementation
still belongs to `specbind-implement`.

You are a thin controller. Existing Skills own all artifacts, judgments,
progress, and checkpoints. The CLI owns lifecycle state. Never author their work
or retain a competing schedule.

## 1. Read the only scheduler

```sh
specbind milestone status --json
```

If there is no active milestone, route to `specbind-discovery` and stop. If
state health is inconsistent, report its diagnostics. Do not guess a repair.

Use only the returned ordered `actionable` entries, their `action` and exact
`commandOperand`, plus `waitingFor`, current blockers, and release blockers.
Never parse the Roadmap to compute waves. Keep a run-local set of action keys
already attempted at unchanged state: `<action>:<commandOperand-or-milestone>`.

## 2. Dispatch one owning workflow at a time

Choose the first safe actionable entry not parked at the current state. Dispatch
a fresh subagent with the exact item, action, applicable authority, and the
installed owning Skill. The dispatched workflow reads its own inputs.

| Status action | Owner |
| --- | --- |
| `requirements`, `design`, `tasks` | `specbind-plan` with explicit all-Spec milestone scope |
| `contract_review` | `specbind-contract-review` |
| `implementation` | `specbind-implement <commandOperand>` |
| `validation` | `specbind-validate-implementation <commandOperand>` |
| `bind_release` | guarded milestone binding, only with an explicitly supplied target release |
| `release_preflight` | report the status-derived release boundary; do not invoke Release |

Use one mutating dispatch at a time. Do not launch dependency-wave items in
parallel and do not predict path conflicts.

Planning still owns its delegation confirmation. Invoking Drive does not grant
gate approvals. Pass through only authority the user explicitly supplied. A
missing confirmation is attention, not permission to approve or a reason to
interrupt while unrelated implementation remains reachable.

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
- If the worktree contains partial, rejected, unrelated, or unattributed work,
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
become `HUMAN_DECISION` attention. Requirements, Design, or Tasks rewinds retain
their existing explicit confirmation. A Direct item that needs canonical
artifacts returns to Discovery.

## 6. Stop before Release

The successful delivery boundary is `release_ready`. Never dispatch
`specbind-release`, execute adapter Prepare/Publish/Verify instructions, or call
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
