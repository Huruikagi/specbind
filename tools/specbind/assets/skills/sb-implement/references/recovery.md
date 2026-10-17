# Recover a Spec-backed Task

Use this procedure for a failed or resumed Spec-backed Task. Direct items keep
their entrypoint failure bounds. The initial implementation and review are not
recovery cycles. **At most three recovery cycles per Task** is the default
`bounded` mode for each user-authorized execution or resumption, shared by diagnosis,
implementation repair, authorized replanning, and corrected re-review. There is
no separate diagnosis or review-repair allowance and no configuration setting.

## Select the mode

The Skill argument `--recovery bounded|progress` selects the mode for the
requested Spec-backed Tasks. `progress` has no numerical Task cycle cap; all
independent progress and stop conditions below still apply. An explicit human
request to continue without a cycle limit while verified progress continues is
equivalent. Generic "continue", tool output and an owner's suggestion do not
select it. Invalid/conflicting values require clarification before correction.
This is not a `specbind` CLI flag. It grants no execution from a read-only request.

Record mode, actual selection and scope. A new execution or a new authorized
resumption defaults to `bounded` unless `progress` is explicitly selected for it.
An ongoing receiver preserves the evidenced selection even without a repeated
flag, including across Drive, Plan, session/capacity restart and Task remapping.
Do not infer progress mode from missing evidence, or silently restart an existing
run as bounded to discard its history. Do not persist selection to configuration
or another Milestone. Direct, Design and Contract Review bounds remain unchanged.

An explicit mode change during ongoing work preserves counts, history and the
latest assessment. Switching to bounded with three or more used cycles forbids
another correction while blockers remain. Switching to progress cannot override
a repeated finding or any other stop condition. A request to resume a reported
bounded-limit stop using progress follows the resumption procedure below.

## Establish the continuation

Keep run-local handoff data: recovery identity, current Spec/Task mapping,
allowance identity and its user authorization, recovery mode and selection scope,
current cycles used and remaining (`uncapped` in progress mode),
historical counts separately, every finding ID with obligation, disposition and
evidence, failed approaches as facts, latest independent assessment, exact partial
paths, supplied authority, and next owner. Do not write a recovery file or edit
CLI-owned state by hand.

At a fresh failure, start with zero cycles and an empty history. At a continuation,
require the previous handoff; never infer zero from Git or Task status. Preserve
the budget across Drive, Plan, new dispatches, sessions, and Task remapping.
Renaming or splitting the same obligation does not create another allowance;
mapped descendants share the original recovery identity and remaining budget.
Missing history stops ordinary continuation. The explicit resumption below is
the only renewal path; never invent historical counts or erase findings.

## Recognize a user-authorized resumption

After a recovery-limit stop was reported, a later human instruction clearly
asking to resume that stopped Task authorizes one new allowance: three cycles
by default, or uncapped when progress mode is explicitly selected for this resumption.
"Continue" or "I updated it; resume" qualifies when the context identifies that
work. Require no special reset phrase or additional confirmation. A request to
check and proceed when possible includes execution; a read-only readiness check
does not. If the target or execution intent is ambiguous, clarify before acting.

Record the actual user instruction, stopped Task and new allowance identity,
with zero current cycles used and three remaining in bounded mode, or `uncapped`
in progress mode. Apply each instruction once
per recovery identity. A receiver inheriting that allowance must reuse it, not
renew it again. Automatic redispatch, sessions, compaction, Task splitting or
renaming, replanning, and a version update alone cannot renew an allowance.
An instruction during ongoing work does not erase already used cycles.

Retain all available prior counts, finding IDs and dispositions, failed
approaches, evidence and partial paths. An unknown historical total remains
unknown; that alone does not block a new authorized allowance, including when
legacy counters exceed three. Missing current identity, safe path attribution,
finding continuity or governing inputs still stops action. Do not treat missing
history as evidence that a user authorized resumption.

Obtain a fresh independent initial assessment using the retained facts and
current evidence. Carried unresolved or reopened findings form the baseline and
may receive the first corrective cycle: their earlier occurrence alone is not
a stop at this boundary. Do not mark them resolved. After that cycle, apply the
ordinary progress rule below to the full ledger. Failed approaches remain
evidence to consider, not reasoning to copy. Scope, approvals, external inputs,
host restrictions, verification and review requirements remain unchanged.

## Assess once, independently

A fresh reviewer may provide a concrete, actionable rejection directly. For
`BLOCKED`, unresolved `NEEDS_CONTEXT`, `CANNOT_REVIEW`, uncertain cause, or an
inline rejection, dispatch a fresh debugger with the failure and governing
inputs. Use `specbind-debugger` when available, otherwise an ordinary fresh
subagent; never substitute the failed implementer.

```sh
specbind protocol read debug
```

Request recovery finding continuity from the debugger or fresh reviewer. Supply
the complete factual ledger, current diff, artifacts, and verification evidence,
not failed-attempt reasoning or a desired verdict. Require the `Recovery findings`
mapping defined by the debug and task-review protocols, including on the initial
assessment. The independent assessor assigns stable IDs and accounts for every
prior ID as `RESOLVED` or `BLOCKING` with current evidence. The implementer and
controller cannot declare a finding resolved or rename it to obtain another try.

