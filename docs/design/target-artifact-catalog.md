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
- [Decision 0057: type-based artifact discovery](./decisions/0057-type-based-artifact-discovery.md)
- [Decision 0059: OKF artifact templates](./decisions/0059-okf-artifact-templates.md)
- [Decision 0060: Requirement ID and heading mapping](./decisions/0060-requirement-id-and-heading-mapping.md)
- [Decision 0061: Design Requirement traceability](./decisions/0061-design-requirement-traceability.md)
- [Decision 0062: minimal active brief profile](./decisions/0062-minimal-active-brief-profile.md)
- [Decision 0063: free-form release adapter profile](./decisions/0063-free-form-release-adapter-profile.md)
- [Decision 0064: path-scoped release finalization guard](./decisions/0064-path-scoped-release-finalization-guard.md)
- [Decision 0065: forceable release target check](./decisions/0065-forceable-release-target-check.md)
- [Decision 0066: agent-judged release and CLI log insertion](./decisions/0066-agent-judged-release-and-cli-log-insertion.md)

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
- Every OKF profile permits and preserves unknown top-level Front Matter extensions under Decision 0045, while known fields and nested SpecBind-owned structures retain their profile-specific validation.

## Project-level artifacts

| Artifact or target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Required for every active milestone, including single-spec work. | Intent confirmed by `specbind-discovery`, persisted by Rust CLI milestone operations, and archived by `specbind-release`. | Draft | An OKF concept with `type: SpecBind Roadmap`; its frontmatter holds a branch-safe UUID v7 milestone ID, the Decision 0054 branch-local Git baseline, an initially null release-version binding, and grouped work items. It represents current state only under Decision 0051 and carries no detailed cross-spec review evidence. An explicitly abandoned unreleased roadmap is removed rather than release-archived. |
| `{{SPEC_DIR}}/state/cross-spec-review.md` | Exists only while the active milestone has a current accepted global cross-spec review. | Cross-spec review authors a candidate judgment; guarded Rust CLI operations persist it for status, completion, and release checks. | Accepted | OKF project-state concept under Decisions 0052 through 0055. Frontmatter retains compact per-item classifications and contract-first input revisions, while the body preserves the AI-authored judgment. Ordinary agents do not preload it, and its milestone ID must match the roadmap. |
| `{{SPEC_DIR}}/releases/<version>-roadmap.md` | Persists as the released milestone-wide scope and dependency record. | `specbind-release`. | Accepted | Each release adds the final active-roadmap snapshot as a new flat file after verified publication; archive collisions must not overwrite history. |
| `{{SPEC_DIR}}/releases/<version>-cross-spec-review.md` | Persists the final accepted milestone-wide cross-spec review evidence and judgment. | `specbind-release`. | Accepted | Companion archive moved from `state/` during the same finalization transaction as the roadmap. |

## Settings artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/settings/templates/` | Installed from official defaults, then maintained and version-controlled by the project. | Project maintainers; consumed by authoring skills. | Accepted | Supported customization surface for generated document structure and format. Spec Markdown templates are final-form OKF prototypes under Decision 0059; their relative paths define initial output paths and instruction comments are removed from materialized artifacts. Updates must not silently overwrite local changes or reconcile existing specs. |
| `{{SPEC_DIR}}/settings/rules/` | Installed from official defaults, then maintained and version-controlled by the project. | Project maintainers; consumed by all supported agents. | Accepted | Supported customization surface for shared judgment criteria and generation principles; includes the concise `okf-artifacts.md` authoring rule accepted by Decision 0049 and replaces editable agent-specific rule copies as the target model. |
| `{{SPEC_DIR}}/settings/release.md` | Installed as a scaffold and maintained as project configuration. | Project maintainers; consumed by `specbind-release`. | Accepted | Free-form OKF project guidance under Decision 0063. Its only known field is `type`; headings are not machine syntax and an empty body means no adapter-specific actions. It cannot override core release gates or evidence requirements. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Draft | A release must not delete a spec merely because its milestone completed. |
| `SpecBind Brief` singleton (`brief.md` by default) | Exists only for one active milestone change. | `specbind-discovery`. | Accepted | Minimal free-form OKF input under Decision 0062. Its only known field is `type`; the CLI does not parse its body or fingerprint it for gate evidence. Same-milestone deltas merge into it, and successful release finalization removes it. |
| `SpecBind Requirements` singleton (`requirements.md` by default) | Holds the complete currently valid requirements across releases. | Requirements workflow. | Accepted | Discovered by OKF type. Front Matter maps the two customizable structural heading labels, while Requirement IDs derive from explicit group number plus Acceptance Criteria list position under Decision 0060. The active requirement set is a separate milestone-scoped concept. |
| `SpecBind Design` collection (`design.md`, `artifact_id: main` by default) | Holds the complete currently valid design across one or more focused documents. | Design workflow. | Accepted | Discovered by OKF type plus stable `artifact_id`; revised in place for an active change. Its stable v1 Front Matter contract is `type`, `artifact_id`, and non-empty `requirement_ids`; unknown project metadata is allowed but ignored by SpecBind. The mapping set exactly matches its italic `_Requirements: ..._` body markers under Decision 0061. |
| `SpecBind Contract` singleton (`contract.md` by default) | Holds the current minimal cross-spec seam manifest across releases. | Design and cross-spec review workflows. | Accepted | Discovered by OKF type. Contains stable Owns, Exports, Consumes, Invariants, and File Ownership entries; never an internal-design summary. |
| `SpecBind Implementation Notes` collection (`implementation-notes.md`, `artifact_id: main` by default) | Optionally persists spec-scoped implementation knowledge across milestones and releases. | Task generation, implementation, debugging, review, and implementation-validation workflows. | Accepted | The known Front Matter fields are `type` and stable `artifact_id`. Live bodies are non-empty free-form Markdown; absence represents no durable notes. Gate fingerprints exclude the collection, release preserves it, and project-wide knowledge should be promoted to `steering/`. |
| `tasks.yaml` | Exists only for the active milestone's structured task plan and execution state. | Task and implementation workflows. | Accepted | The only canonical task artifact; starts fresh between milestones and is removed by successful release finalization. No parallel `tasks.md` view is maintained. |
| `log.md` | Persists per spec as the OKF reserved update log for released changes. | Release finalization workflow. | Accepted | Has no frontmatter. Under Decision 0066, AI supplies one delivered-change summary per participating spec and the CLI inserts its canonical entry under newest-first ISO dates. Release version is the human-facing label, milestone ID is secondary trace metadata, and abandoned work is omitted. |
| `spec.yaml` | Represents lifecycle, active-change metadata, active Requirement IDs, and gate evidence. | Spec lifecycle workflows. | Accepted | The only canonical per-spec metadata artifact; its target states and events are defined in the spec state machine and must represent released / no-active-change without requiring a brief artifact or `tasks.yaml`. |

## Open questions

- What exact transport and inline-Markdown contract should per-spec release-log summaries use?
- How are superseded or removed product capabilities reflected in long-lived specs?
- Should projects be able to opt into a separate audit artifact for abandoned, unreleased milestones?
