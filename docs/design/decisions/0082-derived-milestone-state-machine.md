# 0082: Derive milestone state and phase-relative dependency waves

Status: Accepted

## Context

The Roadmap already persists milestone identity, scope, dependencies, release binding, and sparse Direct completion. Participating Specs persist their own workflow states and evidence, while the accepted contract review is separate project state. Adding a writable milestone `status` or copying per-Spec progress into the Roadmap would create competing authorities.

Roadmap dependencies still need precise phase semantics. Applying every edge to every phase would serialize Requirements and Tasks unnecessarily, while ignoring dependencies until release would allow consumers to design or implement against unfinished producers. Project-revision-scoped completion evidence also requires final validation to converge at one clean Git revision.

## Decision

- The active Roadmap has no persisted milestone `status`, phase, readiness flag, wave number, or aggregate progress counters.
- The CLI derives one milestone read model from the current Roadmap, participating `spec.yaml` and `tasks.yaml` artifacts, Direct status, contract review freshness, target-release binding, Git state, and release guards.
- The normal derived delivery stages are `requirements`, `design`, `cross_spec_review`, `tasks`, `implementation`, `validation`, `release_pending`, and `release_ready`. Mixed per-item progress is represented by the earliest unsatisfied milestone barrier plus item-level detail, not by copying state into the Roadmap.
- Requirements authoring and approval are not dependency-gated.
- Design approval for a Spec waits only for its direct Spec-backed predecessors to have current Design approval. Dependencies involving a Direct item do not gate Design.
- Contract review is one global barrier after every participating Spec has current Design approval and before any current `tasks.yaml` is authored.
- After a fresh review, participating Specs may author and approve Tasks in parallel. Roadmap dependencies do not serialize Tasks.
- Implementation uses the complete Roadmap DAG. A predecessor is implementation-complete when:
  - a Direct item has `status: completed`; or
  - a Spec-backed item has all Tasks completed and unblocked at a clean committed project revision. `release_ready` is not required.
- A pending item is implementation-actionable only when its own prerequisites are ready and every direct Roadmap predecessor is implementation-complete. Topological implementation waves are derived from those predicates; wave numbers are never persisted.
- Final implementation validation begins only after every Roadmap item is implementation-complete. All participating Specs requiring validation or revalidation converge on the same clean project `HEAD`. Parallel validation is allowed, but any later project-content commit makes earlier Spec completion evidence stale under Decision 0080.
- `release_ready` is derived only when every participating Spec has fresh completion evidence at the current clean revision, every Direct item is complete, the applicable contract review is fresh, a target release is bound, and every other deterministic release guard passes.
- Release execution states are run-scoped. Preflight, project Prepare/Publish/Verify work, and core finalization do not add a persisted Roadmap status. Failure before finalization leaves the milestone active; successful finalization archives the Roadmap last and returns the project to no active milestone.
- A crash-interrupted finalization is an invalid recoverable mutation state, not a supported partial-release milestone state. The CLI diagnoses and idempotently resumes or stops for Git-assisted recovery.

### Status command surface

- `specbind milestone status` is the canonical current-milestone read command. It takes no Spec argument and reports only the active Roadmap scope, including both Spec-backed and Direct items.
- When no active Roadmap exists, the command returns `NO_CHANGE NO_ACTIVE_MILESTONE` rather than treating every persistent Spec as one implicit milestone.
- A successful active projection returns `OK MILESTONE_STATUS_REPORTED`. A derived `inconsistent` health result is still a successful read with diagnostics; an unreadable Roadmap fails because no authoritative scope can be projected.
- `specbind spec status <spec>` remains the per-Spec drilldown, while `specbind tasks list/show` remains the task drilldown. Listing every persistent Spec is a separate future inventory concern and is not overloaded onto milestone status.
- The `specbind-status` skill is the convenience router: no argument requests current milestone status, an explicit Spec identity requests per-Spec status, and task-specific questions use the task read commands.

## Consequences

- Roadmap, Spec, review, and Git artifacts each remain authoritative only for facts they already own.
- Requirements and Tasks retain useful parallelism, while Design and Implementation respect dependencies where unfinished upstream work changes their meaning.
- The global validation barrier makes the whole-project revision handshake explicit instead of repeatedly invalidating supposedly final evidence during normal implementation.
- Status tooling can explain current stage, blockers, actionable items, and waves without introducing a new lifecycle artifact.

## Implementation status

The Rust CLI exposes the accepted per-Spec, task, and milestone read models. `milestone status` projects the active Roadmap, participating Spec status models, sparse Direct completion, review freshness, Git state, dependency readiness, actionable items, and release blockers. The shared release-readiness resolver now owns archive and target-path guards, so `release_ready` is derived only when `release preflight` would pass; successful `release finalize` archives the Roadmap last and leaves no active milestone.
