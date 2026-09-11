# 0214: Preserve dispatch-capacity handoffs through Drive

Status: Accepted

## Context

Decision 0213 defines recovery when Plan has started and one of its internal
roles cannot be dispatched. Drive remains the outer controller: it dispatches a
complete owning Skill, normalizes the result, rereads authoritative state, and
reports one accumulated handoff.

Two capacity boundaries remain. The owning Skill may return a restart handoff
whose finding ledger, revision budget, and supplied authority are not durable
CLI state. Drive's compact attention record could lose that information. Or the
host limit may reject the owning Skill before it starts, leaving no owner result
to normalize at all.

## Decision

Drive treats an owning Skill's capacity restart handoff as owner-owned
continuation state. It preserves the handoff fields exactly in its attention set
and final report, then appends its independent `git status --short` and
`milestone status --json` evidence. It does not paraphrase away paths, finding
IDs or dispositions, used or remaining budgets, supplied or omitted authority,
the requested continuation role, or project execution facts. Mechanical rereads
may contradict and block the handoff; they never reconstruct its non-durable
facts.

When finite host capacity prevents a `handler.kind=skill` owner from starting,
Drive:

1. consumes completed Drive-owned receiver results first and uses a safe host
   release operation only when that operation exists, the result was consumed,
   and the receiver will not be continued;
2. continues an addressable receiver only for the same owner, action, item, and
   handler mode when that already-started run lacks its terminal status;
3. otherwise records `EXTERNAL_BLOCK`, rereads Git and milestone state once,
   and does not retry the unchanged dispatch in that run.

The owner-start restart capsule names the exact handler target and mode, action
and command operand, item or milestone, supplied authority, project working
directory and executable/PATH facts, capacity evidence, Git state, and the
fresh owning Skill required to resume. Drive invents no finding ledger or retry
budget when an owner never started. It never releases unrelated receivers,
changes the owner, performs owner work in the controller context, or asks the
user to manage receiver slots.

A returned capsule with unapproved or partial paths keeps the shared worktree
unsafe and stops Drive after the authoritative reread. A clean owner-start block
may allow only another safe actionable entry whose handler does not require the
unavailable receiver capacity. A later Drive run must receive any non-durable
restart capsule needed by the owner; status alone is not evidence for omitted
finding, budget, or authority fields.

This adds no persisted queue, capacity schema, host-specific release command,
or authority. Capacity recovery inside `sb-plan`, `sb-implement`, or another
owner remains that owner's contract; Drive only preserves its returned state.

## Consequences

- A nested Plan stop remains resumable after crossing the Drive boundary.
- Failure before owner startup has a precise restart capsule without pretending
  that owner-specific work began.
- Drive retains sequential ownership and does not become a phase, implementation,
  review, or validation fallback.
- Other owning Skills still need their own internal capacity contract when they
  dispatch multiple roles.

## Verification

Skill conformance tests require lossless nested handoff handling and the
owner-start capsule. A fresh Drive forward test saturates capacity once before
owner startup and once after an unapproved Design reaches Plan, verifying the
clean and dirty stop boundaries separately.
