# Target workflows

This document defines the intended user journeys and responsibility boundaries for the SpecBind v1 skill system. Concrete names belong in the [target skill catalog](./target-skill-catalog.md) and are accepted by [Decision 0075](./decisions/0075-v1-skill-and-orchestration-scope.md).

The detailed milestone document lifecycle is defined in [Active spec lifecycle](./active-spec-lifecycle.md). Deterministic automation boundaries are developed in [CLI and agent responsibility boundary](./cli-agent-boundary.md), and the accepted implementation direction is the [Rust CLI migration](./rust-cli-migration.md).

The target per-spec workflow states, approval invalidation events, and transition guards are defined in [Spec state machine](./spec-state-machine.md). Aggregate stage, phase-relative dependency waves, validation convergence, and release execution are defined in [Milestone state machine](./milestone-state-machine.md).

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

Capability retirement remains explicitly outside v1. Other routing and invalidation rules are defined by the per-Spec state machine and the accepted skill boundary.

## Milestone and release lifecycle

```text
No active milestone
  -> discovery confirms the route and initiates milestone creation
  -> Rust CLI captures clean HEAD and creates roadmap.md with stable milestone identity and baseline revision
  -> active milestone
       -> discovery confirms scope and ordering changes
       -> Rust CLI applies confirmed milestone state
       -> release version may be bound later through the CLI; replacing it is an explicit confirmed rebind
       -> new and existing specs pass Requirements and Design
       -> one contract-first global review passes
       -> Tasks are authored and approved
       -> each Roadmap item is implemented and validated
       -> target release version is required
       -> release readiness
  -> release succeeds
       -> release skill archives roadmap.md by version
  -> no active milestone
```

`roadmap.md` is the required durable representation of every active milestone, including single-item and Direct-only work. It begins from discovery and remains present for the milestone lifetime. Under Decisions 0045, 0046, and 0054, its frontmatter owns the milestone identity, branch-local baseline revision, release binding, and grouped work items while its body remains readable context. Spec-backed progress is derived; Direct items persist only sparse completion state. Decision 0078 stores one accepted free-form cross-spec assessment plus its contract-first input revisions in `state/cross-spec-review.md` whenever the milestone contains a Spec-backed item. Direct-only milestones have no such artifact. Git preserves pre-release history, while release archives the final roadmap and, when present, the accepted global review.

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

Adapter guidance cannot waive a core gate. An empty adapter means no project-specific actions; ambiguous non-empty guidance causes the agent to stop rather than infer commands from unrelated project files. The CLI does not execute natural-language adapter instructions. Finalization uses `specbind release finalize`; any dirty finalization target stops the operation until the path is made Git-clean. V1 has no finalization `--force` bypass.

## Cross-spec review

Every persistent Spec has a Contract containing only the seam that other Specs may observe or depend on. After all participating Designs are approved, cross-spec review reads the Spec-backed Roadmap projection and every current Contract, asks the CLI to validate and build the graph, and loads Requirements or selected Design artifacts only where semantic judgment requires them. Tasks do not yet exist and are never review inputs.

```text
Spec-backed roadmap scope + every current contract
  -> CLI structure, reference, ownership, and graph checks
  -> agent free-form semantic assessment
  -> affected Specs return explicitly to Design when necessary
  -> successful assessment is bound to exact input revisions
```

Direct items are excluded from review scope because requiring no canonical Contract change is already a Direct-route precondition. A missing Contract prevents normal acceptance rather than falling back to an inferred steady state. File-ownership overlap and dependency-cycle findings are warnings for agent judgment; dangling references remain mechanical errors. See Decision 0078 and [Cross-spec contracts](./cross-spec-contracts.md).

## New work

```text
User request
  -> discovery
       -> direct implementation candidate
       -> update an existing spec
       -> create one new spec
            -> requirements
            -> design
            -> cross-spec review
            -> tasks
            -> implementation
            -> implementation validation
       -> create several new specs
            -> roadmap and boundaries
            -> per-spec requirements and design
            -> one cross-spec review
            -> per-spec tasks approval
            -> stop; implementation remains per-item in v1
```

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
| Implementation | Code and tests for one Spec-backed or Direct Roadmap item, progress recording, and maintenance of durable spec-specific knowledge | Silent changes to approved requirements, design, or contract; multi-item orchestration in v1 |
| Task review | Independent or inline task-level conformance review according to the run-scoped review mode | Spec-level implementation acceptance |
| Implementation validation | One Spec's complete task integration, full verification, requirements/design/boundary judgment, and run-scoped candidate evidence produced between CLI completion calls | Replacing task-level review, directly mutating completion state, or persisting failed attempts |
| Cross-spec review | Contract-first dependency, ownership, invariant, impact, and downstream analysis, recorded once as accepted project state | Loading Tasks, replacing local Design review, reviewing Direct items, or copying results into per-Spec evidence |
| Completion verification | Evidence for a specific success claim | Broad design or implementation work |
| Release agent orchestration | Read the adapter, call stateless `specbind release preflight`, execute project phases, reconcile any external partial success, prepare per-spec summaries when Spec-backed work exists, call whole-milestone CLI finalization, and report outcomes | Treating preflight as mutation authority, claiming a subset release, automatically rolling back external systems, reimplementing lifecycle mutations, or bypassing core gates |
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
3. Define the concrete public CLI commands and stable result codes for accepted lifecycle mutations.
4. Refine discovery's existing-Spec update and scope-reconciliation operations from implementation experience.
