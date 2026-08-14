# Rust CLI migration

This document is the working migration plan for [Decision 0006](./decisions/0006-rust-cli.md). It describes how the current TypeScript installer evolves into the Rust implementation of the complete SpecBind CLI.

Status: Draft

## Current implementation baseline

The current CLI under `tools/specbind/` owns more than process startup. Its observable responsibilities include:

- command-line parsing and help/version output
- persisted `.specbind.json` configuration and precedence
- agent, language, operating-system, profile, and manifest selection
- manifest loading and validation
- template-variable resolution and Markdown rendering
- installation planning and dry-run output
- category and overwrite policies
- interactive conflict decisions
- backups and path-safe file operations
- installation summaries and completion guidance

The templates under `tools/specbind/templates/` are product content and remain inputs to the Rust CLI unless a later artifact-layout decision changes them.

The specification root remains configurable, but [Decision 0007](./decisions/0007-spec-root.md) changes the target names to `--spec-dir`, `specDir`, and `{{SPEC_DIR}}`, with `.specbind` as the default for new installations.

## Target command model

The product remains one executable:

```text
specbind
├── install or update agent assets
├── check deterministic spec invariants
└── perform accepted guarded lifecycle operations
```

The current option-only installer interface may remain as a compatibility alias, but new capabilities should use explicit subcommands. A possible working shape is:

```sh
specbind install [options]
specbind check traceability <spec-path> [--json]
specbind milestone <operation> [options]
specbind release <operation> [options]
```

These command names are Draft. The accepted constraint is one Rust `specbind` CLI, not this exact hierarchy.

## Suggested Rust boundaries

The exact crates are not yet fixed, but the code should separate:

- `cli`: arguments, output mode, prompting, and exit mapping
- `config`: configuration loading, validation, and precedence
- `manifest`: schema, loading, and installation planning
- `template`: context construction and deterministic rendering
- `fs`: path safety, diffing, backup, and atomic or guarded writes
- `check`: read-only specification parsers and diagnostics
- `lifecycle`: explicit milestone and release state transitions

Core modules should return structured results and diagnostics. Human-readable and JSON rendering belongs at the CLI boundary so skills and CI consume the same semantics.

## Compatibility inventory

Before implementation, freeze expected behavior for:

| Contract | Current source of evidence | Migration gate |
| --- | --- | --- |
| Installed file trees | Real-manifest tests for Claude Code and Codex | Byte-equivalent output where no intentional template change is declared. |
| Argument behavior | CLI and argument tests | Accepted flags, defaults, aliases, errors, help, and exit behavior are covered. |
| Config precedence | Config merge and store tests | CLI, persisted config, environment, and defaults resolve identically except for the accepted `specDir` rename and `.specbind` default. |
| Manifest semantics | Loader, planner, and processor tests | The same valid manifests plan the same artifacts; invalid input has stable diagnostics. |
| File safety | Executor, file-operation, and path-safety behavior | No path escape, accidental broad overwrite, or backup regression. |
| Interactive policy | Prompt and overwrite behavior | TTY and non-TTY behavior is explicitly tested. |
| Rendering | Template tests | Supported variables, conditional content, and line endings remain defined. |

Golden generated-tree fixtures are preferable to duplicating internal TypeScript unit structure in Rust. They preserve user-visible behavior while allowing an idiomatic rewrite.

## Migration increments

### 1. Contract capture

- Record the current CLI surface and config schema.
- Add golden fixtures for representative installation matrices.
- Classify observed behavior as preserve, intentionally change, or remove.
- Define normalized line-ending and path expectations across platforms.
- Capture `.kiro`, `.specbind`, custom-root, and conflicting-root migration cases.

### 2. Read-only Rust core

- Create the Rust workspace and CLI entry point.
- Load configuration and manifests.
- Resolve templates and produce an installation plan.
- Support dry-run and structured diagnostics without filesystem writes.

### 3. Installer parity

- Add guarded writes, conflict policies, backups, and summaries.
- Match Claude Code and Codex generated trees.
- Verify English and Japanese output/template selection.
- Exercise Windows, macOS, and Linux path behavior in CI.

### 4. Native SpecBind operations

- Implement traceability checking in Rust.
- Add lifecycle checks and accepted mutations incrementally.
- Update generated skills to call stable CLI contracts rather than shell-specific inspection logic.

### 5. Distribution cutover

- Produce checksummed binaries for supported targets.
- Select installation channels and upgrade behavior.
- Make Rust the authoritative `specbind` command.
- Retire TypeScript sources, Node build scripts, and obsolete tests only after parity and upgrade verification.

## Distribution questions

Rust removes the Node runtime requirement but does not by itself choose how users install SpecBind. Candidate channels include:

- GitHub Release binaries and install scripts
- Homebrew and WinGet or Scoop
- Cargo installation for developer-oriented use
- an npm compatibility package that selects a platform binary

The primary channel should make clean install, self-update or package-manager update, version pinning, and checksum verification straightforward for both humans and agents.

## Non-goals of the first migration

- Rewriting or renaming all inherited skills at the same time as the CLI port
- Changing manifest and template formats solely to make the Rust implementation easier
- Encoding semantic requirement or design review into deterministic Rust checks
- Preserving TypeScript module boundaries or implementation details
- Shipping a second long-lived `spec-lint` executable

## Decisions still needed before coding

- Repository layout: replace `tools/specbind` in place or introduce a temporary sibling during parity work.
- Initial compatibility level for the current option-only command.
- Template packaging: compile into the binary, install beside it, or use a hybrid override model.
- Supported release targets and minimum operating-system versions.
- Primary installation and upgrade channels.
- Cutover gate and rollback plan for the first Rust-backed release.
- Duration and removal criteria for `--kiro-dir`, `kiroDir`, and `{{KIRO_DIR}}` compatibility aliases.
