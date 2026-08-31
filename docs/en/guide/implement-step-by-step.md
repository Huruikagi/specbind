# Plan and implement one item at a time

This guide takes one Spec-backed item from Requirements through implementation
validation while you review each artifact and Gate separately. Use it for a
first SpecBind cycle, a high-impact change, or a targeted rerun of one phase.

Discovery must already have created the Milestone and target Spec. If no
Milestone is active, begin with the [new-project](./start-new-project.md) or
[existing-project](./start-existing-project.md) route.

The examples use `csv-export` as the Spec ID. Codex invokes Skills with `$`;
Claude Code uses `/`.

## 1. Confirm the current boundary

```text
$specbind-status
```

Review the Roadmap classification, dependencies, current Gates, and next safe
actions. This guide is for a Spec-backed item. A Direct item has no
Requirements, Design, Contract, or Tasks and goes directly to
`specbind-implement <item-id>`.

## 2. Author Requirements

```text
$specbind-plan-requirements csv-export
```

Requirements describe the complete current behavior of the Spec, not only this
Milestone's diff. When the Brief declares Source Items, confirm that accepted
content has been promoted into authoritative Requirements and Acceptance
Criteria.

Review the result and approve the Requirements Gate. Resolve open product
decisions here instead of hiding them in ambiguous wording to move forward.

## 3. Author and validate Design and Contract

```text
$specbind-plan-design csv-export
```

Design explains how the Requirements will be realized. Contract structures
outward responsibilities, dependencies, and owned file boundaries. After
authoring, an independent validation checks coverage, responsibility boundaries,
and verifiability. Approve the Design Gate only after validation returns
`READY`.

Repair concrete Design findings and validate again. If the defect is in the
Requirements, return to their owning phase instead of compensating in Design.

## 4. Review Contracts across the Milestone

```text
$specbind-contract-review
```

Contract review is Milestone-wide, not a review of only the named Spec. Every
active-Milestone Design must be ready so the review can find ownership overlap,
cycles, compatibility assumptions, and integration gaps.

If another Spec's Design is unfinished, bring it through step 3 first. For a
multi-Spec Milestone, the combined route in
[Plan and Drive a Milestone](./implement-with-plan-and-drive.md) is usually
more convenient.

## 5. Author Tasks

```text
$specbind-plan-tasks csv-export
```

Tasks turn Requirements and Design into an executable order. Review each Task's
scope, completion condition, verification, and Requirement coverage before
approving the Tasks Gate.

Tasks approval completes planning. It does not begin implementation.

## 6. Implement one Roadmap item

```text
$specbind-implement csv-export
```

Implement owns exactly one Roadmap item. For a Spec-backed item, it processes
Tasks sequentially and completes implementation, review, verification, CLI
progress recording, and the project adapter's checkpoint for each Task before
moving on.

If implementation exposes a Requirements, Design, Contract, or Tasks defect,
Implement does not rewrite that upstream artifact. Fresh diagnosis identifies
the owner, the affected Gate is explicitly invalidated, and work resumes
through the owning phase.

## 7. Validate the complete Spec

After every Task is complete, validate the Spec as a whole:

```text
$specbind-validate-implementation csv-export
$specbind-status csv-export
```

Validation evaluates the implementation against current Requirements and
Design. A `GO` result and accepted completion evidence complete the Spec-backed
item and may make dependent Roadmap items actionable.

## 8. Choose the next boundary

Repeat the process for another unfinished item. You may switch to
`specbind-drive` at this point; Drive reconstructs work from current CLI state
and does not redo completed phases.

Completion validation does not run Release. Publication and Milestone
finalization remain a separate explicit workflow in
[Release a milestone](./release.md).

## When to use this route

- Learn the role of each artifact and Gate during the first cycle.
- Review product and Design decisions at separate boundaries.
- Isolate a high-impact Spec from other Milestone work.
- Resume narrowly from the owning phase after invalidation.

When a Milestone has several independent items and you want all safely reachable
work to advance, use [Plan and Drive a Milestone](./implement-with-plan-and-drive.md).

---

[Core concepts](./concepts.md) | [Plan and Drive a Milestone](./implement-with-plan-and-drive.md) | [Release a milestone](./release.md)
