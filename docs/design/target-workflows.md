# Target workflows

This document defines the intended user journeys and responsibility boundaries for the future SpecBind skill system. It stays name-neutral where naming is not yet decided; concrete names belong in the [target skill catalog](./target-skill-catalog.md).

The detailed milestone document lifecycle is defined in [Active spec lifecycle](./active-spec-lifecycle.md). Deterministic automation boundaries are developed in [CLI and agent responsibility boundary](./cli-agent-boundary.md), and the accepted implementation direction is the [Rust CLI migration](./rust-cli-migration.md).

The target per-spec workflow states, approval invalidation events, and transition guards are defined in [Spec state machine](./spec-state-machine.md).

Explicit and delegated gate approval, non-interactive execution, and rejection of the inherited `-y` flag are accepted in [Decision 0012](./decisions/0012-delegated-approval.md).

The active implementation plan and progress use canonical `tasks.yaml` rather than a Markdown task document under [Decision 0013](./decisions/0013-structured-task-artifact.md).

Per-spec lifecycle metadata uses canonical `spec.yaml`; CLI configuration and installation manifests remain outside that decision under [Decision 0014](./decisions/0014-structured-spec-metadata.md).

Status: Draft

## Design goals

- Give users one clear entry point when they do not yet know the right workflow.
- Keep discovery, specification, implementation, and verification responsibilities distinguishable.
- Make every persistent write and approval boundary visible.
- Support both a deliberate phase-by-phase path and an intentional accelerated path.
- Keep equivalent Claude Code and Codex workflows aligned without hiding platform-specific capabilities.
- Treat specs as active, maintained descriptions of the product across milestones and releases.
- Move deterministic parsing, invariant checks, and state mutations out of agent-specific shell instructions and into the bundled CLI.
- Keep document formats and project-wide AI rules customizable through shared settings consumed consistently by every supported agent.
- Make cross-spec review contract-first so context follows affected boundaries rather than total document volume.

Managed Markdown authoring loads the concise shared `{{SPEC_DIR}}/settings/rules/okf-artifacts.md` accepted by [Decision 0049](./decisions/0049-okf-authoring-rule.md). The rule keeps agents aligned with the targeted OKF version and SpecBind's profile boundary, while deterministic CLI validation remains authoritative.

## Spec lifecycle

Specs persist as active specifications while their represented capabilities remain part of the product. Completing implementation or shipping a release does not, by itself, make a spec historical or disposable.

When later work changes an existing capability, discovery should route that work through maintenance of the existing spec where it is the correct boundary. The current inherited discovery flow already has an existing-spec route, but its target behavior needs further refinement from operational experience.

Requirements recorded so far:

- Existing specs remain available across releases.
- Every change-bearing milestone starts with a `roadmap.md`, even when only one spec participates.
- A milestone has a machine-generated stable identity independent of an initially unknown release version.
- Requirements and design are maintained when represented behavior changes.
- Each spec has at most one active milestone change at a time.
- Requirements freezes an explicit active requirement set in `spec.yaml` for downstream design and task coverage.
- Briefs and tasks are milestone-local working documents, not append-only release history.
- Released change history is indexed separately from the current requirements and design.
- A release closes a milestone, not the specs involved in that milestone.
- Improvements to the existing-spec update route will be specified incrementally.

Open questions:

- How discovery decides between updating an existing spec and creating a new spec.
- Which spec phases must rerun after different kinds of change.
- How approval state and implementation tasks reset when an active spec is revised.
- How obsolete capabilities and their specs are retired.

## Milestone and release lifecycle

```text
No active milestone
  -> discovery confirms the route and initiates milestone creation
  -> Rust CLI captures clean HEAD and creates roadmap.md with stable milestone identity and baseline revision
  -> active milestone
       -> discovery confirms scope and ordering changes
       -> Rust CLI applies confirmed milestone state
       -> release version may be bound later through the CLI; replacing it is an explicit confirmed rebind
       -> new and existing specs are created or updated
       -> implementation and validation
       -> target release version is required
       -> release readiness
  -> release succeeds
       -> release skill archives roadmap.md by version
  -> no active milestone
```

