# 0118: Fix the gap analysis skill contract

Status: Accepted

## Context

The semantic half of gap analysis is already fixed. The `gap-analysis` protocol
accepted by [Decision 0094](./0094-embedded-product-protocols.md) owns evidence
gathering, the Missing/Unknown/Constraint distinction, how uncertainty is
classified, and the rule that Research is not authority.
[Decision 0079](./0079-milestone-local-research.md) owns the Research artifact's
lifecycle, and [Decision 0092](./0092-template-skill-authoring-boundary.md)
assigns this skill materialization, current-state replacement, and the promotion
of lasting conclusions.

What none of them fixes is when the skill runs, how its findings reach the
artifacts that outlive it, and what happens to an existing Research document.
The inherited `kiro-validate-gap` is the only description of those, and on the
last one it says the opposite of what Decision 0079 accepted.

## It runs before Requirements, not only after

The inherited skill requires `requirements.md` and stops without one. That
ordering leaves a hole that SpecBind creates deliberately elsewhere:
[Decision 0097](./0097-discovery-routing-and-read-models.md) excludes technology
comparison, library viability, and architecture selection from discovery and
routes them here. So immediately after routing, nobody has established what the
repository already provides, and the next skill to run is the one that writes the
behavior contract.

Requirements is therefore an **input when it exists, not a precondition**. With
no Requirements yet, the skill works from the Brief and the milestone's Roadmap
scope. No gate approval is required either; nothing downstream depends on this
skill having run.

Reads:

| Read | When |
| --- | --- |
| `specbind spec status <spec>` | always |
| The Spec's Brief | always |
| `specbind steering list`, then every document listed | always |
| `specbind milestone scope` | when the Spec has no Requirements yet |
| The Spec's Requirements | only when one exists |
| The Spec's existing Research | only when one exists |
| The Spec's Contract, and those of Specs across a touched seam | when boundaries are in scope |

Steering is read whole, with the same stop condition discovery, requirements, and
design already carry: an `ERROR` line stops the skill. The inventory carries no
relevance field, so a selective read would be a guess from a name.

## Findings reach Requirements through the request, not through a read

Decision 0079 permits Requirements workflows to read Research, but
[Decision 0100](./0100-requirements-skill-contract.md) never took it up: its read
table is five rows and Research is not among them. That omission is correct and
this decision keeps it.

The artifacts already draw the line the omission depends on. The Brief holds the
requested change **in the requester's own terms** — Decision 0100 calls it "the
request" outright — while Requirements is the engineered statement of what the
system must do. Letting the current implementation's shape flow directly into
Requirements collapses that: constraints that are really "the existing code makes
this awkward" become obligations the project has promised, and the next milestone
inherits accidental structure as contract.

The influence path runs through the request instead:

| What the analysis found | Where it goes |
| --- | --- |
| The request cannot be satisfied, or only at a cost the requester would not accept | Back to the user. On agreement, the Brief records the revised request, and Requirements follows from it as usual |
| It exists but restricts how the work can be done | Design input only. Requirements is not touched |

The second row is not new. The protocol's own definition of a Constraint is that
it restricts the approach "in a way **the design** must respect" — it already
routes there.

This mirrors [Decision 0098](./0098-steering-read-surface.md) exactly. Steering
is not fingerprinted, so guidance that changed a decision has to be written where
the work lives. Research has the identical property, and gets the identical
treatment.

**A Brief revision requires the user's agreement.** The Brief holds the
requester's words. An agent rewriting them on the strength of a technical finding
is precisely the collapse this section prevents, so the skill proposes the
revision, and writes it only once the user accepts it.

When the skill runs after Requirements instead, nothing extra is needed: Design
already reads Research under Decision 0104.

## The artifact is created only when the finding outlives the analysis

Decision 0079 makes Research optional and Decision 0059 keeps it from becoming
mandatory ceremony. The inherited skill writes it unconditionally.

The skill judges, and **states its reason either way**. An analysis whose
conclusions the Design will absorb in full needs no separate document; one that
took substantial investigation, or that a later session would otherwise repeat,
does. An explicit request for the document is always sufficient reason.

Greenfield work normally stops before this question. When the analysis finds
there is no meaningful existing implementation to compare against, the skill says
so briefly and stops rather than producing an empty comparison.

## An existing Research is replaced, not appended

The inherited skill appends each new analysis below a horizontal rule and tells
the agent explicitly not to overwrite. Decision 0079 accepts the opposite:
Research is current-state input, not an append-only attempt log, and Git
preserves earlier drafts.

Decision 0079 wins. A document that accumulates every superseded finding forces
each reader to work out which conclusions are still in force, and it is read by
the phase that is deciding what to build.

## Every conclusion names where it must land

