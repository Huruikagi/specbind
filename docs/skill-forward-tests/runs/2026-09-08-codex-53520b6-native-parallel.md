# Forward-test run: 2026-09-08 / native Codex CLI / parallel capability

[Back to the measurement dashboard](../results.md).

- Driver: standalone native `codex exec`, `gpt-5.6-terra` / `medium`, fresh session `01a07e92-3443-71b1-9b30-95a8b8f2085e`, normal automatic approval mode.
- Tested source: final fallback compatibility text committed in `53520b6`, before its grammatical correction; binary SHA256 `887055A0654827FA07E83E310322A7057C197286FF9E2F5DC9D3828A9A029D8E`.
- Fixture: `sb-dp1-0201-cli`, English, base `919b70e`.

## Measurement

DP1: **environment_blocked**. Isolated retained worker dispatch was not verified on this runtime; the driver chose sequential fallback. This does not establish that native Codex universally lacks worktree capability. Concurrent overlap, candidate replay and serial acceptance were not exercised.

The subtotal owner resolved an incompatible executable and returned blocked before mutation. Drive reread the authoritative integration state, parked that item and continued independent shipping. Commit `080428e` contains only shipping Task state, `src/shipping.py` and `tests/test_shipping.py`. Independent inspection and `sh scripts/test.sh` passed one shipping boundary test. The checkout is clean; shipping is 1/1 complete, subtotal is 0/1 pending, and total waits on subtotal. No final Spec validation, release invocation or version binding occurred.

Instrumentation contains shipping implementer and independent reviewer entries but omits the outer owners. This incomplete measurement cannot establish a full DP2 pass or close FT-0051. The successful shipping checkpoint is partial evidence only.

## Read-only debrief dispositions

The same CLI session answered a command-free debrief after mechanical judgment. Git status remained clean before and after it.

| Observation | Disposition |
| --- | --- |
| Initial report asserted the host could not isolate workers, although capability was merely unverified. | Measurement wording corrected here; no general host limitation inferred. |
| Subtotal resolved another CLI despite a PATH brief; shipping used the absolute fixture executable. | Fixture/environment dispatch limitation. Existing owner handoff already requires project-local executable/PATH facts; no new parallel product contract inferred from this failed setup. |
| Worker narration contradicted integration status. | Authoritative integration reread correctly preserved scheduling. |
| The blocked owner did not return retryable. | Existing bounded retry rule explains parking; debrief is not delivery retry authority. |
| Missing outer instrumentation. | Incomplete evidence retained; no successful full-workflow claim. |

## Cleanup

Attributed fixture state is captured above before cleanup. Raw native session/event logs outside the fixture remain available for diagnosis.

Cleanup completed: all attributed fixture directories were removed after evidence collection; the two empty DP1 linked worktrees were removed through Git first.
