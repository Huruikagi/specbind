# 0227: Opt into Task recovery without a cycle cap while progress continues

Status: Accepted

## Context

Decisions 0218 and 0226 stop a Task after three corrective cycles even when
independent assessments establish progress. The maintainer chose an explicit
per-run option to remove that numerical cap while preserving the existing
finding-based stop conditions. Partial improvement is not sufficient progress.

## Decision

`sb-implement` and `sb-drive` accept the Skill argument
`--recovery bounded|progress`. `bounded` remains the default: at most three
cycles per user-authorized Task execution or resumption. `progress` has no
numerical Task cycle cap. An explicit natural-language request to continue
without a cycle limit while independently verified progress continues selects
the same mode. A generic request to continue or finish does not select it.
This is a Skill argument, not a `specbind` CLI flag or project setting. Invalid
or conflicting selections require clarification before corrective work.

The option applies only to Spec-backed Task recovery in the requested run:
selected Tasks for implementation, or Spec-backed Tasks reached within the
active Milestone for Drive. It adds no scope or execution authority to a
read-only request. Direct-item, Design, Contract Review and other phase bounds
remain unchanged, including phases nested inside a Task's planning repair.

An initial independent assessment may authorize the first corrective cycle.
After each correction, every previously blocking finding must be independently
resolved and remaining blockers must be materially new before another cycle
can start. Any unchanged or reopened finding, missing/ambiguous ID mapping,
uncertain cause, unsafe worktree, missing authority or external prerequisite
stops recovery. Renaming or moving a symptom is not a new finding. No blockers
leads to normal verification, required review and completion, not more cycles.

`progress` removes only the three-cycle cap and its derived four-assessment
cap. It retains one initial assessment and at most one reassessment per started
cycle, the existing status-only protocol repair, cycle reservation and
preparation-failure accounting. No extra diagnosis at unchanged evidence is
allowed. A preparation failure leaving a prior finding unresolved still stops.

Implementation owns the factual ledger: mode, explicit selection and its scope,
allowance identity/authorization, current cycles used, historical counts,
finding IDs/dispositions/evidence, failed approaches and partial paths. Report
remaining cycles as `uncapped` in progress mode, never as zero or an invented
finite allowance. Unknown historical totals remain unknown. Drive passes the
selection and ledger to every affected implementation owner, including after
replan and capacity restart. Plan transports them without granting a new mode
or resolving implementation findings. Mapped descendants retain the mode and
one recovery identity; mode selection is never inferred from Git/Task status.

Receivers continuing the same run retain its evidenced mode even when their
dispatch omits a flag. Missing selection evidence does not imply progress mode;
missing necessary continuation data still stops. A new execution or a new
human-authorized resumption defaults to bounded unless progress is explicitly
selected for it. Do not persist the selection into configuration or another
Milestone. Ordinary automatic continuation is not a new execution.

An explicit mode change during an ongoing run preserves its used count, finding
ledger and latest assessment. Switching to bounded with three or more used
cycles stops further correction if blockers remain; switching to progress
cannot override a repeated finding or another stop condition. At a reported
bounded-limit stop, a request to resume using progress follows Decision 0226's
fresh initial assessment and retained-history rules with a new uncapped
allowance. Merely inspecting or mentioning the option grants no resumption.

This refines the fixed-cap/no-option portions of Decisions 0218 and 0226. There
is no schema, protocol verdict or CLI lifecycle change, and no weaker approval,
verification, release or host-permission boundary.

## Verification

Check both agent renderings, default and explicit mode selection, Drive/Plan
transport, and unchanged Direct/phase bounds. Fresh forward tests distinguish
a fourth cycle with proven progress from default exhaustion, partial resolution,
reopened findings, renamed symptoms, missing authority, preparation failure and
mode changes. Verify a real fourth-cycle review can complete normally without
granting release or changing approved scope.
