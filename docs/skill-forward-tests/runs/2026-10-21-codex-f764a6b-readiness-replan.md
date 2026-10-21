# Forward-test run: 2026-10-21 / Codex / Readiness and replan checks

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-21`
- Driver: Codex app, `gpt-6.1-sol` / `medium`
- Driver profile: separate fresh subagents with `fork_turns: none`, standalone
  fixture instructions and English output
- Tested build: `f764a6b` + Decisions 0231/0232 working tree
- Release and both fixture binaries SHA256:
  `64869DE0A868D40F5F9BBE741F3EB37B1AC7D374202C9DF83ABDC96E0EC81266`
- Scenarios: T3 and DR9w (withdrawal-only portion of DR9)
- Fixture language: `en`

The canonical CLI model was already blocked by ENV-0007 in the same session's
[preceding attempt](./2026-10-21-codex-35a9afd-task-revision.md); it was not
retried. These are app-driver measurements, not CLI or Claude Code passes.
DR9w used dispatch instrumentation. Production source and embedded assets did
not change after this binary was built; final validation also covers added
regression tests and documentation.

Installed Codex and Claude references were byte-identical: Drive's replan
reference SHA256 was
`C778C97B8CBB7E1FB6AAEFFDF51F5C82AF45E98B043F636906AE58F83CA6EDE1`,
and the validation withdrawal reference was
`CC62202E21A0DB86128739144141DC34CF139C6748CA8FE198C146D7DE8065C7`.
This verifies distribution alignment, not Claude behavior.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| T3 | pass | none | Four Tasks; 2 and 3 completed, 1 actionable, 4 waiting; fresh Tasks gate, clean checkpoint `8a08b02` | Actual revision preflight, schema read, dry-run and apply preserve old completed 1/2 as new 2/3. Independent status and Tasks reads show 4 waits on effective prerequisites 1, 2 and 3. Only cart `spec.yaml` and `tasks.yaml` changed; traceability 4/4; no implementation. | FT-0074 resolved |
| DR9w | product_failure | Validation owner inherited the Drive conversation | Cart completion withdrawn in `0b4b91b`; no pending implementation or final validation; clean worktree | Driver and owner independently ran the new fixed-revision check before/after withdrawal. Final output proves cart's checkpoint, no remaining targets and fresh earlier gates/review. Actual owner dispatch used `fork_turns: all`. | Existing FT-0073 reproduced; CLI and narrow mutation checks succeeded |

## Sessions and independent evidence

- T3 driver: `01a15d65-f207-7622-89da-5887af84a4c1`.
- DR9w driver: `01a15d66-19e0-7d03-9e64-f493a8ae9752`.
- DR9w validation owner: `01a15d67-3184-7f01-b25b-7b40d4ab5846`.
- Session logs are under the host's `.codex/sessions/2026/10/21/` directory.

For T3, the initial scenario request did not name cart. The driver first asked
for that scope, then presented four Tasks and the exact progress mapping. The
confirmation approved that presented Tasks-only rewind/mapping and stopped
after Tasks. The driver applied and approved the presented plan, then committed
`8a08b02` (`Plan cart cap extraction before existing tasks`). Independent final
reads confirmed the two retained completion records and Task 4's waiting state.
Separate Rust regressions cover a blocked predecessor and grouped ordering.

DR9w retained baseline `afb6d12949ca80d5681b3016400389fbeea02c06`
and terminal recovery revision `5a4ac4c4e29263ea7b46202e36ff781466e85cf3`.
The initial check rejected aliases `before-replan` and `HEAD`; after resolving
full IDs, the check succeeded with temporary-directory write permission.
Both the driver and validation owner used those same fixed revisions.

Independent final `milestone replan check` returned `eligible: [cart]`,
`remaining: []`, and cart checkpoint
`0b4b91bb2e67a6a0a6fb3fdc6e31144552eaa2ee`. It retained
`requiresAuthorityAndDeltaReview: true`; the authorized recovery paths were
cart-input's `spec.yaml` and `tasks.yaml`. The recovery-to-final diff changed
only cart's `spec.yaml`: `release_ready` to `implementation` and removal of
completion evidence. Milestone health was consistent, Contract Review fresh,
cart still had its completed Task, and cart-input's Task remained pending.

The instrumented owner log records the withdrawal-only task, and raw dispatch
arguments independently show `fork_turns: all`. Successful CLI evidence and
artifact convergence do not establish the required fresh-owner isolation.
This run therefore remains a product failure, not a complete DR9w pass or a
full DR9 implementation/final-acceptance measurement.

## Debrief dispositions

Both fixtures were clean before and after their post-judgment read-only
debriefs. No debrief tool actions or fixture repairs were requested.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| T3 | Asked for the named Spec despite only cart being listed | extra-step | discarded | Initial request omitted the name; intended explicit scope boundary, as in the preceding T3 run. |
| T3 | Had to interpret whether an exported cap used by a later Task was unused foundation work | ambiguity | discarded | Driver gave Task 1 an independently tested value and made Task 4 consume it; no violated expectation or reproduced product defect. |
| T3 | Temporary candidate disappeared between shell invocations | extra-step | discarded | Execution-environment persistence issue; identical candidate was recreated and dry-run/apply executed together. No manual progress write or altered mapping. |
| DR9w | Full commit IDs required an extra invocation | extra-step | discarded | Deliberate fixed-identity contract; diagnostic requested the captured full commit ID. |
| DR9w | Temporary Git checkout was denied by the sandbox | extra-step | discarded | Identical read-only project check succeeded with escalated temporary-directory access; no change to product state or eligibility. |
| DR9w | Owner inherited parent history | wrong-action-risk | retained | FT-0073 recurrence independently established by actual dispatch arguments; not inferred from the driver's report. |

## Cleanup

- Fixtures: `C:\Users\hurui\AppData\Local\Temp\sb-order-t3-20261021`
  and `C:\Users\hurui\AppData\Local\Temp\sb-replan-dr9w-20261021`.
  Removed after recording judgment and debrief.
- Main worktree retained only the intended readiness/replan implementation,
  tests, paired documentation, product assets and measurement records.
