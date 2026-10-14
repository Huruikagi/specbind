# 0219: Recover completion invalidated by delegated replanning

Status: Accepted

## Context

Issue #81 exposes a delivery deadlock: an authorized Task replan stales another
participant's project-scoped completion; Drive stops on inconsistent health,
while completion validation waits for remaining implementation. The existing
guarded completion invalidation works before milestone convergence. The
maintainer authorized that aftercare as part of `--replan`.

## Decision

Extend Decision 0199 with completion-only withdrawal caused by the same
authorized replan. Keep Decision 0119 freshness and the milestone convergence
barrier unchanged. No CLI, schema, configuration, or persisted recovery state
is added. This narrowly amends the inconsistent-health stop in Decision 0168
and the confirmation/entry boundary of Decision 0112.

Before dispatching Plan, Drive records a clean committed baseline, active
delivery milestone identity and scope, each participant's state, completion
freshness/revision, and earlier gate freshness. It reports which accepted
completions the replan will stale. After Plan's terminal handoff and checkpoints,
Drive checks current status and the actual intervening Git delta. A retained
continuation may supply these facts; missing history is never replaced by an
assumption about cause from a diagnostic alone.

Automatic withdrawal requires all of the following:

- explicit replan authority for the same active delivery milestone and scope;
- the target had fresh accepted completion at the recorded clean baseline;
- the intervening delta contains only that authorized recovery's changes;
- the target is now `release_ready`, with earlier gates fresh and completion
  stale solely under `FRESHNESS_COMPLETION_PROJECT_CHANGED`;
- every milestone inconsistency is explained by this exact eligible set;
- every target `spec.yaml` is clean and unstaged, and no live owner or unsafe
  partial work remains.

Pre-existing staleness, missing evidence, scope changes, other gate or Contract
Review inconsistencies, unattributed edits, and dirty targets stop automatic
recovery. Merely passing `--replan` does not authorize arbitrary stale evidence
removal. Ordinary invalidation still requires its existing user confirmation.

Drive prechecks the entire eligible set, then dispatches the complete
`sb-validate-implementation` Skill for one target at a time in milestone order,
with explicit completion-withdrawal-only authority, baseline/current evidence,
exact paths, and the remaining set. This is a second explicit exception to
ordinary actionable-entry scheduling; it grants no general inconsistent-state
bypass. Drive never performs the owner's mutation or checkpoint itself.

The validation owner independently checks the supplied evidence and current
state before its usual completion preflight. It uses the existing guarded
`spec completion invalidate` command, verifies that only completion was removed
and state became `implementation`, and closes a separate Git-adapter checkpoint
containing exactly that one withdrawal. It neither changes code nor runs final
validation or accepts completion in this mode. It returns `WAITING` for milestone
convergence with the exact checkpoint and next owner, not `GO` or a claim that
the Spec is complete. Authority already supplied needs no second confirmation.

Drive rereads status and Git after every withdrawal and recognizes only the
remaining prechecked targets. A refusal, changed evidence, missing checkpoint,
or unchanged target stops the route without retrying at unchanged state. Earlier
successful withdrawals remain valid; report partial progress without rollback.
No new withdrawal cycle or Task retry allowance is created. Completed targets
are skipped only when the retained handoff and current checkpoint prove that
exact withdrawal; new sessions do not infer this from status alone.

Once all withdrawals are checkpointed, ordinary status scheduling resumes the
remaining implementation, then final validation after all milestone work
converges. If no implementation is pending, validation may be the next ordinary
action. No old completion evidence is restored, and new completion acceptance
still requires the complete fresh validation handshake.

## Verification

CLI regression fixtures demonstrate completion-only withdrawal while another
Spec has pending work, unchanged earlier gates, the convergence barrier, and
dirty-target refusal. Embedded-Skill conformance and fresh behavioral scenarios
cover delegated withdrawal, checkpoint ownership, continuation, and exclusions.
