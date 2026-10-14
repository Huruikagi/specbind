# Forward-test run: 2026-10-15 / Codex / 8fc9917 + grouped completion

[Back to the measurement dashboard](../results.md).

- Driver: Codex real top-level sessions
- Model/profile: `gpt-5.6-terra` / `medium`
- Tested build: `8fc9917` plus Decisions 0219 and 0220 working tree
- DR9c fixture: `.tmp-issue81-final`, English, dispatch instrumentation,
  project-owned implementer/reviewer overrides installed before the baseline
- DR9c session: `01a13c68-bed8-7d41-955f-d124cad7df52`
- DR11 fixture: `.tmp-issue81-dr11`, separate read-only `dr10` recipe
- DR11 session: `01a13c68-ea6b-7c92-886f-3ebd599cd3e5`
- Drive source SHA-256:
  `c9e4db2c105dac6f8e9ff71382818bb25bdcc70c0acda2146958ed01b816ebe3`
- Validation source SHA-256:
  `e67683bbfe62772c2fc100590f9681a57226d668b80a7ab952dae35516cef5c3`

## Measurements

| Scenario/turn | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR9c / 1 | `environment_invalid` | The driver selected the old global CLI through a Git Bash login shell | Initial clean recovery state, unchanged HEAD `c5c0736` | `sh.exe -lc` replaced fixture PATH; the selected CLI lacked `rule` and JSON status. No lifecycle mutation or owner dispatch occurred. | CLI selection environment |
| DR9c / 2 | `pass` | None | Clean, consistent `release_pending`; both Specs validated with fresh completion; only `RELEASE_VERSION_UNBOUND` remains | Separate withdrawal `33c714c`, implementation `204f9d1`, joint acceptance `1d6ef50`; both evidence revisions equal `204f9d121d6ac2ce35e1ab4f159a1af568ca360c`; four canonical tests pass | FT-0059 resolved |
| DR11 / 1 | `pass` | None | Clean, unchanged HEAD `8fa09d56281388b70342dcc1f78fbac8b83a7560` | Two read actions loaded installed workflows and language policy; all six hypothetical boundaries matched the accepted contract; no mutation or dispatch | None |

The first DR9c turn is not completion evidence. Its supplied native-shell
environment fact allowed execution but did not preserve CLI selection inside a
login profile. A same-session environment correction named the exact
fixture-local executable and explained the PATH change, while preserving the
original task and authority. Clean-status and exact-HEAD guards passed before
the continuation; no fixture file, lifecycle state or policy was repaired.

DR11 distinguishes common-revision grouped acceptance from all stop paths:
NO-GO, unavailable checks and changed HEAD accept none; failed second acceptance
or checkpoint preserves partial work and reports it without rollback, subset
commit, unchanged-state retry or a new-revision continuation.

## DR9c fixture evidence

The same corrected driver turn completed all three owner dispatches:
`withdraw_cart_completion`, `implement_cart_input`, and `validate_specs`.
Instrumentation also observed the internal implementer and independent reviewer.
The log was not fully append-preserved by all contexts, so the final log alone
is not a context count; session dispatch records and intermediate snapshots
establish those distinct owners.

- `33c714cfcf9c954c39af640d1936c312f2802c66` changes only cart's `spec.yaml`:
  state becomes `implementation` and completion disappears. Earlier gate
  evidence is unchanged. Pending cart-input work then resumes.
- `204f9d121d6ac2ce35e1ab4f159a1af568ca360c` adds six lines to
  `tests/test_input.py` for zero-quantity rejection without cart mutation, and
  records cart-input Task 1 complete. No source or upstream artifact changes.
- The single final validator preflights cart-input and cart before either
  acceptance, reads both approved artifact sets and traceability, and runs
  `& 'C:\Program Files\Git\bin\sh.exe' scripts/test.sh` with clean Git status
  immediately before and after. Both candidates preserve that exact command
  and the common implementation revision.
- `1d6ef50ac9d10e3c237a10f852a1b49cc350d8a8` contains exactly both accepted
  `spec.yaml` state/completion transitions. Earlier gates are unchanged.
- Independent fixture inspection finds both completion records `fresh`, both
  Tasks complete, Contract Review fresh, no diagnostics, and a clean worktree.
  The canonical command independently passes all four tests without dirtying
  the fixture. No release binding, preparation, publication or finalization ran.

The initial [separate-commit failure](./2026-10-14-codex-8fc9917-completion-recovery-2.md)
remains recorded; this measurement confirms the changed coordination contract.

## Debrief dispositions

DR9c and DR11 read-only debriefs had zero actions and left HEAD/status unchanged.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Git Bash login selected the old global CLI | wrong-action-risk | environment limitation | Reproduced in turn 1; exact fixture-local selection corrected it without changing the fixture or product. |
| Annotated-tag verification was attempted on a lightweight fixture tag | extra-step | discarded | Driver command choice, recovered with ordinary Git reference inspection; no product defect. |
| Withdrawal needs actual causal evidence beyond the supplied narrative | ambiguity | discarded | Intentional authorization boundary; the complete validation owner owns the independent precheck. |
| Language-style lookup adds a read to a short hypothetical task | extra-step | discarded | Required shared language policy consumption; not a reproduced defect. |
| The happy-path case does not state checkpoint success in advance | ambiguity | discarded | The answer correctly describes the required checkpoint and post-check; the separate failure case measures refusal. |
| A changed HEAD might be a release binding | wrong-action-risk | discarded after source check | Acceptance requires the exact preflight HEAD even for binding metadata; the Skill explicitly requires binding before grouped preflights. Existing-record freshness does not waive the acceptance guard. |

## Verification and cleanup

Repository validation passed: Rust formatting, generated schema check, Clippy
with warnings denied, all workspace tests with all features, release build,
Decision index validation (220 Decisions), and strict bilingual MkDocs build.
The Codex and Claude Code installed Drive/validation bodies matched the source;
the guide language links resolve to their corresponding translation.

All scenario fixtures, sibling driver records and temporary prompts/logs were
removed after the sessions and debriefs finished. These run records preserve
the measured failures and final confirmation.
