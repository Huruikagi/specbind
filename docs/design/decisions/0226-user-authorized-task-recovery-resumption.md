# 0226: Renew Task recovery on an explicit user resumption

Status: Accepted

[Decision 0227](./0227-progress-gated-task-recovery.md) adds explicit selection
of progress mode for an execution or resumption. Three cycles remain the default.

## Context

Issue #83 exposed a Task remaining blocked after an explicitly requested retry
because Decision 0218 retained its exhausted recovery budget. The maintainer
chose three cycles per user-authorized execution or resumption while retaining
all prior evidence and the bounds on autonomous continuation.

## Decision

After a Task's recovery-limit stop has been reported, a later human instruction
clearly asking to resume that stopped work authorizes one new allowance of at
most three cycles for that Task's existing recovery identity. Ordinary wording
such as "continue" or "I updated it; resume" suffices in that context. Require
neither a special reset phrase nor another confirmation. A read-only request to
inspect readiness is not execution authority; an instruction to check and
proceed when possible is. Resolve ambiguity about the target or requested action
before corrective work.

The owner records the allowance identity, the actual user instruction and
stopped Task it covers, current used/remaining cycles, and historical counts
separately. Apply a given resumption instruction once per recovery identity;
handoffs transport the resulting allowance and cannot consume the instruction
again. An ongoing run does not gain another allowance from a progress message.
Automatic redispatch, a new session, compaction, a Task split/rename, replan, or
a version update alone never grants one. Mapped descendants share the allowance.

Preserve all available prior finding IDs, dispositions, failed approaches,
verification evidence, partial paths and authority. Do not erase history or
declare blockers resolved. An unknown historical total, including legacy
diagnosis counters, stays unknown; it alone does not prevent a fresh allowance
based on an evidenced human resumption. Missing current Task identity, safe
worktree attribution, finding continuity, or governing inputs still stops
action. Without a new qualifying instruction, missing history never implies
zero used cycles.

An independent initial assessment of the resumed work establishes the current
baseline. A carried unresolved or reopened finding may authorize the first
corrective cycle; do not stop merely because it already occurred in a previous
allowance. After a corrective cycle in the new allowance, Decision 0218's
progress rule applies: unresolved or reopened findings stop recovery; only all
prior findings resolved plus materially new blockers permit another cycle.
After the third cycle any blocker stops the Task. Retain failed approaches as
factual evidence, not an invitation to repeat unsupported attempts.

The initial assessment plus at most three reassessments applies separately to
each allowance. Existing cycle reservation, preparation-failure accounting,
independent review, normal verification and checkpoint requirements remain.
User resumption grants no new scope, Gate approval, credential, external input,
release permission, or exception to host restrictions. Direct-item, Design and
Contract Review budgets are unchanged. There is no new CLI flag, configuration,
schema, or recovery file; this is conversation handoff data.

Implementation owns the ledger. Drive passes the qualifying user instruction
and prior handoff to the implementation owner even without `--replan`. Plan
transports the allowance and returns mapping and evidence; it cannot renew the
allowance or resolve implementation findings itself. This supersedes only
Decision 0218's unconditional prohibition on renewal by a continuation request.

When the reported Task is recorded blocked and has no ordinary actionable
entry, the evidenced resumption permits dispatch of its complete implementation
owner only. The owner verifies prerequisites and an independently assessed safe
corrective route, reopens that Task through the CLI, and rereads eligibility.
Reopening for retry does not assert that its finding is resolved. Preserve the
same Task's attributable partial work; unknown or conflicting changes, waiting
dependencies and unrelated blocked Tasks remain excluded.

## Verification

Check both agent renderings and continuation handoffs. Fresh forward tests must
distinguish contextual human resumption from automatic continuation and update
alone; preserve unknown historical counts; permit retry of carried blockers;
stop repeated findings after the new cycle; and prove a resumed corrective
review reaches independent review and normal Task completion without another
confirmation or fourth cycle.
