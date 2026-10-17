# Forward-test run: 2026-10-17 / Codex / initial Drive defaults

[Back to the dashboard](../results.md).

- Tested build: `4c8bbd5` plus initial Decision 0228 tree
- Driver: real CLI `gpt-5.6-terra` / `medium`, explicit compatibility comparison
- DR14 internal implementer/reviewer/debugger: same comparison model; product defaults unchanged
- Language: `en`; independent fresh fixtures for DR13 and DR14
- Fixture binary SHA-256: `eacd6aa59928b9969e373a6c621b7f8a234823d0aad8b0534bd67b62edaa6fb0`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state | Evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR13 | `fail` | Case 9 kept progress instead of applying newly resolved bounded while retaining three used cycles | All 139 tracked/unignored file hashes, HEAD and status unchanged | Three read commands loaded both entrypoints, recovery, validated config, milestone status and language style. Ten other cases conform. | FT-0069 |
| DR14 | `environment-invalid` | Initial controller narration was Japanese in the English fixture | Clean Task checkpoint `d3a217f` | Controller read true/progress and dispatched the complete owner, but inherited host-language behavior prevents a clean behavioral verdict. | Host instruction contamination |

DR13 session: `01a149a4-4073-7250-8715-3dc2127f3706`.
DR14 controller: `01a149a4-4454-70f1-8545-b1345b121e13`.
DR14 implementation owner: `01a149a4-be68-78e1-a23b-fbece01bf2ee`.

DR13 exposed conflicting interpretations of a new invocation and a continued
allowance. The correction makes a new human invocation apply its resolved
settings without changing allowance identity, counts or findings, while an
automatic receiver keeps the invoking controller's snapshot. The same scenario
prompt is used for a fresh rerun. No expected answer was added to that prompt.

## Debrief dispositions

DR13's same-session debrief made zero tool calls. The fixture remained unchanged.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| New invocation versus ongoing selection was ambiguous | ambiguity | retained | Reproduced as FT-0069, corrected and awaiting fresh measurement |
| Natural prohibition on plan edits needed mapping to no-replan | wrong-action-risk | discarded | The Skill explicitly gives clear natural-language limits precedence; the driver applied it correctly |
| Language-style read preceded a read-only answer | extra-step | discarded | Required prose policy, not a mutation or unnecessary investigation |

DR14 ultimately completed one Task checkpoint after independent rejection,
diagnosis and a corrected review; it passed five tests. This does not change
its environment-invalid classification. Its debrief made zero tool calls and
retained the clean checkpoint; its explicit-stop observation was discarded
because the user limit was correctly honored.

The fresh rerun explicitly states the standalone fixture's English language in
its project instructions to neutralize inherited host-language guidance. This
is environment hygiene and supplies no workflow or expected-answer hints.
