---
type: SpecBind Git Adapter
---

# Git adapter

This file is project policy for local workflow checkpoints. The sections below
contain a working default. Replace them when the project wants different commit
grouping, messages, or publication behavior.

This file sets policy, not permission. It cannot grant authority SpecBind does
not otherwise have, and it never changes when work becomes eligible to commit:
approvals, task completion, and repository safety stay SpecBind's contracts. A
checkpoint never includes unapproved, rejected, or unrelated work.

Emptying or removing this file means SpecBind commits nothing on its own.

## When to checkpoint

Create one local commit after each eligible workflow unit: completed Discovery,
each approved Requirements, Design, or Tasks gate, an accepted Contract review,
and each completed implementation Task. Keep completion metadata in its own
checkpoint when the completion workflow requires one.

## What to include

Stage only the paths produced by that workflow unit. Leave unrelated work in the
worktree exactly as it is. If the intended paths cannot be separated safely,
stop before committing and report the accepted work as uncommitted.

## Commit messages

Use a concise, outcome-oriented summary in the project's language. Name the
Spec or Direct item when that makes the checkpoint easier to identify. Do not
amend, rebase, squash, or otherwise rewrite history by default; record a later
correction as an additional commit.

## Branches and pushing

Stay on the current branch. Do not create or switch branches solely for a
checkpoint. Do not push unless the user explicitly requested it for the current
run or an applicable project instruction separately requires it. Never
force-push or bypass a protected branch.
