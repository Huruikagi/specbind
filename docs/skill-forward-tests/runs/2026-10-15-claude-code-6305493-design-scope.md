# Forward-test run: 2026-10-15 / Claude Code / Design scope

[Back to the measurement dashboard](../results.md).

- Build: `6305493` plus initial Decision 0223 working tree
- Binary SHA-256: `f1498712ee8c6e4721afaccf9a1574ddd883e2356076c5e23a57498eb333a2af`
- Driver: Windows Claude Code real-session harness
- Model: `claude-opus-5-5`, host default
- Fixture: `en`, VD4

| Scenario | Verdict | Expectation that did not hold | Fixture state | Evidence |
| --- | --- | --- | --- | --- |
| VD4 | `environment_blocked` | Agent execution did not start | Clean at `7f982ee`, unchanged | Provider replied `Credit balance is too low`; zero actions. No product behavior measured |

No retry with another Claude model or account was attempted. A separate Codex
run used a fresh fixture. The unused child's PATH binding was process-local;
no host profile or credentials changed. No debrief was meaningful because no
workflow executed. This fixture and its `.drive` and `.env.sh` siblings were
removed after recording the outcome.
