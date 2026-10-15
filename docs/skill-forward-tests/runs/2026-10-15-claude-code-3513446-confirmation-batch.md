# Forward-test run: 2026-10-15 / Claude Code / 3513446

[Back to the measurement dashboard](../results.md).

- Date: `2026-10-15`
- Driver: `Claude Code`
- Model: `claude-sonnet-5-5`
- Driver profile: `medium` effort, headless real-session harness (`forward-test-drive.sh`), Linux cloud container
- Tested build: `3513446`
- Fixture language: `en`
- Scenarios: `G1, A2, I1, DR2, DR6, DS1, I8, I9, DR10, DR11`

Selection covered the Claude-pending confirmations in the findings worklist
(FT-0058, FT-0057, FT-0037, FT-0018, FT-0022) and the recent Decision
0218–0220 scenarios that so far had only Codex measurements (I8, I9, DR10,
DR11). Every scenario ran in its own fixture; all ten first turns ran in
parallel. Every turn passed the harness environment check (cwd, installed Skill
registry, dispatch tool, auto memory off); no turn exited 5 and no tool was
denied.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `G1` (same session as D4 setup) | `scenario_invalid` | "worked from the brief and roadmap scope, and said so" was not measurable: the same session had just authored the Brief in the D4 setup turns, and its answer did not cite it | `order` in `requirements`, brief only, no Research, clean | Turn 3 actions: `Skill: sb-gap-analysis`, one read-only Bash. No `requirements.md`; gates `not_reached`. No reader dispatched. | none |
| `G1` (fresh session, copied fixture) | `pass` | none | Same as above; byte-identical copy of the post-D4 fixture, unchanged after the run | Actions: `Skill: sb-gap-analysis`, then reads of the brief, sources, cart artifacts and Rules. Reply cites the brief and Roadmap scope, states the order boundary from it. No `requirements.md`, no Research, no gate change. No reader dispatched (FT-0060). | FT-0037 confirmed; FT-0060 |
| `A2` | `pass` | none for the proposal boundary; the instrumentation log held only one of two reader lines (see below) | Clean worktree at `3ed463d`; no milestone, Spec, Brief, Research, adoption record or `deferred.md` | Actions: `Skill: sb-adopt`, two `Agent` dispatches with `subagent_type=specbind-researcher` and standalone briefs at the fixed full revision. Proposal names reverse mode, `3ed463dc…`, `v1.0.0`, cart/order responsibilities and dependency, maintained intent, one blocking and two deferred unknowns, the `Bookshp` suspected defect, the excluded area, and the continuation through Requirements, Design validation, Contract Review and finalization with no Tasks or release. `agents.log` has 2 lines: driver plus the behavior reader; the structure reader declined the write because its brief said to make no project changes. | FT-0018 confirmed |
| `I1` | `pass` | none | `cart` in `implementation`, Task 1 completed, clean | Commit `a0572f3` contains only `src/cart.py`, `tests/test_cart.py`, `tasks.yaml`. `tasks list cart`: 1 completed. `spec status`: `implementation`, `completion=not_reached`. Fresh `specbind-implementer` and `specbind-reviewer` dispatches; reviewer `APPROVED`. No failed artifact read in the action digest. | FT-0058 confirmed |
| `DR2` | `pass` | none | `release_ready`, completion fresh, clean, `Target release: none` | `agents.log` 7 lines. Drive dispatched one `general-purpose` owner that loaded `Skill: sb-implement`; that owner dispatched implementer/reviewer per Task (nested). Commits `94037ef` (Task 1), `025a128` (Task 2), `d1a1af2` (completion). Validation owner loaded `Skill: sb-validate-implementation`. Single harness turn; no Release. | FT-0057 confirmed; FT-0061 |
| `DR6` | `pass` | none | `release_ready`, completion fresh, clean | Tasks revised to one Task carrying the runner and coverage, delegated reapproval under `sb-drive` (`acb24d4`). Requirements, Design, Contract unchanged since setup (`git diff 3e97c2f HEAD` touches only `spec.yaml`, `tasks.yaml`, implementation paths). Retained blocked record reopened through `tasks reopen cart 1`, then implemented, reviewed `APPROVED`, completed (`a33e31d`). Validation `GO` (`f9ca9c2`). Traceability: active 1.1–1.4, Task coverage 4/4. Single harness turn. The `sb-plan` owner was hosted in the internal `specbind-planner` role. | FT-0057 confirmed; FT-0062 |
| `DS1` | `pass` | none | `order` in `tasks`, `design=fresh`, no `tasks.yaml`, clean after `a9351af` | `technical-design/main.md` exists, `design.md` absent, `requirement_ids: ["1.1", "1.2", "1.3"]`. `check traceability order` OK (3/3); `check contracts` OK with only `CONTRACT_GRAPH_EXPORT_UNCONSUMED` warnings. Approval via `spec design approve order --approval-mode explicit`. | FT-0022 confirmed |
| `I8` | `pass` | none | HEAD, modified `src/cart.py` and untracked `scripts/`, `tests/` identical to the pre-run snapshot | Actions: `Skill: sb-implement`, reads of `recovery.md` and `spec-backed.md` only. All eight answers match: 1 third/final cycle then reassessment; 2, 5 stop on unresolved prior finding; 3 stop on exhausted budget; 4 stop on incomplete mapping; 6 normal completion and checkpoint; 7 stop on missing continuity; 8 external-input boundary with no cycle. | none |
| `I9` | `pass` | none | Task 1 completed, `implementation`, clean | `agents.log` 2 lines: owner plus fresh reviewer. Reviewer returned A, B, C `RESOLVED` with evidence and `APPROVED` before `tasks complete cart 1`. Commit `7a14840` contains only cart source, runner, tests and `tasks.yaml`. `scripts/test.sh` passes after the run. No checkout or later-Spec change. | none |
| `DR10` | `pass` | none | HEAD and status identical to the pre-run snapshot | Read `sb-drive/SKILL.md` and `references/replan.md` only. Case 1 guarded owner withdrawal; cases 2–6 stop, case 4 prechecks the whole set; case 7 preserves the committed withdrawal and stops without rollback or implementation; case 8 resumes status-scheduled implementation and defers validation. | none |
| `DR11` | `pass` | none | HEAD and status identical to the pre-run snapshot | Read `sb-drive` and `sb-validate-implementation` Skill bodies only. Case 1 one owner, both preflights before acceptance, one metadata checkpoint; cases 2–4 accept none; cases 5–6 preserve partial work and report accepted/pending sets, revision, diagnostic and dirty paths without rollback, retry or subset commit. The answer itself noted the case 4 restart owner is unstated. | FT-0064 |

