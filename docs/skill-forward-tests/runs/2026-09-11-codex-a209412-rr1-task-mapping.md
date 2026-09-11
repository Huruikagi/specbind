# Forward-test run: 2026-09-11 / Codex / a209412 + Issue 61 tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-11`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `a209412` plus the Issue 61 project-instruction and Tasks-reference working tree
- Fixture language: `en`
- Scenarios: `RR1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `RR1` | `environment_blocked` | The confirmed retained-Task replacement and Tasks approval could not run because the host safety layer did not accept relayed confirmation. | Requirements and Design are fresh for active Requirement `1.5`; renewed Contract Review is fresh at `9679fca`; the old plan and its completed record remain byte-identical and Tasks is not reached; worktree clean. | The driver presented the exact remove-completed-old-Task/create-pending-new-Task mapping and attempted it only after confirmation. `specbind spec status cart` reports `Task plan authority: not current; reconcile in Tasks phase`; `specbind tasks list cart` reports the old `1 completed`; SHA-256 remains `f60fb1c879255a7ee04fdfa52ee5a29d6eada43478d4f54e9f0336210a7d8406`. The judge passed the replacement active-set check and then stopped before complete Design coverage because retained Tasks still make the complete projection fail. | `ENV-0004` |

The host also blocked the confirmed Requirements invalidation. The harness
applied that exact lifecycle mutation from the parent context, after which the
same driver completed replacement Requirements, Design, and renewed Contract
Review. The fixture was not repaired after the final Tasks-mapping block.

## Confirmation turns

- The first turn requested only the exact Requirements rewind cost and forbade mutation.
- The second turn confirmed that exact Requirements rewind, delegated replacement Requirements and Design approvals, required byte-identical Tasks through renewed review, and stopped at the progress mapping.
- The third turn confirmed the exact mapping the driver presented and authorized only its application, Tasks approval, and planning checkpoint.

## Debrief dispositions

The fixture was clean before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `RR1` | Milestone status reportedly exposed validation while the retained plan was non-current and review absent. | `wrong-action-risk` | `retained` | Requires separate reproduction against the current milestone action contract before entering `findings.md`. |
| `RR1` | Baseline comparison required distinguishing a retained Contract invariant from the return-identity change. | `ambiguity` | `discarded` | The accepted baseline and retained Requirements resolved the distinction without a product contradiction. |
| `RR1` | An unquoted comma-containing flow-mapping description produced a schema diagnostic and one rewrite. | `extra-step` | `discarded` | Ordinary YAML quoting error with a precise structural diagnostic. |
| `RR1` | The host rejected both relayed confirmations for destructive state changes. | `ambiguity` | `retained` | Existing `ENV-0004`; the product instructions were followed through the attempted exact mapping. |

## Cleanup

- Fixture path removed after the Issue 61 delivery commit: `C:/Users/hurui/AppData/Local/Temp/sb-rr1-issue61`
- Main worktree after cleanup: forward-test documentation only
