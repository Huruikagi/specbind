# 0180: Delegate binary updates to installation clients

Status: Accepted

This supersedes Decision 0077's reservation of `specbind update` for a future
self-update command and resolves its Issue 13 follow-up. The project-asset
refresh semantics of `specbind install` remain unchanged.

## Context

Decision 0077 separated machine-level binary installation from project-local
asset installation, while reserving an eventual self-update command. Decision
0130 later added mise's GitHub backend over the same GitHub Release archives.

A self-updater would need to infer whether mise, an installer script, or a
custom directory owns the running executable. It would duplicate version
selection, checksum and provenance handling, Windows executable replacement,
rollback, and PATH boundaries already owned by those installation clients.
That additional owner does not improve the project-local refresh contract.

## Decision

SpecBind does not provide or reserve a `specbind update` self-update surface.
The client that installed the binary owns its update:

- mise is the primary documented route. `mise upgrade
  github:Huruikagi/specbind` advances the configured selector, while `mise use
  github:Huruikagi/specbind@<version>` explicitly changes the selected version;
- projects commit applicable `mise.toml` and `mise.lock` changes so the team and
  CI consume the same selected release and distribution inputs; and
- users of the PowerShell or shell installer rerun that installer. SpecBind
  does not detect, replace, remove, or take ownership of package-manager or
  custom binary installations.

Binary update remains separate from project-asset refresh. After the new
binary is selected, a maintainer runs `specbind install --dry-run`, reviews the
plan, and then runs `specbind install`. The new executable is the source of the
embedded assets; running the old executable cannot refresh a project to a newer
asset set.

The existing installation safety boundary remains authoritative:

- replacement, movement, or removal of existing project files requires at
  least one commit and a clean worktree;
- product-managed Skills and marked instruction blocks advance to the current
  embedded set, including planned removal of retired targets;
- existing project-owned templates, Rules, and Adapters are retained, while
  missing newly introduced defaults may be created; and
- Specs, lifecycle state, and release history are not binary-update targets.

Because mise may modify `mise.toml` or `mise.lock`, the public procedure commits
that binary-selection change before applying a project refresh that needs the
clean-worktree guard. Refreshed product assets are reviewed and committed as a
separate project change.

## Consequences

- Binary ownership remains unambiguous and SpecBind does not compete with mise
  or installer scripts.
- One update guide covers version selection, lockfile review, embedded asset
  refresh, and the Git boundary between them.
- A future self-update capability would require a new Decision with a concrete
  installation-ownership need; the absent `update` command name is not a
  compatibility reservation.

## Verification

The paired English and Japanese update guides document the same sequence,
ownership table, and failure boundary. Strict MkDocs and Decision-index checks
verify the published paths and Decision registration.
