# Forward-test measurement ledger

[Back to the forward-test index](../skill-forward-tests.md). Scenario results remain historical measurements; a later pass does not erase an earlier failure.

## Latest run

Runs below span 2026-08-18 through 2026-08-29. The initial Claude Code suite was
measured against builds from `9f8ae39` through `f134915`; later targeted Codex
runs record their own builds below.

### Passing measurements

This table lists only recorded passes. A scenario absent from an agent's column
has no recorded pass for that agent; it does not mean failure. Runs that stopped
without a pass are listed separately below.

| Workflow area | Claude Code passes | Codex passes |
| --- | --- | --- |
| Configuration | None recorded | CF2 |
| Discovery | D1, D2, D4–D6, D8–D12 | D4, D6, D13 |
| Requirements | R1–R5 | R1, R3, R4, R6 |
| Gap analysis | G1 | G1 |
| Checkpoint behavior | C1–C3 | C1–C3 |
| Steering | None recorded | S5 |
| Existing-implementation adoption | None recorded | A1, A2 |
| Design | DS3 | DS1 (workflow only; investigation dispatch was not exercised), DS2, DS3, DS5, DS7, DS8 |
| Tasks | T2 | T1, T2, T4 |
| Contract review | X1–X4 | X1, X2, X4 |
| Implementation | I3, I4 | I1–I4, I6 |
| Debug | DB1 | DB1 |
| Task review | RT1 | RT1, RT2 |
| Design validation | None recorded | VD1, VD2 |
| Implementation validation | VI2, VI3 | VI1–VI3 |
| Claim verification | None recorded | VC1, VC2 |
| Release | RL1–RL3 | RL1–RL4 |
| Planning orchestrators | None recorded | Q0, Q4, B0 |
| End-to-end journey | None recorded | HP1 |

X1, I3, RT1, and DB1 were re-measured on 2026-08-29 against `dc6c022`, driven
as fresh Codex subagents on `gpt-5.6-terra` at medium reasoning with one fixture
per scenario. All four passed their mechanical expectations. X1 kept contract
review absent, left `cart` at `tasks` with fresh Design, and wrote nothing. I3
committed only `CONTRIBUTING.md`, completed the pending Direct item, created no
Spec artifacts, and left only the CLI-owned Roadmap completion edit dirty because
the active adapter covers implementation paths only. RT1 returned `REJECTED`,
left the seeded `src/cart.py` diff byte-unchanged, and kept Task 1 pending. DB1
returned the exact `ARTIFACT` diagnosis for the Requirements/Design conflict,
kept Task 1 pending, and wrote nothing. Pre/post-debrief worktree status matched
in every fixture.

The debrief reproduced two follow-up findings against the installed assets. X1
first constructed `design/implementation` from a lifecycle action before
recovering through `artifact list`; I3 selected Steering before milestone lookup
revealed the matching Direct item. Decision 0160 assigns both precedence rules;
their fixing build is measured separately because this batch remains a
measurement of `dc6c022`. RT1's CRLF warning and DB1's driver-reporting collision
were environment or harness effects. DB1's unique-candidate lookup and optional
read-only behavior probe followed the skill contract and are not retained as
findings.

X1, I3, RT1, and DB1 were re-measured on 2026-08-29 against `1736d0c`, driven
as Claude Code Agent-tool subagents on `claude-opus-5` with no prior context and
one fresh fixture per scenario. All four passed, and each fixture was judged by
command and by a SHA-256 manifest compared against its pre-run snapshot rather
than from the driver's report.

X1 withheld acceptance for the undeclared 99-per-SKU guarantee: it read the
Contract at baseline and current revisions, found the Contract diff empty, and
used the existing `positive-quantity` invariant as the symmetric evidence that
the cap is a missing seam rather than an implementation detail. `milestone
review status` stayed `absent`, no `tasks.yaml` was created, `cart` stayed at
`tasks` with `design=fresh`, and every fixture file was byte-identical. It
stopped for the maintainer before any Design rewind. This reverses the `26518ee`
Claude Code failure and closes the two-readings divergence with Codex.

I3 read `milestone status` as its third command, before any routing decision,
and classified the request as the pending `direct:contributing-guide` item
rather than ordinary work. It committed only `CONTRIBUTING.md` as `ddf3d8e`
under the recipe's Direct checkpoint policy, completed the item through
`direct preflight` and `direct complete --implementation-revision` at that clean
revision, and reached 1/1 Direct items. No Spec directory, brief, requirements,
design, or contract was created; the established `cart` Spec stayed
byte-unchanged. The fixture ends with the CLI's own completion edit to
`.specbind/steering/roadmap.md` uncommitted, which the adapter's
implementation-paths-only scope leaves outside the checkpoint. This reverses the
`26518ee` Claude Code failure.

RT1 returned `REJECTED` with three blocking findings — the cap admitting 100
against the stated maximum of 99, a `"too many"` rejection that never states the
largest accepted quantity, and `setdefault` mutating the cart on a rejected
addition against the Design's unchanged-on-violation guarantee. It read the cart
Contract and both Steering documents alongside the mapped Requirements, Design,
and Task, and probed with `python3 -B` so no bytecode was written. `src/cart.py`
stayed byte-identical to what the recipe wrote, `git status --short` stayed at
` M src/cart.py`, and Task 1 stayed pending.

