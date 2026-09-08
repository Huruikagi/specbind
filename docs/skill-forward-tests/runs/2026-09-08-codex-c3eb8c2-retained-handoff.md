# Forward-test run: 2026-09-08 / Codex / c3eb8c2

[Back to the measurement dashboard](../results.md).

- Driver: fresh Codex subagent, `gpt-5.6-terra` / `medium`.
- Tested build: `c3eb8c2` product assets; binary built before the source commit.
- Binary SHA256: `3D482D69CA2A3CAE3AE0FCBDF1D2A34F17A5A64EA307947A9F6BA5AB6CEAD08C`.
- Fixture language: English.
- Scenario: DP3, based on fresh `dp1` recipe without dispatch instrumentation.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DP3 | pass | none | Original and retained checkouts clean, unchanged; three Tasks pending; total still waits for subtotal and shipping | Original HEAD `eb315c7`, retained-subtotal HEAD `18ae25b`; worktree inventory still has exactly these two; CLI status unchanged | none |

The retained branch was seeded with only `src/subtotal.py`, an attributable
interrupted implementation checkpoint, before the driver ran. The maintainer
request explicitly named that prior result and asked to continue in parallel.
The driver loaded the installed procedure, inspected the known result and
stopped for a separate maintainer-directed operation. No new worker, integration,
cleanup, implementation, release or version binding was observed.

This proves only the known-retained-result stop boundary. It does not establish
DP1's concurrent execution or completed-worker handoff, interruption during
startup, or DP2's successful full sequential fallback. Those remain unmeasured
on this reduced build. Earlier integration-oriented run records are historical;
their expectations are superseded by the narrowed Decision 0201. FT-0052 remains
open pending complete ordinary-role fallback confirmation.

## Confirmation turns

No delivery approval or retry turn was added. The later read-only debrief did
not authorize implementation or integration.

## Debrief dispositions

The same driver reflected without commands after mechanical judgment. Both
checkouts and refs were unchanged before and after the debrief.

| Observation | Disposition |
| --- | --- |
| Known retained work correctly stopped dispatch despite a continuation request. | Intended boundary, no finding. |
| Driver tried nonexistent `specbind status --all` before the prescribed scheduler. | Discarded as a one-off extra command; installed Drive explicitly names the correct command and state was unchanged. |
| Retained checkout lacked ignored fixture-local CLI; a guessed PATH resolved another executable. | Fixture/environment limitation; authoritative original-checkout CLI and Git state were independently rechecked. No retained lifecycle result was accepted from that executable. |

## Cleanup

Fixture paths: `C:/Users/hurui/AppData/Local/Temp/sb-dp3-handoff` and
`C:/Users/hurui/AppData/Local/Temp/sb-dp3-handoff-retained`. Evidence above was
captured before harness cleanup. Cleanup is a test-harness action after judgment,
not behavior of the product route under test.

Cleanup completed: the retained worktree was removed through Git, then the fixture root was removed after absolute-path verification.
