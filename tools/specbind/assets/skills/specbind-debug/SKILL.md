---
name: specbind-debug
description: Use directly when the user asks why a Task failed or cannot be implemented. Establish the root cause, categorize it, and return a next action. Read-only; never starts implementation or applies the fix.
argument-hint: "<failure>"
---

# Diagnose one failure

The **diagnosis is the deliverable**. Someone else applies the fix, from a
context that did not watch this failure happen.

## Final response contract — before any investigation

Your final response is incomplete unless it ends with this exact parseable
shape. Reserve it now and fill it from the evidence:

```text
## Diagnosis
- CATEGORY: IMPLEMENTATION | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED
- CAUSE: <what diverges, and where>
- NEXT_ACTION: <for whoever owns that category>
- UNCERTAIN: <what remains open, or none>
```

Naming a category in prose does not satisfy this contract. Do not rename the
heading, omit a field, or return only a narrative diagnosis.

```sh
specbind protocol read debug
```

## Resolve the subject

Use the explicit failure, Spec, and Task identity supplied by the caller. When a
bare diagnosis request omits them, do not infer identity from a repository path
or from whichever Task you notice first.

```sh
specbind milestone status
specbind tasks list <spec>
```

List Tasks for every active Spec in implementation. For a request that says a
Task failed or cannot be implemented, candidates are blocked Tasks and pending
actionable Tasks. Continue only when exactly one candidate exists across the
active milestone. Otherwise present the canonical candidates and ask the user
which failure to diagnose. The request is not evidence that an arbitrary
candidate failed.

## Change nothing

Read-only means read-only. You may run commands that reproduce or observe the
failure; you may **not** modify any tracked file, and you may not apply the fix
you find.

This is evidentiary, not procedural. A repository you have already edited leaves
the next agent unable to tell which state your reasoning describes, and destroys
the evidence a second round would need.

## When the context is not fresh

You work best dispatched into a clean context, given the failure and the inputs
and **not** the history of attempts that failed. That omission is the mechanism:
a retry that inherits the reasoning which just failed reliably reproduces it.

**If you were invoked inside the session that already failed, say so.** Then:

- name what you are deliberately setting aside — the conclusions this session
  already reached, the theories it already committed to;
- re-derive from the evidence in front of you rather than from where the session
  had got to;
- when the failure has already survived several attempts here, say plainly that
  a fresh session would produce a more trustworthy diagnosis.

Running as though the context were clean is the failure mode this skill exists
to break.

## Establish the cause

Read what the system was supposed to do, not only what it did:

```sh
specbind spec status <spec>
specbind tasks show <spec> <task-id>
specbind artifact list <spec>
specbind artifact read <spec> requirements --for consume
specbind artifact read <spec> contract --for consume
specbind steering list
```

The inventory names split Designs and every
`implementation-notes/<artifact-id>` selector. Read all that govern the failure:

```sh
specbind artifact read <spec> design/<artifact-id> --for consume
specbind artifact read <spec> implementation-notes/<artifact-id> --for consume
```

Steering has no relevance metadata. Read every selector the listing returns:

```sh
specbind steering read <selector> --for consume
```

Zero Steering documents is a complete answer. If the artifact inventory,
Contract read, Steering listing, or any Steering read fails, do not infer the
missing content. Return `UNDETERMINED` and make the failed read the evidence step
that must succeed before the cause can be owned.

An inventory with no notes is a complete answer. When notes exist, a recorded
trap is often exactly the cause.

The cause is where actual behavior **first diverges** from what the approved
artifacts require. Everything after that point is consequence, and the error
message names where something surfaced, not where it went wrong.

If two causes remain possible, say both and say what would distinguish them. A
confident single answer that is wrong costs more than an honest fork.

## Return the diagnosis

End with the exact block reserved above:

```text
## Diagnosis
- CATEGORY: IMPLEMENTATION | PLAN | ARTIFACT | ENVIRONMENT | UNDETERMINED
- CAUSE: <what diverges, and where>
- NEXT_ACTION: <for whoever owns that category>
- UNCERTAIN: <what remains open, or none>
```

The category decides who fixes it, and misrouting is expensive:

- **IMPLEMENTATION** — the code does not do what the design requires. Back to
  the task.
- **PLAN** — the task, its ordering, or its prerequisites are wrong. Back to the
  task plan.
- **ARTIFACT** — the requirements or design specify something unworkable, or
  contradict each other. **No amount of implementation effort fixes this**, and
  handing it back as an implementation defect produces repeated attempts at work
  that cannot succeed.
- **ENVIRONMENT** — the system is not in the state the work assumes. Usually
  outside the change entirely.
- **UNDETERMINED** — the evidence does not yet establish an owner. Name the
  evidence-gathering step that would distinguish the remaining possibilities.

Describe what must become true rather than dictating a diff, unless the exact
edit is itself the finding. The implementer has context you do not.

**Say when you cannot diagnose it.** Report what you ruled out and what evidence
would be needed. That is a useful result; a guess presented as a cause sends the
next round in a direction chosen by nothing, and looks exactly like a real
finding until it fails.

## Boundaries

- Write nothing. No fix, no implementation notes, no task state, no gate.
- Diagnose one failure and return. Counting rounds belongs to the run that
  dispatched you.
- Report in the project's language, with the block above intact.
