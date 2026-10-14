# Forward-test run: 2026-10-14 / Codex / 8fc9917 + final completion recovery package

[Back to the measurement dashboard](../results.md).

- Driver: Codex real top-level session
- Model/profile: `gpt-5.6-terra` / `medium`
- Tested build: `8fc9917` plus Decision 0219, Drive/validation recovery and the
  project-instruction routing clarification
- Fixture: `dr9c`, English, with dispatch instrumentation
- Project-owned implementer/reviewer overrides: `gpt-5.6-terra` / `medium`,
  installed before the clean baseline; driver profile unchanged
- Session: `01a13ab2-998e-7861-bb28-5a9a83069f17`
- Project-instruction source SHA-256:
  `ea15a5078653dd1d17b14572c7771267e92f748d7599e616d20c4bb7f0a67360`
- Drive recovery reference matched the source and both installed agent packages:
  `67e309b6f05d7603bd46c3124e817576faa29eaaa46611c2d7e91f94fbe027e5`.

## Measurements

| Scenario/turn | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR9c / 1 | `environment_blocked` | Final validation commands could not run inside the sandbox | Clean, consistent validation stage; both Tasks completed, no completion accepted | `d9e0c38` withdraws only cart completion; `f70a590` adds the zero-quantity regression and completes cart-input Task 1. Fresh complete owners and internal implementation/review contexts are logged. Both final validators hit Git Bash `CreateFileMapping ... Win32 error 5`. | Local shell environment |
| DR9c / 2 | `product_failure` | Both completion records did not remain fresh after acceptance | Clean, two release-ready Specs; cart fresh, cart-input stale; milestone inconsistent | `f33af64` accepts cart-input against `f70a590`; `cfe0f91` accepts cart against `f33af64`. CLI reports `FRESHNESS_COMPLETION_PROJECT_CHANGED` for cart-input and `MILESTONE_SPEC_INCONSISTENT`. | FT-0059 |

The original #81 recovery boundary succeeded without a confirmation or restart:
one fresh validation owner withdrew completion, verified the narrow delta, and
checkpointed it separately; Drive resumed the pending implementation owner in
the same turn. The final acceptance failure is recorded separately and is not
relabelled a successful end-to-end run.

Independent post-run inspection confirmed four canonical tests pass and both
Tasks are complete. The withdrawal commit changes only cart state and completion;
earlier gates are byte-identical. The implementation checkpoint changes only
`tests/test_input.py` and cart-input Task progress. No Release ran.

## Environment continuation

After turn 1 was judged, the host ran the exact canonical test with native Git
Bash and confirmed all four tests pass. A user-channel environment update named
`C:/Program Files/Git/bin/sh.exe` and authorized its ordinary elevated execution
path for fresh fixture validation. Both clean-worktree and validation-stage
guards passed before sending it. No source, gate, Task or policy was repaired
between turns, and no test expectation was changed. Both validators then ran
fresh checks successfully; the later metadata commits exposed FT-0059.

## Debrief dispositions

The turn-3 read-only debrief had zero actions and preserved the clean worktree.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Drive stopped on inconsistent health despite an actionable validation | wrong-action-risk | discarded | Correct boundary: this new staleness was not caused by the authorized replan and did not qualify for automatic withdrawal. |
| Separate validation commits staled earlier accepted completion | ambiguity | retained | FT-0059, corroborated by the two exact evidence revisions and current CLI diagnostics. |
| Native Git Bash required host execution permission | extra-step | environment limitation | The canonical command itself passed. The original blocked result remains recorded. |

## Cleanup

The fixture and sibling driver records were removed after the
[final follow-up measurement](./2026-10-15-codex-8fc9917-grouped-completion.md)
confirmed the adjacent fix. No result is inferred from the agent's final report alone.
