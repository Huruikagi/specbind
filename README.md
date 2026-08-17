# SpecBind

SpecBind is an experimental command system for binding durable specifications to agent-assisted software delivery.

## Project status

This repository was bootstrapped from [gotalab/cc-sdd](https://github.com/gotalab/cc-sdd) and has been detached from the GitHub fork network. The canonical SpecBind CLI is now being implemented in Rust under [`tools/specbind/`](./tools/specbind/). The inherited TypeScript implementation remains under [`tools/cc-sdd/`](./tools/cc-sdd/) as a temporary migration and comparison oracle.

The Rust CLI currently provides structured artifact loading and semantic validation, configured project-root resolution, recursive OKF artifact discovery with profile diagnostics, public `artifact list/read` text and raw-content commands, `template list/read spec` scaffold discovery and raw reads over project overrides and embedded official defaults, project-independent `protocol list/read` access to the eight embedded product protocols, `steering list/read` discovery of project-level guidance that fails closed on an incomplete collection, `install` planning and guarded product-asset application including per-agent rendering of the embedded `specbind-status` and `specbind-discovery` skills, standalone `check traceability/contracts` gates with CI-usable exit status, public `tasks list/show` projections with derived execution state and effective dependencies, guarded `tasks complete/block/reopen` progress records, project-wide `spec list` enumeration that reports an unreadable Spec instead of failing, composed `spec status` lifecycle reporting, `milestone scope` reads of the current scope in the exact shape a replacement candidate takes, with the authored Roadmap body included only on request, active-scope `milestone status` stage, health, review, progress, dependency, and actionable-work reporting, guarded `milestone create`, `milestone update-scope`, and `milestone rebaseline` transitions, guarded `milestone bind-release` mutation with portable archive-target collision checks, focused `milestone review status` reporting and guarded `milestone review accept` candidate acceptance, guarded `spec requirements/design/tasks approve` and `invalidate` gate transitions, stateless `release preflight` readiness, guarded `release finalize` lifecycle closure with strict per-Spec log summaries and retry recovery, guarded Spec completion preflight/accept/invalidate commands, and guarded Direct preflight/complete commands. It also provides Markdown AST validation and canonical ID extraction for Requirements, Design body-marker equality, typed canonical Contract parsing and project-wide graph validation, active Roadmap parsing and normalized review-scope fingerprinting, strict Cross-spec review candidate resolution, guarded accepted-state persistence, accepted-review freshness evaluation and later-boundary enforcement, cross-artifact Requirement existence and active Design/Task coverage, canonical gate-input resolution and fingerprints, cascading requirements/design/tasks freshness evaluation, Git- and task-aware completion freshness across same-revision multi-Spec metadata transitions, and target-only Git safety for release operations. Remaining lifecycle transitions and installation behavior are being implemented incrementally from the accepted design decisions. `milestone status` derives `release_ready` only when the same stateless preflight checks pass. Until the transition is complete, do not treat inherited cc-sdd behavior as the final SpecBind interface.

For concise snapshots of the current interface, see the [generated skill index](./docs/current-skill-index.md) and [generated artifact index](./docs/current-artifact-index.md). The proposed replacement is tracked separately in the [target skill catalog](./docs/design/target-skill-catalog.md), [target artifact catalog](./docs/design/target-artifact-catalog.md), and [target workflows](./docs/design/target-workflows.md).

The [repository map](./docs/repository-map.md) indexes this repository's own source layout, design documents, and complete decision record.

## Direction

- Define a command system with its own naming and workflow conventions.
- Keep spec-driven development practical for agentic software delivery.
- Rework inherited components deliberately instead of maintaining drop-in compatibility with cc-sdd.
- Stabilize migration and compatibility behavior before creating maintained documentation.

## Repository layout

- `tools/specbind/` — canonical Rust workspace and future distributed `specbind` executable
- `tools/cc-sdd/` — inherited TypeScript migration oracle
- `docs/design/` — target workflows, lifecycle models, and accepted design decisions

## Development

The workspace pins Rust 1.97.1, Rustfmt, and Clippy through [`rust-toolchain.toml`](./tools/specbind/rust-toolchain.toml). Install [Rustup](https://rustup.rs/) before running Cargo commands. Windows development with the default MSVC target also requires Visual Studio Build Tools with the **Desktop development with C++** workload and a Windows SDK.

Run the current CLI from the Rust workspace:

```sh
cd tools/specbind
cargo run -- --help
```

Run the complete Rust verification set before committing Rust changes:

```sh
cargo fmt --all -- --check
cargo run --example generate_schemas -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
```

`Cargo.lock` is committed because SpecBind distributes an application binary. The same checks run on Windows and Linux in [the Rust workflow](./.github/workflows/rust.yml).

The versioned Rust DTOs under [`src/schema/`](./tools/specbind/src/schema/) are the structural source of truth for structured artifacts. After changing them, regenerate the checked-in Draft 2020-12 schemas and review the resulting diff:

```sh
cargo run --example generate_schemas
```

The inherited TypeScript oracle retains its own verification commands:

```sh
cd tools/cc-sdd
npm test
npm run build
```

## Language support

During this stabilization phase, SpecBind officially supports English (`en`) and Japanese (`ja`) only. Additional languages may be reconsidered after the commands, workflows, and documentation have stabilized.

## Upstream and attribution

SpecBind began from the source code of [cc-sdd](https://github.com/gotalab/cc-sdd) by gotalab. We are grateful to the original project and its contributors for the foundation.

SpecBind is an independent project and is not affiliated with or endorsed by gotalab. The original copyright and MIT license notice are retained in [LICENSE](./LICENSE).

## License

MIT License. See [LICENSE](./LICENSE).
