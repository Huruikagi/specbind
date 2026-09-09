# Forward-test run: 2026-09-09 / Codex / 38a37f2

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-09`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `38a37f2`
- Fixture language: `en`
- Scenarios: `ST2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST2` (initial driver) | `environment_invalid` | The driver did not read the installed Status Skill | Clean; cart remained in Design; `tasks.yaml` SHA256 remained `CDD260EA...` | The driver used equivalent CLI reads after inspecting an agent profile, but its debrief confirmed it bypassed the installed Skill | none (`ENV-0005`) |
| `ST2` (installed-Skill retry) | `product_failure` | The response treated the future Tasks owner as the immediate next action and omitted intervening Design and Contract Review | Clean; cart remained in Design; `tasks.yaml` SHA256 remained `CDD260EA...` | The driver read the installed procedure and reported “next action is sb-plan ... reconcile the Tasks phase” although `spec status` derived `Next action: design` | `FT-0052` |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `ST2` (initial driver) | The installed Status Skill was not discovered | `extra-step` | `discarded` | `ENV-0005`; the retry explicitly loaded the fixture instruction and Skill trees |
| `ST2` (installed-Skill retry) | The response jumped from future ownership to Tasks as the next lifecycle action | `wrong-action-risk` | `retained` | `FT-0052` |

## Cleanup

- Fixture paths removed: `/tmp/sb-st2-38a37f2`
- Main worktree after recording: checked separately before the record commit
