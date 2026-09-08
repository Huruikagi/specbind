# 0204: Close or transfer lifecycle mutations before clean successors

Status: Accepted

## Context

Several explicit lifecycle commands correctly preserve history by changing only
machine-owned state. Their owning Skills did not always close the resulting Git
delta before handing control to a successor that requires a clean repository.
That made valid recovery and maintenance operations stop after their first
successful mutation.

The affected paths are completion-evidence invalidation before a new completion
preflight, Requirements and scope remediation discovered by Contract Review,
milestone rebaseline before renewed review, and reverse abandonment or
finalization before another workflow. Deleting retained work, editing lifecycle
files by hand, or weakening clean guards would hide the transition rather than
define its owner.

Decision 0203 deliberately carries one proven Design-rewind delta through
independent Design validation. That exception is specific to a phase whose
meaningful checkpoint cannot exist until validation and approval finish. It is
not a general license to carry dirty lifecycle state across phase ownership.

## Decision

Every CLI-owned lifecycle mutation must either remain with one owning phase
until that phase's coherent checkpoint, or be transferred before mutation so
the receiving phase invokes and checkpoints it. A successful terminal mutation
whose successor requires clean state receives its own narrow checkpoint.

The following ownership applies:

| Mutation | Owner and closure |
| --- | --- |
| `spec completion invalidate` | Implementation validation confirms the evidence withdrawal, invokes it, verifies the exact completion-only `spec.yaml` delta, and checkpoints that delta before rerunning completion preflight. Fresh accepted completion metadata is a later checkpoint. |
| Requirements rewind diagnosed by Contract Review | Contract Review presents the exact cost and obtains operation-specific confirmation, then transfers that confirmation and finding to the Requirements phase. The Requirements receiver invokes invalidation, repairs and approves Requirements, and checkpoints the complete phase result. |
| Milestone scope remediation diagnosed by Contract Review | Contract Review proposes the complete scope correction but performs no scope mutation. Discovery owns its normal confirmation, `update-scope`, Brief work, invalidations, and checkpoint; Plan then advances affected Specs before review resumes. |
| `milestone rebaseline` | Discovery owns explicit revision confirmation, invocation, exact Roadmap/review-removal verification, and a narrow checkpoint before renewed Contract Review. |
| `milestone reverse abandon` | Adoption verifies and checkpoints the exact CLI-owned deletion set before ordinary Discovery or urgent work starts. |
| `milestone reverse finalize` | Adoption always verifies and checkpoints the exact finalization write/removal set before reporting a clean handoff or retrying managed Skill retirement. |

Relayed Requirements-rewind confirmation authorizes only the exact operation
presented by the active review run. It is not general delegation and cannot be
reused after the finding, target state, or rewind cost changes. Scope changes
remain Discovery-confirmed because the complete replacement, new Briefs, and
milestone decomposition are Discovery-owned decisions.

For every narrow terminal checkpoint, the Skill reads the active Git adapter,
stages only the verified paths produced by the command, and follows explicit
commit prohibitions. If an adapter is absent or scaffolded, guidance is unsafe,
or the checkpoint fails, the mutation remains valid but the Skill stops before
the clean-dependent successor and reports the exact dirty state. It never
stashes, discards, amends, or folds unrelated work into the checkpoint.

This Decision narrows the orchestration portions of Decisions 0086, 0089,
0100, 0108, and 0181. Decision 0203 remains the only uncommitted lifecycle-delta
exception during independent validation.

## Consequences

- Completion revalidation can withdraw stale evidence and then perform a fresh
  clean preflight without combining withdrawal and new acceptance.
- Contract Review no longer mutates Requirements or scope on behalf of their
  authoring owners.
- Rebaseline, reverse abandonment, and reverse finalization leave an explicit,
  reviewable history boundary before the next clean-gated workflow.
- Completed Tasks, other Specs' progress, and retained plans are never silently
  deleted or invalidated as remediation shortcuts.
- An unavailable checkpoint is a visible stop, not permission to carry an
  unexplained dirty lifecycle delta forward.

## Verification

Focused Skill contract tests fix each owner, confirmation transfer, exact-delta
check, and checkpoint boundary. CLI integration covers completion invalidation,
the dirty preflight stop, checkpoint, and successful fresh preflight. Fresh
forward tests exercise the composed workflows through installed Skills.
