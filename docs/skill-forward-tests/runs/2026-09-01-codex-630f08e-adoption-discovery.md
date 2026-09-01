# Forward-test run: 2026-09-01 / Codex / 630f08e

[Back to the measurement dashboard](../results.md).

- Date: `2026-09-01`
- Driver: `Codex`
- Model: `gpt-5.6-terra`
- Driver profile: `medium`, fresh contexts with no prior turns
- Tested build: `630f08e`
- Fixture language: `en`
- Scenarios: `A1`, `A2`

## Measurements

| Scenario | Verdict | Expectation that did not hold | Fixture state left behind | Mechanical evidence | Finding |
| --- | --- | --- | --- | --- | --- |
| `A1` | `environment_invalid` | The fresh driver did not receive or read the fixture-installed Discovery Skill, so the product route was not measured. | No dossier, milestone, or Spec; clean worktree. | The driver inferred a CLI path and stopped correctly on `ADOPTION_STEERING_REQUIRED`, but `.agents/skills/specbind-discovery/SKILL.md` was never consumed. | `ENV-0005` |
| `A2` | `environment_invalid` | The product route dispatched both required fresh readers, but fixture instrumentation retained only one line rather than the driver plus both readers, so the dispatch scenario could not claim a pass. | No dossier, milestone, Spec, Brief, or Research; clean worktree. | Preflight returned fixture HEAD `b0a4e5de800f4835c80051dfced4d53d66798715`; two distinct fresh reader contexts reported public-behavior and structure/seam evidence before synthesis, while `.forward-test/agents.log` contained only the structure-reader line. | `FT-0018` |

## Confirmation turns

None. A1 stopped at the missing-Steering prerequisite. A2 presented the
five-field adoption-boundary proposal and stopped for the requested first
confirmation.

## Debrief dispositions

| Scenario | Observation | Impact | Disposition | Reason or finding ID |
| --- | --- | --- | --- | --- |
| `A1` | The fresh driver had no installed-Skill registry entry and proceeded without reading the conventional Skill tree. | `wrong-action-risk` | `retained` | `ENV-0005` |
| `A2` | Capacity permitted only sequential reader dispatch, while the procedure did not say whether sequential fresh readers remained valid. | `ambiguity` | `retained` | `FT-0018`; clarified by `8329421` |
| `A2` | The driver redundantly read milestone and Spec state around the adoption preflight. | `extra-step` | `retained` | `FT-0018`; forbidden by `8329421` |
| `A2` | Shared fixture instrumentation lost context lines despite two observed reader contexts. | `wrong-action-risk` | `retained` | Environment limitation of this attempt; product artifacts stayed unchanged. |

## Cleanup

- Fixture paths removed after recording: `/tmp/sb-a1f-630f08e`, `/tmp/sb-a2f-630f08e`
- Main worktree after recording: only forward-test record changes
