# Forward-test run: 2026-09-07 / Codex / c7d8c27 replan initial

[Back to the measurement dashboard](../results.md).

- Driver: Codex, `gpt-5.6-terra`, `medium`, fresh context per scenario
- Tested build: `c7d8c27` + initial Decision 0199 working tree
- Binary SHA256: `B7063E4813B0D42AB1C6BA526E329B635E5AFB46437F760509F68A747842CAFF`
- Fixture language: `en`
- Scenarios: DR3, DR4

The later resolved-blocker/reopen clarification is not part of this build.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR3 | `environment_blocked` | Automatic approval review rejected final completion acceptance without a separate explicit GO-recording authorization. No replan confirmation was requested. | Design/Contract repaired, Design and Tasks delegated to `sb-drive`, renewed review fresh, Task completed, stage `validation`, clean worktree. | `spec status cart`: all three planning gates fresh, 4/4 coverage, one completed Task. `python -m unittest discover -s tests`: 4 passed. Git diff from `e941745` showed unchanged Requirements and Roadmap scope. Planning commits `8f01e32`, `5816736`, `4c1f450`; implementation `7061dd0`. | ENV-0004 |
| DR3 explicit completion continuation | `pass` | none within the separately authorized completion step | Spec `release_ready`, Milestone `release_pending`, only `RELEASE_VERSION_UNBOUND` remains, clean worktree. | Fresh test run: 4 passed. `a9e61c4` changes only `spec.yaml`; completion is fresh. No release binding, publication, or finalization. | none |
| DR4 initial attempt | `environment_blocked` | A fresh owning-workflow dispatch could not start because all four agent slots were occupied. | Initial fixture unchanged at `ced0a03`. | `git status --short` empty; agent dispatch returned thread limit reached. | none |

## Confirmation and interpretation

DR3 used the scenario's natural-language replan delegation. Recovery proceeded
through independent Design validation, renewed review, Tasks approval,
implementation, review, and final validation without another planning approval
or restart request. The completion rejection was a host approval boundary;
the fixture maintainer then explicitly approved cart completion on fresh GO
and its narrow metadata checkpoint. This continuation does not replace the
initial environment-blocked measurement.

Some nested reports and planning commit messages were Japanese despite the
English fixture. Checkpoints were therefore rechecked against the fixture's
active Git adapter and exact commit paths rather than inferred from reports.
The fresh Design validator also hit the four-agent limit when attempting an
additional code investigator and read the two source files itself. This run
does not claim that extra investigation dispatch succeeded. The independent
Design validator itself ran and returned READY before Design approval.

The first independent judge attempted the common harness command
`sh scripts/test.sh`, which this Design did not select. The fixture's approved
Design and Tasks name `python -m unittest discover -s tests`; that exact command
was then rerun successfully. The mistaken judge command is not a product defect.

## Debrief dispositions

Debriefs were read-only after judgment; Git status remained clean afterwards.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| DR3 | Status still suggested implementation despite the semantic Design mismatch. | wrong-action-risk | discarded | The new explicit semantic recovery exception was found and followed; status intentionally does not judge semantics. |
| DR3 | Automatic review required explicit completion-recording authority. | ambiguity | retained as environment limitation | Existing ENV-0004; the exact approved continuation succeeded. |
| DR4 | Agent capacity prevented fresh dispatch. | extra-step | discarded | Run scenarios sequentially; no product behavior was measured past that boundary. |
| DR4 | Human-readable status was read before the Skill-required JSON status. | extra-step | discarded | One extra read without mutation; not evidence of a missing product contract. |

## Cleanup

Fixtures: `C:\Users\hurui\AppData\Local\Temp\sb-dr3-replan-0199` and
`C:\Users\hurui\AppData\Local\Temp\sb-dr4-replan-0199`.
Cleanup is recorded with the final replan run after all judgments finish.
