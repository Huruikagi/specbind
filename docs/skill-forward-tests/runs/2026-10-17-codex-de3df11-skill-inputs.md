# Forward-test run: 2026-10-17 / Codex / governing inputs

[Back to the dashboard](../results.md).

- Tested build: `de3df11` plus Decision 0229 working tree
- Driver: app `gpt-6.1-sol` / `medium`, each scenario with no inherited turns;
  one canonical CLI attempt using its default `gpt-6.1-sol`
- Fixture language: `en`; scenarios: A2, I3r, I14, VD7, VD8
- Binary SHA-256: `ed1e549dcbbcc2ae3b756aca50638aac7eaa0f5fff75671083253c85e7b1e144`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| I3r CLI | `environment_invalid` | Driver could not start with the default model | Original fixture, no agent actions | Startup HTTP 400: model unsupported with the ChatGPT account; no final result | ENV-0007 |
| I3r app | `pass` | none | Direct completed, implementation commit `ad62493`; only Roadmap completion metadata remains dirty | Fresh independent reviewer read Roadmap, both Steering documents, shared-Contract absent result and actual guide; APPROVED; clean-HEAD preflight then completion; `milestone status` reports 1/1 completed | FT-0072 from debrief, separate from completion result |
| I14 app | `pass` | none | Direct remains pending; original untracked guide preserved | `REROUTABLE` plus `HUMAN_DECISION` to Discovery; no Plan dispatch, gates or writes; HEAD and all original file hashes identical | none |
| A2 app | `product_failure` | The two evidence readers inherited the dispatcher conversation | Proposal awaiting confirmation, no Spec or adoption record, clean Git state | Actual dispatch arguments use `fork_turns: all` twice; instrumentation contains driver plus two readers. All four Steering reads have exactly one selector, with no malformed call | FT-0071; FT-0063 confirmed fixed |
| VD7 app | `environment_invalid` | English standalone fixture produced Japanese narration | No file, HEAD or lifecycle changes | All three Steering documents read; NOT_READY identifies persisted rejected SKU/quantity versus `data-handling`; every baseline file hash unchanged | Host instruction contamination |
| VD8 app | `environment_invalid` | English standalone fixture produced Japanese narration | No file, HEAD or lifecycle changes | `STEERING_FRONTMATTER_INVALID` for `steering/data-handling.md`; BLOCKED / INPUT, no READY/NOT_READY; every baseline file hash unchanged | Host instruction contamination |

VD7/VD8 retain the observed input handling as scoped evidence, not isolated
behavioral passes. Neither crossed a checkpoint. I3r was separately audited:
the fixture adapter explicitly authorizes its implementation checkpoint and
forbids push; host repository commit policy is not needed to explain that commit.
No default-model CLI or platform Skill-registry selection coverage is claimed.
App drivers discovered and read the installed `.agents/skills` packages.

I3r's reviewer actually used `fork_turns: none`, independently read
`task-review`, `milestone scope --include-body`, both `steering read` inputs and
`contract shared read`, and returned APPROVED without writing. Its owning driver
ran Python `compileall`, whitespace and link checks, committed only
`CONTRIBUTING.md`, then completed the item at that committed revision. Baseline
comparison found only the new guide and the Roadmap's `status: completed` line;
the existing idle cart Spec was untouched. The implementation-only adapter
does not authorize a metadata checkpoint; the driver reported that limit and
did not push. Release is not claimed.

A2's full fixed source revision was
`5cf6fea48bfaac1a93994d742a16694e593aec55`. Actual calls separately read
`conventions`, `product`, `structure`, and `technology`. Its proposal named
baseline v1.0.0, cart/order responsibilities and dispositions, then awaited one
confirmation. The bad context inheritance is established by actual tool
arguments, not inferred from reader names or the debrief.

Session evidence:

- I3r CLI: `01a149dd-ffd9-76d2-b89c-4a31d7fc9521`
- I3r app: `01a149e0-cb4b-7db3-b9d0-6a69b07921d4`; reviewer:
  `01a149e1-a5a2-7a81-b860-d60712ce7d39`
- I14: `01a149e0-88a5-7580-a40d-c8136307c091`
- A2: `01a149df-1516-7111-85d8-2756dda197cf`; readers:
  `01a149df-8664-70d0-8192-b8c637548ca3`,
  `01a149df-97bf-72d2-875f-bdb9678bc7db`
- VD7: `01a149dd-664a-7602-a3ed-cc543229c6c8`
- VD8: `01a149dd-7d8c-7700-8a44-d31c30a7b944`

## Confirmation turns

A2 stopped at its proposal; no approval was supplied. I3r's initial request
authorized the guide and independent review. I14 supplied an existing diagnosis
and bounded Drive replan authority; it did not authorize reclassification.
No later confirmation was introduced to teach the expected route.

## Debrief dispositions

Debriefs followed fixture judgment and prohibited commands and writes. Git
status and baseline hashes remained unchanged afterward; I3r retained only the
same Roadmap change. Debriefs do not replace the mechanical verdicts above.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| A2 | “Fresh readers” was implemented as two children inheriting all parent turns | wrong-action-risk | retained | Actual dispatch violates Decision 0109; FT-0071 |
| A2 | Source behavior and maintained intent needed explicit dispositions when tests were absent | ambiguity | discarded | Proposal made the distinction and awaited confirmation as required |
| I3r | Direct reference says `Revision` is unavailable after an uncommitted metadata change, but status prints current HEAD | ambiguity | retained | Reproduced in status and exact Skill wording; FT-0072 |
| I14 | none | — | none | No additional observation |
| VD7 | Deferred adapter was read although no deferred finding was produced | extra-step | discarded | No write or violated obligation |
| VD8 | Batched read continued with protocol/rules after the Steering failure | extra-step | discarded | No semantic verdict, repair or write followed |

FT-0071 and FT-0072 are separate pre-existing wording gaps found during this
batch; they remain on the worklist rather than silently widening Decision 0229.

## Other verification

Rust formatting, generated-schema check, Clippy with warnings denied, the full
workspace/all-features tests and release build passed. Rendered Skill tests
cover Codex, Claude Code and Generic bodies; protocol tests cover Direct inputs
and the Design input stop. Decision index validation and strict bilingual
MkDocs build passed. These checks do not replace missing behavioral coverage.

## Cleanup

Dedicated fixtures `i3r`, `i3r-app`, `i14`, `a2-app`, `vd7`, `vd8`, their setup
logs and baseline records under
`C:/Users/hurui/AppData/Local/Temp/sb-audit-20261017` were removed after recording.
The shared parent also contained the separately recorded Claude attempt.
Main worktree changes are limited to the requested asset corrections, tests,
scenario recipes, decision, bilingual reference and these run records.
