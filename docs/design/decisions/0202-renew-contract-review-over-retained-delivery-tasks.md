# 0202: Renew Contract Review over retained delivery Tasks

Status: Accepted

## Context

Decision 0201 lets Design authoring, validation, and approval ignore a retained
downstream `tasks.yaml` after Requirements or Design is rewound. The document
and its execution records remain as repair input while their Tasks and
completion gate evidence is cleared.

The next milestone barrier still required every participant to be exactly in
`tasks` state and rejected the existence of any `tasks.yaml`. A recovery could
therefore reach fresh Design approval but not Contract Review. Passing the
barrier required deleting saved plans and rewinding otherwise unaffected Specs,
even though Tasks are forbidden review inputs and those Specs may hold valid
completed or in-progress work from the previous accepted review.

That makes artifact presence stand in for ordering authority. It also conflicts
with the lifecycle rule that rewinds preserve downstream documents as stale
repair input and with the rule that unaffected Specs retain their local state
when the milestone review is removed or becomes stale.

## Decision

For an ordinary delivery milestone, Contract Review acceptance requires every
participating Spec to:

- match the active Roadmap milestone;
- hold a fresh Design gate; and
- be at `tasks` or a later delivery state: `implementation` or `release_ready`.

A delivery participant's `tasks.yaml` is not an acceptance input. The guard
does not discover, parse, validate, fingerprint, rewrite, move, or delete it.
The semantic reviewer likewise does not read it. Retained plans and execution
records therefore survive renewed Contract Review byte-for-byte, including for
unaffected participants that remain in a later state.

Contract-first ordering is enforced at the authoritative boundary: no new or
revised Tasks gate can be approved without a fresh accepted Contract Review.
Ordinary Tasks authoring still starts only after that review. A file authored
prematurely has no accepted Tasks authority and gains none from being ignored
by the reviewer; the Tasks owner must review it against the accepted Design and
Contract before the CLI can approve it.

After renewed review acceptance, any participant whose Tasks gate was cleared
is actionable in the Tasks phase. Its Tasks owner revises the retained plan
through normal `tasks.yaml` authorship, states the before/after identity mapping,
and preserves execution records only where obligations and evidence still
apply. Participants whose Tasks gate and progress remain current are not
rewound merely to cross the milestone barrier.

Reverse establishment retains its stricter invariant. Every reverse participant
must be in `adoption_ready`, and any `tasks.yaml` fails acceptance with
`CONTRACT_REVIEW_REVERSE_TASKS_FORBIDDEN`; reverse milestones never enter a
Tasks phase.

Milestone status no longer emits `MILESTONE_TASKS_BEFORE_REVIEW` for delivery
artifact presence. Once all participating Designs are fresh and the review is
absent or stale, `contract_review` is the actionable global barrier regardless
of retained delivery plans. Reverse task-plan presence remains inconsistent.

This Decision narrows Decisions 0078, 0087, 0105, 0108, and 0134 where they
treated any task-plan presence or an exact `tasks` state as a pre-review ordering
violation. It replaces Decision 0199's recovery step that removed saved plans
and rewound unaffected Tasks gates before review. Decision 0201's preservation
rule now applies through Contract Review and into Tasks repair.

## Consequences

- Requirements or Design recovery can proceed through Design validation,
  Design approval, renewed Contract Review, and affected Tasks repair without a
  manual deletion gap.
- Completed, blocked, and pending execution records remain visible until the
  Tasks owner explicitly maps them during plan revision.
- Contract Review stays independent of implementation planning and does not
  claim that a retained plan is valid for the new Design.
- Unaffected Specs keep their approved Tasks and progress while the milestone
  review is renewed against current Contracts.
- No schema, sidecar archive, implicit gate invalidation, or user-operated
  discard command is introduced.

## Verification

Core and CLI tests prove delivery review acceptance ignores even a malformed
retained Task document, still rejects stale Design, permits the review action
from later delivery state, preserves the Task bytes through acceptance, and
keeps reverse Tasks forbidden. A lifecycle integration test covers Requirements
rewind, replacement approval, Design-scoped validation, Design approval,
Contract Review acceptance, Tasks repair, and Tasks approval. Fresh forward
testing covers the same recovery through the installed Skills.
