# Forward-test run: 2026-10-21 / Codex / final worktree check

[Back to the measurement dashboard](../results.md).

- Tested build: `06bd198` + final Decision 0235 tree
- Driver: app `gpt-6.1-sol` / `medium`, `fork_turns: none`
- Fixture language: `en`; isolated standalone projects with their own instructions
- Release and fixture binary SHA256:
  `DF4CF30BF00F0C9135B7BC4BD7909A38E95B029826A6D43D1E704C67DCB9C0E6`
- Spec-backed procedure SHA256:
  `2AA385F1B95A7EFEDE3142F21834035AC38A24FEF67372EAD0E2EF1E8289968C`

The drivers receive only the scenario request and native working-directory/PATH
facts, without naming the Skill or worktree commands. They discover the installed
Skill from the fixture. This measures the app workflow; canonical CLI coverage
remains blocked by ENV-0007 and Claude Code is not measured. The failed initial
attempt is retained in the [separate record](./2026-10-21-codex-06bd198-worktree-initial.md).

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I1 | pass | none | Task 1 completed; Spec still implementation, next action validation, completion not reached; clean local checkpoint `533381f4f967afe4cbe04fe9c69d554146696db0` | External capture before dispatch; `WORKTREE_CHECK_MATCHED` for exactly `src/cart.py` and `tests/test_cart.py`; fresh independent APPROVED review; CLI completion. Independent four-test run and status checks pass; checkpoint contains only source, tests and Tasks execution. | none |
| I4 | pass | none | Task 1 completed; local checkpoint `48174c709f9f09329d27c18d44656915019c7686`; only the original `src/orders.py` remains dirty | Capture records orders as dirty; check matches exactly source and tests, with no unexpected or previously dirty touched paths. Fresh independent APPROVED review, CLI completion and six passing tests. Checkpoint includes only source, tests and Tasks execution. Original orders bytes and index entry are preserved. | none |

I1 driver session: `01a15dee-364a-7e91-bfcf-1151eabe9eba`. Its actual trace
shows installed-procedure reads, capture before dispatch and check before review.
The file snapshot persists in the allowed visualization workspace outside the
fixture. Fresh ordinary implementer/reviewer contexts both use inherited
`gpt-6.1-sol` / `medium`; the app exposes no registered-role selector. Their
completed reports and reads confirm independent review, without claiming the
registered-role dispatch interface was exercised. No recovery cycle, completion
handshake or push occurs.

I4 driver session: `01a15df1-0be7-77f1-b86e-f94e44e18cc6`. Both fresh receiver
contexts use `gpt-6.1-sol` / `medium`. The trace confirms capture/check through
the installed CLI, unchanged HEAD during implementation, and empty
`unexpectedPaths`, `preexistingTouchedPaths` and `declaredUnchangedPaths`.
The unrelated orders SHA256 before and after is
`9ADBB1BFC44A9A4442EE6843E7CF672EF63C900FD01AD770EC1C56A0DED0CCC7`.
The original snapshot independently records `dirty: true`; the final commit
excludes that path and only its original edit remains unstaged. Final Spec state
is implementation with completion not reached and validation as next action.

## Confirmation turns

No additional confirmation was supplied. The task request and fixture Git
guidance authorize the implementation cycle and narrow local checkpoint.

## Debrief dispositions

I1's prescribed debrief follows independent fixture judgment. Before and after,
`git status --short` is empty. Two observations are non-findings: tool-output
truncation required recapturing directly to an external file **before** dispatch
(extra step; no lost evidence or changed boundary), and the host lacks a role
selector despite installed role configuration files (fallback capability, not a
failed registered model). No product defect is inferred from either observation.

I4 reports the same tool-output truncation and direct-to-file capture before
dispatch. Its before/after debrief status remains only ` M src/orders.py`.
Both observations concern the host's displayed-output limit: capture itself
returns complete JSON, and the documented external-file workflow works. They
do not justify weakening exact snapshot retention or adding a product finding.

## Verification and cleanup

All 601 Rust tests pass, including 12 worktree-specific CLI tests. Formatting,
generated schema checks, Clippy with warnings denied, release build, 96 embedded
Skill tests after the final wording change, Decision validation and strict
English/Japanese documentation build pass. CLI tests additionally cover same-path
edits over earlier staged/unstaged work, staging-only changes, renames, ignore-rule
changes, HEAD movement, malformed inputs, repository binding and unsupported
index states; checks leave index bytes unchanged. App runs do not replace that
mechanical coverage.

Fixture roots `ft-worktree-i1-final` and `ft-worktree-i4-final` under
`tools/specbind/target/`, plus their retained external snapshots, are removed
after judgment and debrief checks. Main worktree changes remain this unit's
source, tests, design, bilingual documentation and measurement records.
