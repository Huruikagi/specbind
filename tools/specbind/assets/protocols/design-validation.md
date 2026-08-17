# Design validation protocol

This protocol is the shared semantic baseline for judging whether a Design is
ready to be built on. It applies to every supported agent and cannot be waived
by a project template or shared rule.

One standard serves two moments: the review an author performs before seeking
approval, and the independent validation performed on an authored Design. The
authority and the consequences differ; the criteria do not. A Design that would
fail independent validation is not ready to be submitted for approval.

This protocol owns what makes a Design ready. Repair loops, approval, reporting
format, and how a verdict is delivered belong to the design and validation
skills. The CLI independently verifies traceability markers, active-Requirement
coverage, and Contract structure, so a review that only repeats those checks has
not reviewed anything.

## Requirement coverage is substantive

- Every active Requirement ID must be backed by something concrete: a component,
  an interface, a contract, a flow, a data model, or an operational decision.
  Presence of a traceability marker is a precondition the CLI already checks; the
  reviewer decides whether the marked design actually satisfies the requirement.
- A requirement that introduces an external dependency, integration point,
  runtime prerequisite, migration concern, observability need, security
  constraint, or performance target must be reflected explicitly. Silence on one
  of these is a gap, not an implicit "unchanged".
- When coverage cannot be completed because the requirements themselves are
  ambiguous, contradictory, or underspecified, the correct outcome is to return
  to Requirements, not to invent design detail that makes the gap invisible.

## The boundary must be inspectable

- What the Spec owns and what is explicitly outside it must both be stated.
- Allowed dependencies must be concrete enough that a later reviewer can detect
  a violation. "Depends on the platform layer" is not detectable; a named seam
  is.
- Responsibility or data that appears shared across areas without a clear seam
  is an incomplete design, not a documentation gap.
- Downstream assumptions embedded in an upstream component "for convenience" are
  a defect regardless of how convenient they are.
- A boundary that cannot be explained in a few direct statements is still too
  vague to generate tasks from.
- When the Design reveals several independent responsibility seams that could
  move separately, the answer is to split the Spec or revisit roadmap scope, not
  to force them into one Design.

## It must be buildable as bounded work

- The Design must be implementable as a sequence of bounded tasks with no hidden
  prerequisites. A prerequisite discovered during implementation was a review
  failure, not bad luck.
- Interfaces, contracts, state transitions, and integration points must be
  concrete enough to implement and to verify against.
- Where the architecture intends concurrent implementation, the parallel-safe
  boundaries must be visible.
- A section too vague for a task to reference directly must be rewritten before
  the Design is considered ready.

## It must fit the system it enters

- Integration with existing boundaries, layers, and module organization is part
  of correctness, not style.
- Departures from established patterns are legitimate, but they must be
  deliberate and stated. An unremarked departure is usually an oversight.
- Dependency direction and coupling must be consistent with the architecture the
  change enters.
- Complexity must be proportionate to the requirements. Both an
  under-specified design and an over-built one fail this criterion.

## Findings must be checkable

- A finding names the requirement or boundary it endangers and points at the
  specific place in the Design that causes the concern. A finding without both
  cannot be acted on or disputed.
- A finding states the consequence, not only the observation. "Section X is
  vague" is not reviewable; "Section X does not determine which component owns
  retry, so tasks cannot be bounded" is.
- Rank by what would change the decision. Listing every imperfection dilutes the
  few issues that matter and makes the review harder to act on.
- Recognize what the Design does well when it is true. A review that only
  accumulates objections gives the author no signal about what to preserve.

## The verdict

A Design is ready when every active Requirement is substantively realized, the
owned boundary is explicit and inspectable, the work can be decomposed into
bounded tasks, and it fits the existing architecture with proportionate
complexity and acceptable, stated risk.

It is not ready when it conflicts with the existing architecture in a way the
Design does not resolve, leaves a material requirement unaddressed, hides a
prerequisite, or carries complexity out of proportion to what was asked.

Uncertainty is not a verdict. When readiness cannot be judged from the Design as
written, that itself is a finding: the Design does not yet stand on its own.
