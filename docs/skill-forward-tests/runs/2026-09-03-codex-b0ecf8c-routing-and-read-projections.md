# Forward-test run: 2026-09-03 / Codex / b0ecf8c

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-03`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh context, `fork_turns: none`
- Tested build: `b0ecf8c`
- Fixture language: `en`
- Scenarios: `VC1`, `VD1`, `X2`, `A1`, `DR1`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `VC1` (invalid attempt) | `scenario_invalid` | The harness used nonexistent recipe `vc1`, so it prepared the base fixture rather than `vi1`. | `HEAD` `b09d2e3fbdca`; clean; cart idle with no milestone | `milestone status` returned `NO_ACTIVE_MILESTONE`; `spec status cart` reported all gates `not_reached` | none (harness) |
| `VC1` (fresh retry) | `pass` | none | `HEAD` `f7f1c5c81bf3`; clean; cart remained `implementation`, `completion=not_reached` | Driver returned `VERIFIED`; canonical suite passed 4 tests; runtime bounds passed; no completion acceptance ran | FT-0036 confirmation, with routing friction retained for follow-up |
| `VD1` | `pass` | none | `HEAD` `7541cfc32c9f`; clean; Design and Research unchanged; Design gate remained fresh | `check traceability` exposed `Active requirement set: 1.1, 1.2, 1.3, 1.4`; driver returned `NOT_READY` for the Research deferral | FT-0030, FT-0038 confirmed |
| `X2` | `pass` | none | `HEAD` `f267ca737267`; clean; review remained absent; checkout Contract unchanged | `milestone status` reported `State health: inconsistent` and `CONTRACT_GRAPH_TARGET_ENTRY_MISSING` naming checkout's missing target | FT-0032 confirmed |
| `A1` | `pass` | none | `HEAD` `cc134bbb4da5`; clean; no milestone and zero Specs | The only product workflow command was `adoption preflight`; it returned `ADOPTION_STEERING_REQUIRED` before area or version input | FT-0034, FT-0035 confirmed |
| `DR1` | `pass` | none | `HEAD` `c48e6cb273ea`; clean; Direct progress 1/2; cart paths unchanged from `c19f888`; no tag | The cart summary was passed to its implementer and parked `HUMAN_DECISION`/Discovery reroute; independent `CONTRIBUTING.md` was implemented and completed | FT-0039 confirmed |

## Confirmation turns

None. DR1's ordinary Git checkpoints were workflow behavior, not maintainer
approval. The invalid VC1 attempt was rebuilt at a new path before retrying.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `VC1` | With every Task complete, the driver still saw the claim route and lifecycle route as competing, although it selected the consequence-free owner. | wrong-action-risk | retained | FT-0036 needed explicit precedence after routing. |
| `VC1` | The scaffold Validation adapter and a first Git Bash permission failure added interpretation and retry cost. | ambiguity | discarded | The adapter did not govern this consequence-free check; the exact command passed with the required execution permission. |
| `VD1` | The driver dispatched a reader to decide whether the tiny codebase needed real investigation. | extra-step | discarded | Proportional dispatch judgment, not a reproduced product defect. |
| `VD1` | Research named the cap but not the floor that Design delegated to it. | ambiguity | discarded | This is the intentional semantic defect VD1 requires the validator to catch. |
| `X2` | none | cosmetic | none | none |
| `A1` | The driver read a legacy adoption reference despite an explicit instruction not to use it. | extra-step | discarded | The owning Skill already forbids that read, and the run still stopped at preflight without implementation inspection. |
| `DR1` | The Direct summary required canonical Requirements work that Direct implementation cannot own. | wrong-action-risk | discarded | This is the intentional reroute boundary measured by DR1. |

## Cleanup

- Fixture paths removed: `/tmp/sb-vc1-0187`, `/tmp/sb-vc1b-0187`, `/tmp/sb-vd1-0187`, `/tmp/sb-x2-0187`, `/tmp/sb-a1-0187`, `/tmp/sb-dr1-0187`
- Main worktree after recording: checked separately before the record commit
