# Forward-test measurement ledger

[Back to the forward-test index](../skill-forward-tests.md). Scenario results remain historical measurements; a later pass does not erase an earlier failure.

## Latest run

Runs below span 2026-08-18 through 2026-08-24. The initial Claude Code suite was
measured against builds from `9f8ae39` through `f134915`; later targeted Codex
runs record their own builds below.

### Passing measurements

This table lists only recorded passes. A scenario absent from an agent's column
has no recorded pass for that agent; it does not mean failure. Runs that stopped
without a pass are listed separately below.

| Workflow area | Claude Code passes | Codex passes |
| --- | --- | --- |
| Discovery | D1, D2, D4–D6, D8–D12 | D4, D6 |
| Requirements | R1–R5 | R1, R3, R4 |
| Gap analysis | G1 | G1 |
| Checkpoint behavior | C1–C3 | C1–C3 |
| Steering | None recorded | S5 |
| Existing-implementation adoption | None recorded | A1, A2 |
| Design | None recorded | DS1 (workflow only; investigation dispatch was not exercised), DS2, DS3 |
| Tasks | T2 | T1, T2, T4 |
| Contract review | X3 | X1, X2, X4 |
| Implementation | None recorded | I1–I4 |
| Debug | None recorded | DB1 |
| Task review | None recorded | RT1, RT2 |
| Design validation | None recorded | VD1, VD2 |
| Implementation validation | None recorded | VI1–VI3 |
| Claim verification | None recorded | VC1, VC2 |
| Release | RL1 | RL1–RL4 |
| Planning orchestrators | None recorded | Q4 |
| End-to-end journey | None recorded | HP1 |

C2's dedicated-marker variant passed as Codex on `fb87bb9`. The fixture left the
Discovery milestone, Roadmap, cart state, and Brief uncommitted, made no commit
beyond fixture setup, and did not ask for checkpoint policy. Earlier passing
runs on `ec20755` and `3746108` exposed two usability findings before the final
measurement: marker precedence needed to state that the entire body is ignored,
and Discovery needed to repeat the completion check immediately before Brief
authoring. The final driver followed both rules and stopped in Requirements with
all gates `not_reached`.

C2 was re-measured on `7307f7a` after `scope/v1` began exposing its version as
`const: 1`. The driver read that schema, created the confirmed cart milestone
and Brief, recognized the exact adapter scaffold marker, made no commit, and
left only the Roadmap, cart state, and Brief uncommitted. R4 declined approval
after authoring a complete cart Requirements revision and left its gate
`not_reached`. DS3 stopped on the stale Requirements gate without changing the
Requirements, creating a Design, or invalidating evidence.

X4 and VD1 were first measured on `7307f7a`. X4 passed but exposed that the
contract-review skill called the CLI's `not_applicable` result "not required".
VD1 returned `NOT_READY` for the intended Research dependency but also treated
retained inactive Requirement IDs as missing Design scope. After both readings
were fixed in `3d887b6`, fresh X4 and VD1 runs passed: X4 stopped immediately
without a review artifact, and VD1 judged only the active 4/4 set, reported the
Research deferral, and changed no artifact or lifecycle state.

S5 was measured on 2026-08-21 as Codex with `gpt-5.6-terra` at medium
reasoning. The first run on `3c1b91b`, and a metadata-only retry on `81cc473`,
classified the testing-guidance request as ordinary documentation and wrote a
root `TESTING.md`. After project instructions routed durable project-wide
guidance to Steering, the `d10e05e` run selected `testing` correctly but copied
only part of the scaffold's `maintain` comment and added an unrequested test
suite. The fresh `c6d21fd` run passed: it listed Steering before choosing the
noncolliding `testing` identity, wrote only `steering/testing.md`, omitted the
`create` comment, preserved the complete 658-byte `maintain` comment exactly,
and finished with successful list, projected-read, and diff checks. A read-only
debrief left the fixture state unchanged.

