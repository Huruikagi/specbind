# Forward-test run: 2026-10-17 / Codex / final progress mode

[Back to the dashboard](../results.md).

- Tested build: `0a3fe1e` plus final Decision 0227 tree, including FT-0068 correction
- Driver: real CLI `gpt-5.6-terra` / `medium`, explicit compatibility comparison
- I13 fixture reviewer/debugger: `gpt-5.6-terra` / `medium`; product defaults unchanged
- Fixture language: `en`; scenarios: I12 and I13, fresh fixtures, unchanged prompts
- Binary SHA-256: `a1b08d6b661853347518b0213105e51d2cff8a3fbe7b9fff6f3ddf962d32e841`
- Recovery reference SHA-256: `43da018e764ce4a4c6c6ce609da9ebf89d86eae2f78df75628a3781fdbf1e9a3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I12 | `pass` | none | Original pending diff; HEAD/status/path set/all product hashes unchanged | Four read commands included the complete recovery reference and owner instructions; all 12 decisions matched; no workers, tests or mutation | none |
| I13 | `pass` | none | Task completed; clean single checkpoint `87a6835`; Spec in implementation with fresh gates and no completion handshake | Before dispatch the owner explicitly selected cycle four; real reviewer returned R1–R4 RESOLVED and APPROVED; final ledger retained cart-cap/U1 and reported four used, uncapped remaining. Canonical tests passed; independent fixture inspection confirmed scope and checkpoint | FT-0068 resolved |

The [initial I13 failure](./2026-10-17-codex-0a3fe1e-progress-initial.md) remains
a failed measurement. The correction explicitly reserves the review as a
corrective action and reports the resulting count even without source edits.
No prompt hint about incrementing was added for this rerun.

Evidence sessions:

- I12: `01a14992-346e-7983-97a0-d92480bbb78b`
- I13: `01a14991-be4d-7772-a074-79c831f9cbf4`
- Actual I13 reviewer: `01a14992-256c-77f3-9e9d-0f4ea4f7a686`

The independent reviewer inspected the approved artifacts and unchanged diff,
then ran the four underlying tests with Python's no-bytecode option after its
Git Bash process could not create a Windows mapping. The owner retried the
exact canonical `sh scripts/test.sh` with host-approved process permission and
got four passes. The maintainer independently reran that canonical command:
four passes. No alternative verification weakened the required final check.

`tasks list cart` showed one completed, zero pending and zero blocked Tasks.
`spec status cart` showed consistent implementation, fresh Requirements/Design/
Tasks/Contract Review, coverage 4/4 and completion not reached. One commit since
the baseline changed only `src/cart.py`, `scripts/test.sh`, `tests/__init__.py`,
`tests/test_cart.py` and Task execution metadata; Task plan changes were only
serialization formatting. No checkout, scope, Gate or approved-artifact change.

Both installed agent reference packages matched the final sources. This is
Codex comparison-model evidence, not default-model CLI or Claude behavior.
Drive/Plan mode transport was covered by package conformance and I12's decisions;
no full Drive/replan journey or actual mode-switch execution was measured.

## Confirmation turns

No fresh allowance, reset, additional scope approval or release was requested.
I13 retained the existing execution and its selected progress mode. The only
host permission retry was for actual process/Git prerequisites; no restriction
was bypassed.

## Debrief dispositions

Debriefs followed independent judgment and made zero tool calls. I12 hashes and
status stayed unchanged; I13 retained its clean `87a6835` checkpoint.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| I12 | Distinguishing resumption “continue” from progress selection | ambiguity | discarded | Separate explicit preconditions are satisfied correctly; no fourth bounded cycle was authorized |
| I12 | Finding the installed workflow needed a search | extra-step | discarded | Intentional natural request tests discovery without supplying a Skill name |
| I13 | Recognizing a corrected review consumes a cycle | wrong-action-risk | retained/resolved | FT-0068; corrected instructions were used and cycle four was recorded |
| I13 | Windows mapping failure before canonical tests started | extra-step | discarded | Exact command succeeded with host permission and independently after the run |
| I13 | “Finish only that Task” versus required checkpoint | ambiguity | discarded | Applicable Git adapter requires this completed-Task checkpoint; no extra Task or lifecycle action occurred |

## Cleanup

Dedicated `target/forward-progress-i12-final` and `forward-progress-i13c-final`
fixtures and sibling harness/baseline records were removed after evidence
recording. Main worktree changes contain only the requested feature and records.
