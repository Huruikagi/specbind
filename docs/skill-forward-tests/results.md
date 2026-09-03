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
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `ff010dd` | D16 — owned-path Discovery entry confirmed; full Skill flow blocked by ENV-0005 | [Owned-path routing confirmation](./runs/2026-09-03-codex-ff010dd-file-ownership-routing.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `d59eae8` | D16 — product failure after one environment-invalid attempt; FT-0042 opened | [Owned-path routing](./runs/2026-09-03-codex-d59eae8-file-ownership-routing.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `7954083` | VI1 — pass after correcting one invalid legacy request | [Active adapter consumption](./runs/2026-09-03-codex-7954083-active-adapter-consumption.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `d047229` | A1 — pass; installed Discovery exposes only the current reverse route | [Legacy adoption retirement](./runs/2026-09-03-codex-d047229-legacy-adoption-retirement.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `489d306` | VC1 — pass; route precedence and active claim scope confirmed | [Completion claim confirmation](./runs/2026-09-03-codex-489d306-completion-claim-confirmation.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `3329405` | VC1 — one environment-blocked attempt; retry confirmed active scope but omitted the terminal verdict | [Claim active scope](./runs/2026-09-03-codex-3329405-claim-active-scope.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `7c3dcab` | VC1 — product failure; FT-0040 opened | [Completion route precedence](./runs/2026-09-03-codex-7c3dcab-completion-route-precedence.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `b0ecf8c` | VC1, VD1, X2, A1, DR1 — pass after one invalid VC1 harness attempt | [Routing and read projections](./runs/2026-09-03-codex-b0ecf8c-routing-and-read-projections.md) |
| 2026-09-03 | Claude Code | `claude-opus-5` / Agent-tool subagent | `4635a0b` | VD2, VC2, Q0, DR1 — pass; VC1 — product failure; one environment-invalid Q0 attempt retained | [Claim verification and orchestration batch](./runs/2026-09-03-claude-code-4635a0b-verification-batch.md) |
| 2026-09-03 | Claude Code | `claude-opus-5` / Agent-tool subagent | `9492a83` | A1, X2, VD1, VI4, RT2 — pass; FT-0030..FT-0035 opened | [Approval-free verdict batch](./runs/2026-09-03-claude-code-9492a83-verdict-batch.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `e257930` | A3 — pass; FT-0029 confirmed | [Reverse establishment completion](./runs/2026-09-03-codex-e257930-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `fe35de1` | A3 — product failure at finalization; FT-0028 confirmed | [Reverse finalization drift](./runs/2026-09-03-codex-fe35de1-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `3407b96` | A3 — product failure; temporary adoption record became a phantom Spec | [Reverse record placement](./runs/2026-09-03-codex-3407b96-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `d7b194f` | A3 — environment blocked by fixture-only dispatch instrumentation | [Reverse lifecycle instrumentation](./runs/2026-09-03-codex-d7b194f-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `8110a4a` | A3 — scenario invalid; checkout identity unresolved; FT-0027 ledger behavior confirmed | [Reverse proposal authority](./runs/2026-09-03-codex-8110a4a-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `29b247c` | A3 — product failure; direct Steering/evidence contradiction omitted | [Reverse contradiction classification](./runs/2026-09-03-codex-29b247c-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `a52853e` | A3 — scenario invalid; quantity semantics remained a blocking unknown | [Reverse establishment quantity boundary](./runs/2026-09-03-codex-a52853e-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `2086d1e` | A3 — scenario invalid; maintained behavior made the intended suspected defect blocking | [Reverse establishment ordering](./runs/2026-09-03-codex-2086d1e-reverse-establishment.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `9dadf34` | U1, U2 — pass; one environment-invalid U2 attempt retained | [Explicit update workflow confirmation](./runs/2026-09-03-codex-9dadf34-update-confirmation.md) |
| 2026-09-03 | Codex | `gpt-5.6-terra` / `medium` | `51f765c` | U1 — product failure; U2, U3 — pass | [Explicit update workflow](./runs/2026-09-03-codex-51f765c-update.md) |
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
- Fixed, behavioral confirmation pending: 6
- Active environment limitations: 5

The authoritative rows and stable identifiers are in
[the findings worklist](./findings.md). Historical resolved findings through the
migration remain compacted in the legacy ledger rather than being reconstructed.
