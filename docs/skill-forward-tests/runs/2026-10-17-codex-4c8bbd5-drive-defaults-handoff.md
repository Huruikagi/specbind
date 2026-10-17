# Forward-test run: 2026-10-17 / Codex / initial configured recovery handoff

[Back to the dashboard](../results.md).

- Build: `4c8bbd5` plus Decision 0228 and validated-read correction
- Driver and reviewer/debugger roles: `gpt-5.6-terra` / `medium`, explicit compatibility comparison
- Fresh `dr15c` fixture with dispatch instrumentation and standalone English instructions
- Binary SHA-256: `1fc65366b140e6e87fe24143b29777f8d84aba481cede00f32821ee54d37ff79`
- Controller: `01a149af-5695-7a62-9b8f-02362e5ec0a3`
- Complete implementation owner: `01a149af-fdb6-71d1-8675-87224f9b3e9d`

## Measurement

| Scenario | Verdict | Expectation that did not hold | Fixture state | Evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DR15 | `fail` | New configured progress was replaced by the historical bounded mode; no-scope-expansion was also misread as disabling in-scope replan | Original implementation/test diff preserved; Task newly blocked; HEAD unchanged at `ac23487`; no checkpoint | Controller read configuration but carried bounded/three used to the actual owner, which refused review four and ran `tasks block` | FT-0069 reopened |

The driver had enough evidence: a new Drive invocation, configured true/progress,
three used cycles and an explicitly historical bounded selection. It incorrectly
treated the historical mode as a current user override. No renewal was requested,
but applying a new mode did not require renewal. The implementation owner also
retained the supplied incorrect bounded selection, making this a real transport
failure rather than a missing assertion in the final report.

The correction separates current-request choices from historical handoff fields:
no new allowance preserves counts, and no scope expansion preserves the existing
scope boundary. Neither changes the configured mode or disables in-scope replan.
The fresh rerun retains the exact same prompt.

Four canonical tests and diff checks passed in the failed run; that does not
override the missing fourth review and Task checkpoint. No source was committed,
no planning artifacts were changed and no whole-Spec validation or Release ran.

## Debrief dispositions

The same-session debrief made zero tool calls and preserved the blocked Task,
original partial work and unchanged HEAD.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Historical bounded and no-new-allowance were treated as a current mode override | wrong-action-risk | retained | The measured FT-0069 failure |
| No-scope-expansion was treated as disabling configured replan | wrong-action-risk | retained | Same provenance error; bounded in-scope replan does not expand scope |
| Complete owner dispatch seemed extra for a review-only correction | extra-step | discarded | Required ownership of review, progress and checkpoint; direct reviewer dispatch would omit the outer cycle |
