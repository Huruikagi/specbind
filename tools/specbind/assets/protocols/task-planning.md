# Task planning protocol

This protocol is the shared baseline for turning an approved Design into an
executable task plan. It applies to every supported agent and cannot be waived
by a project template or shared rule.

It owns the judgment the plan must embody. The task-plan schema, positional ID
assignment, dependency validity, active-Requirement coverage, and execution
state are validated by the CLI. Task sizing conventions, decomposition
granularity, and how test work is grouped are project preferences. Approval and
regeneration belong to the tasks skill.

## Every task is work that will be done

The plan contains no optional, aspirational, or nice-to-have entries. A task in
the plan is a commitment that the change is not complete until it is done.

Work that might be worth doing later belongs in a future change, not in this
plan as an entry someone is expected to skip.

## Coverage is delivery, not mapping

Every active Requirement ID must be delivered by the plan. The CLI verifies that
each one is referenced; the planner is responsible for the part it cannot check:

- The referenced tasks, taken together, must actually produce the behavior the
  requirement describes. A requirement attached to a task that only partially
  addresses it is uncovered in every way that matters.
- A requirement that needs setup, integration, and verification is covered when
  all of that work exists in the plan, not when one task mentions its ID.

## Coverage runs both ways

The active Requirement set is what the plan must deliver and also the limit of
what it may deliver. The plan is milestone-local: it exists to produce this
change and is deleted when the milestone closes.

A task whose Requirement IDs are all outside the active set is therefore work
this change was never asked for, or a sign that the active set is missing
something it should contain. The CLI reports it, and the fix is one of those two
answers rather than a reference added to satisfy the check.

Referencing a Requirement the task genuinely touches alongside an active one is
ordinary. Referencing an active Requirement a task does not actually serve, to
clear the report, makes the plan lie about what delivers what.

## Tasks state outcomes

A task states the capability or behavior to achieve and the conditions under
which it is complete.

- Describe the functional work. The Design already owns the mechanism, and
  duplicating file layouts or signatures into the plan creates a second source
  that drifts as soon as implementation begins.
- Each task must be executable as bounded work by someone who reads it together
  with the Design. A task that requires knowledge held only by its author is not
  a task yet.
- Each task must build on what precedes it and connect to the system. Work that
  produces something nothing else uses is either misordered or unnecessary.
- Progress incrementally. A task that jumps several steps at once cannot be
  reviewed or safely retried.

## Completion detail where "done" is not obvious

State explicit completion criteria whenever finishing the task is not decidable
from its statement alone.

Criteria must be checkable by someone other than the implementer. "Works
correctly" is not a criterion; the observable condition that demonstrates it is.
When the task statement already makes completion unambiguous, adding criteria is
noise.

## Order carries the dependencies

Execution order is the primary dependency mechanism: a task may rely on
everything before it having been done.

- Declare an explicit dependency only when order does not already express it,
  typically when a task depends on specific earlier work in a different part of
  the plan, or when the relationship is not evident from position.
- Do not restate a dependency that ordering already implies. An
  over-annotated plan obscures the few relationships that are real.
- Sequence work so that what unblocks others comes first. A plan whose order
  contradicts its real dependencies is misleading even when every explicit
  declaration is correct.

Task identity is positional. Inserting, removing, or reordering tasks renumbers
the ones that follow, and execution state and completion evidence reference
those numbers. Restructuring an approved plan is legitimate; doing it for
cosmetic reasons is not.

## Boundaries make parallel safety decidable

A task declares the area it touches so that overlap can be seen rather than
guessed. Boundaries are what turn parallel-safety from an opinion into a check.

Mark a task parallel-capable only when all of the following hold:

- it has no data or output dependency on work that is still pending
- it touches no file or mutable resource that a concurrent task also touches
- everything it requires is already complete, not merely scheduled earlier
- its declared boundary does not overlap a concurrent task's boundary
- it can be verified independently, without another task's work being present

If any condition is uncertain, the task is not parallel. Sequential execution of
work that could have been concurrent costs time; concurrent execution of work
that conflicts costs correctness. State the blocking relationship explicitly
rather than leaving the reader to wonder why two similar tasks differ.

## Readiness

A plan is ready when every active Requirement is genuinely delivered, each task
is executable bounded work with decidable completion, the order reflects the
real dependencies, explicit declarations cover exactly the relationships order
does not, and every parallel marking survives all five conditions above.

Schema validity and complete requirement references are preconditions, not
evidence of any of the above.
