# Design authoring protocol

This protocol is the shared baseline a Design must meet regardless of project
conventions. It applies to every supported agent and cannot be waived by a
project template or shared rule.

It owns the reasoning a Design must embody and the properties the finished
document must have. Section inventory, ordering, diagram conventions, naming,
language-specific typing rules, and level of detail belong to the project's
template and shared rules. Approval, splitting, and rewind belong to the design
skill. The CLI independently validates traceability syntax, active-Requirement
coverage, and Contract structure.

## Synthesize across the whole requirement set

Design decisions are made against the requirements as a group, not one at a
time. A document assembled requirement by requirement reliably produces
near-duplicate components that each solve one variation of the same problem.

Before settling on an approach, apply these lenses to the full picture:

- **Generalization.** Where several requirements are variations of one
  underlying problem, design the general capability and let each requirement be
  a case of it. Generalize the interface, not the implementation: the
  implementation stays scoped to what the current requirements demand.
- **Build versus adopt.** For each significant component, establish whether the
  problem is already solved by a standard, a platform capability, or an existing
  dependency. Prefer adopting a solution that fits without significant
  adaptation. When building instead, the reason existing solutions were rejected
  is part of the design, not an unstated preference.
- **Reuse inside the repository.** Extending or reusing what the project already
  has takes precedence over introducing a parallel mechanism, unless the design
  states why the existing one cannot carry the change.

## The right design is the smallest one that works

For every component, layer, and abstraction, the question is whether it is
necessary for the requirements in scope.

- Remove anything that exists for hypothetical future requirements. Speculative
  extensibility is a cost paid now for a benefit that usually never arrives.
- Flatten indirection that has exactly one implementation and no concrete second
  case in sight.
- Prefer fewer cohesive components over many fine-grained ones.
- Extensibility belongs at the interface, where it is cheap, rather than in the
  structure, where it is not.

A design that is larger than its requirements is not a safer design; it is a
larger surface for later work to be wrong about.

## The owned boundary is mandatory

A Design is not ready when it explains components but leaves responsibility
seams ambiguous.

- State what this Spec owns before elaborating how it works, and state what is
  explicitly outside that boundary.
- No hidden shared ownership. If two areas appear to co-own the same behavior or
  data, the design is incomplete, not merely undocumented.
- Do not leak downstream-specific behavior or assumptions into an upstream
  boundary.

Naming an owner is optional; leaving the boundary ambiguous is not.

## Realize the Requirements and the Contract

- Every active Requirement ID must be genuinely realized by the design, not
  merely cited. A traceability marker next to a section that does not actually
  satisfy the requirement is worse than a missing one, because it defeats review.
- Where the change touches a persistent seam, the Design and its Contract must
  agree. The Contract states what the Spec owns, exports, and consumes; the
  Design must show how the change realizes exactly that.
- A Contract entry that no part of the design realizes, or a design mechanism
  that crosses a seam the Contract does not declare, is a defect to resolve
  before approval rather than a discrepancy for contract review to discover.

## What belongs in the Contract

The Contract is not a summary of the Design. It is the part of the Design that
other Specs are entitled to rely on, and every entry is a promise that outlives
this milestone.

One question decides membership:

> If this changed, would another Spec's design or verification have to change
> too?

When the answer is yes, it is a Contract entry. When the answer is no, it is
internal to the Spec and belongs in the Design alone, however important it is
there.

Applying that question:

- **Owns** — a responsibility another Spec must not also take. Naming it
  prevents the second implementation, not the misunderstanding.
- **Exports** — a capability, interface, event, or data shape another Spec
  consumes. The entry describes the guarantee, not the current signature.
- **Consumes** — a dependency on another Spec's entry, declared so the producer
  can see who breaks when it changes. An undeclared dependency is invisible to
  the producer at exactly the moment it matters.
- **Invariants** — a guarantee others build on that no single interface carries.
  Ordering, uniqueness, idempotence, and "never observed in state X" belong
  here; the wording is not the promise, the behavior is.
- **File Ownership** — the sparse persistent write boundaries where a conflicting
  change would affect another Spec, under the inclusion test that section already
  carries.

Both failures are real and neither is caught mechanically.

**Under-declaring** is the more dangerous of the two. An unstated seam produces
no dangling reference and no warning, so contract review compares a before and
an after that both omit it, and the first evidence is a consumer breaking after
release.

**Over-declaring** is a slower cost. Internal structure promoted to a Contract
entry becomes something later refactoring must either preserve or renegotiate
through a review, and a Contract that lists everything tells a reader nothing
about which boundaries actually matter.

When membership is genuinely unclear, ask what a consumer would be relying on if
the entry were absent. If nothing outside this Spec could notice, leave it out.

A Spec with no cross-spec seams has an empty Contract with all five headings and
no entries. That is a complete, correct Contract and a deliberate statement; it
is not a placeholder to be filled in later.

## The document stands alone

A Design is the artifact a reviewer, and later an implementer, reads to
understand the change.

- It must be understandable without reading the source, the investigation notes,
  or the conversation that produced it.
- Reference Research for background, never for a conclusion. Any decision that
  matters is restated in the Design itself.
- Stay at the level of interfaces, contracts, and behavior. Specify what a
  component must do and guarantee, not the algorithm that will do it, unless the
  mechanism is itself the decision being made.
- Unresolved questions that survive into the Design are recorded as open
  questions rather than resolved by implication.

## Readiness

A Design is ready for review when the approach was chosen against the whole
requirement set, nothing in it is unnecessary, the owned boundary is explicit,
every active Requirement and the affected Contract are genuinely realized, and
the document can be understood on its own.

Structural validity and complete traceability markers are preconditions, not
evidence of any of the above.
