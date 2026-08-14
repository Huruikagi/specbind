# 0057: Discover spec Markdown artifacts by OKF type

Status: Accepted

## Context

Decision 0045 makes managed Markdown files OKF concept documents with a machine-readable `type`, but earlier lifecycle and gate decisions still identify spec artifacts by fixed filenames. That prevents a design from being split into focused documents and makes a harmless file rename look like a different artifact.

Type-based discovery can preserve human-friendly Markdown organization while giving the CLI a deterministic artifact set. It needs explicit multiplicity, stable collection identities, bounded traversal, and logical evidence keys so that discovery does not become an ambiguous directory search.

## Decision

- The CLI discovers managed spec-local Markdown recursively below `{{SPEC_DIR}}/specs/<canonical-spec>/` by parsing OKF frontmatter and matching the exact `type` value.
- Discovery considers regular `.md` files only and does not follow symbolic links. The OKF-reserved `log.md` is handled separately and has no `type`. The machine artifacts `spec.yaml` and `tasks.yaml` retain their fixed names and are not part of OKF discovery.
- Every non-reserved Markdown file in the discovery scope must be a valid OKF concept under Decision 0045. Invalid frontmatter is reported rather than silently skipped. Unknown valid OKF types are preserved but do not satisfy a SpecBind lifecycle role.
- The v1 spec-local profiles and multiplicities are:

  | `type` | Multiplicity | Persistent identity |
  | --- | --- | --- |
  | `SpecBind Brief` | zero or one while a change is active | singleton role `brief` |
  | `SpecBind Requirements` | exactly one for an established spec | singleton role `requirements` |
  | `SpecBind Contract` | exactly one for an established spec | singleton role `contract` |
  | `SpecBind Design` | one or more from design approval onward | collection role `design/<artifact_id>` |
  | `SpecBind Implementation Notes` | zero or more | collection role `implementation-notes/<artifact_id>` |

- Collection profiles require an `artifact_id` frontmatter field matching `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$`. The pair of exact `type` and `artifact_id` is unique within a spec. Singleton profiles omit `artifact_id`; their role supplies the identity.
- A collection artifact keeps its `artifact_id` when its filename, containing directory, or prose title changes but its semantic responsibility remains the same. Splitting a document keeps the old ID on the continuing responsibility and assigns new IDs to newly separated responsibilities.
- Paths are current locators and diagnostics, not persistent identity. Default templates retain the familiar names `brief.md`, `requirements.md`, `design.md`, `contract.md`, and `implementation-notes.md`; a default single design and implementation-notes artifact use `artifact_id: main`.
- A workflow uses the Decision 0058 `specbind artifact list` read model before loading content. The inventory contains logical role, current SpecBind-root-relative POSIX path, and profile metadata. Agent workflows begin from that compact inventory and request applicable selectors through `specbind artifact read` rather than independently guessing filenames or loading every Markdown file.
- Duplicate singleton roles, duplicate collection IDs, missing required roles, invalid profiles, and paths that escape the spec directory are hard discovery errors for workflows that require the affected role.

## Fingerprint identity

- Markdown fingerprints still cover the complete file, including frontmatter, after line-ending normalization under Decisions 0018 and 0045.
- Gate evidence uses logical artifact keys rather than current paths:
  - requirements gate: `requirements`
  - design gate: `contract` and every discovered `design/<artifact_id>`
- The complete key set belongs to freshness. Adding or removing a design artifact invalidates design approval; renaming or moving a file without changing its bytes or logical identity does not.
- Project-level cross-spec review uses spec-qualified logical selectors:
  - `specs/<canonical-spec>#contract`
  - `specs/<canonical-spec>#requirements`
  - `specs/<canonical-spec>#design/<artifact_id>`
- Fixed machine projections retain path-based keys where the path is itself canonical, including `tasks.yaml#plan` and `steering/roadmap.md#cross-spec-scope`.
- The agent submits logical selectors, not trusted paths or hashes. The CLI resolves each selector against the current typed inventory, reads the artifact, computes the fingerprint, and persists the evidence atomically.

## Lifecycle and migration

- Release finalization removes the discovered `SpecBind Brief` artifact and fixed `tasks.yaml`; it does not assume the brief filename.
- Persistent requirements, design, contract, and implementation-notes artifacts survive release regardless of their current paths.
- Existing canonical files migrate by adding the appropriate exact `type`. Existing `design.md` and `implementation-notes.md` receive `artifact_id: main` unless migration identifies multiple semantic documents that need distinct stable IDs.
- Contract discovery applies independently at the Decision 0054 baseline and current revision. A contract rename with unchanged logical role and content therefore remains the same contract across the diff.

## Consequences

- A spec can split design into files such as `architecture.md`, `persistence.md`, and `error-handling.md` without adding another manifest.
- Human organization and machine identity are decoupled, while duplicate or missing authoritative artifacts remain deterministic errors.
- The Decision 0058 CLI read model presents a small typed artifact index before an agent spends context on document bodies.
- Existing path-keyed gate evidence and accepted cross-spec review state require explicit migration or regeneration; logical-key and path-key representations are not mixed in one evidence record.
