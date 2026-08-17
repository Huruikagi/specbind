---
type: SpecBind Rule
---

# Design principles

This rule is the project's preferred style for technical design documents. It is
a `SpecBind Rule`: your project owns this file and may strengthen, relax,
replace, or remove it. Removing it leaves the design workflow intact and only
removes this project's conventions.

Synthesis, simplification, owned-boundary, self-containment, and Requirement or
Contract realization are product baselines owned by the `design-authoring` and
`design-validation` protocols. They apply whether or not this file exists, and
nothing here can relax them.

## Level of detail

Design states what a component must do and guarantee. It stops short of the
algorithm unless the mechanism is itself the decision being made.

Prefer the smallest document that lets a reviewer judge the approach and an
implementer proceed without guessing. Length is not thoroughness: a long design
that restates one decision in three sections is harder to review than a short
one that states it once.

## Interfaces

- Define behavior through inputs, outputs, and the guarantees that hold, rather
  than through code.
- Make error outcomes part of the interface. A description that covers only the
  success path leaves the most failure-prone behavior undesigned.
- Keep interfaces narrow. One interface serving several unrelated callers
  usually wants to be several interfaces.

Language-specific typing conventions belong to this project's coding standards.
Where they exist, name them here so design and implementation agree.

## Dependency direction

State the intended direction of dependencies for the area being changed, and
treat a design that reverses it as a finding rather than a detail. A dependency
direction that is understood but never written down gets rediscovered,
differently, by each change.

## Data

- Start from the domain concepts, then the storage shape.
- Make consistency boundaries explicit: what must be updated together, and what
  may lag.
- Say how the shape evolves. A data model with no migration story is a decision
  to migrate later, under pressure.

## Error handling and operability

- Prefer failing early and visibly over continuing in an ambiguous state.
- Say what degraded operation looks like when total failure is unacceptable.
- Errors that reach a person should tell them what to do next.
- Note what must be observable to operate this change, when that is not obvious
  from existing practice.

## Diagrams

Include a diagram when it carries structure that prose carries badly: several
interacting components, a multi-step exchange, or a state machine. Skip it when
the text already says it.

Do not restate a diagram in prose. Use the surrounding text for the decisions
and trade-offs the picture cannot show.

## Documentation style

- Declarative and present tense: "the service validates", not "the service
  should validate".
- Consistent terminology. Renaming a concept midway through a document is a
  common source of review confusion.
- No duplicated statements across sections. Reference the earlier one instead.

## Review questions

- Could an implementer act on this without asking the author a question?
- Is every component here required by a requirement in scope?
- Does the document say what happens when things fail, not only when they work?
- Would a reader unfamiliar with this change understand what it owns?