DB1 did not start implementation. It read Requirements, Contract, `design/main`,
both Steering documents, the Task, and `src/cart.py`, returned `CATEGORY:
ARTIFACT` for the Design's silent-trim instruction contradicting Requirement 1.4
and invariant `max-per-sku`, and preserved the exact `## Diagnosis` block with
all four fields. The fixture was byte-identical and Task 1 stayed pending. Under
this driver the Agent-tool harness appends its own `result:` summary line after
the block; that line is the driver's reporting convention, present in all four
runs of this batch and in the `needs input:` form under X1, and is not produced
by the product skill.

Two pending confirmations from the previous batch were settled here. X1 listed
`artifact list cart` and read the reported `design/main` selector with no
shortened-`design` retry. I3 used the accepted `inline` Direct default without
searching settings for a mode.

The 2026-08-29 Claude Code batch was measured against `26518ee`, driven as
Agent-tool subagents on `claude-opus-5` with no prior context, one fixture per
scenario. Passes: D9, D10, D12, R3, R4, C3, DS3, T2, X2, X3, X4, RT1, DB1, VI2,
VI3, I4, RL1, RL2, RL3. Every one was judged from the fixture with a
checksum comparison against its pre-run snapshot; the read-only scenarios
(DS3, RT1, DB1, VI2, VI3, X2, X3, X4, D9, D10, RL1, RL2) left their fixtures
byte-identical. X1 and I3 failed, and five more scenarios stopped without a
measurement because this driver profile cannot supply an approval or dispatch;
all of them are recorded below.

I4 also exercised the implementation path end to end: the unrelated
`src/orders.py` edit survived untouched, the completed task produced one commit
holding only `src/cart.py` and `tasks.yaml`, and no completion handshake ran.
Its dispatch log carried one line, so the run took Decision 0109's main-context
fallback rather than dispatching; that is a workflow pass, not a dispatch
measurement.

T2 settles the ordering the same way its contract allows: the run accepted the
contract review first, committed only `state/contract-review.md`, and then
authored a three-task plan, so a fresh review coexists with `tasks.yaml` — the
combination that is mechanical proof the review came first. It is also the
sharpest evidence for the X1 finding below. T2's driver accepted the same
unchanged-Contract-with-changed-guarantee shape that X1's driver refused, and
explicitly reasoned that the cap should not be declared as an invariant yet.
Two runs of one skill, on one build and one agent, reached opposite conclusions
about the same question.

CF2 was measured as Codex on 2026-08-28 with `gpt-5.6-terra` at medium
reasoning against the working tree based on `d9de45b`, including Decision 0154.
The driver changed only the project-owned Requirements template, adding the
requested `#### Risks` materialization guidance after Acceptance Criteria, and
created one adapter-directed local checkpoint. Mechanical judgment confirmed
`spec/requirements: project-content`, a clean worktree, a readable raw template,
and byte-identical established cart Requirements and `spec.yaml`. No existing
artifact or lifecycle state was reconciled after the request declined it.

DS1 and X1 were re-measured as Codex on 2026-08-29 with `gpt-5.6-terra` at
medium reasoning against `0270764`. DS1 read `contract/v1`, materialized the
fixed `contract.yaml`, declared the established cart dependency, reached 3/3
Design coverage, passed the two-Contract graph check, and checkpointed only the
Design, Contract, and CLI-owned `spec.yaml`; no task plan was created. X1 read
the same fixed Contract path at baseline and current revisions, accepted a fresh
review from only Roadmap scope and Contract inputs, and created no task plan.
Both fixtures were clean before and after their read-only debriefs. The DS1
driver corrected unquoted Requirement IDs before approval, and the X1 driver
regenerated a candidate after a PowerShell quoting mistake; neither workaround
changed the measured outcome or exposed a retained product finding.

DS5 was measured as Codex on 2026-08-29 with `gpt-5.6-terra` at medium
reasoning against `9c8d4b8`. Before editing, the driver used both focused graph
queries, read the `checkout` Contract named by the reverse edge, and stopped on
the unresolved consumer migration and whole-line-item shape. Both Contracts
remained byte-unchanged, the Design gate stayed `not_reached`, and the fixture
was clean before and after the read-only debrief. The first fixture attempt on
`a0f4ac9` never reached an Agent because its precondition still grepped for the
superseded Markdown target spelling; `9c8d4b8` replaced that harness check with
the typed `contract consumers` projection. The debrief observations were either
the scenario's intentional unresolved choice, a corrected command guess, or a
fixture-specific missing implementation consumer, so none was retained as a
product finding.

C2's dedicated-marker variant passed as Codex on `fb87bb9`. The fixture left the
Discovery milestone, Roadmap, cart state, and Brief uncommitted, made no commit
beyond fixture setup, and did not ask for checkpoint policy. Earlier passing
runs on `ec20755` and `3746108` exposed two usability findings before the final
measurement: marker precedence needed to state that the entire body is ignored,
and Discovery needed to repeat the completion check immediately before Brief
authoring. The final driver followed both rules and stopped in Requirements with
all gates `not_reached`.

