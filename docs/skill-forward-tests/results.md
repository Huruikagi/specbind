# Forward-test measurement ledger

[Back to the forward-test index](../skill-forward-tests.md). Scenario results remain historical measurements; a later pass does not erase an earlier failure.

## Latest run

Runs below span 2026-08-18 through 2026-08-21. The initial Claude Code suite was
measured against builds from `9f8ae39` through `f134915`; later targeted Codex
runs record their own builds below.

### Passing measurements

This table lists only recorded passes. A scenario absent from an agent's column
has no recorded pass for that agent; it does not mean failure. Runs that stopped
without a pass are listed separately below.

| Workflow area | Claude Code passes | Codex passes |
| --- | --- | --- |
| Discovery | D1, D2, D4–D6, D8–D12 | D4, D6 |
| Requirements | R1–R5 | R1, R3 |
| Gap analysis | G1 | G1 |
| Checkpoint behavior | C1–C3 | C1, C3 |
| Design | None recorded | DS1 (workflow only; investigation dispatch was not exercised), DS2 |
| Tasks | T2 | T1, T2, T4 |
| Contract review | X3 | X1, X2 |
| Implementation | None recorded | I1–I4 |
| Debug | None recorded | DB1 |
| Task review | None recorded | RT1, RT2 |
| Design validation | None recorded | VD2 |
| Implementation validation | None recorded | VI1–VI3 |
| Claim verification | None recorded | VC1, VC2 |
| Release | RL1 | RL1–RL3 |
| Planning orchestrators | None recorded | Q4 |

### Runs without a passing measurement

| Scenario | Agent | Result | Why no pass was recorded |
| --- | --- | --- | --- |
| D3 | Claude Code | Not measured | The confirmation authorized the whole feature, so later phases rewrote the discovery artifacts before they could be judged. |
| D7 | Claude Code | Not measured | No embedded `specbind-tasks` skill owned plan authoring at the time; the run correctly stopped. |
| D7 | Codex | Environment blocked | The agent stated the correct rewind cost, but the host safety review rejected the confirmed invalidation twice. |
| R1 | Codex | Scenario blocked | The fixture says only that customers can cancel "eligible orders", but never defines eligibility. The Requirements review protocol requires an unknown product expectation to be escalated rather than guessed, so the agent correctly stopped without authoring. |
| R1 | Codex | Environment blocked | After the fixture ambiguity was repaired in `55518ce`, the driver approval mechanism rejected the fixture-required instrumentation write twice, including after the parent explicitly authorized that write. No product workflow ran. |
| C1 | Codex | Product failure | On `9cce3de`, the agent read the quantity limit as ordinary work, bypassed Discovery, and edited `src/cart.py` plus tests. The project instruction admitted that reading; `59ebc5f` clarified the boundary and the fresh C1 run passed. |
| R1 | Codex | Operator stopped | On `9cce3de`, the run produced a valid Requirements draft, but the controller began the usability debrief instead of continuing the required explicit-approval turn. The draft is evidence of neither a pass nor a product failure. |
| S2 | Codex | Environment blocked | On `4738ca2`, bootstrap reached its required three-reader dispatch, but stale host agent threads exhausted the global limit. Steering remained empty and unchanged; no product authoring ran. |
| T1 | Codex | Environment blocked | On `cc37049`, the corrected rule produced a one-task implementation-and-test proposal, but the host safety layer rejected `tasks.yaml` authoring twice, including after explicit Tasks approval. No artifact was written, so this is not a passing remeasurement. |
| D6 | Codex | Product failure | On `4256ab3`, the first Discovery correctly left its new Roadmap uncommitted under an unfilled Git adapter. The confirmed same-session addition then failed with `MILESTONE_ROADMAP_DIRTY`, leaving the original milestone and `cart`-only scope unchanged and creating no `order` Spec. |

Scenarios not named in either table have not produced a recorded result for
either agent. The tables are a measurement ledger, not a coverage checklist.

### Open usability findings

None. This section is the current worklist, not an append-only transcript.

### Fixed, behavioral confirmation pending

