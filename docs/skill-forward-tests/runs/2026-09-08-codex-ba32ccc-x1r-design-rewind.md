# Forward-test run: 2026-09-08 / Codex / ba32ccc + Decision 0203 working tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `ba32ccc` + Decision 0203 working tree
- Fixture language: `en`
- Scenarios: `X1R`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `X1R-A` | `scenario_invalid` | The continuation said only “reapprove”, so the driver recorded an explicit Design approval instead of the intended `sb-plan` delegation. | Clean at `94e4e72`; stage `tasks`; Contract Review fresh; no Tasks; corrected Contract and Design fresh. | Design checkpoint `59c11bd` contained only `contract.yaml`, `design.md`, and the gate-updated `spec.yaml`; review checkpoint `94e4e72` followed it; Design traceability was 4/4; Contract verification passed; dispatch log had two contexts. `spec.yaml` recorded `approval_mode: explicit`. | none |
| `X1R-B` | `environment_blocked` | After the corrected continuation explicitly delegated Design approval to `sb-plan`, the host safety layer rejected the exact confirmed Design invalidation. | Clean and unchanged at setup commit `348435d`; stage `contract_review`; Contract Review absent; Requirements and Design fresh; Tasks and completion not reached. | The driver found the missing maximum-quantity invariant, named Design as owner, presented the exact rewind cost, and stopped. The host rejected `specbind spec design invalidate cart` before execution. | `ENV-0004` |
| `X1R-C` | `environment_blocked` | A fresh retry with the same corrected request and driver profile was rejected at the same confirmed invalidation boundary. | Clean and unchanged at setup commit `b69b1d1`; stage `contract_review`; Contract Review absent; Requirements and Design fresh; Tasks and completion not reached. | The fresh driver again found the omitted invariant, preserved Requirements and absent Tasks, and presented the exact CLI operation. The host required direct end-user approval despite the relayed exact confirmation. | `ENV-0004` |

The scenario continuation was corrected before `X1R-B` to name delegated Design
approval under workflow `sb-plan`. The two blocked attempts do not replace the
initial lifecycle evidence; they show that the remaining unmeasured assertion is
the delegated approval record, not the rewind-delta or checkpoint behavior.

## Confirmation turns

For every attempt, the first turn requested the exact rewind cost and a stop
before mutation. The continuation confirmed that presented Design rewind,
delegated corrected Design approval to `sb-plan` after a fresh independent
`READY`, and required a stop after renewed Contract Review without Tasks or
implementation.

## Debrief dispositions

The fixture was clean before and after each debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `X1R-A` | “reapprove” admitted an explicit-approval reading. | `wrong-action-risk` | `discarded` | Scenario wording was corrected to require `sb-plan` delegation before the fresh retries. |
| `X1R-A` | The unconsumed export warning required recording possible external compatibility impact. | `ambiguity` | `discarded` | The Contract Review protocol already defines the requested-change disposition; the driver applied it correctly. |
| `X1R-A` | The driver first guessed a milestone-local review-record path. | `extra-step` | `discarded` | Git status exposed the generated `.specbind/state/contract-review.md`; no product-contract defect was reproduced. |
| `X1R-C` | The driver questioned who dispatches the requested fresh validator. | `ambiguity` | `discarded` | The complete Plan route owns that dispatch; the host stopped this attempt before the route could be exercised, while `X1R-A` did dispatch a fresh validator. |
| `X1R-C` | The unconsumed export warning required a compatibility disposition. | `wrong-action-risk` | `discarded` | Existing protocol explicitly covers a request that directly changes the unconsumed export. |

## Cleanup

- Fixture paths removed: `/tmp/sb-x1r-0203`, `/tmp/sb-x1r-0203b`, `/tmp/sb-x1r-0203c`
- Main worktree after recording: only the Decision 0203 implementation and this run record were changed.