Q0 was measured as Codex on 2026-08-28 with `gpt-5.6-terra` at medium reasoning.
The first run on `dd8793f` inferred named `cart` scope from the milestone's only
participant and edited Requirements. After quick-plan metadata and installed
project instructions routed ambiguous accelerated planning to scope selection,
the fresh `4bdba02` run read only milestone status, presented named/all intent,
and stopped. The fixture remained at setup commit `56a9956`, with every gate
`not_reached` and a clean worktree. Its read-only debrief left state unchanged
and restated the intentional single-participant ambiguity the explicit scope
rule had resolved, so no usability finding was retained.

B0 was measured with the same Codex profile on fresh build `c0af39f`. An
explicit every-Spec request selected all scope even though the fixture had only
one participant, presented `cart` plus the Requirements, Design, and Tasks gates
under the `specbind-quick-plan` workflow identity, and stopped for delegation
without authoring. The fixture remained at setup commit `df144a7`, every gate
was `not_reached`, and the worktree stayed clean before and after the read-only
debrief. Its observations restated the intentional delegation boundary and the
fixture-only PATH setup, so no usability finding was retained.

R1 was re-measured as Codex on 2026-08-28 against the working tree based on
`92a7705`, including Decision 0150. The driver replaced the deliberately invalid
empty Requirements scaffold with five real criteria, passed strict traceability
before approval, approved active IDs `2.1`–`2.3`, and checkpointed only
`requirements.md` and `spec.yaml`. The resulting Spec was healthy in Design with
the expected missing-Design coverage diagnostics and no Contract. The clean
post-judgment debrief exposed one unresolved wrong-action risk: the Brief names
only cancellation, while the complete-current-contract instruction led the
driver to inspect `src/orders.py` and infer that order placement also belongs in
the new Spec. That source-authority boundary should be investigated separately;
it did not alter this scenario's fail-closed scaffold measurement.

R6 was measured as Codex with `gpt-5.6-terra` at medium reasoning. On `a0f901d`
the driver resolved both Unicode-variable references correctly but lost the
durable `maintain` comment and initially proposed noncanonical active IDs, so no
pass was recorded. After the Requirements skill paired binding substitution
with byte-exact durable-comment copying and explicit positional ID guidance, a
fresh run on `238b210` passed: `fixture-day` appeared exactly twice, no variable
or `create` instruction remained, the complete `maintain` comment survived, and
the project-owned template stayed byte-identical. Requirements approval used
the four authored canonical IDs and left the Spec healthy in Design with only
the expected missing-Design coverage diagnostics. The read-only debrief left
the fixture state unchanged; its two observations restated an intentionally
unspecified product-policy boundary and an already-conditional Contract read,
so neither was retained as an actionable finding.

DS7 and DS8 were measured as Codex on 2026-08-28 with `gpt-5.6-terra` at
medium reasoning for the rule-selected Design template set. The first `effbd28`
runs stopped before authoring: DS7 inferred the active Roadmap as a Steering
selector, while DS8 lacked the parser grammar and error contract needed to
choose a design. After the Skill limited Steering reads to exact listed
selectors and the DS8 fixture supplied those semantics, both passed on
`b5f17ae`: DS7 selected `design/main` and `design/ui`; DS8 selected only
`design/main`; both reached 1/1 traceability with a complete Contract and fresh
Design gate.

The DS7 pass exposed a wrong-action risk after judgment: `template resolve`
reported only a SpecBind-root-relative target, and the driver first wrote under
the project-root `specs/` directory before repairing the placement. Builds
`58f7c39` and `774a426` added one project-root-relative `Project path` and then
removed the competing `Target path`. One intervening run was discarded because
the driver approval prompt said to stop "immediately" and truncated the
Design checkpoint. A fresh `774a426` run then correctly stopped because DS7
still omitted the UI host, data-owner, and verification foundation; the fixture
was repaired rather than weakening Design's fail-closed discovery rule. The
final DS7 run on `7ff63a9` selected both Design artifacts, wrote only their
reported `.specbind/specs/dashboard/` project paths, passed traceability and the
Contract graph, recorded a fresh explicit Design gate, checkpointed commit
`b880344`, and ended clean at `Next action: contract_review` without Tasks.

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

HP1 passed again on 2026-08-25 with a fresh `gpt-5.6-terra` medium Codex driver
against release-candidate build `0f4aee6`. The journey completed Discovery,
delegated planning, independent Design validation, contract review, repeatably
clean implementation verification, completion, annotated local tagging,
detached tagged-tree verification, and finalization. The mechanical judge passed
every expectation at final commit
`97d3cf9b8173bc160a509bc3c2996ce99c462c36`; annotated tag `v1.4.0` points to
`29d24bbd50dd09a4ee0fbcfca0de4ccd42a9b658`, before finalization, and the
fixture recorded eleven fresh dispatch contexts. The canonical seven-test run
passed during judgment, the final worktree was clean, and the fixture retained
no remote.

