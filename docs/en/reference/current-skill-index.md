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
| `sb-drive` | Advance all safely reachable work in the Milestone and stop before release. Resolved `replan` also delegates in-scope replanning. |
| `sb-status` | Show the current state and the next available action. Read-only. |

Drive reads `.specbind.json` `drive.replan` and `drive.recovery` at invocation.
Fresh installs save `true` and `progress`; existing omitted fields use `false`
and `bounded`. Explicit options win, including `--no-replan` and
`--recovery bounded`. Drive passes resolved values and sources to its owners;
config edits do not alter an ongoing run or renew counters. See
[Drive defaults](../guide/customization.md#drive-defaults).

## Skills for specific situations

With Drive replan enabled by configuration or `--replan`, completion records made stale by that authorized
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

When invoked standalone, by default `sb-implement` permits at most three recovery cycles per
user-authorized execution or resumption,
including authorized replanning and corrected re-review. After a repair, it
continues only when independent assessment resolves every earlier finding and
the remaining blockers are new. A repeated or uncertain finding stops the Task;
any blocker after the third cycle also stops it. The budget follows the Task
across handoffs. After a recovery-limit stop is reported, explicitly asking to
resume that work grants a new allowance without a special reset phrase or
another confirmation. A contextual "continue" or "I updated it; resume" is
enough. The first cycle may retry carried blockers; later cycles retain the
same independent progress rules. Prior findings, failed approaches and evidence
are kept, and unknown historical counts remain unknown. Automatic redispatch,
session changes, Task remapping, or a version update alone never renew the
allowance. Scope, approvals, external-input requirements and verification are
unchanged. Direct-item recovery keeps its
separate existing bounds.

To continue past three cycles while independent assessment proves progress,
select `--recovery progress` on `sb-implement` or `sb-drive`. Standalone
implementation defaults to `--recovery bounded`; Drive uses its project default.
These are Skill arguments, not CLI flags. An explicit natural-language request for no cycle limit while progress
continues is equivalent; a generic "continue" does not select progress mode.
Every prior finding must be resolved before another cycle; partial improvement,
reopened findings or uncertain mappings still stop recovery. Counts and history
are retained across handoffs. Selection applies only to that execution or
resumption; a new Drive invocation rereads its defaults without discarding history.
Direct-item, Design and Contract Review bounds remain unchanged.

| Skill | When to use it |
| --- | --- |
| `sb-gap-analysis` | Before planning, compare the intended change with the current code and keep the findings as Research. |
| `sb-validate-design` | Independently check a Design. `sb-plan` runs it automatically after authoring a Design. |
| `sb-review-task` | Review one implemented Task from its diff, without fixing anything. |
| `sb-debug` | Find and classify the root cause when a run has stopped, without fixing anything. |
| `sb-verify-completion` | Check a "done" claim against fresh evidence without changing lifecycle state. |

If a required investigation for Design validation cannot run, the validator
stops with `BLOCKED / ENVIRONMENT` without a Design verdict. Repair the review
environment before resuming validation. Plan does not ask the author to revise
the Design or consume a revision attempt for this stop. A completed review that
finds the Design insufficient still returns `NOT_READY`.

## Temporary Skill

| Skill | When to use it |
| --- | --- |
| `sb-adopt` | Establish Specs from an existing implementation. Installed only with `specbind install --with-adoption` and removed automatically when establishment completes. See [Establish Specs from an existing implementation](../guide/adopt-existing.md). |

## Codex metadata

Codex installations also receive `.agents/skills/<skill>/agents/openai.yaml`,
which gives each Skill a display name such as `SpecBind Plan`, a short
description, and an example prompt for the Codex UI. It does not change how
Skills are invoked.
