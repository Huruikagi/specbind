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
| Discovery | D1, D2, D4–D6, D8–D12 | None recorded |
| Requirements | R1–R5 | R1, R3 |
| Gap analysis | G1 | G1 |
| Checkpoint behavior | C1–C3 | C1, C3 |
| Design | None recorded | DS1 (workflow only; investigation dispatch was not exercised), DS2 |
| Tasks | T2 | T2, T4 |
| Contract review | X3 | X1, X2 |
| Implementation | None recorded | I1–I4 |
| Debug | None recorded | DB1 |
| Task review | None recorded | RT1, RT2 |
| Design validation | None recorded | VD2 |
| Implementation validation | None recorded | VI1–VI3 |
| Claim verification | None recorded | VC1, VC2 |
| Release | RL1 | RL1–RL3 |

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

Scenarios not named in either table have not produced a recorded result for
either agent. The tables are a measurement ledger, not a coverage checklist.

### Usability observations

The first post-run debrief batch ran on 2026-08-21 against `a5c14c8`, after the
R1, DS2, and C1 fixtures had already been judged. `git status --short` was
identical before and after every debrief.

The R1 and DS2 drivers answered in Japanese despite their English fixtures, the
documented signal that host instructions may still be visible. C1 answered in
English; its checkpoint verdict was also re-read directly from the isolated
fixture, including the unchanged Git history.

The next Codex batch measured C1, DS2, and R1 on `9cce3de`. DS2 passed its
artifact expectations. C1 failed by bypassing the workflow, and R1 was stopped
by the controller at its valid draft boundary before approval. After the entry
wording fix, fresh C1 and R1 fixtures on `59ebc5f` passed. All five debriefs were
read-only: `git status --short` was identical before and after each one. Japanese
answers in both R1 runs and the first DS2 run were retained as host-contamination
signals rather than treated as fixture-local evidence.

