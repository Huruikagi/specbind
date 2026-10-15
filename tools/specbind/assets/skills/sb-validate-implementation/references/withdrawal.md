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

For a Drive **completion-withdrawal-only authority** handoff, independently
check the baseline/current Git and status evidence before any mutation. Require
the same active delivery milestone and scope, explicit replan authority, a
clean baseline with this target's completion fresh, and an intervening delta
attributable only to that authorized recovery (plus earlier verified withdrawal
checkpoints in the supplied set). The target must now be `release_ready` with
fresh earlier gates and completion stale solely because of
`FRESHNESS_COMPLETION_PROJECT_CHANGED`. Every milestone inconsistency must be
explained by the remaining prechecked targets. Missing facts, pre-existing
staleness, other gate or Contract Review inconsistencies, a dirty or staged
target `spec.yaml`, and unsafe partial work stop this route without mutation.

Report the concrete withdrawal and use the guarded command and independent,
narrow checkpoint above without a second user confirmation. Handle exactly one
target; never batch-edit state or include another target in its checkpoint.
Verify current state and the actual committed diff, then return `WAITING` for
milestone convergence to Drive with target, checkpoint, earlier-gate freshness,
remaining affected set, and next owner. Do not run completion preflight, final
checks, or acceptance in this withdrawal-only dispatch, even if Tasks happen
to be complete. Do not return `GO`. Drive schedules normal validation later.

If invalidation succeeded but its checkpoint failed, report the exact dirty
path and stop; preserve that valid withdrawal and never restore old evidence.
Without this evidenced delegation, the ordinary confirmation still applies.

Never use it to clear a path to a new `GO`. A refused validation is information
about the implementation.