HP1 passed on 2026-08-25 against release-candidate build `a81826b` with the
same fresh Codex driver profile. The mechanical judge passed every expectation
at final commit `17379d70a35afd699e8565af2e99f60039294a05`; annotated tag
`v1.4.0` points to `5db18a12a571afa29412f3b4c72f1f3ad8675570`, before
finalization, and the fixture recorded twelve dispatch contexts. The five-test
canonical command passed during judgment, the final worktree was clean, and no
remote existed. Completion first returned `NO-GO` after an additional Python
liveness probe generated `src/__pycache__/`; the same driver removed the output,
re-ran the probe with `python -B`, and accepted completion only after the clean
evidence passed. The English fixture again received Japanese driver responses,
and one nested validator missed the fixture CLI on `PATH`; fixture state and the
rc.2 binary's actual command surface were therefore re-read mechanically.

I6 passed on 2026-08-25 with a fresh `gpt-5.6-terra` medium Codex driver
against `243c0f9`. Starting from fixture commit `e805c6b`, the driver completed
both sequential Tasks in exactly two local implementation commits. Commit
`073bca5` contained Task 1's implementation, tests, canonical test command, and
only Task 1's progress transition; `9f722e2` contained Task 2's implementation,
tests, and only Task 2's transition. Mechanical judgment found `cart` still in
implementation with 2/2 Tasks completed, no completion evidence, a clean
worktree, and four passing tests.

I6 passed again on 2026-08-26 against `46dd074`. Starting from fixture commit
`67d4f69`, the fresh driver produced exactly two Task checkpoints at `45b1222`
and `05124f0`, with only the corresponding progress transition in each. The
fixture remained in implementation with 2/2 Tasks completed, no completion
evidence, a clean worktree, and four passing tests. Its post-judgment debrief did
not repeat the earlier `specbind-status` to `specbind status` command
translation.

A1, I1, I3, and RL4 were re-measured on 2026-08-29 with fresh
`gpt-5.6-terra` medium drivers after their conditional procedures moved into
directly linked package resources. A1 passed on `ff304ab` by stopping at
`ADOPTION_STEERING_REQUIRED` without creating a dossier, Roadmap, or Spec. I1
and I3 passed on `09cfc19`: I1 used fresh implementer and reviewer contexts,
completed only its one Task, and stopped before completion; I3 checkpointed
only `CONTRIBUTING.md`, completed its Direct item, and manufactured no Spec
artifacts. RL4 first failed on `ff304ab` because the entrypoint's stop wording
let the driver omit the complete bootstrap proposal. After the boundary was
clarified, a fresh `09cfc19` run proposed the repository-derived local-tag
adapter, obtained configuration-only approval, committed only that adapter,
and stopped without binding, tagging, publishing, pushing, or finalizing.

X1, I3, RT1, and DB1 were re-measured on 2026-08-29 as fresh Codex
`gpt-5.6-terra` medium drivers after the four open findings from `26518ee` were
reproduced against their owning Decisions and assets. X1 passed on `9dc9505`:
the unchanged Contract's missing 99-per-SKU invariant blocked acceptance,
`milestone review status` remained `absent`, no Tasks existed, and the fixture
stayed clean. I3 passed on `c6a5f40`: the driver read the active milestone,
selected the Direct implementation path, checkpointed only `CONTRIBUTING.md`,
and completed the existing item without manufacturing Spec artifacts. RT1
passed on `69b709f`: it read the Contract and both Steering documents, returned
an exact `REJECTED` block with three blocking findings, left the seeded diff
byte-unchanged, and kept Task 1 pending.

DB1 exposed one additional layer before its final pass. On `69b709f` the driver
routed the diagnosis-shaped request through implementation and paraphrased the
nested debugger, so the required result block was still absent. `6be1931`
routed directly to debug and preserved the exact `## Diagnosis` block, but the
diagnosis added a false missing-Contract claim after failing to read that
existing input. The fresh `e18d8fc` run read Requirements, Design, Contract,
both Steering documents, and the Task, returned only the supported `ARTIFACT`
contradiction in the exact block, and left the clean fixture and pending Task
unchanged. Debrief claims that `--for consume` was unsupported were discarded:
direct execution of the fixture binary accepted the documented syntax, proving
the drivers had resolved a stale host CLI for those commands.

### Runs without a passing measurement

