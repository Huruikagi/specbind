# Forward-test run: 2026-09-01 / Codex / e381126

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `e381126`
- Fixture language: `en`
- Scenarios: `D15`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| D15 | environment_blocked | The available read-only Milestone had one closed Issue only; no prepared fixture exposed the required open/closed, duplicate, unresolved, and non-Issue combination. | No active milestone; no Specs; clean fixture worktree. | The retry driver read the installed `sb-discovery` procedure, used authenticated `gh` fallback to read `Huruikagi/specbind`, Milestone `1`, and the complete paginated inventory, then presented all five confirmation fields. `milestone status` and `milestone scope` both returned `NO_ACTIVE_MILESTONE`; `spec list` returned zero. | none |

## Attempts and disposition

The first fresh driver did not locate the installed `.agents/skills` tree and
therefore did not measure the product procedure. It is retained as an
environment-invalid attempt, not a product verdict. The retry used the same
request against a new fixture and reached the guarded confirmation boundary
without mutating the fixture or GitHub.

The partial live Milestone is sufficient evidence for authenticated fallback,
one-shot acquisition, and pre-confirmation immutability. It cannot prove the
full D15 collection combinations. Keep D15's scenario contract unchanged until
a dedicated read-only GitHub fixture is available.

## Debrief disposition

The invalid first driver reported an ambiguity locating installed Skills. It
was discarded because the retry read the installed tree and completed the
procedure. No post-judgment debrief was requested from the retry driver: its
result was environment-blocked rather than a complete scenario measurement.

## Cleanup

- Fixture paths removed after recording: `C:\Users\hurui\AppData\Local\Temp\sb-d15-e381126`, `C:\Users\hurui\AppData\Local\Temp\sb-d15-e381126-2`
- Main worktree after recording: this run record and dashboard update pending.