`roadmap.md` is the required durable representation of every active milestone, including single-spec work. It begins from the discovery user journey and remains present for the lifetime of that milestone. Under [Decisions 0045](./decisions/0045-okf-markdown-artifacts.md), [0046](./decisions/0046-roadmap-work-items.md), and [0054](./decisions/0054-milestone-baseline-revision.md), it is an OKF concept whose YAML frontmatter owns `type: SpecBind Roadmap`, the milestone identity, the branch-local baseline revision, the release binding, and grouped work items while its unparsed Markdown body remains the readable context and rationale. Spec-backed progress is derived, while direct changes persist only the sparse completed state accepted by [Decision 0047](./decisions/0047-sparse-direct-change-status.md). [Decision 0050](./decisions/0050-global-cross-spec-review.md) defines one accepted milestone-wide cross-spec review with no per-spec pass records; [Decisions 0052](./decisions/0052-project-state-artifacts.md), [0053](./decisions/0053-minimal-cross-spec-review-state.md), and [0055](./decisions/0055-cross-spec-review-inputs.md) store its structured classifications, contract-first input revisions, and AI-authored judgment in `state/cross-spec-review.md` so ordinary agents can keep loading the roadmap without that context cost. The CLI always parses every current contract to build the graph, while the agent loads deeper requirements, design, or task plans only when the judgment materially depends on them. Under [Decision 0051](./decisions/0051-current-state-roadmap.md), the active roadmap represents current state only, leaving edit history to Git and preserving only the final snapshot at release. To keep discovery from absorbing every state transition, Rust CLI operations perform clean-baseline capture, mechanical creation, scope updates, explicit rebaseline, direct-change status mutation, consistency checks, and release-version binding from confirmed discovery output; see [Decision 0009](./decisions/0009-milestone-cli-boundary.md). Its Decision 0043 UUID v7 stable identity is mapped to a release version when that version becomes known; replacing a non-null binding uses the explicitly confirmed Decision 0072 rebind operation, while its Decision 0054 baseline remains fixed through ordinary work. `specbind-release` refuses to start release operations until the version is assigned or while a direct change remains pending. After a successful release, it moves the active roadmap and global review state to the paired flat files `releases/<version>-roadmap.md` and `releases/<version>-cross-spec-review.md`.

### Scope removal, abandonment, and rollback

- Removing unstarted work is a confirmed update to the active milestone scope.
- Partially implemented unreleased work is restored through explicit project and Git operations, followed by reconciliation of roadmap and active-spec state. SpecBind does not automatically revert repository content.
- Abandoning the entire unreleased milestone requires explicit user confirmation and reconciled specs before milestone-local artifacts and active-change metadata are cleared. It creates no release-log entry or release-roadmap archive by default.
- Reversing released behavior is new work in a new milestone and returns through the normal release path.

These rules are accepted in [Decision 0005](./decisions/0005-active-change-abandonment.md). Discovery confirms the intent and the Rust CLI owns the guarded milestone-state mutation.

The portable release contract owns gated and idempotent spec finalization. Project-specific packaging, versioning, publishing, and verification instructions come from `{{SPEC_DIR}}/settings/release.md`; see [Decision 0002](./decisions/0002-project-release-adapter.md).

```text
Rust CLI: core preflight and readiness gates
  -> AI agent: adapter Prepare (when applicable)
  -> AI agent: adapter Publish (when applicable) and capture useful project evidence
  -> AI agent: adapter Verify (when applicable), judge success, and prepare spec summaries
  -> Rust CLI: recheck deterministic state, insert logs, and finalize active spec artifacts
  -> AI agent: adapter After finalize (when applicable)
```

Adapter guidance cannot waive a core gate. An empty adapter means no project-specific actions; ambiguous non-empty guidance causes the agent to stop rather than infer commands from unrelated project files. The CLI does not execute natural-language adapter instructions; the agent orchestrates applicable guidance and hands structured results to the CLI under [Decisions 0010](./decisions/0010-release-execution-boundary.md) and [0063](./decisions/0063-free-form-release-adapter-profile.md). Finalization uses `specbind release finalize`; a forceable target-path conflict is reported to the user and may be retried with `--force` only after explicit confirmation under Decision 0065.

## Cross-spec review

Every active spec has a persistent `contract.md` containing only the seam that other specs may observe or depend on. Cross-spec review reads the roadmap and contracts first, asks the CLI to validate and build the dependency graph, and then loads full requirements, design, and tasks only for affected or ambiguous specs.

```text
roadmap + current contracts
  -> CLI structure, reference, ownership, and graph checks
  -> agent semantic classification
       -> LOCAL_ONLY: spec-local review
       -> CONTRACT_COMPATIBLE: targeted consumer review
       -> CONTRACT_BREAKING: downstream revision or revalidation
  -> scoped deep review and milestone evidence
```

Direct implementation candidates must declare contract impact. An unjustified `none` returns the request to discovery for existing-spec or new-spec routing. Missing contracts trigger a safe full-document fallback and a migration diagnostic, not an assumption of no impact. See [Cross-spec contracts](./cross-spec-contracts.md).

## New work

```text
User request
  -> discovery
       -> direct implementation candidate
       -> update an existing spec
       -> create one new spec
            -> requirements
            -> design
            -> tasks
            -> implementation
            -> integration validation
       -> create several new specs
            -> roadmap and boundaries
            -> per-spec requirements, design, and tasks
            -> implementation by dependency order
            -> cross-spec validation
```

Open questions:

- Whether direct implementation remains only a recommendation or gets a dedicated skill.
- Whether multi-spec decomposition belongs to discovery or a separate planning skill.
- Whether the reusable contract-first review stage is implemented inside batch orchestration or as a shared workflow invoked by batch and other routes.

## Existing-system work

