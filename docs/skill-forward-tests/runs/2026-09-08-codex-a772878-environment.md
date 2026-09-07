# Forward-test run: 2026-09-08 / Codex / a772878 environment attempts

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `a772878`
- Fixture language: `en`
- Scenarios: `T1, Q4, DR5`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `T1` temporary-path attempt | `environment_invalid` | The driver could not perform the required artifact mutation | A Tasks draft was prepared but not approved | The host denied the state-changing command under the native Temp path | `none` |
| `Q4` setup attempt | `scenario_invalid` | The fixture used recipe `r1` instead of Q4's required `r4` starting state | Wrong starting state; no Q4 judgment | Recipe identity was corrected before launching the measured retry | `none` |
| `DR5` temporary-path attempt | `environment_invalid` | The driver could not write or execute the required fixture operations | No product mutation | The driver correctly identified the Requirements boundary before the host denied fixture execution | `none` |

## Confirmation turns

No confirmation from these invalid attempts was reused in a measured fixture.

## Debrief dispositions

No usability debrief was retained because none of these attempts measured the
intended product branch.

## Cleanup

- Fixture paths removed: `sb-ft-t1-a772878`, `sb-ft-q4-a772878`, `sb-ft-q4b-a772878`, and `sb-ft-dr5-a772878` under the native Temp directory
- Main worktree after recording: only this forward-test documentation was pending
