# Forward-test run: 2026-09-08 / Codex / fallback clarification

[Back to the measurement dashboard](../results.md).

- Driver: fresh Codex subagent, `gpt-5.6-terra` / `medium`.
- Tested source: `618e22b` plus the ordinary-role fallback clarification subsequently committed in `53520b6`; built before the final genuine-no-subagent compatibility wording.
- Binary SHA256: `2AE73458B9A960CC911D9300816F8BCFCFA85F5AF6F409B434A9D04ABE18ABF1`.
- Fixture: `sb-dp2-0201-c`, English, initial revision `3fb826c`.

## Measurement

DP2: **environment_blocked**. The driver selected sequential fallback and delegated separate complete owners for subtotal and shipping. This exercises the distinction behind FT-0052, but does not establish successful independent Task review or completion.

Automatic approval review rejected the required instrumentation append for the subtotal implementer/debugger, and rejected shipping implementation `apply_patch` as an unapproved external-instruction-based unrelated feature addition. The driver stopped without bypassing either denial. Its canonical test command could not start Bash (`couldn't create signal pipe, Win32 error 5`). No test result is claimed.

Independent fixture inspection confirmed HEAD remains `3fb826c`, only `.specbind/specs/subtotal/tasks.yaml` is modified, subtotal Task 1 is blocked with the instrumentation denial reason, and shipping/total Tasks remain pending. Three instrumentation entries identify Drive and the two sequential owners; no successful reviewer instrumentation is present. No release operation or version binding occurred.

## Read-only debrief

The driver found the isolated-worker versus ordinary-role distinction explicit and followed it. It attributed the stops to approval/environment failures, not unclear Skill wording. This is consistent with the inspected partial evidence; no additional product finding is inferred. Git status before and after the command-free debrief remained the same single Task-state change. FT-0052 remains open for a complete successful confirmation.

## Cleanup

The fixture is attributed test data under the local Temp directory. Its blocked Task state and instrumentation evidence are recorded above before cleanup.

Cleanup completed: all attributed fixture directories were removed after evidence collection; the two empty DP1 linked worktrees were removed through Git first.
