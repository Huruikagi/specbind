# Forward-test run: 2026-09-11 / Codex / 7d98a5d + shared path

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-11`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, no inherited conversation
- Tested build: `7d98a5d` + shared-Contract path working tree
- Binary SHA-256: `3eea0cd69223b71b4348b23fb0bb0ef7ab29882045fce9fd4bdda50ee29c5d77`
- Fixture language: `en`
- Scenarios: `SH1-path`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| SH1-path | pass | none | Shared agreement prepared and checkpointed; Direct pending; review absent; catalogs unchanged; tree clean at `770a330` | `.specbind/specs/shared-contract.yaml` exists and the old `.specbind/shared-contract.yaml` does not; `contract owners locales/en.json` returned `shared-contract#resources/translations`; `spec list` found only the existing cart Spec; commit stat contains only the new shared file | none |

## Confirmation turns

The request stopped after shared proposal preparation, before Contract Review
or catalog implementation. No confirmation turn was needed within that scope.

## Debrief dispositions

The fixture was clean before and after the read-only debrief.

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| SH1-path | A malformed `find -exec` command required rereading the two catalogs separately | extra-step | discarded | Environment command error; it caused no mutation and exposed no product ambiguity |
| SH1-path | Contract validation was repeated after the checkpoint | extra-step | discarded | Harmless extra verification; the required post-authoring result was unchanged |
| SH1-path | Driver narration used Japanese in an English fixture | cosmetic | discarded for this measurement | This scenario measures artifact placement and lifecycle state; all claims were independently read from the fixture |

## Cleanup

- Fixture paths removed: `tools/specbind/target/forward-sh1-path`
- Main worktree after recording: only the shared-Contract path implementation and this run record remain
