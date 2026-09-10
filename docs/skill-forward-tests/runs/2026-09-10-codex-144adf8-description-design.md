# Forward-test run: 2026-09-10 / Codex / 144adf8 + descriptions tree

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-10`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context
- Tested build: `144adf8` + Decisions 0210/0211 before the decomposition-condition repair
- Binary SHA-256: `2794b9dd103f036b1d1a10be0aa6ef5c6436b7ff5e5cd98a8b3dfb495e1a112b`
- Fixture language: `en`
- Scenarios: `DS9`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DS9 | product_failure | Independent infrastructure ownership/recovery concern was not materialized as a one-off supplement | Untracked main Design and empty Contract, Requirements fresh, Design not approved | `artifact list infrastructure` has only `design/main`; its description exactly matches the template. Design traceability is 1/1; settings diff empty. | FT-0053 |

The candidate report treated the ability to describe everything in main as a
reason to omit a supplement. The assessment section added a "would hide"
condition, while Decision 0179 and the later Splitting section require the
independent responsibility without that extra escape. The repair aligns the
assessment with the Decision and explicitly excludes size, Requirement count,
and fit-in-main as extra conditions. No fixture was repaired for judgment.

## Confirmation turns

None. The request stopped at the unapproved Design draft.

## Debrief dispositions

Git status was unchanged before and after the read-only debrief: two untracked
Design/Contract files. The driver explained that it recognized the independent
runtime and recovery concern but used headings inside main instead.

| Observation | Impact | Disposition | Reason |
| --- | --- | --- | --- |
| Generic main coverage displaced the required one-off assessment/materialization | wrong-action-risk | retained | FT-0053; inconsistent assessment conditions reproduced against Decision 0179 |
| Driver chose Compose and concrete health interfaces during Design | ambiguity | discarded | Implementation design choices are subject to the ordinary Design review boundary; not evidence of a description contract defect |

Part of the candidate narration was Japanese despite the English fixture.
Language narration is excluded; artifact and decomposition evidence remains
independently observable and is the basis of the failure verdict.

## Cleanup

Fixture: `G:/specbind/target/forward-ds9-descriptions`. Removed after evidence capture.
