# Forward-test run: 2026-09-01 / Codex / 3da9af7

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `3da9af7`
- Fixture language: `en`
- Scenarios: `D15` (canonical URL selector)

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| D15 canonical URL | pass (URL selector subset) | The live Milestone still lacks D15's multi-entry fixture combinations. | No active milestone; no Specs; clean fixture worktree. | A fresh driver read the installed procedure, accepted `https://github.com/Huruikagi/specbind/milestone/1`, verified the canonical repository and Milestone, read the complete paginated Issue inventory, and presented the five-field proposal. `milestone status` remained `NO_ACTIVE_MILESTONE`; `spec list` found zero. | none |

## Cleanup

- Fixture path removed after recording: `C:\Users\hurui\AppData\Local\Temp\sb-d15-url-3da9af7`
