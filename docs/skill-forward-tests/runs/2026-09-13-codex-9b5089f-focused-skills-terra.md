# Forward-test run: 2026-09-13 / Codex / 9b5089f + Decision 0215

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-13`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, default forward-test profile
- Tested build: `9b5089f` plus the Decision 0215 working tree
- Binary SHA-256: `6200d9b91eccddb18ecad1921a317e786514a367167096c0a5f679f920b7c3b6`
- Fixture language: `en`
- Scenarios: `Q0`, `ST3`, `I1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| Q0 | pass | none for the scope boundary | `order` remained at Requirements; `cart` idle; no artifacts or gates changed | The driver named `order` and all active Specs as choices and stopped. Independent milestone/Spec reads, clean Git status, and unchanged HEAD `e318494580a85d35b85fdebd808f3d153527a682` confirmed no mutation. | none |
| ST3, first fixture | environment_blocked | Fresh canonical checks could not run, so no `VERIFIED` verdict | Implementation; one completed Task; completion not reached; clean HEAD `3f163fb0b4fa5b686be39e4840da97e169b4cf22` unchanged | Driver returned `MANUAL_VERIFY_REQUIRED` after Git Bash signal-pipe failure and automatic approval rejection. Parent status, Task and diff checks confirmed no mutation. | ENV-0004 |
| I1, instrumented fixture | environment_blocked | Fresh implementer could not write the required audit marker, so implementation/review did not run | Only CLI-owned blocked Task state changed; source/tests/HEAD unchanged | `tasks list cart`: 0 completed, 1 blocked; status remained implementation with fresh planning Gates and no completion. Exact diff was confined to `tasks.yaml`. | ENV-0004 |
| I1, ordinary fixture | environment_blocked | Final adapter-directed local checkpoint was rejected; all I1 implementation criteria held | Correct quantity bounds, four passing tests, Task 1 completed, Spec still in implementation without completion; changes uncommitted | Independent `python -m unittest -v` passed 4/4; diff proved unchanged plan semantics plus CLI execution state. Source checks, fresh Gate status, and unchanged HEAD `b2b9a66159de43bab592f84751d3a3a97b8dcdab` confirmed the exact remaining boundary. | ENV-0004 |

The first ST3 Status answer and Q0's initial commentary inherited the host's
Japanese preference. The driver context was explicitly clarified to English.
Q0 measures the observed scope boundary only; no language-discovery claim is
made. ST3 was separately rebuilt and measured with Astra; the blocked Terra
attempt is not evidence of a model capability difference.

I1's instrumentation is optional for ordinary workflow measurement. A second
fresh fixture omits it and retains the same approved plan and scenario request.
It measures implementation behavior, not instrumentation-based dispatch counts.
The fresh implementer returned `READY_FOR_REVIEW`, the independent reviewer
returned `APPROVED`, and the owning workflow recorded the Task through the CLI
before attempting its checkpoint. No separate claim-verification stage or
whole-Spec validation was added to the implementation cycle.

## Confirmation turns

Q0 stopped for scope selection; no scope or delegated Gate approval was supplied.
No approval to record Spec completion was supplied in ST3.
The parent clarified that disposable-fixture verification belongs to the
authorized product update, but the instrumented I1 fresh worker still received
an automatic approval rejection. No host control was bypassed.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| Q0 | Initial host-language carryover and repository listing before the scope question | extra-step | discarded | Driver environment/entry discovery; no phase artifact was read or changed and no scope was inferred. |
| ST3 | Host-language carryover and rejected canonical-test retry | extra-step | retained as environment only | ENV-0004; no product wording friction established. |
| I1, instrumented | Audit marker rejected before implementation | extra-step | retained as environment only | ENV-0004; diagnosis proved CLI/artifacts readable, not permission for the fresh worker's required write. |
| I1, ordinary | Local checkpoint rejected as fixture implementation outside the host request | extra-step | retained as environment only | ENV-0004; product instructions clearly required the checkpoint and no product friction was reported. |

All debriefs were requested after fixture judgment and prohibited commands
and edits. Before/after Git state remained unchanged. No new product finding
was established.

## Mechanical validation

The product passed Rustfmt, schema generation check, Clippy with warnings denied,
the full Rust test suite, and release build. The focused Skill, protocol, and
project-instruction targets passed 105 tests. Decision-index verification and
strict bilingual MkDocs build passed, with corresponding language-selector
links verified.

The generic Skill creator validator does not accept the agent-neutral source's
`argument-hint`. It passed all five changed Codex-rendered packages instead;
Python UTF-8 mode was needed for the Windows default-code-page mismatch. Product
conformance tests separately checked Claude Code and Generic rendering.

## Cleanup

Disposable fixtures are removed after recording: `target/forward-astra-q0`,
`target/forward-astra-st3`, `target/forward-astra-i1`, and
`target/forward-astra-i1-2`. The parent worktree contains only the intended
Decision 0215 changes and run records.
