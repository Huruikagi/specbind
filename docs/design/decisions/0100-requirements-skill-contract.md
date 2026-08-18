# 0100: Fix the requirements skill contract

Status: Accepted

## Context

Almost everything the requirements phase needs is already accepted.
[Decision 0060](./0060-requirement-id-and-heading-mapping.md) derives Requirement IDs,
[Decision 0003](./0003-active-requirement-set.md) stores the active set,
[Decision 0017](./0017-requirements-gate-inputs.md) fixes the gate inputs,
[Decision 0012](./0012-delegated-approval.md) fixes approval authority, and
[Decision 0088](./0088-gate-approval-cli.md) exposes the guarded transitions.
The `requirements` template carries the grammar, the `ears-format` rule carries
project writing preference, and the `requirements-review` protocol carries
semantic quality.

Three things are assigned to the skill and fixed nowhere.
[Decision 0092](./0092-template-skill-authoring-boundary.md) names them exactly:
"the requirements skill owns active selection, approval, and invalidation." The
`requirements-review` protocol says the same from the other side, excluding
review-loop limits, approval authority, and invalidation from its own scope.

[Decision 0098](./0098-steering-read-surface.md) also leaves each skill's
steering discipline to that skill's own decision.

This decision fixes those, and nothing already settled elsewhere.

## Decision

### What the skill reads

| Read | When |
| --- | --- |
| The Spec's Brief | always |
| `specbind spec status <spec>` | always |
| `specbind steering list`, then every document listed | always |
| The Spec's existing Requirements | only when the Spec already has one |
| The Spec's Contract | only when one exists |

The Brief is the request, and `spec status` establishes the lifecycle state the
skill is entering.

The last two rows are conditional because a Spec created by this milestone has
neither. Decision 0089 has `milestone create` write machine state only, and
Decision 0097 keeps discovery from authoring Requirements, so a new Spec holds
only its discovery-authored Brief when this skill starts. The two paths differ:

- **New Spec.** The skill creates the Requirements artifact from the Decision
  0059 template, read through `specbind template read spec requirements`, and
  authors the Spec's first complete contract from the Brief. `spec status`
  reporting a missing Requirements artifact is the expected starting state, not
  a fault.
- **Existing Spec.** The skill reads the current Requirements and revises them
  in place, keeping the document the Spec's complete current contract as the
  review protocol requires.

The Contract is read when present, as context for the boundary this Spec owns.
It is never authored here: Decision 0092 gives the design phase that artifact,
so a new Spec has none until Design runs.

Steering is read whole, as in discovery, because a project constraint on
behavior that is missed here is absent from the contract every later phase is
verified against. This does not make Requirements the only place steering
reaches. The review protocol requires technology, structure, and mechanism to
stay out of Requirements, so technical steering cannot be carried in this
document even in principle; what the design phase reads is fixed by its own
decision, as Decision 0098 leaves each skill's discipline to it.

A steering read that fails stops the skill, for the same reason it stops
discovery: authoring against a knowingly partial view of the project's
constraints produces a contract nobody can trust.

Steering is not a gate input, so a steering document edited after approval
invalidates nothing. The constraints that shaped this document are therefore
written into it, in the Requirements' own terms, rather than left as a reference
to guidance that may since have changed.

### Active selection

The active requirement set is the Requirement IDs this milestone must **deliver
or re-verify**. It is neither the whole document nor only the literal diff.

- Requirements whose behavior this work changes or adds are always in the set.
- Requirements whose correctness depends on that work are in the set even when
  their text is untouched, because they must be re-verified.
- Requirements unrelated to this work stay out, so the milestone is not forced
  to re-plan and re-test the Spec's entire contract.

The two errors are not symmetric. Over-inclusion costs design and task effort on
behavior that did not need it. Under-inclusion means design and tasks never
cover behavior this milestone actually changed, and the gap is invisible: every
mechanical check passes, because coverage is checked against the set the skill
itself chose. When membership is genuinely unclear, include.

The skill states the selection and its reasoning before approving, and the set
is confirmed as part of the approval rather than derived silently.

#### Requirement retirement is deferred

The active set contains live Requirement IDs only, and approval requires it to
be non-empty. SpecBind has no tombstone, retired-ID registry, or completion
contract for proving that an obligation ceased to exist. This decision does not
invent one inside the authoring skill.

The skill therefore does not remove a Requirement group or Acceptance Criterion
from an established Spec. When the requested result requires an identity-bearing
part of the current contract to disappear, it stops before editing and reports
that Requirement retirement is not supported yet. Retiring all behavior owned
by a Spec is likewise a Spec-retirement operation, not an empty Requirements
artifact.

This does not freeze behavior. The skill may revise an existing criterion in
place when the Spec retains the responsibility and the same Requirement ID still
names the changed obligation; that ID is active. It may also add groups or
criteria. The deferred case is removing an obligation without a live identity
for downstream Design, Tasks, and completion verification to cover.

