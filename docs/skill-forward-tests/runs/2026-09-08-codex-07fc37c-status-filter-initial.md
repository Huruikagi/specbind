# Forward-test run: 2026-09-08 / Codex / 07fc37c + initial status-filter tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `07fc37c` + initial status-filter working tree
- Binary SHA256: `5C34A17F608D6FC088C65BD0E680BE156C0CCCDBA838B65BF873ED2EB527757C`
- Fixture language: `en`, then `ja`
- Scenarios: `ST1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST1` (`en`) | `environment_invalid` | The English fixture response was in Japanese, showing host-language contamination | Task state unchanged; only `.specbind/specs/cart/tasks.yaml` dirty | The driver read the installed Status procedure and reported the correct compact facts, but did not follow the fixture language | None |
| `ST1` (`ja`) | `pass` | None | Task state unchanged; only `.specbind/specs/cart/tasks.yaml` dirty | The response omitted healthy State-health, fresh Contract Review, and the semantic-alignment disclaimer while retaining `0/2`, Task 1's blocker, and no actionable work | `FT-0051` surfaced in the post-judgment debrief |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `ST1` (`en`) | The driver also read the non-selected Spec reference | `extra-step` | `discarded` | contaminated attempt; routing already forbids the read |
| `ST1` (`en`, `ja`) | The driver strengthened the recorded file dependency into a prescribed Task-order or plan repair | `wrong-action-risk` | `retained` | `FT-0051`; repeated across both drivers and exceeded Status's recorded-fact boundary |

## Cleanup

- Fixture paths removed: `tools/specbind/target/forward-tests/st1-filter-07fc37c` and `st1-filter-ja-07fc37c`
- Main worktree after recording: only the Status filtering implementation and forward-test records were pending