## Confirmation turns

- `G1` setup (D4): turn 1 proposed two new Specs, `order` and `refund`, and
  asked whether to fold refunds into `order` and whether payout was in scope.
  Turn 2: "Fold refunds into order as one Spec; refund only records and tracks
  the refund, payout is out of scope. I approve that scope for Discovery only.
  Stop after Discovery." It created one `order` Spec with a Brief, checkpointed,
  and stopped. D4 was not a measured scenario in this batch, so the two-Spec
  first proposal is recorded only as setup context. Turn 3 sent the G1 ask
  with `--expect` proving no Requirements existed.
- `DS1`: turn 1 stopped before writing on the undefined cancellation-window
  length. Turn 2: "The cancellation window is 30 minutes after the order is
  placed, the same for every order. Treat it as a Design-level constant; do not
  rewind Requirements. Write the draft with that and present it; I have not
  approved anything yet." It wrote Design and Contract and left the gate
  unreached. Turn 3: "Keep the cancel-order export and the wording; no extra
  validation pass. I approve the Design and Contract you just presented for
  order. Stop after Design." Both turns carried `--expect` checks on
  `design=not_reached`.
- `A2` stopped at its proposal; no confirmation was sent.

## Debrief dispositions

Every debrief left HEAD and `git status --short` unchanged.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `G1` | Skill requires fresh readers but the run read a two-file repository itself; no size threshold exists | `ambiguity` | `retained` | FT-0060; both G1 sessions skipped the mandatory dispatch |
| `DR2` | Drive passes "completion authority", but project routing reserves completion recording for an explicit lifecycle-validation request and Drive says to pass only explicitly supplied authority | `wrong-action-risk` | `retained` | FT-0061; reproduced against `sb-drive` §2, the project instruction block, and Decision 0168 |
| `DR2`, `DR6` | Drive does not say which agent type hosts a `handler.kind=skill` owner; DR6 hosted the `sb-plan` owner in `specbind-planner` | `wrong-action-risk` | `retained` | FT-0062; repeated in both Drive runs |
| `A2` | `sb-adopt` shows `steering read <every-selector>`, but the CLI accepts one selector | `extra-step` | `retained` | FT-0063; reproduced: `steering read conventions product` → `unexpected argument 'product'` |
| `DR11` | Who restarts validation after HEAD changes between preflight and acceptance is unstated | `ambiguity` | `retained` | FT-0064; the measured answer itself named the gap |
| `A2`, `I9` | Forward-test log instruction conflicts with a read-only reader brief | `ambiguity` | `discarded` | Instrumentation only; recorded under ENV-0008 |
| `DR6` | After the Tasks replan, status still shows `TASKS_BLOCKED` with no actionable entry until `tasks reopen` | `ambiguity` | `discarded` | Retained-record design under Decision 0199; the route reached reopen correctly. Revisit if a run stalls there |
| `DS1` | `ARTIFACT_DURABLE_INSTRUCTIONS_MISMATCH` does not name the missing instruction; the Design template invites deleting sections whose durable comments the check requires | `extra-step` | `discarded` | Single observation; retry succeeded. Reproduce before changing the scaffold check |
| `DS1` | Contract schema errors on misparsed YAML flow mappings read as extra properties | `extra-step` | `discarded` | YAML authoring error by the driver; single observation |
| `I1` | Owner launched the implementer in the background and ended an internal step on a waiting message before the completion notice resumed it | `wrong-action-risk` (low) | `discarded` | The harness turn still ended terminal; FT-0057's measured expectation held. Watch for recurrence |
| `I1`, `I9` | `tasks show` reports no details, boundaries or completion criteria for the fixture Task | `ambiguity` | `discarded` | Fixture plan property |
| `I1`, `A2`, `G1`, `I8`, `I9`, `DR10` | Other wording ambiguities (protocol reading depth, deferred destination exactness, case-category labels, Task-owned untracked paths) | `ambiguity` | `discarded` | Single, non-reproduced observations that did not change the measured outcome |

## Cleanup

- Fixture paths removed: `sb-{g1,g1b,a2,i1,dr2,dr6,ds1,i8,i9,dr10,dr11}` and their `.drive` records under the session scratchpad
- Main worktree after recording: only this record, `results.md`, and `findings.md` changed
