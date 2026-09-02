# 0181: Establish reverse Specs as a non-release baseline

Status: Accepted

Supersedes Decision 0175 where it stops adoption before Requirements. The
Steering-first evidence rules and `sb-discovery` integration from Decisions
0143 and 0175 remain authoritative.

## Context

The existing-implementation route treated reverse discovery as preparation for
an ordinary change milestone. That made `sb-plan --all` create Tasks even when
the requested outcome was only to describe the product that already exists.
It also split one establishment decision across several invocations and left
finalization without an identity distinct from a product release.

Existing code can prove what a revision does, but cannot by itself decide what
the product promises. A useful reverse workflow must preserve that distinction
without stopping at every non-blocking observation.

## Decision

Roadmap scope has four disjoint work-item categories: `newSpecs`,
`specUpdates`, `reverseSpecs`, and `directChanges`. A reverse milestone contains
only `reverseSpecs`, has `target_release: null`, and records the existing
product version as `baseline_version`. Its `baseline_revision` is also the
immutable implementation evidence revision.

Each reverse-created Spec records durable establishment provenance:

```yaml
establishment:
  kind: reverse
  source_revision: <baseline_revision>
  baseline_version: <existing product version>
  milestone_id: <reverse milestone>
```

`sb-discovery` owns one explicit reverse mode in a progressively loaded
reference. It scans only the selected implementation at the fixed revision,
proposes the complete Spec set and maintained intent once, and waits for one
confirmation before creating the reverse Roadmap, Specs, Briefs, and Research.
After confirmation the same orchestration continues through Requirements,
Design, Design validation, and milestone Contract Review. It stops only for a
semantic unknown whose answer would materially change the Spec, source drift,
invalid evidence, or another failed lifecycle guard. Independent Specs may
continue while one is blocked, but global review and finalization wait.

Reverse establishment never creates Tasks and never changes implementation,
tests, dependencies, configuration, or Steering. Design approval enters the
dedicated `adoption_ready` state rather than `tasks`. Scope update and
rebaseline are forbidden while the reverse milestone is active. Ordinary
change Discovery may explain a proposed change but cannot mutate this Roadmap;
the reverse milestone is finalized first. An urgent change requires explicit
abandonment and a later reverse run from the new revision.

An observation that looks defective may be recorded through an already active
Deferred Findings Adapter as a suspected defect, tied to the source revision
and evidence locator. It is not treated as a confirmed bug, is not corrected
in this workflow, and does not change the revision. External transmission still
requires the adapter's normal authority. Duplicate identity is source revision,
locator, and claim.

The CLI finalizes with:

```sh
specbind milestone reverse finalize --log-entries <path-or->
```

Finalization requires every reverse Spec to have fresh Requirements and Design
approval, a fresh Contract Review, a clean worktree, and no implementation
changes since the evidence revision. It clears active change state, retains
establishment provenance, removes temporary Brief and Research artifacts,
writes one `Baseline <baseline_version>` entry to each Spec `log.md`, and moves
the Roadmap and Contract Review to `baselines/`. It does not invoke a Release
Adapter, create a tag, publish artifacts, bind `target_release`, or represent a
new product release.

Deferred unknowns are permitted only when a later answer cannot alter current
Spec meaning. Otherwise the affected Spec remains blocked. Starting a reverse
run therefore means completing it in one continuous orchestration unless an
explicit stop condition occurs.

## Consequences

- Reverse discovery, new-Spec delivery, and existing-Spec change are distinct
  lifecycle intents even though they reuse Requirements, Design, and review.
- Existing product identity is visible as `baseline_version` without pretending
  that documentation establishment shipped a release.
- Established Specs become ordinary existing Specs while retaining auditable
  origin.
- The heavy reverse procedure stays out of normal Discovery context until
  explicitly selected.
- Explicit emergency abandonment uses `specbind milestone reverse abandon
  --milestone-id <id>` at a clean repository; manual state deletion is not a
  substitute.

## Verification

Wire-schema and CLI tests cover reverse-only scope validation, provenance,
`adoption_ready`, scope and baseline locks, status actions, Tasks rejection,
source drift, and non-release finalization. A fresh `sb-discovery` forward test
must demonstrate one confirmation, continuous orchestration, blocking unknowns,
and absence of Tasks or release work.
