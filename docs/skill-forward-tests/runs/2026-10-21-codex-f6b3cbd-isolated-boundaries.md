# Forward-test run: 2026-10-21 / Codex / isolated workflow boundaries

[Back to the dashboard](../results.md).

- Tested build: `f6b3cbd`, including product fixes from `b3d2057`
- Driver: app `gpt-6.1-sol` / `medium`, no inherited turns
- Fixture language: `en`; dispatch instrumentation enabled
- Binary SHA-256: `0f22dbdca580bdeaf107f410ea77fa1928281114e42993eaa1b3318e28c15a50`

The [initial attempts](./2026-10-21-codex-b3d2057-boundaries.md) were invalidated
after nested contexts retained the host repository's language preference. This
batch uses new fixture directories, with the standalone-project and selected
language boundary in the fixture's existing project instruction files. The
quoted scenario requests are unchanged. No workflow method, expected dispatch
option, or approval was added to those requests. The product binary is unchanged.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| A2 | `pass` | none | Complete proposal awaiting confirmation; clean HEAD and worktree; no lifecycle artifacts | Both actual reader calls use `fork_turns: none`; parent and both readers report in English; all baseline file hashes unchanged | FT-0071 confirmed |
| I3r | `pass` | none | Guide committed; Direct 1/1 completed; only Roadmap completion metadata dirty | Fresh English reviewer reads all governing inputs; actual preflight/completion succeed; final report identifies current HEAD and separate release blockers | FT-0072 confirmed |
| DR6 | `product_failure` | Drive owners inherit conversation; the Plan owner is reused for implementation | Repaired plan, Task 1/1 completed, completion fresh, clean worktree; no release | Actual Plan and validation calls use `fork_turns: all`; implementation uses `followup_task` on Plan's receiver. Eight canonical tests pass; all six contexts report in English | FT-0073; FT-0065's internal-planner waiting branch not exercised |

A2 fixed source revision: `d42fa30c31a2c04b29fd11af2867060d36500697`. The
proposal names v1.0.0, cart/order responsibilities, the order-to-cart seam,
maintained intent, no blocking/deferred semantic unknowns, the pending README
name mismatch, excluded areas, and the post-confirmation lifecycle. It reports
the absence of repository-local tests as an evidence limitation, with inline
numeric/cart-copy assertions separately identified. No confirmation was sent.
All four Steering selectors were read separately. Three instrumentation entries
corroborate the parent plus two readers; actual tool arguments prove isolation.

I3r's reviewer used `fork_turns: none` and read the task-review protocol, exact
Roadmap body, conventions and structure Steering, shared-Contract absent result,
Git adapter, source modules and actual guide diff. Both owner and reviewer
responded in English. The only implementation checkpoint is
`0737749dffdd536f1fe9c663f158240434b6868f`, adding `CONTRIBUTING.md`. The owner
then ran `milestone direct preflight contributing-guide` and `milestone direct
complete` with that exact revision. Independent CLI judgment reports Direct
1/1 completed, consistent state, that HEAD as Revision, and
`RELEASE_TARGET_DIRTY, RELEASE_VERSION_UNBOUND` as release blockers. The final
report names the dirty Roadmap and adapter's implementation-only commit limit.
Every baseline file except the Roadmap is unchanged, including the idle cart
Spec. The Roadmap diff adds only `status: completed`. The guide, all other file
hashes, HEAD and Git status stayed unchanged after debrief; instrumentation has
two entries. No release or push occurred.

DR6's Plan owner completed at `2026-10-21T04:07:15.785Z`, with checkpoint
`f6e4cb42041fc27fc3417bb936674ff6024b5a20`. It merged old Task 2's runner/test
responsibility into old Task 1 and preserved Task 1's blocked execution entry
and reason. Old Task 2 had no execution entry. Only Tasks and their gate were
revised; Requirements, Design, Contract and accepted Contract Review stayed
unchanged. The owner authored Tasks itself, with no successful internal planner
dispatch. Thus the terminal hand-back does not exercise FT-0065's affected
waiting branch. No same-owner continuation was used to collect unfinished Plan
work; the later continuation instead changed that owner's workflow to
implementation, which is the separate failure below.

