# Forward-test run: 2026-09-30 / Claude Code / 9e2a20e

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-30`
- Driver: `Claude Code` (headless top-level session through `forward-test-drive.sh`)
- Model: `claude-sonnet-5-5`
- Driver profile: `--effort medium`; `acceptEdits` with `Bash` and `Skill` allowed; project and local settings only; auto memory off
- Tested build: `9e2a20e` (FT-0056 Spec-directory path fix)
- Fixture language: `en`
- Scenarios: `D4`, `T1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `D4` | `pass` | `none` | Milestone with one new Spec `order` in `requirements`; Discovery committed; clean worktree | `.specbind/specs/order/` holds only `spec.yaml` and `brief.md`; the Discovery commit touches only those two files and `.specbind/steering/roadmap.md`, so `cart` and `src/` are unchanged; `milestone status` lists `spec:order status=requirements`. Actions: `template resolve spec order brief` before the first and only Brief write to `.specbind/specs/order/brief.md`; no `specs/` or `order/` directory at the repository root | `FT-0056` resolved |
| `T1` | `pass` | `none` | `cart` in `implementation`; `tasks=fresh`; plan and `spec.yaml` committed; clean worktree | `tasks list cart` → `OK TASKS_LISTED`, 2 tasks; `check traceability cart` → `OK TRACEABILITY_VERIFIED`, active set 1.1–1.4; no `execution` key; `spec status cart` → `State: implementation`, `tasks=fresh`. Actions: `ls .specbind/specs/cart`, then the first and only write to `.specbind/specs/cart/tasks.yaml`; no repository-root `specs/` probe | `FT-0056` resolved |

Both runs selected their Skill from the registry and passed the harness
environment check on every turn.

## Confirmation turns

- `D4` turn 1 proposed one new `order` Spec and asked whether refunds should stay
  in `order` or split into a payment Spec. Turn 2: "Keep refunds as an
  order-side obligation inside order; no payment Spec. I approve the Discovery
  scope you just presented for order only. Stop after Discovery."
- `T1` turn 2: "I approve the task plan you just presented. Stop after Tasks."

## Debrief dispositions

`git status --short` and `HEAD` were identical before and after both debriefs.
Neither reported friction about where to write the Brief or the plan.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `T1` | A request naming "the cart change" was resolved to single-phase mode against `mode=all_spec` by inference | `ambiguity` | `retained` | Fourth occurrence across R1, DS1, T1, and this T1; every run chose correctly. Candidate for reproduction against the `sb-plan` first-action contract |
| `T1` | Whether the CLI-modified `spec.yaml` belongs in the phase checkpoint is inferred | `ambiguity` | `retained` | Third occurrence (T1, HP1, this T1); every checkpoint was correct |
| `D4` | `milestone create` writes the Roadmap under `steering/` without naming the path, and template section deletion versus `maintain` comment preservation is unclear | `ambiguity` | `retained` | Single observation; the DS1 Design template observation is similar |
| `D4` | Steering rationale placement in the Brief, open questions inside the scope proposal | `ambiguity`, `extra-step` | `discarded` | Worked as specified; single observations |
| `D4`, `T1` | Commit attribution trailer, skill base directory changing the shell cwd | `cosmetic`, `extra-step` | `discarded` | Host behavior, not product |
| `T1` | Test scaffolding placement when the project has no test command | `ambiguity` | `discarded` | The tasks-generation rule answered it and the plan followed it |

## Cleanup

- Fixture paths removed: the scratch `sb-d4` and `sb-t1b` fixtures and their `.drive` turn records
- Main worktree after recording: only this record, the dashboard, and the findings worklist changed
