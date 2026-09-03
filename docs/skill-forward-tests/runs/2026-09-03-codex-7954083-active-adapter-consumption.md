# Forward-test run: 2026-09-03 / Codex / 7954083

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `7954083`
- Fixture language: `en`
- Scenarios: `VI1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VI1` (invalid attempt) | `scenario_invalid` | The old request asked only whether the work was done, so Decision 0187 required consequence-free claim verification while the scenario expected lifecycle mutation. | `HEAD` `f6793ec`; clean; `cart` remained `implementation`, `completion=not_reached` | Driver returned `VERIFIED`; `spec status cart` still named validation as the next action and no completion evidence existed | none (scenario request corrected) |
| `VI1` (fresh retry) | `pass` | none | `HEAD` `6ac5207`; clean; `cart` reached `release_ready`, `completion=fresh`; only `spec.yaml` changed in the completion checkpoint | `adapter read validation --for consume` returned `NO_CHANGE ADAPTER_SCAFFOLD` without scaffold prose; the recorded Git Bash suite passed 4 tests; completion evidence names implementation revision `8aa28a8` | FT-0041 confirmed |

## Confirmation turns

None. The corrected request explicitly authorized recording lifecycle completion
when validation passed.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `VI1` | A hand-written runtime probe first tested `4 + 95` as over the accepted maximum and was corrected to `4 + 96` after rereading the Requirement. | wrong-action-risk | discarded | The exact greater-than-99 boundary was explicit, the driver corrected its own arithmetic, and the canonical recorded check remained the passing project suite. |

## Cleanup

- Fixture paths removed: `C:\Users\hurui\AppData\Local\Temp\sb-vi1-0189-7954083`, `C:\Users\hurui\AppData\Local\Temp\sb-vi1-0189-7954083-b`
- Main worktree after recording: only this forward-test record, dashboard,
  scenario correction, and finding-lifecycle updates were pending