| Scenario | Agent | Result | Why no pass was recorded |
| --- | --- | --- | --- |
| X1 | Claude Code | Product failure | On `26518ee`, the driver refused acceptance of an unchanged Contract because the milestone's new 99-per-SKU guarantee was not declared as an invariant, and stopped with `milestone review status` still `absent`. Codex accepts the same fixture, so the review contract admits two readings: it tells a reviewer how to judge a *changed* entry but not a behavior change that arrives with no Contract entry at all. Superseded by the passing `1736d0c` Claude Code run. |
| I3 | Claude Code | Product failure | On `26518ee`, the driver wrote `CONTRIBUTING.md` as ordinary work without ever reading `milestone status`, so the milestone's pending `direct:contributing-guide` item stayed `0/1 completed` and the file was left uncommitted. No Spec artifacts were manufactured. Routing settled on "not a Spec, not Steering" without the active milestone ever entering the decision. Superseded by the passing `1736d0c` Claude Code run. |
| DS1, T1, T4, DS4 | Claude Code | Environment blocked | On `26518ee`, each run reached its confirmation boundary correctly — DS1 authored the `order` design and Contract (`check traceability` 3/3, `check contracts` clean), T1 authored a three-task plan passing 4/4 coverage, T4 and DS4 stated their rewind costs, DS4 naming the contract-review deletion — and then refused the approval relayed by the driving session, on the correct ground that another agent's message is not the user's consent. The post-approval half of each scenario is unmeasurable from an Agent-tool subagent. |
| A2 | Claude Code | Environment blocked | On `26518ee`, preflight returned the fixture HEAD, all four Steering documents were read, and the boundary proposal named `cart`, `order`, their dependency, the unmanaged area, and three uncertainties, with no dossier, milestone, Spec, or Brief written and a clean worktree. The agent log held one line: the subagent driver has no dispatch tool, so the required fresh readers never ran. |
| D3 | Claude Code | Not measured | The confirmation authorized the whole feature, so later phases rewrote the discovery artifacts before they could be judged. |
| D7 | Claude Code | Not measured | No embedded `specbind-tasks` skill owned plan authoring at the time; the run correctly stopped. |
| D7 | Codex | Environment blocked | The agent stated the correct rewind cost, but the host safety review rejected the confirmed invalidation twice. |
| R1 | Codex | Scenario blocked | The fixture says only that customers can cancel "eligible orders", but never defines eligibility. The Requirements review protocol requires an unknown product expectation to be escalated rather than guessed, so the agent correctly stopped without authoring. |
| Q0 | Codex | Product failure | On `dd8793f`, the fresh `gpt-5.6-terra` medium driver inferred named `cart` scope from the milestone's only participant and routed directly to Requirements. It added criterion 1.4 to `requirements.md` while leaving every gate `not_reached`, instead of presenting named/all scope choices without authoring. The quick-plan metadata and installed project instructions now route ambiguous accelerated-planning requests to the orchestrator before an actionable phase. |
| R1 | Codex | Environment blocked | After the fixture ambiguity was repaired in `55518ce`, the driver approval mechanism rejected the fixture-required instrumentation write twice, including after the parent explicitly authorized that write. No product workflow ran. |
| R6 | Codex | Product failure | On `a0f901d`, the driver correctly resolved the Unicode `作成日` binding once to `fixture-day` and replaced both references without changing the template, but first proposed noncanonical `R2.AC1`-style active IDs and omitted the template's complete `maintain` comment from the live Requirements. After correction it approved `2.1`-`2.3`; the fixture reached Design with the expected coverage diagnostics, but the durable-comment loss prevented a pass. |
| C1 | Codex | Product failure | On `9cce3de`, the agent read the quantity limit as ordinary work, bypassed Discovery, and edited `src/cart.py` plus tests. The project instruction admitted that reading; `59ebc5f` clarified the boundary and the fresh C1 run passed. |
| S5 | Codex | Product failure | On `3c1b91b` and `81cc473`, durable testing guidance bypassed Steering and became root `TESTING.md`; `d10e05e` routed correctly but partially copied `maintain` and expanded into test implementation. The fresh `c6d21fd` run passed. |
| R1 | Codex | Operator stopped | On `9cce3de`, the run produced a valid Requirements draft, but the controller began the usability debrief instead of continuing the required explicit-approval turn. The draft is evidence of neither a pass nor a product failure. |
| S2 | Codex | Environment blocked | On `4738ca2`, bootstrap reached its required three-reader dispatch, but stale host agent threads exhausted the global limit. Steering remained empty and unchanged; no product authoring ran. |
| T1 | Codex | Environment blocked | On `cc37049`, the corrected rule produced a one-task implementation-and-test proposal, but the host safety layer rejected `tasks.yaml` authoring twice, including after explicit Tasks approval. No artifact was written, so this is not a passing remeasurement. |
| D6 | Codex | Product failure | On `4256ab3`, the first Discovery correctly left its new Roadmap uncommitted under an unfilled Git adapter. The confirmed same-session addition then failed with `MILESTONE_ROADMAP_DIRTY`, leaving the original milestone and `cart`-only scope unchanged and creating no `order` Spec. |
| VD1 | Codex | Product failure | On `7307f7a`, the validator returned the expected `NOT_READY` for Research dependence but also raised inactive Requirements 2.1–2.2 as blocking Design omissions. The fresh `3d887b6` run scoped judgment to the active 4/4 set and passed. |
| RL3 | Codex | Product failure | On `6a29ad7`, a fresh driver misclassified the explicit Front Matter-only Release adapter as an unconfigured scaffold. The classification order was made explicit and the fresh `f069aef` run finalized correctly. |
| RL4 | Codex | Product failure | On `f069aef`, a fresh driver concluded that release documentation was absent without inspecting root `RELEASING.md`. `447c0c6` requires root release-document enumeration before that conclusion; the fresh run found it and passed. |
| RL4 | Codex | Product failure | On `ff304ab`, the fresh driver opened the new bootstrap resource but interpreted the entrypoint's stop boundary as permission to omit the full adapter proposal. `09cfc19` makes proposal presentation mandatory before the approval stop; the fresh retry proposed and checkpointed only the approved adapter, then stopped. |
| I1 | Codex | Environment blocked | On `ff304ab`, the driver did not exercise the installed Skill or its fresh implementation/review dispatch and left only a partial direct edit. A fresh `09cfc19` run exercised both roles and passed. |
| I3 | Codex | Environment blocked | On `ff304ab`, the host rejected fixture CLI execution and did not expose the installed Skill, leaving the Direct item pending. A fresh `09cfc19` run exercised the packaged Direct procedure and passed. |
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
| HP1 | Codex | Product failure | On release candidate build `3475e79`, Discovery and Requirements checkpointed cleanly, but independent Design validation rejected a draft whose Contract promised integer quantities while its Design specified only numeric bounds. The quick-plan driver treated that correctable `NO-GO` as the end of the workflow instead of returning the findings to the owning Design skill for a bounded revision. The fixture stopped in Design with only unapproved `contract.md` and `design.md` drafts dirty, no later gate, implementation, tag, or finalization. Quick-plan and batch-plan were clarified to route one Design-owned revision through a fresh independent validation while approval remains blocked. |
| HP1 | Codex | Product failure | On release candidate build `266c4e7`, Discovery checkpointed cleanly, but the Requirements phase rewrote the existing Spec around quantity limits and silently removed the unrelated cart-read requirement before delegated approval. Quick-plan detected the loss only after approval, so the required rewind was outside delegated authority and the fixture stopped with a fresh Requirements gate plus uncommitted `requirements.md` and `spec.yaml`; no Design, Tasks, implementation, tag, or finalization ran. The Requirements skill was tightened to audit every pre-existing ID and the surrounding ownership text against the entry document and Git diff before approval. |
| HP1 | Codex | Product failure | On release candidate build `8aac7de`, Discovery and the preservation-audited Requirements phase checkpointed cleanly, but quick-plan applied its generic clean-handoff rule to the intentionally unapproved Design and Contract drafts that independent validation needed to read. It correctly refused to create the Design-owned checkpoint itself and stopped before validation with only those two draft paths dirty. The planning orchestrators were clarified to allow exactly that bounded draft handoff, then require the Design approval dispatch to checkpoint and restore a clean tree before contract review. |
| HP1 | Codex | Product failure | On release candidate build `3764d52`, Discovery and independent Design validation passed, but contract review stopped to ask whether the changed `add-item` export had unmanaged callers. The delivery request had already explicitly chosen that exported behavior, the fixture identified no external consumer, and the same request established why the existing unconsumed seam remained. Contract review now records that requested disposition and possible unmanaged impact, and asks only when evidence exposes an additional compatibility choice. |
| HP1 | Codex | Product failure | On release candidate build `08e0ac6`, Discovery and delegated planning completed cleanly, and implementation review correctly rejected an addition path that still accepted zero or negative requested quantities for an existing SKU. The corrective implementer then treated the fixture-required project-local instrumentation append as needing separate user approval, despite the fixture instruction requiring it as the first action. Diagnosis confirmed an authority-interpretation defect and the run stopped with the task pending and only the intended source/test draft paths dirty. The implementation protocol and dispatch brief now classify required non-destructive project-local bookkeeping as ordinary task execution while preserving destructive and external-action boundaries. |
| HP1 | Codex | Product failure | On release candidate build `8a7b6b1`, an English-only fresh run completed Discovery, delegated planning, implementation, completion, local annotated tagging, detached verification, and finalization. Mechanical judgment then reran the canonical `scripts/test.sh`, which exited zero but recreated untracked `src/__pycache__/` and `tests/__pycache__/`, so the final-worktree-clean expectation failed. Implementation had cleaned the same outputs after its own runs, which hid that the public command itself was not repeatably clean. Task implementation and completion validation now require before/after status evidence around an exact canonical invocation with no intervening cleanup. |

