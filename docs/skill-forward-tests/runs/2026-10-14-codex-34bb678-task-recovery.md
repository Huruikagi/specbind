# Forward-test run: 2026-10-14 / Codex / 34bb678 + Task recovery

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-14`
- Driver: Codex, real top-level sessions through `forward-test-drive.sh`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`; registered reviewer uses the installed product profile
- Tested build: `34bb678` plus the Decision 0218 implementation working tree
- Fixture language: `en`
- Scenarios: `I8`, `I9`
- Installed recovery reference SHA-256:
  `c61307ac8c6fff1a15e4e5be5fae8d1ae366f844234e45c4cc6f29ada8f5c913`

The Codex and Claude Code installed recovery references and the source had
identical hashes. This is packaging evidence for both agents; behavioral
measurements below are Codex only.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I8 | `pass` | none for the continuation-decision boundary | Original pending cart diff unchanged; Task remains pending | All eight answers matched the scenario. Actions show installed entrypoint, Spec-backed procedure and recovery reference reads plus the language Rule read, with no mutation or dispatch. HEAD stayed `be90c85d9b223b487eab612be767ae1e17282792`; status remained modified `src/cart.py` and untracked `scripts/`, `tests/`. | none |
| I9 | `pass` | none | Task 1 completed; Spec remains in implementation without completion evidence; clean worktree | Two instrumentation contexts identify the owner and fresh reviewer. The review returned A/B/C RESOLVED with evidence and APPROVED before `tasks complete cart 1`. Commit `2657d62` contains only cart source, runner, tests and Task progress. Independent post-run checks confirmed all four tests pass, fresh earlier gates, and no checkout change. | none |

I8 establishes the choice to continue or stop from supplied independent
assessments, not execution of the corrective owner. Case 1 permitted only the
third cycle; repeated A, incomplete mapping, preparation before A's repair,
exhaustion, missing remap history, and absent credentials all stopped. A fully
resolved third cycle proceeded to ordinary completion without a fourth repair.

I9's actual reviewer message was checked in the driver's session log, rather
than inferred from the owner's final report. It mapped A to the now-working
canonical test runner, B to the bounds check before cart mutation, and C to the
approved cart scope excluding checkout. The terminal verdict was APPROVED.
The owner then ran `tasks complete`, read the Git adapter, and checkpointed.
No fourth cycle or completion acceptance occurred.

The harness's final console rendering exited with a Windows `cp932`
UnicodeEncodeError after the agent had finished and saved its reply and actions.
The full session log, saved reply, reviewer return, and fixture state remained
available and were inspected directly. The read-only debrief used `PYTHONUTF8=1`
and completed normally. This output-rendering limitation did not prevent the
product scenario from running and is not counted as a product failure.

## Debrief dispositions

The I8 debrief ran only after judgment and a clean `.specbind` guard. It had zero
actions, and the before/after fixture status was identical.
I9's debrief also had zero actions and preserved its clean final worktree.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| I8 | Driver initially read the system skill-creator instructions | extra-step | discarded | It then read and applied the installed implementation package; no product action depended on the unrelated read. |
| I8 | Language Rule read was required for read-only interpretation | extra-step | discarded | Intentional existing contract; the absent Rule returned a successful no-change result. |
| I8 | Case 1 supplied safe local setup evidence but omitted an explicit diagnostic category | ambiguity | fixture limitation | The continuation bound was measured; exact corrective-role selection was not. This observation is not evidence of a product defect. |
| I9 | Driver tried reading Roadmap through the Steering selector | extra-step | discarded | Roadmap has a separate owning read surface; the driver recovered using the actual Steering inventory. |
| I9 | The supplied continuation ledger was prose rather than the assessor's formatted mapping | ambiguity | fixture limitation | All identities, dispositions and remaining budget were supplied; the fresh reviewer returned the complete required mapping before progress. |
| I9 | Ordinary Git diff omits untracked runner/test contents | wrong-action-risk | discarded | Both were explicitly inventoried and read for review; this run reproduced no missing-input review or product defect. |

## Cleanup

Both sessions and their read-only debriefs finished before cleanup. Temporary
fixtures `G:/specbind/.tmp-issue76-i8` and `G:/specbind/.tmp-issue76-i9`, their
sibling `.drive` directories, prompts, status snapshots, and test log were
removed after recording the evidence above. Only the intended product,
documentation, test, and run-record changes remained in the main worktree.
