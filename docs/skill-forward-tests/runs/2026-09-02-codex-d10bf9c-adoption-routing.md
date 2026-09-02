# Forward-test run: 2026-09-02 / Codex / d10bf9c

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-02`
- Driver: Codex
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context with no prior turns
- Tested build: `d10bf9c`
- Fixture language: `en`
- Scenarios: `A1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A1` | `pass` | `none` | No dossier, milestone, or Spec; clean worktree. | The fresh driver discovered the installed route, reported running only `specbind adoption preflight`, and stopped on `ADOPTION_STEERING_REQUIRED`. Independent fixture reads reported `NO_ACTIVE_MILESTONE`, zero Specs, no dossier, and an empty `git status --short`. | `FT-0018` partially confirmed |

## Confirmation turns

None. A1 stopped at the missing-Steering prerequisite.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A1` | Steering bootstrap requires a separate invocation after preflight stops. | `wrong-action-risk` | `discarded` | Intentional Steering-first stop accepted by Decisions 0143, 0175, and 0181. |
| `A1` | The driver read `README.md` and `src/` before preflight but did not use them to form boundaries or continue after the diagnostic. | `wrong-action-risk` | `retained` | `FT-0018`; A1 confirms the status/list routing branch, while the independent-reader ordering still requires instrumented A2 confirmation. |

## Cleanup

- Fixture paths removed: `C:\Users\hurui\AppData\Local\Temp\sb-a1-d10bf9c`
- Main worktree after recording: only this run record and dashboard/worklist updates
