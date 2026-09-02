# Forward-test run: 2026-09-02 / Codex / 61d0d47

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-02`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`
- Tested build: `61d0d47`
- Fixture language: `en`
- Scenarios: `A2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| A2 | `product_failure` | The driver inspected both evidence lines itself instead of dispatching at least two fresh readers. | Setup revision `8a8b914bd8550b0cd4fe33418c7415cdfd4de3c6`; 0 Specs; no adoption, milestone, Brief, Research, or Roadmap artifact; clean tracked worktree. | `specbind spec list` reported 0; `.forward-test/agents.log` contained only the driver line. The proposal otherwise named reverse mode, the revision, `v1.0.0`, both responsibilities and their seam, classifications, exclusions, and the post-confirmation flow. | FT-0023 |

The first driver was environment-blocked by fixture instrumentation under
ENV-0004. A fresh retry received the same user-authorized fixture-local logging
fact in its initial prompt and produced the measurable product result above.

## Confirmation turns

The driver stopped at the single reverse-scope proposal and requested explicit
confirmation. No confirmation was supplied.

## Debrief dispositions

No debrief was taken because the product failure was fixed and remeasured on a
new build.

## Cleanup

- Fixture paths removed: `/tmp/sb-a2-61d0d47`
- Main worktree after recording: only the later forward-test records and dashboard edits
