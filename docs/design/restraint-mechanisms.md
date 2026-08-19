# Restraint mechanisms

Status: Partly accepted

This page collects candidate mechanisms for suppressing over-engineering in
projects that adopt SpecBind. Candidate A is accepted and implemented by
[Decision 0121](./decisions/0121-requirements-coverage-is-not-slots.md). The
rest are ideas, kept here so the options stay comparable when the topic is
picked up again.

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

### B. Validation layer

Reuses machinery SpecBind already has, and is likely the highest-value option
per unit of new surface.

- **Reverse traceability (necessity check).** `traceability.rs` currently checks
  that active Requirements are covered by Design and Tasks. Invert it: detect
  Design elements and Tasks that trace to nothing in the active requirement set.
  Over-engineering is largely artifacts without upstream justification, and the
  existing emphasis-marker extraction can decide most of it mechanically.
- **Orphan contract detection.** `contract_graph.rs` already holds the reference
  graph. Warn on contract entries with zero referencing specs as speculative
  seams. This targets the cross-spec layer named as an accelerant.
- **Budget checks.** Per-scale caps on requirement count and design section
  count, enforced by `check` rather than requested in prose.

### C. State and schema layer

Highest leverage, widest blast radius.

- **First-class scale in `spec.yaml`** (for example `small` / `standard` /
  `deep`), fixed during discovery, with the CLI selecting which artifacts are
  required, which gates exist, and which protocols load. This is orthogonal to
  `specbind-quick`: quick reduces approval round trips, scale reduces produced
  volume. A `small` spec could structurally lack a design gate and skip contract
  review.
- **Direct as the discovery default.** Require a positive trigger to choose
  Spec-backed work: a contract is touched, an irreversible data migration is
  involved, or the change exceeds an impact threshold. Keep the thresholds in a
  project-owned rule. Small implementation cost, large expected effect.

### D. Review layer

- Require severity on findings, and let only `blocking` findings hold a gate.
- State in the protocol that a review may not introduce new Requirements. A
  proposal leaves as a roadmap item instead.
- Cap blocking findings per round and cap rounds, then return the judgment to a
  person when a cap is reached.

### E. Observation layer

Not restraint by itself, but its precondition.

- Surface cumulative cost in milestone status: gate invalidation counts, review
  round trips, artifact volume, Direct versus Spec-backed ratio. Gate evidence
  already carries timestamps and revisions, so this is mostly a new read model.
  Visible cost ("this spec rebuilt its design four times") suppresses on its own.

## Suggested order

1. ~~A~~ and D. A is done; D remains light to implement with no structural change.
2. B's reverse traceability and orphan contract detection. SpecBind-specific
   strength, and mechanical rather than prompt-based.
3. C's scale. Adopt only if 1 and 2 prove insufficient, because it propagates
   through schema, state machine, gates, skills, and tests.

## Selection constraint

This area invites over-engineering the restraint itself: new configuration axes,
new gates, and new decision records. Prefer candidates expressible inside the
existing rules, protocols, and traceability surfaces over candidates that add
state or CLI subcommands.