| First seen | Scenario | Finding | Resolution | Status |
| --- | --- | --- | --- | --- |
| `4738ca2` | T1 | The default task rule told projects to choose a test-grouping convention but did not choose one, so the planner had to decide whether one behavior needed a separate test task. | `cc37049` defaults tests into the behavior task and permits a separate verification task only across several earlier tasks or a separately reviewable system boundary. | A fresh driver proposed the expected combined task, but host safety blocked artifact authoring; rerun T1 when that environment stop is absent. |

### Resolved usability findings

Resolved rows retain only the behavior that changed and the build carrying the
fix. Detailed observations, discarded non-defects, and fixture-only workarounds
remain available in Git history.

| Finding | Resolution | Fixed in |
| --- | --- | --- |
| An inactive installed Git scaffold left every accepted phase uncommitted and blocked a same-session scope addition at the Roadmap target guard. | New installs receive active local-checkpoint policy; C1 confirmed the narrow first commit and D6 continued through `update-scope` with the milestone identity, existing item, and Roadmap body preserved. | `c3a0ccf`, confirmed on `8a58244` |
| Discovery drivers twice tried to invalidate Requirements for an idle Spec merely because its Requirements artifact existed. | Discovery now states that `not_reached` is not approved and that artifact presence is not gate evidence; the fresh C1 and D6 drivers proposed no invalidation. | `8a58244` |
| An established Spec with no Design had no explicit target-resolution branch. | Design now branches on whether the Design set exists and resolves the selected template's target path. | `398bbf7` |
| An unconsumed export warning could be read as authority to delete a stable seam. | Design preserves unchanged exports and requires evidence for a consumer or an explicit boundary change. | `9cce3de` |
| Milestone health treated a future Contract review as a present inconsistency. | Milestone health is phase-relative while stale or invalid reviews remain inconsistent. | `17fa76a` |
| `milestone create --scope -` required the agent to infer the scope document shape. | Discovery reads `scope/v1` before authoring the candidate. | `8646136` |
| A quantity-limit change could be classified as ordinary implementation work. | Project instructions explicitly route observable validation rules, limits, rejected cases, and genuine uncertainty into SpecBind. | `59ebc5f` |
| Forward-test instrumentation and phase confirmation could block or over-authorize a run. | Dispatch logging is opt-in, and guarded confirmation names the presented phase inputs and stopping boundary. | `694dca4`, `3ab817a` |
| A newly discovered Spec reported missing Requirements as inconsistent state. | Requirements absence is expected work in the Requirements phase; Spec and milestone health stay consistent while strict traceability remains unchanged. | `a8cae47` |
| Discovery's expected dirty output appeared as a present Release blocker. | Worktree cleanliness is reported only when a clean revision would unlock current progress; release readiness is not evaluated before Validation. | `475f144` |

### Active environment limitation

Codex subagents can inherit host instructions, an older host CLI on `PATH`, or a
skill registry that does not expose the fixture's installed skills. Stale agent
threads can also exhaust nested-dispatch capacity, and the host safety layer can
mistake skill-authored `tasks.yaml` plan content for prohibited CLI-owned
execution-state editing. Japanese answers in an English fixture are one visible
signal. These observations do not become product findings: judge the scenario
from fixture state, and classify a run as environment-blocked when the
product-managed skill was not exercised.

The 2026-08-21 Codex batch on `4738ca2` measured D4 and T1 as passes and stopped
S2 for the agent-thread environment limit. D4 exposed phase-relative
Requirements health, and T1 exposed the missing test-grouping default. After
both fixes, D4 passed again on `cc37049`; the fresh T1 driver proposed the
correct combined task but was environment-blocked before it could author the
plan.

C1 was re-measured on `475f144` as Codex after the phase-relative worktree
change. Discovery produced one uncommitted `cart` Spec update and Brief, changed
neither Requirements nor implementation, and created no commit. Milestone
status reported consistent Requirements work and `Release readiness: not
evaluated until validation` without `WORKTREE_NOT_CLEAN`.

C1 and D6 were re-measured on `8a58244` as Codex after the installed Git adapter
became active default policy. C1 created one local commit containing only the
Roadmap, `cart/spec.yaml`, and its Brief; the worktree was clean, the branch was
unchanged, and no remote existed. D6 then proved the continuation that had failed
on `4256ab3`: the same-session addition kept the milestone ID, the original
`cart` item and `# Roadmap` body, added the `cancellation` Spec, and committed
only its state, Brief, and the updated Roadmap. Requirements and implementation
remained untouched in both scenarios.

