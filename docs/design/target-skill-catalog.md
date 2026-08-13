# Target skill catalog

This document is the working catalog for the SpecBind skill system we intend to build. It describes proposed names and responsibilities before they are implemented.

The catalog is intentionally separate from the [current generated skill index](../current-skill-index.md):

- The current index records what the CLI generates today.
- This catalog records ideas, drafts, accepted decisions, and implementation progress.

Related documents:

- [Target workflows](./target-workflows.md)
- [Decision 0001: skill naming](./decisions/0001-skill-naming.md)

## Status and change types

| Field | Values |
| --- | --- |
| Status | `Idea`, `Draft`, `Accepted`, `Implemented` |
| Change | `Keep`, `Rename`, `Change`, `Merge`, `Split`, `Remove`, `New` |

`Implemented` means the source, both agent templates, tests, and maintained documentation have been updated. It does not merely mean that the design was accepted.

## Working catalog

The inherited `kiro-` prefix will be replaced. The new prefix is deliberately shown as `<prefix>` until the naming decision is accepted.

| Current skill | Target working name | Change | Status | Current responsibility |
| --- | --- | --- | --- | --- |
| `kiro-debug` | `<prefix>-debug` | Rename | Idea | Investigate implementation and verification failures. |
| `kiro-discovery` | `<prefix>-discovery` | Rename | Idea | Classify and decompose new work. |
| `kiro-impl` | `<prefix>-impl` | Rename | Idea | Implement approved tasks with TDD and subagents. |
| `kiro-review` | `<prefix>-review` | Rename | Idea | Review one task implementation. |
| `kiro-spec-batch` | `<prefix>-spec-batch` | Rename | Idea | Generate several specs from a roadmap. |
| `kiro-spec-design` | `<prefix>-spec-design` | Rename | Idea | Create a technical design. |
| `kiro-spec-init` | `<prefix>-spec-init` | Rename | Idea | Initialize a spec. |
| `kiro-spec-quick` | `<prefix>-spec-quick` | Rename | Idea | Run a shortened single-spec workflow. |
| `kiro-spec-requirements` | `<prefix>-spec-requirements` | Rename | Idea | Create requirements. |
| `kiro-spec-status` | `<prefix>-spec-status` | Rename | Idea | Report spec status and progress. |
| `kiro-spec-tasks` | `<prefix>-spec-tasks` | Rename | Idea | Create implementation tasks. |
| `kiro-steering` | `<prefix>-steering` | Rename | Idea | Maintain core project guidance. |
| `kiro-steering-custom` | `<prefix>-steering-custom` | Rename | Idea | Create specialized project guidance. |
| `kiro-validate-design` | `<prefix>-validate-design` | Rename | Idea | Review technical design quality. |
| `kiro-validate-gap` | `<prefix>-validate-gap` | Rename | Idea | Compare requirements with an existing codebase. |
| `kiro-validate-impl` | `<prefix>-validate-impl` | Rename | Idea | Validate feature-level integration and spec coverage. |
| `kiro-verify-completion` | `<prefix>-verify-completion` | Rename | Idea | Verify completion claims with fresh evidence. |

This initial classification records only the known naming direction. Change a row from `Rename` when its responsibility is intentionally changed, merged, split, or removed.

## Skill definition format

Add a detail section only when a skill needs more than a rename. Keep it at contract level until implementation begins.

```md
## `<target-skill-name>`

Status: Draft
Current equivalent: `<current-skill-name>` or None

### Purpose

One responsibility stated from the user's perspective.

### Intended changes

- Difference from the current behavior

### Inputs

- Required user input or repository state

### Writes

- Files or external state created or updated

### Boundaries

- Work this skill must not absorb

### Open questions

- Unresolved design choice
```

For a new skill, add a `New` row to the working catalog and set `Current equivalent` to `None`. For a merge or split, name every affected current skill so migration work remains visible.

## Cross-cutting questions

- What prefix should replace `kiro-`?
- Are `spec-*` names useful to users, or should names describe workflow outcomes more directly?
- Which validation and verification responsibilities should remain separate?
- Should the quick and batch workflows remain skills, or become orchestration modes of a smaller command set?
- How long, if at all, should old skill names remain available as compatibility aliases?
