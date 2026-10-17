# Forward-test run: 2026-10-17 / Claude Code / governing inputs

[Back to the dashboard](../results.md).

- Tested build: `de3df11` plus Decision 0229 working tree
- Driver: real CLI session, configured model `claude-opus-5-5`
- Fixture language: `en`; scenario: A2
- Binary SHA-256: `ed1e549dcbbcc2ae3b756aca50638aac7eaa0f5fff75671083253c85e7b1e144`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| A2 | `environment_blocked` | Provider rejected the request before the first tool action | Original fixture; no proposal or product mutation | Exit 1, `Credit balance too low`, zero actions | Provider credit balance |

The child-only shell environment bound `specbind` to the fixture executable to
avoid ENV-0006; no global profile or installation changed. No Skill selection,
Steering read, dispatch or approval behavior is credited. The Codex app attempt
is recorded separately and cannot substitute for Claude coverage.

## Confirmation turns and debrief

None: the driver never began product work.

## Cleanup

The dedicated `a2` fixture, sibling harness logs and process-local environment
file under `C:/Users/hurui/AppData/Local/Temp/sb-audit-20261017` were removed
after recording, with the other fixtures in this audit batch. Main worktree
changes are the requested corrections and their verification records.
