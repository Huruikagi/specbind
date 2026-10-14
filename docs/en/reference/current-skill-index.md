# Skill index

This page lists the product-managed Skills that `specbind install` places in a
project. For the files they create and maintain, see the
[artifact index](./current-artifact-index.md).

## Where Skills live and how to invoke them

| Agent | Location | Invocation |
| --- | --- | --- |
| Codex | `.agents/skills/sb-*/` | `$sb-*` |
| Claude Code | `.claude/skills/sb-*/` | `/sb-*` |
| Generic | `.agents/skills/sb-*/` | Defined by the host Agent |

Every Agent receives the same 15 Skills. Codex and generic share
`.agents/skills/`, so selecting both installs each Skill once.

## Skills for the everyday workflow

Listed in lifecycle order. See [Core concepts](../guide/concepts.md) for how
they fit together.

| Skill | When to use it |
| --- | --- |
| `sb-configure` | Review or change project configuration, or update SpecBind. |
| `sb-steering` | Create or update durable project guidance (Steering). |
| `sb-discovery` | Start a change: confirm scope and create the Milestone and Specs. |
| `sb-plan` | Plan Requirements, Design, and Tasks for one Spec or the whole Milestone (`--all`), or prepare a shared Contract change (`--shared`). |
| `sb-contract-review` | Review every Contract in the Milestone together before Tasks. |
| `sb-implement` | Implement one Roadmap item, with review of each Task. |
| `sb-validate-implementation` | Validate a completed Spec against its Requirements and record completion on `GO`. |
| `sb-release` | Release the Milestone: bind the version, publish, verify, and finalize. |
| `sb-drive` | Advance all safely reachable work in the Milestone and stop before release. `--replan` also delegates in-scope replanning. |
| `sb-status` | Show the current state and the next available action. Read-only. |

## Skills for specific situations

With `sb-drive --replan`, completion records made stale by that authorized
replan can be withdrawn automatically. Drive verifies the before/after evidence
and delegates each withdrawal to `sb-validate-implementation`, which checkpoints
only that change. Drive then resumes remaining implementation and, once all
Milestone work is complete, schedules fresh validation. Pre-existing staleness,
other inconsistencies, missing history, or an uncommitted target file stop this
automatic recovery. Ordinary completion withdrawal still needs confirmation.

When several Specs are ready for final validation, Drive sends the explicit
set to one validation owner. It validates each Spec at the same committed
revision, accepts only after all pass, and commits their completion records
together so one Spec's acceptance does not stale another's.

For a Spec-backed Task, `sb-implement` permits at most three recovery cycles,
including authorized replanning and corrected re-review. After a repair, it
continues only when independent assessment resolves every earlier finding and
the remaining blockers are new. A repeated or uncertain finding stops the Task;
any blocker after the third cycle also stops it. The budget follows the Task
across handoffs and cannot be reset by restarting Drive. There is no setting
for this limit. Direct-item recovery keeps its separate existing bounds.

| Skill | When to use it |
| --- | --- |
| `sb-gap-analysis` | Before planning, compare the intended change with the current code and keep the findings as Research. |
| `sb-validate-design` | Independently check a Design. `sb-plan` runs it automatically after authoring a Design. |
| `sb-review-task` | Review one implemented Task from its diff, without fixing anything. |
| `sb-debug` | Find and classify the root cause when a run has stopped, without fixing anything. |
| `sb-verify-completion` | Check a "done" claim against fresh evidence without changing lifecycle state. |

## Temporary Skill

| Skill | When to use it |
| --- | --- |
| `sb-adopt` | Establish Specs from an existing implementation. Installed only with `specbind install --with-adoption` and removed automatically when establishment completes. See [Establish Specs from an existing implementation](../guide/adopt-existing.md). |

## Codex metadata

Codex installations also receive `.agents/skills/<skill>/agents/openai.yaml`,
which gives each Skill a display name such as `SpecBind Plan`, a short
description, and an example prompt for the Codex UI. It does not change how
Skills are invoked.