A1 and A2 were measured on 2026-08-24 as Codex with fresh `gpt-5.6-terra`
medium drivers against `d9a1833`. A1 stopped on
`ADOPTION_STEERING_REQUIRED`, named the separate Steering bootstrap route, and
left no dossier, milestone, or Spec. A2 returned the full fixture HEAD
`c15a2fb288168d60d766746c666fe243128d59db`, read all four Steering documents,
and proposed separate `cart` and `order` Specs with their dependency, selected
scope, unmanaged area, and uncertainties. It stopped at the first confirmation
with no dossier, milestone, Spec, Brief, or Research. Both worktrees remained
clean; A2's instrumentation recorded the driver and two fresh readers.

RL3 and RL4 were re-measured on 2026-08-21 with fresh Codex drivers after the
release-policy bootstrap and finalization-checkpoint contract changed. RL3
passed on `d51a12a`: the Front Matter-only Release adapter was listed as active,
the already bound `v1.4.0` milestone finalized without a publication claim, and
one separate local commit contained only the lifecycle archive, log, and state
changes. RL4 passed on `447c0c6`: the driver found root `RELEASING.md`, proposed
the local-tag policy with its exact pre-finalization target, obtained
configuration-only approval, committed only the Release adapter, and stopped
without binding, tagging, publishing, pushing, or finalizing. The resulting
settings commit correctly made the accepted `cart` completion evidence stale.

HP1 passed on 2026-08-22 with a fresh `gpt-5.6-terra` medium Codex driver against
`4ce7e87`. The journey confirmed Discovery, delegated `specbind-quick-plan`
Requirements/Design/Tasks gates, independent Design validation before approval,
two reviewed implementation tasks, the expected pre-completion release blocker,
exact completion evidence, Publish confirmation, detached tagged-tree testing,
and finalization. The judge passed every expectation at final commit
`b0a5eed605572b01ab93d27cb17c407b357fcbf3`; annotated tag `v1.4.0` points to
`85adbb501a942249c94ca1dd7b525adbcdf6083f`, before finalization, and the fixture
recorded eleven dispatch contexts. The final fixture was clean, had no remote,
and retained the required release archives and cart log.

### Runs without a passing measurement

