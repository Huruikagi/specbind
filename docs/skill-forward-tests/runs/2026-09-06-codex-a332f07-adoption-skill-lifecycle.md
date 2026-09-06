# Forward-test run: 2026-09-06 / Codex / a332f07

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-06`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `a332f07`
- Fixture language: `en`
- Scenarios: `A1, A4`

One instrumented A1 attempt is retained separately from its fresh retry because
the harness changed the behavior being measured.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| A1 (instrumented attempt) | `environment_invalid` | The driver must reach the installed product workflow; it instead treated the fixture-only dispatch log instruction as untrusted and stopped the entire request. | Clean worktree; no product command or mutation. | The driver read fixture files, refused `.forward-test/agents.log`, and stopped. | none |
| A1 (fresh non-instrumented retry) | `pass` | none | Clean worktree; no Specs or active Milestone; `sb-adopt` remained installed. | `specbind adoption preflight` returned `ADOPTION_STEERING_REQUIRED`; `git status --short` was empty; `.specbind/specs` remained absent. | none |
| A4 | `pass` | none | Clean worktree after two Git-adapter checkpoints; `cart` and `order` idle with reverse `v1.0.0` provenance; no active Milestone; both Agent copies of `sb-adopt` absent; adoption disabled. | `milestone status --json` returned `NO_ACTIVE_MILESTONE`; `spec list` reported two idle Specs; both Skill paths and the temporary adoption record were absent; `.specbind.json` omitted `adoption`; `git diff 31a6b62..HEAD -- src README.md` was empty; no tag existed. | none |

## Confirmation turns

None. The resumed reverse checkpoint already carried delegated gate authority;
the driver presented and accepted Contract Review without inventing another
scope confirmation.

## Debrief dispositions

The debriefs occurred after fixture judgment. Read-only `git status --short`
checks were empty before and after both debriefs.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| A1 | Missing Steering and the later need for an explicit represented version were clear. | cosmetic | discarded | This is the scenario's intended fail-closed boundary, not workflow friction. |
| A4 | “Continue” initially suggested Drive, while status routed the reverse resume handler to `sb-adopt` and nested Contract Review to `sb-contract-review`. | extra-step | discarded | The exact status handler and complete `sb-adopt` resume procedure resolved ownership before any wrong action or repeated discovery. |
| A4 | `adoption preflight` is not useful as a post-finalization check because persistent Specs now exist. | cosmetic | discarded | The procedure uses Milestone and Spec status for completion evidence and does not instruct a post-finalization adoption preflight. |

## Cleanup

- Fixture paths removed: `/tmp/specbind-forward-a1-a332f07`, `/tmp/specbind-forward-a1b-a332f07`, `/tmp/specbind-forward-a4-a332f07`, `/tmp/specbind-forward-a4b-a332f07`
- Main worktree after recording: clean after the measurement-record commit
