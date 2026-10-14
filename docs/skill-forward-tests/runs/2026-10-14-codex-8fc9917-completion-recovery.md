# Forward-test run: 2026-10-14 / Codex / 8fc9917 + completion recovery

[Back to the measurement dashboard](../results.md).

- Driver: Codex real top-level sessions through `forward-test-drive.sh`
- Model/profile: `gpt-5.6-terra` / `medium`
- Tested build: `8fc9917` plus Decision 0219, Drive and validation Skill changes,
  before the subsequent project-instruction routing clarification
- Fixture language: `en`
- Scenarios: DR9 (default installed internal models), DR10
- Drive recovery reference SHA-256:
  `67e309b6f05d7603bd46c3124e817576faa29eaaa46611c2d7e91f94fbe027e5`
- Source and Codex/Claude Code installed recovery references matched byte-for-byte.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR9 | `environment_blocked` | Remaining implementation and final validation could not execute | Clean, consistent implementation; cart completion withdrawn; cart-input Task 1 pending | Fresh withdrawal owner created `2958d2844980f7b959d5cb47d574709fcb75233c`; its only diff is cart state plus removal of completion. Drive then dispatched the complete implementation owner, whose registered implementer failed to start. | ENV-0007 |
| DR10 | `pass` | none | HEAD `583cbe2` and clean worktree unchanged | Two read-only actions load installed Drive entrypoint and replan procedure. All eight decisions match their boundary expectations; no mutation or dispatch. | none |

DR9 instrumentation records the controller, complete withdrawal owner, and
complete implementation owner. Independent CLI reread reports consistent
health, fresh Contract Review, cart Task 1 completed and no completion, and only
cart-input implementation actionable. No premature completion preflight or
acceptance occurred. The unsupported registered model was `gpt-6.1-sol`; no
substitute role was used inside that run.

The first fixture setup attempt used an incorrect approval argument and stopped
before any agent ran. The recipe was corrected to `--delegation-workflow`, and
the disposable fixture was rebuilt before measurement.

DR9 emitted Japanese commentary/checkpoint text despite the English fixture.
The raw session instructions were inspected: fixture instructions and installed
Skills were selected, and the repository's direct-to-main instructions were not
inherited. User-level Japanese permission guidance remained. The run is not
used as English-language behavior evidence; the follow-up fixture explicitly
isolates English prose. Checkpoint content and lifecycle results above were
read directly, not inferred from its narration.

Sessions: DR9 `01a13aad-47f7-77b3-a7b2-523a71a96835`;
DR10 `01a13aad-87ae-7252-a8f0-83125afd8222`.

## Debrief dispositions

Both read-only debriefs had zero actions and preserved clean fixture status.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| DR9 | Initial commentary before the language Rule read | wrong-action-risk | discarded | Host progress instructions explain the initial update; no lifecycle authority changed. |
| DR9 | Causality comparison required baseline, diff and status evidence | ambiguity | discarded | Intentional boundary. The debrief's claim of staged planning files contradicts the recorded clean status; no product defect was reproduced. |
| DR9 | Registered implementer could not start | wrong-action-risk | retained | Existing ENV-0007; original failure remains recorded. |
| DR10 | Hypothetical delegated authority wording required interpretation | ambiguity | discarded | The scenario explicitly supplies it; the answer preserved the ordinary no-authority boundary. |
| DR10 | Hypothetical cases lack a live scheduler result | wrong-action-risk | discarded | This scenario measures conditional decisions only. No owner was actually selected or run from invented state. |

## Cleanup

Fixtures and sibling driver records were removed after recording the final
follow-up measurement. Only intended repository edits are retained.
