# 0217: Keep owning workflows active through internal dispatch

Status: Accepted

## Context

The Claude Code forward tests on `49c8da0` exposed FT-0057: an implementation
owner launched an internal role in the background, then returned only a waiting
message. Drive resumed it repeatedly or needed a maintainer continuation while
unrecorded Task changes remained. Decisions 0109 and 0192 give the owner the
whole workflow, but do not distinguish background progress from its handoff.

## Decision

An owning workflow keeps its turn active until every internal role it started
returns and the workflow reaches its normal stopping point. Foreground dispatch
is preferred. Background execution changes how the host delivers the result,
not who must collect it, run review, record progress, or apply the checkpoint.
Progress alone is never a terminal owner result and requires no user message.

Drive waits for the complete owner's terminal handoff before selecting another
owner or interpreting lifecycle progress. If an addressable owner nevertheless
ends its turn with a non-terminal update, Drive continues that same receiver
once for the same workflow. It does not change authority, replenish remediation
budgets, start a replacement over partial work, or perform the internal roles.
If the receiver is unavailable or returns another non-terminal response, Drive
rereads Git and milestone status once, records `EXTERNAL_BLOCK` with receiver
identity, last progress, partial paths, and any known live receiver, and stops.
This bounds recovery from a broken handoff without treating elapsed waiting as
failure or cancelling legitimate work.

Explicit user stops and actual host failures still interrupt execution. A
workflow may stop at a required input or exhausted retry boundary, reporting
its partial state and any live receiver. Decision 0214 continues to govern
capacity failures; a capacity restart capsule is already a terminal handoff
and must not be mistaken for a progress update.

## Consequences

- Implementation retains review, Task state, and checkpoint ownership.
- Drive cannot replace a receiver that may still be writing the same worktree.
- No new persistent queue, CLI state, platform-specific dispatch syntax, or
  approval authority is introduced.

## Verification

Skill conformance checks cover both owner waiting and the bounded same-receiver
recovery. Fresh DR2 and DR6 runs must reach validation in the initial turn with
real internal dispatch, per-Task reviews and checkpoints, and no release. The
run record must retain any non-terminal return even if recovery succeeds.
