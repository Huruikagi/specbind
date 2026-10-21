# Forward-test run: 2026-10-21 / Codex / workflow boundaries

[Back to the dashboard](../results.md).

- Tested build: `b3d2057` (fixtures built from the identical working tree over
  `3169dcd` before the fix commit)
- Driver: app `gpt-6.1-sol` / `medium`, no inherited turns; one canonical CLI
  attempt with its default `gpt-6.1-sol` / `medium`
- Fixture language: `en`; scenarios: A2, I3r, DR6; dispatch instrumentation enabled
- Binary SHA-256: `0f22dbdca580bdeaf107f410ea77fa1928281114e42993eaa1b3318e28c15a50`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| A2 CLI | `environment_invalid` | Driver could not start with the default model | Original fixture, no agent actions | HTTP 400: model unsupported with the ChatGPT account; no final result | ENV-0007 reconfirmed |
| A2 app | `environment_invalid` | Both nested readers reported in Japanese in the English standalone fixture | Complete proposal awaiting confirmation; no active milestone or Specs; clean Git state | Actual reader dispatches both use `fork_turns: none`; three instrumentation entries; HEAD and every tracked/unignored baseline file unchanged | FT-0071 dispatch observation only; isolated confirmation requires a new fixture |
| I3r app | `environment_invalid` | The nested reviewer reported in Japanese despite the English fixture | Direct completed at `644ef11`; only Roadmap completion metadata remains dirty | Reviewer reads governing Direct inputs; one implementation-only checkpoint; status reports 1/1 completed, current HEAD and separate release blockers | FT-0072 report observation only; no isolated scenario pass |
| DR6 app | `environment_invalid` | Implementation/review/validation owners retained the host's Japanese preference; the initial Plan owner also inherited conversation | Tasks 2/2 completed, completion fresh, clean worktree; no release | Plan used `fork_turns: all`, waited for its Tasks receiver and returned terminally; six canonical tests pass and upstream inputs remain unchanged | FT-0065 waiting observation only; owner inheritance retained for isolated reproduction |

## Final environment audit

The initial parent-only judgments were withdrawn before recording completion.
All top-level drivers spoke English, but the nested A2 readers, I3r reviewer,
and DR6 implementation/review/validation owners spoke Japanese. This is the
host-instruction contamination signal in the running guide. The initial driver
preface did not protect nested no-history contexts, which independently receive
host project instructions. None of these app attempts is an isolated pass.

The fixture builder was then corrected to carry the standalone-project and
selected-language boundary in its existing instruction files. Fresh measurements
use new fixture directories and a separate run record; these attempts and their
mechanical observations remain intact. Neither a plausible artifact state nor
the fixture adapter's legitimate commit authority clears this isolation defect.

The app driver discovered the installed `.agents/skills` package. Its unprompted
English proposal named reverse mode, the full fixed revision
`191f4b6b54deab988bd0b3315d6a71abbf7e48ce`, baseline v1.0.0, cart/order ownership,
the order-to-cart seam, maintained intent, unknowns, the pending README naming
finding, exclusions, and the post-confirmation lifecycle. No approval was sent.
Four Steering selectors were read separately. The parent session's actual
dispatch arguments name `behavior` and `structure`, each with no inherited
turns; the instrumentation count alone is not the isolation evidence.

After judgment, `adoption preflight` remained ready at the same source revision,
`milestone status` returned `NO_ACTIVE_MILESTONE`, and `spec list` reported zero
Specs. The same driver's read-only debrief left all baseline hashes, HEAD and
status unchanged. Its missing-tests observation is an evidence limitation,
not a reproduced violation of a maintained guarantee.

I3r's reviewer actually read the task-review protocol, Roadmap body, both
Steering documents, explicit shared-Contract absent result, and the guide diff.
The implementation checkpoint `644ef11fd69082cf8372bc639776292d1cea0fad` adds
only `CONTRIBUTING.md`. The owner then ran the clean-revision preflight and
completion handshake. Baseline comparison found only the new guide and the
Roadmap's `status: completed` line; the existing idle cart Spec and every other
baseline file were unchanged. The unprompted English final reply identifies
`.specbind/steering/roadmap.md`, the implementation-only adapter limit, current
HEAD `644ef11`, and `RELEASE_TARGET_DIRTY` plus `RELEASE_VERSION_UNBOUND`. It
does not claim that HEAD is absent or that completion implies release readiness.
The read-only debrief returned `none` and left the same Git state and guide.

