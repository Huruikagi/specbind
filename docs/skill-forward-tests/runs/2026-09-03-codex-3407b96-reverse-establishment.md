# Forward-test run: 2026-09-03 / Codex / 3407b96

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `3407b96`
- Fixture language: `en`
- Scenarios: `A3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A3` | `product_failure` | The driver inserted an unconfigured `specs/` segment when writing the temporary adoption record. The CLI discovered that record as a phantom `adoption` Spec, so the Contract graph blocked before the downstream `order` Design could complete. | The reverse milestone remained active. `cart` reached `adoption_ready`; `order` had approved Requirements and uncommitted Design and Contract files. Source and README remained unchanged from the fixed revision. | The first post-baseline checkpoint `8125471` contains `.specbind/specs/adoption/reverse-discovery.yaml` with the Roadmap, two Specs, Briefs, Research, and deferred finding. `specbind spec list` reported three Specs including unreadable `adoption`; Contract checking reported `CONTRACT_GRAPH_CONTRACT_UNAVAILABLE specs/adoption#contract`. | `FT-0028` |

## Confirmation turns

The driver presented one complete two-Spec proposal with `order` depending on
`cart`, no blocking unknown, and one pending README suspected defect. The
maintainer confirmed that exact proposal once. The driver then created and
checkpointed the reverse baseline and progressed Requirements and the first
Design wave without another routine confirmation.

## Debrief dispositions

`git status --short` showed only the already present untracked `order` Design
and Contract before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A3` | The driver treated `specDir` as a conceptual Specs root and inserted `specs/` despite the configured value `.specbind`. | `wrong-action-risk` | `fix` | `FT-0028` |
| `A3` | The driver asked for no second scope or phase confirmation after the exact proposal was confirmed. | `extra-step` | `discarded` | Expected continuous-orchestration behavior. |

## Cleanup

- Fixture paths removed: pending until the fixed-build retry is complete
- Main worktree after recording: FT-0028 fix and this run record only
