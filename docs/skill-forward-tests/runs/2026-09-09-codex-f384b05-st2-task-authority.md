# Forward-test run: 2026-09-09 / Codex / f384b05

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-09`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `f384b05`
- Fixture language: `en`
- Scenarios: `ST2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST2` | `pass` | none | Clean; cart remained in Design; `tasks.yaml` SHA256 remained `CDD260EA199A6EC2E37BDFA38BA867E1A9B1240C852E04E9E4CFC82DD5791388` | The fresh driver loaded `sb-status`, reported inconsistent state, avoided a provenance label, and ordered Design, Contract Review, then Tasks reconciliation; `--for-design` remained clean for active Requirement `3.1` | `FT-0052` resolved |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `ST2` | The handoff instruction's “only” wording competed mildly with reporting the requested conclusion | `cosmetic` | `discarded` | The compact conclusion was necessary scenario output and introduced no wrong action |
| `ST2` | Initial inventory enumerated the full installed Skill tree before selecting Status | `extra-step` | `discarded` | One bounded read-only inspection with no effect on the answer or fixture |

## Cleanup

- Fixture paths removed: `/tmp/sb-st2-f384b05`
- Main worktree after recording: checked separately before the record commit