Drive's actual Plan dispatch has `fork_turns: all`. After Plan returned, Drive
sent `followup_task` to that same `plan_owner` for `sb-implement`, contrary to
the new ordinary fresh owner required by Decisions 0192/0221 and `sb-drive`.
The later validation receiver is newly created, but its actual call also has
`fork_turns: all`. These observations establish FT-0073 independently of the
driver's final narrative, which calls that validation receiver fresh. A new
agent name does not establish conversation isolation. All six contexts followed
the fixture's English response language; the prior host-language contamination
was not observed in this retry.

The implementation owner obtained an independent blocker reassessment, reopened
Task 1 through the CLI, dispatched an implementer and independent reviewer, and
committed `696235acef202e2f2d14ba83b68a485da98f8ffa`. Final validation accepted
completion against that implementation revision and checkpointed only completion
metadata at `d31bbb5a47a1f4cebeacebd4653578806e6faebd`. Independent judgment
reran the canonical runner: eight tests pass. CLI status reports Task 1/1,
`release_ready`, every gate and Contract Review fresh, consistent state and clean
worktree. The milestone is `release_pending` only for `RELEASE_VERSION_UNBOUND`.
Baseline hashes changed only for `src/cart.py`, `tasks.yaml` and `spec.yaml`;
the runner/tests are new. Earlier gate evidence is unchanged. The seven-entry
instrumentation log includes the reused owner twice; it is not a count of
independent agents. No release or push ran. After debrief, HEAD, all tracked and
unignored file hashes, the instrumentation log and clean Git status were
unchanged, with no new files.

Session evidence:

- A2 driver: `01a15c9a-897e-7d53-b6f2-f1a5f0aea1c5`
- A2 readers: `01a15c9a-f4a4-7362-a48f-d2dc8d37f99d`,
  `01a15c9b-1191-7971-b0d2-5a84b5ef79be`
- I3r driver: `01a15c9d-29d4-7e90-9beb-b5725650562b`; reviewer:
  `01a15c9e-51a4-7652-a93d-37efe829e6f7`
- DR6 driver: `01a15ca2-4895-7913-b566-3d35780e3d52`; Plan/implementation owner:
  `01a15ca2-c356-7902-8338-32db7e680b4a`
- DR6 assessor, implementer and reviewer: `01a15ca4-e852-7530-9d88-ef8a9657ccb3`,
  `01a15ca6-08df-7093-9e86-68b6a600e532`, `01a15ca7-9099-75c3-974d-ad96245c3749`
- DR6 validation owner: `01a15ca9-2a3b-73a2-a6fa-0b942ab169b1`

This is app evidence from actual installed Skill reads, not default-model CLI
or platform Skill-registry selection coverage. The blocked CLI/Claude attempts
remain in their original records.

## Debrief dispositions

Debriefs followed mechanical judgment, prohibited tools/writes, and were checked
against fixture state afterward.

| Scenario | Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- | --- |
| A2 | Whether absent local tests require additional executable acceptance evidence | extra-step | discarded | Inline checks were read-only and explicitly separated from the absent test suite; no violated product obligation was reproduced |
| I3r | Review-mode wording explicitly names flags/defaults but not natural-language requests for independent review | ambiguity | deferred observation, run record only | The explicit request correctly produced required independent review. No wrong action occurred; seek recurrence or a concrete conflicting interpretation before changing the contract |
| DR6 | Reused the completed planning receiver for implementation despite the fresh-owner requirement | wrong-action-risk | retained as FT-0073 | Actual `followup_task` and inherited-history dispatch arguments independently reproduce the violation; loading a different Skill does not make the receiver fresh |

## Other verification

The product fixes passed the full Rust verification set before these runs.
The fixture isolation change passed shell syntax checking and all three fresh
scenario preparations verified that the project instruction boundary survived
installation. Full nested responses are inspected before crediting a result.
Final records passed strict MkDocs build, Decision-index validation and
`git diff --check`. Dashboard totals were recounted from the authoritative rows:
four open findings, four fixed pending confirmation, nine environment limitations.

## Cleanup

After judgment and read-only debrief checks, removed the dedicated root
`C:/Users/hurui/AppData/Local/Temp/sb-ft-20261021-3169dcd`, including all original
and retry fixtures, baseline snapshots and temporary harness logs. Confirmed
that the directory no longer exists. Evidence above and Codex session identifiers
are retained; no consumer fixture files were copied into the main worktree.
