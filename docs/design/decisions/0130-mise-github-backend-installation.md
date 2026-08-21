# 0130: Support installation through the mise GitHub backend

Status: Accepted

## Context

[Decision 0124](./0124-pre-1.0-binary-release-line.md) makes GitHub Releases
the only primary distribution channel and publishes conventionally named
Windows x64 and Linux x64 archives. Users who already manage developer tools
with mise should not need a separate SpecBind-specific plugin or installer to
consume those same release assets.

The mise GitHub backend can list stable GitHub Releases, select an archive for
the current operating system and architecture, verify the selected release
asset through its supported GitHub verification mechanisms, extract it, and
place the executable on the mise-managed path.

## Decision

SpecBind supports this additional installation client:

```sh
mise use github:Huruikagi/specbind
```

This does not create another distribution channel. The command consumes the
same GitHub Release archives accepted by Decision 0124, and the release
workflow remains the source of those archives and `SHA256SUMS`. No custom mise
plugin or mise registry entry is required.

The supported platforms remain Windows x64 and Linux x64 tested under WSL2.
The unqualified command selects the latest stable version that is eligible
under the user's mise settings. A version may be selected explicitly with the
normal backend syntax, for example:

```sh
mise use github:Huruikagi/specbind@0.1.0
```

mise applies a minimum release age to fuzzy requests such as `latest` by
default. A newly published first stable release may therefore require an
explicit version until it becomes eligible. This is mise's supply-chain safety
policy, not a SpecBind prerelease rule. SpecBind does not ask users to disable
that policy globally.

The tag-triggered release workflow smoke-tests the unqualified `mise use`
command on Windows x64 and Linux x64 after publication. That isolated test sets
the minimum release age to zero so it validates the release that triggered the
workflow rather than a previous eligible stable release.

## Consequences

- Existing mise users get a one-command installation that also records
  SpecBind in their selected mise configuration.
- SpecBind keeps one binary source of truth and does not maintain a custom mise
  plugin, registry entry, or second set of packages.
- Release archive naming and executable discovery become behavior covered by
  post-publication smoke tests on both supported platforms.
- The installer scripts remain available for users who do not use mise and
  retain their Decision 0077 checksum and installation-directory contract.
