# Forward-test run: 2026-08-31 / Codex / 7e6fd42

[Back to the measurement dashboard](../results.md).

- Date: `2026-08-31`
- Driver: `Codex`
- Model: `not exposed by the driver session`
- Driver profile: `session default; fresh-context subagents`
- Tested build: `7e6fd42`
- Fixture language: `en`
- Scenarios: `X1, S5`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `X1` recovery | `pass` | none | Design invalidated, corrected, approved, and committed; Contract Review accepted; clean fixture | The first report named Design, the current state and `specbind spec design invalidate cart`; invalidation occurred only after explicit confirmation | `FT-0002` |
| `X1` deep-input attempt 1 | `scenario_invalid` | Design did not specify an observable rejection channel | Review was not accepted | The fixture said only that the implementation “reports 99”, so it could not prove the Contract invariant | none |
| `X1` deep-input attempt 2 | `scenario_invalid` | The repaired recipe still did not fix the missing rejection semantics | Review was not accepted | Mechanical inspection found the same insufficient Design precondition | none |
| `X1` deep-input attempt 3 | `pass` | none | Fresh Contract Review state; milestone at Tasks | `.specbind/state/contract-review.md` recorded `specs/cart#design/main` and its fingerprint after Design established `ValueError` plus no mutation | `FT-0003` |
| `S5` unknown selector | `pass` | none | New `testing` Steering document; existing Steering preserved | The CLI error included `searched_project_path=.specbind/steering`; the fresh driver recovered and subsequent list/read succeeded | `FT-0001` |
| `S5` ambiguous selector, wrong-CLI attempt | `environment_invalid` | The driver invoked a global CLI instead of the fixture-local executable | No valid product measurement | The command did not exercise the installed fixture build | none |
| `S5` ambiguous selector, valid attempt | `product_failure` | Recovery removed a duplicate without Git history proving which path was newly introduced | Ambiguity was removed, but the decision basis was unsafe | The CLI named both colliding paths; debrief showed the Skill allowed choosing from matching content and a copy-like filename alone | `FT-0007` |

## Confirmation turns

X1 received explicit confirmation before Design invalidation and again before
the corrected Design gate. Contract Review acceptance followed only after the
corrected approved Design was committed.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `X1` deep input | Managed-graph checks cannot discover unmanaged source consumers | `extra-step` | `discarded` | Manual source inspection is the intended deep-input escalation |
| `X1` deep input | Acceptance output reports only an input count | `extra-step` | `discarded` | Exact identities remain available in the persisted review state |
| `S5` ambiguous | A duplicate diagnostic names both paths but cannot identify the survivor | `wrong-action-risk` | `retained` | `FT-0007` |

## Cleanup

- Fixture paths removed: `none; ignored fixtures retained until the release batch is recorded`
- Main worktree after recording: run-record changes only
