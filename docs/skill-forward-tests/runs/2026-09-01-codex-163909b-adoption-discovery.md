# Forward-test run: 2026-09-01 / Codex / 163909b

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh contexts with no prior turns
- Tested build: `163909b`
- Fixture language: `en`
- Scenarios: `A1`, `A2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A1` | `pass` | `none` | No dossier, milestone, or Spec; clean worktree. | `specbind adoption preflight` reported `ADOPTION_STEERING_REQUIRED`; `.specbind` retained only settings, Steering, the fixture binary, and its ignore file. | `FT-0018` |
| `A2` | `product_failure` | The driver inspected the implementation itself and did not dispatch the required two fresh readers before synthesis. | No dossier, milestone, Spec, Brief, or Research; clean worktree. | Preflight returned fixture HEAD `bca1217836787601f2e649603a504682c0f06ca3`, but `.forward-test/agents.log` held only the driver line. | `FT-0018` |

## Confirmation turns

None. A1 stopped at the missing-Steering prerequisite. A2 was explicitly
limited to the first adoption-boundary confirmation.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A1` | Ordinary Discovery's unconditional project-shape wording competed with the selected adoption-start reference. | `extra-step` | `retained` | `FT-0018` |
| `A2` | The start reference did not make the two independent evidence lines and the orchestrator's non-substitution rule concrete enough. | `wrong-action-risk` | `retained` | `FT-0018` |

## Cleanup

- Fixture paths removed after both build records were completed: `/tmp/sb-a1-163909b`, `/tmp/sb-a2-163909b`
- Main worktree after recording: only forward-test record changes
