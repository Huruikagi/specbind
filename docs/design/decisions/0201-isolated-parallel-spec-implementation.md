# 0201: Opt in to parallel Spec implementation with sequential host fallback

Status: Accepted

## Context

Decision 0146 keeps Tasks sequential; Decision 0168 initially permits one
mutating Drive owner. Independent Specs can be implemented in isolated host
worktrees without introducing Task-level parallelism.

The initial design also specified candidate replay, serial integration and
interrupted-run recovery in Skill prose. That required the agent to operate a
merge queue without measured reliability. The accepted scope is reduced to one
isolated implementation batch and a retained-branch handoff. Integration is a
separate maintainer-directed operation.

## Decision

### Explicit request and compatible fallback

`sb-drive --parallel [<limit>]` requests parallel Spec implementation. Bare
`--parallel` and natural-language parallel requests without a number use two.
An explicit limit is a positive integer counting Spec owners, not internal
agents; one uses ordinary sequential execution. Recognized following options
remain separate: `--replan --parallel` and `--parallel --replan` both use two.
Reject invalid explicit limits before dispatch. Ordinary Drive stays sequential.

Use current CLI actionable entries, approved inputs and verification prerequisites
to choose independent Spec-backed implementation items. Verify the actual host
can run complete owners in distinct worktrees at a clean explicit base, retrieve
results and retain branches and partial work. Require compatible project Task
checkpoint policy. If unavailable before dispatch, use ordinary sequential
Drive; generic Skill installation or a host name does not prove isolation.

Missing isolation does not remove ordinary fresh-role dispatch. Decision 0109's
disclosed compatibility path applies only to genuine no-subagent hosts;
registered role failures retain Decision 0129's environment-failure boundary.
The same procedure is installed for Codex, Claude Code and generic. Host support
must be observed on the actual runtime, not inferred from another vendor surface.

### One batch, then handoff

Each worker executes the complete `sb-implement` workflow, with sequential Tasks,
ordinary review, tests, CLI progress and Task checkpoints in its own retained
branch. The original checkout receives no worker changes. No parallel planning,
Direct work, whole-Spec validation, release binding or Release runs with a batch.
`--replan` retains its ordinary authority before a batch; worker findings needing
replanning are handed back for a subsequent operation after the batch stops.

After one batch, verify and report each branch, worktree, base/result commits,
checks, Task progress and blockers, then stop even if all workers succeeded.
Retain successful and blocked work, including dirty partial changes. Partial
startup failure also returns a handoff; it never triggers duplicate sequential
implementation of already-started items.

This route performs no merge, cherry-pick, rebase, candidate integration,
worktree cleanup, second batch, downstream implementation or final validation.
Worker Task completion is local to its branch, not Milestone completion.
Integration and any conflict resolution require a separate maintainer-directed
operation. A later ordinary Drive invocation uses the resulting fresh CLI state
and the existing common-revision final validation rules.

Interrupted-batch recovery is not automated. Known retained results stop new
work for those Specs, including fallback, until the maintainer identifies and
handles them. No persistent queue, new progress schema or Git recovery algorithm
is introduced. This narrows Decision 0168 only for isolated implementation;
Decision 0146's Task ordering remains unchanged.

## Verification

Static tests check shared installed routing and the branch-handoff boundary.
Behavioral DP1 must observe overlapping worker execution, ordinary owner review
and checkpoints, retained results, unchanged original checkout and no dependent
work. DP2 measures unchanged sequential fallback; DP3 measures stopping for known
retained results. Historical runs of the earlier integration design remain
historical evidence, not passes for this revision.

Public user-guide instructions remain deferred until the reduced route is
measured successfully. Missing runtime capabilities are recorded as environment
limitations, not product passes; Codex evidence does not certify Claude Code.
