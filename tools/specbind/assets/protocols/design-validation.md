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

## It must stand on its own

The Design is read after the milestone that produced it has closed, by people
who did not participate in it. Everything it needs to mean must be inside it.

- A decision that matters is stated in the Design, not referenced. Research,
  investigation notes, a ticket, a conversation, or a prior review may be cited
  as background; none of them may carry the meaning.
- **Research is the case to check deliberately.** It is milestone-local: it is
  excluded from every gate fingerprint, so editing it invalidates nothing, and
  release finalization deletes it. A Design that defers a normative decision to
  Research is therefore not merely inconvenient to read later — it becomes
  incomplete at release, with nothing reporting the loss.
- The test is a deletion test. Remove the referenced document and read the
  Design again: if a requirement, constraint, interface, or rationale is now
  missing or ambiguous, the Design was depending on it. Reference the source for
  context, then restate the conclusion in the Design itself.
- The same applies to a reference into source code as the definition of intended
  behavior. Code states what the system does now; the Design states what it must
  do.

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

## Every finding gets a disposition

A finding raised in this review ends in exactly one of three states, and the
reviewer names which one. There is no fourth state in which a finding is
mentioned in passing and then carried nowhere.

- **Blocking.** It changes the verdict. It is resolved before the gate is
  crossed.
- **Resolved in place.** It was examined and needs no work, and the reason is
  stated. A judgment made and explained is not an outstanding finding.
- **Deferred.** It is real and actionable, it does not change the verdict, and
  it is written to the destination this project names for deferred findings.

A finding stated in the report and given no disposition is volatile: nothing
downstream carries it, and a reviewer who knows this raises the next one as
blocking to make it survive. Severity inflation is the predictable result of
having nowhere to put a true observation, so the disposition is not optional
bookkeeping.

Deferring is not a way to pass a review that should not pass. A finding that
changes the verdict is blocking whether or not it is convenient, and moving it
to the destination does not settle it.

A project that names no destination has none. State the deferred finding in the
report and say that it is not recorded anywhere, rather than promoting it or
discarding it silently.

## The verdict

A Design is ready when every active Requirement is substantively realized, the
owned boundary is explicit and inspectable, the work can be decomposed into
bounded tasks, the document carries its own meaning, and it fits the existing
architecture with proportionate complexity and acceptable, stated risk.

It is not ready when it conflicts with the existing architecture in a way the
Design does not resolve, leaves a material requirement unaddressed, hides a
prerequisite, defers a decision to a document that will not survive the
milestone, or carries complexity out of proportion to what was asked.

Uncertainty is not a verdict. When readiness cannot be judged from the Design as
written, that itself is a finding: the Design does not yet stand on its own.
