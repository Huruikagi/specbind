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

`parallel: true` permits concurrency, never requires it. Run sequentially unless
the boundaries are clearly disjoint — path overlap is a warning, not proof of
safety, and sequential costs time while concurrent conflict costs correctness.

## 4. The per-task cycle

**One task per cycle. Do not batch.** One recorded completion is one judgment;
running three tasks and recording three completions at the end records judgments
you never separately made.

### a) Dispatch a fresh implementer

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

### b) Parse the status block, never the prose

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

### c) Review

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

Before creating or rewriting the managed Markdown, read its authoring contract:

```sh
specbind protocol read okf-authoring
```

Update the applicable discovered notes artifact in place. When none exists and
the knowledge is durable enough to justify one, start from the default scaffold:

```sh
specbind template read spec implementation-notes/main
```

Materialize it only with real content. The filename is a locator, not identity;
do not guess an existing notes path or create a second `artifact_id` for the same
concern. Omit `create` instructions and copy `maintain` and `consume`
instructions unchanged. If notes already exist and you revise them, read them
with `--for maintain` and preserve their durable comments.

## 5. When something fails

`BLOCKED`, unresolved `NEEDS_CONTEXT`, `CANNOT_REVIEW`, and a rejection surviving
two rounds all go to the same place: **a fresh dispatch that receives the
failure and the inputs, and not the failed attempts.**

Use the registered `specbind-debugger` role when available, with an ordinary
fresh subagent as the fallback. Never substitute the implementer or reviewer
role: fresh diagnostic judgment is the reason this dispatch exists.

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
- **undetermined** → gather the named evidence when it is safely available;
  otherwise report what remains open rather than assigning it to an owner

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

Removing only disposable outputs known to have been created by this task's own
verification is part of producing the clean handoff required by the
task-implementation protocol; it is not permission to rescue pre-existing or
unrelated work.

Committing is the project's call, not a consequence of finishing a task — see
the checkpoint step.

## 7. Finish the implementation work

### Spec-backed: finish at the tasks

When the requested task outcomes are recorded, the implementation work is done.
Proceed to the checkpoint step, then stop as Section 9 says.

### Direct: implement and review

There is no approved task plan to dispatch. Implement the Roadmap item's summary
in this context, against the repository's existing conventions. Before writing,
state the observable done condition and the applicable project checks. If the
summary leaves a product or architecture decision you cannot make narrowly,
stop and route it through discovery; Direct is not permission to invent the
missing canonical artifacts.

Review the resulting diff under the run's selected mode. `inline` applies the
correctness and weakened-verification standard from `task-review` here, using
the Roadmap summary as the obligation. `required` dispatches a fresh reviewer
with that summary, the actual diff, the checks, and the protocol; `off` skips
only this run-scoped review. A rejection may return to implementation at most
twice. `CANNOT_REVIEW` and an unresolved rejection enter the same bounded
diagnosis route as Spec-backed work.

When `required`, use the registered `specbind-reviewer` role when available,
with an ordinary fresh subagent as the fallback.

```sh
specbind protocol read task-review
```

## 8. Checkpoint

This step runs after the implementation work above. For a Direct item it must
therefore establish the clean committed revision **before** Section 9 runs the
completion handshake. Do not skip ahead and return here afterwards.

```sh
specbind adapter read git
```

`NO_CHANGE ADAPTER_ABSENT` means there is no adapter-directed commit. Stop
there — that is an answer, not a missing file to work around.

An adapter carrying the exact `<!-- specbind:adapter-scaffold -->` marker is an
inactive scaffold, not project policy. Treat it as no guidance, say so in one
line, and commit nothing. The marker classifies the whole document: ignore every
other body line even when it looks actionable.

When the adapter has guidance, follow it. The request to perform this mutating
phase authorizes the adapter's narrow local checkpoint as its ordinary final
step. It does not authorize anything broader:

- An explicit user or root instruction that forbids commits wins, and tool
  permissions still apply.
- Commit guidance is not push guidance. Push only where the adapter says to, and
  never force-push, rewrite history, or bypass a protected branch.
- Stage only the paths this run produced. Unrelated work already in the worktree
  is left exactly as it is.
- Stop before the Git operation if the guidance is ambiguous, unsafe, or
  conflicts with something else you were told.

## 9. Where the run ends

### Spec-backed: stop after the checkpoint

Report and stop. **Do not run the completion handshake.** `spec completion
preflight` and `accept` belong to `specbind-validate-implementation`, and a
milestone-wide convergence barrier comes before them.

This is deliberate. You just spent the run convincing yourself each part was
right, which makes you the worst-placed judge of whether the whole Spec is.

### Direct: complete after the checkpoint

There is no later Spec-level validation for Direct work, so this run finishes
the item. Preflight needs the reviewed implementation at a **clean committed
`HEAD`**. Do not manufacture one. If Section 8 produced no commit because the
adapter gave no usable guidance or you lacked authority, say completion needs a
commit and stop; the Roadmap item remains pending.

Otherwise obtain the committed revision and run, in this order:

```sh
specbind milestone direct preflight <direct>
specbind milestone direct complete <direct> --implementation-revision <revision>
```

Do not stop merely because the implementation commit succeeded. The successful
handshake is what records the Direct item complete. If project policy also asks
for lifecycle-state checkpoints, apply it once more to the CLI-owned Roadmap
change after completion.

## Boundaries

- Implement and record progress. Author no requirements, design, contract, or
  `tasks.yaml`; approve and invalidate no gate.
- Write execution state only through `tasks complete|block|reopen`.
- Run no Spec completion handshake, and accept no contract review.
- One item per run.
- Report in the project's language: which tasks were completed, which were
  blocked and why, what was reviewed and its verdict, any notes written, whether
  the work was committed, and what runs next.
