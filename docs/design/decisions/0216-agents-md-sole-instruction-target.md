# 0216: Make root `AGENTS.md` the sole project-instruction target

Status: Accepted

## Context

[Decision 0099](./0099-project-instruction-block.md) writes the marked SpecBind
block to `CLAUDE.md` for Claude Code and to `AGENTS.md` for Codex, because each
agent then read only its own file. Selecting Claude Code therefore created a
`CLAUDE.md` in projects that had none, and selecting both agents kept two copies
of one product-managed block.

Claude Code now reads root `AGENTS.md` when no `CLAUDE.md` or `CLAUDE.local.md`
exists on the working path. The premise of the per-agent split no longer holds
for the ordinary case.

## Decision

- Root `AGENTS.md` is the only file the installer maintains the block in, for
  every Agent. Selecting only `claude-code` creates or appends to `AGENTS.md`
  and never creates `CLAUDE.md`. The block is one shared target, so agent
  removal retains it while any selected Agent remains and only uninstall removes
  it.
- A valid block that an earlier release wrote to `CLAUDE.md` is retired.
  Claude Code ignores `AGENTS.md` while `CLAUDE.md` exists, so a retained copy
  would keep a stale block in front of the agent. `specbind install` with
  project instructions enabled and `claude-code` selected, `specbind
  remove-agent claude-code`, and `specbind uninstall` each remove it. The file is
  deleted when the block was its entire content; otherwise only the marked
  region is removed and every other byte is kept. Removing text is a Decision
  0077 replacement behind the committed clean repository guard, and the entry is
  shown in the plan. Malformed markers stop the operation as in Decision 0099.
- A `CLAUDE.md` that is a link is project wiring, commonly to `AGENTS.md`, and is
  never edited through. Editing it would strip the current block from its
  target.
- A project-owned `CLAUDE.md` without a block is left alone. Such a project
  imports the shared block with an `@AGENTS.md` line in `CLAUDE.md`; the
  installer never writes that line, because it lies outside the markers.
- Disabling project instructions still stops maintenance without removing
  anything, including a legacy `CLAUDE.md` block.

## Consequences

- A Claude-only installation adds one conventional file instead of a
  Claude-specific one, and a mixed installation keeps a single copy of the block.
- Projects upgraded from an earlier release lose the duplicate `CLAUDE.md` block
  on their next install. When that file held only the block, Claude Code falls
  back to `AGENTS.md` automatically.
- A project that keeps its own `CLAUDE.md` must import `AGENTS.md` for Claude
  Code to see the block. The install guide states this; the product does not
  detect it.
- Claude Code environments without `AGENTS.md` support do not see the block
  unless the project imports it.

## Implementation status

Implemented. `project_instructions::TARGET` is the single target and
`legacy_targets` names `CLAUDE.md` for Claude Code. Install planning retires a
legacy block as `remove` or `replace`, and removal planning emits an `update` or
`remove` entry only when a legacy block is present. Tests cover Claude-only and
mixed installs, legacy retirement of block-only and mixed files, a linked
`CLAUDE.md`, and legacy removal through `remove-agent` and `uninstall`.
