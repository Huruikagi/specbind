---
type: SpecBind Rule
---

# Task generation

This rule is the project's preferred style for task plans. It is a
`SpecBind Rule`: your project owns this file and may strengthen, relax, replace,
or remove it. Removing it leaves the task workflow intact and only removes this
project's decomposition conventions.

The task-plan schema, positional identifiers, dependency semantics, required
Requirement coverage, approval, and execution state are owned by the CLI
contract and the `task-planning` protocol. Nothing here can relax them.

## Task size

Prefer a task one person can complete and verify in a single sitting. A task
spanning days usually hides several decisions; a task of a few minutes usually
belongs inside its neighbor.

Split when a task has more than one reason to fail. Combine when two tasks can
never be reviewed independently.

## Decomposition order

Sequence work so that what unblocks other work comes first. In practice that
often means:

1. foundations others depend on: shared types, schema, configuration, test scaffolding
2. the primary behavior the requirements describe
3. wiring the pieces together across boundaries
4. verification that spans the whole change

This is a default, not a required shape. A change that is mostly integration, or
mostly one behavior, should not be padded into four phases to match it.

## Describing a task

State the outcome, not the file layout. The Design already owns the mechanism,
and repeating it in the plan creates a second description that drifts.

Prefer domain language over structural language: "reject an expired token"
rather than "add a check to the middleware". The implementer reads the Design
for the second part.

## Completion detail

Add explicit completion criteria when finishing the task is not obvious from its
statement. Prefer one or two observable conditions over a checklist that
restates the task.

When this project has a convention for what "done" always includes — updated
tests, updated documentation, a passing check — state it once here rather than
repeating it in every task.

## Test work

By default, write tests as part of the task that introduces the behavior. Split
verification into its own task only when it spans behavior delivered by several
earlier tasks or forms a separately reviewable system boundary. Do not create a
second task merely to restate the completion criteria of one implementation
task.

Mixing both conventions without that boundary makes coverage hard to see. A
project that prefers another convention should replace this paragraph and name
any deliberate exceptions here.

## Parallelization

Mark work parallel only when it is genuinely independent; the product baseline
lists the conditions. Being conservative costs elapsed time, while being wrong
costs correctness, so prefer sequential when uncertain.

When this project has areas that are known to conflict — generated files, a
shared migration sequence, a lock file — list them here so planners stop
rediscovering them.

## Review questions

- Could someone else pick up this task from the plan and the Design alone?
- Does each task leave the system in a state worth committing?
- Is the order the real dependency order, or the order they were thought of?
- Does any pair of parallel tasks touch the same place?
