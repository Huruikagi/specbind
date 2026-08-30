# Forward-test measurement dashboard

[Back to the forward-test index](../skill-forward-tests.md).

This page is the current projection of recorded forward-test evidence. It is not
a claim that every passing scenario was measured against current `HEAD`.
Detailed measurements live in the [run archive](./runs/), and actionable
usability state lives in the [findings worklist](./findings.md).

## Normalized measurements

Normalized run records start after the 2026-08-30 migration accepted by
[Decision 0162](../design/decisions/0162-forward-test-record-lifecycle.md).

| Date | Driver | Model/profile | Tested build | Scenarios | Record |
| --- | --- | --- | --- | --- | --- |
| 2026-08-31 | Codex | `gpt-5.6-terra` / `medium` | `3719787` | D14, R7 — pass | [Local Source Collections](./runs/2026-08-31-codex-3719787-source-collections.md) |

Use the [run template](./run-template.md) for the next batch. Update this section
with the date, driver, tested build, scenarios, and link to that run; do not copy
its chronological narrative here.

## Historical passing coverage

This is the passing coverage projected from the legacy ledger through
2026-08-30. A scenario absent from an agent column has no recorded pass for that
agent. These entries intentionally do not infer per-scenario build metadata from
the former chronological prose.

| Workflow area | Claude Code passes | Codex passes |
| --- | --- | --- |
| Configuration | None recorded | CF2 |
| Discovery | D1, D2, D4-D6, D8-D12 | D4, D6, D13 |
| Requirements | R1-R5 | R1, R3, R4, R6 |
| Gap analysis | G1 | G1 |
| Checkpoint behavior | C1-C3 | C1-C3 |
| Steering | S5 | S5 |
| Existing-implementation adoption | None recorded | A1, A2 |
| Design | DS3 | DS1 (workflow only; investigation dispatch was not exercised), DS2, DS3, DS5, DS7, DS8 |
| Tasks | T2 | T1, T2, T4 |
| Contract review | X1-X4 | X1, X2, X4 |
| Implementation | I3, I4 | I1-I4, I6 |
| Debug | DB1 | DB1 |
| Task review | RT1 | RT1, RT2 |
| Design validation | None recorded | VD1, VD2 |
| Implementation validation | VI2, VI3 | VI1-VI3 |
| Claim verification | None recorded | VC1, VC2 |
| Release | RL1-RL3 | RL1-RL4 |
| Planning orchestrators | None recorded | Q0, Q4, B0 |
| End-to-end journey | None recorded | HP1 |

The complete pre-migration record, including product failures, environment
invalid runs, retries, and debrief dispositions, remains in
[the legacy ledger](./runs/legacy-through-2026-08-30.md).

## Current finding state

- Open reproduced product findings: 0
- Fixed, behavioral confirmation pending: 6
- Active environment limitations: 3

The authoritative rows and stable identifiers are in
[the findings worklist](./findings.md). Historical resolved findings through the
migration remain compacted in the legacy ledger rather than being reconstructed.
