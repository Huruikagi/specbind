# Forward-test run: 2026-09-03 / Codex / 7c3dcab

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `7c3dcab`
- Fixture language: `en`
- Scenarios: `VC1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VC1` | `product_failure` | The driver widened “cart work” from the active change to baseline Requirement 2 and returned `NOT_VERIFIED`. | `HEAD` `62180155e3f9`; clean; cart remained `implementation`, `completion=not_reached` | Route precedence worked and nothing mutated, but the driver ignored the CLI-owned active set `1.1`–`1.4` when scoping the claim | FT-0040 |

## Confirmation turns

None.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `VC1` | The current-work claim could be widened to accepted baseline Requirements retained in the document. | wrong-action-risk | retained | FT-0040 |

## Cleanup

- Fixture paths removed: `/tmp/sb-vc1c-0187`
- Main worktree after recording: checked separately before the record commit
