# Forward-test run: 2026-10-15 / Claude Code / 6155a5f

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-15`
- Driver: `Claude Code 2.1.296`
- Model: `claude-sonnet-5-5`
- Driver profile: `sonnet` alias / `medium`, headless real-session harness, Windows
- Tested build: `8d9b2bb` plus the Decision 0221 working tree, committed as `6155a5f` without further product changes
- Release binary SHA-256: `4d0ef2e9321a28b4dff31f2acda29096081351628889be36cae1c39f72ed3eee`
- Fixture language: `en`
- Scenarios: `DR2, DR6, DR11, DR12a, DR12b`

Each attempt used a separate fixture. DR2 and DR6 first attempts resolved the
maintainer's `specbind 0.1.0` after `.bashrc` prepended mise shims (ENV-0006).
They are environment-invalid, even though the harness initialization checks
passed. DR6 was stopped after it began adapting artifacts to that old CLI;
none of its mutations were repaired or reused.

Fresh retries used a child-process-only `BASH_ENV` file that prepended the
fixture bin directory and exported a `specbind` function invoking its exact
native executable. A login-shell probe returned `specbind 1.5.2`; both copied
binaries matched the SHA-256 above. No user profile or global installation was
changed. DR12 used the same isolation. DR11 only read installed Skill files.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR2, first attempt | `environment_invalid` | Current fixture CLI was not used | Clean, HEAD `47f2f36`, no owner dispatched | `rule` and `status --json` rejected; `--version` returned `0.1.0` | ENV-0006 |
| DR6, first attempt | `environment_invalid` | Current fixture CLI was not used | HEAD `054d030`; dirty Design/state, staged Contract YAML deletion and untracked Contract Markdown | Old CLI rejected current commands and reported false stale/missing artifacts; session interrupted | ENV-0006 |
| DR2, fresh retry | `environment_blocked` | Owning implementation did not reach a terminal result or validation | HEAD `1a45f81`; 0/2 Tasks complete; dirty README/source and untracked tests | Ordinary `general-purpose` owner loaded `sb-implement`, dispatched internal implementer/reviewer; review approved, but owner twice returned waiting. Claude rejected `sleep 60` with a Monitor instruction. No completion/checkpoint | ENV-0009; FT-0057 needs reconfirmation |
| DR6, fresh retry | `environment_blocked` | Owning implementation did not reach review, completion or validation | Tasks replan checkpoint `564458a`; earlier gates fresh; 0/1 Tasks complete; dirty source and untracked runner/tests | `sb-plan` and `sb-implement` owners both used `general-purpose`; implementation nested a registered implementer. Monitor schema discovery failed; long waits were rejected. Internal result eventually `READY_FOR_REVIEW`, owner remained non-terminal | ENV-0009; FT-0062 host boundary confirmed; FT-0057 needs reconfirmation |
| DR11 | `pass` | none | HEAD `e733d40`, clean and unchanged | Read installed Drive and validation Skills; all seven responses preserved stop/acceptance boundaries. Cases 4/7 name `BLOCKED`, revisions, dirty paths, stop after reread and a new Drive invocation with fresh whole-set assessment | FT-0064 confirmed, boundary interpretation only |
| DR12a | `pass` | none | Spec `release_ready`, completion fresh, clean at `80cfdb2`; milestone `release_pending` solely for unbound version | Drive dispatched one `general-purpose` validation owner, loaded the installed Skill and ran `sh scripts/test.sh` (4 tests). Acceptance uses preflight `27970170bb20dead7a685302496af9455e2461e4`; checkpoint changes only `spec.yaml`; no extra confirmation or Release | FT-0061 confirmed; FT-0062 validation host confirmed |
| DR12b | `pass` | none | HEAD `8ed1ded`, clean and unchanged; Spec `implementation`, completion `not_reached` | Drive selected from the natural request, parked validation as `HUMAN_DECISION`, dispatched no owner and recorded nothing under the explicit limit | FT-0061 explicit restriction confirmed |

DR12a's model execution completed successfully, but the harness's Python reply
printer failed on an en dash under Windows CP932. Replaying only the existing
reporting code with `PYTHONIOENCODING=utf-8` regenerated the action digest and
confirmed its initialization checks; no model turn or fixture action was
repeated. Subsequent debriefs used UTF-8 output.

## Confirmation turns

No approval or continuation was sent during measurement. DR2/DR6's one owner
continuation happened inside Drive. DR12a demonstrated the ordinary Drive
authority; DR12b supplied an explicit recording limit. Debriefs were sent only
after judging each valid-environment attempt.

## Debrief dispositions

Every debrief had zero tool actions and preserved HEAD and worktree state.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| DR2, DR6 | Owners return waiting, later internal notifications arrive after Drive's single continuation is spent | `wrong-action-risk` | `retained` | FT-0057 returned to confirmation-pending; ENV-0009 prevents an unqualified product regression verdict. Preserve the earlier passing measurement |
| DR2, DR6 | Claude labels a waiting receiver completed; long sleeps are rejected and Monitor requires discovery | `ambiguity` | `retained` | ENV-0009; do not bypass host restrictions or increase Drive's recovery budget from this run |
| DR11 | Mapping NO-GO/manual verification causes and partial-acceptance stops requires combining existing passages | `ambiguity` | `discarded` | Responses respected the existing boundaries; no reproduced new wrong action |
| DR12b | Driver wondered whether validation could assess without recording, then parked the whole recording workflow | `ambiguity` | `discarded` | Correct restriction; claim verification remains a separate route, no new validation mode requested |
| DR2, DR6, DR12a | Instrumentation, execution-fact discovery, baseline reads and repeated status checks add steps | `extra-step` | `discarded` | Fixture instrumentation or existing ownership contract; no new reproduced defect |
| DR11 | Agent-specific Skill copies and the Git adapter's conditional no-commit branch | `cosmetic` | `discarded` | Correct platform copy and conditional reasoning |

## Validation and cleanup

- Rust fmt, schema check, Clippy with warnings denied, all workspace tests and release build passed. The existing metadata-selection assertion was updated to include ordinary Drive authority.
- Decision validation and strict bilingual MkDocs build passed; both guide language links resolve to their counterpart.
- Raw `.drive` events and fixture snapshots are archived locally under ignored `.cache/forward-0221-evidence.tar.gz`, including the invalid attempts. They are not published with this record.
- Temporary fixture directories `sb-0221-{dr2,dr6,dr2b,dr6b,dr11,dr12a,dr12b}` and their `.drive` siblings were removed after archive verification.
- The main worktree contains only this record and the associated dashboard/worklist updates after implementation commit `6155a5f`.
