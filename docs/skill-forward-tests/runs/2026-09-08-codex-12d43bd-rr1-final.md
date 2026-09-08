# Forward-test run: 2026-09-08 / Codex / 12d43bd final RR1

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, `fork_turns: none`
- Tested build: `12d43bd` + final Decision 0202 working tree
- Fixture language: `en`
- Scenarios: `RR1-B`, `RR1-C`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `RR1-B` | `environment_blocked` | After the accepted review and exact progress-mapping confirmation, the Codex host safety layer rejected deleting the superseded completed execution record. | Requirements and Design fresh; Contract Review fresh at `36e4660`; Tasks not reached; retained Task 1 still completed; clean worktree. | `spec status cart` reported stage `tasks`, review `fresh`, Tasks `not_reached`, and `TRACEABILITY_TASK_SCOPE_INACTIVE`; `tasks list cart` reported `1 completed`; the safety rejection persisted after an exact relayed deletion authorization. | `ENV-0004` |
| `RR1-C` | `environment_blocked` | The Codex host safety layer rejected the explicitly confirmed Requirements invalidation before the recovery could mutate lifecycle state. | Original Requirements, Design, Tasks, review, and completed Task remained fresh; clean worktree. | `spec status cart` reported `implementation`, all three gates fresh, review fresh, and `1 completed`; dispatch log had two contexts; the driver reported the exact invalidation was denied because retained physical records and invalidated evidence were treated as conflicting instructions. | `ENV-0004` |

RR1-B independently demonstrated the product behavior under test through the
renewed Contract Review: the old plan and completed execution record survived
the rewind, Design-scoped validation, Design approval, and review checkpoint.
The final Tasks write could not be measured in a fresh subagent. The focused CLI
integration test owns executable evidence for that final replacement and gate
approval.

## Confirmation turns

- In both attempts, confirmed the exact Requirements rewind after the driver
  presented its cumulative cost.
- In RR1-B, separately confirmed the driver's exact old-Task-to-new-Task and
  completed-to-pending mapping. This third turn is now part of the scenario
  contract because an unseen progress disposition must not be pre-approved.
- RR1-B also required a corrective rewind after the driver mangled its first
  assessment through shell interpolation. It recovered to a correct fresh review;
  this one-off driver mistake was not treated as a product finding.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `RR1-C` | Dispatch instrumentation writes before the request's no-mutation boundary. | ambiguity | discarded | Harness instrumentation is an explicit fixture exception and did not touch lifecycle state. |
| `RR1-C` | The driver tried `sb-status` and `specbind status` before the documented milestone command. | extra-step | discarded | The fixture instructions already distinguish Skill names from CLI commands. |
| `RR1-C` | Host safety interpreted retained task records and invalidated lifecycle evidence as contradictory. | wrong-action-risk | retained | Existing `ENV-0004`; fixture state and product instructions remained unchanged. |
| `RR1-C` | The driver had to distinguish replacing the active set from deleting still-valid baseline Requirements prose. | ambiguity | discarded | The requirements-review protocol explicitly retains the complete current contract; the driver selected the correct interpretation. |

The worktree was clean both before and after the read-only debrief.

## Cleanup

- Fixture paths: `/tmp/sb-rr1-0202b`, `/tmp/sb-rr1-0202c` (removed after recording)
- Main worktree after recording: Decision 0202 implementation remained in progress
