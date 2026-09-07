# Forward-test run: 2026-09-07 / Codex / 228570a + initial retirement tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-07`
- Driver: Codex, `gpt-5.6-terra`, `medium`, fresh context
- Tested build: `228570a` + initial Decision 0198 working tree, before the final
  AST code-example hardening and downstream review-protocol additions
- Binary SHA256: `4DAF84702B6885E6EA3E792A899C5CD319070D2BEF2B273F32185251A4778BCA`
- Fixture language: `en`
- Scenario: R3, prepared through `forward-test-scenario.sh r3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| R3 | pass | none | Requirements approved, cart in design; no downstream work | Explicit scope `2.1,2.2`, fresh Requirements gate; checkpoint `e6fdcf8` changed only Requirements and spec.yaml; clean worktree | none |

The Requirements draft retained all three item-holding criteria and marked the
reporting group and both reporting criteria retired. Adjacent prose stated
cessation with no successor. After approval, `check traceability cart` correctly
reported missing Design coverage for `2.1` and `2.2`: retirement was not silently
treated as completion. No implementation or completion evidence was created.

## Confirmation turns

The driver presented the draft and exact selection without approving. It then
received explicit approval of that Requirements draft and `2.1,2.2` only, with
instructions to stop before Design, Tasks and implementation. The checkpoint
followed the fixture Git adapter, not host repository policy.

## Debrief dispositions

Before/after debrief Git reads were clean. The driver reported initially
misreading requirements state as a new Spec and omitting heading labels, then
recovering before approval after the diff and parser diagnostic. These are
debrief-only observations, not evidence of correct pre-draft ledger use. The
owning procedure already explicitly branches on artifact existence and requires
the maintain projection for an existing Spec. No new product finding was
promoted from this self-report. The R3 verdict covers the measured final draft,
approval boundary and reserved identities, not general authoring discipline.

The driver reported in Japanese while artifacts remained English. This run
makes no language-conformance claim.

## Cleanup

Fixture: `G:/specbind/.tmp/retirement-r3`; removed after judgment and debrief.
