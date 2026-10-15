# Forward-test run: 2026-10-15 / Codex / 4022a4f Design scope

[Back to the measurement dashboard](../results.md).

- Build: final Decision 0223 tree committed as `4022a4f`
- Binary SHA-256: `89a07e5f41f21e81eb304399bfab6a5fa903a2c1be35a9064298ba1c0c9108c6`
- Driver: Windows Codex real-session harness, `en` fixture
- Model: `gpt-5.6-terra`, `medium` (host default, confirmed from turn context)
- Scenario: VD4, fresh fixture with the required verification strategy
- Thread: `01a13fdb-ae60-7723-9be3-e3d93b6e6147`

| Scenario | Verdict | Expectation that did not hold | Fixture state | Mechanical evidence |
| --- | --- | --- | --- | --- |
| VD4 | `environment_blocked` | The configured architecture researcher could not start; the owner substituted local inspection, so a complete independent-review pass is not credited | Clean at `c81180a`; all artifacts and gates unchanged; wishlist Contract absent; review absent | Installed Skill read; scoped checks pass; semantic review reached; final `READY` explicitly limits itself to cart/direct consumed seams and requires complete Contract Review. Maintainer reran the whole graph check and it still fails for wishlist |

This confirms the changed scoped routing and final-report wording only.
It does not waive ENV-0007 or count the unexecuted researcher investigation.
Source session records contain the rejected `gpt-6-luna` call, which the harness
shell-action digest omits. The project model configuration was not changed.

## Debrief dispositions

The same driver returned a read-only debrief with zero tools. HEAD and worktree
stayed unchanged. It again reported the failed configured researcher and the
inference that manual local investigation was acceptable. FT-0067 records this
ambiguity separately from the implemented Design Contract scope.

## Mechanical verification

- Full Rust suite: 538 tests passed, including strict direct providers/shared
  resources and a guard test proving local success cannot accept an incomplete
  whole-project Contract Review.
- Final reporting/global-barrier wording: 101 Skill/protocol tests passed.
- Rustfmt, generated-schema check, Clippy with warnings denied and release build
  passed. No schema, lifecycle mutation or approval authority changed.
- Decision index: 223 records. Strict MkDocs build and language links passed.
- Public page pairs: CLI command reference and existing-implementation adoption
  guide. Both languages use the same scope and global-review boundary.
- Both affected rendered Codex Skills passed validation. The validator script
  needed Python UTF-8 mode on Windows; no Skill encoding change was necessary.

Removed the fresh fixture and its `.drive` directory after judgment/debrief.
No host profile, credentials, model configuration or production project changed.
