# 0123: Check coverage in the reverse direction

Status: Accepted

## Context

[Decision 0121](./0121-requirements-coverage-is-not-slots.md) and
[Decision 0122](./0122-finding-disposition-and-deferred-destination.md) bound
authoring and review through protocol prose. Prose is the weakest carrier
available: a skill can fail to read or follow it, and Decision 0094 says so
directly. The candidates collected in
[Restraint mechanisms](../restraint-mechanisms.md) reserve the strongest lever
for the CLI, where an invariant is decided rather than requested.

Every existing traceability check runs one direction, from requirement to
artifact: a Design or Task reference must resolve, an active Requirement must be
covered by a Design, and by a Task once tasks are required. Nothing asks the
opposite question of any artifact, which is the question over-engineering
answers badly — what justifies this?

## Decision

### Reverse traceability applies to tasks, not to designs

The memo proposed flagging Design elements and Tasks that trace to nothing in
the active requirement set. Only half of that is sound.

Requirements and Design are complete-current-contract documents. A Design
section realizing a Requirement outside the active set is describing behavior the
Spec already owns, which is correct and not speculative. Flagging it would make
the product punish accurate documents.

`tasks.yaml` is different. It is milestone-local, excluded from release, and
exists to produce this change. Work in it that the active scope does not account
for is unjustified in the only place where the artifact's own lifetime makes
"unjustified" meaningful.

`traceability::evaluate` therefore gains one issue:

| Code | Meaning |
| --- | --- |
| `TRACEABILITY_TASK_SCOPE_INACTIVE` | An executable task references no active Requirement ID. |

It is evaluated only when an active set is established, so a spec with no active
change is unaffected. Its code shares the `TRACEABILITY_TASK` prefix, so the
existing gate filter already excludes it from the Design gate and it holds the
Tasks gate alone.

An error rather than a warning, for two reasons. `TraceabilityIssue` carries no
severity, and introducing one to soften a single check would thread severity
through the report, gate, and status paths for the sake of a finding people learn
to skip. More importantly the condition is not ambiguous: it means either the
plan carries work nobody asked for or the active set is missing a Requirement it
should hold, and both are defects the author fixes rather than acknowledges.

Serving one active Requirement is sufficient. A task that legitimately touches an
inactive Requirement alongside an active one is ordinary work, and requiring
purity would push authors toward dropping true references.

`task-planning` gains *Coverage runs both ways*, so the planner knows the rule
while authoring rather than discovering it at the gate, and states that adding an
unearned reference to clear the check makes the plan lie about what delivers
what.

### An exported seam nobody consumes is reported

`contract_graph` already resolves every `Consumes` edge into a typed
consumer-to-provider dependency. An `Exports` entry that is no dependency's
provider follows from data the graph holds:

| Code | Severity | Meaning |
| --- | --- | --- |
| `CONTRACT_GRAPH_EXPORT_UNCONSUMED` | Warning | An exported entry is consumed by no managed spec. |

A warning, not an error, and deliberately so. The `contract-review` protocol
already establishes that consumers exist outside the managed graph — a published
interface, another repository, a stored data shape — and the Contract format has
no marker for them. The graph cannot separate a seam serving an external consumer
from one cut for a consumer that never arrived, and a check that cannot tell
those apart must not fail the build.

Only `Exports` is examined. `Owns` records internal ownership rather than a
promise offered to others, so an unconsumed entry there carries no claim to
question.

`contract-review` gains *A seam with no consumer is a claim, not a fact*, which
routes the warning to a decision: name the external consumer, or state that the
project is paying in advance for a boundary. Neither answer is a defect; leaving
the warning unexamined is.

## Consequences

- A task plan that carries work outside the active scope can no longer reach
  Tasks approval. Existing projects with such a plan will see the Tasks gate
  refuse until the plan or the active set is corrected.
- `check contracts` gains a warning that many projects will see immediately.
  Warnings do not fail the command, and the protocol now says what to do with
  one.
- Neither check is new machinery. Both read data the CLI already computes, which
  is what kept them inside the existing surfaces.

## Alternatives considered

- **Flagging Design elements outside the active set.** Rejected: Design is a
  complete-current-contract document, so this would report correct documents as
  defects.
- **Flagging every inactive Requirement ID on a task.** Rejected: it would push
  authors to drop true references rather than to narrow scope.
- **Making the unconsumed export an error.** Rejected: external consumers are
  legitimate and unrepresentable in the Contract format, so the condition is not
  decidable by the graph.
- **A severity field on traceability issues.** Rejected for now: it would touch
  every path that carries a traceability report, to make one check easier to
  ignore.
