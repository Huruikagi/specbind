# 0126: Persist accepted cc-sdd migration resolutions as CLI-owned state

Status: Accepted

The retained-source lifecycle below is superseded by [Decision
0127](./0127-retire-cc-sdd-source-at-final-cutover.md): successful final
`--apply` removes both the legacy source and this temporary resolution record.

## Context

Decision 0125 requires agent-assisted migration to rejoin deterministic CLI
validation. A valid converted target alone cannot prove how an agent disposed
of every legacy customization: some source material is deliberately omitted,
combined, or rewritten rather than represented by a one-to-one target file.
Continuing to emit the same semantic findings after that reviewed work leaves
`--apply` permanently unreachable.

The CLI must recognize reviewed semantic work without treating agent prose as
authority, trusting caller-supplied fingerprints, or making `.kiro` mutable.

## Decision

- An agent authors a transient strict JSON resolution candidate. The CLI reads
  it only from standard input or a regular non-symlink file outside the project
  worktree.
- `specbind migrate cc-sdd --accept-resolution <path|->` accepts the candidate
  only when it exactly covers every current resolvable semantic finding. Path,
  Git, installation, and unknown-asset safety findings cannot be waived.
- The candidate selects the target language and supported agents, gives a
  non-empty assessment, and records each finding as either `converted` with
  one or more concrete target paths or `not_migrated` with no target paths.
- The selected SpecBind installation must already be converged. Acceptance
  requires a clean committed Git worktree, re-resolves the plan and all input
  fingerprints, and atomically writes
  `.specbind/state/cc-sdd-migration.yaml`.
- The persisted YAML is CLI-owned current project state. It stores the accepted
  timestamp, legacy root, target selection, assessment, exact finding keys,
  dispositions, and CLI-computed SHA-256 fingerprints for source and target
  paths. Agents never hand-edit it.
- A later migration plan suppresses a resolved finding only while the complete
  record is valid, the selected installation remains converged, the same
  finding still exists, and every recorded source and target fingerprint still
  matches. Any mismatch restores the original findings and adds a stale or
  invalid resolution diagnostic.
- The record is not gate evidence, approval evidence, completion evidence, or
  release history. Release and milestone operations do not archive it. It is a
  temporary handshake removed with the legacy source by final `--apply` under
  Decision 0127; Git retains the accepted revision.
- Acceptance itself is a reviewable worktree change. The user commits it before
  `--apply`, which retains the clean committed recovery boundary before exact
  legacy-agent cleanup.

## Consequences

- Guided migration can converge without teaching Rust to reproduce semantic
  agent judgment.
- Source or target drift fails closed and returns the project to guided review.
- The committed record explains why a persistent legacy tree no longer emits
  findings, without granting that record lifecycle authority elsewhere.
- Agent-assisted migration has three explicit checkpoints: commit the converted
  target, accept and commit the resolution record, then run `--apply` for the
  remaining deterministic cleanup.
