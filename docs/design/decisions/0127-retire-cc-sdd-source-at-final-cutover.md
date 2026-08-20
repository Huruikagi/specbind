# 0127: Retire the Git-tracked cc-sdd source at final cutover

Status: Accepted

## Context

Decisions 0077 and 0125 originally preserved the `.kiro` source after
migration, while Decision 0126 retained its resolution record for as long as
that source remained. The migration workflow now requires a committed, clean
Git recovery boundary before mutation. Keeping the old source and new workflow
together after successful validation adds ambiguity without adding recovery:
Git already owns the recoverable history.

## Decision

- A finding-free `specbind migrate cc-sdd --apply` is the explicit final
  cutover operation. It installs or verifies the selected SpecBind target and
  then retires the complete configured cc-sdd source root, `.cc-sdd.json`,
  exact known legacy agent skills, and the temporary accepted migration
  resolution record.
- Before any installation or deletion, the CLI requires a committed, clean
  worktree and recursively proves that every file below every cleanup target is
  Git-tracked. Ignored, untracked, linked, reparse-point, changed, or
  out-of-project targets stop the operation without writes.
- Git is the only backup and recovery mechanism. The CLI creates no backup
  tree. A filesystem failure during final cleanup is recovered by restoring the
  cutover commit before retrying.
- The configured legacy source root is deleted only after target planning and
  validation. The target `.specbind` tree and project-owned converted artifacts
  are never cleanup targets.
- Root `AGENTS.md` and `CLAUDE.md` remain project-owned. Exact product-known
  legacy blocks may be removed mechanically in the future, but edited or
  ambiguous instructions must be semantically cleaned and committed before
  final apply; Git recoverability does not authorize guessed text deletion.
- After successful cutover, running the migration command again against a
  current SpecBind installation with no default cc-sdd source returns
  `NO_CHANGE CC_SDD_MIGRATION_COMPLETE`.

This decision supersedes only the post-cutover source-preservation clauses of
Decisions 0077, 0093, 0125, and 0126. Their classification, read-only planning,
semantic-resolution, target-validation, and no-invented-evidence boundaries
remain accepted.

## Consequences

- A completed project has one active workflow rather than parallel `.kiro` and
  `.specbind` sources.
- The accepted resolution record is a migration handshake, not permanent
  project state or audit history; Git retains its accepted revision.
- Repositories containing ignored machine-local data below the legacy root
  must either commit, relocate, or deliberately remove that data before final
  cutover.
