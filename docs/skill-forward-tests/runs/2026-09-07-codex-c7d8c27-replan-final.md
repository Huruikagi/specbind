# Forward-test run: 2026-09-07 / Codex / c7d8c27 replan final

[Back to the measurement dashboard](../results.md).

- Driver: Codex, `gpt-5.6-terra`, `medium`, fresh context per scenario
- Tested build: `c7d8c27` + Decision 0199 working tree with resolved-blocker handling
- Binary SHA256: `96D3B6B15A3143FA4D0ACF52C17D7A3EF2FF6CCB2186300A8556F19BF8842DC8`
- Fixture language: `en`
- Scenarios: DR4, DR6

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR4 original reporting expectation | `scenario_invalid` | The scenario draft required the full rewind cost in ordinary Drive attention before the owning planning phase proposed any rewind. | Unchanged fixture. | Decision 0168 requires owner, cause, and the decision needed to resume; the planning phase owns the detailed pre-invalidation confirmation. The driver reported the Design owner and missing replan authority. | none |
| DR4 corrected authority expectation | `pass` | none | All files and gates unchanged, one pending Task, no delegated gates. | `spec status cart`: requirements/design/tasks fresh, completion not reached. `git diff --exit-code 6602321` and `git status --short`: empty. The driver explicitly identified absent replan authority and did not mutate. | none |
| DR6 first validation dispatch | `environment_blocked` | The receiver could not resolve bare `sh` despite the fixture's supplied native Git Bash path. | Replanned Task implemented, reviewed, and completed; no completion evidence yet, clean tree. | Canonical runner was not replaced by a different check. The receiver returned `MANUAL_VERIFY_REQUIRED`; Drive corrected the environment for the bounded retry. | none |
| DR6 complete Drive run | `pass` | none after the environment retry | Spec `release_ready`, Milestone `release_pending`, all gates fresh, only Tasks delegated to `sb-drive`, one completed Task, clean tree. | `d055101` replans Tasks; `00a9510` implements and completes; `8d70bd0` changes only completion metadata. Native Git Bash running `scripts/test.sh`: 5 passed. Diff of all upstream artifacts/review/scope from `db9e48e`: empty. | none |

The DR4 reporting expectation was corrected against the owning contracts,
not by authorizing a forbidden operation or changing the observed result.
No later Gate invalidation occurred; the phase-level cost confirmation remains
required if the maintainer chooses that path.

## Tasks recovery evidence

DR6 starts at `db9e48e` with Task 1 blocked by the unavailable runner that
Task 2 would create. Recovery commit `d055101` changes only the Tasks plan
and its gate metadata, merging the runner/coverage work into the same Task as
the behavior it proves. The old blocked record remains in that approved plan;
the resumed implementation removes it through the CLI before working.
The Requirements, Design, Contract, accepted Contract Review, and Roadmap diff
against `db9e48e` is empty. This measures Tasks-only recovery independently
from DR3's Design/Contract rewind.

No replan, reapproval, completion, or restart confirmation was supplied during
DR6. The same Drive continued through the known-native-shell environment retry
and completion acceptance. The only remaining guarded action is release-version
binding, which the request did not authorize. No release operation ran.

## Debrief dispositions

DR4 was debriefed only after independent state judgment; Git stayed clean.
It reported the explicit replan boundary and the distinction between status
health and semantic correctness. Both are intended behavior. Two initial
read-only shell invocation failures were avoided by using the supplied native
Git Bash executable; this is fixture environment friction, not a product defect.

Some reports and DR6's implementation commit message were Japanese despite the
English fixture. Checkpoint scope and completion were independently judged from
the fixture's Git adapter, commit diffs, and CLI state. Language behavior is not
a passing claim of this run.

DR6's read-only debrief reported the seeded prerequisite cycle, the retained
blocker requiring exact `tasks reopen`, and the shell environment retry. The
first is the scenario's intended defect; the second is explicitly handled by
the new recovery contract; the third is an environment handoff limitation.
No new product finding was established. Git status stayed clean after debrief.

## Cleanup

All fixture directories were removed after judgment and debrief:

- `C:\Users\hurui\AppData\Local\Temp\sb-dr3-replan-0199`
- `C:\Users\hurui\AppData\Local\Temp\sb-dr4-replan-0199`
- `C:\Users\hurui\AppData\Local\Temp\sb-dr4-replan-0199-b`
- `C:\Users\hurui\AppData\Local\Temp\sb-dr5-replan-0199` (prepared, not driven; no measurement claimed)
- `C:\Users\hurui\AppData\Local\Temp\sb-dr6-replan-0199`

Only the Decision 0199 implementation, documentation, and test evidence remained
in the main worktree before the final commit.
