# 0054: Bind contract diffs to the milestone baseline revision

Status: Accepted

## Context

Cross-spec review needs a stable before-state for contract diffs. Using the Git revision at the start of each review would shrink later reviews to changes since the previous attempt instead of preserving the complete milestone delta. Using a globally latest release is also ambiguous for hotfix branches, worktrees, and milestones that begin from an older release line.

The correct baseline is the repository snapshot immediately before the active milestone is created. It is branch-local, available before a target release name is known, and stable across every review and repair attempt in that milestone.

## Decision

- `SpecBind Roadmap` frontmatter requires `baseline_revision` immediately after `milestone_id`.
- `baseline_revision` is the full lowercase hexadecimal Git commit object ID resolved from `HEAD` immediately before the CLI creates the roadmap. It uses the project-scoped 40- or 64-character representation rules accepted by Decision 0031.
- Milestone creation requires a Git repository, an existing commit, and a clean repository state before the roadmap mutation: no staged changes, tracked worktree changes, untracked files, or dirty submodules. Ignored files do not make the repository dirty.
- Contract diff before-state is read from `baseline_revision`; after-state is the current active contract set at cross-spec review time. Every rerun therefore evaluates the complete cumulative milestone delta.
- The baseline is branch-local. A mainline, hotfix, release, or worktree milestone records the commit from which that particular milestone began rather than resolving a globally latest release.
- Normal milestone scope changes, target-release binding, document edits, implementation commits, and review reruns never rewrite `baseline_revision`.
- Cross-spec review requires the baseline commit to exist in the same repository and to be an ancestor of the current `HEAD`. A missing, foreign, abbreviated, symbolic, or non-ancestor revision is invalid.
- Rebaselining is an explicit, user-confirmed CLI operation, never an inferred repair. It requires a clean repository and an explicit full commit object ID, validates that commit as an ancestor of current `HEAD`, replaces `baseline_revision`, and removes the accepted `state/cross-spec-review.md`. [Decision 0089](./0089-milestone-creation-cli.md) fixes its command syntax.
- For a roadmap `new_specs` item, absence of its contract at the baseline is the expected before-state and the complete current contract is treated as newly added.
- For a `spec_updates` item, a contract missing at the baseline is a migration or consistency failure rather than an implicit empty contract. The existing-spec bootstrap or missing-contract fallback must be resolved before normal Contract-first review can pass.
- The normalized `steering/roadmap.md#cross-spec-scope` input projection accepted by Decision 0055 includes `baseline_revision`, so an explicit rebaseline makes prior review evidence stale even if all current contract files are byte-identical.

## Consequences

```yaml
type: SpecBind Roadmap
milestone_id: 0198b2d1-7c4a-7e31-9f42-8e7c3a110d62
baseline_revision: 0123456789abcdef0123456789abcdef01234567
target_release: null
work_items:
  spec_updates:
    - spec: checkout
      summary: Require authenticated checkout
```

- The phrase "prior released reference" no longer selects the contract-diff baseline. A prior release and the milestone baseline commonly coincide, but the baseline field is authoritative.
- Hotfix review remains isolated from unrelated newer mainline releases.
- Review retries cannot accidentally erase earlier same-milestone contract changes from the comparison.
- The single additional roadmap scalar is routinely visible, while detailed diff evidence and AI judgment remain outside always-loaded steering context.
