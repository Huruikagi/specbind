# 0104: Fix the design skill contract

Status: Accepted

## Context

The design phase is the most heavily pre-decided phase in SpecBind, and almost
none of what it decides is left to the skill.
[Decision 0038](./0038-design-gate-inputs.md) fixes the gate inputs,
[Decision 0061](./0061-design-requirement-traceability.md) fixes traceability,
[Decision 0056](./0056-canonical-contract-markdown.md) fixes the Contract's
syntax and identity rules,
[Decision 0088](./0088-gate-approval-cli.md) exposes the guarded transitions,
[Decision 0032](./0032-gate-local-freshness-chain.md) fixes freshness, and
[Decision 0078](./0078-contract-first-review-between-design-and-tasks.md) fixes
what happens immediately after this gate. Three product protocols —
`design-discovery`, `design-authoring`, and `design-validation` — carry the
semantic baseline, and `design-principles.md` and `contract-principles.md` carry
project preference.

[Decision 0092](./0092-template-skill-authoring-boundary.md) names what is left:
"Design protocols own discovery, authoring, and validation baselines; the design
skills own selection, orchestration, approval, and rewind," and for the Contract,
"skills own update timing and review orchestration."
[Decision 0098](./0098-steering-read-surface.md) leaves each skill's steering
discipline to that skill's own decision.

Those assignments have no contract. This decision fixes them, and one gap they
depend on: nothing states what belongs in a Contract at all.

## The Contract inclusion test belongs in a protocol

Decision 0092 assigns the "semantic seam and compatibility baseline" to the
design and cross-spec protocols. The compatibility half exists: the
`contract-review` protocol judges what a changed, added, or removed entry does
to its consumers. The inclusion half does not. Decision 0056 supplies an
inclusion test for File Ownership only, and `design-authoring` requires the
Design and Contract to agree without saying what should have been in the
Contract in the first place.

That gap is load-bearing. A Contract that under-declares passes every mechanical
check — an unstated seam has no dangling reference — and contract review then
compares a before-state and an after-state that both omit it. A Contract that
over-declares turns internal structure into a durable promise that later
refactoring must either honor or renegotiate through review.

Applying Decision 0092's allocation test places this in a protocol rather than
in this skill. It must remain true when a project replaces every template and
rule, it is substantial semantic content, and it is shared by `specbind-design`,
`specbind-gap-analysis`, and `specbind-contract-review`. The
`design-authoring` protocol therefore gains the inclusion test, stated as the
consequence question: an entry belongs in the Contract when another Spec's
design or verification would have to change if it changed. This decision records
that placement; the protocol text owns the wording.

The skill keeps update timing, which is what Decision 0092 assigned it.

## Decision

### What the skill reads

| Read | When |
| --- | --- |
| `specbind spec status <spec>` | always |
| The Spec's Requirements | always |
| The Spec's Brief | always |
| `specbind steering list`, then every document listed | always |
| The Spec's existing Design set and Contract | only when the Spec has them |
| The Spec's Research | only when one exists |
| The Contracts of Specs this one consumes or is consumed by | when the change touches a seam |

`spec status` establishes the lifecycle state and, decisively, whether the
requirements gate is approved and fresh. The Requirements are the obligation
this phase must realize; the Brief is why this milestone is changing it.

Steering is read whole, with the same stop condition as discovery and
requirements: an `ERROR` line from `steering list` or `steering read` stops the
skill rather than producing a design against a knowingly partial view of the
project's constraints. This is where the project's technical steering finally
lands. Decision 0100 keeps technology, structure, and mechanism out of
Requirements as a matter of protocol, so a constraint on how this project builds
things cannot have been carried by the previous phase even in principle; if the
design phase also reads selectively, that guidance reaches no authoritative
artifact at all.

Research is read when present and is never cited as authority. Decision 0079
excludes it from gate fingerprints and deletes it at release, so a Design whose
meaning depends on it becomes incomplete the moment the milestone closes. Any
conclusion the Design needs is restated in the Design.

### Prerequisites are checked, not repaired

The skill authors nothing until `spec status` reports the requirements gate
approved and fresh, and the Spec a current participant of the active Roadmap.

A Spec that has reached the design state and has no design artifact yet reports
`Health: inconsistent` with a missing-coverage diagnostic for every active
Requirement. That is the expected starting state of this phase, not a fault, and
the skill says so rather than treating the diagnostic as damage to repair. The
requirements skill has the same rule for a missing Requirements artifact under
Decision 0100.

