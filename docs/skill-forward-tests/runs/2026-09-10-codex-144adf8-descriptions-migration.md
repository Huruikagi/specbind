# Forward-test run: 2026-09-10 / Codex / 144adf8 + description migration tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-10`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context
- Tested build: `144adf8` + Decisions 0210/0211 working tree before the final direct entrypoint link
- Binary SHA-256: `c85ba6455f04938dffccfdda16323feba985a50c3c04a9b0a304b67ed4b62ea1`
- Fixture language: `en`
- Scenarios: `MG1`

The fixture used the recorded development interval `1.4.4 -> 1.5.0` explicitly.
No release version or installation client was changed. The later direct
`sb-configure` entrypoint link was not in this fixture; the existing update
reference already linked the identical migration procedure that this run used.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| MG1 | pass | none | Clean local checkpoint `fbc7d7f`; cart remains idle with no milestone | Before confirmation, clean diff and three pending paths. After confirmation, exactly three added description lines; unchanged Requirements body, Contract, state, and settings. Repeated original-interval plan reports all three entries complete and no targets; `spec list` and `steering list` expose descriptions. | none |

## Confirmation turns

The driver presented exact Front Matter additions for cart Requirements and
conventions/structure Steering without writing. Confirmation authorized those
three descriptions and their local checkpoint only, excluding body/template edits,
lifecycle state, approvals, binary updates, and milestone creation.

## Debrief dispositions

Git status was clean before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| MG1 | Default target was the fixture's unchanged `1.4.4` version; explicit `--to 1.5.0` produced the intended plan | wrong-action-risk | discarded | Deliberate development-fixture version mismatch; real default target is the executing release as specified. The driver used the recorded explicit interval before reporting results. |
| MG1 | Driver checked for write commands before treating description as agent-authored Markdown | ambiguity | discarded | Procedure and project instructions distinguish authored artifacts from CLI-owned state; the confirmed narrow patch preserved the boundary. No reproducible contract contradiction. |
| MG1 | `artifact list` did not display Requirements descriptions; driver also used `spec list` | extra-step | discarded | Intended per-profile projection and an explicit verification step in the migration procedure. |

## Cleanup

Fixture: `G:/specbind/target/forward-mg1`. Removed after evidence capture.
