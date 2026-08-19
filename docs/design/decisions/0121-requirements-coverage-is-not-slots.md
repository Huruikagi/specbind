# 0121: Bound Requirements coverage to what the Spec owes

Status: Accepted

## Context

Projects adopting a specification-driven workflow report a failure mode where
scope and cost grow faster than delivered product. The candidate responses are
collected in [Restraint mechanisms](../restraint-mechanisms.md). Reviewing the
accepted product protocols against that list shows the restraint baseline is
already present in three of the four authoring layers:

- [`design-authoring`](../../../tools/specbind/assets/protocols/design-authoring.md)
  states that the right design is the smallest one that works, and rejects
  speculative extensibility and single-implementation indirection.
- [`task-planning`](../../../tools/specbind/assets/protocols/task-planning.md)
  states that every task is work that will be done, with no aspirational entries.
- [`task-implementation`](../../../tools/specbind/assets/protocols/task-implementation.md)
  forbids implementing adjacent work and requires the smallest decision that
  satisfies the requirement where the Design is silent.

`requirements-review` has no equivalent. Its Coverage section is an enumeration
of categories a document must cover, and
[Decision 0092](./0092-template-skill-authoring-boundary.md) and
[Decision 0093](./0093-default-shared-rule-set.md) leave no room to compensate
for that in a shared rule, because a rule may not restate or relax a product
baseline.

The enumeration is correct about what a Requirements document is judged against
and incorrect about how an authoring agent reads it. A category list reads as a
set of fields to fill, and the cost of filling an empty one does not stop at the
document: the active Requirement ID set makes it accountable scope, the Design
must realize it, and the task plan must deliver it. Requirements inflation is
therefore the one restraint gap that is amplified by every downstream gate rather
than absorbed by it.

## Decision

`requirements-review` gains one section, *Every requirement is one the Spec
owes*, placed immediately after Coverage.

- The coverage categories are what the document is judged against, not slots a
  complete document has filled. An inapplicable category is complete when absent.
- A category is covered because the Spec's behavior meets it, never because the
  category exists.
- Document size follows the Spec's responsibility. Neither length nor brevity is
  evidence about coverage.
- A real concern owned elsewhere is stated as an expectation of that boundary,
  not as an obligation this Spec owes.
- Behavior untouched by the current change stays as already written. Re-deriving
  it in more detail rewrites the current contract while appearing to improve
  coverage.

The section closes by routing the genuinely unknown case to the existing
escalation baseline, so it cannot be read as licence to omit or to invent.

### Carrier

The statement is a product protocol section, not a new shared rule and not an
install-time option.

- Protocols are embedded in the binary, so the change reaches every project on
  its next upgrade without an install refresh and without touching
  project-owned files.
- `specbind-requirements` already reads `requirements-review`, so no skill,
  agent template, or rule-loading table changes.
- Under Decision 0093 a shared rule may not restate a protocol baseline, which
  rules out a `restraint-principles.md` carrying the same content.
- Coverage proportionality is not a project preference. A project that wants
  more coverage strengthens it through its own rules, which this baseline
  already permits; nothing is served by letting a project turn proportionality
  off.

Consequently this decision adds no CLI flag, no configuration field, and no
enablement axis, preserving the Decision 0093 position that installed policy has
no enablement field in v1.

## Consequences

- Requirements authoring and review gain an explicit restraint statement
  matching the ones design, planning, and implementation already have.
- Existing approved Requirements documents are unaffected. The section changes
  how the next authoring or review pass judges a document; it is not a
  retroactive defect definition, and it creates no invalidation.
- The remaining candidates in [Restraint mechanisms](../restraint-mechanisms.md)
  are unchanged by this decision. Reverse traceability, orphan contract
  detection, review severity floors, and spec scale remain open.

## Alternatives considered

- **A `restraint-principles.md` shared rule.** Rejected: it would restate
  `design-authoring` and `task-planning` baselines, which Decision 0093 forbids,
  and it would reach existing projects only as an uncommitted install addition.
- **An `install` flag gating the rule.** Rejected: installed rules are created
  only when missing, so the flag would carry meaning on first install alone,
  while adding a permanent CLI surface and either an unpersisted behavior
  difference across refreshes or a new persisted configuration field.
- **Rewriting the Coverage enumeration itself.** Rejected: the enumeration is
  the correct statement of what review judges, and shortening it would weaken
  coverage rather than bound it.
