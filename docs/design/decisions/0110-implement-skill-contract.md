# 0110: Fix the implement skill contract

Status: Accepted

[Decision 0146](./0146-sequential-v1-tasks-and-per-task-checkpoints.md)
supersedes concurrent Task dispatch and places the adapter decision for a
completed Task inside its per-Task cycle.

## Context

[Decision 0075](./0075-v1-skill-and-orchestration-scope.md) fixes what
`specbind-implement` targets: exactly one Roadmap item per invocation,
executing the approved local plan for a Spec-backed item and performing the
scoped work for a Direct item without creating canonical artifacts. It also
fixes the review-mode defaults and bounds automatic debug and remediation at two
rounds.

[Decision 0095](./0095-task-progress-cli.md) exposes `tasks complete`, `block`,
and `reopen`, one task per invocation, and explicitly assigns the request
surface to this skill: it "accepts a group, a set of tasks, or a general
instruction to continue, resolves that against the current plan and actionable
set, and records each task individually."
[Decision 0086](./0086-completion-cli-handshake.md) gives Direct items their own
handshake and states that "the implementation skill performs its run-scoped
checks and semantic judgment" for them.
[Decision 0080](./0080-v1-task-contract-and-completion-details.md) fixes what a
blocked task does to the run.
[Decision 0026](./0026-runtime-implementation-notes.md) gives this skill the
durability judgment for Implementation Notes.
[Decision 0109](./0109-subagent-dispatch-contract.md) fixes fresh-context
dispatch, which this skill uses more than any other.

What remains is the orchestration those decisions assume: which item, in what
order, dispatched how, and where the run stops.

## Three protocols, and why a dispatched role needs one

This skill dispatches three roles, and each needs a semantic baseline the
subagent can actually reach. Decision 0109 fixes the payload as a brief plus a
protocol selector, so this decision adds to the Decision 0094 set:

| Selector | Role |
| --- | --- |
| `task-implementation` | implementing one task from the approved plan |
| `task-review` | the independent verdict on one implemented task |
| `debug` | fresh-context root-cause diagnosis after a failure |

`task-review` and `debug` have two consumers each — this skill dispatching them,
and `specbind-review-task` and `specbind-debug` when a user invokes those
directly — so Decision 0094's allocation test places them in protocols
straightforwardly.

`task-implementation` has one consumer and would otherwise stay skill-local
under that test. It is a protocol anyway, for a reason Decision 0094 could not
have anticipated: **a dispatched role's baseline must be a protocol regardless of
how many skills consume it, because a subagent can reach protocols and cannot be
assumed to reach skill bodies.** Decision 0109 declines to claim dispatched
skill-loading as a platform capability, so the CLI-readable protocol is the only
carrier that always works. Inlining the baseline into the brief instead would
put a non-waivable semantic standard in prose that each dispatch composes
freshly, which is exactly the drift the protocol layer exists to prevent.

## Decision

### One item, and which kind

The skill implements exactly one Roadmap item per invocation, named by the user
or resolved from `milestone status` when only one is actionable. It never walks
the milestone: Decision 0075 leaves v1 without an orchestrator, and the
dependency waves are a read model that per-item runs follow rather than a
schedule this skill executes.

The item's kind decides everything after that:

- **Spec-backed.** The Spec must be in `implementation` with fresh gates. The
  approved plan is the work.
- **Direct.** There is no plan, no Requirements, no Design, and no Contract. The
  Roadmap item's summary and the repository are the work, and the skill creates
  none of those artifacts. If the work turns out to require them, the premise
  that made it Direct has failed: the skill stops and reports that it needs
  rerouting, rather than quietly authoring a Spec's worth of artifacts.

### Prerequisites are checked, not repaired

`spec status` must report `implementation` with requirements, design, and tasks
fresh; `milestone status` must show the item's implementation-phase predecessors
complete. A Spec that is not there is reported and routed to the owning phase.
The skill never approves a gate, never revises the plan, and never edits
Requirements, Design, or the Contract to make a task implementable — a task that
cannot be implemented as written is a finding for the plan or the design, which
the `task-implementation` protocol already fixes as a stop condition.

### Selection follows the read model

