# Forward-test findings worklist

[Back to the measurement dashboard](./results.md).

This is the mutable, triaged worklist. Measurements stay in the
[run archive](./runs/). A mechanical scenario pass does not by itself resolve a
usability finding; the exact affected branch must be confirmed in a fresh
fixture.

## Open

| ID | First seen | Scenario | Finding | Evidence | Next confirmation |
| --- | --- | --- | --- | --- | --- |
| FT-0011 | `388b8bc` | R8 | Requirements authoring can mistake an intentionally abstract before/after boundary for missing information and stop instead of preserving its stated abstraction. | Two fresh Codex drivers stopped on the same cancellation-window Brief until told not to invent a duration or closing event. | Clarify the Requirements contract's treatment of intentionally abstract boundaries, then rerun R8 without the extra clarification. |
| FT-0012 | `c012d50` | R6, R8 | The new-Spec path can issue a Contract read even though the Skill already says no Contract exists before Design. | Both fresh runs executed `artifact read order contract --for consume` and received `ARTIFACT_SELECTOR_NOT_FOUND` before continuing. | Give the new-Spec branch an explicit no-Contract read rule or a non-error existence route, then rerun a new-Spec Requirements scenario. |

## Fixed, behavioral confirmation pending

None.

## Resolved after migration

Historical resolved findings through 2026-08-30 remain in the
[legacy ledger](./runs/legacy-through-2026-08-30.md#resolved-usability-findings).

| ID | First seen | Scenario | Finding | Resolution | Behavioral confirmation |
| --- | --- | --- | --- | --- | --- |
| FT-0001 | `1736d0c` | S5 | A Steering read failure did not name the project path searched. | `6df80fc` adds `searched_project_path=.specbind/steering` to unknown and ambiguous selector diagnostics. | `7e6fd42`: fresh unknown and ambiguous branches both named `.specbind/steering`. |
| FT-0002 | `1736d0c` | X1 | Contract Review did not name Design as the owner of `contract.yaml` or provide the exact rewind command. | `7e6fd42` requires the Design owner, full rewind state, and exact invalidation command in the terminal report. | `7e6fd42`: fresh review reported the boundary, then invalidated only after confirmation. |
| FT-0003 | `1736d0c` | X1 | Acceptance used a prefixed deep-input selector without explaining its relation to the selector returned by `artifact list`. | `ae6c562` maps an exact listed selector to its persisted deep-input identity. | `7e6fd42`: accepted review recorded `specs/cart#design/main` and its fingerprint. |
| FT-0004 | `1736d0c` | RT1 | Review's read-only boundary and deferred adapter write had no stated ordering. | Decision 0159 fixes the verdict first under a byte-identical worktree, then permits only the adapter-directed deferred record. | `1a843d9`: fresh APPROVED review wrote only `.specbind/deferred.md` after the verdict. |
| FT-0005 | `1736d0c` | CLI recovery | Unknown nested commands could suggest an unrelated top-level command. | Decision 0159 disables token-only similarity suggestions while retaining help and usage. | `d993293`: unknown `milestone stats` emitted no unrelated suggestion and the driver recovered through supported routing. |
| FT-0006 | `4738ca2` | T1 | The default task rule did not decide whether one behavior needed a separate test task. | `cc37049` defaults tests into the behavior task unless verification is separately reviewable. | `d993293`: fresh T1 authored and approved one Task covering behavior plus automated tests. |
| FT-0007 | `7e6fd42` | S5 | Duplicate Steering recovery could choose a survivor from matching content or a copy-like filename without provenance. | `8aaa198` permits deletion only when Git history proves the path is the newly introduced duplicate. | `8aaa198`: fresh recovery inspected both histories and removed only the later duplicate. |
| FT-0008 | `d993293` | S5 | Steering authoring said not to read milestone state while requiring a completion-safety status preflight. | `7e6fd42` limits status to write-safety and excludes it from content evidence. | `1a843d9`: fresh authoring used status only for safety; independent readers supplied content evidence. |
| FT-0009 | `8aaa198` | RT1 | Review prose required initial status capture, but its command example listed the diff first. | `1a843d9` puts `git status --short` before `git diff` and tests that order. | `1a843d9`: fresh deferred review followed the aligned order and reported no ambiguity. |
| FT-0010 | `8aaa198` | HP1 | After every Task completed, `spec status` still reported implementation while milestone status routed to validation. | `20ca375` derives per-Spec validation when no Task is pending or blocked and tests text plus JSON output. | `20ca375`: at the same completed-Task fixture revision, fresh CLI reads agreed on validation and left the worktree clean. |

## Active environment limitations

| ID | Limitation | Effect |
| --- | --- | --- |
| ENV-0001 | A Claude Code Agent-tool subagent does not see fixture-installed Skills in its registry. | A fallback that reads the packaged `SKILL.md` can measure the body, but not platform selection or dispatch. A run that instead infers commands is environment-invalid. |
| ENV-0002 | The Claude Code driver appends its own status line after the agent report. | Exact terminal result blocks must be judged before the harness-owned line; the extra line is not Skill output. |
| ENV-0003 | A Claude Code Agent-tool subagent refuses approval relayed by the driving session. | Authoring scenarios that cross approval are environment-blocked at the draft boundary unless consent comes from the user through a valid channel. |
| ENV-0004 | A Codex subagent's host safety layer can reject an exact explicit gate approval relayed by the driving session. | Judge the authored artifact before the boundary and, when the maintainer has explicitly approved the exact artifact and IDs, record any driving-session execution separately from agent behavior. |

Remove an environment row when it no longer affects interpretation. Do not move
it into the product finding lifecycle merely because it recurs.
