# SpecBind Repository Guidelines

## Repository Purpose

- This repository develops SpecBind itself. It is not a consumer project using SpecBind to deliver an application.
- SpecBind was bootstrapped from `gotalab/cc-sdd` and has been detached from the GitHub fork network.
- Treat inherited cc-sdd code and documentation as migration inputs, not as the final SpecBind interface.
- Prefer SpecBind-specific commands, workflows, terminology, and compatibility decisions over drop-in cc-sdd compatibility unless compatibility is explicitly required.

## Source Layout

- `tools/cc-sdd/src/` — inherited TypeScript CLI retained as a migration and comparison oracle
- `tools/cc-sdd/test/` — inherited TypeScript automated tests
- `tools/cc-sdd/templates/` — current files installed into consumer projects
- `tools/specbind/` — canonical Rust CLI workspace
- `tools/specbind/src/` — Rust CLI implementation
- `tools/specbind/src/yaml.rs` — restricted YAML-to-neutral-JSON parser boundary
- `tools/specbind/src/schema/` — authoritative versioned structured-artifact wire models and schema generator
- `tools/specbind/src/schema/runtime.rs` — parser, schema selection, validation, and wire-deserialization load boundary
- `tools/specbind/schemas/` — generated, checked-in Draft 2020-12 distribution schemas
- `tools/specbind/tests/` — Rust CLI integration tests
- `.github/workflows/rust.yml` — Windows and Linux Rust verification

The root `.kiro/` directory is not used to develop SpecBind and is intentionally ignored. Current consumer-facing `.kiro/` files must be maintained under `tools/cc-sdd/templates/shared/settings/` until the Rust templates replace them. Do not require `/kiro-*` workflows for this repository unless the user explicitly requests one.

## Development Workflow

- Follow the user's requested scope and make changes directly unless they explicitly request a SpecBind specification workflow.
- Keep changes narrow and preserve unrelated work in the worktree.
- When changing installed behavior, update the relevant source, templates, tests, and documentation together.
- Keep Claude Code and Codex templates aligned where they implement the same contract, while preserving platform-specific invocation syntax and capabilities.

## Rust Toolchain and Dependencies

- Use the toolchain and `rustfmt`/`clippy` components pinned by `tools/specbind/rust-toolchain.toml`.
- Windows MSVC validation requires Visual Studio Build Tools with the C++ workload and a Windows SDK.
- Commit `tools/specbind/Cargo.lock` and keep dependency versions and features centralized in `tools/specbind/Cargo.toml`.
- Prefer focused external crates behind SpecBind-owned module boundaries; third-party types do not define public artifact or CLI contracts.
- Change structured artifact contracts through the versioned wire models and conformance fixtures, then regenerate schemas with `cargo run --example generate_schemas`; do not hand-edit generated schemas.

## Validation

Run Rust verification from `tools/specbind/`:

```sh
cargo fmt --all -- --check
cargo run --example generate_schemas -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
```

Run inherited TypeScript verification from `tools/cc-sdd/`:

```sh
npm test
npm run build
```

- Add or update focused tests for behavior changes.
- Before reporting completion, inspect the final diff and confirm generated or installed templates still match their intended consumer environment.

## Language

- Respond to the user in Japanese unless they request another language.
- Preserve the language of an existing document unless the task requires translation.
- English and Japanese are the only officially supported product languages during the current stabilization phase.