Scenarios not named in either table have not produced a recorded result for
either agent. The tables are a measurement ledger, not a coverage checklist.

### Open usability findings

Reproduced against the `1736d0c` fixtures and their installed assets, not from
the drivers' reports alone. None was fixed in the run that found it.

| Scenario | Surface | Finding | Reproduction | Impact |
| --- | --- | --- | --- | --- |

None. Decision 0159 resolved the actionable `1736d0c` findings; the two
`dc6c022` follow-ups are awaiting confirmation on their fixing build below.

### Environment limitations affecting interpretation

| Limitation | Effect on this batch |
| --- | --- |
| A Claude Code Agent-tool subagent does not see the fixture's installed skills in its Skill registry. All four drivers hit `Unknown skill: specbind-*` and fell back to reading `SKILL.md` from disk, which is faithful to the document but is not the platform selection path the project instructions name. Those instructions forbid translating a Skill name into a CLI command and name no fallback for this state. | The four passes measure the skill bodies, not skill selection or dispatch. The gap is only reachable when a platform fails to register installed skills, so it is not recorded as a product finding. |
| The same driver appends its own status line (`result:` or `needs input:`) after the agent's report. | DB1's exact `## Diagnosis` block is terminal in the diagnosis but is followed by that harness line. Present in all four runs regardless of skill, so it is driver reporting, not skill output. |

