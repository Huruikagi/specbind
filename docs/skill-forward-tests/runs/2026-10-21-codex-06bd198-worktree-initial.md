# Forward-test run: 2026-10-21 / Codex / initial worktree check

[Back to the measurement dashboard](../results.md).

- Tested build: `06bd198` + initial Decision 0235 tree
- Driver: app `gpt-6.1-sol` / `medium`, `fork_turns: none`; canonical CLI attempt
- Fixture language: `en`, standalone projects with their own instructions
- Binary SHA256: `B0B8ABE55416AA01A5C682EED5082612C8130B112C5442603CF6DFD5E6BDBB21`
- Spec-backed procedure SHA256: `A0802290CD64B4EEF31BF604B62C132FDE5AD3335D50305B5BEADC66EF5D8261`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I1 / CLI | environment_invalid | Driver could not start | Original clean fixture | Session `01a15de9-ed39-7ca2-be6f-9cc659e7dd61`, zero actions, account rejects `gpt-6.1-sol`; no final response | ENV-0007 |
| I1 / app | environment_invalid | Original snapshot unavailable, so review/completion could not proceed | Task 1 blocked through CLI; source and six tests preserved, uncommitted | Installed Skill read; external capture succeeds; check fails with OS errors 2 and 3 because the sandbox TEMP location differs. Fresh debugger diagnoses ENVIRONMENT; no replacement capture, review or completion. Independent status confirms blocked Task and original HEAD `156dc5b02868848e959f038fbadb279e05fb30cb`. | ENV-0010 |

App driver session: `01a15dea-11d8-7b61-a64a-89168c8738a4`. Actual trace
contains the initial capture and the exact failed check calls. The run leaves
`src/cart.py`, `tests/test_cart.py` and `.specbind/specs/cart/tasks.yaml` dirty.
This proves the failure stop, not the successful implementation boundary.
No confirmation was supplied.

## Debrief dispositions

After independent judgment, the driver reported one `Other / wrong-action-risk`
observation: it assumed `$env:TEMP` persisted across tools. The capture pipeline
returned zero, but subsequent checks could read neither current TEMP nor the
original absolute path. It preserved changes and obtained diagnosis instead of
recapturing. Before/after debrief status was identical.

Retain ENV-0010 as an environment limitation. Clarify the product procedure to
retain JSON in memory or a persistent accessible external location; this does not
add a new recovery permission. Fresh runs use a separately built final tree.
That tree also rejects trailing-dot/space path aliases on Windows; the initial
run makes no claim about that added input guard.

## Cleanup

Fixtures `ft-worktree-cli`, `ft-worktree-i1`, and the unused initial `ft-worktree-i4`
under `tools/specbind/target/` are removed after evidence/debrief collection.
The failed sandbox TEMP snapshot is already inaccessible. The main checkout
contains only this unit's implementation, docs, tests and measurement records.
