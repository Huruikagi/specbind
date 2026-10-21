# 0220: Validate converged Specs before one completion checkpoint

Status: Accepted

Decision [0233](./0233-cli-grouped-completion.md) moves the mechanical grouped
acceptance and checkpoint checks into the CLI, preserving the independent
per-Spec assessments, common revision and Git-adapter ownership below.

## Context

The #81 recovery forward test exposed FT-0059: Drive dispatched separate Spec
validation owners, each checkpointing acceptance. The second owner preflighted
at the first completion commit, so its evidence used another implementation
revision and staled the first Spec. Decision 0086 already requires acceptance
metadata for a common revision to be checkpointed together, but the one-Spec
dispatch boundary did not coordinate that transaction. The maintainer approved
including this adjacent repair in #81.

## Decision

Amend Decision 0112's one-Spec validation boundary and Decision 0168's dispatch
granularity. One complete validation owner may handle an explicitly supplied
set of Specs. Drive groups the current actionable entries having the same
ordinary validation Skill target and mode, preserving their exact operands and
order. It invents no targets and never includes parked entries, Direct items or
completion-withdrawal-only work. Drive remains an orchestrator, not a validator
or checkpoint owner. Every included action key consumes its ordinary attempt.

The validation owner preflights every target while the worktree is clean and
requires the same HEAD from all preflights. It reads and assesses each Spec's
active requirements independently. Shared canonical checks may provide common
evidence only where their coverage warrants it; per-Spec semantic verdicts
remain mandatory. Any failed preflight, NO-GO or MANUAL_VERIFY_REQUIRED stops
the set before acceptance. Passing subsets are not accepted ahead of repairs.

After every target earns GO, the owner rechecks the clean unchanged HEAD,
sequentially accepts all candidates against that revision using the existing
guarded CLI, and checkpoints exactly the accepted metadata set once under the
Git adapter. No preflight, check run or commit occurs between acceptances.
Afterwards it verifies that every record remains fresh and reports the actual
checkpoint. A Git adapter that forbids commits retains the existing explicit
uncommitted handoff boundary.

An acceptance or checkpoint failure stops immediately, preserving valid partial
acceptance and reporting the accepted subset, pending candidates, common
revision, diagnostic and dirty paths. No rollback, unchanged-state retry,
subset commit followed by new-revision acceptance, or competing mutating owner
is allowed. Partial acceptance is not a completed validation run. No durable
batch artifact, new CLI/configuration surface, or freshness exception is added.

## Verification

The #81 CLI regression continues through common-revision preflight/acceptance
and a grouped metadata checkpoint, asserting both records remain fresh. Skill
conformance covers dispatch grouping and failure boundaries. A fresh DR9c
behavioral run must reach consistent completion after the replan withdrawal
and implementation checkpoints; earlier failed runs remain recorded.