The next Codex batch measured D6, Q4, and RL2 on `4256ab3`. D6 failed because
the uncommitted Roadmap produced by its first Discovery could not pass the
confirmed `update-scope` target guard. Q4 passed: one explicit delegation drove
the single `cart` Spec through all three planning gates, accepted one fresh
Contract Review before Tasks approval, and stopped before implementation. RL2
passed again by creating the approved local tag, failing origin verification,
and preserving the active release-ready milestone without logs or archives.

D5 failed first and passed after the framing rule was corrected. R5 was blocked
once by a recipe that built a state its own request contradicted, and passed
after the recipe was fixed.

T2, X3, and RL1 were measured separately on 2026-08-19 against `366eb39`, as
Claude Code, and all three passed. They were selected as the newest rules with
the most expensive failures, one per skill. Two findings came out of them: the
tasks skill said nothing about YAML quoting, and the T2 row was measuring a stop
rather than the ordering that stop was protecting. A third observation is
recorded in the driving rules above, because one run installed packages into the
host environment.

R3 and G1 were re-measured on 2026-08-20 against `e1f024d`, as Codex, and
both passed. R3 left the established Requirements and gate untouched when asked
to retire behavior. G1 read the Brief and full Roadmap scope before Requirements
existed, dispatched two independent repository reads, and created no
Requirements, Research, or gate evidence. The first G1 attempt reported in
Japanese against the English fixture and was discarded as host-instruction
contamination; a fresh fixture and session produced the recorded pass.

DS1, T2, and X1 were re-measured on 2026-08-20 against `65bdc89`, as Codex, and
all three passed their workflow expectations. X1 accepted a fresh
Contract Review from only the Roadmap scope and Contract inputs, and created no
task plan. T2 stopped before authoring because the review was absent, preserving
the required ordering. DS1 produced a 3/3 traceable design and coherent
Contract, obtained explicit design approval, and created no task plan. Its
fixture was small enough that the driven context did not dispatch independent
investigation, so this is a workflow result rather than a measurement of the
design dispatch path.

DS1 was re-measured on 2026-08-21 against `de2a99d`, as Codex using the default
Terra/medium driver profile, after target-aware template resolution and
phase-relative status were added. Its precondition reported a consistent
unstarted Design with `Next action: design` and three expected coverage items.
The project-owned Design template lived at `technical-design/main.md`; the run
authored that exact target, did not create the conventional `design.md`, created
the embedded Contract target, stopped for explicit approval, and then ended at
`State: tasks` with Design fresh, traceability 3/3, a valid Contract graph, and
no `tasks.yaml`. Investigation dispatch was not exercised.

I2 and DB1 were measured on 2026-08-20 against `38920a0`, as Codex, and
both passed. I2 dispatched a fresh implementer and diagnosis, categorized the
approved Requirements/Design contradiction as `ARTIFACT`, left the task pending,
and changed neither artifact. DB1 returned the same category directly and left
the worktree byte-identical.

The Codex I3 run first exposed a harness defect: the scenario required a
completed Direct item while leaving the Git adapter scaffold in place, which
means commit nothing and makes the handshake's clean committed revision
unobtainable. The `i3` recipe gained real Direct checkpoint policy in
`1a10b00`. That build then
exposed a skill defect rather than passing: the run committed the guide and
stopped because the skill described checkpoint through a backward reference
from an earlier handshake section. After the workflow was made physically
linear, I3 passed against `d21590f`: the guide was committed, the Direct
handshake ran, and `milestone status` reported 1/1 completed. The initial
failure and its fix are retained here because a pass on retry is a finding, not
a flake.

VI1–VI3, VC1–VC2, and RL1–RL3 were measured as Codex on 2026-08-20. The final
passing builds were `18da4be` for VI2, `e76d36a` for VC1, `bf61d4e` for VI3,
VC2, RL1, and RL3, and `8421484` for VI1 and RL2.

