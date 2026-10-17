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

A request to drive the active milestone authorizes ordinary completion
recording on `GO` by the validation owner, including its Git-adapter checkpoint.
Pass that Drive authority with the exact actionable Spec set; no additional
completion confirmation is needed. An explicit user limit, such as stopping
before completion recording or making no changes, overrides this authority.
Park excluded actions as `HUMAN_DECISION`; never turn a read-only claim check
into a Drive request. This grants no planning Gate approval or Release authority.

## Carry an explicit Task resumption

After a reported Task recovery-limit stop, carry a later explicit human request
to resume that work to `sb-implement`, together with the prior recovery handoff.
Include the actual user instruction and its target, allowance identity/current
counts if already assigned, and historical counts/findings/evidence separately.
The implementation owner applies its recovery procedure to start at most one new
allowance for that instruction; Drive never resets counters itself. Contextual
"continue" or "I updated it; resume" needs no special reset wording or second
confirmation. Read-only checks, automatic redispatch, compaction, sessions and
version updates alone do not renew it. This routing applies without `--replan`
and grants no additional planning authority or unsafe-worktree exception.

For that exact reported recovery-limit stop, a blocked Task may have no ordinary
actionable implementation entry. After reading current Milestone and Task state,
dispatch the complete `sb-implement` owner for the named item and Task with the
resumption evidence, even in that case. This narrow continuation route does not
create a scheduler: do not select other blocked or waiting Tasks. Preserve only
attributable partial work for this same Task; conflicting or unknown changes
still stop the run. The owner verifies prerequisites, handles reopening through
the CLI, and applies independent recovery assessment before correction.

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
Only the evidenced completion-withdrawal route in [Authorized replan
routing](references/replan.md), with supplied replan authority and retained
before/after evidence, may recover the narrowly identified inconsistency before
ordinary scheduling. The flag alone is not evidence of its cause.

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
  registered internal agent role. Host every owning Skill in an ordinary fresh
  subagent (Claude Code: `general-purpose`; Codex: the ordinary/default agent),
  never in a registered internal role such as `specbind-planner`,
  `specbind-implementer`, or `specbind-reviewer`. If an ordinary owner host is
  unavailable, return `EXTERNAL_BLOCK`; do not substitute an internal role.
  Tell the fresh subagent to execute the exact
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

### Keep converged Spec validation at one revision

When the first safe entry targets `sb-validate-implementation` for ordinary
completion validation, collect all currently actionable entries with that same
target and mode, in returned order. Dispatch the complete Skill **once for that
explicit Spec set**, passing each exact entry and completion authority. This is
one owning workflow, not parallel per-Spec dispatch. Never include a parked
entry, infer extra targets from the Roadmap, or mix withdrawal-only recovery
with validation. Mark each included action key attempted on return.

The owner preflights every target at the same clean HEAD, assesses each Spec,
and accepts only after all have `GO`, then checkpoints the accepted metadata
set together. Never request a separate acceptance commit per Spec. Drive does
not collect candidate evidence, synthesize verdicts, accept completion, or make
that checkpoint itself. Preserve the owner's per-Spec findings and partial
acceptance handoff; reread authoritative state before scheduling again.

If the owner reports `BLOCKED` because HEAD changed or the worktree became
dirty before acceptance, stop this Drive run after one Git/status reread.
Preserve its target set, preflight and observed revisions, dirty paths and
restart instruction. Do not retry or switch owners even if status exposes the
same validation action or the new HEAD is clean. A later user invocation of
Drive rereads current actionable entries and dispatches a fresh ordinary
validation owner to preflight and assess the entire current set again; prior
candidates and `GO` verdicts cannot be reused. Drive never commits or cleans
the intervening changes to make this restart possible.

### Wait for the owning workflow to finish

Require each owner to keep its turn active while its internal roles run and to
return only at its workflow's stopping point. Prefer foreground dispatch; a
background receiver still belongs to this Drive turn. Wait for its terminal
handoff before interpreting lifecycle state or selecting another owner.

"Waiting", "started", and internal-role progress are non-terminal updates. If
an addressable owner ends its turn with only such an update, continue that same
receiver once to collect its running roles and finish the same workflow. This
does not grant authority or reset any retry budget. Never start a replacement
owner over its partial work or take over its internal roles. If that continuation
still returns no terminal handoff, or the receiver cannot be addressed, report
`EXTERNAL_BLOCK` with the receiver identity, last progress, and exact partial
paths after one Git/status reread. Preserve any known live receiver in the
handoff and stop; do not imply the worktree is safe for another mutating owner.
An actual capacity stop follows the separate contract below.

### Preserve dispatch-capacity handoffs

The exact `handler.target` owning Skill is the first and only receiver you may
attempt for this action. Before it starts, do not dispatch a capacity probe,
approval or readiness reviewer, diagnosis helper, or any internal role. Keep
Drive-owned mechanical preflight in this context; owner-domain inspection
begins only inside the complete owner after it receives the required project
cwd, executable, PATH, and instruction facts.

On a host capacity failure, or an owner-returned capacity restart handoff, read
[Dispatch-capacity recovery](references/capacity.md) completely before retrying,
releasing a receiver, selecting other work, or returning the handoff. It owns the
bounded recovery, exact restart capsule, and unsafe-worktree stop. Do not load
it during ordinary successful dispatch.

`handler.target=sb-adopt` with `handler.mode=reverse_resume` does not turn a
generic Drive request into reverse Gate authority. Unless the maintainer
explicitly authorized reverse continuation, park it as `HUMAN_DECISION` and
name `sb-adopt` as the continuation owner.

Use one mutating dispatch at a time. Do not launch dependency-wave items in
parallel and do not predict path conflicts.

Planning still owns its delegation confirmation. Invoking Drive without
`--replan` does not grant gate approvals. Pass through only authority supplied
by the user's request, including ordinary completion authority from a Drive
request as defined above. A missing confirmation is attention, not permission to approve or a reason to
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
