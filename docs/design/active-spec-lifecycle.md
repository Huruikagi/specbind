# Active spec lifecycle

Status: Draft

This document develops the active-spec direction into a portable SpecBind contract. The target per-spec states, events, invalidation rules, and transition diagram are defined in [Spec state machine](./spec-state-machine.md); aggregate phase, dependency waves, and release readiness are defined in [Milestone state machine](./milestone-state-machine.md). [Decision 0012](./decisions/0012-delegated-approval.md) defines explicit and delegated gate approval. It is informed by [pc-build-planner Issue #50](https://github.com/Huruikagi/pc-build-planner/issues/50), where the current project-local workflow exposed the cost of mixing active milestone work with accumulated history.

The source project overrides the generated skills and adds repository-local skills. This document therefore separates reusable product requirements from that repository's current implementation.

## Problem

Long-lived specs need to remain the current description of the product, but milestone-specific working documents grow indefinitely when they also preserve delivery history:

- The active change and pending tasks become harder to find.
- Agents repeatedly load completed work that is irrelevant to the current milestone.
- Task numbering and resume logic span unrelated milestones.
- Release gates can confuse current evidence with historical completion.
- A discovery brief can accumulate unrelated historical detail when it is treated as both the current change input and an append-only decision log.

## Target document responsibilities

| Artifact | Responsibility | During an active change | After release |
| --- | --- | --- | --- |
| `SpecBind Brief` | Why the spec changes in the current milestone. | At most one discovered artifact, including same-milestone deltas. | Removed. |
| `SpecBind Research` | Optional current brownfield gap-analysis findings used as non-authoritative authoring input. | At most one discovered artifact; revised in place rather than appended as attempt history. | Removed. |
| `SpecBind Requirements` | The complete set of currently valid requirements. | The discovered singleton is revised in place. | Preserved. |
| `SpecBind Design` set | The complete currently valid design, optionally split by stable `artifact_id`. | One or more discovered artifacts are revised as needed. | Preserved. |
| `SpecBind Contract` | The current minimal cross-spec seam manifest. | The discovered singleton is revised only when externally observable seams change. | Preserved. |
| `SpecBind Implementation Notes` set | Optional free-form implementation knowledge useful to later AI runs. | Zero or more discovered artifacts are read and maintained when durable spec-specific knowledge is discovered. | Preserved. |
| `tasks.yaml` | Structured executable plan and progress for the current milestone's change. | Contains only current tasks and machine-validated execution state. | Removed. |
| `log.md` | Per-spec OKF update log of released changes and adopted baselines. | Preserved; not pre-edited during ordinary release or reverse orchestration. | The CLI inserts one concise Release or Baseline summary under the applicable newest-first date heading. |
| `spec.yaml` | Current lifecycle, active-change metadata, and gate evidence. | Represents an active change. | Represents released state with no active change. |
| `roadmap.md` | Intent, scope, and dependencies for the active milestone. | Exists under `steering/` and is maintained. | Moved to `releases/<version>-roadmap.md`. |
| `state/contract-review.md` | Current accepted milestone-wide contract review inputs and free-form AI judgment. | Exists only after review passes for a milestone containing Spec-backed items. | Moved to `releases/<version>-contract-review.md`; absent for Direct-only releases. |
| `baselines/<version>-roadmap.md` and `baselines/<version>-contract-review.md` | Non-release reverse-establishment history. | Absent during ordinary delivery. | Receive the reverse Roadmap and accepted review while Specs retain establishment provenance. |

Absence of `SpecBind Brief`, `SpecBind Research`, and `tasks.yaml` is the normal idle state of a released Spec. Placeholder working documents should not be required. Decision 0057 discovers Spec-local Markdown by OKF type; familiar Markdown filenames remain template defaults rather than lifecycle identity.

## Milestone identity and release binding

Every milestone has a `roadmap.md`, including a milestone that changes only one existing spec or creates only one new spec. Discovery confirms the route and initiates the milestone before it hands work to later phases. To avoid growing discovery into a general state-management skill, Rust CLI milestone operations perform roadmap creation and later mechanical updates under [Decision 0009](./decisions/0009-milestone-cli-boundary.md). A missing roadmap means there is no active milestone.

The milestone's stable identity does not depend on knowing the final release version. The working model separates:

- `milestone_id`: CLI-generated canonical UUID v7 accepted by Decision 0043
- `baseline_revision`: full Git commit object ID captured from clean `HEAD` immediately before milestone creation under Decision 0054
- `baseline_version`: existing product version required only by a Decision 0181 reverse milestone
- `target_release`: concrete Decision 0073 portable release label, initially unset when necessary

Under [Decision 0045](./decisions/0045-okf-markdown-artifacts.md), the roadmap is an OKF concept document and this mapping is authoritative YAML frontmatter:

```markdown
---
type: SpecBind Roadmap
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
baseline_revision: 0123456789abcdef0123456789abcdef01234567
target_release: null
work_items:
  spec_updates:
    - spec: checkout
      summary: Require authenticated checkout
---
```

When the release version becomes known, the workflow binds it to the active milestone:

```markdown
---
type: SpecBind Roadmap
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
baseline_revision: 0123456789abcdef0123456789abcdef01234567
target_release: v1.4.0
work_items:
  spec_updates:
    - spec: checkout
      summary: Require authenticated checkout
---
```

This is a metadata mapping, not textual replacement. Requirements, tasks, evidence, and spec changes continue to refer to the stable milestone identity. Under [Decision 0072](./decisions/0072-explicit-release-rebinding.md), changing an unshipped target version updates the binding in one authoritative place instead of rewriting milestone artifacts: initial binding is ordinary, while replacing a non-null value requires the explicit `--rebind` operation and user confirmation in agent-assisted use. The generated ID is not intended to be selected or named by the user. The baseline revision remains unchanged through ordinary work so contract review always compares the complete milestone delta.

The Rust CLI requires a clean repository immediately before roadmap creation, captures the full current `HEAD` as `baseline_revision`, and generates the UUID v7 locally without mutating a project counter. Mainline, hotfix, and worktree milestones therefore receive both collision-resistant identities and branch-local diff baselines. `specbind-release` requires a concrete `target_release` matching [Decision 0073](./decisions/0073-portable-release-version.md) for every release and refuses to begin release operations while it is unset or invalid.

## Active change

Each spec has at most one active change at a time. Under [Decision 0062](./decisions/0062-minimal-active-brief-profile.md), the active brief is free-form authoring input rather than another state schema. It should ordinarily communicate the context needed for the change, which may include:

- problem and desired outcome
- in-scope and out-of-scope behavior
- boundary impact and dependencies
- source request or issue

None of those topics is a required parsed section. Milestone identity is connected through the roadmap and `spec.yaml`, while release binding is roadmap-owned under Decision 0044; neither belongs in brief metadata. Additional deltas discovered in the same milestone are merged into that active brief. A later milestone creates a new brief artifact; it does not append the new change to the previous milestone's brief.

Discovery owns creating the active brief and transitioning an idle released spec into an active-change state. The precise routing and approval invalidation rules still need refinement.

## Active requirement set

The singleton `SpecBind Requirements` artifact remains the complete current requirement set. Separately, the requirements phase must establish an explicit active requirement set for the milestone and store it in `spec.yaml`; see [Decision 0003](./decisions/0003-active-requirement-set.md) and [Decision 0014](./decisions/0014-structured-spec-metadata.md):

- It contains every Requirement ID that must be implemented or revalidated for the active change.
- It may include unchanged existing requirements when the change requires their reimplementation or revalidation.
- It is not implicitly equal to every requirement in the requirements artifact.
- Requirements approval freezes the set for downstream phases.
- Changing the set returns the workflow to the requirements phase and invalidates affected downstream approval.
- Design traces the same set, and tasks must provide 100% coverage of it.

Within `spec.yaml`, `active_change.requirement_ids: null` means the set has not yet been established. Requirements approval replaces it with a non-empty, unique, deterministically ordered array of canonical Requirement IDs under Decision 0040. Release finalization clears `active_change` as part of returning the spec to released / idle state.

## Cross-spec contract

Every active spec maintains one fixed `contract.yaml` artifact as the current source of truth for seams other specs may observe or consume. It persists across releases and changes alongside the discovered design set; see [Cross-spec contracts](./cross-spec-contracts.md), [Decision 0011](./decisions/0011-cross-spec-contract.md), and [Decision 0155](./decisions/0155-versioned-yaml-contract-artifact.md).

The contract contains only stable ownership, exports, consumes, cross-spec invariants, and File Ownership entries. Design review confirms that internal design implements the contract and that an active change has not omitted a required contract update. CLI checks validate structure and references; agent review determines semantic compatibility and downstream revalidation scope.

## Active tasks and coverage

`tasks.yaml` contains only tasks for the active milestone:

- Task numbering restarts for a new milestone.
- Same-milestone deltas may merge into the active task plan.
- Tasks from earlier milestones are never merge input for a new task plan.
- Every active Requirement ID maps to at least one executable task.
- One task may cover several requirements, and one requirement may map to several tasks.
- Missing coverage for any active Requirement ID prevents completion of the tasks phase.
- Tasks are not deleted until the release gate confirms zero incomplete or blocked tasks and valid completion evidence.

Feature-level completion validation begins from a clean committed Git revision and uses the CLI preflight/accept handshake accepted by [Decisions 0029](./decisions/0029-completion-validation-handshake.md) and [0086](./decisions/0086-completion-cli-handshake.md). The agent runs project validation between those calls; the CLI accepts evidence only if the implementation revision, lifecycle inputs, approvals, and completed-task state are unchanged. Stale accepted completion is explicitly invalidated before a replacement handshake.

Only accepted `GO` evidence is persisted in `spec.yaml`; failed, manual-required, or rejected validation attempts remain run-scoped output under [Decision 0030](./decisions/0030-persist-only-accepted-completion-evidence.md). The accepted record attests that the mandatory semantic validation protocol passed, but does not persist redundant per-dimension pass flags or a duplicated `GO` field under [Decision 0034](./decisions/0034-do-not-persist-semantic-pass-flags.md).

Completion relies on the gate-local freshness chain accepted by [Decision 0032](./decisions/0032-gate-local-freshness-chain.md). Requirements, design, contract, active Requirement IDs, and the task plan remain owned by their earlier gates rather than being copied into completion evidence.

Contract review is milestone-wide state represented once under Decisions 0050, 0052, and 0078. `state/contract-review.md` stores exact input revisions and the accepted free-form AI judgment outside always-loaded Roadmap context; the Roadmap remains scope owner. Per-Spec completion evidence does not duplicate it, and its absence or staleness does not by itself make an unaffected `release_ready` Spec locally inconsistent.

The roadmap's machine-readable scope uses the grouped `work_items` frontmatter accepted by [Decision 0046](./decisions/0046-roadmap-work-items.md). New specs, existing-spec updates, and direct changes remain distinct categories; typed references form the dependency graph. Spec-backed progress is derived from each spec's lifecycle, while a direct change persists only optional `status: completed` under [Decision 0047](./decisions/0047-sparse-direct-change-status.md), with absence meaning pending. The Markdown body carries milestone context and rationale but has no CLI-parsed grammar.

Under [Decision 0051](./decisions/0051-current-state-roadmap.md), the active roadmap is a current-state manifest rather than an embedded change log. Confirmed scope, dependencies, direct-change completion, and release binding replace their current representations in place. When Spec-backed work exists, the separate accepted global review state is likewise current-only. Git records pre-release edits, release archives preserve the released scope and applicable review evidence, and per-spec `log.md` files summarize Spec-backed results.

## Released history

Under [Decision 0048](./decisions/0048-okf-spec-log.md), `log.md` is a navigable OKF update log, not a snapshot of the entire spec. Released entries are grouped under newest-first ISO `YYYY-MM-DD` date headings. Each entry uses release version as its primary human-facing label; the machine-generated milestone ID remains secondary trace metadata. An entry should include enough information to locate and understand the historical change:

- release version
- milestone ID
- released status
- problem and delivered-scope summary
- preserved contracts or explicitly unchanged behavior
- completed-task count and active-requirement coverage
- validation result and timestamp
- project release reference or relevant commit when useful
- related roadmap, issue, or follow-up

The complete pre-finalization Brief, optional Research, and `tasks.yaml` normally remain available through ordinary Git history or a project-created release reference. The roadmap remains directly available under `releases/<version>-roadmap.md`; a Spec-backed release also preserves its accepted review under `releases/<version>-contract-review.md`. Each participating spec's `log.md` points to the archived roadmap and may include project references when useful, but SpecBind requires no universal tag or commit field. A Direct-only release has no per-spec log entry.

The brief may provide drafting context for the problem summary, but it is not authoritative released state. Log content must agree with the final requirements, active Requirement IDs, completed tasks, roadmap, accepted completion evidence, and contract review; see [Decisions 0017](./decisions/0017-requirements-gate-inputs.md) and [0066](./decisions/0066-agent-judged-release-and-cli-log-insertion.md).

## Scope removal, abandonment, and rollback

These operations are intentionally distinct from successful release finalization; see [Decision 0005](./decisions/0005-active-change-abandonment.md):

- Unstarted scope can be removed by revising the active milestone and its affected briefs.
- Partially implemented unreleased work must be restored with explicit project and Git operations. SpecBind then reconciles its active artifacts and metadata with that repository state; it does not perform an automatic revert.
- An entire unreleased milestone can be abandoned only with explicit user confirmation. After Requirements and Design have been restored or reconciled, lifecycle cleanup removes milestone-local Brief, Research, and Tasks artifacts, clears affected `active_change` state, and removes `steering/roadmap.md` plus any matching `state/contract-review.md`.
- An abandoned unreleased milestone does not add per-spec release-log entries or a file under `releases/` by default. Committed work remains discoverable through Git history.
- A rollback of released behavior is represented as a new active change in a new milestone and is released normally.

Rust CLI milestone operations own the mechanical state transitions for scope changes and full abandonment. Discovery remains the user entry point, interprets intent, and obtains confirmation; the CLI validates and applies the deterministic transition. There is no separate `specbind-milestone` skill. See [CLI and agent responsibility boundary](./cli-agent-boundary.md).

## Release finalization contract

The portable release contract is a gated state transition. Project publication is supplied by the [project release adapter](./decisions/0002-project-release-adapter.md), while the lifecycle gates and SpecBind artifact finalization remain core behavior:

1. The release agent loads the active roadmap, target version, and the free-form `{{SPEC_DIR}}/settings/adapters/release.md`, then validates its OKF profile and interprets any applicable project guidance under Decisions 0063 and 0101.
2. The release agent runs the stateless `specbind release preflight` readiness check accepted by [Decision 0069](./decisions/0069-stateless-release-preflight.md).
3. Without creating an aggregate readiness record, the CLI derives readiness from current artifacts: it resolves participating specs, requires a concrete target version, requires every direct change to be completed, and verifies applicable tasks, approvals, completion evidence, fresh contract review, and lifecycle consistency under [Decision 0070](./decisions/0070-derived-release-readiness.md).
4. After successful preflight, the agent runs any applicable project preparation, publication, and verification guidance and judges the result with the human.
5. For a Spec-backed milestone, the agent submits one delivered-change summary per participating spec to the Rust CLI finalization boundary. A Direct-only milestone omits this input.
6. Without trusting or accepting a preflight token, the CLI independently rechecks core invariants, validates all evidence it can verify, and confirms that every resolved finalization target path is safe under Decision 0064.
7. For Spec-backed work, the CLI inserts one version-labeled, idempotent release entry into each participating spec's `log.md` under the applicable newest-first date heading.
8. The CLI removes each participating Spec's discovered Brief and optional Research artifacts plus fixed `tasks.yaml`.
9. The CLI transitions each `spec.yaml` to released / no-active-change state.
10. The CLI moves an applicable accepted `state/contract-review.md` to its release archive, then moves `steering/roadmap.md` last as the finalization completion marker. Direct-only milestones skip the review archive.
11. The CLI verifies the resulting idle state. A crash-interrupted mutation is idempotently retryable or stops for Git-assisted recovery under Decision 0081.
12. The agent runs optional project After finalize instructions and reports their result separately.

If publishing or release verification fails, finalization does not run and active documents remain intact. External partial success is reconciled by the agent and human without creating a partially released SpecBind milestone. Core finalization always covers the complete participating set as one logical transition; re-running it must not duplicate `log.md` entries or remove unrelated work. An After finalize failure becomes follow-up work and does not undo or reopen the release. See [Decisions 0010](./decisions/0010-release-execution-boundary.md) and [0071](./decisions/0071-no-partial-milestone-release.md).

## Lifecycle and dependency semantics

The authoritative per-spec workflow states and guarded transitions are defined in [Spec state machine](./spec-state-machine.md). In particular, an inconsistency is derived health over a declared workflow state rather than another freely writable lifecycle value.

The workflow must distinguish these states:

| State | Expected working files | Meaning |
| --- | --- | --- |
| Released and idle | No brief artifact or `tasks.yaml` | The current requirements and design are implemented; no active milestone change exists. |
| Active change before task generation | A singleton brief artifact exists; `tasks.yaml` may not yet exist | Spec revision is in progress. |
| Active implementation | A singleton brief artifact and `tasks.yaml` exist | Current milestone tasks and approvals determine readiness. |
| Interrupted or inconsistent | Metadata says active but required phase artifacts are missing | Resume or repair is required. |

Dependencies must also distinguish:

- a dependency on the current approved spec contract
- a dependency on an active revision that must complete first
- a dependency on released behavior, established from the current contract and per-spec release log

Task dependencies remain local to one spec's active `tasks.yaml`. Active revision ordering belongs to the milestone roadmap, persistent observable dependencies belong to the singleton contract artifact, and released dependencies use current contracts plus release history; see [Decision 0027](./decisions/0027-spec-local-task-dependencies.md).

A released spec without `tasks.yaml` must not be treated as unimplemented.

## Initial migration

Existing projects need a one-time cutover separate from normal release finalization:

1. Group earlier completed briefs and tasks by released change where evidence permits.
2. Backfill concise date-grouped `log.md` entries with release and validation references.
3. Preserve only the current milestone's brief and tasks as the active working set.
4. Reconstruct and verify the current active requirement set.
5. Leave the current active documents in place until their normal release finalization.

Historical full documents remain authoritative at their existing tags or commits. Migration should not manufacture certainty when old evidence is incomplete.

## Generalization boundary

Issue #50 comes from a repository with local skill overrides. These details are useful evidence but are not automatically SpecBind-wide requirements:

| pc-build-planner detail | SpecBind treatment |
| --- | --- |
| `kiro-spec-update-batch` as a separate skill | Existing-spec batch orchestration is required; whether it is a separate skill remains open. |
| `kiro-record-validation` and a roadmap validation table | Completion evidence belongs in `spec.yaml` through the integration-validation and CLI handshake accepted by Decision 0029; Decision 0037 fixes its strict three-field shape, while Decisions 0050 and 0052 keep one global cross-spec record in project state. |
| `kiro-impl-direct` and structured direct candidates | Direct-work support is a separate workflow decision, not required by the active-spec lifecycle itself. |
| GitHub Issues, GitHub milestones, Actions, ZIP packaging, and project version scripts | Repository-specific instructions in the release adapter, not hard-coded core behavior. |
| Direct commits to `main` and exact commit messages | Repository policy, not a universal SpecBind requirement. |
| v0.5.1 cutover sequence | Concrete example of the general one-time migration pattern. |

The project-local append-only Change Brief behavior is the observed problem, not a behavior to preserve in the target contract.

## Skills affected by the portable contract

- `specbind-discovery`: analyze and route work, create one active brief, and initiate confirmed milestone changes.
- bundled CLI: capture and validate the milestone baseline, create and update the active roadmap, bind the target release, perform confirmed rebaseline or abandonment cleanup, and check active Requirement ID traceability.
- requirements workflow: revise current requirements and freeze the active requirement set.
- design workflow: revise current design, trace the active requirement set, and maintain the cross-spec contract.
- tasks workflow: generate a milestone-local plan with complete active-requirement coverage.
- implementation and validation workflows: operate only on current tasks and current milestone evidence.
- status workflow: report released state, active-change state, current tasks, and latest history separately.
- completion verification: distinguish current coverage from historical release records.
- contract review: read contracts first and deepen into affected specs only when boundaries change or remain ambiguous.
- `specbind-release`: perform gated, idempotent finalization after release success.

Plan's default routing and named and all-Spec scope modes through Tasks approval
are fixed by Decision 0161. Implementation remains one Roadmap item per
`specbind-implement` invocation. Decision 0168 accepts `specbind-drive` as the
thin milestone-wide controller that composes those existing runs, parks
branch-local attention while independent work remains reachable, and stops
before release execution. It is implemented by the embedded `specbind-drive`
package; [Issue #9](https://github.com/Huruikagi/specbind/issues/9) retains the
design and implementation history.

## Open questions

- Whether projects need an opt-in audit record for abandoned, unreleased
  milestones remains part of
  [Issue #8](https://github.com/Huruikagi/specbind/issues/8).
