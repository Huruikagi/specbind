# Forward-test run: 2026-09-06 / Codex / e8db9fd

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-06`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `d7a6356` + working tree, committed as `e8db9fd`
- Fixture language: `en`
- Scenarios: `D15` release-title subset, initial binding
- Binary SHA256: `2595B3303F829679056035C7616D681B081DB798E8DF0F9AB466986885C0A35A`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| D15 initial title binding | `product_failure` | Approved Discovery did not bind the title or finish the Brief. | Active milestone `01a073b2-1e83-7811-a34b-e3265388ea58`, `installation` in Requirements, target release `none`; untracked Roadmap and Spec state, no Brief or checkpoint. | Before approval, status was `NO_ACTIVE_MILESTONE` and Git was clean. After approval, `milestone scope --include-body` retained Issue #11 and exact `v1.0.0` provenance; status showed the unbound target. Binding returned `MILESTONE_ROADMAP_DIRTY` because the new Roadmap was untracked. | FT-0047 |

The authenticated read-only source was `Huruikagi/specbind` Milestone #1,
title `v1.0.0`, with one closed Issue #11. The proposal included that exact
target in Source coverage. This measures initial binding only, not the full
D15 multi-entry combinations, nonmatching titles, or conflicting bindings.
The failed order was also reproduced from the owning CLI: `bind_release`
requires a clean Roadmap through `ensure_target_clean`.

## Confirmation turns

The driver received: "I approve the Discovery proposal you just presented,
including its displayed release target, for Discovery only. Stop after
Discovery." No separate version question occurred. The provider required
binding before Brief/checkpoint, and the driver correctly stopped on its error.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| D15 | The prescribed create-then-bind order conflicts with the clean-target guard. | wrong-action-risk | retained | FT-0047; fix the procedure to finish the complete capture checkpoint before binding. |
| D15 | Extra Release and cart reads, and uncertainty about choosing one installation Spec. | extra-step | discarded | No separate reproduced product defect; the displayed boundary was confirmed before mutation. |

The driver reported in Japanese despite the English fixture. No checkpoint
authority conclusion is drawn from this run: it stopped before any checkpoint.
The CLI rejection and unbound persisted state independently establish the
product defect. Git status was identical before and after the read-only debrief.

## Cleanup

- Fixture path: `C:\Users\hurui\AppData\Local\Temp\sb-d15-release-d7a6356`; removed after recording.
- Main worktree: ordering fix and forward-test evidence remained for the next commit.
