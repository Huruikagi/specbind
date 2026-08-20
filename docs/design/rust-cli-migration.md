# Rust CLI migration

This document is the working migration plan for [Decision 0006](./decisions/0006-rust-cli.md). It describes how the inherited TypeScript installer is retired in favor of the complete Rust `specbind` CLI.

Status: In progress — the native product surface is implemented; explicit
cc-sdd migration and distribution cutover remain.

## Current implementation baseline

The inherited TypeScript implementation under `tools/cc-sdd/` remains useful as
executable migration evidence for its option parsing, manifests, template
rendering, installation planning, overwrite prompts, backups, and installed
skill trees. The canonical Rust CLI now owns `.specbind.json`, installation,
rendering, and the SpecBind skill tree; inherited surfaces are not automatically
the Rust product contract.

The accepted v1 contract deliberately removes public manifests, profiles, operating-system selectors, overwrite and backup modes, `--yes`, and inherited compatibility aliases. [Decision 0077](./decisions/0077-v1-installation-distribution-and-migration.md) defines the smaller installation and migration surface. [Decision 0081](./decisions/0081-v1-release-git-path-and-cli-safety.md) makes Git, guarded target paths, and deterministic retry the safety model.

The target structured artifacts remain `spec.yaml` and `tasks.yaml` under Decisions 0013 and 0014. The authoritative v1 wire models accepted by Decision 0085 now live under `tools/specbind/src/schema/`; they generate the checked-in distribution schemas at `tools/specbind/schemas/`, which are embedded in the matching binary and checked against Rust-owned conformance fixtures. The runtime loader rejects prohibited YAML features, selects the explicit schema version, evaluates the matching artifact schema, and deserializes schema-valid neutral values into the matching wire model. Domain conversion validates artifact-local lifecycle and task-plan semantics. Type-based OKF discovery resolves current logical artifacts, the Requirements AST parser derives canonical Requirement IDs, the Design parser validates body-marker equality, the Contract AST parser produces the typed canonical five-section manifest, the project-wide Contract graph resolves dependencies and review warnings, the active Roadmap parser validates its DAG and produces the normalized review-scope fingerprint, and the Contract review operation derives authoritative Contract-first inputs, atomically persists accepted state behind Git and lifecycle guards, evaluates accepted-review freshness from the strict persisted profile and current authoritative inputs, and enforces that state at Tasks approval, implementation validation, and release preflight. The traceability resolver checks Design and Task references plus active-scope coverage, and the freshness read model compares requirements, design, contract, and task-plan inputs before evaluating completion from current Task execution state and the exact Git implementation-revision relationship. The CLI resolves the configured SpecBind root from the containing Git project, exposes the accepted deterministic artifact inventory and raw single-selector content reads, resolves artifact templates from project overrides and embedded official defaults through `template list/read spec`, exposes the embedded product protocols through `protocol list/read` without requiring a project, exposes the deterministic `check traceability` and `check contracts` gates, renders validated task hierarchy, progress, effective prerequisites, blockers, and actionable work through `tasks list/show`, composes those projections with declared state, consistency diagnostics, traceability, and gate freshness through `spec status`, creates the active milestone, replaces its confirmed scope, and rebaselines it through `milestone create`, `milestone update-scope`, and `milestone rebaseline`, derives active Roadmap scope and stateless release readiness through `milestone status` and `release preflight`, reports focused contract review state and accepts one strict guarded review candidate through `milestone review status` and `milestone review accept`, crosses and rewinds the Requirements, Design, and Tasks gates through `spec <gate> approve` and `spec <gate> invalidate`, and performs ordered Roadmap-last lifecycle closure through `release finalize`. Finalization validates strict log-entry JSON, inserts localized OKF log entries, clears active Spec state, removes milestone-local artifacts, archives review state when applicable, and recognizes safe interrupted or completed retries. The hand-authored copies under `tools/cc-sdd/schemas/` remain migration snapshots rather than current authority. Shared `settings/templates/` and `settings/rules/` are user-owned customization surfaces; generated agent assets are product-managed.

[Decision 0084](./decisions/0084-rust-dependency-strategy.md) accepts a dependency-positive implementation strategy: focused crates own general-purpose mechanics behind SpecBind module boundaries, while SpecBind retains its exact format, lifecycle, diagnostics, and safety contracts. Git remains an installed-executable adapter so repository decisions agree with the Git implementation and configuration that v1 already requires.

