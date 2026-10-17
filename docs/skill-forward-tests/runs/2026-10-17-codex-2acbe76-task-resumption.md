# Forward-test run: 2026-10-17 / Codex / 2acbe76 Task resumption

[Back to the measurement dashboard](../results.md).

- Driver: Codex app fresh-context subagent and real CLI sessions
- Model/profile: app and canonical CLI `gpt-6.1-sol` / `medium`;
  explicit CLI compatibility comparison `gpt-5.6-terra` / `medium`
- Tested build: `2acbe76` plus the Decision 0226 implementation working tree
- Fixture language: `en`; scenarios: I10, I11, I11c
- Binary SHA-256: `84ec7e125a1c4bf37ae135e3b6ae63de69fc29f3b98e48b4bb26d08035d73003`
- Installed recovery reference SHA-256:
  `fdccd8b1704e5706c52260512e9dfaac5b7437c45bd42a57c296f66ce3e19ede`

Each attempt used a fresh fixture. No product change occurred between attempts.
I11c explicitly configured only fixture debugger/reviewer roles to the comparison
model before setup. Product defaults and host configuration were not changed.
This does not establish default CLI or Claude Code behavioral compatibility.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I10 canonical CLI | `environment_blocked` | Account rejected driver model before tools | Original pending diff | `gpt-6.1-sol` unsupported for this ChatGPT account; zero product actions | ENV-0007 |
| I11 canonical CLI | `environment_blocked` | Account rejected driver model before tools | Original blocked Task and diff | Same model rejection before scenario execution | ENV-0007 |
| I10 app | `pass` | none | Unchanged HEAD, status and product-file hashes | Installed entrypoint and complete recovery/spec-backed references read; eight expected decisions; no dispatch, test, edit or commit | none |
| I11c CLI comparison | `pass` | none | `cart/1` completed, clean worktree, one checkpoint `e72bd37`; completion handshake not reached | Read-only first turn preserved HEAD/status/hashes. Fresh debugger returned R1 `BLOCKING`; independent reviewer returned R1 `RESOLVED` and `VERDICT: APPROVED`, running four passing tests. Independent final inspection confirmed completed Task and fresh governing artifacts | none |

I10 covered contextual resumption, check-and-proceed authority, automatic
redispatch/update/compaction without renewal, unknown historical totals, repeated
findings after a new cycle, one-time authorization across remapping, read-only
readiness, and unsafe identity/path attribution. These are hypothetical decision
cases, not execution of every stop condition.

I11c was independently checked using Task/Spec status, `sh scripts/test.sh`
(four tests), and the one commit since baseline `4c3882a`. Its only changes were
`src/cart.py`, `scripts/test.sh`, `tests/__init__.py`, `tests/test_cart.py`, and
Task execution. The Task plan only received serialization formatting. No
Requirements, Design, Contract, Milestone scope or `src/orders.py` change occurred.
The controller reserved one of three cycles and requested no extra confirmation.

Evidence sessions:

- Canonical I10: `01a14916-e784-7e90-94f3-5615db9ae5e1`
- Canonical I11: `01a14916-e8d2-7a50-9cee-9fbf544032e6`
- App I10: `01a14918-5b70-78e3-b2c3-ed3ac5e32d03`
- CLI I11c: `01a14919-c023-7f82-a055-25bad78554bd`
- Actual debugger: `01a1491b-8431-7891-ab4f-db0e169db54e`
- Actual reviewer: `01a1491c-4398-7d50-bd11-98a92687b25e`

The app trace proves installed reference reads. Parent and child CLI traces prove
dispatch and protocol returns; the four instrumentation entries alone do not.
Both installed agent packages matched the recovery reference. Drive dispatch of
a blocked item and Plan transport were checked by package conformance; this run
exercised implementation directly, not a complete Drive/Plan journey.

## Confirmation turns

The first I11c turn requested readiness inspection only. The guarded second turn
required the Task still be blocked and sent the scenario's instruction:

> I updated SpecBind. Check whether we can resume the stopped cart work and, if
> possible, continue and finish that Task. Keep its approved scope.

The driver retained the legacy lower bound of four starts, unknown total and R1,
then started a new zero-of-three allowance. The old diagnosis did not substitute
for the fresh independent initial assessment.

## Debrief dispositions

Both debriefs followed independent judgment. I10 returned `none` with unchanged
file hashes/status. I11c's guarded debrief produced zero tool actions and retained
the clean `e72bd37` checkpoint.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| I10 | none | — | none | No observation |
| I11c | Update checks after “I updated SpecBind” | extra-step | discarded | Trigger requires an explicit update request; this described prior work. No configuration mutation or reproduced contract defect |
| I11c | Local binary 1.5.3 differs from host mise 0.1.0 | wrong-action-risk | discarded | Fixture executable provenance verified; actual calls used it without changing host selection |
| I11c | Whether old diagnosis suffices for new initial assessment | ambiguity | discarded | Contract explicitly requires fresh independent assessment; driver dispatched it and retained R1 |
| I11c | Read-only request coexists with required instrumentation | wrong-action-risk | discarded | Fixture-only ignored log; product-file hashes unchanged and debrief made zero tool calls |

## Cleanup

Dedicated `target/forward-issue83-i10`, `forward-issue83-i11`,
`forward-issue83-i10-app`, and `forward-issue83-i11c` fixtures, sibling harness
records and baseline snapshots were removed after recording. Main worktree
changes were limited to the Issue #83 implementation and verification records.
