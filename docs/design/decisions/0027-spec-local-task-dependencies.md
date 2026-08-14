# 0027: Keep task dependencies local to one spec

Status: Accepted

## Context

Some inherited cc-sdd projects contain AI-authored `_Depends:_` text that appears to name a task in another spec. The documented cc-sdd task format, however, uses unqualified numeric Task IDs within one `tasks.md`; multi-spec ordering belongs to the roadmap and cross-spec consistency review.

SpecBind task artifacts are also milestone-local. A dependency on another spec's positional Task ID would become dangling when that spec revises its plan, completes a different milestone, or removes `tasks.yaml` during release finalization. Supporting such references would require a cross-artifact task scheduler, task identity stable across milestones, and downstream invalidation rules that are not otherwise needed.

## Decision

- Every Task ID and every `depends_on` entry in `tasks.yaml` is an unqualified positional reference resolved only within that same file.
- `depends_on` cannot name a task in another spec. Qualified values such as `other-spec/1.2`, `other-spec:1.2`, or paths into another task artifact are invalid.
- The CLI derives a task dependency graph from exactly one spec's current `tasks.yaml`; it does not load other task artifacts to decide whether a local task is actionable.
- Active cross-spec ordering is represented at spec/change granularity in the milestone `roadmap.md`.
- Persistent cross-spec dependencies on observable seams are represented by `contract.md` `Consumes` references and reviewed through the contract graph.
- A dependency on already released behavior is established from the current contract plus changelog and immutable release evidence, not from historical Task IDs.
- If a task cannot proceed until work in another active spec completes, the workflow updates the roadmap dependency and the relevant contract or design context. A local `blocked_reason` may explain the immediate wait, but it is not the authoritative cross-spec dependency edge.
- Migration treats an inherited `_Depends:_` value that is not an unqualified local numeric Task ID as ambiguous input. It reports the source text and requires routing to roadmap, contract, or ordinary prose instead of stripping qualifiers or guessing a local Task ID.

## Consequences

- Task status and actionability remain deterministic from one milestone-local artifact.
- Positional Task IDs do not need lifecycle or namespace guarantees beyond their own `tasks.yaml`.
- Cross-spec scheduling survives task-plan replacement because its authoritative edge is not tied to an ephemeral Task ID.
- Fine-grained coordination may still be explained in design, task details, or implementation notes, but only roadmap and contract edges carry cross-spec machine semantics.
