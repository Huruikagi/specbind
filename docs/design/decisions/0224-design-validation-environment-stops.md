# 0224: Stop Design validation when required investigation cannot execute

Status: Accepted

## Context

FT-0067 reproduced a validator returning `READY` after its configured researcher
failed to start and it substituted its own investigation. The existing model
fallback prohibition did not explicitly address manual investigation. The
maintainer chose to stop failed required investigations without preventing
small local context reads chosen before dispatch.

## Decision

Once Design validation dispatches required independent investigation, it must
receive the result before judging readiness. Startup or execution failure is
`STATUS: BLOCKED`, `BLOCKER: ENVIRONMENT`, with the failed role/operation and an
explicit statement that no Design verdict was completed. Do not substitute
local investigation, another role or model, issue `READY` or `NOT_READY`, write
deferred findings, or mutate artifacts or lifecycle state.

A small bounded context read can remain local when no independent investigation
was needed. Do not reclassify a failed dispatch as that path after the fact.
An absent named-role capability still permits the documented ordinary fresh
subagent path before dispatch; a configured role failing to start does not.

Plan treats the environment stop as neither a blocking Design finding nor an
authoring revision. Preserve the finding ledger, revision budget and bounded
dirty handoff, and resume independent validation after repairing the execution
environment. Approval still requires an actual `READY` result.

This refines the semantic verdict language of Decisions 0114 and 0194 without
adding a persisted verdict or changing the lifecycle schema. An insufficient
Design still receives `NOT_READY`; inability to execute review has no verdict.

## Verification

Check both installed agent renderings and the Plan receiver boundary. Run the
local Design scenario with working investigation capability, and a separate
fixture with an intentionally unavailable configured researcher. The latter
must stop without substitution, verdict, artifact mutation or gate changes.
Retain environment failures and fixture setup separately from product results.
