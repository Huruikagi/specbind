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
