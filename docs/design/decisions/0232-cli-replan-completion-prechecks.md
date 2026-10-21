# 0232: CLI prechecks for replan completion recovery

Status: Accepted

## Decision

Move Decision 0219's deterministic baseline, eligibility and checkpoint checks
into read-only `milestone replan capture` and `milestone replan check --baseline
<commit> --recovery-revision <commit>`. Both emit JSON, require a clean checkout
and change no project files or lifecycle state. Capture runs before Plan and
returns the exact baseline revision and fresh accepted completions. Check uses
that fixed baseline and Plan's terminal checkpoint as the fixed recovery
revision; neither value may advance to hide intervening work.

Check reconstructs committed states in disposable local repositories without
network access, project hooks or configured checkout filters. It verifies the
baseline was consistent with fresh Contract Review, and re-evaluates the whole
recovery snapshot. The Roadmap scope and Requirements authority must be
unchanged. A target must have had fresh completion at baseline and retain that
same Spec evidence at recovery, now stale solely because project content
changed. Earlier gates and Contract Review must be fresh; all inconsistencies
must be explained by the exact target set. Targets are returned in Roadmap order.
Missing history or unsupported snapshot inputs fail without a fallback verdict.

After the fixed recovery revision, only one-target completion-withdrawal
checkpoints in that order are allowed. Each must have one parent, change exactly
the target Spec file, move release_ready to implementation, and remove only
completion evidence. Every commit is examined, including reverted changes;
merges, unrelated changes, duplicate withdrawals and missing checkpoints stop
the route. Check returns remaining targets and proven withdrawal commits, and
rechecks current clean HEAD before returning. Historical state is never inferred
from current status alone. No persisted recovery ledger or new wire artifact is
introduced.

The output is mechanical evidence, not authorization or a semantic causality
verdict. Drive and the withdrawal owner still verify the supplied replan
authority, no live owner, and the actual baseline-to-recovery Git delta against
the authorized work. Check reports changed paths for that review. Do not infer
causality from an allowed path or a stale diagnostic alone. Both owners run the
same check with the fixed revisions; failures stop instead of falling back to
manual status interpretation. Existing `spec completion invalidate`, its narrow
checkpoint, ordinary explicit confirmation and final validation remain separate.

This supersedes Decision 0219's statement that no CLI is added, without widening
the delegated authority or relaxing convergence. Sequential readiness remains
independent under Decision 0231.

## Verification

Tests cover fresh baseline capture, complete-set eligibility, missing or stale
history, changed scope/Requirements, dirty state, another stale gate, semantic
authority not being inferred, exact ordered withdrawal checkpoints and rejected
unrelated/reverted commits. Skill conformance and fresh forward tests verify
the new reads and continued owner/confirmation boundaries.

The [first focused DR9w measurement](../../skill-forward-tests/runs/2026-10-21-codex-f764a6b-readiness-replan.md)
confirmed the CLI reads and narrow withdrawal checkpoint, but reproduced
FT-0073's inherited owner context. Fresh-owner isolation remains unconfirmed;
that existing finding is not resolved by these deterministic prechecks.
