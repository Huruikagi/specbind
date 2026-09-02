# Forward-test run: 2026-09-03 / Codex / 29b247c

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `29b247c`
- Fixture language: `en`
- Scenarios: `A3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A3` | `product_failure` | The driver read both `README.md` and Steering but omitted the direct `Bookshp` versus `Bookshop` contradiction from the proposal, leaving no pending suspected-defect record for the post-create ordering branch. | No active milestone, 0 Specs, no deferred destination, clean worktree at `a26cc1c005612ef2e2d74de1f7aa9e102b11a6de`. | `milestone status` returned `NO_ACTIVE_MILESTONE`; `spec list` found 0; `.specbind/deferred.md` was absent; `git status --short` was empty. Instrumentation recorded the driver and two independent readers. | `FT-0027` |

## Confirmation turns

The driver reached the proposal boundary with no blocking unknown but omitted
the expected suspected defect. The proposal was not confirmed because its
finding set was incomplete for this scenario.

## Debrief dispositions

`git status --short` was empty before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A3` | The driver read legacy adoption references despite the entry procedure selecting only the current reverse reference. | `extra-step` | `discarded` | The current instruction is explicit and the extra read did not affect fixture state. |
| `A3` | The driver followed the mandatory fresh-reader ordering before synthesis. | `extra-step` | `discarded` | Required product behavior, not usability friction. |

## Cleanup

- Fixture paths removed: `C:\Users\hurui\AppData\Local\Temp\sb-a3-29b247c`
- Main worktree after recording: FT-0027 fix and this run record only
