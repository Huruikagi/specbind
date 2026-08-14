# 0005: Keep abandonment separate from release history

Status: Accepted

## Context

An active milestone may lose scope, be abandoned entirely, or require selected implementation work to be rolled back before release. These cases are different from reversing behavior that has already shipped.

SpecBind artifacts describe the intended and current product state, while Git remains the authoritative mechanism for restoring code and document content. Automatically reverting repository changes would be unsafe when work is shared across specs, commits, or unrelated working-tree edits.

## Decision

- SpecBind will not introduce a dedicated `specbind-cancel` skill.
- Removing unstarted work from an active milestone is a normal milestone-scope revision.
- Rolling back partially implemented, unreleased work is explicit project work performed with version-control operations. SpecBind does not automatically revert code or specification content.
- After a partial rollback, the active roadmap, briefs, current requirements and design, tasks, and `spec.json` state must be reconciled with the repository state before work continues.
- Reversing released behavior is a new active milestone change and follows the normal specification and release workflow.
- Abandoning an entire unreleased milestone is an explicit, user-confirmed lifecycle operation. It may close active-change metadata and remove milestone-local `brief.md`, `tasks.yaml`, and `steering/roadmap.md` only after affected repository and active-spec content has been restored or reconciled.
- An abandoned unreleased milestone is not added to per-spec `changelog.md` and its roadmap is not archived under `releases/` by default. Git history remains available when the abandoned work was committed.

## Safety rules

- Never infer full-milestone abandonment from a new discovery request.
- Never discard working-tree changes or rewrite Git history as part of SpecBind lifecycle cleanup.
- Never clear active-change metadata while requirements or design still describe abandoned behavior.
- Stop for explicit reconciliation when retained changes overlap abandoned scope or ownership is ambiguous.
- Keep released rollback distinct from unreleased abandonment so the changelog remains an index of actual releases.

## Consequences

- Scope editing, content restoration, and lifecycle cleanup remain distinct operations.
- Release archives contain released milestones rather than cancelled drafts.
- The discovery entry point does not need to own Git rollback behavior.
- Rust CLI milestone operations can own roadmap state changes without absorbing discovery analysis; see [Decision 0009](./0009-milestone-cli-boundary.md).

## Open questions

- Whether projects that require an audit trail should be able to opt into a separate abandoned-milestone record outside `releases/`.
