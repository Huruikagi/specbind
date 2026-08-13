# Target artifact catalog

This document is the working catalog for the files the future SpecBind workflow will install, create, maintain, or remove. It describes proposed artifact lifecycles before they are implemented.

The catalog is intentionally separate from the [current generated artifact index](../current-artifact-index.md):

- The current index records what the CLI and skills produce today.
- This catalog records intended ownership and lifecycle changes.

Related documents:

- [Target skill catalog](./target-skill-catalog.md)
- [Target workflows](./target-workflows.md)

Status: Draft

## Lifecycle principles

- Specs are active specifications of the product, not disposable delivery plans.
- Existing specs should be maintained as the represented behavior changes.
- A milestone groups the work intended for an active release cycle.
- At most one active milestone is represented by `roadmap.md`.
- Release completion ends the active milestone but does not retire the specs it changed.

## Project-level artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/steering/roadmap.md` | Exists only while a milestone is active. | Created and maintained by `specbind-discovery`; removed by `specbind-release`. | Draft | `{{SPEC_DIR}}` is a placeholder until the root directory decision is made. |

## Spec artifacts

| Target path | Lifecycle | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| `{{SPEC_DIR}}/specs/<feature>/` | Persists across milestones and releases while the represented capability remains active. | Spec authoring and maintenance skills. | Draft | A release must not delete a spec merely because its milestone completed. |
| `requirements.md` | Updated when user-visible behavior or constraints change. | Requirements workflow. | Draft | Existing-spec update behavior will be refined separately. |
| `design.md` | Updated when the active technical design changes. | Design workflow. | Draft | Historical design preservation is not yet defined. |
| `tasks.md` | Supports implementation progress for spec changes. | Task and implementation workflows. | Draft | Reset, replacement, or archival behavior between milestones is not yet defined. |

## Open questions

- What metadata identifies the active milestone and its intended release?
- Does discovery create `roadmap.md` for every milestone, including a single-spec milestone?
- What evidence must the release skill require before removing `roadmap.md`?
- Is milestone or release history stored anywhere before `roadmap.md` is deleted?
- How are superseded or removed product capabilities reflected in long-lived specs?
- Should task history remain in an active spec or move to a release-history artifact?
