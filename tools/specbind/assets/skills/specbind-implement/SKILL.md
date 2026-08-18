---
name: specbind-implement
description: Implement one roadmap item — executing a Spec's approved task plan with dispatched implementers and reviewers, or performing and completing a Direct change.
argument-hint: "<item> [tasks] [--review required|inline|off]"
---

# Implement one item

**One roadmap item per run.** Not the milestone. There is no orchestrator in v1;
dependency waves are something you read, not something you execute.

## 1. Find out which kind of item this is

```sh
specbind milestone status
```

The kind decides the whole run:

- **Spec-backed** — there is an approved task plan, and it is the work.
- **Direct** — there is no plan, no requirements, no design, no contract. The
  item's summary and the repository are the work.

**Never create canonical artifacts for a Direct item.** If the work turns out to
need requirements, design, or a contract, the premise that made it Direct has
failed. Stop and report that it needs rerouting through discovery.

## 2. Check the prerequisites

For a Spec-backed item:

```sh
specbind spec status <spec>
```

It must report `State: implementation` with `requirements=fresh, design=fresh,
tasks=fresh`. For either kind, `milestone status` must show this item's
implementation-phase predecessors complete.

If not, stop and say so. Route to the owning phase. **Never** approve a gate,
revise the plan, or edit requirements, design, or the contract to make a task
implementable. A task that cannot be implemented as written is a finding about
the plan or the design — that is a real outcome, not an obstacle.

## 3. Select the work (Spec-backed)

```sh
specbind tasks list <spec>
specbind tasks show <spec> <task-id>
```

The read model already marks each task `pending actionable` or `pending
waiting` and names its effective prerequisites. Resolve what the user asked for
— a task, a group, a set, or "continue" — against the actionable set, and work
in plan order.

**Do not start a task the read model says is waiting.** `tasks complete` would
refuse it anyway; picking it means your plan and the real plan already disagree.

`parallel: true` permits concurrency, never requires it. Run sequentially unless
the boundaries are clearly disjoint — path overlap is a warning, not proof of
safety, and sequential costs time while concurrent conflict costs correctness.

## 4. The per-task cycle

**One task per cycle. Do not batch.** One recorded completion is one judgment;
running three tasks and recording three completions at the end records judgments
you never separately made.

### a) Dispatch a fresh implementer

Give it a brief that stands alone — it saw nothing you saw:

- the task: title, details, completion criteria
- the requirement IDs it carries, and the artifact paths to read them from
- the design artifacts that govern this work
- the project's applicable verification commands
- any implementation notes bearing on this task's boundary

and the protocol it must read:

```sh
specbind protocol read task-implementation
```

### b) Parse the status block, never the prose

Require a structured result with a status from a closed set:
`READY_FOR_REVIEW`, `BLOCKED`, `NEEDS_CONTEXT`.

If the block is missing, ambiguous, or replaced with narrative, **re-dispatch
once asking only for the block.** Never infer success because nothing said
otherwise.

- `READY_FOR_REVIEW` → review it
- `NEEDS_CONTEXT` → re-dispatch once with what it asked for; still unresolved →
  diagnose
- `BLOCKED` → diagnose

### c) Review

Default is `required` for Spec-backed work, `inline` for Direct. `--review
required|inline|off` is run-scoped, and plain requests like "skip review" mean
`off`. When ambiguous, keep the default.

**`required` means a fresh dispatch.** Give the reviewer the diff and the
artifact paths — not the implementer's account of what it did — and:

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

### d) Record

```sh
specbind tasks complete <spec> <task-id>
specbind tasks block <spec> <task-id> --reason <reason>
```

One task per invocation. Never edit `tasks.yaml` to record progress.

### e) Note what outlives the task

Write implementation notes when the run found something the **next** agent needs:
a non-obvious constraint, a dependency that behaved unlike the plan assumed, a
trap that would otherwise be repeated.

The bar is durability, not activity. A note restating what the task did is noise
— the plan and Git already have that.

**A diagnosis that found something durable is recorded here, by you.** The debug
run writes nothing, and its report is run-scoped: what it learned about a trap
survives only if you write it down. That is how the next task avoids the same
issue instead of rediscovering it.

## 5. When something fails

`BLOCKED`, unresolved `NEEDS_CONTEXT`, and a rejection surviving two rounds all
go to the same place: **a fresh dispatch that receives the failure and the
inputs, and not the failed attempts.**

```sh
specbind protocol read debug
```

Withholding the attempt history is the mechanism, not an oversight. A retry that
inherits the reasoning that just failed reproduces it.

**At most two diagnosis rounds.** Route by the category the diagnosis returns:

- **implementation defect** → fresh implementer with the fix plan
- **plan, design, or requirements defect** → **leave the run.** Report it to the
  user. No amount of implementation effort repairs an artifact that specifies
  something unworkable.
- **environment or dependency** → usually outside the change; report it

When the rounds are spent, record the task blocked with a reason naming what is
unresolved. A blocked task is a legitimate outcome.

## 6. Never rescue the worktree

A blocked task stops this Spec's run whenever partial source changes remain.
Later independent tasks may continue **only** when the worktree is clean and you
have confirmed no dependency or boundary conflict.

**Never `git reset`, stash, revert, or create a WIP commit to get out of that
state.** Those destroy work the user has not seen, and what they leave behind is
indistinguishable from work that was never done. Report the partial change and
stop.

Committing is the project's call, not a consequence of finishing a task — see
the checkpoint step.

## 7. Where the run ends

### Spec-backed: stop at the tasks

When the requested tasks are recorded, report and stop. **Do not run the
completion handshake.** `spec completion preflight` and `accept` belong to
`specbind-validate-implementation`, and a milestone-wide convergence barrier
comes before them.

This is deliberate. You just spent the run convincing yourself each part was
right, which makes you the worst-placed judge of whether the whole Spec is.

### Direct: complete the item

There is no Spec-level validation for Direct work, so this run finishes it. Run
the project's checks yourself, then:

```sh
specbind milestone direct preflight <direct>
specbind milestone direct complete <direct> --implementation-revision <revision>
```

Preflight needs a **clean committed `HEAD`**. Do not manufacture one. If the
worktree is dirty because the adapter told you not to commit, say completion
needs a commit and stop.

## 8. Checkpoint, if the project asks

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means the project wants no commit from you. Stop
there — that is an answer, not a missing file to work around.

The same applies when the adapter still carries its `specbind:instruction`
comments: that is the scaffold as installed, not policy the project wrote. Treat
it as no guidance, say so in one line, and commit nothing.

When the adapter has guidance, follow it. It sets **policy, not permission**:

- It grants no authority by existing. The user's request, the root agent
  instructions, and your tool permissions still decide what you may do.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

## Boundaries

- Implement and record progress. Author no requirements, design, contract, or
  `tasks.yaml`; approve and invalidate no gate.
- Write execution state only through `tasks complete|block|reopen`.
- Run no Spec completion handshake, and accept no contract review.
- One item per run.
- Report in the project's language: which tasks were completed, which were
  blocked and why, what was reviewed and its verdict, any notes written, whether
  the work was committed, and what runs next.