### Fixed, behavioral confirmation pending

| First seen | Scenario | Finding | Resolution | Status |
| --- | --- | --- | --- | --- |
| `dc6c022` | X1 | A lifecycle action label could be guessed as a Design artifact ID before selector discovery. | Decision 0160 requires `artifact list` first, exact reported selectors only, and explicitly excludes lifecycle states and actions from the artifact namespace. | Awaiting a fresh X1 run on the fixing build. |
| `dc6c022` | I3 | A tracked Direct item that also looked like durable guidance could be routed to Steering before active milestone lookup. | Decision 0160 makes active milestone matching precede change-request surface classification and gives a matching tracked item precedence. | Awaiting a fresh I3 run on the fixing build. |
| `1736d0c` | RT1 | Review's read-only boundary and deferred adapter write had no stated ordering. | Decision 0159 fixes the verdict first under a byte-identical worktree, then permits only the adapter-directed deferred record as a separate post-verdict mutation. | RT1 confirmed the read-only verdict path on `dc6c022`; a deferred-candidate scenario is still needed for the post-verdict write branch. |
| `1736d0c` | CLI recovery | Unknown nested commands could suggest an unrelated top-level command. | Decision 0159 disables token-only similarity suggestions while retaining help and usage. | Focused parser tests pass; behavioral recovery confirmation remains pending. |
| `4738ca2` | T1 | The default task rule told projects to choose a test-grouping convention but did not choose one, so the planner had to decide whether one behavior needed a separate test task. | `cc37049` defaults tests into the behavior task and permits a separate verification task only across several earlier tasks or a separately reviewable system boundary. | A fresh driver proposed the expected combined task, but host safety blocked artifact authoring; rerun T1 when that environment stop is absent. |

### Resolved usability findings

Resolved rows retain only the behavior that changed and the build carrying the
fix. Detailed observations, discarded non-defects, and fixture-only workarounds
remain available in Git history.

