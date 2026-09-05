# Forward-test run: 2026-09-06 / Codex / d738099

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-06`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `e8db9fd` + working tree, committed as `d738099`
- Fixture language: `en`
- Scenarios: `D15` release-title subset, initial binding
- Binary SHA256: `DB6CB62162A76530F0F5365996E72C40E342DEEB2E0A678A804AB0FF7D0BC877`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| D15 initial title binding | `pass` | none | Active milestone `01a073b8-069e-75b1-ac18-38c17bef6fc3`, exact target `v1.0.0`, `installation` at Requirements, substantive Brief, clean worktree. | Before approval, `milestone status` reported `NO_ACTIVE_MILESTONE` and Git was clean. After approval, status reported target `v1.0.0` and consistent health. `artifact read installation brief --for consume` returned the captured request and Issue #11 URL. Commit `6266c9d` contains Roadmap, Spec state, and Brief; `9fda14f` changes only Roadmap `target_release: null` to `target_release: v1.0.0`. | FT-0047 resolved |

The read-only source was `Huruikagi/specbind` Milestone #1, title `v1.0.0`,
with one closed Issue #11. No publication or GitHub write was performed. This
run measures initial binding and checkpoint order only: full multi-entry D15,
nonmatching titles, explicit overrides, conflicting bindings, and no-commit
adapter branches were not behaviorally measured. Mechanical Skill tests cover
the title recognition grammar and the documented guard branches.

## Confirmation turns

The displayed Source coverage included no current binding and proposed exact
`target_release=v1.0.0` from the Milestone title. The driver received: "I approve
the Discovery proposal you just presented, including its displayed release
target, for Discovery only. Stop after Discovery." It finished both checkpoints
without a separate version or checkpoint question and stopped at Requirements.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| D15 | The driver would prefer explicit staged-path inspection; all three captured paths were in the first commit. | cosmetic | discarded | Independent commit inspection proves exactly the expected Discovery outputs; no unintended path was committed. |
| D15 | Initial progress preceded the absent language-style read; cart reads established its ownership boundary. | extra-step | discarded | No additional language rule or reproduced routing defect. |

The conversational report was Japanese despite the English fixture; persisted
Brief and commit messages were English. The exact two commits, Roadmap-only
binding diff, and clean final status independently establish the measured
checkpoint outcome. Git status was clean before and after the read-only debrief.

## Cleanup

- Removed after recording: `C:\Users\hurui\AppData\Local\Temp\sb-d15-release-final`.
- An unused setup at `C:\Users\hurui\AppData\Local\Temp\sb-d15-release-retry` was also removed; no driver ran against it.
- Main worktree: run record, dashboard, resolved finding, and a Decision prose correction remained for the evidence commit.
