# Forward-test run: 2026-09-09 / Codex / 4c5c2b4

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-09`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `4c5c2b4`
- Fixture language: `en`
- Scenarios: `ST2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST2` | `product_failure` | Lifecycle order was correct, but the response called the plan retained even though current state cannot prove provenance | Clean; cart remained in Design; `tasks.yaml` SHA256 remained `CDD260EA...` | The driver loaded `sb-status`, named Design then Contract Review then Tasks, but reported “its retained Task plan” | `FT-0052` |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `ST2` | One unsupported command spelling was corrected after rereading the named-Spec procedure | `extra-step` | `discarded` | Harmless read-only recovery; the prescribed command then succeeded |
| `ST2` | The answer labeled unknown plan provenance as retained | `ambiguity` | `retained` | `FT-0052` |

## Cleanup

- Fixture paths removed: `/tmp/sb-st2-4c5c2b4`
- Main worktree after recording: checked separately before the record commit
