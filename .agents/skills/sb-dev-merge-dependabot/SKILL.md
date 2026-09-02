---
name: sb-dev-merge-dependabot
description: Review and integrate Dependabot pull requests in Huruikagi/specbind one at a time, with current-branch checks, main CI verification, and affected dependency or toolchain validation. Use when asked to handle or merge SpecBind Dependabot PRs; do not use for dependency upgrades that have no Dependabot PR.
---

# Merge SpecBind Dependabot updates

This is a maintenance workflow for the SpecBind repository itself. It is not
installed into consumer projects. A request to inspect or assess updates is
read-only; merge, close, commit, and push only when the user asks to incorporate
or otherwise act on the PRs.

## Establish the current set

Start from a clean, synchronized `main` and preserve unrelated worktree changes.
Confirm `origin` instead of assuming the repository owner, then use the available
GitHub integration or authenticated `gh` to list open PRs authored by Dependabot.
Read each selected PR's body, changed files, commits, merge state, and complete
check rollup.

Classify each PR by the surface it changes and the compatibility question it
raises. For Cargo updates, inspect direct feature selection, release notes for
breaking changes, affected code paths, `Cargo.toml`, and `Cargo.lock`. For a Rust
toolchain update, inspect `rust-toolchain.toml`, workspace `rust-version`, and
the corresponding contributor documentation. Do not treat a green historical
check as proof against the current `main`.

Choose an order that makes failures attributable. Updates sharing a manifest or
lockfile are merged one at a time, normally from the smallest and least risky
change to the largest compatibility surface. State any update that needs a
product or compatibility decision before mutating GitHub.

## Refresh and merge one PR at a time

Before each merge:

1. Update or rebase the PR branch onto the current `main` when it is behind.
2. Re-read the PR after the refresh. Dependabot may change the resolved version,
   lockfile, title, or commit while rebasing.
3. Wait for every required check on the refreshed head. Do not run the merge
   command while a check is pending, failing, cancelled, or stale, even when
   GitHub reports the branch as mergeable.
4. Confirm the refreshed diff still matches the classification, then squash
   merge the PR.
5. Identify the exact push workflow for the resulting `main` commit and wait for
   it to succeed before advancing to the next PR.

Stop the affected update if the refreshed diff expands unexpectedly, a required
check fails, the branch conflicts, or the compatibility decision is unresolved.
Other independent updates may continue only when they do not share the failing
surface or lockfile.

## Keep Rust toolchain and MSRV decisions explicit

`tools/specbind/rust-toolchain.toml` selects the development and CI toolchain.
The workspace `package.rust-version` declares the minimum supported Rust version
(MSRV). They may differ.

Do not raise MSRV merely because Dependabot raised the development toolchain.
Preserve the existing `rust-version` unless new source or an accepted dependency
requires a newer compiler, or the user explicitly chooses a compatibility
change. If the versions differ, make contributor documentation describe both
roles accurately. Validate with the development toolchain and run the full
locked test suite with the retained MSRV:

```sh
cargo +<msrv> test --locked --workspace --all-features
```

If the MSRV must rise, update `rust-version` and its documentation together and
report the compatibility change separately from the routine toolchain bump.

## Validate the integrated Rust state

After all selected Cargo or toolchain PRs are on local `main`, run from
`tools/specbind/`:

```sh
cargo fmt --all -- --check
cargo run --locked --example generate_schemas -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
cargo build --locked --workspace --release
```

Use the repository validation guidance for any additional changed surface. A
Cargo-only update does not by itself require TypeScript, documentation-site, or
skill forward tests. Never hand-edit generated schemas to make the check pass.

If integration reveals a necessary source or documentation adjustment, keep it
narrow, rerun the affected checks, commit it as a separate completed unit on
`main`, and push it according to the repository workflow. Do not publish a
release or create follow-up Issues unless separately requested.

## Finish from current evidence

Re-list open Dependabot PRs for the requested scope, verify `main` matches
`origin/main`, and confirm the worktree is clean. Report:

- each merged or deliberately unmerged PR and its final resolved version;
- the compatibility judgment for non-trivial updates;
- the exact successful `main` CI runs and local validation;
- any follow-up commit;
- remaining open Dependabot PRs and the reason each remains.

