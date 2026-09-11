# Repository map

This page is the single index of how this repository is organized to develop SpecBind itself. [CLAUDE.md](https://github.com/Huruikagi/specbind/blob/main/CLAUDE.md) and [AGENTS.md](https://github.com/Huruikagi/specbind/blob/main/AGENTS.md) link here instead of duplicating it, so one edit keeps every agent's map current.

For the files the CLI installs into consumer projects, see the [artifact index](./en/reference/current-artifact-index.md) and the [skill index](./en/reference/current-skill-index.md).

The canonical Rust dependency direction and module-boundary rules are defined
in [Implementation architecture](./architecture.md).

## Source layout

- `tools/cc-sdd/src/` — inherited TypeScript CLI retained as a migration and comparison oracle
- `tools/cc-sdd/test/` — inherited TypeScript reference tests, excluded from routine product verification
- `tools/cc-sdd/schemas/` — inherited hand-authored schema snapshots retained as migration inputs
- `tools/cc-sdd/templates/` — inherited template snapshots retained as migration and comparison inputs
- `tools/specbind/` — canonical Rust CLI workspace
- `tools/specbind/src/` — Rust CLI implementation
- `tools/specbind/src/catalog.rs` and `catalog/` — facade and implementations for adapters, protocols, rules, skills, templates, and steering
- `tools/specbind/src/documents.rs` and `documents/` — facade and implementations for Requirements, Design, Contract, Roadmap, and traceability semantics
- `tools/specbind/src/documents/instruction.rs` — scoped managed-Markdown instruction parsing, validation, semantic masking, and read projection
- `tools/specbind/src/foundation.rs` and `foundation/` — facade and implementations for configuration, fingerprints, and restricted YAML
- `tools/specbind/src/installation.rs` and `installation/` — facade and implementations for installation, agent roles, and project instructions
- `tools/specbind/src/lifecycle.rs` and `lifecycle/` — facade and implementations for guarded lifecycle operations
- `tools/specbind/src/read_model.rs` and `read_model/` — facade and implementations for non-authoritative current-state projections
- `tools/specbind/src/infrastructure.rs` and `infrastructure/` — crate-private Git and guarded-filesystem adapters
- `tools/specbind/src/foundation/yaml.rs` — restricted YAML-to-neutral-JSON parser boundary
- `tools/specbind/src/schema/` — authoritative versioned structured-artifact wire models and schema generator
- `tools/specbind/src/schema/runtime.rs` — parser, schema selection, validation, and wire-deserialization load boundary
- `tools/specbind/src/domain/` — artifact-local semantic validation and validated domain wrappers
- `tools/specbind/src/cli.rs` — transport facade and command-family re-exports
- `tools/specbind/src/cli/output.rs` — shared CLI output contract, stream routing, escaping, and text-layout helpers
- `tools/specbind/src/cli/` — stable command-family facades for migration, reads, tasks, external input, and lifecycle operations
- `tools/specbind/src/cli/read/` — artifact/check, installation, catalog, and project-scope read command execution/rendering
- `tools/specbind/src/cli/read/embedded_catalog_commands.rs` — project-independent Protocol and schema catalog commands backed entirely by embedded binary assets
- `tools/specbind/src/cli/read/template_commands.rs` — project-bound Spec, Milestone, and Steering template discovery, reads, and target resolution
- `tools/specbind/src/cli/read/adapter_commands.rs`, `rule_commands.rs`, and `steering_commands.rs` — project-bound catalog commands separated by the artifact family they expose
- `tools/specbind/src/cli/lifecycle/` — completion, gate, milestone mutation, release, contract-review, and status command execution/rendering
- `tools/specbind/src/cli/lifecycle/status_commands.rs` and `status_commands/` — stable status command facade with separate Spec, Milestone, and shared JSON renderers
- `tools/specbind/src/artifacts.rs` — stable public facade and result models for spec-local artifact reads
- `tools/specbind/src/artifacts/discovery.rs` — filesystem discovery, logical identity, metadata-profile validation, and partial inventory
- `tools/specbind/src/artifacts/resolution.rs` — typed Spec and Task loads, gate-input resolution, fingerprints, and traceability projections
- `tools/specbind/src/installation/install.rs` — installation planning, guarded asset application, and repository guards
- `tools/specbind/src/installation/removal.rs` and `removal/` — stable exact-removal facade with separate planning, config reads, Git/filesystem guards, and config-last apply
- `tools/specbind/src/installation/agent_role.rs` — stable subagent roles, cost-aware defaults, project capability overrides, and Codex and Claude Code rendering
- `tools/specbind/src/migration.rs` — public historical cc-sdd migration models and orchestration boundary
- `tools/specbind/src/migration/project.rs` — cumulative version-range project migration catalog and guarded raw description probes
- `tools/specbind/src/documents/description.rs` — optional responsibility metadata validation for Requirements, Design, and Steering
- `tools/specbind/src/migration/inventory.rs` — read-only historical cc-sdd inventory and conversion planning
- `tools/specbind/src/migration/apply.rs` — Git-guarded deterministic apply and final source retirement
- `tools/specbind/src/migration/resolution.rs` — guarded agent-resolution acceptance and source/target freshness checks
- `tools/specbind/src/args.rs` — command-line argument definitions, walkable by skill conformance tests
- `tools/specbind/src/catalog/skill.rs` — embedded product-managed skills and per-agent rendering
- `tools/specbind/src/catalog/protocol.rs` — embedded product-protocol registry and raw reads
- `tools/specbind/src/lifecycle/task_progress.rs` — guarded task execution progress records
- `tools/specbind/src/catalog/rule.rs` — embedded default shared-rule installation assets
- `tools/specbind/src/catalog/template.rs` — OKF artifact template discovery, profile validation, and raw reads over project-owned overrides and embedded defaults
- `tools/specbind/assets/templates/` — official embedded OKF artifact templates for each supported language
- `tools/specbind/assets/bundle-index/` — localized product-managed block for the shared OKF bundle-root index
- `tools/specbind/assets/protocols/` — immutable English product protocols exposed by `protocol read`
- `tools/specbind/assets/skills/` — one agent-neutral source per product-managed skill
- `tools/specbind/assets/rules/` — official default project-owned shared rules written by `install`
- `tools/specbind/src/documents/requirements.rs` — Markdown AST validation and canonical Requirement ID extraction
- `tools/specbind/src/documents/design.rs` — Design emphasis-marker extraction and Front Matter traceability equality
- `tools/specbind/src/documents/traceability.rs` — cross-artifact Requirement existence plus active Design and Task coverage, exposed by `check traceability`
- `tools/specbind/src/schema/contract.rs` and `domain/contract.rs` — versioned Contract wire model and artifact-local semantic validation
- `tools/specbind/src/read_model/contract_graph.rs` — project-wide Contract reference, ownership-overlap, and dependency-cycle read model
- `tools/specbind/src/read_model/milestone_status.rs` and `milestone_status/` — stable Milestone status facade with separate action derivation, diagnostics, and release-readiness projections
- `tools/specbind/src/documents/roadmap.rs` — active Roadmap parsing, DAG validation, and normalized cross-spec scope projection
- `tools/specbind/src/lifecycle/cross_spec_review.rs` — strict review candidate and authoritative Contract-first input revision resolution
- `tools/specbind/src/foundation/fingerprint.rs` — Markdown and normalized typed task-plan fingerprint producers
- `tools/specbind/src/read_model/freshness.rs` — gate-local requirements, design, and tasks freshness evaluation
- `tools/specbind/src/lifecycle/approval.rs` and `approval/` — stable guarded gate-transition facade with separate evidence construction and persistence safety
- `tools/specbind/src/read_model/release_readiness.rs` — stateless whole-milestone release readiness and target-only Git safety validation
- `tools/specbind/src/lifecycle/release_log.rs` — strict release-summary JSON and localized canonical OKF `log.md` updates
- `tools/specbind/src/lifecycle/release_finalize.rs` — ordered, guarded, retry-safe whole-milestone finalization
- `tools/specbind/src/lifecycle/completion/` — Spec and Direct completion candidate validation, guarded transitions, and shared preflight checks
- `tools/specbind/src/infrastructure/repository.rs` — installed-Git process adapter shared by lifecycle and status read models
- `tools/specbind/src/infrastructure/guarded_fs.rs` — regular-file guards and atomic replacement for SpecBind-owned state
- `tools/specbind/src/lifecycle/milestone/` — guarded active-Roadmap creation, scope replacement, rebaseline, and release binding
- `tools/specbind/src/lifecycle/release.rs` — portable release labels and case-insensitive archive-target collision resolution
- `tools/specbind/schemas/` — generated, checked-in Draft 2020-12 distribution schemas
- `tools/specbind/tests/` — Rust CLI integration tests
- `tools/specbind/tests/cli/gates/` and `project_reads/` — command-area test modules for Gate lifecycle/status and project-bound reads
- `scripts/check_decisions.py` — Decision filename, heading, identifier, and Decision-index consistency check
- `.github/workflows/rust.yml` — ordinary Linux Rust verification
- `.github/workflows/release.yml` — native Windows, Linux, and macOS release verification and packaging
- `.github/workflows/decisions.yml` — focused Decision-index verification for affected pushes and pull requests

## User documentation

Public documentation lives under `docs/en/` (published at the site root) and
`docs/ja/` (published under `/ja/`). Both trees use identical relative paths
for `index.md`, `guide/`, and `reference/`, and each page is maintained as a
translation pair. The `nav` in `mkdocs.yml` is the authoritative page list and
order. The [documentation authoring policy](./documentation-authoring.md)
defines bilingual maintenance and verification.

The documentation site is configured by `mkdocs.yml`, built with the pinned
dependency in `requirements-docs.txt`, and deployed by
`.github/workflows/pages.yml`. Run `python -m mkdocs build --strict` from the
repository root to verify it locally.

## Design documents

| Document | Role |
| --- | --- |
| [Spec state machine](./design/spec-state-machine.md) | Per-spec states, events, invalidation rules, and transition diagram |
| [Milestone state machine](./design/milestone-state-machine.md) | Derived milestone stage, phase-relative dependency waves, and aggregate read model |
| [Cross-spec contracts](./design/cross-spec-contracts.md) | Detailed draft for persistent cross-spec seam manifests and contract-first review |
| [Skill forward tests](./skill-forward-tests.md) | Index for the behavioral procedure, measurement dashboard, run archive, findings worklist, and scenario contracts for embedded skills |

This repository's development Skills are:

- `.agents/skills/sb-dev-merge-dependabot/` for sequential Dependabot review,
  integration, main-CI confirmation, and affected dependency validation;
- `.agents/skills/sb-dev-forward-test/` for behavioral verification of
  embedded product Skills;
- `.agents/skills/sb-dev-sync-docs/` for bilingual public-documentation
  maintenance; and
- `.agents/skills/sb-dev-resolve-dogfooding/` for triage, product fixes,
  delivery, and Issue disposition of maintainer dogfooding findings.

They are never installed into a consumer project.

## Decision records

The complete Decision index, with each Decision's status and summary, is
[docs/design/decisions/index.md](./design/decisions/index.md). Accepted
decisions are authoritative; a superseded decision is retained for history.
