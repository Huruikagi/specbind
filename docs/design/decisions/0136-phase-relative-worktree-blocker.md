# 0136: Report worktree cleanliness only when it blocks current progress

Status: Accepted

## Context

Discovery and every authoring or implementation phase normally leave intended
project changes in the worktree. `milestone status` nevertheless reported
`WORKTREE_NOT_CLEAN` under `Release blockers` immediately after Discovery. That
made healthy current work look like a fault and described repository-wide
cleanliness as a release guard even though release finalization applies
path-scoped Git safety.

A clean committed revision is still required when it establishes
implementation completion for a dependency or starts final implementation
validation. That guard must remain visible when it actually prevents the next
workflow action.

## Decision

`milestone status` reports `WORKTREE_NOT_CLEAN` as a current blocker only when
the same milestone state at a clean revision would advance the derived stage or
unlock an additional action. Ordinary dirty work during Discovery, Requirements,
Design, Tasks, or ongoing Implementation is not a blocker.

Before the milestone reaches Validation, status renders:

```text
Release readiness: not evaluated until validation
```

It does not render the future release-blocker list. When work is otherwise ready
for a clean-revision boundary, status instead also renders:

```text
Current blockers: WORKTREE_NOT_CLEAN
Worktree action: review and commit or otherwise reconcile current changes to continue
```

Once the milestone reaches Validation, `Release blockers` is rendered as the
release-readiness projection. Repository-wide `WORKTREE_NOT_CLEAN` is not one
of those release blockers; release preflight and finalization retain their
existing accepted Git and path-safety guards.

The completion handshake remains strict. This display change grants no commit,
stash, discard, or other Git mutation authority.

## Consequences

- Discovery output no longer presents its own authored files as a release fault.
- A missing clean commit becomes visible at the point where it blocks dependency
  progress or Validation.
- Agents can distinguish current workflow blockers from later release
  prerequisites without weakening completion or release safety.

## Implementation status

Implemented in the milestone read model and text renderer. CLI coverage proves
the dirty Discovery, dirty clean-revision boundary, and clean Validation views.
