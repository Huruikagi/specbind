# 0006: Reimplement the SpecBind CLI in Rust

Status: Accepted

Decisions 0077 and 0081 narrow the v1 port: public manifests, backups, overwrite policy, compatibility aliases, and untested platform targets are not parity requirements. The current TypeScript implementation is migration evidence rather than the final Rust interface.

## Context

SpecBind currently ships a Node.js and TypeScript CLI that installs and updates agent skills, settings, rules, and templates. The target product also needs deterministic specification checks and guarded lifecycle operations.

Keeping the installer in Node while adding a separate Rust checker would split packaging, versions, diagnostics, and shared parsing rules across executables. SpecBind is now independent from cc-sdd, so it can choose one implementation and distribution model for its entire CLI.

## Decision

- Reimplement the SpecBind CLI itself in Rust.
- Continue to expose one product command named `specbind` rather than a separate `spec-lint` product.
- Include installation and idempotent agent-asset refresh behavior, deterministic checks, and accepted lifecycle operations in the same CLI codebase and release version. The `update` command name remains reserved for a possible future binary updater.
- Preserve templates as data owned and distributed by SpecBind; Rust replaces the CLI implementation, not the agent-facing Markdown artifact model.
- Do not require Node.js at runtime after the Rust CLI becomes the supported implementation.
- Treat the current TypeScript implementation and tests as the behavioral reference during migration, not as a permanent compatibility layer.
- Define migration compatibility relative to the same product contract and input. Accepted SpecBind feature changes may intentionally add, remove, rename, or revise generated artifacts.
- Keep semantic authoring and review in agent skills. Rust does not replace AI judgment.

## Migration principle

The migration is incremental and contract-driven:

1. Classify inherited TypeScript behavior as retained evidence, intentional removal, or explicit cc-sdd migration input.
2. Build the read-only Rust core with embedded schemas, assets, and installation planning.
3. Implement the v1 installer and plan-first `specbind migrate cc-sdd` flow.
4. Add deterministic checks, read models, and guarded lifecycle operations.
5. Move the inherited TypeScript tree temporarily to `tools/cc-sdd/`, make Rust canonical at `tools/specbind/`, and verify the cutover fixtures.
6. Publish checksummed Windows x64 and WSL2-tested Linux x64 binaries and installers.
7. Remove the temporary TypeScript implementation after install, migration, lifecycle, and distribution verification succeeds.

The detailed increments and remaining implementation choices live in [Rust CLI migration](../rust-cli-migration.md).

## Consequences

- The installer and machine-checking features share one parser, error model, release, and distribution channel.
- Binary builds and release automation for the explicitly supported v1 targets become core product infrastructure.
- Existing TypeScript tests need black-box equivalents or fixtures that can exercise both implementations during migration.
- Compatibility fixtures need an explicit product-contract version or change rationale so intended To-Be artifact evolution is not mistaken for a porting regression.
- The repository removes npm as a runtime distribution requirement at cutover. An npm launcher is only a possible post-v1 channel.
- Rust implementation details must not leak into generated skills; skills depend on the `specbind` command contract.
