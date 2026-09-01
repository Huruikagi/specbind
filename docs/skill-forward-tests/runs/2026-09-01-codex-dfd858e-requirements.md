# Forward-test run: 2026-09-01 / Codex / dfd858e

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh-context subagents with no prior turns
- Tested build: `dfd858e`
- Fixture language: `ja`
- Scenarios: `R8`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R8` attempt 1 | `product_failure` | Requirements was not authored because the driver inferred an unlisted Steering selector. | `order` remained in `requirements`; no Requirements or Contract; clean worktree. | `steering list` returned only `conventions` and `structure`; the driver additionally ran `steering read roadmap --for consume`, received `STEERING_READ_INVALID`, and stopped. | `FT-0013` |
| `R8` attempt 2 | `environment_invalid` | The driver did not read the fixture-installed Requirements Skill, so the resulting artifact cannot confirm its behavior. | `order` reached `design`; Requirements gate `fresh`; active IDs `1.1`, `1.2`, `1.3`; no Contract or Design; modified `spec.yaml` and untracked `requirements.md`. | The artifact preserved the abstract cancellation window and passed structural reads, but the debrief reported that the Skill was absent from its registry and that it authored from templates and protocols instead. | `none` |
| `R8` attempt 3 | `product_failure` | Requirements was not authored because the driver again inferred an unlisted Steering selector. | `order` remained in `requirements`; no Requirements or Contract; clean worktree. | With the fixture instructions and installed skill tree explicitly applicable, `steering list` again returned only `conventions` and `structure`; the driver additionally ran `steering read roadmap --for consume`, received `STEERING_READ_INVALID`, and stopped. | `FT-0013` |

The tested build contains the contract and Skill changes for `FT-0011` and
`FT-0012`, plus a focused passing Skill contract test. Neither finding moves to
resolved because both valid R8 attempts stopped on `FT-0013` before the affected
authoring branch completed. They remain fixed with behavioral confirmation
pending.

## Confirmation turns

Only the environment-invalid fallback reached confirmation. It first authored
the Requirements without presenting the active IDs. When asked what it needed
before completion, it presented IDs `1.1`, `1.2`, and `1.3`; the maintainer
approved exactly that document and selection and told it to stop after
Requirements. It approved the gate and did not enter Design.

## Debrief dispositions

Each fixture had the same `git status --short` before and after its read-only
debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `R8` attempt 1 | The driver treated `.specbind/steering/roadmap.md` as Steering despite the closed two-item CLI listing and stopped on the invalid read. | `wrong-action-risk` | `retained` | `FT-0013` |
| `R8` attempt 2 | The Codex driver registry did not expose the fixture-installed phase Skill, and the driver authored manually from lower-level surfaces. | `wrong-action-risk` | `discarded` | Environment-invalid attempt; it did not measure the product Skill body. |
| `R8` attempt 2 | The manual fallback first tried the nonexistent protocol selector `requirements` before recovering to `requirements-review`. | `extra-step` | `discarded` | Consequence of bypassing the installed Skill, which names the exact selector. |
| `R8` attempt 3 | The driver repeated the unlisted Roadmap-as-Steering read and stopped on `STEERING_READ_INVALID`. | `wrong-action-risk` | `retained` | `FT-0013` |

## Cleanup

- Fixture paths removed: `G:\specbind\tools\specbind\target\forward-tests\r8-dfd858e`, `G:\specbind\tools\specbind\target\forward-tests\r8-dfd858e-2`, and `G:\specbind\tools\specbind\target\forward-tests\r8-dfd858e-3`
- Main worktree after recording: run record, findings worklist, and dashboard update only
