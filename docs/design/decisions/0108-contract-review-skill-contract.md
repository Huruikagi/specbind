# 0108: Fix the contract review skill contract

Status: Accepted

Decision 0159 clarifies that every gate invalidation requires explicit user
confirmation after the complete rewind cost is presented, even when milestone
scope does not change materially.

Decision 0155 replaces historical type-based Contract discovery with a read of
the fixed `<specDir>/specs/<spec>/contract.yaml` path at the baseline revision.

## Context

The milestone-wide contract review is fixed in more detail than any other phase.
[Decision 0078](./0078-contract-first-review-between-design-and-tasks.md) fixes
when it runs, what it requires, the candidate shape, the accepted artifact, and
the two-rerun remediation limit.
[Decision 0087](./0087-milestone-review-cli.md) exposes
`milestone review status` and `milestone review accept`.
[Decision 0054](./0054-milestone-baseline-revision.md) makes the baseline the
diff anchor, [Decision 0055](./0055-cross-spec-review-inputs.md) makes the inputs
Contract-first, and [Decision 0082](./0082-derived-milestone-state-machine.md)
makes it a single global barrier. The `contract-review` protocol carries the
compatibility judgment and `contract-principles.md` carries project seam policy.
[Decision 0106](./0106-contract-review-naming.md) fixes the name and
[Decision 0107](./0107-spec-status-contract-review-barrier.md) makes the barrier
visible from the Spec side.

The protocol names what is left: "the review skill owns remediation,
confirmation, and how many times it reruns."

Authoring the skill surfaced one thing that was not left to it so much as
missing. The protocol requires the judgment to start from the Contract
difference against the milestone baseline, and no command reported the baseline.
`milestone status` printed `Revision:`, which is current `HEAD`. The baseline
lived only in the Roadmap's Front Matter, which is machine state no skill should
be parsing by hand. This decision adds it to the read model.

## Decision

### The baseline is reported

`milestone status` gains a `Baseline:` field carrying the Roadmap's
`baseline_revision`.

It is the anchor the entire review is computed against, it is already parsed by
the CLI, and printing the current revision beside it removes an ambiguity that
`Revision:` alone created. Without it the skill's only route to the before-state
was reading Roadmap Front Matter directly, which every other workflow is
forbidden to do.

### What the skill reads

| Read | When |
| --- | --- |
| `specbind milestone status` | always |
| `specbind milestone scope` | always |
| `specbind check contracts` | always |
| Every current Contract in the project | always |
| The same Contracts at the baseline revision | always |
| `specbind spec status <spec>` for each participant | always |
| Requirements or Design of a specific Spec | only when the conclusion depends on it |

Every current Contract means every one, not only the participants'. The review's
question is whether the milestone leaves the project's seams coherent, and a
Spec outside the milestone is the consumer most likely to break precisely
because nobody is looking at it.

The before-state is read at `Baseline:` through ordinary Git. The comparison is
the review's entry point, so a run that never established what changed has not
performed the review, however carefully it read the current graph.

Artifact identity remains type-based at both revisions. The skill resolves the
configured `specDir`, enumerates the historical Spec directory, and identifies
the lowercase Markdown artifact whose Front Matter `type` is
`SpecBind Contract`. It does not reuse the current path or assume the default
`.specbind/specs/<spec>/contract.md` locator: a rename or move does not turn one
logical Contract into a removal and an addition.

**Steering is not read.** [Decision 0093](./0093-default-shared-rule-set.md)
assigns `contract-principles.md` to this skill and assigns no steering document
to it. The distinction holds: this skill judges whether the graph is coherent
against itself and its consumers, not whether a Design followed project
guidance, which belongs to the design phase and to `specbind-validate-design`.
Project seam policy reaches this skill through the rule that exists for it.

### Prerequisites are checked, not repaired

The skill runs before authoring and reports rather than fixes:

- **Direct-only milestone.** `milestone review status` reports `not required`.
  There is nothing to review, and the skill says so and stops.
- **A participant is not ready.** Acceptance requires every participating
  Spec-backed item to hold a fresh Design gate and sit in the `tasks` state. A
  Spec that does not is reported and routed to its phase. The skill never
  approves a gate to make the barrier passable.
- **A task plan already exists.** Acceptance refuses with
  `CONTRACT_REVIEW_TASKS_ALREADY_EXIST`. The skill reports which Spec holds the
  plan and stops. It does not delete it: the ordering was already lost, and
  discarding authored work is the user's decision, not a step in a review. This
  is the same boundary [Decision 0105](./0105-tasks-skill-contract.md) draws from
  the other side.

### Deep inputs are declared, not collected

`deepInputs` names only the Requirements or Design artifacts the judgment
actually relied on, as the protocol requires. Two consequences make this
concrete rather than stylistic.

Every declared input is fingerprinted into the accepted artifact, so it becomes a
freshness input: editing it later makes the review stale and blocks Tasks
approval, implementation validation, and release preflight. A file declared
because it was opened, rather than because the conclusion turned on it, buys
recurring invalidation for no evidentiary gain.

