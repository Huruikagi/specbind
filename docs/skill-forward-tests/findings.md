# Forward-test findings worklist

[Back to the measurement dashboard](./results.md).

This is the mutable, triaged worklist. Measurements stay in the
[run archive](./runs/). A mechanical scenario pass does not by itself resolve a
usability finding; the exact affected branch must be confirmed in a fresh
fixture.

## Open

No reproduced unresolved product finding remains at the migration baseline.

## Fixed, behavioral confirmation pending

| ID | First seen | Scenario | Finding | Resolution | Confirmation needed |
| --- | --- | --- | --- | --- | --- |
| FT-0001 | `1736d0c` | S5 | A Steering read failure did not name the project path searched. | `6df80fc` adds `searched_project_path=.specbind/steering` to unknown and ambiguous selector diagnostics. | Run a fresh behavioral recovery branch covering both errors. |
| FT-0002 | `1736d0c` | X1 | Contract Review did not name Design as the owner of `contract.yaml` or provide the exact rewind command. | `ae6c562` assigns the Design set and Contract to the Design phase and names `specbind spec design invalidate <spec>`. | Exercise the invalidation branch after the cost and owner are presented. |
| FT-0003 | `1736d0c` | X1 | Acceptance used a prefixed deep-input selector without explaining its relation to the selector returned by `artifact list`. | `ae6c562` maps an exact listed selector `<selector>` to `specs/<canonical-spec>#<selector>`. | Exercise an acceptance candidate that consumes the mapped selector. |
| FT-0004 | `1736d0c` | RT1 | Review's read-only boundary and deferred adapter write had no stated ordering. | Decision 0159 fixes the verdict first under a byte-identical worktree, then permits only the adapter-directed deferred record as a separate post-verdict mutation. | Exercise a deferred-candidate post-verdict write branch. |
| FT-0005 | `1736d0c` | CLI recovery | Unknown nested commands could suggest an unrelated top-level command. | Decision 0159 disables token-only similarity suggestions while retaining help and usage. | Run a fresh behavioral recovery scenario for an unknown nested command. |
| FT-0006 | `4738ca2` | T1 | The default task rule did not decide whether one behavior needed a separate test task. | `cc37049` defaults tests into the behavior task and permits separate verification only across several earlier tasks or a separately reviewable system boundary. | Rerun T1 through artifact authoring without the host safety stop. |

## Resolved after migration

None yet. Historical resolved findings through 2026-08-30 remain in the
[legacy ledger](./runs/legacy-through-2026-08-30.md#resolved-usability-findings).
New rows retain only the behavior change, fixing build, and confirming run.

## Active environment limitations

| ID | Limitation | Effect |
| --- | --- | --- |
| ENV-0001 | A Claude Code Agent-tool subagent does not see fixture-installed Skills in its registry. | A fallback that reads the packaged `SKILL.md` can measure the body, but not platform selection or dispatch. A run that instead infers commands is environment-invalid. |
| ENV-0002 | The Claude Code driver appends its own status line after the agent report. | Exact terminal result blocks must be judged before the harness-owned line; the extra line is not Skill output. |
| ENV-0003 | A Claude Code Agent-tool subagent refuses approval relayed by the driving session. | Authoring scenarios that cross approval are environment-blocked at the draft boundary unless consent comes from the user through a valid channel. |

Remove an environment row when it no longer affects interpretation. Do not move
it into the product finding lifecycle merely because it recurs.
