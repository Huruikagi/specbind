# 0165: Preserve completion freshness across release binding

Status: Accepted

## Context

Completion evidence names the exact Git revision whose implementation was
validated. The freshness evaluator already permits the later `spec.yaml`
transition that records that evidence, but otherwise treats every path changed
after the implementation revision as a reason to revalidate.

That rule makes a release label behave like implementation. Binding or
rebinding writes only the active Roadmap's `target_release`, so a Spec already
at `release_ready` becomes stale even though Requirements, Design, Contract,
Tasks, implementation, and verification commands are unchanged. Repeating the
completion handshake produces the same semantic judgment at a commit that
differs only by release identity.

Decision 0072 already excludes `target_release` from Spec-local Gate inputs and
says release binding does not invalidate completion evidence. Decisions 0115
and 0119 documented the operational revalidation cost instead because the Git
freshness contract had no way to recognize that exact metadata transition.
The resulting user workflow is expensive and the accepted contracts disagree
about whether binding is completion-invalidating.

## Decision

### Recognize a closed set of evidence-preserving transitions

Completion remains bound to its persisted `implementation_revision`. A later
checkout is fresh only when every tracked difference from that revision is one
of these structurally proven transitions:

1. a participating Spec's existing `implementation` to `release_ready`
   transition with completion evidence bound to that implementation revision;
2. an initial bind or explicit rebind that changes only the active Roadmap's
   `target_release` to one valid non-null release label.

The second transition is recognized only at the configured SpecBind root's
exact `steering/roadmap.md` path. Both the baseline and current Roadmap must be
valid. Reapplying the ordinary `bind-release` transformation to the baseline
with the current label must reproduce the current file byte for byte. This
proves that the milestone identity, baseline revision, work items, dependencies,
Direct statuses, extensions, and Markdown body did not change alongside the
label.

The check applies to committed history and to the current worktree. A dirty
Roadmap containing only the exact binding transition does not stale a Spec's
completion, but whole-milestone release readiness still requires the ordinary
clean-worktree guard. The binding therefore needs a normal project checkpoint
before release preflight can pass.

The release-binding exception is not admitted by the same-revision guarded
acceptance check for another Spec. Completion acceptance still permits only the
other participating Specs' pending completion transitions. A binding must be
checkpointed first; a later Spec may then validate at the new `HEAD` while the
earlier Spec's evidence remains fresh at its original implementation revision.

No path category, filename, commit message, or claim that a change is
"metadata" grants an exception. Any current Roadmap scope or body edit, project
policy, Steering, Requirements, Design, Contract, Tasks, implementation,
configuration, or unrelated repository change continues to stale earlier
completion evidence. Any other path appearing in later commit history remains
stale even when its current bytes were reverted.

### Keep release authority and release guards unchanged

Initial binding still requires a user-supplied opaque release label. Replacing
a non-null label still requires the explicit `--rebind` operation and the
existing user confirmation in an agent-assisted workflow. Archive collision,
active-milestone, target-path, clean-worktree, publication, verification, and
finalization guards are unchanged.

The exemption says only that release identity is not an implementation input.
It does not make a release ready, authorize publication, or preserve completion
across release-specific source changes made by Prepare guidance.

### Update orchestration and convergence language

`specbind-release` no longer warns that binding late spends accepted completion
evidence. After a successful binding mutation it follows the Git adapter for one
narrow checkpoint containing only the Roadmap. Without active checkpoint policy
it stops and reports the dirty Roadmap; it does not proceed to release work.

Final validation converges on one implementation state rather than requiring
every completion record to contain the same raw Git object ID. Different
`implementation_revision` values are acceptable only when the complete
difference is the closed evidence-preserving transition set above.

The general writing caution remains. Ordinary managed-Markdown authoring is not
a release-binding transition and still stales accepted completion evidence.

## Superseded statements

- This decision narrows Decision 0080's rule that every later non-metadata
  project commit stales completion.
- It narrows Decision 0082's same-`HEAD` convergence wording to convergence
  modulo the recognized metadata transitions.
- It supersedes Decision 0115's "Binding late invalidates completion" section
  and its associated revalidation warning.
- It adds the exact release-binding exception to Decision 0119's general rule
  for writing while completion stands.
- It implements Decision 0072's existing semantic statement that binding and
  rebinding do not invalidate completion evidence.

All other boundaries and consequences of those decisions remain accepted.

## Consequences

- A maintainer may choose the release identity after one or more Specs reach
  `release_ready` without rerunning their completion validation.
- Completion evidence continues to identify the exact implementation that was
  tested; no evidence is rewritten or silently advanced to the binding commit.
- Rust, not an agent or path heuristic, proves the only newly tolerated
  successor state.
- Release binding still creates one ordinary metadata checkpoint before a clean
  release preflight.
- Broader project changes retain the conservative whole-project freshness
  guarantee.

## Verification

Focused Rust tests cover pending and committed initial binding, committed
rebinding, and rejection when a Roadmap body change accompanies the label.
Release forward scenario RL1 starts with accepted completion and no version,
then proves that a user-supplied binding preserves completion freshness and
reaches clean release readiness without another completion handshake.

## Implementation status

Implemented in the completion freshness evaluator, release and implementation
validation Skills, the `okf-authoring` protocol, the release guide, and RL1.
