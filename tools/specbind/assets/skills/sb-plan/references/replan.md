# Replan during an authorized Drive run

Read this procedure only for a Drive dispatch with explicit replan authority,
named affected Specs, and a concrete Design, Contract, or Tasks finding. This
is recovery within the existing delivery Milestone, not a new planning scope.
The option grants Design/Tasks rewind and approval authority under workflow
`sb-drive`. Do not request the same permission again. Ordinary Plan delegation
and an unqualified Drive request do not grant this authority.

## Establish the change and its cost

Read current Milestone scope/status, the affected Spec status and artifacts,
and the relevant Contract providers and consumers. Compare against the original
approved Requirements and Milestone scope supplied by Drive. Requirements and
scope/dependencies remain fixed. Do not reinterpret a Requirement, retire one,
or move an obligation outside scope to make recovery fit. Return a concrete
user decision before mutation if the boundary cannot be preserved, including
an unsettled external compatibility or migration obligation.

Choose the earliest necessary owner: Tasks alone when Design and Contract are
still valid, otherwise Design (which owns Contract). Name the affected Specs,
changed obligations, existing progress, and invalidation cost. Report this as
progress, not a confirmation question. A Design rewind clears its Design,
Tasks and completion evidence and removes the milestone-wide Contract Review;
a Tasks rewind clears only its Tasks and completion evidence. Neither removes
the source implementation or automatically clears task execution records.

Establish the project cwd, CLI/PATH, applicable instructions, and exact phase
reference paths as in the normal Plan dispatch procedure. Pass `sb-drive` as
the delegation workflow, exact Design/Tasks authority, original scope boundary,
and accumulated review findings/budgets to every owning receiver. No receiver
may infer authority omitted from its dispatch.

## Preserve work before rewinding

Inspect Git status and the exact diff. Preserve reported partial implementation
paths from the interrupted owner unchanged throughout planning; carry those
paths back for implementation and review. Do not switch to independent work
while they remain, and never use a WIP commit, stash, reset, or discard as
recovery. Unattributed, unrelated, or overlapping edits block this handoff.

Before changing a plan with progress, establish a recoverable Git revision of
the existing plan and execution records. Follow the Git adapter for a narrow
checkpoint of already approved plan/progress metadata if needed; this exception
does not permit committing partial source or an unapproved replacement plan.
If no recoverable revision or permitted checkpoint is available, report that
concrete blocker. Never overwrite a dirty `spec.yaml` to bypass a CLI guard.

## Repair through the owners

For a Tasks-only correction, dispatch the Tasks phase to invalidate its gate,
revise and reapprove the plan under the supplied authority. Keep the accepted
Contract Review and upstream gates. The author reports the before/after Task
identity mapping and preserves execution records only for unchanged obligations
with still-applicable evidence. Removed or changed work must not inherit a
completed record. This mapping is the Tasks procedure's existing exception to
ordinary CLI-owned progress; it is not permission to mark new work complete.
For retained blocked records, report exactly which cause is resolved and the
mapped Task ID. The resumed implementation owner uses `tasks reopen` before
selecting work; a valid new gate alone does not clear an old blocker.

For a Design or Contract correction:

1. Establish every Spec-backed participant from current Milestone scope. The
   renewed Contract Review requires fresh Design but admits retained delivery
   Tasks and later delivery states. Account for the global semantic review cost
   without rewinding otherwise unaffected Tasks gates or progress.
2. Through the respective owning phases, invalidate Design only for changed
   Specs. Keep the Requirements gates where valid. Leave every retained
   `tasks.yaml` and its execution records in place; Contract Review does not
   read them, and no owner removes them to cross the barrier.
3. Dispatch Design authors for the affected Specs in dependency order, then
   fresh independent `sb-validate-design`, then Design approval only on `READY`.
   Follow the normal Plan validation and finding-continuity rules. The normal
   phase checkpoint also accounts for the owned rewind/removal metadata paths;
   do not let those become unattributed work at a fresh receiver.
4. Dispatch `sb-contract-review` for the whole Milestone. It reads all persistent
   seams, including outside-scope consumers, and accepts only a passing review.
   A finding routes back to its owner within the existing remediation budget.
5. After acceptance, dispatch Tasks owners to revise the retained plans in place,
   recheck execution readiness against the new Design/Contract, and reapprove.
   Preserve unchanged proven work using an explicit identity mapping; clear
   records for changed obligations so implementation proves them again. A saved
   completed status alone is not proof after its inputs have changed.

Only the owning phases edit their artifacts; only CLI commands edit gate state.
Do not relax CLI guards, manually edit `spec.yaml`, restore old gate/completion
evidence, or skip a review because this is recovery. Restoring a saved plan is
Tasks authorship after Contract Review, never restoration of old approval.

## Return to Drive, not to the user for a restart

Complete the adapter-directed planning checkpoints. Report revised artifacts,
gate/review results, remapped or reopened work, any carried partial source
paths, and current status. Return to the invoking Drive so it immediately
resumes implementation and final validation. Replanning does not complete the
delivery request.

Existing Design validation and Contract Review remediation budgets and finding
history survive all recovery dispatches. A repeated unresolved defect or an
exhausted budget returns an unfinished result; do not obtain a new budget by
starting another planning run. No persistent recovery queue or authority file
is created. If interrupted, recover from current CLI state and Git evidence,
with authority still explicit in the resumed request/context.
