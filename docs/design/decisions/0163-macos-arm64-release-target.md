# 0163: Add a tested macOS ARM64 release target

Status: Accepted

## Context

Decisions 0077 and 0124 limited binary distribution to Windows x64 and Linux
x64 until another native environment could build and run a release artifact.
GitHub Actions now provides a standard hosted Apple Silicon runner, so macOS
ARM64 can meet the same native-build and fail-closed publication boundary.

Discovering a platform failure only after pushing a release tag is needlessly
late. The release matrix therefore also needs a manually dispatched preflight
that exercises the exact build, archive, and checksum path without publishing.

## Decision

### One additional supported target

SpecBind releases add the native `aarch64-apple-darwin` target on the
`macos-15` GitHub-hosted ARM64 runner. Its archive is
`specbind-v<VERSION>-aarch64-apple-darwin.tar.gz` and contains `specbind`,
`README.md`, and `LICENSE`.

The release workflow builds the target natively, runs the complete Rust gates,
extracts the archive, and verifies:

- the reported executable version;
- an embedded schema read; and
- a representative project `install --dry-run` command.

`SHA256SUMS` covers the Windows x64, Linux x64, and macOS ARM64 archives. A
missing target or checksum stops publication. The release notes record the
native macOS version and architecture used for the build and smoke test.

### Preflight and installation

The same release workflow accepts a manually supplied Cargo version. A manual
run verifies the three build artifacts and checksums but cannot publish a
GitHub Release. A tag-triggered run retains publication authority.

On macOS ARM64, `install.sh` selects the `aarch64-apple-darwin` archive,
verifies it through `SHA256SUMS`, and installs it to `$HOME/.local/bin` unless
overridden. After publication, CI runs both `install.sh` and mise against the
released artifact on the hosted macOS ARM64 runner.

### Deferred targets and signing

macOS Intel and Linux ARM64 remain unsupported until they have their own native
build and runtime verification. macOS code signing, notarization, and a
universal binary remain separate future decisions. The unsigned archive is an
explicit limitation, not an implied signing claim.

## Consequences

- A release can be preflighted on every supported target before its tag exists.
- Apple Silicon users receive a checksum-verified native archive and installer
  path backed by runtime evidence.
- Adding one hosted target does not imply macOS Intel, Linux ARM64, universal
  binaries, code signing, or notarization support.

## Implementation status

Implemented by the release workflow, shell installer target selection, release
operations, and post-publication macOS installer and mise smoke tests.
