# Forward-test run: 2026-10-02 / Claude Code / 49c8da0

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-02`
- Driver: `Claude Code` 2.1.286 (headless top-level session through `forward-test-drive.sh`)
- Model: `claude-sonnet-5-5`
- Driver profile: `--effort medium`; `acceptEdits` with `Bash` and `Skill` allowed; project and local settings only; auto memory off; `HOME` pointed at an empty directory for every credited run (see ENV-0006)
- Tested build: `49c8da0`
- Fixture language: `en`
- Scenarios: `VI1`, `RL3`, `G1`, `I1` (instrumented), `I7` (instrumented), `DR1` (instrumented), `DR2` (instrumented), `DR6` (instrumented), `A2` (instrumented), `Q4` (instrumented)

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VI1` attempt 1 | `environment_invalid` | The session's shell resolved `specbind 0.1.0` from the maintainer's mise shims instead of the fixture CLI | Unchanged and clean at the recipe `HEAD` | The first command returned `unrecognized subcommand 'rule'`; the foreign binary reported `design=stale` and `COMPLETION_SPEC_GATE_STALE`, while the fixture CLI at the same revision reported every gate fresh and `SPEC_COMPLETION_PREFLIGHT_READY`. The harness environment check did not detect it | ENV-0006 |
| `VI1` | `pass` | `none` | `cart` at `release_ready`, `completion=fresh`; one new commit `40b842c` containing only `spec.yaml`; clean | Registry selected `sb-validate-implementation`. `adapter read validation --for consume` → `NO_CHANGE ADAPTER_SCAFFOLD`. Recorded `mechanical_checks`: `scripts/test.sh`, exit 0, which exists and ran. `implementation_revision` `0586bab…` equals the validated `HEAD`. No dispatch; the small fixture stayed in one context | `none` |
| `RL3` | `pass` | `none` | No active milestone; `cart` `idle`; one new commit `158a8e9`; clean; no tag | Registry selected `sb-release`. `log.md` holds one canonical entry under `## 2026-10-02` with `v1.4.0`, the milestone ID, and the roadmap link; its summary describes the delivered cap. `releases/v1.4.0-roadmap.md` and `releases/v1.4.0-contract-review.md` exist; the Roadmap, Contract Review, Brief, and `tasks.yaml` are gone. The commit holds only those lifecycle paths; no push was attempted | `none`; confirms the release branch of FT-0037 |
| `G1` | `product_failure` | "The skill ran": no Skill was selected. The run answered from its own read-only exploration | Unchanged and clean; `order` still at `requirements` with only `brief.md` and `spec.yaml` | The action digest holds four `Bash` reads and no `Skill:` line. No Requirements artifact, research document, or gate change | FT-0037 |
| `I1` | `pass` | `none` | `cart` in `implementation`, Task 1 completed, `completion=not_reached`; one commit `f1adc9e` with `src/cart.py`, `tests/test_cart.py`, and `tasks.yaml`; clean | Registry selected `sb-implement`. Dispatch log holds three lines: the driver, an implementer, and an independent reviewer. `tasks.yaml` gained only `execution.tasks."1".status: completed` beside the CLI's own reserialization. The cap rejects above 99 and leaves the cart unchanged; five tests pass | `none` |
| `I7` | `scenario_invalid` | "The worktree ends clean": `.specbind/steering/conventions.md` stayed modified | Task 1 completed; commit `13a6d49` holds `src/cart.py`, `scripts/test.sh`, `tests/`, and `tasks.yaml`; the recipe's uncommitted Steering edit remains; milestone status reports `WORKTREE_NOT_CLEAN` | Every other expectation held: the dispatch log records the owning continuation and one fresh reviewer; the reviewer approved from the diff and approved inputs; `tasks complete cart 1` ran only after `APPROVED`; no checkout or later-Spec path changed; four tests pass. The recipe leaves the Verification section it appends to Steering inside the "pending Task's uncommitted diff", and no Task owns that path. The 2026-09-06 Codex run committed it as "checkpoint support"; this run excluded it as work the Task does not own | `none`; see the note below |
| `DR1` | `pass` | `none` | `contributing-guide` completed, `cart-contract-change` pending; two commits (`CONTRIBUTING.md`, then the Roadmap completion line); clean; no release | Registry selected `sb-drive`. Dispatch log holds three lines: the driver and one fresh `sb-implement` owner per Direct item. The cart owner returned the reroute without changing a cart path. `git diff --stat` from the recipe `HEAD` shows only `CONTRIBUTING.md` and one added Roadmap line. Direct review ran `inline`, the documented default | `none` |
| `DR2` | `pass` | `none` | `cart` `release_ready`, both Tasks completed, milestone `release_pending` on `RELEASE_VERSION_UNBOUND`; three commits; clean | Dispatch log holds seven lines: the driver, a fresh `sb-implement` owner, an implementer and a reviewer for each Task, and a fresh `sb-validate-implementation` owner. Drive dispatched no internal role as a handler. Commit `7bfcaa8` records Task 1 only and `51780c1` records Task 2 only. Release did not run | FT-0057 |
| `DR6` | `pass` after one continuation turn | "In this same run": turn 1 ended while the `sb-implement` owner's internal implementer was still running, with Task 1 files untracked and neither Task recorded | `cart` `release_ready`, both Tasks completed, `Delegated gates: tasks (sb-drive)`, milestone `release_pending`; four commits; clean | Turn 1 invalidated and reapproved only Tasks under `sb-drive` (commit `dd57d2b`, `tasks.yaml` and `spec.yaml` only) and asked for no confirmation. Requirements, Design, Contract, and the accepted Contract Review are byte-identical to the recipe. Turn 2, "Continue the run until it reaches its stopping point.", completed Task 1 and Task 2 in separate commits, each after an `APPROVED` review; Task 2's first review rejected, one repair followed, and the second review approved. Validation recorded `sh scripts/test.sh`, six tests. Dispatch log holds eleven lines. The blocked record was removed by the Tasks rewind rather than relabeled | FT-0057 |
| `A2` | `product_failure` | "Fresh readers map the repository before the driver synthesizes" and "the agent log records the driver plus at least two fresh readers": the log holds one line | Unchanged and clean; no milestone, Spec, Brief, Research, or adoption record | Registry selected `sb-adopt`; preflight ran first and the proposal's source revision is the fixture `HEAD` `6304d2a…`. The proposal is single and complete: reverse mode, `v1.0.0`, both responsibilities, the dependency, deferred unknowns, the `Bookshp` suspected defect, the excluded area, and the continuation through finalization without Tasks or release. The reply states that it read the two modules itself and dispatched no reader | FT-0018 |
| `Q4` | `pass` | `none` | `cart` in `implementation`; all three gates fresh; `Delegated gates: requirements (sb-plan), design (sb-plan), tasks (sb-plan)`; Contract Review fresh; four commits; clean; no implementation path changed | Turn 1 named the milestone, the item, and the three gates and stopped with every gate `not_reached` and a clean worktree. Turn 2 took exactly that one confirmation. Dispatch log holds seven lines: the driver, Requirements, Design authoring, independent validation, revalidation after one revision, one Contract Review, and Tasks. The Contract Review commit `9aa1f35` precedes the Tasks commit `778d563`. `check traceability cart` → `TRACEABILITY_VERIFIED`. This also satisfies Q1's three expectations | `none` |

