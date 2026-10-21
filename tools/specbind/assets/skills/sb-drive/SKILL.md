---
name: sb-drive
description: Drive the active milestone through every safe reachable planning, implementation, and validation action. Park branch-local attention and continue elsewhere; stop before release execution.
argument-hint: "[--replan|--no-replan] [--target-release <version>] [--recovery bounded|progress]"
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

A Drive request limited to one recovery action, such as "only withdraw stale
completion and stop", still uses this controller and a separately dispatched
owner. The limit narrows the work; it does not make Drive the validation owner.
Reading another Skill for routing does not transfer ownership into this context.
Read the applicable replan procedure and dispatch its isolated complete owner
even when no implementation or final validation is authorized afterward.

A request to drive the active milestone authorizes ordinary completion
recording on `GO` by the validation owner, including its Git-adapter checkpoint.
Pass that Drive authority with the exact actionable Spec set; no additional
completion confirmation is needed. An explicit user limit, such as stopping
before completion recording or making no changes, overrides this authority.
Park excluded actions as `HUMAN_DECISION`; never turn a read-only claim check
into a Drive request. Planning authority depends on the resolved replan setting
below; Drive never grants Release authority.

## Resolve project Drive defaults

Before scheduling each new invocation or human-authorized resumption, run
`specbind configuration show`. Any error stops the workflow; never silently
fall back from invalid configuration. Resolve each setting independently:
explicit current-request options or clear natural-language limits override
the root `.specbind.json` `drive` member, then omitted fields fall back to
`replan: false` and `recovery: bounded`. New installations explicitly save
`replan: true` and `recovery: progress`; existing omitted fields keep the legacy
fallback. Accept `--replan`, `--no-replan`, and `--recovery bounded|progress`.
These are Skill options, not a `specbind` CLI flag. Clarify invalid/conflicting
explicit choices before corrective work. Report the effective settings and
their sources (request, configured value, or legacy fallback) before dispatch.

Only the current request's choices override configuration. A previous mode
quoted in a historical recovery handoff is evidence, not a current option or
an instruction to preserve that mode in a new invocation. Likewise, "no new
allowance" preserves counts, not selection. "No scope expansion" preserves
Drive's existing scope boundary and does not disable in-scope replan; an actual
prohibition on revising planning artifacts does disable it.

When explaining the current project's Drive defaults in a read-only request,
also run `specbind configuration show`; reading raw JSON alone is not validated
configuration. This read grants no execution authority. Pure hypothetical
values without a claim about the current project need no project mutation.

Snapshot both resolved settings and provenance for this run. Generic "continue"
does not itself select progress, but a new Drive invocation uses its configured
default. A clear request to continue without a cycle limit while progress is
independently verified selects `progress`, which removes only the numerical
Spec-backed Task cap. A config edit, automatic redispatch or session change
does not alter a running selection or renew an allowance.

Distinguish a fresh human Drive invocation from an automatic receiver of the
same invocation. The fresh invocation's resolved settings replace the prior
selection even when it carries an existing recovery allowance. Keep that
allowance's identity, used count and findings unless the qualifying resumption
below renews it. For example, a new invocation resolving bounded with three
cycles already used cannot start cycle four while blockers remain. An automatic
receiver instead preserves the invoking controller's snapshot. Tell owners
which boundary occurred; retaining history never means ignoring a new invocation's
resolved setting.

Pass both resolved settings, selection provenance, actual user request and scope to every affected
`sb-implement` owner in this active Milestone, including after replan or a
capacity restart. Carry existing ledgers unchanged. Receivers continuing the
same run retain its evidenced mode even when no flag is repeated. Implementation
owns progress judgment and mode changes; Drive never infers progress from a
success message or resets counts. `progress` still stops on an unresolved or
reopened prior finding, missing continuity or external/authority boundaries.
It does not change Direct, Design or Contract Review bounds or authorize
execution from a read-only request. A new invocation rereads defaults; this
never discards history or independently renews an allowance. Standalone
`sb-implement` does not read Drive defaults.

## Carry an explicit Task resumption

After a reported Task recovery-limit stop, carry a later explicit human request
to resume that work to `sb-implement`, together with the prior recovery handoff.
Include the actual user instruction and its target, allowance identity/current
counts if already assigned, recovery mode and its selection evidence, and
historical counts/findings/evidence separately.
The implementation owner applies its recovery procedure to start at most one new
allowance for that instruction (uncapped only with resolved progress selection);
Drive never resets counters itself. Contextual
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

When resolved `replan` is true, read [Authorized replan routing](references/replan.md)
completely before scheduling work. An actual Drive execution request with that
configured default grants the same bounded authority as `--replan`; do not ask
for it again. `--no-replan` or an explicit prohibition overrides the default.
Configuration alone grants no execution or read-only override.
Without that authority, do not load the procedure and
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

Every new owning workflow starts without inheriting Drive's conversation or any
previous owner's context. Select the host's no-history option explicitly
(Codex `fork_turns: "none"` when exposed); a new receiver name with inherited
history is not fresh. Send a self-contained brief with the facts below and any
required recovery evidence, rather than forking the conversation. If the host
cannot provide an isolated ordinary owner, return `EXTERNAL_BLOCK` before
dispatch; do not substitute inherited history or execute the owner yourself.
This applies to ordinary scheduling, replan, completion withdrawal, explicit
Task resumption and capacity restarts alike.

After a terminal handoff, the next owning workflow gets a new receiver. Never
reuse a completed receiver through resume, follow-up or message tools, even for
the same Spec or the same Skill. In particular, Plan's receiver must not become
the implementation owner, and a withdrawal owner must not become the final
validation owner. Carry authority, exact revisions, findings, Task mappings and
budget counts in the new brief; isolation never discards or resets that evidence.

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
exception requires the same Skill, action, item and handler mode, with no
terminal handoff yet; it never permits a switch from planning to implementation
or from withdrawal to final validation. A terminal `WAITING` for milestone
convergence is a completed handoff, not unfinished internal-role progress. This
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

Planning still owns its delegation confirmation when resolved replan is false.
Pass through only authority supplied
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

- `RETRYABLE` — the same owning workflow may repeat within its existing bound;
  a new dispatch after its terminal handoff still requires a fresh receiver
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
