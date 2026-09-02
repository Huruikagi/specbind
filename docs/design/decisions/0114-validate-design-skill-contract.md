# 0114: Fix the design validation skill contract

Status: Accepted

Decision 0152 later adds the project Design-template selection Rule to the
validator's read set so a missing applicable focused Design is `NOT_READY`.

## Context

`specbind-validate-design` is the second consumer of the `design-validation`
protocol, which [Decision 0094](./0094-embedded-product-protocols.md) assigns to
it and to `specbind-design`.
[Decision 0104](./0104-design-skill-contract.md) already fixed the relationship
between them: the design skill applies the same standard to its own draft before
seeking approval, and this skill is "a separate, independently invoked skill and
is not a precondition of this gate."

[Decision 0079](./0079-milestone-local-research.md) names it as the checker for
one specific property — that persistent artifacts do not defer normative meaning
to Research — whose baseline now lives in the protocol under that decision's
recorded placement.

What remains is small and mostly follows the pattern
[Decision 0111](./0111-review-task-and-debug-skill-contracts.md) established for
the other read-only skills. One thing does not, and it is the reason this
decision exists rather than a sentence in 0104.

## There is no "cannot judge" verdict

The other two validating skills each have an escape for the case where the
judgment cannot be reached. `task-review` has `CANNOT_REVIEW` for a change
entangled with unrelated work; `specbind-validate-implementation` has
`MANUAL_VERIFY_REQUIRED` for a check that cannot be performed here.

Design validation has none, and the protocol is explicit about why:

> Uncertainty is not a verdict. When readiness cannot be judged from the Design
> as written, that itself is a finding: the Design does not yet stand on its own.

The asymmetry is real rather than an oversight. Those other two escapes exist
because something outside the artifact can be missing — a runnable environment,
a separable diff. A Design's inputs are always present: the Design is the
artifact, and if it cannot be judged from what it says, that **is** the defect
being looked for. An escape hatch here would let the most important failure —
a Design too vague to build from — exit as "inconclusive" instead of "not
ready".

So the verdict set is two values, and inconclusiveness resolves to the negative
one with the reason attached.

## Decision

### Subject and moment

The skill validates one Spec's complete current Design set together with its
Contract, applying the `design-validation` protocol. The criteria are identical
to the ones the design skill applies to its own draft; only the authority
differs, exactly as Decision 0104 fixed.

It is useful at any point after a Design exists, and it is not a gate
precondition. Requiring it before every approval would put an optional second
opinion in front of routine work; forbidding it after approval would remove the
independent check exactly when a reviewer most wants one.

### Verdicts

`READY` and `NOT_READY`, with findings.

`NOT_READY` covers both a Design that is wrong and a Design that cannot be
judged, and the finding says which. `READY` asserts that every active
Requirement is substantively realized, the owned boundary is inspectable, the
work decomposes into bounded tasks, the document carries its own meaning, and it
fits the architecture it enters.

Fitting the architecture may require reading existing code, but existing code
is context rather than implementation evidence. This validation is deliberately
available before implementation; the absence of the proposed behavior from the
current code is expected and is not a Design finding by itself.

### The CLI checks are preconditions, not the review

The skill runs `specbind check traceability <spec>` and
`specbind check contracts` because they are cheap and a structural failure makes
semantic review premature.

Decision [0186](./0186-reverse-design-contract-preflight.md) later narrows that
premise for dependency-ordered reverse establishment. Missing Contracts are
provisional only for other reverse participants that current milestone status
proves are waiting for an earlier Design dependency. The current Contract and
every other graph error remain mandatory, and the complete graph is still
required at Contract Review.

It never presents them as its contribution. The protocol says a review that only
repeats what the CLI already verified "has not reviewed anything," and the
distinction matters most here: complete traceability markers are exactly the
evidence that makes an unrealized Requirement look covered.

### Research deference is checked by deletion

The Decision 0079 property is applied as the protocol's deletion test: remove the
referenced document and read the Design again. A requirement, constraint,
interface, or rationale that is now missing or ambiguous was being carried by
something that will not survive the milestone.

Research is the case that matters, because it is excluded from every gate
fingerprint and deleted at release, so nothing mechanical will ever report the
loss.

### It changes nothing, including the gate

The skill edits no Design, no Contract, and no other artifact, and it does not
invalidate the design gate on a `NOT_READY` verdict.

Decision 0104 gives the rewind to `specbind-design`, with a stated cost and a
required confirmation, because invalidation also deletes the accepted contract
review. A validator that rewound on its own verdict would discard
milestone-scoped work as a side effect of an opinion the user did not ask to act
on. The verdict is reported; acting on it is a decision with an owner.

### Boundary

- Validate one Spec's Design set and Contract, and return a verdict.
- Author nothing, repair nothing, approve nothing, invalidate nothing.
- Do not author or revise Research; that belongs to `specbind-gap-analysis`.

## Consequences

- A Design too vague to judge fails rather than escaping as inconclusive, which
  is the outcome the missing escape hatch is there to force.
- The independent check is available before and after approval, and carries no
  authority in either position.
- The Research property named by Decision 0079 has a stated method rather than
  an instruction to notice it.
- A `NOT_READY` verdict cannot destroy the milestone's contract review, because
  the rewind stays with the skill whose decision records its cost.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-validate-design/SKILL.md` is
embedded and installed.

Its forward tests are specified as scenarios VD1 and VD2 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
