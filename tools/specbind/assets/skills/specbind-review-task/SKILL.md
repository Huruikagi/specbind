---
name: specbind-review-task
description: Judge whether one implemented task is done correctly, from the actual diff and the approved requirements, design, and plan.
argument-hint: "<spec> <task-id>"
---

# Review one task

Verdict on **one task**: is it done correctly?

You read. You do not fix, do not implement, and do not record anything.

## 1. Read the change first

```sh
git diff
git status --short
```

**The diff is what happened. The report is what someone believes happened.** If
you were given an implementer's summary, treat it as a claim to check, not as
the subject of the review. A report and a diff that disagree is itself a
finding, and the diff is what is true.

A review that restates the report has reviewed nothing — and it is worse than no
review, because the verdict it produces will be trusted.

## 2. Read what the task was supposed to do

```sh
specbind tasks show <spec> <task-id>
specbind artifact list <spec>
specbind artifact read <spec> requirements
```

The inventory names every split Design and every
`implementation-notes/<artifact-id>` selector. Read all Designs that govern the
task and all Implementation Notes — a recorded trap may be exactly what this
change walked into:

```sh
specbind artifact read <spec> design/<artifact-id>
specbind artifact read <spec> implementation-notes/<artifact-id>
```

Read the requirement IDs the task carries **in the requirements' own words**,
not through the task title. An inventory with no notes is a complete answer.

Then apply the standard:

```sh
specbind protocol read task-review
```

## 3. Scope yourself to this task

If the working tree holds changes this task does not own, do **not** guess which
hunks belong to it and do not review the rest silently.

That is what `CANNOT_REVIEW` is for. A verdict on an unknown subject is worth
less than saying the subject could not be determined.

## 4. Return the verdict

Always return this block, whether a person or another run is reading it:

```text
## Review
- VERDICT: APPROVED | REJECTED | CANNOT_REVIEW
- FINDINGS:
  - [BLOCKING|DEFERRED|RESOLVED] <requirement or behavior at risk> — <where> — <consequence>
```

Every finding carries a disposition. `APPROVED` with an undisposed finding
attached is not a verdict, and a finding with no disposition is one nobody
carries.

Every `REJECTED` names what would make it approvable. Rank by what changes the
verdict: wrong behavior, an unmet requirement, an unhandled case the requirement
covers, and **weakened verification** are rejections. A deleted assertion, a
loosened tolerance, or a skipped test is a rejection unless the change genuinely
made that check obsolete and says so.

Say what is right when it is true. A review that only accumulates objections
leaves the next attempt rewriting work that was already correct.

Uncertainty is never an approval.

## 5. Record deferred findings

A deferred finding needs the destination this project names, or it is not
deferred — it is dropped, and the next review raises its successor as blocking
to keep that from happening again.

```sh
specbind adapter read deferred
```

`NO_CHANGE ADAPTER_ABSENT` means the project has no destination. Say so in one
line and record nothing. Do not invent a place to put it.

Unlike the Git adapter, the installed scaffold carries a working default, so
follow its guidance as written unless the project emptied or replaced it. Write
only what the adapter says to write. Read the destination only far enough to
avoid recording the same finding twice; nothing in it is a source of work for
you, and no entry there becomes work until a person puts it on the Roadmap.

## Boundaries

- **Never fix what you find.** Repairing the change destroys the thing under
  review and leaves the implementer holding a verdict on work it did not write.
- Never run `tasks complete`, `tasks block`, or any gate command. Recording a
  task is the implementer's judgment, not a consequence of your verdict.
- Never write implementation notes. Durable knowledge goes in your findings; the
  run that applies the fix records it.
- Review one task. Work belonging to another task is worth naming and is not a
  reason to reject this one.
