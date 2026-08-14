# 0064: Guard only SpecBind finalization target paths

Status: Accepted

## Context

Release practices vary substantially between projects. Some require a globally clean repository and an immutable tag; others intentionally release from a workspace containing unrelated local changes or use an external system with different revision semantics. Making SpecBind persist a release-source revision or enforce repository-wide cleanliness would duplicate project release policy and interfere with valid adapters.

The core CLI still needs to avoid overwriting, deleting, or moving uncommitted SpecBind state during finalization. That safety requirement is narrower and deterministic because the CLI can resolve its exact mutation set before writing.

## Decision

- SpecBind does not persist or require a core `release_source_revision`, immutable tag, Release URL, deployment ID, or other publication reference. Projects may create and record those references through their release adapter when useful.
- Release preflight and finalization do not require the entire Git working tree to be clean and do not reject a candidate merely because `HEAD` advanced during project-specific release work.
- Immediately before finalization, the CLI resolves the complete set of paths it will create, modify, delete, or move. This set includes, as applicable:
  - each participating spec's `spec.yaml`
  - each participating spec's `log.md`
  - each discovered active `SpecBind Brief`
  - each participating spec's fixed `tasks.yaml`
  - `steering/roadmap.md`
  - the matching `state/cross-spec-review.md`
  - the version-prefixed roadmap and cross-spec-review archive destinations
- Every resolved finalization target path must be safe relative to Git state:
  - every required existing source is tracked
  - an existing source or modified file has no staged or unstaged change relative to `HEAD`
  - an expected-absent destination has no conflicting untracked file
  - an existing retry destination is clean and passes the existing identity-and-content idempotency check
- Dirty or untracked files outside the resolved finalization target set do not block core finalization. A project adapter or repository instruction may impose a stricter repository-wide policy, but that is not a SpecBind core invariant.
- The CLI still revalidates current lifecycle state, gate freshness, roadmap membership, release binding, accepted cross-spec review, task completion, archive collision rules, and all other deterministic finalization guards against the current artifact contents.
- A target-path conflict returns a path-specific diagnostic and performs no mutation. The CLI never resets, stashes, stages, commits, or discards user changes. The agent or user resolves the affected path explicitly and retries.

## Release history and references

- Git history normally retains the pre-finalization Brief and `tasks.yaml`; a project-created tag or external release reference may make that state easier to locate but is not required by SpecBind.
- `log.md` may mention a project tag, Release URL, deployment identifier, or relevant commit when useful. None is a mandatory structured field in the core log profile.
- Removal of the mandatory immutable-reference concept does not decide the remaining release-evidence schema. The finalization boundary may still require concise evidence that applicable project and core verification succeeded, without requiring a universal publication identifier.

## Consequences

- SpecBind protects every file it mutates without taking ownership of unrelated repository work.
- Projects remain free to choose tags, release commits, external identifiers, global clean-tree checks, or looser publication workflows.
- Finalization can coexist with unrelated dirty files while refusing to destroy uncommitted SpecBind lifecycle state.
- Release evidence can focus on successful checks instead of forcing one revision model across every project.
