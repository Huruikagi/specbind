# 0153: Unify planning orchestration under quick-plan

Status: Accepted

Supersedes: [Decision 0120](./0120-quick-and-batch-orchestration-contracts.md),
[Decision 0128](./0128-plan-orchestrator-names.md), and the two-planning-skill
clauses of [Decision 0075](./0075-v1-skill-and-orchestration-scope.md)

## Context

Decision 0120 deliberately reduced `specbind-quick-plan` and
`specbind-batch-plan` to one orchestration contract at two scales. They use the
same phase Skills, delegated gates, Design validation, global Contract Review,
CLI guards, checkpoint handoffs, retry classification, and Tasks-approval
stopping point. Their remaining public distinction is whether the request names
one Spec or explicitly selects every Spec in the active milestone.

That cardinality does not justify two installed Skills. It makes discovery
choose between almost identical entry points, duplicates a long product-managed
contract, and creates a routing failure mode without adding a lifecycle
boundary. The inherited cc-sdd Skills needed the split because quick skipped
validation while batch computed its own dependency waves and performed a
different cross-Spec review. SpecBind has rejected those semantic differences.

The product has not reached its first stable release. It does not need to carry
an alias or migration surface for the removed planning Skill.

## Decision

### One Skill, two explicit scope modes

V1 installs only `specbind-quick-plan` for accelerated planning. It supports:

- **named scope**: one named or targeted Spec-backed Roadmap item;
- **all scope**: every Spec-backed participant in the active milestone, selected
  by `--all` or an equally explicit all-Spec request.

Both modes use the same workflow identity, `specbind-quick-plan`, in delegated
gate evidence. `quick` describes fewer approval round trips, not a promise that
the run contains only one Spec. `plan` continues to state the stopping point:
Tasks approval, before implementation.

`specbind-batch-plan` is removed. No alias, forwarding Skill, deprecation stub,
or compatibility diagnostic is installed.

### A bare invocation asks for intent

A direct invocation with neither a named target nor explicit all-Spec intent
may read `specbind milestone status` to present the available scope, but it does
not dispatch a phase, author an artifact, or approve a gate. It asks the user to
choose one named Spec or all Specs and stops for the answer.

The Skill never infers all scope from participant count. One participating Spec
still leaves two different user intents: target that item, or select the complete
milestone Spec set. Direct items are displayed as outside planning scope rather
than absorbed.

Scope selection is distinct from delegated-gate authorization. After scope is
known, the Skill presents the milestone, exact items, gate names, and durable
workflow identity and takes one bounded confirmation. One user answer may both
select scope and authorize delegation only when it explicitly does both.

### The all-scope scheduler is the shared algorithm

The unified Skill uses the milestone-scale orchestration model for both modes:

- Requirements are not dependency-gated;
- Design waits on direct Spec-backed predecessors with current Design approval;
- one global Contract Review follows current Design approval for every
  participating Spec;
- Tasks are parallel again after the accepted review;
- `specbind milestone status` is reread between rounds instead of independently
  computing dependency waves.

All scope dispatches every in-scope actionable item. Named scope filters each
round to the selected item and never expands merely to clear a dependency or the
global barrier. If another participating Spec must progress before the selected
item can reach Tasks approval, the named run reports that outside-scope blocker
and stops. Discovery or an explicit all-scope request owns any scope expansion.

The remaining orchestration contract from Decision 0120 is unchanged: fresh
phase dispatch, bounded Design revision and revalidation, clean adapter-directed
checkpoint handoffs, status-based failure classification, no gate invalidation
under delegation, no Roadmap mutation, and no implementation or release work.

## Consequences

- Skill discovery has one accelerated planning entry point rather than two
  cardinality-based alternatives.
- Named and milestone-wide planning retain different scopes without duplicating
  their lifecycle semantics.
- A bare invocation is safe and inspectable instead of guessing how much work
  the maintainer intended.
- Delegated gate evidence has one current workflow identity.
- The public v1 Skill set decreases by one with no compatibility alias.

## Implementation status

Implemented. The embedded Skill, registry, generated index, design catalog,
mechanical tests, and behavioral scenario contract use the unified
`specbind-quick-plan` surface.