A stale or unapproved requirements gate is reported and routed to
`specbind-requirements`. The design skill never approves or invalidates the
requirements gate, and never edits Requirements to make its own work possible.
Editing an approved upstream artifact from a downstream phase invalidates that
gate as a side effect and leaves the user with a freshness diagnostic instead of
the decision they would have made.

### The Design set is the Spec's complete current design

The Design collection is persistent and current-state, exactly as Requirements
are. An established Spec's existing design artifacts are revised in place; this
milestone's change is folded into the document that owns that concern rather
than appended as a milestone-shaped supplement. A reader arriving after release
must be able to understand the system from the Design set alone, with no
knowledge of which milestone contributed which paragraph.

Coverage is scoped, and that is a separate axis: Decision 0061 requires the set
to cover every active Requirement ID, while inactive Requirements may remain
mapped by the persistent design and are not re-argued.

### Decomposition

A Spec's default decomposition is the single `main` artifact its template
carries. The skill splits only when the design contains responsibility seams
that a reader would follow independently, and each `artifact_id` then names a
durable concern rather than a slice of this milestone's work.

Identity churn is expensive here in a way it is not elsewhere. Decision 0038
fingerprints the complete logical-key set, so adding or removing a design
identity invalidates approval by itself, and Decision 0061 requires each file's
Front Matter and body markers to agree exactly. Reorganizing an established
Spec's design set is therefore a deliberate act with a stated reason, not
housekeeping performed in passing.

When the Design reveals responsibility seams that could move separately, the
`design-validation` protocol's answer applies: raise splitting the Spec or
revisiting Roadmap scope with the user. The skill does not create or rescope
Specs itself.

### Contract update timing

Every Spec reaching design approval has exactly one Contract, and the design
phase is what puts it there. Decision 0038 refuses approval without it and does
not interpret its absence as an absence of cross-spec impact.

- **A Spec with no Contract** — including every Spec this milestone created —
  gets one, materialized from `template read spec contract`. A Spec with no
  cross-spec seams gets the canonical empty Contract: five headings, no entries.
  An empty Contract is a statement, and it is the statement Decision 0056
  requires.
- **A Spec with a Contract** — revised in place when this change adds, alters,
  or removes a seam, and left byte-identical when it does not. Rewording an
  untouched entry is not free: Decision 0038 fingerprints the whole file, so a
  cosmetic edit invalidates approval and forces a new contract review.
- Entry IDs are stable under Decision 0056. The skill does not rename an ID whose
  semantic boundary is unchanged, because another Spec's `Consumes` entry
  resolves through it.

Removal is permitted here, unlike Requirement retirement under Decision 0100.
The asymmetry is real rather than an inconsistency: a Requirement ID is an
identity that design, tasks, and completion verification are each required to
cover, so removing one leaves obligations with nothing to point at, whereas a
Contract entry's only structural dependents are other Specs' `Consumes` entries,
which `check contracts` resolves by name. Removal is representable, so it is
allowed and then judged — by contract review, which exists for precisely this.

The skill runs `specbind check contracts` before approving and resolves what it
reports. A reference left dangling by a removal is fixed in this Spec, or the
consuming Spec needs owned work — which is surfaced to the user as a scope
question. The skill never edits another Spec's Contract to make its own graph
clean. Ownership overlaps and cycles are warnings under Decision 0078; the skill
states why an overlap is acceptable or treats it as a finding, rather than
passing it silently to review.

### Self-review before approval

The `design-validation` protocol is the standard the skill applies to its own
draft before seeking approval. Decision 0094 gives the protocol both consumers
for this reason: the criteria are identical, and a Design that would fail
independent validation is not ready to be submitted.

`specbind-validate-design` is a separate, independently invoked skill and is not
a precondition of this gate. Requiring it would put an optional second opinion
in front of every approval; making the self-review optional would leave approval
resting on structural checks the CLI already performs.

`specbind check traceability <spec>` is run before approving. The approval
enforces it regardless, but running it first turns a refused approval into a
diagnostic the skill can act on.

### Approval

Approval follows Decision 0100's contract without variation, because the
authority question is identical at every gate. The skill approves through
`specbind spec design approve` only after the validation protocol's judgment is
satisfied and it holds either explicit authority — the user approved this design
after seeing it — or delegated authority under Decision 0012, whose workflow
label comes from the run context and is never invented. Absence of a prompt, a
non-interactive invocation, and a scripted run grant nothing. Neither form
authorizes approving to resolve a failing check, and no mutating command is used
to discover what it accepts.

