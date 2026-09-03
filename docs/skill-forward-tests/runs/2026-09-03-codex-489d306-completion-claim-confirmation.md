# Forward-test run: 2026-09-03 / Codex / 489d306

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `489d306`
- Fixture language: `en`
- Scenarios: `VC1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VC1` | `pass` | none | `HEAD` `d8f9dbd4167f`; clean; cart remained `implementation`, `completion=not_reached` | Driver returned `VERIFIED` for current work `1.1`–`1.4`; exact active set, canonical 4-test suite, and runtime bounds all passed; no completion evidence was written | FT-0036 and FT-0040 confirmed |

## Confirmation turns

None.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `VC1` | The canonical Git Bash test initially hit Win32 signal-pipe denial and required the exact command to be rerun with execution permission. | extra-step | discarded | Environment execution boundary; the approved exact command passed and left the fixture clean. |
| `VC1` | The active set lookup was needed to distinguish current cart work from the retained baseline. | ambiguity | discarded | The Skill now names the exact CLI projection and the driver followed it without widening the claim. |

## Cleanup

- Fixture paths removed: `/tmp/sb-vc1f-0187`
- Main worktree after recording: checked separately before the record commit