Task plans are never inputs, and the CLI rejects them. The review happens before
plans exist so that plans are written against a settled seam.

### Remediation is bounded and does not mutate

Decision 0078 allows the skill to remediate and rerun **at most twice**. That
bound is on the skill's own automatic attempts, not on the user's patience: after
two rounds the affected Specs remain in Design, no artifact is written, and the
skill reports what is unresolved.

Within those rounds the skill changes nothing by itself. Decision 0078 is
explicit that review does not mutate Spec state. Where the review concludes that
a Spec needs owned work, the skill:

- presents the affected Spec and what is wrong with the seam;
- obtains confirmation where the milestone's scope changes materially;
- invokes the explicit operation — `milestone update-scope`, or a gate
  invalidation — rather than editing an artifact or a Contract itself.

A Spec added to scope must be brought through Design before acceptance. It
cannot be recorded as follow-up behind a passing review, because the accepted
artifact has no field in which a caveat could live.

### External consumers are the skill's own responsibility

The protocol makes impact on consumers SpecBind does not manage part of this
review, and notes that nothing will detect it. This skill is where that becomes
an action: it names the affected consumer, states the impact, and brings it to
the user when the change requires a decision they own.

The delivery request can already contain that decision. When it explicitly asks
for the changed exported behavior, the review records the requested disposition
and possible unmanaged impact instead of asking the user to reconfirm the same
choice. It stops only for an additional consumer or compatibility choice that
the request and project evidence did not settle. Directly requesting use or
change of an otherwise unconsumed export is also a stated reason to keep that
seam for the milestone.

Silence here cannot be recovered later. Every other finding has some downstream
check that might catch it; this one has none.

### Acceptance is a judgment, not an approval

The skill accepts through `milestone review accept --candidate -` when the
protocol's judgment is satisfied and no finding is unresolved.

Acceptance requires no explicit or delegated approval authority, and this differs
deliberately from the three Spec gates. The accepted artifact has exactly four
fields — `type`, `milestone_id`, `passed_at`, `input_revisions` — with no
`approval_mode` and no `delegation_workflow`, and Decision 0087's command accepts
no approval flag. There is nowhere to record a user's approval, so requiring one
would be ceremony that leaves no trace and that no later boundary could verify.
What the artifact records is the assessment: the skill's own reasoning, in its
own words.

That is not a licence to accept quietly. The skill presents the assessment and
its findings before accepting, and it stops for the user whenever a finding needs
a decision they own, which the two preceding sections require. The difference
from a gate is what is being authorized, not whether the user is informed.

There is no partial, conditional, or provisional acceptance. An unresolved
finding means the review has not passed.

### The assessment

The assessment is the durable explanation, written so that a reader who did not
participate can tell what was examined and why the conclusion holds. The skill
states what changed in the Contract graph relative to the baseline, who depends
on each change, and why the milestone leaves the seams coherent — including, when
it is the case, that nothing changed.

An unchanged Contract is not by itself evidence that no persistent seam changed.
The Roadmap scope states the delivery's claimed behavior. The reviewer compares
those claims with the current Contract and treats a new ownership boundary,
exported behavior, consumed seam, invariant, or file-ownership boundary that the
Contract does not declare as a finding. When the scope raises that possibility,
the reviewer reads the relevant Requirements or Design as a declared deep input
to decide whether the Contract is incomplete; it does not reinterpret Contract
silence as permission to accept.

That deep read starts from `artifact list`. Split Design selectors are discovered
as `design/<artifact_id>` rather than guessed from a conventional filename or a
shortened `design` selector.

A single-participant milestone with an unchanged Contract is a complete review
only when the scoped behavior introduces no missing persistent seam or guarantee.
Its assessment is then short. It is not a smaller review than a multi-Spec one;
it answered the same question and the answer was brief. Padding it produces a
record whose length implies scrutiny that did not occur.

### Boundary

- The skill authors no Spec artifact. Requirements, Design, and Contracts belong
  to their phases, and `tasks.yaml` to the phase after this one.
- It writes no machine state and never edits `spec.yaml` or the Roadmap
  directly.
- It never deletes a task plan, and never approves or invalidates a gate to make
  the barrier passable. It invokes invalidation only as the confirmed outcome of
  a finding.

## Consequences

- The before-state has a supported read, so the comparison the whole review rests
  on stops depending on hand-parsed machine state.
- `Revision:` and `Baseline:` appear together, so neither can be mistaken for the
  other.
- Deep-input discipline has a stated cost, so restraint is a described tradeoff
  rather than a matter of taste.
- The remediation bound is attached to the skill's automatic attempts, so a
  stalled review returns work to Design instead of retrying indefinitely.
- Acceptance has a stated authority model that matches the artifact's actual
  shape, so the skill neither invents an approval nor accepts without presenting
  its reasoning.
- A short assessment for a small milestone is explicitly correct, which removes
  the incentive to inflate the one review that is easiest to inflate.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-contract-review/SKILL.md` is
embedded and installed. `milestone status` reports `Baseline:`.

Its forward tests are specified as scenarios X1 through X4 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
