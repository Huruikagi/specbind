# 0225: Surface project-owned Claude instruction import checks

Status: Accepted

## Context

Issue #82 showed a successful refresh retiring the legacy SpecBind block from
`CLAUDE.md` while retaining project text. Decision 0216 intentionally leaves
imports to the project, but an all-keep plan and enabled configuration can hide
that the project still needs to arrange loading of `AGENTS.md`. The maintainer
chose an attention message rather than automatic wiring.

## Decision

When project instructions are enabled and `claude-code` is selected, `install`
(including dry run and no-change results) and `configuration show` report a
non-blocking reminder for each root `CLAUDE.md` or `CLAUDE.local.md` present.
Install reports the planned resulting state: a block-only legacy file being
deleted needs no reminder; a retained file does. Previously refreshed projects
with no legacy block receive the same reminder.

Ask the user to verify that the project loads `AGENTS.md`, for example through
`@AGENTS.md`. This is a presence-based check, not a claim that imports are
missing or that Claude Code actually loaded instructions. Do not parse imports,
follow links to inspect their content, or suppress the reminder based on a
literal import line. Links and custom project wiring may already work. An
inspection error reports uncertainty without preventing installation.

Never add an import, rewrite project text, or change existing retirement,
repository guards, success status, configuration, or file-action counts.
Disabled instructions and other agent selections do not receive this reminder.
Only root files are inspected; inherited and external agent settings are not
validated. This amends Decision 0216's no-detection consequence, while keeping
the project responsible for wiring.

## Verification

Exercise mixed-file retirement, block-only deletion, previously refreshed
files, local instructions, links, existing imports, disabled instructions and
other agent selections. Check dry-run, apply, no-change and configuration
output, preserved bytes, and unchanged action counts.