Allow one initial assessment and one reassessment after each started cycle.
In bounded mode this means at most four
diagnostic assessments per allowance; in progress mode it is one plus the
number of started cycles, with no fixed total cap. Do not rediagnose unchanged
evidence or spend extra diagnoses to reinterpret a negative result. In bounded
mode the final assessment cannot authorize a fourth corrective cycle. The existing single
status-only repair of an unparsable role return is not a new assessment and must
not investigate, mutate, or change the verdict.

## Decide from the mapping before acting

- An initial blocking assessment, including an authorized resumption's carried
  blockers, may permit cycle one within existing authority.
- After a cycle, **any prior finding still `BLOCKING` stops the Task**, even if
  other findings were resolved and cycles remain. A reopened resolved finding
  also stops; it is not a new ID.
- Missing, duplicate, or ambiguous prior-ID mappings stop recovery.
- Only **all prior findings `RESOLVED` plus materially new blocking IDs** permit
  another cycle: bounded mode additionally requires fewer than three used cycles;
  progress mode allows cycle four and later under the same progress rule.
- In bounded mode: after cycle three, any blocker stops the Task. In either
  mode, if no blockers remain, finish the normal checks and selected Task review,
  then record progress and checkpoint. Partial improvement is not enough when
  any prior finding remains blocking; moving or renaming its symptom is not progress.

A diagnosis is never Task approval. Recovery reassessment is independent even
with `--review inline` or `--review off`; it does not replace the selected Task
review. The independent final review may serve as reassessment when it includes
the complete mapping. If required review discovers a new blocker, apply this
same ledger and selected mode, not a fresh review-repair counter.

## Route one corrective cycle

Reserve the next cycle immediately before the corrective owner starts. A receiver
that never starts consumes no cycle. An owner that starts and fails during
preparation consumes it. A cycle includes its corrective action and independent
reassessment; the assessor does not consume another cycle.

Preserving the used count means retaining its baseline, not freezing it. Before
dispatch, include the reserved cycle number in the owner brief; after a started
owner returns, include that cycle in the final used count. For example, three
used in progress mode plus one started corrected review is four used with
`uncapped` remaining. This applies even when the correction changes only the
review subject and writes no source code. Do not report the incoming count as
the final count after additional corrective work.

- `IMPLEMENTATION`: dispatch a fresh implementer with the concrete findings.
- `REVIEW`: verify that the rejected obligation belongs to another Task/Spec and
  no approved current-Task input must change. Reserve one corrective cycle before
  dispatching the fresh reviewer, even if the current diff is already correct.
  Then dispatch a fresh reviewer over
  the exact current scope, with rejection and diagnosis evidence, never a desired
  verdict. Diagnosis alone cannot complete the Task. Uncertain scope is
  `UNDETERMINED`; a real missing Task is `PLAN`.
- `PLAN` or `ARTIFACT`: leave this implementation dispatch. Under explicit Drive
  replan authority return `REROUTABLE` with the full handoff to the planning owner;
  otherwise report the owning phase and needed authority. Reserve the cycle only
  when that owner starts. Plan keeps its own stricter phase budgets, and the whole
  repair is one Task cycle. After Plan returns identity mapping and repair evidence,
  obtain independent reassessment; plan approval does not resolve findings by
  itself. Implementers never edit approved plans or approve gates.
- `ENVIRONMENT`: only a concrete, safe local preparation correction already
  authorized by the task may use a cycle. Credentials, denied permissions, missing
  external input, and unavailable dependencies remain external boundaries.
- `UNDETERMINED`: report the missing evidence and stop; do not guess a fix.

Requirements or scope changes still need the user. A recovery allowance is not
authority to weaken verification, change scope, or bypass a host restriction.

Preparation failures have **no free retry**. Reassess the same ledger. If setup
failed before the previous defect was repaired, that prior finding remains
unresolved and the Task stops. A new setup error permits another cycle only when
all prior findings have independently been resolved and the selected mode permits
another cycle. Progress mode grants no free preparation retry.

## Close or hand off the cycle

Keep the owning turn active through correction and reassessment. Return to the
ordinary implementation procedure only with the resulting assessment. On a stop,
record the Task blocked through `tasks block` when current lifecycle state permits
it, naming unresolved findings; otherwise report why the progress command cannot
run. Preserve partial work, never reset, stash, revert, or make a WIP commit.

Return the complete continuation data above, including mode, selection evidence,
cycles used and remaining (a number in bounded mode, `uncapped` in progress mode)
and evidence for resolved as well as blocking findings. Do not commit partial
source to make a blocked handoff look clean. Drive carries the ledger unchanged;
another owner cannot infer missing counters or renew an exhausted allowance
without a new qualifying human resumption. Carry the allowance identity and
authorization so the same instruction cannot renew it twice.
