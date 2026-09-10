# Forward-test run: 2026-09-10 / Codex / 144adf8 + final description checks

[Back to the measurement dashboard](../results.md).

- Driver: `Codex`, `gpt-5.6-terra`, `medium`, fresh context
- Tested build: `144adf8` + final description/migration tree and artifact creation check
- Binary SHA-256: `8ec52a7df1d928239a9773a246d3dc39aa324f417a9db85941e4b4f84cd34f32`
- Fixture language: `en`
- Scenario: `MG2`, prepared with the `ds9` recipe

## Measurements

| Scenario | Verdict | Expectation | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| MG2 | pass | Agreed one-off gets its own description; main inherits its template | Three untracked files: main Design, runtime-operations Design, Contract; approval pending | Both artifact checks succeed; traceability covers `1.1`; Contract graph has no ownership fault or cycle; settings, Requirements, and Spec-state diff empty | None |

The parent independently ran artifact inventory, both explicit scaffold checks,
traceability, Contract checks, and Git comparison after the driver finished.
Main describes component responsibilities, interactions, data boundaries, and
implementation decisions using the exact template sentence. The one-off describes
deployment configuration, Secret handling, health signaling, and runtime recovery.
The Contract warning concerns the fixture's pre-existing unconsumed cart export.

No lifecycle approval, Tasks, implementation, or fixture repair occurred. This
measures the already-assessed one-off path; it does not resolve generic DS9's
decomposition finding FT-0053.

## Read-only debrief

The driver confirmed reading the description and artifact-check guidance before
presenting the draft. It reported these recoverable authoring faults; no files
changed during the debrief.

| Observation | Impact | Disposition |
| --- | --- | --- |
| First scaffold checks rejected omitted durable instructions; driver restored the literal instruction set and rechecked both documents successfully | Bounded repair | Expected mechanical creation guard; no bypass or parent fixture repair |
| First traceability used `1` instead of canonical `1.1`; driver corrected Front Matter and markers before presentation | Bounded repair | Existing traceability contract enforced |
| Pre-authoring Contract lookup reported the not-yet-created infrastructure Contract unavailable | Diagnostic | Authoring the scoped Contract resolved the precondition; final graph check passed |

## Product verification

The same final source passed 497 Windows Rust tests, formatting, generated-schema
consistency, all-target/all-feature Clippy, release build, 211-Decision index
validation, and strict English/Japanese documentation build. No release was made.

## Cleanup

Fixture: `G:/specbind/target/forward-mg2-check`. Removed after evidence and read-only debrief capture.
