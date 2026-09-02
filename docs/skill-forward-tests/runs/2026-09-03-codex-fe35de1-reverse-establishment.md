# Forward-test run: 2026-09-03 / Codex / fe35de1

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `fe35de1`
- Fixture language: `en`
- Scenarios: `A3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A3` | `product_failure` | Reverse finalization classified the required post-creation `.specbind/deferred.md` checkpoint as implementation source drift. | Clean active reverse milestone at `adoption_ready`; both Specs and the Contract Review are fresh. No Tasks, source, test, dependency, configuration, or Steering changes exist. | `spec list` reported exactly `cart` and `order`; `check contracts` verified 2 Contracts and 1 dependency with no warning; the adoption record is correctly at `.specbind/adoption/reverse-discovery.yaml`; `git diff 1ed4662 -- src README.md` was empty; finalization returned `ADOPTION_SOURCE_REVISION_STALE` for `.specbind/deferred.md`. | `FT-0029`; confirms `FT-0028` |

## Confirmation turns

The driver presented one complete two-Spec proposal and received one exact
confirmation. It then progressed continuously through both Requirements,
dependency-ordered Design waves, independent validation, and the milestone
Contract Review. No second scope or phase confirmation was requested.

## Debrief dispositions

The fixture was clean before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A3` | The required deferred-finding write is rejected by the final source-drift guard. | `wrong-action-risk` | `fix` | `FT-0029` |
| `A3` | The `order` validator found an ambiguous Python import spelling and the Design was remediated before approval. | `extra-step` | `discarded` | Correct semantic validation behavior; no product-surface ambiguity remained. |

## Cleanup

- Fixture paths removed: `C:\Users\hurui\AppData\Local\Temp\sb-a3-fe35de1`
- Main worktree after recording: FT-0029 fix and this run record only