| Build | Scenario | Agent | Surface | Impact | Observation and evidence | Workaround | Contract check |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `a5c14c8` | R1 | Codex | Protocol | wrong-action-risk | The brief says only "eligible orders" while the Requirements review protocol says an unknown expectation must be escalated, not filled with a plausible guess. | The agent stopped without creating Requirements. | Reproduced; this is a scenario contradiction, not a product defect. |
| `a5c14c8` | R1 | Codex | CLI | extra-step | The agent guessed `specbind spec show order --include-body`, which does not exist. | It read `spec --help`, then used `spec status` and `artifact read`. | Not a contract gap: the project instructions already say to run `specbind --help` when a command is unfamiliar. |
| `a5c14c8` | R1 | Codex | Other | ambiguity | The harness says to append its instrumentation record "Before doing anything else", although that direction is learned only by reading `AGENTS.md`. | It appended the record immediately after reading the instruction. | Reproduced as harness wording only; it did not affect product behavior. |
| `a5c14c8` | DS2 | Codex | Skill | ambiguity | `artifact list cart` reported no Design, but the Existing Spec branch says only to "revise the current design artifacts in place". | The agent borrowed the New Spec path-discovery commands, resolved `design/main`, and created the reported target. | Reproduced; the established-Spec-with-no-Design state has no explicit branch. |
| `a5c14c8` | DS2 | Codex | CLI | wrong-action-risk | `check contracts` warned that `add-item` was unconsumed and said to confirm an external consumer or retire the seam, but the fixture provided no evidence with which to decide that. | The agent preserved the stable existing export. | Reproduced; preserving all four stable IDs was also the scenario requirement. |
| `a5c14c8` | C1 | Codex | Other | wrong-action-risk | The agent read the `specbind-status` skill name as a `specbind status` CLI command and got an unknown-subcommand error. | It used `specbind milestone status`. | Not reproduced as written: `AGENTS.md` names the skill, not that CLI command. The similar names remain the observed source of the misread. |
| `a5c14c8` | C1 | Codex | Skill | ambiguity | The driver confirmed "discovery for this cart change only" while the skill asks for explicit agreement to the whole plan. | The agent treated the phase-limited confirmation as authorization for the plan it had just presented. | Harness wording, not a demonstrated product defect. |
| `a5c14c8` | C1 | Codex | CLI | extra-step | `milestone create --help` names `--scope <SCOPE>` but neither it nor the skill shows the JSON scope shape; the first stdin attempt reached EOF. | The agent inferred and supplied the scope document. | Reproduced; the skill shows `--scope -` without a complete input example. |
| `a5c14c8` | C1 | Codex | CLI | wrong-action-risk | After successful discovery, `milestone status` reported `Health: inconsistent` and `CONTRACT_REVIEW_MISSING` even though its actionable phase was Requirements. | The agent trusted `Actionable: spec:cart action=requirements` and stopped at the requested boundary. | Reproduced; `spec status cart` was phase-relative and consistent, but milestone health still treated the later contract review as an inconsistency. |
| `9cce3de` | C1 | Codex | Protocol | wrong-action-risk | The project instruction's “changes what a Spec owns” wording let a per-SKU quantity limit be classified as a small ordinary implementation change. | The agent implemented the cap directly. | Reproduced from the fixture; fixed in `59ebc5f` by naming observable validation rules, limits, and rejected cases and routing genuine uncertainty into Discovery. |
| `9cce3de` | C1 | Codex | Other | ambiguity | The request did not prescribe an exception type, message, or mutation behavior on rejection. | The agent invented `ValueError` and pre-mutation validation. | Not a direct-work contract gap: these are Requirements and Design decisions the bypassed workflow should have owned. |
| `9cce3de` | C1 | Codex | Other | extra-step | The minimal fixture had no test framework or dependency manifest. | The agent added `unittest` coverage and removed generated caches. | Fixture characteristic only; it became irrelevant when the corrected entry rule prevented implementation. |
| `9cce3de` | DS2 | Codex | CLI | extra-step | The agent reported that `template resolve` was unavailable and used `template list` to find `technical-design/main.md`. | It read `output_path` from the listing. | Not reproduced with the fixture binary, which exposes `template resolve`; the driver picked up an older host CLI, so this is environment contamination. |
| `9cce3de` | DS2 | Codex | Skill | wrong-action-risk | The request supplied no authority to approve the Design gate. | The agent left a validated draft and did not approve. | Expected guarded-boundary behavior, not a defect; DS2's artifact expectations passed. |
| `9cce3de` | R1 | Codex | CLI | extra-step | The agent treated the `specbind-status` skill name as a `specbind status` command. | It read help and used `specbind spec status order`. | The fixture contains the named skill; the Japanese response and host command lookup are environment-contamination signals. |
| `9cce3de` | R1 | Codex | Skill | ambiguity | The driver said the installed `specbind-*` skills were absent from its available-skill list. | It authored from the CLI, Brief, steering, and template. | The fixture contains `.agents/skills/specbind-requirements`; dynamic skill discovery was not exposed to this spawned driver, so the run did not prove the skill path. |
| `9cce3de` | R1 | Codex | Template | wrong-action-risk | The Brief names an open and closed cancellation window but not its duration. | The agent left duration out of scope and specified behavior against the open/closed predicate. | Not a product gap: R1 deliberately supplies the observable boundary without asking Requirements to invent a duration. |
| `59ebc5f` | C1 | Codex | Skill | extra-step | The agent tried requirements invalidation before scope creation even though the idle Spec held no approved gate. | It accepted `SPEC_REQUIREMENTS_STATE_INVALID` and continued with milestone creation. | Not a contract gap: Discovery limits invalidation to Specs that already hold approved gates; the agent combined two separate ordering rules. |
| `59ebc5f` | C1 | Codex | CLI | ambiguity | The expected dirty Discovery result appeared among general release blockers as `WORKTREE_NOT_CLEAN`. | It compared the dirty paths with its own outputs and treated milestone health as authoritative. | Reproduced but not a health contradiction: the report says `Health: consistent`; release blockers describe later release readiness. |
| `59ebc5f` | R1 | Codex | Protocol | wrong-action-risk | The cancellation-window duration was unspecified. | The agent specified behavior only while the window is open and after it closes. | Not a defect for the same R1 boundary reason recorded above. |
| `59ebc5f` | R1 | Codex | Skill | ambiguity | The standard confirmation approved the Requirements but did not explicitly name the active Requirement ID selection. | The agent inferred all four authored criteria as active. | Reproduced in the harness wording; the confirmation now approves both the presented document and presented selection and forbids filling in an omitted value. |

A completed debrief with no finding is recorded as `none`, so absence of a row
is not mistaken for an uneventful run.

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
