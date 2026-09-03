# Implement a Spec-backed item

## Check the prerequisite

```sh
specbind spec status <spec>
```

It must report `State: implementation` with `requirements=fresh, design=fresh,
tasks=fresh`. If not, stop and say so. Route to the owning phase. **Never**
approve a gate, revise the plan, or edit requirements, design, or the contract
to make a task implementable. A task that cannot be implemented as written is a
finding about the plan or the design — that is a real outcome, not an obstacle.

## Select the work

```sh
specbind tasks list <spec>
specbind tasks show <spec> <task-id>
specbind artifact list <spec>
```

Read every `implementation-notes/<artifact-id>` selector the inventory names:

```sh
specbind artifact read <spec> implementation-notes/<artifact-id> --for consume
```

The notes are optional. An inventory with none is a complete answer; do not
materialize an empty placeholder.

The read model already marks each task `pending actionable` or `pending
waiting` and names its effective prerequisites. Resolve what the user asked for
— a task, a group, a set, or "continue" — against the actionable set, and work
in plan order.

**Do not start a task the read model says is waiting.** `tasks complete` would
refuse it anyway; picking it means your plan and the real plan already disagree.

Task execution is sequential. Finish one task's implementation, review, recorded
outcome, durable notes, and checkpoint decision before selecting the next task.
The plan order is the dependency order; do not dispatch several tasks into one
shared worktree at once.

## The per-task cycle

**One task per cycle. Do not batch.** One recorded completion is one judgment;
running three tasks and recording three completions at the end records judgments
you never separately made.

### Dispatch a fresh implementer

Use the registered `specbind-implementer` role when the host provides it;
otherwise dispatch an ordinary fresh subagent with the same brief and protocol.
The role selects capability, not scope or authority.
Fallback is only for an absent role. If a registered role exists but its
configured model cannot start, report a configuration or environment failure;
do not silently change its capability. This applies to every registered role in
this run.

Give it a brief that stands alone — it saw nothing you saw:

- the task: title, details, completion criteria
- the requirement IDs it carries, and the artifact paths to read them from
- the design artifacts that govern this work
- the project's applicable verification commands
- any implementation notes bearing on this task's boundary
- the project-local instruction files it must obey, including that required
  non-destructive bookkeeping inside the project is ordinary task execution and
  does not need a second user approval

and the protocol it must read:

```sh
specbind protocol read task-implementation
```

After dispatch, let the implementer finish its work and verification and return
the protocol's structured status. **Do not interrupt it, ask for an immediate
return, or turn a progress check into a stop request.** Waiting for the result is
part of the dispatch. An implementer that is forced to return before verification
leaves a partial change that this workflow must treat as blocked.

### Parse the status block, never the prose

Require a structured result with a status from a closed set:
`READY_FOR_REVIEW`, `BLOCKED`, `NEEDS_CONTEXT`.

The exact block is fixed by the protocol. If it is missing, ambiguous, or
replaced with narrative, **re-dispatch once asking only for the block.** Never
infer success because nothing said otherwise.

- `READY_FOR_REVIEW` → review it
- `NEEDS_CONTEXT` → re-dispatch once with what it asked for; still unresolved →
  diagnose
- `BLOCKED` → diagnose

Before accepting `READY_FOR_REVIEW`, compare the worktree with the snapshot
from before dispatch. New caches, reports, coverage data, or other verification
leftovers outside the intended `CHANGED` paths make the result not ready. Send
the exact generated paths back to a fresh implementer within the normal retry
limit so it can prevent their creation or remove only outputs created by this
task. The orchestrator never deletes them itself. If generated ownership is not
certain, diagnose and stop rather than cleaning a possibly unrelated path.

### Review

Default is `required` for Spec-backed work, `inline` for Direct. `--review
required|inline|off` is run-scoped, and plain requests like "skip review" mean
`off`. When ambiguous, keep the default.

