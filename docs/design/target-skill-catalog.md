# Target skill catalog

This document is the working catalog for the SpecBind skill system we intend to build. It describes proposed names and responsibilities before they are implemented.

The catalog is intentionally separate from the [current generated skill index](../current-skill-index.md):

- The current index records what the CLI generates today.
- This catalog records ideas, drafts, accepted decisions, and implementation progress.

Related documents:

- [Target workflows](./target-workflows.md)
- [Target artifact catalog](./target-artifact-catalog.md)
- [Active spec lifecycle](./active-spec-lifecycle.md)
- [Spec state machine](./spec-state-machine.md)
- [CLI and agent responsibility boundary](./cli-agent-boundary.md)
- [Decision 0001: skill naming](./decisions/0001-skill-naming.md)
- [Decision 0002: project release adapter](./decisions/0002-project-release-adapter.md)
- [Decision 0003: active requirement set](./decisions/0003-active-requirement-set.md)
- [Decision 0004: release history layout](./decisions/0004-release-history-layout.md)
- [Decision 0005: active change abandonment](./decisions/0005-active-change-abandonment.md)
- [Decision 0009: milestone CLI boundary](./decisions/0009-milestone-cli-boundary.md)
- [Decision 0010: release execution boundary](./decisions/0010-release-execution-boundary.md)
- [Decision 0011: cross-spec contract manifest](./decisions/0011-cross-spec-contract.md)
- [Decision 0012: delegated approval](./decisions/0012-delegated-approval.md)
- [Decision 0013: structured task artifact](./decisions/0013-structured-task-artifact.md)
- [Decision 0014: structured spec metadata](./decisions/0014-structured-spec-metadata.md)

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
| `kiro-spec-batch` | `specbind-spec-batch` | Change | Draft | Generate several specs and perform contract-first cross-spec review. |
| `kiro-spec-design` | `specbind-spec-design` | Change | Draft | Maintain current design, active-requirement traceability, and the cross-spec contract. |
| `kiro-spec-init` | `specbind-spec-init` | Rename | Idea | Initialize a spec. |
| `kiro-spec-quick` | `specbind-spec-quick` | Rename | Idea | Run a shortened single-spec workflow. |
| `kiro-spec-requirements` | `specbind-spec-requirements` | Change | Draft | Maintain current requirements and freeze active Requirement IDs in `spec.yaml`. |
| `kiro-spec-status` | `specbind-spec-status` | Change | Draft | Explain released state, active change, current tasks, and history using the CLI status/read model rather than reparsing artifacts. |
| `kiro-spec-tasks` | `specbind-spec-tasks` | Change | Draft | Create a milestone-local plan covering the active requirement set. |
| `kiro-steering` | `specbind-steering` | Rename | Idea | Maintain core project guidance. |
| `kiro-steering-custom` | `specbind-steering-custom` | Rename | Idea | Create specialized project guidance. |
| `kiro-validate-design` | `specbind-validate-design` | Change | Draft | Review technical design quality and design-to-contract consistency. |
| `kiro-validate-gap` | `specbind-validate-gap` | Rename | Idea | Compare requirements with an existing codebase. |
| `kiro-validate-impl` | `specbind-validate-impl` | Change | Draft | Validate current milestone integration and active-requirement coverage. |
| `kiro-verify-completion` | `specbind-verify-completion` | Change | Draft | Verify current completion without confusing historical evidence. |
| None | `specbind-release` | New | Draft | Complete a release and close its active milestone. |
| None | `specbind-customize` | New | Idea | Post-v1 candidate for interactively customizing shared artifact templates and authoring rules. |

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

## `specbind-release`

Status: Draft

Current equivalent: None

### Purpose

Complete a release and close the active milestone represented by `roadmap.md`.

### Intended behavior

- Confirm that the active milestone is ready to close.
- Require every roadmap direct change to have sparse `status: completed` state.
- Read project-specific release instructions from `{{SPEC_DIR}}/settings/release.md`.
- Run the stateless `specbind release preflight` command and stop before adapter work unless it returns `OK RELEASE_READY`.
- Execute the adapter's Prepare, Publish, and Verify instructions as the AI agent.
- Prepare one delivered-change summary per participating spec and submit them to the Rust CLI for guarded log insertion and finalization.
- Append an idempotent history entry for every participating spec.
- Remove participating specs' active `brief.md` and `tasks.yaml` after successful release.
- Transition their metadata to released / no-active-change state.
- Archive `{{SPEC_DIR}}/steering/roadmap.md` as `{{SPEC_DIR}}/releases/<version>-roadmap.md`.
- Run optional After finalize project instructions only after core finalization succeeds.
- Preserve the active specs updated during the milestone.

### Inputs

- The active `roadmap.md`
- A concrete target release version
- `{{SPEC_DIR}}/settings/release.md`
- Existing lifecycle artifacts from which the CLI derives release readiness under Decision 0070

### Writes

- Per-spec date-grouped `log.md` entries
- Per-spec released / no-active-change metadata
- Removal of finalized `brief.md` and `tasks.yaml`
- Version-prefixed milestone roadmap under `{{SPEC_DIR}}/releases/`

### Boundaries

- Must not delete specs merely because the milestone is complete.
- Must stop before release operations when the target release version is unset.
- Must stop while any roadmap direct change remains pending.
- Must accept an empty adapter as no project-specific actions, while stopping when non-empty guidance is ambiguous or unsafe and never weakening core evidence requirements.
- Must not let adapter instructions weaken core readiness or finalization gates.
- Must not treat `settings/release.md` code blocks as CLI-executable hooks.
- Must not bypass CLI finalization through direct ad hoc artifact deletion or metadata edits.
- Must not remove active documents before applicable release work and required verification succeed.
- Must not treat a successful preflight as finalization authority or attempt to pass it back as a token; finalization rechecks current state independently.
- Must allow unrelated dirty files while refusing uncommitted or conflicting paths that CLI finalization will mutate.
- Build the strict Decision 0068 log-entry JSON outside the project or pass it on standard input.
- On `FINALIZE_TARGET_DIRTY`, must show the affected paths and obtain explicit user confirmation before retrying the same `specbind release finalize --log-entries ...` request with `--force`.
- Must not use `--force` to bypass archive collisions or any lifecycle, freshness, evidence, schema, or path-safety guard.
- Must not overwrite a conflicting roadmap archive.
- Must be idempotent when finalization is retried.
- Must not request or emulate finalization for only a subset of the active milestone's participating specs.
- On partial external success, must preserve the active milestone, report observed external state, and coordinate retry or reconciliation instead of claiming a SpecBind release or automatic rollback.
- Must report an After finalize failure as follow-up work without reopening the finalized milestone.
- Must consume concise English CLI results by default and translate or explain them in the user's language when useful; use `--json` only when structured details are needed.

## Cross-cutting questions

- Are `spec-*` names useful to users, or should names describe workflow outcomes more directly?
- Which validation and verification responsibilities should remain separate?
- Should the quick and batch workflows remain skills, or become orchestration modes of a smaller command set?
- How long, if at all, should old skill names remain available as compatibility aliases?
- Which skills become thinner once deterministic checks and lifecycle mutations move into the bundled CLI?