## Repository and cutover layout

The preparatory move establishes this repository layout:

```text
tools/cc-sdd/    # inherited TypeScript comparison and migration oracle
tools/specbind/  # canonical Rust workspace, created by the next scaffold increment
```

The TypeScript move is complete before Rust scaffolding so the accepted product path can be created without an in-place language rewrite. Delete the temporary implementation only after the cutover gates pass.

## Target command model

The product is one executable:

```text
specbind
├── artifact <list|read>
├── install [options]
├── protocol <list|read>
├── schema <list|read>
├── check <traceability|contracts>
├── template <list|read>
├── tasks <list|show|complete|block|reopen>
├── adapter <list|read>
├── steering <list|read>
├── spec <operation>
├── milestone <operation>
├── release <preflight|finalize>
└── migrate cc-sdd [--apply]  # accepted, not yet implemented
```

There is no option-only compatibility alias. `specbind install` performs initial installation and idempotent agent-asset refresh; the command name `update` remains available for a future binary-update workflow. Lifecycle commands are non-interactive. The installer may prompt only in a TTY; non-TTY execution supplies its choices explicitly.

Accepted named commands include the Decision 0086 Spec and Direct completion handshakes, `specbind milestone bind-release`, `specbind release preflight`, and `specbind release finalize`. Finalization has no `--force` bypass.

## Suggested Rust boundaries

The dependency direction is accepted by Decision 0084, and the code should separate:

- `cli`: arguments, TTY prompting for installation, text rendering, stream routing, and exit mapping
- `config`: `.specbind.json` loading and validation
- `repository`: narrow installed-Git executable adapter with caller-owned product diagnostics
- `guarded_fs`: regular-file validation and atomic replacement for SpecBind-owned state
- `assets`: embedded agent assets and installation planning
- `template`: embedded defaults, project-owned overrides, and deterministic rendering
- `fs`: project-path validation and semantic reads
- `check`: read-only artifact parsers and diagnostics
- `lifecycle`: explicit milestone, task, and release transitions
- `schema`: versioned wire models, deterministic schema generation, embedded generated-schema lookup, structural validation, domain conversion, and shared conformance fixtures
- `migration`: read-only cc-sdd planning and explicit `--apply`

Core modules return structured internal results. V1 exposes concise English text with stable codes, not a public JSON result envelope. Project-specific release instructions remain agent-executed natural language and never become unrestricted CLI hooks.

## Compatibility inventory

Compatibility work distinguishes porting evidence from the accepted interface:

- **Porting evidence:** retained behavior that still matches an accepted SpecBind decision.
- **Intentional removal:** inherited behavior explicitly excluded by Decisions 0075 through 0081.
- **Migration input:** cc-sdd files that `specbind migrate cc-sdd` may recognize without becoming supported aliases.

Capture focused fixtures for:

| Contract | Migration gate |
| --- | --- |
| Installed Claude Code and Codex trees | Generated product-managed assets match the accepted catalog; user-owned settings are never overwritten. |
| `.specbind.json` | Only the v1 fields and precedence accepted by Decision 0077 are supported. |
| Artifact templates and schemas | Embedded defaults and runtime schemas match the CLI version; custom settings remain external. |
| Git and path safety | Root, submodule, ignored-path, portability, and clean-target cases match Decision 0081. |
| TTY behavior | Installer prompting and non-interactive lifecycle behavior are covered. |
| cc-sdd migration | Default plan is read-only; `--apply` handles only unambiguous known artifacts and stops on ambiguity. Decision 0125 routes semantic findings to the version-compatible GitHub Pages guide and requires guided work to rejoin deterministic validation. |
| Output | Stable English result codes, stdout/stderr routing, sanitization, and zero/nonzero exits are covered. |

Golden generated-tree fixtures should identify the accepted decision that explains each intentional difference. Regenerating snapshots alone is not sufficient evidence.

## Migration increments

### 1. Contract capture

- Complete for the accepted SpecBind interface: Decisions 0075 through 0125
  classify the retained, replaced, and removed product behavior.
- Current install and lifecycle fixtures cover clean and locally modified
  product assets, existing project-owned settings, custom roots, project
  instruction blocks, Git guards, and portable path failures.
