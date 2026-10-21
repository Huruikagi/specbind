# 0233: CLI checks and applies an explicit completion set

Status: Accepted

## Decision

Add `spec completion batch preflight --spec <id>` (repeat the option),
`spec completion batch accept --evidence <file|->`, and
`spec completion batch verify --evidence <file|->`. Successful operations return
command-specific JSON. After valid input is parsed, acceptance failures also
return a nonzero JSON report; input, preflight and verification failures retain
the usual CLI diagnostics. The explicit nonempty target set is never inferred
from the Roadmap.
Duplicate, noncanonical or out-of-milestone targets are rejected. A single
target is supported; existing individual completion commands remain compatible.

Preflight checks every target in implementation at one clean committed HEAD,
with the existing convergence, earlier-gate and Contract Review guards. It
returns that full revision and exact project-relative metadata paths. It does
not execute checks or establish a semantic GO verdict.

The strict transient `completion-batch/v1` candidate contains schemaVersion,
one implementationRevision, and targets with spec and mechanicalChecks. Each
target uses the same successful-command evidence contract as individual
completion. It carries no persisted GO flag, timestamp or inferred authority.
The validation owner assesses every target before submitting any acceptance.

Accept validates the entire candidate and all current targets, prepares all
metadata in memory, and rechecks the clean unchanged revision before its first
write. Before each guarded file replacement it revalidates lifecycle inputs,
the fixed HEAD, unchanged target bytes, and the exact preceding writes. It
allows only its own metadata changes, with no staged or unrelated changes.
Ordinary invalid inputs therefore write nothing. Files are replaced atomically
individually; this is not an atomic multi-file transaction or a concurrency lock.

On a write or intervening-state failure, stop and return accepted, pending,
the original revision, target paths and diagnostics with a nonzero exit status.
Never roll back accepted evidence, auto-retry, or commit a subset. The report
describes writes confirmed by this invocation, not unknown concurrent work.
Process termination can prevent a report; inspect actual state before recovery.
No durable batch ledger or automatic partial-resume authority is introduced.

The Git adapter remains responsible for one narrow metadata checkpoint. Verify
requires a clean HEAD whose sole parent is the candidate's implementation
revision, exactly the supplied target paths, and only implementation-to-
release_ready plus matching completion evidence changes in their wire models.
Every target must still have fresh completion. Missing, extra, mixed, split or
merged checkpoints fail without mutation. Reusing a candidate at another HEAD
does not authorize acceptance. Git adapters that prohibit commits retain the
explicit uncommitted handoff; they do not run checkpoint verification.

This replaces Decision 0220's agent-managed sequential acceptance loop, common
revision comparison and checkpoint-delta inspection with CLI operations. Actual
verification commands, per-Spec meaning, original authority and GO judgments
remain with the independent validation owner. Drive keeps its existing routing
and isolated-owner contract. No persisted Spec schema or freshness exception
changes, and the CLI never runs Git commit or project verification commands.

## Verification

Cover two-target acceptance at one revision, invalid later targets with no
writes, strict input and duplicate rejection, dirty/stale/changed HEAD stops,
precise injected mid-write failure reports, and checkpoint verification against
extra files, earlier-gate edits, split commits and stale evidence. Generate and
check the transient schema. Fresh grouped-validation forward tests must use
the installed batch surface and leave both completions fresh after one commit.
