# Forward-test run: 2026-09-11 / Codex / a328f82

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-11`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `a328f82`
- Fixture language: `en`
- Scenarios: `Q6`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `Q6` | `product_failure` | After creating the unapproved Design locally, Plan requested a free slot instead of returning the complete terminal restart handoff | Requirements fresh; Design and Tasks not reached; Contract Review absent; `design.md` untracked and `contract.yaml` modified | Traceability was 4/4; `spec status cart` remained at Design; no gate or checkpoint advanced | Issue `#59`; `FT-0054` |

## Confirmation turns

The driver inferred delegated gate authority from the requested approved-plan
outcome instead of taking the required named-gate confirmation.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `Q6` | The capacity stop requested slot management and omitted the structured restart fields | `wrong-action-risk` | `retained` | Issue `#59`; terminal format strengthened in later builds |
| `Q6` | The complete-route confirmation was inferred instead of obtained | `wrong-action-risk` | `retained` | `FT-0054`; reproduced by another fresh driver |
| `Q6` | Inactive body markers initially failed traceability | `wrong-action-risk` | `discarded` | the CLI named the exact active set and the driver corrected the draft |

## Cleanup

- Fixture path removed: `sb-q6-a328f82`
- Main worktree after recording: forward-test records only
