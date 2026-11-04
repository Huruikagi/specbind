# Forward-test run: 2026-11-04 / Codex / harness and workflow checks

[Back to the measurement dashboard](../results.md).

- Tested product build: `d7973d7`; no product source or embedded asset changes
- Driver: Codex CLI attempt and app drivers, `gpt-6.1-sol` / `medium`
- App driver isolation: `fork_turns: none`, standalone English fixture instructions
- Fixture language: `en`
- Scenarios: B0, I4
- Release and fixture binary SHA256:
  `87C9C826D1475F6A528D2F52011D31E40851DA8DE5FA9091F0765F5C8C47947C`

The app drivers received only native working-directory/PATH facts, the
standalone-project boundary, and the scenario request. They discovered the
installed Skills. These results do not establish Claude coverage or clear
ENV-0007. The harness-only repair below does not change the tested product.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| B0 / CLI | environment_blocked | Provider rejected the configured model before product tools ran | Original clean fixture at `54bec74d3a3541b072736339f65431f9543e1c42` | Session `01a1a51c-1339-7181-98d9-708eb7d2aa00`; `model_not_found`, `turn.failed`, zero actions, no final response | ENV-0007 |
| B0 / app | pass | none for the scope/delegation boundary | Original clean fixture at `233d93ac76a81bab525fab1109fd9452825db90d`; order's four gates remain `not_reached` | Installed `sb-plan` read; milestone status identifies only `order`; response names milestone, complete scope and Requirements/Design/Tasks delegation, then stops before authoring | none |
| I4 / app | pass | none | Task 1 completed at local checkpoint `97210ee84f3f8cf1f8081b8b28d739822332d266`; only original `src/orders.py` remains dirty | CLI capture/check matches exactly source and test paths before independent review; five tests pass; checkpoint includes only source, tests and Tasks execution; dirty file bytes and index entry unchanged | none |

B0 app session: `01a1a51c-8c73-7b60-9864-1eebecb24b60`. The initial sandboxed
rule read failed with a project-root permission error; the driver retried the
same read with elevated execution and received `NO_CHANGE RULE_ABSENT` before
reading milestone status. Its first shell call also read `complete-route.md`
after the failed rule read. This is a required-input sequencing deviation, not
evidence of a clean error-stop path. B0's scope/delegation assertions hold, but
this run does not resolve FT-0054 or prove the post-confirmation Plan lifecycle.

I4 app session: `01a1a51d-e2fd-70f1-8fdd-ce6f121d4161`. The initial sandbox
milestone read similarly required an elevated retry. Actual dispatch calls
show separate implementer and reviewer contexts with `fork_turns: none`; the
app's ordinary-context fallback was used, not a registered-role selector.

The retained pre-dispatch snapshot records baseline
`ec65a056d24fbebeb9e4d4a02d64bc4b3bbb68f5` and `src/orders.py` as dirty.
The actual check output is `WORKTREE_CHECK_MATCHED`, with unchanged HEAD and
empty `unexpectedPaths`, `preexistingTouchedPaths`, and
`declaredUnchangedPaths`. The independent review returns `APPROVED` before CLI
Task completion and the narrow local checkpoint. The final Spec remains in
implementation, with validation next and completion `not_reached`.

Original and final orders SHA256:
`9ADBB1BFC44A9A4442EE6843E7CF672EF63C900FD01AD770EC1C56A0DED0CCC7`.
Original snapshot and final index both record blob
`65fac3a95a9583f67aece322226140fdc71291b5`, mode `100644`.
Independent `python -m unittest discover -s tests -v` passes all five tests.

## Confirmation turns

No confirmation was supplied. B0 ends at its required confirmation boundary.
I4's request and fixture Git policy authorize only the implementation cycle and
narrow local checkpoint; no Spec completion, release, or push occurs.

## Debrief dispositions

Both drivers received the read-only debrief only after fixture judgment.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| B0, I4 | The reported shell exit code was zero despite an earlier CLI error | wrong-action-risk | discarded as a CLI defect; documented as a measurement pitfall | Actual shell calls followed the failed native read with successful `Get-Content`; wrapper status is not the CLI status. Keep dependent reads separate and inspect each result. |
| B0 | Read complete-route after the failed language-style read in the same shell call | extra-step | recorded execution deviation | The installed stop instruction is explicit. This run does not measure that error-stop boundary. |
| I4 | Displayed capture JSON was truncated by the tool output limit | extra-step | none | Before dispatch, the driver captured the full JSON directly to an external persistent file; this matches the documented supported path. No replacement snapshot was taken after implementation. |
| I4 | Read recovery guidance before any failure | extra-step | discarded | Unnecessary driver read; no recovery, mutation, authority expansion or product ambiguity followed. |

After the debrief, B0 retains its original HEAD and clean worktree; I4 retains
its checkpoint and only the original dirty orders path with the same hash.

## Harness repair and verification

Saved-event interpretation moves from an embedded shell heredoc into
`forward_test_report.py`. The old reporter returned success for a failed Codex
turn with a reply, a truncated stream, an empty reply, a Claude error result,
and reported Claude permission denials. Focused offline cases reproduced these
failures before the repair. The reporter now distinguishes agent failure (4)
from incomplete or invalid evidence (5), surfaces provider errors, and retains
recoverable command errors/metadata warnings without misclassifying them as
failed turns. Success remains permission to judge the fixture, never a pass.

All 18 offline harness tests pass, including start/resume isolation, rejected
confirmation boundaries and shell exit-code propagation for four failure modes.
Replaying the actual failed CLI event stream now reports `model_not_found`
and exits 4. No second provider request was needed for that diagnostic check.

Repository verification also passes: Rust formatting, generated-schema check,
Clippy with warnings denied, all 616 Rust tests, and the release build. The
shell syntax check and strict bilingual MkDocs build pass. No installed
template or product Skill was changed.

## Cleanup

After final state and debrief checks, removed the three disposable fixture roots
`G:/specbind-forward-b0-20261104`, `G:/specbind-forward-b0-app-20261104`, and
`G:/specbind-forward-i4-app-20261104`, the first fixture's `.drive` directory,
and the external `specbind-i4-snapshot.json`. The original raw agent sessions
remain available by the session IDs above; this record retains the measured
state and relevant evidence.