- Initial migration fixtures now cover the historical `.cc-sdd.json`, legacy
  `.kiro` and `spec.json`, Codex and Claude Code skill detection, mixed language,
  multiple Specs, Design traceability, customized rules, no-write planning,
  and guarded `--apply`. Automatic apply currently covers only finding-free
  configuration and exact known Codex or Claude Code legacy skills. Spec and
  task conversion, exact quick-start recognition, and platform-specific legacy
  layouts remain outstanding.
- The accepted agent-assisted path is published in Japanese and English under
  `docs/guide/`; stable findings and language-aware guide selection are
  implemented. Retry-safe recognition of guided target work remains
  outstanding.

### 2. Read-only Rust core

Implemented. The Rust workspace owns the CLI, configuration, embedded schemas,
templates, protocols, rules, adapters, and all seventeen product skills. It
provides artifact discovery, structural and semantic validation, deterministic
read models, dry-run installation planning, concise diagnostics, and stable exit
behavior.

### 3. Installer and migration

`specbind install` is implemented with additive agent selection, idempotent
product-asset refresh, project-owned template, rule, and adapter preservation,
and optional marked project-instruction blocks. Initial installation still
requires explicit agent and language values; TTY prompting is outstanding.

`specbind migrate cc-sdd` now implements the read-only, fail-closed inventory
and plan. It reads the original cc-sdd configuration name `.cc-sdd.json`, not
the `.specbind.json` name introduced by the later repository-wide rename. The
current `tools/cc-sdd` tree therefore remains a migration oracle for inherited
artifact shapes, but Git history is authoritative for renamed brand-specific
inputs. `--apply` now installs the unambiguous configuration-and-agent subset,
retires exact known legacy skills only after successful installation, and
recognizes its own converged target on retry. Legacy Spec conversion and
guided-work convergence remain outstanding.
[Decision 0125](./decisions/0125-agent-assisted-cc-sdd-migration.md) defines the
supported Pages handoff and deterministic rejoin boundary.

### 4. Native SpecBind operations

Implemented. The Rust CLI owns structured loading, traceability, Contract graph
checks, task and lifecycle read models, gate freshness, milestone creation and
scope changes, task progress, Spec and Direct completion handshakes, Contract
review acceptance, gate approval and invalidation, release binding, preflight,
and retry-safe whole-milestone finalization. The complete seventeen-skill v1 set
uses those stable commands and the embedded protocol and schema read surfaces.

Mechanical conformance runs in CI. Behavioral skill coverage remains a manual
stabilization activity recorded in [Skill forward tests](../skill-forward-tests.md).

### 5. Distribution cutover

- Complete: the Rust workspace is canonical at `tools/specbind/`, while
  `tools/cc-sdd/` remains the temporary migration oracle.
- Produce Windows x64 and Linux x64 GitHub Release binaries plus `SHA256SUMS`.
- Publish PowerShell and shell installers that select the latest stable version by default, accept explicit prerelease versions, verify checksums, and never edit PATH.
- Verify `%LOCALAPPDATA%\SpecBind\bin` and `$HOME/.local/bin` defaults plus `--install-dir`.
- Start the public Rust release line under the pre-1.0 policy in
  [Decision 0124](./decisions/0124-pre-1.0-binary-release-line.md).
- Retire the temporary TypeScript tree only after install, migration, artifact, lifecycle, and distribution fixtures pass.

## V1 distribution boundary

V1 officially supports Windows x64 and Linux x64 as tested through WSL2. Native macOS ARM64, macOS Intel, and Linux ARM64 are deferred until corresponding test environments exist. GitHub Releases and the two installer scripts are the only primary distribution channel. Homebrew, WinGet, Scoop, Cargo installation, npm launchers, self-update, code signing, and notarization are post-v1 options.

The installers verify the selected archive against `SHA256SUMS`, install to the platform default or `--install-dir`, and print an exact PATH follow-up when needed. They do not modify shell profiles. `specbind --version` reports the installed version.

## Non-goals of the first migration

- Drop-in cc-sdd command or configuration compatibility
- Public custom installation manifests or profiles
- Backup, overwrite-policy, or destructive force machinery
- macOS or Linux ARM64 release claims without test coverage
- Encoding semantic requirement, design, review, or validation judgments in Rust
- A second long-lived `spec-lint` executable

## Remaining implementation details

- Minimum supported Windows and WSL2 Linux versions
- Minimum supported Rust version
- Exact archive names and installer-script URLs
- Exact internal Rust crate layout and dependency versions/features after the YAML and MSRV spikes
- Cutover and rollback checklist for the first Rust-backed release
