# Forward-test run: 2026-10-04 / Codex / e8a5019

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-04`
- Driver: Codex, headless top-level sessions through `forward-test-drive.sh`
- Model/profile: `gpt-5.6-terra` / `medium`; installed role models unchanged
- Tested build: `62e4eea` plus the working tree committed as `e8a5019`
- Release executable SHA-256: `47db8e112874e94f950975ae562b6810637cd2c890c6f741a9cbd08d33c1f265`
- Fixture language: `en`
- Scenarios: G1, A2 (instrumented), DR2 (instrumented); I7 recipe validation

Claude CLI was unavailable on this host, including the mise lookup. These are
Codex measurements, not a repeat of the Claude Code batch. Every fixture used
the same release executable and installed the changed shared Skill bodies.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| G1 | `environment_blocked` | Registered researcher models could not start; the driver continued direct inspection, so a complete delegated analysis is not credited | Clean; order remains at Requirements with all gates `not_reached`; no Requirements or Research created | Exact G1 request selected and read installed `sb-gap-analysis/SKILL.md`, then read the gap-analysis protocol, Brief, complete scope, and Steering. This confirms the selection branch in Codex only | FT-0037; ENV-0007 |
| A2 | `environment_invalid` | Dispatch instrumentation is incomplete: no driver entry, two reader entries rather than the required driver plus readers; independent fresh-context coverage cannot be credited | Clean at `cdf7d4e5d6ffce5ff38d6652c1f5e90c964f5bb9`; no Roadmap or temporary adoption record | Installed `sb-adopt` was read and preflight fixed that revision. The raw session records two reader dispatches, both with inherited context (`fork_turns=all`); the instrumentation log records observable/structural investigation. The proposal preserves the confirmation boundary, but this is not a full A2 pass | FT-0018 remains confirmation-pending |
| DR2 | `environment_blocked` | Internal implementer configured for `gpt-6.1-sol` failed before implementation | Clean; cart remains in implementation with 0/2 Tasks complete and Task 1 actionable; no release | Drive read installed `sb-drive`, dispatched the owner, and reread Git/status after the environment stop. The returned error was `gpt-6.1-sol model is not supported when using Codex with a ChatGPT account.` The internal waiting/checkpoint branch was not reached | FT-0057 and FT-0058 remain confirmation-pending; ENV-0007 |

The same model failure would prevent I1 and DR6 from exercising their internal
implementation cycles. They were not retried with different role capabilities.
Rerun I1, DR2, DR6, G1, and instrumented A2 under a supported, fully observable
driver before closing the corresponding findings.

### I7 fixture repair

The fresh `i7` recipe passed its setup assertions. Commit `c6f5ff1` contains
only the Steering verification guidance; the remaining status is exactly
`M src/cart.py`, `?? scripts/`, and `?? tests/`. The canonical tests pass,
Steering is clean, and `src/orders.py` is unchanged. This fixes the contradictory
setup reported by Claude without weakening the clean-worktree expectation or
making a Task commit unrelated guidance. It is recipe evidence, not an I7
behavioral pass.

## Confirmation turns

None. A2 stopped at its proposal. G1 is analysis only and DR2 stopped on the
unsupported role model. No approval, model override, or continuation authority
was supplied.

## Debrief dispositions

All three completed drivers received the standard read-only debrief after
fixture judgment. Their debrief action digests contain zero commands, and Git
status and HEAD were unchanged before and after.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| G1 | Configured researchers could not start; driver interpreted direct reading as permissible | `wrong-action-risk` | retained in this record | ENV-0007 invalidates full delegated coverage; selection evidence remains separate |
| A2 | README naming drift and absent tests required explicit classifications | `ambiguity` | discarded | The proposal disclosed the suspected defect and limited maintained guarantees; intended reconciliation behavior |
| DR2 | Role capability failed only after status had reported actionable work | `extra-step` | retained | ENV-0007; lifecycle actionability does not prove account model access |

A2's first turn completed and saved its reply/action files, but the harness's
Python console print raised `UnicodeEncodeError` under Windows CP932. The saved
UTF-8 reply and raw session were used for judgment. Debriefs ran with
`PYTHONIOENCODING=utf-8`; no agent or product setting was changed.

## Other triage and validation

- FT-0058 now gives the configured path base and selector-based read route;
  implementer briefs must carry correct artifact paths and require direct reads
  of governing artifacts, rather than treating paraphrases as authority.
- Decision 0217 records owner waiting and bounded same-receiver recovery;
  capacity handoffs and existing retry/approval authority remain intact.
- The older FT-0053 and FT-0054 findings remain open. This batch provides no
  new evidence resolving Design decomposition or complete-Plan delegation.
- Other single observations from the Claude batch, including Requirements
  overwrite attempts caught by the CLI and release-summary coverage, remain
  observations in that original run; no unproven contract change was made.
- Rust formatting, generated-schema checks, Clippy with warnings denied, full
  workspace tests, and release build passed. The focused Skill/project-instruction
  suites passed 97 tests. Decision-index validation (217 entries), strict
  MkDocs, shell syntax, and final diff checks passed.

## Cleanup

- Removed the `sb-ft1004-{g1,a2,dr2,i7}` fixtures, their three `.drive`
  directories, and the DR2 setup log from the native Windows temporary folder.
- No fixture or generated build output was staged in the product repository.
