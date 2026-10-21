# Forward-test run: 2026-10-21 / Codex / Drive owner isolation

[Back to the measurement dashboard](../results.md).

- Tested build: `2036dec` + FT-0073 clarification working tree
- Driver: Codex app, `gpt-6.1-sol` / `medium`, fresh scenario drivers with
  `fork_turns: none`
- Fixture language: `en`, standalone project instructions, dispatch instrumentation
- Release and fixture binary SHA256:
  `5563A4E1D7E17D361EDF6E5A81BF8F18052E84FDF022B664FB4350281A5CA0DA`

The same-session canonical CLI attempt was already blocked by ENV-0007 in the
[Tasks revision run](./2026-10-21-codex-35a9afd-task-revision.md), so these fresh
fixtures used app drivers. No CLI or Claude behavioral pass is claimed.
Both installed agent packages contain the explicit isolation, receiver-lifetime
and terminal-WAITING rules. The tests inspect actual dispatch arguments, not
just instrumentation names or the driver's claim to have used fresh owners.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR6 | pass | none | Clean completion checkpoint `841b11b`; cart release_ready, 2/2 Tasks complete, all gates and Contract Review fresh; release version unbound | Actual Plan, implementation and validation dispatches have distinct identities and `fork_turns: none`; no Drive follow-up reuses a completed owner. Independent canonical runner passes all 6 tests. | FT-0073's Plan-to-implementation and validation branches confirmed |
| DR9w | product_failure | Drive executed withdrawal itself instead of dispatching an isolated owner | Clean narrow checkpoint `9b35cab`; cart completion removed, cart-input still pending | Raw driver calls load Drive and validation, then directly invalidate and commit; no spawn call, one instrumentation entry. Independent fixed-revision check and final diff otherwise satisfy the withdrawal/stop contract. | FT-0073 remains unresolved on this build's withdrawal route |

## DR6 evidence

Driver session: `01a15d80-9254-7a83-9572-dda50cb4fa2b`.
The three actual `spawn_agent` calls use no inherited turns:

| Workflow | Dispatch UTC | Owner session |
| --- | --- | --- |
| Plan | 08:08:52.225 | `01a15d81-8712-72c3-9d4f-58a00688d2d8` |
| Implementation | 08:13:39.553 | `01a15d85-e972-7330-9eed-66f680a594cf` |
| Validation | 08:21:05.371 | `01a15d8c-b6ec-70e1-b177-4e5591669f57` |

All are ordinary complete Skill owners. Plan's internal Tasks receiver is
`01a15d82-a3b3-71c2-beef-13d4c03673b7`; Plan collected its result and returned
the completed phase without Drive's same-owner continuation. This also exercises
FT-0065's waiting boundary in Codex; it is not a new Claude confirmation.

Baseline `821af1f37d30cb96e6ff3287641eeaf511c8308f` was clean.
Plan's checkpoint `898a849bca496f7ce181123f3f9ed107d27983cd` changes only
cart's Tasks and its gate. Old pending Task 2 became new Task 1, establishing
the runner and executable smoke tests; old blocked Task 1 became new Task 2,
retaining the exact blocked record. Requirements, Design, Contract and
accepted Contract Review stayed unchanged. There was no user confirmation
between the original authorized request and the completed workflow.

Implementation checkpoint `9c0b2ff30a775ab93456eb07205fe5e911b6e923` adds the
runner and tests. An independent reviewer approved that work and separately
confirmed the mapped block's cause was resolved. The owner then reopened
Task 2 through the CLI. Checkpoint `1b50d761f79899f4b682e06224978defe48c01e7`
adds quantity bounds and regression coverage after another independent review.
Unknown historical recovery counts were kept unavailable, not invented or reset.

Validation checkpoint `841b11b0b966ed5f6c762deeec20fa77a5c8b2d5` accepts
completion at the implementation revision. Independent final status reports
consistent state, every gate fresh, 2/2 Tasks completed, coverage 4/4 and only
`RELEASE_VERSION_UNBOUND`. The final baseline-to-HEAD changed paths are cart's
`spec.yaml`, `tasks.yaml`, `src/cart.py`, `scripts/test.sh` and
`tests/test_cart.py`. The canonical runner independently passes six tests,
including 1/99 boundaries, accumulation, and rejected operations leaving both
new/existing SKU carts unchanged. No Release execution occurred.

One internal implementer startup reported model capacity before any work.
The implementation owner used its bounded retry and subsequently completed
the ordinary implementation/review path. This is environment friction, not a
Drive receiver-reuse exception or a new product finding.

## Debrief dispositions

Debriefs were requested only after independent judgment, with tools and writes
prohibited. Git status was clean before and after the DR6 debrief.
DR9w also remained clean through its read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| DR6 | Replan snapshot needed escalated temporary-directory access | extra-step | discarded | The identical command succeeded; known sandbox constraint, with no product-state cleanup or eligibility bypass. |
| DR6 | Preserved blocker reason still used old Task numbers after mapping | ambiguity / wrong-action-risk | discarded as a defect in this run | Historical evidence is intentionally preserved. The exact mapping was carried to the new owner and independently checked before CLI reopening; silently rewriting the record would lose that history. |
| DR9w | Driver treated itself as withdrawal owner after reading both Skills | wrong-action-risk | retained under FT-0073 | Raw calls independently prove no owner dispatch. The follow-up correction explicitly preserves the controller role for a withdrawal-only Drive request and adds the same boundary to the withdrawal procedure. |
| DR9w | Passed a tag instead of its full revision | extra-step | discarded | Intended immutable revision contract; resolved ID succeeds without moving either checkpoint. |
| DR9w | Temporary Git snapshot required escalation | extra-step | discarded | Same known sandbox requirement as DR6; identical check succeeded without project-state mutation. |

DR9w driver session: `01a15d8f-033e-7380-95f7-e020754ce3ea`.
Baseline: `adf8690f3cda51f5ea3c22e114da70e05161601e`; recovery:
`5ee7fdcbf856a6841c1647d1edd079fac6e6a146`; withdrawal:
`9b35cab6b4231dc9202604dbc28af8e1607c9dd6`. Independent check returns
`eligible: [cart]`, `withdrawn: [cart]`, `remaining: []` with original fixed
revisions. Only cart's `spec.yaml` changes, from release_ready to implementation
with completion removed; earlier gates/review stay fresh. These successful
mechanical checks do not turn the failed ownership measurement into a pass.

## Verification and cleanup

Rust formatting, generated-schema check, all-target/all-feature Clippy, all 569
tests and the release build passed. The final wording also passed all 95 Skill
conformance tests in both rendered agent packages. No runtime or schema model
change was needed; Decision 0221 clarifies the existing fresh-owner boundary.

The [final withdrawal run](./2026-10-21-codex-2036dec-owner-isolation-final.md)
records the subsequent correction, fresh measurement and cleanup of all three
dedicated fixtures. This failed DR9w measurement is retained unchanged.
