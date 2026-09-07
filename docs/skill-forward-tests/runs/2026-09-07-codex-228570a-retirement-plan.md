# Forward-test run: 2026-09-07 / Codex / 228570a + retirement tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-07`
- Driver: Codex, `gpt-5.6-terra`, `medium`, fresh context
- Tested build: `228570a` + Decision 0198 working tree with AST code-example
  handling and downstream review-protocol guidance
- Driver binary SHA256: `C9A2CBBAA70C5A7619147B2AAEDFD4C97D87BB6DECBC9F79BCB81367C3CD25D3`
- Fixture language: `en`
- Scenario: R3P, using a fresh verified R3 recipe

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| R3P | pass | none | cart in implementation; one pending Task; no implementation or completion | Requirements/Design/Tasks and Contract Review fresh; Design and Task coverage 2/2; live catalog 3 and retired IDs 2.1,2.2; clean worktree | none |

The driver marked the reporting group retired, retained its old criteria and
explained cessation underneath. Item-holding Requirements remained live.
Design and the sole Task plan verify absence of a reporting callable while
retaining mutation. No dummy behavioral Requirement or new lifecycle artifact
was introduced. Contract stayed unchanged; its existing unconsumed `add-item`
export warning was assessed without removing the retained mutation boundary.

The driver requested independent Design validation. A fresh validator executed
the installed workflow read-only and returned `READY`; the owning Plan driver
then continued through Design approval, Contract Review and Tasks approval.
All three gates used the existing `sb-plan` delegation. Checkpoints were
`fe731ac`, `2cd5876`, `5733fdd`, and `d6be108`. The diff from the recorded Roadmap
baseline contained only SpecBind artifacts, with no `src` changes or test code.
This is a planning measurement, not a completed implementation journey or an
instrumented dispatch-conformance claim.

## Later build verification

After the driver build was frozen, the parser's group-marker recognition was
hardened for customized heading labels containing colons. This does not change
the ordinary heading used by this fixture. Do not attribute the driver's run
to that later build. Its full 450 Rust tests passed, including a new custom-label
regression. The resulting release binary
`BF35947EDAA8914B1E59BCAF9944C5CC2A4C5FCAE4251CD34ABC2F01705FD9F4`
independently read this completed fixture: all prior gates and review remained
fresh, live Requirements were 3, retired IDs were `2.1,2.2`, and both Design and
Task coverage remained `2/2`. Those final reads made no fixture changes.

## Debrief dispositions

Before/after debrief Git reads were clean. The driver noted the phase procedure's
optional standalone second opinion versus Plan's required independent review.
The owner resolved that by following Plan; the phase author does not own the
orchestrator's review dispatch. No bypass occurred. This is recorded as usability
context, not a new retirement finding. It also reported unspecified historical
reporting API names, the existing unconsumed mutation export, and an incorrectly
quoted Task scalar. The first two are fixture context addressed in Design and
Contract Review; the scalar was rejected and corrected before Tasks approval.
No actionable new product finding was established by these debrief observations.

The driver reported in Japanese while artifacts remained English. This run
makes no language-conformance claim.

## Cleanup

Fixture: `G:/specbind/.tmp/retirement-plan-final`; removed after judgment, debrief,
and final-binary read verification.