Work is selected from `tasks list` and `tasks show`, which already report each
task as actionable or waiting and name the effective prerequisites. The skill
resolves the user's request — a task, a group, a set, or "continue" — against
that actionable set and executes in plan order.

It does not execute a task the read model says is waiting. Decision 0095's
`complete` guard would refuse it anyway; selecting it in the first place means
the run and the plan already disagree.

Concurrency stays opt-in. `parallel: true` permits concurrent execution and
never requires it, and the skill runs sequentially whenever the boundaries are
not clearly disjoint, because Decision 0080 makes path overlap a warning rather
than proof of safety.

### The per-task cycle

Each task is one cycle, and the cycle is not batched. Decision 0095 makes one
recorded completion one judgment; running three tasks and recording three
completions at the end records judgments that were never separately made.

1. **Dispatch a fresh implementer** with a self-contained brief — the task, its
   Requirement IDs, the artifact paths that govern it, its completion criteria,
   the applicable verification commands, and any Implementation Notes bearing on
   its boundary — plus the `task-implementation` selector.
2. **Parse the structured return.** Under Decision 0109 the status comes from a
   closed set and prose is never parsed. An unusable return is re-dispatched
   once, asking only for the status block. The `task-implementation` protocol
   owns the exact block and maps an approved-artifact contradiction to
   `BLOCKED`, so the dispatcher does not invent a fourth status from prose.
3. **Review**, per the run's review mode below.
4. **Record** the outcome through `tasks complete` or `tasks block`.

Before dispatch, the skill discovers every Implementation Notes artifact through
`artifact list` and reads each typed selector; absence is a complete answer.
Implementation Notes are written when the run discovers knowledge that outlives
the task: a non-obvious constraint, a dependency behavior that contradicted the
plan, a trap the next task would otherwise repeat. Decision 0026 makes the
judgment this skill's, and its bar is durability, not activity. A note that
restates what the task did is noise; the plan and Git already record that.
Creation or revision reads `okf-authoring`; a new collection starts from the
default `implementation-notes/main` scaffold only when durable content exists.

### Review mode binds where Decision 0075 put it

`required` is the default for Spec-backed work and `inline` for Direct. Both are
run-scoped, and `off` never disables the later implementation validation or
completion verification.

Under `required`, review is a **fresh dispatch** carrying the `task-review`
selector and the diff, not the implementer's account of it. The value is the
independence: a reviewer that watched the work be justified is not an
independent check on whether it is right. Under `inline`, the same protocol is
applied in the run's own context, which is weaker and is why it is not the
Spec-backed default.

A rejection returns the task to the implementer with the findings. Decision 0075
bounds this at two rounds; after that the task is blocked with the outstanding
findings as its reason. `CANNOT_REVIEW` is not a rejection to retry blindly: it
enters diagnosis with the reason the review subject could not be determined.

Fresh implementers also obey the project-local instruction files that govern
their working directory. Ordinary non-destructive bookkeeping required by those
instructions, such as appending an instrumentation context marker inside the
project, is part of the authorized implementation run and does not require a
second user confirmation merely because it is not product source. Explicit
user/root prohibitions, tool-enforced approval, destructive actions, external
publication, and credential boundaries still win. The dispatch brief states
this so a corrective implementer does not misclassify mandatory local operating
state as a missing product decision.

### Failure has one route, and it is bounded

`BLOCKED`, an unresolved need for context, `CANNOT_REVIEW`, and a review
rejection surviving two rounds all lead to the same place: a fresh dispatch
carrying the `debug` selector, which receives the failure and the inputs and
**not the failed attempts**. That omission is the mechanism under Decision
0109.

Diagnosis is bounded at two rounds under Decision 0075. The diagnosis routes by
its own category: an implementation defect returns to a fresh implementer with
the fix plan; a plan, design, or requirements defect leaves the run entirely and
is reported to the user, because no amount of implementation effort repairs an
artifact that specifies something unworkable. `UNDETERMINED` gathers the named
distinguishing evidence when safely available and otherwise remains explicitly
unrouted; forcing an owner would undo the diagnosis's honesty.

When the rounds are exhausted, the task is recorded blocked with a reason that
names what is unresolved. A blocked task is a legitimate outcome, not a failure
of the run.