Deferring retirement is a workflow boundary, not a claim that it must be solved
before the first release. A later decision may add retirement semantics without
invalidating artifacts produced under this contract, because none of them
contains a retired identity that needs reinterpretation.

### Approval

The skill approves through `specbind spec requirements approve` only after the
review protocol's judgment is satisfied **and** it holds authority for this gate.
Authority is one of two things, never their absence:

- **Explicit.** The user approved this document and this selection after seeing
  them. `--approval-mode explicit`, the normal path.
- **Delegated.** A run context the user intentionally started already authorized
  this gate by name, under Decision 0012. The user does not confirm this
  document, because delegation is exactly the decision to skip that pause for
  artifacts not yet written. Every semantic and mechanical check still runs, and
  the selection is still stated in the report so the delegation remains
  auditable.

  The workflow label comes from that run context and is never invented. Decision
  0012 has it identify the accelerated workflow whose authority is being
  exercised, so a name the skill made up identifies nothing and turns the one
  auditable trace of a skipped confirmation into a fiction. When no name was
  given, there is no delegation: the skill presents its result and stops.

  A forward test reached this by the other road. Told only that the content was
  pre-approved, an agent chose delegated, had no label, and probed the command
  with a placeholder to learn whether the flag was validated — which recorded a
  real approval, because approval has no dry run. It caught and corrected that
  itself. The rule above removes the reason to experiment: a mutating command is
  never the way to find out what it accepts.

Absence of a prompt, a non-interactive invocation, and a scripted run grant no
authority. Without either form, the skill presents its result and stops.

Neither form authorizes approving to resolve a failing check. A refused approval
is information about the artifact, not an obstacle to route around.

### Review loop

The skill revises and re-presents rather than approving a document it knows to
be weak. It stops and asks the user when the same objection survives one
revision.

A repeated objection means the disagreement is about intent, not wording, and
further unattended rewriting produces variations of the same misunderstanding.
This is a substantive stopping rule rather than an iteration count, because the
number of useful revisions depends on the request, while a recurring objection
reliably indicates the skill has run out of information it can supply itself.

### Invalidation

Routed work reaches this skill with the gate already rewound, because discovery
performs confirmed invalidations before changing scope. That path is unchanged.

The skill is nonetheless the owner of invalidation, and this is where that
ownership is exercised: when it is invoked directly on a Spec whose requirements
gate is approved, it runs `specbind spec requirements invalidate` itself, after
explicit user confirmation. Leaving the user to run it by hand would make the
one recovery path the only step in the workflow the CLI exposes but no skill
performs.

It never edits underneath an approved gate. Editing an approved artifact leaves
evidence describing a revision that no longer exists, and the CLI then refuses
later gates citing freshness rather than the edit that caused it.

Confirmation is required and cannot be inferred, because invalidation clears the
downstream design, tasks, and completion evidence. The skill states that cost
before asking. Delegated authority does not cover it: Decision 0012 delegation
authorizes accepting gates, not discarding accepted work.

### Boundary

- The skill authors the Requirements artifact only. The Contract is the design
  phase's to maintain, and brownfield comparison belongs to
  `specbind-gap-analysis`.
- It writes no machine state. The active set reaches `spec.yaml` only through
  the approve command.
- It does not renumber existing Requirement groups to close gaps. Decision 0060
  makes identity positional, so renumbering silently reassigns IDs that
  `spec.yaml`, design traceability, and task coverage already reference.

## Consequences

- Active selection has a stated rule and a stated bias, so the one failure that
  passes every mechanical check has a documented defense.
- Approval authority is unambiguous at the point it is exercised, not only in
  the decision that defines the modes.
- The review loop terminates on a signal that means something, rather than on an
  arbitrary count.
- A new Spec has a defined starting path, so the phase that materializes the
  first Requirements artifact is named rather than assumed.
- Constraints taken from steering are written into the document, so approval
  does not depend on guidance that nothing fingerprints.
- Approving and invalidating both have a named authority, and the recovery path
  the CLI exposes has a skill that performs it.
- Requirement retirement stops explicitly instead of manufacturing a live
  active-set entry for an obligation that no longer exists.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-requirements/SKILL.md` is
embedded and installed, carrying the conditional reads and the new-versus-
existing Spec branch, whole-set steering reading with its stop condition and the
requirement to write constraints into the document, the active-selection rule
with its stated bias toward inclusion, the retirement stop with the revise-and-add
cases it leaves open, the two forms of approval authority, the repeated-objection
stopping rule, and confirmed self-invalidation when the gate is already approved.

The Decision 0096 conformance check covers its invocations; it was confirmed to
reject both a renamed `spec requirements invalidate` route and an unknown
`--delegation-flow` option. Its forward tests are specified as scenarios R1
through R5 in [Skill forward tests](../../skill-forward-tests.md) and are run
manually. R1, R3, R4, and R5 have passed against a fixture project: the first
authoring of a new Spec stops short of the Contract, a requested retirement stops
before editing, absent authority produces a complete document and no approval,
and an approved gate is invalidated only after the cost is stated and confirmed.
