# Forward-test run: 2026-09-10 / Codex / 144adf8 + inventory trial

[Back to the measurement dashboard](../results.md).

- Driver: `Codex`, `gpt-5.6-terra`, `medium`, fresh context
- Tested build: `144adf8` + descriptions and explicit Design inventory trial
- Binary SHA-256: `fa80557fd9631495aae8be9b5fcb7fc7e248e95e45889ec195d921e8e3000a93`
- Fixture language: `en`
- Scenario: `DS9`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| DS9 | product_failure | Required one-off omitted; main description replaced instead of inherited | Only untracked main Design and Contract | Artifact inventory contains only `design/main`; its description differs from the template. Settings, Requirements, and Spec state remain unchanged. | FT-0053 |

The read-only debrief retained the same files. The driver reported reading the
split and description instructions, treating runtime operations as duplicative
of main, and substituting its own main responsibility sentence. Explicit
inventory wording therefore did not resolve the generic decomposition failure.

The unsuccessful decomposition edits from both retries were withdrawn. FT-0053
remains open for reconciling the generic assessment contract and scenario. The
description work adds an explicit scaffold check and is measured separately by
MG2 with an already agreed one-off split. This does not reclassify DS9 as passing.

## Cleanup

Fixture: `G:/specbind/target/forward-ds9-descriptions-inventory`. Removed after evidence capture.
