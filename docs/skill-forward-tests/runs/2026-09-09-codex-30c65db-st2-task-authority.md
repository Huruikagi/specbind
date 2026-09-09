# Forward-test run: 2026-09-09 / Codex / 30c65db

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-09`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `30c65db`
- Fixture language: `en`
- Scenarios: `ST2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST2` | `scenario_invalid` | The recipe inherited VD3's pending Task, while ST2 requires a completed execution record for the previous active set | Clean; cart remained in Design with one pending Task | Direct `spec status` and `tasks list` showed `0 completed, 1 pending`; the driver therefore diagnosed a different state | none (fixture) |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

None. The invalid precondition was established mechanically before a product
verdict.

## Cleanup

- Fixture paths removed: `/tmp/sb-st2-30c65db`
- Main worktree after recording: checked separately before the record commit
