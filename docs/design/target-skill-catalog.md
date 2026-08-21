# Target skill catalog

This document records the design history and implementation status of the
SpecBind skill system. Decision 0075's complete v1 set is now embedded and
installed; the current concise interface is indexed in the
[current generated skill index](../current-skill-index.md).

The catalog is intentionally separate from the [current generated skill index](../current-skill-index.md):

- The current index records what the CLI generates today.
- This catalog records the inherited mapping, accepted changes, and implementation progress.

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
- [Decision 0075: v1 skill and orchestration scope](./decisions/0075-v1-skill-and-orchestration-scope.md)
- [Decision 0092: template and skill authoring boundary](./decisions/0092-template-skill-authoring-boundary.md)
- [Decision 0093: default shared-rule set](./decisions/0093-default-shared-rule-set.md)
- [Decision 0094: embedded product protocols](./decisions/0094-embedded-product-protocols.md)
- [Decision 0101: project adapter directory and Git workflow](./decisions/0101-project-adapter-directory-and-git-workflow.md)
- [Decision 0137: active default Git checkpoints](./decisions/0137-active-default-git-checkpoints.md)

Skills that may create Git checkpoints or push read the project-owned
`settings/adapters/git.md` contract when present. The active default chooses one
local commit per eligible workflow unit. A request for that mutating phase
authorizes only this narrow checkpoint; the adapter neither makes unaccepted
work eligible nor grants push, branch, or history-rewriting authority.

## Status and change types

| Field | Values |
| --- | --- |
| Status | `Idea`, `Draft`, `Accepted`, `Implemented` |
| Change | `Keep`, `Rename`, `Change`, `Merge`, `Split`, `Remove`, `New` |

`Implemented` means the source, both agent templates, tests, and maintained documentation have been updated. It does not merely mean that the design was accepted.

## Working catalog

Decision 0075 accepts the v1 public skill set below. Compatibility aliases are not shipped.

| Current skill | Target working name | Change | Status | Current responsibility |
| --- | --- | --- | --- | --- |
| `kiro-debug` | `specbind-debug` | Change | Implemented | Perform read-only fresh-context root-cause analysis and return a bounded next action. |
| `kiro-discovery` | `specbind-discovery` | Change | Implemented | Analyze requests, classify Roadmap items, confirm scope, and invoke guarded milestone initialization or update. |
| `kiro-impl` | `specbind-implement` | Change | Implemented | Implement one Spec-backed or Direct Roadmap item. |
| `kiro-review` | `specbind-review-task` | Rename | Implemented | Review one task implementation using the actual diff and approved inputs. |
| `kiro-spec-batch` | `specbind-batch-plan` | Change | Implemented | Bring all Spec-backed milestone items through Tasks approval without implementation. |
| `kiro-spec-design` | `specbind-design` | Change | Implemented | Maintain current design, active-requirement traceability, and the cross-spec contract. |
| `kiro-spec-init` | None | Remove | Implemented | Initialization is a deterministic Rust CLI operation invoked by discovery. |
| `kiro-spec-quick` | `specbind-quick-plan` | Change | Implemented | Bring one Spec-backed item through Tasks approval using delegated gates. |
| `kiro-spec-requirements` | `specbind-requirements` | Change | Implemented | Maintain current requirements and freeze active Requirement IDs in `spec.yaml`. |
| `kiro-spec-status` | `specbind-status` | Change | Implemented | Route no-argument requests to current milestone status, explicit Spec requests to per-Spec status, and task questions to task read models; explain history only from separate authoritative history reads. |
| `kiro-spec-tasks` | `specbind-tasks` | Change | Implemented | Create a milestone-local plan covering the active requirement set after contract review. |
| `kiro-steering` and `kiro-steering-custom` | `specbind-steering` | Merge | Implemented | Bootstrap, synchronize, or add project guidance identified by OKF type and `artifact_id`. |
| `kiro-validate-design` | `specbind-validate-design` | Change | Implemented | Review technical design quality and design-to-contract consistency. |
| `kiro-validate-gap` | `specbind-gap-analysis` | Change | Implemented | Compare a brownfield codebase with intended requirements and persist current milestone research when useful. |
| `kiro-validate-impl` | `specbind-validate-implementation` | Change | Implemented | Validate one Spec's complete implementation and active-requirement coverage through the Decision 0086 preflight, transient evidence, acceptance, and invalidation contract. |
| `kiro-verify-completion` | `specbind-verify-completion` | Change | Implemented | Apply the mandatory completion-verification protocol without becoming a workflow stage. |
| None | `specbind-contract-review` | New | Implemented | Review the complete current contract graph after Design approval and before Tasks authoring. |
| None | `specbind-release` | New | Implemented | Complete a release and close its active milestone. |

