# Forward-test run: 2026-09-09 / Codex / 50f5d09

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-09`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `50f5d09`
- Fixture language: `en`
- Scenarios: `ST2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST2` | `scenario_invalid` | “Is the cart state broken?” selected diagnosis rather than the intended Status route | Clean; cart remained in Design; completed Task plan remained byte-identical | Precheck showed `1 completed, 0 pending`; the fresh driver returned a Debug diagnosis instead of a Status report | none (request wording) |

## Confirmation turns

None. The driver made no mutation.

## Debrief dispositions

None. The request was corrected to ask directly for current status and the
existing plan's next handling.

## Cleanup

- Fixture paths removed: `/tmp/sb-st2-50f5d09`
- Main worktree after recording: checked separately before the record commit
