# 0195: Recover boundedly from a Task review scope defect

Status: Accepted

## Context

The Spec-backed implementation cycle already bounds review remediation and
fresh-context diagnosis. A rejected review normally returns concrete findings
to an implementer, while a diagnosis categorized as `PLAN` or `ARTIFACT` leaves
the implementation run because approved work must change before it can proceed.

Dogfooding exposed a different failure. A reviewer treated work assigned to a
later Task or Spec as a blocking obligation of the current Task. The corrective
implementer correctly could not repair that finding inside the approved scope,
but the debug protocol had no category for an incorrect review subject. Calling
it `PLAN` then forced the same stop used for a genuinely defective task plan.

The approved artifacts are not defective in this case, and diagnosis is not an
approval. The implementation owner needs one bounded path back to a new
independent judgment over the correct subject.

## Decision

The debug protocol adds `REVIEW` as a cause category. It applies only when the
evidence establishes all of the following:

- a blocking review finding requires work outside the current Task's approved
  Requirement IDs, completion criteria, Design responsibility, or declared
  ownership;
- that work belongs to another Task, Spec, or later lifecycle boundary; and
- neither the current task plan nor another approved artifact must change for
  the current Task to be judged.

A missing or incorrectly ordered Task remains `PLAN`. Contradictory or
unworkable Requirements or Design remain `ARTIFACT`. Uncertain ownership remains
`UNDETERMINED`; it is never promoted to `REVIEW` merely to keep a run moving.

When `sb-implement` receives a valid `REVIEW` diagnosis, it may dispatch a fresh
independent reviewer over the current Task. The brief carries the exact Task
scope, the rejected findings, and the diagnosis evidence. It does not carry a
desired verdict. The new reviewer applies the ordinary `task-review` protocol
from the diff and approved artifacts, and only its returned verdict can approve
or reject the Task.

The context executing `sb-implement` remains the cycle owner across that
dispatch. It waits for the reviewer, parses the verdict, then resumes progress,
notes, and checkpoint handling itself. It never delegates the whole cycle to a
reviewer or returns the review block as the implementation result.

A request to continue or recover a pending implementation while carrying a
returned review or diagnosis remains owned by `sb-implement`. Project routing
selects that owner before the direct `sb-review-task` and `sb-debug` routes so
the fresh verdict can still be followed by Task progress and the checkpoint.
A diagnosis-only request and a standalone request to judge a diff retain their
read-only direct routes.

This re-review consumes the existing review/remediation budget. It does not add
or reset attempts. If no round remains, the Task is blocked with the outstanding
finding and diagnosis. A `PLAN` or `ARTIFACT` diagnosis still leaves the run and
cannot use this path.

The task-review protocol makes the scope boundary explicit: another Task or
Spec's unimplemented obligation may be noted or deferred under project policy,
but it cannot block the current Task unless an approved current-Task input makes
that obligation part of the subject.

## Consequences

- A reviewer scope mistake can be corrected without changing accepted artifacts
  or requiring a new user instruction.
- Diagnosis still cannot turn a rejection into approval; a fresh reviewer owns
  the replacement verdict.
- The existing attempt ceiling continues to stop repeated disagreement.
- Real plan and artifact defects retain their fail-closed route.

## Verification

Mechanical Skill and protocol tests require the new category, the evidence
threshold, fresh re-review, unchanged attempt budget, and the prohibition on
diagnosis-as-approval. Delivery forward tests cover the recovery path when an
initial reviewer-scope rejection is exercised.
