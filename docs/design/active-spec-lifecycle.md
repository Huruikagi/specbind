# Active spec lifecycle

Status: Draft

This document develops the active-spec direction into a portable SpecBind contract. The target per-spec states, events, invalidation rules, and transition diagram are defined in [Spec state machine](./spec-state-machine.md). It is informed by [pc-build-planner Issue #50](https://github.com/Huruikagi/pc-build-planner/issues/50), where the current project-local workflow exposed the cost of mixing active milestone work with accumulated history.

The source project overrides the generated skills and adds repository-local skills. This document therefore separates reusable product requirements from that repository's current implementation.

## Problem

Long-lived specs need to remain the current description of the product, but milestone-specific working documents grow indefinitely when they also preserve delivery history:

- The active change and pending tasks become harder to find.
- Agents repeatedly load completed work that is irrelevant to the current milestone.
- Task numbering and resume logic span unrelated milestones.
- Release gates can confuse current evidence with historical completion.
- A discovery brief becomes both the current change input and an append-only decision log.

## Target document responsibilities

| Artifact | Responsibility | During an active change | After release |
| --- | --- | --- | --- |
| `brief.md` | Why the spec changes in the current milestone. | Exactly one active change, including same-milestone deltas. | Removed. |
| `requirements.md` | The complete set of currently valid requirements. | Revised in place. | Preserved. |
| `design.md` | The complete currently valid design. | Revised in place. | Preserved. |
| `contract.md` | The current minimal cross-spec seam manifest. | Revised only when externally observable seams change. | Preserved. |
| `tasks.md` | Executable plan for the current milestone's change. | Contains only current tasks and numbers them from the start. | Removed. |
| `changelog.md` | Per-spec index of released changes and their evidence. | Preserved; normally not the active authoring surface. | One concise entry appended. |
| `spec.json` | Current lifecycle, active-change metadata, and approvals. | Represents an active change. | Represents released state with no active change. |
| `roadmap.md` | Scope, dependencies, and evidence for the active milestone. | Exists under `steering/` and is maintained. | Moved to `releases/<version>-roadmap.md`. |

Absence of `brief.md` and `tasks.md` is the normal idle state of a released spec. Placeholder working documents should not be required.

## Milestone identity and release binding

Every milestone has a `roadmap.md`, including a milestone that changes only one existing spec or creates only one new spec. Discovery confirms the route and initiates the milestone before it hands work to later phases. To avoid growing discovery into a general state-management skill, Rust CLI milestone operations perform roadmap creation and later mechanical updates under [Decision 0009](./decisions/0009-milestone-cli-boundary.md). A missing roadmap means there is no active milestone.

The milestone's stable identity does not depend on knowing the final release version. The working model separates:

- `milestone_id`: opaque, machine-generated stable identity assigned when discovery starts the milestone
- `target_release`: concrete release version, initially unset when necessary
- `change_id`: stable per-spec change identity associated with `milestone_id`, not derived from `target_release`

Conceptual roadmap metadata:

```yaml
milestone_id: <generated-id>
target_release: null
```

When the release version becomes known, the workflow binds it to the active milestone:

```yaml
milestone_id: <generated-id>
target_release: v1.4.0
```

This is a metadata mapping, not textual replacement. Requirements, tasks, evidence, and Change IDs continue to refer to the stable milestone identity. Changing an unshipped target version updates the binding in one authoritative place instead of rewriting milestone artifacts. The generated ID is not intended to be selected or named by the user.

The exact generated-ID format remains an implementation choice. `specbind-release` requires a concrete `target_release` for every release and refuses to begin release operations while it is unset.

## Active change

Each spec has at most one active change at a time. The active brief should identify at least:

- stable milestone ID
- target release when assigned
- stable change ID
- problem and desired outcome
- in-scope and out-of-scope behavior
- boundary impact and dependencies
- source request or issue

Additional deltas discovered in the same milestone are merged into that active brief. A later milestone creates a new `brief.md`; it does not append the new change to the previous milestone's brief.

Discovery owns creating the active brief and transitioning an idle released spec into an active-change state. The precise routing and approval invalidation rules still need refinement.

## Active requirement set

`requirements.md` remains the complete current requirement set. Separately, the requirements phase must establish an explicit active requirement set for the milestone and store it in `spec.json`; see [Decision 0003](./decisions/0003-active-requirement-set.md):

- It contains every Requirement ID that must be implemented or revalidated for the active change.
- It may include unchanged existing requirements when the change requires their reimplementation or revalidation.
- It is not implicitly equal to every requirement in `requirements.md`.
- Requirements approval freezes the set for downstream phases.
- Changing the set returns the workflow to the requirements phase and invalidates affected downstream approval.
- Design traces the same set, and tasks must provide 100% coverage of it.

Within `spec.json`, `active_change.requirement_ids: null` means the set has not yet been established. Requirements approval replaces it with a unique, deterministically ordered array of canonical Requirement IDs. Release finalization clears `active_change` as part of returning the spec to released / idle state.

## Cross-spec contract

Every active spec maintains `contract.md` as the current source of truth for seams other specs may observe or consume. It persists across releases and changes alongside design; see [Cross-spec contracts](./cross-spec-contracts.md) and [Decision 0011](./decisions/0011-cross-spec-contract.md).

The contract contains only stable ownership, exports, consumes, cross-spec invariants, and File Ownership entries. Design review confirms that internal design implements the contract and that an active change has not omitted a required contract update. CLI checks validate structure and references; agent review determines semantic compatibility and downstream revalidation scope.

## Active tasks and coverage

`tasks.md` contains only tasks for the active milestone:

- Task numbering restarts for a new milestone.
- Same-milestone deltas may merge into the active task plan.
- Tasks from earlier milestones are never merge input for a new task plan.
- Every active Requirement ID maps to at least one executable task.
- One task may cover several requirements, and one requirement may map to several tasks.
- Missing coverage for any active Requirement ID prevents completion of the tasks phase.
- Tasks are not deleted until the release gate confirms zero incomplete or blocked tasks and valid completion evidence.

## Released history

`changelog.md` is a navigable index, not a snapshot of the entire spec. Released entries are organized and presented by release version; the machine-generated milestone ID remains secondary trace metadata. An entry should include enough information to locate and understand the historical change:

- release version
- milestone ID and change ID
- released status
- problem and delivered-scope summary
- preserved contracts or explicitly unchanged behavior
- completed-task count and active-requirement coverage
- validation result and timestamp
- immutable release reference, normally a tag
- relevant implementation, version, and finalization commits
- related roadmap, issue, or follow-up

The complete pre-finalization `brief.md` and `tasks.md` remain available from the immutable release reference. The roadmap also remains directly available under `releases/<version>-roadmap.md`. Git history is not the only index: each spec's `changelog.md` points to the relevant release and roadmap references.

## Scope removal, abandonment, and rollback

These operations are intentionally distinct from successful release finalization; see [Decision 0005](./decisions/0005-active-change-abandonment.md):

- Unstarted scope can be removed by revising the active milestone and its affected briefs.
- Partially implemented unreleased work must be restored with explicit project and Git operations. SpecBind then reconciles its active artifacts and metadata with that repository state; it does not perform an automatic revert.
- An entire unreleased milestone can be abandoned only with explicit user confirmation. After requirements and design have been restored or reconciled, lifecycle cleanup removes its milestone-local briefs and tasks, clears affected `active_change` state, and removes `steering/roadmap.md`.
- An abandoned unreleased milestone does not add per-spec changelog entries or a file under `releases/` by default. Committed work remains discoverable through Git history.
- A rollback of released behavior is represented as a new active change in a new milestone and is released normally.

Rust CLI milestone operations own the mechanical state transitions for scope changes and full abandonment. Discovery remains the user entry point, interprets intent, and obtains confirmation; the CLI validates and applies the deterministic transition. There is no separate `specbind-milestone` skill. See [CLI and agent responsibility boundary](./cli-agent-boundary.md).

## Release finalization contract

The portable release contract is a gated state transition. Project publication is supplied by the [project release adapter](./decisions/0002-project-release-adapter.md), while the lifecycle gates and SpecBind artifact finalization remain core behavior:

1. The release agent loads the active roadmap, target version, and `{{SPEC_DIR}}/settings/release.md`, then validates that required adapter phases are present.
2. The release agent asks the Rust CLI to run core preflight.
3. The CLI resolves participating specs, requires a concrete target version, and verifies current tasks, approvals, completion evidence, contract-impact/downstream-review evidence, and lifecycle consistency.
4. After successful preflight, the agent runs project Prepare, Publish, and Verify instructions in order and captures structured evidence.
5. The agent submits the target version, immutable reference, and evidence to the Rust CLI finalization boundary.
6. The CLI independently rechecks core invariants and confirms all publication evidence it can verify, including that the immutable reference retains the active working documents.
7. The CLI appends one version-keyed, idempotent release entry to each participating spec's `changelog.md`.
8. The CLI removes each participating spec's `brief.md` and `tasks.md`.
9. The CLI transitions each `spec.json` to released / no-active-change state.
10. The CLI moves `steering/roadmap.md` to `releases/<version>-roadmap.md`, refusing conflicting archive content.
11. The CLI persists finalization as one coherent state change and verifies the resulting idle state.
12. The agent runs optional project After finalize instructions and reports their result separately.

If publishing or release verification fails, finalization does not run and active documents remain intact. Re-running finalization must not duplicate changelog entries or remove unrelated work. An After finalize failure does not undo the release or core finalization. See [Decision 0010](./decisions/0010-release-execution-boundary.md).

## Lifecycle and dependency semantics

The authoritative per-spec workflow states and guarded transitions are defined in [Spec state machine](./spec-state-machine.md). In particular, an inconsistency is derived health over a declared workflow state rather than another freely writable lifecycle value.

The workflow must distinguish these states:

| State | Expected working files | Meaning |
| --- | --- | --- |
| Released and idle | No `brief.md` or `tasks.md` | The current requirements and design are implemented; no active milestone change exists. |
| Active change before task generation | `brief.md` exists; `tasks.md` may not yet exist | Spec revision is in progress. |
| Active implementation | `brief.md` and `tasks.md` exist | Current milestone tasks and approvals determine readiness. |
| Interrupted or inconsistent | Metadata says active but required phase artifacts are missing | Resume or repair is required. |

Dependencies must also distinguish:

- a dependency on the current approved spec contract
- a dependency on an active revision that must complete first
- a dependency on a released implementation, proven by changelog and immutable release evidence

A released spec without `tasks.md` must not be treated as unimplemented.

## Initial migration

Existing projects need a one-time cutover separate from normal release finalization:

1. Group earlier completed briefs and tasks by released change where evidence permits.
2. Backfill concise `changelog.md` entries with release and validation references.
3. Preserve only the current milestone's brief and tasks as the active working set.
4. Reconstruct and verify the current active requirement set.
5. Leave the current active documents in place until their normal release finalization.

Historical full documents remain authoritative at their existing tags or commits. Migration should not manufacture certainty when old evidence is incomplete.

## Generalization boundary

Issue #50 comes from a repository with local skill overrides. These details are useful evidence but are not automatically SpecBind-wide requirements:

| pc-build-planner detail | SpecBind treatment |
| --- | --- |
| `kiro-spec-update-batch` as a separate skill | Existing-spec batch orchestration is required; whether it is a separate skill remains open. |
| `kiro-record-validation` and a roadmap validation table | Stable completion evidence is required; its storage location and skill boundary remain open. |
| `kiro-impl-direct` and structured direct candidates | Direct-work support is a separate workflow decision, not required by the active-spec lifecycle itself. |
| GitHub Issues, GitHub milestones, Actions, ZIP packaging, and project version scripts | Repository-specific instructions in the release adapter, not hard-coded core behavior. |
| Direct commits to `main` and exact commit messages | Repository policy, not a universal SpecBind requirement. |
| v0.5.1 cutover sequence | Concrete example of the general one-time migration pattern. |

The project-local append-only Change Brief behavior is the observed problem, not a behavior to preserve in the target contract.

## Skills affected by the portable contract

- `specbind-discovery`: analyze and route work, create one active brief, and initiate confirmed milestone changes.
- bundled CLI: create and update the active roadmap, bind the target release, perform confirmed abandonment cleanup, and check active Requirement ID traceability.
- requirements workflow: revise current requirements and freeze the active requirement set.
- design workflow: revise current design, trace the active requirement set, and maintain the cross-spec contract.
- tasks workflow: generate a milestone-local plan with complete active-requirement coverage.
- implementation and validation workflows: operate only on current tasks and current milestone evidence.
- status workflow: report released state, active-change state, current tasks, and latest history separately.
- completion verification: distinguish current coverage from historical release evidence.
- cross-spec review: read contracts first and deepen into affected specs only when boundaries change or remain ambiguous.
- `specbind-release`: perform gated, idempotent finalization after release success.

Batch update and evidence-recording responsibilities are required, but their final skill boundaries are not yet decided.

## Open questions

- The generated milestone ID format and the authoritative location of the release-version binding.
- Whether rebinding a target release requires explicit approval after implementation has started.
- Whether one milestone can contain multiple active Change IDs for the same spec.
- The exact `changelog.md` schema and evidence granularity.
- Whether projects need an opt-in audit record for abandoned, unreleased milestones.
- The exact approval-evidence, provenance, and artifact-fingerprint schema used by the target `spec.json` state model.
- Whether immutable history may use something other than a Git release tag.
