# Withdraw completion evidence

A Spec at `release_ready` whose evidence no longer holds is revalidated. State
what invalidation clears — completion evidence only; requirements, design, and
tasks stay approved — and run it after the user confirms, unless the delegated
withdrawal conditions below already supply that exact authority:

```sh
specbind spec completion invalidate <spec>
```

After success, verify that the command changed only the target `spec.yaml`, that
its state moved from `release_ready` to `implementation`, and that only
completion evidence was removed. Read the Git adapter and checkpoint exactly
that evidence-withdrawal delta before running a new preflight:

```sh
specbind adapter read git --for consume
```

This is an independent, narrow checkpoint. Do not include source work, amend an
earlier commit, or carry the dirty invalidation into validation. If the adapter
is absent or scaffolded, forbids the commit, is ambiguous, or the checkpoint
fails, stop and report the valid invalidation plus its exact dirty path. The
clean-repository completion preflight cannot resume until that checkpoint is
closed. For ordinary revalidation, after it is closed, begin again at preflight;
a later `GO` acceptance is
checkpointed separately as fresh completion metadata.

### Delegated withdrawal before convergence

For a Drive **completion-withdrawal-only authority** handoff, independently run:

```sh
specbind milestone replan check --baseline <baselineRevision> --recovery-revision <recoveryRevision>
```

Use the fixed revisions supplied by Drive. Require this target first in
`remaining` and the reported `currentRevision` unchanged before mutation.
The CLI checks historical freshness, scope, earlier gates/review, completion
stale solely under `FRESHNESS_COMPLETION_PROJECT_CHANGED`, and exact prior
withdrawal checkpoints. Missing history, pre-existing staleness, other
inconsistencies or dirty state refuse the check. Never replace a refusal with
your own eligibility verdict or change the revisions to make it pass.

Independently verify the original replan authority, no live owner, and the
actual baseline-to-recovery Git delta and history against the authorized repair.
`recoveryChangedPaths` is an index, not semantic attribution. The output explicitly
requires this authority and delta review; a successful mechanical check alone
does not authorize withdrawal.

Report the concrete withdrawal and use the guarded command and independent,
narrow checkpoint above without a second user confirmation. Handle exactly one
target; never batch-edit state or include another target in its checkpoint.
Rerun the same check after the checkpoint and require this exact entry in
`withdrawn` with an advanced `remaining` set, then return `WAITING` for
milestone convergence to Drive with target, checkpoint, earlier-gate freshness,
remaining affected set, and next owner. Do not run completion preflight, final
checks, or acceptance in this withdrawal-only dispatch, even if Tasks happen
to be complete. Do not return `GO`. Drive schedules normal validation later.

If invalidation succeeded but its checkpoint failed, report the exact dirty
path and stop; preserve that valid withdrawal and never restore old evidence.
Without this evidenced delegation, the ordinary confirmation still applies.

Never use it to clear a path to a new `GO`. A refused validation is information
about the implementation.
