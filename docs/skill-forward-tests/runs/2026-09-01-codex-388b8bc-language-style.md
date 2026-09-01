# Forward-test run: 2026-09-01 / Codex / 388b8bc

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh-context subagent with no prior turns
- Tested build: `388b8bc`
- Fixture language: `ja`
- Scenarios: `R8`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R8` | `pass` | `none` | `order` in `design`; Requirements gate `fresh`; active IDs `1.1`, `1.2`; uncommitted `requirements.md` and approval state | The Japanese installation contained `settings/rules/language-style.md`, and the installed Requirements Skill read it before authoring. The live artifact used `type: SpecBind Requirements`, canonical IDs `1.1` and `1.2`, and natural Japanese for the accepted cancellation and rejected late attempt. It contained none of `authoring context`, `live artifact`, or `scaffold`. After explicit approval, `spec status order` reported `State: design`, `State health: consistent`, and `requirements=fresh`; traceability reported only the expected missing Design coverage. | `FT-0011`, `FT-0012` |

## Confirmation turns

The first driver stopped because the Brief did not quantify when the
cancellation window closes. The maintainer clarified that the Brief's abstract
before/after boundary was the intended contract. The driver then authored the
Requirements, proposed active IDs `1.1` and `1.2`, and stopped for approval.
The maintainer approved exactly that document and selection and instructed the
driver to stop after Requirements.

The host safety layer rejected the driver's narrow approval command twice.
The driving session ran that exact explicit approval against the same fixed
fixture; the CLI accepted it and no Design work ran. The first `/tmp` fixture
attempt was discarded before product mutation because the driver could not
write outside the workspace sandbox; the recorded measurement uses the fresh
workspace-local ignored fixture.

## Debrief dispositions

The fixture had the same modified `spec.yaml` and untracked `requirements.md`
before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `R8` | The driver treated the Brief's intentionally abstract before/after cancellation boundary as underspecified and stopped twice across fresh drivers. | `wrong-action-risk` | `retained` | `FT-0011` |
| `R8` | The new-Spec path still attempted a Contract read and received `ARTIFACT_SELECTOR_NOT_FOUND`; R6 reported the same extra step. | `extra-step` | `retained` | `FT-0012` |
| `R8` | The supplied Git Bash `/g/...` PATH was interpreted by the driver shell as WSL `/mnt/g/...`, so it used the fixture-relative executable. | `extra-step` | `discarded` | Harness path mismatch; the fixed fixture binary was still used. |
| `R8` | The host safety layer rejected the relayed explicit approval twice. | `ambiguity` | `discarded` | Environment limitation; the driving session executed the exact approved command and measured the resulting product state. |

## Cleanup

- Fixture paths removed:
  `C:\Users\hurui\AppData\Local\Temp\specbind-r8-388b8bc` and
  `G:\specbind\tools\specbind\target\forward-tests\r8-388b8bc`
- Main worktree after recording: run record and dashboard update only
