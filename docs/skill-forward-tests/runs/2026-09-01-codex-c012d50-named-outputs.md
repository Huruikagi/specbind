# Forward-test run: 2026-09-01 / Codex / c012d50

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `c012d50`
- Fixture language: `en`
- Scenarios: `R6`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R6` | `pass` | `none` | `order` in `design`; Requirements gate `fresh`; active IDs `1.1`, `1.2`, `1.3`; clean worktree; checkpoint `5a5a695` | Before approval, `check traceability order` passed with inactive coverage. After approval, `spec status order` reported `State: design`, `State health: consistent`, and `requirements=fresh`; traceability reported only the expected missing Design coverage. The live Requirements contained `fixture-day` exactly twice, no `{{作成日}}`, and no `create` instruction. The unchanged project template retained two references and exactly one `create output=作成日` declaration. `git show --name-only HEAD` listed only `requirements.md` and `spec.yaml`. | `none` |

## Confirmation turns

The first driver response authored the complete Requirements draft. On
continuation it presented active IDs `1.1`, `1.2`, and `1.3` and stopped. The
maintainer approved exactly those Requirements and IDs and instructed the driver
to stop after Requirements.

## Debrief dispositions

The fixture was clean before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `R6` | The new-Spec path says to read Contract when one exists but gives no separate existence probe; a direct read returned `ARTIFACT_SELECTOR_NOT_FOUND`. | `extra-step` | `discarded` | The same Skill immediately states that a new Spec correctly has no Contract until Design, the run continued safely, and no recurring ambiguity was established. |
| `R6` | The driver considered whether “an order they placed” required an explicit rejection for another customer and added Requirement `1.3`. | `wrong-action-risk` | `discarded` | The Brief explicitly limits cancellation to an order the customer placed, the scenario requires a complete responsibility contract, and the added boundary contradicted no fixture evidence or scenario expectation. |

## Cleanup

- Fixture paths removed: `C:\Users\hurui\AppData\Local\Temp\sb-r6-c012d50`
- Main worktree after recording: run record and dashboard update only
