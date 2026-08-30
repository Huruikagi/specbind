# 0080: Fix v1 Task, Contract, and completion details

Status: Accepted

[Decision 0146](./0146-sequential-v1-tasks-and-per-task-checkpoints.md)
supersedes Task-level concurrent actionability. Boundaries remain scope and
review evidence, not a scheduling claim.

[Decision 0165](./0165-release-binding-preserves-completion.md) narrows the
project-revision rule for one exact active-Roadmap `target_release` bind or
rebind. That transition preserves completion freshness; unrelated paths and
other Roadmap changes remain stale.

## Context

The accepted structured Task plan leaves `boundaries` and `contracts` as unconstrained strings, assigns semantic completion-criteria judgment partly to Rust, and omits working directory from mechanical evidence. The Git completion handshake also returns redundant fingerprints even though every authoritative input is version-controlled and the complete worktree must remain clean.

## Decision

### Task plan

- `boundaries` is an optional set of primary change scopes relative to the SpecBind project root, not the current Git repository or shell working directory.
- A boundary is either an exact portable path or a directory subtree ending in `/**`. V1 supports no other `*`, `?`, character-class, or mid-path `**` syntax.
- Boundary comparison is lexical, normalized to POSIX separators, and ASCII case-insensitive for portability. The CLI does not interpret nested submodule ownership.
- Overlap between concurrently actionable `parallel: true` Tasks is a warning, not automatic rejection. Paths cannot prove safety for generated outputs, databases, services, or semantic conflicts; the agent owns the final parallel judgment.
- `contracts` optionally names current entries in the same Spec as `<section>/<entry-id>`, for example `exports/checkout-result`. Other Specs are reached through the local Contract's `Consumes` entries and Roadmap dependencies.
- A supplied Contract reference must resolve. V1 does not require machine coverage of every changed or deleted Contract entry; Task review owns that semantic coverage.
- Rust validates explicit `completion_criteria` shape but never claims that omission is semantically justified. Task review requires criteria whenever title, details, project checks, and approved artifacts do not make completion unambiguous.
- Task review modes remain run-scoped. Spec-backed implementation defaults to `required`; Direct implementation defaults to `inline` and may be strengthened automatically based on risk.
- A blocked Task stops the current Spec run whenever partial source changes remain. Later independent Tasks may continue only when the worktree is clean and the agent confirms no dependency or boundary conflict. The workflow never automatically resets, stashes, or creates a WIP commit.

### Contract references and paths

- Canonical Spec IDs, Direct IDs, collection `artifact_id` values, Contract entry IDs, and `Consumes` local IDs are lowercase kebab-case values from 1 through 64 characters in v1.
- `Consumes` may target `owns`, `exports`, `invariants`, or `file-ownership`. A `Consumes` entry cannot target another `consumes` entry; it names the final owned boundary directly.
- File Ownership uses the same exact-path or terminal-`/**` path subset as Task boundaries. Overlap detection is ASCII case-insensitive and remains a warning for semantic review.

### Completion evidence and Direct completion

- A completion preflight returns only the full clean project `HEAD` as `implementation_revision`. The guarded acceptance call receives that revision and candidate mechanical checks; the CLI recomputes current gates, task completion, review freshness, and repository cleanliness.
- Returning task-plan or lifecycle fingerprints to the agent is unnecessary because any tracked change either changes `HEAD` when committed or violates the full clean-worktree guard when uncommitted.
- Each mechanical check may contain optional `working_directory`, a portable project-root-relative POSIX path. Omission means the SpecBind project root.
- Completion evidence remains project-revision-scoped. A later non-metadata project commit makes earlier per-Spec completion evidence stale even if changed paths appear unrelated. V1 does not infer semantic non-impact from path boundaries.
- A milestone therefore converges by implementing all items and rerunning `specbind-validate-implementation` only for Specs whose evidence is stale at the final code revision. A future milestone orchestrator may schedule these validations more efficiently.
- Direct completion reuses the same run-scoped clean-revision handshake. After verification, the CLI rechecks the revision, cleanliness, Roadmap identity, and dependency readiness, then persists only `status: completed`; it stores no Direct revision or command evidence.
- Decision 0086 fixes the public Direct handshake as `specbind milestone direct preflight <direct>` followed by `specbind milestone direct complete <direct> --implementation-revision <revision>`.
- For implementation ordering, a Direct dependency is satisfied by `status: completed`. A Spec-backed dependency is satisfied when all its Tasks are completed and the implementation is clean and committed; `release_ready` is not required. Decision 0082 defines the complete phase-relative semantics and final validation convergence barrier.

## Consequences

- Subagents receive concrete but non-authoritative path scopes, while semantic concurrency remains conservative.
- Task-to-Contract traceability follows one local path without inventing cross-Spec Task IDs.
- Completion handshakes stay small and text-CLI-friendly without weakening Git freshness.
- Multi-Spec milestones may require final revalidation, an explicit v1 tradeoff for whole-project safety.
