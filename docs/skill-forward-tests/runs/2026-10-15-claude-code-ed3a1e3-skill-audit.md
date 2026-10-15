# Forward-test run: 2026-10-15 / Claude Code / Skill audit initial build

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-15`
- Driver: Claude Code, Windows headless real-session harness
- Model: `claude-opus-5-5` (host default; no model override)
- Tested build: `ed3a1e3` plus the initial Decision 0222 working tree
- Release binary SHA-256: `7d9f2009a2f65511dc51e5772a4075f361949568ea75b29423fe76440935b830`
- Fixture language: `en`
- Scenarios: `B2a`, `DB2`, `VI5`

This build predates the Direct-diagnosis debrief clarification measured in the
[fresh follow-up](./2026-10-15-claude-code-0b6c394-direct-diagnosis.md).
Plan and validation behavior did not change between these measurements and
`0b6c394`; two trailing reference-file blank lines were removed before commit.

Each fixture had its own copied CLI. A child-process-only `BASH_ENV` file
prepended that fixture's bin and defined an exact-executable `specbind` function
to avoid the known login-profile PATH drift. No host profile was changed.
`PYTHONIOENCODING=utf-8` kept the harness reply printer usable on Windows.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| B2a | `pass` | none for shared-worktree scheduling | Both Specs at `implementation`, Requirements/Design/Tasks fresh; one accepted fresh Contract Review; clean at `51016e9`; no implementation changes | Nine fresh phase/validation/review receivers plus the owner were recorded. Cart Design/validation/approval completed at `15a03ac` before order Design started; order checkpoint `696aa67`; review `9dfa09a`; separate Tasks checkpoints `9432413` and `51016e9`. Every delta is a planning artifact or CLI-owned planning state | none |
| DB2 | `pass` | none for Direct diagnosis; debrief exposed an unsupported extra shared-ownership assertion | Clean, unchanged at `e3aa974`; Direct item pending; no synthetic Spec artifact | Natural request selected `sb-debug`; current check reproduced exit 1, guide contained `npm test`, exact summary required Python unittest; terminal category `IMPLEMENTATION`; no Spec/Task reads or writes | FT-0066 |
| VI5 | `pass` | none | `cart` at `release_ready`, completion fresh, clean at `236b1c3` | New `references/withdrawal.md` was read before mutation; withdrawal checkpoint `48253cd` then canonical `sh scripts/test.sh` passed four tests; separate acceptance checkpoint `236b1c3`; both commits change only `spec.yaml`, earlier gates unchanged; new evidence uses `48253cd` | none |

B2a deliberately starts with valid Contracts for two existing Specs. It does
not measure the separate absent-Contract question for new delivery Specs in
Decision 0222. Its dirty-handoff failure branch was not injected in this run.
Drive capacity recovery was extracted without text changes and mechanically
checked; this batch did not saturate a live receiver pool or claim a fresh DR7
measurement.

The first B2a recipe preparation stopped because Python's unittest runner
rejects an empty test set. No agent was started against that fixture. The recipe
now includes a real baseline test, and a fresh directory was prepared and
verified before the measured run; the failed setup was not repaired in place.

VI5's first turn had three denied PowerShell calls. The driver used the allowed
Bash surface, reproduced the current fixture CLI and completed both boundaries.
The denials are retained as environment friction, not hidden or counted as
executed verification commands.

## Confirmation turns

- B2a first presented all-Spec scope, gate names, delegation identity and the
  Tasks stopping point without authoring. A second user-channel message
  authorized Requirements, Design and Tasks for cart and order and explicitly
  stopped before implementation. The harness checked Design state and clean
  tracked/index state before sending it.
- VI5 first presented completion-only invalidation and its independent
  checkpoint without changing state. The scenario confirmation authorized that
  exact invalidation and fresh validation/acceptance. The harness checked stale
  completion and clean tracked/index state before sending it.
- DB2 was diagnosis-only throughout.

## Debrief dispositions

Every debrief had zero tool actions and preserved HEAD and worktree status.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| DB2 | Done condition/checks could be mistaken for required Roadmap fields; shared Contract read had no exact command and an unsupported ownership assertion followed | ambiguity / wrong-action-risk | retained and fixed | FT-0066; follow-up explicitly identifies evidence sources, separates the Direct branch and reads shared Contract state |
| DB2 | No persisted Direct failure record | ambiguity | discarded | The user named the check and the driver reproduced its current failure; no Task failure state is required |
| B2a | Design and Tasks also mapped baseline cart IDs 2.1/2.2; owner described this as potential scope expansion | wrong-action-risk | retained as a scoped observation, not a proven contract defect | Design permits retained mappings and task-planning permits genuinely touched baseline IDs alongside active IDs. The plan preserves dictionary reads without a new API. Extra mappings alone do not prove unauthorized behavior; no gate was rewound |
| B2a | Delegation required a confirmation even though approved plans were requested | extra-step | discarded | Existing explicit gate-authorization contract; the driver correctly waited |
| B2a | Drafts needed template-section/line-ending repairs | extra-step | discarded | Authors repaired their own drafts before independent review and approval; no handoff contamination |
| B2a | Durable maintain instruction remained in Design | cosmetic | discarded | Required by the scoped authoring protocol, not a leaked create instruction |
| VI5 | Relative diagnostic path caused an initial missing-file read | extra-step | retained in this record | Driver found the configured path, made no wrong write, and completed the exact withdrawal |
| VI5 | Git adapter checkpoint wording, ignored bytecode, release blocker/state labels | ambiguity / cosmetic | discarded | Completion metadata checkpoint is explicit in the fixture adapter; ignored bytecode is the fixture baseline; status labels are distinct projections. No changed authority or failing required check |

## Cleanup

Removed the isolated fixture directories `sb-audit-20261015-b2a`,
`sb-audit-20261015-b2a-2`, `sb-audit-20261015-db2`, and
`sb-audit-20261015-vi5` under the Windows user temp directory, their harness
state directories and task-specific environment files after judgment/debrief.
Main worktree changes were limited to the audit source, tests and these records.
