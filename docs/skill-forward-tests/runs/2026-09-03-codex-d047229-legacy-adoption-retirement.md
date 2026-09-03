# Forward-test run: 2026-09-03 / Codex / d047229

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `d047229`
- Fixture language: `en`
- Scenarios: `A1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A1` | `pass` | none | `HEAD` `246343d780a0`; clean; no milestone, Spec, or temporary adoption record | Installed Discovery references were exactly `github-milestone.md`, `local-files.md`, and `reverse.md`; the driver ran `specbind adoption preflight`, which returned `ADOPTION_STEERING_REQUIRED`; an independent fixture read confirmed the same diagnostic and no writes | none |

## Confirmation turns

None. The missing committed Steering baseline is an intentional stop before any
confirmation or mutation.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A1` | The driver looked for a CLI continuation after the diagnostic routed the maintainer to `sb-steering` bootstrap. | wrong-action-risk | discarded | A1 intentionally stops at the cross-Skill maintainer checkpoint; the driver still selected the reverse route, ran preflight first, and made no change. |

The fixture was clean both before and after the read-only debrief.

## Cleanup

- Fixture paths removed: `/tmp/sb-a1-d047229`
- Main worktree after recording: checked separately before the record commit
