# 0221: Clarify Drive validation authority, owner hosts, and revision-change stops

Status: Accepted

## Context

The Claude `3513446` forward tests exposed three gaps: Drive passes completion
authority while direct validation routing requires an explicit request
(FT-0061); a complete Skill owner can be hosted in its internal planner role
(FT-0062); and grouped validation stops on changed HEAD without naming its
restart owner (FT-0064). Correct final artifacts alone do not settle these
authority and handoff contracts.

## Decision

Amend Decisions 0168 and 0220 as follows.

A user request to Drive the active milestone includes ordinary completion
recording on GO and its Git-adapter checkpoint for the exact actionable Spec
set. Drive passes this authority to the validation owner without asking for an
additional completion confirmation. Explicit user restrictions on mutation or
completion recording take precedence. A read-only claim check, a status result,
or an unsupported agent assertion grants no such authority. Planning Gate
approvals, completion withdrawal, and Release keep their separate contracts.

Every `handler.kind=skill` owner runs in an ordinary fresh subagent, never a
registered internal role. Claude Code uses `general-purpose`; Codex uses its
ordinary/default agent. The owner loads the exact installed Skill and only then
dispatches its internal roles. An unavailable ordinary host is EXTERNAL_BLOCK,
not permission to substitute an internal planner, implementer, or reviewer.

FT-0073 clarified that fresh means no inherited dispatcher or prior-owner
conversation, not merely a newly named receiver. Drive explicitly selects the
host's no-history option and passes a self-contained brief with project inputs,
authority and required recovery evidence. If that isolation is unavailable,
the route stops with EXTERNAL_BLOCK. This includes replan and withdrawal owners.
A terminal handoff ends that receiver's assignment: the next workflow gets a
new receiver even for the same Spec or Skill. Follow-up is limited to collecting
an unfinished workflow with the same Skill, action, item and mode before its
terminal handoff. Terminal WAITING for milestone convergence is not an internal
role still running. Recovery budgets and authority survive in the explicit
handoff; no conversation inheritance or receiver reuse is needed to retain them.
Limiting a Drive request to withdrawal and its checkpoint narrows the action
set, not the owner boundary. The controller still dispatches the isolated
validation workflow; reading that Skill does not turn Drive into its owner.

After all validation targets earn GO, the owner checks that HEAD still equals
the common preflight revision and the worktree is clean. A mismatch before
acceptance accepts nothing and returns BLOCKED with the entire target set,
preflight revision, observed HEAD, dirty paths, failed check, and restart
instruction. It must not repair, commit, clean, or reuse candidates and verdicts
against another revision. It does not return RETRYABLE.

Drive rereads Git and authoritative status once, preserves that handoff, and
stops the run even if the changed HEAD is clean. After the intervening work is
resolved, a new user invocation of Drive reconstructs current actionable work
and dispatches a fresh ordinary validation owner. That owner repeats all
preflights, checks and per-Spec assessments for the complete current set.
Standalone validation similarly requires a new validation run. No persistent
restart state or automatic retry budget is added. Failures after acceptance
begins retain Decision 0220's partial-acceptance preservation boundary.

## Verification

Both rendered agent packages and the installed project instruction block must
agree on Drive authority while preserving direct read-only claim routing.
DR2 checks completion without a second permission request; DR6 checks the
ordinary host for the planning owner. DR11 checks changed-revision handoff and
fresh-run restart, including a dirty worktree at unchanged HEAD. DR12 isolates
ordinary completion authority and an explicit recording limit from nested
implementation. These checks
do not replace measured grouped acceptance in DR9c.

DR6 also checks actual no-history dispatch arguments and distinct receiver
identities across Plan, implementation and validation. DR9w checks isolated
withdrawal dispatch with the retained fixed revisions. Artifact convergence or
an owner's claim to be fresh is insufficient evidence for these boundaries.