`I7` needs a maintainer decision rather than a product change. The scenario
says the recipe "leaves the correct cart implementation and tests as the pending
Task's uncommitted diff" and expects a clean final worktree, but the recipe also
leaves a Steering edit no Task owns. One driver committed it and the other did
not, and `RT2` already requires unowned work to be excluded rather than silently
covered. Either the recipe should commit the Steering edit before leaving the
Task diff, or the expectation should name it.

## Confirmation turns

- `Q4` turn 1 presented the delegation and stopped. Turn 2: "I confirm the
  delegation you just presented: accept the requirements, design, and tasks
  gates for cart under sb-plan for this run. Stop after Tasks approval." It was
  gated by `--expect` checks that `requirements=not_reached` and that the
  worktree was clean.
- `DR6` turn 2 was not a confirmation. Turn 1's final reply said the
  implementation was still in progress in the background, so the maintainer
  message was only "Continue the run until it reaches its stopping point." It
  supplied no authority beyond the original request.
- `A2` stopped at its proposal, which is the scenario's boundary. No
  confirmation was sent.

## Debrief dispositions

`git status --short` and `HEAD` were identical before and after every debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `DR2`, `DR6` | A dispatched `sb-implement` owner ended its turn with "waiting for the implementer" instead of a terminal result, three times in DR2 and twice in DR6. Drive could not tell a stalled owner from a slow one; it resumed the owner, then told it to run synchronously (DR2) or dispatched a fresh owner over the partial worktree (DR6) | `wrong-action-risk` | `retained` | FT-0057 |
| `I1`, `I7` | `artifact list` printed `path=specs/cart/requirements.md`; the first `cat specs/cart/...` failed because the path is relative to `.specbind/` | `extra-step` | `retained` | FT-0058; the DR2 and DR6 owners made the same first attempt in their action digests |
| `A2` | The reader requirement is a `MUST` with no threshold, and the run judged two small files not worth a dispatch | `ambiguity` | `retained` | FT-0018 |
| `G1` | The request read as a read-only question, `sb-gap-analysis` "records findings", and the routing line requires an explicit request to compare | `ambiguity` | `retained` | FT-0037 |
| `Q4` | The Requirements dispatch brief did not say `requirements.md` already existed. The receiver overwrote it as a new Spec, the CLI refused twice (`SPEC_REQUIREMENTS_RETIREMENT_UNSUPPORTED`, then a stale selection), and the receiver restored the file from Git and revised instead | `wrong-action-risk` | `retained` | Single observation; the CLI caught it and the final Requirements keep the baseline IDs. Reproduce against the complete-route brief contract before the next Plan batch |
| `I1` | The implementer's brief carried a paraphrase of the Requirements, and the implementer did not open the artifacts itself | `wrong-action-risk` | `retained` | Single observation; the reviewer read the artifacts directly. Check against the dispatch-brief contract of Decision 0109 |
| `Q4` | The Design author put a new test path in the change boundary without checking Contract `file_ownership`; validation returned `CART-D-1` and one revision resolved it | `extra-step` | `discarded` | Independent validation worked as designed |
| `Q4` | A Design-phase note that `python` was missing was labelled deferred, recorded nowhere, and was wrong; the Tasks author found the interpreter | `ambiguity` | `retained` | Single observation; watch for unrecorded deferred findings in Plan handoffs |
| `Q4`, `DR1`, `VI1` | `mode=all_spec` for a request naming one Spec loosely; "in one go" does not authorize gates | `ambiguity` | `discarded` | The run chose correctly; the stop is FT-0054's required boundary |
| `RL3` | The run finalized without a separate confirmation because the empty adapter skips the Publish sections | `ambiguity` | `discarded` | The scenario's explicit release request with a bound version is the authority |
| `RL3` | The log summary was written after reading only the first 40 lines of the Requirements and neither the Design nor Tasks | `wrong-action-risk` | `retained` | Single observation; the written summary is accurate. Watch for recurrence on a larger release |
| `I7` | The round budget already consumed is not visible in any state; the driver treated the reported rejection as round one | `ambiguity` | `retained` | Single observation |
| `DR1` | Status keeps a parked Direct reroute actionable, and Drive creates no durable record of the park | `ambiguity` | `discarded` | Decision 0187 keeps prose classification out of the CLI on purpose |
| `DR1` | A dispatched owner first tried PowerShell for the instrumentation line and was denied, then used Bash | `cosmetic` | `discarded` | Fixture instrumentation, not product surface |
| `DR6` | `milestone status` reported `TASKS_BLOCKED` with no actionable entry and no owner to route to | `ambiguity` | `discarded` | The supplied replan authority selected the owner, as DR6 intends |
| `VI1` | The Skill does not say how to find the canonical command when the Validation adapter is a scaffold | `ambiguity` | `discarded` | The run found the documented project command; single observation |
| `A2` | Whether a missing input guarantee is a deferred or blocking unknown | `wrong-action-risk` | `discarded` | The proposal surfaced it for the maintainer, which is the confirmation's purpose |
| several | LF-to-CRLF warnings on staging; chained commands hiding which one failed; the instrumentation line's cost | `cosmetic` | `discarded` | Host and fixture properties |

## Cleanup

- Fixture paths removed: the scratch `sbft-*` fixtures, their `.drive` turn records, and the empty `sbft-home` directory under the shell's `/tmp`
- Main worktree after recording: only this record, the dashboard, and the findings worklist changed
