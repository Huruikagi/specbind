# Forward-test run: 2026-09-13 / Codex / 9b5089f + Decision 0215

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-13`
- Driver: `Codex`
- Model: `gpt-6-astra`
- Driver profile: `medium`; intentional model comparison with the Terra run
- Tested build: `9b5089f` plus the Decision 0215 working tree
- Binary SHA-256: `6200d9b91eccddb18ecad1921a317e786514a367167096c0a5f679f920b7c3b6`
- Fixture language: `en`
- Scenarios: `ST3`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| ST3 | pass | none | Cart remained in implementation, one Task completed, completion not reached | Both turns read installed packages; the claim verdict was `VERIFIED`; `sh scripts/test.sh` passed four tests. Independent status/Task/diff reads and a canonical test rerun confirmed no writes and unchanged HEAD `552062dcd714a92a12bee022af799012a21856b5`. | none |

The first Status answer distinguished completed Tasks from unrecorded Spec
completion. The follow-up selected consequence-free verification, scoped its
claim to active Requirements 1.1–1.4, and did not demand a separate invented
runtime command. The canonical tests load the module and exercise its public
operation. Requirements 2.1–2.2 remained outside the active claim.

The Terra attempt encountered host restrictions before a full verdict; these
runs do not establish a model quality or performance difference.

## Confirmation turns

No product approval was needed. The prompt explicitly identified the authorized
disposable fixture and English response context. The driver requested the host
permission needed to execute the canonical Git Bash test.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| ST3 | One failed lookup assumed Task `1.1` before the Task list supplied `1` | extra-step | discarded | Incidental wrong operand; no ambiguous product instruction was identified. |

The post-judgment debrief ran without commands or writes; before/after Git
status remained clean. No product friction was reported.

## Cleanup

The disposable fixture `target/forward-astra-st3-2` is removed after recording.
The parent worktree contains only the intended Decision 0215 changes and run
records.
