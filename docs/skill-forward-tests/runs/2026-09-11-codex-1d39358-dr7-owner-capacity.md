# Forward-test run: 2026-09-11 / Codex / 1d39358

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-11`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `1d39358`
- Fixture language: `en`
- Scenarios: `DR7`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `DR7` | `product_failure` | Drive stopped when the owning Skill could not be started, but returned narrative instead of the required complete pre-owner restart capsule | Implementation remained actionable with 0/2 Tasks complete; Requirements, Design, Tasks, and Contract Review stayed fresh; clean fixture | Capacity rejected the owner dispatch; no lifecycle or source path changed, but the result omitted the seven required labeled fields | `FT-0055` |

## Confirmation turns

None. The request authorized Drive without replan or release authority.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `DR7` | The instrumentation-first rule and the fixture language rule appeared to compete | `ambiguity` | `discarded` | fixture-only protocol detail; it did not affect the measured owner-capacity branch |
| `DR7` | Capacity rejected the owning Skill dispatch | `extra-step` | `discarded` | intentional scenario condition; the missing capsule fields were retained separately as `FT-0055` |

## Cleanup

- Fixture path removed: `sb-dr7-1d39358`
- Main worktree after recording: forward-test records only
