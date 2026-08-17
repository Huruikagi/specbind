# Requirements review protocol

This protocol is the shared semantic baseline a Requirements document must meet
before it is approved. It applies to every supported agent and cannot be waived
by a project template or shared rule.

It covers judgment only. The CLI independently validates heading grammar,
Requirement ID derivation, uniqueness, active-scope membership, and Design and
Task coverage; passing those checks does not mean the requirements are good.
Review-loop limits, approval authority, and invalidation belong to the
requirements skill, not to this protocol.

## The document is the complete current contract

A Requirements document states the Spec's **complete current behavioral
contract**, not the delta requested by the active milestone.

- When a change adds behavior, integrate it into the existing contract so the
  document still reads as one coherent statement of what the Spec does today.
- When a change alters behavior, revise the affected requirement in place rather
  than appending a contradicting one. Two requirements that disagree are a
  defect even if the newer one is correct.
- Do not leave obsolete or contradictory behavior in the current contract. The
  requirements skill currently defers Requirement retirement, so when the
  intended result requires a Requirement group or Acceptance Criterion to
  disappear, stop instead of deleting it or marking it obsolete in prose.
- Revising an obligation in place is ordinary requirements work when the Spec
  retains the responsibility and the same Requirement ID still names the
  changed obligation. Retirement is the removal of that obligation without a
  live identity for downstream coverage.
- Behavior that exists and is still owned by this Spec stays in the document
  even when the current milestone does not touch it.

The active Requirement ID set is a separate concern: it selects which of these
requirements the current change is accountable for. Keeping the catalog complete
does not mean claiming everything as active scope.

## Requirement identity is durable

Requirement IDs are derived from heading numbers and acceptance-criterion list
position. Design traceability markers, task mappings, and accepted gate evidence
all reference them.

- Do not renumber groups or reorder criteria to tidy the document. A cosmetic
  edit that shifts positions silently repoints every downstream reference.
- Group numbers are presentation order only, and gaps are acceptable. Leave a gap
  where a requirement was removed rather than closing it.
- When a criterion genuinely no longer applies, removing it is correct; just
  recognize that later criteria in the same group shift and their references
  must be revisited.

## Coverage

The document must cover, in user- or operator-observable terms:

- the core user journeys the Spec is responsible for
- the scope boundary, made explicit wherever it could otherwise be misread
- primary error cases and the meaningful edge conditions a user or operator sees
- domain rules, compliance constraints, security and privacy expectations, and
  operational constraints that materially shape observable behavior

When the Spec depends on adjacent systems, Specs, or workflows, state what it
expects from them and what it does not own, whenever that distinction changes
user-visible behavior or operator expectations.

Express the boundary as responsibility and expectation, not as architecture.
Components, layers, and internal ownership belong to Design.

## Quality of individual requirements

- Every acceptance criterion must be observable and decidable. A reviewer must be
  able to say whether it holds without reading the implementation.
- Normalize vague language such as "fast", "robust", or "secure" into a concrete
  observable expectation whenever the source material supports it. When it does
  not, that is an ambiguity to resolve, not a phrase to keep.
- Keep implementation choices out. Technology, structure, and mechanism belong in
  Design unless the choice is itself the requirement.
- Group related behavior into coherent requirements instead of restating the same
  obligation in several places. A duplicated obligation drifts.
- Non-functional expectations stay user- or operator-observable.

Projects may add writing conventions, including preferred EARS patterns, through
their own shared rules. Those conventions strengthen this baseline; they never
replace the requirement that a criterion be observable and decidable.

## Ambiguity is escalated, not guessed

When coverage cannot be completed because the request, steering context, or
existing artifacts are ambiguous, contradictory, or underspecified, stop and ask.

Do not invent a plausible requirement to make the document look complete. An
invented requirement becomes an approved contract that later Design, Tasks, and
validation are measured against.

Repairing a locally incomplete draft is ordinary authoring work and needs no
escalation. Escalate when the missing information is a real product decision.

## Readiness for approval

Requirements are ready for approval when the document is the complete current
contract, its coverage is adequate for the Spec's responsibility, each criterion
is observable and decidable, no known ambiguity remains unresolved, and the
proposed active Requirement ID set is exactly what this change is accountable
for.

Structural validity is a precondition, not evidence of any of the above.