| Finding | Resolution | Fixed in |
| --- | --- | --- |
| A missing Design seam could be rewound without an explicit maintainer confirmation when milestone scope itself was unchanged. | Contract review presents the full downstream loss and requires explicit user confirmation before every gate invalidation. X1 stopped with Design fresh and review absent. | `dc6c022`, confirmed on `dc6c022` |
| Milestone actions exposed typed item keys but not the exact operand accepted by item mutation commands. | Spec and Direct actions expose `command_operand`; I3 used the Direct operand to complete the pending item. | `dc6c022`, confirmed on `dc6c022` |
| Machine-consistent state was labeled unqualified `Health`, allowing it to be read as semantic prose agreement. | Status labels it `State health` and separately reports `Semantic alignment: not evaluated`; DB1 still diagnosed the artifact contradiction from governing inputs. | `dc6c022`, confirmed on `dc6c022` |
| Bare review and diagnosis requests had no fail-closed subject-selection procedure. | Both skills enumerate milestone/task projections, proceed only for exactly one valid candidate, and otherwise ask. RT1 and DB1 selected the sole pending Task. | `dc6c022`, confirmed on `dc6c022` |
| Direct completion could leave its CLI-owned Roadmap edit dirty without connecting that state to adapter authority. | Direct completion uses a separate metadata checkpoint only when active policy covers it; otherwise it reports the dirty Roadmap, unavailable revision, and exact narrower-policy reason. | `dc6c022`, confirmed on `dc6c022` |
| The default Contract Rule looked like an unfilled authoring prompt instead of current compatibility policy. | The embedded Rule states a conservative live default that projects can replace through the existing Rule surface. | `dc6c022`, confirmed on `dc6c022` |
| Contract review could accept an unchanged Contract even when the scoped delivery added a persistent guarantee that Contract never declared. | Review compares Roadmap behavior to the current Contract, deep-reads only when needed, and blocks a missing seam or guarantee. X1 kept review absent for the missing 99-per-SKU invariant. | `9dc9505`, confirmed on `9dc9505` |
| A request matching a pending Direct item could be classified as ordinary work without reading the active milestone. | Project instructions require `milestone status` before ordinary-work classification and route a matching pending item to implementation. I3 completed the existing Direct item and created no Spec artifacts. | `ec93e3a`, confirmed on `c6a5f40` |
| Contract review deep reading did not discover split Design selectors, so a driver tried the invalid shortened `design` selector. | The review lists the Spec's artifacts and reads the reported `design/<artifact-id>` selector before declaring a deep input. X1 read `design/main` with no retry. | `c6a5f40`, confirmed on `1736d0c` |
| The Direct procedure referred to the run's "selected mode" without stating where the selection came from. | An explicit override resolves first; otherwise both entrypoint and procedure state the accepted `inline` Direct default. I3 used `inline` without searching settings. | `69b709f`, confirmed on `1736d0c` |
| Direct task review omitted the Spec Contract and Steering from its required inputs and had no project-instruction route. | The route is explicit; review reads Contract and every Steering document in addition to mapped artifacts, failing closed on an incomplete set. RT1 read both Steering documents and rejected the unchanged seeded diff. | `e8cb515`, confirmed on `69b709f` |
| Debug could return only prose without the mandated result block, and a diagnosis-shaped request could be absorbed by implementation and paraphrase a nested result. | Debug's exact block is a pre-command validity rule, diagnosis requests route directly to it, and the complete governing input set prevents absence guesses. DB1 returned the exact `ARTIFACT` block with only the supported Requirements/Design contradiction. | `65fc207`, `6be1931`, `e18d8fc`, confirmed on `e18d8fc` |
| Completed implementation Tasks could be accumulated and combined into one checkpoint even though the default Git adapter names each Task as one workflow unit. | Task execution is sequential, and every completed Task resolves its own adapter-directed checkpoint before the next Task is selected. I6 confirmed two requested Tasks become two commits with one progress transition each. | `243c0f9`, confirmed on `243c0f9` |
| `scope/v1` exposed only `minimum: 0` for `schemaVersion`, so an author had to infer the version from the selector. | The generated schema now fixes `schemaVersion` with `const: 1`, matching runtime acceptance and the other v1 schemas; C2 authored the candidate from the corrected schema. | `6d1d2e5`, confirmed on `7307f7a` |
| Design validation could treat Requirements retained outside the active milestone set as missing Design scope. | Validation fixes the review scope from status and traceability before reading prose and treats inactive Requirements as context only. | `3d887b6`, confirmed on `3d887b6` |
| Contract review described the Direct-only stop as `not required`, while the public CLI prints `Status: not_applicable`. | The skill now names the exact public status and explains that it means no review is required. | `3d887b6`, confirmed on `3d887b6` |
| Adapter state overloaded the template-only `specbind:instruction` token, used a raw substring check, and required a deferred-specific compatibility exception. | Inactive adapters use the exact Markdown comment `<!-- specbind:adapter-scaffold -->`; marker-like prose, code, longer comments, and the template token are ordinary adapter content. C2 confirmed marked Git policy opts out without asking or committing. | `ec20755`, confirmed on `fb87bb9` |
| An untouched Release scaffold could be treated as an explicit no-op and let a first invocation finalize without configuring project policy. | Release now derives a complete adapter from repository evidence, obtains configuration-only approval, checkpoints only that adapter, invalidates affected completion evidence, and stops. | `a576cf6`, confirmed on `447c0c6` |
| Successful `release finalize` left its lifecycle archive and log mutations outside a checkpoint. | Release snapshots the worktree before finalization, then uses active Git policy to create a separate local commit containing only newly changed finalization lifecycle paths; publication approval does not authorize pushing it. | `a576cf6`, confirmed on `d51a12a` |
| Empty adapter content and an installed scaffold could both be reported as `state=scaffold`, contradicting the dedicated-marker contract and the Release empty-body meaning. | Skill classification checks the exact read result first, and `adapter list` now reports scaffold only for the exact dedicated marker; a Front Matter-only adapter is active. | `f069aef`, `d51a12a`, confirmed on `d51a12a` |
| Release bootstrap could declare policy evidence absent after reading only `README.md`, even when root `RELEASING.md` existed. | Bootstrap enumerates root release-document candidates such as `RELEASE*`, `RELEASING*`, and `CHANGELOG*` before concluding that evidence is absent. | `447c0c6`, confirmed on `447c0c6` |
| A progressive Release entrypoint could tell the driver to stop at a referenced bootstrap boundary before the driver presented the complete configuration proposal. | The entrypoint now requires repository investigation and the full replacement proposal before stopping for explicit approval; absence of approval cannot suppress the proposal. | `09cfc19`, confirmed on `09cfc19` |
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

A Claude Code Agent-tool subagent is a valid driver only for scenarios that do
not cross an approval. It refuses an approval relayed by the driving session —
correctly, since another agent's message is not the user's consent — so every
authoring phase stops with its draft unapproved. It also has no dispatch tool
and does not see the fixture's installed skills in its Skill registry, so it
reads `SKILL.md` from disk and takes the main-context fallback for dispatch.
Judge those runs as environment-blocked past the confirmation turn rather than
as product failures, and use a real session in the fixture directory to measure
the second half.

Host-instruction inheritance shows up here the same way it does under Codex:
in the first `26518ee` batch, two of six drivers answered in Japanese against an
`en` fixture despite the standalone-fixture statement. Naming the inherited
rules explicitly — that no other repository's language or commit policy
applies — stopped it, and the DB1 rerun under that wording answered in English
and reached the same diagnosis.

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

D13 passed on `acf4dd3` as Codex with `gpt-5.6-terra` at medium reasoning. A
committed project-owned Roadmap template replaced the default body with a
distinctive `Delivery promise`: Discovery filled that section with the
milestone-wide cart request, removed the `create` instruction, retained the
`maintain` instruction, and let the CLI generate the complete live Front Matter.
The settings template remained byte-identical, the one-item `cart` scope was
consistent in Requirements, and the Git adapter left a clean local checkpoint.
The driver reported in Japanese against the English fixture, so host-instruction
contamination remained visible; fixture state was re-read mechanically and
satisfied every D13 expectation. The read-only usability debrief reported no
friction and left the worktree unchanged.

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
