# Target artifact catalog

This document is the working catalog for the files the future SpecBind workflow will install, create, maintain, or remove. It describes proposed artifact lifecycles before they are implemented.

The catalog is intentionally separate from the [current generated artifact index](../current-artifact-index.md):

- The current index records what the CLI and skills produce today.
- This catalog records intended ownership and lifecycle changes.

Related documents:

- [Target skill catalog](./target-skill-catalog.md)
- [Target workflows](./target-workflows.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)
- [Decision 0002: project release adapter](./decisions/0002-project-release-adapter.md)
- [Decision 0003: active requirement set](./decisions/0003-active-requirement-set.md)
- [Decision 0004: release history layout](./decisions/0004-release-history-layout.md)
- [Decision 0005: active change abandonment](./decisions/0005-active-change-abandonment.md)
- [Decision 0007: configurable `.specbind` root](./decisions/0007-spec-root.md)

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

## Project-level artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Required for every active milestone, including single-spec work. | Initiated by `specbind-discovery`, persisted by the proposed milestone lifecycle, and archived by `specbind-release`. | Draft | Holds a machine-generated milestone ID and an initially optional release-version binding. An explicitly abandoned unreleased roadmap is removed rather than release-archived. `{{SPEC_DIR}}` remains a placeholder until the root directory decision is made. |
| `{{SPEC_DIR}}/releases/<version>-roadmap.md` | Persists as the released milestone-wide scope and dependency record. | `specbind-release`. | Accepted | Each release adds a new flat file after verified publication; archive collisions must not overwrite history. |

## Settings artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/settings/release.md` | Installed as a scaffold and maintained as project configuration. | Project maintainers; consumed by `specbind-release`. | Draft | Defines Prepare, Publish, Verify, and After finalize instructions without overriding core release gates. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Draft | A release must not delete a spec merely because its milestone completed. |
| `brief.md` | Exists only for one active milestone change. | `specbind-discovery`. | Draft | Removed by successful release finalization; same-milestone deltas merge into the active brief. |
| `requirements.md` | Holds the complete currently valid requirements across releases. | Requirements workflow. | Draft | The active requirement set is a separate milestone-scoped concept. |
| `design.md` | Holds the complete currently valid design across releases. | Design workflow. | Draft | Revised in place for an active change. |
| `tasks.md` | Exists only for the active milestone's task plan. | Task and implementation workflows. | Draft | Starts fresh between milestones and is removed by successful release finalization. |
| `changelog.md` | Persists per spec as an index of released changes and evidence. | Release finalization workflow. | Accepted | Released entries use release version as the human-facing key and milestone ID as secondary trace metadata. Unreleased abandoned work is omitted by default. |
| `spec.json` | Represents lifecycle, active-change metadata, active Requirement IDs, and current approvals. | Spec lifecycle workflows. | Draft | Source of truth for current milestone scope; must also represent released / no-active-change without requiring `brief.md` or `tasks.md`. |

## Open questions

- What exact schema and ID format identify the active milestone, release binding, and remaining active-change fields?
- What exact Markdown schema and validation rules should `settings/release.md` use?
- What exact evidence schema must the release skill require before finalization?
- How are superseded or removed product capabilities reflected in long-lived specs?
- Should projects be able to opt into a separate audit artifact for abandoned, unreleased milestones?
