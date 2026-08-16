# 0029: Bind completion validation to an unchanged Git revision

Status: Accepted

## Context

Feature-level validation may run several mechanical commands and semantic reviews over a non-trivial period. A result is not trustworthy if the repository changes between the start of validation and the lifecycle transition, or if the recorded evidence names a commit that did not contain the validated working tree.

The validation agent can orchestrate tests and judgment, but the Rust CLI owns deterministic repository inspection and the guarded `IMPLEMENTATION_VALIDATED` transition. The agent must not be the sole authority for either the baseline revision or its freshness.

## Decision

Completion validation uses the Decision 0086 `specbind spec completion preflight <spec>` and `specbind spec completion accept <spec> --evidence <path|->` handshake. Stale accepted completion is first cleared through `specbind spec completion invalidate <spec>`.

### Preflight

- The validation skill asks the CLI to begin completion validation for one explicit active spec.
- The CLI requires a Git repository, resolves the full current `HEAD` commit object ID as the scalar `implementation_revision` accepted by Decision 0031, and rejects a repository without a commit.
- Before returning the baseline, the CLI requires a clean repository status: no staged changes, tracked worktree changes, untracked files, or dirty submodules. Ignored build and tool outputs do not make the repository dirty.
- Preflight is read-only. It returns only the full `implementation_revision`; that baseline is run-scoped candidate data, not persisted approval or completion evidence.

### Validation

- The skill runs the full active-spec validation scope against that checkout, including required mechanical commands and semantic coverage, integration, and design assessments. Under Decisions 0050 and 0052, it consumes the current global contract-impact and downstream-review result through the CLI instead of duplicating those assessments in per-spec completion evidence.
- A validation command that changes tracked, staged, untracked, or submodule state prevents acceptance. The change must be reconciled and committed, then the complete validation rerun from a new preflight baseline.
- `NO-GO` and `MANUAL_VERIFY_REQUIRED` never request the state transition. They report remediation or missing manual evidence while the spec remains in `implementation`.
- Under Decision 0030, unsuccessful and manual-required results remain run-scoped output and are never written to `spec.yaml` as attempt history.

### Guarded acceptance

- For `GO`, the skill submits the preflight `implementation_revision` and structured candidate validation evidence to the CLI.
- Immediately before mutation, the CLI independently requires the same `HEAD`, unchanged task-plan and lifecycle input revisions, current prior gates, and zero pending or blocked executable tasks. The first acceptance requires a clean repository. Later acceptances at that same revision may coexist only with already-validated, CLI-shaped completion metadata mutations for other participating Specs; any other staged, tracked, untracked, or submodule change rejects the candidate.
- Any mismatch rejects the candidate without partially recording completion evidence or changing lifecycle state.
- On success, the CLI atomically records completion evidence and transitions the spec from `implementation` to `release_ready`.
- The CLI's own `spec.yaml` evidence mutation occurs only after the clean-state check. Multiple Specs validated at one revision may be accepted sequentially before their completion-only metadata changes are committed together.

### After acceptance

- The completion evidence remains bound to the validated implementation commit, even when the subsequent commit contains the CLI-generated `spec.yaml` evidence mutation.
- A later freshness check may accept such a successor commit only when every change since `implementation_revision` is an expected SpecBind completion-evidence mutation. Any implementation, spec input, task, configuration, or other repository-content change requires `COMPLETION_INVALIDATED` and a new validation handshake.
- Release preflight rechecks this relationship; Decision 0031 excludes a branch name, abbreviated hash, timestamp, conversation report, or earlier command output as a substitute for the recorded full commit identity.

## Consequences

- Validation evidence identifies an exact reproducible code state instead of a moving branch or dirty checkout.
- Changes during validation fail closed and cannot be hidden by an agent's stale success report.
- Validation commands that rewrite snapshots, formatting, generated tracked files, or lockfiles require a commit and rerun.
- Completion metadata can remain repository-managed without pretending that its post-validation commit was itself the implementation revision.
- Initial completion validation is Git-dependent. Supporting another version-control or immutable-snapshot provider requires a later adapter and schema decision with equivalent freshness guarantees.
