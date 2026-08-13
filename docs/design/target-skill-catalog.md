# Target skill catalog

This document is the working catalog for the SpecBind skill system we intend to build. It describes proposed names and responsibilities before they are implemented.

The catalog is intentionally separate from the [current generated skill index](../current-skill-index.md):

- The current index records what the CLI generates today.
- This catalog records ideas, drafts, accepted decisions, and implementation progress.

Related documents:

- [Target workflows](./target-workflows.md)
- [Target artifact catalog](./target-artifact-catalog.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)
- [CLI and agent responsibility boundary](./cli-agent-boundary.md)
- [Decision 0001: skill naming](./decisions/0001-skill-naming.md)
- [Decision 0002: project release adapter](./decisions/0002-project-release-adapter.md)
- [Decision 0003: active requirement set](./decisions/0003-active-requirement-set.md)
- [Decision 0004: release history layout](./decisions/0004-release-history-layout.md)
- [Decision 0005: active change abandonment](./decisions/0005-active-change-abandonment.md)

## Status and change types

| Field | Values |
| --- | --- |
| Status | `Idea`, `Draft`, `Accepted`, `Implemented` |
| Change | `Keep`, `Rename`, `Change`, `Merge`, `Split`, `Remove`, `New` |

`Implemented` means the source, both agent templates, tests, and maintained documentation have been updated. It does not merely mean that the design was accepted.

## Working catalog

The inherited `kiro-` prefix will be replaced with `specbind-`. This prefix decision is accepted, while each skill's final responsibility and suffix remain subject to review.

| Current skill | Target working name | Change | Status | Current responsibility |
| --- | --- | --- | --- | --- |
| `kiro-debug` | `specbind-debug` | Rename | Idea | Investigate implementation and verification failures. |
| `kiro-discovery` | `specbind-discovery` | Change | Draft | Analyze requests, classify work, and route active spec maintenance. |
| `kiro-impl` | `specbind-impl` | Change | Draft | Implement only the active milestone's approved tasks. |
| `kiro-review` | `specbind-review` | Rename | Idea | Review one task implementation. |
| `kiro-spec-batch` | `specbind-spec-batch` | Rename | Idea | Generate several specs from a roadmap. |
| `kiro-spec-design` | `specbind-spec-design` | Change | Draft | Maintain current design and trace the active requirement set. |
| `kiro-spec-init` | `specbind-spec-init` | Rename | Idea | Initialize a spec. |
| `kiro-spec-quick` | `specbind-spec-quick` | Rename | Idea | Run a shortened single-spec workflow. |
| `kiro-spec-requirements` | `specbind-spec-requirements` | Change | Draft | Maintain current requirements and freeze active Requirement IDs in `spec.json`. |
| `kiro-spec-status` | `specbind-spec-status` | Change | Draft | Distinguish released state, active change, current tasks, and history. |
| `kiro-spec-tasks` | `specbind-spec-tasks` | Change | Draft | Create a milestone-local plan covering the active requirement set. |
| `kiro-steering` | `specbind-steering` | Rename | Idea | Maintain core project guidance. |
| `kiro-steering-custom` | `specbind-steering-custom` | Rename | Idea | Create specialized project guidance. |
| `kiro-validate-design` | `specbind-validate-design` | Rename | Idea | Review technical design quality. |
| `kiro-validate-gap` | `specbind-validate-gap` | Rename | Idea | Compare requirements with an existing codebase. |
| `kiro-validate-impl` | `specbind-validate-impl` | Change | Draft | Validate current milestone integration and active-requirement coverage. |
| `kiro-verify-completion` | `specbind-verify-completion` | Change | Draft | Verify current completion without confusing historical evidence. |
| None | `specbind-milestone` | New | Draft | Manage active milestone state without absorbing discovery analysis. |
| None | `specbind-release` | New | Draft | Complete a release and close its active milestone. |

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

## `specbind-milestone`

Status: Draft

Current equivalent: None

### Purpose

Own the mechanical lifecycle of the single active milestone while keeping discovery focused on understanding and routing work.

From the user's perspective, discovery still starts a milestone. Internally, it delegates persistent milestone-state changes to this responsibility.

### Intended behavior

- Create `steering/roadmap.md` with a machine-generated stable milestone ID.
- Apply confirmed scope and ordering updates supplied by discovery.
- Bind or rebind the target release version before release begins.
- Perform explicit, user-confirmed abandonment cleanup after repository and spec content is reconciled.

### Inputs

- Confirmed discovery output or an explicit milestone-state request
- Current roadmap and participating-spec state
- A concrete target version when binding a release

### Writes

- `{{SPEC_DIR}}/steering/roadmap.md`
- Participating specs' active-change lifecycle metadata during confirmed abandonment
- Removal of milestone-local artifacts during confirmed abandonment

### Boundaries

- Must not decide whether a request needs an existing spec, a new spec, or direct work.
- Must not author detailed requirements or design.
- Must not automatically revert code, Git history, requirements, or design.
- Must not archive an unreleased abandoned roadmap under `releases/`.
- Must not perform publication or successful-release finalization.

### Open questions

- Whether users invoke `specbind-milestone` directly or it remains an internal responsibility used by discovery and release.
- Whether release-version binding belongs here permanently or becomes a release preflight operation.
- Whether deterministic CLI milestone commands make this separate agent skill unnecessary.

## `specbind-release`

Status: Draft

Current equivalent: None

### Purpose

Complete a release and close the active milestone represented by `roadmap.md`.

### Intended behavior

- Confirm that the active milestone is ready to close.
- Read project-specific release instructions from `{{SPEC_DIR}}/settings/release.md`.
- Run the adapter's Prepare, Publish, and Verify phases around the non-overridable core gates.
- Verify an immutable release reference that preserves the active working documents.
- Append an idempotent history entry for every participating spec.
- Remove participating specs' active `brief.md` and `tasks.md` after successful release.
- Transition their metadata to released / no-active-change state.
- Archive `{{SPEC_DIR}}/steering/roadmap.md` as `{{SPEC_DIR}}/releases/<version>-roadmap.md`.
- Run optional After finalize project instructions only after core finalization succeeds.
- Preserve the active specs updated during the milestone.

### Inputs

- The active `roadmap.md`
- A concrete target release version
- `{{SPEC_DIR}}/settings/release.md`
- Release-readiness state and evidence, to be defined

### Writes

- Per-spec `changelog.md` entries
- Per-spec released / no-active-change metadata
- Removal of finalized `brief.md` and `tasks.md`
- Version-prefixed milestone roadmap under `{{SPEC_DIR}}/releases/`

### Boundaries

- Must not delete specs merely because the milestone is complete.
- Must stop before release operations when the target release version is unset.
- Must stop when the adapter lacks safe Publish or Verify instructions.
- Must not let adapter instructions weaken core readiness or finalization gates.
- Must not remove active documents before the release succeeds and an immutable reference is verified.
- Must not overwrite a conflicting roadmap archive.
- Must be idempotent when finalization is retried.

### Open questions

- What exact adapter schema and validation rules should be used?
- What exact release-readiness evidence schema is mandatory?
- What happens when release succeeds only partially?

## Cross-cutting questions

- Are `spec-*` names useful to users, or should names describe workflow outcomes more directly?
- Which validation and verification responsibilities should remain separate?
- Should the quick and batch workflows remain skills, or become orchestration modes of a smaller command set?
- How long, if at all, should old skill names remain available as compatibility aliases?
- Which skills become thinner once deterministic checks and lifecycle mutations move into the bundled CLI?
