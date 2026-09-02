# 0184: Route explicit updates through sb-configure

Status: Accepted

## Context

[Decision 0180](./0180-delegate-binary-updates-to-installation-clients.md)
assigns binary ownership to mise or the original installer and keeps binary
selection separate from project-asset refresh. The paired public guides explain
that workflow to maintainers, but an installed Agent still lacks an executable
procedure for proving ownership, preserving an exact selector, crossing the two
Git checkpoints, and continuing safely after `specbind install` replaces the
active Skill package.

A separate update Skill would duplicate `sb-configure` ownership of
installation and aftercare. Loading the full update procedure for unrelated
configuration would also defeat the progressive package boundary.

## Decision

- `sb-configure` remains the sole installed entry point for configuration and
  update coordination. Its discovery description explicitly includes a
  maintainer-requested SpecBind binary or update-coupled project-asset refresh.
- The package carries `references/update.md` and routes to it only for an
  explicit binary update, mise-selected version change, or project-asset
  refresh requested as part of that update. There is no automatic check,
  background network activity, or `specbind update` command.
- Before invoking mise, the Agent must prove from active mise configuration
  that the selected project uses the `github:Huruikagi/specbind` backend. An
  executable name or PATH location is insufficient. Unproved ownership routes
  to the original installation client and public guide.
- The configured selector is preserved. A moving selector may be advanced by
  `mise upgrade`; an exact pin requires an explicit target and uses `mise use`.
  The Agent never selects a prerelease, downgrade, different selector, or
  weakened mise safety policy implicitly.
- Binary selection and project-asset refresh are two distinct Git workflow
  units. The active Git adapter controls each narrow local checkpoint. The
  first must leave the worktree clean before a replacing or removing install
  plan; the second contains only the reviewed refresh paths.
- The explicit update request authorizes the applicable installation-client
  operation and the exact reviewed `specbind install` plan. It does not
  authorize push, branch or history changes, releases, deployment, destructive
  removal outside the reviewed product-managed plan, or changes to
  project-owned settings and durable artifacts.
- After `specbind install`, the Agent must read the newly installed
  `sb-configure/SKILL.md`, `references/update.md`, and
  `references/aftercare.md` for its active Agent target before continuing. A
  missing or unreadable new package stops the run; cached pre-update
  instructions cannot supply aftercare.

## Consequences

- A natural explicit update request can select one existing Skill and execute
  the documented workflow without making binary ownership ambiguous.
- Progressive disclosure keeps network and mutation instructions out of
  ordinary configuration runs.
- Self-replacement of the procedure becomes an explicit continuation boundary
  instead of an implicit assumption.
- Non-mise installations remain reader-directed because the Agent cannot prove
  or safely automate their installation-client contract generically.

## Verification

Mechanical tests prove direct routing, package rendering for both Agent target
layouts, guarded install refresh and removal, command ordering, ownership and
exact-pin boundaries, two checkpoints, and mandatory post-replacement reload.
A fresh-fixture forward test exercises natural discovery, refusal without
proved ownership or an exact target, controlled two-phase sequencing, and
continuation from the newly installed package.
