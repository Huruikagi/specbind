# Forward-test run: 2026-10-21 / Codex / CLI grouped completion

[Back to the measurement dashboard](../results.md).

- Tested build: `1875a0a` + Decision 0233 implementation tree
- Driver: Codex app `gpt-6.1-sol` / `medium`, `fork_turns: none`
- Fixture language: `en`, standalone project instructions
- Release and fixture binary SHA256:
  `AC8EB37CFA7BF773A466C5A57F21C0A0C101894B8E0D693766E3A7AB29822D39`
- Validation Skill SHA256:
  `289704B99F4DE6E40D7BA84322DAB07E8C12007FF6CDD1F584F5AFB9170AD2E3`

Canonical CLI ENV-0007 remains an environment limitation from the preceding
runs. These are app measurements; no headless CLI or Claude behavioral pass is
claimed. Both rendered packages are covered by the automated Skill checks.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR11 | pass | none | Original clean HEAD `ee0bf01acca76f21f79f28f3031a06830f8b9315` retained | All seven decisions preserve all-GO, fixed-revision, no-retry and partial-write boundaries; case 1 explicitly uses batch preflight/accept/verify. Actual trace contains only installed instruction/Skill reads and language-style read. | none |
| DR16 | product_failure | Owner omitted the Steering command declaration and substituted `./scripts/test.sh` for the declared `sh scripts/test.sh` | Clean checkpoint `c08504114a4697ba3e132fc3368d7b083b185e09`; both records mechanically fresh and milestone release_pending, but semantic GO is not established by the required command | One no-history owner, batch preflight/schema/accept/verify and exact two-file checkpoint all work. Independent batch verification succeeds; the raw trace shows only the substituted command executed. | FT-0075 |

DR11 session: `01a15dad-f296-7bc3-81cd-a8000872cd6b`. The fixture was clean
before and after judgment. No owner dispatch or hypothetical mutation occurred.

DR16 driver session: `01a15dad-369a-7d21-bc29-605766a28316`; validation owner:
`01a15dad-ec7e-7b30-8859-554324fc48a5`. Actual dispatch at
`08:57:21.770Z` uses `fork_turns: none`; two instrumentation entries corroborate
the separate contexts. Both records name implementation revision
`3ada58423eeb571c24c882183d60ae47f15ad422`. Only the two `spec.yaml` files
changed, preserving earlier gates. The candidate records the exact executed
PowerShell/Git Bash wrapper and zero exit, so the defect is command selection,
not fabricated command evidence or batch persistence. Source inspection proves
Steering declares `sh scripts/test.sh`; it was never read by the validation
owner. The original fixture is not repaired to turn this measurement into a pass.

## Confirmation turns

None. DR11 is read-only; ordinary Drive supplied DR16 completion authority.

## Debrief dispositions

Both drivers reported `none`; before/after debrief status remained clean at
their measured HEADs. The observed DR16 command-selection defect remains a
finding despite the lack of self-reported friction.

## Verification and cleanup

Formatting, generated schema checks, all-target/all-feature Clippy, all 577
Rust tests and the release build passed. The initial schema-fixture extension
filter and Clippy diagnostics were fixed before this build; no behavioral run
measured those intermediate trees. Decision validation and strict bilingual
documentation build passed, including reciprocal language links.

Removed both dedicated fixtures after judgment and debrief:
`C:\Users\hurui\AppData\Local\Temp\sb-batch-dr11-20261021` and
`C:\Users\hurui\AppData\Local\Temp\sb-batch-dr16-20261021`.
Their setup log and external candidate were also removed. The failure and
session evidence remain here; the successful fresh retry is recorded in the
[final-build run](./2026-10-21-codex-1875a0a-completion-batch-final.md).