`design approve` takes no IDs, paths, or fingerprints. Decision 0038 derives its
complete input set, so there is nothing for the skill to submit and nothing it
could submit incorrectly.

The report states what the design decided, which active Requirements it realizes
and how, and what changed in the Contract — the last of these being the input
contract review will start from.

### Review loop

The skill revises and re-presents rather than approving a design it knows to be
weak, and stops to ask the user when the same objection survives one revision,
for the reason Decision 0100 gives: a repeated objection is a disagreement about
intent, which further unattended rewriting cannot resolve.

Where an objection reveals that the Requirements themselves are ambiguous,
contradictory, or underspecified, the `design-validation` protocol already fixes
the outcome — return to Requirements rather than invent design detail that hides
the gap. The skill reports that and stops; it does not perform the requirements
rewind itself.

### Rewind

The skill owns the design rewind and exercises it here: invoked directly on a
Spec whose design gate is approved, it does not edit, and it runs
`specbind spec design invalidate` only after explicit user confirmation.

The cost stated before asking is larger than the requirements gate's and is
stated in full. Under Decision 0088, `design invalidate` clears design, tasks,
and completion evidence **and deletes the accepted contract review**, because
Decision 0078 accepts that review after design approval and it cannot survive a
rewind past it. A user who confirms without being told that is discarding a
milestone-wide artifact they did not know about.

Delegated authority does not cover invalidation. Decision 0012 delegation
authorizes accepting gates, not discarding accepted work.

The skill never edits underneath an approved gate, for Decision 0100's reason:
the evidence then describes a revision that no longer exists, and the CLI refuses
later gates citing freshness rather than the edit that caused it.

### Boundary

- The skill authors the Design set and this Spec's Contract. Requirements belong
  to the previous phase, `tasks.yaml` to the next, and brownfield comparison and
  Research to `specbind-gap-analysis`.
- It writes no machine state and never edits `spec.yaml`.
- It does not accept the contract review, add Roadmap items, or create Specs.
  It surfaces the need and lets the owning operation perform it.
- It does not author Research, and does not resolve a Design gap by recording it
  there.

## Consequences

- The Contract has a stated inclusion test for the first time, placed where a
  project cannot remove it and where the two other skills that need it can read
  it.
- The design phase is the second whole-steering reader, so technical project
  guidance has an authoritative destination.
- Contract entry removal is allowed with a stated reason for differing from
  Requirement retirement, so the two rules do not read as an accident.
- The rewind's true cost, including the deleted contract review, is stated
  before the user confirms it.
- Approval authority, the review loop, and the no-edit-under-an-approved-gate
  rule are identical across gates, so the workflow has one answer to those
  questions rather than one per phase.
- `specbind-validate-design` keeps a purpose distinct from the authoring
  self-review instead of becoming a mandatory second pass.

## Implementation status

Implemented. `tools/specbind/assets/skills/specbind-design/SKILL.md` is embedded
and installed, carrying the conditional reads with whole-set steering and its
stop condition, the prerequisite check that routes rather than repairs, complete
current-state authoring with its decomposition rule, Contract materialization
including the canonical empty case, the removal path with its
`check contracts` obligation, the `design-validation` self-review, the two forms
of approval authority, the repeated-objection stopping rule, and confirmed
self-invalidation that states the deleted contract review.

`tools/specbind/assets/protocols/design-authoring.md` carries the Contract
inclusion test.

[Decision 0109](./0109-subagent-dispatch-contract.md) subsequently added
fresh-context subagent dispatch to the investigation step: independent areas are
dispatched with self-contained briefs and the `design-discovery` selector, return
findings summaries, and synthesis stays in the main context because choosing the
approach needs the whole picture. Dispatch is skipped for a change that follows
an established pattern.

[Decision 0132](./0132-target-aware-template-resolution.md) subsequently made
creation depend on the Design set rather than the age of the Spec. When an
established Spec has no Design artifact, the skill lists, resolves, and reads the
configured Design templates exactly as it does for a new Spec; only a Design set
that already exists is revised in place.

Its forward tests are specified as scenarios DS1 through DS6 in
[Skill forward tests](../../skill-forward-tests.md) and are run manually.