The implementation-validation batch produced four findings before those final
passes. Natural "is this done?" wording first routed to status or the
consequence-free claim verifier; `7c8e573` and `18da4be` separated those three
routes. VI1 then exposed a fixture implementation that mutated an empty cart on
rejection and a validator ambiguity about Requirements retained outside the
active ID set; `e76d36a` fixed the fixture and `8421484` put the active-set rule
at the start of the skill. VI3 substituted the underlying Python runner when
the canonical test script was absent and incorrectly accepted completion;
`bf61d4e` made a missing canonical command an immediate
`MANUAL_VERIFY_REQUIRED`. The final runs left VI1 at `release_ready`, VI2 at
`implementation` with `NO-GO`, and VI3 at `implementation` with
`MANUAL_VERIFY_REQUIRED`.

VC1 and VC2 both preserved the consequence-free boundary: VC1 returned
`VERIFIED` without completion evidence, while VC2 returned `NOT_VERIFIED` for
the outstanding task and changed no lifecycle state.

RL1 stopped with `RELEASE_VERSION_UNBOUND`, and RL3 finalized `v1.4.0` with the
canonical delivered-change log entry and both milestone archives. RL2 first
exposed a recipe defect: committing its release adapter after completion
acceptance staled the evidence before release work began. `8421484` moved that
policy commit before acceptance. The rerun created the confirmed local tag,
could not verify it on the absent `origin`, and correctly left the milestone
active with no archive or log.

The next Codex batch used `gpt-5.6-terra` at medium reasoning. C1, T4, X2, and
I4 passed against `eba2faf`: discovery left its draft uncommitted, the Tasks
rewind preserved Requirements, Design, and the accepted Contract Review, the
cross-Spec review surfaced the out-of-scope `checkout` consumer without editing
it, and implementation left the unrelated dirty edit byte-for-byte intact.

Three findings were fixed and re-run. C3 first exposed a fixture defect: the
Brief helper copied its Problem into Desired outcome, contradicting the scoped
quantity cap. `476985b` gave every recipe an explicit desired outcome; C3 then
authored an uncommitted Requirements draft and left the gate unapproved when
approval was declined. VD2 first returned `NOT_READY` and then invalidated the
Design gate, deleting the accepted Contract Review despite the read-only
contract. `476985b` moved the no-invalidation rule before all commands; the
rerun returned the same verdict while preserving every gate and review record.
I1 first dispatched an implementer but requested an immediate return before
verification, leaving a partial change and a pending task. `d5878f7` made
waiting for the implementer's verified structured result explicit; the rerun
used three recorded contexts (driver, implementer, reviewer), completed the
task, and stopped in `implementation` without a completion handshake.

I1 was re-measured on 2026-08-21 against `4d9cb10`, as Codex, after Decision
0129 installed role capability adapters. The generated implementer and reviewer
roles both selected `gpt-5.6-terra` at medium reasoning. The run again recorded
exactly three contexts with standalone implementation and review briefs,
received `READY_FOR_REVIEW` and `APPROVED`, completed only Task 1, and stopped in
`implementation` without completion evidence. The fixture proves the installed
configuration and fresh dispatch behavior; it does not independently expose
the runtime model identity chosen by the host, so model selection itself remains
configuration evidence rather than a behavioral assertion.

D7 remains unmeasured under Codex: the agent correctly stated the Tasks rewind
cost, but the host safety review rejected the confirmed invalidation twice.
That is an execution-environment stop, not a product verdict. The Quick and
Batch additions were intentionally deferred after this batch produced three
actionable findings; the matrix remains a measurement record rather than a
coverage target.

RT1 and RT2 were measured as Codex on 2026-08-20 with `gpt-5.6-terra` at medium
reasoning. RT2 passed against `e543c07`, returning `CANNOT_REVIEW` for the
unrelated `src/orders.py` change and modifying neither file. RT1 returned the
correct `REJECTED` verdict on that build, but its Python probe created an
untracked `src/__pycache__/`, violating the review's read-only boundary.
`7b19472` moved the no-write rule before every probe and required before/after
status checks. The rerun rejected the off-by-one implementation, left the exact
recipe diff in place, kept the task pending, and generated no cache or report.

Eight product defects surfaced: the missing workflow-entry condition, its
missing new-responsibility rule, the framing unit, the unfilled-adapter stop,
two unpublished schemas, the invented delegation label, and a block that
forbade task-plan authoring. Four of them were re-run after the fix and
confirmed changed.
