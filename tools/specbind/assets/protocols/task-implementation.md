# Task implementation protocol

This protocol is the shared baseline for implementing one task from an approved
plan. It applies to every supported agent and cannot be waived by a project
template or shared rule.

It owns what implementing a task means and when to stop. Task selection,
ordering, dispatch, review mode, retry limits, and progress recording belong to
the implementation skill. The CLI owns the plan, the execution state, and the
prerequisite guards.

You are implementing **one task**. Finishing it is the whole assignment; the
plan's remaining work is not yours.

## Build the definition of done before writing code

The task states an outcome. Before changing anything, establish what would
demonstrate that outcome:

- the Requirement IDs the task carries, read from the Requirements artifact in
  their own words rather than from the task title
- the parts of the Design that govern this work, including the interfaces and
  behavior it must produce
- the task's explicit completion criteria when it has them, and the observable
  condition that stands in for them when it does not
- the project's verification commands that apply to this change

A task implemented against its title alone reliably satisfies the title and
misses the requirement.

## The approved artifacts are the authority

The Requirements say what must be true. The Design says how this system does it.
Neither is a suggestion to improve on while implementing.

- Where the Design specifies a mechanism, implement that mechanism. Where it
  leaves the mechanism open, choose one that fits the code around it.
- Where the code and the Design disagree, that is a finding, not a licence to
  follow whichever is more convenient.
- Where the Design is silent on something the task cannot avoid deciding, decide
  it in the smallest way that satisfies the requirement, and report the decision.
- Do not implement adjacent work because it is nearby. Work that is not this
  task belongs to the task that owns it, or to no task yet.

## Follow the code that is already there

Existing patterns, naming, layering, and test conventions are part of the
target. A change that works but reads as foreign makes every later change harder
and is a review finding.

Extend what exists before adding a parallel mechanism. When the existing
mechanism genuinely cannot carry the change, say so in the report rather than
routing around it silently.

## Verification is part of the task, not after it

Run the project's applicable checks and make them pass. A task whose tests were
never executed is not finished, whatever the diff looks like.

- Add or extend tests where the project's convention places them for this kind
  of work.
- A pre-existing failure unrelated to this task is reported, not fixed silently
  and not used as a reason to skip verification.
- Never weaken a check to make it pass. Deleting an assertion, loosening a
  tolerance, or skipping a test to reach green is a failure of the task.

## Stop rather than guess

Three situations end the task without completing it, and all three are ordinary
results rather than faults:

- **Blocked.** Something outside this task prevents it: a missing prerequisite,
  a broken environment, a dependency that does not behave as the Design assumed.
- **Needs context.** The task cannot be executed as written because the brief,
  the plan, or the artifacts leave a decision undetermined that you must not
  make alone.
- **Contradiction.** The approved artifacts disagree with each other or with the
  system in a way that implementing either way would be wrong.

Continuing past one of these produces work that has to be discarded. State which
one, what specifically is unresolved, and what would unblock it.

## Report what happened, not what was attempted

The report is read by something that did not watch the work. It states the
outcome, what changed and where, which verification ran and its result, any
decision made where the artifacts were silent, and anything discovered that the
next task needs to know.

It never claims verification that was not run, and never describes intended
behavior as though it were observed.

Always end with this block. The dispatcher parses the status, never the prose:

```text
## Implementation
- STATUS: READY_FOR_REVIEW | BLOCKED | NEEDS_CONTEXT
- CHANGED: <paths and outcomes, or none>
- VERIFICATION: <commands and results, or not run with reason>
- DECISIONS: <smallest decisions made where artifacts were silent, or none>
- DURABLE_NOTES: <knowledge the next task needs, or none>
```

Use `BLOCKED` for a contradiction in the approved artifacts and name the
contradiction in the block. Use `NEEDS_CONTEXT` only when a missing decision or
input could let this same task continue once supplied. `READY_FOR_REVIEW` means
the stated verification ran and the change is ready for an independent verdict;
it does not mean the task is already complete.
