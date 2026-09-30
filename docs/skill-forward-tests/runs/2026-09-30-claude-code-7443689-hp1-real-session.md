# Forward-test run: 2026-09-30 / Claude Code / 7443689 / HP1

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-30`
- Driver: `Claude Code` (headless top-level session through `forward-test-drive.sh`)
- Model: `claude-sonnet-5-5`
- Driver profile: `--effort medium`; `acceptEdits` with `Bash` and `Skill` allowed; project and local settings only; auto memory off
- Tested build: `7443689` (product identical to `8bde0c6`)
- Fixture language: `en`
- Scenarios: `HP1` (journey harness, dispatch-instrumented)

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `HP1` as written | `scenario_invalid` | Line 7 requires completion to be accepted after line 6, but line 6 ("Is the cart work done?") routes to consequence-free verification under Decision 0187 | Unchanged by the line 6 turn: `completion=not_reached`, `SPEC_VALIDATION_INCOMPLETE`, clean worktree, `HEAD` `32ce63a` | Turn 7 selected `sb-verify-completion`, ran the test command, returned `VERDICT: NOT_VERIFIED` for lifecycle completion, and wrote nothing | `none`; line 6 corrected in `journey-scenarios.md` |
| `HP1` with corrected line 6 | `pass` | `none` | No active milestone; `cart` idle; release log and both `v1.4.0` archives; annotated `v1.4.0` at `1d9fc1b`; final commit `4f48b1d`; no remote; clean worktree | `forward-test-journey.sh judge hp1`: all 13 checks `PASS`; tagged commit `1d9fc1bab5f85038fbdc0e609ebcad68f62021b3`; final commit `4f48b1d425a929b405ffc1f5d56ca7afae25d5e0`; dispatch contexts `11` | `FT-0056` (debrief) |

The corrected-line measurement continued the same session after the stale turn,
which was read-only and left the fixture byte-identical (checked by `--expect`
before the next message). The session therefore carried one extra consequence-free
verification turn that a fresh run of the corrected script would not have. A
fresh rerun of the corrected script is the clean measurement; this result shows
that every later boundary works once completion authority is explicit.

Dispatch was real, not the main-context fallback. The eleven log lines are the
driven context plus ten fresh contexts:

| Phase | Fresh contexts |
| --- | --- |
| Requirements | 1 (approval under delegation) |
| Design | 1 author, 2 independent validators (first `NOT_READY` on an undeclared `scripts/test.sh` owner, then `READY` after one revision); the author was resumed for approval |
| Contract Review | 1 |
| Tasks | 1 |
| Implementation | 2 implementers, 2 independent Task reviewers |

## Confirmation turns

Every scripted line was sent only after `--expect` proved its boundary: no active
milestone before Discovery approval; clean Discovery before the delegation
proposal; `requirements=not_reached` before delegated authority; `tasks=fresh`
before implementation; both Tasks complete before binding; bound target and
`SPEC_VALIDATION_INCOMPLETE` before the completion turn; `completion=fresh` and
no tag before the release request; no tag and `HEAD` at the proposed commit
before Publish.

- Line 3 presented the delegated gates (Requirements, Design after `READY`,
  Tasks) and excluded Contract Review and gate invalidation before any dispatch.
- Line 5 bound `v1.4.0`, committed only the Roadmap target, and stopped at the
  expected `RELEASE_SPEC_NOT_VALIDATED` preflight.
- Corrected line 6 selected `sb-validate-implementation`, recorded `GO`, and
  checkpointed only `spec.yaml`.
- Line 7 ran Prepare and paused at Publish with the exact tag, commit, Verify
  plan, and finalization log line.

## Debrief dispositions

`git status --short`, `HEAD`, and tags were identical before and after the
debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `HP1` | Discovery gives the Brief path as `<specDir>/<spec>/brief.md`; Spec artifacts live under `<specDir>/specs/<spec>/` | `wrong-action-risk` | `retained` | Reproduced in `sb-discovery/references/ordinary.md`; recurs with T1 and the HP1 Tasks subagent guessing `specs/cart/` → `FT-0056` |
| `HP1` | The Roadmap is stored beside Steering but is not a Steering document | `ambiguity` | `discarded` | Already resolved by FT-0013; the driver applied the closed-set rule |
| `HP1` | Resuming the Design author after validation needed a deferred-tool lookup for `SendMessage` | `extra-step` | `discarded` | Host tool loading, not product |
| `HP1` | Git adapter does not enumerate paths per checkpoint; finalization deletions had to be staged by inference | `ambiguity` | `retained` | Checkpoints were correct; watch with the T1 `spec.yaml` staging observation |
| `HP1` | Repeating the instrumentation paragraph in every brief | `extra-step` | `discarded` | Fixture instrumentation, not product |
| `HP1` | Piping the test command through `tail` hid its exit status | `extra-step` | `discarded` | Driver's own command; corrected before relying on it |
| `HP1` | Task checkpoints staged by explicit path | `cosmetic` | `discarded` | Worked as specified |

## Cleanup

- Fixture paths removed: the scratch `sb-hp1` fixture and its `sb-hp1.drive` turn records
- Main worktree after recording: only this record, the dashboard, findings, and running guide changed
