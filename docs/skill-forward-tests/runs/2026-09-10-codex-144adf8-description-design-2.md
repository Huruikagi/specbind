# Forward-test run: 2026-09-10 / Codex / 144adf8 + assessment repair

[Back to the measurement dashboard](../results.md).

- Driver: `Codex`, `gpt-5.6-terra`, `medium`, fresh context
- Tested build: `144adf8` + description tree and first decomposition-condition repair
- Binary SHA-256: `c0b127a5049f5aea162b142edae1299ac7a32f7cde1fdfaedf61b58806110882`
- Fixture language: `en`
- Scenario: `DS9`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DS9 | product_failure | One-off assessment/materialization still omitted after condition alignment | Untracked `design.md` and `contract.yaml`, no lifecycle mutation | `artifact list infrastructure` contains only `design/main`, with the exact main-template description. Settings and Spec-state diff empty. | FT-0053 |

No confirmation was given and no fixture was repaired. The read-only debrief
left the same two untracked files. The driver explicitly confirmed reading the
new conditions, recognized that the independent runtime/recovery concern met
them, and reported substituting sections inside main. The condition-only repair
therefore does not count as behavioral resolution.

Follow-up product changes require a concrete document inventory before writing,
including responsibilities, descriptions, traceability, and paths. The phase
introduction now says Design collection rather than "two artifacts"; the main
template's size-based split advice is aligned with responsibility-based splitting
in both languages. This is a new build, not a reinterpretation of this result.

## Cleanup

Fixture: `G:/specbind/target/forward-ds9-descriptions-final`. Removed after evidence capture.
