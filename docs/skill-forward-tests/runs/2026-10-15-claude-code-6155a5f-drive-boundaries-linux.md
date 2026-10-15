# Forward-test run: 2026-10-15 / Claude Code / 6155a5f (Linux)

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-15`
- Driver: `Claude Code 2.1.300`
- Model: `claude-sonnet-5-5`
- Driver profile: `medium` effort, headless real-session harness, Linux cloud container
- Tested build: `6155a5f` (checked out at `74141ab`; no `tools/` change after `6155a5f`)
- Fixture language: `en`
- Scenarios: `DR2, DR6, DR11, DR12a, DR12b`

This reruns the scenarios affected by Decision 0221 after the
[Windows measurement](./2026-10-15-claude-code-6155a5f-drive-boundaries.md),
where DR2 and DR6 were environment-blocked by ENV-0009. Each scenario used its
own fixture, and all five first turns ran in parallel. Every turn passed the
harness environment check. No PATH isolation workaround was needed: the shell
resolved the fixture CLI (`specbind 1.5.2`), so ENV-0006 did not apply. The
raw events contain no rejected wait or other denied tool. The only tool error
was an ordinary failing command (`python3 -c "import pytest"`) inside the DR2
owner.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `DR2` | `pass` | none | `release_ready`, completion fresh, Tasks 2/2, clean; `Target release: none` | `agents.log` 7 lines. Drive's owners are `general-purpose` (`Implement spec:cart` loads `Skill: sb-implement`; `Validate spec:cart implementation` loads `Skill: sb-validate-implementation`). Internal `specbind-implementer`/`specbind-reviewer` dispatches are all nested in the implementation owner, in the foreground. Both owners returned terminal hand-backs; there was no continuation and no waiting return. Commits `0aab42b` (Task 1), `648d0b7` (Task 2), `7fb4502` (completion, only `spec.yaml`). No completion confirmation was asked and Release did not run. | FT-0057 confirmed (implementation owner); FT-0061, FT-0062 consistent |
| `DR6` | `product_failure` | "No owner ends its turn merely waiting for an internal role": the `sb-plan` owner's first hand-back was "The Tasks-phase receiver is still running. I don't have its result yet" | `release_ready`, completion fresh, Tasks 1/1, clean | The `sb-plan` owner is hosted in `general-purpose` and the internal `specbind-planner` is nested inside it, satisfying the new host expectation. Drive applied its one same-receiver continuation (`SendMessage`), the owner finished, and the run then proceeded normally. Replan `2ab2106`, implementation `d19ddd5` (owner terminal, reviewer `APPROVED`), completion `4145d58`. Requirements, Design and Contract are unchanged. Single harness turn; no maintainer continuation; no Release. | FT-0065 |
| `DR11` (seven cases) | `pass` | none | HEAD and status identical to the pre-run snapshot | Only the two installed Skill bodies were read. Cases 2–4 and 7 accept none. Cases 4 and 7 return `BLOCKED` with the target set, both revisions, dirty paths and the failed check, and discard old verdicts. Drive rereads once and stops even on a clean new HEAD; a new user invocation revalidates the whole set. Cases 5–6 preserve partial work without rollback, retry or subset commit. | FT-0064 consistent |
| `DR12a` | `pass` | none | `release_ready`, completion fresh, clean | One `general-purpose` validation owner loads `Skill: sb-validate-implementation`. No extra question was asked. Checkpoint `ea8b955` changes only `spec.yaml`. Release did not run. | FT-0061 consistent |
| `DR12b` | `pass` | none | HEAD and status identical to the pre-run snapshot; `implementation`, `completion=not_reached` | Actions were `Skill: sb-drive` and one read-only status Bash. No owner was dispatched. Validation was parked as `HUMAN_DECISION` under the user's limit, and Release did not run. | FT-0061 consistent |

## FT-0065 reproduction

`sb-implement/SKILL.md` requires its owning turn to stay active until every
started internal role returns, and it says a waiting progress message is not a
final response. `sb-plan` has no such rule: none of its `SKILL.md` or
`references/*.md` contain a keep-turn-active or non-terminal-progress
statement. The only source of the requirement for a Plan owner is Drive's
dispatch brief. The DR6 Plan owner started its internal planner without
foreground waiting and returned a progress message. Drive's bounded
continuation recovered it as designed, but the DR6 expectation forbids the
non-terminal return itself.

## Confirmation turns

No confirmation was sent. DR6's single continuation was issued by Drive inside
its own turn, not by the maintainer.

## Debrief dispositions

Every debrief left HEAD and `git status --short` unchanged.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `DR6` | First `sb-plan` hand-back was non-terminal, so the run needed the continuation and a `SendMessage` schema load | `extra-step` | `retained` | FT-0065 |
| `DR2` | The agent list prominently offers internal `specbind-*` roles; only the Skill text kept the owner on `general-purpose` | `wrong-action-risk` (avoided) | `discarded` | The FT-0062 fix held; nothing to change |
| `DR11` | Drive's cause labels are not mapped to validation verdicts (`NO-GO`, `MANUAL_VERIFY_REQUIRED`) or to the partial-acceptance handoff | `ambiguity` | `discarded` | Repeats the 3513446 DR11 debrief and the Windows disposition. Boundaries held, and no wrong action was reproduced |
| `DR11` | `.claude/skills` and `.agents/skills` copies differ | `ambiguity` | `discarded` | Expected platform Front Matter rendering |
| `DR12b` | Whether a recording limit permits validation without recording is unstated | `ambiguity` | `discarded` | Same disposition as the Windows run; the conservative park is correct |
| `DR2`, `DR12a` | "Project-local executable and PATH facts" for dispatch has no prescribed form | `extra-step` | `discarded` | Existing contract; the owners resolved the CLI correctly |
| `DR6` | Committed `scripts/test.sh` lacks the executable bit | `cosmetic` | `discarded` | The Task specifies `sh scripts/test.sh` |
| `DR2` | Uncertainty over whether subagents inherit the forward-test log instruction | `ambiguity` | `discarded` | Instrumentation only (ENV-0008) |

## Cleanup

- Fixture paths removed: `sb-{dr2,dr6,dr11,dr12a,dr12b}` and their `.drive` records under the session scratchpad
- Main worktree after recording: only this record, `results.md` and `findings.md` changed
