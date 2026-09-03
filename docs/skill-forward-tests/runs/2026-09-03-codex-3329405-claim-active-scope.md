# Forward-test run: 2026-09-03 / Codex / 3329405

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `3329405`
- Fixture language: `en`
- Scenarios: `VC1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VC1` (first attempt) | `environment_blocked` | The product workflow never began. | `HEAD` `af933e6b214a`; clean and unchanged | The host safety layer refused fixture instruction and CLI execution before the Skill ran | ENV-0004 |
| `VC1` (fresh retry) | `product_failure` | The terminal report omitted the required explicit `VERIFIED` verdict. | `HEAD` `0145283026e7`; clean; cart remained `implementation`, `completion=not_reached` | The driver did use `Active requirement set: 1.1, 1.2, 1.3, 1.4`, passed the 4-test suite and runtime check, and made no mutation, confirming FT-0040's branch; its report-only framing suppressed the verdict block | none; not reproduced on `489d306` |

## Confirmation turns

None. The blocked attempt was rebuilt at a new path before retrying.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `VC1` | The driver used the exact active set rather than baseline Requirement 2. | ambiguity | discarded | The new instruction resolved the intended scope; this lookup is required evidence, not product friction. |
| `VC1` | The report omitted the mandated verdict form. | ambiguity | discarded | Single driver-framing occurrence, not reproduced by the next fresh driver. |

## Cleanup

- Fixture paths removed: `/tmp/sb-vc1d-0187`, `/tmp/sb-vc1e-0187`
- Main worktree after recording: checked separately before the record commit
