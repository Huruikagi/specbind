# 0107: Report the contract-review barrier in Spec status

Status: Accepted

## Context

[Decision 0025](./0025-task-read-model.md) fixes what `spec status` reports:
declared state, consistency health, gate freshness, task progress, next
actionable work, blockers, and Requirement coverage. Its `Gates:` line is, in
practice, the prerequisites of the Spec's next transition.

That line is incomplete. Under
[Decision 0088](./0088-gate-approval-cli.md), `tasks approve` has three
prerequisites: a fresh requirements gate, a fresh design gate, and the fresh
milestone-owned contract review required by
[Decision 0078](./0078-contract-first-review-between-design-and-tasks.md).
`spec status` reports the first two and is silent about the third. The same
review also gates implementation validation and release preflight.

The result is observable. A Spec parked behind the barrier reports:

```text
  State: tasks
  State health: consistent
  Semantic alignment: not evaluated
  Gates: requirements=fresh, design=fresh, tasks=not_reached, completion=not_reached
  Blockers: none
  Diagnostics: none
```

while `milestone status` simultaneously reports stage `contract_review`,
`Contract review: absent`, an actionable `contract_review`, and a diagnostic
naming the missing artifact. Everything needed is known; it is simply absent
from the view a per-Spec question returns.

[Decision 0105](./0105-tasks-skill-contract.md) had to compensate for this in
prose, requiring the tasks skill to run `milestone review status` before
authoring because nothing in its own Spec view would mention the barrier. A
skill instructed to consult a second command to learn a prerequisite of its next
action is describing a gap in the read model.

The gap is worst in the single-Spec case, which is also the case most likely to
be misjudged as not needing a review at all. With one participating Spec there
is nothing else on screen to suggest that a milestone-wide step exists.

## Decision

`spec status` reports the milestone-owned contract review as a `Contract
review:` field, using the same `fresh | stale | absent | invalid` vocabulary as
`milestone status`.

### When it is reported

The field appears when the Spec's declared state is `tasks`, `implementation`,
or `release_ready`.

Those are the states from which the review is both runnable and a prerequisite
of something the Spec still needs: Tasks approval, implementation validation,
and the release the Spec is waiting on. Before `tasks`, acceptance is not
possible at all — it requires every participating Spec to hold current Design
approval — so reporting an absent review in `requirements` or `design` would
present an expected condition as a finding. It is omitted there, and the
evaluation is skipped entirely rather than computed and hidden, so its Git work
is not paid for in the states that cannot use it.

### When it is omitted despite being evaluated

Two outcomes are suppressed because they are not statements about the review:

- **No trustworthy active Roadmap.** The freshness evaluation reports `invalid`
  when the Roadmap cannot be parsed, which is indistinguishable at this line
  from a malformed review artifact. A reader would go looking for a broken
  review file while the fault is the Roadmap. The milestone read model reports
  that failure with diagnostics that name it.
- **`not required`.** A Direct-only Roadmap cannot participate a Spec, so this
  outcome means the Roadmap no longer contains a Spec whose `spec.yaml` claims
  it. That contradiction belongs to the milestone read model as well.

A malformed accepted review, where the Roadmap resolved, still reports
`invalid`, because there the answer really is about the review.

### It does not affect health

The field never contributes to `health` and adds no entry to `Diagnostics:`.

Decision 0078 keeps the review out of the per-Spec invariant: "Cross-spec review
is milestone-level state, not part of the per-Spec `release_ready` invariant.
Unaffected Specs retain their local state when the global review becomes stale."
A stale milestone review must not make every participating Spec report itself as
inconsistent, because their own artifacts and evidence are intact. The field
reports a fact about the Spec's surroundings, not a fault in the Spec.

`Blockers:` also stays task-level under Decision 0025 rather than absorbing this,
so its meaning remains one thing.

## Consequences

- The `Gates:` line and this field together state every prerequisite of the
  Spec's next transition, so a refused approval is predictable from the status
  the caller already read.
- The tasks skill's ordering check becomes an explanation of a barrier the CLI
  reports, rather than a workaround for a barrier it does not.
- A single-Spec milestone shows the milestone-wide step in the per-Spec view,
  where the omission was least likely to be noticed.
- `spec status` performs the review evaluation, including its Git work, in three
  states where it previously did not. It already resolves Git for completion
  freshness, so this adds no new class of dependency.
- Two failure modes are deliberately not surfaced here, on the grounds that
  naming the wrong owner is worse than staying silent while another command
  names the right one.

## Implementation status

Implemented. `spec_status::resolve` evaluates the review only in the three
qualifying states, suppresses the unparsed-Roadmap and `not required` outcomes,
and carries the result in a field that no health computation reads. `cli.rs`
renders it directly after `Gates:` using the shared `review_name` vocabulary.

The regression test drives one Spec from `design` through `tasks` to an accepted
review and asserts the omission, the `absent` report, the `fresh` report, and
that health stays `consistent` throughout.
