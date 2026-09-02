# Forward-test run: 2026-09-03 / Codex / 8110a4a

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `8110a4a`
- Fixture language: `en`
- Scenarios: `A3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A3` | `scenario_invalid` | The fixture did not decide whether README's “checkout” and `orders.place` were one maintained operation, and it expected the product-name mismatch to be a suspected defect without giving that disposition authority. | No active milestone, 0 Specs, no deferred destination, clean worktree at `a9c0c89192f4534a7c9ca9e8fa1c781bbd11ea9b`. | `milestone status` returned `NO_ACTIVE_MILESTONE`; `spec list` found 0; `.specbind/deferred.md` was absent; `git status --short` was empty. Instrumentation recorded the driver and two readers. The proposal visibly excluded the `Bookshp` mismatch as historical detail, confirming FT-0027's closed-ledger fix. | `none` |

## Confirmation turns

The driver stopped on the checkout-versus-place semantic question before an
approvable proposal. No confirmation was given.

## Debrief dispositions

`git status --short` was empty before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A3` | The driver overread legacy adoption references but did not use them for mutation. | `extra-step` | `discarded` | Existing routing explicitly selects the current reverse procedure. |
| `A3` | Checkout wording could denote `orders.place` or an absent separate entry point. | `wrong-action-risk` | `discarded` | Scenario setup issue; the next fixture makes the product identity explicit. |

## Cleanup

- Fixture paths removed: pending until the fresh retry is complete
- Main worktree after recording: A3 authority repair and this run record only
