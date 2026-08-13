# Active spec lifecycle

Status: Draft

This document develops the active-spec direction into a portable SpecBind contract. It is informed by [pc-build-planner Issue #50](https://github.com/Huruikagi/pc-build-planner/issues/50), where the current project-local workflow exposed the cost of mixing active milestone work with accumulated history.

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
| `tasks.md` | Executable plan for the current milestone's change. | Contains only current tasks and numbers them from the start. | Removed. |
| `changelog.md` | Index of released or cancelled changes and their evidence. | Preserved; normally not the active authoring surface. | One concise entry appended. |
| `spec.json` | Current lifecycle, active-change metadata, and approvals. | Represents an active change. | Represents released state with no active change. |
| `roadmap.md` | Scope, dependencies, and evidence for the active milestone. | Exists and is maintained. | Removed by successful release finalization. |

Absence of `brief.md` and `tasks.md` is the normal idle state of a released spec. Placeholder working documents should not be required.

## Milestone identity and release binding

Every milestone has a `roadmap.md`, including a milestone that changes only one existing spec or creates only one new spec. Discovery creates the roadmap before it hands work to later phases. A missing roadmap means there is no active milestone.

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

`requirements.md` remains the complete current requirement set. Separately, the requirements phase must establish an explicit active requirement set for the milestone:

- It contains every Requirement ID that must be implemented or revalidated for the active change.
- It may include unchanged existing requirements when the change requires their reimplementation or revalidation.
- It is not implicitly equal to every requirement in `requirements.md`.
- Requirements approval freezes the set for downstream phases.
- Changing the set returns the workflow to the requirements phase and invalidates affected downstream approval.
- Design traces the same set, and tasks must provide 100% coverage of it.

The machine-readable storage location for this set remains undecided.

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
- status such as released or cancelled
- problem and delivered-scope summary
- preserved contracts or explicitly unchanged behavior
- completed-task count and active-requirement coverage
- validation result and timestamp
- immutable release reference, normally a tag
- relevant implementation, version, and finalization commits
- related roadmap, issue, or follow-up

The complete pre-finalization `brief.md`, `tasks.md`, and `roadmap.md` remain available from the immutable release reference. Git history is not the only index: `changelog.md` points to the relevant references.

## Release finalization contract

The portable release contract is a gated state transition, independent of any repository's packaging or publishing mechanism:

1. Resolve the active milestone and participating specs.
2. Require a concrete target release version; stop before release operations when it is unset.
3. Verify zero incomplete or blocked current tasks and clean completion evidence.
4. Produce and verify an immutable release reference that still contains the active working documents.
5. Append one version-keyed, idempotent release entry to each participating spec's `changelog.md`.
6. Remove each participating spec's `brief.md` and `tasks.md`.
7. Transition each `spec.json` to released / no-active-change state.
8. Remove `roadmap.md` only when it contains no scope for another milestone.
9. Persist finalization as one coherent state change and verify the resulting idle state.

If publishing or release verification fails, finalization does not run and active documents remain intact. Re-running finalization must not duplicate changelog entries or remove unrelated work.

## Lifecycle and dependency semantics

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
| GitHub Issues, GitHub milestones, Actions, ZIP packaging, and project version scripts | Repository-specific release adapter or guidance, not the portable release contract. |
| Direct commits to `main` and exact commit messages | Repository policy, not a universal SpecBind requirement. |
| v0.5.1 cutover sequence | Concrete example of the general one-time migration pattern. |

The project-local append-only Change Brief behavior is the observed problem, not a behavior to preserve in the target contract.

## Skills affected by the portable contract

- `specbind-discovery`: create one active brief, route existing-spec changes, and initialize active-change state.
- requirements workflow: revise current requirements and freeze the active requirement set.
- design workflow: revise current design and trace the active requirement set.
- tasks workflow: generate a milestone-local plan with complete active-requirement coverage.
- implementation and validation workflows: operate only on current tasks and current milestone evidence.
- status workflow: report released state, active-change state, current tasks, and latest history separately.
- completion verification: distinguish current coverage from historical release evidence.
- `specbind-release`: perform gated, idempotent finalization after release success.

Batch update and evidence-recording responsibilities are required, but their final skill boundaries are not yet decided.

## Open questions

- Where the active requirement set is stored and how its schema is validated.
- The generated milestone ID format and the authoritative location of the release-version binding.
- Whether rebinding a target release requires explicit approval after implementation has started.
- Whether one milestone can contain multiple active Change IDs for the same spec.
- The exact `changelog.md` schema and evidence granularity.
- Which workflow finalizes cancelled changes.
- The target `spec.json` state model and migration compatibility.
- How partial finalization works when a roadmap contains more than one intended release.
- Whether immutable history may use something other than a Git release tag.
