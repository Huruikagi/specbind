# 0006: Reimplement the SpecBind CLI in Rust

Status: Accepted

## Context

SpecBind currently ships a Node.js and TypeScript CLI that installs and updates agent skills, settings, rules, and templates. The target product also needs deterministic specification checks and guarded lifecycle operations.

Keeping the installer in Node while adding a separate Rust checker would split packaging, versions, diagnostics, and shared parsing rules across executables. SpecBind is now independent from cc-sdd, so it can choose one implementation and distribution model for its entire CLI.

## Decision

- Reimplement the SpecBind CLI itself in Rust.
- Continue to expose one product command named `specbind` rather than a separate `spec-lint` product.
- Include installation and update behavior, deterministic checks, and accepted lifecycle operations in the same CLI codebase and release version.
- Preserve templates as data owned and distributed by SpecBind; Rust replaces the CLI implementation, not the agent-facing Markdown artifact model.
- Do not require Node.js at runtime after the Rust CLI becomes the supported implementation.
- Treat the current TypeScript implementation and tests as the behavioral reference during migration, not as a permanent compatibility layer.
- Keep semantic authoring and review in agent skills. Rust does not replace AI judgment.

## Migration principle

The migration should be incremental and parity-driven:

1. Establish black-box fixtures for the current installer and updater.
2. Define stable command, config, manifest, filesystem-safety, and output contracts.
3. Implement equivalent Rust planning and dry-run behavior without writing files.
4. Implement file application, conflict handling, backup, and interactive behavior.
5. Add the new deterministic checks and lifecycle operations to the Rust command model.
6. Compare generated trees and observable behavior across supported agents, languages, operating systems, and overwrite policies.
7. Switch release packaging to the Rust binary only after the agreed compatibility gates pass.
8. Remove the TypeScript implementation and Node runtime dependency in a later cleanup change.

Exact package-manager installers and the duration of any transition wrapper remain separate design decisions.

## Consequences

- The installer and machine-checking features share one parser, error model, release, and distribution channel.
- Cross-platform binary builds and release automation become core product infrastructure.
- Existing TypeScript tests need black-box equivalents or fixtures that can exercise both implementations during migration.
- The repository can eventually remove npm as a runtime distribution requirement, though npm may remain one installation channel if it wraps platform binaries.
- Rust implementation details must not leak into generated skills; skills depend on the `specbind` command contract.

## Open questions

- Final crate and workspace layout.
- Binary embedding versus release-adjacent packaging of templates.
- Supported installation channels and target triples.
- Whether an npm compatibility package downloads or bundles platform binaries.
- Compatibility guarantees for current installer arguments and `.specbind.json`, except for the accepted spec-root rename in [Decision 0007](./0007-spec-root.md).
- Whether the TypeScript and Rust CLIs coexist for one release or switch at a major-version boundary.
