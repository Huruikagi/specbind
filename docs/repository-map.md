# Repository map

This page is the single index of how this repository is organized to develop SpecBind itself. [CLAUDE.md](https://github.com/Huruikagi/specbind/blob/main/CLAUDE.md) and [AGENTS.md](https://github.com/Huruikagi/specbind/blob/main/AGENTS.md) link here instead of duplicating it, so one edit keeps every agent's map current.

For the files the CLI installs into consumer projects, see the [current generated artifact index](./current-artifact-index.md) and the [current generated skill index](./current-skill-index.md).

The canonical Rust dependency direction and module-boundary rules are defined
in [Implementation architecture](./architecture.md).

## Source layout

- `tools/cc-sdd/src/` — inherited TypeScript CLI retained as a migration and comparison oracle
- `tools/cc-sdd/test/` — inherited TypeScript automated tests
- `tools/cc-sdd/schemas/` — inherited hand-authored schema snapshots retained as migration inputs
- `tools/cc-sdd/templates/` — current files installed into consumer projects
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
- `tools/specbind/src/cli.rs` — shared CLI output contract, rendering helpers, and command-family re-exports
- `tools/specbind/src/cli/` — stable command-family facades for migration, reads, tasks, external input, and lifecycle operations
- `tools/specbind/src/cli/read/` — artifact/check, installation, catalog, and project-scope read command execution/rendering
- `tools/specbind/src/cli/lifecycle/` — completion, gate, milestone mutation, release, contract-review, and status command execution/rendering
- `tools/specbind/src/artifacts.rs` — stable public facade and result models for spec-local artifact reads
- `tools/specbind/src/artifacts/discovery.rs` — filesystem discovery, logical identity, metadata-profile validation, and partial inventory
- `tools/specbind/src/artifacts/resolution.rs` — typed Spec and Task loads, gate-input resolution, fingerprints, and traceability projections
- `tools/specbind/src/installation/install.rs` — installation planning, guarded asset application, and repository guards
- `tools/specbind/src/installation/removal.rs` — exact agent-removal and project-uninstall planning, Git/filesystem guards, and config-last apply
- `tools/specbind/src/installation/agent_role.rs` — stable subagent roles, cost-aware defaults, project capability overrides, and Codex and Claude Code rendering
- `tools/specbind/src/migration.rs` — public historical cc-sdd migration models and orchestration boundary
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
- `tools/specbind/assets/protocols/` — immutable English product protocols exposed by `protocol read`
- `tools/specbind/assets/skills/` — one agent-neutral source per product-managed skill
- `tools/specbind/assets/rules/` — official default project-owned shared rules written by `install`
- `tools/specbind/src/documents/requirements.rs` — Markdown AST validation and canonical Requirement ID extraction
- `tools/specbind/src/documents/design.rs` — Design emphasis-marker extraction and Front Matter traceability equality
- `tools/specbind/src/documents/traceability.rs` — cross-artifact Requirement existence plus active Design and Task coverage, exposed by `check traceability`
- `tools/specbind/src/documents/contract.rs` — canonical Contract Markdown parsing and artifact-local semantic validation
- `tools/specbind/src/read_model/contract_graph.rs` — project-wide Contract reference, ownership-overlap, and dependency-cycle read model
- `tools/specbind/src/documents/roadmap.rs` — active Roadmap parsing, DAG validation, and normalized cross-spec scope projection
- `tools/specbind/src/lifecycle/cross_spec_review.rs` — strict review candidate and authoritative Contract-first input revision resolution
- `tools/specbind/src/foundation/fingerprint.rs` — Markdown and normalized typed task-plan fingerprint producers
- `tools/specbind/src/read_model/freshness.rs` — gate-local requirements, design, and tasks freshness evaluation
- `tools/specbind/src/lifecycle/approval.rs` — guarded Requirements, Design, and Tasks gate approval and invalidation transitions
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
- `scripts/check_decisions.py` — Decision filename, heading, identifier, and repository-map consistency check
- `.github/workflows/rust.yml` — Windows and Linux Rust verification
- `.github/workflows/decisions.yml` — focused Decision-index verification for affected pushes and pull requests

## User documentation

| Document | Role |
| --- | --- |
| [Documentation site transition entry](./index.md) | GitHub Pages landing page while the English root tree is prepared |
| [Documentation authoring policy](./documentation-authoring.md) | Japanese-first source policy, final bilingual path mapping, and contributor verification |
| [Japanese user guide (Preview)](./ja/index.md) | Entry point for source-built Preview users |
| [Japanese Getting Started](./ja/guide/getting-started.md) | Project installation and the first Spec-backed change through implementation validation |
| [Japanese basic concepts](./ja/guide/concepts.md) | User-facing explanation of Skills, CLI, Specs, Milestones, artifacts, and gates |
| [Japanese customization guide](./ja/guide/customization.md) | Supported project customization surfaces, boundaries, and verification entry points |
| [Japanese removal and uninstall guide](./ja/guide/uninstall.md) | Exact Agent removal and explicit durable-knowledge uninstall workflow |
| [cc-sdd migration entry](./guide/migration/cc-sdd.md) | Language-neutral entry for automatic and agent-assisted migration guidance |
| [Japanese cc-sdd migration guide](./ja/guide/migrate-from-cc-sdd.md) | User and agent playbook for a guarded cc-sdd cutover |
| [English cc-sdd migration guide](./guide/en/migrate-from-cc-sdd.md) | English user and agent playbook for a guarded cc-sdd cutover |
| [Current generated skill index](./current-skill-index.md) | Current installed skill surface for Codex and Claude Code |
| [Current generated artifact index](./current-artifact-index.md) | Current installation and lifecycle artifact surface |

The documentation site is configured by `mkdocs.yml`, built with the pinned
dependency in `requirements-docs.txt`, and deployed by
`.github/workflows/pages.yml`. Run `python -m mkdocs build --strict` from the
repository root to verify it locally.

## Design documents

| Document | Role |
| --- | --- |
| [Target skill catalog](./design/target-skill-catalog.md) | Working catalog for proposed skill names and responsibilities |
| [Target artifact catalog](./design/target-artifact-catalog.md) | Proposed artifact ownership and lifecycle |
| [Target workflows](./design/target-workflows.md) | Proposed user journeys and responsibility boundaries |
| [Active spec lifecycle](./design/active-spec-lifecycle.md) | Detailed draft for active specs and milestone finalization |
| [Spec state machine](./design/spec-state-machine.md) | Draft per-spec states, events, invalidation rules, and transition diagram |
| [Milestone state machine](./design/milestone-state-machine.md) | Derived milestone stage, phase-relative dependency waves, and aggregate read model |
| [CLI and agent boundary](./design/cli-agent-boundary.md) | Proposed boundary between bundled deterministic CLI operations and agent skills |
| [Cross-spec contracts](./design/cross-spec-contracts.md) | Detailed draft for persistent cross-spec seam manifests and contract-first review |
| [Rust CLI migration](./design/rust-cli-migration.md) | Working migration plan from the TypeScript installer to the complete Rust CLI |
| [Restraint mechanisms](./design/restraint-mechanisms.md) | Idea-stage options for suppressing over-engineering in adopting projects |
| [Skill forward tests](./skill-forward-tests.md) | Index for the behavioral procedure, measurement ledger, and scenario contracts for embedded skills |

This repository's own agent skills live in `.agents/skills/`: `specbind-new-agent` for adding coding-agent support, and `specbind-forward-test` for running the behavioral verification above. They are development assets and are never installed into a consumer project.

## Decision records

Accepted decisions are authoritative. A superseded decision is retained for history; follow the decision that replaced it.
Run `python scripts/check_decisions.py` from the repository root to verify that
Decision filenames, headings, identifiers, and this index remain consistent.

| Decision | Status | Summary |
| --- | --- | --- |
| [0001](./design/decisions/0001-skill-naming.md) | Superseded by 0075 | Replacement of inherited skill naming |
| [0002](./design/decisions/0002-project-release-adapter.md) | Accepted | Core-plus-project-adapter release direction |
| [0003](./design/decisions/0003-active-requirement-set.md) | Accepted | Storage contract for current milestone Requirement IDs |
| [0004](./design/decisions/0004-release-history-layout.md) | Accepted | Per-spec changelog and roadmap archive layout |
| [0005](./design/decisions/0005-active-change-abandonment.md) | Accepted | Scope removal, abandonment, and rollback boundaries |
| [0006](./design/decisions/0006-rust-cli.md) | Accepted | Direction to reimplement the complete SpecBind CLI in Rust |
| [0007](./design/decisions/0007-spec-root.md) | Accepted | Configurable spec root with `.specbind` as the new-project default |
| [0008](./design/decisions/0008-customization-surface.md) | Accepted | Shared templates and rules as the stable project customization surface |
| [0009](./design/decisions/0009-milestone-cli-boundary.md) | Accepted | Discovery-plus-Rust-CLI milestone responsibility boundary |
| [0010](./design/decisions/0010-release-execution-boundary.md) | Accepted | AI adapter execution and Rust CLI release-finalization boundary |
| [0011](./design/decisions/0011-cross-spec-contract.md) | Accepted | Persistent contract manifest and contract-first contract review direction |
| [0012](./design/decisions/0012-delegated-approval.md) | Accepted | Explicit-versus-delegated gate approval and non-interactive execution boundary |
| [0013](./design/decisions/0013-structured-task-artifact.md) | Accepted | Structured `tasks.yaml` source-of-truth direction |
| [0014](./design/decisions/0014-structured-spec-metadata.md) | Accepted | Structured `spec.yaml` source-of-truth direction |
| [0015](./design/decisions/0015-runtime-schema-layout.md) | Accepted | Versioned runtime-schema location and validation layers |
| [0016](./design/decisions/0016-fingerprint-value-format.md) | Accepted | Tagged lowercase SHA-256 fingerprint representation |
| [0017](./design/decisions/0017-requirements-gate-inputs.md) | Accepted | Requirements-gate fingerprint boundary excluding `brief.md` |
| [0018](./design/decisions/0018-gate-input-comparison.md) | Accepted | Markdown normalization, Requirement ID snapshot, and task-plan projection boundaries |
| [0019](./design/decisions/0019-task-ordering-and-dependencies.md) | Accepted | Ordered task execution with sparse dependency exceptions |
| [0020](./design/decisions/0020-positional-task-ids.md) | Accepted | cc-sdd-style positional numeric Task IDs |
| [0021](./design/decisions/0021-optional-completion-criteria.md) | Accepted | Dedicated, conditionally optional task completion criteria |
| [0022](./design/decisions/0022-no-optional-tasks.md) | Accepted | Removal of inherited optional tasks from `tasks.yaml` v1 |
| [0023](./design/decisions/0023-sparse-task-plan-shape.md) | Accepted | Strict sparse group and executable-task plan objects |
| [0024](./design/decisions/0024-sparse-task-execution-state.md) | Accepted | Sparse persisted completed and blocked task states |
| [0025](./design/decisions/0025-task-read-model.md) | Accepted | Human and JSON CLI projections for task status and detail |
| [0026](./design/decisions/0026-runtime-implementation-notes.md) | Accepted | Persistent free-form implementation memory for later agents |
| [0027](./design/decisions/0027-spec-local-task-dependencies.md) | Accepted | Local-only Task IDs with roadmap and contract routing for cross-spec dependencies |
| [0028](./design/decisions/0028-task-plan-fingerprint.md) | Accepted | Normalized typed-plan projection and canonical fingerprint algorithm |
| [0029](./design/decisions/0029-completion-validation-handshake.md) | Accepted | Clean Git revision handshake for guarded completion validation |
| [0030](./design/decisions/0030-persist-only-accepted-completion-evidence.md) | Accepted | Current-state-only storage for successful completion evidence |
| [0031](./design/decisions/0031-project-scoped-revision-format.md) | Accepted | Scalar Git implementation revision interpreted from project context |
| [0032](./design/decisions/0032-gate-local-freshness-chain.md) | Accepted | Gate-local revision ownership and cascading freshness semantics |
| [0033](./design/decisions/0033-completion-mechanical-checks.md) | Accepted | Concise categorized command evidence for successful completion validation |
| [0034](./design/decisions/0034-do-not-persist-semantic-pass-flags.md) | Accepted | Semantic validation protocol without redundant persisted pass flags |
| [0035](./design/decisions/0035-roadmap-owned-cross-spec-review.md) | Superseded by 0078 | Roadmap ownership for contract-impact and downstream-review evidence |
| [0036](./design/decisions/0036-rfc3339-gate-timestamps.md) | Accepted | Timezone-qualified RFC 3339 format for gate timestamps |
| [0037](./design/decisions/0037-minimal-completion-evidence-shape.md) | Accepted | Strict three-field completion evidence object |
| [0038](./design/decisions/0038-design-gate-inputs.md) | Accepted | Design and contract fingerprint inputs for the design gate |
| [0039](./design/decisions/0039-minimal-tasks-gate-evidence.md) | Accepted | Minimal approval evidence for the normalized task-plan projection |
| [0040](./design/decisions/0040-state-gate-evidence-invariants.md) | Accepted | Sparse cumulative evidence and semantic state-to-gate invariants |
| [0041](./design/decisions/0041-no-per-spec-change-id.md) | Accepted | Milestone-plus-spec identity without a separate per-spec change ID |
| [0042](./design/decisions/0042-sequential-milestone-id.md) | Superseded by 0043 | Project-sequential milestone ID |
| [0043](./design/decisions/0043-uuidv7-milestone-id.md) | Accepted | Branch-safe UUID v7 milestone IDs |
| [0044](./design/decisions/0044-minimal-spec-root.md) | Accepted | Minimal strict `spec.yaml` root and active-change object |
| [0045](./design/decisions/0045-okf-markdown-artifacts.md) | Accepted | OKF Front Matter profile for every managed Markdown artifact |
| [0046](./design/decisions/0046-roadmap-work-items.md) | Accepted | Grouped Roadmap work items for new Specs, Spec updates, and Direct changes |
| [0047](./design/decisions/0047-sparse-direct-change-status.md) | Accepted | Sparse persisted completed status for Direct changes |
| [0048](./design/decisions/0048-okf-spec-log.md) | Accepted | Canonical OKF `log.md` for per-spec release history |
| [0049](./design/decisions/0049-okf-authoring-rule.md) | Superseded by 0094 | Concise installed OKF authoring rule |
| [0050](./design/decisions/0050-global-cross-spec-review.md) | Accepted | One global accepted contract review per milestone |
| [0051](./design/decisions/0051-current-state-roadmap.md) | Accepted | Current-state-only active Roadmap |
| [0052](./design/decisions/0052-project-state-artifacts.md) | Accepted | Project-wide machine state separated from steering |
| [0053](./design/decisions/0053-minimal-cross-spec-review-state.md) | Superseded by 0078 | Structured classifications paired with an AI-authored review |
| [0054](./design/decisions/0054-milestone-baseline-revision.md) | Accepted | Milestone baseline revision as the contract-diff anchor |
| [0055](./design/decisions/0055-cross-spec-review-inputs.md) | Accepted | Contract-first review inputs |
| [0056](./design/decisions/0056-canonical-contract-markdown.md) | Accepted | Canonical five-section Markdown contract manifests |
| [0057](./design/decisions/0057-type-based-artifact-discovery.md) | Accepted | Type-based OKF artifact discovery |
| [0058](./design/decisions/0058-artifact-inventory-read-model.md) | Accepted | Artifact inventory separated from raw content reads |
| [0059](./design/decisions/0059-okf-artifact-templates.md) | Accepted | Final-form OKF documents as artifact templates |
| [0060](./design/decisions/0060-requirement-id-and-heading-mapping.md) | Accepted | Requirement IDs derived from mapped headings and list position |
| [0061](./design/decisions/0061-design-requirement-traceability.md) | Accepted | Explicit Design traceability in Front Matter and body markers |
| [0062](./design/decisions/0062-minimal-active-brief-profile.md) | Accepted | Minimal free-form active brief profile |
| [0063](./design/decisions/0063-free-form-release-adapter-profile.md) | Accepted | Free-form agent-interpreted release adapter profile |
| [0064](./design/decisions/0064-path-scoped-release-finalization-guard.md) | Accepted | Path-scoped release finalization Git guard |
| [0065](./design/decisions/0065-forceable-release-target-check.md) | Superseded by 0081 | Guarded release finalization with a narrow force override |
| [0066](./design/decisions/0066-agent-judged-release-and-cli-log-insertion.md) | Accepted | Agent-judged release success with CLI-inserted spec logs |
| [0067](./design/decisions/0067-text-first-english-cli-results.md) | Accepted | Concise, text-first, English-only CLI results |
| [0068](./design/decisions/0068-release-log-summary-input.md) | Accepted | Strict JSON per-spec summaries as release-finalization input |
| [0069](./design/decisions/0069-stateless-release-preflight.md) | Accepted | Stateless read-only release preflight |
| [0070](./design/decisions/0070-derived-release-readiness.md) | Accepted | Derived release readiness without a new evidence artifact |
| [0071](./design/decisions/0071-no-partial-milestone-release.md) | Accepted | No partially released milestone representation |
| [0072](./design/decisions/0072-explicit-release-rebinding.md) | Accepted | Explicit operation required for release rebinding |
| [0073](./design/decisions/0073-portable-release-version.md) | Accepted | Opaque portable release-version label |
| [0074](./design/decisions/0074-defer-json-cli-output.md) | Accepted | JSON CLI output deferred until after v1 |
| [0075](./design/decisions/0075-v1-skill-and-orchestration-scope.md) | Accepted | Fixed v1 skill and orchestration scope |
| [0076](./design/decisions/0076-project-global-artifact-language.md) | Accepted | One project-global artifact language |
| [0077](./design/decisions/0077-v1-installation-distribution-and-migration.md) | Accepted | v1 installation, distribution, and cc-sdd migration contract |
| [0078](./design/decisions/0078-contract-first-review-between-design-and-tasks.md) | Accepted | One free-form contract-first review between Design and Tasks |
| [0079](./design/decisions/0079-milestone-local-research.md) | Accepted | Optional research as a milestone-local singleton |
| [0080](./design/decisions/0080-v1-task-contract-and-completion-details.md) | Accepted | Fixed v1 Task, Contract, and completion details |
| [0081](./design/decisions/0081-v1-release-git-path-and-cli-safety.md) | Accepted | Tightened v1 release, Git, path, and CLI safety |
| [0082](./design/decisions/0082-derived-milestone-state-machine.md) | Accepted | Derived milestone state and phase-relative dependency waves |
| [0083](./design/decisions/0083-json-schema-structural-authority.md) | Superseded by 0085 | JSON Schema authoritative over Rust artifact models |
| [0084](./design/decisions/0084-rust-dependency-strategy.md) | Accepted | Focused Rust dependencies behind SpecBind-owned boundaries |
| [0085](./design/decisions/0085-rust-wire-model-schema-generation.md) | Accepted | JSON Schema generated from versioned Rust wire models |
| [0086](./design/decisions/0086-completion-cli-handshake.md) | Accepted | Spec and Direct completion CLI handshake |
| [0087](./design/decisions/0087-milestone-review-cli.md) | Accepted | Milestone-owned contract review commands |
| [0088](./design/decisions/0088-gate-approval-cli.md) | Accepted | Spec gate approval and invalidation commands |
| [0089](./design/decisions/0089-milestone-creation-cli.md) | Accepted | Milestone creation, scope, and rebaseline commands |
| [0090](./design/decisions/0090-standalone-check-cli.md) | Accepted | Standalone traceability and contract check commands |
| [0091](./design/decisions/0091-installed-template-surface.md) | Accepted | Embedded scaffold set separated from the installed customization surface |
| [0092](./design/decisions/0092-template-skill-authoring-boundary.md) | Accepted | Artifact scaffold guidance separated from authoring workflow policy |
| [0093](./design/decisions/0093-default-shared-rule-set.md) | Accepted | Narrow installed shared-rule set and explicit skill consumers |
| [0094](./design/decisions/0094-embedded-product-protocols.md) | Accepted | Immutable shared semantic protocols exposed through the CLI |
| [0095](./design/decisions/0095-task-progress-cli.md) | Accepted | Guarded task execution progress commands |
| [0096](./design/decisions/0096-skill-asset-layout.md) | Accepted | One agent-neutral source per product-managed skill |
| [0097](./design/decisions/0097-discovery-routing-and-read-models.md) | Accepted | Discovery routing contract and the read models it requires |
| [0098](./design/decisions/0098-steering-read-surface.md) | Accepted | Steering documents identified by OKF type and read through the CLI |
| [0099](./design/decisions/0099-project-instruction-block.md) | Accepted | Marked SpecBind block maintained in root agent instruction files |
| [0100](./design/decisions/0100-requirements-skill-contract.md) | Accepted | Active selection, approval, review loop, and invalidation for the requirements skill |
| [0101](./design/decisions/0101-project-adapter-directory-and-git-workflow.md) | Accepted | Project adapter directory and free-form Git workflow guidance |
| [0102](./design/decisions/0102-workflow-entry-condition.md) | Accepted | When a request enters the SpecBind workflow at all |
| [0103](./design/decisions/0103-schema-read-surface.md) | Accepted | Embedded artifact and command-input schemas readable through the CLI |
| [0104](./design/decisions/0104-design-skill-contract.md) | Accepted | Reads, Contract update timing, approval, and rewind for the design skill |
| [0105](./design/decisions/0105-tasks-skill-contract.md) | Accepted | Review ordering, schema-driven authoring, renumbering safety, and approval for the tasks skill |
| [0106](./design/decisions/0106-contract-review-naming.md) | Accepted | Rename of the cross-spec review to the contract review |
| [0107](./design/decisions/0107-spec-status-contract-review-barrier.md) | Accepted | Contract-review barrier reported in Spec status from the tasks state onward |
| [0108](./design/decisions/0108-contract-review-skill-contract.md) | Accepted | Reads, baseline comparison, deep-input discipline, remediation, and acceptance for the contract review skill |
| [0109](./design/decisions/0109-subagent-dispatch-contract.md) | Accepted | Fresh-context subagent dispatch, its neutral expression, and the structured return |
| [0110](./design/decisions/0110-implement-skill-contract.md) | Accepted | Item selection, per-task dispatch cycle, bounded failure routing, and where the implement run stops |
| [0111](./design/decisions/0111-review-task-and-debug-skill-contracts.md) | Accepted | Two moments, the read-only boundary, and unfresh-context honesty for the review and debug skills |
| [0112](./design/decisions/0112-validate-implementation-skill-contract.md) | Accepted | Completion-verification protocol, the three verdicts, run-not-assembled evidence, and the multi-Spec metadata commit |
| [0113](./design/decisions/0113-verify-completion-skill-contract.md) | Accepted | Claim-shaped subject, distinct verdicts, and the consequence-free boundary for the claim verification skill |
| [0114](./design/decisions/0114-validate-design-skill-contract.md) | Accepted | Two verdicts with no inconclusive escape, the deletion test, and no self-initiated rewind for design validation |
| [0115](./design/decisions/0115-release-skill-contract.md) | Accepted | Binding order, confirmed publication, verification as a completion claim, and delivered-change summaries |
| [0116](./design/decisions/0116-spec-status-delegated-gates.md) | Accepted | Delegated gates and their workflow reported in Spec status |
| [0117](./design/decisions/0117-steering-authoring-contract.md) | Accepted | Steering authoring, in-place synchronization, and the steering template scope |
| [0118](./design/decisions/0118-gap-analysis-skill-contract.md) | Accepted | Gap analysis before Requirements, the request-mediated influence path, and marked conclusions |
| [0119](./design/decisions/0119-writing-while-a-completion-stands.md) | Accepted | One statement of what writing costs once a Spec holds accepted completion |
| [0120](./design/decisions/0120-quick-and-batch-orchestration-contracts.md) | Accepted | Quick-plan and batch-plan orchestration, phase-specific dependency shape, and retry classification |
| [0121](./design/decisions/0121-requirements-coverage-is-not-slots.md) | Accepted | Requirements coverage bounded to what the Spec owes |
| [0122](./design/decisions/0122-finding-disposition-and-deferred-destination.md) | Accepted | Finding disposition and the project-named destination for deferred findings |
| [0123](./design/decisions/0123-reverse-traceability-and-unconsumed-seams.md) | Accepted | Reverse task-scope traceability and unconsumed exported seams |
| [0124](./design/decisions/0124-pre-1.0-binary-release-line.md) | Accepted | Pre-1.0 public binary release and distribution contract |
| [0125](./design/decisions/0125-agent-assisted-cc-sdd-migration.md) | Accepted | Agent-assisted cc-sdd migration and GitHub Pages handoff contract |
| [0126](./design/decisions/0126-cli-owned-cc-sdd-migration-resolution.md) | Accepted | CLI-owned accepted cc-sdd migration resolution and freshness contract |
| [0127](./design/decisions/0127-retire-cc-sdd-source-at-final-cutover.md) | Accepted | Git-guarded retirement of cc-sdd sources at final cutover |
| [0128](./design/decisions/0128-plan-orchestrator-names.md) | Accepted | Planning orchestrator names expose their Tasks-approval stopping point |
| [0129](./design/decisions/0129-agent-role-capability-adapters.md) | Accepted | Agent-role capability adapters with project model overrides |
| [0130](./design/decisions/0130-mise-github-backend-installation.md) | Accepted | Installation through mise's GitHub backend over the existing release assets |
| [0131](./design/decisions/0131-okf-deferred-destination-and-adapter-state.md) | Accepted | Exact OKF-conformant deferred destination and visible adapter state |
| [0132](./design/decisions/0132-target-aware-template-resolution.md) | Accepted | Target-aware Spec template path and provenance resolution |
| [0133](./design/decisions/0133-phase-relative-spec-status.md) | Accepted | Phase-relative Spec health, expected work, and workflow action |
| [0134](./design/decisions/0134-phase-relative-milestone-review-health.md) | Accepted | Absent milestone review treated as expected workflow work |
| [0135](./design/decisions/0135-phase-relative-requirements-status.md) | Accepted | Absent Requirements treated as expected phase work |
| [0136](./design/decisions/0136-phase-relative-worktree-blocker.md) | Accepted | Worktree cleanliness reported only when it blocks current progress |
| [0137](./design/decisions/0137-active-default-git-checkpoints.md) | Accepted | Active local Git checkpoints for newly installed projects |
| [0138](./design/decisions/0138-dedicated-adapter-scaffold-marker.md) | Accepted | Dedicated exact marker for inactive adapter scaffolds |
| [0139](./design/decisions/0139-scoped-artifact-instructions.md) | Accepted | Lifecycle-scoped create, maintain, and consume instructions |
| [0140](./design/decisions/0140-release-adapter-bootstrap-and-finalization-checkpoint.md) | Accepted | One-time Release adapter bootstrap and post-finalization metadata checkpoint |
| [0141](./design/decisions/0141-guarded-agent-removal-and-project-uninstall.md) | Accepted | Exact planned agent removal and explicit durable-knowledge uninstall policy |
| [0142](./design/decisions/0142-bilingual-documentation-authoring-and-publishing.md) | Accepted | Japanese-first documentation authoring and English-default bilingual publishing hierarchy |
