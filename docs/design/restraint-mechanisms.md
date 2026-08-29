# Restraint mechanisms

Status: Partly accepted

This page collects candidate mechanisms for suppressing over-engineering in
projects that adopt SpecBind. Candidates A and D are accepted, and B is accepted
in part, by Decisions
[0121](./decisions/0121-requirements-coverage-is-not-slots.md),
[0122](./decisions/0122-finding-disposition-and-deferred-destination.md), and
[0123](./decisions/0123-reverse-traceability-and-unconsumed-seams.md). The rest
are ideas, kept here so the options stay comparable when the topic is picked up
again.

## Problem

Observed in cc-sdd-based projects: specification-driven development, cross-spec
contracts, and review steps at many points combine to grow scope and cost while
product implementation advances slowly. The workflow's strengths and this
failure mode share the same machinery, so the answer is an opt-in restraint
surface rather than a weaker workflow.

## Why the workflow accelerates over-engineering

1. **Template emptiness pressure.** A prepared section reads as something that
   must be filled. A small change still grows full requirements and design.
2. **Reviews rarely return nothing.** An LLM reviewer finds something, and a
   finding almost always adds scope rather than removing it.
3. **Contracts invite speculative generality.** Seams get cut for hypothetical
   future consumers.
4. **Approval gates treat volume as safety.** More written content feels safer
   to approve.
5. **Cost is unobserved.** Round trips, invalidations, and discarded work leave
   no visible trace.

Design consequence: prose asking the agent to be concise is the weakest lever.
Prefer mechanisms that either fail mechanically or leave no structural place to
write the excess.

## Candidates

### A. Authoring baseline — resolved by Decision 0121

Reviewing the accepted protocols against this list showed the restraint baseline
was already present in three of the four authoring layers: `design-authoring`
("the right design is the smallest one that works"), `task-planning` ("every
task is work that will be done"), and `task-implementation` ("do not implement
adjacent work because it is nearby").

The gap was `requirements-review`, whose Coverage section enumerates categories
and so reads as a set of fields to fill. That gap is the one that is amplified
rather than absorbed downstream: an invented requirement enters the active set,
the Design must realize it, and the plan must deliver it.

[Decision 0121](./decisions/0121-requirements-coverage-is-not-slots.md) adds
*Every requirement is one the Spec owes* to `requirements-review`. It is carried
as a protocol section rather than a new shared rule or an install-time option,
because Decision 0093 forbids a rule that restates a protocol baseline, and
because coverage proportionality is not a project preference worth switching off.

### B. Validation layer — partly resolved by Decision 0123

The sketch below proposed flagging Design elements and Tasks that trace to
nothing in the active requirement set. Only the task half survived contact with
the model: Requirements and Design are complete-current-contract documents, so a
Design section realizing an inactive Requirement is describing behavior the Spec
already owns. Flagging it would punish accurate documents. `tasks.yaml` is
milestone-local and exists to produce this change, so unjustified work there is
meaningfully unjustified.

[Decision 0123](./decisions/0123-reverse-traceability-and-unconsumed-seams.md)
adds `TRACEABILITY_TASK_SCOPE_INACTIVE`, an error holding the Tasks gate when an
executable task references no active Requirement ID, and
`CONTRACT_GRAPH_EXPORT_UNCONSUMED`, a warning when an exported seam reaches no
managed consumer. The second is a warning because external consumers are
legitimate and the Contract format cannot express them, so the graph cannot tell
a premature seam from one serving something outside it.

Still open from this candidate:

- **Budget checks.** Per-scale caps on requirement count and design section
  count, enforced by `check` rather than requested in prose. These depend on
  candidate C, since there is no scale to key a budget to yet.

### C. State and schema layer

Highest leverage, widest blast radius.

- **First-class scale in `spec.yaml`** (for example `small` / `standard` /
  `deep`), fixed during discovery, with the CLI selecting which artifacts are
  required, which gates exist, and which protocols load. This is orthogonal to
  `specbind-plan`: delegated approval reduces approval round trips, while scale
  reduces produced volume. A `small` spec could structurally lack a design gate and skip contract
  review.
- **Direct as the discovery default.** Require a positive trigger to choose
  Spec-backed work: a contract is touched, an irreversible data migration is
  involved, or the change exceeds an impact threshold. Keep the thresholds in a
  project-owned rule. Small implementation cost, large expected effect.

### D. Review layer — resolved by Decision 0122

The severity floor turned out to be half-present already: `task-review` states
that naming, formatting, and unstated preferences are not rejecting on their
own, and `design-validation` requires ranking by what would change the decision.
What was missing was the destination, and the reason it was missing is the
reason reviewers inflate severity.

Field observation from cc-sdd: reviewers raise findings as critical because a
finding is otherwise volatile. Nothing downstream picks up a non-blocking
observation, so blocking is the only way to make it survive. Severity inflation
is a rational adaptation to a missing destination, not reviewer error, and
raising the floor alone does not remove it. Either reviewers keep inflating, or
the observations are genuinely lost. Both outcomes are worse than today.

No SpecBind artifact held such an observation: the Roadmap is milestone-scoped
and archived at release, Research and `tasks.yaml` are milestone-local and
deleted at release, `log.md` is history, and steering carries durable convention
rather than pending work.

[Decision 0122](./decisions/0122-finding-disposition-and-deferred-destination.md)
adds *Every finding gets a disposition* to `task-review`, `design-validation`,
and `requirements-review`, and a `deferred` project adapter naming where a
deferred finding is recorded. It deliberately excludes `contract-review`, whose
prohibition on absorbing scope expansion a deferred lane would exit, and
`completion-verification`, which is an evidence gate rather than a review.

Two parts of the original sketch were dropped. Round and finding caps were
rejected: a count limit can hide a genuine blocker, and Decision 0094 places
review-loop limits in the owning skill. "A review may not introduce new
Requirements" needed no new text, because `task-review` and `design-validation`
already say it from their own directions.

The constraint that shaped the design: a backlog an authoring agent reads
becomes a scope source, which would reopen from the back door what Decision 0121
closed at the front. The destination is written to, read only to avoid recording
a duplicate, and re-enters the workflow only by a person putting an item on the
Roadmap.

### E. Observation layer

Not restraint by itself, but its precondition.

- Surface cumulative cost in milestone status: gate invalidation counts, review
  round trips, artifact volume, Direct versus Spec-backed ratio. Gate evidence
  already carries timestamps and revisions, so this is mostly a new read model.
  Visible cost ("this spec rebuilt its design four times") suppresses on its own.

## Suggested order

1. ~~A and D.~~ Both done.
2. ~~B's reverse traceability and orphan contract detection.~~ Done, minus the
   budget checks, which have nothing to key a budget to until C exists.
3. C's scale. Adopt only if 1 and 2 prove insufficient, because it propagates
   through schema, state machine, gates, skills, and tests.

## Selection constraint

This area invites over-engineering the restraint itself: new configuration axes,
new gates, and new decision records. Prefer candidates expressible inside the
existing rules, protocols, and traceability surfaces over candidates that add
state or CLI subcommands.