Research is deleted at release finalization together with the Brief and the task
plan. A conclusion recorded only there is one the project has decided to forget,
and Decision 0079 requires anything still needed afterwards to be incorporated
into the authoritative artifacts.

Nothing performed that incorporation. This skill cannot: it authors neither
Requirements, Design, nor Contract. So it does the part it can, and marks each
conclusion with where it has to end up:

| Destination | For |
| --- | --- |
| Brief | It changed what is being asked for |
| Requirements | It changes an obligation the system must meet |
| Design or Contract | It constrains or decides how the work is built |
| Steering | It is durable project knowledge beyond this milestone |
| Nowhere | Analysis that informed the choice and needs no afterlife |

`specbind-design` promotes the destinations it owns and surfaces the rest rather
than leaving them to expire. A marked Requirements destination is a rewind
decision, which Decision 0104 already gives the design skill a path to raise.

The last row carries weight equal to the others. Marking everything for promotion
buries the conclusions that genuinely need it, which is the same failure
Decision 0098 identified for guidance that merely confirmed a convention.

## The analysis may leave the repository

Claims about the current system come from reading the current system; the
protocol already requires that. Questions about the world outside it — a
dependency's constraints, a protocol's semantics, whether a known integration
problem exists — are legitimately answered elsewhere, and the Research template's
"Sources consulted" section already anticipates it.

External investigation is therefore permitted, with two conditions: sources are
recorded, and an external claim is never presented as an observation about this
repository. No other embedded skill needs this, which is why it is stated here
rather than assumed.

## Investigation is dispatched

The independent lines of investigation — what exists in the affected area, what
conventions constrain it, what integration surfaces it meets, what an external
dependency actually offers — are dispatched as fresh-context subagents under
[Decision 0109](./0109-subagent-dispatch-contract.md). Each reads widely and
returns a pattern, and none of what it read needs to stay in the analyzing
context.

## Writing Research after an accepted completion stales it

Research lives in the project tree, so writing it is an ordinary project change.
Under [Decision 0080](./0080-v1-task-contract-and-completion-details.md) the only
change accepted completion evidence tolerates is a Spec's own completion
transition, so running this skill after any Spec in the milestone has accepted
completion stales that Spec and forces its handshake to be re-run.

The skill checks and says so before writing, rather than after. In the ordinary
ordering the question never arises, because gap analysis precedes implementation.

This is the third contract to state a version of this — Decision 0115 found it
for release binding and Decision 0117 for steering. It is not specific to any of
them, and a general statement addressed to every authoring skill is the better
shape. It is deferred rather than written here, because three instances is where
the pattern becomes visible and four is where guessing its general form stops
being necessary.

[Decision 0119](./0119-writing-while-a-completion-stands.md) writes that general
statement and places it in the `okf-authoring` protocol, so this skill no longer
carries the derivation itself.

## Boundary

- The skill authors Research, with one narrow exception: when its findings show
  that the request itself must change, it revises the Brief only after the user
  accepts the new terms. Requirements, Design, Contract, and `tasks.yaml` belong
  to their own phases, and it writes no machine state.
- It informs; it does not decide. The protocol owns that rule, and the skill does
  not acquire the decision by being the one that gathered the evidence.
- It is not a gate and not a precondition. No approval waits on it, and Design
  proceeds whether or not it ran.
- It does not create Roadmap items or Specs when the analysis suggests the scope
  was wrong. It reports that, and discovery owns the change.

## Consequences

- The hole discovery deliberately leaves has a skill that fills it, at the point
  where the answer is still cheap to act on.
- Requirements keeps its closed read table, and the current implementation's
  convenience has no path into the project's obligations.
- A conclusion worth keeping is marked with its destination while the analysis
  that produced it is still in hand, rather than being rediscovered after the
  milestone deleted it.
- Research states the current view of the investigation, so the phase reading it
  does not have to date the findings itself.
- Three contracts now state the same completion-freshness caution, which is the
  evidence that it belongs somewhere general.

## Implementation status

Implemented. The embedded `specbind-gap-analysis` skill reads the protocol first,
treats Requirements as an input rather than a precondition, stops early on
greenfield work, dispatches its investigation, routes an unmeetable request back
to the user before any artifact is touched, reads the Roadmap scope when no
Requirements exist, revises the Brief only after the user accepts changed terms,
replaces rather than appends an existing Research document, and marks every
conclusion with its destination.

`specbind-design` handles those marks: it promotes the Design and Contract ones,
surfaces a Requirements mark as the rewind decision it is, surfaces a Steering
mark for `specbind-steering`, and states a judgment that a mark needs no action
rather than letting it expire silently.

Forward-test scenarios G1 through G8 remain outstanding, pending a run against
the fixture project. Scenario G8 now measures the `okf-authoring` rule that
Decision 0119 owns, rather than a statement carried by this skill.
