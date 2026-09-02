# Forward-test run: 2026-09-03 / Codex / 9dadf34

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: Codex
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context with no prior turns
- Tested build: `9dadf34`
- Fixture language: `en`
- Scenarios: `U1`, `U2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `U1` | `pass` | `none` | HEAD unchanged; clean worktree; no project mise configuration. | The driver discovered `sb-configure`, recorded the binary and repository preflight, inspected mise ownership, and stopped before mutation because `config_source.path` named the user-global configuration outside the fixture. Direct Git reads confirmed the setup commit and clean state. | `FT-0025` confirmed |
| `U2` | `environment_invalid` | The PATH fact was masked by an inherited PowerShell function and global `specbind`, so the preview came from a different binary than the apply. | Simulated version `1.3.0`; `latest` retained; clean worktree after two local commits. | The debrief reported a four-replacement preview but a two-replacement apply. A direct unqualified preview reproduced the four stale global-product replacements, while the exact fixture executable reported 108 keeps after the run. | `none` |
| `U2` retry | `pass` | `none` | Simulated version `1.3.0`; `latest` retained; clean worktree after two local commits. | With both fixture-native applications fixed as environment facts, the preview and apply each contained exactly the two installed update references. Commit `d60754d` contained only `mise.lock`; commit `75b1e33` contained only both update references. The final exact fixture dry run reported 108 keeps and no create, replace, or remove. | `FT-0026` confirmed |

## Confirmation turns

None. U1 stopped at failed project ownership proof. Both U2 attempts had complete
update authority; the first was retained only to document host command-resolution
contamination before the clean retry.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `U1` | The fixture executable and user-global mise configuration disagreed. | `extra-step` | `discarded` | This was the scenario's intended refusal condition; it correctly prevented global mutation. |
| `U2` | An inherited PowerShell function and global binary overrode the fixture PATH fact. | `wrong-action-risk` | `discarded` | Environment-invalid attempt; the exact fixture-native retry removed the contamination. |
| `U2` | The preview and apply differed when different executables were resolved. | `wrong-action-risk` | `discarded` | Environment-invalid consequence reproduced with the global binary; the exact fixture retry previewed and applied the same two paths. |
| `U2` retry | The update procedure and Git adapter separated ownership, binary selection, asset refresh, reload, and checkpoints without ambiguity. | `extra-step` | `none` | No product friction; confirms `FT-0026`. |
| `U2` retry | The controlled fixture did not implement the unprescribed `mise --version` command. | `extra-step` | `discarded` | Fixture-only probe; every command prescribed by the update procedure succeeded. |

Every debrief left its already judged fixture unchanged.

## Cleanup

- Fixture paths removed: `u1-9dadf34`, `u2-9dadf34`, and
  `u2-9dadf34-native` under the repository-local `target/forward-tests/`
- Main worktree after recording: only this run record, dashboard, worklist, and
  prior run cleanup evidence
