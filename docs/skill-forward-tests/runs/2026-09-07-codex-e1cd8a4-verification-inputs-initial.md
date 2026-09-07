# Forward-test run: 2026-09-07 / Codex / e1cd8a4

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-07`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `e1cd8a4` + initial Issue #46 working tree, before preserving
  stronger Design verification was made explicit
- Binary SHA256: `68b527094f3421009cd0423389d9b31e187628d6ec1eec1ed81d91946e69dec1`
- Fixture language: `en`
- Scenarios: `T6`, `T7`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| T6 | `product_failure` | The driver replaced required real-connection proof with boundary checks without reporting the Design/ordering mismatch. | Two draft Tasks; Tasks gate not reached; upstream artifacts and implementation unchanged. | The draft references only `scripts/test.sh`; cart Design still explicitly rejects direct calls as sufficient, while `milestone scope` orders cart-input after cart. The final response attributed the lack of approval only to absent authority. | FT-0049 |
| T7 | `pass` | none | One approved Task, cart in implementation, cart-input still in tasks with no plan, clean worktree. | `tasks list cart`: 1 pending/actionable; `check traceability cart`: 4/4; direct input includes 98 plus 2 with unchanged-state rejection. Checkpoint `ef47ac5` changed only cart Tasks and gate metadata; diff against setup `268b312` proved upstream and implementation unchanged. | none |

## Confirmation turns

T7 first produced a validated draft without approving it. The maintainer approved
that exact cart plan for Tasks only, explicitly excluding implementation and
other planning phases. The fixture's Git adapter required the resulting local
checkpoint. T6 was judged before any approval: its draft and report had already
omitted the required mismatch.

## Debrief dispositions

Both debriefs were read-only; before/after Git status matched (T6 only untracked
Tasks, T7 clean).

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| T6 | The driver noticed the circular verification prerequisite but interpreted the protocol's boundary-proof option as permission to replace the stronger Design condition. | wrong-action-risk | retained | FT-0049; explicitly preserve approved Design verification obligations before applying boundary-proof alternatives. |
| T7 | A request to write Tasks did not itself grant gate approval. | ambiguity | discarded | Expected authority boundary; the subsequent exact Tasks approval completed correctly. |

Both driver summaries used Japanese despite the standalone English environment.
Artifacts were English; the checkpoint was separately checked against the fixture's
active Git adapter. These scenarios make no language-conformance claim.

An initial T6 setup stopped before driver dispatch because the newly authored
fixture Contract and Requirements used incorrect structural fields. The recipe
was corrected and a new fixture proved all preconditions before either run.
This was a harness setup error, not a product measurement.

## Cleanup

- Fixtures: `tools/specbind/target/forward-tests/issue46-t6` (setup only),
  `issue46-t6-v2`, and `issue46-t7`; removed after the final measurements.
- Main worktree after recording: Issue #46 implementation and verification files.
