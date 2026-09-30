# Forward-test run: 2026-09-30 / Claude Code / 8bde0c6

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-30`
- Driver: `Claude Code` (headless top-level session through `forward-test-drive.sh`)
- Model: `claude-sonnet-5-5`
- Driver profile: `--effort medium`; `acceptEdits` with `Bash` and `Skill` allowed; project and local settings only; auto memory off
- Tested build: `8bde0c6` (harness script uncommitted working tree; no product change)
- Fixture language: `en`
- Scenarios: `R1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R1` | `pass` | `none` | `order` in `design`; `requirements=fresh`; approval committed as the fixture's third commit; clean worktree; no `contract.yaml` | Turn 1 actions: `Skill: sb-plan`, `template resolve`, draft write, `check traceability order` returned `OK TRACEABILITY_VERIFIED` with coverage inactive; the boundary `--expect` checks passed (`requirements.md` present, `requirements=not_reached`). Turn 2: `spec requirements approve order --approval-mode explicit --requirement-ids 1.1,1.2,1.3,2.1,2.2,3.1,3.2,3.3`; `spec.yaml` carries the eight IDs in `requirement_ids` and `approved_requirement_ids`; `check traceability order` now reports only `TRACEABILITY_DESIGN_COVERAGE_MISSING`; `spec status order` reports `State health: consistent` | `none` |

The session initialization record listed every installed `sb-*` Skill, a
subagent dispatch tool, the fixture as cwd, and no auto memory path, so the
environment check passed on every turn. ENV-0001 and ENV-0003 did not apply: the
registry selected `sb-plan`, and the approval sent as a user turn was accepted
without a relay refusal.

## Confirmation turns

Turn 1 stopped at the Requirements approval boundary with the draft written and
the gate unreached, and presented the document and the proposed active set.
Turn 2: "I approve the Requirements and the active Requirement ID set 1.1, 1.2,
1.3, 2.1, 2.2, 3.1, 3.2, 3.3 you just presented. Stop after Requirements." The
run approved, followed the Git adapter's checkpoint, and stopped before Design.

A deliberately false `--expect` sent afterward was refused with exit 3 and
created no turn record.

## Debrief dispositions

`git status --short` and `HEAD` were identical before and after the debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `R1` | `milestone status` offered `mode=all_spec` while the request named one Spec and phase only loosely ("the new order spec"); the driver chose single-phase mode | `ambiguity` | `retained` | Single observation; watch for recurrence in single-phase requests |
| `R1` | Requirements for ownership, double cancellation, and a missing order went beyond the Brief's two outcomes | `ambiguity` | `discarded` | R1 expects a complete contract for the responsibility rather than the Brief's delta; the additions were presented for approval |
| `R1` | The steering next-step convention had to be applied to each rejection by hand, and one criterion was corrected during self-review | `wrong-action-risk` | `discarded` | Self-corrected before the traceability check; the approved document satisfies the convention |
| `R1` | New-document template instructions (copied comments, `{{spec}}` output) are many small manual rules | `extra-step` | `discarded` | No incorrect output; single observation |
| `R1` | First-action guard and traceability output wording | `cosmetic`, `extra-step` | `discarded` | Worked as specified |

## Cleanup

- Fixture paths removed: the scratch `sb-r1` fixture and its `sb-r1.drive` turn records
- Main worktree after recording: only the harness, its documentation, and this record changed
