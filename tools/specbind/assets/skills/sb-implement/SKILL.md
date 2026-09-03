---
name: sb-implement
description: Implement one roadmap item — executing a Spec's approved task plan with dispatched implementers and reviewers, or performing and completing a Direct change. Do not use for a diagnosis-only request asking why work failed or cannot be implemented.
argument-hint: "<item> [tasks] [--review required|inline|off]"
---

# Implement one item

## Apply project language style

Before authoring any artifact or user-facing prose, read:

```sh
specbind rule read language-style --for consume
```

Apply returned policy only to natural-language prose. `NO_CHANGE RULE_ABSENT`
means no additional project preference; any `ERROR` line stops the workflow.

**One roadmap item per run.** Not the milestone. There is no orchestrator in v1;
dependency waves are something you read, not something you execute.

The user's request must match a pending Spec-backed or Direct item in the active
Roadmap. A bare change request is not a Direct item, even when it names the exact
source file and tells you to edit it. If no active Roadmap or matching pending
item exists, stop before reading or changing implementation and route the
request through `sb-discovery`.

## 1. Find out which kind of item this is

```sh
specbind milestone status
```

The kind decides the whole run:

- **Spec-backed** — there is an approved task plan, and it is the work.
- **Direct** — there is no plan, no requirements, no design, no contract. The
  item's summary and the repository are the work.

Resolve review mode here, before loading the procedure. An explicit `--review`
value wins; otherwise the default is `required` for Spec-backed work and
`inline` for Direct work. “Selected mode” in either procedure means this resolved
value, not a setting to search for elsewhere.

**Never create canonical artifacts for a Direct item.** If the work turns out to
need requirements, design, or a contract, the premise that made it Direct has
failed. Stop and report that it needs rerouting through discovery.

For either kind, `milestone status` must show this item's implementation-phase
predecessors complete. If not, stop and say so. Route to the owning phase.

## 2. Load only the item's procedure

Read the directly applicable reference completely before changing anything:

- For a Spec-backed item, read [Spec-backed implementation](references/spec-backed.md).
  It owns the approved-plan prerequisite, Task selection, sequential per-Task
  cycles, progress, notes, completed-Task checkpoints, and stopping point. Do not
  load the Direct procedure.
- For a Direct item, read [Direct implementation](references/direct.md). It owns
  implementation and review against the Roadmap summary, its checkpoint, and
  the Direct completion handshake. Do not load the Spec-backed Task cycle.

## 3. When something fails

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

## 4. Never rescue the worktree

A blocked task stops this Spec's run. It is not an eligible completed-Task
checkpoint, and partial source changes are never committed as a workaround.

**Never `git reset`, stash, revert, or create a WIP commit to get out of that
state.** Those destroy work the user has not seen, and what they leave behind is
indistinguishable from work that was never done. Report the partial change and
stop.

Removing only disposable outputs known to have been created by this task's own
verification is part of producing the clean handoff required by the
task-implementation protocol; it is not permission to rescue pre-existing or
unrelated work.

Committing remains the project's call — the completed-task checkpoint step reads
and follows that policy inside each cycle.

## Boundaries

- Implement and record progress. Author no requirements, design, contract, or
  `tasks.yaml`; approve and invalidate no gate.
- Write execution state only through `tasks complete|block|reopen`.
- Run no Spec completion handshake, and accept no contract review.
- One item per run.
- Report in the project's language: which tasks were completed, which were
  blocked and why, what was reviewed and its verdict, any notes written, whether
  the work was committed, and what runs next.
