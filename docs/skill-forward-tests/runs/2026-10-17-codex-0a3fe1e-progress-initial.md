# Forward-test run: 2026-10-17 / Codex / initial progress mode

[Back to the dashboard](../results.md).

- Tested build: `0a3fe1e` plus initial Decision 0227 working tree
- Drivers: app `gpt-6.1-sol` / `medium`; real CLI compatibility comparison
  `gpt-5.6-terra` / `medium`, with fixture-only reviewer/debugger overrides
- Fixture language: `en`; scenarios: I12, I13
- Binary SHA-256: `196fc040fcb8d81b1034f762b15060cd0acff6b1dcf97c21a55d30447b906db4`
- Recovery reference SHA-256: `bff79c7a66b0e1296b32aa49e7cffaa446afb705abfeb27a5a529b9882c82f3d`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I12 app | `environment_invalid` | English fixture produced Japanese narration despite the standalone-project instruction | Original pending diff; unchanged HEAD, status and every product-file hash | Installed Skills were read and decisions matched, but host language contamination prevents crediting this attempt | Host instruction contamination |
| I12 CLI comparison | `pass` | none | Original pending diff; byte-identical product files and HEAD | Six read commands included complete recovery/spec-backed references; all 12 decisions matched; no workers, tests or mutation | none |
| I13 CLI comparison | `product_failure` | A started corrective scope review did not advance the reported used count from three to four | Task completed, clean checkpoint `2549b62`, Spec in implementation | Actual reviewer mapped R1–R4 RESOLVED and APPROVED, canonical four tests passed, only source/tests/Task progress committed; final owner nevertheless reported three used and no cycle reservation was reported | FT-0068 |

I13's fixture status showed fresh Requirements/Design/Tasks and Contract Review,
completion not reached. Independent test rerun passed. The single checkpoint
contained `src/cart.py`, `scripts/test.sh`, `tests/__init__.py`,
`tests/test_cart.py`, and `tasks.yaml`; Task plan meaning was unchanged.
Its correct completion does not erase the accounting failure.

CLI sessions: I12 `01a1498c-fcfa-7eb3-9f01-becf9e4ea187`;
I13 `01a1498b-bb19-7460-af2c-fff98a810d5c`;
actual I13 reviewer `01a1498c-5d65-7671-8135-bac108177ebf`.
Both installed agent packages had the same recovery hash. No default-model CLI
or Claude behavioral coverage is claimed: those environments were blocked in
the preceding same-day resumption batch, so this comparison was explicit.

## Confirmation turns

No new resumption or allowance was granted in I13. The exact scenario request
carried the earlier progress selection, U1 and three used cycles. The request
was not amended to teach cycle reservation. Checkpoint escalation was permitted
by the host after an index-lock permission error.

## Debrief dispositions

Debriefs were read-only after judgment; CLI debriefs used zero tools. Before/after
HEAD, status and product hashes stayed unchanged (I13 retained its clean commit).

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| I12 app/CLI | New Milestone plus the same spent identity requires separating mode default from allowance renewal | ambiguity | discarded | Scenario combines independent facts; both drivers retained historical usage and selected bounded, as required. No reset was authorized |
| I12 CLI | Combined read truncated, then bounded rereads recovered references | extra-step | discarded | Tool output volume, not a product defect |
| I12 CLI | “Continue” can mean resumption after a reported stop, but not progress selection | wrong-action-risk | discarded | Distinct explicit preconditions in the contract; case 1 correctly stayed bounded. Public guide wording clarified same-run mode retention |
| I12 CLI | Language rule read returned RULE_ABSENT | extra-step | discarded | Required project-policy lookup behaved as designed |
| I13 CLI | Git index-lock write needed escalation | extra-step | discarded | Host permission boundary; exact scoped retry succeeded |

FT-0068 is measured behavior, independent of the debrief. The correction makes
reservation explicit in the review route and requires the outgoing count to
include the started reviewer even when source is unchanged. A fresh fixture
must confirm it; the failed attempt is retained here.

## Cleanup

Dedicated `target/forward-progress-i12`, `forward-progress-i12-cli` and
`forward-progress-i13c` fixtures and their sibling harness/baseline records are
removed after evidence recording. Main changes are the requested mode and tests.
