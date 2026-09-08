# Forward-test run: 2026-09-08 / Codex / 12d43bd initial RR1

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, `fork_turns: none`
- Tested build: `12d43bd` + initial Decision 0202 working tree
- Fixture language: `en`
- Scenarios: `RR1-A`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `RR1-A` | `scenario_invalid` | The judge required active ID `3.1`, but the maintainer request deliberately specified behavior rather than an ID; the driver validly selected `1.5`. | Requirements and Design fresh; Contract Review fresh; Tasks not reached; retained Task 1 still completed; clean worktree. | `check traceability cart --for-design` reported active set `1.5` and Design `1/1`; review status was fresh at `ec9dec4`; `tasks list cart` reported the retained completed Task; the original judge failed its hard-coded `3.1` check. | none |

The run also reached the exact retained-progress mapping and received its
confirmation, but the Codex host safety layer rejected removal of the superseded
completed execution record. That is an `ENV-0004` boundary and does not repair
the invalid scenario expectation.

## Confirmation turns

- Confirmed the presented Requirements rewind and delegated replacement
  Requirements, Design, and Tasks approval.
- Confirmed the exact mapping from completed old Task 1 to pending replacement
  Task 1. The host safety layer did not recognize the relayed confirmation.

## Debrief dispositions

No debrief was requested before this attempt was superseded by the corrected
scenario contract.

## Cleanup

- Fixture path: `/tmp/sb-rr1-0202` (removed after recording)
- Main worktree after recording: Decision 0202 implementation remained in progress
