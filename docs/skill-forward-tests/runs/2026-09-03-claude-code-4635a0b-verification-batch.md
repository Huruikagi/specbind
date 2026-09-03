# Forward-test run: 2026-09-03 / Claude Code / 4635a0b

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Claude Code`
- Model: `claude-opus-5`
- Driver profile: Agent-tool subagent, fresh context, no inherited turns
- Tested build: `4635a0b`
- Fixture language: `en`
- Scenarios: `VD2`, `VC1`, `VC2`, `Q0`, `DR1`

`4635a0b` adds only forward-test records on top of `9492a83`; no product source
differs between the two builds. The commit is recorded because a run measures
the build it actually ran against, not because product behavior changed.

Scenarios continue to avoid a guarded approval, per ENV-0003. This batch targets
areas with no recorded Claude Code pass: Design validation, claim verification,
the Plan orchestrator, and Drive.

ENV-0001 recurred in every run. All six drivers read `SKILL.md` from disk after
`Skill(sb-*)` returned `Unknown skill`, so this batch measures Skill bodies and
not platform Skill selection or dispatch.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VD2` | `pass` | none | `HEAD` `5352292` unchanged; `design=fresh` retained; `.specbind/state/contract-review.md` still present at md5 `3e123b77…`; `design.md` md5 `48820eac…` and `requirements.md` md5 `37a40e06…` unchanged; only `.specbind/deferred.md` created | Verdict `NOT_READY` naming the design's silent-trim mechanism against Requirement 1.4 and the `max-per-sku` invariant; no gate command ran, so the validator did not discard the milestone's accepted contract review over its own opinion | none |
| `VC1` | `product_failure` | `spec status cart` must still report `State: implementation` with no completion evidence, and nothing may be written | `cart` advanced to `release_ready` with `completion=fresh`; new commit `e76d57e` recording `.specbind/specs/cart/spec.yaml`; milestone moved to `release_pending` | The run reached a correct `GO` on the evidence, then ran `spec completion accept cart --evidence -` and committed. Advancing a true claim is precisely the failure this scenario exists to catch. Root cause is routing, not the validation procedure: the run stated it took `sb-validate-implementation` because the installed instruction block sends "is this Spec done?" there | FT-0036 |
| `VC2` | `pass` | none | `HEAD` `aeb5e71` unchanged, worktree clean; `tasks list cart` still 2 completed / 1 pending | The run reported the work as not done and named pending task 3 ("Reject a quantity above the cap") as the gap; `spec completion preflight cart` exited 1 with `COMPLETION_MILESTONE_NOT_CONVERGED`; no task was run to make the claim true and no completion evidence was written | none |
| `Q0` (first attempt) | `environment_invalid` | The scenario never ran | Base fixture at `HEAD` `4052e37`, unchanged | The attempt used the bare fixture, which has no active milestone, so the run stopped on `NO_ACTIVE_MILESTONE` before the scope question exists. That measures the milestone guard, not Q0. The scenario document stated no starting state, which is the harness defect this attempt exposed; it is fixed in the same change | none (harness) |
| `Q0` (retry) | `pass` | none | `HEAD` `61933ee` unchanged, worktree clean; `.specbind/specs/order/` still holds only `brief.md` and `spec.yaml`; all gates `not_reached` | From `r1`: the run read `milestone status`, presented the named-Spec choice (`order`) and the all-Spec choice as two explicit options, and stopped. It stated that one participant does not permit inferring all scope, and asked for gate delegation as a separate question rather than treating scope selection as authorization. No phase dispatched, no artifact authored, no gate approved | none |
| `DR1` | `pass` | none | `HEAD` `8751135`; commits `547c0c3` (CONTRIBUTING guide) and `8751135` (Direct completion state); `git diff e64d3af..HEAD -- .specbind/specs/cart src/` empty; `milestone scope` byte-identical to baseline; worktree clean; no release bound | `sb-drive` was selected from the milestone-wide request and did not collapse to one `sb-implement` invocation. `cart-contract-change` was parked as `HUMAN_DECISION`/`REROUTABLE` because its summary requires canonical Requirements work, leaving every cart artifact and implementation path untouched. `contributing-guide` was implemented, reviewed, checkpointed, and recorded through `milestone direct preflight` then `direct complete`, giving `Direct progress: 1/2`. The reroute appears in the accumulated attention report at the end, not as a stop at the first item | none |

## Confirmation turns

None. All five scenarios stop before a guarded transition, so no approval was
relayed and ENV-0003 was not reached. `DR1` performs its own checkpoint commits
under the active Git adapter; that is workflow behavior, not a maintainer
approval.

## Debrief dispositions

