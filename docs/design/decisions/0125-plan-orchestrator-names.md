# 0125: Name the planning orchestrators by their stopping point

Status: Accepted

## Context

Decisions 0075 and 0120 introduced `specbind-quick` and `specbind-batch` as
orchestrators that stop after Tasks approval. Neither skill implements code,
validates completion, or releases a milestone.

The names do not expose that boundary. In particular, `quick` can reasonably be
read as taking one item quickly through the complete delivery lifecycle, while
`batch` can be read as the corresponding milestone-wide implementation runner.
That reading conflicts with Decision 0075, which deliberately provides no
milestone-wide implementation orchestrator in v1.

The skills already describe their outcome as an approved plan. Naming that
outcome makes their stopping point visible before a user invokes either one.

## Decision

Rename the two planning orchestrators as a pair:

| Before | After |
| --- | --- |
| `specbind-quick` | `specbind-quick-plan` |
| `specbind-batch` | `specbind-batch-plan` |

`quick` continues to mean one Spec-backed item with fewer approval round trips.
`batch` continues to mean every Spec-backed item in the active milestone. `plan`
states the shared outcome: approved Tasks, with implementation not started.

The recorded `delegation_workflow` value is the invoked skill's new name, so
delegated approvals created by these workflows record `specbind-quick-plan` or
`specbind-batch-plan`.

No legacy aliases are installed. SpecBind has not released the old public skill
surface, and Decision 0075 already establishes that v1 ships no legacy skill
aliases. Carrying aliases now would create two names for one authorization
identity before compatibility requires it.

Decision filenames remain stable. Existing accepted decisions and current
design documents use the new names where they state the governing contract;
superseded historical decisions remain unchanged.

## Consequences

- The stopping point is visible in both public orchestrator names.
- The one-item and milestone-wide skills remain an obvious pair.
- Implementation remains owned by `specbind-implement`, one roadmap item per
  invocation.
- Installed skill paths and delegated approval evidence change together, so the
  displayed workflow name always identifies an installed skill.

## Implementation status

Implemented. The embedded assets, installation targets, tests, user
documentation, forward-test expectations, and current design contracts use the
new names.
