# Forward-test run: 2026-09-08 / Codex / 4577078

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-08`
- Driver: Codex
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context with no prior turns
- Tested build: `4577078`
- Fixture language: `en`
- Scenarios: `U4`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `U4` | `pass` | `none` | Simulated version `1.3.0`; `latest` retained; only the original unstaged `src/cart.py` edit remains. | Commit `f6a2fab` contains only `mise.lock`; `d793294` contains only the two refreshed update references. The `src/cart.py` diff remains exactly the fixture's `# unrelated maintainer work` addition, `git stash list` is empty, both old-package markers are absent, and the final exact fixture dry run reports 0 create, 0 replace, 114 keep, and 0 remove. | `none` |

## Confirmation turns

None. The explicit update request supplied the update authority; the existing
unrelated edit did not overlap a selection or refresh target.

## Debrief dispositions

The driver reported `none`. The fixture remained dirty only at `src/cart.py`
before and after the read-only debrief.

## Cleanup

- Fixture path removed: `tools/specbind/target/forward-tests/u4-4577078`
- Main worktree after recording: this run record and dashboard update only
