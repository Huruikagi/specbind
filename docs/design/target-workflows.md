# Target workflows

This document defines the intended user journeys and responsibility boundaries for the future SpecBind skill system. It stays name-neutral where naming is not yet decided; concrete names belong in the [target skill catalog](./target-skill-catalog.md).

The detailed milestone document lifecycle is defined in [Active spec lifecycle](./active-spec-lifecycle.md). Deterministic automation boundaries are developed in [CLI and agent responsibility boundary](./cli-agent-boundary.md), and the accepted implementation direction is the [Rust CLI migration](./rust-cli-migration.md).

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

## Spec lifecycle

Specs persist as active specifications while their represented capabilities remain part of the product. Completing implementation or shipping a release does not, by itself, make a spec historical or disposable.

When later work changes an existing capability, discovery should route that work through maintenance of the existing spec where it is the correct boundary. The current inherited discovery flow already has an existing-spec route, but its target behavior needs further refinement from operational experience.

Requirements recorded so far:

- Existing specs remain available across releases.
- Every change-bearing milestone starts with a `roadmap.md`, even when only one spec participates.
- A milestone has a machine-generated stable identity independent of an initially unknown release version.
- Requirements and design are maintained when represented behavior changes.
- Each spec has at most one active milestone change at a time.
- Requirements freezes an explicit active requirement set in `spec.json` for downstream design and task coverage.
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
  -> milestone lifecycle creates roadmap.md with stable milestone identity
  -> active milestone
       -> milestone lifecycle maintains confirmed scope and ordering
       -> release version may be bound later through milestone lifecycle
       -> new and existing specs are created or updated
       -> implementation and validation
       -> target release version is required
       -> release readiness
  -> release succeeds
       -> release skill archives roadmap.md by version
  -> no active milestone
```

`roadmap.md` is the required durable representation of every active milestone, including single-spec work. It begins from the discovery user journey and remains present for the lifetime of that milestone. To keep discovery from absorbing every state transition, the proposed milestone-lifecycle responsibility performs the mechanical creation, scope updates, and release-version binding from confirmed discovery output. Its machine-generated stable identity is mapped to a release version when that version becomes known; artifacts are not renamed or rewritten merely to replace a provisional version. `specbind-release` refuses to start release operations until the version is assigned. After a successful release, it moves the active roadmap from `steering/` to the new flat file `releases/<version>-roadmap.md`.

### Scope removal, abandonment, and rollback

- Removing unstarted work is a confirmed update to the active milestone scope.
- Partially implemented unreleased work is restored through explicit project and Git operations, followed by reconciliation of roadmap and active-spec state. SpecBind does not automatically revert repository content.
- Abandoning the entire unreleased milestone requires explicit user confirmation and reconciled specs before milestone-local artifacts and active-change metadata are cleared. It creates no changelog entry or release-roadmap archive by default.
- Reversing released behavior is new work in a new milestone and returns through the normal release path.

These rules are accepted in [Decision 0005](./decisions/0005-active-change-abandonment.md). Whether milestone lifecycle is a directly invocable skill or an internal responsibility remains Draft.

The portable release contract owns gated and idempotent spec finalization. Project-specific packaging, versioning, publishing, and verification instructions come from `{{SPEC_DIR}}/settings/release.md`; see [Decision 0002](./decisions/0002-project-release-adapter.md).

```text
core preflight and readiness gates
  -> adapter: Prepare
  -> adapter: Publish
  -> adapter: Verify
  -> core: verify immutable reference and finalize active spec artifacts
  -> adapter: After finalize (optional)
```

An adapter phase cannot waive a core gate. If Publish or Verify instructions are missing, release stops before publication rather than inferring commands from unrelated project files.

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
- Whether cross-spec validation is part of batch orchestration or an explicit final skill.

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
| Milestone lifecycle (proposed) | Roadmap creation, confirmed scope updates, release-version binding, and confirmed abandonment cleanup | Request analysis, spec authoring, automatic Git rollback, or release publication |
| Requirements | User-visible behavior, constraints, acceptance criteria | Architecture and implementation sequencing |
| Design | Architecture, interfaces, data flow, file boundaries, active-requirement traceability | Task execution or unapproved scope changes |
| Tasks | Executable decomposition, dependencies, verification expectations, complete active-requirement coverage | Implementation or historical task accumulation |
| Implementation | Code and tests for approved tasks, progress recording | Silent changes to approved requirements or design |
| Review | Independent task-level conformance review | Feature-level integration acceptance |
| Integration validation | Cross-task behavior, full verification, spec coverage | Replacing missing task-level review |
| Completion verification | Evidence for a specific success claim | Broad design or implementation work |
| Release core | Readiness gates, verified publication boundary, and active-spec finalization | Project-specific build or publication commands |
| Release adapter | Project-specific Prepare, Publish, Verify, and optional After finalize instructions | Weakening core gates or directly defining spec lifecycle semantics |

## CLI and agent execution order

Where a transition has both mechanical and semantic requirements, the workflow should make both layers explicit:

```text
bundled CLI: parse and check deterministic invariants
  -> agent: review meaning and explain or repair issues
  -> user or workflow: approve the transition when required
  -> bundled CLI: perform explicit, guarded state mutation
```

The first concrete checker validates active Requirement ID traceability across `requirements.md`, `spec.json`, `design.md`, and `tasks.md`. Skills consume its concise or JSON result instead of independently rebuilding the same check with shell searches. Mechanical success is necessary but never substitutes for semantic review.

## Approval and automation model

The future workflow needs an explicit answer for each transition:

| Transition | Current target question |
| --- | --- |
| Discovery -> spec work | Is the selected route confirmed by the user? |
| Requirements -> design | Are requirements accepted, and what counts as acceptance? |
| Design -> tasks | Has technical design passed its review gate? |
| Tasks -> implementation | Are task boundaries and dependencies approved? |
| Implementation -> completion | Which reviews and fresh verification evidence are required? |
| Milestone -> release | What proves every required milestone item is ready? |
| Release version assignment | Is a concrete target version bound to the active milestone? |
| Release -> milestone closed | Did the release succeed before `roadmap.md` is archived out of `steering/`? |

Accelerated and batch workflows may automate transitions, but they should reuse the same phase contracts rather than define competing document formats or success criteria.

## Topics to resolve next

1. Define the initial `specbind check traceability` contract and diagnostic schema.
2. Refine discovery's existing-spec update route and decide which milestone operations move into the CLI.
3. Define milestone contents and release-readiness criteria.
4. Define the concrete responsibilities of `specbind-release` and its CLI operations.
5. Decide whether quick and batch remain first-class skills.
6. Review the separation among task review, integration validation, and completion verification.
