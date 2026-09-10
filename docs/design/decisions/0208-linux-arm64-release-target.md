# 0208: Add a tested Linux ARM64 release target

Status: Accepted

## Context

[Issue #12](https://github.com/Huruikagi/specbind/issues/12) requires native
runtime evidence before extending distribution. Decision 0163 added macOS
ARM64; Linux ARM64 can now use the GitHub-hosted `ubuntu-24.04-arm` runner.

## Decision

Add `aarch64-unknown-linux-gnu` to the native release matrix without changing
the pinned Rust toolchain. Run formatting, schema generation checks, Clippy,
the full test suite, and the release build on that runner. Package `specbind`,
`README.md`, and `LICENSE` as
`specbind-v<VERSION>-aarch64-unknown-linux-gnu.tar.gz`.

Extract and run each Linux archive on its native runner, checking the version,
an embedded schema read, and `install --dry-run --agent codex --language en`
in a temporary Git project. Record the runner image and actual architecture
in release notes. Require the four exact target archives before generating
and verifying `SHA256SUMS`; a missing archive stops publication.

The shell installer maps `Linux:aarch64` and `Linux:arm64` to the new GNU
archive and retains checksum and installed-version verification. Unsupported
platform/architecture combinations still fail before download. After
publication, test both the shell installer and the mise GitHub backend on
the native ARM64 runner.

Manual release preflight validates all four archives without publishing.
Installer and mise verification against published assets remains a
tag-triggered post-publication check. Linux ARM64 assets first become
available with the next release; existing releases are not modified.

## Consequences

- Linux ARM64 has the same native build and runtime publication gate as
  existing targets, rather than relying on cross-compilation alone.
- Decisions 0077, 0124, 0130, and 0163 are extended only for Linux ARM64 GNU.
  macOS Intel remains deferred under Issue #12. musl, universal binaries,
  signing, and notarization are not added by this decision.

## Implementation status

Implemented in the release and mise workflows, shell installer, installer
regression checks, and distribution documentation.