**`required` means a fresh dispatch.** Give the reviewer the diff and the
artifact paths — not the implementer's account of what it did — and:

Use the registered `specbind-reviewer` role when available, with an ordinary
fresh subagent as the fallback.

```sh
specbind protocol read task-review
```

The value is independence. A reviewer that watched the work be justified is not
a check on whether it is right.

`inline` applies the same protocol in your own context. It is weaker, which is
why it is not the default for Spec-backed work.

Verdicts are `APPROVED`, `REJECTED`, or `CANNOT_REVIEW`. A rejection goes back
to a fresh implementer with the findings — **at most two rounds**. After that
the task is blocked, with the outstanding findings as the reason.

`CANNOT_REVIEW` is not a rejection to retry blindly. It enters diagnosis with
the reason the subject could not be judged.

### Record

```sh
specbind tasks complete <spec> <task-id>
specbind tasks block <spec> <task-id> --reason <reason>
```

One task per invocation. Never edit `tasks.yaml` to record progress.

### Note what outlives the task

Write implementation notes when the run found something the **next** agent needs:
a non-obvious constraint, a dependency that behaved unlike the plan assumed, a
trap that would otherwise be repeated.

The bar is durability, not activity. A note restating what the task did is noise
— the plan and Git already have that.

**A diagnosis that found something durable is recorded here, by you.** The debug
run writes nothing, and its report is run-scoped: what it learned about a trap
survives only if you write it down. That is how the next task avoids the same
issue instead of rediscovering it.

Before creating or rewriting the managed Markdown, read its authoring contract:

```sh
specbind protocol read okf-authoring
```

Update the applicable discovered notes artifact in place. When none exists and
the knowledge is durable enough to justify one, start from the default scaffold:

```sh
specbind template read spec implementation-notes/main
```

Follow every `create output=<name>` instruction once to produce its named output.
An output may be a short string or a Markdown fragment. Replace every reference
to that name with the same produced output. Materialize it only with real
content. The filename is a locator, not identity;
do not guess an existing notes path or create a second `artifact_id` for the same
concern. Omit `create` instructions and copy `maintain` and `consume`
instructions unchanged. If notes already exist and you revise them, read them
with `--for maintain` and preserve their durable comments.

### Checkpoint the completed task

Only a task recorded `completed` is an eligible implementation checkpoint. Read
the project policy now, before selecting another task:

```sh
specbind adapter read git --for consume
```

`NO_CHANGE ADAPTER_ABSENT` or `NO_CHANGE ADAPTER_SCAFFOLD` means there is no
adapter-directed commit: say so in one line and commit nothing. Neither result
turns into a request for new policy.

When the adapter has guidance, follow it for **this task now**. The request to
perform this mutating phase authorizes that narrow local checkpoint as the
ordinary final step of this task's cycle. Stage only:

- the deliberate implementation and test paths produced for this task
- the `tasks.yaml` execution-state change produced by `tasks complete`
- Implementation Notes created or revised from this task's durable finding

Never include another task, unrelated work, completion metadata, rejected work,
or partial implementation. An explicit user or root instruction that forbids
commits wins, and tool permissions still apply. Commit guidance is not push
guidance; never force-push or rewrite history. If the paths cannot be separated
safely, stop before the Git operation and report the completed task as
uncommitted.

Record whether the checkpoint committed, was intentionally absent or inactive,
or failed. Then re-read `tasks list` before selecting the next task. Never defer
several eligible Task checkpoints to the end of the run.

## Finish and stop

When the requested task outcomes are recorded, the implementation work is done.
Every completed task has already crossed its own checkpoint decision.

Report and stop. **Do not run the completion handshake.** `spec completion
preflight` and `accept` belong to `sb-validate-implementation`, and a
milestone-wide convergence barrier comes before them.

This is deliberate. You just spent the run convincing yourself each part was
right, which makes you the worst-placed judge of whether the whole Spec is.
