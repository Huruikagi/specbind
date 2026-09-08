# Proposal: parallel Spec implementation in isolated worktrees

Status: Draft for discussion; not an accepted product contract.

## Problem and proposed scope

Independent Specs can spend most of their implementation time waiting for a
single mutating Drive dispatch. Decision 0146 removed shared-worktree Task
concurrency because diff ownership, review, remediation, generated outputs,
and selective commits became interdependent. Host-managed worktrees now offer
an execution boundary that addresses those problems.

Start with opt-in parallel implementation of independent **Spec-backed Roadmap
items**. Keep each Spec's Tasks sequential and preserve the complete
`sb-implement` cycle. Integrate results serially. Do not parallelize planning,
replanning, Direct items, finalization, or release through this new mode.
Direct items remain eligible for ordinary sequential execution between batches;
excluding them initially avoids concurrent writes to the shared Roadmap.

This proposal does not enable the feature merely by installing an update.
Ordinary Drive remains sequential. An illustrative `sb-drive --parallel 2`
would authorize bounded isolated dispatch, not gate changes or publication.
The spelling and supported host invocation remain subject to validation.

## Existing contracts retained

- Decision 0146: plan order remains a dependency; one completed Task produces
  one default checkpoint including its execution transition and durable notes.
- Decision 0168 and the current Drive Skill: status supplies actionable work;
  owning Skills retain implementation, review, progress, and checkpoint duties.
- Decisions 0082/0086: all participating implementation converges before final
  Spec validation; accepted evidence binds to a common clean project revision.
- Decisions 0190/0197: file ownership is evidence, not semantic independence;
  verification inputs must actually be available when the Task completes.
- Decision 0144: existing persisted artifacts keep their established meaning.
  No `parallel` Task field or reinterpretation of old plan order is proposed.

The new Decision would replace only Drive's single-mutating-dispatch restriction
for the bounded implementation mode and define integration responsibility.
Task completion remains branch-local state; importing it requires the acceptance
procedure below. It is not a new global completion or release state.

## Authority and ownership

| Owner | Responsibility |
| --- | --- |
| CLI | Existing actionability, dependencies, artifact freshness, guarded lifecycle transitions |
| Drive | Select eligible Specs, delegate whole owning workflows, coordinate serial acceptance, reread state |
| `sb-implement` | Task implementation, fresh review, bounded repair, progress, per-Task checkpoint |
| Host execution support | Isolated checkout, correct starting revision, process/session lifetime, result delivery |
| Git adapter policy | Whether and how reviewed commits may be integrated and retained |
| Validation Skill | Final semantic and mechanical validation on the converged revision |

Drive must not author conflict resolutions or reinterpret failed review itself.
A conflict or semantic integration failure returns to `sb-implement` with the
current approved scope and affected Tasks. Gate defects route to their existing
owners and authority boundaries. A generic parallel invocation grants no replan
or destructive recovery authority.

## Start a bounded batch

1. Read authoritative milestone status from the designated integration checkout.
   Require a clean committed revision for this initial implementation mode.
   Other unrelated worktrees may be dirty; never modify or clean them.
2. Select at most the requested number of already-actionable Spec-backed items.
   Keep status order for deterministic dispatch and integration. Do not infer
   dependency waves from Roadmap prose or release downstream work early.
3. Confirm approved scope, current inputs, available verification data, and
   execution environment. Shared database state, ports, external services,
   generated outputs, and lockfiles need explicit handling where relevant.
   Different owned paths alone do not prove independence. Ambiguity falls back
   to sequential work rather than inventing product interfaces or metadata.
4. Pin the same full integration commit for every worker. Verify the actual
   checkout revision before mutation, not merely a host's selected branch label.
5. Delegate the complete `sb-implement` Skill with item identity, base revision,
   working directory, executable/PATH, instruction paths, authority and limits.
   Internal implementer and reviewer roles stay inside that worker checkout;
   do not give each internal role a different worktree.

Use bounded batches initially: finish and accept the current batch before
launching another. This avoids a persistent dynamic scheduler and limits how
far worker bases drift. One blocked worker need not prevent acceptance of an
independent successful result, provided that no shared prerequisite changed.

## Worker result and serial acceptance

A normal worker runs its existing sequential Tasks through the owning Skill's
terminal handoff. It cannot mutate the integration checkout or integrate another
worker. It does not run final Spec completion validation or release.

Return a run-scoped handoff containing Spec identity, base and result commit
IDs, retained worktree/branch, completed Task IDs, changed paths, verification
results, and blockers. Git and CLI state must independently substantiate it.
A prose success message or a clean checkout alone is insufficient.

For each successful result, in dispatch order:

1. Verify the exact worker base/result and attributable commit range. Inspect
   its changes against approved Task scope. Include only that Spec's Task
   execution transitions, implementation/tests and permitted durable notes.
   Reject foreign progress, upstream artifact changes and release metadata.
2. Reread integration state. Check milestone identity, membership, dependencies,
   approved inputs, and changes since dispatch. A changed approval or relevant
   prerequisite parks the result; unrelated commits require impact judgment,
   not an automatic assumption of validity.
3. Assemble a candidate from current integration HEAD in a separate temporary
   checkout, preserving the worker's per-Task commit units. Apply the project's
   Git policy. Do not expose a partially integrated candidate as the scheduler's
   authoritative checkout.
4. Review the combined candidate for interaction effects and run affected Task
   checks plus required project integration checks. Worker checks remain useful
   evidence, but cannot prove the new combined tree. Conflicts or substantive
   repairs require the owning workflow and fresh review. If Task behavior is no
   longer proved, use existing reopen/repair/complete operations on the candidate;
   do not silently preserve completion or manufacture a new Task.
