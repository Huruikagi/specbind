# 0203: Carry a CLI-owned Design rewind through validation

Status: Accepted

## Context

Design and Contract findings can require the confirmed operation
`specbind spec design invalidate <spec>`. The CLI correctly changes that
Spec's `spec.yaml` immediately: Design, Tasks, and completion evidence are
cleared and the Spec returns to `design`.

Plan deliberately keeps an unapproved Design and Contract dirty until an
independent Design validator returns `READY`. Its clean-handoff exception,
however, admitted only those authored artifacts and an applicable deferred
finding destination. It explicitly rejected `spec.yaml` before approval. A
valid CLI invalidation therefore created a path the same recovery could neither
commit nor carry to validation.

Committing invalidation alone would invent a partial checkpoint. Discarding,
stashing, or editing `spec.yaml` would erase or falsify CLI-owned lifecycle
state. A bounded recovery needs to carry that state without turning every dirty
`spec.yaml` into phase-owned work.

## Decision

A confirmed Design invalidation creates one **Design-rewind delta** that belongs
to the continuing Design phase. The delta is the exact lifecycle-path set
changed by the successful CLI operation: that Spec's `spec.yaml` and, when an
accepted milestone Contract Review existed, its removal. These are not authored
artifacts and grant no permission to edit machine state.

The owner that invokes invalidation must:

1. record `git status --short` before the command and prove that the target
   `spec.yaml` and accepted-review path have no pre-existing change;
2. invoke the exact confirmed CLI operation;
3. immediately record the successful result, `specbind spec status <spec>`, the
   exact changed-path set, and its Git diff; and
4. pass those paths and the captured diff unchanged through the Design author,
   independent validator, and approval receiver.

Before independent validation, Plan rereads status and the lifecycle diff. It
may admit the exact `spec.yaml` change and accepted-review removal alongside the
reported Design set, this Spec's Contract, and an applicable verified deferred
destination only when the current path set and diff exactly match the captured
Design-rewind delta and status reports the expected `design` state with Design
not reached and review absent. The validator remains read-only and treats the
proven delta as lifecycle context, not as a Design finding.

After `READY`, the Design approval command updates the same `spec.yaml`. The
approval receiver's ordinary Design checkpoint contains the Design set,
Contract, gate-updated `spec.yaml`, the proven accepted-review removal when
present, and applicable deferred destination. No invalidation-only checkpoint
is created.

This is a closed exception. A pre-existing lifecycle-path change, missing
capture, changed path set or diff, manually edited lifecycle field, second Spec metadata path,
unreported path, or unrelated dirty work is not attributable to the rewind and
still stops the handoff. Requirements and Tasks rewinds do not acquire this
exception by analogy; their owning phase contracts must define any future need.

This Decision narrows the clean-handoff clauses in Decisions 0120, 0170, 0194,
and 0199 only for a proven CLI-owned Design invalidation.

## Consequences

- Contract Review and Design-validation findings can return through Design
  repair, independent validation, reapproval, checkpoint, and renewed review.
- The invalidation remains uncommitted until the complete Design phase closes;
  no partial checkpoint or user-operated stash/discard workaround is added.
- Unrelated or manually edited lifecycle changes remain fail-closed.
- No CLI, schema, or persisted provenance artifact is required; the proof lives
  only in the active orchestration context and current Git diff.

## Verification

Focused Skill tests require capture, exact-diff verification, validator
read-only handling, and approval-checkpoint closure. A CLI integration test
proves an uncommitted Design invalidation delta can remain present while Design
is revised and reapproved. Fresh forward testing exercises the composed
Contract Review finding recovery.
