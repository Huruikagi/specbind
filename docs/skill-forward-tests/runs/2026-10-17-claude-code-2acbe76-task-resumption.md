# Forward-test run: 2026-10-17 / Claude Code / 2acbe76 Task resumption

[Back to the measurement dashboard](../results.md).

- Driver: Claude Code, real top-level session through `forward-test-drive.sh`
- Model: `claude-opus-5-5`, agent-default effort
- Tested build: `2acbe76` plus the Decision 0226 implementation working tree
- Fixture language: `en`
- Scenario: I11, prepared with dispatch instrumentation
- Binary SHA-256: `84ec7e125a1c4bf37ae135e3b6ae63de69fc29f3b98e48b4bb26d08035d73003`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I11 | `environment_blocked` | The provider could not start product work | Original blocked Task and pending cart diff | First turn returned `Credit balance is too low`; zero tool actions, process exit 1 | Provider credit availability; no product verdict |

No model substitution or provider/account change was made. No resumption turn
or debrief was sent because the scenario never executed. This attempt does not
establish Claude Code behavioral compatibility. Both agent packages are still
covered by static conformance and installed-reference hash comparisons.

## Cleanup

The dedicated fixture and sibling `.drive` records are removed after recording
their evidence. No product files or host configuration were changed by the run.
