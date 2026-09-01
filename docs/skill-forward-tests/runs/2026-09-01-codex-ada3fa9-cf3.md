# Forward-test run: 2026-09-01 / Codex / ada3fa9

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `ada3fa9`
- Fixture language: `en`
- Scenarios: `CF3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| CF3 | `pass` | none | Modified only `.specbind/settings/templates/specs/design.md`; `cart` remained idle. | `steering list` found two documents; `template list spec` retained seven recognized templates; `design-template-selection` was unchanged; `spec status cart` reported idle; `git diff --check` passed. | FT-0019 |

## Confirmation turns

None. The maintainer's explicit configuration request authorized a project-owned
template edit; no candidate-set change was inferred.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| CF3 | The first `template read` omitted its scope. | `extra-step` | discarded | CLI help supplied the accepted route. |
| CF3 | The request did not name whether UI should be edited, but the selection Rule limits UI to user-visible behavior and the driver left it unchanged. | `wrong-action-risk` | discarded | The current Rule supplied a correct, observed resolution. |

## Cleanup

- Fixture paths removed: pending after this run record is committed.
- Main worktree after recording: contains this confirmation record and dashboard projection.
