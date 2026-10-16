# Forward-test run: 2026-10-16 / Codex / 84bed31 Design environment stops

[Back to the measurement dashboard](../results.md).

- Tested build: final Decision 0224 tree committed as `84bed31`
- Binary SHA-256: `84983441ef4495e80b7f2a903e52957a5a8923fb4483163020dbe5aee7897352`
- Fixture language: `en`
- Natural request in every measured run: `Check the cart design before we build on it.`
- CLI driver: Codex 0.152.0, explicit `gpt-5.6-terra` / `medium`
  compatibility comparison. The canonical `gpt-6.1-sol` driver default remains
  unchanged. VD4c sets the fixture researcher to `gpt-5.6-terra`; VD6 deliberately
  sets it to `specbind-forward-test-unavailable-model` before session startup.
- App driver comparison: fresh `gpt-6.1-sol` / `medium`, `fork_turns: none`.

## Measurements

| Scenario / surface | Verdict | Expectation that did not hold | Fixture evidence |
| --- | --- | --- | --- |
| VD4c / real CLI session | `pass` | None for the local-review and successful-investigation boundary | Configured researcher started, read cart/order code and project context, returned its result, and completed before the owner issued `READY`. The final report explicitly limits readiness to cart/direct consumed seams and requires complete Contract Review. Clean HEAD `a716bab` is unchanged. |
| VD6 / real CLI session | `pass` | None | The named researcher actually failed with the intentionally configured unsupported model. The owner returned `STATUS: BLOCKED`, `BLOCKER: ENVIRONMENT`, and no Design verdict. No substitute investigation, subsequent command, model change, finding write, or lifecycle mutation occurred. Clean HEAD `1c20e41` is unchanged. |
| VD4 / app subagent | `environment_invalid` | Host language contamination: final prose was Japanese despite the standalone English fixture | The installed Skill body was read, scoped checks passed, and local context reading led to `READY` without nested dispatch. Clean HEAD `e50c742` is unchanged. Do not count this as a top-level registry, named-role, or default-model independent-investigation pass. |

Maintainer judgment reran `spec status cart`, `milestone review status`, the
Design-scoped Contract check, and the whole graph check in all three fixtures.
The local check passes; the global check still fails for wishlist's absent
Contract. Cart remains in `tasks` with Requirements/Design fresh and
Tasks/completion not reached; Contract Review remains absent. `git status
--porcelain=v1` is empty and no wishlist Contract was created.

The successful CLI research child inherited the review owner's context
(`fork_turns: all`); it was separate from the fixture's shell-authored Design.
This confirms executed investigation, not clean-room context isolation.

Session evidence:

- VD4c owner: `01a1441c-624b-7051-9523-12c9c5c29766`.
- VD4c researcher: `01a1441d-3c8d-7fc2-a870-17b8d4af08c6`; turn context confirms
  `gpt-5.6-terra` / `medium`, actual code reads and completed activity. Its final
  result reports no mismatch or hidden prerequisite.
- VD6 owner: `01a1441c-e0ea-72b0-8c9a-8a3b75ddee8c`. Raw session evidence records
  `agent_type: specbind-researcher` and the child error: the configured
  `specbind-forward-test-unavailable-model` is unsupported for this account.
  The shell-action digest alone does not expose nested dispatch errors.
- App driver: `/root/vd4_app` in the maintainer session.

## Environment probes and setup

Before product testing, an empty-directory CLI probe requesting only
`MODEL_READY` rejected `gpt-6.1-sol` before any tool action (thread
`01a14414-8c60-7253-b031-8ef0c86daf71`). An app subagent with the same model and
effort returned `MODEL_READY`. ENV-0007 remains active for the canonical CLI
profile. Official [model guidance](https://learn.chatgpt.com/docs/models)
describes account/client-dependent availability; selecting a model does not
establish account access. No host configuration, credentials, or product role
defaults were changed.

The first VD4c recipe attempt stopped before any driver with
`MILESTONE_REPOSITORY_DIRTY`: regenerated role assets had not been checkpointed.
The recipe now commits those setup assets before milestone creation. The
successful measurement used a separate fresh directory; this setup failure is
not a product test result.

## Debrief dispositions

Debriefs were requested only after judgment and a clean-worktree check. Both
CLI resumes ran zero tools. The app debrief returned `none`. HEAD and worktrees
remained unchanged after all debriefs.

| Observation | Disposition |
| --- | --- |
| VD6 described the unavailable researcher and separate semantic investigation as friction; it explicitly recognized that substitution was forbidden | Expected deliberate failure input and review boundary; FT-0067's exact affected branch is confirmed fixed |
| VD4c noted that its initial commentary preceded the language-style read | Initial commentary preceded Skill discovery, and the fixture had no language-style rule. Retained here as an ordering observation, not evidence of a new language-policy failure or authority to change the tested Skill |
| VD4c used `spec list` to resolve cart and the CLI's active IDs to distinguish acceptance criteria | Existing inventory and scope rules worked as intended; no new finding |

One initial debrief invocation incorrectly supplied a string rather than a
shell expression to `--expect`. The guard rejected it and sent nothing. The
corrected invocation used the verified clean-worktree boundary; it did not
rerun or change the scenario.

## Verification and cleanup

- Focused Skill/protocol checks: 102 tests passed, including both installed
  agent renderings and Plan's no-approval/no-revision/no-ledger-change boundary.
- Full Rust suite, Rustfmt, schema check, Clippy with warnings denied, release
  build and scenario shell syntax passed.
- Decision index: 224 unique records. Strict MkDocs build passed; the updated
  English/Japanese Skill index pages and reciprocal language links were checked.
- Fixtures removed after evidence capture: `sb-vd4c-20261016` (failed setup),
  `sb-vd4c-20261016-b`, `sb-vd6-20261016`, `sb-vd4-app-20261016`, their created
  `.drive` directories, and the empty `sb-review-probe-20261016` directory,
  all under the current user's local Temp directory.

FT-0065 and other unrelated worklist entries are outside this change. Plan's
environment-stop propagation is guarded by the installed-asset tests; this
batch did not run an entire nested Plan recovery conversation.
