# Target artifact catalog

This document is the working catalog for the files the future SpecBind workflow will install, create, maintain, or remove. It describes proposed artifact lifecycles before they are implemented.

The catalog is intentionally separate from the [current generated artifact index](../current-artifact-index.md):

- The current index records what the CLI and skills produce today.
- This catalog records intended ownership and lifecycle changes.

Related documents:

- [Target skill catalog](./target-skill-catalog.md)
- [Target workflows](./target-workflows.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)

Status: Draft

## Lifecycle principles

- Specs are active specifications of the product, not disposable delivery plans.
- Existing specs should be maintained as the represented behavior changes.
- A milestone groups the work intended for an active release cycle.
- Every milestone has a `roadmap.md`, including a milestone containing only one spec change.
- At most one active milestone is represented by `roadmap.md`.
- Machine-generated milestone identity is separate from its optional, later-bound release version.
- A concrete release version is mandatory before release execution begins.
- Release completion ends the active milestone but does not retire the specs it changed.

## Project-level artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Required for every active milestone, including single-spec work. | Created and maintained by `specbind-discovery`; removed by `specbind-release`. | Draft | Holds a machine-generated milestone ID and an initially optional release-version binding. `{{SPEC_DIR}}` remains a placeholder until the root directory decision is made. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Draft | A release must not delete a spec merely because its milestone completed. |
| `brief.md` | Exists only for one active milestone change. | `specbind-discovery`. | Draft | Removed by successful release finalization; same-milestone deltas merge into the active brief. |
| `requirements.md` | Holds the complete currently valid requirements across releases. | Requirements workflow. | Draft | The active requirement set is a separate milestone-scoped concept. |
| `design.md` | Holds the complete currently valid design across releases. | Design workflow. | Draft | Revised in place for an active change. |
| `tasks.md` | Exists only for the active milestone's task plan. | Task and implementation workflows. | Draft | Starts fresh between milestones and is removed by successful release finalization. |
| `changelog.md` | Persists as an index of released or cancelled changes and evidence. | Release and cancellation finalization workflows. | Draft | Released entries use release version as the human-facing key and milestone ID as secondary trace metadata; cancelled-entry naming remains open. |
| `spec.json` | Represents lifecycle, active-change metadata, and current approvals. | Spec lifecycle workflows. | Draft | Must represent released / no-active-change without requiring `brief.md` or `tasks.md`. |

## Open questions

- What exact schema and ID format identify the active milestone, release binding, active change, and active requirement set?
- What exact evidence schema must the release skill require before finalization?
- How are superseded or removed product capabilities reflected in long-lived specs?
- How are cancelled changes finalized and indexed?