### The worktree is never rescued

Decision 0080 fixes the hard part: a blocked task stops the current Spec run
whenever partial source changes remain. Later independent tasks may continue only
when the worktree is clean and the skill confirms no dependency or boundary
conflict.

The skill **never** resets, stashes, reverts, or creates a WIP commit to get out
of that state. Those actions destroy work the user has not seen, and the state
they produce is indistinguishable from work that was never done. It reports the
partial change and stops.

This does not preserve disposable outputs that the current task's own
verification just created. The implementation protocol requires a status
snapshot and a clean handoff: when ownership is certain, the implementer
prevents or removes only its generated caches, coverage data, or reports before
returning `READY_FOR_REVIEW`. The orchestrator does not clean them on the
implementer's behalf, and uncertainty remains a stop rather than authority to
delete.

A task that creates or changes the canonical verification command has a stronger
obligation: the command itself must be repeatably clean. The implementer reruns
the exact public command from a clean snapshot and may not rely on a separate
post-run cleanup that future validators, release checkouts, and users will not
perform. Recreated untracked caches or reports keep the task out of
`READY_FOR_REVIEW` until the command suppresses or owns their disposal.

Committing is adapter-governed under Decision 0101 and is not implied by
completing a task.

### Where the run stops

The two kinds end differently, and the asymmetry is deliberate:

- **Spec-backed.** The run ends when the requested tasks are recorded, or when
  it stops early. It does **not** run the completion handshake: Decision 0086
  routes Spec completion through `specbind-validate-implementation`, and
  Decision 0082 puts a milestone-wide convergence barrier before it. A skill
  that implemented the last task is the worst-placed judge of whether the whole
  Spec is correct, having just spent the run convincing itself of each part.
- **Direct.** The run completes the item, because Decision 0086 assigns the
  Direct handshake to this skill and no Spec-level validation covers it. It
  implements the Roadmap summary in the main context, applies the selected
  review mode to the actual diff, and runs the project checks. With no Task or
  approved Spec artifacts, the Roadmap summary is the review obligation; work
  that needs those artifacts is rerouted through discovery rather than guessed.
  The skill then performs its adapter-governed checkpoint **before**
  `milestone direct preflight <direct>` and the corresponding
  `milestone direct complete` command.

The Direct handshake requires a clean committed `HEAD`. The skill does not
manufacture one: if the adapter gives no usable commit guidance or the run lacks
commit authority, it reports that completion needs a commit and stops. Reading
the adapter only after attempting preflight would make the required revision
unobtainable by construction, so Direct is the one path where checkpoint order
precedes lifecycle completion.

### Boundary

- The skill implements and records progress. It authors no Requirements, Design,
  Contract, or `tasks.yaml`, and approves and invalidates no gate.
- It writes execution state only through the Decision 0095 commands, never by
  editing `tasks.yaml`.
- It performs no Spec completion handshake and accepts no cross-spec review.
- It implements one item, not the milestone.

## Consequences

- The three dispatched roles have reachable baselines, and the rule that made
  `task-implementation` a protocol is stated so the next dispatched role is not
  re-argued from consumer count.
- The per-task cycle keeps one recorded completion equal to one judgment, which
  is the property Decision 0095's guards assume.
- Independent review is a dispatch rather than a mode of the same context, so
  the default review mode is worth more than a self-check.
- Every failure path converges on a bounded, fresh-context diagnosis that routes
  by category, so an artifact defect stops being retried as an implementation
  defect.
- The refusal to rescue a dirty worktree means some runs end needing a human
  decision. That is the intended trade against destroying unseen work.
- Spec completion stays with the skill that validates rather than the skill that
  built, so the convergence barrier has an owner that did not just do the work.

## Implementation status

Implemented. The `tools/specbind/assets/skills/specbind-implement/` package is
embedded and installed. Its entrypoint keeps the shared review, diagnosis, and
authority controls and directly routes the mutually exclusive Spec-backed and
Direct procedures under `references/`. The `task-implementation`, `task-review`,
and `debug` protocols are embedded.

Its forward tests are specified as scenarios I1 through I5 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