5. Before updating the integration branch, compare its HEAD with the expected
   value and require its checkout still to be clean. If another session moved
   it, rebuild the candidate against the new revision. Never force an update.
6. Accept only the verified candidate through the Git adapter's allowed method,
   then reread status. Only now may its completed Tasks satisfy descendants.

Branch-local completed Tasks are acceptable because execution transitions do
not change the Task-plan fingerprint. Final Spec completion evidence is not
copied from workers: after all implementation converges, use the existing
common-revision validation and metadata acceptance sequence.

The first implementation should verify a candidate by replaying whole Task
commits. Do not add a second integration ledger or synthetic accepted status.
If replay cannot preserve attribution under a project's Git policy, report that
mode as unsupported rather than silently flattening Task checkpoints.

## Interruption, failure and resume

- A worker with partial changes remains in its worktree. Do not stash, delete,
  commit rejected work, or reset it to let another item proceed.
- Keep every unaccepted result reachable by a named branch or an equivalent
  host retention guarantee; a temporary path or detached HEAD alone is not
  sufficient evidence of durable retention.
- After interruption, reconstruct integrated progress from the integration
  checkout's CLI state. Inspect retained workers by identity, base and diff
  before reusing them. Lost host conversation is not lost implementation, but
  it is not authorization to guess which Spec an unattributed branch belongs to.
- In the initial mode, ask the user to identify ambiguous retained work rather
  than dispatching a duplicate or inventing a persistent queue.
- A host failure parks that worker. An unsafe integration checkout stops
  acceptance and new batches; it does not justify destroying worker results.
- Cleanup follows confirmed acceptance or explicit discard authority, never
  the mere receipt of a completion notification.

## Host-specific feasibility

Official documentation checked 2026-09-08; these are documentation findings,
not executed compatibility tests or a declared minimum supported version.

### Claude Code

Worktree isolation is available on Agent calls and custom subagents. Prefer
explicit isolation on the outer owning-workflow dispatch. Agent Teams are not
required; their name-based routing can otherwise change a subagent into a
teammate sharing the main directory. Verify actual isolation at startup.

The documented default worktree base is the default branch, not necessarily the
approved local revision. `worktree.baseRef: "head"` uses current HEAD; an exact
other base can be prepared through Git. Verify the full commit either way.

Current documentation allows nested subagents (default depth three), which can
support Drive -> implementation owner -> implementer/reviewer. Check the actual
version, depth setting and Agent tool availability before accepting this route.
Use fresh-context review; a conversation fork is not an independent reviewer.

Background permission requests can reach the main session. Environment setup
must be worktree-local: hook `CLAUDE_PROJECT_DIR` remains the original root,
while hook input `cwd` follows the executing directory. Non-interactive `-p`
worktrees are not removed at exit. Changed worker retention and cleanup must be
verified for the selected host mode.

Sources: [worktrees](https://code.claude.com/docs/en/worktrees),
[subagents](https://code.claude.com/docs/en/sub-agents),
[Agent Teams limitations](https://code.claude.com/docs/en/agent-teams#limitations).

### Codex

The app supports worktree chats and explicit starting branches. A worktree fork
can also carry uncommitted changes: this repository's design experiment copied
six unrelated dirty files into the new checkout. Therefore a worktree's existence
is not proof of a clean pinned base. Select committed input explicitly and verify
status and HEAD before dispatch.

App-created workers may start detached and managed worktrees have a host-owned
lifecycle. Retain results explicitly and verify available session-control tools.
App thread support must not be advertised as identical CLI subagent support;
unsupported surfaces continue sequentially. Handoff moves a chat between
checkouts; it is not by itself the SpecBind integration acceptance protocol.

Source: [Codex worktrees](https://learn.chatgpt.com/docs/environments/git-worktrees).

## Verification before accepting the design

Use fresh fixtures and inspect files, commits, CLI state and actual worker cwd:

1. Two independent Specs execute concurrently; each keeps per-Task checkpoints;
   a downstream Spec becomes actionable only after both are accepted.
2. One worker blocks with dirty files; the other can be accepted; blocked work
   survives stopping and resuming the session without a WIP checkpoint.
3. Cleanly applying commits still break a shared behavior: candidate checks
   reject them before authoritative integration progress advances.
4. Approval/input changes and concurrent integration HEAD movement reject stale
   candidates without overwriting another session's work.
5. Restart before acceptance and after acceptance: no duplicate execution,
   foreign progress adoption or lost retained commits.
6. Wrong default base, copied dirty files, missing tools/depth, permission prompt,
   and host cleanup behavior are detected with actionable fallback.
7. All implementation converges and existing Spec completion accepts evidence
   at the common final revision; worker-local evidence never bypasses it.

Run real host tests separately for Claude Code and the supported Codex surface.
Confirm that environment setup and extra integration validation do not erase
elapsed-time savings on a representative two-Spec fixture. Measure wall time,
repair/review repetition, and operational interruptions; no benchmark threshold
or extra permanent artifact is required to make the initial decision.

## Recommended next decision

Accept the execution/acceptance boundary only after the host fixture proves
nested owning-workflow execution, exact-base isolation and result retention.
Keep this document a draft until then. The eventual implementation should add
the smallest host dispatch guidance and Drive procedure necessary, with tests
and generated/installed documentation updated together. Any new CLI projection
must have a demonstrated mechanical consumer; any persistent run state requires
a separate concrete recovery need.