Each fixture was read before and after the debrief and was unchanged in all five
judged scenarios. The failed `Q0` attempt was not debriefed, because a scenario
that never ran produces no observation about the Skill.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| all | `Skill(sb-*)` returned `Unknown skill` while the fixture Skill tree was on disk. | wrong-action-risk | discarded | ENV-0001, already an active environment limitation. |
| `VC1` | The installed instruction block routes "is this Spec done?" to `sb-validate-implementation` and tells the reader not to answer from "consequence-free claim checking" — the exact phrase in `sb-verify-completion`'s own description. | wrong-action-risk | retained | FT-0036; realized as an actual state mutation and commit in this run. |
| `VC2` | The same routing sentence carries a precondition ("when every Task is complete") that was false here, and the block gives no rule for that case; the driver chose `sb-validate-implementation` without reading `sb-verify-completion`. | ambiguity | retained | FT-0036; independent corroboration from a second scenario. |
| `VD2`, `VC1`, `Q0` | The instruction block says "Work through those installed `specbind-*` Skills" while its own examples and the on-disk directories use `sb-*`. | ambiguity | retained | FT-0037 |
| `VD2` | No command enumerates the active Requirement IDs, so review scope was inferred from the Design's own `requirement_ids` front matter — the artifact under review. | ambiguity | retained | FT-0038; same surface as the `Requirements: 6` unit ambiguity recorded at `9492a83`, now with the circularity made explicit. |
| `VD2` | A `NOT_READY` verdict on an already-approved gate has no destination: the Skill forbids invalidating, and the deferred adapter excludes verdict-changing findings, so the blocking findings survive only as prose. | ambiguity | retained in this record | The behavior is the accepted contract; whether the verdict should persist anywhere is a decision question, not a defect. |
| `VD2` | `check contracts` delivers `CONTRACT_GRAPH_EXPORT_UNCONSUMED` as a warning under an `OK` result with no stated disposition. | ambiguity | retained in this record | Second occurrence; first seen at `9492a83` under VD1. Reproduce against the `contract-principles` decision before a finding identity. |
| `VC1` | `adapter read` returns scaffold text with no status line saying it is a scaffold; only an HTML comment marks it. | ambiguity | retained in this record | Concrete. A validator could execute scaffold prose as project procedure. |
| `VC1` | Nothing in SpecBind state declares which command is canonical; `scripts/test.sh` says so only in a source comment, while substituting a command is a hard protocol failure. | wrong-action-risk | retained in this record | Reproduce on a fixture whose canonical command carries no comment. |
| `VC1` | The completion evidence model has no slot for the mandatory runtime-liveness and coverage checks, so the persisted record shows only the test suite. | ambiguity | retained in this record | Check the completion evidence decision before treating this as a gap. |
| `VC1`, `VD1` (prior batch) | No surface reports whether the `specbind-reviewer` / `specbind-researcher` role is registered, while the fallback rule makes guessing wrong a configuration failure. | wrong-action-risk | retained in this record | Second occurrence across batches. Reproduce on a Spec large enough to require dispatch. |
| `Q0` | Sections that dispatch `sb-validate-design` and `sb-contract-review` as Skills assume a receiver that can invoke them, which ENV-0001 makes impossible here. | wrong-action-risk | discarded | Latent under ENV-0001; not reached, and not a defect outside that environment limitation. |
| `Q0` | The Skill requires the stopping response to carry the scope question, which the driver framing ("report only what you changed and what you ran") pulls against. | extra-step | discarded | Driver-prompt artifact, not product friction. The run satisfied both. |
| `DR1` | `milestone status` presents the unimplementable Direct item as a normal actionable entry — `waitingFor: []`, `currentBlockers: []`, `diagnostics: []`, health `consistent` — while Drive is told status is the schedule. The only evidence of unsafety is English prose in `summary`. | wrong-action-risk | retained | FT-0039 |
| `DR1` | `sb-drive` presupposes a dispatchable subagent and says nothing about a runtime that cannot dispatch one. | ambiguity | retained in this record | Entangled with ENV-0001 here; reproduce where dispatch exists before treating it as a Skill defect. |
| `DR1` | `--json` is accepted by `milestone status` but rejected by `spec list`. | extra-step | retained in this record | Concrete and cheap to confirm; check whether the asymmetry is deliberate. |
| `DR1` | No surface enumerates the project's applicable checks for Direct work; the validation adapter is an inactive scaffold and governs Spec validation anyway. | ambiguity | retained in this record | Overlaps the `VC1` canonical-command observation; treat them together. |
| `VC2` | Tasks 1 and 2 are recorded complete while no commit touches `src/`, and no diagnostic flags recorded-complete-without-implementation. | wrong-action-risk | discarded | Fixture-only: the `t3` recipe fabricates the completed task state without implementing it. Not evidence of a defect encountered naturally. |
| `VC2` | The Skill's stop-at-preflight rule would have produced a report naming only the pending task. | wrong-action-risk | discarded | Follows from the same fixture-only state above. |
| `VD2`, `Q0` | `CLAUDE.md` and `AGENTS.md` are byte-identical and both were read in full. | cosmetic | discarded | The fixture installs for both agents by design. |
| `VC2` | Momentary ambiguity between the launching repository's Japanese rule and the standalone fixture. | cosmetic | discarded | The standalone statement resolved it; the `en` report confirms no contamination. |

## Cleanup

- Fixture paths removed: `/tmp/sb-db1`, `/tmp/sb-vi1`, `/tmp/sb-t3`, `/tmp/sb-dr1`, `/tmp/sb-q0`, `/tmp/sb-q0b`
- Main worktree after recording: only this run record, the Q0 scenario starting
  state, the dashboard projection, and the findings worklist were modified
