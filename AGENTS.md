# SpecBind Repository Guidelines

## Repository Purpose

- This repository develops SpecBind itself. It is not a consumer project using SpecBind to deliver an application.
- SpecBind was bootstrapped from `gotalab/cc-sdd` and has been detached from the GitHub fork network.
- Treat inherited cc-sdd code and documentation as migration inputs, not as the final SpecBind interface.
- Prefer SpecBind-specific commands, workflows, terminology, and compatibility decisions over drop-in cc-sdd compatibility unless compatibility is explicitly required.

## Source Layout

The repository source layout, design documents, and the complete decision record are indexed in [docs/repository-map.md](docs/repository-map.md). Read it before navigating unfamiliar parts of the tree or citing a decision.

The root `.kiro/` directory is not used to develop SpecBind and is intentionally ignored. Current consumer-facing `.kiro/` files must be maintained under `tools/cc-sdd/templates/shared/settings/` until the Rust templates replace them. Do not require `/kiro-*` or `$kiro-*` workflows for this repository unless the user explicitly requests one.

## Development Workflow

- Follow the user's requested scope and make changes directly unless they explicitly request a SpecBind specification workflow.
- This repository currently uses a direct-to-`main` personal-development workflow. Unless the user explicitly asks to stop before committing or pushing, commit each completed unit of work to `main` and push it to `origin/main` before reporting completion.
- Keep changes narrow and preserve unrelated work in the worktree.
- When changing installed behavior, update the relevant source, templates, tests, and documentation together.
- Keep Claude Code and Codex templates aligned where they implement the same contract, while preserving platform-specific invocation syntax and capabilities.
- For adding or extending coding-agent support, Codex agents must use `.agents/skills/specbind-new-agent/SKILL.md`.

## Rust Toolchain and Dependencies

- `tools/specbind/rust-toolchain.toml` pins the supported development toolchain and required `rustfmt` and `clippy` components. Do not silently change the pinned toolchain or `rust-version`.
- Windows MSVC validation requires Visual Studio Build Tools with the C++ workload and a Windows SDK. Do not switch the canonical Windows target to GNU merely to avoid this prerequisite.
- `tools/specbind/Cargo.lock` is committed because SpecBind is a distributed application.
- Keep workspace dependency versions and feature selection centralized in `tools/specbind/Cargo.toml`. Disable broad default features where the accepted product contract does not need them, especially networking and terminal styling.
- Prefer focused external crates behind SpecBind-owned module boundaries. Do not expose third-party result or model types as public artifact or CLI contracts.
- Use stable Rustfmt options only. Configure lint levels in the workspace manifest and add narrowly scoped `allow` entries only with a concrete reason.
- Treat versioned wire models under `tools/specbind/src/schema/` as the structural source of truth. A structural change must update the accepted design, wire model, generated schema, and conformance fixtures together.
- Regenerate schemas with `cargo run --example generate_schemas`; never hand-edit generated files under `tools/specbind/schemas/`.

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
