# 0137: Install an active default for local Git checkpoints

Status: Accepted

[Decision 0140](./0140-release-adapter-bootstrap-and-finalization-checkpoint.md)
adds two eligible Release units: the confirmed one-time adapter bootstrap and
the lifecycle metadata produced by successful core finalization.

[Decision 0138](./0138-dedicated-adapter-scaffold-marker.md) supersedes the
legacy `specbind:instruction` compatibility behavior. Inactive adapter state now
uses only the exact dedicated scaffold marker.

## Context

Decision 0101 installs `settings/adapters/git.md` as an inactive scaffold.
Until a project replaces its `specbind:instruction` comment, every phase leaves
its accepted work uncommitted. Most projects first try SpecBind without editing
its settings, so the inactive scaffold makes the least configured experience the
least recoverable one. It also prevents a later same-session scope update from
crossing a clean-revision boundary after Discovery created the active Roadmap.

An accepted phase already has a precise eligibility boundary. Discovery waits
for the guarded milestone mutation and every owed Brief; Requirements, Design,
and Tasks wait for their guarded approvals; Contract review waits for accepted
review state; implementation waits for its required review or verification and
recorded progress. A local commit at those boundaries preserves a coherent
recovery point. If later work corrects it, an additional commit is ordinary Git
history; an explicitly requested safe amend remains available outside the
default.

## Decision

The embedded Git adapter is active project policy rather than an authoring
scaffold. It contains no `specbind:instruction` marker and directs product
skills to:

- create one local checkpoint after each eligible workflow unit;
- stage only paths produced by that unit, preserving unrelated work;
- use a concise outcome-oriented commit message;
- remain on the current branch; and
- neither push nor amend, rebase, force-push, or otherwise rewrite history by
  default.

For implementation, one completed Task is the normal checkpoint unit. A later
completion-metadata checkpoint remains separate where the completion contract
requires it. Accelerated planning may combine gates only where its owning skill
already defines one orchestration checkpoint.

Requesting a mutating SpecBind workflow authorizes this narrow local checkpoint
as an ordinary final step of that workflow. The adapter still grants no broader
authority by existing: an explicit user or root instruction can forbid commits,
tool permissions still apply, and push, branch creation or switching, tags,
publication, and history rewriting require their own authority. A delegated
gate approval does not widen any of those boundaries.

Missing `git.md`, a body with no guidance, or a legacy installed copy that still
carries `specbind:instruction` means no adapter-directed commit or push. This
preserves an explicit project choice and avoids rewriting existing project-owned
settings. `specbind install` writes the active default only when the file is
absent; an install refresh never replaces an existing copy.

## Consequences

- A newly installed project gets durable checkpoints without first customizing
  an operational setting.
- Corrections normally appear as follow-up commits instead of making accepted
  work disappear from history.
- Projects that want larger checkpoint groups or no automatic commits can edit,
  empty, or remove their project-owned adapter.
- Push and history mutation remain opt-in operations with separate authority.
- Existing project-owned Git adapters retain their current behavior. A project
  with no Git adapter receives the active default on its next install.

## Implementation status

Implemented by the localized embedded Git adapters, every checkpoint-consuming
product skill, adapter-state tests, user documentation, and the checkpoint
forward-test scenario. A fresh behavioral measurement is recorded in the
forward-test ledger.
