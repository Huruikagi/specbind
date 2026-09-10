# Forward-test run: 2026-09-10 / Codex / fd2b28f + reverse shared integration

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-10`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, no inherited conversation
- Tested build: `fd2b28f` + Decision 0209 working tree with reverse integration
- Binary SHA-256: `8130cfd5be943464918db955f0265dd7504a5846a7468a198172bb3008616403`
- Fixture language: `en`
- Scenarios: `SH3`

This measures a later build than SH1/SH2. The final allowance for a follow-up
Direct to name an already-used shared resource was added afterward and is
covered by its focused lifecycle regression, not by this reverse scenario.
The review instructions were subsequently clarified to name the reverse
proposal record and adoption owner explicitly, matching the route this driver
already followed; that clarification is not attributed to the tested binary.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| SH3 | pass | none | Baseline v1.0.0 finalized, shared agreement retained, cart/order idle, no active milestone, clean tree at f0563cb | status JSON NO_ACTIVE_MILESTONE; spec list exactly cart/order; check contracts: 2 Contracts, 1 dependency, no findings/cycles; archived review includes shared-contract fingerprint; final two commits change only accepted review/adoption lifecycle artifacts and integration retirement, not README/source/shared agreement | none |

## Confirmation turns

The explicit reverse-resume request authorized the existing confirmed proposal
through review and finalization. The driver presented the assessment before
acceptance without adding an ordinary delivery confirmation boundary.

## Debrief dispositions

The tree was clean before and after the no-tools debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| SH3 | Initial Steering Skill selection corrected after reading typed reverse handler | wrong-action-risk | discarded | No Steering mutation; CLI and installed adoption owner already provided the correct route |
| SH3 | Shared reverse scope required reading the temporary confirmed proposal rather than delivery scope fields | ambiguity | resolved | The checkpointed adoption record supplied the proposed shared resource; review instructions now explicitly name that existing source and the adoption continuation owner |
| SH3 | Requirements and Designs were read and declared as deep inputs | extra-step | none | The assessment relied on them to check omitted seams; declared fingerprints accurately record that reliance |

The driver used English as requested. No language-conformance claim is made
for the earlier SH1/SH2 runs.

## Cleanup

The dedicated `tools/specbind/target/forward-sh3-0209` fixture is removed after
recording. Only the intended Decision 0209 work remains in the main worktree.
