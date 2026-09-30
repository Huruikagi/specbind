# Forward-test run: 2026-09-30 / Codex / fc7ffe8

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-30`
- Driver: `Codex` (headless top-level session through `forward-test-drive.sh`)
- Model: `gpt-5.6-terra`
- Driver profile: default `medium`; `workspace-write`; codex-cli `0.152.0`
- Tested build: `fc7ffe8ef48d60e2aa9fac89c15f35403e8782c7`
- Fixture language: `en`
- Scenarios: `R1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `R1` | `environment_invalid` | The driver did not retain fixture CLI isolation or clean memory context | Untracked `requirements.md` with two criteria; `order` at `requirements`, all gates `not_reached`; no Contract; HEAD `eb29347b18a9bec5868ec7218946ad2a9b569566` | Raw turn 01 read host `MEMORY.md` and the fixture's `.agents/skills/sb-plan/SKILL.md`; bare `specbind rule read language-style --for consume` failed with `unrecognized subcommand 'rule'`. The fixture binary supports that command. A native shell probe resolved the fixture binary with `pwsh -NoProfile`, but host mise `github-huruikagi-specbind/0.1.0/specbind.exe` with the profile loaded. | Harness isolation defect; no product finding |

The exact scenario request was sent without extra guidance. Thread
`01a0f2bf-5b42-78a2-bd69-180fcb67be16` started and executed model-generated commands;
this was not a host-sandbox launch block. The invalid run was interrupted before
approval once contamination was established. No confirmation or lifecycle
mutation was supplied by the maintainer.

Independent commands using the fixture's `.specbind/bin/specbind.exe` returned:

- `milestone status`: `requirements`, consistent, no diagnostics.
- `milestone scope`: only the new `order` Spec, cancellation responsibility.
- `spec list`: established `cart` plus `order`; no order Contract.
- `spec status order`: `requirements=not_reached`, coverage inactive.
- `check traceability order`: `OK TRACEABILITY_VERIFIED`, two Requirements,
  no active IDs.
- `git status --short`: only `?? .specbind/specs/order/requirements.md`.

ENV-0005: the installed `sb-plan` body was read, but this contaminated run is
not evidence of a valid complete Skill workflow. ENV-0004: approval was never
reached or sent, so the post-approval lifecycle boundary remains unmeasured.

## Confirmation turns

None. Sending approval to an invalid run would not measure R1.

## Debrief dispositions

No debrief was requested from this interrupted, contaminated session. The
observed failures were reproduced with read-only shell commands instead;
product usability cannot be inferred from the mismatched CLI.

## Cleanup

- Fixture and sibling turn records removed after evidence recording: `C:/Users/hurui/AppData/Local/Temp/sb-r1-codex-fc7ffe8-20260930{,.drive}`.
- Main worktree after recording: only harness correction, its focused checks,
  and forward-test documentation.