| Scenario | Agent | Result | Why no pass was recorded |
| --- | --- | --- | --- |
| D3 | Claude Code | Not measured | The confirmation authorized the whole feature, so later phases rewrote the discovery artifacts before they could be judged. |
| D7 | Claude Code | Not measured | No embedded `specbind-tasks` skill owned plan authoring at the time; the run correctly stopped. |
| D7 | Codex | Environment blocked | The agent stated the correct rewind cost, but the host safety review rejected the confirmed invalidation twice. |
| R1 | Codex | Scenario blocked | The fixture says only that customers can cancel "eligible orders", but never defines eligibility. The Requirements review protocol requires an unknown product expectation to be escalated rather than guessed, so the agent correctly stopped without authoring. |
| R1 | Codex | Environment blocked | After the fixture ambiguity was repaired in `55518ce`, the driver approval mechanism rejected the fixture-required instrumentation write twice, including after the parent explicitly authorized that write. No product workflow ran. |
| C1 | Codex | Product failure | On `9cce3de`, the agent read the quantity limit as ordinary work, bypassed Discovery, and edited `src/cart.py` plus tests. The project instruction admitted that reading; `59ebc5f` clarified the boundary and the fresh C1 run passed. |
| S5 | Codex | Product failure | On `3c1b91b` and `81cc473`, durable testing guidance bypassed Steering and became root `TESTING.md`; `d10e05e` routed correctly but partially copied `maintain` and expanded into test implementation. The fresh `c6d21fd` run passed. |
| R1 | Codex | Operator stopped | On `9cce3de`, the run produced a valid Requirements draft, but the controller began the usability debrief instead of continuing the required explicit-approval turn. The draft is evidence of neither a pass nor a product failure. |
| S2 | Codex | Environment blocked | On `4738ca2`, bootstrap reached its required three-reader dispatch, but stale host agent threads exhausted the global limit. Steering remained empty and unchanged; no product authoring ran. |
| T1 | Codex | Environment blocked | On `cc37049`, the corrected rule produced a one-task implementation-and-test proposal, but the host safety layer rejected `tasks.yaml` authoring twice, including after explicit Tasks approval. No artifact was written, so this is not a passing remeasurement. |
| D6 | Codex | Product failure | On `4256ab3`, the first Discovery correctly left its new Roadmap uncommitted under an unfilled Git adapter. The confirmed same-session addition then failed with `MILESTONE_ROADMAP_DIRTY`, leaving the original milestone and `cart`-only scope unchanged and creating no `order` Spec. |
| VD1 | Codex | Product failure | On `7307f7a`, the validator returned the expected `NOT_READY` for Research dependence but also raised inactive Requirements 2.1–2.2 as blocking Design omissions. The fresh `3d887b6` run scoped judgment to the active 4/4 set and passed. |
| RL3 | Codex | Product failure | On `6a29ad7`, a fresh driver misclassified the explicit Front Matter-only Release adapter as an unconfigured scaffold. The classification order was made explicit and the fresh `f069aef` run finalized correctly. |
| RL4 | Codex | Product failure | On `f069aef`, a fresh driver concluded that release documentation was absent without inspecting root `RELEASING.md`. `447c0c6` requires root release-document enumeration before that conclusion; the fresh run found it and passed. |
| HP1 | Codex | Product failure | On `2ec33fd`, a `gpt-5.6-terra` medium driver stopped for Discovery confirmation without presenting the whole scope plan its owning skill requires. The clean fixture stayed at commit `8f546b55b5631c0b070a014d9a3e8d6a2215161d` with no milestone or tag, `cart` idle, and one dispatch-log context. The missing plan made the first confirmation unapprovable, so no later journey phase was run. |
| HP1 | Codex | Environment blocked | On `ef536c8`, the temp-directory driver was denied the required fixture-local dispatch-log write twice, including after the existing user authority was relayed. No product workflow ran. |
| HP1 | Codex | Product failure | On `ef536c8`, a fresh workspace-local `gpt-5.6-terra` medium driver named the `cart` update but again omitted explicit empty New Specs, invalidations, and dependencies from its confirmation payload. It also tried `milestone create --scope` with prose before confirmation; the CLI rejected the path-like argument. The clean fixture stayed at `9f12ed713d4fe97842bca25583d51bb8408aaa17` with no milestone or tag, `cart` idle, and one dispatch-log context. |
| HP1 | Codex | Product failure | On `4b44b63`, a fresh `gpt-5.6-terra` medium driver inferred that the initial "Ship" request approved scope before the required later confirmation, then created and committed the `cart` Discovery milestone. The fixture stopped clean at `753c44e6ab5cb53fa6e0d1909ebb915b935e5af0`, with `cart` in Requirements, no tag, and one dispatch-log context; no later HP1 phase was authorized. |
| HP1 | Codex | Product failure | On `9d56c69`, a fresh `gpt-5.6-terra` medium driver correctly presented all four fields and made no lifecycle mutation, but declared `cart update -> release v1.4.0` as a dependency even though a release label is not a Roadmap work item. The clean fixture stayed at `81d7cdbfedb77c23d58f8f701f6967ac461c66f4` with no milestone or tag, `cart` idle, and one dispatch-log context. |
| HP1 | Codex | Product failure | On `27a0a76`, Discovery passed and committed the confirmed `cart` scope, but the same `gpt-5.6-terra` medium driver routed the subsequent named-item planning request to `specbind-batch-plan` instead of `specbind-quick-plan`. The fixture stayed clean at `ef70a903f3c1b54fcbeabf1044abb518137dce3b`, with `cart` in Requirements, no tag, and one dispatch-log context; no planning gate was authored or approved. |
| HP1 | Codex | Product failure | On `9895438`, Discovery and the delegated `specbind-quick-plan` gates passed, but the approved plan separated quantity behavior from its automated coverage while the canonical `scripts/test.sh` interface did not yet exist. Task 1.1 therefore could not satisfy the implementation protocol's verification requirement. The run stopped at clean checkpoint `c7019d797cc203c98b04d5fa458069f973868259` plus the reported partial `src/cart.py` edit and generated `src/__pycache__/`, with `cart` still in implementation, no tag, and three dispatch-log contexts. |
| HP1 | Codex | Product failure | On `0c6ef43`, Discovery and Requirements passed, but independent Design validation found that the authored Design required the absent `sh scripts/test.sh` interface while claiming the change was confined to `src/cart.py`. After one environment-permission retry for the fixture's ignored dispatch instrumentation, the fresh validator returned `NOT_READY`. The fixture stopped at `754e550dcce6f7b6e453aea6d0cb364eeebde739` with an untracked Design, `cart` still in Design, no tag, and three dispatch-log contexts. |
| HP1 | Codex | Product failure | On `493acfe`, the corrected Design and one-task implementation-and-test plan passed all delegated planning gates, but `specbind-quick-plan` reported success without the adapter-directed Requirements and Tasks checkpoints. The fixture stopped at `744feae8e7065fa01361e7d3118e17fd2875af6b` with modified Requirements and `spec.yaml`, untracked `tasks.yaml`, `cart` in implementation, no tag, and six dispatch-log contexts. |
| HP1 | Codex | Product failure | On `e51ca0a`, planning checkpoints and implementation passed, but completion acceptance persisted the placeholder smoke command `PYTHONDONTWRITEBYTECODE=1 py -3 -c smoke-cart` instead of the full command actually executed. The same run also approved Design before its independent validation, making the later `READY` verdict retroactive rather than a prerequisite. The clean fixture stopped at `cc4bbd793ec9be45a51d83bac22d1ea223d83f22` with `cart` in `release_ready`, no tag, and twelve dispatch-log contexts; release was not authorized after the evidence mismatch was found. |
| HP1 | Codex | Product failure | On release candidate build `f107d95`, Discovery, delegated planning, independent Design validation with one bounded remediation, implementation, review, and release binding all completed with clean checkpoints. At the exact `Is the cart work done?` boundary, the driver answered from `spec status` that completion was `not_reached` instead of invoking implementation validation. The fixture stopped clean at `ef81f50` with one completed Task, fresh Requirements/Design/Tasks and Contract Review, `cart` still in `implementation`, no tag, and no finalization. The installed project instructions and validation-skill description were strengthened before a fresh rerun. |
| HP1 | Codex | Product failure | On release candidate build `5f74154`, the corrected routing invoked implementation validation at `Is the cart work done?`, but task verification had left known generated `src/__pycache__/` and `tests/__pycache__/` paths. The completion preflight therefore stopped with `COMPLETION_WORKTREE_DIRTY`. The fixture remained at binding commit `030d2cf`, with one completed Task, fresh earlier gates and Contract Review, no completion evidence, no tag, and no finalization. The task-implementation protocol and orchestrator were tightened to require a clean generated-output handoff before `READY_FOR_REVIEW`. |

