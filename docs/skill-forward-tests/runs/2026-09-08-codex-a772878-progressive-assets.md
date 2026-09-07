# Forward-test run: 2026-09-08 / Codex / a772878

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `a772878`
- Fixture language: `en`
- Scenarios: `T1, Q4, DR5`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `T1` | `pass` | `none` | Requirements, Design, Contract Review, and Tasks were fresh; one pending Task; clean fixture | `milestone status` reported implementation with fresh Contract Review; `spec status cart` reported all three gates fresh; commit `9703d1c` contained the approved Tasks plan and no implementation | `none` |
| `Q4` | `environment_blocked` | Contract Review and Tasks could not be measured because the fresh validator could not execute the fixture-local binary | Requirements were fresh; an unapproved `design.md` was left untracked; Design and Tasks remained `not_reached`; Contract Review was absent | `milestone status` reported design; `spec status cart` reported Requirements fresh and delegated to `sb-plan`; commit `d84b749` checkpointed Requirements | `FT-0050` from the post-judgment debrief, not from the environment stop |
| `DR5` | `pass` | `none` | The approved 99 limit was implemented and its sole Task completed; Requirements, Design, and Contract stayed at 99; clean fixture at validation | Commit `2dabf8f` changed only `tasks.yaml`, `src/cart.py`, and `tests/test_cart.py`; four tests passed; `spec status cart` reported 1/1 Tasks complete; the requested 120 limit was classified as a separate Requirements decision | `none` |

## Confirmation turns

T1 received Tasks-gate authority only after presenting its complete draft and
was told to stop after that phase. Q4 received one bounded delegation for
Requirements, Design, and Tasks and was told to stop after the plan. DR5's
initial request explicitly delegated only Design, Contract, and Tasks recovery;
it gave no Requirements or release authority.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `T1` | none | `cosmetic` | `none` | none |
| `Q4` | The Design phase said validation was merely a user-invoked second opinion while the complete route required it before delegated approval | `wrong-action-risk` | `retained` | `FT-0050`; reproduced against Decisions 0104 and 0194 before editing |
| `Q4` | The fixture-local validator executable was denied twice | `extra-step` | `discarded` | host environment limitation, not a product contract |
| `DR5` | The inactive validation adapter supplied no canonical completion command | `ambiguity` | `discarded` | fixture intentionally lacks a verification contract; the driver safely returned `MANUAL_VERIFY_REQUIRED` |

## Cleanup

- Fixture paths removed: `ft-t1-a772878`, `ft-q4-a772878`, and `ft-dr5-a772878` under the run's visualization root
- Main worktree after recording: only this forward-test documentation was pending