DR6's Plan owner collected its internal Tasks receiver and returned a terminal
result at `2026-10-21T03:42:42Z`, after that receiver completed at
`03:42:28Z`. It reported the repaired mapping, delegated approval, clean
checkpoint `5af51c101d88e217dc7b33a57866bf0b8e3a394c`, and implementation handoff.
The driver sent no same-owner continuation. This is scoped evidence for
FT-0065's waiting rule, not isolated confirmation or a complete DR6 pass.

The independent failure is established by Drive's actual first owner call:
`task_name: plan`, `fork_turns: all`. Decisions 0192/0221 and DR6 require an
ordinary fresh owner; creating a new named agent with inherited conversation
does not satisfy that requirement. Retain this owner-dispatch observation for
reproduction in the isolated fixture; it is separate from A2's reader boundary.
An environment-only status reply from the supervisor listed the four active
contexts after the driver asked about capacity; it supplied no workflow method,
approval, correction, or continuation.

DR6 finished with plan checkpoint `5af51c1`, runner checkpoint `3cc365b`, bounds
checkpoint `85b5e64`, and completion-only checkpoint `db421f5`. The old runner
Task 2 became new Task 1; old blocked Task 1 became new Task 2 with its historical
block preserved. After runner implementation, a debugger reassessed the cause
and implementation reopened Task 2 through the CLI. Both Task reviews returned
APPROVED; final validation returned GO and recorded completion against
`85b5e64c2a54bdb601f4969205ce7537fb5de096`.

Independent judgment reran `sh scripts/test.sh` (six passing tests) and read
milestone status, Spec status and Tasks. The result is consistent, Tasks 2/2,
all gates including completion fresh, and only `RELEASE_VERSION_UNBOUND` blocks
release. Baseline hashes changed only for `src/cart.py`, `tasks.yaml` and
`spec.yaml`; Requirements, Design, Contract, the accepted Contract Review and
the upstream gate evidence are preserved. New runner/test files are committed.
The final HEAD is `db421f599c04f6ba439dcc77123a44cfa08603ca`; the worktree is
clean. The fixture Git adapter explicitly authorizes these local checkpoints,
so host repository policy is not needed to explain them. No release ran.
After the read-only debrief returned `none`, HEAD, every tracked/unignored file,
the ten-entry instrumentation log and clean Git status were unchanged.

Session evidence:

- A2 CLI: `01a15c84-b8e7-7de3-aeb8-db84f00d855b`
- A2 app: `01a15c85-b5cd-7d21-b639-d393d8785444`
- I3r app: `01a15c87-d263-78e2-b673-0f9d16c77b06`; reviewer:
  `01a15c88-ffbe-75b0-95bc-876e4b073ae9`
- DR6 app: `01a15c8b-1ddc-78c0-a43f-c99296a081ed`; Plan owner:
  `01a15c8b-bee1-7731-abce-bd48bec2ea55`; Tasks receiver:
  `01a15c8c-5c3e-7852-995b-faaf8a4159b8`

No canonical default-model CLI, platform Skill-registry selection, or isolated
app scenario coverage is claimed. The mechanical observations remain scoped.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| A2 | Missing local tests do not fit neatly into the unknown-product-guarantee categories | ambiguity | discarded | Coverage remained explicitly unverified; no blocking guarantee was invented or silently accepted |
| I3r | none | — | none | No additional observation |
| DR6 | none | — | none | Owner-inheritance observation comes from actual dispatch arguments, not the debrief |

## Other verification

Rust formatting, generated-schema check, Clippy with warnings denied, full
workspace/all-features tests (including 95 Skill tests), and release build passed.
Rendered adoption checks cover Codex, Claude Code and Generic; Plan ownership
checks cover Codex and Claude Code. Decision index validation and strict bilingual
MkDocs build passed. These checks do not replace blocked behavioral coverage.

## Cleanup

After judgment and read-only debrief checks, removed the dedicated root
`C:/Users/hurui/AppData/Local/Temp/sb-ft-20261021-3169dcd`, including all original
and retry fixtures, baseline snapshots and temporary harness logs. Confirmed
that the directory no longer exists. Evidence above and Codex session identifiers
are retained; no consumer fixture files were copied into the main worktree.
