# 0197: Check the inputs needed to verify a Task

Status: Accepted

## Context

[Decision 0105](./0105-tasks-skill-contract.md) assigns Tasks authoring and
approval to the planning workflow. Its execution-readiness audit checks whether
a verification command or test interface exists by the time a Task finishes.
That alone does not establish that the check can receive the input or state it
needs. Issue #46 observed a provider Spec whose verification needed a real input
connection owned by a later consumer Spec.

Specs divide product responsibilities. Requiring each Spec to prove the entire
integrated path independently would encourage interfaces added only to satisfy
the workflow. Cross-Spec work and later integration checks are legitimate.

## Decision

Extend the existing Tasks execution-readiness audit. For each completion check,
identify what it proves, the input or state it needs, how that input or state
will be supplied, and whether it is available when the Task finishes. Existing
checks and fixtures can supply this evidence; no separate inventory is required.

Only when a check depends on another Spec, inspect the relevant provider
Contract and Design and the Roadmap execution order. Read that Spec's Tasks
only if they exist and their detail is needed to resolve availability. This is
a targeted prerequisite check, not another milestone-wide Contract Review or a
requirement to author every Spec's Tasks before approving any of them.

Distinguish proving the current Task's responsibility with boundary input from
proving the real connection after integration. A later integration check is
valid when the earlier Task already proves its own responsibility. It cannot
retroactively make an unverified behavior Task complete.

The audit starts from approved Design verification obligations as well as draft
Task checks. Boundary proof is not permission to omit or replace a stronger
Design completion condition. Report that mismatch before presenting the plan as
ready and return the correction to Design before approving Tasks.

Prefer an existing input path or a small test helper when sufficient. Do not
require a new public API, injection layer, or permanent abstraction solely to
make Specs independently finishable. If necessary, revise the completion
condition, verification placement, or work order within the approved scope.
Do not weaken a Requirement or move an obligation to another owner silently.
When the correction needs Design, Contract, Roadmap, or another Spec's plan to
change, report the concrete mismatch and route to its owner under the existing
approval and invalidation rules. Tasks authorship remains local to the target.

## Consequences

- An existing test command is no longer enough to justify an executable
  completion condition when its required input is unavailable.
- Responsibility boundaries and legitimate cross-Spec work are preserved.
- The change adds no schema field, ledger, Skill, CLI gate, cross-Spec Task ID,
  or execution-order policy. The judgment remains with the planning agent.
- Contract Review still precedes Tasks and does not consume them.
