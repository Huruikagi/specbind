# Forward-test run: 2026-09-08 / Codex / 07fc37c + final status-filter tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `07fc37c` + final status-filter working tree
- Binary SHA256: `814311E307E23436FCEC4170A2AE491910A3085288A08D793B6C7B12609F70A3`
- Fixture language: `ja`
- Scenarios: `ST1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `ST1` (first attempt) | `environment_invalid` | The specified Git Bash failed before product commands ran | Task state unchanged; only `.specbind/specs/cart/tasks.yaml` dirty | The driver reported Win32 signal-pipe error 5 and produced no status judgment | None |
| `ST1` (PowerShell retry) | `pass` | None | Task state unchanged; only `.specbind/specs/cart/tasks.yaml` dirty | The response retained implementation stage, `0/2`, the exact file dependency, no actionable work, and the condition to resolve; it omitted healthy evidence and selected no repair mechanism | `FT-0051` resolved |

## Confirmation turns

None. Status remained read-only.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `ST1` (PowerShell retry) | The driver read Milestone status once before loading the installed Skill and once afterward | `extra-step` | `discarded` | one redundant read with no user-visible or state impact |

## Cleanup

- Fixture paths removed: `tools/specbind/target/forward-tests/st1-filter-final-07fc37c` and `st1-filter-final2-07fc37c`
- Main worktree after recording: only the Status filtering implementation and forward-test records were pending
