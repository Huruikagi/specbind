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
| 2026-09-02 | Codex | `gpt-5.6-terra` / `medium` | `d10bf9c` | A1 — pass; explicit reverse routing confirmed before status/list reads | [Existing-implementation routing](./runs/2026-09-02-codex-d10bf9c-adoption-routing.md) |
| 2026-09-02 | Codex | `gpt-5.6-terra` / `medium` | `7ffd343` | VI4 — pass | [Project Validation adapter](./runs/2026-09-02-codex-7ffd343-validation-adapter.md) |
| 2026-09-02 | Codex | `gpt-5.6-terra` / `medium` | `efba29e` | A2 — pass | [Reverse establishment proposal confirmation](./runs/2026-09-02-codex-efba29e-reverse-discovery.md) |
| 2026-09-02 | Codex | `gpt-5.6-terra` / `medium` | `d76af34` | A2 — product failure | [Reverse conflict classification](./runs/2026-09-02-codex-d76af34-reverse-discovery.md) |
| 2026-09-02 | Codex | `gpt-5.6-terra` / `medium` | `61d0d47` | A2 — product failure | [Reverse independent evidence dispatch](./runs/2026-09-02-codex-61d0d47-reverse-discovery.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `0886c8d` | DS9 — pass | [One-off Design materialization](./runs/2026-09-01-codex-0886c8d-ds9.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `f516d32` | DS9 — product failure | [Focused one-off Design assessment](./runs/2026-09-01-codex-f516d32-ds9.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `58e6155` | DS9 — pass | [One-off Design supplement confirmation](./runs/2026-09-01-codex-58e6155-ds9.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `8db72c0` | DS9 — product failure | [One-off Design supplement proposal](./runs/2026-09-01-codex-8db72c0-ds9.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `ada3fa9` | CF3 — pass | [Configuration candidate inference confirmation](./runs/2026-09-01-codex-ada3fa9-cf3.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `086620a` | CF3 — product failure | [Configuration candidate inference](./runs/2026-09-01-codex-086620a-cf3.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `3da9af7` | D15 canonical URL selector — pass; full multi-entry fixture remains blocked | [GitHub Milestone canonical URL](./runs/2026-09-01-codex-3da9af7-github-milestone-url.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `e381126` | D15 — environment blocked; authenticated fallback and confirmation boundary measured | [GitHub Milestone provider](./runs/2026-09-01-codex-e381126-github-milestone.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `db7686e` | S5 — pass | [Steering scaffold check](./runs/2026-09-01-codex-db7686e-steering-check.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `b1498fa` | D1 — environment-invalid attempt retained | [Skill namespace separation](./runs/2026-09-01-codex-b1498fa-skill-namespace.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `630f08e` | A1, A2 — environment-invalid attempts retained | [Adoption dispatch clarification](./runs/2026-09-01-codex-630f08e-adoption-discovery.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `163909b` | A1 — pass; A2 — product failure | [Adoption folded into Discovery](./runs/2026-09-01-codex-163909b-adoption-discovery.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `65a54e8` | Q0 — pass | [Planning scope fail-closed confirmation](./runs/2026-09-01-codex-65a54e8-q0.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `38a2951` | T1 — pass; Q0 — invalid attempts retained | [Plan phase package consolidation](./runs/2026-09-01-codex-38a2951-plan-merge.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `bcc05ca` | HP1 — environment blocked before product workflow | [Stable promotion journey](./runs/2026-09-01-codex-bcc05ca-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `3b294db` | HP1 — environment blocked after correct Requirements preservation | [Final RC planning journey](./runs/2026-09-01-codex-3b294db-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `8ee613d` | HP1 — environment blocked; split planning product failure retained | [Requirements semantic preflight](./runs/2026-09-01-codex-8ee613d-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `57ec8f6` | HP1 — product failure | [Requirements preservation ordering](./runs/2026-09-01-codex-57ec8f6-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `d68ae66` | HP1 — product failure | [Project-local CLI dispatch](./runs/2026-09-01-codex-d68ae66-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `78ec888` | R8 — pass; HP1 — product failure | [Requirements recovery and Design handoff](./runs/2026-09-01-codex-78ec888-planning.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `42724f6` | R8 — product failure; blocked attempt retained | [Resolved Requirements path routing](./runs/2026-09-01-codex-42724f6-r8.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `b8485c2` | HP1 — environment invalid/blocked | [Initial RC journey attempts](./runs/2026-09-01-codex-b8485c2-hp1.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `dfd858e` | R8 — product failure; environment-invalid fallback retained | [Requirements finding remediation](./runs/2026-09-01-codex-dfd858e-requirements.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `388b8bc` | R8 — pass | [Japanese language-style Rule](./runs/2026-09-01-codex-388b8bc-language-style.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `035f093` | DR1 — pass | [Milestone Drive branch-local attention](./runs/2026-09-01-codex-035f093-drive.md) |
| 2026-09-01 | Codex | `gpt-5.6-terra` / `medium` | `c012d50` | R6 — pass | [Named template creation outputs](./runs/2026-09-01-codex-c012d50-named-outputs.md) |
| 2026-08-31 | Codex | session default / fresh-context subagent | `20ca375` | completed-Task status recovery — pass | [Spec status routes to validation](./runs/2026-08-31-codex-20ca375-spec-status-validation.md) |
| 2026-08-31 | Codex | session default / fresh-context subagents | `1a843d9` | RT1, S5 — pass | [Final v1 recovery confirmations](./runs/2026-08-31-codex-1a843d9-v1-rc3-final.md) |
| 2026-08-31 | Codex | session default / fresh-context subagents | `8aaa198` | S5, RT1, HP1 — pass | [Duplicate recovery, deferred review, and HP1](./runs/2026-08-31-codex-8aaa198-v1-rc3.md) |
| 2026-08-31 | Codex | session default / fresh-context subagents | `7e6fd42` | X1 — pass; S5 — pass/product failure; invalid attempts retained | [Contract and Steering recovery](./runs/2026-08-31-codex-7e6fd42-v1-rc3-recovery.md) |
| 2026-08-31 | Codex | session default / fresh-context subagents | `d993293` | S5, RT1, CLI recovery, T1 — pass; X1 — product failure; blocked attempt retained | [Initial v1 rc.3 batch](./runs/2026-08-31-codex-d993293-v1-rc3.md) |
| 2026-08-31 | Codex | `gpt-5.6-terra` / `medium` | `546394f` | RL1 — pass | [Release binding preserves completion](./runs/2026-08-31-codex-546394f-release-binding.md) |
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
- Fixed, behavioral confirmation pending: 3
- Active environment limitations: 5

The authoritative rows and stable identifiers are in
[the findings worklist](./findings.md). Historical resolved findings through the
migration remain compacted in the legacy ledger rather than being reconstructed.
