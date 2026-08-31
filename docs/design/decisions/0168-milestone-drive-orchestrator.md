# 0168: Add a milestone-wide drive orchestrator

Status: Accepted

## Context

Decision 0075 deliberately left v1 without a milestone-wide implementation
orchestrator. Since then, `specbind-plan` has gained a complete named and
all-Spec scheduler through Tasks approval, `specbind-implement` has fixed a
one-item and sequential per-Task contract, and `specbind milestone status
--json` has become the authoritative project-wide read model. The status model
already reports typed actionable work for Requirements, Design, Contract
Review, Tasks, Implementation, validation, release binding, and release
preflight with exact command operands where needed.

The missing product behavior is therefore not another lifecycle state machine.
It is a user-facing controller that can keep selecting existing owning
workflows, re-read authoritative state after each handoff, and continue through
independent work when one branch needs attention.

A generic `BLOCKED` result is insufficient for that controller. A blocked Task,
a dependency wait, a missing human decision, and an unavailable external
prerequisite have different owners and propagation scopes. None necessarily
means that every other reachable Roadmap item must stop.

## Decision

### One milestone-wide entry point

SpecBind adds the accepted product-managed Skill name `specbind-drive`. It
drives the active milestone as far as authoritative state, existing workflow
contracts, and current authority safely permit in one invocation.

The Skill:

- starts from `specbind milestone status --json` and treats its state health,
  actionable list, command operands, dependency waits, and blockers as the
  mechanical authority;
- selects one safe reachable action at a time and delegates it to the existing
  owning Skill or guarded CLI workflow;
- re-reads milestone status after every delegated workflow and never schedules
  from retained conversational assumptions;
- may enter at any active milestone stage and continues until the milestone
  reaches `release_ready` or no safe reachable action remains; and
- reports work completed, attention items, dependency effects, and the exact
  boundary reached.

`specbind-drive` owns orchestration only. It authors no Requirements, Design,
Contract, Tasks, implementation, validation evidence, release content, or
lifecycle state itself. It defines no second progress format and never infers a
transition that the status and owning workflow do not expose.

### Existing workflows remain the owners

The driver composes the current public surface:

- `specbind-plan` owns milestone planning through Tasks approval, including its
  existing named and `--all` scope, phase dispatch, review, gate, and delegation
  contracts;
- `specbind-implement` owns exactly one Spec-backed or Direct Roadmap item and
  preserves its sequential per-Task implementation, review, progress, debug,
  and checkpoint cycle;
- `specbind-validate-implementation` owns one Spec's final implementation
  validation and completion handshake; and
- the guarded milestone CLI owns target-release binding and the status read
  model.

The driver does not execute `specbind-release`, project Prepare, Publish,
Verify, or core finalization. Reaching `release_ready` is its successful delivery
boundary. Release remains a separate explicit externally consequential
workflow.

### Cause and scheduler disposition are separate

A delegated workflow or authoritative re-read may identify these run-scoped
causes:

- `RETRYABLE` — the same owning operation may be repeated within its existing
  bounded retry contract;
- `REPAIRABLE` — the owning workflow may apply concrete evaluator or reviewer
  findings within its existing remediation contract;
- `REROUTABLE` — an earlier owning phase is required;
- `WAITING` — a predecessor or global barrier is not yet satisfied;
- `BLOCKED` — the owning surface has recorded or established that the item
  cannot currently progress;
- `HUMAN_DECISION` — meaning, scope, responsibility, authority, or an
  irreversible consequence requires a maintainer decision;
- `EXTERNAL_BLOCK` — the current environment cannot satisfy a prerequisite; and
- `COMPLETE` — the delegated boundary is complete.

These causes do not directly decide whether the driver stops. After every
re-read, the driver chooses one separate disposition:

- `CONTINUE_ELSEWHERE` when another independent safe action remains;
- `STOP_RUN` when no safe reachable action remains or a shared-resource
  condition makes switching unsafe; or
- `COMPLETE` when the delivery boundary has been reached.

Owning workflows retain their existing retry and remediation limits. The driver
does not reset a spent attempt budget, parse prose to manufacture a retry, or
re-dispatch an unchanged action indefinitely.

### Attention set and graph propagation

The driver keeps a run-local attention set for `WAITING`, `BLOCKED`,
`HUMAN_DECISION`, and `EXTERNAL_BLOCK` results. Adding an item to that set does
not immediately interrupt the user while another independent safe action is
reachable.

Each attention entry reports:

- the owning workflow and affected Roadmap item or milestone barrier;
- the cause and concise evidence;
- the direct action or decision needed to resume;
- descendants or barriers that cannot become reachable because of it; and
- whether the driver continued elsewhere or stopped.

Propagation follows the authoritative graph:

- an item-local cause parks that item and makes its Roadmap descendants wait;
- an unsatisfied Design branch prevents the global Contract Review barrier but
  does not prevent other reachable Design work;
- an incomplete implementation branch prevents its descendants and the global
  validation barrier but does not prevent independent implementation items;
- a release-binding decision does not prevent earlier delivery work; and
- an unsafe shared worktree or another repository-wide handoff failure stops
  the run because switching items would lose reliable ownership of changes.

The driver asks for accumulated human decisions when no independent safe action
remains, rather than interrupting at the first decision encountered. A user may
also explicitly request an earlier checkpoint and attention report.

### Authority does not expand through orchestration

Invoking `specbind-drive` authorizes selection and delegation across the active
milestone. It does not by itself grant gate approval, gate invalidation, scope
mutation, release-version choice, destructive recovery, external publication,
credential use, or acceptance of breaking Contract consequences.

Existing run-scoped delegated gate authority may be supplied to the owning Plan
workflow. Without applicable authority, the transition becomes a
`HUMAN_DECISION` attention item while independent work continues. Requirements,
Design, or Tasks invalidation still uses its existing explicit confirmation and
rewind contract. A Direct item that proves to require canonical Spec artifacts
still returns to Discovery and scope confirmation; the driver never performs
that reclassification silently.

### Initial execution is sequential and re-entrant

The accepted first implementation dispatches only one mutating owning workflow
at a time. This makes repository ownership and the existing per-Task checkpoint
contract the conflict-control mechanism. Dependency-wave parallelism,
predictive path-conflict analysis, and a persistent concurrency setting are not
part of the first implementation.

The driver is re-entrant. It creates no driver checkpoint, queue, batch status,
or persistent authority artifact. Durable progress and blockers remain in the
existing owning surfaces. A later invocation reconstructs reachable work and
attention from authoritative status and fresh owning-workflow diagnosis rather
than trusting a previous conversation or driver report.

## Consequences

- The user-facing capability is broader than the original implementation-only
  Issue framing, while the existing phase and per-item contracts remain intact.
- A local block delays only the affected subgraph; the driver stops only after
  safe reachable work is exhausted or a global safety condition prevents
  switching.
- The initial sequential controller needs no repository conflict predictor and
  no new CLI lifecycle mutation.
- `milestone status --json` is the mechanical scheduler input, while semantic
  decisions and mutation authority stay with the owning workflows.
- The attention report is a run result, not a new source of truth. A future
  persistent driver queue or parallel scheduler requires a separate Decision.

## Implementation status

Implemented by the embedded `specbind-drive` package, installation registry,
project routing instructions, mechanical tests, generated reference index, and
behavioral forward-test scenario. Issue #9 retains the design history.
