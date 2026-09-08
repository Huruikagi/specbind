# 0201: Opt in to parallel Spec implementation with sequential host fallback

Status: Accepted

## Context

Decision 0146 removed Task-level parallelism because shared-worktree ownership,
review, remediation and checkpoints were interdependent. Decision 0168 chose a
single mutating Drive owner initially. Isolated host execution can now support
independent Specs without changing sequential Task-plan semantics. Generic
Skill installation, however, makes no promise of worktree/session support.

## Decision

### Optional execution strategy, compatible default

`sb-drive --parallel <limit>` requests bounded parallel Spec implementation.
The limit is a positive integer; one is sequential. An explicit natural-language
parallel request without a limit uses two. Invalid option values are diagnosed
before dispatch. These are Skill arguments, not new CLI command arguments.
Ordinary Drive remains sequential; upgrades introduce no persistent setting.

Select only already-actionable independent Spec-backed implementation entries
from current integration-checkout status. Each worker executes the complete
`sb-implement` workflow and keeps its Tasks sequential with per-Task review and
checkpoints. Planning, Direct items, replanning, final validation and release
binding do not run alongside a batch. Release remains outside Drive.

### Capability-dependent fallback

Before batching, verify actual isolated concurrent dispatch, explicit base/cwd,
owning-workflow and fresh-review capability, environment setup, result retrieval,
retention, compatible Git policy and coordinated serial integration. Neither
an agent name nor a shell supporting `git worktree` proves those capabilities.

If support is missing or unverified, report why once and continue the existing
sequential workflow. This applies to generic Skill-only hosts and equally to
Codex or Claude Code surfaces lacking the required tools. Do not require a new
plugin, alter permissions, or remove existing review/approval guards. A real
worktree/environment blocker still follows its existing stop semantics.

The same agent-neutral procedure is installed for all three Agent profiles.
Host checks belong to conditional execution guidance, not separate progress
schemas, role identities, persistent configuration or a host-version allowlist.
Internal roles stay in the owning worker checkout; fresh review does not inherit
implementation conversation history. Unsupported host modes are not advertised
as automatically supported based on another surface from the same vendor.

### Integration is the acceptance boundary

Pin full clean base revisions before worker mutation. Copied dirty changes,
default-branch drift and detached/disposable results must be detected. Keep
unaccepted work reachable and attributable across interruptions.

Drive coordinates serial acceptance through existing owning workflows and Git
policy. Verify a result's complete attributable commit range and current approved
inputs. Replay whole Task checkpoint units in a separate candidate based on
current integration HEAD. Freshly review interaction effects and run affected
Task/project checks there before a coordinated, clean, expected-base fast-forward.
If current HEAD changes or exclusive write coordination cannot be established,
stop acceptance rather than overwriting another writer. Failed candidates never
advance the integration checkout's authoritative Task progress.

The implementation owner renews invalid Task proof through existing explicit
reopen/implement/review/complete operations and preserves retry limits. Unresolved
replay conflicts are retained attention; Drive does not author conflict fixes or
bypass owning-workflow guards. Upstream defects return to their owners. Even when
`--replan` is authorized, wait for the batch to quiesce and reassess retained
results after changed approvals; workers never replan independently.

Only accepted integration-checkout status can unlock descendants. Branch-local
Task completion keeps its existing interpretation and does not count as final
Spec validation. After all implementation converges, Decisions 0082/0086's common
clean revision validation and acceptance handshake remain mandatory.

### Interruption and scope

Retain blocked workers with their partial changes and accepted Task checkpoints;
never manufacture WIP commits or discard results to make scheduling easier.
Another independent successful worker can be integrated. Reconstruct integrated
progress from CLI state and identify retained results from real refs/diffs/host
sessions before any duplicate dispatch, including on sequential fallback.
Ambiguous retained work needs attribution, not a new persistent run ledger.

Cleanup requires verified acceptance of all useful changes or explicit discard
authority. No new Task state, integration ledger, gate, CLI mutation or schema is
introduced. This Decision supersedes only Decision 0168's single-mutating-owner
restriction for this opt-in mode; Decision 0146's Task ordering remains intact.

## Verification

Skill tests check opt-in routing and identical installed procedures for Codex,
Claude Code and generic, including sequential fallback and integration guards.
Behavioral scenarios cover isolated batching and unsupported-host fallback;
record the actual runtime and distinguish missing host capabilities from product
failures. Passing a Codex fixture does not certify Claude Code execution.
