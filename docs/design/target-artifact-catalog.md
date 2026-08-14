# Target artifact catalog

This document is the working catalog for the files the future SpecBind workflow will install, create, maintain, or remove. It describes proposed artifact lifecycles before they are implemented.

The catalog is intentionally separate from the [current generated artifact index](../current-artifact-index.md):

- The current index records what the CLI and skills produce today.
- This catalog records intended ownership and lifecycle changes.

Related documents:

- [Target skill catalog](./target-skill-catalog.md)
- [Target workflows](./target-workflows.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)
- [Spec state machine](./spec-state-machine.md)
- [Decision 0002: project release adapter](./decisions/0002-project-release-adapter.md)
- [Decision 0003: active requirement set](./decisions/0003-active-requirement-set.md)
- [Decision 0004: release history layout](./decisions/0004-release-history-layout.md)
- [Decision 0005: active change abandonment](./decisions/0005-active-change-abandonment.md)
- [Decision 0007: configurable `.specbind` root](./decisions/0007-spec-root.md)
- [Decision 0008: shared settings customization](./decisions/0008-customization-surface.md)
- [Decision 0011: cross-spec contract manifest](./decisions/0011-cross-spec-contract.md)
- [Decision 0012: delegated approval](./decisions/0012-delegated-approval.md)
- [Decision 0013: structured task artifact](./decisions/0013-structured-task-artifact.md)
- [Decision 0014: structured spec metadata](./decisions/0014-structured-spec-metadata.md)
- [Decision 0045: OKF Markdown artifacts](./decisions/0045-okf-markdown-artifacts.md)
- [Decision 0046: grouped roadmap work items](./decisions/0046-roadmap-work-items.md)
- [Decision 0047: sparse direct-change status](./decisions/0047-sparse-direct-change-status.md)
- [Decision 0048: OKF per-spec log](./decisions/0048-okf-spec-log.md)
- [Decision 0049: concise OKF authoring rule](./decisions/0049-okf-authoring-rule.md)

Status: Draft

`{{SPEC_DIR}}` is configurable and defaults to `.specbind` for new installations. Existing explicitly configured roots remain valid; migration from the inherited `.kiro` default must be deliberate and guarded.

## Lifecycle principles

- Specs are active specifications of the product, not disposable delivery plans.
- Existing specs should be maintained as the represented behavior changes.
- A milestone groups the work intended for an active release cycle.
- Every milestone has a `roadmap.md`, including a milestone containing only one spec change.
- At most one active milestone is represented by `roadmap.md`.
- Machine-generated milestone identity is separate from its optional, later-bound release version.
- A concrete release version is mandatory before release execution begins.
- Release completion ends the active milestone but does not retire the specs it changed.
- Abandoning an unreleased milestone does not create release history by default.
- The configured spec root is an OKF v0.2 Knowledge Bundle. Its managed Markdown artifacts use YAML frontmatter plus free-form Markdown under Decision 0045.

## Project-level artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Required for every active milestone, including single-spec work. | Intent confirmed by `specbind-discovery`, persisted by Rust CLI milestone operations, and archived by `specbind-release`. | Draft | An OKF concept with `type: SpecBind Roadmap`; its frontmatter holds a branch-safe UUID v7 milestone ID, an initially null release-version binding, and grouped work items. It represents current state only under Decision 0051 and carries no detailed cross-spec review evidence. An explicitly abandoned unreleased roadmap is removed rather than release-archived. |
| `{{SPEC_DIR}}/state/cross-spec-review.yaml` | Exists only while the active milestone has a current accepted global cross-spec review. | Cross-spec review through guarded Rust CLI operations; consumed by status, completion, and release checks. | Accepted | Strict project-state YAML under Decision 0052. Ordinary agents do not preload it; the internal milestone ID must match the roadmap. |
| `{{SPEC_DIR}}/releases/<version>-roadmap.md` | Persists as the released milestone-wide scope and dependency record. | `specbind-release`. | Accepted | Each release adds the final active-roadmap snapshot as a new flat file after verified publication; archive collisions must not overwrite history. |
| `{{SPEC_DIR}}/releases/<version>-cross-spec-review.yaml` | Persists the final accepted milestone-wide cross-spec review evidence. | `specbind-release`. | Accepted | Companion archive moved from `state/` during the same finalization transaction as the roadmap. |

## Settings artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/settings/templates/` | Installed from official defaults, then maintained and version-controlled by the project. | Project maintainers; consumed by authoring skills. | Accepted | Supported customization surface for generated document structure and format. Updates must not silently overwrite local changes. |
| `{{SPEC_DIR}}/settings/rules/` | Installed from official defaults, then maintained and version-controlled by the project. | Project maintainers; consumed by all supported agents. | Accepted | Supported customization surface for shared judgment criteria and generation principles; includes the concise `okf-artifacts.md` authoring rule accepted by Decision 0049 and replaces editable agent-specific rule copies as the target model. |
| `{{SPEC_DIR}}/settings/release.md` | Installed as a scaffold and maintained as project configuration. | Project maintainers; consumed by `specbind-release`. | Draft | Defines Prepare, Publish, Verify, and After finalize instructions without overriding core release gates. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Draft | A release must not delete a spec merely because its milestone completed. |
| `brief.md` | Exists only for one active milestone change. | `specbind-discovery`. | Draft | Removed by successful release finalization; same-milestone deltas merge into the active brief. |
| `requirements.md` | Holds the complete currently valid requirements across releases. | Requirements workflow. | Draft | The active requirement set is a separate milestone-scoped concept. |
| `design.md` | Holds the complete currently valid design across releases. | Design workflow. | Draft | Revised in place for an active change. |
| `contract.md` | Holds the current minimal cross-spec seam manifest across releases. | Design and cross-spec review workflows. | Accepted | Contains stable Owns, Exports, Consumes, Invariants, and File Ownership entries; never an internal-design summary. |
| `implementation-notes.md` | Optionally persists spec-scoped implementation knowledge across milestones and releases. | Task generation, implementation, debugging, review, and implementation-validation workflows. | Accepted | The body is free-form Markdown; only the common OKF frontmatter and artifact type profile are structured. Project-wide knowledge should be promoted to `steering/`. |
| `tasks.yaml` | Exists only for the active milestone's structured task plan and execution state. | Task and implementation workflows. | Accepted | The only canonical task artifact; starts fresh between milestones and is removed by successful release finalization. No parallel `tasks.md` view is maintained. |
| `log.md` | Persists per spec as the OKF reserved update log for released changes and evidence. | Release finalization workflow. | Accepted | Has no frontmatter. Entries are grouped under newest-first ISO dates; release version is the human-facing entry label and milestone ID is secondary trace metadata. Unreleased abandoned work is omitted by default. |
| `spec.yaml` | Represents lifecycle, active-change metadata, active Requirement IDs, and gate evidence. | Spec lifecycle workflows. | Accepted | The only canonical per-spec metadata artifact; its target states and events are defined in the spec state machine and must represent released / no-active-change without requiring `brief.md` or `tasks.yaml`. |

## Open questions

- What exact schema and fingerprint inputs represent `state/cross-spec-review.yaml` under Decisions 0050 and 0052?
- What exact Markdown schema and validation rules should `settings/release.md` use?
- What exact evidence schema must the release skill require before finalization?
- What exact Markdown grammar and entry ID format represent contracts?
- How are superseded or removed product capabilities reflected in long-lived specs?
- Should projects be able to opt into a separate audit artifact for abandoned, unreleased milestones?
