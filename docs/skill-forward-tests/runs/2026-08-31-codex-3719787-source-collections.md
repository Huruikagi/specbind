# Forward-test run: 2026-08-31 / Codex / 3719787

[Back to the measurement dashboard](../results.md).

- Date: `2026-08-31`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `3719787`
- Fixture language: `en`
- Scenarios: `D14`, `R7`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| D14 | pass | none | Consistent active milestone at Requirements; two new Specs; `reminders` waits for `task-management`; Roadmap and two Briefs uncommitted; no Requirements, Design, or Contract | `milestone scope --include-body` contained both exact Source Items once, their dispositions, two Specs, and the dependency; each `artifact read <spec> brief` contained only its relevant item; `git diff --exit-code -- docs/product-definition` passed; recursive file listing found only `spec.yaml` and `brief.md` below both Specs | none |
| R7 | pass | none | `task` at Design with six fresh active Requirements; Requirements and `spec.yaml` uncommitted; Design not started | `artifact read task requirements` restated creation, optional due time, incomplete listing, completion, and unknown-task rejection without a source-link dependency; `spec status task` reported `requirements=fresh`, `design=not_reached`, and consistent health; `git diff --exit-code -- docs/product-definition .specbind/specs/task/brief.md` passed | none |

## Confirmation turns

- D14 stopped on the five-field Discovery proposal. The driver approved
  Discovery only and explicitly prohibited Requirements and Design.
- R7 stopped after drafting the complete Requirements and active IDs. The
  driver explicitly approved Requirements only and prohibited Design.

## Debrief dispositions

Fixture state was read before and after both debriefs; milestone/spec status and
Git status were unchanged.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| D14 | The run tried `artifact list` without a Spec and an invented `protocol read Discovery` selector before using the exact installed commands. | extra-step | discarded | The installed Skill explicitly says `specbind spec list` and names each protocol selector; the wrong probes did not come from ambiguous product guidance and did not affect state. |
| D14 | Brief authoring is a direct managed-artifact write rather than an `artifact write` command. | cosmetic | discarded | The installed Skill explicitly assigns Brief authoring to Discovery and supplies the template/read boundary; no missing product operation was encountered. |
| D14 | The fixture's Git Adapter recommended a checkpoint while the driver prohibited commits. | cosmetic | discarded | The explicit maintainer constraint correctly took priority and is test-harness-specific. |
| R7 | Direct Requirements phase routing, exact Source Item reading, promotion, and approval were clear. | cosmetic | none | No friction or workaround was reported. |

## Cleanup

- Fixture paths removed after recording: `C:\Users\hurui\AppData\Local\Temp\specbind-ft-d14-3719787`, `C:\Users\hurui\AppData\Local\Temp\specbind-ft-r7-3719787`
- Main worktree after recording: only this run record and dashboard update were pending.