This classification now records the implemented v1 migration from the inherited
set. Future rows should use `Rename` only when responsibility is unchanged;
otherwise use `Change`, `Merge`, `Split`, `Remove`, or `New`.

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

Status: Implemented

Current equivalent: None

### Purpose

Complete a release and close the active milestone represented by `roadmap.md`.

### Intended behavior

- Confirm that the active milestone is ready to close.
- Require every roadmap direct change to have sparse `status: completed` state.
- Read project-specific release instructions from `{{SPEC_DIR}}/settings/adapters/release.md`.
- Run the stateless `specbind release preflight` command and stop before adapter work unless it returns `OK RELEASE_READY`.
- Execute the adapter's Prepare, Publish, and Verify instructions as the AI agent.
- For a Spec-backed milestone, prepare one delivered-change summary per participating spec and submit them to the Rust CLI for guarded log insertion and finalization.
- Append an idempotent history entry for every participating spec when Spec-backed work exists.
- Remove participating specs' active Brief, optional Research, and `tasks.yaml` after successful release.
- Transition their metadata to released / no-active-change state.
- Archive `{{SPEC_DIR}}/steering/roadmap.md` as `{{SPEC_DIR}}/releases/<version>-roadmap.md`.
- Run optional After finalize project instructions only after core finalization succeeds.
- Preserve the active specs updated during the milestone.

### Inputs

- The active `roadmap.md`
- A concrete target release version
- `{{SPEC_DIR}}/settings/adapters/release.md`
- Existing lifecycle artifacts from which the CLI derives release readiness under Decision 0070

### Writes

- Per-spec date-grouped `log.md` entries for Spec-backed milestones
- Per-spec released / no-active-change metadata
- Removal of finalized Brief, optional Research, and `tasks.yaml`
- Version-prefixed milestone roadmap under `{{SPEC_DIR}}/releases/`

### Boundaries

- Must not delete specs merely because the milestone is complete.
- Must stop before release operations when the target release version is unset.
- Must stop while any roadmap direct change remains pending.
- Must accept an empty adapter as no project-specific actions, while stopping when non-empty guidance is ambiguous or unsafe and never weakening core evidence requirements.
- Must not let adapter instructions weaken core readiness or finalization gates.
- Must not treat `settings/adapters/release.md` code blocks as CLI-executable hooks.
- Must not bypass CLI finalization through direct ad hoc artifact deletion or metadata edits.
- Must not remove active documents before applicable release work and required verification succeed.
- Must not treat a successful preflight as finalization authority or attempt to pass it back as a token; finalization rechecks current state independently.
- Must allow unrelated dirty files while refusing uncommitted or conflicting paths that CLI finalization will mutate.
- Build the strict Decision 0068 log-entry JSON outside the project or pass it on standard input.
- On `FINALIZE_TARGET_DIRTY`, must show the affected paths and stop until they are committed, stashed, or otherwise made clean.
- Must not offer or emulate a release-finalization `--force` bypass.
- Must not overwrite a conflicting roadmap archive.
- Must be idempotent when finalization is retried.
- Must not request or emulate finalization for only a subset of the active milestone's participating specs.
- On partial external success, must preserve the active milestone, report observed external state, and coordinate retry or reconciliation instead of claiming a SpecBind release or automatic rollback.
- Must report an After finalize failure as follow-up work without reopening the finalized milestone.
- Must consume concise English CLI results and stable codes, then translate or explain them in the user's language when useful. V1 has no JSON output mode under Decision 0074.

## Post-v1 candidates

- A milestone-wide implementation orchestrator with dependency-wave and
  subagent coordination, tracked by
  [Issue #9](https://github.com/Huruikagi/specbind/issues/9).
- A dedicated customization convenience skill over the stable shared settings
  surface, tracked by
  [Issue #10](https://github.com/Huruikagi/specbind/issues/10).
- Agent removal and uninstall workflows, tracked by
  [Issue #11](https://github.com/Huruikagi/specbind/issues/11).
