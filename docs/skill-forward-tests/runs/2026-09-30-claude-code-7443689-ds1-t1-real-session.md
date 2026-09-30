# Forward-test run: 2026-09-30 / Claude Code / 7443689

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-30`
- Driver: `Claude Code` (headless top-level session through `forward-test-drive.sh`)
- Model: `claude-sonnet-5-5`
- Driver profile: `--effort medium`; `acceptEdits` with `Bash` and `Skill` allowed; project and local settings only; auto memory off
- Tested build: `7443689` (product identical to `8bde0c6`; adds only the harness)
- Fixture language: `en`
- Scenarios: `DS1` (instrumented), `T1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `DS1` | `pass` | `none` | `order` in `tasks`; `design=fresh`; Design, Contract, and `spec.yaml` committed; clean worktree | `.specbind/specs/order/` holds `technical-design/main.md`, `contract.yaml`, no `design.md`, no `tasks.yaml`; `check traceability order` → `OK TRACEABILITY_VERIFIED`, Design coverage 3/3, Front Matter `requirement_ids` `"1.1"`, `"1.2"`, `"1.3"`; `check contracts` → `OK CONTRACTS_VERIFIED`, 0 ownership findings; `spec status order` → `State: tasks`, `design=fresh`, consistent. Approval ran `spec design approve order --approval-mode explicit` | `none` |
| `T1` | `pass` | `none` | `cart` in `implementation`; `tasks=fresh`; plan and `spec.yaml` committed; clean worktree | `tasks list cart` → `OK TASKS_LISTED`, 2 tasks; `check traceability cart` → `OK TRACEABILITY_VERIFIED`, active set 1.1–1.4; `grep -c execution tasks.yaml` → 0; `spec status cart` → `State: implementation`, `tasks=fresh`, consistent. Approval ran `spec tasks approve cart --approval-mode explicit` | `none` |

Every turn passed the harness environment check, and both runs selected
`sb-plan` from the platform registry. Neither was affected by ENV-0001 or
ENV-0003.

DS1's dispatch log holds one line: the Design phase took no investigation
dispatch. The Design procedure dispatches only when the investigation would
crowd the design, and this fixture is two small modules, so skipping it is
conforming. This is a workflow pass, not a dispatch measurement.

## Confirmation turns

- `DS1` turn 1 wrote the Design and Contract, left the gate unreached, and raised
  one blocking question: the Requirements name a cancellation window without its
  length, and the fixture defines none. Turn 2 answered as the maintainer, "The
  cancellation window is 30 minutes after the order is placed, the same for
  every order. Update the draft with that and present it again; I have not
  approved anything yet." The run revised the Design only and presented it
  again. Turn 3: "I approve the Design and Contract you just presented for
  order. Stop after Design." It approved, checkpointed, and stopped before the
  Contract Review.
- `T1` turn 1 wrote the plan and stopped before approval. Turn 2: "I approve the
  task plan you just presented. Stop after Tasks." It approved, checkpointed,
  and started no implementation.

Every confirmation was gated by `--expect` checks proving the draft existed and
the gate was still `not_reached`; T1's also proved the plan had no `execution`
key.

## Debrief dispositions

`git status --short` and `HEAD` were identical before and after both debriefs.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `DS1`, `T1` | `milestone status` reported `mode=all_spec` for a request that named one Spec and phase only loosely, and the driver had to decide that the request counted as named | `ambiguity` | `retained` | Third occurrence in this driver's runs (R1, DS1, T1); every run chose correctly. Reproduce against the `sb-plan` first-action contract before changing it |
| `DS1` | The template tells the author to remove inapplicable sections, but the scaffold check rejects a live Design whose durable `maintain` comments differ from the scaffold, and does not name the differing comment | `extra-step` | `retained` | Single observation with concrete evidence (`ARTIFACT_DURABLE_INSTRUCTIONS_MISMATCH`); reproduce before recording a finding |
| `DS1` | Described the Design description guidance as contradicting `ARTIFACT_DESCRIPTION_MISMATCH` | `ambiguity` | `discarded` | The Design procedure already requires the template's literal description on materialization; misreading |
| `DS1` | The window length is a Requirements gap rather than a Design decision | `ambiguity` | `discarded` | Fixture property; the run correctly asked instead of inventing a value |
| `DS1` | Inferred "no deferred destination" from truncated `adapter read deferred` output | `wrong-action-risk` | `discarded` | Driver's own truncation; the full adapter text was available |
| `DS1` | Flow-mapping YAML description, missing-Contract graph errors, first-action guard wording | `cosmetic` | `discarded` | Worked as specified |
| `T1` | `artifact list` paths are relative to `.specbind/`, so the driver first looked for `specs/cart` at the repository root | `extra-step` | `retained` | Single observation; watch for recurrence |
| `T1` | Whether the CLI-modified `spec.yaml` belongs in the checkpoint was inferred rather than stated | `ambiguity` | `discarded` | The run staged the correct paths; single observation |

## Cleanup

- Fixture paths removed: the scratch `sb-ds1` and `sb-t1` fixtures and their `.drive` turn records
- Main worktree after recording: only this record and the dashboard row changed
