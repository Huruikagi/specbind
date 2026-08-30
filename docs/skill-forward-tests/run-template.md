# Forward-test run: YYYY-MM-DD / DRIVER / BUILD

[Back to the measurement dashboard](./results.md).

- Date: `YYYY-MM-DD`
- Driver: `Codex | Claude Code`
- Model: `<model>`
- Driver profile: `<reasoning or relevant override>`
- Tested build: `<short commit>`
- Fixture language: `<language>`
- Scenarios: `<IDs>`

One file records one driver against one tested build. If the build changes, copy
this template to a new run file. Do not replace a failed attempt with its retry.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `<ID>` | `pass | product_failure | scenario_invalid | environment_invalid | environment_blocked` | `none` | `<status, artifacts, Git state>` | `<commands and observed results>` | `none | FT-NNNN` |

## Confirmation turns

Record only confirmation boundaries that affected interpretation, including the
phase approved and where the driver was told to stop.

## Debrief dispositions

The debrief occurs only after fixture judgment and a before/after read-only
check. Record concise observations and their disposition here. Only reproduced
actionable findings enter `findings.md`.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `<ID>` | `<concise evidence-backed observation>` | `cosmetic | extra-step | ambiguity | wrong-action-risk` | `retained | discarded | none` | `<reason or FT-NNNN>` |

## Cleanup

- Fixture paths removed: `<paths>`
- Main worktree after recording: `<git status evidence>`
