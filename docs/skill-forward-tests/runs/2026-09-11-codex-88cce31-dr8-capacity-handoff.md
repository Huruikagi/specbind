# Forward-test run: 2026-09-11 / Codex / 88cce31 / DR8

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-11`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `88cce31`
- Fixture language: `en`
- Scenarios: `DR8`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `DR8` | `product_failure` | Drive spent the final child slot on an approval helper before starting the exact `sb-plan` owner, so the nested Plan capacity handoff was never produced | Requirements fresh; Design and Tasks not reached; Contract Review absent; only `design.md` and `spec.yaml` modified | The live hierarchy and dispatch log showed `approval_review` before `sb-plan`; Plan startup then failed for capacity and Drive returned a pre-owner capsule | `FT-0055` |

## Confirmation turns

The request explicitly delegated Design and Tasks approval to the Drive run;
Requirements was already fresh.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `DR8` | The driver did not identify its pre-owner approval helper as friction | `wrong-action-risk` | `retained` | mechanical dispatch evidence reproduced `FT-0055` independently of the debrief |

## Cleanup

- Fixture path removed: `sb-dr8-88cce31`
- Main worktree after recording: forward-test records only
