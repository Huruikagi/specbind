# Forward-test run: 2026-10-21 / Codex / Grouped completion with declared checks

[Back to the measurement dashboard](../results.md).

- Tested build: `1875a0a` + final Decision 0233 and FT-0075 clarification tree
- Driver: Codex app `gpt-6.1-sol` / `medium`, `fork_turns: none`
- Fixture language: `en`, standalone instructions and dispatch instrumentation
- Release and fixture binary SHA256:
  `14BD8857CF4BEB1986AD647AE0933D836C56F049B2D9DC4D29B5C086B707AFF5`
- Validation Skill SHA256:
  `72CE86AF4059852CAFC1A6F1E4EF9AA6B9D5C3DDF55D06F95451A675DC622B3D`

The [initial measurement](./2026-10-21-codex-1875a0a-completion-batch.md)
retains DR11's seven passing boundaries and DR16's command-selection failure.
This build requires reading declared commands from Steering and automation
before fixing the check set, and preserves the exact inner invocation through
host-shell wrappers. CLI mechanics are unchanged. Canonical headless CLI
ENV-0007 remains; no Claude behavioral pass is claimed.

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR16, first fixture | scenario_invalid | Success fixture lacked direct boundary tests expressly required by its approved Design | Clean original HEAD `63754c6997369ec98929f0cfe20b89445ba0fb55`, no accepted completion or new commit | Owner reads Steering, runs exact `sh scripts/test.sh` with exit 0 and three tests, but returns cart NO-GO for missing direct 98+2 coverage; accepts none. Drive stops. | Harness correction only; no product relaxation |
| DR16, corrected fixture | environment_invalid | Acceptance command resolved a host mise shim rather than the fixture binary | Original clean HEAD `aef9bc97cc685062c6625357e400b98167c62eba`; both implementation, no acceptance or commit | Owner reads Steering, runs exact canonical command with eight passing tests and clean snapshots, and assesses both GO. Native PATH construction fails during acceptance; mise attempts an unrelated download and fails DNS. Owner and Drive stop without retry; explicit fixture-binary status confirms no mutation. | ENV-0006 host executable selection |
| DR16, native-PATH fixture | pass | none | Clean single checkpoint `f4f139a8c84b14280312f125666eca82fb2fdf35`, both completions fresh, milestone release_pending | Isolated ordinary owner reads declared command, executes canonical eight-test suite, calls batch accept once and batch verify after one two-file commit; independent verification confirms unchanged earlier gates and common revision. No release or push. | FT-0075 resolved |

Driver session: `01a15db2-f71b-7561-aeec-42cbe5216f92`; validation owner:
`01a15db3-e4e3-7a01-8775-fc483e3a4a9a`. Source inspection confirms the Design
names direct `add_item` verification including held 98 plus addition 2, while
the original fixture had only a direct 1+2 test and JSON integration rejection.
The successful-run precondition was wrong. The DR16 recipe now supplies direct
new/existing additions, exact cap, cumulative/new over-cap and nonpositive
rejection with unchanged-cart/message checks before committing the fixture.
The canonical suite verifies that preparation. Existing DR9/DR10 recipes and
product acceptance criteria remain unchanged; the first fixture is not repaired
in place or retrospectively counted as passing.

The corrected-fixture driver is `01a15db7-8ab4-71a3-a5eb-ff6e65fa3379`,
owner `01a15db8-1cc8-7d80-ac94-44ac924284b9`. This environment-invalid
measurement is retained separately. The next fresh driver receives a literal
native PowerShell PATH prefix as an environment fact, in addition to the Git
Bash PATH and native shell location; the maintainer request remains unchanged.

The passing driver is `01a15dbb-bd1e-7af3-b552-400d9784b255`, owner
`01a15dbc-480a-7262-b39a-caa78b03093d`. Actual dispatch at `09:13:02.714Z`
uses `fork_turns: none`; two instrumentation entries confirm the separate
contexts. The initial implementation revision is
`ff79a13b4b60994db08e4cf387b2139234dddc73`, the checkpoint's sole parent.
The owner reads `steering read conventions --for consume` before execution and
preserves `sh scripts/test.sh` inside its native Git Bash wrapper. Eight tests
pass with clean before/after snapshots. Both candidates persist the exact
shell command text, including its PATH assignment and status capture.

The trace proves batch preflight, schema read, one batch acceptance with empty
pending, and batch verification with the same external candidate. Independent
fixture-binary verification returns `COMPLETION_BATCH_VERIFIED`; both Spec
status reports show every gate fresh and health consistent. Git shows only
`cart-input/spec.yaml` and `cart/spec.yaml` changed from implementation to
release_ready with completion added; earlier evidence is unchanged. Drive
rereads current status and stops at release_pending with only the unbound
release-version blocker. No new completion confirmation was requested.

## Confirmation and debrief

No human confirmation was requested. The first driver reported ambiguity
between validation's instruction to return findings to implementation and
Drive's restriction to current actionable entries. Its actual stop is correct:
an unchanged attempted validation action is parked; naming a repair owner does
not create a new scheduling authority. This is discarded as a new product
finding after comparison with those explicit boundaries. Before/after debrief
status remained clean at the original HEAD; no commands or changes occurred
during the debrief.

The corrected-fixture driver reported host CLI shadowing and native `sh`
resolution friction. Both are environment setup observations, not adapter or
product changes: the canonical command passes under the provided native Git
Bash, and explicit fixture-binary reads work. They are recorded under ENV-0006
without a duplicate product finding. Its debrief was also read-only, confirmed
by unchanged clean status.

The passing driver reported `none`. Independent Git status was clean before
and after its read-only debrief.

## Verification and cleanup

Final formatting, schema checks, all-target/all-feature Clippy, all 577 Rust
tests and release build passed. The 233-Decision index and strict bilingual
documentation build also passed. Both rendered agent packages retain the new
batch surface; only Codex app behavior was measured here. The paired English
and Japanese CLI reference pages were updated and their reciprocal language
links checked.

After judgment and read-only debriefs, removed the three dedicated fixtures:

- `C:\Users\hurui\AppData\Local\Temp\sb-batch-dr16-final-20261021`
- `C:\Users\hurui\AppData\Local\Temp\sb-batch-dr16-ready-20261021`
- `C:\Users\hurui\AppData\Local\Temp\sb-batch-dr16-native-20261021`

Their setup logs and the passing external candidate were also removed. Cleanup
checked every resolved path as a direct child of the native temporary directory
before deletion. Session IDs and all unsuccessful attempts remain recorded.