```text
Existing repository
  -> project guidance bootstrap or sync
  -> discovery
       -> direct change
       -> update an existing spec
       -> create a new spec
            -> optional codebase gap analysis
            -> requirements and design
            -> tasks and implementation
            -> integration validation
```

The target contract should state when project guidance and gap analysis are optional, recommended, or required. They should not become implicit prerequisites merely because they exist as skills.

## Responsibility boundaries

| Stage | Owns | Does not own |
| --- | --- | --- |
| Discovery | Routing, scope clarification, decomposition, active brief, and confirmed milestone intent | Detailed requirements, technical design, or mechanical milestone-state transitions |
| Rust CLI milestone operations | Roadmap creation, confirmed scope updates, release-version binding, consistency checks, and confirmed abandonment cleanup | Request analysis, spec authoring, automatic Git rollback, or release publication |
| Requirements | User-visible behavior, constraints, acceptance criteria | Architecture and implementation sequencing |
| Design | Architecture, interfaces, data flow, file boundaries, active-requirement traceability, and current contract maintenance | Task execution or unapproved scope changes |
| Tasks | Executable decomposition, dependencies, verification expectations, complete active-requirement coverage, and loading persistent spec implementation notes when present | Implementation or historical task accumulation |
| Implementation | Code and tests for approved tasks, progress recording, and maintenance of durable spec-specific knowledge in `implementation-notes.md` | Silent changes to approved requirements or design |
| Review | Independent task-level conformance review | Feature-level integration acceptance |
| Integration validation | Spec-local task integration, full verification, requirements/design/boundary judgment, and run-scoped candidate evidence produced between the CLI completion preflight and guarded acceptance calls | Replacing missing task-level review, directly mutating completion state, or persisting semantic pass checklists and failed attempts in lifecycle metadata |
| Cross-spec review | Contract-first dependency, ownership, invariant, impact, and downstream review analysis, with the accepted milestone-wide result recorded once in the active roadmap | Reloading every complete spec by default, replacing local design review, or copying the result into each spec's completion evidence |
| Completion verification | Evidence for a specific success claim | Broad design or implementation work |
| Release agent orchestration | Read the adapter, call stateless `specbind release preflight`, execute project phases, reconcile any external partial success, prepare per-spec summaries, call whole-milestone CLI finalization, and report outcomes | Treating preflight as mutation authority, claiming a subset release, automatically rolling back external systems, reimplementing lifecycle mutations, or bypassing core gates |
| Rust CLI release core | Readiness gates, mechanically verifiable evidence, idempotent active-spec finalization, and stable diagnostics | Executing natural-language project publication instructions or claiming unobservable external success |
| Release adapter | Project-specific Prepare, Publish, Verify, and optional After finalize instructions | Weakening core gates or directly defining spec lifecycle semantics |

## CLI and agent execution order

Where a transition has both mechanical and semantic requirements, the workflow should make both layers explicit:

```text
bundled CLI: parse and check deterministic invariants
  -> agent: review meaning and explain or repair issues
  -> user or workflow: approve the transition when required
  -> bundled CLI: perform explicit, guarded state mutation
```

The first concrete checker validates active Requirement ID traceability across `requirements.md`, `spec.yaml`, `design.md`, and `tasks.yaml`. Skills consume its concise text result and stable code instead of independently rebuilding the same check with shell searches. Mechanical success is necessary but never substitutes for semantic review.

## Approval and automation model

The future workflow needs an explicit answer for each transition:

| Transition | Current target question |
| --- | --- |
| Discovery -> spec work | Is the selected route confirmed by the user? |
| Requirements -> design | Did the review pass, and does current-revision evidence record valid explicit or delegated approval? |
| Design -> tasks | Did technical and contract review pass, and is approval authorized for the current revision? |
| Tasks -> implementation | Did task review pass, and is approval authorized for the current task revision? |
| Implementation -> completion | Which reviews and fresh verification evidence are required? |
| Milestone -> release | What proves every required milestone item is ready? |
| Release version assignment | Is a concrete target version bound to the active milestone? |
| Release -> milestone closed | Did the release succeed before `roadmap.md` is archived out of `steering/`? |

Accelerated and batch workflows may automate transitions, but they should reuse the same phase contracts rather than define competing document formats or success criteria.

An accelerated workflow keeps run-scoped `delegated` authorization for named future gates in its orchestration context; it does not add a project artifact. Each gate still runs its normal checks and emits the same approval event as the deliberate path. Delegation only removes the extra confirmation pause after a passing gate. `--non-interactive` does not imply approval and stops when neither valid explicit approval nor in-scope delegated authorization is available.

## Topics to resolve next

1. Define the initial `specbind check traceability` contract and diagnostic schema.
2. Refine discovery's existing-spec update route and turn the draft spec-state events into accepted CLI mutation contracts.
3. Define milestone contents and release-readiness criteria.
4. Define the concrete responsibilities of `specbind-release` and its CLI operations.
5. Decide whether quick and batch remain first-class skills.
6. Review the separation among task review, integration validation, and completion verification.