Scenarios not named in either table have not produced a recorded result for
either agent. The tables are a measurement ledger, not a coverage checklist.

### Open usability findings

| First seen | Scenario | Finding | Status |
| --- | --- | --- | --- |
| `ef536c8` | HP1 | Two fresh drivers read the project instruction's installed `specbind-status` Skill as a CLI command (`specbind-status` or `specbind status`) before finding the on-disk Skill and using its actual status reads. | Reproduced on `4b44b63`; investigate how the project instruction distinguishes Skill invocation from CLI syntax without teaching a platform-specific command form. |
| `4ce7e87` | HP1 | The Git adapter says each completed implementation Task is an eligible workflow unit, while `specbind-implement` reaches its checkpoint only after all requested task outcomes are recorded; the passing driver combined two task completions into one commit. | Investigate checkpoint timing ownership before the next implementation-forward-test batch; HP1's artifact and release judge passed, but it does not assert one commit per Task. |

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
| `scope/v1` exposed only `minimum: 0` for `schemaVersion`, so an author had to infer the version from the selector. | The generated schema now fixes `schemaVersion` with `const: 1`, matching runtime acceptance and the other v1 schemas; C2 authored the candidate from the corrected schema. | `6d1d2e5`, confirmed on `7307f7a` |
| Design validation could treat Requirements retained outside the active milestone set as missing Design scope. | Validation fixes the review scope from status and traceability before reading prose and treats inactive Requirements as context only. | `3d887b6`, confirmed on `3d887b6` |
| Contract review described the Direct-only stop as `not required`, while the public CLI prints `Status: not_applicable`. | The skill now names the exact public status and explains that it means no review is required. | `3d887b6`, confirmed on `3d887b6` |
| Adapter state overloaded the template-only `specbind:instruction` token, used a raw substring check, and required a deferred-specific compatibility exception. | Inactive adapters use the exact Markdown comment `<!-- specbind:adapter-scaffold -->`; marker-like prose, code, longer comments, and the template token are ordinary adapter content. C2 confirmed marked Git policy opts out without asking or committing. | `ec20755`, confirmed on `fb87bb9` |
| An untouched Release scaffold could be treated as an explicit no-op and let a first invocation finalize without configuring project policy. | Release now derives a complete adapter from repository evidence, obtains configuration-only approval, checkpoints only that adapter, invalidates affected completion evidence, and stops. | `a576cf6`, confirmed on `447c0c6` |
| Successful `release finalize` left its lifecycle archive and log mutations outside a checkpoint. | Release snapshots the worktree before finalization, then uses active Git policy to create a separate local commit containing only newly changed finalization lifecycle paths; publication approval does not authorize pushing it. | `a576cf6`, confirmed on `d51a12a` |
| Empty adapter content and an installed scaffold could both be reported as `state=scaffold`, contradicting the dedicated-marker contract and the Release empty-body meaning. | Skill classification checks the exact read result first, and `adapter list` now reports scaffold only for the exact dedicated marker; a Front Matter-only adapter is active. | `f069aef`, `d51a12a`, confirmed on `d51a12a` |
| Release bootstrap could declare policy evidence absent after reading only `README.md`, even when root `RELEASING.md` existed. | Bootstrap enumerates root release-document candidates such as `RELEASE*`, `RELEASING*`, and `CHANGELOG*` before concluding that evidence is absent. | `447c0c6`, confirmed on `447c0c6` |
| A marked adapter could retain actionable-looking scaffold text, leaving precedence implicit. | Every consuming Skill states that the marker classifies the whole document and all remaining body lines are ignored. | `3746108`, confirmed on `fb87bb9` |
| Discovery read the authoring protocol before a Brief but did not repeat its completion-state check after applying milestone scope. | Discovery now runs `milestone status` after reading `okf-authoring` and immediately before the first Brief write. | `fb87bb9`, confirmed on `fb87bb9` |
| An inactive installed Git scaffold left every accepted phase uncommitted and blocked a same-session scope addition at the Roadmap target guard. | New installs receive active local-checkpoint policy; C1 confirmed the narrow first commit and D6 continued through `update-scope` with the milestone identity, existing item, and Roadmap body preserved. | `c3a0ccf`, confirmed on `8a58244` |
| Discovery drivers twice tried to invalidate Requirements for an idle Spec merely because its Requirements artifact existed. | Discovery now states that `not_reached` is not approved and that artifact presence is not gate evidence; the fresh C1 and D6 drivers proposed no invalidation. | `8a58244` |
| An established Spec with no Design had no explicit target-resolution branch. | Design now branches on whether the Design set exists and resolves the selected template's target path. | `398bbf7` |
| An unconsumed export warning could be read as authority to delete a stable seam. | Design preserves unchanged exports and requires evidence for a consumer or an explicit boundary change. | `9cce3de` |
| Milestone health treated a future Contract review as a present inconsistency. | Milestone health is phase-relative while stale or invalid reviews remain inconsistent. | `17fa76a` |
| `milestone create --scope -` required the agent to infer the scope document shape. | Discovery reads `scope/v1` before authoring the candidate. | `8646136` |
| A quantity-limit change could be classified as ordinary implementation work. | Project instructions explicitly route observable validation rules, limits, rejected cases, and genuine uncertainty into SpecBind. | `59ebc5f` |
| Durable project-wide guidance such as testing could be classified as ordinary documentation without opening the Steering skill. | Project instructions route testing, API, security, and deployment conventions to `specbind-steering`, independent of Spec or behavior changes. | `d10e05e`, confirmed on `c6d21fd` |
| A materializer could excerpt a durable scaffold instruction or change project state merely to make new guidance true. | Steering treats each durable comment as an indivisible byte-for-byte block, verifies it against the scaffold, and forbids supporting implementation outside the documentation request. | `c6d21fd`, confirmed on `c6d21fd` |
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
